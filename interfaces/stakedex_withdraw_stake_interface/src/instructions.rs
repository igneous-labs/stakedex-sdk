use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::AccountInfo,
    entrypoint::ProgramResult,
    instruction::{AccountMeta, Instruction},
    program::{invoke, invoke_signed},
    pubkey::Pubkey,
};
pub const EVERSOL_STAKE_POOL_WITHDRAW_STAKE_IX_ACCOUNTS_LEN: usize = 10usize;
#[derive(Copy, Clone, Debug)]
pub struct EversolStakePoolWithdrawStakeAccounts<
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
    pub eversol_stake_pool_program: &'me AccountInfo<'a0>,
    pub withdraw_stake_spl_stake_pool: &'me AccountInfo<'a1>,
    pub withdraw_stake_validator_list: &'me AccountInfo<'a2>,
    pub withdraw_stake_withdraw_authority: &'me AccountInfo<'a3>,
    pub withdraw_stake_stake_to_split: &'me AccountInfo<'a4>,
    pub withdraw_stake_manager_fee: &'me AccountInfo<'a5>,
    pub clock: &'me AccountInfo<'a6>,
    pub token_program: &'me AccountInfo<'a7>,
    pub stake_program: &'me AccountInfo<'a8>,
    pub system_program: &'me AccountInfo<'a9>,
}
#[derive(Copy, Clone, Debug)]
pub struct EversolStakePoolWithdrawStakeKeys {
    pub eversol_stake_pool_program: Pubkey,
    pub withdraw_stake_spl_stake_pool: Pubkey,
    pub withdraw_stake_validator_list: Pubkey,
    pub withdraw_stake_withdraw_authority: Pubkey,
    pub withdraw_stake_stake_to_split: Pubkey,
    pub withdraw_stake_manager_fee: Pubkey,
    pub clock: Pubkey,
    pub token_program: Pubkey,
    pub stake_program: Pubkey,
    pub system_program: Pubkey,
}
impl<'me> From<&EversolStakePoolWithdrawStakeAccounts<'me, '_, '_, '_, '_, '_, '_, '_, '_, '_, '_>>
    for EversolStakePoolWithdrawStakeKeys
{
    fn from(
        accounts: &EversolStakePoolWithdrawStakeAccounts<
            'me,
            '_,
            '_,
            '_,
            '_,
            '_,
            '_,
            '_,
            '_,
            '_,
            '_,
        >,
    ) -> Self {
        Self {
            eversol_stake_pool_program: *accounts.eversol_stake_pool_program.key,
            withdraw_stake_spl_stake_pool: *accounts.withdraw_stake_spl_stake_pool.key,
            withdraw_stake_validator_list: *accounts.withdraw_stake_validator_list.key,
            withdraw_stake_withdraw_authority: *accounts.withdraw_stake_withdraw_authority.key,
            withdraw_stake_stake_to_split: *accounts.withdraw_stake_stake_to_split.key,
            withdraw_stake_manager_fee: *accounts.withdraw_stake_manager_fee.key,
            clock: *accounts.clock.key,
            token_program: *accounts.token_program.key,
            stake_program: *accounts.stake_program.key,
            system_program: *accounts.system_program.key,
        }
    }
}
impl From<&EversolStakePoolWithdrawStakeKeys>
    for [AccountMeta; EVERSOL_STAKE_POOL_WITHDRAW_STAKE_IX_ACCOUNTS_LEN]
{
    fn from(keys: &EversolStakePoolWithdrawStakeKeys) -> Self {
        [
            AccountMeta::new_readonly(keys.eversol_stake_pool_program, false),
            AccountMeta::new(keys.withdraw_stake_spl_stake_pool, false),
            AccountMeta::new(keys.withdraw_stake_validator_list, false),
            AccountMeta::new_readonly(keys.withdraw_stake_withdraw_authority, false),
            AccountMeta::new(keys.withdraw_stake_stake_to_split, false),
            AccountMeta::new(keys.withdraw_stake_manager_fee, false),
            AccountMeta::new_readonly(keys.clock, false),
            AccountMeta::new_readonly(keys.token_program, false),
            AccountMeta::new_readonly(keys.stake_program, false),
            AccountMeta::new_readonly(keys.system_program, false),
        ]
    }
}
impl<'a> From<&EversolStakePoolWithdrawStakeAccounts<'_, 'a, 'a, 'a, 'a, 'a, 'a, 'a, 'a, 'a, 'a>>
    for [AccountInfo<'a>; EVERSOL_STAKE_POOL_WITHDRAW_STAKE_IX_ACCOUNTS_LEN]
{
    fn from(
        accounts: &EversolStakePoolWithdrawStakeAccounts<
            '_,
            'a,
            'a,
            'a,
            'a,
            'a,
            'a,
            'a,
            'a,
            'a,
            'a,
        >,
    ) -> Self {
        [
            accounts.eversol_stake_pool_program.clone(),
            accounts.withdraw_stake_spl_stake_pool.clone(),
            accounts.withdraw_stake_validator_list.clone(),
            accounts.withdraw_stake_withdraw_authority.clone(),
            accounts.withdraw_stake_stake_to_split.clone(),
            accounts.withdraw_stake_manager_fee.clone(),
            accounts.clock.clone(),
            accounts.token_program.clone(),
            accounts.stake_program.clone(),
            accounts.system_program.clone(),
        ]
    }
}
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug)]
pub struct EversolStakePoolWithdrawStakeIxArgs {}
#[derive(Copy, Clone, Debug)]
pub struct EversolStakePoolWithdrawStakeIxData<'me>(pub &'me EversolStakePoolWithdrawStakeIxArgs);
pub const EVERSOL_STAKE_POOL_WITHDRAW_STAKE_IX_DISCM: u8 = 0u8;
impl<'me> From<&'me EversolStakePoolWithdrawStakeIxArgs>
    for EversolStakePoolWithdrawStakeIxData<'me>
{
    fn from(args: &'me EversolStakePoolWithdrawStakeIxArgs) -> Self {
        Self(args)
    }
}
impl BorshSerialize for EversolStakePoolWithdrawStakeIxData<'_> {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        writer.write_all(&[EVERSOL_STAKE_POOL_WITHDRAW_STAKE_IX_DISCM])?;
        self.0.serialize(writer)
    }
}
pub fn eversol_stake_pool_withdraw_stake_ix<
    K: Into<EversolStakePoolWithdrawStakeKeys>,
    A: Into<EversolStakePoolWithdrawStakeIxArgs>,
>(
    accounts: K,
    args: A,
) -> std::io::Result<Instruction> {
    let keys: EversolStakePoolWithdrawStakeKeys = accounts.into();
    let metas: [AccountMeta; EVERSOL_STAKE_POOL_WITHDRAW_STAKE_IX_ACCOUNTS_LEN] = (&keys).into();
    let args_full: EversolStakePoolWithdrawStakeIxArgs = args.into();
    let data: EversolStakePoolWithdrawStakeIxData = (&args_full).into();
    Ok(Instruction {
        program_id: crate::ID,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn eversol_stake_pool_withdraw_stake_invoke<
    'a,
    A: Into<EversolStakePoolWithdrawStakeIxArgs>,
>(
    accounts: &EversolStakePoolWithdrawStakeAccounts<'_, 'a, 'a, 'a, 'a, 'a, 'a, 'a, 'a, 'a, 'a>,
    args: A,
) -> ProgramResult {
    let ix = eversol_stake_pool_withdraw_stake_ix(accounts, args)?;
    let account_info: [AccountInfo<'a>; EVERSOL_STAKE_POOL_WITHDRAW_STAKE_IX_ACCOUNTS_LEN] =
        accounts.into();
    invoke(&ix, &account_info)
}
pub fn eversol_stake_pool_withdraw_stake_invoke_signed<
    'a,
    A: Into<EversolStakePoolWithdrawStakeIxArgs>,
>(
    accounts: &EversolStakePoolWithdrawStakeAccounts<'_, 'a, 'a, 'a, 'a, 'a, 'a, 'a, 'a, 'a, 'a>,
    args: A,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let ix = eversol_stake_pool_withdraw_stake_ix(accounts, args)?;
    let account_info: [AccountInfo<'a>; EVERSOL_STAKE_POOL_WITHDRAW_STAKE_IX_ACCOUNTS_LEN] =
        accounts.into();
    invoke_signed(&ix, &account_info, seeds)
}
pub const SOCEAN_STAKE_POOL_WITHDRAW_STAKE_IX_ACCOUNTS_LEN: usize = 10usize;
#[derive(Copy, Clone, Debug)]
pub struct SoceanStakePoolWithdrawStakeAccounts<
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
    pub socean_stake_pool_program: &'me AccountInfo<'a0>,
    pub withdraw_stake_spl_stake_pool: &'me AccountInfo<'a1>,
    pub withdraw_stake_validator_list: &'me AccountInfo<'a2>,
    pub withdraw_stake_withdraw_authority: &'me AccountInfo<'a3>,
    pub withdraw_stake_stake_to_split: &'me AccountInfo<'a4>,
    pub withdraw_stake_manager_fee: &'me AccountInfo<'a5>,
    pub clock: &'me AccountInfo<'a6>,
    pub token_program: &'me AccountInfo<'a7>,
    pub stake_program: &'me AccountInfo<'a8>,
    pub system_program: &'me AccountInfo<'a9>,
}
#[derive(Copy, Clone, Debug)]
pub struct SoceanStakePoolWithdrawStakeKeys {
    pub socean_stake_pool_program: Pubkey,
    pub withdraw_stake_spl_stake_pool: Pubkey,
    pub withdraw_stake_validator_list: Pubkey,
    pub withdraw_stake_withdraw_authority: Pubkey,
    pub withdraw_stake_stake_to_split: Pubkey,
    pub withdraw_stake_manager_fee: Pubkey,
    pub clock: Pubkey,
    pub token_program: Pubkey,
    pub stake_program: Pubkey,
    pub system_program: Pubkey,
}
impl<'me> From<&SoceanStakePoolWithdrawStakeAccounts<'me, '_, '_, '_, '_, '_, '_, '_, '_, '_, '_>>
    for SoceanStakePoolWithdrawStakeKeys
{
    fn from(
        accounts: &SoceanStakePoolWithdrawStakeAccounts<
            'me,
            '_,
            '_,
            '_,
            '_,
            '_,
            '_,
            '_,
            '_,
            '_,
            '_,
        >,
    ) -> Self {
        Self {
            socean_stake_pool_program: *accounts.socean_stake_pool_program.key,
            withdraw_stake_spl_stake_pool: *accounts.withdraw_stake_spl_stake_pool.key,
            withdraw_stake_validator_list: *accounts.withdraw_stake_validator_list.key,
            withdraw_stake_withdraw_authority: *accounts.withdraw_stake_withdraw_authority.key,
            withdraw_stake_stake_to_split: *accounts.withdraw_stake_stake_to_split.key,
            withdraw_stake_manager_fee: *accounts.withdraw_stake_manager_fee.key,
            clock: *accounts.clock.key,
            token_program: *accounts.token_program.key,
            stake_program: *accounts.stake_program.key,
            system_program: *accounts.system_program.key,
        }
    }
}
impl From<&SoceanStakePoolWithdrawStakeKeys>
    for [AccountMeta; SOCEAN_STAKE_POOL_WITHDRAW_STAKE_IX_ACCOUNTS_LEN]
{
    fn from(keys: &SoceanStakePoolWithdrawStakeKeys) -> Self {
        [
            AccountMeta::new_readonly(keys.socean_stake_pool_program, false),
            AccountMeta::new(keys.withdraw_stake_spl_stake_pool, false),
            AccountMeta::new(keys.withdraw_stake_validator_list, false),
            AccountMeta::new_readonly(keys.withdraw_stake_withdraw_authority, false),
            AccountMeta::new(keys.withdraw_stake_stake_to_split, false),
            AccountMeta::new(keys.withdraw_stake_manager_fee, false),
            AccountMeta::new_readonly(keys.clock, false),
            AccountMeta::new_readonly(keys.token_program, false),
            AccountMeta::new_readonly(keys.stake_program, false),
            AccountMeta::new_readonly(keys.system_program, false),
        ]
    }
}
impl<'a> From<&SoceanStakePoolWithdrawStakeAccounts<'_, 'a, 'a, 'a, 'a, 'a, 'a, 'a, 'a, 'a, 'a>>
    for [AccountInfo<'a>; SOCEAN_STAKE_POOL_WITHDRAW_STAKE_IX_ACCOUNTS_LEN]
{
    fn from(
        accounts: &SoceanStakePoolWithdrawStakeAccounts<'_, 'a, 'a, 'a, 'a, 'a, 'a, 'a, 'a, 'a, 'a>,
    ) -> Self {
        [
            accounts.socean_stake_pool_program.clone(),
            accounts.withdraw_stake_spl_stake_pool.clone(),
            accounts.withdraw_stake_validator_list.clone(),
            accounts.withdraw_stake_withdraw_authority.clone(),
            accounts.withdraw_stake_stake_to_split.clone(),
            accounts.withdraw_stake_manager_fee.clone(),
            accounts.clock.clone(),
            accounts.token_program.clone(),
            accounts.stake_program.clone(),
            accounts.system_program.clone(),
        ]
    }
}
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug)]
pub struct SoceanStakePoolWithdrawStakeIxArgs {}
#[derive(Copy, Clone, Debug)]
pub struct SoceanStakePoolWithdrawStakeIxData<'me>(pub &'me SoceanStakePoolWithdrawStakeIxArgs);
pub const SOCEAN_STAKE_POOL_WITHDRAW_STAKE_IX_DISCM: u8 = 1u8;
impl<'me> From<&'me SoceanStakePoolWithdrawStakeIxArgs>
    for SoceanStakePoolWithdrawStakeIxData<'me>
{
    fn from(args: &'me SoceanStakePoolWithdrawStakeIxArgs) -> Self {
        Self(args)
    }
}
impl BorshSerialize for SoceanStakePoolWithdrawStakeIxData<'_> {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        writer.write_all(&[SOCEAN_STAKE_POOL_WITHDRAW_STAKE_IX_DISCM])?;
        self.0.serialize(writer)
    }
}
pub fn socean_stake_pool_withdraw_stake_ix<
    K: Into<SoceanStakePoolWithdrawStakeKeys>,
    A: Into<SoceanStakePoolWithdrawStakeIxArgs>,
>(
    accounts: K,
    args: A,
) -> std::io::Result<Instruction> {
    let keys: SoceanStakePoolWithdrawStakeKeys = accounts.into();
    let metas: [AccountMeta; SOCEAN_STAKE_POOL_WITHDRAW_STAKE_IX_ACCOUNTS_LEN] = (&keys).into();
    let args_full: SoceanStakePoolWithdrawStakeIxArgs = args.into();
    let data: SoceanStakePoolWithdrawStakeIxData = (&args_full).into();
    Ok(Instruction {
        program_id: crate::ID,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn socean_stake_pool_withdraw_stake_invoke<'a, A: Into<SoceanStakePoolWithdrawStakeIxArgs>>(
    accounts: &SoceanStakePoolWithdrawStakeAccounts<'_, 'a, 'a, 'a, 'a, 'a, 'a, 'a, 'a, 'a, 'a>,
    args: A,
) -> ProgramResult {
    let ix = socean_stake_pool_withdraw_stake_ix(accounts, args)?;
    let account_info: [AccountInfo<'a>; SOCEAN_STAKE_POOL_WITHDRAW_STAKE_IX_ACCOUNTS_LEN] =
        accounts.into();
    invoke(&ix, &account_info)
}
pub fn socean_stake_pool_withdraw_stake_invoke_signed<
    'a,
    A: Into<SoceanStakePoolWithdrawStakeIxArgs>,
>(
    accounts: &SoceanStakePoolWithdrawStakeAccounts<'_, 'a, 'a, 'a, 'a, 'a, 'a, 'a, 'a, 'a, 'a>,
    args: A,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let ix = socean_stake_pool_withdraw_stake_ix(accounts, args)?;
    let account_info: [AccountInfo<'a>; SOCEAN_STAKE_POOL_WITHDRAW_STAKE_IX_ACCOUNTS_LEN] =
        accounts.into();
    invoke_signed(&ix, &account_info, seeds)
}
pub const SPL_STAKE_POOL_WITHDRAW_STAKE_IX_ACCOUNTS_LEN: usize = 10usize;
#[derive(Copy, Clone, Debug)]
pub struct SplStakePoolWithdrawStakeAccounts<
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
    pub spl_stake_pool_program: &'me AccountInfo<'a0>,
    pub withdraw_stake_spl_stake_pool: &'me AccountInfo<'a1>,
    pub withdraw_stake_validator_list: &'me AccountInfo<'a2>,
    pub withdraw_stake_withdraw_authority: &'me AccountInfo<'a3>,
    pub withdraw_stake_stake_to_split: &'me AccountInfo<'a4>,
    pub withdraw_stake_manager_fee: &'me AccountInfo<'a5>,
    pub clock: &'me AccountInfo<'a6>,
    pub token_program: &'me AccountInfo<'a7>,
    pub stake_program: &'me AccountInfo<'a8>,
    pub system_program: &'me AccountInfo<'a9>,
}
#[derive(Copy, Clone, Debug)]
pub struct SplStakePoolWithdrawStakeKeys {
    pub spl_stake_pool_program: Pubkey,
    pub withdraw_stake_spl_stake_pool: Pubkey,
    pub withdraw_stake_validator_list: Pubkey,
    pub withdraw_stake_withdraw_authority: Pubkey,
    pub withdraw_stake_stake_to_split: Pubkey,
    pub withdraw_stake_manager_fee: Pubkey,
    pub clock: Pubkey,
    pub token_program: Pubkey,
    pub stake_program: Pubkey,
    pub system_program: Pubkey,
}
impl<'me> From<&SplStakePoolWithdrawStakeAccounts<'me, '_, '_, '_, '_, '_, '_, '_, '_, '_, '_>>
    for SplStakePoolWithdrawStakeKeys
{
    fn from(
        accounts: &SplStakePoolWithdrawStakeAccounts<'me, '_, '_, '_, '_, '_, '_, '_, '_, '_, '_>,
    ) -> Self {
        Self {
            spl_stake_pool_program: *accounts.spl_stake_pool_program.key,
            withdraw_stake_spl_stake_pool: *accounts.withdraw_stake_spl_stake_pool.key,
            withdraw_stake_validator_list: *accounts.withdraw_stake_validator_list.key,
            withdraw_stake_withdraw_authority: *accounts.withdraw_stake_withdraw_authority.key,
            withdraw_stake_stake_to_split: *accounts.withdraw_stake_stake_to_split.key,
            withdraw_stake_manager_fee: *accounts.withdraw_stake_manager_fee.key,
            clock: *accounts.clock.key,
            token_program: *accounts.token_program.key,
            stake_program: *accounts.stake_program.key,
            system_program: *accounts.system_program.key,
        }
    }
}
impl From<&SplStakePoolWithdrawStakeKeys>
    for [AccountMeta; SPL_STAKE_POOL_WITHDRAW_STAKE_IX_ACCOUNTS_LEN]
{
    fn from(keys: &SplStakePoolWithdrawStakeKeys) -> Self {
        [
            AccountMeta::new_readonly(keys.spl_stake_pool_program, false),
            AccountMeta::new(keys.withdraw_stake_spl_stake_pool, false),
            AccountMeta::new(keys.withdraw_stake_validator_list, false),
            AccountMeta::new_readonly(keys.withdraw_stake_withdraw_authority, false),
            AccountMeta::new(keys.withdraw_stake_stake_to_split, false),
            AccountMeta::new(keys.withdraw_stake_manager_fee, false),
            AccountMeta::new_readonly(keys.clock, false),
            AccountMeta::new_readonly(keys.token_program, false),
            AccountMeta::new_readonly(keys.stake_program, false),
            AccountMeta::new_readonly(keys.system_program, false),
        ]
    }
}
impl<'a> From<&SplStakePoolWithdrawStakeAccounts<'_, 'a, 'a, 'a, 'a, 'a, 'a, 'a, 'a, 'a, 'a>>
    for [AccountInfo<'a>; SPL_STAKE_POOL_WITHDRAW_STAKE_IX_ACCOUNTS_LEN]
{
    fn from(
        accounts: &SplStakePoolWithdrawStakeAccounts<'_, 'a, 'a, 'a, 'a, 'a, 'a, 'a, 'a, 'a, 'a>,
    ) -> Self {
        [
            accounts.spl_stake_pool_program.clone(),
            accounts.withdraw_stake_spl_stake_pool.clone(),
            accounts.withdraw_stake_validator_list.clone(),
            accounts.withdraw_stake_withdraw_authority.clone(),
            accounts.withdraw_stake_stake_to_split.clone(),
            accounts.withdraw_stake_manager_fee.clone(),
            accounts.clock.clone(),
            accounts.token_program.clone(),
            accounts.stake_program.clone(),
            accounts.system_program.clone(),
        ]
    }
}
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug)]
pub struct SplStakePoolWithdrawStakeIxArgs {}
#[derive(Copy, Clone, Debug)]
pub struct SplStakePoolWithdrawStakeIxData<'me>(pub &'me SplStakePoolWithdrawStakeIxArgs);
pub const SPL_STAKE_POOL_WITHDRAW_STAKE_IX_DISCM: u8 = 2u8;
impl<'me> From<&'me SplStakePoolWithdrawStakeIxArgs> for SplStakePoolWithdrawStakeIxData<'me> {
    fn from(args: &'me SplStakePoolWithdrawStakeIxArgs) -> Self {
        Self(args)
    }
}
impl BorshSerialize for SplStakePoolWithdrawStakeIxData<'_> {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        writer.write_all(&[SPL_STAKE_POOL_WITHDRAW_STAKE_IX_DISCM])?;
        self.0.serialize(writer)
    }
}
pub fn spl_stake_pool_withdraw_stake_ix<
    K: Into<SplStakePoolWithdrawStakeKeys>,
    A: Into<SplStakePoolWithdrawStakeIxArgs>,
>(
    accounts: K,
    args: A,
) -> std::io::Result<Instruction> {
    let keys: SplStakePoolWithdrawStakeKeys = accounts.into();
    let metas: [AccountMeta; SPL_STAKE_POOL_WITHDRAW_STAKE_IX_ACCOUNTS_LEN] = (&keys).into();
    let args_full: SplStakePoolWithdrawStakeIxArgs = args.into();
    let data: SplStakePoolWithdrawStakeIxData = (&args_full).into();
    Ok(Instruction {
        program_id: crate::ID,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn spl_stake_pool_withdraw_stake_invoke<'a, A: Into<SplStakePoolWithdrawStakeIxArgs>>(
    accounts: &SplStakePoolWithdrawStakeAccounts<'_, 'a, 'a, 'a, 'a, 'a, 'a, 'a, 'a, 'a, 'a>,
    args: A,
) -> ProgramResult {
    let ix = spl_stake_pool_withdraw_stake_ix(accounts, args)?;
    let account_info: [AccountInfo<'a>; SPL_STAKE_POOL_WITHDRAW_STAKE_IX_ACCOUNTS_LEN] =
        accounts.into();
    invoke(&ix, &account_info)
}
pub fn spl_stake_pool_withdraw_stake_invoke_signed<'a, A: Into<SplStakePoolWithdrawStakeIxArgs>>(
    accounts: &SplStakePoolWithdrawStakeAccounts<'_, 'a, 'a, 'a, 'a, 'a, 'a, 'a, 'a, 'a, 'a>,
    args: A,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let ix = spl_stake_pool_withdraw_stake_ix(accounts, args)?;
    let account_info: [AccountInfo<'a>; SPL_STAKE_POOL_WITHDRAW_STAKE_IX_ACCOUNTS_LEN] =
        accounts.into();
    invoke_signed(&ix, &account_info, seeds)
}
pub const LIDO_WITHDRAW_STAKE_IX_ACCOUNTS_LEN: usize = 10usize;
#[derive(Copy, Clone, Debug)]
pub struct LidoWithdrawStakeAccounts<
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
    pub lido_program: &'me AccountInfo<'a0>,
    pub withdraw_stake_solido: &'me AccountInfo<'a1>,
    pub withdraw_stake_voter: &'me AccountInfo<'a2>,
    pub withdraw_stake_stake_to_split: &'me AccountInfo<'a3>,
    pub withdraw_stake_stake_authority: &'me AccountInfo<'a4>,
    pub withdraw_stake_validator_list: &'me AccountInfo<'a5>,
    pub clock: &'me AccountInfo<'a6>,
    pub token_program: &'me AccountInfo<'a7>,
    pub stake_program: &'me AccountInfo<'a8>,
    pub system_program: &'me AccountInfo<'a9>,
}
#[derive(Copy, Clone, Debug)]
pub struct LidoWithdrawStakeKeys {
    pub lido_program: Pubkey,
    pub withdraw_stake_solido: Pubkey,
    pub withdraw_stake_voter: Pubkey,
    pub withdraw_stake_stake_to_split: Pubkey,
    pub withdraw_stake_stake_authority: Pubkey,
    pub withdraw_stake_validator_list: Pubkey,
    pub clock: Pubkey,
    pub token_program: Pubkey,
    pub stake_program: Pubkey,
    pub system_program: Pubkey,
}
impl<'me> From<&LidoWithdrawStakeAccounts<'me, '_, '_, '_, '_, '_, '_, '_, '_, '_, '_>>
    for LidoWithdrawStakeKeys
{
    fn from(
        accounts: &LidoWithdrawStakeAccounts<'me, '_, '_, '_, '_, '_, '_, '_, '_, '_, '_>,
    ) -> Self {
        Self {
            lido_program: *accounts.lido_program.key,
            withdraw_stake_solido: *accounts.withdraw_stake_solido.key,
            withdraw_stake_voter: *accounts.withdraw_stake_voter.key,
            withdraw_stake_stake_to_split: *accounts.withdraw_stake_stake_to_split.key,
            withdraw_stake_stake_authority: *accounts.withdraw_stake_stake_authority.key,
            withdraw_stake_validator_list: *accounts.withdraw_stake_validator_list.key,
            clock: *accounts.clock.key,
            token_program: *accounts.token_program.key,
            stake_program: *accounts.stake_program.key,
            system_program: *accounts.system_program.key,
        }
    }
}
impl From<&LidoWithdrawStakeKeys> for [AccountMeta; LIDO_WITHDRAW_STAKE_IX_ACCOUNTS_LEN] {
    fn from(keys: &LidoWithdrawStakeKeys) -> Self {
        [
            AccountMeta::new_readonly(keys.lido_program, false),
            AccountMeta::new(keys.withdraw_stake_solido, false),
            AccountMeta::new_readonly(keys.withdraw_stake_voter, false),
            AccountMeta::new(keys.withdraw_stake_stake_to_split, false),
            AccountMeta::new_readonly(keys.withdraw_stake_stake_authority, false),
            AccountMeta::new(keys.withdraw_stake_validator_list, false),
            AccountMeta::new_readonly(keys.clock, false),
            AccountMeta::new_readonly(keys.token_program, false),
            AccountMeta::new_readonly(keys.stake_program, false),
            AccountMeta::new_readonly(keys.system_program, false),
        ]
    }
}
impl<'a> From<&LidoWithdrawStakeAccounts<'_, 'a, 'a, 'a, 'a, 'a, 'a, 'a, 'a, 'a, 'a>>
    for [AccountInfo<'a>; LIDO_WITHDRAW_STAKE_IX_ACCOUNTS_LEN]
{
    fn from(
        accounts: &LidoWithdrawStakeAccounts<'_, 'a, 'a, 'a, 'a, 'a, 'a, 'a, 'a, 'a, 'a>,
    ) -> Self {
        [
            accounts.lido_program.clone(),
            accounts.withdraw_stake_solido.clone(),
            accounts.withdraw_stake_voter.clone(),
            accounts.withdraw_stake_stake_to_split.clone(),
            accounts.withdraw_stake_stake_authority.clone(),
            accounts.withdraw_stake_validator_list.clone(),
            accounts.clock.clone(),
            accounts.token_program.clone(),
            accounts.stake_program.clone(),
            accounts.system_program.clone(),
        ]
    }
}
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug)]
pub struct LidoWithdrawStakeIxArgs {}
#[derive(Copy, Clone, Debug)]
pub struct LidoWithdrawStakeIxData<'me>(pub &'me LidoWithdrawStakeIxArgs);
pub const LIDO_WITHDRAW_STAKE_IX_DISCM: u8 = 3u8;
impl<'me> From<&'me LidoWithdrawStakeIxArgs> for LidoWithdrawStakeIxData<'me> {
    fn from(args: &'me LidoWithdrawStakeIxArgs) -> Self {
        Self(args)
    }
}
impl BorshSerialize for LidoWithdrawStakeIxData<'_> {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        writer.write_all(&[LIDO_WITHDRAW_STAKE_IX_DISCM])?;
        self.0.serialize(writer)
    }
}
pub fn lido_withdraw_stake_ix<K: Into<LidoWithdrawStakeKeys>, A: Into<LidoWithdrawStakeIxArgs>>(
    accounts: K,
    args: A,
) -> std::io::Result<Instruction> {
    let keys: LidoWithdrawStakeKeys = accounts.into();
    let metas: [AccountMeta; LIDO_WITHDRAW_STAKE_IX_ACCOUNTS_LEN] = (&keys).into();
    let args_full: LidoWithdrawStakeIxArgs = args.into();
    let data: LidoWithdrawStakeIxData = (&args_full).into();
    Ok(Instruction {
        program_id: crate::ID,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn lido_withdraw_stake_invoke<'a, A: Into<LidoWithdrawStakeIxArgs>>(
    accounts: &LidoWithdrawStakeAccounts<'_, 'a, 'a, 'a, 'a, 'a, 'a, 'a, 'a, 'a, 'a>,
    args: A,
) -> ProgramResult {
    let ix = lido_withdraw_stake_ix(accounts, args)?;
    let account_info: [AccountInfo<'a>; LIDO_WITHDRAW_STAKE_IX_ACCOUNTS_LEN] = accounts.into();
    invoke(&ix, &account_info)
}
pub fn lido_withdraw_stake_invoke_signed<'a, A: Into<LidoWithdrawStakeIxArgs>>(
    accounts: &LidoWithdrawStakeAccounts<'_, 'a, 'a, 'a, 'a, 'a, 'a, 'a, 'a, 'a, 'a>,
    args: A,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let ix = lido_withdraw_stake_ix(accounts, args)?;
    let account_info: [AccountInfo<'a>; LIDO_WITHDRAW_STAKE_IX_ACCOUNTS_LEN] = accounts.into();
    invoke_signed(&ix, &account_info, seeds)
}
