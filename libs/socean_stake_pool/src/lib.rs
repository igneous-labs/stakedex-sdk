use anyhow::Result;
use solana_program::{borsh0_10::try_from_slice_unchecked, pubkey::Pubkey, stake_history::Epoch};
use spl_stake_pool::{
    error::StakePoolError,
    find_stake_program_address, find_withdraw_authority_program_address,
    state::{StakePool, StakeStatus, ValidatorList},
};
use stakedex_sdk_common::{
    socean_program, socean_stake_pool, WithdrawStakeQuote, STAKE_ACCOUNT_RENT_EXEMPT_LAMPORTS,
};

mod stakedex_traits;
pub use stakedex_traits::*;

pub const SOCEAN_STAKE_POOL_LABEL: &str = "Socean";

#[derive(Clone, Default)]
pub struct SoceanStakePoolStakedex {
    stake_pool: StakePool,
    validator_list: ValidatorList,
    curr_epoch: Epoch,
}

impl SoceanStakePoolStakedex {
    pub fn update_stake_pool(&mut self, data: &[u8]) -> Result<()> {
        self.stake_pool = try_from_slice_unchecked::<StakePool>(data)?;
        Ok(())
    }

    pub fn update_validator_list(&mut self, data: &[u8]) -> Result<()> {
        self.validator_list = try_from_slice_unchecked::<ValidatorList>(data)?;
        Ok(())
    }

    pub fn withdraw_authority() -> Pubkey {
        find_withdraw_authority_program_address(&socean_program::ID, &socean_stake_pool::ID).0
    }

    /// Find and return validator stake account
    pub fn vsa(voter: &Pubkey) -> Pubkey {
        find_stake_program_address(&socean_program::ID, voter, &socean_stake_pool::ID, None).0
    }

    pub fn is_updated_this_epoch(&self) -> bool {
        self.stake_pool.last_update_epoch >= self.curr_epoch
    }

    fn get_withdraw_stake_quote_for_validator_copied(
        &self,
        validator_index: usize,
        withdraw_amount: u64,
    ) -> Result<WithdrawStakeQuote, StakePoolError> {
        let validator_list_entry = self.validator_list.validators.get(validator_index).unwrap();
        // only handle withdrawal from active stake accounts for simplicity.
        // Likely other stake pools can't accept non active stake anyway
        if validator_list_entry.status != StakeStatus::Active.into() {
            return Err(StakePoolError::InvalidState);
        }
        let stake_pool = &self.stake_pool;
        let pool_tokens = withdraw_amount;

        // Copied from:
        // https://github.com/igneous-labs/solana-program-library/blob/9a09813dc47df286c448629015e585e5d2beb3d9/stake-pool/program/src/processor.rs#L2257-L2276
        let pool_tokens_fee = u64::try_from(
            stake_pool
                .stake_withdrawal_fee
                .apply(pool_tokens)
                .ok_or(StakePoolError::CalculationFailure)?,
        )
        .map_err(|_| StakePoolError::CalculationFailure)?;
        let pool_tokens_burnt = pool_tokens
            .checked_sub(pool_tokens_fee)
            .ok_or(StakePoolError::CalculationFailure)?;

        let withdraw_lamports = stake_pool
            .calc_lamports_withdraw_amount(pool_tokens_burnt)
            .ok_or(StakePoolError::CalculationFailure)?;
        if withdraw_lamports == 0 {
            return Err(StakePoolError::WithdrawalTooSmall);
        }
        // end copy

        // according to https://github.com/solana-labs/solana-program-library/blob/58c1226a513d3d8bb2de8ec67586a679be7fd2d4/stake-pool/program/src/state.rs#L536C1-L542
        // `active_stake_lamports` = delegation.stake - MIN_ACTIVE_STAKE_LAMPORTS.
        // Withdrawals must leave at least MIN_ACTIVE_STAKE_LAMPORTS active stake in vsa
        if withdraw_lamports > u64::from(validator_list_entry.active_stake_lamports) {
            return Err(StakePoolError::InvalidState);
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
        let _sp = DepositSolWrapper(SoceanStakePoolStakedex::default());
    }
}
