use std::collections::HashMap;

use anyhow::{anyhow, Result};
use jupiter_core::amm::{KeyedAccount, Quote, QuoteParams, SwapParams};
use solana_sdk::{account::Account, instruction::Instruction, pubkey::Pubkey, system_program};
use stakedex_interface::{
    StakeWrappedSolArgs, StakeWrappedSolIxArgs, StakeWrappedSolKeys, SwapViaStakeArgs,
    SwapViaStakeIxArgs, SwapViaStakeKeys,
};
use stakedex_sdk_common::{
    bsol, cws_wsol_bridge_in, daopool_stake_pool, daosol, find_bridge_stake, find_fee_token_acc,
    find_sol_bridge_out, first_avail_quote, jito_stake_pool, jitosol, jpool_stake_pool, jsol,
    laine_stake_pool, lainesol, quote_pool_pair, solblaze_stake_pool, BaseStakePoolAmm, DepositSol,
    DepositStake, WithdrawStake,
};
use stakedex_spl_stake_pool::SplStakePoolStakedex;

pub struct Stakedex {
    daopool: SplStakePoolStakedex,
    jito: SplStakePoolStakedex,
    jpool: SplStakePoolStakedex,
    laine: SplStakePoolStakedex,
    solblaze: SplStakePoolStakedex,
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

impl Stakedex {
    pub fn from_fetched_accounts(accounts: &HashMap<Pubkey, Account>) -> Result<Self> {
        Ok(Self {
            daopool: SplStakePoolStakedex::from_keyed_account(&get_keyed_account(
                accounts,
                &daopool_stake_pool::ID,
            )?)?,
            jito: SplStakePoolStakedex::from_keyed_account(&get_keyed_account(
                accounts,
                &jito_stake_pool::ID,
            )?)?,
            jpool: SplStakePoolStakedex::from_keyed_account(&get_keyed_account(
                accounts,
                &jpool_stake_pool::ID,
            )?)?,
            laine: SplStakePoolStakedex::from_keyed_account(&get_keyed_account(
                accounts,
                &laine_stake_pool::ID,
            )?)?,
            solblaze: SplStakePoolStakedex::from_keyed_account(&get_keyed_account(
                accounts,
                &solblaze_stake_pool::ID,
            )?)?,
        })
    }

    pub fn get_accounts_to_update(&self) -> Vec<Pubkey> {
        [
            self.daopool.get_accounts_to_update(),
            self.jito.get_accounts_to_update(),
            self.jpool.get_accounts_to_update(),
            self.laine.get_accounts_to_update(),
            self.solblaze.get_accounts_to_update(),
        ]
        .concat()
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
        let deposit_to_virtual_ix = deposit_to.virtual_ix(&deposit_quote)?;
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
