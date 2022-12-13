use borsh::{BorshDeserialize, BorshSerialize};
#[derive(Clone, Debug, BorshDeserialize, BorshSerialize)]
pub struct WithdrawStakeAccountData {
    pub amount: u64,
    pub validator_index: u32,
}
