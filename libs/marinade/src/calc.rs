//! Copied from https://github.com/marinade-finance/liquid-staking-program/blob/main/programs/marinade-finance/src/calc.rs

use anyhow::Result;

/// calculate amount*numerator/denominator
/// as value  = shares * share_price where share_price=total_value/total_shares
/// or shares = amount_value / share_price where share_price=total_value/total_shares
///     => shares = amount_value * 1/share_price where 1/share_price=total_shares/total_value
pub fn proportional(amount: u64, numerator: u64, denominator: u64) -> Result<u64> {
    if denominator == 0 {
        return Ok(amount);
    }
    Ok(u64::try_from(
        (amount as u128) * (numerator as u128) / (denominator as u128),
    )?)
}

pub fn shares_from_value(value: u64, total_value: u64, total_shares: u64) -> Result<u64> {
    if total_shares == 0 {
        //no shares minted yet / First mint
        Ok(value)
    } else {
        proportional(value, total_shares, total_value)
    }
}
