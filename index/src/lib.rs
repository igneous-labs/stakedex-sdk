use std::collections::HashMap;

use jupiter_core::amm::{Amm, KeyedAccount};
use solana_program::{
    clock::Clock,
    pubkey::Pubkey,
    sysvar::{self, clock},
};
use stakedex_interface::{
    DexRecord, DexRecordAccount, DexRecordDepositSol, DexRecordOneWayPoolPair,
    DexRecordTwoWayPoolPair,
};

mod deposit_sol;
mod one_way_pool_pair;
mod two_way_pool_pair;

pub use deposit_sol::*;
pub use one_way_pool_pair::*;
pub use two_way_pool_pair::*;

pub fn init_accounts_to_fetch(dex_record: &DexRecordAccount) -> Vec<Pubkey> {
    match &dex_record.record {
        DexRecord::DepositSol(s) => init_accounts_to_fetch_deposit_sol(s).to_vec(),
        DexRecord::OneWayPoolPair(s) => init_accounts_to_fetch_one_way_pool_pair(s).to_vec(),
        DexRecord::TwoWayPoolPair(s) => init_accounts_to_fetch_two_way_pool_pair(s).to_vec(),
    }
}

fn init_accounts_to_fetch_deposit_sol(s: &DexRecordDepositSol) -> [Pubkey; 1] {
    [s.main_account]
}

fn init_accounts_to_fetch_one_way_pool_pair(s: &DexRecordOneWayPoolPair) -> [Pubkey; 3] {
    [
        s.withdraw_stake_main_account,
        s.deposit_stake_main_account,
        clock::ID,
    ]
}

fn init_accounts_to_fetch_two_way_pool_pair(s: &DexRecordTwoWayPoolPair) -> [Pubkey; 3] {
    [s.a_main_account, s.b_main_account, clock::ID]
}

fn get_keyed_account(
    accounts_map: &HashMap<Pubkey, solana_sdk::account::Account>,
    key: &Pubkey,
) -> Result<KeyedAccount, anyhow::Error> {
    Ok(KeyedAccount {
        key: *key,
        account: accounts_map
            .get(key)
            .ok_or_else(|| anyhow::anyhow!("Missing account {}", key))?
            .clone(),
        params: None,
    })
}

pub fn init_amm(
    dex_record: &DexRecordAccount,
    accounts_map: &HashMap<Pubkey, solana_sdk::account::Account>,
) -> Result<Box<dyn Amm>, anyhow::Error> {
    match &dex_record.record {
        DexRecord::DepositSol(s) => init_deposit_sol(s, accounts_map),
        DexRecord::OneWayPoolPair(s) => init_one_way_pool_pair(s, accounts_map),
        DexRecord::TwoWayPoolPair(s) => init_two_way_pool_pair(s, accounts_map),
    }
}

fn init_deposit_sol(
    dex_record_deposit_sol: &DexRecordDepositSol,
    accounts_map: &HashMap<Pubkey, solana_sdk::account::Account>,
) -> Result<Box<dyn Amm>, anyhow::Error> {
    let main_account_pk = init_accounts_to_fetch_deposit_sol(dex_record_deposit_sol)[0];
    let main_account = get_keyed_account(accounts_map, &main_account_pk)?;
    load_deposit_sol(&dex_record_deposit_sol.ty, &main_account)
}

fn init_one_way_pool_pair(
    dex_record_one_way_pool_pair: &DexRecordOneWayPoolPair,
    accounts_map: &HashMap<Pubkey, solana_sdk::account::Account>,
) -> Result<Box<dyn Amm>, anyhow::Error> {
    let keys = init_accounts_to_fetch_one_way_pool_pair(dex_record_one_way_pool_pair);
    let withdraw_stake_main_account_pk = keys[0];
    let deposit_stake_main_account_pk = keys[1];

    let clock_acc = accounts_map
        .get(&sysvar::clock::ID)
        .ok_or_else(|| anyhow::anyhow!("Missing account clock"))?;
    let clock: Clock = bincode::deserialize(clock_acc.data.as_ref())?;

    let withdraw_stake_main_account =
        get_keyed_account(accounts_map, &withdraw_stake_main_account_pk)?;
    let deposit_stake_main_account =
        get_keyed_account(accounts_map, &deposit_stake_main_account_pk)?;
    load_one_way_pool_pair(
        &dex_record_one_way_pool_pair.withdraw_stake_ty,
        &withdraw_stake_main_account,
        &dex_record_one_way_pool_pair.deposit_stake_ty,
        &deposit_stake_main_account,
        clock,
    )
}

fn init_two_way_pool_pair(
    dex_record_two_way_pool_pair: &DexRecordTwoWayPoolPair,
    accounts_map: &HashMap<Pubkey, solana_sdk::account::Account>,
) -> Result<Box<dyn Amm>, anyhow::Error> {
    let keys = init_accounts_to_fetch_two_way_pool_pair(dex_record_two_way_pool_pair);
    let a_main_account_pk = keys[0];
    let b_main_account_pk = keys[1];

    let clock_acc = accounts_map
        .get(&sysvar::clock::ID)
        .ok_or_else(|| anyhow::anyhow!("Missing account clock"))?;
    let clock: Clock = bincode::deserialize(clock_acc.data.as_ref())?;

    let a_main_account = get_keyed_account(accounts_map, &a_main_account_pk)?;
    let b_main_account = get_keyed_account(accounts_map, &b_main_account_pk)?;
    load_two_way_pool_pair(
        &dex_record_two_way_pool_pair.a_ty,
        &a_main_account,
        &dex_record_two_way_pool_pair.b_ty,
        &b_main_account,
        clock,
    )
}
