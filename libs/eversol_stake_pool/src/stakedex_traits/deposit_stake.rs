use anyhow::Result;
use solana_program::{instruction::Instruction, stake, sysvar};
use stakedex_deposit_stake_interface::{
    eversol_stake_pool_deposit_stake_ix, EversolStakePoolDepositStakeIxArgs,
    EversolStakePoolDepositStakeKeys,
};
use stakedex_sdk_common::{
    eversol_program, eversol_stake_pool, DepositStake, DepositStakeInfo, DepositStakeQuote,
    WithdrawStakeQuote,
};

use crate::EversolStakePoolStakedex;

impl DepositStake for EversolStakePoolStakedex {
    fn can_accept_stake_deposits(&self) -> bool {
        self.is_updated_this_epoch()
    }

    /// Reference (copy-pasta):
    /// https://github.com/everstake/solana-program-library/blob/22534fe3885e698598e92b2fe20da3a8adbfc5ff/stake-pool/program/src/processor.rs#L2309-L2355
    fn get_deposit_stake_quote_unchecked(
        &self,
        withdraw_stake_quote: WithdrawStakeQuote,
    ) -> DepositStakeQuote {
        self.get_deposit_stake_quote_copied(withdraw_stake_quote)
            .unwrap_or_default()
    }

    fn virtual_ix(
        &self,
        quote: &DepositStakeQuote,
        _deposit_stake_info: &DepositStakeInfo,
    ) -> Result<Instruction> {
        Ok(eversol_stake_pool_deposit_stake_ix(
            EversolStakePoolDepositStakeKeys {
                eversol_stake_pool_program: eversol_program::ID,
                deposit_stake_spl_stake_pool: eversol_stake_pool::ID,
                deposit_stake_validator_list: self.stake_pool.validator_list,
                deposit_stake_deposit_authority: self.stake_pool.stake_deposit_authority,
                deposit_stake_withdraw_authority: Self::withdraw_authority(),
                deposit_stake_reserve_stake: self.stake_pool.reserve_stake,
                deposit_stake_manager_fee: self.stake_pool.manager_fee_account,
                deposit_stake_validator_stake: Self::vsa(&quote.voter),
                clock: sysvar::clock::ID,
                stake_history: sysvar::stake_history::ID,
                token_program: spl_token::ID,
                stake_program: stake::program::ID,
            },
            EversolStakePoolDepositStakeIxArgs {},
        )?)
    }
}
