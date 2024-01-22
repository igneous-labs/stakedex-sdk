use jupiter_amm_interface::{QuoteParams, SwapMode, SwapParams};
use lazy_static::lazy_static;
use solana_account_decoder::UiAccountEncoding;
use solana_client::{
    rpc_client::RpcClient,
    rpc_config::{RpcSimulateTransactionAccountsConfig, RpcSimulateTransactionConfig},
    rpc_response::{Response, RpcSimulateTransactionResult},
};
use solana_sdk::{
    account::Account, program_pack::Pack, pubkey::Pubkey, signer::Signer, transaction::Transaction,
};
use spl_associated_token_account::get_associated_token_address;
use spl_token::native_mint;
use stakedex_sdk::Stakedex;
use stakedex_sdk_common::{
    bsol, cogentsol, daosol, jitosol, jsol, lainesol, msol, risksol, scnsol, stsol,
};
use std::{collections::HashMap, iter::zip};

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
    let fetched = RPC.get_multiple_accounts(accounts_pubkeys).unwrap();
    zip(accounts_pubkeys, fetched)
        .filter_map(|(pubkey, opt)| match opt {
            Some(acc) => Some((*pubkey, acc)),
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
        amount: 1_000_000_000,
        input_mint: unknown_token,
        output_mint: bsol::ID,
        swap_mode: SwapMode::default(),
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
fn test_swap_via_stake_scnsol_msol() {
    test_swap_via_stake(scnsol::ID, msol::ID, None);
}

#[test]
fn test_swap_via_stake_scnsol_stsol() {
    test_swap_via_stake(scnsol::ID, stsol::ID, None);
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
fn test_swap_via_stake_stsol_msol() {
    test_swap_via_stake(stsol::ID, msol::ID, None);
}

fn test_swap_via_stake(input_mint: Pubkey, output_mint: Pubkey, amount: Option<u64>) {
    // ignore these for now cause whale doesn't have them
    if input_mint.eq(&bsol::ID)
        || output_mint.eq(&bsol::ID)
        || input_mint.eq(&cogentsol::ID)
        || output_mint.eq(&cogentsol::ID)
        || input_mint.eq(&daosol::ID)
        || output_mint.eq(&daosol::ID)
        || input_mint.eq(&jitosol::ID)
        || output_mint.eq(&jitosol::ID)
        || input_mint.eq(&lainesol::ID)
        || output_mint.eq(&lainesol::ID)
        || input_mint.eq(&risksol::ID)
        || output_mint.eq(&risksol::ID)
        // invalid inputs / output mints
        || input_mint.eq(&msol::ID)
        || output_mint.eq(&stsol::ID)
    {
        return;
    }

    let source_token_account = get_associated_token_address(&whale::ID, &input_mint);
    let source_balance = RPC
        .get_token_account_balance(&source_token_account)
        .map_err(|err| {
            println!("Could not swap {} to {}", input_mint, output_mint);
            err
        })
        .unwrap();

    let destination_token_account = get_associated_token_address(&whale::ID, &output_mint);
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
        amount,
        input_mint,
        output_mint,
        swap_mode: SwapMode::default(),
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
            let destination_token_account = get_associated_token_address(&whale::ID, &output_mint);
            let ix = STAKEDEX
                .swap_via_stake_ix(
                    &SwapParams {
                        jupiter_program_id: &jupiter_program::ID,
                        in_amount: quote.in_amount,
                        destination_mint: output_mint,
                        source_mint: input_mint,
                        destination_token_account,
                        source_token_account,
                        token_transfer_authority: whale::ID,
                        open_order_address: None,
                        quote_mint_to_referrer: None,
                        out_amount: quote.out_amount,
                    },
                    0,
                )
                .unwrap();
            let mut tx = Transaction::new_with_payer(&[ix], Some(&whale::ID));
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

            if result.value.err.is_some() {
                println!(
                    "Could not swap {} to {} for {} lamports.\nLogs: {:?}",
                    input_mint, output_mint, amount, result.value
                );
                panic!();
            }

            let res_accounts = result.value.accounts.unwrap();
            let res_source_account = res_accounts[0].as_ref().unwrap();
            let res_destination_account = res_accounts[1].as_ref().unwrap();

            let (decoded_source_account, decoded_destination_account) = (
                res_source_account.decode::<Account>().unwrap(),
                res_destination_account.decode::<Account>().unwrap(),
            );

            let after_source_amount =
                spl_token::state::Account::unpack(&decoded_source_account.data)
                    .unwrap()
                    .amount;
            let after_destination_amount =
                spl_token::state::Account::unpack(&decoded_destination_account.data)
                    .unwrap()
                    .amount;

            println!("Before input balance: {:?}\nAfter input balance: {:?}\nBefore output balance: {:?}\nAfter output balance: {:?}", before_source_amount, after_source_amount, before_destination_amount, after_destination_amount);

            assert_eq!(quote.in_amount, before_source_amount - after_source_amount);
            assert_eq!(
                quote.out_amount,
                after_destination_amount - before_destination_amount
            );
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
    let max_withdraw_lamports_bef_fees = u128::from(max_withdraw_lamports)
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
            amount: max_withdraw_jsol,
            input_mint: jsol::ID,
            output_mint: msol::ID,
            swap_mode: SwapMode::default(),
        })
        .unwrap();
    let should_fail = STAKEDEX.quote_swap_via_stake(&QuoteParams {
        amount: max_withdraw_jsol + 1,
        input_mint: jsol::ID,
        output_mint: msol::ID,
        swap_mode: SwapMode::default(),
    });
    assert!(should_fail.is_err());

    // try simulating max possible quote
    let result = sim_swap_via_stake(
        &STAKEDEX,
        &RPC,
        TestSwapViaStakeArgs {
            amount: max_possible_quote.in_amount,
            input_mint: jsol::ID,
            output_mint: msol::ID,
            signer: whale::ID,
            src_token_acc: get_associated_token_address(&whale::ID, &jsol::ID),
            dst_token_acc: get_associated_token_address(&whale::ID, &msol::ID),
        },
    );
    assert_sim_success(&result);
}

pub struct TestSwapViaStakeArgs {
    pub amount: u64,
    pub input_mint: Pubkey,
    pub output_mint: Pubkey,
    pub signer: Pubkey,
    pub src_token_acc: Pubkey,
    pub dst_token_acc: Pubkey,
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
        amount,
        input_mint,
        output_mint,
        signer,
        src_token_acc,
        dst_token_acc,
    }: TestSwapViaStakeArgs,
) -> Response<RpcSimulateTransactionResult> {
    let quote = stakedex
        .quote_swap_via_stake(&QuoteParams {
            amount,
            input_mint,
            output_mint,
            swap_mode: SwapMode::default(),
        })
        .unwrap();
    // println!("{:?}", quote);
    let ix = stakedex
        .swap_via_stake_ix(
            &SwapParams {
                jupiter_program_id: &jupiter_program::ID,
                in_amount: quote.in_amount,
                out_amount: quote.out_amount,
                destination_mint: output_mint,
                source_mint: input_mint,
                destination_token_account: dst_token_acc,
                source_token_account: src_token_acc,
                token_transfer_authority: signer,
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
    rpc.simulate_transaction_with_config(
        &tx,
        RpcSimulateTransactionConfig {
            accounts: Some(RpcSimulateTransactionAccountsConfig {
                addresses: vec![src_token_acc.to_string(), dst_token_acc.to_string()],
                encoding: Some(UiAccountEncoding::Base64),
            }),
            ..RpcSimulateTransactionConfig::default()
        },
    )
    .unwrap()
}
