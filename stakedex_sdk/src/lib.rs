use std::collections::HashMap;

use anyhow::{anyhow, Result};
use itertools::Itertools;
use jupiter_amm_interface::{AccountMap, Amm, KeyedAccount, Quote, QuoteParams, SwapParams};
use solana_sdk::{
    account::Account, clock::Clock, instruction::Instruction, pubkey::Pubkey, system_program,
};
use spl_token::native_mint;
pub use stakedex_interface::ID as stakedex_program_id;
use stakedex_interface::{
    DepositStakeKeys, StakeWrappedSolIxArgs, StakeWrappedSolKeys, SwapViaStakeIxArgs,
    SwapViaStakeKeys,
};
use stakedex_lido::LidoStakedex;
use stakedex_marinade::MarinadeStakedex;
use stakedex_sdk_common::{
    bsol, cogent_stake_pool, cogentsol, cws_wsol_bridge_in, daopool_stake_pool, daosol,
    find_bridge_stake, find_fee_token_acc, find_sol_bridge_out, first_avail_quote, jito_stake_pool,
    jitosol, jpool_stake_pool, jsol, laine_stake_pool, lainesol, lido_state, lst, marinade_state,
    mrgn_stake_pool, msol, quote_pool_pair, risklol_stake_pool, risksol, scnsol, socean_stake_pool,
    solblaze_stake_pool, stsol, BaseStakePoolAmm, DepositSol, DepositSolWrapper, DepositStake,
    DepositStakeInfo, DepositStakeQuote, InitFromKeyedAccount, OneWayPoolPair, TwoWayPoolPair,
    WithdrawStake, WithdrawStakeQuote, DEPOSIT_STAKE_DST_TOKEN_ACCOUNT_INDEX,
    SWAP_VIA_STAKE_DST_TOKEN_MINT_ACCOUNT_INDEX, SWAP_VIA_STAKE_SRC_TOKEN_MINT_ACCOUNT_INDEX,
};
use stakedex_socean_stake_pool::SoceanStakePoolStakedex;
use stakedex_spl_stake_pool::SplStakePoolStakedex;
use stakedex_unstake_it::UnstakeItStakedex;

pub const N_POOLS: usize = 12;

pub const N_DEPOSIT_SOL_POOLS: usize = 10;

pub const N_DEPOSIT_STAKE_POOLS: usize = 11;

pub const N_WITHDRAW_STAKE_POOLS: usize = 10;

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
    pub cogent: SplStakePoolStakedex,
    pub daopool: SplStakePoolStakedex,
    pub jito: SplStakePoolStakedex,
    pub jpool: SplStakePoolStakedex,
    pub laine: SplStakePoolStakedex,
    pub mrgn: SplStakePoolStakedex,
    pub risklol: SplStakePoolStakedex,
    pub solblaze: SplStakePoolStakedex,
    pub socean: SoceanStakePoolStakedex,
    pub unstakeit: UnstakeItStakedex,
    pub marinade: MarinadeStakedex,
    pub lido: LidoStakedex,
}

fn get_keyed_account(accounts: &HashMap<Pubkey, Account>, key: &Pubkey) -> Result<KeyedAccount> {
    Ok(KeyedAccount {
        key: *key,
        account: accounts
            .get(key)
            .ok_or_else(|| anyhow!("Missing account {}", key))?
            .clone(),
        params: None,
    })
}

fn init_from_keyed_account<P: InitFromKeyedAccount>(
    accounts: &HashMap<Pubkey, Account>,
    key: &Pubkey,
) -> Result<P> {
    let keyed_acc = get_keyed_account(accounts, key)?;
    P::from_keyed_account(&keyed_acc)
}

impl Stakedex {
    /// Gets the list of accounts that must be fetched first to initialize
    /// Stakedex by passing the result into from_fetched_accounts()
    pub fn init_accounts() -> [Pubkey; N_POOLS] {
        [
            cogent_stake_pool::ID,
            daopool_stake_pool::ID,
            jito_stake_pool::ID,
            jpool_stake_pool::ID,
            laine_stake_pool::ID,
            mrgn_stake_pool::ID,
            risklol_stake_pool::ID,
            solblaze_stake_pool::ID,
            socean_stake_pool::ID,
            stakedex_unstake_it::find_pool_sol_reserves().0,
            marinade_state::ID,
            lido_state::ID,
        ]
    }

    pub fn from_fetched_accounts(
        accounts: &HashMap<Pubkey, Account>,
    ) -> (Self, Vec<anyhow::Error>) {
        // So that stakedex is still useable even if some pools fail to load
        let mut errs = Vec::new();

        let socean =
            init_from_keyed_account(accounts, &socean_stake_pool::ID).unwrap_or_else(|e| {
                errs.push(e);
                SoceanStakePoolStakedex::default()
            });

        let unstakeit =
            init_from_keyed_account(accounts, &stakedex_unstake_it::find_pool_sol_reserves().0)
                .unwrap_or_else(|e| {
                    errs.push(e);
                    UnstakeItStakedex::default()
                });

        let marinade = init_from_keyed_account(accounts, &marinade_state::ID).unwrap_or_else(|e| {
            errs.push(e);
            MarinadeStakedex::default()
        });

        let lido = init_from_keyed_account(accounts, &lido_state::ID).unwrap_or_else(|e| {
            errs.push(e);
            LidoStakedex::default()
        });

        let spl_stake_pools = [
            cogent_stake_pool::ID,
            daopool_stake_pool::ID,
            jito_stake_pool::ID,
            jpool_stake_pool::ID,
            laine_stake_pool::ID,
            mrgn_stake_pool::ID,
            risklol_stake_pool::ID,
            solblaze_stake_pool::ID,
        ]
        .map(|pool_id| {
            init_from_keyed_account(accounts, &pool_id).unwrap_or_else(|e| {
                errs.push(e);
                SplStakePoolStakedex::default()
            })
        });
        // unwrap safety: spl_stake_pools length is known
        let mut spl_stake_pools_iter = spl_stake_pools.into_iter();
        (
            // NB: take note of order of `spl_stake_pools
            Self {
                cogent: spl_stake_pools_iter.next().unwrap(),
                daopool: spl_stake_pools_iter.next().unwrap(),
                jito: spl_stake_pools_iter.next().unwrap(),
                jpool: spl_stake_pools_iter.next().unwrap(),
                laine: spl_stake_pools_iter.next().unwrap(),
                mrgn: spl_stake_pools_iter.next().unwrap(),
                risklol: spl_stake_pools_iter.next().unwrap(),
                solblaze: spl_stake_pools_iter.next().unwrap(),
                socean,
                unstakeit,
                marinade,
                lido,
            },
            errs,
        )
    }

    pub fn all_pools(&self) -> [&dyn BaseStakePoolAmm; N_POOLS] {
        [
            &self.cogent,
            &self.daopool,
            &self.jito,
            &self.jpool,
            &self.laine,
            &self.mrgn,
            &self.risklol,
            &self.solblaze,
            &self.socean,
            &self.unstakeit,
            &self.marinade,
            &self.lido,
        ]
    }

    pub fn all_pools_mut(&mut self) -> [&mut dyn BaseStakePoolAmm; N_POOLS] {
        [
            &mut self.cogent,
            &mut self.daopool,
            &mut self.jito,
            &mut self.jpool,
            &mut self.laine,
            &mut self.mrgn,
            &mut self.risklol,
            &mut self.solblaze,
            &mut self.socean,
            &mut self.unstakeit,
            &mut self.marinade,
            &mut self.lido,
        ]
    }

    pub fn get_accounts_to_update(&self) -> Vec<Pubkey> {
        self.all_pools().iter().fold(Vec::new(), |mut vec, p| {
            vec.append(&mut p.get_accounts_to_update());
            vec
        })
    }

    /// Note: consumes accounts_map
    pub fn update(&mut self, account_map: HashMap<Pubkey, Account>) -> Vec<anyhow::Error> {
        // unstake.it special-case: required reinitialization to save sol_reserves_lamports correctly
        let maybe_unstake_it_init_err = match init_from_keyed_account(
            &account_map,
            &stakedex_unstake_it::find_pool_sol_reserves().0,
        ) {
            Ok(unstakeit) => {
                self.unstakeit = unstakeit;
                None
            }
            Err(e) => Some(e),
        };

        let mut errs = self.update_data(&account_map);
        if let Some(e) = maybe_unstake_it_init_err {
            errs.push(e);
        }
        errs
    }

    pub fn update_data(&mut self, account_map: &AccountMap) -> Vec<anyhow::Error> {
        // So that other pools are still updated even if some pools fail to update
        self.all_pools_mut()
            .iter_mut()
            .fold(Vec::new(), |mut vec, p| {
                if let Err(e) = p.update(account_map) {
                    vec.push(e);
                }
                vec
            })
    }

    fn token_to_deposit_sol(&self) -> [(Pubkey, &dyn DepositSol); N_DEPOSIT_SOL_POOLS] {
        [
            (bsol::ID, &self.solblaze),
            (cogentsol::ID, &self.cogent),
            (daosol::ID, &self.daopool),
            (jitosol::ID, &self.jito),
            (jsol::ID, &self.jpool),
            (lainesol::ID, &self.laine),
            (lst::ID, &self.mrgn),
            (risksol::ID, &self.risklol),
            (scnsol::ID, &self.socean),
            (msol::ID, &self.marinade),
        ]
    }

    pub fn get_deposit_sol_pool(&self, token: &Pubkey) -> Option<&dyn DepositSol> {
        self.token_to_deposit_sol()
            .into_iter()
            .find(|(token_key, _)| token_key == token)
            .map(|(_, ptr)| ptr)
    }

    pub fn token_to_deposit_stake(&self) -> [(Pubkey, &dyn DepositStake); N_DEPOSIT_STAKE_POOLS] {
        [
            (bsol::ID, &self.solblaze),
            (cogentsol::ID, &self.cogent),
            (daosol::ID, &self.daopool),
            (jitosol::ID, &self.jito),
            (jsol::ID, &self.jpool),
            (lainesol::ID, &self.laine),
            (lst::ID, &self.mrgn),
            (risksol::ID, &self.risklol),
            (scnsol::ID, &self.socean),
            (msol::ID, &self.marinade),
            (native_mint::ID, &self.unstakeit),
        ]
    }

    pub fn get_deposit_stake_pool(&self, token: &Pubkey) -> Option<&dyn DepositStake> {
        self.token_to_deposit_stake()
            .into_iter()
            .find(|(token_key, _)| token_key == token)
            .map(|(_, ptr)| ptr)
    }

    pub fn token_to_withdraw_stake(
        &self,
    ) -> [(Pubkey, &dyn WithdrawStake); N_WITHDRAW_STAKE_POOLS] {
        [
            (bsol::ID, &self.solblaze),
            (cogentsol::ID, &self.cogent),
            (daosol::ID, &self.daopool),
            (jitosol::ID, &self.jito),
            (jsol::ID, &self.jpool),
            (lainesol::ID, &self.laine),
            (lst::ID, &self.mrgn),
            (risksol::ID, &self.risklol),
            (scnsol::ID, &self.socean),
            (stsol::ID, &self.lido),
        ]
    }

    pub fn get_withdraw_stake_pool(&self, token: &Pubkey) -> Option<&dyn WithdrawStake> {
        self.token_to_withdraw_stake()
            .into_iter()
            .find(|(token_key, _)| token_key == token)
            .map(|(_, ptr)| ptr)
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
        quote_pool_pair(quote_params, withdraw_from, deposit_to)
    }

    pub fn swap_via_stake_ix(
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
        // TODO: this is doing the same computation as it did in quote, should we cache this somehow?
        let (withdraw_quote, deposit_quote) =
            first_avail_quote(swap_params.in_amount, withdraw_from, deposit_to)?;
        let bridge_stake_seed_le_bytes = bridge_stake_seed.to_le_bytes();
        let bridge_stake = find_bridge_stake(
            &swap_params.token_transfer_authority,
            &bridge_stake_seed_le_bytes,
        )
        .0;
        let deposit_stake_info = DepositStakeInfo { addr: bridge_stake };

        let mut ix = stakedex_interface::swap_via_stake_ix(
            SwapViaStakeKeys {
                user: swap_params.token_transfer_authority,
                src_token_from: swap_params.source_token_account,
                src_token_mint: swap_params.source_mint,
                dest_token_to: swap_params.destination_token_account,
                dest_token_mint: swap_params.destination_mint,
                dest_token_fee_token_account: find_fee_token_acc(&swap_params.destination_mint).0,
                bridge_stake,
            },
            SwapViaStakeIxArgs {
                amount: swap_params.in_amount,
                bridge_stake_seed,
            },
        )?;
        for mint_idx in [
            SWAP_VIA_STAKE_SRC_TOKEN_MINT_ACCOUNT_INDEX,
            SWAP_VIA_STAKE_DST_TOKEN_MINT_ACCOUNT_INDEX,
        ] {
            if ix.accounts[mint_idx].pubkey == native_mint::ID {
                ix.accounts[mint_idx].is_writable = false;
            }
        }
        let withdraw_from_virtual_ix = withdraw_from.virtual_ix(&withdraw_quote)?;
        ix.accounts.extend(withdraw_from_virtual_ix.accounts);
        let deposit_to_virtual_ix = deposit_to.virtual_ix(&deposit_quote, &deposit_stake_info)?;
        ix.accounts.extend(deposit_to_virtual_ix.accounts);
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
        let (sol_bridge_out, _) = find_sol_bridge_out();

        let mut ix = stakedex_interface::stake_wrapped_sol_ix(
            StakeWrappedSolKeys {
                user: swap_params.token_transfer_authority,
                wsol_from: swap_params.source_token_account,
                dest_token_to: swap_params.destination_token_account,
                wsol_mint: swap_params.source_mint,
                dest_token_mint: swap_params.destination_mint,
                token_program: spl_token::ID,
                system_program: system_program::ID,
                wsol_bridge_in: cws_wsol_bridge_in(&sol_bridge_out),
                sol_bridge_out,
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
    pub fn get_amms(&self) -> Vec<Box<dyn Amm + Send + Sync>> {
        #[derive(Clone)]
        enum Stakedex {
            SplStakePool(SplStakePoolStakedex),
            Socean(SoceanStakePoolStakedex),
            UnstakeIt(UnstakeItStakedex),
            Marinade(MarinadeStakedex),
            Lido(LidoStakedex),
        }

        let stakedexes = vec![
            Stakedex::SplStakePool(self.cogent.clone()),
            Stakedex::SplStakePool(self.daopool.clone()),
            Stakedex::SplStakePool(self.jito.clone()),
            Stakedex::SplStakePool(self.jpool.clone()),
            Stakedex::SplStakePool(self.laine.clone()),
            Stakedex::SplStakePool(self.mrgn.clone()),
            Stakedex::SplStakePool(self.risklol.clone()),
            Stakedex::SplStakePool(self.solblaze.clone()),
            Stakedex::Socean(self.socean.clone()),
            Stakedex::UnstakeIt(self.unstakeit.clone()),
            Stakedex::Marinade(self.marinade.clone()),
            Stakedex::Lido(self.lido.clone()),
        ];

        let mut amms: Vec<Box<dyn Amm + Send + Sync>> = Vec::new();
        for stakedex in stakedexes.iter() {
            match stakedex {
                Stakedex::SplStakePool(spl_stake_pool) => {
                    amms.push(Box::new(DepositSolWrapper(spl_stake_pool.clone())))
                }
                Stakedex::Socean(socean) => amms.push(Box::new(DepositSolWrapper(socean.clone()))),
                Stakedex::Marinade(marinade) => {
                    amms.push(Box::new(DepositSolWrapper(marinade.clone())))
                }
                // non-DepositSol
                Stakedex::UnstakeIt(_) => (),
                Stakedex::Lido(_) => (),
            }
        }

        // SplStakePool WithdrawStake + DepositStake
        // Socean WithdrawStake + DepositStake
        // UnstakeIt DepositStake
        // Marinade DepositStake
        // Lido WithdrawStake
        for (first_stakedex, second_stakedex) in stakedexes.into_iter().tuple_combinations() {
            match (first_stakedex, second_stakedex) {
                (Stakedex::SplStakePool(p1), Stakedex::SplStakePool(p2)) => {
                    amms.push(Box::new(TwoWayPoolPair {
                        p1,
                        p2,
                        clock: Clock::default(),
                    }));
                }
                match_stakedexes!(SplStakePool, Socean, p1, p2) => {
                    amms.push(Box::new(TwoWayPoolPair {
                        p1,
                        p2,
                        clock: Clock::default(),
                    }));
                }
                match_stakedexes!(SplStakePool, Marinade, withdraw, deposit) => {
                    amms.push(Box::new(OneWayPoolPair {
                        withdraw,
                        deposit,
                        clock: Clock::default(),
                    }));
                }
                match_stakedexes!(SplStakePool, UnstakeIt, withdraw, deposit) => {
                    amms.push(Box::new(OneWayPoolPair {
                        withdraw,
                        deposit,
                        clock: Clock::default(),
                    }));
                }
                match_stakedexes!(Socean, UnstakeIt, withdraw, deposit) => {
                    amms.push(Box::new(OneWayPoolPair {
                        withdraw,
                        deposit,
                        clock: Clock::default(),
                    }));
                }
                match_stakedexes!(Socean, Marinade, withdraw, deposit) => {
                    amms.push(Box::new(OneWayPoolPair {
                        withdraw,
                        deposit,
                        clock: Clock::default(),
                    }));
                }
                match_stakedexes!(Lido, SplStakePool, withdraw, deposit) => {
                    amms.push(Box::new(OneWayPoolPair {
                        withdraw,
                        deposit,
                        clock: Clock::default(),
                    }));
                }
                match_stakedexes!(Lido, Socean, withdraw, deposit) => {
                    amms.push(Box::new(OneWayPoolPair {
                        withdraw,
                        deposit,
                        clock: Clock::default(),
                    }));
                }
                match_stakedexes!(Lido, UnstakeIt, withdraw, deposit) => {
                    amms.push(Box::new(OneWayPoolPair {
                        withdraw,
                        deposit,
                        clock: Clock::default(),
                    }));
                }
                match_stakedexes!(Lido, Marinade, withdraw, deposit) => {
                    amms.push(Box::new(OneWayPoolPair {
                        withdraw,
                        deposit,
                        clock: Clock::default(),
                    }));
                }
                match_stakedexes!(Marinade, UnstakeIt, _, _) => (), // Cannot do anything with those two
                match_same_stakedex!(Socean)
                | match_same_stakedex!(UnstakeIt)
                | match_same_stakedex!(Marinade)
                | match_same_stakedex!(Lido) => (), // Invalid if found
            }
        }

        println!(
            "StakeDex amms: {:?}",
            amms.iter().map(|amm| amm.label()).collect::<Vec<_>>()
        );

        amms
    }
}
