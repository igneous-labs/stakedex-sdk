use anyhow::Result;
use jupiter_amm_interface::KeyedAccount;

pub trait InitFromKeyedAccount: Sized {
    fn from_keyed_account(keyed_account: &KeyedAccount) -> Result<Self>;
}
