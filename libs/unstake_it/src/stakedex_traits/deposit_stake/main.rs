use anyhow::Result;
use solana_program::{instruction::Instruction, pubkey::Pubkey, stake, system_program, sysvar};
use stakedex_deposit_stake_interface::{
    unstake_it_deposit_stake_ix, UnstakeItDepositStakeKeys,
    UNSTAKE_IT_DEPOSIT_STAKE_IX_ACCOUNTS_LEN,
};
use stakedex_sdk_common::{
    unstake_it_pool, unstake_it_program, DepositStake, DepositStakeInfo, DepositStakeQuote,
    WithdrawStakeQuote,
};
use unstake_lib::PoolBalance;

use crate::{find_stake_account_record, quote_deposit_stake, UnstakeItStakedex};

impl DepositStake for UnstakeItStakedex {
    fn can_accept_stake_deposits(&self) -> bool {
        true
    }

    fn get_deposit_stake_quote_unchecked(
        &self,
        withdraw_stake_quote: WithdrawStakeQuote,
    ) -> DepositStakeQuote {
        quote_deposit_stake(
            &self.fee.fee,
            withdraw_stake_quote,
            PoolBalance {
                pool_incoming_stake: self.pool.incoming_stake,
                sol_reserves_lamports: self.sol_reserves_lamports,
            },
        )
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

    fn underlying_liquidity(&self) -> Option<&Pubkey> {
        Some(&unstake_it_pool::ID)
    }

    fn accounts_len(&self) -> usize {
        UNSTAKE_IT_DEPOSIT_STAKE_IX_ACCOUNTS_LEN
    }
}
