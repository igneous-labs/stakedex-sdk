use anyhow::Result;
use jupiter_amm_interface::{AccountMap, AmmContext, KeyedAccount};
use solana_program::pubkey::Pubkey;
use stakedex_sdk_common::{
    account_missing_err, marinade_program, marinade_state, msol, BaseStakePoolAmm,
    InitFromKeyedAccount,
};

use crate::{MarinadeStakedex, MARINADE_LABEL};

impl InitFromKeyedAccount for MarinadeStakedex {
    /// Initialize from state
    fn from_keyed_account(keyed_account: &KeyedAccount, _amm_context: &AmmContext) -> Result<Self> {
        let mut res = Self::default();
        res.update_state(&keyed_account.account.data)?;
        // NOTE: validator_records is not initialized until self.update() is
        // called for the first time with fetched on-chain data
        Ok(res)
    }
}

impl BaseStakePoolAmm for MarinadeStakedex {
    fn program_id(&self) -> Pubkey {
        marinade_program::ID
    }

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
