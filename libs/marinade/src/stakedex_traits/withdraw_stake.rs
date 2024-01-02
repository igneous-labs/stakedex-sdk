use anyhow::Result;
use solana_program::{instruction::Instruction, stake, system_program, sysvar};
use stakedex_sdk_common::{
    marinade_program, marinade_state, WithdrawStake, WithdrawStakeBase, WithdrawStakeQuote,
};
use stakedex_withdraw_stake_interface::{marinade_withdraw_stake_ix, MarinadeWithdrawStakeKeys};

use crate::MarinadeStakedex;

impl WithdrawStakeBase for MarinadeStakedex {
    fn can_accept_stake_withdrawals(&self) -> bool {
        !self.state.paused && self.state.withdraw_stake_account_enabled
    }

    fn virtual_ix(&self, _quote: &WithdrawStakeQuote) -> Result<Instruction> {
        Ok(marinade_withdraw_stake_ix(MarinadeWithdrawStakeKeys {
            marinade_program: marinade_program::ID,
            withdraw_stake_marinade_state: marinade_state::ID,
            withdraw_stake_marinade_treasury: self.state.treasury_msol_account,
            withdraw_stake_validator_list: self.state.validator_system.validator_list.account,
            withdraw_stake_stake_list: self.state.stake_system.stake_list.account,
            withdraw_stake_withdraw_authority: marinade_program::STAKE_WITHDRAW_AUTH_ID,
            withdraw_stake_deposit_authority: marinade_program::STAKE_DEPOSIT_AUTH_ID,
            clock: sysvar::clock::ID,
            token_program: spl_token::ID,
            stake_program: stake::program::ID,
            system_program: system_program::ID,
            // TODO: marinade doesn't provide an easy way of matching voter to stake account
            // so we might have to fetch all their stake accounts on update...
            withdraw_stake_stake_to_split: Default::default(),
        })?)
    }
}

impl WithdrawStake for MarinadeStakedex {
    fn withdraw_stake_quote_iter_dyn(
        &self,
        _withdraw_amount: u64,
    ) -> Box<dyn Iterator<Item = WithdrawStakeQuote> + '_> {
        todo!()
    }
}
