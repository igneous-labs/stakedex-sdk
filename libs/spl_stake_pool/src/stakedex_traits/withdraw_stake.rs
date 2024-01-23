use anyhow::Result;
use solana_program::{instruction::Instruction, pubkey::Pubkey, stake, system_program, sysvar};
use spl_stake_pool::{find_stake_program_address, MINIMUM_ACTIVE_STAKE};
use stakedex_sdk_common::{WithdrawStakeBase, WithdrawStakeIter, WithdrawStakeQuote};
use stakedex_withdraw_stake_interface::{
    spl_stake_pool_withdraw_stake_ix, SplStakePoolWithdrawStakeKeys,
    SPL_STAKE_POOL_WITHDRAW_STAKE_IX_ACCOUNTS_LEN,
};

use crate::SplStakePoolStakedex;

pub struct WithdrawStakeQuoteIter<'a> {
    pool: &'a SplStakePoolStakedex,
    withdraw_amount: u64,
    state: WithdrawStakeQuoteIterState,
}

pub enum WithdrawStakeQuoteIterState {
    Normal(usize),
    Preferred,
    Ended,
}

impl<'a> WithdrawStakeQuoteIter<'a> {
    fn next_normal(
        &self,
        curr_index: usize,
    ) -> Option<(WithdrawStakeQuote, WithdrawStakeQuoteIterState)> {
        let wsq = self
            .pool
            .get_withdraw_stake_quote_for_validator_copied(curr_index, self.withdraw_amount)
            .unwrap_or_default();
        let next_state = if curr_index >= self.pool.validator_list.validators.len() - 1 {
            WithdrawStakeQuoteIterState::Ended
        } else {
            WithdrawStakeQuoteIterState::Normal(curr_index + 1)
        };
        Some((wsq, next_state))
    }

    fn next_preferred(&self) -> Option<(WithdrawStakeQuote, WithdrawStakeQuoteIterState)> {
        // unwrap-safety: WithdrawStakeQuoteIter is only created by SplStakePoolStakedex::withdraw_stake_quote_iter()
        let preferred_voter = self
            .pool
            .stake_pool
            .preferred_withdraw_validator_vote_address
            .unwrap();
        let (preferred_index, vsi) = self
            .pool
            .validator_list
            .validators
            .iter()
            .enumerate()
            .find(|(_, vsi)| vsi.vote_account_address == preferred_voter)?;
        // preferred cant service withdrawals, fallback to normal
        if u64::from(vsi.active_stake_lamports) <= MINIMUM_ACTIVE_STAKE {
            return Some((
                WithdrawStakeQuote::default(),
                WithdrawStakeQuoteIterState::Normal(0),
            ));
        }
        match self
            .pool
            .get_withdraw_stake_quote_for_validator_copied(preferred_index, self.withdraw_amount)
        {
            Ok(wsq) => Some((wsq, WithdrawStakeQuoteIterState::Ended)),
            Err(_) => None, // preferred can still service withdrawals but this withdraw amt too much: end iteration
        }
    }
}

impl<'a> Iterator for WithdrawStakeQuoteIter<'a> {
    type Item = WithdrawStakeQuote;

    fn next(&mut self) -> Option<Self::Item> {
        let (ret, next_state) = match self.state {
            WithdrawStakeQuoteIterState::Normal(i) => self.next_normal(i),
            WithdrawStakeQuoteIterState::Preferred => self.next_preferred(),
            WithdrawStakeQuoteIterState::Ended => None,
        }?;
        self.state = next_state;
        Some(ret)
    }
}

impl WithdrawStakeIter for SplStakePoolStakedex {
    type Iter<'me> = WithdrawStakeQuoteIter<'me>;

    fn withdraw_stake_quote_iter(&self, withdraw_amount: u64) -> Self::Iter<'_> {
        WithdrawStakeQuoteIter {
            pool: self,
            withdraw_amount,
            state: match self.stake_pool.preferred_withdraw_validator_vote_address {
                None => WithdrawStakeQuoteIterState::Normal(0),
                Some(_) => WithdrawStakeQuoteIterState::Preferred,
            },
        }
    }
}

impl WithdrawStakeBase for SplStakePoolStakedex {
    fn can_accept_stake_withdrawals(&self) -> bool {
        self.stake_pool.last_update_epoch >= self.curr_epoch
    }

    fn virtual_ix(&self, quote: &WithdrawStakeQuote) -> Result<Instruction> {
        let withdraw_stake_stake_to_split =
            find_stake_program_address(&spl_stake_pool::ID, &quote.voter, &self.stake_pool_addr).0;
        Ok(spl_stake_pool_withdraw_stake_ix(
            SplStakePoolWithdrawStakeKeys {
                spl_stake_pool_program: spl_stake_pool::ID,
                withdraw_stake_spl_stake_pool: self.stake_pool_addr,
                withdraw_stake_validator_list: self.stake_pool.validator_list,
                withdraw_stake_withdraw_authority: self.withdraw_authority_addr,
                withdraw_stake_manager_fee: self.stake_pool.manager_fee_account,
                withdraw_stake_stake_to_split,
                clock: sysvar::clock::ID,
                token_program: spl_token::ID,
                stake_program: stake::program::ID,
                system_program: system_program::ID,
            },
        )?)
    }

    fn accounts_len(&self) -> usize {
        SPL_STAKE_POOL_WITHDRAW_STAKE_IX_ACCOUNTS_LEN
    }

    fn underlying_liquidity(&self) -> Option<&Pubkey> {
        Some(&self.stake_pool_addr)
    }
}
