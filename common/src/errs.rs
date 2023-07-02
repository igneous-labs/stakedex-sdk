use solana_program::pubkey::Pubkey;

pub fn account_missing_err(pk: &Pubkey) -> anyhow::Error {
    anyhow::anyhow!("{} missing in accounts_map", pk)
}

#[derive(thiserror::Error, Copy, Clone, Debug, PartialEq)]
pub enum DepositStakeQuoteErr {
    #[error("Stake pool cannot accept stake deposits at this time")]
    CannotAcceptStakeDeposits,
}

#[derive(thiserror::Error, Copy, Clone, Debug, PartialEq)]
pub enum WithdrawStakeQuoteErr {
    #[error("Stake pool cannot accept stake withdrawal at this time")]
    CannotAcceptStakeWithdrawals,
}

#[derive(thiserror::Error, Copy, Clone, Debug, PartialEq)]
pub enum SwapViaStakeQuoteErr {
    #[error("{0}")]
    Deposit(DepositStakeQuoteErr),

    #[error("{0}")]
    Withdraw(WithdrawStakeQuoteErr),

    #[error("No route found between pools")]
    NoRouteFound,
}

impl From<WithdrawStakeQuoteErr> for SwapViaStakeQuoteErr {
    fn from(value: WithdrawStakeQuoteErr) -> Self {
        Self::Withdraw(value)
    }
}

impl From<DepositStakeQuoteErr> for SwapViaStakeQuoteErr {
    fn from(value: DepositStakeQuoteErr) -> Self {
        Self::Deposit(value)
    }
}
