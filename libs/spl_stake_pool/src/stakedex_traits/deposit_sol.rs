use anyhow::{anyhow, Result};
use solana_program::instruction::Instruction;
use spl_stake_pool::error::StakePoolError;
use stakedex_deposit_sol_interface::{
    spl_stake_pool_deposit_sol_ix, SplStakePoolDepositSolKeys,
    SPL_STAKE_POOL_DEPOSIT_SOL_IX_ACCOUNTS_LEN,
};
use stakedex_sdk_common::{DepositSol, DepositSolQuote};

use crate::{
    deposit_cap_guard::{to_deposit_cap_guard_ix, DepositCap},
    SplStakePoolStakedex,
};

impl DepositSol for SplStakePoolStakedex {
    fn can_accept_sol_deposits(&self) -> bool {
        if self.stake_pool.sol_deposit_authority.is_some() {
            return false;
        }
        self.is_updated_this_epoch()
    }

    fn get_deposit_sol_quote_unchecked(&self, lamports: u64) -> Result<DepositSolQuote> {
        // Reference: https://github.com/solana-labs/solana-program-library/blob/56cdef9ee82877622a074aa74560742264f20591/stake-pool/program/src/processor.rs#L2268
        let new_pool_tokens = self
            .stake_pool
            .calc_pool_tokens_for_deposit(lamports)
            .ok_or(StakePoolError::CalculationFailure)?;
        if self.is_sol_deposit_capped() {
            let deposit_cap = self
                .deposit_cap_state
                .as_ref()
                .ok_or_else(|| anyhow!("deposit cap state not yet fetched"))?;
            let will_exceed_deposit_cap = match deposit_cap {
                DepositCap::Lamports(max_lamports) => {
                    let new_pool_lamports = self.stake_pool.total_lamports.saturating_add(lamports);
                    new_pool_lamports > *max_lamports
                }
                DepositCap::LstAtomics(max_lst_atomics) => {
                    let new_lst_atomics = self
                        .stake_pool
                        .pool_token_supply
                        .saturating_add(new_pool_tokens);
                    new_lst_atomics > *max_lst_atomics
                }
            };
            if will_exceed_deposit_cap {
                return Err(anyhow!("deposit will exceed cap"));
            }
        }
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
        // spl_stake_pool_deposit_sol_ix works for all spl-stake-pool like
        // (spl, sanctum-spl, sanctum-spl-multi) because the accounts interface is the exact same
        let ix = spl_stake_pool_deposit_sol_ix(SplStakePoolDepositSolKeys {
            spl_stake_pool_program: self.stake_pool_program,
            stake_pool: self.stake_pool_addr,
            stake_pool_withdraw_authority: self.withdraw_authority_addr(),
            stake_pool_manager_fee: self.stake_pool.manager_fee_account,
            stake_pool_reserve_stake: self.stake_pool.reserve_stake,
        })?;
        Ok(if self.is_sol_deposit_capped() {
            to_deposit_cap_guard_ix(ix, self.spl_deposit_cap_guard_program_address)
        } else {
            ix
        })
    }

    #[inline]
    fn accounts_len(&self) -> usize {
        SPL_STAKE_POOL_DEPOSIT_SOL_IX_ACCOUNTS_LEN
    }
}
