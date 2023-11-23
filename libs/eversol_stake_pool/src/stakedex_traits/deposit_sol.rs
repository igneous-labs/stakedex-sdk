use anyhow::Result;
use solana_program::{instruction::Instruction, native_token};
use spl_stake_pool::error::StakePoolError;
use stakedex_deposit_sol_interface::{
    eversol_stake_pool_deposit_sol_ix, EversolStakePoolDepositSolIxArgs,
    EversolStakePoolDepositSolKeys,
};
use stakedex_sdk_common::{eversol_program, eversol_stake_pool, DepositSol, DepositSolQuote};

use crate::EversolStakePoolStakedex;

impl DepositSol for EversolStakePoolStakedex {
    fn can_accept_sol_deposits(&self) -> bool {
        self.is_updated_this_epoch()
    }

    /// Reference (copy-pasta):
    /// https://github.com/everstake/solana-program-library/blob/22534fe3885e698598e92b2fe20da3a8adbfc5ff/stake-pool/program/src/processor.rs#L2309-L2355
    fn get_deposit_sol_quote_unchecked(&self, deposit_lamports: u64) -> Result<DepositSolQuote> {
        let new_pool_tokens_wo_idle_fee = self
            .stake_pool
            .convert_amount_of_lamports_to_amount_of_pool_tokens(deposit_lamports)
            .ok_or(StakePoolError::CalculationFailure)?;

        let new_pool_tokens_from_deposit_threshold = if self.stake_pool.no_fee_deposit_threshold > 0
            && native_token::sol_to_lamports(self.stake_pool.no_fee_deposit_threshold as f64)
                < deposit_lamports
        {
            self.stake_pool
                .convert_amount_of_lamports_to_amount_of_pool_tokens(native_token::sol_to_lamports(
                    self.stake_pool.no_fee_deposit_threshold as f64,
                ))
                .ok_or(StakePoolError::CalculationFailure)?
        } else {
            new_pool_tokens_wo_idle_fee
        };

        let new_pool_tokens = self
            .stake_pool
            .calculate_deposit_amount_by_reward_simulation(new_pool_tokens_wo_idle_fee)
            .ok_or(StakePoolError::CalculationFailure)?;

        let pool_tokens_sol_deposit_fee = self
            .stake_pool
            .calc_pool_tokens_sol_deposit_fee(new_pool_tokens_from_deposit_threshold)
            .ok_or(StakePoolError::CalculationFailure)?;
        let pool_tokens_user = new_pool_tokens
            .checked_sub(pool_tokens_sol_deposit_fee)
            .ok_or(StakePoolError::CalculationFailure)?;

        // eversol doesnt support referrer

        Ok(DepositSolQuote {
            in_amount: deposit_lamports,
            out_amount: pool_tokens_user,
            fee_amount: pool_tokens_sol_deposit_fee,
        })
    }

    fn virtual_ix(&self) -> Result<Instruction> {
        Ok(eversol_stake_pool_deposit_sol_ix(
            EversolStakePoolDepositSolKeys {
                eversol_stake_pool_program: eversol_program::ID,
                stake_pool: eversol_stake_pool::ID,
                stake_pool_withdraw_authority: Self::withdraw_authority(),
                stake_pool_manager_fee: self.stake_pool.manager_fee_account,
                stake_pool_reserve_stake: self.stake_pool.reserve_stake,
            },
            EversolStakePoolDepositSolIxArgs {},
        )?)
    }
}
