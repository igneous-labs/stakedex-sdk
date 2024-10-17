use anyhow::Result;
use solana_program::{instruction::Instruction, pubkey::Pubkey, stake, sysvar};
use spl_stake_pool::{error::StakePoolError, state::StakePool, MINIMUM_RESERVE_LAMPORTS};
use stakedex_sdk_common::{WithdrawSol, WithdrawSolQuote, STAKE_ACCOUNT_RENT_EXEMPT_LAMPORTS};
use stakedex_withdraw_sol_interface::{
    spl_stake_pool_withdraw_sol_ix, SplStakePoolWithdrawSolKeys,
    SPL_STAKE_POOL_WITHDRAW_SOL_IX_ACCOUNTS_LEN,
};

use crate::SplStakePoolStakedexWithWithdrawSol;

// Adapted from: https://github.com/solana-labs/solana-program-library/blob/17a228bb8e36737209ca5d5375415c70da37c311/stake-pool/program/src/lib.rs#L80-L84
// Will have to change if network changes rent-exempt parameters
const TOTAL_MIN_RESERVE_LAMPORTS: u64 =
    STAKE_ACCOUNT_RENT_EXEMPT_LAMPORTS + MINIMUM_RESERVE_LAMPORTS;

impl WithdrawSol for SplStakePoolStakedexWithWithdrawSol {
    fn get_withdraw_sol_quote(&self, lst: u64) -> Result<WithdrawSolQuote> {
        // Interface does not work for pools with permissioned SOL withdrawals
        if self.inner.stake_pool.sol_withdraw_authority.is_some() {
            return Err(StakePoolError::InvalidSolWithdrawAuthority.into());
        }
        if !self.inner.is_updated_this_epoch() {
            return Err(StakePoolError::StakeListAndPoolOutOfDate.into());
        }
        let quote = get_withdraw_sol_quote_copied(&self.inner.stake_pool, lst)?;
        let curr_reserve_lamports: u64 = self
            .reserve_stake_lamports
            .ok_or(StakePoolError::WrongStakeStake)?
            .into();
        // Adapted from:
        // https://github.com/solana-labs/solana-program-library/blob/17a228bb8e36737209ca5d5375415c70da37c311/stake-pool/program/src/processor.rs#L3102-L3116
        let new_reserve_lamports = curr_reserve_lamports.saturating_sub(quote.out_amount);
        if new_reserve_lamports < TOTAL_MIN_RESERVE_LAMPORTS {
            return Err(StakePoolError::SolWithdrawalTooLarge.into());
        }
        Ok(quote)
    }

    #[inline]
    fn virtual_ix(&self) -> Result<Instruction> {
        // spl_stake_pool_withdraw_sol_ix works for all spl-stake-pool like
        // (spl, sanctum-spl, sanctum-spl-multi) because the accounts interface is the exact same
        Ok(spl_stake_pool_withdraw_sol_ix(
            SplStakePoolWithdrawSolKeys {
                spl_stake_pool_program: self.inner.stake_pool_program,
                withdraw_sol_spl_stake_pool: self.inner.stake_pool_addr,
                withdraw_sol_withdraw_authority: self.inner.withdraw_authority_addr(),
                withdraw_sol_reserve_stake: self.inner.stake_pool.reserve_stake,
                withdraw_sol_manager_fee: self.inner.stake_pool.manager_fee_account,
                clock: sysvar::clock::ID,
                stake_history: sysvar::stake_history::ID,
                stake_program: stake::program::ID,
                withdraw_sol_token_program: self.inner.stake_pool.token_program_id,
            },
        )?)
    }

    #[inline]
    fn accounts_len(&self) -> usize {
        SPL_STAKE_POOL_WITHDRAW_SOL_IX_ACCOUNTS_LEN
    }

    fn underlying_liquidity(&self) -> Option<&Pubkey> {
        Some(&self.inner.stake_pool.reserve_stake)
    }
}

// Assumes
// - manager fee account is a valid token account (fees will be 0 otherwise)
fn get_withdraw_sol_quote_copied(sp: &StakePool, pool_tokens: u64) -> Result<WithdrawSolQuote> {
    // Copied from
    // https://github.com/solana-labs/solana-program-library/blob/17a228bb8e36737209ca5d5375415c70da37c311/stake-pool/program/src/processor.rs#L3066-L3094
    let pool_tokens_fee = sp
        .calc_pool_tokens_sol_withdrawal_fee(pool_tokens)
        .ok_or(StakePoolError::CalculationFailure)?;
    let pool_tokens_burnt = pool_tokens
        .checked_sub(pool_tokens_fee)
        .ok_or(StakePoolError::CalculationFailure)?;
    let withdraw_lamports = sp
        .calc_lamports_withdraw_amount(pool_tokens_burnt)
        .ok_or(StakePoolError::CalculationFailure)?;
    if withdraw_lamports == 0 {
        return Err(StakePoolError::WithdrawalTooSmall.into());
    }

    // estimate pool_tokens_fee in terms of SOL instead of LST
    let est_lamports_fee = sp
        // calc_lamports_withdraw_amount() is just pool_tokens * pool_lamports / pool_mint_supply
        .calc_lamports_withdraw_amount(pool_tokens_fee)
        .ok_or(StakePoolError::CalculationFailure)?;
    Ok(WithdrawSolQuote {
        in_amount: pool_tokens,
        out_amount: withdraw_lamports,
        fee_amount: est_lamports_fee,
    })
}
