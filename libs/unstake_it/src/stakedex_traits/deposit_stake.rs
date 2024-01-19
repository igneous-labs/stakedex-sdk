use anyhow::Result;
use solana_program::{instruction::Instruction, stake, system_program, sysvar};
use stakedex_deposit_stake_interface::{unstake_it_deposit_stake_ix, UnstakeItDepositStakeKeys};
use stakedex_sdk_common::{
    unstake_it_pool, unstake_it_program, DepositStake, DepositStakeInfo, DepositStakeQuote,
    WithdrawStakeQuote, ZERO_DATA_ACC_RENT_EXEMPT_LAMPORTS,
};
use std::cmp::Ordering;
use unstake_lib::{ApplyFeeArgs, PoolBalance, UnstakeFeeCalc};

use crate::{find_stake_account_record, UnstakeItStakedex};

impl DepositStake for UnstakeItStakedex {
    fn can_accept_stake_deposits(&self) -> bool {
        true
    }

    fn get_deposit_stake_quote_unchecked(
        &self,
        withdraw_stake_quote: WithdrawStakeQuote,
    ) -> DepositStakeQuote {
        let fee_amount = match self.fee.fee.apply(ApplyFeeArgs {
            pool_balance: PoolBalance {
                pool_incoming_stake: self.pool.incoming_stake,
                sol_reserves_lamports: self.sol_reserves_lamports,
            },
            stake_account_lamports: withdraw_stake_quote.lamports_out,
        }) {
            Some(f) => f,
            None => return DepositStakeQuote::default(),
        };
        let tokens_out = withdraw_stake_quote.lamports_out.saturating_sub(fee_amount);
        match tokens_out.cmp(&self.sol_reserves_lamports) {
            // not enough liquidity
            Ordering::Greater => return DepositStakeQuote::default(),
            Ordering::Less => {
                // cannot leave reserves below rent-exempt min
                if self.sol_reserves_lamports - tokens_out < ZERO_DATA_ACC_RENT_EXEMPT_LAMPORTS {
                    return DepositStakeQuote::default();
                }
            }
            Ordering::Equal => (),
        }
        DepositStakeQuote {
            tokens_out,
            fee_amount,
            voter: withdraw_stake_quote.voter,
        }
    }

    fn virtual_ix(
        &self,
        _quote: &DepositStakeQuote,
        deposit_stake_info: &DepositStakeInfo,
    ) -> Result<Instruction> {
        Ok(unstake_it_deposit_stake_ix(UnstakeItDepositStakeKeys {
            unstakeit_program: unstake_it_program::ID,
            deposit_stake_unstake_pool: unstake_it_pool::ID,
            deposit_stake_pool_sol_reserves: unstake_it_program::SOL_RESERVES_ID,
            deposit_stake_stake_acc_record: find_stake_account_record(&deposit_stake_info.addr).0,
            deposit_stake_unstake_fee: unstake_it_program::FEE_ID,
            deposit_stake_protocol_fee: unstake_it_program::PROTOCOL_FEE_ID,
            deposit_stake_protocol_fee_dest: self.protocol_fee.destination,
            clock: sysvar::clock::ID,
            token_program: spl_token::ID,
            stake_program: stake::program::ID,
            system_program: system_program::ID,
        })?)
    }
}
