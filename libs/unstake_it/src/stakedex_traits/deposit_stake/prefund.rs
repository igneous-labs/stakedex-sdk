use solana_program::instruction::Instruction;
use stakedex_jup_interface::{PrefundRepayParams, PREFUND_FLASH_LOAN_LAMPORTS};
use stakedex_sdk_common::{DepositStake, DepositStakeInfo, DepositStakeQuote, WithdrawStakeQuote};
use unstake_lib::{PoolBalance, RationalQty};

use crate::{quote_deposit_stake, UnstakeItStakedexPrefund};

impl DepositStake for UnstakeItStakedexPrefund {
    fn can_accept_stake_deposits(&self) -> bool {
        self.0.can_accept_stake_deposits()
    }

    fn get_deposit_stake_quote_unchecked(
        &self,
        withdraw_stake_quote: WithdrawStakeQuote,
    ) -> DepositStakeQuote {
        // modify pool_incoming_stake and sol_reserves_lamports
        // according to the prefund unstake
        let params = PrefundRepayParams {
            fee: self.0.fee.fee.clone(),
            incoming_stake: self.0.pool.incoming_stake,
            sol_reserves_lamports: self.0.sol_reserves_lamports,
            protocol_fee_dest: self.0.protocol_fee.destination,
        };
        let slumdog_target_lamports = match params.slumdog_target_lamports() {
            Ok(s) => s,
            Err(_) => return DepositStakeQuote::default(),
        };
        let slumdog_unstake_fee =
            slumdog_target_lamports.saturating_sub(PREFUND_FLASH_LOAN_LAMPORTS);
        // TODO: make this a method on ProtocolFee
        let protocol_fee = match self.0.protocol_fee.fee_ratio.floor_mul(slumdog_unstake_fee) {
            Some(p) => p,
            None => return DepositStakeQuote::default(),
        };
        let pool_incoming_stake = match self
            .0
            .pool
            .incoming_stake
            .checked_add(slumdog_target_lamports)
        {
            Some(p) => p,
            None => return DepositStakeQuote::default(),
        };
        let sol_reserves_lamports = match self
            .0
            .sol_reserves_lamports
            .checked_sub(PREFUND_FLASH_LOAN_LAMPORTS)
            .and_then(|x| x.checked_sub(protocol_fee))
        {
            Some(s) => s,
            None => return DepositStakeQuote::default(),
        };
        quote_deposit_stake(
            &self.0.fee.fee,
            withdraw_stake_quote,
            PoolBalance {
                pool_incoming_stake,
                sol_reserves_lamports,
            },
        )
    }

    fn virtual_ix(
        &self,
        quote: &DepositStakeQuote,
        deposit_stake_info: &DepositStakeInfo,
    ) -> anyhow::Result<Instruction> {
        // this can be done because `quote` and `deposit_stake_info`
        // isn't used by UnstakeItStakedex::virtual_ix()
        self.0.virtual_ix(quote, deposit_stake_info)
    }

    fn accounts_len(&self) -> usize {
        self.0.accounts_len()
    }
}
