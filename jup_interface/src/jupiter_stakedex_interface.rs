pub use jupiter_amm_interface::{AccountMap, KeyedAccount, Swap};
use solana_sdk::instruction::AccountMeta;
use stakedex_sdk_common::stakedex_program;

pub static STAKEDEX_ACCOUNT_META: AccountMeta = AccountMeta {
    pubkey: stakedex_program::ID,
    is_signer: false,
    is_writable: false,
};
