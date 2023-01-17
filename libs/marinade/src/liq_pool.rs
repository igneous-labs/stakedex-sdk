//! Copied from https://github.com/marinade-finance/liquid-staking-program/blob/main/programs/marinade-finance/src/liq_pool.rs

use marinade_finance_interface::LiqPool;
use solana_program::pubkey::Pubkey;
use stakedex_sdk_common::marinade_program;

pub struct LiqPoolWrapper<'a>(pub &'a LiqPool);

impl<'a> LiqPoolWrapper<'a> {
    pub const SOL_LEG_SEED: &'static [u8] = b"liq_sol";
    pub const MSOL_LEG_AUTHORITY_SEED: &'static [u8] = b"liq_st_sol_authority";

    pub fn find_sol_leg_address(state: &Pubkey) -> (Pubkey, u8) {
        Pubkey::find_program_address(
            &[&state.to_bytes()[..32], Self::SOL_LEG_SEED],
            &marinade_program::ID,
        )
    }

    pub fn find_msol_leg_authority(state: &Pubkey) -> (Pubkey, u8) {
        Pubkey::find_program_address(
            &[&state.to_bytes()[..32], Self::MSOL_LEG_AUTHORITY_SEED],
            &marinade_program::ID,
        )
    }
}
