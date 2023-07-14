pub use jupiter_amm_interface::{AccountMap, KeyedAccount, Swap};
use solana_program::instruction::AccountMeta;

pub static STAKEDEX_ACCOUNT_META: AccountMeta = AccountMeta {
    pubkey: stakedex_interface::ID,
    is_signer: false,
    is_writable: false,
};
