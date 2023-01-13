//! TODO: No longer needed once we open-source unstake.it since we can import it directly

use spl_math::precise_number::PreciseNumber;
use unstake_it_interface::FeeEnum;

use crate::rational_into_precise_number;

pub fn apply_fee(
    fee_enum: &FeeEnum,
    pool_incoming_stake: u64,
    sol_reserves_lamports: u64,
    stake_account_lamports: u64,
) -> Option<u64> {
    let fee_ratio = match fee_enum {
        FeeEnum::Flat { ratio } => rational_into_precise_number(ratio)?,
        FeeEnum::LiquidityLinear { params } => {
            // linear interpolation from max_liq_remaining to zero_liq_remaining where y-intercept at max_liq_remaining
            // x-axis is liquidity consumed in lamports
            // y-axis is fee ratio (e.g. 0.01 is 1% fees)
            //
            // let I = pool_incoming_stake, S = stake_account_lamports,
            // m = slope, c = y-intercept at max_liq_remaining
            // new liquidity consumed after unstake = I + (1 - y)S
            // y = m(I + (1 - y)S) + c
            // y = mI + mS - mSy + c
            // y(1 + mS) = m(I + S) + c
            // y = (m(I + S) + c) / (1 + mS)
            //
            // since m <<< 1, use 1/m where possible to preserve precision
            // y = m(I + S + c/m) / m(1/m + S)
            // y = (I + S + c/m) / (1/m + S)
            // TODO: check overflow conditions due to large numbers
            //
            // note: fee_ratio can go >zero_liq_remaining
            // if I + (1 - y)S > pool_owned_lamports

            let zero_liq_fee = rational_into_precise_number(&params.zero_liq_remaining)?;
            let max_liq_fee = rational_into_precise_number(&params.max_liq_remaining)?;
            let owned_lamports =
                (pool_incoming_stake as u128).checked_add(sol_reserves_lamports as u128)?;

            let slope_num = zero_liq_fee.checked_sub(&max_liq_fee)?;
            let slope_denom = PreciseNumber::new(owned_lamports)?;

            let incoming_plus_stake =
                (pool_incoming_stake as u128).checked_add(stake_account_lamports as u128)?;
            let num = slope_denom
                .checked_mul(&max_liq_fee)?
                .checked_div(&slope_num)?
                .checked_add(&PreciseNumber::new(incoming_plus_stake)?)?;
            let denom = slope_denom
                .checked_div(&slope_num)?
                .checked_add(&PreciseNumber::new(stake_account_lamports as u128)?)?;
            num.checked_div(&denom)?
        }
    };

    PreciseNumber::new(stake_account_lamports as u128)?
        .checked_mul(&fee_ratio)?
        .ceiling()?
        .to_imprecise()
        .and_then(|v| u64::try_from(v).ok())
}
