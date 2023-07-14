use anyhow::Result;
use solana_program::{
    borsh::try_from_slice_unchecked, instruction::Instruction, pubkey::Pubkey, stake,
    system_program, sysvar,
};
use stakedex_deposit_stake_interface::{
    unstake_it_deposit_stake_ix, UnstakeItDepositStakeIxArgs, UnstakeItDepositStakeKeys,
};
use stakedex_sdk_common::{
    account_missing_err,
    jupiter_stakedex_interface::{AccountMap, KeyedAccount},
    unstake_it_pool, unstake_it_program, BaseStakePoolAmm, DepositStake, DepositStakeInfo,
    DepositStakeQuote, InitFromKeyedAccount, WithdrawStakeQuote,
};
use unstake_it_interface::{Fee, FeeEnum, Pool, ProtocolFee};

use crate::{
    apply_fee, find_fee, find_pool_sol_reserves, find_protocol_fee, find_stake_account_record,
    zero_rational, UNSTAKE_IT_LABEL,
};

#[derive(Clone)]
pub struct UnstakeItStakedex {
    pool: Pool,
    fee: Fee,
    protocol_fee: ProtocolFee,
    sol_reserves_lamports: u64,
}

impl Default for UnstakeItStakedex {
    fn default() -> Self {
        Self {
            pool: Pool {
                fee_authority: Pubkey::default(),
                lp_mint: Pubkey::default(),
                incoming_stake: u64::default(),
            },
            fee: Fee {
                fee: FeeEnum::Flat {
                    ratio: zero_rational(),
                },
            },
            protocol_fee: ProtocolFee {
                destination: Pubkey::default(),
                authority: Pubkey::default(),
                fee_ratio: zero_rational(),
                referrer_fee_ratio: zero_rational(),
            },
            sol_reserves_lamports: u64::default(),
        }
    }
}

impl UnstakeItStakedex {
    // All update methods dont check account discm

    pub fn update_pool(&mut self, data: &[u8]) -> Result<()> {
        self.pool = try_from_slice_unchecked::<Pool>(&data[8..])?;
        Ok(())
    }

    pub fn update_fee(&mut self, data: &[u8]) -> Result<()> {
        self.fee = try_from_slice_unchecked::<Fee>(&data[8..])?;
        Ok(())
    }

    pub fn update_protocol_fee(&mut self, data: &[u8]) -> Result<()> {
        self.protocol_fee = try_from_slice_unchecked::<ProtocolFee>(&data[8..])?;
        Ok(())
    }
}

impl InitFromKeyedAccount for UnstakeItStakedex {
    fn from_keyed_account(_keyed_account: &KeyedAccount) -> Result<Self> {
        Ok(UnstakeItStakedex::default())
    }
}

impl BaseStakePoolAmm for UnstakeItStakedex {
    fn stake_pool_label(&self) -> &'static str {
        UNSTAKE_IT_LABEL
    }

    fn main_state_key(&self) -> Pubkey {
        unstake_it_pool::ID
    }

    fn staked_sol_mint(&self) -> Pubkey {
        spl_token::native_mint::ID
    }

    fn get_accounts_to_update(&self) -> Vec<Pubkey> {
        Vec::from([
            unstake_it_pool::ID,
            find_pool_sol_reserves().0,
            find_fee().0,
            find_protocol_fee().0,
        ])
    }

    fn update(&mut self, accounts_map: &AccountMap) -> Result<()> {
        let pool_data = accounts_map
            .get(&unstake_it_pool::ID)
            .ok_or_else(|| account_missing_err(&unstake_it_pool::ID))?
            .data
            .as_ref();
        self.update_pool(pool_data)?;
        let fee_data = accounts_map
            .get(&find_fee().0)
            .ok_or_else(|| account_missing_err(&find_fee().0))?
            .data
            .as_ref();
        self.update_fee(fee_data)?;
        let protocol_fee_data = accounts_map
            .get(&find_protocol_fee().0)
            .ok_or_else(|| account_missing_err(&find_protocol_fee().0))?
            .data
            .as_ref();
        self.update_protocol_fee(protocol_fee_data)?;
        Ok(())
    }
}

impl DepositStake for UnstakeItStakedex {
    fn can_accept_stake_deposits(&self) -> bool {
        true
    }

    fn get_deposit_stake_quote_unchecked(
        &self,
        withdraw_stake_quote: WithdrawStakeQuote,
    ) -> DepositStakeQuote {
        let fee_amount = match apply_fee(
            &self.fee.fee,
            self.pool.incoming_stake,
            self.sol_reserves_lamports,
            withdraw_stake_quote.lamports_out,
        ) {
            Some(f) => f,
            None => return DepositStakeQuote::default(),
        };
        let tokens_out = withdraw_stake_quote.lamports_out - fee_amount;
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
        Ok(unstake_it_deposit_stake_ix(
            UnstakeItDepositStakeKeys {
                unstakeit_program: unstake_it_program::ID,
                deposit_stake_unstake_pool: unstake_it_pool::ID,
                deposit_stake_pool_sol_reserves: find_pool_sol_reserves().0,
                deposit_stake_stake_acc_record: find_stake_account_record(&deposit_stake_info.addr)
                    .0,
                deposit_stake_unstake_fee: find_fee().0,
                deposit_stake_protocol_fee: find_protocol_fee().0,
                deposit_stake_protocol_fee_dest: self.protocol_fee.destination,
                clock: sysvar::clock::ID,
                token_program: spl_token::ID,
                stake_program: stake::program::ID,
                system_program: system_program::ID,
            },
            UnstakeItDepositStakeIxArgs {},
        )?)
    }
}
