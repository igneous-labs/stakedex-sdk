use anyhow::Result;
use jupiter_amm_interface::{AccountMap, AmmContext, KeyedAccount};
use solana_program::pubkey::Pubkey;
use stakedex_sdk_common::{
    account_missing_err, unstake_it_pool, unstake_it_program, BaseStakePoolAmm,
    InitFromKeyedAccount,
};

use crate::{UnstakeItStakedex, UNSTAKE_IT_LABEL};

impl InitFromKeyedAccount for UnstakeItStakedex {
    /// Not usable until first update()
    #[inline]
    fn from_keyed_account(
        _keyed_account: &KeyedAccount,
        _amm_context: &AmmContext,
    ) -> Result<Self> {
        Ok(UnstakeItStakedex::default())
    }
}

impl BaseStakePoolAmm for UnstakeItStakedex {
    #[inline]
    fn program_id(&self) -> Pubkey {
        unstake_it_program::ID
    }

    #[inline]
    fn stake_pool_label(&self) -> &'static str {
        UNSTAKE_IT_LABEL
    }

    #[inline]
    fn main_state_key(&self) -> Pubkey {
        unstake_it_pool::ID
    }

    #[inline]
    fn staked_sol_mint(&self) -> Pubkey {
        spl_token::native_mint::ID
    }

    #[inline]
    fn get_accounts_to_update(&self) -> Vec<Pubkey> {
        Vec::from([
            unstake_it_pool::ID,
            unstake_it_program::SOL_RESERVES_ID,
            unstake_it_program::FEE_ID,
            unstake_it_program::PROTOCOL_FEE_ID,
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
            .get(&unstake_it_program::FEE_ID)
            .ok_or_else(|| account_missing_err(&unstake_it_program::FEE_ID))?
            .data
            .as_ref();
        self.update_fee(fee_data)?;
        let protocol_fee_data = accounts_map
            .get(&unstake_it_program::PROTOCOL_FEE_ID)
            .ok_or_else(|| account_missing_err(&unstake_it_program::PROTOCOL_FEE_ID))?
            .data
            .as_ref();
        self.update_protocol_fee(protocol_fee_data)?;
        self.sol_reserves_lamports = accounts_map
            .get(&unstake_it_program::SOL_RESERVES_ID)
            .ok_or_else(|| account_missing_err(&unstake_it_program::SOL_RESERVES_ID))?
            .lamports;
        Ok(())
    }
}
