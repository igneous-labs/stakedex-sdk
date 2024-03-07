use solana_program::pubkey::Pubkey;
use solana_program::{pubkey::PubkeyError, stake};

pub const FEE_TOKEN_ACCOUNT_SEED_PREFIX: &[u8; 3] = b"fee";

pub const BRIDGE_STAKE_SEED_PREFIX: &[u8; 12] = b"bridge_stake";

pub const SLUMDOG_SEED: &str = "slumdog";

pub fn fee_token_account_seeds(token_mint: &Pubkey) -> [&[u8]; 2] {
    [FEE_TOKEN_ACCOUNT_SEED_PREFIX, token_mint.as_ref()]
}

pub fn deposit_stake_amm_key_seeds(main_state_key: &Pubkey) -> [&[u8]; 1] {
    [main_state_key.as_ref()]
}

pub fn stake_pool_pair_amm_key_seeds<'seeds>(
    pool1: &'seeds Pubkey,
    pool2: &'seeds Pubkey,
) -> [&'seeds [u8]; 2] {
    if pool1 < pool2 {
        [pool1.as_ref(), pool2.as_ref()]
    } else {
        [pool2.as_ref(), pool1.as_ref()]
    }
}

pub fn bridge_stake_seeds<'seeds>(
    user: &'seeds Pubkey,
    bridge_stake_seed_le_bytes: &'seeds [u8; 4],
) -> [&'seeds [u8]; 3] {
    [
        BRIDGE_STAKE_SEED_PREFIX,
        user.as_ref(),
        bridge_stake_seed_le_bytes.as_ref(),
    ]
}

pub fn find_fee_token_acc(mint: &Pubkey) -> (Pubkey, u8) {
    Pubkey::find_program_address(&fee_token_account_seeds(mint), &stakedex_interface::ID)
}

pub fn find_deposit_stake_amm_key(main_state_key: &Pubkey) -> (Pubkey, u8) {
    Pubkey::find_program_address(
        &deposit_stake_amm_key_seeds(main_state_key),
        &stakedex_interface::ID,
    )
}

pub fn find_bridge_stake(user: &Pubkey, bridge_stake_seed_le_bytes: &[u8; 4]) -> (Pubkey, u8) {
    Pubkey::find_program_address(
        &bridge_stake_seeds(user, bridge_stake_seed_le_bytes),
        &stakedex_interface::ID,
    )
}

pub fn find_stake_pool_pair_amm_key(pool1: &Pubkey, pool2: &Pubkey) -> (Pubkey, u8) {
    Pubkey::find_program_address(
        &stake_pool_pair_amm_key_seeds(pool1, pool2),
        &stakedex_interface::ID,
    )
}

pub fn slumdog_stake_create_with_seed(bridge_stake: &Pubkey) -> Result<Pubkey, PubkeyError> {
    Pubkey::create_with_seed(bridge_stake, SLUMDOG_SEED, &stake::program::ID)
}
