use solana_program::pubkey::Pubkey;

pub const WSOL_BRIDGE_IN_SEED: &str = "wsol_bridge_in";

pub const SOL_BRIDGE_OUT_SEED: &[u8; 14] = b"sol_bridge_out";

pub const FEE_TOKEN_ACCOUNT_SEED_PREFIX: &[u8; 3] = b"fee";

// TODO: uncomment for SwapViaStake
// pub const BRIDGE_STAKE_SEED_PREFIX: &[u8; 12] = b"bridge_stake";

pub fn sol_bridge_out_seeds() -> [&'static [u8]; 1] {
    [SOL_BRIDGE_OUT_SEED]
}

pub fn fee_token_account_seeds(token_mint: &Pubkey) -> [&[u8]; 2] {
    [FEE_TOKEN_ACCOUNT_SEED_PREFIX, token_mint.as_ref()]
}

pub fn deposit_stake_amm_key_seeds(main_state_key: &Pubkey) -> [&[u8]; 1] {
    [main_state_key.as_ref()]
}

// TODO: uncomment for SwapViaStake
/*
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
*/

pub fn cws_wsol_bridge_in(sol_bridge_out: &Pubkey) -> Pubkey {
    // unwrap() safety:
    // - MaxSeedLengthExceeded: WSOL_BRIDGE_IN_SEED is safe
    // - InvalidSeeds: tested with program key, is safe
    // - IllegalOwner: spl_token::ID is not illegal
    Pubkey::create_with_seed(sol_bridge_out, WSOL_BRIDGE_IN_SEED, &spl_token::ID).unwrap()
}

pub fn find_sol_bridge_out() -> (Pubkey, u8) {
    Pubkey::find_program_address(&sol_bridge_out_seeds(), &stakedex_interface::ID)
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
