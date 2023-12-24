use std::iter::zip;

use solana_client::rpc_client::RpcClient;
use solana_program::sysvar;
use stakedex_lido::LidoStakedex;
use stakedex_sdk_common::{
    jupiter_stakedex_interface::KeyedAccount, lido_state, lido_validator_list, BaseStakePoolAmm,
    InitFromKeyedAccount,
};

#[test]
fn test_mainnet() {
    let client = RpcClient::new(std::env::var("SOLANA_RPC_URL").unwrap());
    let keys = vec![lido_state::ID, lido_validator_list::ID, sysvar::clock::ID];
    let accounts = client.get_multiple_accounts(&keys).unwrap();
    let keyed_state = KeyedAccount {
        key: lido_state::ID,
        account: accounts[0].clone().unwrap(),
        params: None,
    };
    let accounts_map = zip(keys, accounts.into_iter().map(|o| o.unwrap())).collect();
    let mut lido = LidoStakedex::from_keyed_account(&keyed_state).unwrap();
    lido.update(&accounts_map).unwrap();
}
