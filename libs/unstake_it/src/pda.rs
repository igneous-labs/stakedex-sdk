use solana_program::pubkey::Pubkey;
use stakedex_sdk_common::{unstake_it_pool, unstake_it_program};

pub fn find_stake_account_record(stake_account: &Pubkey) -> (Pubkey, u8) {
    Pubkey::find_program_address(
        &[&unstake_it_pool::ID.to_bytes(), &stake_account.to_bytes()],
        &unstake_it_program::ID,
    )
}
