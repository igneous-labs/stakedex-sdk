use anyhow::Result;
use solana_program::{instruction::Instruction, stake, sysvar};
use spl_stake_pool::{
    find_deposit_authority_program_address, find_stake_program_address, state::StakeStatus,
};
use stakedex_deposit_stake_interface::{
    spl_stake_pool_deposit_stake_ix, SplStakePoolDepositStakeKeys,
};
use stakedex_sdk_common::{DepositStake, DepositStakeInfo, DepositStakeQuote, WithdrawStakeQuote};

use crate::SplStakePoolStakedex;

impl DepositStake for SplStakePoolStakedex {
    fn can_accept_stake_deposits(&self) -> bool {
        if self.stake_pool.stake_deposit_authority
            != find_deposit_authority_program_address(&spl_stake_pool::ID, &self.stake_pool_addr).0
        {
            return false;
        }
        self.is_updated_this_epoch()
    }

    // TODO: maybe refactor to same style as eversol
    // (_copied() function that returns Result and can copy pasta from on-chain src directly)
    fn get_deposit_stake_quote_unchecked(
        &self,
        withdraw_stake_quote: WithdrawStakeQuote,
    ) -> DepositStakeQuote {
        if let Some(v) = self.stake_pool.preferred_deposit_validator_vote_address {
            if withdraw_stake_quote.voter != v {
                return DepositStakeQuote::default();
            }
        }
        let validator_list_entry = match self.validator_list.find(&withdraw_stake_quote.voter) {
            Some(r) => r,
            None => return DepositStakeQuote::default(),
        };
        if validator_list_entry.status != StakeStatus::Active.into() {
            return DepositStakeQuote::default();
        }
        // Reference: https://github.com/solana-labs/solana-program-library/blob/stake-pool-v0.6.4/stake-pool/program/src/processor.rs#L1971
        let total_deposit_lamports = withdraw_stake_quote.lamports_out;
        let stake_deposit_lamports = withdraw_stake_quote.lamports_staked;

        let new_pool_tokens = match self
            .stake_pool
            .calc_pool_tokens_for_deposit(total_deposit_lamports)
        {
            Some(r) => r,
            None => return DepositStakeQuote::default(),
        };
        let new_pool_tokens_from_stake = match self
            .stake_pool
            .calc_pool_tokens_for_deposit(stake_deposit_lamports)
        {
            Some(r) => r,
            None => return DepositStakeQuote::default(),
        };
        let new_pool_tokens_from_sol = match new_pool_tokens.checked_sub(new_pool_tokens_from_stake)
        {
            Some(r) => r,
            None => return DepositStakeQuote::default(),
        };

        let stake_deposit_fee = match self
            .stake_pool
            .calc_pool_tokens_stake_deposit_fee(new_pool_tokens_from_stake)
        {
            Some(r) => r,
            None => return DepositStakeQuote::default(),
        };
        let sol_deposit_fee = match self
            .stake_pool
            .calc_pool_tokens_sol_deposit_fee(new_pool_tokens_from_sol)
        {
            Some(r) => r,
            None => return DepositStakeQuote::default(),
        };
        let total_fee = match stake_deposit_fee.checked_add(sol_deposit_fee) {
            Some(r) => r,
            None => return DepositStakeQuote::default(),
        };
        let pool_tokens_user = match new_pool_tokens.checked_sub(total_fee) {
            Some(r) => r,
            None => return DepositStakeQuote::default(),
        };
        // since we set referrer to the receiving fee_token_acc, referral fee is effectively kicked back to user
        let pool_tokens_referral_fee = match self
            .stake_pool
            .calc_pool_tokens_stake_referral_fee(total_fee)
        {
            Some(r) => r,
            None => return DepositStakeQuote::default(),
        };
        let tokens_out = match pool_tokens_user.checked_add(pool_tokens_referral_fee) {
            Some(r) => r,
            None => return DepositStakeQuote::default(),
        };
        let fee_amount = match total_fee.checked_sub(pool_tokens_referral_fee) {
            Some(r) => r,
            None => return DepositStakeQuote::default(),
        };

        DepositStakeQuote {
            tokens_out,
            fee_amount,
            voter: withdraw_stake_quote.voter,
        }
    }

    fn virtual_ix(
        &self,
        quote: &DepositStakeQuote,
        _deposit_stake_info: &DepositStakeInfo,
    ) -> Result<Instruction> {
        let deposit_stake_validator_stake = find_stake_program_address(
            &spl_stake_pool::ID,
            &quote.voter,
            &self.stake_pool_addr,
            None,
        )
        .0;
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
        )?)
    }
}
