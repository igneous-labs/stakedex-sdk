use anyhow::{anyhow, Result};
use jupiter_core::amm::KeyedAccount;
use lido::{
    processor::StakeType,
    token::{Lamports, Rational, StLamports},
    MINIMUM_STAKE_ACCOUNT_BALANCE,
};
use solana_program::{
    instruction::Instruction, native_token::LAMPORTS_PER_SOL, pubkey::Pubkey, stake,
    system_program, sysvar,
};
use stakedex_deposit_sol_interface::{
    lido_deposit_sol_ix, LidoDepositSolIxArgs, LidoDepositSolKeys,
};
use stakedex_sdk_common::{
    account_missing_err, lido_program, lido_state, stsol, BaseStakePoolAmm, DepositSol,
    DepositSolQuote, InitFromKeyedAccount, WithdrawStake, WithdrawStakeQuote,
    STAKE_ACCOUNT_RENT_EXEMPT_LAMPORTS,
};
use stakedex_withdraw_stake_interface::{
    lido_withdraw_stake_ix, LidoWithdrawStakeIxArgs, LidoWithdrawStakeKeys,
};
use std::{collections::HashMap, ops::Add};

use crate::{LidoStakedex, LIDO_LABEL};

impl InitFromKeyedAccount for LidoStakedex {
    /// Initialize from lido
    fn from_keyed_account(keyed_account: &KeyedAccount) -> Result<Self> {
        let mut res = Self::default();
        res.update_lido_state(&keyed_account.account.data)?;
        // NOTE: validator_list is not initialized until self.update() is
        // called for the first time with fetched on-chain data
        Ok(res)
    }
}

impl BaseStakePoolAmm for LidoStakedex {
    fn stake_pool_label(&self) -> &'static str {
        LIDO_LABEL
    }

    fn main_state_key(&self) -> Pubkey {
        lido_state::ID
    }

    fn staked_sol_mint(&self) -> Pubkey {
        stsol::ID
    }

    fn get_accounts_to_update(&self) -> Vec<Pubkey> {
        vec![
            lido_state::ID,
            self.lido_state.validator_list,
            sysvar::clock::ID,
        ]
    }

    fn update(&mut self, accounts_map: &HashMap<Pubkey, Vec<u8>>) -> Result<()> {
        let state_data = accounts_map
            .get(&lido_state::ID)
            .ok_or_else(|| account_missing_err(&lido_state::ID))?;
        self.update_lido_state(state_data)?;
        let validator_list_data = accounts_map
            .get(&self.lido_state.validator_list)
            .ok_or_else(|| account_missing_err(&self.lido_state.validator_list))?;
        self.update_validator_list(validator_list_data)?;
        let clock_data = accounts_map
            .get(&sysvar::clock::ID)
            .ok_or_else(|| account_missing_err(&sysvar::clock::ID))?;
        self.update_curr_epoch(clock_data)?;
        Ok(())
    }
}

// Ref: https://github.com/lidofinance/solido/blob/2e017631bdd4a87f19fb0f168cce30b6748031b8/program/src/processor.rs#L916
fn get_quote_for_validator_copied(
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
    let lamports_staked = lamports_out - STAKE_ACCOUNT_RENT_EXEMPT_LAMPORTS;
    Ok(WithdrawStakeQuote {
        lamports_out,
        lamports_staked,
        fee_amount: 0,
        voter: validator.vote_account_address,
    })
}

impl WithdrawStake for LidoStakedex {
    fn is_validator_index_out_of_bounds(&self, validator_index: usize) -> bool {
        validator_index >= self.validator_list.len()
    }

    fn can_accept_stake_withdrawals(&self) -> bool {
        self.lido_state.exchange_rate.computed_in_epoch >= self.curr_epoch
    }

    fn get_quote_for_validator_unchecked(
        &self,
        validator_index: usize,
        withdraw_amount: u64,
    ) -> WithdrawStakeQuote {
        get_quote_for_validator_copied(self, validator_index, withdraw_amount).unwrap_or_default()
    }

    fn virtual_ix(&self, quote: &WithdrawStakeQuote) -> Result<Instruction> {
        let validator = self
            .validator_list
            .iter()
            .find(|v| v.vote_account_address == quote.voter)
            .ok_or_else(|| anyhow!("could not find validator"))?;
        Ok(lido_withdraw_stake_ix(
            LidoWithdrawStakeKeys {
                lido_program: lido_program::ID,
                withdraw_stake_solido: lido_state::ID,
                withdraw_stake_stake_authority: self
                    .lido_state
                    .get_stake_authority(&lido_program::ID, &lido_state::ID)?,
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
            },
            LidoWithdrawStakeIxArgs {},
        )?)
    }
}

impl DepositSol for LidoStakedex {
    fn get_deposit_sol_quote(&self, user_lamports: u64) -> Result<DepositSolQuote> {
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
