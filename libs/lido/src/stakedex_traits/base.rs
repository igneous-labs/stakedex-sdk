use anyhow::Result;
use jupiter_amm_interface::{AccountMap, AmmContext, KeyedAccount};
use solana_program::pubkey::Pubkey;
use stakedex_sdk_common::{
    account_missing_err, lido_program, lido_state, stsol, BaseStakePoolAmm, InitFromKeyedAccount,
};

use crate::{LidoStakedex, LIDO_LABEL};

impl InitFromKeyedAccount for LidoStakedex {
    /// Initialize from lido
    fn from_keyed_account(keyed_account: &KeyedAccount, amm_context: &AmmContext) -> Result<Self> {
        let mut res = Self::default();
        res.update_lido_state(&keyed_account.account.data)?;
        res.curr_epoch = amm_context.clock_ref.epoch.clone();
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
        vec![lido_state::ID, self.lido_state.validator_list]
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
        Ok(())
    }
}
