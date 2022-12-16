use anyhow::{anyhow, Result};
use borsh::BorshDeserialize;
use jupiter_core::amm::KeyedAccount;
use solana_program::{instruction::Instruction, pubkey::Pubkey, stake, sysvar};
use spl_stake_pool::{
    find_stake_program_address, find_withdraw_authority_program_address,
    state::{StakePool, StakeStatus, ValidatorList},
};
use stakedex_deposit_stake_interface::{
    spl_stake_pool_deposit_stake_ix, SplStakePoolDepositStakeIxArgs, SplStakePoolDepositStakeKeys,
};
use stakedex_sdk_common::{BaseStakePoolAmm, DepositStake, DepositStakeQuote, WithdrawStakeQuote};

use crate::SPL_STAKE_POOL_STATE_TO_LABEL;

#[derive(Clone, Default)]
pub struct SplStakePoolDepositWithdrawStake {
    stake_pool_addr: Pubkey,
    withdraw_authority_addr: Pubkey,
    stake_pool_label: &'static str,
    stake_pool: StakePool,
    validator_list: ValidatorList,
}

impl SplStakePoolDepositWithdrawStake {
    /// Initialize from stake pool main account
    pub fn from_keyed_account(keyed_account: &KeyedAccount) -> Result<Self> {
        let mut res = Self::default();
        res.stake_pool_addr = keyed_account.key;
        res.withdraw_authority_addr =
            find_withdraw_authority_program_address(&spl_stake_pool::ID, &res.stake_pool_addr).0;
        res.stake_pool_label = SPL_STAKE_POOL_STATE_TO_LABEL
            .get(&res.stake_pool_addr)
            .ok_or_else(|| anyhow!("Unknown spl stake pool: {}", res.stake_pool_addr))?;
        res.update_stake_pool(&keyed_account.account.data)?;
        // NOTE: the validator_list is not initialized until self.update() is
        // called for the first time with fetched on-chain data
        Ok(res)
    }

    pub fn update_stake_pool(&mut self, data: &[u8]) -> Result<()> {
        self.stake_pool = StakePool::try_from_slice(data)?;
        Ok(())
    }

    pub fn update_validator_list(&mut self, data: &[u8]) -> Result<()> {
        self.validator_list = ValidatorList::try_from_slice(data)?;
        Ok(())
    }
}

impl BaseStakePoolAmm for SplStakePoolDepositWithdrawStake {
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
        Vec::from([self.stake_pool_addr, self.stake_pool.validator_list])
    }

    fn update(&mut self, accounts_map: &std::collections::HashMap<Pubkey, Vec<u8>>) -> Result<()> {
        let stake_pool_data = accounts_map.get(&self.stake_pool_addr).unwrap();
        self.update_stake_pool(stake_pool_data)?;
        let validator_list_data = accounts_map.get(&self.stake_pool.validator_list).unwrap();
        self.update_validator_list(validator_list_data)
    }
}

// TODO: handle case where stake pool hasn't been updated for the current epoch

impl DepositStake for SplStakePoolDepositWithdrawStake {
    fn get_deposit_stake_quote(
        &self,
        withdraw_stake_quote: WithdrawStakeQuote,
    ) -> Option<DepositStakeQuote> {
        let validator_list_entry = self.validator_list.find(&withdraw_stake_quote.voter)?;
        if validator_list_entry.status != StakeStatus::Active {
            return None;
        }
        // Reference: https://github.com/solana-labs/solana-program-library/blob/stake-pool-v0.6.4/stake-pool/program/src/processor.rs#L1971
        let total_deposit_lamports = withdraw_stake_quote.lamports_out;
        let stake_deposit_lamports = withdraw_stake_quote.lamports_staked;

        let new_pool_tokens = self
            .stake_pool
            .calc_pool_tokens_for_deposit(total_deposit_lamports)?;
        let new_pool_tokens_from_stake = self
            .stake_pool
            .calc_pool_tokens_for_deposit(stake_deposit_lamports)?;
        let new_pool_tokens_from_sol = new_pool_tokens.checked_sub(new_pool_tokens_from_stake)?;

        let stake_deposit_fee = self
            .stake_pool
            .calc_pool_tokens_stake_deposit_fee(new_pool_tokens_from_stake)?;
        let sol_deposit_fee = self
            .stake_pool
            .calc_pool_tokens_sol_deposit_fee(new_pool_tokens_from_sol)?;
        let total_fee = stake_deposit_fee.checked_add(sol_deposit_fee)?;
        let pool_tokens_user = new_pool_tokens.checked_sub(total_fee)?;

        Some(DepositStakeQuote {
            tokens_out: pool_tokens_user,
            fee_amount: total_fee,
            voter: withdraw_stake_quote.voter,
        })
    }

    fn virtual_ix(&self, quote: &DepositStakeQuote) -> Result<Instruction> {
        let deposit_stake_validator_stake =
            find_stake_program_address(&spl_stake_pool::ID, &quote.voter, &self.stake_pool_addr).0;
        Ok(spl_stake_pool_deposit_stake_ix(
            SplStakePoolDepositStakeKeys {
                spl_stake_pool_program: spl_stake_pool::ID,
                deposit_stake_spl_stake_pool: self.stake_pool_addr,
                deposit_stake_validator_list: self.stake_pool.validator_list,
                deposit_stake_deposit_authority: self.stake_pool.stake_deposit_authority,
                deposit_stake_withdraw_authority: self.withdraw_authority_addr,
                deposit_stake_reserve_stake: self.stake_pool.reserve_stake,
                deposit_stake_manager_fee: self.stake_pool.manager_fee_account,
                deposit_stake_validator_stake,
                clock: sysvar::clock::ID,
                stake_history: sysvar::stake_history::ID,
                token_program: spl_token::ID,
                stake_program: stake::program::ID,
            },
            SplStakePoolDepositStakeIxArgs {},
        )?)
    }
}
