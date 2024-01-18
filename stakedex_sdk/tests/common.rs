use std::{collections::HashMap, fs, iter::zip};

use sanctum_solana_test_utils::{test_fixtures_dir, ExtendedProgramTest};
use solana_program_test::{BanksClient, ProgramTest, ProgramTestContext};
use solana_sdk::{account::Account, clock::Clock, hash::Hash, pubkey::Pubkey, signature::Keypair};
use stakedex_sdk::Stakedex;

const TEST_FIXTURES_EPOCH: u64 = 561;

pub fn stakedex_program_test() -> ProgramTest {
    let mut pt = ProgramTest::default();
    let rd = fs::read_dir(test_fixtures_dir()).unwrap();
    for entry in rd {
        let entry = entry.unwrap();
        let path = entry.path();
        if let Some(ext) = path.extension() {
            if ext == "json" {
                pt = pt.add_test_fixtures_account(path.file_name().unwrap());
            }
        }
    }
    pt
}

pub async fn start_bc(pt: ProgramTest) -> (BanksClient, Keypair, Hash) {
    let ctx = pt.start_with_context().await;
    ctx.set_sysvar(&Clock {
        epoch: TEST_FIXTURES_EPOCH,
        ..Default::default()
    });
    let ProgramTestContext {
        banks_client,
        payer,
        last_blockhash,
        ..
    } = ctx;
    (banks_client, payer, last_blockhash)
}

async fn bc_fetch_accounts(bc: &mut BanksClient, keys: &[Pubkey]) -> HashMap<Pubkey, Account> {
    let mut accounts = Vec::with_capacity(keys.len());
    for pk in keys {
        let a = bc.get_account(*pk).await.unwrap();
        accounts.push(a);
    }
    zip(keys, accounts)
        // errors silently on missing accounts
        .filter_map(|(pubkey, opt)| opt.map(|acc| (*pubkey, acc)))
        .collect()
}

/// Ignores init errors, since not all pools are going to be initialized
pub async fn init_stakedex(bc: &mut BanksClient) -> Stakedex {
    let init_keys = Stakedex::init_accounts();
    let init_accounts = bc_fetch_accounts(bc, &init_keys).await;
    let (mut stakedex, _errs) = Stakedex::from_fetched_accounts(&init_accounts);
    let update_accounts = bc_fetch_accounts(bc, &stakedex.get_accounts_to_update()).await;
    let _errs = stakedex.update(update_accounts);
    stakedex
}
