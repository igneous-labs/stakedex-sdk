use anyhow::Result;
use solana_program::{
    borsh::try_from_slice_unchecked, clock::Clock, instruction::Instruction, native_token,
    pubkey::Pubkey, stake, stake_history::Epoch, system_program, sysvar,
};
use spl_stake_pool::{
    error::StakePoolError,
    find_stake_program_address, find_withdraw_authority_program_address,
    state::{StakePool, StakeStatus, ValidatorList},
    MINIMUM_ACTIVE_STAKE,
};
use stakedex_deposit_sol_interface::{
    eversol_stake_pool_deposit_sol_ix, EversolStakePoolDepositSolIxArgs,
    EversolStakePoolDepositSolKeys,
};
use stakedex_deposit_stake_interface::{
    eversol_stake_pool_deposit_stake_ix, EversolStakePoolDepositStakeIxArgs,
    EversolStakePoolDepositStakeKeys,
};
use stakedex_sdk_common::{
    account_missing_err, esol, eversol_program, eversol_stake_pool,
    jupiter_stakedex_interface::{AccountMap, KeyedAccount},
    BaseStakePoolAmm, DepositSol, DepositSolQuote, DepositStake, DepositStakeInfo,
    DepositStakeQuote, InitFromKeyedAccount, WithdrawStake, WithdrawStakeQuote,
    STAKE_ACCOUNT_RENT_EXEMPT_LAMPORTS,
};
use stakedex_withdraw_stake_interface::{
    eversol_stake_pool_withdraw_stake_ix, EversolStakePoolWithdrawStakeIxArgs,
    EversolStakePoolWithdrawStakeKeys,
};

use crate::EVERSOL_STAKE_POOL_LABEL;

#[derive(Clone, Default)]
pub struct EversolStakePoolStakedex {
    stake_pool: StakePool,
    validator_list: ValidatorList,
    curr_epoch: Epoch,
}

impl EversolStakePoolStakedex {
    pub fn update_stake_pool(&mut self, data: &[u8]) -> Result<()> {
        self.stake_pool = try_from_slice_unchecked::<StakePool>(data)?;
        Ok(())
    }

    pub fn update_validator_list(&mut self, data: &[u8]) -> Result<()> {
        self.validator_list = try_from_slice_unchecked::<ValidatorList>(data)?;
        Ok(())
    }

    pub fn withdraw_authority() -> Pubkey {
        find_withdraw_authority_program_address(&eversol_program::ID, &eversol_stake_pool::ID).0
    }

    /// Find and return validator stake account
    pub fn vsa(voter: &Pubkey) -> Pubkey {
        find_stake_program_address(&eversol_program::ID, voter, &eversol_stake_pool::ID).0
    }

    pub fn is_updated_this_epoch(&self) -> bool {
        self.stake_pool.last_update_epoch >= self.curr_epoch
    }

    /// Reference (copy-pasta):
    /// https://github.com/everstake/solana-program-library/blob/22534fe3885e698598e92b2fe20da3a8adbfc5ff/stake-pool/program/src/processor.rs#L2309-L2355
    fn get_deposit_stake_quote_copied(
        &self,
        withdraw_stake_quote: WithdrawStakeQuote,
    ) -> Result<DepositStakeQuote, StakePoolError> {
        if let Some(v) = self.stake_pool.preferred_deposit_validator_vote_address {
            if withdraw_stake_quote.voter != v {
                return Err(StakePoolError::InvalidPreferredValidator);
            }
        }
        let validator_list_entry = self
            .validator_list
            .find(&withdraw_stake_quote.voter)
            .ok_or(StakePoolError::ValidatorNotFound)?;
        if validator_list_entry.status != StakeStatus::Active {
            return Err(StakePoolError::InvalidState);
        }
        let total_deposit_lamports = withdraw_stake_quote.lamports_out;
        let stake_deposit_lamports = withdraw_stake_quote.lamports_staked;
        let sol_deposit_lamports = total_deposit_lamports - stake_deposit_lamports;
        let stake_pool = &self.stake_pool;

        let new_pool_tokens = stake_pool
            .convert_amount_of_lamports_to_amount_of_pool_tokens(
                stake_pool
                    .calculate_deposit_amount_by_reward_simulation(total_deposit_lamports)
                    .ok_or(StakePoolError::CalculationFailure)?,
            )
            .ok_or(StakePoolError::CalculationFailure)?;

        let new_pool_tokens_from_deposit_threshold = if stake_pool.no_fee_deposit_threshold > 0
            && native_token::sol_to_lamports(stake_pool.no_fee_deposit_threshold as f64)
                < stake_deposit_lamports
        {
            stake_pool
                .convert_amount_of_lamports_to_amount_of_pool_tokens(native_token::sol_to_lamports(
                    stake_pool.no_fee_deposit_threshold as f64,
                ))
                .ok_or(StakePoolError::CalculationFailure)?
        } else {
            stake_pool
                .convert_amount_of_lamports_to_amount_of_pool_tokens(stake_deposit_lamports)
                .ok_or(StakePoolError::CalculationFailure)?
        };

        let new_pool_tokens_from_sol = stake_pool
            .convert_amount_of_lamports_to_amount_of_pool_tokens(sol_deposit_lamports)
            .ok_or(StakePoolError::CalculationFailure)?;

        let stake_deposit_fee = stake_pool
            .calc_pool_tokens_stake_deposit_fee(new_pool_tokens_from_deposit_threshold)
            .ok_or(StakePoolError::CalculationFailure)?;
        let sol_deposit_fee = stake_pool
            .calc_pool_tokens_sol_deposit_fee(new_pool_tokens_from_sol)
            .ok_or(StakePoolError::CalculationFailure)?;

        let total_fee = stake_deposit_fee
            .checked_add(sol_deposit_fee)
            .ok_or(StakePoolError::CalculationFailure)?;
        let pool_tokens_user = new_pool_tokens
            .checked_sub(total_fee)
            .ok_or(StakePoolError::CalculationFailure)?;

        // eversol doesnt support referrer

        Ok(DepositStakeQuote {
            tokens_out: pool_tokens_user,
            fee_amount: total_fee,
            voter: withdraw_stake_quote.voter,
        })
    }

    /// Reference (copy-pasta):
    /// https://github.com/everstake/solana-program-library/blob/22534fe3885e698598e92b2fe20da3a8adbfc5ff/stake-pool/program/src/processor.rs#L2476-L2489
    fn get_quote_for_validator_copied(
        &self,
        validator_index: usize,
        pool_tokens: u64,
    ) -> Result<WithdrawStakeQuote, StakePoolError> {
        let validator_list_entry = self.validator_list.validators.get(validator_index).unwrap();
        // only handle withdrawal from active stake accounts for simplicity.
        // Likely other stake pools can't accept non active stake anyway
        if validator_list_entry.status != StakeStatus::Active {
            return Err(StakePoolError::InvalidState);
        }
        if let Some(v) = self.stake_pool.preferred_withdraw_validator_vote_address {
            if validator_list_entry.vote_account_address != v {
                return Err(StakePoolError::InvalidPreferredValidator);
            }
        }
        let stake_pool = &self.stake_pool;

        let pool_tokens_fee = stake_pool
            .calc_pool_tokens_stake_withdrawal_fee(pool_tokens)
            .ok_or(StakePoolError::CalculationFailure)?;
        let pool_tokens_burnt = pool_tokens
            .checked_sub(pool_tokens_fee)
            .ok_or(StakePoolError::CalculationFailure)?;

        let withdraw_lamports = stake_pool
            .convert_amount_of_pool_tokens_to_amount_of_lamports(pool_tokens_burnt)
            .ok_or(StakePoolError::CalculationFailure)?;

        if withdraw_lamports
            > validator_list_entry
                .active_stake_lamports
                .checked_sub(MINIMUM_ACTIVE_STAKE)
                .ok_or(StakePoolError::CalculationFailure)?
        {
            return Err(StakePoolError::StakeLamportsNotEqualToMinimum);
        }

        let lamports_staked = withdraw_lamports
            .checked_sub(STAKE_ACCOUNT_RENT_EXEMPT_LAMPORTS)
            .ok_or(StakePoolError::CalculationFailure)?;

        Ok(WithdrawStakeQuote {
            lamports_out: withdraw_lamports,
            lamports_staked,
            fee_amount: pool_tokens_fee,
            voter: validator_list_entry.vote_account_address,
        })
    }
}

impl InitFromKeyedAccount for EversolStakePoolStakedex {
    /// Initialize from stake pool main account
    fn from_keyed_account(keyed_account: &KeyedAccount) -> Result<Self> {
        let mut res = Self::default();
        res.update_stake_pool(&keyed_account.account.data)?;
        // NOTE: the validator_list is not initialized until self.update() is
        // called for the first time with fetched on-chain data
        Ok(res)
    }
}

impl BaseStakePoolAmm for EversolStakePoolStakedex {
    fn stake_pool_label(&self) -> &'static str {
        EVERSOL_STAKE_POOL_LABEL
    }

    fn main_state_key(&self) -> Pubkey {
        eversol_stake_pool::ID
    }

    fn staked_sol_mint(&self) -> Pubkey {
        esol::ID
    }

    fn get_accounts_to_update(&self) -> Vec<Pubkey> {
        Vec::from([
            eversol_stake_pool::ID,
            self.stake_pool.validator_list,
            sysvar::clock::ID,
        ])
    }

    fn update(&mut self, accounts_map: &AccountMap) -> Result<()> {
        let stake_pool_data = accounts_map
            .get(&self.main_state_key())
            .ok_or_else(|| account_missing_err(&self.main_state_key()))?
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

impl DepositSol for EversolStakePoolStakedex {
    fn can_accept_sol_deposits(&self) -> bool {
        self.is_updated_this_epoch()
    }

    /// Reference (copy-pasta):
    /// https://github.com/everstake/solana-program-library/blob/22534fe3885e698598e92b2fe20da3a8adbfc5ff/stake-pool/program/src/processor.rs#L2309-L2355
    fn get_deposit_sol_quote_unchecked(&self, deposit_lamports: u64) -> Result<DepositSolQuote> {
        let new_pool_tokens_wo_idle_fee = self
            .stake_pool
            .convert_amount_of_lamports_to_amount_of_pool_tokens(deposit_lamports)
            .ok_or(StakePoolError::CalculationFailure)?;

        let new_pool_tokens_from_deposit_threshold = if self.stake_pool.no_fee_deposit_threshold > 0
            && native_token::sol_to_lamports(self.stake_pool.no_fee_deposit_threshold as f64)
                < deposit_lamports
        {
            self.stake_pool
                .convert_amount_of_lamports_to_amount_of_pool_tokens(native_token::sol_to_lamports(
                    self.stake_pool.no_fee_deposit_threshold as f64,
                ))
                .ok_or(StakePoolError::CalculationFailure)?
        } else {
            new_pool_tokens_wo_idle_fee
        };

        let new_pool_tokens = self
            .stake_pool
            .calculate_deposit_amount_by_reward_simulation(new_pool_tokens_wo_idle_fee)
            .ok_or(StakePoolError::CalculationFailure)?;

        let pool_tokens_sol_deposit_fee = self
            .stake_pool
            .calc_pool_tokens_sol_deposit_fee(new_pool_tokens_from_deposit_threshold)
            .ok_or(StakePoolError::CalculationFailure)?;
        let pool_tokens_user = new_pool_tokens
            .checked_sub(pool_tokens_sol_deposit_fee)
            .ok_or(StakePoolError::CalculationFailure)?;

        // eversol doesnt support referrer

        Ok(DepositSolQuote {
            in_amount: deposit_lamports,
            out_amount: pool_tokens_user,
            fee_amount: pool_tokens_sol_deposit_fee,
        })
    }

    fn virtual_ix(&self) -> Result<Instruction> {
        Ok(eversol_stake_pool_deposit_sol_ix(
            EversolStakePoolDepositSolKeys {
                eversol_stake_pool_program: eversol_program::ID,
                stake_pool: eversol_stake_pool::ID,
                stake_pool_withdraw_authority: Self::withdraw_authority(),
                stake_pool_manager_fee: self.stake_pool.manager_fee_account,
                stake_pool_reserve_stake: self.stake_pool.reserve_stake,
            },
            EversolStakePoolDepositSolIxArgs {},
        )?)
    }
}

impl DepositStake for EversolStakePoolStakedex {
    fn can_accept_stake_deposits(&self) -> bool {
        self.is_updated_this_epoch()
    }

    /// Reference (copy-pasta):
    /// https://github.com/everstake/solana-program-library/blob/22534fe3885e698598e92b2fe20da3a8adbfc5ff/stake-pool/program/src/processor.rs#L2309-L2355
    fn get_deposit_stake_quote_unchecked(
        &self,
        withdraw_stake_quote: WithdrawStakeQuote,
    ) -> DepositStakeQuote {
        self.get_deposit_stake_quote_copied(withdraw_stake_quote)
            .unwrap_or_default()
    }

    fn virtual_ix(
        &self,
        quote: &DepositStakeQuote,
        _deposit_stake_info: &DepositStakeInfo,
    ) -> Result<Instruction> {
        Ok(eversol_stake_pool_deposit_stake_ix(
            EversolStakePoolDepositStakeKeys {
                eversol_stake_pool_program: eversol_program::ID,
                deposit_stake_spl_stake_pool: eversol_stake_pool::ID,
                deposit_stake_validator_list: self.stake_pool.validator_list,
                deposit_stake_deposit_authority: self.stake_pool.stake_deposit_authority,
                deposit_stake_withdraw_authority: Self::withdraw_authority(),
                deposit_stake_reserve_stake: self.stake_pool.reserve_stake,
                deposit_stake_manager_fee: self.stake_pool.manager_fee_account,
                deposit_stake_validator_stake: Self::vsa(&quote.voter),
                clock: sysvar::clock::ID,
                stake_history: sysvar::stake_history::ID,
                token_program: spl_token::ID,
                stake_program: stake::program::ID,
            },
            EversolStakePoolDepositStakeIxArgs {},
        )?)
    }
}

impl WithdrawStake for EversolStakePoolStakedex {
    fn can_accept_stake_withdrawals(&self) -> bool {
        self.stake_pool.last_update_epoch >= self.curr_epoch
    }

    fn get_quote_for_validator_unchecked(
        &self,
        validator_index: usize,
        withdraw_amount: u64,
    ) -> WithdrawStakeQuote {
        self.get_quote_for_validator_copied(validator_index, withdraw_amount)
            .unwrap_or_default()
    }

    fn is_validator_index_out_of_bounds(&self, validator_index: usize) -> bool {
        validator_index >= self.validator_list.validators.len()
    }

    fn virtual_ix(&self, quote: &WithdrawStakeQuote) -> Result<Instruction> {
        Ok(eversol_stake_pool_withdraw_stake_ix(
            EversolStakePoolWithdrawStakeKeys {
                eversol_stake_pool_program: eversol_program::ID,
                withdraw_stake_spl_stake_pool: eversol_stake_pool::ID,
                withdraw_stake_validator_list: self.stake_pool.validator_list,
                withdraw_stake_withdraw_authority: Self::withdraw_authority(),
                withdraw_stake_manager_fee: self.stake_pool.manager_fee_account,
                withdraw_stake_stake_to_split: Self::vsa(&quote.voter),
                clock: sysvar::clock::ID,
                token_program: spl_token::ID,
                stake_program: stake::program::ID,
                system_program: system_program::ID,
            },
            EversolStakePoolWithdrawStakeIxArgs {},
        )?)
    }
}
