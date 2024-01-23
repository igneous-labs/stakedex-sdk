use anyhow::Result;
use jupiter_amm_interface::Quote;
use rust_decimal::{
    prelude::{FromPrimitive, Zero},
    Decimal,
};
use solana_program::{instruction::Instruction, pubkey::Pubkey};

use crate::{apply_global_fee, BaseStakePoolAmm, DepositStakeQuoteErr};

use super::withdraw_stake::WithdrawStakeQuote;

#[derive(Clone, Copy, Debug, Default)]
pub struct DepositStakeQuote {
    /// Output tokens, after subtracting fees
    pub tokens_out: u64,

    /// In terms of output tokens
    pub fee_amount: u64,

    /// Active voter of the input stake account
    pub voter: Pubkey,
}

impl DepositStakeQuote {
    pub fn is_zero_out(&self) -> bool {
        self.tokens_out == 0
    }
}

/// Info about the stake account to be deposited
#[derive(Clone, Copy, Debug)]
pub struct DepositStakeInfo {
    pub addr: Pubkey,
}

pub trait DepositStake: BaseStakePoolAmm {
    /// Quotes a deposit stake operation that corresponds to the given withdraw_stake_quote
    ///
    /// This should only include the stake pool's deposit stake fees, not stakedex's global fees
    /// Returns Err if stake pool cannot currently accept stake deposits (e.g. not yet updated for this epoch)
    /// Returns DepositStakeQuote::default() if unable to handle withdraw_stake_quote (e.g. cannot accept provided voter)
    fn get_deposit_stake_quote(
        &self,
        withdraw_stake_quote: WithdrawStakeQuote,
    ) -> Result<DepositStakeQuote, DepositStakeQuoteErr> {
        if !self.can_accept_stake_deposits() {
            return Err(DepositStakeQuoteErr::CannotAcceptStakeDeposits);
        }
        Ok(self.get_deposit_stake_quote_unchecked(withdraw_stake_quote))
    }

    fn can_accept_stake_deposits(&self) -> bool;

    /// Inner impl fn, should not be called directly. Instead, call
    /// get_deposit_stake_quote()
    fn get_deposit_stake_quote_unchecked(
        &self,
        withdraw_stake_quote: WithdrawStakeQuote,
    ) -> DepositStakeQuote;

    fn virtual_ix(
        &self,
        quote: &DepositStakeQuote,
        deposit_stake_info: &DepositStakeInfo,
    ) -> Result<Instruction>;

    fn convert_deposit_stake_quote(&self, in_amount: u64, quote: DepositStakeQuote) -> Quote {
        let aft_global_fees = apply_global_fee(quote.tokens_out);
        let total_fees = quote.fee_amount + aft_global_fees.fee;
        let final_out_amount = aft_global_fees.remainder;
        let before_fees = (final_out_amount + total_fees) as f64;
        // Decimal::from_f64() returns None if infinite or NaN (before_fees = 0)
        let fee_pct =
            Decimal::from_f64((total_fees as f64) / before_fees).unwrap_or_else(Decimal::zero);
        Quote {
            in_amount,
            out_amount: final_out_amount,
            fee_amount: total_fees,
            fee_pct,
            // TODO: fee_mint == staked_sol_mint is true for all stake pools for now
            fee_mint: self.staked_sol_mint(),
            ..Quote::default()
        }
    }

    fn underlying_liquidity(&self) -> Option<&Pubkey> {
        None
    }

    fn accounts_len(&self) -> usize;
}
