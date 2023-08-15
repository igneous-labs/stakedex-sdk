use jupiter_amm_interface::{QuoteParams, SwapParams};
use lazy_static::lazy_static;
use solana_account_decoder::UiAccountEncoding;
use solana_client::{
    rpc_client::RpcClient,
    rpc_config::{RpcSimulateTransactionAccountsConfig, RpcSimulateTransactionConfig},
};
use solana_sdk::{
    account::Account, program_pack::Pack, pubkey::Pubkey, signer::Signer, transaction::Transaction,
};
use spl_associated_token_account::get_associated_token_address;
use spl_token::native_mint;
use stakedex_sdk::Stakedex;
use stakedex_sdk_common::{
    bsol, cogentsol, daosol, esol, jitosol, jsol, lainesol, msol, risksol, scnsol, stsol,
};
use std::{collections::HashMap, iter::zip, str::FromStr};

const WHALE: &str = "9uyDy9VDBw4K7xoSkhmCAm8NAFCwu4pkF6JeHUCtVKcX";

mod jupiter_program {
    // NOT IN USE, JUST BECAUSE ITS REQUIRED AS A STRUCT FIELD FOR jupiter_amm_interface::SwapParams
    solana_sdk::declare_id!("JUP4Fb2cqiRUcaTHdrPC8h2gNsA2ETXiPDD33WcGuJB");
}

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
fn test_swap_via_stake_unknown_token() {
    let unknown_token = Pubkey::new_unique();
    let res = STAKEDEX.quote_swap_via_stake(&QuoteParams {
        in_amount: 1_000_000_000,
        input_mint: unknown_token,
        output_mint: bsol::ID,
    });
    assert!(res.is_err());
}

// unstakeit
#[test]
fn test_swap_via_stake_bsol_unstakeit() {
    test_swap_via_stake(bsol::ID, native_mint::ID, None);
}

#[test]
fn test_swap_via_stake_cogentsol_unstakeit() {
    test_swap_via_stake(cogentsol::ID, native_mint::ID, None);
}

#[test]
fn test_swap_via_stake_daosol_unstakeit() {
    test_swap_via_stake(daosol::ID, native_mint::ID, None);
}

#[test]
fn test_swap_via_stake_jitosol_unstakeit() {
    test_swap_via_stake(jitosol::ID, native_mint::ID, None);
}

#[test]
fn test_swap_via_stake_jsol_unstakeit() {
    test_swap_via_stake(jsol::ID, native_mint::ID, None);
}

#[test]
fn test_swap_via_stake_lainesol_unstakeit() {
    test_swap_via_stake(lainesol::ID, native_mint::ID, None);
}

#[test]
fn test_swap_via_stake_risksol_unstakeit() {
    test_swap_via_stake(risksol::ID, native_mint::ID, None);
}

#[test]
fn test_swap_via_stake_scnsol_unstakeit() {
    test_swap_via_stake(scnsol::ID, native_mint::ID, None);
}

#[test]
fn test_swap_via_stake_esol_unstakeit() {
    test_swap_via_stake(esol::ID, native_mint::ID, None);
}

#[test]
fn test_swap_via_stake_msol_unstakeit() {
    test_swap_via_stake(msol::ID, native_mint::ID, None);
}

#[test]
fn test_swap_via_stake_stsol_unstakeit() {
    test_swap_via_stake(stsol::ID, native_mint::ID, None);
}

// bsol to xsol
#[test]
fn test_swap_via_stake_bsol_cogentsol() {
    test_swap_via_stake(bsol::ID, cogentsol::ID, None);
}

#[test]
fn test_swap_via_stake_bsol_daosol() {
    test_swap_via_stake(bsol::ID, daosol::ID, None);
}

#[test]
fn test_swap_via_stake_bsol_jitosol() {
    test_swap_via_stake(bsol::ID, jitosol::ID, None);
}

#[test]
fn test_swap_via_stake_bsol_jsol() {
    test_swap_via_stake(bsol::ID, jsol::ID, None);
}

#[test]
fn test_swap_via_stake_bsol_lainesol() {
    test_swap_via_stake(bsol::ID, lainesol::ID, None);
}

#[test]
fn test_swap_via_stake_bsol_risksol() {
    test_swap_via_stake(bsol::ID, risksol::ID, None);
}

#[test]
fn test_swap_via_stake_bsol_scnsol() {
    test_swap_via_stake(bsol::ID, scnsol::ID, None);
}

#[test]
fn test_swap_via_stake_bsol_esol() {
    test_swap_via_stake(bsol::ID, esol::ID, None);
}

#[test]
fn test_swap_via_stake_bsol_msol() {
    test_swap_via_stake(bsol::ID, msol::ID, None);
}

#[test]
fn test_swap_via_stake_bsol_stsol() {
    test_swap_via_stake(bsol::ID, stsol::ID, None);
}

// cogentsol to xsol
#[test]
fn test_swap_via_stake_cogentsol_bsol() {
    test_swap_via_stake(cogentsol::ID, bsol::ID, None);
}

#[test]
fn test_swap_via_stake_cogentsol_daosol() {
    test_swap_via_stake(cogentsol::ID, daosol::ID, None);
}

#[test]
fn test_swap_via_stake_cogentsol_jitosol() {
    test_swap_via_stake(cogentsol::ID, jitosol::ID, None);
}

#[test]
fn test_swap_via_stake_cogentsol_jsol() {
    test_swap_via_stake(cogentsol::ID, jsol::ID, None);
}

#[test]
fn test_swap_via_stake_cogentsol_lainesol() {
    test_swap_via_stake(cogentsol::ID, lainesol::ID, None);
}

#[test]
fn test_swap_via_stake_cogentsol_risksol() {
    test_swap_via_stake(cogentsol::ID, risksol::ID, None);
}

#[test]
fn test_swap_via_stake_cogentsol_scnsol() {
    test_swap_via_stake(cogentsol::ID, scnsol::ID, None);
}

#[test]
fn test_swap_via_stake_cogentsol_esol() {
    test_swap_via_stake(cogentsol::ID, esol::ID, None);
}

#[test]
fn test_swap_via_stake_cogentsol_msol() {
    test_swap_via_stake(cogentsol::ID, msol::ID, None);
}

#[test]
fn test_swap_via_stake_cogentsol_stsol() {
    test_swap_via_stake(cogentsol::ID, stsol::ID, None);
}

// daosol to xsol
#[test]
fn test_swap_via_stake_daosol_bsol() {
    test_swap_via_stake(daosol::ID, bsol::ID, None);
}

#[test]
fn test_swap_via_stake_daosol_cogentsol() {
    test_swap_via_stake(daosol::ID, cogentsol::ID, None);
}

#[test]
fn test_swap_via_stake_daosol_jitosol() {
    test_swap_via_stake(daosol::ID, jitosol::ID, None);
}

#[test]
fn test_swap_via_stake_daosol_jsol() {
    test_swap_via_stake(daosol::ID, jsol::ID, None);
}

#[test]
fn test_swap_via_stake_daosol_lainesol() {
    test_swap_via_stake(daosol::ID, lainesol::ID, None);
}

#[test]
fn test_swap_via_stake_daosol_risksol() {
    test_swap_via_stake(daosol::ID, risksol::ID, None);
}

#[test]
fn test_swap_via_stake_daosol_scnsol() {
    test_swap_via_stake(daosol::ID, scnsol::ID, None);
}

#[test]
fn test_swap_via_stake_daosol_esol() {
    test_swap_via_stake(daosol::ID, esol::ID, None);
}

#[test]
fn test_swap_via_stake_daosol_msol() {
    test_swap_via_stake(daosol::ID, msol::ID, None);
}

#[test]
fn test_swap_via_stake_daosol_stsol() {
    test_swap_via_stake(daosol::ID, stsol::ID, None);
}

// jitosol to xsol
#[test]
fn test_swap_via_stake_jitosol_bsol() {
    test_swap_via_stake(jitosol::ID, bsol::ID, None);
}

#[test]
fn test_swap_via_stake_jitosol_cogentsol() {
    test_swap_via_stake(jitosol::ID, cogentsol::ID, None);
}

#[test]
fn test_swap_via_stake_jitosol_daosol() {
    test_swap_via_stake(jitosol::ID, daosol::ID, None);
}

#[test]
fn test_swap_via_stake_jitosol_jsol() {
    test_swap_via_stake(jitosol::ID, jsol::ID, None);
}

#[test]
fn test_swap_via_stake_jitosol_lainesol() {
    test_swap_via_stake(jitosol::ID, lainesol::ID, None);
}

#[test]
fn test_swap_via_stake_jitosol_risksol() {
    test_swap_via_stake(jitosol::ID, risksol::ID, None);
}

#[test]
fn test_swap_via_stake_jitosol_scnsol() {
    test_swap_via_stake(jitosol::ID, scnsol::ID, None);
}

#[test]
fn test_swap_via_stake_jitosol_esol() {
    test_swap_via_stake(jitosol::ID, esol::ID, None);
}

#[test]
fn test_swap_via_stake_jitosol_msol() {
    test_swap_via_stake(jitosol::ID, msol::ID, None);
}

#[test]
fn test_swap_via_stake_jitosol_stsol() {
    test_swap_via_stake(jitosol::ID, stsol::ID, None);
}

// jsol to xsol
#[test]
fn test_swap_via_stake_jsol_bsol() {
    test_swap_via_stake(jsol::ID, bsol::ID, None);
}

#[test]
fn test_swap_via_stake_jsol_cogentsol() {
    test_swap_via_stake(jsol::ID, cogentsol::ID, None);
}

#[test]
fn test_swap_via_stake_jsol_daosol() {
    test_swap_via_stake(jsol::ID, daosol::ID, None);
}

#[test]
fn test_swap_via_stake_jsol_jitosol() {
    test_swap_via_stake(jsol::ID, jitosol::ID, None);
}

#[test]
fn test_swap_via_stake_jsol_lainesol() {
    test_swap_via_stake(jsol::ID, lainesol::ID, None);
}

#[test]
fn test_swap_via_stake_jsol_risksol() {
    test_swap_via_stake(jsol::ID, risksol::ID, None);
}

#[test]
fn test_swap_via_stake_jsol_scnsol() {
    test_swap_via_stake(jsol::ID, scnsol::ID, None);
}

#[test]
fn test_swap_via_stake_jsol_esol() {
    test_swap_via_stake(jsol::ID, esol::ID, None);
}

#[test]
fn test_swap_via_stake_jsol_msol() {
    test_swap_via_stake(jsol::ID, msol::ID, None);
}

#[test]
fn test_swap_via_stake_jsol_stsol() {
    test_swap_via_stake(jsol::ID, stsol::ID, None);
}

// lainesol to xsol
#[test]
fn test_swap_via_stake_lainesol_bsol() {
    test_swap_via_stake(lainesol::ID, bsol::ID, None);
}

#[test]
fn test_swap_via_stake_lainesol_cogentsol() {
    test_swap_via_stake(lainesol::ID, cogentsol::ID, None);
}

#[test]
fn test_swap_via_stake_lainesol_daosol() {
    test_swap_via_stake(lainesol::ID, daosol::ID, None);
}

#[test]
fn test_swap_via_stake_lainesol_jitosol() {
    test_swap_via_stake(lainesol::ID, jitosol::ID, None);
}

#[test]
fn test_swap_via_stake_lainesol_jsol() {
    test_swap_via_stake(lainesol::ID, jsol::ID, None);
}

#[test]
fn test_swap_via_stake_lainesol_risksol() {
    test_swap_via_stake(lainesol::ID, risksol::ID, None);
}

#[test]
fn test_swap_via_stake_lainesol_scnsol() {
    test_swap_via_stake(lainesol::ID, scnsol::ID, None);
}

#[test]
fn test_swap_via_stake_lainesol_esol() {
    test_swap_via_stake(lainesol::ID, esol::ID, None);
}

#[test]
fn test_swap_via_stake_lainesol_msol() {
    test_swap_via_stake(lainesol::ID, msol::ID, None);
}

#[test]
fn test_swap_via_stake_lainesol_stsol() {
    test_swap_via_stake(lainesol::ID, stsol::ID, None);
}

// risksol to xsol
#[test]
fn test_swap_via_stake_risksol_bsol() {
    test_swap_via_stake(risksol::ID, bsol::ID, None);
}

#[test]
fn test_swap_via_stake_risksol_cogentsol() {
    test_swap_via_stake(risksol::ID, cogentsol::ID, None);
}

#[test]
fn test_swap_via_stake_risksol_daosol() {
    test_swap_via_stake(risksol::ID, daosol::ID, None);
}

#[test]
fn test_swap_via_stake_risksol_jitosol() {
    test_swap_via_stake(risksol::ID, jitosol::ID, None);
}

#[test]
fn test_swap_via_stake_risksol_jsol() {
    test_swap_via_stake(risksol::ID, jsol::ID, None);
}

#[test]
fn test_swap_via_stake_risksol_lainesol() {
    test_swap_via_stake(risksol::ID, lainesol::ID, None);
}

#[test]
fn test_swap_via_stake_risksol_scnsol() {
    test_swap_via_stake(risksol::ID, scnsol::ID, None);
}

#[test]
fn test_swap_via_stake_risksol_esol() {
    test_swap_via_stake(risksol::ID, esol::ID, None);
}

#[test]
fn test_swap_via_stake_risksol_msol() {
    test_swap_via_stake(risksol::ID, msol::ID, None);
}

#[test]
fn test_swap_via_stake_risksol_stsol() {
    test_swap_via_stake(risksol::ID, stsol::ID, None);
}

// scnsol to xsol
#[test]
fn test_swap_via_stake_scnsol_bsol() {
    test_swap_via_stake(scnsol::ID, bsol::ID, None);
}

#[test]
fn test_swap_via_stake_scnsol_cogentsol() {
    test_swap_via_stake(scnsol::ID, cogentsol::ID, None);
}

#[test]
fn test_swap_via_stake_scnsol_daosol() {
    test_swap_via_stake(scnsol::ID, daosol::ID, None);
}

#[test]
fn test_swap_via_stake_scnsol_jitosol() {
    test_swap_via_stake(scnsol::ID, jitosol::ID, None);
}

#[test]
fn test_swap_via_stake_scnsol_jsol() {
    test_swap_via_stake(scnsol::ID, jsol::ID, None);
}

#[test]
fn test_swap_via_stake_scnsol_lainesol() {
    test_swap_via_stake(scnsol::ID, lainesol::ID, None);
}

#[test]
fn test_swap_via_stake_scnsol_risksol() {
    test_swap_via_stake(scnsol::ID, risksol::ID, None);
}

#[test]
fn test_swap_via_stake_scnsol_esol() {
    test_swap_via_stake(scnsol::ID, esol::ID, None);
}

#[test]
fn test_swap_via_stake_scnsol_msol() {
    test_swap_via_stake(scnsol::ID, msol::ID, None);
}

#[test]
fn test_swap_via_stake_scnsol_stsol() {
    test_swap_via_stake(scnsol::ID, stsol::ID, None);
}

// esol to xsol
#[test]
fn test_swap_via_stake_esol_bsol() {
    test_swap_via_stake(esol::ID, bsol::ID, None);
}

#[test]
fn test_swap_via_stake_esol_cogentsol() {
    test_swap_via_stake(esol::ID, cogentsol::ID, None);
}

#[test]
fn test_swap_via_stake_esol_daosol() {
    test_swap_via_stake(esol::ID, daosol::ID, None);
}

#[test]
fn test_swap_via_stake_esol_jitosol() {
    test_swap_via_stake(esol::ID, jitosol::ID, None);
}

#[test]
fn test_swap_via_stake_esol_jsol() {
    test_swap_via_stake(esol::ID, jsol::ID, None);
}

#[test]
fn test_swap_via_stake_esol_lainesol() {
    test_swap_via_stake(esol::ID, lainesol::ID, None);
}

#[test]
fn test_swap_via_stake_esol_risksol() {
    test_swap_via_stake(esol::ID, risksol::ID, None);
}

#[test]
fn test_swap_via_stake_esol_scnsol() {
    test_swap_via_stake(esol::ID, scnsol::ID, None);
}

#[test]
fn test_swap_via_stake_esol_msol() {
    test_swap_via_stake(esol::ID, msol::ID, None);
}

#[test]
fn test_swap_via_stake_esol_stsol() {
    test_swap_via_stake(esol::ID, stsol::ID, None);
}
#[test]
fn test_swap_via_stake_msol_bsol() {
    test_swap_via_stake(msol::ID, bsol::ID, None);
}

#[test]
fn test_swap_via_stake_msol_cogentsol() {
    test_swap_via_stake(msol::ID, cogentsol::ID, None);
}

#[test]
fn test_swap_via_stake_msol_daosol() {
    test_swap_via_stake(msol::ID, daosol::ID, None);
}

#[test]
fn test_swap_via_stake_msol_jitosol() {
    test_swap_via_stake(msol::ID, jitosol::ID, None);
}

#[test]
fn test_swap_via_stake_msol_jsol() {
    test_swap_via_stake(msol::ID, jsol::ID, None);
}

#[test]
fn test_swap_via_stake_msol_lainesol() {
    test_swap_via_stake(msol::ID, lainesol::ID, None);
}

#[test]
fn test_swap_via_stake_msol_risksol() {
    test_swap_via_stake(msol::ID, risksol::ID, None);
}

#[test]
fn test_swap_via_stake_msol_scnsol() {
    test_swap_via_stake(msol::ID, scnsol::ID, None);
}

#[test]
fn test_swap_via_stake_msol_esol() {
    test_swap_via_stake(msol::ID, esol::ID, None);
}

#[test]
fn test_swap_via_stake_msol_stsol() {
    test_swap_via_stake(msol::ID, stsol::ID, None);
}

// stsol to xsol
#[test]
fn test_swap_via_stake_stsol_bsol() {
    test_swap_via_stake(stsol::ID, bsol::ID, None);
}

#[test]
fn test_swap_via_stake_stsol_cogentsol() {
    test_swap_via_stake(stsol::ID, cogentsol::ID, None);
}

#[test]
fn test_swap_via_stake_stsol_daosol() {
    test_swap_via_stake(stsol::ID, daosol::ID, None);
}

#[test]
fn test_swap_via_stake_stsol_jitosol() {
    test_swap_via_stake(stsol::ID, jitosol::ID, None);
}

#[test]
fn test_swap_via_stake_stsol_jsol() {
    test_swap_via_stake(stsol::ID, jsol::ID, None);
}

#[test]
fn test_swap_via_stake_stsol_lainesol() {
    test_swap_via_stake(stsol::ID, lainesol::ID, None);
}

#[test]
fn test_swap_via_stake_stsol_risksol() {
    test_swap_via_stake(stsol::ID, risksol::ID, None);
}

#[test]
fn test_swap_via_stake_stsol_scnsol() {
    test_swap_via_stake(stsol::ID, scnsol::ID, None);
}

#[test]
fn test_swap_via_stake_stsol_esol() {
    test_swap_via_stake(stsol::ID, esol::ID, None);
}

#[test]
fn test_swap_via_stake_stsol_msol() {
    test_swap_via_stake(stsol::ID, msol::ID, None);
}

fn test_swap_via_stake(input_mint: Pubkey, output_mint: Pubkey, amount: Option<u64>) {
    // let user_keypair = read_keypair_file("../test-key.json").unwrap();
    // let user_pk = user_keypair.pubkey();
    let whale_pk = Pubkey::from_str(WHALE).unwrap();

    let source_token_account = get_associated_token_address(&whale_pk, &input_mint);
    let source_balance = RPC
        .get_token_account_balance(&source_token_account)
        .map_err(|err| {
            println!("Could not swap {} to {}", input_mint, output_mint);
            err
        })
        .unwrap();

    let destination_token_account = get_associated_token_address(&whale_pk, &output_mint);
    let destination_balance = RPC
        .get_token_account_balance(&destination_token_account)
        .map_err(|err| {
            println!("Could not swap {} to {}", input_mint, output_mint);
            err
        })
        .unwrap();

    let before_source_amount: u64 = source_balance.amount.parse().unwrap();
    let amount = amount.unwrap_or(before_source_amount);
    let before_destination_amount: u64 = destination_balance.amount.parse().unwrap();

    let res = STAKEDEX.quote_swap_via_stake(&QuoteParams {
        in_amount: amount,
        input_mint,
        output_mint,
    });

    match res {
        Err(err) => {
            println!(
                "Could not swap {} to {} for {} lamports. Reason: {}",
                input_mint, output_mint, amount, err
            );
            assert!(err.to_string() == "No route found between pools")
        }
        Ok(quote) => {
            let destination_token_account = get_associated_token_address(&whale_pk, &output_mint);
            let ix = STAKEDEX
                .swap_via_stake_ix(
                    &SwapParams {
                        jupiter_program_id: &jupiter_program::ID,
                        in_amount: quote.in_amount,
                        destination_mint: output_mint,
                        source_mint: input_mint,
                        user_destination_token_account: destination_token_account,
                        user_source_token_account: source_token_account,
                        user_transfer_authority: whale_pk,
                        open_order_address: None,
                        quote_mint_to_referrer: None,
                    },
                    0,
                )
                .unwrap();
            let mut tx = Transaction::new_with_payer(&[ix], Some(&whale_pk));
            let rbh = RPC.get_latest_blockhash().unwrap();
            // partial_sign just to add recentblockhash
            let no_signers: Vec<Box<dyn Signer>> = vec![];
            tx.partial_sign(&no_signers, rbh);
            let result = RPC
                .simulate_transaction_with_config(
                    &tx,
                    RpcSimulateTransactionConfig {
                        accounts: Some(RpcSimulateTransactionAccountsConfig {
                            addresses: vec![
                                source_token_account.to_string(),
                                destination_token_account.to_string(),
                            ],
                            encoding: Some(UiAccountEncoding::JsonParsed),
                        }),
                        ..RpcSimulateTransactionConfig::default()
                    },
                )
                .unwrap();

            let res_accounts = result.value.accounts.unwrap();
            let res_source_account = res_accounts[0].as_ref().unwrap();
            let res_destination_account = res_accounts[1].as_ref().unwrap();

            if let (Some(decoded_source_account), Some(decoded_destination_account)) = (
                res_source_account.decode::<Account>(),
                res_destination_account.decode::<Account>(),
            ) {
                let after_source_amount =
                    spl_token_2022::state::Account::unpack(&decoded_source_account.data)
                        .unwrap()
                        .amount;
                let after_destination_amount =
                    spl_token_2022::state::Account::unpack(&decoded_destination_account.data)
                        .unwrap()
                        .amount;

                println!("Before input balance: {:?}\nAfter input balance: {:?}\nBefore output balance: {:?}\nAfter output balance: {:?}", before_source_amount, after_source_amount, before_destination_amount, after_destination_amount);

                assert!(quote.in_amount - before_source_amount == after_source_amount);
                assert!(quote.out_amount + before_destination_amount == after_destination_amount);
            }
        }
    }
}

#[test]
fn test_jsol_drain_vsa_edge_case() {
    // assumes
    // - jsol has no preferred validator
    // - marinade should accept any of jpool's vsas
    // - WHALE has 350k jSOL, that should be > any individual vsa of jpool
    assert!(STAKEDEX
        .jpool
        .stake_pool
        .preferred_withdraw_validator_vote_address
        .is_none());
    let largest_active_stake_vsi = STAKEDEX
        .jpool
        .validator_list
        .validators
        .iter()
        .max_by_key(|v| v.active_stake_lamports)
        .unwrap();
    let max_withdraw_lamports = largest_active_stake_vsi.active_stake_lamports;
    let parts_after_fees = (STAKEDEX.jpool.stake_pool.stake_withdrawal_fee.denominator
        - STAKEDEX.jpool.stake_pool.stake_withdrawal_fee.numerator)
        as u128;
    let max_withdraw_lamports_bef_fees = ((max_withdraw_lamports as u128)
        * (STAKEDEX.jpool.stake_pool.stake_withdrawal_fee.denominator as u128)
        + parts_after_fees
        - 1)
        / parts_after_fees;
    let max_withdraw_jsol = STAKEDEX
        .jpool
        .stake_pool
        .calc_pool_tokens_for_deposit(max_withdraw_lamports_bef_fees.try_into().unwrap())
        .unwrap();
    let max_possible_quote = STAKEDEX
        .quote_swap_via_stake(&QuoteParams {
            in_amount: max_withdraw_jsol,
            input_mint: jsol::ID,
            output_mint: msol::ID,
        })
        .unwrap();
    let should_fail = STAKEDEX.quote_swap_via_stake(&QuoteParams {
        in_amount: max_withdraw_jsol + 1,
        input_mint: jsol::ID,
        output_mint: msol::ID,
    });
    assert!(should_fail.is_err());
    // try simulating max possible quote
    let whale_pk = Pubkey::from_str(WHALE).unwrap();
    let user_source_token_account = get_associated_token_address(&whale_pk, &jsol::ID);
    let user_destination_token_account = get_associated_token_address(&whale_pk, &msol::ID);
    let params = SwapParams {
        jupiter_program_id: &jupiter_program::ID,
        in_amount: max_possible_quote.in_amount,
        destination_mint: msol::ID,
        source_mint: jsol::ID,
        user_destination_token_account,
        user_source_token_account,
        user_transfer_authority: whale_pk,
        open_order_address: None,
        quote_mint_to_referrer: None,
    };
    let ix = STAKEDEX.swap_via_stake_ix(&params, 0).unwrap();
    let mut tx = Transaction::new_with_payer(&[ix], Some(&whale_pk));
    let rbh = RPC.get_latest_blockhash().unwrap();
    // partial_sign just to add recentblockhash
    let no_signers: Vec<Box<dyn Signer>> = vec![];
    tx.partial_sign(&no_signers, rbh);
    let result = RPC.simulate_transaction(&tx).unwrap();
    assert!(result.value.err.is_none());
}
