use std::{fs::File, path::PathBuf};

use clap::Args;
use solana_sdk::{pubkey::Pubkey, stake, system_program, sysvar};
use spl_token::native_mint;

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

pub mod reserve_pool_program {
    // \xde\...\xc6 = reserve pool
    sanctum_macros::declare_program_keys!(
        "unpXTU2Ndrc7WWNyEhQWe4udTzSibLPi25SXv2xbCHQ",
        [
            ("sol_reserves", b"\xde\x91\xbbP4tnb;\xfb6\xb7=\xae\"\xa4\x83\xb7\xcf\'\xd2\xad\x83\xfa\x8cx\x05\xa6\xcc`+\xc6"),
            ("fee", b"\xde\x91\xbbP4tnb;\xfb6\xb7=\xae\"\xa4\x83\xb7\xcf\'\xd2\xad\x83\xfa\x8cx\x05\xa6\xcc`+\xc6", b"fee"),
            ("protocol_fee", b"protocol-fee")
        ]
    );
}

const RESERVE_POOL_ACCOUNTS: [Pubkey; 4] = [
    reserve_pool_program::ID,
    reserve_pool_program::SOL_RESERVES_ID,
    reserve_pool_program::FEE_ID,
    reserve_pool_program::PROTOCOL_FEE_ID,
];

pub mod router_program {
    sanctum_macros::declare_program_keys!(
        "stkitrT1Uoy18Dk1fTrgPw8W6MVzoCfYoAFT4MLsmhq",
        [
            ("sol-bridge-out", b"sol_bridge_out"),
            ("prefunder", b"prefunder"),
        ]
    );
}

const ROUTER_PROGRAM_ACCOUNTS: [Pubkey; 3] = [
    router_program::ID,
    router_program::SOL_BRIDGE_OUT_ID,
    router_program::PREFUNDER_ID,
];
