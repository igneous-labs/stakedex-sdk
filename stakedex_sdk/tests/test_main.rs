use jupiter_amm_interface::{QuoteParams, SwapParams};
use lazy_static::lazy_static;
use solana_account_decoder::UiAccountEncoding;
use solana_client::{
    rpc_client::RpcClient,
    rpc_config::{RpcSimulateTransactionAccountsConfig, RpcSimulateTransactionConfig},
    rpc_response::{Response, RpcSimulateTransactionResult},
};
use solana_sdk::{account::Account, pubkey::Pubkey, signer::Signer, transaction::Transaction};
use spl_associated_token_account::get_associated_token_address;
use spl_token::native_mint;
use stakedex_sdk::Stakedex;
use stakedex_sdk_common::{bsol, esol, jitosol, jsol, msol, stsol};
use std::{collections::HashMap, iter::zip, str::FromStr};

// Alameda account with yuge mSOL, jSOL, stSOL holdings
pub mod whale {
    solana_sdk::declare_id!("9uyDy9VDBw4K7xoSkhmCAm8NAFCwu4pkF6JeHUCtVKcX");
}

pub mod jupiter_program {
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
fn test_quote_swap_via_stake_jitosol_bsol() {
    STAKEDEX
        .quote_swap_via_stake(&QuoteParams {
            in_amount: 1_000_000_000,
            input_mint: jitosol::ID,
            output_mint: bsol::ID,
        })
        .unwrap();
}

#[test]
fn test_quote_swap_via_stake_esol_bsol() {
    STAKEDEX
        .quote_swap_via_stake(&QuoteParams {
            in_amount: 1_000_000_000, // 1_000_000_000_000
            input_mint: esol::ID,
            output_mint: bsol::ID,
        })
        .unwrap();
}

#[test]
fn test_quote_swap_via_stake_unknown_token() {
    let unknown_token = Pubkey::new_unique();
    let res = STAKEDEX.quote_swap_via_stake(&QuoteParams {
        in_amount: 1_000_000_000,
        input_mint: unknown_token,
        output_mint: bsol::ID,
    });
    assert!(res.is_err());
}

#[test]
fn test_swap_via_stake_stsol_unstakeit() {
    let resp = sim_swap_via_stake_with_whale(&STAKEDEX, &RPC, stsol::ID, native_mint::ID);
    assert_sim_success(&resp);
}

#[test]
fn test_swap_via_stake_jsol_msol() {
    let resp = sim_swap_via_stake_with_whale(&STAKEDEX, &RPC, jsol::ID, msol::ID);
    assert_sim_success(&resp);
}

#[test]
fn test_swap_via_stake_jitosol_bsol() {
    let signer = Pubkey::from_str("71WDyyCsZwyEYDV91Qrb212rdg6woCHYQhFnmZUBxiJ6").unwrap();
    let src_token_acc = Pubkey::from_str("83ohMPRdV5XJ868EmbDHyKEgK1gdaSctHpiXVcWiPBn7").unwrap();
    let dst_token_acc = Pubkey::from_str("9D1JFhFFd4rHWdJfjVkBaD6u3ZKcGtz7W4N1mqYUaM3T").unwrap();
    let resp = sim_swap_via_stake(
        &STAKEDEX,
        &RPC,
        TestSwapViaStakeArgs {
            in_amount: 100_000_000,
            input_mint: jitosol::ID,
            output_mint: bsol::ID,
            signer,
            src_token_acc,
            dst_token_acc,
        },
    );
    assert_sim_success(&resp);
}

#[test]
fn test_swap_via_stake_bsol_jitosol() {
    let signer = Pubkey::from_str("71WDyyCsZwyEYDV91Qrb212rdg6woCHYQhFnmZUBxiJ6").unwrap();
    let src_token_acc = Pubkey::from_str("9D1JFhFFd4rHWdJfjVkBaD6u3ZKcGtz7W4N1mqYUaM3T").unwrap();
    let dst_token_acc = Pubkey::from_str("83ohMPRdV5XJ868EmbDHyKEgK1gdaSctHpiXVcWiPBn7").unwrap();
    let resp = sim_swap_via_stake(
        &STAKEDEX,
        &RPC,
        TestSwapViaStakeArgs {
            in_amount: 100_000_000,
            input_mint: bsol::ID,
            output_mint: jitosol::ID,
            signer,
            src_token_acc,
            dst_token_acc,
        },
    );
    assert_sim_success(&resp);
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
    let result = sim_swap_via_stake(
        &STAKEDEX,
        &RPC,
        TestSwapViaStakeArgs {
            in_amount: max_possible_quote.in_amount,
            input_mint: jsol::ID,
            output_mint: msol::ID,
            signer: whale::ID,
            src_token_acc: get_associated_token_address(&whale::ID, &jsol::ID),
            dst_token_acc: get_associated_token_address(&whale::ID, &msol::ID),
        },
    );
    assert!(result.value.err.is_none());
}

pub struct TestSwapViaStakeArgs {
    pub in_amount: u64,
    pub input_mint: Pubkey,
    pub output_mint: Pubkey,
    pub signer: Pubkey,
    pub src_token_acc: Pubkey,
    pub dst_token_acc: Pubkey,
}

pub fn sim_swap_via_stake_with_whale(
    stakedex: &Stakedex,
    rpc: &RpcClient,
    input_mint: Pubkey,
    output_mint: Pubkey,
) -> Response<RpcSimulateTransactionResult> {
    sim_swap_via_stake(
        stakedex,
        rpc,
        TestSwapViaStakeArgs {
            in_amount: 100_000_000_000,
            input_mint,
            output_mint,
            signer: whale::ID,
            src_token_acc: get_associated_token_address(&whale::ID, &input_mint),
            dst_token_acc: get_associated_token_address(&whale::ID, &output_mint),
        },
    )
}

fn assert_sim_success(response: &Response<RpcSimulateTransactionResult>) {
    if let Some(e) = &response.value.err {
        eprintln!("{:#?}", response.value.logs.as_ref().unwrap());
        eprintln!("ERROR: {e}");
    }
}

pub fn sim_swap_via_stake(
    stakedex: &Stakedex,
    rpc: &RpcClient,
    TestSwapViaStakeArgs {
        in_amount,
        input_mint,
        output_mint,
        signer,
        src_token_acc,
        dst_token_acc,
    }: TestSwapViaStakeArgs,
) -> Response<RpcSimulateTransactionResult> {
    let quote = stakedex
        .quote_swap_via_stake(&QuoteParams {
            in_amount,
            input_mint,
            output_mint,
        })
        .unwrap();
    // println!("{:?}", quote);
    let ix = stakedex
        .swap_via_stake_ix(
            &SwapParams {
                jupiter_program_id: &jupiter_program::ID,
                in_amount: quote.in_amount,
                destination_mint: output_mint,
                source_mint: input_mint,
                user_destination_token_account: dst_token_acc,
                user_source_token_account: src_token_acc,
                user_transfer_authority: signer,
                open_order_address: None,
                quote_mint_to_referrer: None,
            },
            0,
        )
        .unwrap();
    // let msg = Message::new(&[ix], Some(&whale_pk));
    // let blockhash = RPC.get_latest_blockhash().unwrap();
    let rbh = rpc.get_latest_blockhash().unwrap();
    let mut tx = Transaction::new_with_payer(&[ix], Some(&signer));
    // partial_sign just to add recentblockhash
    let no_signers: Vec<Box<dyn Signer>> = vec![];
    tx.partial_sign(&no_signers, rbh);
    let result = rpc
        .simulate_transaction_with_config(
            &tx,
            RpcSimulateTransactionConfig {
                accounts: Some(RpcSimulateTransactionAccountsConfig {
                    addresses: vec![src_token_acc.to_string(), dst_token_acc.to_string()],
                    encoding: Some(UiAccountEncoding::Base64),
                }),
                ..RpcSimulateTransactionConfig::default()
            },
        )
        .unwrap();
    if let Some(err) = result.value.err {
        panic!("{err}");
    }
    result
}
