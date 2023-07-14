use anyhow::Result;
use solana_program::{instruction::Instruction, pubkey::Pubkey, stake, system_program, sysvar};
use stakedex_deposit_sol_interface::{
    marinade_deposit_sol_ix, MarinadeDepositSolIxArgs, MarinadeDepositSolKeys,
};
use stakedex_deposit_stake_interface::{
    marinade_deposit_stake_ix, MarinadeDepositStakeIxArgs, MarinadeDepositStakeKeys,
};
use stakedex_sdk_common::{
    account_missing_err,
    jupiter_stakedex_interface::{AccountMap, KeyedAccount},
    marinade_program, marinade_state, msol, BaseStakePoolAmm, DepositSol, DepositSolQuote,
    DepositStake, DepositStakeInfo, DepositStakeQuote, InitFromKeyedAccount, WithdrawStakeQuote,
};

use crate::{
    liq_pool::LiqPoolWrapper, state::StateWrapper, validator_system::ValidatorRecordWrapper,
    MarinadeStakedex, MARINADE_LABEL,
};

impl InitFromKeyedAccount for MarinadeStakedex {
    /// Initialize from state
    fn from_keyed_account(keyed_account: &KeyedAccount) -> Result<Self> {
        let mut res = Self::default();
        res.update_state(&keyed_account.account.data)?;
        // NOTE: validator_records is not initialized until self.update() is
        // called for the first time with fetched on-chain data
        Ok(res)
    }
}

impl BaseStakePoolAmm for MarinadeStakedex {
    fn stake_pool_label(&self) -> &'static str {
        MARINADE_LABEL
    }

    fn main_state_key(&self) -> Pubkey {
        marinade_state::ID
    }

    fn staked_sol_mint(&self) -> Pubkey {
        msol::ID
    }

    fn get_accounts_to_update(&self) -> Vec<Pubkey> {
        vec![
            marinade_state::ID,
            self.state.validator_system.validator_list.account,
        ]
    }

    fn update(&mut self, accounts_map: &AccountMap) -> Result<()> {
        let state_data = accounts_map
            .get(&marinade_state::ID)
            .ok_or_else(|| account_missing_err(&marinade_state::ID))?
            .data
            .as_ref();
        self.update_state(state_data)?;
        let validator_records_data = accounts_map
            .get(&self.state.validator_system.validator_list.account)
            .ok_or_else(|| {
                account_missing_err(&self.state.validator_system.validator_list.account)
            })?
            .data
            .as_ref();
        self.update_validator_records(validator_records_data)?;
        Ok(())
    }
}

impl DepositSol for MarinadeStakedex {
    fn get_deposit_sol_quote(&self, user_lamports: u64) -> Result<DepositSolQuote> {
        // Reference: https://github.com/marinade-finance/liquid-staking-program/blob/main/programs/marinade-finance/src/state/deposit.rs#L27
        let out_amount = StateWrapper(&self.state).calc_msol_from_lamports(user_lamports)?;
        // TODO: this is a simplified calc that doesn't account for the liquidity pool, which can result in a diff of 1 lamport
        Ok(DepositSolQuote {
            in_amount: user_lamports,
            out_amount,
            fee_amount: 0,
        })
    }

    fn virtual_ix(&self) -> Result<Instruction> {
        Ok(marinade_deposit_sol_ix(
            MarinadeDepositSolKeys {
                marinade_program: marinade_program::ID,
                marinade_state: marinade_state::ID,
                msol_mint_authority: StateWrapper::find_msol_mint_authority(&marinade_state::ID).0,
                marinade_reserve: StateWrapper::find_reserve_address(&marinade_state::ID).0,
                marinade_liq_pool_msol_leg: self.state.liq_pool.msol_leg,
                marinade_liq_pool_msol_leg_auth: LiqPoolWrapper::find_msol_leg_authority(
                    &marinade_state::ID,
                )
                .0,
                marinade_liq_pool_sol_leg: LiqPoolWrapper::find_sol_leg_address(
                    &marinade_state::ID,
                )
                .0,
            },
            MarinadeDepositSolIxArgs {},
        )?)
    }
}

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
        let msol_full = state
            .calc_msol_from_lamports(withdraw_stake_quote.lamports_out)
            .unwrap();
        let msol_to_mint = state
            .calc_msol_from_lamports(withdraw_stake_quote.lamports_staked)
            .unwrap();
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
        Ok(marinade_deposit_stake_ix(
            MarinadeDepositStakeKeys {
                marinade_program: marinade_program::ID,
                deposit_stake_marinade_state: marinade_state::ID,
                deposit_stake_validator_list: self.state.validator_system.validator_list.account,
                deposit_stake_stake_list: self.state.stake_system.stake_list.account,
                deposit_stake_duplication_flag: ValidatorRecordWrapper::find_duplication_flag(
                    &marinade_state::ID,
                    &quote.voter,
                )
                .0,
                deposit_stake_msol_mint_auth: StateWrapper::find_msol_mint_authority(
                    &marinade_state::ID,
                )
                .0,
                clock: sysvar::clock::ID,
                rent: sysvar::rent::ID,
                system_program: system_program::ID,
                token_program: spl_token::ID,
                stake_program: stake::program::ID,
            },
            MarinadeDepositStakeIxArgs {},
        )?)
    }
}
