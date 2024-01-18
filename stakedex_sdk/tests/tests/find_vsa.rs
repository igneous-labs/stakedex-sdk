use solana_sdk::pubkey::Pubkey;
use spl_token::native_mint;
use stakedex_sdk_common::{bsol, first_avail_quote, msol, scnsol, stsol, DepositStakeInfo};

use crate::common::{init_stakedex, stakedex_program_test, start_bc};

const WITHDRAW_STAKE_FROM_TOKENS: [(Pubkey, usize); 3] =
    [(bsol::ID, 4), (scnsol::ID, 4), (stsol::ID, 3)];
const DEPOSIT_STAKE_TO_TOKENS: [(Pubkey, Option<usize>); 4] = [
    (bsol::ID, Some(5)),
    (scnsol::ID, Some(5)),
    (msol::ID, None),
    (native_mint::ID, None),
];

pub const QUOTE_AMT: u64 = 1_000_000_000;

#[tokio::test]
async fn find_vsa() {
    let pt = stakedex_program_test();
    let (mut bc, _payer, _last_blockhash) = start_bc(pt).await;
    let stakedex = init_stakedex(&mut bc).await;
    for (from, split_offset) in WITHDRAW_STAKE_FROM_TOKENS {
        for (to, merge_offset) in DEPOSIT_STAKE_TO_TOKENS {
            if from == to {
                continue;
            }
            let withdraw_from = stakedex.get_withdraw_stake_pool(&from).unwrap();
            let deposit_to = stakedex.get_deposit_stake_pool(&to).unwrap();
            let (withdraw_quote, deposit_quote) =
                match first_avail_quote(QUOTE_AMT, withdraw_from, deposit_to) {
                    Ok(q) => q,
                    Err(e) => {
                        println!("{from} -> {to} failed: {e}");
                        continue;
                    }
                };
            let withdraw_ix = withdraw_from.virtual_ix(&withdraw_quote).unwrap();
            let split_from = withdraw_ix.accounts[split_offset].pubkey;
            let deposit_stake_info = DepositStakeInfo {
                addr: Pubkey::default(),
            };
            let deposit_ix = deposit_to
                .virtual_ix(&deposit_quote, &deposit_stake_info)
                .unwrap();
            let merge_to = merge_offset.map(|i| deposit_ix.accounts[i].pubkey);
            println!(
                "{from} | {split_from} | {to} | {}",
                merge_to.map(|pk| pk.to_string()).unwrap_or_default()
            );
        }
    }
}
