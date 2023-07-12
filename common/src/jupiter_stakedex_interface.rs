pub use jupiter_amm_interface::{AccountMap, KeyedAccount, Swap};
use solana_program::instruction::AccountMeta;
use solana_program::pubkey;
use solana_program::pubkey::Pubkey;

// TODO: Move this to the interface, dynamic?
const JUPITER_PROGRAM_ID: Pubkey = pubkey!("BUG9oTodrkqFqxLhvvaUtwsPLtk1wkcTC544fQn9fWPm");

pub const JUPITER_ACCOUNT_META: AccountMeta = AccountMeta {
    pubkey: JUPITER_PROGRAM_ID,
    is_signer: false,
    is_writable: false,
};

pub static STAKEDEX_ACCOUNT_META: AccountMeta = AccountMeta {
    pubkey: stakedex_interface::ID,
    is_signer: false,
    is_writable: false,
};
