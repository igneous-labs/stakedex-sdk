use anyhow::anyhow;
use borsh::BorshDeserialize;
use solana_program::{pubkey::Pubkey, stake::state::StakeState};
use solana_sdk::account::Account;

/// Simplified version of solana_program::StakeState
/// containing only the params we require to give a quote
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct WithdrawableStakeAcc {
    pub lamports: u64,
    pub voter: Pubkey,

    /// state.delegation.stake
    pub stake: u64,
}

impl TryFrom<&Account> for WithdrawableStakeAcc {
    type Error = anyhow::Error;

    /// Errors if:
    /// - account is not a stake account
    /// - stake account is not withdrawable
    ///     - stake account is not delegated
    ///     - deactivation_epoch is not u64::MAX
    fn try_from(Account { lamports, data, .. }: &Account) -> Result<Self, Self::Error> {
        let state = StakeState::try_from_slice(data)?;
        let delegation = state
            .delegation()
            .ok_or_else(|| anyhow!("stake not delegated"))?;
        if delegation.deactivation_epoch != u64::MAX {
            return Err(anyhow!("stake not active"));
        }
        Ok(Self {
            lamports: *lamports,
            voter: delegation.voter_pubkey,
            stake: delegation.stake,
        })
    }
}

impl TryFrom<Account> for WithdrawableStakeAcc {
    type Error = anyhow::Error;

    fn try_from(value: Account) -> Result<Self, Self::Error> {
        Self::try_from(&value)
    }
}
