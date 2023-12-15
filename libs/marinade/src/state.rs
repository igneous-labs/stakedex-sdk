use anyhow::{anyhow, Result};
use marinade_finance_interface::State;

use crate::calc::shares_from_value;

/// NewType for copy-pasta-ing marinade state methods
pub struct StateWrapper<'a>(pub &'a State);

impl<'a> StateWrapper<'a> {
    pub fn calc_msol_from_lamports(&self, stake_lamports: u64) -> Result<u64> {
        shares_from_value(
            stake_lamports,
            self.total_virtual_staked_lamports(),
            self.0.msol_supply,
        )
    }

    pub fn total_virtual_staked_lamports(&self) -> u64 {
        // if we get slashed it may be negative but we must use 0 instead
        self.total_lamports_under_control()
            .saturating_sub(self.0.circulating_ticket_balance) //tickets created -> cooling down lamports or lamports already in reserve and not claimed yet
    }

    pub fn total_lamports_under_control(&self) -> u64 {
        self.0
            .validator_system
            .total_active_balance
            .checked_add(self.total_cooling_down())
            .expect("Stake balance overflow")
            .checked_add(self.0.available_reserve_balance) // reserve_pda.lamports() - self.rent_exempt_for_token_acc
            .expect("Total SOLs under control overflow")
    }

    pub fn total_cooling_down(&self) -> u64 {
        self.0
            .stake_system
            .delayed_unstake_cooling_down
            .checked_add(self.0.emergency_cooling_down)
            .expect("Total cooling down overflow")
    }

    pub fn check_staking_cap(&self, transfering_lamports: u64) -> Result<()> {
        let result_amount = self
            .total_lamports_under_control()
            .checked_add(transfering_lamports)
            .ok_or_else(|| anyhow!("SOL overflow"))?;
        if result_amount > self.0.staking_sol_cap {
            return Err(anyhow!(
                "Staking cap reached {}/{}",
                result_amount,
                self.0.staking_sol_cap
            ));
        }
        Ok(())
    }
}
