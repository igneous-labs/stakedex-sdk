use std::collections::HashMap;

use anyhow::{anyhow, Result};
use jupiter_core::amm::{Amm, Quote, QuoteParams, SwapLegAndAccountMetas, SwapParams};
use rust_decimal::Decimal;
use solana_program::{
    instruction::{AccountMeta, Instruction},
    pubkey::Pubkey,
    system_program,
};
use spl_token::native_mint;
use stakedex_interface::StakeWrappedSolKeys;

use crate::pda::{
    cws_wsol_bridge_in, find_deposit_stake_amm_key, find_fee_token_acc, find_sol_bridge_out,
};

#[derive(Copy, Clone, Debug)]
pub struct DepositSolQuote {
    pub out_amount: u64,
    pub fee_amount: u64,
    pub fee_pct: Decimal,
}

impl From<DepositSolQuote> for Quote {
    fn from(deposit_sol_quote: DepositSolQuote) -> Self {
        Quote {
            out_amount: deposit_sol_quote.out_amount,
            fee_amount: deposit_sol_quote.fee_amount,
            fee_pct: deposit_sol_quote.fee_pct,
            ..Quote::default()
        }
    }
}

pub trait DepositSol {
    fn label(&self) -> String;

    fn main_state_key(&self) -> Pubkey;

    fn get_deposit_sol_quote(&self, lamports: u64) -> DepositSolQuote;

    fn staked_sol_mint(&self) -> Pubkey;

    fn get_accounts_to_update(&self) -> Vec<Pubkey>;

    fn update(&self, accounts_map: &HashMap<Pubkey, Vec<u8>>) -> Result<()>;

    fn virtual_ix(&self) -> Instruction;
}

// newtype pattern in order to impl external trait on internal generic
#[derive(Copy, Clone)]
pub struct DepositSolWrapper<T: DepositSol + Clone + Send + Sync + 'static>(T);

impl<T> Amm for DepositSolWrapper<T>
where
    T: DepositSol + Clone + Send + Sync,
{
    fn label(&self) -> String {
        self.0.label()
    }

    // To avoid key clashes with existing stake pools on jup (Marinade),
    // we can use a PDA
    fn key(&self) -> Pubkey {
        find_deposit_stake_amm_key(&self.0.main_state_key()).0
    }

    fn get_reserve_mints(&self) -> Vec<Pubkey> {
        Vec::from([native_mint::ID, self.0.staked_sol_mint()])
    }

    fn get_accounts_to_update(&self) -> Vec<Pubkey> {
        self.0.get_accounts_to_update()
    }

    fn update(&mut self, accounts_map: &HashMap<Pubkey, Vec<u8>>) -> Result<()> {
        self.0.update(accounts_map)
    }

    fn quote(&self, quote_params: &QuoteParams) -> Result<Quote> {
        // TODO: not sure how yall wanna handle one-way swaps. Return Err or just 0 out quote (Quote::default())?
        if quote_params.input_mint != native_mint::ID
            || quote_params.output_mint != self.0.staked_sol_mint()
        {
            return Ok(Quote::default());
        }
        Ok(self.0.get_deposit_sol_quote(quote_params.in_amount).into())
    }

    fn get_swap_leg_and_account_metas(
        &self,
        swap_params: &SwapParams,
    ) -> Result<SwapLegAndAccountMetas> {
        let sol_bridge_out = find_sol_bridge_out().0;
        let mut metas = Vec::from(<[AccountMeta; 10]>::from(&StakeWrappedSolKeys {
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
        }));
        metas.extend(self.0.virtual_ix().accounts);
        // TODO: jupiter overrides
        Err(anyhow!("UNIMPLEMENTED"))
    }

    fn clone_amm(&self) -> Box<dyn Amm + Send + Sync> {
        Box::new(self.clone())
    }
}
