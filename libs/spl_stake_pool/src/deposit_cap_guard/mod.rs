use anyhow::{anyhow, Result};
use solana_program::{
    instruction::{AccountMeta, Instruction},
    pubkey::Pubkey,
};
use stakedex_sdk_common::spl_deposit_cap_guard_program;

pub fn find_spl_deposit_cap_guard_state(
    program_id: &Pubkey,
    spl_stake_pool: &Pubkey,
) -> (Pubkey, u8) {
    Pubkey::find_program_address(&[spl_stake_pool.as_ref()], program_id)
}

const LAMPORTS_TY_DISCM: u8 = 1;

const LST_ATOMICS_TY_DISCM: u8 = 2;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DepositCap {
    Lamports(u64),
    LstAtomics(u64),
}

impl DepositCap {
    pub fn try_from_buf(buf: &[u8; 9]) -> Result<Self> {
        let (ty, amt) = buf.split_last().unwrap();
        let amt: &[u8; 8] = amt.try_into().unwrap();
        let amt = u64::from_le_bytes(*amt);
        Ok(match *ty {
            LAMPORTS_TY_DISCM => Self::Lamports(amt),
            LST_ATOMICS_TY_DISCM => Self::LstAtomics(amt),
            _ => Err(anyhow!("invalid deposit cap"))?,
        })
    }
}

pub fn to_deposit_cap_guard_ix(
    mut spl_deposit_ix: Instruction,
    deposit_cap_guard_state: Pubkey,
) -> Instruction {
    spl_deposit_ix.accounts.splice(
        0..0,
        [
            AccountMeta {
                pubkey: spl_deposit_cap_guard_program::ID,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: deposit_cap_guard_state,
                is_signer: false,
                is_writable: false,
            },
        ],
    );
    spl_deposit_ix
}
