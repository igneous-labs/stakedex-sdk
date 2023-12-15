use anyhow::Result;
use solana_program::{clock::Clock, pubkey::Pubkey, sysvar};
use stakedex_sdk_common::{
    account_missing_err, esol, eversol_stake_pool,
    jupiter_stakedex_interface::{AccountMap, KeyedAccount},
    BaseStakePoolAmm, InitFromKeyedAccount,
};

use crate::{EversolStakePoolStakedex, EVERSOL_STAKE_POOL_LABEL};

impl InitFromKeyedAccount for EversolStakePoolStakedex {
    /// Initialize from stake pool main account
    fn from_keyed_account(keyed_account: &KeyedAccount) -> Result<Self> {
        let mut res = Self::default();
        res.update_stake_pool(&keyed_account.account.data)?;
        // NOTE: the validator_list is not initialized until self.update() is
        // called for the first time with fetched on-chain data
        Ok(res)
    }
}

impl BaseStakePoolAmm for EversolStakePoolStakedex {
    fn stake_pool_label(&self) -> &'static str {
        EVERSOL_STAKE_POOL_LABEL
    }

    fn main_state_key(&self) -> Pubkey {
        eversol_stake_pool::ID
    }

    fn staked_sol_mint(&self) -> Pubkey {
        esol::ID
    }

    fn get_accounts_to_update(&self) -> Vec<Pubkey> {
        Vec::from([
            eversol_stake_pool::ID,
            self.stake_pool.validator_list,
            sysvar::clock::ID,
        ])
    }

    fn update(&mut self, accounts_map: &AccountMap) -> Result<()> {
        let stake_pool_data = accounts_map
            .get(&self.main_state_key())
            .ok_or_else(|| account_missing_err(&self.main_state_key()))?
            .data
            .as_ref();
        self.update_stake_pool(stake_pool_data)?;
        let validator_list_data = accounts_map
            .get(&self.stake_pool.validator_list)
            .ok_or_else(|| account_missing_err(&self.stake_pool.validator_list))?
            .data
            .as_ref();
        self.update_validator_list(validator_list_data)?;
        let clock_data = accounts_map
            .get(&sysvar::clock::ID)
            .ok_or_else(|| account_missing_err(&sysvar::clock::ID))?
            .data
            .as_ref();
        let clock: Clock = bincode::deserialize(clock_data)?;
        self.curr_epoch = clock.epoch;
        Ok(())
    }
}
