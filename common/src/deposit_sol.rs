use std::collections::HashMap;

use anyhow::{anyhow, Result};
use jupiter_core::amm::{Amm, Quote, QuoteParams, SwapLegAndAccountMetas, SwapParams};
use rust_decimal::{
    prelude::{FromPrimitive, Zero},
    Decimal,
};
use solana_program::{
    instruction::{AccountMeta, Instruction},
    pubkey::Pubkey,
    system_program,
};
use spl_token::native_mint;
use stakedex_interface::{StakeWrappedSolKeys, STAKE_WRAPPED_SOL_IX_ACCOUNTS_LEN};

use crate::{
    fees::apply_global_fee,
    pda::{
        cws_wsol_bridge_in, find_deposit_stake_amm_key, find_fee_token_acc, find_sol_bridge_out,
    },
};

#[derive(Copy, Clone, Debug)]
pub struct DepositSolQuote {
    pub out_amount: u64,
    pub fee_amount: u64,
}

pub trait DepositSol {
    fn stake_pool_label(&self) -> &'static str;

    fn main_state_key(&self) -> Pubkey;

    /// This should only include the stake pool's fees, not stakedex's global fees
    fn get_deposit_sol_quote(&self, lamports: u64) -> Result<DepositSolQuote>;

    fn staked_sol_mint(&self) -> Pubkey;

    fn get_accounts_to_update(&self) -> Vec<Pubkey>;

    fn update(&mut self, accounts_map: &HashMap<Pubkey, Vec<u8>>) -> Result<()>;

    fn virtual_ix(&self) -> Result<Instruction>;

    fn convert_quote(&self, deposit_sol_quote: DepositSolQuote) -> Quote {
        let aft_global_fees = apply_global_fee(deposit_sol_quote.out_amount);
        let total_fees = deposit_sol_quote.fee_amount + aft_global_fees.fee;
        let final_out_amount = aft_global_fees.remainder;
        let before_fees = (final_out_amount + total_fees) as f64;
        // Decimal::from_f64() returns None if infinite / NaN (before_fees = 0)
        let fee_pct =
            Decimal::from_f64((total_fees as f64) / before_fees).unwrap_or_else(Decimal::zero);
        Quote {
            out_amount: final_out_amount,
            fee_amount: total_fees,
            fee_pct,
            // TODO: fee_mint == staked_sol_mint is true for all stake pools for now
            fee_mint: self.staked_sol_mint(),
            ..Quote::default()
        }
    }
}

// newtype pattern in order to impl external trait on internal generic
#[derive(Copy, Clone)]
pub struct DepositSolWrapper<T: DepositSol + Clone + Send + Sync + 'static>(pub T);

impl<T> Amm for DepositSolWrapper<T>
where
    T: DepositSol + Clone + Send + Sync,
{
    fn label(&self) -> String {
        format!("{} (StakeDex)", self.0.stake_pool_label())
    }

    // To avoid key clashes with existing stake pools on jup (Marinade),
    // we can use a PDA like this
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
        if quote_params.input_mint != native_mint::ID
            || quote_params.output_mint != self.0.staked_sol_mint()
        {
            return Err(anyhow!(
                "Cannot handle {} -> {}",
                quote_params.input_mint,
                quote_params.output_mint
            ));
        }
        let deposit_sol_quote = self.0.get_deposit_sol_quote(quote_params.in_amount)?;
        let quote = self.0.convert_quote(deposit_sol_quote);
        Ok(quote)
    }

    fn get_swap_leg_and_account_metas(
        &self,
        swap_params: &SwapParams,
    ) -> Result<SwapLegAndAccountMetas> {
        let (sol_bridge_out, _) = find_sol_bridge_out();
        let mut metas = Vec::from(<[AccountMeta; STAKE_WRAPPED_SOL_IX_ACCOUNTS_LEN]>::from(
            &StakeWrappedSolKeys {
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
        ));
        let deposit_sol_virtual_ix = self.0.virtual_ix()?;
        metas.extend(deposit_sol_virtual_ix.accounts);
        // TODO: jupiter overrides
        Err(anyhow!("UNIMPLEMENTED"))
    }

    fn clone_amm(&self) -> Box<dyn Amm + Send + Sync> {
        Box::new(self.clone())
    }
}
