use std::{
    num::NonZeroU64,
    sync::{atomic::AtomicU64, Arc},
};

use anyhow::{anyhow, Result};
use deposit_cap_guard::{find_spl_deposit_cap_guard_state, DepositCap};
use solana_program::{borsh1::try_from_slice_unchecked, pubkey::Pubkey};
use spl_stake_pool::{
    error::StakePoolError,
    find_deposit_authority_program_address, find_withdraw_authority_program_address,
    state::{StakePool, StakeStatus, ValidatorList},
    MINIMUM_ACTIVE_STAKE,
};
use stakedex_sdk_common::{
    spl_deposit_cap_guard_program, WithdrawStakeQuote, STAKE_ACCOUNT_RENT_EXEMPT_LAMPORTS,
};

mod deposit_cap_guard;
mod stakedex_traits;

/// vsas are required to always have a min of
/// minimum_stake_lamports(meta, stake_program_min_delegation)
/// = meta.rent_exempt_reserve + max(MINIMUM_ACTIVE_STAKE, stake_program_min_delegation)
/// total lamports.
const VSA_MIN_LAMPORTS: u64 = STAKE_ACCOUNT_RENT_EXEMPT_LAMPORTS + MINIMUM_ACTIVE_STAKE;

#[derive(Clone, Copy, Debug, Default)]
pub struct SplStakePoolStakedexInitKeys {
    pub stake_pool_program: Pubkey,
    pub stake_pool_addr: Pubkey,
}

/// A SPL stake pool with possibly custom program ID.
/// Works for different deploys of spl stake pool prog - spl, sanctum spl, sanctum spl multi
#[derive(Debug, Clone, Default)]
pub struct SplStakePoolStakedex {
    pub stake_pool_addr: Pubkey,
    pub stake_pool_program: Pubkey,
    pub stake_pool_label: String,
    pub stake_pool: StakePool,
    pub validator_list: ValidatorList,
    pub curr_epoch: Arc<AtomicU64>,
    pub deposit_authority_program_address: Pubkey,
    pub spl_deposit_cap_guard_program_address: Pubkey,
    pub deposit_cap_state: Option<DepositCap>,
}

impl SplStakePoolStakedex {
    pub fn new_uninitialized(
        SplStakePoolStakedexInitKeys {
            stake_pool_program,
            stake_pool_addr,
        }: SplStakePoolStakedexInitKeys,
        curr_epoch: Arc<AtomicU64>,
    ) -> Self {
        let (deposit_authority_program_address, _bump) =
            find_deposit_authority_program_address(&stake_pool_program, &stake_pool_addr);
        let (spl_deposit_cap_guard_program_address, _bump) =
            find_spl_deposit_cap_guard_state(&spl_deposit_cap_guard_program::ID, &stake_pool_addr);
        Self {
            stake_pool_addr,
            stake_pool_program,
            deposit_authority_program_address,
            spl_deposit_cap_guard_program_address,
            curr_epoch,
            ..Default::default()
        }
    }

    pub fn update_stake_pool(&mut self, data: &[u8]) -> Result<()> {
        self.stake_pool = try_from_slice_unchecked::<StakePool>(data)?;
        Ok(())
    }

    pub fn update_validator_list(&mut self, data: &[u8]) -> Result<()> {
        self.validator_list = try_from_slice_unchecked::<ValidatorList>(data)?;
        Ok(())
    }

    pub fn update_deposit_cap_state(
        &mut self,
        deposit_cap_state_account_data: &[u8],
    ) -> Result<()> {
        const STATE_DEPOSIT_CAP_OFFSET: usize = 0;
        let deposit_cap: &[u8; 9] = deposit_cap_state_account_data
            .get(STATE_DEPOSIT_CAP_OFFSET..STATE_DEPOSIT_CAP_OFFSET + 9)
            .ok_or_else(|| anyhow!("Invalid deposit cap state account data"))?
            .try_into()
            .unwrap();
        self.deposit_cap_state = Some(DepositCap::try_from_buf(deposit_cap)?);
        Ok(())
    }

    pub fn is_updated_this_epoch(&self) -> bool {
        self.stake_pool.last_update_epoch
            >= self.curr_epoch.load(std::sync::atomic::Ordering::Relaxed)
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

        // `active_stake_lamports` is the total lamports of the vsa - includes rent-exempt as well
        let active_stake_lamports = u64::from(validator_list_entry.active_stake_lamports);
        if withdraw_lamports > active_stake_lamports
            || (active_stake_lamports - withdraw_lamports) < VSA_MIN_LAMPORTS
        {
            // actually ProgramError::InsufficientFunds in program, but our err type is StakePoolError here
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

    #[inline]
    pub fn is_sol_deposit_capped(&self) -> bool {
        self.stake_pool.sol_deposit_authority == Some(self.spl_deposit_cap_guard_program_address)
    }

    #[inline]
    pub fn is_stake_deposit_capped(&self) -> bool {
        self.stake_pool.stake_deposit_authority == self.spl_deposit_cap_guard_program_address
    }
}

/// Newtype encapsulating [`SplStakePoolStakedex`] because
/// DepositSol, DepositStake, WithdrawStake does not require fetching reserve stake account
/// for quoting, only WithdrawSol does.
#[derive(Debug, Clone, Default)]
pub struct SplStakePoolStakedexWithWithdrawSol {
    pub inner: SplStakePoolStakedex,
    // NonZero: reserve should always have at least rent-exempt lamports.
    // Initialize with `None`, fetch reserve account to update
    pub reserve_stake_lamports: Option<NonZeroU64>,
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
