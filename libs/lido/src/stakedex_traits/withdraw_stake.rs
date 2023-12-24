use anyhow::{anyhow, Result};
use lido::{
    processor::StakeType,
    token::{Lamports, Rational, StLamports},
    MINIMUM_STAKE_ACCOUNT_BALANCE,
};
use solana_program::{
    instruction::Instruction, native_token::LAMPORTS_PER_SOL, stake, system_program, sysvar,
};
use stakedex_sdk_common::{
    lido_program, lido_state, WithdrawStakeBase, WithdrawStakeIter, WithdrawStakeQuote,
    STAKE_ACCOUNT_RENT_EXEMPT_LAMPORTS,
};
use stakedex_withdraw_stake_interface::{lido_withdraw_stake_ix, LidoWithdrawStakeKeys};
use std::ops::Add;

use crate::LidoStakedex;

// Ref: https://github.com/lidofinance/solido/blob/2e017631bdd4a87f19fb0f168cce30b6748031b8/program/src/processor.rs#L916
fn get_withdraw_stake_quote_for_validator_copied(
    lido: &LidoStakedex,
    validator_index: usize,
    withdraw_amount: u64,
) -> Result<WithdrawStakeQuote> {
    let amount = StLamports(withdraw_amount);
    let validator = lido
        .validator_list
        .get(validator_index)
        .ok_or_else(|| anyhow!("Validator record not found"))?;
    // LidoError doesnt impl Error
    let maximum_stake_validator = lido
        .validator_list
        .iter()
        .max_by_key(|v| v.effective_stake_balance)
        .ok_or_else(|| anyhow!("No active validators"))?;
    let maximum_stake_balance = maximum_stake_validator.effective_stake_balance;
    if validator.effective_stake_balance == Lamports(0) {
        return Err(anyhow!("validator has no stake"));
    }
    if validator.effective_stake_balance < maximum_stake_balance {
        return Err(anyhow!("validator with more stake exists"));
    }
    let sol_to_withdraw = lido
        .lido_state
        .exchange_rate
        .exchange_st_sol(amount)
        .map_err(|_| anyhow!("no stSOL minted"))?;
    // TODO: this is = accounts.source_stake_account.lamports()
    // rn because there's only 1 active stake account
    // per validator, might change in the future
    let source_balance = validator.effective_stake_balance;
    let max_withdraw_amount = (source_balance
        * Rational {
            numerator: 1,
            denominator: 10,
        })
    .expect("Multiplying with 0.1 does not overflow or divide by zero.")
    .add(Lamports(10 * LAMPORTS_PER_SOL))
    .map_err(|_| anyhow!("math error max_withdraw_amount"))?;
    if sol_to_withdraw > max_withdraw_amount {
        return Err(anyhow!("withdrawal leaves pool imbalanced"));
    }
    let remaining_balance =
        (source_balance - sol_to_withdraw).map_err(|_| anyhow!("math error remaining_balance"))?;
    if remaining_balance < MINIMUM_STAKE_ACCOUNT_BALANCE {
        return Err(anyhow!("withdrawal leaves stake acc below min"));
    }

    let lamports_out = sol_to_withdraw.0;
    if lamports_out < STAKE_ACCOUNT_RENT_EXEMPT_LAMPORTS {
        return Err(anyhow!("withdrawal too small"));
    }
    let lamports_staked = lamports_out - STAKE_ACCOUNT_RENT_EXEMPT_LAMPORTS;
    Ok(WithdrawStakeQuote {
        lamports_out,
        lamports_staked,
        fee_amount: 0,
        voter: validator.vote_account_address,
    })
}

// Lido only allows withdrawing from largest validator

pub struct WithdrawStakeQuoteIter<'a> {
    pool: &'a LidoStakedex,
    withdraw_amount: u64,
    has_checked_largest_validator: bool,
}

impl<'a> Iterator for WithdrawStakeQuoteIter<'a> {
    type Item = WithdrawStakeQuote;

    fn next(&mut self) -> Option<Self::Item> {
        if self.has_checked_largest_validator {
            return None;
        }

        let (maximum_stake_validator_index, _) = self
            .pool
            .validator_list
            .iter()
            .enumerate()
            .max_by_key(|(_, v)| v.effective_stake_balance)?;
        let wsq = get_withdraw_stake_quote_for_validator_copied(
            self.pool,
            maximum_stake_validator_index,
            self.withdraw_amount,
        )
        .ok()?;

        self.has_checked_largest_validator = true;
        Some(wsq)
    }
}

impl WithdrawStakeIter for LidoStakedex {
    type Iter<'me> = WithdrawStakeQuoteIter<'me>;

    fn withdraw_stake_quote_iter(&self, withdraw_amount: u64) -> Self::Iter<'_> {
        WithdrawStakeQuoteIter {
            pool: self,
            withdraw_amount,
            has_checked_largest_validator: false,
        }
    }
}

impl WithdrawStakeBase for LidoStakedex {
    fn can_accept_stake_withdrawals(&self) -> bool {
        self.lido_state.exchange_rate.computed_in_epoch >= self.curr_epoch
    }

    fn virtual_ix(&self, quote: &WithdrawStakeQuote) -> Result<Instruction> {
        let validator = self
            .validator_list
            .iter()
            .find(|v| v.vote_account_address == quote.voter)
            .ok_or_else(|| anyhow!("could not find validator"))?;
        Ok(lido_withdraw_stake_ix(LidoWithdrawStakeKeys {
            lido_program: lido_program::ID,
            withdraw_stake_solido: lido_state::ID,
            withdraw_stake_stake_authority: lido_program::STAKE_AUTHORITY_ID,
            withdraw_stake_stake_to_split: validator
                .find_stake_account_address(
                    &lido_program::ID,
                    &lido_state::ID,
                    validator.stake_seeds.begin,
                    StakeType::Stake,
                )
                .0,
            withdraw_stake_voter: quote.voter,
            withdraw_stake_validator_list: self.lido_state.validator_list,
            clock: sysvar::clock::ID,
            system_program: system_program::ID,
            stake_program: stake::program::ID,
            token_program: spl_token::ID,
        })?)
    }
}
