use borsh::{BorshDeserialize, BorshSerialize};
#[derive(Clone, Debug, BorshDeserialize, BorshSerialize)]
pub struct DepositStakeAccountData {
    pub validator_index: u32,
}
