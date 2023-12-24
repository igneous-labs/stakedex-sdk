use anyhow::Result;
use solana_program::instruction::Instruction;
use stakedex_deposit_sol_interface::{marinade_deposit_sol_ix, MarinadeDepositSolKeys};
use stakedex_sdk_common::{marinade_program, marinade_state, DepositSol, DepositSolQuote};

use crate::{state::StateWrapper, MarinadeStakedex};

impl DepositSol for MarinadeStakedex {
    fn can_accept_sol_deposits(&self) -> bool {
        true
    }

    fn get_deposit_sol_quote_unchecked(&self, user_lamports: u64) -> Result<DepositSolQuote> {
        // Reference: https://github.com/marinade-finance/liquid-staking-program/blob/main/programs/marinade-finance/src/state/deposit.rs#L27
        let out_amount = StateWrapper(&self.state).calc_msol_from_lamports(user_lamports)?;
        // TODO: this is a simplified calc that doesn't account for the liquidity pool, which can result in a diff of 1 lamport
        Ok(DepositSolQuote {
            in_amount: user_lamports,
            out_amount,
            fee_amount: 0,
        })
    }

    fn virtual_ix(&self) -> Result<Instruction> {
        Ok(marinade_deposit_sol_ix(MarinadeDepositSolKeys {
            marinade_program: marinade_program::ID,
            marinade_state: marinade_state::ID,
            msol_mint_authority: marinade_program::MSOL_MINT_AUTH_ID,
            marinade_reserve: marinade_program::RESERVE_ID,
            marinade_liq_pool_msol_leg: self.state.liq_pool.msol_leg,
            marinade_liq_pool_msol_leg_auth: marinade_program::LIQ_POOL_MSOL_LEG_AUTHORITY_ID,
            marinade_liq_pool_sol_leg: marinade_program::LIQ_POOL_SOL_LEG_ID,
        })?)
    }
}
