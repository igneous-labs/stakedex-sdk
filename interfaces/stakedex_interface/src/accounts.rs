use crate::*;
use borsh::{BorshDeserialize, BorshSerialize};
#[derive(Clone, Debug, BorshDeserialize, BorshSerialize)]
pub struct DexRecordAccount {
    pub record: DexRecord,
}
