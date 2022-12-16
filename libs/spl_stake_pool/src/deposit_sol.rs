//! Account data fields and other data required to quote stake wrapped SOL

use anyhow::{anyhow, Result};
use borsh::BorshDeserialize;
use jupiter_core::amm::KeyedAccount;
use solana_program::{instruction::Instruction, pubkey::Pubkey};
use spl_stake_pool::{
    error::StakePoolError, find_withdraw_authority_program_address, state::StakePool,
};
use stakedex_deposit_sol_interface::{
    spl_stake_pool_deposit_sol_ix, SplStakePoolDepositSolIxArgs, SplStakePoolDepositSolKeys,
};
use stakedex_sdk_common::{BaseStakePoolAmm, DepositSol, DepositSolQuote};

use crate::SPL_STAKE_POOL_STATE_TO_LABEL;

/// Note: we do not currently handle stake pools that have
/// gated deposits (SOL deposits must be signed with sol_deposit_authority)
#[derive(Clone, Default)]
pub struct SplStakePoolDepositSol {
    stake_pool_addr: Pubkey,
    withdraw_authority_addr: Pubkey,
    stake_pool_label: &'static str,
    stake_pool: StakePool,
}

impl SplStakePoolDepositSol {
    /// Initialize from stake pool main account
    pub fn from_keyed_account(keyed_account: &KeyedAccount) -> Result<Self> {
        let mut res = Self::default();
        res.stake_pool_addr = keyed_account.key;
        res.withdraw_authority_addr =
            find_withdraw_authority_program_address(&spl_stake_pool::ID, &res.stake_pool_addr).0;
        res.stake_pool_label = SPL_STAKE_POOL_STATE_TO_LABEL
            .get(&res.stake_pool_addr)
            .ok_or_else(|| anyhow!("Unknown spl stake pool: {}", res.stake_pool_addr))?;
        res.update_fields(&keyed_account.account.data)?;
        Ok(res)
    }

    pub fn update_fields(&mut self, state_account_data: &[u8]) -> Result<()> {
        self.stake_pool = StakePool::try_from_slice(state_account_data)?;
        Ok(())
    }
}

impl BaseStakePoolAmm for SplStakePoolDepositSol {
    fn stake_pool_label(&self) -> &'static str {
        self.stake_pool_label
    }

    fn main_state_key(&self) -> Pubkey {
        self.stake_pool_addr
    }

    fn staked_sol_mint(&self) -> Pubkey {
        self.stake_pool.pool_mint
    }

    fn get_accounts_to_update(&self) -> Vec<Pubkey> {
        Vec::from([self.stake_pool_addr])
    }

    fn update(&mut self, accounts_map: &std::collections::HashMap<Pubkey, Vec<u8>>) -> Result<()> {
        let stake_pool_data = accounts_map.get(&self.stake_pool_addr).unwrap();
        self.update_fields(stake_pool_data)
    }
}

impl DepositSol for SplStakePoolDepositSol {
    fn get_deposit_sol_quote(&self, lamports: u64) -> Result<DepositSolQuote> {
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
        Ok(DepositSolQuote {
            in_amount: lamports,
            out_amount: pool_tokens_user,
            fee_amount: pool_tokens_sol_deposit_fee,
        })
    }

    fn virtual_ix(&self) -> Result<Instruction> {
        Ok(spl_stake_pool_deposit_sol_ix(
            SplStakePoolDepositSolKeys {
                spl_stake_pool_program: spl_stake_pool::ID,
                stake_pool: self.stake_pool_addr,
                stake_pool_withdraw_authority: self.withdraw_authority_addr,
                stake_pool_manager_fee: self.stake_pool.manager_fee_account,
                stake_pool_reserve_stake: self.stake_pool.reserve_stake,
            },
            SplStakePoolDepositSolIxArgs {},
        )?)
    }
}
