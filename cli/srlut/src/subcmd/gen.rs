use std::{fs::File, path::PathBuf};

use clap::Args;
use solana_sdk::{pubkey::Pubkey, stake, system_program, sysvar};
use spl_token::native_mint;
use stakedex_sdk_common::{stakedex_program, unstake_it_program, wsol_bridge_in};

use crate::lut_list::LutList;

use super::Subcmd;

#[derive(Args, Debug)]
#[clap(long_about = "Generate the json pubkey list of accounts that the LUT should contain")]
pub struct GenArgs {
    #[clap(
        long,
        short,
        default_value = "./keys.json",
        help = "Path to save output json file to"
    )]
    pub out: PathBuf,
}

impl GenArgs {
    pub async fn run(args: crate::Args) {
        let Self { out } = match args.subcmd {
            Subcmd::Gen(a) => a,
            _ => unreachable!(),
        };

        let keys = [
            SYSVARS.as_slice(),
            COMMON_PROGRAMS.as_slice(),
            COMMON_MINTS.as_slice(),
            RESERVE_POOL_ACCOUNTS.as_slice(),
            ROUTER_PROGRAM_ACCOUNTS.as_slice(),
        ]
        .concat();

        let file = File::create(out).unwrap();
        serde_json::to_writer(file, &LutList(keys)).unwrap();

        eprintln!("Done");
    }
}

const SYSVARS: [Pubkey; 9] = [
    sysvar::clock::ID,
    sysvar::rent::ID,
    sysvar::stake_history::ID,
    sysvar::epoch_schedule::ID,
    sysvar::instructions::ID,
    sysvar::rewards::ID,
    sysvar::epoch_rewards::ID,
    sysvar::slot_hashes::ID,
    sysvar::slot_history::ID,
];

const COMMON_PROGRAMS: [Pubkey; 4] = [
    system_program::ID,
    stake::program::ID,
    spl_token::ID,
    spl_associated_token_account::ID,
];

const COMMON_MINTS: [Pubkey; 1] = [native_mint::ID];

const RESERVE_POOL_ACCOUNTS: [Pubkey; 4] = [
    unstake_it_program::ID,
    unstake_it_program::SOL_RESERVES_ID,
    unstake_it_program::FEE_ID,
    unstake_it_program::PROTOCOL_FEE_ID,
];

const ROUTER_PROGRAM_ACCOUNTS: [Pubkey; 4] = [
    stakedex_program::ID,
    stakedex_program::SOL_BRIDGE_OUT_ID,
    stakedex_program::PREFUNDER_ID,
    wsol_bridge_in::ID,
];
