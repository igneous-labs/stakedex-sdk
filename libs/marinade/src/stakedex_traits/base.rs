use anyhow::Result;
use solana_program::pubkey::Pubkey;
use stakedex_sdk_common::{
    account_missing_err,
    jupiter_stakedex_interface::{AccountMap, KeyedAccount},
    marinade_state, msol, BaseStakePoolAmm, InitFromKeyedAccount,
};

use crate::{MarinadeStakedex, MARINADE_LABEL};

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
            self.state.stake_system.stake_list.account,
        ]
    }

    fn update(&mut self, accounts_map: &AccountMap) -> Result<()> {
        let state_data = accounts_map
            .get(&marinade_state::ID)
            .ok_or_else(|| account_missing_err(&marinade_state::ID))?
            .data
            .as_ref();
        self.update_state(state_data)?;

        let validator_list_pubkey = self.state.validator_system.validator_list.account;
        let validator_records_data = accounts_map
            .get(&validator_list_pubkey)
            .ok_or_else(|| account_missing_err(&validator_list_pubkey))?
            .data
            .as_ref();
        self.update_validator_records(validator_records_data)?;

        let stake_list_pubkey = self.state.stake_system.stake_list.account;
        let stake_records_data = accounts_map
            .get(&stake_list_pubkey)
            .ok_or_else(|| account_missing_err(&stake_list_pubkey))?
            .data
            .as_ref();
        self.update_stake_records(stake_records_data)?;

        Ok(())
    }
}
