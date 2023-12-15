use anyhow::Result;
use solana_program::{instruction::Instruction, sysvar};
use spl_stake_pool::error::StakePoolError;
use stakedex_deposit_sol_interface::{
    socean_stake_pool_deposit_sol_ix, SoceanStakePoolDepositSolIxArgs,
    SoceanStakePoolDepositSolKeys,
};
use stakedex_sdk_common::{socean_program, socean_stake_pool, DepositSol, DepositSolQuote};

use crate::SoceanStakePoolStakedex;

impl DepositSol for SoceanStakePoolStakedex {
    fn can_accept_sol_deposits(&self) -> bool {
        self.is_updated_this_epoch()
    }

    fn get_deposit_sol_quote_unchecked(&self, lamports: u64) -> Result<DepositSolQuote> {
        // Reference: https://github.com/solana-labs/solana-program-library/blob/56cdef9ee82877622a074aa74560742264f20591/stake-pool/program/src/processor.rs#L2268
        let new_pool_tokens = self
            .stake_pool
            .calc_pool_tokens_for_deposit(lamports)
            .ok_or(StakePoolError::CalculationFailure)?;
        let pool_tokens_sol_deposit_fee = self
            .stake_pool
            .calc_pool_tokens_sol_deposit_fee(new_pool_tokens)
            .ok_or(StakePoolError::CalculationFailure)?;
        let pool_tokens_user = new_pool_tokens
            .checked_sub(pool_tokens_sol_deposit_fee)
            .ok_or(StakePoolError::CalculationFailure)?;
        let pool_tokens_referral_fee = self
            .stake_pool
            .calc_pool_tokens_sol_referral_fee(pool_tokens_sol_deposit_fee)
            .ok_or(StakePoolError::CalculationFailure)?;
        // since we set referrer to the receiving fee_token_acc, referral fee is effectively kicked back to user
        let out_amount = pool_tokens_user
            .checked_add(pool_tokens_referral_fee)
            .ok_or(StakePoolError::CalculationFailure)?;
        let fee_amount = pool_tokens_sol_deposit_fee
            .checked_sub(pool_tokens_referral_fee)
            .ok_or(StakePoolError::CalculationFailure)?;
        Ok(DepositSolQuote {
            in_amount: lamports,
            out_amount,
            fee_amount,
        })
    }

    fn virtual_ix(&self) -> Result<Instruction> {
        Ok(socean_stake_pool_deposit_sol_ix(
            SoceanStakePoolDepositSolKeys {
                socean_stake_pool_program: socean_program::ID,
                clock: sysvar::clock::ID,
                stake_pool: socean_stake_pool::ID,
                stake_pool_withdraw_authority: Self::withdraw_authority(),
                stake_pool_manager_fee: self.stake_pool.manager_fee_account,
                stake_pool_reserve_stake: self.stake_pool.reserve_stake,
            },
            SoceanStakePoolDepositSolIxArgs {},
        )?)
    }
}
