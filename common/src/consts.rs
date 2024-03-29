// hardcode for simplicity. Need to refactor when rent becomes variable.
pub const STAKE_ACCOUNT_RENT_EXEMPT_LAMPORTS: u64 = 2_282_880;

// hardcode for simplicity. Need to refactor when rent becomes variable.
pub const ZERO_DATA_ACC_RENT_EXEMPT_LAMPORTS: u64 = 890_880;

pub const DEPOSIT_STAKE_DST_TOKEN_ACCOUNT_INDEX: usize = 4;

pub const PREFUND_WITHDRAW_STAKE_SRC_TOKEN_MINT_IDX: usize = 3;

pub const DEPOSIT_STAKE_DST_TOKEN_MINT_IDX: usize = 4;

/// Also applies to PrefundSwapViaStake
pub const SWAP_VIA_STAKE_SRC_TOKEN_MINT_ACCOUNT_INDEX: usize = 5;

/// Also applies to PrefundSwapViaStake
pub const SWAP_VIA_STAKE_DST_TOKEN_MINT_ACCOUNT_INDEX: usize = 6;

pub const TEMPORARY_JUP_AMM_LABEL: &str = "Sanctum";
