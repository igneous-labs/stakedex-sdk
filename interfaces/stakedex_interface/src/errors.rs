use solana_program::{
    decode_error::DecodeError,
    msg,
    program_error::{PrintProgramError, ProgramError},
};
use thiserror::Error;
#[derive(Clone, Copy, Debug, Eq, Error, num_derive::FromPrimitive, PartialEq)]
pub enum StakedexError {
    #[error("Wrong wsol_bridge_in account")]
    WrongWsolBridgeIn = 0,
    #[error("Wrong sol_bridge_out account")]
    WrongSolBridgeOut = 1,
    #[error("Wrong fee token account")]
    WrongFeeTokenAccount = 2,
    #[error("Wrong token program")]
    WrongTokenProgram = 3,
    #[error("Not system account")]
    NotSystemAccount = 4,
    #[error("Not wSOL mint")]
    NotWsolMint = 5,
    #[error("Wrong stake pool program")]
    WrongStakePoolProgram = 6,
    #[error("Wrong bridge stake account")]
    WrongBridgeStake = 7,
    #[error("Wrong admin authority")]
    WrongAdmin = 8,
    #[error("Stake account has no voter")]
    StakeAccInactive = 9,
    #[error("Validator not part of stake pool")]
    ValidatorNotPartOfStakePool = 10,
    #[error("Not signed by admin")]
    NotSignedByAdmin = 11,
    #[error("Numerical error")]
    NumericalError = 12,
    #[error("Unsupported stake pool program")]
    UnsupportedProgram = 13,
    #[error("If you see this, there's a serious bug somewhere")]
    UnreachableError = 14,
}
impl From<StakedexError> for ProgramError {
    fn from(e: StakedexError) -> Self {
        ProgramError::Custom(e as u32)
    }
}
impl<T> DecodeError<T> for StakedexError {
    fn type_of() -> &'static str {
        "StakedexError"
    }
}
impl PrintProgramError for StakedexError {
    fn print<E>(&self)
    where
        E: 'static
            + std::error::Error
            + DecodeError<E>
            + PrintProgramError
            + num_traits::FromPrimitive,
    {
        msg!(&self.to_string());
    }
}
