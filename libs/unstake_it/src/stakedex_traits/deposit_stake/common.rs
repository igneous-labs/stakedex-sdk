use std::cmp::Ordering;

use stakedex_sdk_common::{
    DepositStakeQuote, WithdrawStakeQuote, ZERO_DATA_ACC_RENT_EXEMPT_LAMPORTS,
};
use unstake_interface::FeeEnum;
use unstake_lib::{ApplyFeeArgs, PoolBalance, UnstakeFeeCalc};

pub(crate) fn quote_deposit_stake(
    fee: &FeeEnum,
    withdraw_stake_quote: WithdrawStakeQuote,
    PoolBalance {
        pool_incoming_stake,
        sol_reserves_lamports,
    }: PoolBalance,
) -> DepositStakeQuote {
    let fee_amount = match fee.apply(ApplyFeeArgs {
        pool_balance: PoolBalance {
            pool_incoming_stake,
            sol_reserves_lamports,
        },
        stake_account_lamports: withdraw_stake_quote.lamports_out,
    }) {
        Some(f) => f,
        None => return DepositStakeQuote::default(),
    };
    let tokens_out = withdraw_stake_quote.lamports_out.saturating_sub(fee_amount);
    match tokens_out.cmp(&sol_reserves_lamports) {
        // not enough liquidity
        Ordering::Greater => return DepositStakeQuote::default(),
        Ordering::Less => {
            // cannot leave reserves below rent-exempt min
            if sol_reserves_lamports - tokens_out < ZERO_DATA_ACC_RENT_EXEMPT_LAMPORTS {
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
