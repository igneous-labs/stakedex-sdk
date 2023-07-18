use crate::*;
use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::AccountInfo,
    entrypoint::ProgramResult,
    instruction::{AccountMeta, Instruction},
    program::{invoke, invoke_signed},
    program_error::ProgramError,
    pubkey::Pubkey,
};
#[derive(Clone, Debug, PartialEq)]
pub enum StakedexProgramIx {
    StakeWrappedSol(StakeWrappedSolIxArgs),
    SwapViaStake(SwapViaStakeIxArgs),
    CreateFeeTokenAccount(CreateFeeTokenAccountIxArgs),
    CloseFeeTokenAccount(CloseFeeTokenAccountIxArgs),
    WithdrawFees(WithdrawFeesIxArgs),
    DepositStake(DepositStakeIxArgs),
}
impl BorshSerialize for StakedexProgramIx {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        match self {
            Self::StakeWrappedSol(args) => {
                STAKE_WRAPPED_SOL_IX_DISCM.serialize(writer)?;
                args.serialize(writer)
            }
            Self::SwapViaStake(args) => {
                SWAP_VIA_STAKE_IX_DISCM.serialize(writer)?;
                args.serialize(writer)
            }
            Self::CreateFeeTokenAccount(args) => {
                CREATE_FEE_TOKEN_ACCOUNT_IX_DISCM.serialize(writer)?;
                args.serialize(writer)
            }
            Self::CloseFeeTokenAccount(args) => {
                CLOSE_FEE_TOKEN_ACCOUNT_IX_DISCM.serialize(writer)?;
                args.serialize(writer)
            }
            Self::WithdrawFees(args) => {
                WITHDRAW_FEES_IX_DISCM.serialize(writer)?;
                args.serialize(writer)
            }
            Self::DepositStake(args) => {
                DEPOSIT_STAKE_IX_DISCM.serialize(writer)?;
                args.serialize(writer)
            }
        }
    }
}
impl StakedexProgramIx {
    pub fn deserialize(buf: &mut &[u8]) -> std::io::Result<Self> {
        let maybe_discm = u8::deserialize(buf)?;
        match maybe_discm {
            STAKE_WRAPPED_SOL_IX_DISCM => Ok(Self::StakeWrappedSol(
                StakeWrappedSolIxArgs::deserialize(buf)?,
            )),
            SWAP_VIA_STAKE_IX_DISCM => {
                Ok(Self::SwapViaStake(SwapViaStakeIxArgs::deserialize(buf)?))
            }
            CREATE_FEE_TOKEN_ACCOUNT_IX_DISCM => Ok(Self::CreateFeeTokenAccount(
                CreateFeeTokenAccountIxArgs::deserialize(buf)?,
            )),
            CLOSE_FEE_TOKEN_ACCOUNT_IX_DISCM => Ok(Self::CloseFeeTokenAccount(
                CloseFeeTokenAccountIxArgs::deserialize(buf)?,
            )),
            WITHDRAW_FEES_IX_DISCM => Ok(Self::WithdrawFees(WithdrawFeesIxArgs::deserialize(buf)?)),
            DEPOSIT_STAKE_IX_DISCM => Ok(Self::DepositStake(DepositStakeIxArgs::deserialize(buf)?)),
            _ => Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!("discm {:?} not found", maybe_discm),
            )),
        }
    }
}
pub const STAKE_WRAPPED_SOL_IX_ACCOUNTS_LEN: usize = 10;
#[derive(Copy, Clone, Debug)]
pub struct StakeWrappedSolAccounts<'me, 'info> {
    ///The authority of wsol_account
    pub user: &'me AccountInfo<'info>,
    ///The wrapped SOL token account to stake wrapped SOL from
    pub wsol_from: &'me AccountInfo<'info>,
    ///The liquid staked SOL token account to receive the resulting tokens
    pub dest_token_to: &'me AccountInfo<'info>,
    ///The PDA that serves as the wSOL account to bridge user's wSOL to SOL. Pubkey::create_with_seed(). base = sol_bridge_out.pubkey, seed = 'wsol_bridge_in'. owner = token_program
    pub wsol_bridge_in: &'me AccountInfo<'info>,
    ///The PDA that serves as the system account to bridge user's wSOL to SOL. Seeds = ['sol_bridge_out']
    pub sol_bridge_out: &'me AccountInfo<'info>,
    ///The liquid staked SOL token account collecting fees. PDA. Seeds = ['fee', dest_token_mint.pubkey]
    pub dest_token_fee_token_account: &'me AccountInfo<'info>,
    ///The liquid staked SOL mint
    pub dest_token_mint: &'me AccountInfo<'info>,
    ///wSOL token mint
    pub wsol_mint: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug)]
pub struct StakeWrappedSolKeys {
    ///The authority of wsol_account
    pub user: Pubkey,
    ///The wrapped SOL token account to stake wrapped SOL from
    pub wsol_from: Pubkey,
    ///The liquid staked SOL token account to receive the resulting tokens
    pub dest_token_to: Pubkey,
    ///The PDA that serves as the wSOL account to bridge user's wSOL to SOL. Pubkey::create_with_seed(). base = sol_bridge_out.pubkey, seed = 'wsol_bridge_in'. owner = token_program
    pub wsol_bridge_in: Pubkey,
    ///The PDA that serves as the system account to bridge user's wSOL to SOL. Seeds = ['sol_bridge_out']
    pub sol_bridge_out: Pubkey,
    ///The liquid staked SOL token account collecting fees. PDA. Seeds = ['fee', dest_token_mint.pubkey]
    pub dest_token_fee_token_account: Pubkey,
    ///The liquid staked SOL mint
    pub dest_token_mint: Pubkey,
    ///wSOL token mint
    pub wsol_mint: Pubkey,
    pub token_program: Pubkey,
    pub system_program: Pubkey,
}
impl From<&StakeWrappedSolAccounts<'_, '_>> for StakeWrappedSolKeys {
    fn from(accounts: &StakeWrappedSolAccounts) -> Self {
        Self {
            user: *accounts.user.key,
            wsol_from: *accounts.wsol_from.key,
            dest_token_to: *accounts.dest_token_to.key,
            wsol_bridge_in: *accounts.wsol_bridge_in.key,
            sol_bridge_out: *accounts.sol_bridge_out.key,
            dest_token_fee_token_account: *accounts.dest_token_fee_token_account.key,
            dest_token_mint: *accounts.dest_token_mint.key,
            wsol_mint: *accounts.wsol_mint.key,
            token_program: *accounts.token_program.key,
            system_program: *accounts.system_program.key,
        }
    }
}
impl From<&StakeWrappedSolKeys> for [AccountMeta; STAKE_WRAPPED_SOL_IX_ACCOUNTS_LEN] {
    fn from(keys: &StakeWrappedSolKeys) -> Self {
        [
            AccountMeta::new_readonly(keys.user, false),
            AccountMeta::new(keys.wsol_from, false),
            AccountMeta::new(keys.dest_token_to, false),
            AccountMeta::new(keys.wsol_bridge_in, false),
            AccountMeta::new(keys.sol_bridge_out, false),
            AccountMeta::new(keys.dest_token_fee_token_account, false),
            AccountMeta::new(keys.dest_token_mint, false),
            AccountMeta::new_readonly(keys.wsol_mint, false),
            AccountMeta::new_readonly(keys.token_program, false),
            AccountMeta::new_readonly(keys.system_program, false),
        ]
    }
}
impl From<[Pubkey; STAKE_WRAPPED_SOL_IX_ACCOUNTS_LEN]> for StakeWrappedSolKeys {
    fn from(pubkeys: [Pubkey; STAKE_WRAPPED_SOL_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            user: pubkeys[0],
            wsol_from: pubkeys[1],
            dest_token_to: pubkeys[2],
            wsol_bridge_in: pubkeys[3],
            sol_bridge_out: pubkeys[4],
            dest_token_fee_token_account: pubkeys[5],
            dest_token_mint: pubkeys[6],
            wsol_mint: pubkeys[7],
            token_program: pubkeys[8],
            system_program: pubkeys[9],
        }
    }
}
impl<'info> From<&StakeWrappedSolAccounts<'_, 'info>>
    for [AccountInfo<'info>; STAKE_WRAPPED_SOL_IX_ACCOUNTS_LEN]
{
    fn from(accounts: &StakeWrappedSolAccounts<'_, 'info>) -> Self {
        [
            accounts.user.clone(),
            accounts.wsol_from.clone(),
            accounts.dest_token_to.clone(),
            accounts.wsol_bridge_in.clone(),
            accounts.sol_bridge_out.clone(),
            accounts.dest_token_fee_token_account.clone(),
            accounts.dest_token_mint.clone(),
            accounts.wsol_mint.clone(),
            accounts.token_program.clone(),
            accounts.system_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; STAKE_WRAPPED_SOL_IX_ACCOUNTS_LEN]>
    for StakeWrappedSolAccounts<'me, 'info>
{
    fn from(arr: &'me [AccountInfo<'info>; STAKE_WRAPPED_SOL_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            user: &arr[0],
            wsol_from: &arr[1],
            dest_token_to: &arr[2],
            wsol_bridge_in: &arr[3],
            sol_bridge_out: &arr[4],
            dest_token_fee_token_account: &arr[5],
            dest_token_mint: &arr[6],
            wsol_mint: &arr[7],
            token_program: &arr[8],
            system_program: &arr[9],
        }
    }
}
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct StakeWrappedSolIxArgs {
    pub stake_wrapped_sol_args: StakeWrappedSolArgs,
}
#[derive(Clone, Debug, PartialEq)]
pub struct StakeWrappedSolIxData(pub StakeWrappedSolIxArgs);
pub const STAKE_WRAPPED_SOL_IX_DISCM: u8 = 0u8;
impl From<StakeWrappedSolIxArgs> for StakeWrappedSolIxData {
    fn from(args: StakeWrappedSolIxArgs) -> Self {
        Self(args)
    }
}
impl BorshSerialize for StakeWrappedSolIxData {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        writer.write_all(&[STAKE_WRAPPED_SOL_IX_DISCM])?;
        self.0.serialize(writer)
    }
}
impl StakeWrappedSolIxData {
    pub fn deserialize(buf: &mut &[u8]) -> std::io::Result<Self> {
        let maybe_discm = u8::deserialize(buf)?;
        if maybe_discm != STAKE_WRAPPED_SOL_IX_DISCM {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    STAKE_WRAPPED_SOL_IX_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self(StakeWrappedSolIxArgs::deserialize(buf)?))
    }
}
pub fn stake_wrapped_sol_ix<K: Into<StakeWrappedSolKeys>, A: Into<StakeWrappedSolIxArgs>>(
    accounts: K,
    args: A,
) -> std::io::Result<Instruction> {
    let keys: StakeWrappedSolKeys = accounts.into();
    let metas: [AccountMeta; STAKE_WRAPPED_SOL_IX_ACCOUNTS_LEN] = (&keys).into();
    let args_full: StakeWrappedSolIxArgs = args.into();
    let data: StakeWrappedSolIxData = args_full.into();
    Ok(Instruction {
        program_id: crate::ID,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn stake_wrapped_sol_invoke<'info, A: Into<StakeWrappedSolIxArgs>>(
    accounts: &StakeWrappedSolAccounts<'_, 'info>,
    args: A,
) -> ProgramResult {
    let ix = stake_wrapped_sol_ix(accounts, args)?;
    let account_info: [AccountInfo<'info>; STAKE_WRAPPED_SOL_IX_ACCOUNTS_LEN] = accounts.into();
    invoke(&ix, &account_info)
}
pub fn stake_wrapped_sol_invoke_signed<'info, A: Into<StakeWrappedSolIxArgs>>(
    accounts: &StakeWrappedSolAccounts<'_, 'info>,
    args: A,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let ix = stake_wrapped_sol_ix(accounts, args)?;
    let account_info: [AccountInfo<'info>; STAKE_WRAPPED_SOL_IX_ACCOUNTS_LEN] = accounts.into();
    invoke_signed(&ix, &account_info, seeds)
}
pub fn stake_wrapped_sol_verify_account_keys(
    accounts: &StakeWrappedSolAccounts<'_, '_>,
    keys: &StakeWrappedSolKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (accounts.user.key, &keys.user),
        (accounts.wsol_from.key, &keys.wsol_from),
        (accounts.dest_token_to.key, &keys.dest_token_to),
        (accounts.wsol_bridge_in.key, &keys.wsol_bridge_in),
        (accounts.sol_bridge_out.key, &keys.sol_bridge_out),
        (
            accounts.dest_token_fee_token_account.key,
            &keys.dest_token_fee_token_account,
        ),
        (accounts.dest_token_mint.key, &keys.dest_token_mint),
        (accounts.wsol_mint.key, &keys.wsol_mint),
        (accounts.token_program.key, &keys.token_program),
        (accounts.system_program.key, &keys.system_program),
    ] {
        if actual != expected {
            return Err((*actual, *expected));
        }
    }
    Ok(())
}
pub fn stake_wrapped_sol_verify_account_privileges(
    accounts: &StakeWrappedSolAccounts<'_, '_>,
) -> Result<(), ProgramError> {
    for should_be_writable in [
        accounts.wsol_from,
        accounts.dest_token_to,
        accounts.wsol_bridge_in,
        accounts.sol_bridge_out,
        accounts.dest_token_fee_token_account,
        accounts.dest_token_mint,
    ] {
        if !should_be_writable.is_writable {
            return Err(ProgramError::InvalidAccountData);
        }
    }
    for should_be_signer in [accounts.user] {
        if !should_be_signer.is_signer {
            return Err(ProgramError::MissingRequiredSignature);
        }
    }
    Ok(())
}
pub const SWAP_VIA_STAKE_IX_ACCOUNTS_LEN: usize = 7;
#[derive(Copy, Clone, Debug)]
pub struct SwapViaStakeAccounts<'me, 'info> {
    ///The authority of src_token_from. Needs to be mutable to support marinde deposit stake.
    pub user: &'me AccountInfo<'info>,
    ///The token account to swap src tokens from
    pub src_token_from: &'me AccountInfo<'info>,
    ///The token account to receive dest tokens to
    pub dest_token_to: &'me AccountInfo<'info>,
    ///The bridge stake account thats withdrawn then deposited. PDA. seeds = ['bridge_stake', user.pubkey, SwapArgs.bridge_stake_seed]. Might be long-lived, make sure the seed is not already in use
    pub bridge_stake: &'me AccountInfo<'info>,
    ///The dest_token_mint token account collecting fees. PDA. Seeds = ['fee', dest_token_mint.pubkey]
    pub dest_token_fee_token_account: &'me AccountInfo<'info>,
    ///Input token mint. If this is wrapped SOL, the account can be set to read-only
    pub src_token_mint: &'me AccountInfo<'info>,
    ///Output token mint. If this is wrapped SOL, the account can be set to read-only
    pub dest_token_mint: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug)]
pub struct SwapViaStakeKeys {
    ///The authority of src_token_from. Needs to be mutable to support marinde deposit stake.
    pub user: Pubkey,
    ///The token account to swap src tokens from
    pub src_token_from: Pubkey,
    ///The token account to receive dest tokens to
    pub dest_token_to: Pubkey,
    ///The bridge stake account thats withdrawn then deposited. PDA. seeds = ['bridge_stake', user.pubkey, SwapArgs.bridge_stake_seed]. Might be long-lived, make sure the seed is not already in use
    pub bridge_stake: Pubkey,
    ///The dest_token_mint token account collecting fees. PDA. Seeds = ['fee', dest_token_mint.pubkey]
    pub dest_token_fee_token_account: Pubkey,
    ///Input token mint. If this is wrapped SOL, the account can be set to read-only
    pub src_token_mint: Pubkey,
    ///Output token mint. If this is wrapped SOL, the account can be set to read-only
    pub dest_token_mint: Pubkey,
}
impl From<&SwapViaStakeAccounts<'_, '_>> for SwapViaStakeKeys {
    fn from(accounts: &SwapViaStakeAccounts) -> Self {
        Self {
            user: *accounts.user.key,
            src_token_from: *accounts.src_token_from.key,
            dest_token_to: *accounts.dest_token_to.key,
            bridge_stake: *accounts.bridge_stake.key,
            dest_token_fee_token_account: *accounts.dest_token_fee_token_account.key,
            src_token_mint: *accounts.src_token_mint.key,
            dest_token_mint: *accounts.dest_token_mint.key,
        }
    }
}
impl From<&SwapViaStakeKeys> for [AccountMeta; SWAP_VIA_STAKE_IX_ACCOUNTS_LEN] {
    fn from(keys: &SwapViaStakeKeys) -> Self {
        [
            AccountMeta::new_readonly(keys.user, true),
            AccountMeta::new(keys.src_token_from, false),
            AccountMeta::new(keys.dest_token_to, false),
            AccountMeta::new(keys.bridge_stake, false),
            AccountMeta::new(keys.dest_token_fee_token_account, false),
            AccountMeta::new(keys.src_token_mint, false),
            AccountMeta::new(keys.dest_token_mint, false),
        ]
    }
}
impl From<[Pubkey; SWAP_VIA_STAKE_IX_ACCOUNTS_LEN]> for SwapViaStakeKeys {
    fn from(pubkeys: [Pubkey; SWAP_VIA_STAKE_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            user: pubkeys[0],
            src_token_from: pubkeys[1],
            dest_token_to: pubkeys[2],
            bridge_stake: pubkeys[3],
            dest_token_fee_token_account: pubkeys[4],
            src_token_mint: pubkeys[5],
            dest_token_mint: pubkeys[6],
        }
    }
}
impl<'info> From<&SwapViaStakeAccounts<'_, 'info>>
    for [AccountInfo<'info>; SWAP_VIA_STAKE_IX_ACCOUNTS_LEN]
{
    fn from(accounts: &SwapViaStakeAccounts<'_, 'info>) -> Self {
        [
            accounts.user.clone(),
            accounts.src_token_from.clone(),
            accounts.dest_token_to.clone(),
            accounts.bridge_stake.clone(),
            accounts.dest_token_fee_token_account.clone(),
            accounts.src_token_mint.clone(),
            accounts.dest_token_mint.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; SWAP_VIA_STAKE_IX_ACCOUNTS_LEN]>
    for SwapViaStakeAccounts<'me, 'info>
{
    fn from(arr: &'me [AccountInfo<'info>; SWAP_VIA_STAKE_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            user: &arr[0],
            src_token_from: &arr[1],
            dest_token_to: &arr[2],
            bridge_stake: &arr[3],
            dest_token_fee_token_account: &arr[4],
            src_token_mint: &arr[5],
            dest_token_mint: &arr[6],
        }
    }
}
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct SwapViaStakeIxArgs {
    pub swap_via_stake_args: SwapViaStakeArgs,
}
#[derive(Clone, Debug, PartialEq)]
pub struct SwapViaStakeIxData(pub SwapViaStakeIxArgs);
pub const SWAP_VIA_STAKE_IX_DISCM: u8 = 1u8;
impl From<SwapViaStakeIxArgs> for SwapViaStakeIxData {
    fn from(args: SwapViaStakeIxArgs) -> Self {
        Self(args)
    }
}
impl BorshSerialize for SwapViaStakeIxData {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        writer.write_all(&[SWAP_VIA_STAKE_IX_DISCM])?;
        self.0.serialize(writer)
    }
}
impl SwapViaStakeIxData {
    pub fn deserialize(buf: &mut &[u8]) -> std::io::Result<Self> {
        let maybe_discm = u8::deserialize(buf)?;
        if maybe_discm != SWAP_VIA_STAKE_IX_DISCM {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    SWAP_VIA_STAKE_IX_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self(SwapViaStakeIxArgs::deserialize(buf)?))
    }
}
pub fn swap_via_stake_ix<K: Into<SwapViaStakeKeys>, A: Into<SwapViaStakeIxArgs>>(
    accounts: K,
    args: A,
) -> std::io::Result<Instruction> {
    let keys: SwapViaStakeKeys = accounts.into();
    let metas: [AccountMeta; SWAP_VIA_STAKE_IX_ACCOUNTS_LEN] = (&keys).into();
    let args_full: SwapViaStakeIxArgs = args.into();
    let data: SwapViaStakeIxData = args_full.into();
    Ok(Instruction {
        program_id: crate::ID,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn swap_via_stake_invoke<'info, A: Into<SwapViaStakeIxArgs>>(
    accounts: &SwapViaStakeAccounts<'_, 'info>,
    args: A,
) -> ProgramResult {
    let ix = swap_via_stake_ix(accounts, args)?;
    let account_info: [AccountInfo<'info>; SWAP_VIA_STAKE_IX_ACCOUNTS_LEN] = accounts.into();
    invoke(&ix, &account_info)
}
pub fn swap_via_stake_invoke_signed<'info, A: Into<SwapViaStakeIxArgs>>(
    accounts: &SwapViaStakeAccounts<'_, 'info>,
    args: A,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let ix = swap_via_stake_ix(accounts, args)?;
    let account_info: [AccountInfo<'info>; SWAP_VIA_STAKE_IX_ACCOUNTS_LEN] = accounts.into();
    invoke_signed(&ix, &account_info, seeds)
}
pub fn swap_via_stake_verify_account_keys(
    accounts: &SwapViaStakeAccounts<'_, '_>,
    keys: &SwapViaStakeKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (accounts.user.key, &keys.user),
        (accounts.src_token_from.key, &keys.src_token_from),
        (accounts.dest_token_to.key, &keys.dest_token_to),
        (accounts.bridge_stake.key, &keys.bridge_stake),
        (
            accounts.dest_token_fee_token_account.key,
            &keys.dest_token_fee_token_account,
        ),
        (accounts.src_token_mint.key, &keys.src_token_mint),
        (accounts.dest_token_mint.key, &keys.dest_token_mint),
    ] {
        if actual != expected {
            return Err((*actual, *expected));
        }
    }
    Ok(())
}
pub fn swap_via_stake_verify_account_privileges(
    accounts: &SwapViaStakeAccounts<'_, '_>,
) -> Result<(), ProgramError> {
    for should_be_writable in [
        accounts.user,
        accounts.src_token_from,
        accounts.dest_token_to,
        accounts.bridge_stake,
        accounts.dest_token_fee_token_account,
        accounts.src_token_mint,
        accounts.dest_token_mint,
    ] {
        if !should_be_writable.is_writable {
            return Err(ProgramError::InvalidAccountData);
        }
    }
    for should_be_signer in [accounts.user] {
        if !should_be_signer.is_signer {
            return Err(ProgramError::MissingRequiredSignature);
        }
    }
    Ok(())
}
pub const CREATE_FEE_TOKEN_ACCOUNT_IX_ACCOUNTS_LEN: usize = 5;
#[derive(Copy, Clone, Debug)]
pub struct CreateFeeTokenAccountAccounts<'me, 'info> {
    ///The person paying for the new fee token account. Can be anyone.
    pub payer: &'me AccountInfo<'info>,
    ///The self-owned fee token account to be created. Seeds = ['fee', mint_pubkey]
    pub fee_token_account: &'me AccountInfo<'info>,
    pub mint: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug)]
pub struct CreateFeeTokenAccountKeys {
    ///The person paying for the new fee token account. Can be anyone.
    pub payer: Pubkey,
    ///The self-owned fee token account to be created. Seeds = ['fee', mint_pubkey]
    pub fee_token_account: Pubkey,
    pub mint: Pubkey,
    pub token_program: Pubkey,
    pub system_program: Pubkey,
}
impl From<&CreateFeeTokenAccountAccounts<'_, '_>> for CreateFeeTokenAccountKeys {
    fn from(accounts: &CreateFeeTokenAccountAccounts) -> Self {
        Self {
            payer: *accounts.payer.key,
            fee_token_account: *accounts.fee_token_account.key,
            mint: *accounts.mint.key,
            token_program: *accounts.token_program.key,
            system_program: *accounts.system_program.key,
        }
    }
}
impl From<&CreateFeeTokenAccountKeys> for [AccountMeta; CREATE_FEE_TOKEN_ACCOUNT_IX_ACCOUNTS_LEN] {
    fn from(keys: &CreateFeeTokenAccountKeys) -> Self {
        [
            AccountMeta::new(keys.payer, true),
            AccountMeta::new(keys.fee_token_account, false),
            AccountMeta::new_readonly(keys.mint, false),
            AccountMeta::new_readonly(keys.token_program, false),
            AccountMeta::new_readonly(keys.system_program, false),
        ]
    }
}
impl From<[Pubkey; CREATE_FEE_TOKEN_ACCOUNT_IX_ACCOUNTS_LEN]> for CreateFeeTokenAccountKeys {
    fn from(pubkeys: [Pubkey; CREATE_FEE_TOKEN_ACCOUNT_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            payer: pubkeys[0],
            fee_token_account: pubkeys[1],
            mint: pubkeys[2],
            token_program: pubkeys[3],
            system_program: pubkeys[4],
        }
    }
}
impl<'info> From<&CreateFeeTokenAccountAccounts<'_, 'info>>
    for [AccountInfo<'info>; CREATE_FEE_TOKEN_ACCOUNT_IX_ACCOUNTS_LEN]
{
    fn from(accounts: &CreateFeeTokenAccountAccounts<'_, 'info>) -> Self {
        [
            accounts.payer.clone(),
            accounts.fee_token_account.clone(),
            accounts.mint.clone(),
            accounts.token_program.clone(),
            accounts.system_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; CREATE_FEE_TOKEN_ACCOUNT_IX_ACCOUNTS_LEN]>
    for CreateFeeTokenAccountAccounts<'me, 'info>
{
    fn from(arr: &'me [AccountInfo<'info>; CREATE_FEE_TOKEN_ACCOUNT_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            payer: &arr[0],
            fee_token_account: &arr[1],
            mint: &arr[2],
            token_program: &arr[3],
            system_program: &arr[4],
        }
    }
}
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct CreateFeeTokenAccountIxArgs {}
#[derive(Clone, Debug, PartialEq)]
pub struct CreateFeeTokenAccountIxData(pub CreateFeeTokenAccountIxArgs);
pub const CREATE_FEE_TOKEN_ACCOUNT_IX_DISCM: u8 = 2u8;
impl From<CreateFeeTokenAccountIxArgs> for CreateFeeTokenAccountIxData {
    fn from(args: CreateFeeTokenAccountIxArgs) -> Self {
        Self(args)
    }
}
impl BorshSerialize for CreateFeeTokenAccountIxData {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        writer.write_all(&[CREATE_FEE_TOKEN_ACCOUNT_IX_DISCM])?;
        self.0.serialize(writer)
    }
}
impl CreateFeeTokenAccountIxData {
    pub fn deserialize(buf: &mut &[u8]) -> std::io::Result<Self> {
        let maybe_discm = u8::deserialize(buf)?;
        if maybe_discm != CREATE_FEE_TOKEN_ACCOUNT_IX_DISCM {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    CREATE_FEE_TOKEN_ACCOUNT_IX_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self(CreateFeeTokenAccountIxArgs::deserialize(buf)?))
    }
}
pub fn create_fee_token_account_ix<
    K: Into<CreateFeeTokenAccountKeys>,
    A: Into<CreateFeeTokenAccountIxArgs>,
>(
    accounts: K,
    args: A,
) -> std::io::Result<Instruction> {
    let keys: CreateFeeTokenAccountKeys = accounts.into();
    let metas: [AccountMeta; CREATE_FEE_TOKEN_ACCOUNT_IX_ACCOUNTS_LEN] = (&keys).into();
    let args_full: CreateFeeTokenAccountIxArgs = args.into();
    let data: CreateFeeTokenAccountIxData = args_full.into();
    Ok(Instruction {
        program_id: crate::ID,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn create_fee_token_account_invoke<'info, A: Into<CreateFeeTokenAccountIxArgs>>(
    accounts: &CreateFeeTokenAccountAccounts<'_, 'info>,
    args: A,
) -> ProgramResult {
    let ix = create_fee_token_account_ix(accounts, args)?;
    let account_info: [AccountInfo<'info>; CREATE_FEE_TOKEN_ACCOUNT_IX_ACCOUNTS_LEN] =
        accounts.into();
    invoke(&ix, &account_info)
}
pub fn create_fee_token_account_invoke_signed<'info, A: Into<CreateFeeTokenAccountIxArgs>>(
    accounts: &CreateFeeTokenAccountAccounts<'_, 'info>,
    args: A,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let ix = create_fee_token_account_ix(accounts, args)?;
    let account_info: [AccountInfo<'info>; CREATE_FEE_TOKEN_ACCOUNT_IX_ACCOUNTS_LEN] =
        accounts.into();
    invoke_signed(&ix, &account_info, seeds)
}
pub fn create_fee_token_account_verify_account_keys(
    accounts: &CreateFeeTokenAccountAccounts<'_, '_>,
    keys: &CreateFeeTokenAccountKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (accounts.payer.key, &keys.payer),
        (accounts.fee_token_account.key, &keys.fee_token_account),
        (accounts.mint.key, &keys.mint),
        (accounts.token_program.key, &keys.token_program),
        (accounts.system_program.key, &keys.system_program),
    ] {
        if actual != expected {
            return Err((*actual, *expected));
        }
    }
    Ok(())
}
pub fn create_fee_token_account_verify_account_privileges(
    accounts: &CreateFeeTokenAccountAccounts<'_, '_>,
) -> Result<(), ProgramError> {
    for should_be_writable in [accounts.payer, accounts.fee_token_account] {
        if !should_be_writable.is_writable {
            return Err(ProgramError::InvalidAccountData);
        }
    }
    for should_be_signer in [accounts.payer] {
        if !should_be_signer.is_signer {
            return Err(ProgramError::MissingRequiredSignature);
        }
    }
    Ok(())
}
pub const CLOSE_FEE_TOKEN_ACCOUNT_IX_ACCOUNTS_LEN: usize = 5;
#[derive(Copy, Clone, Debug)]
pub struct CloseFeeTokenAccountAccounts<'me, 'info> {
    ///The authorized program admin
    pub admin: &'me AccountInfo<'info>,
    ///The self-owned fee token account to close. Must be empty or wrapped SOL. Seeds = ['fee', mint_pubkey]
    pub fee_token_account: &'me AccountInfo<'info>,
    ///Refund fee_token_account's rent lamports to here
    pub close_to: &'me AccountInfo<'info>,
    pub mint: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug)]
pub struct CloseFeeTokenAccountKeys {
    ///The authorized program admin
    pub admin: Pubkey,
    ///The self-owned fee token account to close. Must be empty or wrapped SOL. Seeds = ['fee', mint_pubkey]
    pub fee_token_account: Pubkey,
    ///Refund fee_token_account's rent lamports to here
    pub close_to: Pubkey,
    pub mint: Pubkey,
    pub token_program: Pubkey,
}
impl From<&CloseFeeTokenAccountAccounts<'_, '_>> for CloseFeeTokenAccountKeys {
    fn from(accounts: &CloseFeeTokenAccountAccounts) -> Self {
        Self {
            admin: *accounts.admin.key,
            fee_token_account: *accounts.fee_token_account.key,
            close_to: *accounts.close_to.key,
            mint: *accounts.mint.key,
            token_program: *accounts.token_program.key,
        }
    }
}
impl From<&CloseFeeTokenAccountKeys> for [AccountMeta; CLOSE_FEE_TOKEN_ACCOUNT_IX_ACCOUNTS_LEN] {
    fn from(keys: &CloseFeeTokenAccountKeys) -> Self {
        [
            AccountMeta::new_readonly(keys.admin, true),
            AccountMeta::new(keys.fee_token_account, false),
            AccountMeta::new(keys.close_to, false),
            AccountMeta::new_readonly(keys.mint, false),
            AccountMeta::new_readonly(keys.token_program, false),
        ]
    }
}
impl From<[Pubkey; CLOSE_FEE_TOKEN_ACCOUNT_IX_ACCOUNTS_LEN]> for CloseFeeTokenAccountKeys {
    fn from(pubkeys: [Pubkey; CLOSE_FEE_TOKEN_ACCOUNT_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            admin: pubkeys[0],
            fee_token_account: pubkeys[1],
            close_to: pubkeys[2],
            mint: pubkeys[3],
            token_program: pubkeys[4],
        }
    }
}
impl<'info> From<&CloseFeeTokenAccountAccounts<'_, 'info>>
    for [AccountInfo<'info>; CLOSE_FEE_TOKEN_ACCOUNT_IX_ACCOUNTS_LEN]
{
    fn from(accounts: &CloseFeeTokenAccountAccounts<'_, 'info>) -> Self {
        [
            accounts.admin.clone(),
            accounts.fee_token_account.clone(),
            accounts.close_to.clone(),
            accounts.mint.clone(),
            accounts.token_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; CLOSE_FEE_TOKEN_ACCOUNT_IX_ACCOUNTS_LEN]>
    for CloseFeeTokenAccountAccounts<'me, 'info>
{
    fn from(arr: &'me [AccountInfo<'info>; CLOSE_FEE_TOKEN_ACCOUNT_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            admin: &arr[0],
            fee_token_account: &arr[1],
            close_to: &arr[2],
            mint: &arr[3],
            token_program: &arr[4],
        }
    }
}
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct CloseFeeTokenAccountIxArgs {}
#[derive(Clone, Debug, PartialEq)]
pub struct CloseFeeTokenAccountIxData(pub CloseFeeTokenAccountIxArgs);
pub const CLOSE_FEE_TOKEN_ACCOUNT_IX_DISCM: u8 = 3u8;
impl From<CloseFeeTokenAccountIxArgs> for CloseFeeTokenAccountIxData {
    fn from(args: CloseFeeTokenAccountIxArgs) -> Self {
        Self(args)
    }
}
impl BorshSerialize for CloseFeeTokenAccountIxData {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        writer.write_all(&[CLOSE_FEE_TOKEN_ACCOUNT_IX_DISCM])?;
        self.0.serialize(writer)
    }
}
impl CloseFeeTokenAccountIxData {
    pub fn deserialize(buf: &mut &[u8]) -> std::io::Result<Self> {
        let maybe_discm = u8::deserialize(buf)?;
        if maybe_discm != CLOSE_FEE_TOKEN_ACCOUNT_IX_DISCM {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    CLOSE_FEE_TOKEN_ACCOUNT_IX_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self(CloseFeeTokenAccountIxArgs::deserialize(buf)?))
    }
}
pub fn close_fee_token_account_ix<
    K: Into<CloseFeeTokenAccountKeys>,
    A: Into<CloseFeeTokenAccountIxArgs>,
>(
    accounts: K,
    args: A,
) -> std::io::Result<Instruction> {
    let keys: CloseFeeTokenAccountKeys = accounts.into();
    let metas: [AccountMeta; CLOSE_FEE_TOKEN_ACCOUNT_IX_ACCOUNTS_LEN] = (&keys).into();
    let args_full: CloseFeeTokenAccountIxArgs = args.into();
    let data: CloseFeeTokenAccountIxData = args_full.into();
    Ok(Instruction {
        program_id: crate::ID,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn close_fee_token_account_invoke<'info, A: Into<CloseFeeTokenAccountIxArgs>>(
    accounts: &CloseFeeTokenAccountAccounts<'_, 'info>,
    args: A,
) -> ProgramResult {
    let ix = close_fee_token_account_ix(accounts, args)?;
    let account_info: [AccountInfo<'info>; CLOSE_FEE_TOKEN_ACCOUNT_IX_ACCOUNTS_LEN] =
        accounts.into();
    invoke(&ix, &account_info)
}
pub fn close_fee_token_account_invoke_signed<'info, A: Into<CloseFeeTokenAccountIxArgs>>(
    accounts: &CloseFeeTokenAccountAccounts<'_, 'info>,
    args: A,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let ix = close_fee_token_account_ix(accounts, args)?;
    let account_info: [AccountInfo<'info>; CLOSE_FEE_TOKEN_ACCOUNT_IX_ACCOUNTS_LEN] =
        accounts.into();
    invoke_signed(&ix, &account_info, seeds)
}
pub fn close_fee_token_account_verify_account_keys(
    accounts: &CloseFeeTokenAccountAccounts<'_, '_>,
    keys: &CloseFeeTokenAccountKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (accounts.admin.key, &keys.admin),
        (accounts.fee_token_account.key, &keys.fee_token_account),
        (accounts.close_to.key, &keys.close_to),
        (accounts.mint.key, &keys.mint),
        (accounts.token_program.key, &keys.token_program),
    ] {
        if actual != expected {
            return Err((*actual, *expected));
        }
    }
    Ok(())
}
pub fn close_fee_token_account_verify_account_privileges(
    accounts: &CloseFeeTokenAccountAccounts<'_, '_>,
) -> Result<(), ProgramError> {
    for should_be_writable in [accounts.fee_token_account, accounts.close_to] {
        if !should_be_writable.is_writable {
            return Err(ProgramError::InvalidAccountData);
        }
    }
    for should_be_signer in [accounts.admin] {
        if !should_be_signer.is_signer {
            return Err(ProgramError::MissingRequiredSignature);
        }
    }
    Ok(())
}
pub const WITHDRAW_FEES_IX_ACCOUNTS_LEN: usize = 5;
#[derive(Copy, Clone, Debug)]
pub struct WithdrawFeesAccounts<'me, 'info> {
    ///The authorized program admin
    pub admin: &'me AccountInfo<'info>,
    ///The self-owned fee token account to withdraw fees from. Seeds = ['fee', mint_pubkey]
    pub fee_token_account: &'me AccountInfo<'info>,
    ///Withdraw accumulated fees to here
    pub withdraw_to: &'me AccountInfo<'info>,
    pub mint: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug)]
pub struct WithdrawFeesKeys {
    ///The authorized program admin
    pub admin: Pubkey,
    ///The self-owned fee token account to withdraw fees from. Seeds = ['fee', mint_pubkey]
    pub fee_token_account: Pubkey,
    ///Withdraw accumulated fees to here
    pub withdraw_to: Pubkey,
    pub mint: Pubkey,
    pub token_program: Pubkey,
}
impl From<&WithdrawFeesAccounts<'_, '_>> for WithdrawFeesKeys {
    fn from(accounts: &WithdrawFeesAccounts) -> Self {
        Self {
            admin: *accounts.admin.key,
            fee_token_account: *accounts.fee_token_account.key,
            withdraw_to: *accounts.withdraw_to.key,
            mint: *accounts.mint.key,
            token_program: *accounts.token_program.key,
        }
    }
}
impl From<&WithdrawFeesKeys> for [AccountMeta; WITHDRAW_FEES_IX_ACCOUNTS_LEN] {
    fn from(keys: &WithdrawFeesKeys) -> Self {
        [
            AccountMeta::new_readonly(keys.admin, true),
            AccountMeta::new(keys.fee_token_account, false),
            AccountMeta::new(keys.withdraw_to, false),
            AccountMeta::new_readonly(keys.mint, false),
            AccountMeta::new_readonly(keys.token_program, false),
        ]
    }
}
impl From<[Pubkey; WITHDRAW_FEES_IX_ACCOUNTS_LEN]> for WithdrawFeesKeys {
    fn from(pubkeys: [Pubkey; WITHDRAW_FEES_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            admin: pubkeys[0],
            fee_token_account: pubkeys[1],
            withdraw_to: pubkeys[2],
            mint: pubkeys[3],
            token_program: pubkeys[4],
        }
    }
}
impl<'info> From<&WithdrawFeesAccounts<'_, 'info>>
    for [AccountInfo<'info>; WITHDRAW_FEES_IX_ACCOUNTS_LEN]
{
    fn from(accounts: &WithdrawFeesAccounts<'_, 'info>) -> Self {
        [
            accounts.admin.clone(),
            accounts.fee_token_account.clone(),
            accounts.withdraw_to.clone(),
            accounts.mint.clone(),
            accounts.token_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; WITHDRAW_FEES_IX_ACCOUNTS_LEN]>
    for WithdrawFeesAccounts<'me, 'info>
{
    fn from(arr: &'me [AccountInfo<'info>; WITHDRAW_FEES_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            admin: &arr[0],
            fee_token_account: &arr[1],
            withdraw_to: &arr[2],
            mint: &arr[3],
            token_program: &arr[4],
        }
    }
}
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct WithdrawFeesIxArgs {}
#[derive(Clone, Debug, PartialEq)]
pub struct WithdrawFeesIxData(pub WithdrawFeesIxArgs);
pub const WITHDRAW_FEES_IX_DISCM: u8 = 4u8;
impl From<WithdrawFeesIxArgs> for WithdrawFeesIxData {
    fn from(args: WithdrawFeesIxArgs) -> Self {
        Self(args)
    }
}
impl BorshSerialize for WithdrawFeesIxData {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        writer.write_all(&[WITHDRAW_FEES_IX_DISCM])?;
        self.0.serialize(writer)
    }
}
impl WithdrawFeesIxData {
    pub fn deserialize(buf: &mut &[u8]) -> std::io::Result<Self> {
        let maybe_discm = u8::deserialize(buf)?;
        if maybe_discm != WITHDRAW_FEES_IX_DISCM {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    WITHDRAW_FEES_IX_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self(WithdrawFeesIxArgs::deserialize(buf)?))
    }
}
pub fn withdraw_fees_ix<K: Into<WithdrawFeesKeys>, A: Into<WithdrawFeesIxArgs>>(
    accounts: K,
    args: A,
) -> std::io::Result<Instruction> {
    let keys: WithdrawFeesKeys = accounts.into();
    let metas: [AccountMeta; WITHDRAW_FEES_IX_ACCOUNTS_LEN] = (&keys).into();
    let args_full: WithdrawFeesIxArgs = args.into();
    let data: WithdrawFeesIxData = args_full.into();
    Ok(Instruction {
        program_id: crate::ID,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn withdraw_fees_invoke<'info, A: Into<WithdrawFeesIxArgs>>(
    accounts: &WithdrawFeesAccounts<'_, 'info>,
    args: A,
) -> ProgramResult {
    let ix = withdraw_fees_ix(accounts, args)?;
    let account_info: [AccountInfo<'info>; WITHDRAW_FEES_IX_ACCOUNTS_LEN] = accounts.into();
    invoke(&ix, &account_info)
}
pub fn withdraw_fees_invoke_signed<'info, A: Into<WithdrawFeesIxArgs>>(
    accounts: &WithdrawFeesAccounts<'_, 'info>,
    args: A,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let ix = withdraw_fees_ix(accounts, args)?;
    let account_info: [AccountInfo<'info>; WITHDRAW_FEES_IX_ACCOUNTS_LEN] = accounts.into();
    invoke_signed(&ix, &account_info, seeds)
}
pub fn withdraw_fees_verify_account_keys(
    accounts: &WithdrawFeesAccounts<'_, '_>,
    keys: &WithdrawFeesKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (accounts.admin.key, &keys.admin),
        (accounts.fee_token_account.key, &keys.fee_token_account),
        (accounts.withdraw_to.key, &keys.withdraw_to),
        (accounts.mint.key, &keys.mint),
        (accounts.token_program.key, &keys.token_program),
    ] {
        if actual != expected {
            return Err((*actual, *expected));
        }
    }
    Ok(())
}
pub fn withdraw_fees_verify_account_privileges(
    accounts: &WithdrawFeesAccounts<'_, '_>,
) -> Result<(), ProgramError> {
    for should_be_writable in [accounts.fee_token_account, accounts.withdraw_to] {
        if !should_be_writable.is_writable {
            return Err(ProgramError::InvalidAccountData);
        }
    }
    for should_be_signer in [accounts.admin] {
        if !should_be_signer.is_signer {
            return Err(ProgramError::MissingRequiredSignature);
        }
    }
    Ok(())
}
pub const DEPOSIT_STAKE_IX_ACCOUNTS_LEN: usize = 5;
#[derive(Copy, Clone, Debug)]
pub struct DepositStakeAccounts<'me, 'info> {
    ///The withdraw authority of stake_account. Needs to be mutable to support marinade deposit stake.
    pub user: &'me AccountInfo<'info>,
    ///The stake account to deposit
    pub stake_account: &'me AccountInfo<'info>,
    ///The token account to receive dest tokens to
    pub dest_token_to: &'me AccountInfo<'info>,
    ///The dest_token_mint token account collecting fees. PDA. Seeds = ['fee', dest_token_mint.pubkey]
    pub dest_token_fee_token_account: &'me AccountInfo<'info>,
    ///Output token mint. If this is wrapped SOL, the account can be set to read-only
    pub dest_token_mint: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug)]
pub struct DepositStakeKeys {
    ///The withdraw authority of stake_account. Needs to be mutable to support marinade deposit stake.
    pub user: Pubkey,
    ///The stake account to deposit
    pub stake_account: Pubkey,
    ///The token account to receive dest tokens to
    pub dest_token_to: Pubkey,
    ///The dest_token_mint token account collecting fees. PDA. Seeds = ['fee', dest_token_mint.pubkey]
    pub dest_token_fee_token_account: Pubkey,
    ///Output token mint. If this is wrapped SOL, the account can be set to read-only
    pub dest_token_mint: Pubkey,
}
impl From<&DepositStakeAccounts<'_, '_>> for DepositStakeKeys {
    fn from(accounts: &DepositStakeAccounts) -> Self {
        Self {
            user: *accounts.user.key,
            stake_account: *accounts.stake_account.key,
            dest_token_to: *accounts.dest_token_to.key,
            dest_token_fee_token_account: *accounts.dest_token_fee_token_account.key,
            dest_token_mint: *accounts.dest_token_mint.key,
        }
    }
}
impl From<&DepositStakeKeys> for [AccountMeta; DEPOSIT_STAKE_IX_ACCOUNTS_LEN] {
    fn from(keys: &DepositStakeKeys) -> Self {
        [
            AccountMeta::new(keys.user, true),
            AccountMeta::new(keys.stake_account, false),
            AccountMeta::new(keys.dest_token_to, false),
            AccountMeta::new(keys.dest_token_fee_token_account, false),
            AccountMeta::new(keys.dest_token_mint, false),
        ]
    }
}
impl From<[Pubkey; DEPOSIT_STAKE_IX_ACCOUNTS_LEN]> for DepositStakeKeys {
    fn from(pubkeys: [Pubkey; DEPOSIT_STAKE_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            user: pubkeys[0],
            stake_account: pubkeys[1],
            dest_token_to: pubkeys[2],
            dest_token_fee_token_account: pubkeys[3],
            dest_token_mint: pubkeys[4],
        }
    }
}
impl<'info> From<&DepositStakeAccounts<'_, 'info>>
    for [AccountInfo<'info>; DEPOSIT_STAKE_IX_ACCOUNTS_LEN]
{
    fn from(accounts: &DepositStakeAccounts<'_, 'info>) -> Self {
        [
            accounts.user.clone(),
            accounts.stake_account.clone(),
            accounts.dest_token_to.clone(),
            accounts.dest_token_fee_token_account.clone(),
            accounts.dest_token_mint.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; DEPOSIT_STAKE_IX_ACCOUNTS_LEN]>
    for DepositStakeAccounts<'me, 'info>
{
    fn from(arr: &'me [AccountInfo<'info>; DEPOSIT_STAKE_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            user: &arr[0],
            stake_account: &arr[1],
            dest_token_to: &arr[2],
            dest_token_fee_token_account: &arr[3],
            dest_token_mint: &arr[4],
        }
    }
}
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct DepositStakeIxArgs {}
#[derive(Clone, Debug, PartialEq)]
pub struct DepositStakeIxData(pub DepositStakeIxArgs);
pub const DEPOSIT_STAKE_IX_DISCM: u8 = 5u8;
impl From<DepositStakeIxArgs> for DepositStakeIxData {
    fn from(args: DepositStakeIxArgs) -> Self {
        Self(args)
    }
}
impl BorshSerialize for DepositStakeIxData {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        writer.write_all(&[DEPOSIT_STAKE_IX_DISCM])?;
        self.0.serialize(writer)
    }
}
impl DepositStakeIxData {
    pub fn deserialize(buf: &mut &[u8]) -> std::io::Result<Self> {
        let maybe_discm = u8::deserialize(buf)?;
        if maybe_discm != DEPOSIT_STAKE_IX_DISCM {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    DEPOSIT_STAKE_IX_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self(DepositStakeIxArgs::deserialize(buf)?))
    }
}
pub fn deposit_stake_ix<K: Into<DepositStakeKeys>, A: Into<DepositStakeIxArgs>>(
    accounts: K,
    args: A,
) -> std::io::Result<Instruction> {
    let keys: DepositStakeKeys = accounts.into();
    let metas: [AccountMeta; DEPOSIT_STAKE_IX_ACCOUNTS_LEN] = (&keys).into();
    let args_full: DepositStakeIxArgs = args.into();
    let data: DepositStakeIxData = args_full.into();
    Ok(Instruction {
        program_id: crate::ID,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn deposit_stake_invoke<'info, A: Into<DepositStakeIxArgs>>(
    accounts: &DepositStakeAccounts<'_, 'info>,
    args: A,
) -> ProgramResult {
    let ix = deposit_stake_ix(accounts, args)?;
    let account_info: [AccountInfo<'info>; DEPOSIT_STAKE_IX_ACCOUNTS_LEN] = accounts.into();
    invoke(&ix, &account_info)
}
pub fn deposit_stake_invoke_signed<'info, A: Into<DepositStakeIxArgs>>(
    accounts: &DepositStakeAccounts<'_, 'info>,
    args: A,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let ix = deposit_stake_ix(accounts, args)?;
    let account_info: [AccountInfo<'info>; DEPOSIT_STAKE_IX_ACCOUNTS_LEN] = accounts.into();
    invoke_signed(&ix, &account_info, seeds)
}
pub fn deposit_stake_verify_account_keys(
    accounts: &DepositStakeAccounts<'_, '_>,
    keys: &DepositStakeKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (accounts.user.key, &keys.user),
        (accounts.stake_account.key, &keys.stake_account),
        (accounts.dest_token_to.key, &keys.dest_token_to),
        (
            accounts.dest_token_fee_token_account.key,
            &keys.dest_token_fee_token_account,
        ),
        (accounts.dest_token_mint.key, &keys.dest_token_mint),
    ] {
        if actual != expected {
            return Err((*actual, *expected));
        }
    }
    Ok(())
}
pub fn deposit_stake_verify_account_privileges(
    accounts: &DepositStakeAccounts<'_, '_>,
) -> Result<(), ProgramError> {
    for should_be_writable in [
        accounts.user,
        accounts.stake_account,
        accounts.dest_token_to,
        accounts.dest_token_fee_token_account,
        accounts.dest_token_mint,
    ] {
        if !should_be_writable.is_writable {
            return Err(ProgramError::InvalidAccountData);
        }
    }
    for should_be_signer in [accounts.user] {
        if !should_be_signer.is_signer {
            return Err(ProgramError::MissingRequiredSignature);
        }
    }
    Ok(())
}
