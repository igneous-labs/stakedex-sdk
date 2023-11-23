use anyhow::{anyhow, Result};
use lido::token::Lamports;
use solana_program::instruction::Instruction;
use stakedex_deposit_sol_interface::{
    lido_deposit_sol_ix, LidoDepositSolIxArgs, LidoDepositSolKeys,
};
use stakedex_sdk_common::{lido_program, lido_state, DepositSol, DepositSolQuote};

use crate::LidoStakedex;

impl DepositSol for LidoStakedex {
    fn can_accept_sol_deposits(&self) -> bool {
        true
    }

    fn get_deposit_sol_quote_unchecked(&self, user_lamports: u64) -> Result<DepositSolQuote> {
        let out_amount = self
            .lido_state
            .exchange_rate
            .exchange_sol(Lamports(user_lamports))
            .map_err(|_| anyhow!("math error get_deposit_sol"))?
            .0;
        Ok(DepositSolQuote {
            in_amount: user_lamports,
            out_amount,
            fee_amount: 0,
        })
    }

    fn virtual_ix(&self) -> Result<Instruction> {
        Ok(lido_deposit_sol_ix(
            LidoDepositSolKeys {
                lido_program: lido_program::ID,
                solido: lido_state::ID,
                lido_reserve: self
                    .lido_state
                    .get_reserve_account(&lido_program::ID, &lido_state::ID)?,
                stsol_mint_authority: self
                    .lido_state
                    .get_mint_authority(&lido_program::ID, &lido_state::ID)?,
            },
            LidoDepositSolIxArgs {},
        )?)
    }
}
