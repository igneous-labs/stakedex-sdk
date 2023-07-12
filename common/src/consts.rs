// hardcode for simplicity. Need to refactor when rent becomes variable.
pub const STAKE_ACCOUNT_RENT_EXEMPT_LAMPORTS: u64 = 2_282_880;

/// TODO: change to 4 when payer is removed
pub const DEPOSIT_STAKE_DST_TOKEN_ACCOUNT_INDEX: usize = 5;

/// TODO: change to 5 when payer is removed
pub const SWAP_VIA_STAKE_SRC_TOKEN_MINT_ACCOUNT_INDEX: usize = 6;

/// TODO: change to 6 when payer is removed
pub const SWAP_VIA_STAKE_DST_TOKEN_MINT_ACCOUNT_INDEX: usize = 7;
