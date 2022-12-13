use crate::*;
use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::AccountInfo,
    entrypoint::ProgramResult,
    instruction::{AccountMeta, Instruction},
    program::{invoke, invoke_signed},
    pubkey::Pubkey,
};
#[derive(Copy, Clone, Debug)]
pub struct StakeWrappedSolAccounts<
    'me,
    'a0: 'me,
    'a1: 'me,
    'a2: 'me,
    'a3: 'me,
    'a4: 'me,
    'a5: 'me,
    'a6: 'me,
    'a7: 'me,
    'a8: 'me,
    'a9: 'me,
> {
    ///The authority of wsol_account
    pub user: &'me AccountInfo<'a0>,
    ///The wrapped SOL token account to stake wrapped SOL from
    pub wsol_from: &'me AccountInfo<'a1>,
    ///The liquid staked SOL token account to receive the resulting tokens
    pub dest_token_to: &'me AccountInfo<'a2>,
    ///The PDA that serves as the wSOL account to bridge user's wSOL to SOL. Pubkey::create_with_seed(). base = sol_bridge_out.pubkey, seed = 'wsol_bridge_in'. owner = token_program
    pub wsol_bridge_in: &'me AccountInfo<'a3>,
    ///The PDA that serves as the system account to bridge user's wSOL to SOL. Seeds = ['sol_bridge_out']
    pub sol_bridge_out: &'me AccountInfo<'a4>,
    ///The liquid staked SOL token account collecting fees. PDA. Seeds = ['fee', dest_token_mint.pubkey]
    pub dest_token_fee_token_account: &'me AccountInfo<'a5>,
    ///The liquid staked SOL mint
    pub dest_token_mint: &'me AccountInfo<'a6>,
    ///wSOL token mint
    pub wsol_mint: &'me AccountInfo<'a7>,
    pub token_program: &'me AccountInfo<'a8>,
    pub system_program: &'me AccountInfo<'a9>,
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
impl<'me> From<&StakeWrappedSolAccounts<'me, '_, '_, '_, '_, '_, '_, '_, '_, '_, '_>>
    for StakeWrappedSolKeys
{
    fn from(
        accounts: &StakeWrappedSolAccounts<'me, '_, '_, '_, '_, '_, '_, '_, '_, '_, '_>,
    ) -> Self {
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
impl From<&StakeWrappedSolKeys> for [AccountMeta; 10] {
    fn from(keys: &StakeWrappedSolKeys) -> Self {
        [
            AccountMeta::new_readonly(keys.user, true),
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
impl<'a> From<&StakeWrappedSolAccounts<'_, 'a, 'a, 'a, 'a, 'a, 'a, 'a, 'a, 'a, 'a>>
    for [AccountInfo<'a>; 10]
{
    fn from(
        accounts: &StakeWrappedSolAccounts<'_, 'a, 'a, 'a, 'a, 'a, 'a, 'a, 'a, 'a, 'a>,
    ) -> Self {
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
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug)]
pub struct StakeWrappedSolIxArgs {
    pub stake_wrapped_sol_args: StakeWrappedSolArgs,
}
#[derive(Copy, Clone, Debug)]
pub struct StakeWrappedSolIxData<'me>(pub &'me StakeWrappedSolIxArgs);
pub const STAKE_WRAPPED_SOL_IX_DISCM: u8 = 0u8;
impl<'me> From<&'me StakeWrappedSolIxArgs> for StakeWrappedSolIxData<'me> {
    fn from(args: &'me StakeWrappedSolIxArgs) -> Self {
        Self(args)
    }
}
impl BorshSerialize for StakeWrappedSolIxData<'_> {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        writer.write_all(&[STAKE_WRAPPED_SOL_IX_DISCM])?;
        self.0.serialize(writer)
    }
}
pub fn stake_wrapped_sol_ix<K: Into<StakeWrappedSolKeys>, A: Into<StakeWrappedSolIxArgs>>(
    accounts: K,
    args: A,
) -> std::io::Result<Instruction> {
    let keys: StakeWrappedSolKeys = accounts.into();
    let metas: [AccountMeta; 10] = (&keys).into();
    let args_full: StakeWrappedSolIxArgs = args.into();
    let data: StakeWrappedSolIxData = (&args_full).into();
    Ok(Instruction {
        program_id: crate::ID,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn stake_wrapped_sol_invoke<'a, A: Into<StakeWrappedSolIxArgs>>(
    accounts: &StakeWrappedSolAccounts<'_, 'a, 'a, 'a, 'a, 'a, 'a, 'a, 'a, 'a, 'a>,
    args: A,
) -> ProgramResult {
    let ix = stake_wrapped_sol_ix(accounts, args)?;
    let account_info: [AccountInfo<'a>; 10] = accounts.into();
    invoke(&ix, &account_info)
}
pub fn stake_wrapped_sol_invoke_signed<'a, A: Into<StakeWrappedSolIxArgs>>(
    accounts: &StakeWrappedSolAccounts<'_, 'a, 'a, 'a, 'a, 'a, 'a, 'a, 'a, 'a, 'a>,
    args: A,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let ix = stake_wrapped_sol_ix(accounts, args)?;
    let account_info: [AccountInfo<'a>; 10] = accounts.into();
    invoke_signed(&ix, &account_info, seeds)
}
#[derive(Copy, Clone, Debug)]
pub struct SwapViaStakeAccounts<
    'me,
    'a0: 'me,
    'a1: 'me,
    'a2: 'me,
    'a3: 'me,
    'a4: 'me,
    'a5: 'me,
    'a6: 'me,
    'a7: 'me,
> {
    ///The payer for any additional rent required e.g. for the bridge stake account
    pub payer: &'me AccountInfo<'a0>,
    ///The authority of src_token_from
    pub user: &'me AccountInfo<'a1>,
    ///The token account to swap src tokens from
    pub src_token_from: &'me AccountInfo<'a2>,
    ///The token account to receive dest tokens to
    pub dest_token_to: &'me AccountInfo<'a3>,
    ///The bridge stake account thats withdrawn then deposited. PDA. seeds = ['bridge_stake', user.pubkey, SwapArgs.bridge_stake_seed]. Might be long-lived, make sure the seed is not already in use
    pub bridge_stake: &'me AccountInfo<'a4>,
    ///The dest_token_mint token account collecting fees. PDA. Seeds = ['fee', dest_token_mint.pubkey]
    pub dest_token_fee_token_account: &'me AccountInfo<'a5>,
    pub src_token_mint: &'me AccountInfo<'a6>,
    pub dest_token_mint: &'me AccountInfo<'a7>,
}
#[derive(Copy, Clone, Debug)]
pub struct SwapViaStakeKeys {
    ///The payer for any additional rent required e.g. for the bridge stake account
    pub payer: Pubkey,
    ///The authority of src_token_from
    pub user: Pubkey,
    ///The token account to swap src tokens from
    pub src_token_from: Pubkey,
    ///The token account to receive dest tokens to
    pub dest_token_to: Pubkey,
    ///The bridge stake account thats withdrawn then deposited. PDA. seeds = ['bridge_stake', user.pubkey, SwapArgs.bridge_stake_seed]. Might be long-lived, make sure the seed is not already in use
    pub bridge_stake: Pubkey,
    ///The dest_token_mint token account collecting fees. PDA. Seeds = ['fee', dest_token_mint.pubkey]
    pub dest_token_fee_token_account: Pubkey,
    pub src_token_mint: Pubkey,
    pub dest_token_mint: Pubkey,
}
impl<'me> From<&SwapViaStakeAccounts<'me, '_, '_, '_, '_, '_, '_, '_, '_>> for SwapViaStakeKeys {
    fn from(accounts: &SwapViaStakeAccounts<'me, '_, '_, '_, '_, '_, '_, '_, '_>) -> Self {
        Self {
            payer: *accounts.payer.key,
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
impl From<&SwapViaStakeKeys> for [AccountMeta; 8] {
    fn from(keys: &SwapViaStakeKeys) -> Self {
        [
            AccountMeta::new(keys.payer, true),
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
impl<'a> From<&SwapViaStakeAccounts<'_, 'a, 'a, 'a, 'a, 'a, 'a, 'a, 'a>> for [AccountInfo<'a>; 8] {
    fn from(accounts: &SwapViaStakeAccounts<'_, 'a, 'a, 'a, 'a, 'a, 'a, 'a, 'a>) -> Self {
        [
            accounts.payer.clone(),
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
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug)]
pub struct SwapViaStakeIxArgs {
    pub swap_via_stake_args: SwapViaStakeArgs,
}
#[derive(Copy, Clone, Debug)]
pub struct SwapViaStakeIxData<'me>(pub &'me SwapViaStakeIxArgs);
pub const SWAP_VIA_STAKE_IX_DISCM: u8 = 1u8;
impl<'me> From<&'me SwapViaStakeIxArgs> for SwapViaStakeIxData<'me> {
    fn from(args: &'me SwapViaStakeIxArgs) -> Self {
        Self(args)
    }
}
impl BorshSerialize for SwapViaStakeIxData<'_> {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        writer.write_all(&[SWAP_VIA_STAKE_IX_DISCM])?;
        self.0.serialize(writer)
    }
}
pub fn swap_via_stake_ix<K: Into<SwapViaStakeKeys>, A: Into<SwapViaStakeIxArgs>>(
    accounts: K,
    args: A,
) -> std::io::Result<Instruction> {
    let keys: SwapViaStakeKeys = accounts.into();
    let metas: [AccountMeta; 8] = (&keys).into();
    let args_full: SwapViaStakeIxArgs = args.into();
    let data: SwapViaStakeIxData = (&args_full).into();
    Ok(Instruction {
        program_id: crate::ID,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn swap_via_stake_invoke<'a, A: Into<SwapViaStakeIxArgs>>(
    accounts: &SwapViaStakeAccounts<'_, 'a, 'a, 'a, 'a, 'a, 'a, 'a, 'a>,
    args: A,
) -> ProgramResult {
    let ix = swap_via_stake_ix(accounts, args)?;
    let account_info: [AccountInfo<'a>; 8] = accounts.into();
    invoke(&ix, &account_info)
}
pub fn swap_via_stake_invoke_signed<'a, A: Into<SwapViaStakeIxArgs>>(
    accounts: &SwapViaStakeAccounts<'_, 'a, 'a, 'a, 'a, 'a, 'a, 'a, 'a>,
    args: A,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let ix = swap_via_stake_ix(accounts, args)?;
    let account_info: [AccountInfo<'a>; 8] = accounts.into();
    invoke_signed(&ix, &account_info, seeds)
}
#[derive(Copy, Clone, Debug)]
pub struct CreateFeeTokenAccountAccounts<'me, 'a0: 'me, 'a1: 'me, 'a2: 'me, 'a3: 'me, 'a4: 'me> {
    ///The person paying for the new fee token account. Can be anyone.
    pub payer: &'me AccountInfo<'a0>,
    ///The self-owned fee token account to be created. Seeds = ['fee', mint_pubkey]
    pub fee_token_account: &'me AccountInfo<'a1>,
    pub mint: &'me AccountInfo<'a2>,
    pub token_program: &'me AccountInfo<'a3>,
    pub system_program: &'me AccountInfo<'a4>,
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
impl<'me> From<&CreateFeeTokenAccountAccounts<'me, '_, '_, '_, '_, '_>>
    for CreateFeeTokenAccountKeys
{
    fn from(accounts: &CreateFeeTokenAccountAccounts<'me, '_, '_, '_, '_, '_>) -> Self {
        Self {
            payer: *accounts.payer.key,
            fee_token_account: *accounts.fee_token_account.key,
            mint: *accounts.mint.key,
            token_program: *accounts.token_program.key,
            system_program: *accounts.system_program.key,
        }
    }
}
impl From<&CreateFeeTokenAccountKeys> for [AccountMeta; 5] {
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
impl<'a> From<&CreateFeeTokenAccountAccounts<'_, 'a, 'a, 'a, 'a, 'a>> for [AccountInfo<'a>; 5] {
    fn from(accounts: &CreateFeeTokenAccountAccounts<'_, 'a, 'a, 'a, 'a, 'a>) -> Self {
        [
            accounts.payer.clone(),
            accounts.fee_token_account.clone(),
            accounts.mint.clone(),
            accounts.token_program.clone(),
            accounts.system_program.clone(),
        ]
    }
}
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug)]
pub struct CreateFeeTokenAccountIxArgs {}
#[derive(Copy, Clone, Debug)]
pub struct CreateFeeTokenAccountIxData<'me>(pub &'me CreateFeeTokenAccountIxArgs);
pub const CREATE_FEE_TOKEN_ACCOUNT_IX_DISCM: u8 = 2u8;
impl<'me> From<&'me CreateFeeTokenAccountIxArgs> for CreateFeeTokenAccountIxData<'me> {
    fn from(args: &'me CreateFeeTokenAccountIxArgs) -> Self {
        Self(args)
    }
}
impl BorshSerialize for CreateFeeTokenAccountIxData<'_> {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        writer.write_all(&[CREATE_FEE_TOKEN_ACCOUNT_IX_DISCM])?;
        self.0.serialize(writer)
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
    let metas: [AccountMeta; 5] = (&keys).into();
    let args_full: CreateFeeTokenAccountIxArgs = args.into();
    let data: CreateFeeTokenAccountIxData = (&args_full).into();
    Ok(Instruction {
        program_id: crate::ID,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn create_fee_token_account_invoke<'a, A: Into<CreateFeeTokenAccountIxArgs>>(
    accounts: &CreateFeeTokenAccountAccounts<'_, 'a, 'a, 'a, 'a, 'a>,
    args: A,
) -> ProgramResult {
    let ix = create_fee_token_account_ix(accounts, args)?;
    let account_info: [AccountInfo<'a>; 5] = accounts.into();
    invoke(&ix, &account_info)
}
pub fn create_fee_token_account_invoke_signed<'a, A: Into<CreateFeeTokenAccountIxArgs>>(
    accounts: &CreateFeeTokenAccountAccounts<'_, 'a, 'a, 'a, 'a, 'a>,
    args: A,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let ix = create_fee_token_account_ix(accounts, args)?;
    let account_info: [AccountInfo<'a>; 5] = accounts.into();
    invoke_signed(&ix, &account_info, seeds)
}
#[derive(Copy, Clone, Debug)]
pub struct CloseFeeTokenAccountAccounts<'me, 'a0: 'me, 'a1: 'me, 'a2: 'me, 'a3: 'me, 'a4: 'me> {
    ///The authorized program admin
    pub admin: &'me AccountInfo<'a0>,
    ///The self-owned fee token account to close. Must be empty or wrapped SOL. Seeds = ['fee', mint_pubkey]
    pub fee_token_account: &'me AccountInfo<'a1>,
    ///Refund fee_token_account's rent lamports to here
    pub close_to: &'me AccountInfo<'a2>,
    pub mint: &'me AccountInfo<'a3>,
    pub token_program: &'me AccountInfo<'a4>,
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
impl<'me> From<&CloseFeeTokenAccountAccounts<'me, '_, '_, '_, '_, '_>>
    for CloseFeeTokenAccountKeys
{
    fn from(accounts: &CloseFeeTokenAccountAccounts<'me, '_, '_, '_, '_, '_>) -> Self {
        Self {
            admin: *accounts.admin.key,
            fee_token_account: *accounts.fee_token_account.key,
            close_to: *accounts.close_to.key,
            mint: *accounts.mint.key,
            token_program: *accounts.token_program.key,
        }
    }
}
impl From<&CloseFeeTokenAccountKeys> for [AccountMeta; 5] {
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
impl<'a> From<&CloseFeeTokenAccountAccounts<'_, 'a, 'a, 'a, 'a, 'a>> for [AccountInfo<'a>; 5] {
    fn from(accounts: &CloseFeeTokenAccountAccounts<'_, 'a, 'a, 'a, 'a, 'a>) -> Self {
        [
            accounts.admin.clone(),
            accounts.fee_token_account.clone(),
            accounts.close_to.clone(),
            accounts.mint.clone(),
            accounts.token_program.clone(),
        ]
    }
}
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug)]
pub struct CloseFeeTokenAccountIxArgs {}
#[derive(Copy, Clone, Debug)]
pub struct CloseFeeTokenAccountIxData<'me>(pub &'me CloseFeeTokenAccountIxArgs);
pub const CLOSE_FEE_TOKEN_ACCOUNT_IX_DISCM: u8 = 3u8;
impl<'me> From<&'me CloseFeeTokenAccountIxArgs> for CloseFeeTokenAccountIxData<'me> {
    fn from(args: &'me CloseFeeTokenAccountIxArgs) -> Self {
        Self(args)
    }
}
impl BorshSerialize for CloseFeeTokenAccountIxData<'_> {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        writer.write_all(&[CLOSE_FEE_TOKEN_ACCOUNT_IX_DISCM])?;
        self.0.serialize(writer)
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
    let metas: [AccountMeta; 5] = (&keys).into();
    let args_full: CloseFeeTokenAccountIxArgs = args.into();
    let data: CloseFeeTokenAccountIxData = (&args_full).into();
    Ok(Instruction {
        program_id: crate::ID,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn close_fee_token_account_invoke<'a, A: Into<CloseFeeTokenAccountIxArgs>>(
    accounts: &CloseFeeTokenAccountAccounts<'_, 'a, 'a, 'a, 'a, 'a>,
    args: A,
) -> ProgramResult {
    let ix = close_fee_token_account_ix(accounts, args)?;
    let account_info: [AccountInfo<'a>; 5] = accounts.into();
    invoke(&ix, &account_info)
}
pub fn close_fee_token_account_invoke_signed<'a, A: Into<CloseFeeTokenAccountIxArgs>>(
    accounts: &CloseFeeTokenAccountAccounts<'_, 'a, 'a, 'a, 'a, 'a>,
    args: A,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let ix = close_fee_token_account_ix(accounts, args)?;
    let account_info: [AccountInfo<'a>; 5] = accounts.into();
    invoke_signed(&ix, &account_info, seeds)
}
#[derive(Copy, Clone, Debug)]
pub struct WithdrawFeesAccounts<'me, 'a0: 'me, 'a1: 'me, 'a2: 'me, 'a3: 'me, 'a4: 'me> {
    ///The authorized program admin
    pub admin: &'me AccountInfo<'a0>,
    ///The self-owned fee token account to withdraw fees from. Seeds = ['fee', mint_pubkey]
    pub fee_token_account: &'me AccountInfo<'a1>,
    ///Withdraw accumulated fees to here
    pub withdraw_to: &'me AccountInfo<'a2>,
    pub mint: &'me AccountInfo<'a3>,
    pub token_program: &'me AccountInfo<'a4>,
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
impl<'me> From<&WithdrawFeesAccounts<'me, '_, '_, '_, '_, '_>> for WithdrawFeesKeys {
    fn from(accounts: &WithdrawFeesAccounts<'me, '_, '_, '_, '_, '_>) -> Self {
        Self {
            admin: *accounts.admin.key,
            fee_token_account: *accounts.fee_token_account.key,
            withdraw_to: *accounts.withdraw_to.key,
            mint: *accounts.mint.key,
            token_program: *accounts.token_program.key,
        }
    }
}
impl From<&WithdrawFeesKeys> for [AccountMeta; 5] {
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
impl<'a> From<&WithdrawFeesAccounts<'_, 'a, 'a, 'a, 'a, 'a>> for [AccountInfo<'a>; 5] {
    fn from(accounts: &WithdrawFeesAccounts<'_, 'a, 'a, 'a, 'a, 'a>) -> Self {
        [
            accounts.admin.clone(),
            accounts.fee_token_account.clone(),
            accounts.withdraw_to.clone(),
            accounts.mint.clone(),
            accounts.token_program.clone(),
        ]
    }
}
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug)]
pub struct WithdrawFeesIxArgs {}
#[derive(Copy, Clone, Debug)]
pub struct WithdrawFeesIxData<'me>(pub &'me WithdrawFeesIxArgs);
pub const WITHDRAW_FEES_IX_DISCM: u8 = 4u8;
impl<'me> From<&'me WithdrawFeesIxArgs> for WithdrawFeesIxData<'me> {
    fn from(args: &'me WithdrawFeesIxArgs) -> Self {
        Self(args)
    }
}
impl BorshSerialize for WithdrawFeesIxData<'_> {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        writer.write_all(&[WITHDRAW_FEES_IX_DISCM])?;
        self.0.serialize(writer)
    }
}
pub fn withdraw_fees_ix<K: Into<WithdrawFeesKeys>, A: Into<WithdrawFeesIxArgs>>(
    accounts: K,
    args: A,
) -> std::io::Result<Instruction> {
    let keys: WithdrawFeesKeys = accounts.into();
    let metas: [AccountMeta; 5] = (&keys).into();
    let args_full: WithdrawFeesIxArgs = args.into();
    let data: WithdrawFeesIxData = (&args_full).into();
    Ok(Instruction {
        program_id: crate::ID,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn withdraw_fees_invoke<'a, A: Into<WithdrawFeesIxArgs>>(
    accounts: &WithdrawFeesAccounts<'_, 'a, 'a, 'a, 'a, 'a>,
    args: A,
) -> ProgramResult {
    let ix = withdraw_fees_ix(accounts, args)?;
    let account_info: [AccountInfo<'a>; 5] = accounts.into();
    invoke(&ix, &account_info)
}
pub fn withdraw_fees_invoke_signed<'a, A: Into<WithdrawFeesIxArgs>>(
    accounts: &WithdrawFeesAccounts<'_, 'a, 'a, 'a, 'a, 'a>,
    args: A,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let ix = withdraw_fees_ix(accounts, args)?;
    let account_info: [AccountInfo<'a>; 5] = accounts.into();
    invoke_signed(&ix, &account_info, seeds)
}
