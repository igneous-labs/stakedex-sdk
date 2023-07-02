use jupiter_core::amm::QuoteParams;
use lazy_static::lazy_static;
use solana_client::rpc_client::RpcClient;
use solana_sdk::{account::Account, pubkey::Pubkey};
use stakedex_sdk::Stakedex;
use stakedex_sdk_common::{bsol, jitosol};
use std::{collections::HashMap, iter::zip};

lazy_static! {
    static ref RPC: RpcClient = RpcClient::new(std::env::var("SOLANA_RPC_URL").unwrap());
    static ref STAKEDEX: Stakedex = {
        let init_accounts = fetch_accounts(&Stakedex::init_accounts());
        let (mut stakedex, errs) = Stakedex::from_fetched_accounts(&init_accounts);
        if !errs.is_empty() {
            eprintln!("init errs {:?}", errs);
        }
        // Need to update() one more time to fetch auxilliary accounts (e.g. validatorlist)
        let update_accounts = fetch_accounts(&stakedex.get_accounts_to_update());
        let errs = stakedex.update(update_accounts);
        if !errs.is_empty() {
            eprintln!("update errs {:?}", errs);
        }
        stakedex
    };
}

fn fetch_accounts(accounts_pubkeys: &[Pubkey]) -> HashMap<Pubkey, Account> {
    let fetched = RPC.get_multiple_accounts(&accounts_pubkeys).unwrap();
    zip(accounts_pubkeys, fetched)
        .filter_map(|(pubkey, opt)| match opt {
            Some(acc) => Some((pubkey.clone(), acc)),
            None => {
                eprintln!("Missing acc {}", pubkey);
                None
            }
        })
        .collect()
}

#[test]
fn test_swap_via_stake_jitosol_bsol() {
    STAKEDEX
        .quote_swap_via_stake(&QuoteParams {
            in_amount: 1_000_000_000,
            input_mint: jitosol::ID,
            output_mint: bsol::ID,
        })
        .unwrap();
}
