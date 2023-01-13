use anyhow::Result;
use solana_program::{instruction::Instruction, pubkey::Pubkey};

use crate::BaseStakePoolAmm;

use super::withdraw_stake::WithdrawStakeQuote;

// TODO: include additional rent payments?
#[derive(Clone, Copy, Debug, Default)]
pub struct DepositStakeQuote {
    /// Output tokens, after subtracting fees
    pub tokens_out: u64,

    /// In terms of output tokens
    pub fee_amount: u64,

    /// Active voter of the input stake account
    pub voter: Pubkey,
}

impl DepositStakeQuote {
    pub fn is_zero_out(&self) -> bool {
        self.tokens_out == 0
    }
}

/// Info about the stake account to be deposited
#[derive(Clone, Copy, Debug)]
pub struct DepositStakeInfo {
    pub addr: Pubkey,
}

pub trait DepositStake: BaseStakePoolAmm {
    /// This should only include the stake pool's deposit stake fees, not stakedex's global fees
    /// Returns None if stake pool cannot currently accept stake deposits (e.g. not yet updated for this epoch)
    /// Returns DepositStakeQuote::default() if unable to handle withdraw_stake_quote (e.g. cannot accept provided voter)
    fn get_deposit_stake_quote(
        &self,
        withdraw_stake_quote: WithdrawStakeQuote,
    ) -> Option<DepositStakeQuote> {
        if !self.can_accept_stake_deposits() {
            return None;
        }
        Some(self.get_deposit_stake_quote_unchecked(withdraw_stake_quote))
    }

    fn can_accept_stake_deposits(&self) -> bool;

    /// Inner impl fn, should not be called directly. Instead, call
    /// get_deposit_stake_quote()
    fn get_deposit_stake_quote_unchecked(
        &self,
        withdraw_stake_quote: WithdrawStakeQuote,
    ) -> DepositStakeQuote;

    fn virtual_ix(
        &self,
        quote: &DepositStakeQuote,
        deposit_stake_info: &DepositStakeInfo,
    ) -> Result<Instruction>;
}
