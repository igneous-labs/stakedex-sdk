use anyhow::{anyhow, Result};
use jupiter_amm_interface::AccountMap;
use solana_sdk::{account::Account, pubkey::Pubkey};
use stakedex_sdk_common::{
    unstake_it_pool, unstake_it_program, STAKE_ACCOUNT_RENT_EXEMPT_LAMPORTS,
    ZERO_DATA_ACC_RENT_EXEMPT_LAMPORTS,
};
use unstake_interface::{
    Fee, FeeAccount, FeeEnum, Pool, PoolAccount, ProtocolFee, ProtocolFeeAccount,
};
use unstake_lib::{PoolBalance, ReverseFeeArgs, UnstakeFeeCalc};

/// unstakeit pool account data required
/// to give an instant unstake quote in order to power the prefund flash loan
#[derive(Clone, Debug)]
pub struct PrefundRepayParams {
    pub fee: FeeEnum,
    pub incoming_stake: u64,
    pub sol_reserves_lamports: u64,
    pub protocol_fee_dest: Pubkey,
}

impl PrefundRepayParams {
    pub const ACCOUNTS_TO_UPDATE: [Pubkey; 4] = [
        unstake_it_pool::ID,
        unstake_it_program::FEE_ID,
        unstake_it_program::SOL_RESERVES_ID,
        unstake_it_program::PROTOCOL_FEE_ID,
    ];

    pub fn try_init(accounts_map: &AccountMap) -> Result<Self> {
        let fee = extract_fee_enum(accounts_map)?;
        let incoming_stake = extract_incoming_stake(accounts_map)?;
        let sol_reserves_lamports = extract_sol_reserves_lamports(accounts_map)?;
        let protocol_fee_dest = extract_protocol_fee_dest(accounts_map)?;
        Ok(Self {
            fee,
            incoming_stake,
            sol_reserves_lamports,
            protocol_fee_dest,
        })
    }

    pub fn update(&mut self, accounts_map: &AccountMap) -> Result<()> {
        let fee = extract_fee_enum(accounts_map)?;
        let incoming_stake = extract_incoming_stake(accounts_map)?;
        let sol_reserves_lamports = extract_sol_reserves_lamports(accounts_map)?;
        let protocol_fee_dest = extract_protocol_fee_dest(accounts_map)?;
        *self = Self {
            fee,
            incoming_stake,
            sol_reserves_lamports,
            protocol_fee_dest,
        };
        Ok(())
    }

    /// Computes the lamports that must be split off from bridge_stake to slumdog_stake in order to
    /// instant unstake slumdog_stake to repay the prefund flash loan.
    ///
    /// This value is basically a SOL denominated fee for the user and should be subtracted from both
    /// `withdraw_stake_quote.lamports_out` and `withdraw_stake_quote.staked_lamports` before passing `withdraw_stake_quote` to
    /// get_deposit_stake_quote()
    pub fn prefund_split_lamports(&self) -> Result<u64> {
        // TODO: STAKE_ACCOUNT_RENT_EXEMPT_LAMPORTS will change with:
        // - dynamic rent
        // - SOL minimum delegation feature
        let lamports_required = 2 * STAKE_ACCOUNT_RENT_EXEMPT_LAMPORTS;
        if self.sol_reserves_lamports < lamports_required + ZERO_DATA_ACC_RENT_EXEMPT_LAMPORTS {
            return Err(anyhow!("Not enough liquidity for slumdog instant unstake"));
        }
        let slumdog_target_lamports = self
            .fee
            .pseudo_reverse(ReverseFeeArgs {
                pool_balance: PoolBalance {
                    pool_incoming_stake: self.incoming_stake,
                    sol_reserves_lamports: self.sol_reserves_lamports,
                },
                lamports_after_fee: lamports_required,
            })
            .ok_or_else(|| anyhow!("pseudo_reverse() MathError"))?;
        Ok(slumdog_target_lamports.saturating_sub(STAKE_ACCOUNT_RENT_EXEMPT_LAMPORTS))
    }
}

fn extract_fee_enum(accounts_map: &AccountMap) -> Result<FeeEnum> {
    let fee_acc = accounts_map
        .get(&unstake_it_program::FEE_ID)
        .ok_or_else(|| anyhow!("Missing fee account"))?;
    let FeeAccount(Fee { fee }) = FeeAccount::deserialize(&fee_acc.data)?;
    Ok(fee)
}

fn extract_incoming_stake(accounts_map: &AccountMap) -> Result<u64> {
    let pool_acc = accounts_map
        .get(&unstake_it_pool::ID)
        .ok_or_else(|| anyhow!("Missing pool account"))?;
    let PoolAccount(Pool { incoming_stake, .. }) = PoolAccount::deserialize(&pool_acc.data)?;
    Ok(incoming_stake)
}

fn extract_sol_reserves_lamports(accounts_map: &AccountMap) -> Result<u64> {
    let Account {
        lamports: sol_reserves_lamports,
        ..
    } = accounts_map
        .get(&unstake_it_program::SOL_RESERVES_ID)
        .ok_or_else(|| anyhow!("Missing SOL reserves account"))?;
    Ok(*sol_reserves_lamports)
}

fn extract_protocol_fee_dest(accounts_map: &AccountMap) -> Result<Pubkey> {
    let protocol_fee_acc = accounts_map
        .get(&unstake_it_program::PROTOCOL_FEE_ID)
        .ok_or_else(|| anyhow!("Missing protocol fee account"))?;
    let ProtocolFeeAccount(ProtocolFee { destination, .. }) =
        ProtocolFeeAccount::deserialize(&protocol_fee_acc.data)?;
    Ok(destination)
}
