//! TODO: No longer needed once we open-source unstake.it since we can import it directly

use spl_math::precise_number::PreciseNumber;
use unstake_it_interface::Rational;

pub fn rational_into_precise_number(rational: &Rational) -> Option<PreciseNumber> {
    PreciseNumber::new(rational.num as u128)?
        .checked_div(&PreciseNumber::new(rational.denom as u128)?)
}

pub fn zero_rational() -> Rational {
    Rational { num: 0, denom: 1 }
}
