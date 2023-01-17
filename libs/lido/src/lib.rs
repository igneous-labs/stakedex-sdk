use anyhow::Result;
use lido::state::{AccountType, Lido, Validator};

mod stakedex_traits;

use solana_program::{
    borsh::try_from_slice_unchecked, clock::Clock, program_pack::Pack, stake_history::Epoch,
};
pub use stakedex_traits::*;

pub const LIDO_LABEL: &str = "lido";

// Because ListHeader::LEN is private
pub const LIST_HEADER_LEN: usize =
    std::mem::size_of::<u32>() + std::mem::size_of::<AccountType>() + std::mem::size_of::<u8>();

#[derive(Clone, Default)]
pub struct LidoStakedex {
    lido_state: Lido,
    validator_list: Vec<Validator>,
    curr_epoch: Epoch,
}

impl LidoStakedex {
    pub fn update_lido_state(&mut self, data: &[u8]) -> Result<()> {
        self.lido_state = try_from_slice_unchecked(data)?;
        Ok(())
    }

    pub fn update_validator_list(&mut self, data: &[u8]) -> Result<()> {
        // first 4 bytes is len as u32
        let len = u32::from_le_bytes(data[..4].try_into().unwrap()) as usize;
        let mut validator_list = Vec::with_capacity(len);
        let validator_iter = data[4..].chunks_exact(Validator::LEN).enumerate();
        for (index, record) in validator_iter {
            if len == index {
                break;
            }
            validator_list.push(try_from_slice_unchecked::<Validator>(record)?);
        }
        self.validator_list = validator_list;
        Ok(())
    }

    pub fn update_curr_epoch(&mut self, clock_data: &[u8]) -> Result<()> {
        let clock: Clock = bincode::deserialize(clock_data)?;
        self.curr_epoch = clock.epoch;
        Ok(())
    }
}
