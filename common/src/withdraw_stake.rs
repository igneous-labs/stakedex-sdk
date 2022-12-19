use anyhow::Result;
use solana_program::{instruction::Instruction, pubkey::Pubkey};

use crate::BaseStakePoolAmm;

// TODO: include additional rent payments?
#[derive(Clone, Copy, Debug, Default)]
pub struct WithdrawStakeQuote {
    /// Total lamports contained in the output stake account,
    /// after subtracting fees
    pub lamports_out: u64,

    /// delegation.stake.lamports of the output stake account
    /// This is basically `lamports_out` - rent_exempt_minimum
    pub lamports_staked: u64,

    /// In terms of input mint
    pub fee_amount: u64,

    /// Active voter of the output stake account
    pub voter: Pubkey,
}

impl WithdrawStakeQuote {
    pub fn is_zero_out(&self) -> bool {
        self.lamports_out == 0
    }
}

pub struct WithdrawStakeQuoteIter<'a, W: WithdrawStake> {
    pool: &'a W,
    withdraw_amount: u64,
    curr_validator_index: usize,
}

pub trait WithdrawStake: BaseStakePoolAmm + Sized {
    fn withdraw_stake_quote_iter(&self, withdraw_amount: u64) -> WithdrawStakeQuoteIter<Self> {
        WithdrawStakeQuoteIter {
            pool: self,
            withdraw_amount,
            curr_validator_index: 0,
        }
    }

    /// Returns None if validator_index out of bounds.
    /// Returns None if stake pool cannot currently accept stake withdrawals
    /// (e.g. spl not yet updated for this epoch)
    /// Returns WithdrawStakeQuote::default() if given validator cant service withdrawal
    /// eg withdraw_amount > validator stake
    fn get_quote_for_validator(
        &self,
        validator_index: usize,
        withdraw_amount: u64,
    ) -> Option<WithdrawStakeQuote> {
        if self.is_validator_index_out_of_bounds(validator_index) {
            return None;
        }
        if !self.can_accept_stake_withdrawals() {
            return None;
        }
        Some(self.get_quote_for_validator_unchecked(validator_index, withdraw_amount))
    }

    fn is_validator_index_out_of_bounds(&self, validator_index: usize) -> bool;

    fn can_accept_stake_withdrawals(&self) -> bool;

    /// panics if validator_index out of bounds
    /// is_validator_index_out_of_bounds() should be called before calling this
    /// Inner impl fn, should not be called directly. Instead, call
    /// get_quote_for_validator()
    fn get_quote_for_validator_unchecked(
        &self,
        validator_index: usize,
        withdraw_amount: u64,
    ) -> WithdrawStakeQuote;

    fn virtual_ix(&self, quote: &WithdrawStakeQuote) -> Result<Instruction>;
}

impl<'a, W: WithdrawStake> Iterator for WithdrawStakeQuoteIter<'a, W> {
    type Item = WithdrawStakeQuote;

    fn next(&mut self) -> Option<Self::Item> {
        let res = self
            .pool
            .get_quote_for_validator(self.curr_validator_index, self.withdraw_amount);
        self.curr_validator_index += 1;
        res
    }
}
