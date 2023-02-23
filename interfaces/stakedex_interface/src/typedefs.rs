use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::pubkey::Pubkey;
#[derive(Clone, Debug, BorshDeserialize, BorshSerialize)]
pub struct DexRecordDepositSol {
    pub ty: DepositSolType,
    pub mint: Pubkey,
    pub main_account: Pubkey,
}
#[derive(Clone, Debug, BorshDeserialize, BorshSerialize)]
pub struct DexRecordOneWayPoolPair {
    pub withdraw_stake_ty: WithdrawStakeType,
    pub deposit_stake_ty: DepositStakeType,
    pub withdraw_stake_mint: Pubkey,
    pub deposit_stake_mint: Pubkey,
    pub withdraw_stake_main_account: Pubkey,
    pub deposit_stake_main_account: Pubkey,
}
#[derive(Clone, Debug, BorshDeserialize, BorshSerialize)]
pub struct DexRecordTwoWayPoolPair {
    pub a_ty: DepositWithdrawStakeType,
    pub b_ty: DepositWithdrawStakeType,
    pub a_mint: Pubkey,
    pub b_mint: Pubkey,
    pub a_main_account: Pubkey,
    pub b_main_account: Pubkey,
}
#[derive(Clone, Debug, BorshDeserialize, BorshSerialize)]
pub struct StakeWrappedSolArgs {
    pub amount: u64,
}
#[derive(Clone, Debug, BorshDeserialize, BorshSerialize)]
pub struct SwapViaStakeArgs {
    pub amount: u64,
    pub bridge_stake_seed: u32,
}
#[derive(Clone, Debug, BorshDeserialize, BorshSerialize)]
pub struct RecordDexArgs {
    pub dex_record: DexRecord,
}
#[derive(Clone, Debug, BorshDeserialize, BorshSerialize)]
pub enum DexRecord {
    DepositSol(DexRecordDepositSol),
    OneWayPoolPair(DexRecordOneWayPoolPair),
    TwoWayPoolPair(DexRecordTwoWayPoolPair),
}
#[derive(Clone, Debug, BorshDeserialize, BorshSerialize)]
pub enum DepositSolType {
    Eversol,
    Lido,
    Marinade,
    Socean,
    Spl,
}
#[derive(Clone, Debug, BorshDeserialize, BorshSerialize)]
pub enum DepositStakeType {
    Eversol,
    Marinade,
    Socean,
    Spl,
    Unstakeit,
}
#[derive(Clone, Debug, BorshDeserialize, BorshSerialize)]
pub enum WithdrawStakeType {
    Eversol,
    Lido,
    Socean,
    Spl,
}
#[derive(Clone, Debug, BorshDeserialize, BorshSerialize)]
pub enum DepositWithdrawStakeType {
    Eversol,
    Socean,
    Spl,
}
