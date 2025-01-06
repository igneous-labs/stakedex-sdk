use anyhow::Result;
use solana_program::{borsh1::try_from_slice_unchecked, pubkey::Pubkey};
use unstake_interface::{Fee, FeeEnum, Pool, ProtocolFee, Rational};

mod pda;
mod stakedex_traits;

pub use pda::*;

use stakedex_traits::*;

pub const UNSTAKE_IT_LABEL: &str = "Unstake.it";

pub const ZERO_RATIONAL: Rational = Rational { num: 0, denom: 1 };

#[derive(Clone)]
pub struct UnstakeItStakedex {
    pub pool: Pool,
    pub fee: Fee,
    pub protocol_fee: ProtocolFee,
    pub sol_reserves_lamports: u64,
}

impl Default for UnstakeItStakedex {
    fn default() -> Self {
        Self {
            pool: Pool {
                fee_authority: Pubkey::default(),
                lp_mint: Pubkey::default(),
                incoming_stake: u64::default(),
            },
            fee: Fee {
                fee: FeeEnum::Flat {
                    ratio: ZERO_RATIONAL,
                },
            },
            protocol_fee: ProtocolFee {
                destination: Pubkey::default(),
                authority: Pubkey::default(),
                fee_ratio: ZERO_RATIONAL,
                referrer_fee_ratio: ZERO_RATIONAL,
            },
            sol_reserves_lamports: u64::default(),
        }
    }
}

impl UnstakeItStakedex {
    // All update methods dont check account discm

    pub fn update_pool(&mut self, data: &[u8]) -> Result<()> {
        self.pool = try_from_slice_unchecked::<Pool>(&data[8..])?;
        Ok(())
    }

    pub fn update_fee(&mut self, data: &[u8]) -> Result<()> {
        self.fee = try_from_slice_unchecked::<Fee>(&data[8..])?;
        Ok(())
    }

    pub fn update_protocol_fee(&mut self, data: &[u8]) -> Result<()> {
        self.protocol_fee = try_from_slice_unchecked::<ProtocolFee>(&data[8..])?;
        Ok(())
    }
}

#[derive(Clone, Default)]
pub struct UnstakeItStakedexPrefund(pub UnstakeItStakedex);
