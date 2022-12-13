use borsh::{BorshDeserialize, BorshSerialize};
#[derive(Clone, Debug, BorshDeserialize, BorshSerialize)]
pub struct LidoDepositSolAccountData {
    pub lamports: u64,
}
#[derive(Clone, Debug, BorshDeserialize, BorshSerialize)]
pub struct MarinadeDepositSolAccountData {
    pub lamports: u64,
}
