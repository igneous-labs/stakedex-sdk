use anyhow::Result;
use solana_program::{clock::Clock, pubkey::Pubkey, sysvar};
use stakedex_sdk_common::{
    account_missing_err,
    jupiter_stakedex_interface::{AccountMap, KeyedAccount},
    BaseStakePoolAmm, InitFromKeyedAccount,
};

use crate::SplStakePoolStakedex;

impl InitFromKeyedAccount for SplStakePoolStakedex {
    /// Initialize from stake pool main account
    fn from_keyed_account(
        KeyedAccount {
            key,
            account,
            params,
        }: &KeyedAccount,
    ) -> Result<Self> {
        let mut res = Self {
            stake_pool_program: account.owner,
            stake_pool_addr: *key,
            ..Default::default()
        };

        res.update_stake_pool(&account.data)?;

        res.stake_pool_label = params
            .as_ref()
            .map_or_else(|| None, |v| v.as_str())
            .map_or_else(
                || format!("{} stake pool", res.stake_pool.pool_mint),
                |token_name| format!("{token_name} stake pool"),
            );
        // NOTE: the validator_list is not initialized until self.update() is
        // called for the first time with fetched on-chain data
        Ok(res)
    }
}

impl BaseStakePoolAmm for SplStakePoolStakedex {
    fn stake_pool_label(&self) -> &str {
        &self.stake_pool_label
    }

    fn main_state_key(&self) -> Pubkey {
        self.stake_pool_addr
    }

    fn staked_sol_mint(&self) -> Pubkey {
        self.stake_pool.pool_mint
    }

    fn get_accounts_to_update(&self) -> Vec<Pubkey> {
        Vec::from([
            self.stake_pool_addr,
            self.stake_pool.validator_list,
            sysvar::clock::ID,
        ])
    }

    fn update(&mut self, accounts_map: &AccountMap) -> Result<()> {
        let stake_pool_data = accounts_map
            .get(&self.stake_pool_addr)
            .ok_or_else(|| account_missing_err(&self.stake_pool_addr))?
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
