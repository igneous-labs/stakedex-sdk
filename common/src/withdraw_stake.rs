// TODO: remove once everyone has upgraded to ^1.17
#![allow(deprecated)]

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

pub trait WithdrawStakeIter {
    /// Iter should return WithdrawStakeQuote::default() if the current validator
    /// cant service the withdrawal but next ones maybe can.
    ///
    /// Otherwise, it should return None to indicate iteration has ended and stop searching
    type Iter<'me>: Iterator<Item = WithdrawStakeQuote>
    where
        Self: 'me;

    fn withdraw_stake_quote_iter(&self, withdraw_amount: u64) -> Self::Iter<'_>;
}

pub trait WithdrawStakeBase {
    fn can_accept_stake_withdrawals(&self) -> bool;

    fn virtual_ix(&self, quote: &WithdrawStakeQuote) -> Result<Instruction>;
}

pub trait WithdrawStake: BaseStakePoolAmm + WithdrawStakeBase {
    fn withdraw_stake_quote_iter_dyn(
        &self,
        withdraw_amount: u64,
    ) -> Box<dyn Iterator<Item = WithdrawStakeQuote> + '_>;
}

impl<T: WithdrawStakeIter + WithdrawStakeBase + BaseStakePoolAmm> WithdrawStake for T {
    fn withdraw_stake_quote_iter_dyn(
        &self,
        withdraw_amount: u64,
    ) -> Box<dyn Iterator<Item = WithdrawStakeQuote> + '_> {
        Box::new(self.withdraw_stake_quote_iter(withdraw_amount))
    }
}
