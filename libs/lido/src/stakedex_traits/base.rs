use anyhow::Result;
use jupiter_amm_interface::{AccountMap, KeyedAccount};
use solana_program::{pubkey::Pubkey, sysvar};
use stakedex_sdk_common::{
    account_missing_err, lido_program, lido_state, stsol, BaseStakePoolAmm, InitFromKeyedAccount,
};

use crate::{LidoStakedex, LIDO_LABEL};

impl InitFromKeyedAccount for LidoStakedex {
    /// Initialize from lido
    fn from_keyed_account(keyed_account: &KeyedAccount) -> Result<Self> {
        let mut res = Self::default();
        res.update_lido_state(&keyed_account.account.data)?;
        // NOTE: validator_list is not initialized until self.update() is
        // called for the first time with fetched on-chain data
        Ok(res)
    }
}

impl BaseStakePoolAmm for LidoStakedex {
    fn program_id(&self) -> Pubkey {
        lido_program::ID
    }

    fn stake_pool_label(&self) -> &'static str {
        LIDO_LABEL
    }

    fn main_state_key(&self) -> Pubkey {
        lido_state::ID
    }

    fn staked_sol_mint(&self) -> Pubkey {
        stsol::ID
    }

    fn get_accounts_to_update(&self) -> Vec<Pubkey> {
        vec![
            lido_state::ID,
            self.lido_state.validator_list,
            sysvar::clock::ID,
        ]
    }

    fn update(&mut self, accounts_map: &AccountMap) -> Result<()> {
        let state_data = accounts_map
            .get(&lido_state::ID)
            .ok_or_else(|| account_missing_err(&lido_state::ID))?
            .data
            .as_ref();
        self.update_lido_state(state_data)?;
        let validator_list_data = accounts_map
            .get(&self.lido_state.validator_list)
            .ok_or_else(|| account_missing_err(&self.lido_state.validator_list))?
            .data
            .as_ref();
        self.update_validator_list(validator_list_data)?;
        let clock_data = accounts_map
            .get(&sysvar::clock::ID)
            .ok_or_else(|| account_missing_err(&sysvar::clock::ID))?
            .data
            .as_ref();
        self.update_curr_epoch(clock_data)?;
        Ok(())
    }
}
