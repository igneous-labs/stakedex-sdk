use solana_program::pubkey::Pubkey;
use stakedex_sdk_common::{unstake_it_pool, unstake_it_program};

pub const FEE_SEED_SUFFIX: &[u8] = b"fee";

pub const PROTOCOL_FEE_SEED: &[u8] = b"protocol-fee";

pub fn find_protocol_fee() -> (Pubkey, u8) {
    Pubkey::find_program_address(&[PROTOCOL_FEE_SEED], &unstake_it_program::ID)
}

pub fn find_fee() -> (Pubkey, u8) {
    Pubkey::find_program_address(
        &[&unstake_it_pool::ID.to_bytes(), FEE_SEED_SUFFIX],
        &unstake_it_program::ID,
    )
}

pub fn find_pool_sol_reserves() -> (Pubkey, u8) {
    Pubkey::find_program_address(&[&unstake_it_pool::ID.to_bytes()], &unstake_it_program::ID)
}

pub fn find_stake_account_record(stake_account: &Pubkey) -> (Pubkey, u8) {
    Pubkey::find_program_address(
        &[&unstake_it_pool::ID.to_bytes(), &stake_account.to_bytes()],
        &unstake_it_program::ID,
    )
}
