use std::collections::HashMap;

use anyhow::{anyhow, Result};
use jupiter_core::amm::{KeyedAccount, Quote, QuoteParams, SwapParams};
use solana_sdk::{account::Account, instruction::Instruction, pubkey::Pubkey, system_program};
use spl_token::native_mint;
use stakedex_eversol_stake_pool::EversolStakePoolStakedex;
use stakedex_interface::{
    StakeWrappedSolArgs, StakeWrappedSolIxArgs, StakeWrappedSolKeys, SwapViaStakeArgs,
    SwapViaStakeIxArgs, SwapViaStakeKeys,
};
use stakedex_marinade::MarinadeStakedex;
use stakedex_sdk_common::{
    bsol, cws_wsol_bridge_in, daopool_stake_pool, daosol, esol, eversol_stake_pool,
    find_bridge_stake, find_fee_token_acc, find_sol_bridge_out, first_avail_quote, jito_stake_pool,
    jitosol, jpool_stake_pool, jsol, laine_stake_pool, lainesol, marinade_state, msol,
    quote_pool_pair, scnsol, socean_stake_pool, solblaze_stake_pool, BaseStakePoolAmm, DepositSol,
    DepositStake, DepositStakeInfo, InitFromKeyedAccount, WithdrawStake,
};
use stakedex_socean_stake_pool::SoceanStakePoolStakedex;
use stakedex_spl_stake_pool::SplStakePoolStakedex;
use stakedex_unstake_it::UnstakeItStakedex;

pub const N_POOLS: usize = 9;

#[derive(Clone, Default)]
pub struct Stakedex {
    daopool: SplStakePoolStakedex,
    jito: SplStakePoolStakedex,
    jpool: SplStakePoolStakedex,
    laine: SplStakePoolStakedex,
    solblaze: SplStakePoolStakedex,
    socean: SoceanStakePoolStakedex,
    eversol: EversolStakePoolStakedex,
    unstakeit: UnstakeItStakedex,
    marinade: MarinadeStakedex,
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
            daopool_stake_pool::ID,
            jito_stake_pool::ID,
            jpool_stake_pool::ID,
            laine_stake_pool::ID,
            solblaze_stake_pool::ID,
            socean_stake_pool::ID,
            eversol_stake_pool::ID,
            stakedex_unstake_it::find_pool_sol_reserves().0,
            marinade_state::ID,
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

        let eversol =
            init_from_keyed_account(accounts, &eversol_stake_pool::ID).unwrap_or_else(|e| {
                errs.push(e);
                EversolStakePoolStakedex::default()
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

        let spl_stake_pools = [
            daopool_stake_pool::ID,
            jito_stake_pool::ID,
            jpool_stake_pool::ID,
            laine_stake_pool::ID,
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
            Self {
                daopool: spl_stake_pools_iter.next().unwrap(),
                jito: spl_stake_pools_iter.next().unwrap(),
                jpool: spl_stake_pools_iter.next().unwrap(),
                laine: spl_stake_pools_iter.next().unwrap(),
                solblaze: spl_stake_pools_iter.next().unwrap(),
                socean,
                unstakeit,
                eversol,
                marinade,
            },
            errs,
        )
    }

    pub fn all_pools(&self) -> [&dyn BaseStakePoolAmm; N_POOLS] {
        [
            &self.daopool,
            &self.jito,
            &self.jpool,
            &self.laine,
            &self.solblaze,
            &self.socean,
            &self.eversol,
            &self.unstakeit,
            &self.marinade,
        ]
    }

    pub fn all_pools_mut(&mut self) -> [&mut dyn BaseStakePoolAmm; N_POOLS] {
        [
            &mut self.daopool,
            &mut self.jito,
            &mut self.jpool,
            &mut self.laine,
            &mut self.solblaze,
            &mut self.socean,
            &mut self.eversol,
            &mut self.unstakeit,
            &mut self.marinade,
        ]
    }

    pub fn get_accounts_to_update(&self) -> Vec<Pubkey> {
        self.all_pools().iter().fold(Vec::new(), |mut vec, p| {
            vec.append(&mut p.get_accounts_to_update());
            vec
        })
    }

    /// Note: consumes accounts_map
    pub fn update(&mut self, accounts_map: HashMap<Pubkey, Account>) -> Vec<anyhow::Error> {
        // unstake.it special-case: required reinitialization to save sol_reserves_lamports correctly
        let maybe_unstake_it_init_err = match init_from_keyed_account(
            &accounts_map,
            &stakedex_unstake_it::find_pool_sol_reserves().0,
        ) {
            Ok(unstakeit) => {
                self.unstakeit = unstakeit;
                None
            }
            Err(e) => Some(e),
        };

        let accounts_data_map = accounts_map
            .into_iter()
            .map(|(pubkey, acc)| (pubkey, acc.data))
            .collect();

        let mut errs = self.update_data(&accounts_data_map);
        if let Some(e) = maybe_unstake_it_init_err {
            errs.push(e);
        }
        errs
    }

    pub fn update_data(
        &mut self,
        accounts_data_map: &HashMap<Pubkey, Vec<u8>>,
    ) -> Vec<anyhow::Error> {
        // So that other pools are still updated even if some pools fail to update
        self.all_pools_mut()
            .iter_mut()
            .fold(Vec::new(), |mut vec, p| {
                if let Err(e) = p.update(accounts_data_map) {
                    vec.push(e);
                }
                vec
            })
    }

    pub fn get_deposit_sol_pool(&self, token: &Pubkey) -> Option<&dyn DepositSol> {
        if bsol::check_id(token) {
            Some(&self.solblaze)
        } else if daosol::check_id(token) {
            Some(&self.daopool)
        } else if jitosol::check_id(token) {
            Some(&self.jito)
        } else if jsol::check_id(token) {
            Some(&self.jpool)
        } else if lainesol::check_id(token) {
            Some(&self.laine)
        } else if scnsol::check_id(token) {
            Some(&self.socean)
        } else if esol::check_id(token) {
            Some(&self.eversol)
        } else if msol::check_id(token) {
            Some(&self.marinade)
        } else {
            None
        }
    }

    pub fn get_deposit_stake_pool(&self, token: &Pubkey) -> Option<&dyn DepositStake> {
        if bsol::check_id(token) {
            Some(&self.solblaze)
        } else if daosol::check_id(token) {
            Some(&self.daopool)
        } else if jitosol::check_id(token) {
            Some(&self.jito)
        } else if jsol::check_id(token) {
            Some(&self.jpool)
        } else if lainesol::check_id(token) {
            Some(&self.laine)
        } else if scnsol::check_id(token) {
            Some(&self.socean)
        } else if esol::check_id(token) {
            Some(&self.eversol)
        } else if native_mint::check_id(token) {
            Some(&self.unstakeit)
        } else if msol::check_id(token) {
            Some(&self.marinade)
        } else {
            None
        }
    }

    pub fn get_withdraw_stake_pool(&self, token: &Pubkey) -> Option<&dyn WithdrawStake> {
        if bsol::check_id(token) {
            Some(&self.solblaze)
        } else if daosol::check_id(token) {
            Some(&self.daopool)
        } else if jitosol::check_id(token) {
            Some(&self.jito)
        } else if jsol::check_id(token) {
            Some(&self.jpool)
        } else if lainesol::check_id(token) {
            Some(&self.laine)
        } else if scnsol::check_id(token) {
            Some(&self.socean)
        } else if esol::check_id(token) {
            Some(&self.eversol)
        } else {
            None
        }
    }

    pub fn quote_swap_via_stake(&self, quote_params: &QuoteParams) -> Result<Quote> {
        let withdraw_from = self
            .get_withdraw_stake_pool(&quote_params.input_mint)
            .ok_or_else(|| anyhow!("pool not found {}", quote_params.input_mint))?;
        let deposit_to = self
            .get_deposit_stake_pool(&quote_params.output_mint)
            .ok_or_else(|| anyhow!("pool not found {}", quote_params.output_mint))?;
        quote_pool_pair(quote_params, withdraw_from, deposit_to)
    }

    pub fn swap_via_stake_ix(
        &self,
        swap_params: &SwapParams,
        bridge_stake_seed: u32,
    ) -> Result<Instruction> {
        let withdraw_from = self
            .get_withdraw_stake_pool(&swap_params.source_mint)
            .ok_or_else(|| anyhow!("pool not found {}", swap_params.source_mint))?;
        let deposit_to = self
            .get_deposit_stake_pool(&swap_params.destination_mint)
            .ok_or_else(|| anyhow!("pool not found {}", swap_params.destination_mint))?;
        // TODO: this is doing the same computation as it did in quote, should we cache this somehow?
        let (withdraw_quote, deposit_quote) =
            first_avail_quote(swap_params.in_amount, withdraw_from, deposit_to)
                .ok_or_else(|| anyhow!("No route found between pools"))?;
        let bridge_stake_seed_le_bytes = bridge_stake_seed.to_le_bytes();
        let bridge_stake = find_bridge_stake(
            &swap_params.user_transfer_authority,
            &bridge_stake_seed_le_bytes,
        )
        .0;
        let deposit_stake_info = DepositStakeInfo { addr: bridge_stake };

        let mut ix = stakedex_interface::swap_via_stake_ix(
            SwapViaStakeKeys {
                payer: swap_params.user_transfer_authority,
                user: swap_params.user_transfer_authority,
                src_token_from: swap_params.user_source_token_account,
                src_token_mint: swap_params.source_mint,
                dest_token_to: swap_params.user_destination_token_account,
                dest_token_mint: swap_params.destination_mint,
                dest_token_fee_token_account: find_fee_token_acc(&swap_params.destination_mint).0,
                bridge_stake,
            },
            SwapViaStakeIxArgs {
                swap_via_stake_args: SwapViaStakeArgs {
                    amount: swap_params.in_amount,
                    bridge_stake_seed,
                },
            },
        )?;
        let withdraw_from_virtual_ix = withdraw_from.virtual_ix(&withdraw_quote)?;
        ix.accounts.extend(withdraw_from_virtual_ix.accounts);
        let deposit_to_virtual_ix = deposit_to.virtual_ix(&deposit_quote, &deposit_stake_info)?;
        ix.accounts.extend(deposit_to_virtual_ix.accounts);
        Ok(ix)
    }

    pub fn quote_stake_wrapped_sol(&self, quote_params: &QuoteParams) -> Result<Quote> {
        let deposit_to = self
            .get_deposit_sol_pool(&quote_params.output_mint)
            .ok_or_else(|| anyhow!("pool not found {}", quote_params.output_mint))?;
        let deposit_sol_quote = deposit_to.get_deposit_sol_quote(quote_params.in_amount)?;
        let quote = deposit_to.convert_quote(deposit_sol_quote);
        Ok(quote)
    }

    pub fn stake_wrapped_sol_ix(&self, swap_params: &SwapParams) -> Result<Instruction> {
        let deposit_to = self
            .get_deposit_sol_pool(&swap_params.destination_mint)
            .ok_or_else(|| anyhow!("pool not found {}", swap_params.destination_mint))?;
        let (sol_bridge_out, _) = find_sol_bridge_out();

        let mut ix = stakedex_interface::stake_wrapped_sol_ix(
            StakeWrappedSolKeys {
                user: swap_params.user_transfer_authority,
                wsol_from: swap_params.user_source_token_account,
                dest_token_to: swap_params.user_destination_token_account,
                wsol_mint: swap_params.source_mint,
                dest_token_mint: swap_params.destination_mint,
                token_program: spl_token::ID,
                system_program: system_program::ID,
                wsol_bridge_in: cws_wsol_bridge_in(&sol_bridge_out),
                sol_bridge_out,
                dest_token_fee_token_account: find_fee_token_acc(&swap_params.destination_mint).0,
            },
            StakeWrappedSolIxArgs {
                stake_wrapped_sol_args: StakeWrappedSolArgs {
                    amount: swap_params.in_amount,
                },
            },
        )?;
        let deposit_sol_virtual_ix = deposit_to.virtual_ix()?;
        ix.accounts.extend(deposit_sol_virtual_ix.accounts);
        Ok(ix)
    }
}
