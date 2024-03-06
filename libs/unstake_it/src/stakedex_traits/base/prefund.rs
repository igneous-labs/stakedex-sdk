use jupiter_amm_interface::AccountMap;
use solana_program::pubkey::Pubkey;
use stakedex_sdk_common::BaseStakePoolAmm;

use crate::UnstakeItStakedexPrefund;

impl BaseStakePoolAmm for UnstakeItStakedexPrefund {
    fn program_id(&self) -> Pubkey {
        self.0.program_id()
    }

    fn stake_pool_label(&self) -> &str {
        self.0.stake_pool_label()
    }

    fn main_state_key(&self) -> Pubkey {
        self.0.main_state_key()
    }

    fn staked_sol_mint(&self) -> Pubkey {
        self.0.staked_sol_mint()
    }

    fn get_accounts_to_update(&self) -> Vec<Pubkey> {
        self.0.get_accounts_to_update()
    }

    fn update(&mut self, account_map: &AccountMap) -> anyhow::Result<()> {
        self.0.update(account_map)
    }
}
