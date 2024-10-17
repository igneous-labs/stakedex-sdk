//! Stake pools that accept direct SOL deposits or withdrawals

mod deposit_sol;
// mod deposit_withdraw_sol;
mod withdraw_sol;

pub use deposit_sol::*;
// pub use deposit_withdraw_sol::*;
pub use withdraw_sol::*;
