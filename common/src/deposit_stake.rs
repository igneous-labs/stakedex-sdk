use anyhow::Result;
use solana_program::{instruction::Instruction, pubkey::Pubkey};

use crate::BaseStakePoolAmm;

use super::withdraw_stake::WithdrawStakeQuote;

// TODO: include additional rent payments?
#[derive(Clone, Copy, Debug)]
pub struct DepositStakeQuote {
    /// Output tokens, after subtracting fees
    pub tokens_out: u64,

    /// In terms of output tokens
    pub fee_amount: u64,

    /// Active voter of the input stake account
    pub voter: Pubkey,
}

pub trait DepositStake: BaseStakePoolAmm {
    /// This should only include the stake pool's fees, not stakedex's global fees
    /// Returns none if unable to handle withdraw_stake_quote (e.g. cannot accept provided voter)
    fn get_deposit_stake_quote(
        &self,
        withdraw_stake_quote: WithdrawStakeQuote,
    ) -> Option<DepositStakeQuote>;

    fn virtual_ix(&self, quote: &DepositStakeQuote) -> Result<Instruction>;
}
