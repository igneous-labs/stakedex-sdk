use anyhow::Result;
use solana_program::{instruction::Instruction, stake, system_program, sysvar};
use stakedex_deposit_stake_interface::{
    marinade_deposit_stake_ix, MarinadeDepositStakeKeys, MARINADE_DEPOSIT_STAKE_IX_ACCOUNTS_LEN,
};
use stakedex_sdk_common::{
    marinade_program, marinade_state, DepositStake, DepositStakeInfo, DepositStakeQuote,
    WithdrawStakeQuote,
};

use crate::{state::StateWrapper, validator_system::ValidatorRecordWrapper, MarinadeStakedex};

impl DepositStake for MarinadeStakedex {
    fn can_accept_stake_deposits(&self) -> bool {
        true
    }

    fn get_deposit_stake_quote_unchecked(
        &self,
        withdraw_stake_quote: WithdrawStakeQuote,
    ) -> DepositStakeQuote {
        // https://github.com/marinade-finance/liquid-staking-program/blob/main/programs/marinade-finance/src/stake_system/deposit_stake_account.rs#L89
        if withdraw_stake_quote.lamports_staked < self.state.stake_system.min_stake {
            return DepositStakeQuote::default();
        }
        if StateWrapper(&self.state)
            .check_staking_cap(withdraw_stake_quote.lamports_staked)
            .is_err()
        {
            return DepositStakeQuote::default();
        }
        if !self
            .validator_records
            .iter()
            .any(|v| v.validator_account == withdraw_stake_quote.voter)
            && self.state.validator_system.auto_add_validator_enabled == 0
        {
            return DepositStakeQuote::default();
        }
        let state = StateWrapper(&self.state);
        let msol_full = match state.calc_msol_from_lamports(withdraw_stake_quote.lamports_out) {
            Ok(r) => r,
            Err(_e) => return DepositStakeQuote::default(),
        };
        let msol_to_mint = match state.calc_msol_from_lamports(withdraw_stake_quote.lamports_staked)
        {
            Ok(r) => r,
            Err(_e) => return DepositStakeQuote::default(),
        };
        if msol_full < msol_to_mint {
            return DepositStakeQuote::default();
        }
        DepositStakeQuote {
            tokens_out: msol_to_mint,
            voter: withdraw_stake_quote.voter,
            fee_amount: msol_full - msol_to_mint,
        }
    }

    fn virtual_ix(
        &self,
        quote: &DepositStakeQuote,
        _deposit_stake_info: &DepositStakeInfo,
    ) -> Result<Instruction> {
        Ok(marinade_deposit_stake_ix(MarinadeDepositStakeKeys {
            marinade_program: marinade_program::ID,
            deposit_stake_marinade_state: marinade_state::ID,
            deposit_stake_validator_list: self.state.validator_system.validator_list.account,
            deposit_stake_stake_list: self.state.stake_system.stake_list.account,
            deposit_stake_duplication_flag: ValidatorRecordWrapper::find_duplication_flag(
                &marinade_state::ID,
                &quote.voter,
            )
            .0,
            deposit_stake_msol_mint_auth: marinade_program::MSOL_MINT_AUTH_ID,
            clock: sysvar::clock::ID,
            rent: sysvar::rent::ID,
            system_program: system_program::ID,
            token_program: spl_token::ID,
            stake_program: stake::program::ID,
        })?)
    }

    fn accounts_len(&self) -> usize {
        MARINADE_DEPOSIT_STAKE_IX_ACCOUNTS_LEN
    }
}
