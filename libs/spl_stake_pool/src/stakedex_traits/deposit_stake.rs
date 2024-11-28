use anyhow::Result;
use solana_program::{instruction::Instruction, stake, sysvar};
use spl_stake_pool::{find_stake_program_address, state::StakeStatus};
use stakedex_deposit_stake_interface::{
    spl_stake_pool_deposit_stake_ix, SplStakePoolDepositStakeKeys,
    SPL_STAKE_POOL_DEPOSIT_STAKE_IX_ACCOUNTS_LEN,
};
use stakedex_sdk_common::{
    DepositStake, DepositStakeInfo, DepositStakeQuote, WithdrawStakeQuote,
    STAKE_ACCOUNT_RENT_EXEMPT_LAMPORTS,
};

use crate::{
    deposit_cap_guard::{to_deposit_cap_guard_ix, DepositCap},
    SplStakePoolStakedex, SplStakePoolStakedexWithWithdrawSol,
};

impl DepositStake for SplStakePoolStakedex {
    fn can_accept_stake_deposits(&self) -> bool {
        if self.stake_pool.stake_deposit_authority != self.deposit_authority_program_address {
            return false;
        }
        self.is_updated_this_epoch()
    }

    // TODO: maybe refactor to same style as eversol
    // (_copied() function that returns Result and can copy pasta from on-chain src directly)
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
        if validator_list_entry.status != StakeStatus::Active.into() {
            return DepositStakeQuote::default();
        }
        // This is a newly added validator, so the vsa is not yet active.
        // We don't handle depositing to merge with the transient stake account for now.
        if u64::from(validator_list_entry.active_stake_lamports)
            <= STAKE_ACCOUNT_RENT_EXEMPT_LAMPORTS
        {
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

        if self.is_stake_deposit_capped() {
            let deposit_cap = match self.deposit_cap_state.as_ref() {
                Some(d) => d,
                None => return DepositStakeQuote::default(),
            };
            let will_exceed_deposit_cap = match deposit_cap {
                DepositCap::Lamports(max_lamports) => {
                    let new_pool_lamports = self
                        .stake_pool
                        .total_lamports
                        .saturating_add(total_deposit_lamports);
                    new_pool_lamports > *max_lamports
                }
                DepositCap::LstAtomics(max_lst_atomics) => {
                    let new_lst_atomics = self
                        .stake_pool
                        .pool_token_supply
                        .saturating_add(new_pool_tokens);
                    new_lst_atomics > *max_lst_atomics
                }
            };
            if will_exceed_deposit_cap {
                return DepositStakeQuote::default();
            }
        }

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
        let deposit_stake_validator_stake = find_stake_program_address(
            &self.stake_pool_program,
            &quote.voter,
            &self.stake_pool_addr,
            None,
        )
        .0;
        // spl_stake_pool_deposit_stake_ix works for all spl-stake-pool like
        // (spl, sanctum-spl, sanctum-spl-multi) because the accounts interface is the exact same
        let ix = spl_stake_pool_deposit_stake_ix(SplStakePoolDepositStakeKeys {
            spl_stake_pool_program: self.stake_pool_program,
            deposit_stake_spl_stake_pool: self.stake_pool_addr,
            deposit_stake_validator_list: self.stake_pool.validator_list,
            deposit_stake_deposit_authority: self.stake_pool.stake_deposit_authority,
            deposit_stake_withdraw_authority: self.withdraw_authority_addr(),
            deposit_stake_reserve_stake: self.stake_pool.reserve_stake,
            deposit_stake_manager_fee: self.stake_pool.manager_fee_account,
            deposit_stake_validator_stake,
            clock: sysvar::clock::ID,
            stake_history: sysvar::stake_history::ID,
            token_program: spl_token::ID,
            stake_program: stake::program::ID,
        })?;
        Ok(if self.is_stake_deposit_capped() {
            to_deposit_cap_guard_ix(ix, self.spl_deposit_cap_guard_program_address)
        } else {
            ix
        })
    }

    fn accounts_len(&self) -> usize {
        SPL_STAKE_POOL_DEPOSIT_STAKE_IX_ACCOUNTS_LEN
    }
}

impl DepositStake for SplStakePoolStakedexWithWithdrawSol {
    #[inline]
    fn can_accept_stake_deposits(&self) -> bool {
        self.inner.can_accept_stake_deposits()
    }

    #[inline]
    fn get_deposit_stake_quote_unchecked(
        &self,
        withdraw_stake_quote: WithdrawStakeQuote,
    ) -> DepositStakeQuote {
        self.inner
            .get_deposit_stake_quote_unchecked(withdraw_stake_quote)
    }

    #[inline]
    fn virtual_ix(
        &self,
        quote: &DepositStakeQuote,
        deposit_stake_info: &DepositStakeInfo,
    ) -> Result<Instruction> {
        self.inner.virtual_ix(quote, deposit_stake_info)
    }

    #[inline]
    fn accounts_len(&self) -> usize {
        self.inner.accounts_len()
    }
}
