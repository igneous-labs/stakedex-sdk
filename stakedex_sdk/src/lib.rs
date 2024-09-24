use anyhow::{anyhow, Result};
use itertools::Itertools;
use jupiter_amm_interface::{
    AccountMap, Amm, AmmContext, ClockRef, KeyedAccount, Quote, QuoteParams, SwapParams,
};
use lazy_static::lazy_static;
use sanctum_lst_list::{PoolInfo, SanctumLst, SanctumLstList};
use solana_sdk::{instruction::Instruction, pubkey::Pubkey, system_program};
use spl_token::native_mint;
use stakedex_interface::{
    DepositStakeKeys, PrefundSwapViaStakeIxArgs, PrefundSwapViaStakeKeys,
    PrefundWithdrawStakeIxArgs, PrefundWithdrawStakeKeys, StakeWrappedSolIxArgs,
    StakeWrappedSolKeys, SwapViaStakeArgs,
};
use stakedex_jup_interface::{
    manual_concat_get_account_metas, prefund_get_account_metas, quote_pool_pair, DepositSolWrapper,
    OneWayPoolPair, PrefundRepayParams, TwoWayPoolPair,
};
use stakedex_lido::LidoStakedex;
use stakedex_marinade::MarinadeStakedex;
use stakedex_sdk_common::{
    find_fee_token_acc, lido_state, marinade_state, msol, stakedex_program, stsol,
    unstake_it_program, wsol_bridge_in, BaseStakePoolAmm, DepositSol, DepositStake,
    DepositStakeInfo, DepositStakeQuote, InitFromKeyedAccount, WithdrawStake, WithdrawStakeQuote,
    DEPOSIT_STAKE_DST_TOKEN_ACCOUNT_INDEX,
};
use stakedex_spl_stake_pool::SplStakePoolStakedex;
use stakedex_unstake_it::{UnstakeItStakedex, UnstakeItStakedexPrefund};

pub use stakedex_interface::ID as stakedex_program_id;

/// mainnet LUT that contains prefund accounts and other common accounts
pub mod srlut {
    solana_sdk::declare_id!("KtrvWWkPkhSWM9VMqafZhgnTuozQiHzrBDT8oPcMj3T");
}

pub const SWAP_VIA_STAKE_COMPUTE_BUDGET_LIMIT: u32 = 400_000;

lazy_static! {
    static ref SANCTUM_LST_LIST: SanctumLstList = SanctumLstList::load();
}

#[macro_export]
macro_rules! match_stakedexes {
    ( $Variant1:ident, $Variant2:ident, $first:pat, $second:pat ) => {
        (Stakedex::$Variant1($first), Stakedex::$Variant2($second))
            | (Stakedex::$Variant2($second), Stakedex::$Variant1($first))
    };
}

#[macro_export]
macro_rules! match_same_stakedex {
    ( $Variant:ident) => {
        (Stakedex::$Variant(_), Stakedex::$Variant(_))
    };
}

#[derive(Clone, Default)]
pub struct Stakedex {
    pub spls: Vec<SplStakePoolStakedex>,
    pub unstakeit: UnstakeItStakedexPrefund,
    pub marinade: MarinadeStakedex,
    pub lido: LidoStakedex,
}

fn get_keyed_account(accounts: &AccountMap, key: &Pubkey) -> Result<KeyedAccount> {
    Ok(KeyedAccount {
        key: *key,
        account: accounts
            .get(key)
            .ok_or_else(|| anyhow!("Missing account {}", key))?
            .clone(),
        params: None,
    })
}

fn init_from_keyed_account_no_params<P: InitFromKeyedAccount>(
    accounts: &AccountMap,
    key: &Pubkey,
) -> Result<P> {
    let keyed_acc = get_keyed_account(accounts, key)?;

    P::from_keyed_account(
        &keyed_acc,
        &AmmContext {
            clock_ref: ClockRef::default(),
        },
    )
}

impl Stakedex {
    /// Gets the list of accounts that must be fetched first to initialize
    /// Stakedex by passing the result into from_fetched_accounts()
    pub fn init_accounts<'a, I: Iterator<Item = &'a SanctumLst>>(sanctum_lsts: I) -> Vec<Pubkey> {
        sanctum_lsts
            .filter_map(|lst| match lst.pool {
                PoolInfo::SanctumSpl(accounts)
                | PoolInfo::Spl(accounts)
                | PoolInfo::SanctumSplMulti(accounts) => Some(accounts.pool),
                PoolInfo::Lido
                | PoolInfo::Marinade
                | PoolInfo::ReservePool
                | PoolInfo::SPool(..) => None,
            })
            .chain([
                unstake_it_program::SOL_RESERVES_ID,
                marinade_state::ID,
                lido_state::ID,
            ])
            .collect()
    }

    /// `sanctum_lsts` must be the same iterator passed to [`Self::init_accounts()`]
    pub fn from_fetched_accounts<'a>(
        sanctum_lsts: impl Iterator<Item = &'a SanctumLst>,
        accounts: &AccountMap,
        amm_context: &AmmContext,
    ) -> (Self, Vec<anyhow::Error>) {
        // So that stakedex is still useable even if some pools fail to load
        let mut errs = Vec::new();

        let unstakeit = UnstakeItStakedexPrefund(
            init_from_keyed_account_no_params(accounts, &unstake_it_program::SOL_RESERVES_ID)
                .unwrap_or_else(|e| {
                    errs.push(e);
                    UnstakeItStakedex::default()
                }),
        );

        let marinade = init_from_keyed_account_no_params(accounts, &marinade_state::ID)
            .unwrap_or_else(|e| {
                errs.push(e);
                MarinadeStakedex::default()
            });

        let lido =
            init_from_keyed_account_no_params(accounts, &lido_state::ID).unwrap_or_else(|e| {
                errs.push(e);
                LidoStakedex::default()
            });

        let spls = sanctum_lsts
            .filter_map(|lst| match lst.pool {
                PoolInfo::SanctumSpl(spl_accs)
                | PoolInfo::Spl(spl_accs)
                | PoolInfo::SanctumSplMulti(spl_accs) => {
                    let name = &lst.name;
                    let pool = spl_accs.pool;

                    get_keyed_account(accounts, &pool)
                        .map_or_else(Err, |mut ka| {
                            ka.params = Some(name.as_str().into());
                            SplStakePoolStakedex::from_keyed_account(&ka, &amm_context)
                        })
                        .ok()
                }
                PoolInfo::Lido
                | PoolInfo::Marinade
                | PoolInfo::ReservePool
                | PoolInfo::SPool(..) => None,
            })
            .collect();

        (
            Self {
                spls,
                unstakeit,
                marinade,
                lido,
            },
            errs,
        )
    }

    pub fn all_pools(&self) -> impl Iterator<Item = &dyn BaseStakePoolAmm> {
        self.spls
            .iter()
            .map(|spl| spl as &dyn BaseStakePoolAmm)
            .chain([
                &self.unstakeit as &dyn BaseStakePoolAmm,
                &self.marinade as &dyn BaseStakePoolAmm,
                &self.lido as &dyn BaseStakePoolAmm,
            ])
    }

    pub fn all_pools_mut(&mut self) -> impl Iterator<Item = &mut dyn BaseStakePoolAmm> {
        self.spls
            .iter_mut()
            .map(|spl| spl as &mut dyn BaseStakePoolAmm)
            .chain([
                &mut self.unstakeit as &mut dyn BaseStakePoolAmm,
                &mut self.marinade as &mut dyn BaseStakePoolAmm,
                &mut self.lido as &mut dyn BaseStakePoolAmm,
            ])
    }

    pub fn get_accounts_to_update(&self) -> Vec<Pubkey> {
        self.all_pools().fold(Vec::new(), |mut vec, p| {
            vec.append(&mut p.get_accounts_to_update());
            vec
        })
    }

    pub fn update(&mut self, account_map: &AccountMap) -> Vec<anyhow::Error> {
        // unstake.it special-case: required reinitialization to save sol_reserves_lamports correctly
        let maybe_unstake_it_init_err = match init_from_keyed_account_no_params(
            account_map,
            &unstake_it_program::SOL_RESERVES_ID,
        ) {
            Ok(unstakeit) => {
                self.unstakeit = UnstakeItStakedexPrefund(unstakeit);
                None
            }
            Err(e) => Some(e),
        };

        let mut errs = self.update_data(account_map);
        if let Some(e) = maybe_unstake_it_init_err {
            errs.push(e);
        }
        errs
    }

    pub fn update_data(&mut self, account_map: &AccountMap) -> Vec<anyhow::Error> {
        // accumulate errs in a vec so that other pools are still updated even if some pools fail to update
        self.all_pools_mut().fold(Vec::new(), |mut err_vec, p| {
            if let Err(e) = p.update(account_map) {
                err_vec.push(e);
            }
            err_vec
        })
    }

    pub fn prefund_repay_params(&self) -> PrefundRepayParams {
        PrefundRepayParams {
            fee: self.unstakeit.0.fee.fee.clone(),
            incoming_stake: self.unstakeit.0.pool.incoming_stake,
            sol_reserves_lamports: self.unstakeit.0.sol_reserves_lamports,
            protocol_fee_dest: self.unstakeit.0.protocol_fee.destination,
        }
    }

    // TODO: maybe build an index of { mint: SplStakePoolStakedex } for faster lookups
    // when there be many SplStakePoolStakedexes in the future
    pub fn get_deposit_sol_pool(&self, mint: &Pubkey) -> Option<&dyn DepositSol> {
        Some(match *mint {
            msol::ID => &self.marinade,
            mint => self
                .spls
                .iter()
                .find(|SplStakePoolStakedex { stake_pool, .. }| stake_pool.pool_mint == mint)?,
        })
    }

    pub fn get_deposit_stake_pool(&self, mint: &Pubkey) -> Option<&dyn DepositStake> {
        Some(match *mint {
            msol::ID => &self.marinade,
            native_mint::ID => &self.unstakeit,
            mint => self
                .spls
                .iter()
                .find(|SplStakePoolStakedex { stake_pool, .. }| stake_pool.pool_mint == mint)?,
        })
    }

    pub fn get_withdraw_stake_pool(&self, mint: &Pubkey) -> Option<&dyn WithdrawStake> {
        Some(match *mint {
            stsol::ID => &self.lido,
            mint => self
                .spls
                .iter()
                .find(|SplStakePoolStakedex { stake_pool, .. }| stake_pool.pool_mint == mint)?,
        })
    }

    pub fn quote_swap_via_stake(&self, quote_params: &QuoteParams) -> Result<Quote> {
        let withdraw_from = self
            .get_withdraw_stake_pool(&quote_params.input_mint)
            .ok_or_else(|| anyhow!("pool not found for input mint {}", quote_params.input_mint))?;
        let deposit_to = self
            .get_deposit_stake_pool(&quote_params.output_mint)
            .ok_or_else(|| {
                anyhow!(
                    "pool not found for output mint {}",
                    quote_params.output_mint
                )
            })?;
        quote_pool_pair(
            quote_params,
            &self.prefund_repay_params(),
            withdraw_from,
            deposit_to,
        )
    }

    pub fn manual_concat_prefund_swap_via_stake_ixs(
        &self,
        swap_params: &SwapParams,
        bridge_stake_seed: u32,
    ) -> Result<[Instruction; 2]> {
        let withdraw_from = self
            .get_withdraw_stake_pool(&swap_params.source_mint)
            .ok_or_else(|| anyhow!("pool not found for src mint {}", swap_params.source_mint))?;
        let deposit_to = self
            .get_deposit_stake_pool(&swap_params.destination_mint)
            .ok_or_else(|| {
                anyhow!(
                    "pool not found for dst mint {}",
                    swap_params.destination_mint
                )
            })?;
        let mut prefund_withdraw_stake_ix = stakedex_interface::prefund_withdraw_stake_ix(
            // dont cares for keys, since we replace them with
            // get_account_metas()
            PrefundWithdrawStakeKeys {
                user: Pubkey::default(),
                src_token_from: Pubkey::default(),
                bridge_stake: Pubkey::default(),
                src_token_mint: Pubkey::default(),
                prefunder: Pubkey::default(),
                slumdog_stake: Pubkey::default(),
                unstakeit_program: Pubkey::default(),
                unstake_pool: Pubkey::default(),
                pool_sol_reserves: Pubkey::default(),
                unstake_fee: Pubkey::default(),
                slumdog_stake_acc_record: Pubkey::default(),
                unstake_protocol_fee: Pubkey::default(),
                unstake_protocol_fee_dest: Pubkey::default(),
                clock: Pubkey::default(),
                stake_program: Pubkey::default(),
                system_program: Pubkey::default(),
            },
            PrefundWithdrawStakeIxArgs {
                args: SwapViaStakeArgs {
                    amount: swap_params.in_amount,
                    bridge_stake_seed,
                },
            },
        )?;
        let mut deposit_stake_ix = stakedex_interface::deposit_stake_ix(
            // dont cares for keys, since we replace them with
            // get_account_metas()
            DepositStakeKeys {
                user: Pubkey::default(),
                stake_account: Pubkey::default(),
                dest_token_to: Pubkey::default(),
                dest_token_fee_token_account: Pubkey::default(),
                dest_token_mint: Pubkey::default(),
            },
        )?;
        let metas = manual_concat_get_account_metas(
            swap_params,
            &self.prefund_repay_params(),
            withdraw_from,
            deposit_to,
            bridge_stake_seed,
        )?;
        let split_at = metas
            .iter()
            .position(|meta| *meta == swap_params.placeholder_account_meta())
            .unwrap();
        prefund_withdraw_stake_ix.accounts = metas[..split_at].into();
        deposit_stake_ix.accounts = metas[split_at + 1..].into();
        Ok([prefund_withdraw_stake_ix, deposit_stake_ix])
    }

    pub fn prefund_swap_via_stake_ix(
        &self,
        swap_params: &SwapParams,
        bridge_stake_seed: u32,
    ) -> Result<Instruction> {
        let withdraw_from = self
            .get_withdraw_stake_pool(&swap_params.source_mint)
            .ok_or_else(|| anyhow!("pool not found for src mint {}", swap_params.source_mint))?;
        let deposit_to = self
            .get_deposit_stake_pool(&swap_params.destination_mint)
            .ok_or_else(|| {
                anyhow!(
                    "pool not found for dst mint {}",
                    swap_params.destination_mint
                )
            })?;
        let mut ix = stakedex_interface::prefund_swap_via_stake_ix(
            // dont cares for keys, since we replace them with
            // get_account_metas()
            PrefundSwapViaStakeKeys {
                user: Pubkey::default(),
                src_token_from: Pubkey::default(),
                dest_token_to: Pubkey::default(),
                bridge_stake: Pubkey::default(),
                dest_token_fee_token_account: Pubkey::default(),
                src_token_mint: Pubkey::default(),
                dest_token_mint: Pubkey::default(),
                prefunder: Pubkey::default(),
                slumdog_stake: Pubkey::default(),
                unstakeit_program: Pubkey::default(),
                unstake_pool: Pubkey::default(),
                pool_sol_reserves: Pubkey::default(),
                unstake_fee: Pubkey::default(),
                slumdog_stake_acc_record: Pubkey::default(),
                unstake_protocol_fee: Pubkey::default(),
                unstake_protocol_fee_dest: Pubkey::default(),
                clock: Pubkey::default(),
                stake_program: Pubkey::default(),
                system_program: Pubkey::default(),
            },
            PrefundSwapViaStakeIxArgs {
                args: SwapViaStakeArgs {
                    amount: swap_params.in_amount,
                    bridge_stake_seed,
                },
            },
        )?;
        // TODO: this is doing the same computation as it did in quote, should we cache this somehow?
        ix.accounts = prefund_get_account_metas(
            swap_params,
            &self.prefund_repay_params(),
            withdraw_from,
            deposit_to,
            bridge_stake_seed,
        )?;
        Ok(ix)
    }

    pub fn quote_stake_wrapped_sol(&self, quote_params: &QuoteParams) -> Result<Quote> {
        let deposit_to = self
            .get_deposit_sol_pool(&quote_params.output_mint)
            .ok_or_else(|| {
                anyhow!(
                    "pool not found for output mint {}",
                    quote_params.output_mint
                )
            })?;
        let deposit_sol_quote = deposit_to.get_deposit_sol_quote(quote_params.amount)?;
        let quote = deposit_to.convert_quote(deposit_sol_quote);
        Ok(quote)
    }

    pub fn stake_wrapped_sol_ix(&self, swap_params: &SwapParams) -> Result<Instruction> {
        let deposit_to = self
            .get_deposit_sol_pool(&swap_params.destination_mint)
            .ok_or_else(|| {
                anyhow!(
                    "pool not found for dst mint {}",
                    swap_params.destination_mint
                )
            })?;
        let mut ix = stakedex_interface::stake_wrapped_sol_ix(
            StakeWrappedSolKeys {
                user: swap_params.token_transfer_authority,
                wsol_from: swap_params.source_token_account,
                dest_token_to: swap_params.destination_token_account,
                wsol_mint: swap_params.source_mint,
                dest_token_mint: swap_params.destination_mint,
                token_program: spl_token::ID,
                system_program: system_program::ID,
                wsol_bridge_in: wsol_bridge_in::ID,
                sol_bridge_out: stakedex_program::SOL_BRIDGE_OUT_ID,
                dest_token_fee_token_account: find_fee_token_acc(&swap_params.destination_mint).0,
            },
            StakeWrappedSolIxArgs {
                amount: swap_params.in_amount,
            },
        )?;
        let deposit_sol_virtual_ix = deposit_to.virtual_ix()?;
        ix.accounts.extend(deposit_sol_virtual_ix.accounts);
        Ok(ix)
    }

    /// input_mint = voter pubkey for deposit stake
    pub fn quote_deposit_stake(&self, quote_params: &QuoteParams) -> Result<Quote> {
        let (deposit_to, dsq) = self.quote_deposit_stake_dsq(
            &quote_params.output_mint,
            &quote_params.input_mint,
            quote_params.amount,
        )?;
        Ok(deposit_to.convert_deposit_stake_quote(quote_params.amount, dsq))
    }

    /// Inner fn for [`Self::quote_deposit_stake()`].
    /// Returns (stake pool, DepositStakeQuote)
    fn quote_deposit_stake_dsq(
        &self,
        output_mint: &Pubkey,
        voter: &Pubkey,
        in_amount: u64,
    ) -> Result<(&dyn DepositStake, DepositStakeQuote)> {
        let deposit_to = self
            .get_deposit_stake_pool(output_mint)
            .ok_or_else(|| anyhow!("pool not found for output mint {}", output_mint))?;
        let wsq = WithdrawStakeQuote::from_lamports_and_voter(in_amount, *voter);
        let dsq = deposit_to.get_deposit_stake_quote(wsq)?;
        if dsq.is_zero_out() {
            return Err(anyhow!("pool cannot accept stake account"));
        }
        Ok((deposit_to, dsq))
    }

    /// source_mint = voter pubkey for stake acc to be deposited
    /// source_token_account = stake acc to be deposited
    pub fn deposit_stake_ix(&self, swap_params: &SwapParams) -> Result<Instruction> {
        let (deposit_to, dsq) = self.quote_deposit_stake_dsq(
            &swap_params.destination_mint,
            &swap_params.source_mint,
            swap_params.in_amount,
        )?;
        let stake_account = swap_params.source_token_account;
        let mut ix = stakedex_interface::deposit_stake_ix(DepositStakeKeys {
            user: swap_params.token_transfer_authority,
            stake_account,
            dest_token_to: swap_params.destination_token_account,
            dest_token_fee_token_account: find_fee_token_acc(&swap_params.destination_mint).0,
            dest_token_mint: swap_params.destination_mint,
        })?;
        let deposit_to_virtual_ix = deposit_to.virtual_ix(
            &dsq,
            &DepositStakeInfo {
                addr: stake_account,
            },
        )?;
        if ix.accounts[DEPOSIT_STAKE_DST_TOKEN_ACCOUNT_INDEX].pubkey == native_mint::ID {
            ix.accounts[DEPOSIT_STAKE_DST_TOKEN_ACCOUNT_INDEX].is_writable = false;
        }
        ix.accounts.extend(deposit_to_virtual_ix.accounts);
        Ok(ix)
    }

    /// Creates all possible Amms from the underlying available Stakedexes
    pub fn get_amms(self) -> Vec<Box<dyn Amm + Send + Sync>> {
        #[derive(Clone)]
        enum Stakedex {
            SplStakePool(SplStakePoolStakedex),
            UnstakeIt(UnstakeItStakedexPrefund),
            Marinade(MarinadeStakedex),
            Lido(LidoStakedex),
        }

        let Self {
            spls,
            unstakeit,
            marinade,
            lido,
        } = self;

        let stakedexes: Vec<Stakedex> = spls
            .into_iter()
            .map(Stakedex::SplStakePool)
            .chain([
                Stakedex::UnstakeIt(unstakeit),
                Stakedex::Marinade(marinade),
                Stakedex::Lido(lido),
            ])
            .collect();

        let mut amms: Vec<Box<dyn Amm + Send + Sync>> = Vec::new();
        for stakedex in stakedexes.iter() {
            match stakedex {
                Stakedex::SplStakePool(spl_stake_pool) => {
                    amms.push(Box::new(DepositSolWrapper(spl_stake_pool.clone())))
                }
                Stakedex::Marinade(marinade) => {
                    amms.push(Box::new(DepositSolWrapper(marinade.clone())))
                }
                // non-DepositSol
                Stakedex::UnstakeIt(_) => (),
                Stakedex::Lido(_) => (),
            }
        }

        // SplStakePool WithdrawStake + DepositStake
        // UnstakeIt DepositStake
        // Marinade DepositStake
        // Lido WithdrawStake
        for (first_stakedex, second_stakedex) in stakedexes.into_iter().tuple_combinations() {
            match (first_stakedex, second_stakedex) {
                (Stakedex::SplStakePool(p1), Stakedex::SplStakePool(p2)) => {
                    amms.push(Box::new(TwoWayPoolPair::new(p1, p2)));
                }
                match_stakedexes!(SplStakePool, Marinade, withdraw, deposit) => {
                    amms.push(Box::new(OneWayPoolPair::new(withdraw, deposit)));
                }
                match_stakedexes!(SplStakePool, UnstakeIt, withdraw, deposit) => {
                    amms.push(Box::new(OneWayPoolPair::new(withdraw, deposit)));
                }
                match_stakedexes!(Lido, SplStakePool, withdraw, deposit) => {
                    amms.push(Box::new(OneWayPoolPair::new(withdraw, deposit)));
                }
                match_stakedexes!(Lido, UnstakeIt, withdraw, deposit) => {
                    amms.push(Box::new(OneWayPoolPair::new(withdraw, deposit)));
                }
                match_stakedexes!(Lido, Marinade, withdraw, deposit) => {
                    amms.push(Box::new(OneWayPoolPair::new(withdraw, deposit)));
                }
                match_stakedexes!(Marinade, UnstakeIt, _, _) => (), // Cannot do anything with those two
                match_same_stakedex!(UnstakeIt)
                | match_same_stakedex!(Marinade)
                | match_same_stakedex!(Lido) => (), // Invalid if found
            }
        }

        amms
    }
}

/// Used by jup, do not delete
pub mod test_utils {
    pub use stakedex_jup_interface::DepositSolWrapper;
    pub use stakedex_lido::LidoStakedex;
    pub use stakedex_marinade::MarinadeStakedex;
    pub use stakedex_sdk_common::{
        jito_stake_pool, lido_program, lido_state, marinade_program, marinade_state,
        socean_program, socean_stake_pool, unstake_it_pool,
    };
    pub use stakedex_spl_stake_pool as spl_stake_pool;
    pub use stakedex_spl_stake_pool::SplStakePoolStakedex;
    pub use stakedex_unstake_it::UnstakeItStakedex;
}
