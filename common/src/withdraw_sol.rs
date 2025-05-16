use anyhow::Result;
use jupiter_amm_interface::Quote;
use rust_decimal::{
    prelude::{FromPrimitive, Zero},
    Decimal,
};
use solana_program::{instruction::Instruction, pubkey::Pubkey};

use crate::{apply_withdraw_wrapped_sol_stakedex_fee, wsol, BaseStakePoolAmm};

#[derive(Copy, Clone, Debug)]
pub struct WithdrawSolQuote {
    pub in_amount: u64,

    /// After subtracting withdraw fees
    pub out_amount: u64,

    /// Withdrawal fees, in SOL
    pub fee_amount: u64,
}

pub trait WithdrawSol: BaseStakePoolAmm {
    /// This should only include the stake pool's fees, not stakedex's global fees
    fn get_withdraw_sol_quote(&self, lst: u64) -> Result<WithdrawSolQuote>;

    fn virtual_ix(&self) -> Result<Instruction>;

    fn accounts_len(&self) -> usize;

    fn convert_quote(&self, withdraw_sol_quote: WithdrawSolQuote) -> Quote {
        let aft_global_fees =
            apply_withdraw_wrapped_sol_stakedex_fee(withdraw_sol_quote.out_amount);
        let total_fees = withdraw_sol_quote.fee_amount + aft_global_fees.fee;
        let final_out_amount = aft_global_fees.remainder;
        let before_fees = (final_out_amount + total_fees) as f64;
        // Decimal::from_f64() returns None if infinite or NaN (before_fees = 0)
        let fee_pct =
            Decimal::from_f64((total_fees as f64) / before_fees).unwrap_or_else(Decimal::zero);
        Quote {
            in_amount: withdraw_sol_quote.in_amount,
            out_amount: final_out_amount,
            fee_amount: total_fees,
            fee_pct,
            // since stakedex program levies fee on output mint,
            // we count all fees in terms of output mint (wsol) to be consistent
            fee_mint: wsol::ID,
            ..Quote::default()
        }
    }

    fn underlying_liquidity(&self) -> Option<&Pubkey> {
        None
    }
}
