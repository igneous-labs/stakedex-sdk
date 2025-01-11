use jupiter_amm_interface::{
    AccountMap, AmmContext, ClockRef, Quote, QuoteParams, SwapMode, SwapParams,
};
use lazy_static::lazy_static;
use sanctum_lst_list::SanctumLstList;
use solana_account_decoder::UiAccountEncoding;
use solana_client::{
    rpc_client::RpcClient,
    rpc_config::{RpcSimulateTransactionAccountsConfig, RpcSimulateTransactionConfig},
};
use solana_sdk::{
    account::Account,
    address_lookup_table::{state::AddressLookupTable, AddressLookupTableAccount},
    clock::Clock,
    compute_budget,
    instruction::Instruction,
    message::{v0::Message, VersionedMessage},
    program_pack::Pack,
    pubkey::Pubkey,
    sysvar,
    transaction::VersionedTransaction,
};
use spl_associated_token_account::get_associated_token_address;
use spl_token::native_mint;
use stakedex_sdk::{srlut, Stakedex, SWAP_VIA_STAKE_COMPUTE_BUDGET_LIMIT};
use stakedex_sdk_common::{bsol, jitosol, jsol, msol, pwrsol};
use std::{cmp, iter::zip};

// JSOL whale. Last known balances:
// - SOL: 1 (enough for a new token account)
// - JSOL: 175721.072392432
pub mod whale {
    solana_sdk::declare_id!("5HByfdAGKAJ44Qsq7pLX65n8faur4mvCT81Nqc8LusUL");
}

pub mod jupiter_program {
    // NOT IN USE, JUST BECAUSE ITS REQUIRED AS A STRUCT FIELD FOR jupiter_amm_interface::SwapParams
    solana_sdk::declare_id!("JUP4Fb2cqiRUcaTHdrPC8h2gNsA2ETXiPDD33WcGuJB");
}

fn get_clock() -> Clock {
    bincode::deserialize::<Clock>(&RPC.get_account(&sysvar::clock::ID).unwrap().data).unwrap()
}

lazy_static! {
    static ref RPC: RpcClient = RpcClient::new(std::env::var("SOLANA_RPC_URL").unwrap());
    static ref STAKEDEX: Stakedex = {
        let sll = SanctumLstList::load().sanctum_lst_list;
        let init_accounts = fetch_accounts(&Stakedex::init_accounts(sll.iter()));
        let amm_context = AmmContext {
            clock_ref: ClockRef::from(get_clock()),
        };
        let (mut stakedex, errs) = Stakedex::from_fetched_accounts(sll.iter(), &init_accounts, &amm_context);
        if !errs.is_empty() {
            eprintln!("init errs {:?}", errs);
        }
        // Need to update() one more time to fetch auxilliary accounts (e.g. validatorlist)
        let update_accounts = fetch_accounts(&stakedex.get_accounts_to_update());
        let errs = stakedex.update(&update_accounts);
        if !errs.is_empty() {
            eprintln!("update errs {:?}", errs);
        }
        stakedex
    };
    // With the change to PrefundSwapViaStake, all TXs now must use a LUT or it wont fit
    static ref SRLUT: AddressLookupTableAccount = {
        let srlut_acc_data = RPC.get_account_data(&srlut::ID).unwrap();
        let AddressLookupTable { addresses, .. } = AddressLookupTable::deserialize(&srlut_acc_data).unwrap();
        AddressLookupTableAccount {
            key: srlut::ID,
            addresses: Vec::from(addresses),
        }
    };
}

fn fetch_accounts(accounts_pubkeys: &[Pubkey]) -> AccountMap {
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

const SMALL_JSOL_SWAP_AMT: u64 = 10_000_000_000; // 10 JSOL

// unstakeit

#[test]
fn test_swap_via_stake_jsol_unstakeit() {
    test_prefund_swap_via_stake(jsol::ID, native_mint::ID, SMALL_JSOL_SWAP_AMT);
}

#[test]
fn test_manual_concat_swap_via_stake_jsol_unstakeit() {
    test_manual_concat_prefund_swap_via_stake(jsol::ID, native_mint::ID, SMALL_JSOL_SWAP_AMT);
}

/*
// scnsol temporarily disabled before migration
#[test]
fn test_swap_via_stake_scnsol_unstakeit() {
    test_swap_via_stake(scnsol::ID, native_mint::ID, u64::MAX);
}
*/

// jsol to xsol

/*
// no route
#[test]
fn test_swap_via_stake_jsol_bsol() {
    test_swap_via_stake(jsol::ID, bsol::ID, SMALL_JSOL_SWAP_AMT);
}
*/

/*
// no route
#[test]
fn test_swap_via_stake_jsol_cogentsol() {
    test_swap_via_stake(jsol::ID, cogentsol::ID, SMALL_JSOL_SWAP_AMT);
}
*/

// jito already tests spl -> spl
/*
#[test]
fn test_swap_via_stake_jsol_daosol() {
    test_prefund_swap_via_stake(jsol::ID, daosol::ID, SMALL_JSOL_SWAP_AMT);
}

#[test]
fn test_manual_concat_swap_via_stake_jsol_daosol() {
    test_manual_concat_prefund_swap_via_stake(jsol::ID, daosol::ID, SMALL_JSOL_SWAP_AMT);
}
 */

#[test]
fn test_swap_via_stake_jsol_jitosol() {
    test_prefund_swap_via_stake(jsol::ID, jitosol::ID, SMALL_JSOL_SWAP_AMT);
}

#[test]
fn test_manual_concat_swap_via_stake_jsol_jitosol() {
    test_manual_concat_prefund_swap_via_stake(jsol::ID, jitosol::ID, SMALL_JSOL_SWAP_AMT);
}

#[test]
fn test_swap_via_stake_jsol_pwrsol() {
    test_prefund_swap_via_stake(jsol::ID, pwrsol::ID, SMALL_JSOL_SWAP_AMT);
}

#[test]
fn test_manual_concat_swap_via_stake_jsol_pwrsol() {
    test_manual_concat_prefund_swap_via_stake(jsol::ID, pwrsol::ID, SMALL_JSOL_SWAP_AMT);
}

/*
// no route
#[test]
fn test_swap_via_stake_jsol_lainesol() {
    test_swap_via_stake(jsol::ID, lainesol::ID, SMALL_JSOL_SWAP_AMT);
}
 */

/*
// Stake pool cannot accept stake deposits at this time, riskSOL appears to be unmaintained
#[test]
fn test_swap_via_stake_jsol_risksol() {
    test_swap_via_stake(jsol::ID, risksol::ID, SMALL_JSOL_SWAP_AMT);
}
*/

/*
// scnsol temporarily disabled before migration
#[test]
fn test_swap_via_stake_jsol_scnsol() {
    test_swap_via_stake(jsol::ID, scnsol::ID, SMALL_JSOL_SWAP_AMT);
}
 */

#[test]
fn test_swap_via_stake_jsol_msol() {
    test_prefund_swap_via_stake(jsol::ID, msol::ID, SMALL_JSOL_SWAP_AMT);
}

#[test]
fn test_manual_concat_swap_via_stake_jsol_msol() {
    test_manual_concat_prefund_swap_via_stake(jsol::ID, msol::ID, SMALL_JSOL_SWAP_AMT);
}

// scnsol to xsol

/*
// no route
#[test]
fn test_swap_via_stake_scnsol_bsol() {
    test_swap_via_stake(scnsol::ID, bsol::ID, u64::MAX);
}
*/

/*
// no route
#[test]
fn test_swap_via_stake_scnsol_cogentsol() {
    test_swap_via_stake(scnsol::ID, cogentsol::ID, u64::MAX);
}
*/

/*
// no route
#[test]
fn test_swap_via_stake_scnsol_daosol() {
    test_swap_via_stake(scnsol::ID, daosol::ID, u64::MAX);
}
*/

/*
// scnsol temporarily disabled before migration
#[test]
fn test_swap_via_stake_scnsol_jitosol() {
    test_swap_via_stake(scnsol::ID, jitosol::ID, u64::MAX);
}
*/

/*
// scnsol temporarily disabled before migration
#[test]
fn test_swap_via_stake_scnsol_jsol() {
    test_swap_via_stake(scnsol::ID, jsol::ID, u64::MAX);
}
*/

/*
// no route
#[test]
fn test_swap_via_stake_scnsol_lainesol() {
    test_swap_via_stake(scnsol::ID, lainesol::ID, u64::MAX);
}
*/

/*
// Stake pool cannot accept stake deposits at this time, riskSOL appears to be unmaintained
#[test]
fn test_swap_via_stake_scnsol_risksol() {
    test_swap_via_stake(scnsol::ID, risksol::ID, u64::MAX);
}
 */

/*
// scnsol temporarily disabled before migration
#[test]
fn test_swap_via_stake_scnsol_msol() {
    test_swap_via_stake(scnsol::ID, msol::ID, u64::MAX);
}
*/

// Set amount to u64::MAX to swap the entire input ATA balance
fn test_prefund_swap_via_stake(input_mint: Pubkey, output_mint: Pubkey, amount: u64) {
    test_sim_prefund_swap_via_stake(
        &STAKEDEX,
        TestSwapViaStakeArgs {
            amount,
            input_mint,
            output_mint,
            signer: whale::ID,
            src_token_acc: get_associated_token_address(&whale::ID, &input_mint),
            dst_token_acc: get_associated_token_address(&whale::ID, &output_mint),
        },
    );
}

// Set amount to u64::MAX to swap the entire input ATA balance
fn test_manual_concat_prefund_swap_via_stake(input_mint: Pubkey, output_mint: Pubkey, amount: u64) {
    test_sim_manual_concat_prefund_swap_via_stake(
        &STAKEDEX,
        TestSwapViaStakeArgs {
            amount,
            input_mint,
            output_mint,
            signer: whale::ID,
            src_token_acc: get_associated_token_address(&whale::ID, &input_mint),
            dst_token_acc: get_associated_token_address(&whale::ID, &output_mint),
        },
    );
}

/*
// TODO: currently no shared validators between JPool and marinade,
// need to deposit into other pool to test this case
#[test]
fn test_jsol_drain_vsa_edge_case() {
    // assumes
    // - jsol has no preferred validator
    // - marinade should accept any of jpool's vsas
    // - WHALE has 350k JSOL, that should be > any individual vsa of jpool
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
    sim_swap_via_stake(
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
}
*/

#[derive(Clone, Copy, Debug)]
pub struct TestSwapViaStakeArgs {
    pub amount: u64,
    pub input_mint: Pubkey,
    pub output_mint: Pubkey,
    pub signer: Pubkey,
    pub src_token_acc: Pubkey,
    pub dst_token_acc: Pubkey,
}

/// - uses min(amount, src_balance) as input amount
/// - if dst_token_acc is signer's ATA and doesn't exist, prefixes
///   the simulated tx with a create ATA instruction
///
/// Returns (amt_to_swap, prefix_ixs, before_source_amount, before_destination_amount)
fn setup_swap_via_stake(
    TestSwapViaStakeArgs {
        amount,
        input_mint,
        output_mint,
        signer,
        src_token_acc,
        dst_token_acc,
    }: TestSwapViaStakeArgs,
) -> (u64, Vec<Instruction>, u64, u64) {
    let source_balance = RPC
        .get_token_account_balance(&src_token_acc)
        .inspect_err(|err| {
            println!("Could not swap {input_mint} to {output_mint}: {err}");
        })
        .unwrap();

    let mut ixs = Vec::with_capacity(4);
    ixs.extend([
        compute_budget::ComputeBudgetInstruction::set_compute_unit_limit(
            SWAP_VIA_STAKE_COMPUTE_BUDGET_LIMIT,
        ),
        compute_budget::ComputeBudgetInstruction::set_compute_unit_price(3),
    ]);

    let (before_destination_amount, prefix_ixs) =
        match RPC.get_token_account_balance(&dst_token_acc) {
            Ok(b) => (b.amount.parse().unwrap(), ixs),
            Err(_e) => {
                let ata = get_associated_token_address(&signer, &output_mint);
                if dst_token_acc != ata {
                    panic!("dst_token_acc {dst_token_acc} does not exist and is not ATA");
                }
                ixs.push(
                    spl_associated_token_account::instruction::create_associated_token_account(
                        &signer,
                        &signer,
                        &output_mint,
                        // TODO: support token-22
                        &spl_token::ID,
                    ),
                );
                (0, ixs)
            }
        };
    let before_source_amount: u64 = source_balance.amount.parse().unwrap();
    let amount = cmp::min(before_source_amount, amount);

    (
        amount,
        prefix_ixs,
        before_source_amount,
        before_destination_amount,
    )
}

fn quote_swap_via_stake(
    stakedex: &Stakedex,
    amount: u64,
    TestSwapViaStakeArgs {
        input_mint,
        output_mint,
        ..
    }: TestSwapViaStakeArgs,
) -> Quote {
    match stakedex.quote_swap_via_stake(&QuoteParams {
        amount,
        input_mint,
        output_mint,
        swap_mode: SwapMode::default(),
    }) {
        Ok(q) => q,
        Err(err) => {
            panic!(
                "Could not swap {} {} to {}. Reason: {}",
                amount, input_mint, output_mint, err
            );
            /*
            // EDIT: dont ignore errors, comment out the tests instead
            //
            // - ignores these errors:
            //     - no route found between pools
            //     - stake pool cannot accept stake deposits at this time
            let estr = err.to_string();
            if estr != "No route found between pools"
                && estr != "Stake pool cannot accept stake deposits at this time"
            {
                panic!("{estr}");
            }
            return;
             */
        }
    }
}

fn simulate_check_swap_via_stake(
    TestSwapViaStakeArgs {
        amount,
        input_mint,
        output_mint,
        signer,
        src_token_acc,
        dst_token_acc,
    }: TestSwapViaStakeArgs,
    quote: Quote,
    ixs: Vec<Instruction>,
    before_source_amount: u64,
    before_destination_amount: u64,
) {
    let rbh = RPC.get_latest_blockhash().unwrap();
    let tx = VersionedTransaction {
        signatures: vec![Default::default()], // for payer
        message: VersionedMessage::V0(
            Message::try_compile(&signer, &ixs, &[SRLUT.to_owned()], rbh).unwrap(),
        ),
    };

    let result = RPC
        .simulate_transaction_with_config(
            &tx,
            RpcSimulateTransactionConfig {
                accounts: Some(RpcSimulateTransactionAccountsConfig {
                    addresses: vec![src_token_acc.to_string(), dst_token_acc.to_string()],
                    encoding: Some(UiAccountEncoding::Base64), // UiAccount::decode::<Account>() does NOT work for JSONn
                }),
                ..RpcSimulateTransactionConfig::default()
            },
        )
        .unwrap();

    if result.value.err.is_some() {
        panic!(
            "Could not swap {} to {} for {} lamports.\nLogs: {:?}",
            input_mint, output_mint, amount, result.value
        );
    }

    let res_accounts = result.value.accounts.unwrap();
    let res_source_account = res_accounts[0].as_ref().unwrap();
    let res_destination_account = res_accounts[1].as_ref().unwrap();

    let (decoded_source_account, decoded_destination_account) = (
        res_source_account.decode::<Account>().unwrap(),
        res_destination_account.decode::<Account>().unwrap(),
    );

    let after_source_amount = spl_token::state::Account::unpack(&decoded_source_account.data)
        .unwrap()
        .amount;
    let after_destination_amount =
        spl_token::state::Account::unpack(&decoded_destination_account.data)
            .unwrap()
            .amount;

    // println!("Before input balance: {:?}\nAfter input balance: {:?}\nBefore output balance: {:?}\nAfter output balance: {:?}", before_source_amount, after_source_amount, before_destination_amount, after_destination_amount);

    assert_eq!(quote.in_amount, before_source_amount - after_source_amount);
    let actual_out_amount = after_destination_amount - before_destination_amount;
    // TODO: this can be off by a couple of lamports for anything -unstakeit-> SOL
    // depending on size, (around 1 lamport per x10 JSOL)
    // presumably due to precision losses in pseudo_reverse()
    assert_eq!(quote.out_amount, actual_out_amount);
}

pub fn test_sim_prefund_swap_via_stake(stakedex: &Stakedex, args: TestSwapViaStakeArgs) {
    let TestSwapViaStakeArgs {
        input_mint,
        output_mint,
        signer,
        src_token_acc,
        dst_token_acc,
        ..
    } = args;

    let (amount, mut ixs, before_source_amount, before_destination_amount) =
        setup_swap_via_stake(args);

    let quote = quote_swap_via_stake(stakedex, amount, args);

    ixs.push(
        stakedex
            .prefund_swap_via_stake_ix(
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
                    missing_dynamic_accounts_as_default: false,
                    swap_mode: SwapMode::ExactIn,
                },
                0,
            )
            .unwrap(),
    );

    simulate_check_swap_via_stake(
        args,
        quote,
        ixs,
        before_source_amount,
        before_destination_amount,
    );
}

pub fn test_sim_manual_concat_prefund_swap_via_stake(
    stakedex: &Stakedex,
    args: TestSwapViaStakeArgs,
) {
    let TestSwapViaStakeArgs {
        input_mint,
        output_mint,
        signer,
        src_token_acc,
        dst_token_acc,
        ..
    } = args;

    let (amount, mut ixs, before_source_amount, before_destination_amount) =
        setup_swap_via_stake(args);

    let quote = quote_swap_via_stake(stakedex, amount, args);

    ixs.extend(
        stakedex
            .manual_concat_prefund_swap_via_stake_ixs(
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
                    missing_dynamic_accounts_as_default: false,
                    swap_mode: SwapMode::ExactIn,
                },
                0,
            )
            .unwrap(),
    );

    simulate_check_swap_via_stake(
        args,
        quote,
        ixs,
        before_source_amount,
        before_destination_amount,
    );
}
