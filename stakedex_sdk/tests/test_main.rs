use jupiter_amm_interface::{QuoteParams, SwapMode, SwapParams};
use lazy_static::lazy_static;
use solana_account_decoder::UiAccountEncoding;
use solana_client::{
    rpc_client::RpcClient,
    rpc_config::{RpcSimulateTransactionAccountsConfig, RpcSimulateTransactionConfig},
};
use solana_sdk::{account::Account, pubkey::Pubkey, signer::Signer, transaction::Transaction};
use spl_associated_token_account::get_associated_token_address;
use spl_token::native_mint;
use stakedex_sdk::Stakedex;
use stakedex_sdk_common::{bsol, esol, jitosol, jsol, msol, stsol};
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
fn test_swap_via_stake_jitosol_bsol() {
    STAKEDEX
        .quote_swap_via_stake(&QuoteParams {
            amount: 1_000_000_000,
            input_mint: jitosol::ID,
            output_mint: bsol::ID,
            swap_mode: jupiter_amm_interface::SwapMode::ExactIn,
        })
        .unwrap();
}

#[test]
fn test_swap_via_stake_esol_bsol() {
    STAKEDEX
        .quote_swap_via_stake(&QuoteParams {
            amount: 1_000_000_000, // 1_000_000_000_000
            input_mint: esol::ID,
            output_mint: bsol::ID,
            swap_mode: jupiter_amm_interface::SwapMode::ExactIn,
        })
        .unwrap();
}

#[test]
fn test_swap_via_stake_unknown_token() {
    let unknown_token = Pubkey::new_unique();
    let res = STAKEDEX.quote_swap_via_stake(&QuoteParams {
        amount: 1_000_000_000,
        input_mint: unknown_token,
        output_mint: bsol::ID,
        swap_mode: jupiter_amm_interface::SwapMode::ExactIn,
    });
    assert!(res.is_err());
}

fn test_swap_via_stake(input_mint: Pubkey, output_mint: Pubkey) {
    let whale_pk = Pubkey::from_str(WHALE).unwrap();
    let res = STAKEDEX.quote_swap_via_stake(&QuoteParams {
        amount: 100_000_000_000,
        input_mint,
        output_mint,
        swap_mode: SwapMode::ExactIn,
    });
    match res {
        Err(err) => assert!(err.to_string() == "No route found between pools"),
        Ok(quote) => {
            println!("{:?}", quote);
            let source_token_account = get_associated_token_address(&whale_pk, &input_mint);
            let destination_token_account = get_associated_token_address(&whale_pk, &output_mint);
            let ix = STAKEDEX
                .swap_via_stake_ix(
                    &SwapParams {
                        jupiter_program_id: &jupiter_program::ID,
                        in_amount: quote.in_amount,
                        out_amount: 0,
                        destination_mint: output_mint,
                        source_mint: input_mint,
                        source_token_account,
                        destination_token_account,
                        token_transfer_authority: whale_pk,
                        open_order_address: None,
                        quote_mint_to_referrer: None,
                    },
                    0,
                )
                .unwrap();
            // let msg = Message::new(&[ix], Some(&whale_pk));
            // let blockhash = RPC.get_latest_blockhash().unwrap();
            let tx = Transaction::new_with_payer(&[ix], Some(&whale_pk));
            let result = RPC
                .simulate_transaction_with_config(
                    &tx,
                    RpcSimulateTransactionConfig {
                        accounts: Some(RpcSimulateTransactionAccountsConfig {
                            addresses: vec![
                                source_token_account.to_string(),
                                destination_token_account.to_string(),
                            ],
                            encoding: Some(UiAccountEncoding::Base64),
                        }),
                        ..RpcSimulateTransactionConfig::default()
                    },
                )
                .unwrap();
            println!(
                "1 {}\n2 {}",
                source_token_account, destination_token_account
            );
            println!("Simulate result: {:?}", result.value);
            println!("{:?}", result.value.accounts.unwrap());
        }
    }
}

#[test]
fn test_swap_via_stake_stsol_unstakeit() {
    test_swap_via_stake(stsol::ID, native_mint::ID);
}

#[test]
fn test_swap_via_stake_jsol_msol() {
    test_swap_via_stake(jsol::ID, msol::ID);
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
        .max_by_key(|v| u64::from(v.active_stake_lamports))
        .unwrap();
    let max_withdraw_lamports = largest_active_stake_vsi.active_stake_lamports;
    let parts_after_fees = (STAKEDEX.jpool.stake_pool.stake_withdrawal_fee.denominator
        - STAKEDEX.jpool.stake_pool.stake_withdrawal_fee.numerator)
        as u128;
    let max_withdraw_lamports_bef_fees = (u128::from(u64::from(max_withdraw_lamports))
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
            amount: 100_000_000_000,
            input_mint: stsol::ID,
            output_mint: native_mint::ID,
            swap_mode: jupiter_amm_interface::SwapMode::ExactIn,
        })
        .unwrap();
    let should_fail = STAKEDEX.quote_swap_via_stake(&QuoteParams {
        amount: max_withdraw_jsol + 1,
        input_mint: jsol::ID,
        output_mint: msol::ID,
        swap_mode: SwapMode::ExactIn,
    });
    assert!(should_fail.is_err());
    // try simulating max possible quote
    let whale_pk = Pubkey::from_str(WHALE).unwrap();
    let source_token_account = get_associated_token_address(&whale_pk, &jsol::ID);
    let destination_token_account = get_associated_token_address(&whale_pk, &msol::ID);
    let params = SwapParams {
        jupiter_program_id: &jupiter_program::ID,
        in_amount: max_possible_quote.in_amount,
        out_amount: 0,
        destination_mint: msol::ID,
        source_mint: jsol::ID,
        source_token_account,
        destination_token_account,
        token_transfer_authority: whale_pk,
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
