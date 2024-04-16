use anyhow::Result;
use jupiter_amm_interface::{AmmContext, KeyedAccount};

pub trait InitFromKeyedAccount: Sized {
    fn from_keyed_account(keyed_account: &KeyedAccount, amm_context: &AmmContext) -> Result<Self>;
}
