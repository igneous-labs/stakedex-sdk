use anyhow::{anyhow, Result};
use solana_program::{
    borsh::try_from_slice_unchecked, clock::Clock, instruction::Instruction, pubkey::Pubkey, stake,
    stake_history::Epoch, system_program, sysvar,
};
use spl_stake_pool::{
    error::StakePoolError,
    find_stake_program_address, find_withdraw_authority_program_address,
    state::{StakePool, StakeStatus, ValidatorList},
};
use stakedex_deposit_sol_interface::{
    spl_stake_pool_deposit_sol_ix, SplStakePoolDepositSolIxArgs, SplStakePoolDepositSolKeys,
    SPL_STAKE_POOL_DEPOSIT_SOL_IX_ACCOUNTS_LEN,
};
use stakedex_deposit_stake_interface::{
    spl_stake_pool_deposit_stake_ix, SplStakePoolDepositStakeIxArgs, SplStakePoolDepositStakeKeys,
    SPL_STAKE_POOL_DEPOSIT_STAKE_IX_ACCOUNTS_LEN,
};
use stakedex_sdk_common::{
    account_missing_err,
    jupiter_stakedex_interface::{AccountMap, KeyedAccount},
    BaseStakePoolAmm, DepositSol, DepositSolQuote, DepositStake, DepositStakeInfo,
    DepositStakeQuote, InitFromKeyedAccount, WithdrawStake, WithdrawStakeQuote,
    STAKE_ACCOUNT_RENT_EXEMPT_LAMPORTS,
};
use stakedex_withdraw_stake_interface::{
    spl_stake_pool_withdraw_stake_ix, SplStakePoolWithdrawStakeIxArgs,
    SplStakePoolWithdrawStakeKeys, SPL_STAKE_POOL_WITHDRAW_STAKE_IX_ACCOUNTS_LEN,
};

use crate::SPL_STAKE_POOL_STATE_TO_LABEL;

#[derive(Clone, Default)]
pub struct SplStakePoolStakedex {
    pub stake_pool_addr: Pubkey,
    pub withdraw_authority_addr: Pubkey,
    pub stake_pool_label: &'static str,
    pub stake_pool: StakePool,
    pub validator_list: ValidatorList,
    pub curr_epoch: Epoch,
}

impl SplStakePoolStakedex {
    pub fn update_stake_pool(&mut self, data: &[u8]) -> Result<()> {
        self.stake_pool = try_from_slice_unchecked::<StakePool>(data)?;
        Ok(())
    }

    pub fn update_validator_list(&mut self, data: &[u8]) -> Result<()> {
        self.validator_list = try_from_slice_unchecked::<ValidatorList>(data)?;
        Ok(())
    }

    pub fn is_updated_this_epoch(&self) -> bool {
        self.stake_pool.last_update_epoch >= self.curr_epoch
    }
}

impl InitFromKeyedAccount for SplStakePoolStakedex {
    /// Initialize from stake pool main account
    fn from_keyed_account(keyed_account: &KeyedAccount) -> Result<Self> {
        let mut res = Self::default();
        res.stake_pool_addr = keyed_account.key;
        res.withdraw_authority_addr =
            find_withdraw_authority_program_address(&spl_stake_pool::ID, &res.stake_pool_addr).0;
        res.stake_pool_label = SPL_STAKE_POOL_STATE_TO_LABEL
            .get(&res.stake_pool_addr)
            .ok_or_else(|| anyhow!("Unknown spl stake pool: {}", res.stake_pool_addr))?;
        res.update_stake_pool(&keyed_account.account.data)?;
        // NOTE: the validator_list is not initialized until self.update() is
        // called for the first time with fetched on-chain data
        Ok(res)
    }
}

impl BaseStakePoolAmm for SplStakePoolStakedex {
    fn stake_pool_label(&self) -> &'static str {
        self.stake_pool_label
    }

    fn main_state_key(&self) -> Pubkey {
        self.stake_pool_addr
    }

    fn staked_sol_mint(&self) -> Pubkey {
        self.stake_pool.pool_mint
    }

    fn get_accounts_to_update(&self) -> Vec<Pubkey> {
        Vec::from([
            self.stake_pool_addr,
            self.stake_pool.validator_list,
            sysvar::clock::ID,
        ])
    }

    fn update(&mut self, accounts_map: &AccountMap) -> Result<()> {
        let stake_pool_data = accounts_map
            .get(&self.stake_pool_addr)
            .ok_or_else(|| account_missing_err(&self.stake_pool_addr))?
            .data
            .as_ref();
        self.update_stake_pool(stake_pool_data)?;
        let validator_list_data = accounts_map
            .get(&self.stake_pool.validator_list)
            .ok_or_else(|| account_missing_err(&self.stake_pool.validator_list))?
            .data
            .as_ref();
        self.update_validator_list(validator_list_data)?;
        let clock_data = accounts_map
            .get(&sysvar::clock::ID)
            .ok_or_else(|| account_missing_err(&sysvar::clock::ID))?
            .data
            .as_ref();
        let clock: Clock = bincode::deserialize(clock_data)?;
        self.curr_epoch = clock.epoch;
        Ok(())
    }
}

impl DepositSol for SplStakePoolStakedex {
    fn can_accept_sol_deposits(&self) -> bool {
        self.is_updated_this_epoch()
    }

    fn get_deposit_sol_quote_unchecked(&self, lamports: u64) -> Result<DepositSolQuote> {
        // Reference: https://github.com/solana-labs/solana-program-library/blob/56cdef9ee82877622a074aa74560742264f20591/stake-pool/program/src/processor.rs#L2268
        let new_pool_tokens = self
            .stake_pool
            .calc_pool_tokens_for_deposit(lamports)
            .ok_or(StakePoolError::CalculationFailure)?;
        let pool_tokens_sol_deposit_fee = self
            .stake_pool
            .calc_pool_tokens_sol_deposit_fee(new_pool_tokens)
            .ok_or(StakePoolError::CalculationFailure)?;
        let pool_tokens_user = new_pool_tokens
            .checked_sub(pool_tokens_sol_deposit_fee)
            .ok_or(StakePoolError::CalculationFailure)?;
        let pool_tokens_referral_fee = self
            .stake_pool
            .calc_pool_tokens_sol_referral_fee(pool_tokens_sol_deposit_fee)
            .ok_or(StakePoolError::CalculationFailure)?;
        // since we set referrer to the receiving fee_token_acc, referral fee is effectively kicked back to user
        let out_amount = pool_tokens_user
            .checked_add(pool_tokens_referral_fee)
            .ok_or(StakePoolError::CalculationFailure)?;
        let fee_amount = pool_tokens_sol_deposit_fee
            .checked_sub(pool_tokens_referral_fee)
            .ok_or(StakePoolError::CalculationFailure)?;
        Ok(DepositSolQuote {
            in_amount: lamports,
            out_amount,
            fee_amount,
        })
    }

    fn virtual_ix(&self) -> Result<Instruction> {
        Ok(spl_stake_pool_deposit_sol_ix(
            SplStakePoolDepositSolKeys {
                spl_stake_pool_program: spl_stake_pool::ID,
                stake_pool: self.stake_pool_addr,
                stake_pool_withdraw_authority: self.withdraw_authority_addr,
                stake_pool_manager_fee: self.stake_pool.manager_fee_account,
                stake_pool_reserve_stake: self.stake_pool.reserve_stake,
            },
            SplStakePoolDepositSolIxArgs {},
        )?)
    }

    fn accounts_len(&self) -> usize {
        SPL_STAKE_POOL_DEPOSIT_SOL_IX_ACCOUNTS_LEN
    }
}

impl DepositStake for SplStakePoolStakedex {
    fn can_accept_stake_deposits(&self) -> bool {
        self.is_updated_this_epoch()
    }

    // TODO: maybe refactor to same style as eversol
    // (_copy() function that returns Result and can copy pasta from on-chain src directly)
    fn get_deposit_stake_quote_unchecked(
        &self,
        withdraw_stake_quote: WithdrawStakeQuote,
    ) -> DepositStakeQuote {
        if let Some(v) = self.stake_pool.preferred_deposit_validator_vote_address {
            if withdraw_stake_quote.voter != v {
                return DepositStakeQuote::default();
            }
        }
        let validator_list_entry = match self.validator_list.find(&withdraw_stake_quote.voter) {
            Some(r) => r,
            None => return DepositStakeQuote::default(),
        };
        if validator_list_entry.status != StakeStatus::Active {
            return DepositStakeQuote::default();
        }
        // Reference: https://github.com/solana-labs/solana-program-library/blob/stake-pool-v0.6.4/stake-pool/program/src/processor.rs#L1971
        let total_deposit_lamports = withdraw_stake_quote.lamports_out;
        let stake_deposit_lamports = withdraw_stake_quote.lamports_staked;

        let new_pool_tokens = match self
            .stake_pool
            .calc_pool_tokens_for_deposit(total_deposit_lamports)
        {
            Some(r) => r,
            None => return DepositStakeQuote::default(),
        };
        let new_pool_tokens_from_stake = match self
            .stake_pool
            .calc_pool_tokens_for_deposit(stake_deposit_lamports)
        {
            Some(r) => r,
            None => return DepositStakeQuote::default(),
        };
        let new_pool_tokens_from_sol = match new_pool_tokens.checked_sub(new_pool_tokens_from_stake)
        {
            Some(r) => r,
            None => return DepositStakeQuote::default(),
        };

        let stake_deposit_fee = match self
            .stake_pool
            .calc_pool_tokens_stake_deposit_fee(new_pool_tokens_from_stake)
        {
            Some(r) => r,
            None => return DepositStakeQuote::default(),
        };
        let sol_deposit_fee = match self
            .stake_pool
            .calc_pool_tokens_sol_deposit_fee(new_pool_tokens_from_sol)
        {
            Some(r) => r,
            None => return DepositStakeQuote::default(),
        };
        let total_fee = match stake_deposit_fee.checked_add(sol_deposit_fee) {
            Some(r) => r,
            None => return DepositStakeQuote::default(),
        };
        let pool_tokens_user = match new_pool_tokens.checked_sub(total_fee) {
            Some(r) => r,
            None => return DepositStakeQuote::default(),
        };
        // since we set referrer to the receiving fee_token_acc, referral fee is effectively kicked back to user
        let pool_tokens_referral_fee = match self
            .stake_pool
            .calc_pool_tokens_stake_referral_fee(total_fee)
        {
            Some(r) => r,
            None => return DepositStakeQuote::default(),
        };
        let tokens_out = match pool_tokens_user.checked_add(pool_tokens_referral_fee) {
            Some(r) => r,
            None => return DepositStakeQuote::default(),
        };
        let fee_amount = match total_fee.checked_sub(pool_tokens_referral_fee) {
            Some(r) => r,
            None => return DepositStakeQuote::default(),
        };

        DepositStakeQuote {
            tokens_out,
            fee_amount,
            voter: withdraw_stake_quote.voter,
        }
    }

    fn virtual_ix(
        &self,
        quote: &DepositStakeQuote,
        _deposit_stake_info: &DepositStakeInfo,
    ) -> Result<Instruction> {
        let deposit_stake_validator_stake =
            find_stake_program_address(&spl_stake_pool::ID, &quote.voter, &self.stake_pool_addr).0;
        Ok(spl_stake_pool_deposit_stake_ix(
            SplStakePoolDepositStakeKeys {
                spl_stake_pool_program: spl_stake_pool::ID,
                deposit_stake_spl_stake_pool: self.stake_pool_addr,
                deposit_stake_validator_list: self.stake_pool.validator_list,
                deposit_stake_deposit_authority: self.stake_pool.stake_deposit_authority,
                deposit_stake_withdraw_authority: self.withdraw_authority_addr,
                deposit_stake_reserve_stake: self.stake_pool.reserve_stake,
                deposit_stake_manager_fee: self.stake_pool.manager_fee_account,
                deposit_stake_validator_stake,
                clock: sysvar::clock::ID,
                stake_history: sysvar::stake_history::ID,
                token_program: spl_token::ID,
                stake_program: stake::program::ID,
            },
            SplStakePoolDepositStakeIxArgs {},
        )?)
    }

    fn accounts_len(&self) -> usize {
        SPL_STAKE_POOL_DEPOSIT_STAKE_IX_ACCOUNTS_LEN
    }
}

impl WithdrawStake for SplStakePoolStakedex {
    fn can_accept_stake_withdrawals(&self) -> bool {
        self.stake_pool.last_update_epoch >= self.curr_epoch
    }

    // TODO: maybe refactor to same style as eversol
    // (_copy() function that returns Result and can copy pasta from on-chain src directly)
    fn get_quote_for_validator_unchecked(
        &self,
        validator_index: usize,
        withdraw_amount: u64,
    ) -> WithdrawStakeQuote {
        let validator_list_entry = self.validator_list.validators.get(validator_index).unwrap();
        // only handle withdrawal from active stake accounts for simplicity.
        // Likely other stake pools can't accept non active stake anyway
        if validator_list_entry.status != StakeStatus::Active {
            return WithdrawStakeQuote::default();
        }
        if let Some(v) = self.stake_pool.preferred_withdraw_validator_vote_address {
            if validator_list_entry.vote_account_address != v {
                return WithdrawStakeQuote::default();
            }
        }
        // Reference: https://github.com/solana-labs/solana-program-library/blob/58c1226a513d3d8bb2de8ec67586a679be7fd2d4/stake-pool/program/src/processor.rs#L2297
        let pool_tokens = withdraw_amount;
        let pool_tokens_fee = match self
            .stake_pool
            .calc_pool_tokens_stake_withdrawal_fee(pool_tokens)
        {
            Some(r) => r,
            None => return WithdrawStakeQuote::default(),
        };
        let pool_tokens_burnt = match pool_tokens.checked_sub(pool_tokens_fee) {
            Some(r) => r,
            None => return WithdrawStakeQuote::default(),
        };
        let withdraw_lamports = match self
            .stake_pool
            .calc_lamports_withdraw_amount(pool_tokens_burnt)
        {
            Some(r) => r,
            None => return WithdrawStakeQuote::default(),
        };
        if withdraw_lamports > validator_list_entry.active_stake_lamports {
            return WithdrawStakeQuote::default();
        }
        let lamports_staked =
            match withdraw_lamports.checked_sub(STAKE_ACCOUNT_RENT_EXEMPT_LAMPORTS) {
                Some(r) => r,
                None => return WithdrawStakeQuote::default(),
            };
        WithdrawStakeQuote {
            lamports_out: withdraw_lamports,
            lamports_staked,
            fee_amount: pool_tokens_fee,
            voter: validator_list_entry.vote_account_address,
        }
    }

    fn is_validator_index_out_of_bounds(&self, validator_index: usize) -> bool {
        validator_index >= self.validator_list.validators.len()
    }

    fn virtual_ix(&self, quote: &WithdrawStakeQuote) -> Result<Instruction> {
        let withdraw_stake_stake_to_split =
            find_stake_program_address(&spl_stake_pool::ID, &quote.voter, &self.stake_pool_addr).0;
        Ok(spl_stake_pool_withdraw_stake_ix(
            SplStakePoolWithdrawStakeKeys {
                spl_stake_pool_program: spl_stake_pool::ID,
                withdraw_stake_spl_stake_pool: self.stake_pool_addr,
                withdraw_stake_validator_list: self.stake_pool.validator_list,
                withdraw_stake_withdraw_authority: self.withdraw_authority_addr,
                withdraw_stake_manager_fee: self.stake_pool.manager_fee_account,
                withdraw_stake_stake_to_split,
                clock: sysvar::clock::ID,
                token_program: spl_token::ID,
                stake_program: stake::program::ID,
                system_program: system_program::ID,
            },
            SplStakePoolWithdrawStakeIxArgs {},
        )?)
    }

    fn accounts_len(&self) -> usize {
        SPL_STAKE_POOL_WITHDRAW_STAKE_IX_ACCOUNTS_LEN
    }
}
