pub const GLOBAL_FEE_BPS: u64 = 1;

pub struct AfterFees {
    pub fee: u64,
    pub remainder: u64,
}

pub fn apply_global_fee(amount: u64) -> AfterFees {
    // cast-safety: GLOBAL_FEE_BPS is < 10_000, no overflow should happen
    let fee = ((amount as u128) * (GLOBAL_FEE_BPS as u128) / 10_000u128) as u64;
    let fee = fee.max(1);
    let remainder = amount - fee;
    AfterFees { fee, remainder }
}
