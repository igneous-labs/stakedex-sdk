use borsh::{BorshDeserialize, BorshSerialize};
#[derive(Clone, Debug, BorshDeserialize, BorshSerialize)]
pub struct StakeWrappedSolArgs {
    pub amount: u64,
}
#[derive(Clone, Debug, BorshDeserialize, BorshSerialize)]
pub struct SwapViaStakeArgs {
    pub amount: u64,
    pub bridge_stake_seed: u32,
}
