use anyhow::Result;
use jupiter_amm_interface::AccountMap;

use solana_program::pubkey::Pubkey;

pub trait BaseStakePoolAmm {
    /// stake pool program ID
    /// NB: this is not necessarily the program to invoke to execute the deposit/withdraw:
    /// e.g. a spl pool behind the deposit cap guard program
    /// will invoke the deposit cap guard program instead
    fn program_id(&self) -> Pubkey;

    fn stake_pool_label(&self) -> &str;

    /// For ID purposes
    fn main_state_key(&self) -> Pubkey;

    /// This is wrapped SOL in the case of unstake.it pool
    fn staked_sol_mint(&self) -> Pubkey;

    fn get_accounts_to_update(&self) -> Vec<Pubkey>;

    fn update(&mut self, account_map: &AccountMap) -> Result<()>;
}
