//! Current fees:
//! - no fees on StakeWrappedSol
//! - no fees on (Prefund)SwapViaStake/DepositStake if output mint is SOL (via reserve pool)
//! - no fees on (Prefund)WithdrawStake
//! - 1 bps fee on WithdrawWrappedSol
//! - 10 bps fee on everything else (Prefund)SwapViaStake (charged via DepositStake) for LST -> LST

pub const WITHDRAW_WRAPPED_SOL_STAKEDEX_FEE_BPS: u64 = 1;
pub const DEPOSIT_STAKE_STAKEDEX_FEE_BPS: u64 = 10;

pub struct AfterFees {
    pub fee: u64,
    pub remainder: u64,
}

pub fn apply_withdraw_wrapped_sol_stakedex_fee(amount: u64) -> AfterFees {
    apply_fee_bps(amount, WITHDRAW_WRAPPED_SOL_STAKEDEX_FEE_BPS)
}

pub fn apply_deposit_stake_stakedex_fee(amount: u64) -> AfterFees {
    apply_fee_bps(amount, DEPOSIT_STAKE_STAKEDEX_FEE_BPS)
}

fn apply_fee_bps(amount: u64, bps: u64) -> AfterFees {
    // cast-safety: bps must be < 10_000, s.t. no overflow should happen
    let fee = ((amount as u128) * (bps as u128) / 10_000u128) as u64;
    let fee = fee.max(1);
    let remainder = amount - fee;
    AfterFees { fee, remainder }
}
