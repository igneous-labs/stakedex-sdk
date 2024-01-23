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
use std::io::Read;
#[derive(Clone, Debug, PartialEq)]
pub enum StakedexProgramIx {
    StakeWrappedSol(StakeWrappedSolIxArgs),
    SwapViaStake(SwapViaStakeIxArgs),
    CreateFeeTokenAccount,
    CloseFeeTokenAccount,
    WithdrawFees,
    DepositStake,
    PrefundWithdrawStake(PrefundWithdrawStakeIxArgs),
    PrefundSwapViaStake(PrefundSwapViaStakeIxArgs),
}
impl StakedexProgramIx {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm_buf = [0u8; 1];
        reader.read_exact(&mut maybe_discm_buf)?;
        let maybe_discm = maybe_discm_buf[0];
        match maybe_discm {
            STAKE_WRAPPED_SOL_IX_DISCM => Ok(Self::StakeWrappedSol(
                StakeWrappedSolIxArgs::deserialize(&mut reader)?,
            )),
            SWAP_VIA_STAKE_IX_DISCM => Ok(Self::SwapViaStake(SwapViaStakeIxArgs::deserialize(
                &mut reader,
            )?)),
            CREATE_FEE_TOKEN_ACCOUNT_IX_DISCM => Ok(Self::CreateFeeTokenAccount),
            CLOSE_FEE_TOKEN_ACCOUNT_IX_DISCM => Ok(Self::CloseFeeTokenAccount),
            WITHDRAW_FEES_IX_DISCM => Ok(Self::WithdrawFees),
            DEPOSIT_STAKE_IX_DISCM => Ok(Self::DepositStake),
            PREFUND_WITHDRAW_STAKE_IX_DISCM => Ok(Self::PrefundWithdrawStake(
                PrefundWithdrawStakeIxArgs::deserialize(&mut reader)?,
            )),
            PREFUND_SWAP_VIA_STAKE_IX_DISCM => Ok(Self::PrefundSwapViaStake(
                PrefundSwapViaStakeIxArgs::deserialize(&mut reader)?,
            )),
            _ => Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!("discm {:?} not found", maybe_discm),
            )),
        }
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        match self {
            Self::StakeWrappedSol(args) => {
                writer.write_all(&[STAKE_WRAPPED_SOL_IX_DISCM])?;
                args.serialize(&mut writer)
            }
            Self::SwapViaStake(args) => {
                writer.write_all(&[SWAP_VIA_STAKE_IX_DISCM])?;
                args.serialize(&mut writer)
            }
            Self::CreateFeeTokenAccount => writer.write_all(&[CREATE_FEE_TOKEN_ACCOUNT_IX_DISCM]),
            Self::CloseFeeTokenAccount => writer.write_all(&[CLOSE_FEE_TOKEN_ACCOUNT_IX_DISCM]),
            Self::WithdrawFees => writer.write_all(&[WITHDRAW_FEES_IX_DISCM]),
            Self::DepositStake => writer.write_all(&[DEPOSIT_STAKE_IX_DISCM]),
            Self::PrefundWithdrawStake(args) => {
                writer.write_all(&[PREFUND_WITHDRAW_STAKE_IX_DISCM])?;
                args.serialize(&mut writer)
            }
            Self::PrefundSwapViaStake(args) => {
                writer.write_all(&[PREFUND_SWAP_VIA_STAKE_IX_DISCM])?;
                args.serialize(&mut writer)
            }
        }
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
fn invoke_instruction<'info, A: Into<[AccountInfo<'info>; N]>, const N: usize>(
    ix: &Instruction,
    accounts: A,
) -> ProgramResult {
    let account_info: [AccountInfo<'info>; N] = accounts.into();
    invoke(ix, &account_info)
}
fn invoke_instruction_signed<'info, A: Into<[AccountInfo<'info>; N]>, const N: usize>(
    ix: &Instruction,
    accounts: A,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let account_info: [AccountInfo<'info>; N] = accounts.into();
    invoke_signed(ix, &account_info, seeds)
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
    ///System program. The deposit SOL accounts slice follows.
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
    ///System program. The deposit SOL accounts slice follows.
    pub system_program: Pubkey,
}
impl From<StakeWrappedSolAccounts<'_, '_>> for StakeWrappedSolKeys {
    fn from(accounts: StakeWrappedSolAccounts) -> Self {
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
impl From<StakeWrappedSolKeys> for [AccountMeta; STAKE_WRAPPED_SOL_IX_ACCOUNTS_LEN] {
    fn from(keys: StakeWrappedSolKeys) -> Self {
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
impl<'info> From<StakeWrappedSolAccounts<'_, 'info>>
    for [AccountInfo<'info>; STAKE_WRAPPED_SOL_IX_ACCOUNTS_LEN]
{
    fn from(accounts: StakeWrappedSolAccounts<'_, 'info>) -> Self {
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
pub const STAKE_WRAPPED_SOL_IX_DISCM: u8 = 0u8;
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct StakeWrappedSolIxArgs {
    pub amount: u64,
}
#[derive(Clone, Debug, PartialEq)]
pub struct StakeWrappedSolIxData(pub StakeWrappedSolIxArgs);
impl From<StakeWrappedSolIxArgs> for StakeWrappedSolIxData {
    fn from(args: StakeWrappedSolIxArgs) -> Self {
        Self(args)
    }
}
impl StakeWrappedSolIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm_buf = [0u8; 1];
        reader.read_exact(&mut maybe_discm_buf)?;
        let maybe_discm = maybe_discm_buf[0];
        if maybe_discm != STAKE_WRAPPED_SOL_IX_DISCM {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    STAKE_WRAPPED_SOL_IX_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self(StakeWrappedSolIxArgs::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&[STAKE_WRAPPED_SOL_IX_DISCM])?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn stake_wrapped_sol_ix_with_program_id(
    program_id: Pubkey,
    keys: StakeWrappedSolKeys,
    args: StakeWrappedSolIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; STAKE_WRAPPED_SOL_IX_ACCOUNTS_LEN] = keys.into();
    let data: StakeWrappedSolIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn stake_wrapped_sol_ix(
    keys: StakeWrappedSolKeys,
    args: StakeWrappedSolIxArgs,
) -> std::io::Result<Instruction> {
    stake_wrapped_sol_ix_with_program_id(crate::ID, keys, args)
}
pub fn stake_wrapped_sol_invoke_with_program_id(
    program_id: Pubkey,
    accounts: StakeWrappedSolAccounts<'_, '_>,
    args: StakeWrappedSolIxArgs,
) -> ProgramResult {
    let keys: StakeWrappedSolKeys = accounts.into();
    let ix = stake_wrapped_sol_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn stake_wrapped_sol_invoke(
    accounts: StakeWrappedSolAccounts<'_, '_>,
    args: StakeWrappedSolIxArgs,
) -> ProgramResult {
    stake_wrapped_sol_invoke_with_program_id(crate::ID, accounts, args)
}
pub fn stake_wrapped_sol_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: StakeWrappedSolAccounts<'_, '_>,
    args: StakeWrappedSolIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: StakeWrappedSolKeys = accounts.into();
    let ix = stake_wrapped_sol_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn stake_wrapped_sol_invoke_signed(
    accounts: StakeWrappedSolAccounts<'_, '_>,
    args: StakeWrappedSolIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    stake_wrapped_sol_invoke_signed_with_program_id(crate::ID, accounts, args, seeds)
}
pub fn stake_wrapped_sol_verify_account_keys(
    accounts: StakeWrappedSolAccounts<'_, '_>,
    keys: StakeWrappedSolKeys,
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
pub fn stake_wrapped_sol_verify_writable_privileges<'me, 'info>(
    accounts: StakeWrappedSolAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.wsol_from,
        accounts.dest_token_to,
        accounts.wsol_bridge_in,
        accounts.sol_bridge_out,
        accounts.dest_token_fee_token_account,
        accounts.dest_token_mint,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn stake_wrapped_sol_verify_signer_privileges<'me, 'info>(
    accounts: StakeWrappedSolAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.user] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn stake_wrapped_sol_verify_account_privileges<'me, 'info>(
    accounts: StakeWrappedSolAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    stake_wrapped_sol_verify_writable_privileges(accounts)?;
    stake_wrapped_sol_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const SWAP_VIA_STAKE_IX_ACCOUNTS_LEN: usize = 7;
#[derive(Copy, Clone, Debug)]
pub struct SwapViaStakeAccounts<'me, 'info> {
    ///The authority of src_token_from. Needs to be mutable to support marinade deposit stake.
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
    ///Output token mint. If this is wrapped SOL, the account can be set to read-only. The withdraw stake and deposit stake accounts slices follow.
    pub dest_token_mint: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug)]
pub struct SwapViaStakeKeys {
    ///The authority of src_token_from. Needs to be mutable to support marinade deposit stake.
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
    ///Output token mint. If this is wrapped SOL, the account can be set to read-only. The withdraw stake and deposit stake accounts slices follow.
    pub dest_token_mint: Pubkey,
}
impl From<SwapViaStakeAccounts<'_, '_>> for SwapViaStakeKeys {
    fn from(accounts: SwapViaStakeAccounts) -> Self {
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
impl From<SwapViaStakeKeys> for [AccountMeta; SWAP_VIA_STAKE_IX_ACCOUNTS_LEN] {
    fn from(keys: SwapViaStakeKeys) -> Self {
        [
            AccountMeta::new(keys.user, false),
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
impl<'info> From<SwapViaStakeAccounts<'_, 'info>>
    for [AccountInfo<'info>; SWAP_VIA_STAKE_IX_ACCOUNTS_LEN]
{
    fn from(accounts: SwapViaStakeAccounts<'_, 'info>) -> Self {
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
pub const SWAP_VIA_STAKE_IX_DISCM: u8 = 1u8;
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct SwapViaStakeIxArgs {
    pub args: SwapViaStakeArgs,
}
#[derive(Clone, Debug, PartialEq)]
pub struct SwapViaStakeIxData(pub SwapViaStakeIxArgs);
impl From<SwapViaStakeIxArgs> for SwapViaStakeIxData {
    fn from(args: SwapViaStakeIxArgs) -> Self {
        Self(args)
    }
}
impl SwapViaStakeIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm_buf = [0u8; 1];
        reader.read_exact(&mut maybe_discm_buf)?;
        let maybe_discm = maybe_discm_buf[0];
        if maybe_discm != SWAP_VIA_STAKE_IX_DISCM {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    SWAP_VIA_STAKE_IX_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self(SwapViaStakeIxArgs::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&[SWAP_VIA_STAKE_IX_DISCM])?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn swap_via_stake_ix_with_program_id(
    program_id: Pubkey,
    keys: SwapViaStakeKeys,
    args: SwapViaStakeIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; SWAP_VIA_STAKE_IX_ACCOUNTS_LEN] = keys.into();
    let data: SwapViaStakeIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn swap_via_stake_ix(
    keys: SwapViaStakeKeys,
    args: SwapViaStakeIxArgs,
) -> std::io::Result<Instruction> {
    swap_via_stake_ix_with_program_id(crate::ID, keys, args)
}
pub fn swap_via_stake_invoke_with_program_id(
    program_id: Pubkey,
    accounts: SwapViaStakeAccounts<'_, '_>,
    args: SwapViaStakeIxArgs,
) -> ProgramResult {
    let keys: SwapViaStakeKeys = accounts.into();
    let ix = swap_via_stake_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn swap_via_stake_invoke(
    accounts: SwapViaStakeAccounts<'_, '_>,
    args: SwapViaStakeIxArgs,
) -> ProgramResult {
    swap_via_stake_invoke_with_program_id(crate::ID, accounts, args)
}
pub fn swap_via_stake_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: SwapViaStakeAccounts<'_, '_>,
    args: SwapViaStakeIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: SwapViaStakeKeys = accounts.into();
    let ix = swap_via_stake_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn swap_via_stake_invoke_signed(
    accounts: SwapViaStakeAccounts<'_, '_>,
    args: SwapViaStakeIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    swap_via_stake_invoke_signed_with_program_id(crate::ID, accounts, args, seeds)
}
pub fn swap_via_stake_verify_account_keys(
    accounts: SwapViaStakeAccounts<'_, '_>,
    keys: SwapViaStakeKeys,
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
pub fn swap_via_stake_verify_writable_privileges<'me, 'info>(
    accounts: SwapViaStakeAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
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
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn swap_via_stake_verify_signer_privileges<'me, 'info>(
    accounts: SwapViaStakeAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.user] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn swap_via_stake_verify_account_privileges<'me, 'info>(
    accounts: SwapViaStakeAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    swap_via_stake_verify_writable_privileges(accounts)?;
    swap_via_stake_verify_signer_privileges(accounts)?;
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
impl From<CreateFeeTokenAccountAccounts<'_, '_>> for CreateFeeTokenAccountKeys {
    fn from(accounts: CreateFeeTokenAccountAccounts) -> Self {
        Self {
            payer: *accounts.payer.key,
            fee_token_account: *accounts.fee_token_account.key,
            mint: *accounts.mint.key,
            token_program: *accounts.token_program.key,
            system_program: *accounts.system_program.key,
        }
    }
}
impl From<CreateFeeTokenAccountKeys> for [AccountMeta; CREATE_FEE_TOKEN_ACCOUNT_IX_ACCOUNTS_LEN] {
    fn from(keys: CreateFeeTokenAccountKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.payer,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.fee_token_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.system_program,
                is_signer: false,
                is_writable: false,
            },
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
impl<'info> From<CreateFeeTokenAccountAccounts<'_, 'info>>
    for [AccountInfo<'info>; CREATE_FEE_TOKEN_ACCOUNT_IX_ACCOUNTS_LEN]
{
    fn from(accounts: CreateFeeTokenAccountAccounts<'_, 'info>) -> Self {
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
pub const CREATE_FEE_TOKEN_ACCOUNT_IX_DISCM: u8 = 2u8;
#[derive(Clone, Debug, PartialEq)]
pub struct CreateFeeTokenAccountIxData;
impl CreateFeeTokenAccountIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm_buf = [0u8; 1];
        reader.read_exact(&mut maybe_discm_buf)?;
        let maybe_discm = maybe_discm_buf[0];
        if maybe_discm != CREATE_FEE_TOKEN_ACCOUNT_IX_DISCM {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    CREATE_FEE_TOKEN_ACCOUNT_IX_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&[CREATE_FEE_TOKEN_ACCOUNT_IX_DISCM])
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn create_fee_token_account_ix_with_program_id(
    program_id: Pubkey,
    keys: CreateFeeTokenAccountKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; CREATE_FEE_TOKEN_ACCOUNT_IX_ACCOUNTS_LEN] = keys.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: CreateFeeTokenAccountIxData.try_to_vec()?,
    })
}
pub fn create_fee_token_account_ix(
    keys: CreateFeeTokenAccountKeys,
) -> std::io::Result<Instruction> {
    create_fee_token_account_ix_with_program_id(crate::ID, keys)
}
pub fn create_fee_token_account_invoke_with_program_id(
    program_id: Pubkey,
    accounts: CreateFeeTokenAccountAccounts<'_, '_>,
) -> ProgramResult {
    let keys: CreateFeeTokenAccountKeys = accounts.into();
    let ix = create_fee_token_account_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn create_fee_token_account_invoke(
    accounts: CreateFeeTokenAccountAccounts<'_, '_>,
) -> ProgramResult {
    create_fee_token_account_invoke_with_program_id(crate::ID, accounts)
}
pub fn create_fee_token_account_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: CreateFeeTokenAccountAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: CreateFeeTokenAccountKeys = accounts.into();
    let ix = create_fee_token_account_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn create_fee_token_account_invoke_signed(
    accounts: CreateFeeTokenAccountAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    create_fee_token_account_invoke_signed_with_program_id(crate::ID, accounts, seeds)
}
pub fn create_fee_token_account_verify_account_keys(
    accounts: CreateFeeTokenAccountAccounts<'_, '_>,
    keys: CreateFeeTokenAccountKeys,
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
pub fn create_fee_token_account_verify_writable_privileges<'me, 'info>(
    accounts: CreateFeeTokenAccountAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.payer, accounts.fee_token_account] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn create_fee_token_account_verify_signer_privileges<'me, 'info>(
    accounts: CreateFeeTokenAccountAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.payer] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn create_fee_token_account_verify_account_privileges<'me, 'info>(
    accounts: CreateFeeTokenAccountAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    create_fee_token_account_verify_writable_privileges(accounts)?;
    create_fee_token_account_verify_signer_privileges(accounts)?;
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
impl From<CloseFeeTokenAccountAccounts<'_, '_>> for CloseFeeTokenAccountKeys {
    fn from(accounts: CloseFeeTokenAccountAccounts) -> Self {
        Self {
            admin: *accounts.admin.key,
            fee_token_account: *accounts.fee_token_account.key,
            close_to: *accounts.close_to.key,
            mint: *accounts.mint.key,
            token_program: *accounts.token_program.key,
        }
    }
}
impl From<CloseFeeTokenAccountKeys> for [AccountMeta; CLOSE_FEE_TOKEN_ACCOUNT_IX_ACCOUNTS_LEN] {
    fn from(keys: CloseFeeTokenAccountKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.admin,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.fee_token_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.close_to,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_program,
                is_signer: false,
                is_writable: false,
            },
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
impl<'info> From<CloseFeeTokenAccountAccounts<'_, 'info>>
    for [AccountInfo<'info>; CLOSE_FEE_TOKEN_ACCOUNT_IX_ACCOUNTS_LEN]
{
    fn from(accounts: CloseFeeTokenAccountAccounts<'_, 'info>) -> Self {
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
pub const CLOSE_FEE_TOKEN_ACCOUNT_IX_DISCM: u8 = 3u8;
#[derive(Clone, Debug, PartialEq)]
pub struct CloseFeeTokenAccountIxData;
impl CloseFeeTokenAccountIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm_buf = [0u8; 1];
        reader.read_exact(&mut maybe_discm_buf)?;
        let maybe_discm = maybe_discm_buf[0];
        if maybe_discm != CLOSE_FEE_TOKEN_ACCOUNT_IX_DISCM {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    CLOSE_FEE_TOKEN_ACCOUNT_IX_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&[CLOSE_FEE_TOKEN_ACCOUNT_IX_DISCM])
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn close_fee_token_account_ix_with_program_id(
    program_id: Pubkey,
    keys: CloseFeeTokenAccountKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; CLOSE_FEE_TOKEN_ACCOUNT_IX_ACCOUNTS_LEN] = keys.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: CloseFeeTokenAccountIxData.try_to_vec()?,
    })
}
pub fn close_fee_token_account_ix(keys: CloseFeeTokenAccountKeys) -> std::io::Result<Instruction> {
    close_fee_token_account_ix_with_program_id(crate::ID, keys)
}
pub fn close_fee_token_account_invoke_with_program_id(
    program_id: Pubkey,
    accounts: CloseFeeTokenAccountAccounts<'_, '_>,
) -> ProgramResult {
    let keys: CloseFeeTokenAccountKeys = accounts.into();
    let ix = close_fee_token_account_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn close_fee_token_account_invoke(
    accounts: CloseFeeTokenAccountAccounts<'_, '_>,
) -> ProgramResult {
    close_fee_token_account_invoke_with_program_id(crate::ID, accounts)
}
pub fn close_fee_token_account_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: CloseFeeTokenAccountAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: CloseFeeTokenAccountKeys = accounts.into();
    let ix = close_fee_token_account_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn close_fee_token_account_invoke_signed(
    accounts: CloseFeeTokenAccountAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    close_fee_token_account_invoke_signed_with_program_id(crate::ID, accounts, seeds)
}
pub fn close_fee_token_account_verify_account_keys(
    accounts: CloseFeeTokenAccountAccounts<'_, '_>,
    keys: CloseFeeTokenAccountKeys,
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
pub fn close_fee_token_account_verify_writable_privileges<'me, 'info>(
    accounts: CloseFeeTokenAccountAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.fee_token_account, accounts.close_to] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn close_fee_token_account_verify_signer_privileges<'me, 'info>(
    accounts: CloseFeeTokenAccountAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.admin] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn close_fee_token_account_verify_account_privileges<'me, 'info>(
    accounts: CloseFeeTokenAccountAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    close_fee_token_account_verify_writable_privileges(accounts)?;
    close_fee_token_account_verify_signer_privileges(accounts)?;
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
impl From<WithdrawFeesAccounts<'_, '_>> for WithdrawFeesKeys {
    fn from(accounts: WithdrawFeesAccounts) -> Self {
        Self {
            admin: *accounts.admin.key,
            fee_token_account: *accounts.fee_token_account.key,
            withdraw_to: *accounts.withdraw_to.key,
            mint: *accounts.mint.key,
            token_program: *accounts.token_program.key,
        }
    }
}
impl From<WithdrawFeesKeys> for [AccountMeta; WITHDRAW_FEES_IX_ACCOUNTS_LEN] {
    fn from(keys: WithdrawFeesKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.admin,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.fee_token_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.withdraw_to,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_program,
                is_signer: false,
                is_writable: false,
            },
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
impl<'info> From<WithdrawFeesAccounts<'_, 'info>>
    for [AccountInfo<'info>; WITHDRAW_FEES_IX_ACCOUNTS_LEN]
{
    fn from(accounts: WithdrawFeesAccounts<'_, 'info>) -> Self {
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
pub const WITHDRAW_FEES_IX_DISCM: u8 = 4u8;
#[derive(Clone, Debug, PartialEq)]
pub struct WithdrawFeesIxData;
impl WithdrawFeesIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm_buf = [0u8; 1];
        reader.read_exact(&mut maybe_discm_buf)?;
        let maybe_discm = maybe_discm_buf[0];
        if maybe_discm != WITHDRAW_FEES_IX_DISCM {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    WITHDRAW_FEES_IX_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&[WITHDRAW_FEES_IX_DISCM])
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn withdraw_fees_ix_with_program_id(
    program_id: Pubkey,
    keys: WithdrawFeesKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; WITHDRAW_FEES_IX_ACCOUNTS_LEN] = keys.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: WithdrawFeesIxData.try_to_vec()?,
    })
}
pub fn withdraw_fees_ix(keys: WithdrawFeesKeys) -> std::io::Result<Instruction> {
    withdraw_fees_ix_with_program_id(crate::ID, keys)
}
pub fn withdraw_fees_invoke_with_program_id(
    program_id: Pubkey,
    accounts: WithdrawFeesAccounts<'_, '_>,
) -> ProgramResult {
    let keys: WithdrawFeesKeys = accounts.into();
    let ix = withdraw_fees_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn withdraw_fees_invoke(accounts: WithdrawFeesAccounts<'_, '_>) -> ProgramResult {
    withdraw_fees_invoke_with_program_id(crate::ID, accounts)
}
pub fn withdraw_fees_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: WithdrawFeesAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: WithdrawFeesKeys = accounts.into();
    let ix = withdraw_fees_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn withdraw_fees_invoke_signed(
    accounts: WithdrawFeesAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    withdraw_fees_invoke_signed_with_program_id(crate::ID, accounts, seeds)
}
pub fn withdraw_fees_verify_account_keys(
    accounts: WithdrawFeesAccounts<'_, '_>,
    keys: WithdrawFeesKeys,
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
pub fn withdraw_fees_verify_writable_privileges<'me, 'info>(
    accounts: WithdrawFeesAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.fee_token_account, accounts.withdraw_to] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn withdraw_fees_verify_signer_privileges<'me, 'info>(
    accounts: WithdrawFeesAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.admin] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn withdraw_fees_verify_account_privileges<'me, 'info>(
    accounts: WithdrawFeesAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    withdraw_fees_verify_writable_privileges(accounts)?;
    withdraw_fees_verify_signer_privileges(accounts)?;
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
    ///Output token mint. If this is wrapped SOL, the account can be set to read-only. The deposit stake accounts slice follows.
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
    ///Output token mint. If this is wrapped SOL, the account can be set to read-only. The deposit stake accounts slice follows.
    pub dest_token_mint: Pubkey,
}
impl From<DepositStakeAccounts<'_, '_>> for DepositStakeKeys {
    fn from(accounts: DepositStakeAccounts) -> Self {
        Self {
            user: *accounts.user.key,
            stake_account: *accounts.stake_account.key,
            dest_token_to: *accounts.dest_token_to.key,
            dest_token_fee_token_account: *accounts.dest_token_fee_token_account.key,
            dest_token_mint: *accounts.dest_token_mint.key,
        }
    }
}
impl From<DepositStakeKeys> for [AccountMeta; DEPOSIT_STAKE_IX_ACCOUNTS_LEN] {
    fn from(keys: DepositStakeKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.user,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.stake_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.dest_token_to,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.dest_token_fee_token_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.dest_token_mint,
                is_signer: false,
                is_writable: true,
            },
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
impl<'info> From<DepositStakeAccounts<'_, 'info>>
    for [AccountInfo<'info>; DEPOSIT_STAKE_IX_ACCOUNTS_LEN]
{
    fn from(accounts: DepositStakeAccounts<'_, 'info>) -> Self {
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
pub const DEPOSIT_STAKE_IX_DISCM: u8 = 5u8;
#[derive(Clone, Debug, PartialEq)]
pub struct DepositStakeIxData;
impl DepositStakeIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm_buf = [0u8; 1];
        reader.read_exact(&mut maybe_discm_buf)?;
        let maybe_discm = maybe_discm_buf[0];
        if maybe_discm != DEPOSIT_STAKE_IX_DISCM {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    DEPOSIT_STAKE_IX_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&[DEPOSIT_STAKE_IX_DISCM])
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn deposit_stake_ix_with_program_id(
    program_id: Pubkey,
    keys: DepositStakeKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; DEPOSIT_STAKE_IX_ACCOUNTS_LEN] = keys.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: DepositStakeIxData.try_to_vec()?,
    })
}
pub fn deposit_stake_ix(keys: DepositStakeKeys) -> std::io::Result<Instruction> {
    deposit_stake_ix_with_program_id(crate::ID, keys)
}
pub fn deposit_stake_invoke_with_program_id(
    program_id: Pubkey,
    accounts: DepositStakeAccounts<'_, '_>,
) -> ProgramResult {
    let keys: DepositStakeKeys = accounts.into();
    let ix = deposit_stake_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn deposit_stake_invoke(accounts: DepositStakeAccounts<'_, '_>) -> ProgramResult {
    deposit_stake_invoke_with_program_id(crate::ID, accounts)
}
pub fn deposit_stake_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: DepositStakeAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: DepositStakeKeys = accounts.into();
    let ix = deposit_stake_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn deposit_stake_invoke_signed(
    accounts: DepositStakeAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    deposit_stake_invoke_signed_with_program_id(crate::ID, accounts, seeds)
}
pub fn deposit_stake_verify_account_keys(
    accounts: DepositStakeAccounts<'_, '_>,
    keys: DepositStakeKeys,
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
pub fn deposit_stake_verify_writable_privileges<'me, 'info>(
    accounts: DepositStakeAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.user,
        accounts.stake_account,
        accounts.dest_token_to,
        accounts.dest_token_fee_token_account,
        accounts.dest_token_mint,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn deposit_stake_verify_signer_privileges<'me, 'info>(
    accounts: DepositStakeAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.user] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn deposit_stake_verify_account_privileges<'me, 'info>(
    accounts: DepositStakeAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    deposit_stake_verify_writable_privileges(accounts)?;
    deposit_stake_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const PREFUND_WITHDRAW_STAKE_IX_ACCOUNTS_LEN: usize = 16;
#[derive(Copy, Clone, Debug)]
pub struct PrefundWithdrawStakeAccounts<'me, 'info> {
    ///The withdraw authority of stake_account. Needs to be mutable and system account to receive slumlord flash loan.
    pub user: &'me AccountInfo<'info>,
    ///The token account to burn src tokens from in order to withdraw stake
    pub src_token_from: &'me AccountInfo<'info>,
    ///The bridge stake account thats withdrawn and given to the user. PDA. seeds = ['bridge_stake', user.pubkey, SwapArgs.bridge_stake_seed]. Might be long-lived, make sure the seed is not already in use
    pub bridge_stake: &'me AccountInfo<'info>,
    ///Input LST token mint
    pub src_token_mint: &'me AccountInfo<'info>,
    ///The system account PDA that contains enough SOL to prefund 2 stake accounts for withdrawal. Someone must send SOL to here to initialize it. Seeds = ['prefunder']
    pub prefunder: &'me AccountInfo<'info>,
    ///The slumdog stake account is split from bridge_stake upon stake withdraw and instant unstaked to repay slumlord's flash loan. create_with_seed(bridge_stake.pubkey, 'slumdog', stake_program). Might be long-lived, but should be not in use as long as bridge_stake is not in use
    pub slumdog_stake: &'me AccountInfo<'info>,
    ///Sanctum unstake program. unpXTU2Ndrc7WWNyEhQWe4udTzSibLPi25SXv2xbCHQ
    pub unstakeit_program: &'me AccountInfo<'info>,
    ///Sanctum unstake pool. FypPtwbY3FUfzJUtXHSyVRokVKG2jKtH29FmK4ebxRSd
    pub unstake_pool: &'me AccountInfo<'info>,
    ///Sanctum unstake pool SOL reserves. 3rBnnH9TTgd3xwu48rnzGsaQkSr1hR64nY71DrDt6VrQ
    pub pool_sol_reserves: &'me AccountInfo<'info>,
    ///Sanctum unstake pool Fee account. 5Pcu8WeQa3VbBz2vdBT49Rj4gbS4hsnfzuL1LmuRaKFY
    pub unstake_fee: &'me AccountInfo<'info>,
    ///Sanctum unstake pool stake account record for slumdog stake. PDA of sanctum unstake program. Seeds = [unstakePool.pubkey, slumdogStake.pubkey].
    pub slumdog_stake_acc_record: &'me AccountInfo<'info>,
    ///Sanctum unstake pool protocol fee account. 2hN9UhvRFVfPYKL6rZJ5YiLEPCLTpN755pgwDJHWgFbU
    pub unstake_protocol_fee: &'me AccountInfo<'info>,
    ///Sanctum unstake pool protocol fee destination. unstakeProtocolFee.destination
    pub unstake_protocol_fee_dest: &'me AccountInfo<'info>,
    ///sysvar clock
    pub clock: &'me AccountInfo<'info>,
    ///stake program
    pub stake_program: &'me AccountInfo<'info>,
    ///System program. The withdraw stake accounts slices follow.
    pub system_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug)]
pub struct PrefundWithdrawStakeKeys {
    ///The withdraw authority of stake_account. Needs to be mutable and system account to receive slumlord flash loan.
    pub user: Pubkey,
    ///The token account to burn src tokens from in order to withdraw stake
    pub src_token_from: Pubkey,
    ///The bridge stake account thats withdrawn and given to the user. PDA. seeds = ['bridge_stake', user.pubkey, SwapArgs.bridge_stake_seed]. Might be long-lived, make sure the seed is not already in use
    pub bridge_stake: Pubkey,
    ///Input LST token mint
    pub src_token_mint: Pubkey,
    ///The system account PDA that contains enough SOL to prefund 2 stake accounts for withdrawal. Someone must send SOL to here to initialize it. Seeds = ['prefunder']
    pub prefunder: Pubkey,
    ///The slumdog stake account is split from bridge_stake upon stake withdraw and instant unstaked to repay slumlord's flash loan. create_with_seed(bridge_stake.pubkey, 'slumdog', stake_program). Might be long-lived, but should be not in use as long as bridge_stake is not in use
    pub slumdog_stake: Pubkey,
    ///Sanctum unstake program. unpXTU2Ndrc7WWNyEhQWe4udTzSibLPi25SXv2xbCHQ
    pub unstakeit_program: Pubkey,
    ///Sanctum unstake pool. FypPtwbY3FUfzJUtXHSyVRokVKG2jKtH29FmK4ebxRSd
    pub unstake_pool: Pubkey,
    ///Sanctum unstake pool SOL reserves. 3rBnnH9TTgd3xwu48rnzGsaQkSr1hR64nY71DrDt6VrQ
    pub pool_sol_reserves: Pubkey,
    ///Sanctum unstake pool Fee account. 5Pcu8WeQa3VbBz2vdBT49Rj4gbS4hsnfzuL1LmuRaKFY
    pub unstake_fee: Pubkey,
    ///Sanctum unstake pool stake account record for slumdog stake. PDA of sanctum unstake program. Seeds = [unstakePool.pubkey, slumdogStake.pubkey].
    pub slumdog_stake_acc_record: Pubkey,
    ///Sanctum unstake pool protocol fee account. 2hN9UhvRFVfPYKL6rZJ5YiLEPCLTpN755pgwDJHWgFbU
    pub unstake_protocol_fee: Pubkey,
    ///Sanctum unstake pool protocol fee destination. unstakeProtocolFee.destination
    pub unstake_protocol_fee_dest: Pubkey,
    ///sysvar clock
    pub clock: Pubkey,
    ///stake program
    pub stake_program: Pubkey,
    ///System program. The withdraw stake accounts slices follow.
    pub system_program: Pubkey,
}
impl From<PrefundWithdrawStakeAccounts<'_, '_>> for PrefundWithdrawStakeKeys {
    fn from(accounts: PrefundWithdrawStakeAccounts) -> Self {
        Self {
            user: *accounts.user.key,
            src_token_from: *accounts.src_token_from.key,
            bridge_stake: *accounts.bridge_stake.key,
            src_token_mint: *accounts.src_token_mint.key,
            prefunder: *accounts.prefunder.key,
            slumdog_stake: *accounts.slumdog_stake.key,
            unstakeit_program: *accounts.unstakeit_program.key,
            unstake_pool: *accounts.unstake_pool.key,
            pool_sol_reserves: *accounts.pool_sol_reserves.key,
            unstake_fee: *accounts.unstake_fee.key,
            slumdog_stake_acc_record: *accounts.slumdog_stake_acc_record.key,
            unstake_protocol_fee: *accounts.unstake_protocol_fee.key,
            unstake_protocol_fee_dest: *accounts.unstake_protocol_fee_dest.key,
            clock: *accounts.clock.key,
            stake_program: *accounts.stake_program.key,
            system_program: *accounts.system_program.key,
        }
    }
}
impl From<PrefundWithdrawStakeKeys> for [AccountMeta; PREFUND_WITHDRAW_STAKE_IX_ACCOUNTS_LEN] {
    fn from(keys: PrefundWithdrawStakeKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.user,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.src_token_from,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.bridge_stake,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.src_token_mint,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.prefunder,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.slumdog_stake,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.unstakeit_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.unstake_pool,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.pool_sol_reserves,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.unstake_fee,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.slumdog_stake_acc_record,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.unstake_protocol_fee,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.unstake_protocol_fee_dest,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.clock,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.stake_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.system_program,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; PREFUND_WITHDRAW_STAKE_IX_ACCOUNTS_LEN]> for PrefundWithdrawStakeKeys {
    fn from(pubkeys: [Pubkey; PREFUND_WITHDRAW_STAKE_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            user: pubkeys[0],
            src_token_from: pubkeys[1],
            bridge_stake: pubkeys[2],
            src_token_mint: pubkeys[3],
            prefunder: pubkeys[4],
            slumdog_stake: pubkeys[5],
            unstakeit_program: pubkeys[6],
            unstake_pool: pubkeys[7],
            pool_sol_reserves: pubkeys[8],
            unstake_fee: pubkeys[9],
            slumdog_stake_acc_record: pubkeys[10],
            unstake_protocol_fee: pubkeys[11],
            unstake_protocol_fee_dest: pubkeys[12],
            clock: pubkeys[13],
            stake_program: pubkeys[14],
            system_program: pubkeys[15],
        }
    }
}
impl<'info> From<PrefundWithdrawStakeAccounts<'_, 'info>>
    for [AccountInfo<'info>; PREFUND_WITHDRAW_STAKE_IX_ACCOUNTS_LEN]
{
    fn from(accounts: PrefundWithdrawStakeAccounts<'_, 'info>) -> Self {
        [
            accounts.user.clone(),
            accounts.src_token_from.clone(),
            accounts.bridge_stake.clone(),
            accounts.src_token_mint.clone(),
            accounts.prefunder.clone(),
            accounts.slumdog_stake.clone(),
            accounts.unstakeit_program.clone(),
            accounts.unstake_pool.clone(),
            accounts.pool_sol_reserves.clone(),
            accounts.unstake_fee.clone(),
            accounts.slumdog_stake_acc_record.clone(),
            accounts.unstake_protocol_fee.clone(),
            accounts.unstake_protocol_fee_dest.clone(),
            accounts.clock.clone(),
            accounts.stake_program.clone(),
            accounts.system_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; PREFUND_WITHDRAW_STAKE_IX_ACCOUNTS_LEN]>
    for PrefundWithdrawStakeAccounts<'me, 'info>
{
    fn from(arr: &'me [AccountInfo<'info>; PREFUND_WITHDRAW_STAKE_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            user: &arr[0],
            src_token_from: &arr[1],
            bridge_stake: &arr[2],
            src_token_mint: &arr[3],
            prefunder: &arr[4],
            slumdog_stake: &arr[5],
            unstakeit_program: &arr[6],
            unstake_pool: &arr[7],
            pool_sol_reserves: &arr[8],
            unstake_fee: &arr[9],
            slumdog_stake_acc_record: &arr[10],
            unstake_protocol_fee: &arr[11],
            unstake_protocol_fee_dest: &arr[12],
            clock: &arr[13],
            stake_program: &arr[14],
            system_program: &arr[15],
        }
    }
}
pub const PREFUND_WITHDRAW_STAKE_IX_DISCM: u8 = 6u8;
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct PrefundWithdrawStakeIxArgs {
    pub args: SwapViaStakeArgs,
}
#[derive(Clone, Debug, PartialEq)]
pub struct PrefundWithdrawStakeIxData(pub PrefundWithdrawStakeIxArgs);
impl From<PrefundWithdrawStakeIxArgs> for PrefundWithdrawStakeIxData {
    fn from(args: PrefundWithdrawStakeIxArgs) -> Self {
        Self(args)
    }
}
impl PrefundWithdrawStakeIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm_buf = [0u8; 1];
        reader.read_exact(&mut maybe_discm_buf)?;
        let maybe_discm = maybe_discm_buf[0];
        if maybe_discm != PREFUND_WITHDRAW_STAKE_IX_DISCM {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    PREFUND_WITHDRAW_STAKE_IX_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self(PrefundWithdrawStakeIxArgs::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&[PREFUND_WITHDRAW_STAKE_IX_DISCM])?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn prefund_withdraw_stake_ix_with_program_id(
    program_id: Pubkey,
    keys: PrefundWithdrawStakeKeys,
    args: PrefundWithdrawStakeIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; PREFUND_WITHDRAW_STAKE_IX_ACCOUNTS_LEN] = keys.into();
    let data: PrefundWithdrawStakeIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn prefund_withdraw_stake_ix(
    keys: PrefundWithdrawStakeKeys,
    args: PrefundWithdrawStakeIxArgs,
) -> std::io::Result<Instruction> {
    prefund_withdraw_stake_ix_with_program_id(crate::ID, keys, args)
}
pub fn prefund_withdraw_stake_invoke_with_program_id(
    program_id: Pubkey,
    accounts: PrefundWithdrawStakeAccounts<'_, '_>,
    args: PrefundWithdrawStakeIxArgs,
) -> ProgramResult {
    let keys: PrefundWithdrawStakeKeys = accounts.into();
    let ix = prefund_withdraw_stake_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn prefund_withdraw_stake_invoke(
    accounts: PrefundWithdrawStakeAccounts<'_, '_>,
    args: PrefundWithdrawStakeIxArgs,
) -> ProgramResult {
    prefund_withdraw_stake_invoke_with_program_id(crate::ID, accounts, args)
}
pub fn prefund_withdraw_stake_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: PrefundWithdrawStakeAccounts<'_, '_>,
    args: PrefundWithdrawStakeIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: PrefundWithdrawStakeKeys = accounts.into();
    let ix = prefund_withdraw_stake_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn prefund_withdraw_stake_invoke_signed(
    accounts: PrefundWithdrawStakeAccounts<'_, '_>,
    args: PrefundWithdrawStakeIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    prefund_withdraw_stake_invoke_signed_with_program_id(crate::ID, accounts, args, seeds)
}
pub fn prefund_withdraw_stake_verify_account_keys(
    accounts: PrefundWithdrawStakeAccounts<'_, '_>,
    keys: PrefundWithdrawStakeKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (accounts.user.key, &keys.user),
        (accounts.src_token_from.key, &keys.src_token_from),
        (accounts.bridge_stake.key, &keys.bridge_stake),
        (accounts.src_token_mint.key, &keys.src_token_mint),
        (accounts.prefunder.key, &keys.prefunder),
        (accounts.slumdog_stake.key, &keys.slumdog_stake),
        (accounts.unstakeit_program.key, &keys.unstakeit_program),
        (accounts.unstake_pool.key, &keys.unstake_pool),
        (accounts.pool_sol_reserves.key, &keys.pool_sol_reserves),
        (accounts.unstake_fee.key, &keys.unstake_fee),
        (
            accounts.slumdog_stake_acc_record.key,
            &keys.slumdog_stake_acc_record,
        ),
        (
            accounts.unstake_protocol_fee.key,
            &keys.unstake_protocol_fee,
        ),
        (
            accounts.unstake_protocol_fee_dest.key,
            &keys.unstake_protocol_fee_dest,
        ),
        (accounts.clock.key, &keys.clock),
        (accounts.stake_program.key, &keys.stake_program),
        (accounts.system_program.key, &keys.system_program),
    ] {
        if actual != expected {
            return Err((*actual, *expected));
        }
    }
    Ok(())
}
pub fn prefund_withdraw_stake_verify_writable_privileges<'me, 'info>(
    accounts: PrefundWithdrawStakeAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.user,
        accounts.src_token_from,
        accounts.bridge_stake,
        accounts.src_token_mint,
        accounts.prefunder,
        accounts.slumdog_stake,
        accounts.unstake_pool,
        accounts.pool_sol_reserves,
        accounts.slumdog_stake_acc_record,
        accounts.unstake_protocol_fee_dest,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn prefund_withdraw_stake_verify_signer_privileges<'me, 'info>(
    accounts: PrefundWithdrawStakeAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.user] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn prefund_withdraw_stake_verify_account_privileges<'me, 'info>(
    accounts: PrefundWithdrawStakeAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    prefund_withdraw_stake_verify_writable_privileges(accounts)?;
    prefund_withdraw_stake_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const PREFUND_SWAP_VIA_STAKE_IX_ACCOUNTS_LEN: usize = 19;
#[derive(Copy, Clone, Debug)]
pub struct PrefundSwapViaStakeAccounts<'me, 'info> {
    ///The authority of src_token_from, mutable system account. Prefunds the rent for the stake accounts, amount is refunded via instant unstake.
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
    ///The system account PDA that contains enough SOL to prefund 2 stake accounts for withdrawal. Someone must send SOL to here to initialize it. Seeds = ['prefunder']
    pub prefunder: &'me AccountInfo<'info>,
    ///The slumdog stake account is split from bridge_stake upon stake withdraw and instant unstaked to refund user. create_with_seed(bridge_stake.pubkey, 'slumdog', stake_program). Might be long-lived, but should be not in use as long as bridge_stake is not in use
    pub slumdog_stake: &'me AccountInfo<'info>,
    ///Sanctum unstake program. unpXTU2Ndrc7WWNyEhQWe4udTzSibLPi25SXv2xbCHQ
    pub unstakeit_program: &'me AccountInfo<'info>,
    ///Sanctum unstake pool. FypPtwbY3FUfzJUtXHSyVRokVKG2jKtH29FmK4ebxRSd
    pub unstake_pool: &'me AccountInfo<'info>,
    ///Sanctum unstake pool SOL reserves. 3rBnnH9TTgd3xwu48rnzGsaQkSr1hR64nY71DrDt6VrQ
    pub pool_sol_reserves: &'me AccountInfo<'info>,
    ///Sanctum unstake pool Fee account. 5Pcu8WeQa3VbBz2vdBT49Rj4gbS4hsnfzuL1LmuRaKFY
    pub unstake_fee: &'me AccountInfo<'info>,
    ///Sanctum unstake pool stake account record for slumdog stake. PDA of sanctum unstake program. Seeds = [unstakePool.pubkey, slumdogStake.pubkey].
    pub slumdog_stake_acc_record: &'me AccountInfo<'info>,
    ///Sanctum unstake pool protocol fee account. 2hN9UhvRFVfPYKL6rZJ5YiLEPCLTpN755pgwDJHWgFbU
    pub unstake_protocol_fee: &'me AccountInfo<'info>,
    ///Sanctum unstake pool protocol fee destination. unstakeProtocolFee.destination
    pub unstake_protocol_fee_dest: &'me AccountInfo<'info>,
    ///sysvar clock
    pub clock: &'me AccountInfo<'info>,
    ///stake program
    pub stake_program: &'me AccountInfo<'info>,
    ///System program. The withdraw stake and deposit stake accounts slices follow.
    pub system_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug)]
pub struct PrefundSwapViaStakeKeys {
    ///The authority of src_token_from, mutable system account. Prefunds the rent for the stake accounts, amount is refunded via instant unstake.
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
    ///The system account PDA that contains enough SOL to prefund 2 stake accounts for withdrawal. Someone must send SOL to here to initialize it. Seeds = ['prefunder']
    pub prefunder: Pubkey,
    ///The slumdog stake account is split from bridge_stake upon stake withdraw and instant unstaked to refund user. create_with_seed(bridge_stake.pubkey, 'slumdog', stake_program). Might be long-lived, but should be not in use as long as bridge_stake is not in use
    pub slumdog_stake: Pubkey,
    ///Sanctum unstake program. unpXTU2Ndrc7WWNyEhQWe4udTzSibLPi25SXv2xbCHQ
    pub unstakeit_program: Pubkey,
    ///Sanctum unstake pool. FypPtwbY3FUfzJUtXHSyVRokVKG2jKtH29FmK4ebxRSd
    pub unstake_pool: Pubkey,
    ///Sanctum unstake pool SOL reserves. 3rBnnH9TTgd3xwu48rnzGsaQkSr1hR64nY71DrDt6VrQ
    pub pool_sol_reserves: Pubkey,
    ///Sanctum unstake pool Fee account. 5Pcu8WeQa3VbBz2vdBT49Rj4gbS4hsnfzuL1LmuRaKFY
    pub unstake_fee: Pubkey,
    ///Sanctum unstake pool stake account record for slumdog stake. PDA of sanctum unstake program. Seeds = [unstakePool.pubkey, slumdogStake.pubkey].
    pub slumdog_stake_acc_record: Pubkey,
    ///Sanctum unstake pool protocol fee account. 2hN9UhvRFVfPYKL6rZJ5YiLEPCLTpN755pgwDJHWgFbU
    pub unstake_protocol_fee: Pubkey,
    ///Sanctum unstake pool protocol fee destination. unstakeProtocolFee.destination
    pub unstake_protocol_fee_dest: Pubkey,
    ///sysvar clock
    pub clock: Pubkey,
    ///stake program
    pub stake_program: Pubkey,
    ///System program. The withdraw stake and deposit stake accounts slices follow.
    pub system_program: Pubkey,
}
impl From<PrefundSwapViaStakeAccounts<'_, '_>> for PrefundSwapViaStakeKeys {
    fn from(accounts: PrefundSwapViaStakeAccounts) -> Self {
        Self {
            user: *accounts.user.key,
            src_token_from: *accounts.src_token_from.key,
            dest_token_to: *accounts.dest_token_to.key,
            bridge_stake: *accounts.bridge_stake.key,
            dest_token_fee_token_account: *accounts.dest_token_fee_token_account.key,
            src_token_mint: *accounts.src_token_mint.key,
            dest_token_mint: *accounts.dest_token_mint.key,
            prefunder: *accounts.prefunder.key,
            slumdog_stake: *accounts.slumdog_stake.key,
            unstakeit_program: *accounts.unstakeit_program.key,
            unstake_pool: *accounts.unstake_pool.key,
            pool_sol_reserves: *accounts.pool_sol_reserves.key,
            unstake_fee: *accounts.unstake_fee.key,
            slumdog_stake_acc_record: *accounts.slumdog_stake_acc_record.key,
            unstake_protocol_fee: *accounts.unstake_protocol_fee.key,
            unstake_protocol_fee_dest: *accounts.unstake_protocol_fee_dest.key,
            clock: *accounts.clock.key,
            stake_program: *accounts.stake_program.key,
            system_program: *accounts.system_program.key,
        }
    }
}
impl From<PrefundSwapViaStakeKeys> for [AccountMeta; PREFUND_SWAP_VIA_STAKE_IX_ACCOUNTS_LEN] {
    fn from(keys: PrefundSwapViaStakeKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.user,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.src_token_from,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.dest_token_to,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.bridge_stake,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.dest_token_fee_token_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.src_token_mint,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.dest_token_mint,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.prefunder,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.slumdog_stake,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.unstakeit_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.unstake_pool,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.pool_sol_reserves,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.unstake_fee,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.slumdog_stake_acc_record,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.unstake_protocol_fee,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.unstake_protocol_fee_dest,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.clock,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.stake_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.system_program,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; PREFUND_SWAP_VIA_STAKE_IX_ACCOUNTS_LEN]> for PrefundSwapViaStakeKeys {
    fn from(pubkeys: [Pubkey; PREFUND_SWAP_VIA_STAKE_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            user: pubkeys[0],
            src_token_from: pubkeys[1],
            dest_token_to: pubkeys[2],
            bridge_stake: pubkeys[3],
            dest_token_fee_token_account: pubkeys[4],
            src_token_mint: pubkeys[5],
            dest_token_mint: pubkeys[6],
            prefunder: pubkeys[7],
            slumdog_stake: pubkeys[8],
            unstakeit_program: pubkeys[9],
            unstake_pool: pubkeys[10],
            pool_sol_reserves: pubkeys[11],
            unstake_fee: pubkeys[12],
            slumdog_stake_acc_record: pubkeys[13],
            unstake_protocol_fee: pubkeys[14],
            unstake_protocol_fee_dest: pubkeys[15],
            clock: pubkeys[16],
            stake_program: pubkeys[17],
            system_program: pubkeys[18],
        }
    }
}
impl<'info> From<PrefundSwapViaStakeAccounts<'_, 'info>>
    for [AccountInfo<'info>; PREFUND_SWAP_VIA_STAKE_IX_ACCOUNTS_LEN]
{
    fn from(accounts: PrefundSwapViaStakeAccounts<'_, 'info>) -> Self {
        [
            accounts.user.clone(),
            accounts.src_token_from.clone(),
            accounts.dest_token_to.clone(),
            accounts.bridge_stake.clone(),
            accounts.dest_token_fee_token_account.clone(),
            accounts.src_token_mint.clone(),
            accounts.dest_token_mint.clone(),
            accounts.prefunder.clone(),
            accounts.slumdog_stake.clone(),
            accounts.unstakeit_program.clone(),
            accounts.unstake_pool.clone(),
            accounts.pool_sol_reserves.clone(),
            accounts.unstake_fee.clone(),
            accounts.slumdog_stake_acc_record.clone(),
            accounts.unstake_protocol_fee.clone(),
            accounts.unstake_protocol_fee_dest.clone(),
            accounts.clock.clone(),
            accounts.stake_program.clone(),
            accounts.system_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; PREFUND_SWAP_VIA_STAKE_IX_ACCOUNTS_LEN]>
    for PrefundSwapViaStakeAccounts<'me, 'info>
{
    fn from(arr: &'me [AccountInfo<'info>; PREFUND_SWAP_VIA_STAKE_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            user: &arr[0],
            src_token_from: &arr[1],
            dest_token_to: &arr[2],
            bridge_stake: &arr[3],
            dest_token_fee_token_account: &arr[4],
            src_token_mint: &arr[5],
            dest_token_mint: &arr[6],
            prefunder: &arr[7],
            slumdog_stake: &arr[8],
            unstakeit_program: &arr[9],
            unstake_pool: &arr[10],
            pool_sol_reserves: &arr[11],
            unstake_fee: &arr[12],
            slumdog_stake_acc_record: &arr[13],
            unstake_protocol_fee: &arr[14],
            unstake_protocol_fee_dest: &arr[15],
            clock: &arr[16],
            stake_program: &arr[17],
            system_program: &arr[18],
        }
    }
}
pub const PREFUND_SWAP_VIA_STAKE_IX_DISCM: u8 = 7u8;
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct PrefundSwapViaStakeIxArgs {
    pub args: SwapViaStakeArgs,
}
#[derive(Clone, Debug, PartialEq)]
pub struct PrefundSwapViaStakeIxData(pub PrefundSwapViaStakeIxArgs);
impl From<PrefundSwapViaStakeIxArgs> for PrefundSwapViaStakeIxData {
    fn from(args: PrefundSwapViaStakeIxArgs) -> Self {
        Self(args)
    }
}
impl PrefundSwapViaStakeIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm_buf = [0u8; 1];
        reader.read_exact(&mut maybe_discm_buf)?;
        let maybe_discm = maybe_discm_buf[0];
        if maybe_discm != PREFUND_SWAP_VIA_STAKE_IX_DISCM {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    PREFUND_SWAP_VIA_STAKE_IX_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self(PrefundSwapViaStakeIxArgs::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&[PREFUND_SWAP_VIA_STAKE_IX_DISCM])?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn prefund_swap_via_stake_ix_with_program_id(
    program_id: Pubkey,
    keys: PrefundSwapViaStakeKeys,
    args: PrefundSwapViaStakeIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; PREFUND_SWAP_VIA_STAKE_IX_ACCOUNTS_LEN] = keys.into();
    let data: PrefundSwapViaStakeIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn prefund_swap_via_stake_ix(
    keys: PrefundSwapViaStakeKeys,
    args: PrefundSwapViaStakeIxArgs,
) -> std::io::Result<Instruction> {
    prefund_swap_via_stake_ix_with_program_id(crate::ID, keys, args)
}
pub fn prefund_swap_via_stake_invoke_with_program_id(
    program_id: Pubkey,
    accounts: PrefundSwapViaStakeAccounts<'_, '_>,
    args: PrefundSwapViaStakeIxArgs,
) -> ProgramResult {
    let keys: PrefundSwapViaStakeKeys = accounts.into();
    let ix = prefund_swap_via_stake_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn prefund_swap_via_stake_invoke(
    accounts: PrefundSwapViaStakeAccounts<'_, '_>,
    args: PrefundSwapViaStakeIxArgs,
) -> ProgramResult {
    prefund_swap_via_stake_invoke_with_program_id(crate::ID, accounts, args)
}
pub fn prefund_swap_via_stake_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: PrefundSwapViaStakeAccounts<'_, '_>,
    args: PrefundSwapViaStakeIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: PrefundSwapViaStakeKeys = accounts.into();
    let ix = prefund_swap_via_stake_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn prefund_swap_via_stake_invoke_signed(
    accounts: PrefundSwapViaStakeAccounts<'_, '_>,
    args: PrefundSwapViaStakeIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    prefund_swap_via_stake_invoke_signed_with_program_id(crate::ID, accounts, args, seeds)
}
pub fn prefund_swap_via_stake_verify_account_keys(
    accounts: PrefundSwapViaStakeAccounts<'_, '_>,
    keys: PrefundSwapViaStakeKeys,
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
        (accounts.prefunder.key, &keys.prefunder),
        (accounts.slumdog_stake.key, &keys.slumdog_stake),
        (accounts.unstakeit_program.key, &keys.unstakeit_program),
        (accounts.unstake_pool.key, &keys.unstake_pool),
        (accounts.pool_sol_reserves.key, &keys.pool_sol_reserves),
        (accounts.unstake_fee.key, &keys.unstake_fee),
        (
            accounts.slumdog_stake_acc_record.key,
            &keys.slumdog_stake_acc_record,
        ),
        (
            accounts.unstake_protocol_fee.key,
            &keys.unstake_protocol_fee,
        ),
        (
            accounts.unstake_protocol_fee_dest.key,
            &keys.unstake_protocol_fee_dest,
        ),
        (accounts.clock.key, &keys.clock),
        (accounts.stake_program.key, &keys.stake_program),
        (accounts.system_program.key, &keys.system_program),
    ] {
        if actual != expected {
            return Err((*actual, *expected));
        }
    }
    Ok(())
}
pub fn prefund_swap_via_stake_verify_writable_privileges<'me, 'info>(
    accounts: PrefundSwapViaStakeAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.user,
        accounts.src_token_from,
        accounts.dest_token_to,
        accounts.bridge_stake,
        accounts.dest_token_fee_token_account,
        accounts.src_token_mint,
        accounts.dest_token_mint,
        accounts.prefunder,
        accounts.slumdog_stake,
        accounts.unstake_pool,
        accounts.pool_sol_reserves,
        accounts.slumdog_stake_acc_record,
        accounts.unstake_protocol_fee_dest,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn prefund_swap_via_stake_verify_signer_privileges<'me, 'info>(
    accounts: PrefundSwapViaStakeAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.user] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn prefund_swap_via_stake_verify_account_privileges<'me, 'info>(
    accounts: PrefundSwapViaStakeAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    prefund_swap_via_stake_verify_writable_privileges(accounts)?;
    prefund_swap_via_stake_verify_signer_privileges(accounts)?;
    Ok(())
}
