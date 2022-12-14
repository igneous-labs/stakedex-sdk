use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::AccountInfo,
    entrypoint::ProgramResult,
    instruction::{AccountMeta, Instruction},
    program::{invoke, invoke_signed},
    pubkey::Pubkey,
};
pub const EVERSOL_STAKE_POOL_DEPOSIT_STAKE_IX_ACCOUNTS_LEN: usize = 12usize;
#[derive(Copy, Clone, Debug)]
pub struct EversolStakePoolDepositStakeAccounts<
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
    'a10: 'me,
    'a11: 'me,
> {
    pub eversol_stake_pool_program: &'me AccountInfo<'a0>,
    pub deposit_stake_spl_stake_pool: &'me AccountInfo<'a1>,
    pub deposit_stake_validator_list: &'me AccountInfo<'a2>,
    pub deposit_stake_deposit_authority: &'me AccountInfo<'a3>,
    pub deposit_stake_withdraw_authority: &'me AccountInfo<'a4>,
    pub deposit_stake_validator_stake: &'me AccountInfo<'a5>,
    pub deposit_stake_reserve_stake: &'me AccountInfo<'a6>,
    pub deposit_stake_manager_fee: &'me AccountInfo<'a7>,
    pub clock: &'me AccountInfo<'a8>,
    pub stake_history: &'me AccountInfo<'a9>,
    pub token_program: &'me AccountInfo<'a10>,
    pub stake_program: &'me AccountInfo<'a11>,
}
#[derive(Copy, Clone, Debug)]
pub struct EversolStakePoolDepositStakeKeys {
    pub eversol_stake_pool_program: Pubkey,
    pub deposit_stake_spl_stake_pool: Pubkey,
    pub deposit_stake_validator_list: Pubkey,
    pub deposit_stake_deposit_authority: Pubkey,
    pub deposit_stake_withdraw_authority: Pubkey,
    pub deposit_stake_validator_stake: Pubkey,
    pub deposit_stake_reserve_stake: Pubkey,
    pub deposit_stake_manager_fee: Pubkey,
    pub clock: Pubkey,
    pub stake_history: Pubkey,
    pub token_program: Pubkey,
    pub stake_program: Pubkey,
}
impl<'me>
    From<&EversolStakePoolDepositStakeAccounts<'me, '_, '_, '_, '_, '_, '_, '_, '_, '_, '_, '_, '_>>
    for EversolStakePoolDepositStakeKeys
{
    fn from(
        accounts: &EversolStakePoolDepositStakeAccounts<
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
            '_,
            '_,
        >,
    ) -> Self {
        Self {
            eversol_stake_pool_program: *accounts.eversol_stake_pool_program.key,
            deposit_stake_spl_stake_pool: *accounts.deposit_stake_spl_stake_pool.key,
            deposit_stake_validator_list: *accounts.deposit_stake_validator_list.key,
            deposit_stake_deposit_authority: *accounts.deposit_stake_deposit_authority.key,
            deposit_stake_withdraw_authority: *accounts.deposit_stake_withdraw_authority.key,
            deposit_stake_validator_stake: *accounts.deposit_stake_validator_stake.key,
            deposit_stake_reserve_stake: *accounts.deposit_stake_reserve_stake.key,
            deposit_stake_manager_fee: *accounts.deposit_stake_manager_fee.key,
            clock: *accounts.clock.key,
            stake_history: *accounts.stake_history.key,
            token_program: *accounts.token_program.key,
            stake_program: *accounts.stake_program.key,
        }
    }
}
impl From<&EversolStakePoolDepositStakeKeys>
    for [AccountMeta; EVERSOL_STAKE_POOL_DEPOSIT_STAKE_IX_ACCOUNTS_LEN]
{
    fn from(keys: &EversolStakePoolDepositStakeKeys) -> Self {
        [
            AccountMeta::new_readonly(keys.eversol_stake_pool_program, false),
            AccountMeta::new(keys.deposit_stake_spl_stake_pool, false),
            AccountMeta::new(keys.deposit_stake_validator_list, false),
            AccountMeta::new_readonly(keys.deposit_stake_deposit_authority, false),
            AccountMeta::new_readonly(keys.deposit_stake_withdraw_authority, false),
            AccountMeta::new(keys.deposit_stake_validator_stake, false),
            AccountMeta::new(keys.deposit_stake_reserve_stake, false),
            AccountMeta::new(keys.deposit_stake_manager_fee, false),
            AccountMeta::new_readonly(keys.clock, false),
            AccountMeta::new_readonly(keys.stake_history, false),
            AccountMeta::new_readonly(keys.token_program, false),
            AccountMeta::new_readonly(keys.stake_program, false),
        ]
    }
}
impl<'a>
    From<&EversolStakePoolDepositStakeAccounts<'_, 'a, 'a, 'a, 'a, 'a, 'a, 'a, 'a, 'a, 'a, 'a, 'a>>
    for [AccountInfo<'a>; EVERSOL_STAKE_POOL_DEPOSIT_STAKE_IX_ACCOUNTS_LEN]
{
    fn from(
        accounts: &EversolStakePoolDepositStakeAccounts<
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
            'a,
            'a,
        >,
    ) -> Self {
        [
            accounts.eversol_stake_pool_program.clone(),
            accounts.deposit_stake_spl_stake_pool.clone(),
            accounts.deposit_stake_validator_list.clone(),
            accounts.deposit_stake_deposit_authority.clone(),
            accounts.deposit_stake_withdraw_authority.clone(),
            accounts.deposit_stake_validator_stake.clone(),
            accounts.deposit_stake_reserve_stake.clone(),
            accounts.deposit_stake_manager_fee.clone(),
            accounts.clock.clone(),
            accounts.stake_history.clone(),
            accounts.token_program.clone(),
            accounts.stake_program.clone(),
        ]
    }
}
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug)]
pub struct EversolStakePoolDepositStakeIxArgs {}
#[derive(Copy, Clone, Debug)]
pub struct EversolStakePoolDepositStakeIxData<'me>(pub &'me EversolStakePoolDepositStakeIxArgs);
pub const EVERSOL_STAKE_POOL_DEPOSIT_STAKE_IX_DISCM: u8 = 0u8;
impl<'me> From<&'me EversolStakePoolDepositStakeIxArgs>
    for EversolStakePoolDepositStakeIxData<'me>
{
    fn from(args: &'me EversolStakePoolDepositStakeIxArgs) -> Self {
        Self(args)
    }
}
impl BorshSerialize for EversolStakePoolDepositStakeIxData<'_> {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        writer.write_all(&[EVERSOL_STAKE_POOL_DEPOSIT_STAKE_IX_DISCM])?;
        self.0.serialize(writer)
    }
}
pub fn eversol_stake_pool_deposit_stake_ix<
    K: Into<EversolStakePoolDepositStakeKeys>,
    A: Into<EversolStakePoolDepositStakeIxArgs>,
>(
    accounts: K,
    args: A,
) -> std::io::Result<Instruction> {
    let keys: EversolStakePoolDepositStakeKeys = accounts.into();
    let metas: [AccountMeta; EVERSOL_STAKE_POOL_DEPOSIT_STAKE_IX_ACCOUNTS_LEN] = (&keys).into();
    let args_full: EversolStakePoolDepositStakeIxArgs = args.into();
    let data: EversolStakePoolDepositStakeIxData = (&args_full).into();
    Ok(Instruction {
        program_id: crate::ID,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn eversol_stake_pool_deposit_stake_invoke<'a, A: Into<EversolStakePoolDepositStakeIxArgs>>(
    accounts: &EversolStakePoolDepositStakeAccounts<
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
        'a,
        'a,
    >,
    args: A,
) -> ProgramResult {
    let ix = eversol_stake_pool_deposit_stake_ix(accounts, args)?;
    let account_info: [AccountInfo<'a>; EVERSOL_STAKE_POOL_DEPOSIT_STAKE_IX_ACCOUNTS_LEN] =
        accounts.into();
    invoke(&ix, &account_info)
}
pub fn eversol_stake_pool_deposit_stake_invoke_signed<
    'a,
    A: Into<EversolStakePoolDepositStakeIxArgs>,
>(
    accounts: &EversolStakePoolDepositStakeAccounts<
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
        'a,
        'a,
    >,
    args: A,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let ix = eversol_stake_pool_deposit_stake_ix(accounts, args)?;
    let account_info: [AccountInfo<'a>; EVERSOL_STAKE_POOL_DEPOSIT_STAKE_IX_ACCOUNTS_LEN] =
        accounts.into();
    invoke_signed(&ix, &account_info, seeds)
}
pub const SOCEAN_STAKE_POOL_DEPOSIT_STAKE_IX_ACCOUNTS_LEN: usize = 12usize;
#[derive(Copy, Clone, Debug)]
pub struct SoceanStakePoolDepositStakeAccounts<
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
    'a10: 'me,
    'a11: 'me,
> {
    pub socean_stake_pool_program: &'me AccountInfo<'a0>,
    pub deposit_stake_spl_stake_pool: &'me AccountInfo<'a1>,
    pub deposit_stake_validator_list: &'me AccountInfo<'a2>,
    pub deposit_stake_deposit_authority: &'me AccountInfo<'a3>,
    pub deposit_stake_withdraw_authority: &'me AccountInfo<'a4>,
    pub deposit_stake_validator_stake: &'me AccountInfo<'a5>,
    pub deposit_stake_reserve_stake: &'me AccountInfo<'a6>,
    pub deposit_stake_manager_fee: &'me AccountInfo<'a7>,
    pub clock: &'me AccountInfo<'a8>,
    pub stake_history: &'me AccountInfo<'a9>,
    pub token_program: &'me AccountInfo<'a10>,
    pub stake_program: &'me AccountInfo<'a11>,
}
#[derive(Copy, Clone, Debug)]
pub struct SoceanStakePoolDepositStakeKeys {
    pub socean_stake_pool_program: Pubkey,
    pub deposit_stake_spl_stake_pool: Pubkey,
    pub deposit_stake_validator_list: Pubkey,
    pub deposit_stake_deposit_authority: Pubkey,
    pub deposit_stake_withdraw_authority: Pubkey,
    pub deposit_stake_validator_stake: Pubkey,
    pub deposit_stake_reserve_stake: Pubkey,
    pub deposit_stake_manager_fee: Pubkey,
    pub clock: Pubkey,
    pub stake_history: Pubkey,
    pub token_program: Pubkey,
    pub stake_program: Pubkey,
}
impl<'me>
    From<&SoceanStakePoolDepositStakeAccounts<'me, '_, '_, '_, '_, '_, '_, '_, '_, '_, '_, '_, '_>>
    for SoceanStakePoolDepositStakeKeys
{
    fn from(
        accounts: &SoceanStakePoolDepositStakeAccounts<
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
            '_,
            '_,
        >,
    ) -> Self {
        Self {
            socean_stake_pool_program: *accounts.socean_stake_pool_program.key,
            deposit_stake_spl_stake_pool: *accounts.deposit_stake_spl_stake_pool.key,
            deposit_stake_validator_list: *accounts.deposit_stake_validator_list.key,
            deposit_stake_deposit_authority: *accounts.deposit_stake_deposit_authority.key,
            deposit_stake_withdraw_authority: *accounts.deposit_stake_withdraw_authority.key,
            deposit_stake_validator_stake: *accounts.deposit_stake_validator_stake.key,
            deposit_stake_reserve_stake: *accounts.deposit_stake_reserve_stake.key,
            deposit_stake_manager_fee: *accounts.deposit_stake_manager_fee.key,
            clock: *accounts.clock.key,
            stake_history: *accounts.stake_history.key,
            token_program: *accounts.token_program.key,
            stake_program: *accounts.stake_program.key,
        }
    }
}
impl From<&SoceanStakePoolDepositStakeKeys>
    for [AccountMeta; SOCEAN_STAKE_POOL_DEPOSIT_STAKE_IX_ACCOUNTS_LEN]
{
    fn from(keys: &SoceanStakePoolDepositStakeKeys) -> Self {
        [
            AccountMeta::new_readonly(keys.socean_stake_pool_program, false),
            AccountMeta::new(keys.deposit_stake_spl_stake_pool, false),
            AccountMeta::new(keys.deposit_stake_validator_list, false),
            AccountMeta::new_readonly(keys.deposit_stake_deposit_authority, false),
            AccountMeta::new_readonly(keys.deposit_stake_withdraw_authority, false),
            AccountMeta::new(keys.deposit_stake_validator_stake, false),
            AccountMeta::new(keys.deposit_stake_reserve_stake, false),
            AccountMeta::new(keys.deposit_stake_manager_fee, false),
            AccountMeta::new_readonly(keys.clock, false),
            AccountMeta::new_readonly(keys.stake_history, false),
            AccountMeta::new_readonly(keys.token_program, false),
            AccountMeta::new_readonly(keys.stake_program, false),
        ]
    }
}
impl<'a>
    From<&SoceanStakePoolDepositStakeAccounts<'_, 'a, 'a, 'a, 'a, 'a, 'a, 'a, 'a, 'a, 'a, 'a, 'a>>
    for [AccountInfo<'a>; SOCEAN_STAKE_POOL_DEPOSIT_STAKE_IX_ACCOUNTS_LEN]
{
    fn from(
        accounts: &SoceanStakePoolDepositStakeAccounts<
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
            'a,
            'a,
        >,
    ) -> Self {
        [
            accounts.socean_stake_pool_program.clone(),
            accounts.deposit_stake_spl_stake_pool.clone(),
            accounts.deposit_stake_validator_list.clone(),
            accounts.deposit_stake_deposit_authority.clone(),
            accounts.deposit_stake_withdraw_authority.clone(),
            accounts.deposit_stake_validator_stake.clone(),
            accounts.deposit_stake_reserve_stake.clone(),
            accounts.deposit_stake_manager_fee.clone(),
            accounts.clock.clone(),
            accounts.stake_history.clone(),
            accounts.token_program.clone(),
            accounts.stake_program.clone(),
        ]
    }
}
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug)]
pub struct SoceanStakePoolDepositStakeIxArgs {}
#[derive(Copy, Clone, Debug)]
pub struct SoceanStakePoolDepositStakeIxData<'me>(pub &'me SoceanStakePoolDepositStakeIxArgs);
pub const SOCEAN_STAKE_POOL_DEPOSIT_STAKE_IX_DISCM: u8 = 1u8;
impl<'me> From<&'me SoceanStakePoolDepositStakeIxArgs> for SoceanStakePoolDepositStakeIxData<'me> {
    fn from(args: &'me SoceanStakePoolDepositStakeIxArgs) -> Self {
        Self(args)
    }
}
impl BorshSerialize for SoceanStakePoolDepositStakeIxData<'_> {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        writer.write_all(&[SOCEAN_STAKE_POOL_DEPOSIT_STAKE_IX_DISCM])?;
        self.0.serialize(writer)
    }
}
pub fn socean_stake_pool_deposit_stake_ix<
    K: Into<SoceanStakePoolDepositStakeKeys>,
    A: Into<SoceanStakePoolDepositStakeIxArgs>,
>(
    accounts: K,
    args: A,
) -> std::io::Result<Instruction> {
    let keys: SoceanStakePoolDepositStakeKeys = accounts.into();
    let metas: [AccountMeta; SOCEAN_STAKE_POOL_DEPOSIT_STAKE_IX_ACCOUNTS_LEN] = (&keys).into();
    let args_full: SoceanStakePoolDepositStakeIxArgs = args.into();
    let data: SoceanStakePoolDepositStakeIxData = (&args_full).into();
    Ok(Instruction {
        program_id: crate::ID,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn socean_stake_pool_deposit_stake_invoke<'a, A: Into<SoceanStakePoolDepositStakeIxArgs>>(
    accounts: &SoceanStakePoolDepositStakeAccounts<
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
        'a,
        'a,
    >,
    args: A,
) -> ProgramResult {
    let ix = socean_stake_pool_deposit_stake_ix(accounts, args)?;
    let account_info: [AccountInfo<'a>; SOCEAN_STAKE_POOL_DEPOSIT_STAKE_IX_ACCOUNTS_LEN] =
        accounts.into();
    invoke(&ix, &account_info)
}
pub fn socean_stake_pool_deposit_stake_invoke_signed<
    'a,
    A: Into<SoceanStakePoolDepositStakeIxArgs>,
>(
    accounts: &SoceanStakePoolDepositStakeAccounts<
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
        'a,
        'a,
    >,
    args: A,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let ix = socean_stake_pool_deposit_stake_ix(accounts, args)?;
    let account_info: [AccountInfo<'a>; SOCEAN_STAKE_POOL_DEPOSIT_STAKE_IX_ACCOUNTS_LEN] =
        accounts.into();
    invoke_signed(&ix, &account_info, seeds)
}
pub const SPL_STAKE_POOL_DEPOSIT_STAKE_IX_ACCOUNTS_LEN: usize = 12usize;
#[derive(Copy, Clone, Debug)]
pub struct SplStakePoolDepositStakeAccounts<
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
    'a10: 'me,
    'a11: 'me,
> {
    pub spl_stake_pool_program: &'me AccountInfo<'a0>,
    pub deposit_stake_spl_stake_pool: &'me AccountInfo<'a1>,
    pub deposit_stake_validator_list: &'me AccountInfo<'a2>,
    pub deposit_stake_deposit_authority: &'me AccountInfo<'a3>,
    pub deposit_stake_withdraw_authority: &'me AccountInfo<'a4>,
    pub deposit_stake_validator_stake: &'me AccountInfo<'a5>,
    pub deposit_stake_reserve_stake: &'me AccountInfo<'a6>,
    pub deposit_stake_manager_fee: &'me AccountInfo<'a7>,
    pub clock: &'me AccountInfo<'a8>,
    pub stake_history: &'me AccountInfo<'a9>,
    pub token_program: &'me AccountInfo<'a10>,
    pub stake_program: &'me AccountInfo<'a11>,
}
#[derive(Copy, Clone, Debug)]
pub struct SplStakePoolDepositStakeKeys {
    pub spl_stake_pool_program: Pubkey,
    pub deposit_stake_spl_stake_pool: Pubkey,
    pub deposit_stake_validator_list: Pubkey,
    pub deposit_stake_deposit_authority: Pubkey,
    pub deposit_stake_withdraw_authority: Pubkey,
    pub deposit_stake_validator_stake: Pubkey,
    pub deposit_stake_reserve_stake: Pubkey,
    pub deposit_stake_manager_fee: Pubkey,
    pub clock: Pubkey,
    pub stake_history: Pubkey,
    pub token_program: Pubkey,
    pub stake_program: Pubkey,
}
impl<'me>
    From<&SplStakePoolDepositStakeAccounts<'me, '_, '_, '_, '_, '_, '_, '_, '_, '_, '_, '_, '_>>
    for SplStakePoolDepositStakeKeys
{
    fn from(
        accounts: &SplStakePoolDepositStakeAccounts<
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
            '_,
            '_,
        >,
    ) -> Self {
        Self {
            spl_stake_pool_program: *accounts.spl_stake_pool_program.key,
            deposit_stake_spl_stake_pool: *accounts.deposit_stake_spl_stake_pool.key,
            deposit_stake_validator_list: *accounts.deposit_stake_validator_list.key,
            deposit_stake_deposit_authority: *accounts.deposit_stake_deposit_authority.key,
            deposit_stake_withdraw_authority: *accounts.deposit_stake_withdraw_authority.key,
            deposit_stake_validator_stake: *accounts.deposit_stake_validator_stake.key,
            deposit_stake_reserve_stake: *accounts.deposit_stake_reserve_stake.key,
            deposit_stake_manager_fee: *accounts.deposit_stake_manager_fee.key,
            clock: *accounts.clock.key,
            stake_history: *accounts.stake_history.key,
            token_program: *accounts.token_program.key,
            stake_program: *accounts.stake_program.key,
        }
    }
}
impl From<&SplStakePoolDepositStakeKeys>
    for [AccountMeta; SPL_STAKE_POOL_DEPOSIT_STAKE_IX_ACCOUNTS_LEN]
{
    fn from(keys: &SplStakePoolDepositStakeKeys) -> Self {
        [
            AccountMeta::new_readonly(keys.spl_stake_pool_program, false),
            AccountMeta::new(keys.deposit_stake_spl_stake_pool, false),
            AccountMeta::new(keys.deposit_stake_validator_list, false),
            AccountMeta::new_readonly(keys.deposit_stake_deposit_authority, false),
            AccountMeta::new_readonly(keys.deposit_stake_withdraw_authority, false),
            AccountMeta::new(keys.deposit_stake_validator_stake, false),
            AccountMeta::new(keys.deposit_stake_reserve_stake, false),
            AccountMeta::new(keys.deposit_stake_manager_fee, false),
            AccountMeta::new_readonly(keys.clock, false),
            AccountMeta::new_readonly(keys.stake_history, false),
            AccountMeta::new_readonly(keys.token_program, false),
            AccountMeta::new_readonly(keys.stake_program, false),
        ]
    }
}
impl<'a> From<&SplStakePoolDepositStakeAccounts<'_, 'a, 'a, 'a, 'a, 'a, 'a, 'a, 'a, 'a, 'a, 'a, 'a>>
    for [AccountInfo<'a>; SPL_STAKE_POOL_DEPOSIT_STAKE_IX_ACCOUNTS_LEN]
{
    fn from(
        accounts: &SplStakePoolDepositStakeAccounts<
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
            'a,
            'a,
        >,
    ) -> Self {
        [
            accounts.spl_stake_pool_program.clone(),
            accounts.deposit_stake_spl_stake_pool.clone(),
            accounts.deposit_stake_validator_list.clone(),
            accounts.deposit_stake_deposit_authority.clone(),
            accounts.deposit_stake_withdraw_authority.clone(),
            accounts.deposit_stake_validator_stake.clone(),
            accounts.deposit_stake_reserve_stake.clone(),
            accounts.deposit_stake_manager_fee.clone(),
            accounts.clock.clone(),
            accounts.stake_history.clone(),
            accounts.token_program.clone(),
            accounts.stake_program.clone(),
        ]
    }
}
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug)]
pub struct SplStakePoolDepositStakeIxArgs {}
#[derive(Copy, Clone, Debug)]
pub struct SplStakePoolDepositStakeIxData<'me>(pub &'me SplStakePoolDepositStakeIxArgs);
pub const SPL_STAKE_POOL_DEPOSIT_STAKE_IX_DISCM: u8 = 2u8;
impl<'me> From<&'me SplStakePoolDepositStakeIxArgs> for SplStakePoolDepositStakeIxData<'me> {
    fn from(args: &'me SplStakePoolDepositStakeIxArgs) -> Self {
        Self(args)
    }
}
impl BorshSerialize for SplStakePoolDepositStakeIxData<'_> {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        writer.write_all(&[SPL_STAKE_POOL_DEPOSIT_STAKE_IX_DISCM])?;
        self.0.serialize(writer)
    }
}
pub fn spl_stake_pool_deposit_stake_ix<
    K: Into<SplStakePoolDepositStakeKeys>,
    A: Into<SplStakePoolDepositStakeIxArgs>,
>(
    accounts: K,
    args: A,
) -> std::io::Result<Instruction> {
    let keys: SplStakePoolDepositStakeKeys = accounts.into();
    let metas: [AccountMeta; SPL_STAKE_POOL_DEPOSIT_STAKE_IX_ACCOUNTS_LEN] = (&keys).into();
    let args_full: SplStakePoolDepositStakeIxArgs = args.into();
    let data: SplStakePoolDepositStakeIxData = (&args_full).into();
    Ok(Instruction {
        program_id: crate::ID,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn spl_stake_pool_deposit_stake_invoke<'a, A: Into<SplStakePoolDepositStakeIxArgs>>(
    accounts: &SplStakePoolDepositStakeAccounts<'_, 'a, 'a, 'a, 'a, 'a, 'a, 'a, 'a, 'a, 'a, 'a, 'a>,
    args: A,
) -> ProgramResult {
    let ix = spl_stake_pool_deposit_stake_ix(accounts, args)?;
    let account_info: [AccountInfo<'a>; SPL_STAKE_POOL_DEPOSIT_STAKE_IX_ACCOUNTS_LEN] =
        accounts.into();
    invoke(&ix, &account_info)
}
pub fn spl_stake_pool_deposit_stake_invoke_signed<'a, A: Into<SplStakePoolDepositStakeIxArgs>>(
    accounts: &SplStakePoolDepositStakeAccounts<'_, 'a, 'a, 'a, 'a, 'a, 'a, 'a, 'a, 'a, 'a, 'a, 'a>,
    args: A,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let ix = spl_stake_pool_deposit_stake_ix(accounts, args)?;
    let account_info: [AccountInfo<'a>; SPL_STAKE_POOL_DEPOSIT_STAKE_IX_ACCOUNTS_LEN] =
        accounts.into();
    invoke_signed(&ix, &account_info, seeds)
}
pub const MARINADE_DEPOSIT_STAKE_IX_ACCOUNTS_LEN: usize = 11usize;
#[derive(Copy, Clone, Debug)]
pub struct MarinadeDepositStakeAccounts<
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
    'a10: 'me,
> {
    pub marinade_program: &'me AccountInfo<'a0>,
    pub deposit_stake_marinade_state: &'me AccountInfo<'a1>,
    pub deposit_stake_validator_list: &'me AccountInfo<'a2>,
    pub deposit_stake_stake_list: &'me AccountInfo<'a3>,
    pub deposit_stake_duplication_flag: &'me AccountInfo<'a4>,
    pub deposit_stake_msol_mint_auth: &'me AccountInfo<'a5>,
    pub clock: &'me AccountInfo<'a6>,
    pub rent: &'me AccountInfo<'a7>,
    pub system_program: &'me AccountInfo<'a8>,
    pub token_program: &'me AccountInfo<'a9>,
    pub stake_program: &'me AccountInfo<'a10>,
}
#[derive(Copy, Clone, Debug)]
pub struct MarinadeDepositStakeKeys {
    pub marinade_program: Pubkey,
    pub deposit_stake_marinade_state: Pubkey,
    pub deposit_stake_validator_list: Pubkey,
    pub deposit_stake_stake_list: Pubkey,
    pub deposit_stake_duplication_flag: Pubkey,
    pub deposit_stake_msol_mint_auth: Pubkey,
    pub clock: Pubkey,
    pub rent: Pubkey,
    pub system_program: Pubkey,
    pub token_program: Pubkey,
    pub stake_program: Pubkey,
}
impl<'me> From<&MarinadeDepositStakeAccounts<'me, '_, '_, '_, '_, '_, '_, '_, '_, '_, '_, '_>>
    for MarinadeDepositStakeKeys
{
    fn from(
        accounts: &MarinadeDepositStakeAccounts<'me, '_, '_, '_, '_, '_, '_, '_, '_, '_, '_, '_>,
    ) -> Self {
        Self {
            marinade_program: *accounts.marinade_program.key,
            deposit_stake_marinade_state: *accounts.deposit_stake_marinade_state.key,
            deposit_stake_validator_list: *accounts.deposit_stake_validator_list.key,
            deposit_stake_stake_list: *accounts.deposit_stake_stake_list.key,
            deposit_stake_duplication_flag: *accounts.deposit_stake_duplication_flag.key,
            deposit_stake_msol_mint_auth: *accounts.deposit_stake_msol_mint_auth.key,
            clock: *accounts.clock.key,
            rent: *accounts.rent.key,
            system_program: *accounts.system_program.key,
            token_program: *accounts.token_program.key,
            stake_program: *accounts.stake_program.key,
        }
    }
}
impl From<&MarinadeDepositStakeKeys> for [AccountMeta; MARINADE_DEPOSIT_STAKE_IX_ACCOUNTS_LEN] {
    fn from(keys: &MarinadeDepositStakeKeys) -> Self {
        [
            AccountMeta::new_readonly(keys.marinade_program, false),
            AccountMeta::new(keys.deposit_stake_marinade_state, false),
            AccountMeta::new(keys.deposit_stake_validator_list, false),
            AccountMeta::new(keys.deposit_stake_stake_list, false),
            AccountMeta::new(keys.deposit_stake_duplication_flag, false),
            AccountMeta::new_readonly(keys.deposit_stake_msol_mint_auth, false),
            AccountMeta::new_readonly(keys.clock, false),
            AccountMeta::new_readonly(keys.rent, false),
            AccountMeta::new_readonly(keys.system_program, false),
            AccountMeta::new_readonly(keys.token_program, false),
            AccountMeta::new_readonly(keys.stake_program, false),
        ]
    }
}
impl<'a> From<&MarinadeDepositStakeAccounts<'_, 'a, 'a, 'a, 'a, 'a, 'a, 'a, 'a, 'a, 'a, 'a>>
    for [AccountInfo<'a>; MARINADE_DEPOSIT_STAKE_IX_ACCOUNTS_LEN]
{
    fn from(
        accounts: &MarinadeDepositStakeAccounts<'_, 'a, 'a, 'a, 'a, 'a, 'a, 'a, 'a, 'a, 'a, 'a>,
    ) -> Self {
        [
            accounts.marinade_program.clone(),
            accounts.deposit_stake_marinade_state.clone(),
            accounts.deposit_stake_validator_list.clone(),
            accounts.deposit_stake_stake_list.clone(),
            accounts.deposit_stake_duplication_flag.clone(),
            accounts.deposit_stake_msol_mint_auth.clone(),
            accounts.clock.clone(),
            accounts.rent.clone(),
            accounts.system_program.clone(),
            accounts.token_program.clone(),
            accounts.stake_program.clone(),
        ]
    }
}
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug)]
pub struct MarinadeDepositStakeIxArgs {}
#[derive(Copy, Clone, Debug)]
pub struct MarinadeDepositStakeIxData<'me>(pub &'me MarinadeDepositStakeIxArgs);
pub const MARINADE_DEPOSIT_STAKE_IX_DISCM: u8 = 3u8;
impl<'me> From<&'me MarinadeDepositStakeIxArgs> for MarinadeDepositStakeIxData<'me> {
    fn from(args: &'me MarinadeDepositStakeIxArgs) -> Self {
        Self(args)
    }
}
impl BorshSerialize for MarinadeDepositStakeIxData<'_> {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        writer.write_all(&[MARINADE_DEPOSIT_STAKE_IX_DISCM])?;
        self.0.serialize(writer)
    }
}
pub fn marinade_deposit_stake_ix<
    K: Into<MarinadeDepositStakeKeys>,
    A: Into<MarinadeDepositStakeIxArgs>,
>(
    accounts: K,
    args: A,
) -> std::io::Result<Instruction> {
    let keys: MarinadeDepositStakeKeys = accounts.into();
    let metas: [AccountMeta; MARINADE_DEPOSIT_STAKE_IX_ACCOUNTS_LEN] = (&keys).into();
    let args_full: MarinadeDepositStakeIxArgs = args.into();
    let data: MarinadeDepositStakeIxData = (&args_full).into();
    Ok(Instruction {
        program_id: crate::ID,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn marinade_deposit_stake_invoke<'a, A: Into<MarinadeDepositStakeIxArgs>>(
    accounts: &MarinadeDepositStakeAccounts<'_, 'a, 'a, 'a, 'a, 'a, 'a, 'a, 'a, 'a, 'a, 'a>,
    args: A,
) -> ProgramResult {
    let ix = marinade_deposit_stake_ix(accounts, args)?;
    let account_info: [AccountInfo<'a>; MARINADE_DEPOSIT_STAKE_IX_ACCOUNTS_LEN] = accounts.into();
    invoke(&ix, &account_info)
}
pub fn marinade_deposit_stake_invoke_signed<'a, A: Into<MarinadeDepositStakeIxArgs>>(
    accounts: &MarinadeDepositStakeAccounts<'_, 'a, 'a, 'a, 'a, 'a, 'a, 'a, 'a, 'a, 'a, 'a>,
    args: A,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let ix = marinade_deposit_stake_ix(accounts, args)?;
    let account_info: [AccountInfo<'a>; MARINADE_DEPOSIT_STAKE_IX_ACCOUNTS_LEN] = accounts.into();
    invoke_signed(&ix, &account_info, seeds)
}
pub const UNSTAKE_IT_DEPOSIT_STAKE_IX_ACCOUNTS_LEN: usize = 11usize;
#[derive(Copy, Clone, Debug)]
pub struct UnstakeItDepositStakeAccounts<
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
    'a10: 'me,
> {
    pub unstakeit_program: &'me AccountInfo<'a0>,
    pub deposit_stake_unstake_pool: &'me AccountInfo<'a1>,
    pub deposit_stake_pool_sol_reserves: &'me AccountInfo<'a2>,
    pub deposit_stake_unstake_fee: &'me AccountInfo<'a3>,
    pub deposit_stake_stake_acc_record: &'me AccountInfo<'a4>,
    pub deposit_stake_protocol_fee: &'me AccountInfo<'a5>,
    pub deposit_stake_protocol_fee_dest: &'me AccountInfo<'a6>,
    pub clock: &'me AccountInfo<'a7>,
    pub stake_program: &'me AccountInfo<'a8>,
    pub system_program: &'me AccountInfo<'a9>,
    pub token_program: &'me AccountInfo<'a10>,
}
#[derive(Copy, Clone, Debug)]
pub struct UnstakeItDepositStakeKeys {
    pub unstakeit_program: Pubkey,
    pub deposit_stake_unstake_pool: Pubkey,
    pub deposit_stake_pool_sol_reserves: Pubkey,
    pub deposit_stake_unstake_fee: Pubkey,
    pub deposit_stake_stake_acc_record: Pubkey,
    pub deposit_stake_protocol_fee: Pubkey,
    pub deposit_stake_protocol_fee_dest: Pubkey,
    pub clock: Pubkey,
    pub stake_program: Pubkey,
    pub system_program: Pubkey,
    pub token_program: Pubkey,
}
impl<'me> From<&UnstakeItDepositStakeAccounts<'me, '_, '_, '_, '_, '_, '_, '_, '_, '_, '_, '_>>
    for UnstakeItDepositStakeKeys
{
    fn from(
        accounts: &UnstakeItDepositStakeAccounts<'me, '_, '_, '_, '_, '_, '_, '_, '_, '_, '_, '_>,
    ) -> Self {
        Self {
            unstakeit_program: *accounts.unstakeit_program.key,
            deposit_stake_unstake_pool: *accounts.deposit_stake_unstake_pool.key,
            deposit_stake_pool_sol_reserves: *accounts.deposit_stake_pool_sol_reserves.key,
            deposit_stake_unstake_fee: *accounts.deposit_stake_unstake_fee.key,
            deposit_stake_stake_acc_record: *accounts.deposit_stake_stake_acc_record.key,
            deposit_stake_protocol_fee: *accounts.deposit_stake_protocol_fee.key,
            deposit_stake_protocol_fee_dest: *accounts.deposit_stake_protocol_fee_dest.key,
            clock: *accounts.clock.key,
            stake_program: *accounts.stake_program.key,
            system_program: *accounts.system_program.key,
            token_program: *accounts.token_program.key,
        }
    }
}
impl From<&UnstakeItDepositStakeKeys> for [AccountMeta; UNSTAKE_IT_DEPOSIT_STAKE_IX_ACCOUNTS_LEN] {
    fn from(keys: &UnstakeItDepositStakeKeys) -> Self {
        [
            AccountMeta::new_readonly(keys.unstakeit_program, false),
            AccountMeta::new(keys.deposit_stake_unstake_pool, false),
            AccountMeta::new(keys.deposit_stake_pool_sol_reserves, false),
            AccountMeta::new_readonly(keys.deposit_stake_unstake_fee, false),
            AccountMeta::new(keys.deposit_stake_stake_acc_record, false),
            AccountMeta::new_readonly(keys.deposit_stake_protocol_fee, false),
            AccountMeta::new(keys.deposit_stake_protocol_fee_dest, false),
            AccountMeta::new_readonly(keys.clock, false),
            AccountMeta::new_readonly(keys.stake_program, false),
            AccountMeta::new_readonly(keys.system_program, false),
            AccountMeta::new_readonly(keys.token_program, false),
        ]
    }
}
impl<'a> From<&UnstakeItDepositStakeAccounts<'_, 'a, 'a, 'a, 'a, 'a, 'a, 'a, 'a, 'a, 'a, 'a>>
    for [AccountInfo<'a>; UNSTAKE_IT_DEPOSIT_STAKE_IX_ACCOUNTS_LEN]
{
    fn from(
        accounts: &UnstakeItDepositStakeAccounts<'_, 'a, 'a, 'a, 'a, 'a, 'a, 'a, 'a, 'a, 'a, 'a>,
    ) -> Self {
        [
            accounts.unstakeit_program.clone(),
            accounts.deposit_stake_unstake_pool.clone(),
            accounts.deposit_stake_pool_sol_reserves.clone(),
            accounts.deposit_stake_unstake_fee.clone(),
            accounts.deposit_stake_stake_acc_record.clone(),
            accounts.deposit_stake_protocol_fee.clone(),
            accounts.deposit_stake_protocol_fee_dest.clone(),
            accounts.clock.clone(),
            accounts.stake_program.clone(),
            accounts.system_program.clone(),
            accounts.token_program.clone(),
        ]
    }
}
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug)]
pub struct UnstakeItDepositStakeIxArgs {}
#[derive(Copy, Clone, Debug)]
pub struct UnstakeItDepositStakeIxData<'me>(pub &'me UnstakeItDepositStakeIxArgs);
pub const UNSTAKE_IT_DEPOSIT_STAKE_IX_DISCM: u8 = 4u8;
impl<'me> From<&'me UnstakeItDepositStakeIxArgs> for UnstakeItDepositStakeIxData<'me> {
    fn from(args: &'me UnstakeItDepositStakeIxArgs) -> Self {
        Self(args)
    }
}
impl BorshSerialize for UnstakeItDepositStakeIxData<'_> {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        writer.write_all(&[UNSTAKE_IT_DEPOSIT_STAKE_IX_DISCM])?;
        self.0.serialize(writer)
    }
}
pub fn unstake_it_deposit_stake_ix<
    K: Into<UnstakeItDepositStakeKeys>,
    A: Into<UnstakeItDepositStakeIxArgs>,
>(
    accounts: K,
    args: A,
) -> std::io::Result<Instruction> {
    let keys: UnstakeItDepositStakeKeys = accounts.into();
    let metas: [AccountMeta; UNSTAKE_IT_DEPOSIT_STAKE_IX_ACCOUNTS_LEN] = (&keys).into();
    let args_full: UnstakeItDepositStakeIxArgs = args.into();
    let data: UnstakeItDepositStakeIxData = (&args_full).into();
    Ok(Instruction {
        program_id: crate::ID,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn unstake_it_deposit_stake_invoke<'a, A: Into<UnstakeItDepositStakeIxArgs>>(
    accounts: &UnstakeItDepositStakeAccounts<'_, 'a, 'a, 'a, 'a, 'a, 'a, 'a, 'a, 'a, 'a, 'a>,
    args: A,
) -> ProgramResult {
    let ix = unstake_it_deposit_stake_ix(accounts, args)?;
    let account_info: [AccountInfo<'a>; UNSTAKE_IT_DEPOSIT_STAKE_IX_ACCOUNTS_LEN] = accounts.into();
    invoke(&ix, &account_info)
}
pub fn unstake_it_deposit_stake_invoke_signed<'a, A: Into<UnstakeItDepositStakeIxArgs>>(
    accounts: &UnstakeItDepositStakeAccounts<'_, 'a, 'a, 'a, 'a, 'a, 'a, 'a, 'a, 'a, 'a, 'a>,
    args: A,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let ix = unstake_it_deposit_stake_ix(accounts, args)?;
    let account_info: [AccountInfo<'a>; UNSTAKE_IT_DEPOSIT_STAKE_IX_ACCOUNTS_LEN] = accounts.into();
    invoke_signed(&ix, &account_info, seeds)
}
