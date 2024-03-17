use anyhow::Result;
use solana_program::{borsh0_10::try_from_slice_unchecked, pubkey::Pubkey, stake_history::Epoch};
use spl_stake_pool::{
    error::StakePoolError,
    find_withdraw_authority_program_address,
    state::{StakePool, StakeStatus, ValidatorList},
};
use stakedex_sdk_common::{WithdrawStakeQuote, STAKE_ACCOUNT_RENT_EXEMPT_LAMPORTS};

mod stakedex_traits;
pub use stakedex_traits::*;

/// A SPL stake pool with possibly custom program ID
#[derive(Clone, Default)]
pub struct SplStakePoolStakedex {
    pub stake_pool_addr: Pubkey,
    pub stake_pool_program: Pubkey,
    pub stake_pool_label: String,
    pub stake_pool: StakePool,
    pub validator_list: ValidatorList,
    pub curr_epoch: Epoch,
}

impl SplStakePoolStakedex {
    pub fn update_stake_pool(&mut self, data: &[u8]) -> Result<()> {
        self.stake_pool = try_from_slice_unchecked::<StakePool>(data)?;
        Ok(())
    }

    pub fn update_validator_list(&mut self, data: &[u8]) -> Result<()> {
        self.validator_list = try_from_slice_unchecked::<ValidatorList>(data)?;
        Ok(())
    }

    pub fn is_updated_this_epoch(&self) -> bool {
        self.stake_pool.last_update_epoch >= self.curr_epoch
    }

    /// Computes and returns the stake withdraw authority PDA
    /// Assumes all included pools are permissionless, i.e. using the default withdraw authority
    pub fn withdraw_authority_addr(&self) -> Pubkey {
        find_withdraw_authority_program_address(&self.stake_pool_program, &self.stake_pool_addr).0
    }

    fn get_withdraw_stake_quote_for_validator_copied(
        &self,
        validator_index: usize,
        withdraw_amount: u64,
    ) -> Result<WithdrawStakeQuote, StakePoolError> {
        let validator_list_entry = self
            .validator_list
            .validators
            .get(validator_index)
            .ok_or(StakePoolError::ValidatorNotFound)?;
        // only handle withdrawal from active stake accounts for simplicity.
        // Likely other stake pools can't accept non active stake anyway
        if validator_list_entry.status != StakeStatus::Active.into() {
            return Err(StakePoolError::InvalidState);
        }
        let stake_pool = &self.stake_pool;
        let pool_tokens = withdraw_amount;

        // Copied from:
        // https://github.com/solana-labs/solana-program-library/blob/stake-pool-v1.0.0/stake-pool/program/src/processor.rs#L2297
        let pool_tokens_fee = stake_pool
            .calc_pool_tokens_stake_withdrawal_fee(pool_tokens)
            .ok_or(StakePoolError::CalculationFailure)?;
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
    use stakedex_jup_interface::DepositSolWrapper;

    use crate::*;

    #[test]
    fn test_wrapper_impls_amm_correctly_compile_time() {
        // DepositSolWrapper<SplStakePoolDepositSol>
        // impls Amm
        let _sp = DepositSolWrapper(SplStakePoolStakedex::default());
    }
}
