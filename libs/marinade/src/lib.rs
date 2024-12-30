use anyhow::{anyhow, Result};
use borsh::BorshDeserialize;
use consts::VALIDATOR_RECORD_BYTE_LENGTH;
use marinade_finance_interface::{
    Fee, FeeCents, LiqPool, List, StakeSystem, State, ValidatorRecord, ValidatorSystem,
};
use solana_program::{borsh0_10::try_from_slice_unchecked, pubkey::Pubkey};

mod calc;
mod consts;
mod stakedex_traits;
mod state;

pub mod validator_system;

pub const MARINADE_LABEL: &str = "Marinade";

#[derive(Clone)]
pub struct MarinadeStakedex {
    pub state: State,
    pub validator_records: Vec<ValidatorRecord>,
}

impl Default for MarinadeStakedex {
    fn default() -> Self {
        let empty_list = List {
            account: Pubkey::default(),
            item_size: 0,
            count: 0,
            reserved1: Pubkey::default(),
            reserved2: 0,
        };
        let zero_fee = Fee { basis_points: 0 };
        let zero_fee_cents = FeeCents { bp_cents: 0 };
        Self {
            state: State {
                msol_mint: Pubkey::default(),
                admin_authority: Pubkey::default(),
                operational_sol_account: Pubkey::default(),
                treasury_msol_account: Pubkey::default(),
                reserve_bump_seed: 0,
                msol_mint_authority_bump_seed: 0,
                rent_exempt_for_token_acc: 0,
                reward_fee: zero_fee.clone(),
                stake_system: StakeSystem {
                    stake_list: empty_list.clone(),
                    delayed_unstake_cooling_down: 0,
                    stake_deposit_bump_seed: 0,
                    stake_withdraw_bump_seed: 0,
                    slots_for_stake_delta: 0,
                    last_stake_delta_epoch: 0,
                    min_stake: 0,
                    extra_stake_delta_runs: 0,
                },
                validator_system: ValidatorSystem {
                    validator_list: empty_list,
                    manager_authority: Pubkey::default(),
                    total_active_balance: 0,
                    total_validator_score: 0,
                    auto_add_validator_enabled: 0,
                },
                liq_pool: LiqPool {
                    lp_mint: Pubkey::default(),
                    lp_mint_authority_bump_seed: 0,
                    sol_leg_bump_seed: 0,
                    msol_leg_authority_bump_seed: 0,
                    msol_leg: Pubkey::default(),
                    lp_liquidity_target: 0,
                    lp_max_fee: zero_fee.clone(),
                    lp_min_fee: zero_fee.clone(),
                    treasury_cut: zero_fee.clone(),
                    lp_supply: 0,
                    lent_from_sol_leg: 0,
                    liquidity_sol_cap: 0,
                },
                available_reserve_balance: 0,
                msol_supply: 0,
                msol_price: 0,
                circulating_ticket_count: 0,
                circulating_ticket_balance: 0,
                lent_from_reserve: 0,
                min_deposit: 0,
                min_withdraw: 0,
                staking_sol_cap: 0,
                emergency_cooling_down: 0,
                pause_authority: Pubkey::default(),
                paused: false,
                delayed_unstake_fee: zero_fee_cents.clone(),
                withdraw_stake_account_fee: zero_fee_cents,
                withdraw_stake_account_enabled: false,
                last_stake_move_epoch: 0,
                stake_moved: 0,
                max_stake_moved_per_epoch: zero_fee,
            },
            validator_records: Vec::new(),
        }
    }
}

impl MarinadeStakedex {
    // All update methods dont check account discm

    pub fn update_state(&mut self, data: &[u8]) -> Result<()> {
        self.state = try_from_slice_unchecked::<State>(&data[8..])?;
        Ok(())
    }

    /// data is account data of state.validator_system.validator_list.account
    pub fn update_validator_records(&mut self, data: &[u8]) -> Result<()> {
        // first 8 bytes are len
        let len_slice = data
            .get(..8)
            .ok_or_else(|| anyhow!("Could not read validator records len"))?;
        let len = u64::try_from_slice(len_slice)?;
        let records_slice = data
            .get(8..)
            .ok_or_else(|| anyhow!("Could not read validator records data"))?;
        let validator_record_iter = records_slice
            .chunks_exact(VALIDATOR_RECORD_BYTE_LENGTH)
            .enumerate();
        self.validator_records.clear();
        for (index, record) in validator_record_iter {
            if len == index as u64 {
                break;
            }
            self.validator_records
                .push(try_from_slice_unchecked::<ValidatorRecord>(record)?);
        }
        Ok(())
    }
}
