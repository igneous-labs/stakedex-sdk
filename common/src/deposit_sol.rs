use anyhow::Result;
use jupiter_amm_interface::Quote;
use rust_decimal::{
    prelude::{FromPrimitive, Zero},
    Decimal,
};
use solana_program::instruction::Instruction;

use crate::{BaseStakePoolAmm, DepositSolQuoteError};

#[derive(Copy, Clone, Debug)]
pub struct DepositSolQuote {
    pub in_amount: u64,

    /// After subtracting deposit fees
    pub out_amount: u64,

    /// Deposit fees, in staked_sol_mint
    pub fee_amount: u64,
}

pub trait DepositSol: BaseStakePoolAmm {
    fn can_accept_sol_deposits(&self) -> bool;

    /// This should only include the stake pool's fees, not stakedex's global fees
    fn get_deposit_sol_quote_unchecked(&self, lamports: u64) -> Result<DepositSolQuote>;

    fn virtual_ix(&self) -> Result<Instruction>;

    fn get_deposit_sol_quote(&self, lamports: u64) -> Result<DepositSolQuote> {
        if !self.can_accept_sol_deposits() {
            return Err(DepositSolQuoteError::CannotAcceptSolDeposits.into());
        }
        self.get_deposit_sol_quote_unchecked(lamports)
    }

    fn convert_quote(&self, deposit_sol_quote: DepositSolQuote) -> Quote {
        // no stakedex fees for StakeWrappedSol
        let total_fees = deposit_sol_quote.fee_amount;
        let final_out_amount = deposit_sol_quote.out_amount;
        let before_fees = (final_out_amount + total_fees) as f64;
        // Decimal::from_f64() returns None if infinite or NaN (before_fees = 0)
        let fee_pct =
            Decimal::from_f64((total_fees as f64) / before_fees).unwrap_or_else(Decimal::zero);
        Quote {
            in_amount: deposit_sol_quote.in_amount,
            out_amount: final_out_amount,
            fee_amount: total_fees,
            fee_pct,
            // TODO: fee_mint == staked_sol_mint is true for all stake pools for now
            fee_mint: self.staked_sol_mint(),
            ..Quote::default()
        }
    }

    fn accounts_len(&self) -> usize;
}
