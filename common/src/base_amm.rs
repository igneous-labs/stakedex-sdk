use anyhow::Result;
use std::collections::HashMap;

use solana_program::pubkey::Pubkey;

pub trait BaseStakePoolAmm {
    fn stake_pool_label(&self) -> &'static str;

    /// For ID purposes
    fn main_state_key(&self) -> Pubkey;

    /// This is wrapped SOL in the case of unstake.it pool
    fn staked_sol_mint(&self) -> Pubkey;

    fn get_accounts_to_update(&self) -> Vec<Pubkey>;

    fn update(&mut self, accounts_map: &HashMap<Pubkey, Vec<u8>>) -> Result<()>;
}
