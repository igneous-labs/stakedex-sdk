use anyhow::Result;
use solana_program::{
    borsh::try_from_slice_unchecked, native_token, pubkey::Pubkey, stake_history::Epoch,
};
use spl_stake_pool::{
    error::StakePoolError,
    find_stake_program_address, find_withdraw_authority_program_address,
    state::{StakePool, StakeStatus, ValidatorList},
    MINIMUM_ACTIVE_STAKE,
};
use stakedex_sdk_common::{
    eversol_program, eversol_stake_pool, DepositStakeQuote, WithdrawStakeQuote,
    STAKE_ACCOUNT_RENT_EXEMPT_LAMPORTS,
};

mod stakedex_traits;
pub use stakedex_traits::*;

pub const EVERSOL_STAKE_POOL_LABEL: &str = "Eversol";

#[derive(Clone, Default)]
pub struct EversolStakePoolStakedex {
    stake_pool: StakePool,
    validator_list: ValidatorList,
    curr_epoch: Epoch,
}

impl EversolStakePoolStakedex {
    pub fn update_stake_pool(&mut self, data: &[u8]) -> Result<()> {
        self.stake_pool = try_from_slice_unchecked::<StakePool>(data)?;
        Ok(())
    }

    pub fn update_validator_list(&mut self, data: &[u8]) -> Result<()> {
        self.validator_list = try_from_slice_unchecked::<ValidatorList>(data)?;
        Ok(())
    }

    pub fn withdraw_authority() -> Pubkey {
        find_withdraw_authority_program_address(&eversol_program::ID, &eversol_stake_pool::ID).0
    }

    /// Find and return validator stake account
    pub fn vsa(voter: &Pubkey) -> Pubkey {
        find_stake_program_address(&eversol_program::ID, voter, &eversol_stake_pool::ID).0
    }

    pub fn is_updated_this_epoch(&self) -> bool {
        self.stake_pool.last_update_epoch >= self.curr_epoch
    }

    /// Reference (copy-pasta):
    /// https://github.com/everstake/solana-program-library/blob/22534fe3885e698598e92b2fe20da3a8adbfc5ff/stake-pool/program/src/processor.rs#L2309-L2355
    fn get_deposit_stake_quote_copied(
        &self,
        withdraw_stake_quote: WithdrawStakeQuote,
    ) -> Result<DepositStakeQuote, StakePoolError> {
        if let Some(v) = self.stake_pool.preferred_deposit_validator_vote_address {
            if withdraw_stake_quote.voter != v {
                return Err(StakePoolError::InvalidPreferredValidator);
            }
        }
        let validator_list_entry = self
            .validator_list
            .find(&withdraw_stake_quote.voter)
            .ok_or(StakePoolError::ValidatorNotFound)?;
        if validator_list_entry.status != StakeStatus::Active {
            return Err(StakePoolError::InvalidState);
        }
        let total_deposit_lamports = withdraw_stake_quote.lamports_out;
        let stake_deposit_lamports = withdraw_stake_quote.lamports_staked;
        let sol_deposit_lamports = total_deposit_lamports - stake_deposit_lamports;
        let stake_pool = &self.stake_pool;

        let new_pool_tokens = stake_pool
            .convert_amount_of_lamports_to_amount_of_pool_tokens(
                stake_pool
                    .calculate_deposit_amount_by_reward_simulation(total_deposit_lamports)
                    .ok_or(StakePoolError::CalculationFailure)?,
            )
            .ok_or(StakePoolError::CalculationFailure)?;

        let new_pool_tokens_from_deposit_threshold = if stake_pool.no_fee_deposit_threshold > 0
            && native_token::sol_to_lamports(stake_pool.no_fee_deposit_threshold as f64)
                < stake_deposit_lamports
        {
            stake_pool
                .convert_amount_of_lamports_to_amount_of_pool_tokens(native_token::sol_to_lamports(
                    stake_pool.no_fee_deposit_threshold as f64,
                ))
                .ok_or(StakePoolError::CalculationFailure)?
        } else {
            stake_pool
                .convert_amount_of_lamports_to_amount_of_pool_tokens(stake_deposit_lamports)
                .ok_or(StakePoolError::CalculationFailure)?
        };

        let new_pool_tokens_from_sol = stake_pool
            .convert_amount_of_lamports_to_amount_of_pool_tokens(sol_deposit_lamports)
            .ok_or(StakePoolError::CalculationFailure)?;

        let stake_deposit_fee = stake_pool
            .calc_pool_tokens_stake_deposit_fee(new_pool_tokens_from_deposit_threshold)
            .ok_or(StakePoolError::CalculationFailure)?;
        let sol_deposit_fee = stake_pool
            .calc_pool_tokens_sol_deposit_fee(new_pool_tokens_from_sol)
            .ok_or(StakePoolError::CalculationFailure)?;

        let total_fee = stake_deposit_fee
            .checked_add(sol_deposit_fee)
            .ok_or(StakePoolError::CalculationFailure)?;
        let pool_tokens_user = new_pool_tokens
            .checked_sub(total_fee)
            .ok_or(StakePoolError::CalculationFailure)?;

        // eversol doesnt support referrer

        Ok(DepositStakeQuote {
            tokens_out: pool_tokens_user,
            fee_amount: total_fee,
            voter: withdraw_stake_quote.voter,
        })
    }

    /// Reference (copy-pasta):
    /// https://github.com/everstake/solana-program-library/blob/22534fe3885e698598e92b2fe20da3a8adbfc5ff/stake-pool/program/src/processor.rs#L2476-L2489
    fn get_withdraw_stake_quote_for_validator_copied(
        &self,
        validator_index: usize,
        pool_tokens: u64,
    ) -> Result<WithdrawStakeQuote, StakePoolError> {
        let validator_list_entry = self.validator_list.validators.get(validator_index).unwrap();
        // only handle withdrawal from active stake accounts for simplicity.
        // Likely other stake pools can't accept non active stake anyway
        if validator_list_entry.status != StakeStatus::Active {
            return Err(StakePoolError::InvalidState);
        }

        let stake_pool = &self.stake_pool;

        let pool_tokens_fee = stake_pool
            .calc_pool_tokens_stake_withdrawal_fee(pool_tokens)
            .ok_or(StakePoolError::CalculationFailure)?;
        let pool_tokens_burnt = pool_tokens
            .checked_sub(pool_tokens_fee)
            .ok_or(StakePoolError::CalculationFailure)?;

        let withdraw_lamports = stake_pool
            .convert_amount_of_pool_tokens_to_amount_of_lamports(pool_tokens_burnt)
            .ok_or(StakePoolError::CalculationFailure)?;

        if withdraw_lamports
            > validator_list_entry
                .active_stake_lamports
                .saturating_sub(MINIMUM_ACTIVE_STAKE)
        {
            return Err(StakePoolError::StakeLamportsNotEqualToMinimum);
        }

        let lamports_staked = withdraw_lamports
            .checked_sub(STAKE_ACCOUNT_RENT_EXEMPT_LAMPORTS)
            .ok_or(StakePoolError::CalculationFailure)?;

        Ok(WithdrawStakeQuote {
            lamports_out: withdraw_lamports,
            lamports_staked,
            fee_amount: pool_tokens_fee,
            voter: validator_list_entry.vote_account_address,
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::*;
    use stakedex_sdk_common::DepositSolWrapper;

    #[test]
    fn test_wrapper_impls_amm_correctly_compile_time() {
        // DepositSolWrapper<SplStakePoolDepositSol>
        // impls Amm
        let _sp = DepositSolWrapper(EversolStakePoolStakedex::default());
        // sp.clone_amm();
    }
}
