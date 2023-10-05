use anyhow::Result;
use solana_program::pubkey::Pubkey;
use stakedex_sdk_common::{
    account_missing_err,
    jupiter_stakedex_interface::{AccountMap, KeyedAccount},
    unstake_it_pool, BaseStakePoolAmm, InitFromKeyedAccount,
};

use crate::{
    find_fee, find_pool_sol_reserves, find_protocol_fee, UnstakeItStakedex, UNSTAKE_IT_LABEL,
};

impl InitFromKeyedAccount for UnstakeItStakedex {
    fn from_keyed_account(_keyed_account: &KeyedAccount) -> Result<Self> {
        Ok(UnstakeItStakedex::default())
    }
}

impl BaseStakePoolAmm for UnstakeItStakedex {
    fn stake_pool_label(&self) -> &'static str {
        UNSTAKE_IT_LABEL
    }

    fn main_state_key(&self) -> Pubkey {
        unstake_it_pool::ID
    }

    fn staked_sol_mint(&self) -> Pubkey {
        spl_token::native_mint::ID
    }

    fn get_accounts_to_update(&self) -> Vec<Pubkey> {
        Vec::from([
            unstake_it_pool::ID,
            find_pool_sol_reserves().0,
            find_fee().0,
            find_protocol_fee().0,
        ])
    }

    fn update(&mut self, accounts_map: &AccountMap) -> Result<()> {
        let pool_data = accounts_map
            .get(&unstake_it_pool::ID)
            .ok_or_else(|| account_missing_err(&unstake_it_pool::ID))?
            .data
            .as_ref();
        self.update_pool(pool_data)?;
        let fee_data = accounts_map
            .get(&find_fee().0)
            .ok_or_else(|| account_missing_err(&find_fee().0))?
            .data
            .as_ref();
        self.update_fee(fee_data)?;
        let protocol_fee_data = accounts_map
            .get(&find_protocol_fee().0)
            .ok_or_else(|| account_missing_err(&find_protocol_fee().0))?
            .data
            .as_ref();
        self.update_protocol_fee(protocol_fee_data)?;
        self.sol_reserves_lamports = accounts_map
            .get(&find_pool_sol_reserves().0)
            .ok_or_else(|| account_missing_err(&find_pool_sol_reserves().0))?
            .lamports;
        Ok(())
    }
}
