use anyhow::{anyhow, Result};
use solana_program::{
    clock::Clock,
    instruction::Instruction,
    pubkey::Pubkey,
    stake::state::{Delegation, Stake, StakeState},
};

use crate::{BaseStakePoolAmm, STAKE_ACCOUNT_RENT_EXEMPT_LAMPORTS};

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
    // TODO: activation epoch
    // (marinade only accepts >=2 epochs activation)
    // Any way to get without fetching all stake pools' stake accounts?
}

impl WithdrawStakeQuote {
    pub fn is_zero_out(&self) -> bool {
        self.lamports_out == 0
    }

    pub fn from_lamports_and_voter(stake_acc_lamports: u64, voter: Pubkey) -> Self {
        let (lamports_out, lamports_staked) =
            if stake_acc_lamports > STAKE_ACCOUNT_RENT_EXEMPT_LAMPORTS {
                (
                    stake_acc_lamports,
                    stake_acc_lamports - STAKE_ACCOUNT_RENT_EXEMPT_LAMPORTS,
                )
            } else {
                (0, 0)
            };

        Self {
            lamports_out,
            lamports_staked,
            fee_amount: 0,
            voter,
        }
    }

    pub fn from_delegation(d: &Delegation, stake_acc_lamports: u64) -> Self {
        Self {
            lamports_out: stake_acc_lamports,
            lamports_staked: d.stake,
            fee_amount: 0,
            voter: d.voter_pubkey,
        }
    }

    pub fn from_stake(s: &Stake, stake_acc_lamports: u64) -> Self {
        Self::from_delegation(&s.delegation, stake_acc_lamports)
    }

    pub fn try_from_stake_acc(
        s: &StakeState,
        stake_acc_lamports: u64,
        clock: &Clock,
    ) -> Result<Self> {
        if let StakeState::Stake(meta, stake) = s {
            if meta.lockup.is_in_force(clock, None) {
                return Err(anyhow!("Stake acc lockup in force"));
            }
            return Ok(Self::from_stake(stake, stake_acc_lamports));
        }
        Err(anyhow!("Stake acc not staked"))
    }
}

pub struct WithdrawStakeQuoteIter {
    withdraw_amount: u64,
    curr_validator_index: usize,
}

impl WithdrawStakeQuoteIter {
    pub fn next<P: WithdrawStake + ?Sized>(&mut self, pool: &P) -> Option<WithdrawStakeQuote> {
        let res = pool.get_quote_for_validator(self.curr_validator_index, self.withdraw_amount);
        self.curr_validator_index += 1;
        res
    }
}

pub trait WithdrawStake: BaseStakePoolAmm {
    fn withdraw_stake_quote_iter(&self, withdraw_amount: u64) -> WithdrawStakeQuoteIter {
        WithdrawStakeQuoteIter {
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
