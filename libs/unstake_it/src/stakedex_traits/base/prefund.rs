use jupiter_amm_interface::AccountMap;
use solana_program::pubkey::Pubkey;
use stakedex_sdk_common::BaseStakePoolAmm;

use crate::UnstakeItStakedexPrefund;

impl BaseStakePoolAmm for UnstakeItStakedexPrefund {
    #[inline]
    fn program_id(&self) -> Pubkey {
        self.0.program_id()
    }

    #[inline]
    fn stake_pool_label(&self) -> &str {
        self.0.stake_pool_label()
    }

    #[inline]
    fn main_state_key(&self) -> Pubkey {
        self.0.main_state_key()
    }

    #[inline]
    fn staked_sol_mint(&self) -> Pubkey {
        self.0.staked_sol_mint()
    }

    #[inline]
    fn get_accounts_to_update(&self) -> Vec<Pubkey> {
        self.0.get_accounts_to_update()
    }

    #[inline]
    fn update(&mut self, account_map: &AccountMap) -> anyhow::Result<()> {
        self.0.update(account_map)
    }
}
