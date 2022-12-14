use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::AccountInfo,
    entrypoint::ProgramResult,
    instruction::{AccountMeta, Instruction},
    program::{invoke, invoke_signed},
    pubkey::Pubkey,
};
pub const EVERSOL_STAKE_POOL_DEPOSIT_SOL_IX_ACCOUNTS_LEN: usize = 5usize;
#[derive(Copy, Clone, Debug)]
pub struct EversolStakePoolDepositSolAccounts<'me, 'a0: 'me, 'a1: 'me, 'a2: 'me, 'a3: 'me, 'a4: 'me>
{
    pub eversol_stake_pool_program: &'me AccountInfo<'a0>,
    pub stake_pool: &'me AccountInfo<'a1>,
    pub stake_pool_withdraw_authority: &'me AccountInfo<'a2>,
    pub stake_pool_reserve_stake: &'me AccountInfo<'a3>,
    pub stake_pool_manager_fee: &'me AccountInfo<'a4>,
}
#[derive(Copy, Clone, Debug)]
pub struct EversolStakePoolDepositSolKeys {
    pub eversol_stake_pool_program: Pubkey,
    pub stake_pool: Pubkey,
    pub stake_pool_withdraw_authority: Pubkey,
    pub stake_pool_reserve_stake: Pubkey,
    pub stake_pool_manager_fee: Pubkey,
}
impl<'me> From<&EversolStakePoolDepositSolAccounts<'me, '_, '_, '_, '_, '_>>
    for EversolStakePoolDepositSolKeys
{
    fn from(accounts: &EversolStakePoolDepositSolAccounts<'me, '_, '_, '_, '_, '_>) -> Self {
        Self {
            eversol_stake_pool_program: *accounts.eversol_stake_pool_program.key,
            stake_pool: *accounts.stake_pool.key,
            stake_pool_withdraw_authority: *accounts.stake_pool_withdraw_authority.key,
            stake_pool_reserve_stake: *accounts.stake_pool_reserve_stake.key,
            stake_pool_manager_fee: *accounts.stake_pool_manager_fee.key,
        }
    }
}
impl From<&EversolStakePoolDepositSolKeys>
    for [AccountMeta; EVERSOL_STAKE_POOL_DEPOSIT_SOL_IX_ACCOUNTS_LEN]
{
    fn from(keys: &EversolStakePoolDepositSolKeys) -> Self {
        [
            AccountMeta::new_readonly(keys.eversol_stake_pool_program, false),
            AccountMeta::new(keys.stake_pool, false),
            AccountMeta::new_readonly(keys.stake_pool_withdraw_authority, false),
            AccountMeta::new(keys.stake_pool_reserve_stake, false),
            AccountMeta::new(keys.stake_pool_manager_fee, false),
        ]
    }
}
impl<'a> From<&EversolStakePoolDepositSolAccounts<'_, 'a, 'a, 'a, 'a, 'a>>
    for [AccountInfo<'a>; EVERSOL_STAKE_POOL_DEPOSIT_SOL_IX_ACCOUNTS_LEN]
{
    fn from(accounts: &EversolStakePoolDepositSolAccounts<'_, 'a, 'a, 'a, 'a, 'a>) -> Self {
        [
            accounts.eversol_stake_pool_program.clone(),
            accounts.stake_pool.clone(),
            accounts.stake_pool_withdraw_authority.clone(),
            accounts.stake_pool_reserve_stake.clone(),
            accounts.stake_pool_manager_fee.clone(),
        ]
    }
}
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug)]
pub struct EversolStakePoolDepositSolIxArgs {}
#[derive(Copy, Clone, Debug)]
pub struct EversolStakePoolDepositSolIxData<'me>(pub &'me EversolStakePoolDepositSolIxArgs);
pub const EVERSOL_STAKE_POOL_DEPOSIT_SOL_IX_DISCM: u8 = 0u8;
impl<'me> From<&'me EversolStakePoolDepositSolIxArgs> for EversolStakePoolDepositSolIxData<'me> {
    fn from(args: &'me EversolStakePoolDepositSolIxArgs) -> Self {
        Self(args)
    }
}
impl BorshSerialize for EversolStakePoolDepositSolIxData<'_> {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        writer.write_all(&[EVERSOL_STAKE_POOL_DEPOSIT_SOL_IX_DISCM])?;
        self.0.serialize(writer)
    }
}
pub fn eversol_stake_pool_deposit_sol_ix<
    K: Into<EversolStakePoolDepositSolKeys>,
    A: Into<EversolStakePoolDepositSolIxArgs>,
>(
    accounts: K,
    args: A,
) -> std::io::Result<Instruction> {
    let keys: EversolStakePoolDepositSolKeys = accounts.into();
    let metas: [AccountMeta; EVERSOL_STAKE_POOL_DEPOSIT_SOL_IX_ACCOUNTS_LEN] = (&keys).into();
    let args_full: EversolStakePoolDepositSolIxArgs = args.into();
    let data: EversolStakePoolDepositSolIxData = (&args_full).into();
    Ok(Instruction {
        program_id: crate::ID,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn eversol_stake_pool_deposit_sol_invoke<'a, A: Into<EversolStakePoolDepositSolIxArgs>>(
    accounts: &EversolStakePoolDepositSolAccounts<'_, 'a, 'a, 'a, 'a, 'a>,
    args: A,
) -> ProgramResult {
    let ix = eversol_stake_pool_deposit_sol_ix(accounts, args)?;
    let account_info: [AccountInfo<'a>; EVERSOL_STAKE_POOL_DEPOSIT_SOL_IX_ACCOUNTS_LEN] =
        accounts.into();
    invoke(&ix, &account_info)
}
pub fn eversol_stake_pool_deposit_sol_invoke_signed<
    'a,
    A: Into<EversolStakePoolDepositSolIxArgs>,
>(
    accounts: &EversolStakePoolDepositSolAccounts<'_, 'a, 'a, 'a, 'a, 'a>,
    args: A,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let ix = eversol_stake_pool_deposit_sol_ix(accounts, args)?;
    let account_info: [AccountInfo<'a>; EVERSOL_STAKE_POOL_DEPOSIT_SOL_IX_ACCOUNTS_LEN] =
        accounts.into();
    invoke_signed(&ix, &account_info, seeds)
}
pub const LIDO_DEPOSIT_SOL_IX_ACCOUNTS_LEN: usize = 4usize;
#[derive(Copy, Clone, Debug)]
pub struct LidoDepositSolAccounts<'me, 'a0: 'me, 'a1: 'me, 'a2: 'me, 'a3: 'me> {
    pub lido_program: &'me AccountInfo<'a0>,
    pub solido: &'me AccountInfo<'a1>,
    pub lido_reserve: &'me AccountInfo<'a2>,
    pub stsol_mint_authority: &'me AccountInfo<'a3>,
}
#[derive(Copy, Clone, Debug)]
pub struct LidoDepositSolKeys {
    pub lido_program: Pubkey,
    pub solido: Pubkey,
    pub lido_reserve: Pubkey,
    pub stsol_mint_authority: Pubkey,
}
impl<'me> From<&LidoDepositSolAccounts<'me, '_, '_, '_, '_>> for LidoDepositSolKeys {
    fn from(accounts: &LidoDepositSolAccounts<'me, '_, '_, '_, '_>) -> Self {
        Self {
            lido_program: *accounts.lido_program.key,
            solido: *accounts.solido.key,
            lido_reserve: *accounts.lido_reserve.key,
            stsol_mint_authority: *accounts.stsol_mint_authority.key,
        }
    }
}
impl From<&LidoDepositSolKeys> for [AccountMeta; LIDO_DEPOSIT_SOL_IX_ACCOUNTS_LEN] {
    fn from(keys: &LidoDepositSolKeys) -> Self {
        [
            AccountMeta::new_readonly(keys.lido_program, false),
            AccountMeta::new(keys.solido, false),
            AccountMeta::new(keys.lido_reserve, false),
            AccountMeta::new_readonly(keys.stsol_mint_authority, false),
        ]
    }
}
impl<'a> From<&LidoDepositSolAccounts<'_, 'a, 'a, 'a, 'a>>
    for [AccountInfo<'a>; LIDO_DEPOSIT_SOL_IX_ACCOUNTS_LEN]
{
    fn from(accounts: &LidoDepositSolAccounts<'_, 'a, 'a, 'a, 'a>) -> Self {
        [
            accounts.lido_program.clone(),
            accounts.solido.clone(),
            accounts.lido_reserve.clone(),
            accounts.stsol_mint_authority.clone(),
        ]
    }
}
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug)]
pub struct LidoDepositSolIxArgs {}
#[derive(Copy, Clone, Debug)]
pub struct LidoDepositSolIxData<'me>(pub &'me LidoDepositSolIxArgs);
pub const LIDO_DEPOSIT_SOL_IX_DISCM: u8 = 1u8;
impl<'me> From<&'me LidoDepositSolIxArgs> for LidoDepositSolIxData<'me> {
    fn from(args: &'me LidoDepositSolIxArgs) -> Self {
        Self(args)
    }
}
impl BorshSerialize for LidoDepositSolIxData<'_> {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        writer.write_all(&[LIDO_DEPOSIT_SOL_IX_DISCM])?;
        self.0.serialize(writer)
    }
}
pub fn lido_deposit_sol_ix<K: Into<LidoDepositSolKeys>, A: Into<LidoDepositSolIxArgs>>(
    accounts: K,
    args: A,
) -> std::io::Result<Instruction> {
    let keys: LidoDepositSolKeys = accounts.into();
    let metas: [AccountMeta; LIDO_DEPOSIT_SOL_IX_ACCOUNTS_LEN] = (&keys).into();
    let args_full: LidoDepositSolIxArgs = args.into();
    let data: LidoDepositSolIxData = (&args_full).into();
    Ok(Instruction {
        program_id: crate::ID,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn lido_deposit_sol_invoke<'a, A: Into<LidoDepositSolIxArgs>>(
    accounts: &LidoDepositSolAccounts<'_, 'a, 'a, 'a, 'a>,
    args: A,
) -> ProgramResult {
    let ix = lido_deposit_sol_ix(accounts, args)?;
    let account_info: [AccountInfo<'a>; LIDO_DEPOSIT_SOL_IX_ACCOUNTS_LEN] = accounts.into();
    invoke(&ix, &account_info)
}
pub fn lido_deposit_sol_invoke_signed<'a, A: Into<LidoDepositSolIxArgs>>(
    accounts: &LidoDepositSolAccounts<'_, 'a, 'a, 'a, 'a>,
    args: A,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let ix = lido_deposit_sol_ix(accounts, args)?;
    let account_info: [AccountInfo<'a>; LIDO_DEPOSIT_SOL_IX_ACCOUNTS_LEN] = accounts.into();
    invoke_signed(&ix, &account_info, seeds)
}
pub const MARINADE_DEPOSIT_SOL_IX_ACCOUNTS_LEN: usize = 7usize;
#[derive(Copy, Clone, Debug)]
pub struct MarinadeDepositSolAccounts<
    'me,
    'a0: 'me,
    'a1: 'me,
    'a2: 'me,
    'a3: 'me,
    'a4: 'me,
    'a5: 'me,
    'a6: 'me,
> {
    pub marinade_program: &'me AccountInfo<'a0>,
    pub marinade_state: &'me AccountInfo<'a1>,
    pub marinade_liq_pool_sol_leg: &'me AccountInfo<'a2>,
    pub marinade_liq_pool_msol_leg: &'me AccountInfo<'a3>,
    pub marinade_liq_pool_msol_leg_auth: &'me AccountInfo<'a4>,
    pub marinade_reserve: &'me AccountInfo<'a5>,
    pub msol_mint_authority: &'me AccountInfo<'a6>,
}
#[derive(Copy, Clone, Debug)]
pub struct MarinadeDepositSolKeys {
    pub marinade_program: Pubkey,
    pub marinade_state: Pubkey,
    pub marinade_liq_pool_sol_leg: Pubkey,
    pub marinade_liq_pool_msol_leg: Pubkey,
    pub marinade_liq_pool_msol_leg_auth: Pubkey,
    pub marinade_reserve: Pubkey,
    pub msol_mint_authority: Pubkey,
}
impl<'me> From<&MarinadeDepositSolAccounts<'me, '_, '_, '_, '_, '_, '_, '_>>
    for MarinadeDepositSolKeys
{
    fn from(accounts: &MarinadeDepositSolAccounts<'me, '_, '_, '_, '_, '_, '_, '_>) -> Self {
        Self {
            marinade_program: *accounts.marinade_program.key,
            marinade_state: *accounts.marinade_state.key,
            marinade_liq_pool_sol_leg: *accounts.marinade_liq_pool_sol_leg.key,
            marinade_liq_pool_msol_leg: *accounts.marinade_liq_pool_msol_leg.key,
            marinade_liq_pool_msol_leg_auth: *accounts.marinade_liq_pool_msol_leg_auth.key,
            marinade_reserve: *accounts.marinade_reserve.key,
            msol_mint_authority: *accounts.msol_mint_authority.key,
        }
    }
}
impl From<&MarinadeDepositSolKeys> for [AccountMeta; MARINADE_DEPOSIT_SOL_IX_ACCOUNTS_LEN] {
    fn from(keys: &MarinadeDepositSolKeys) -> Self {
        [
            AccountMeta::new_readonly(keys.marinade_program, false),
            AccountMeta::new(keys.marinade_state, false),
            AccountMeta::new(keys.marinade_liq_pool_sol_leg, false),
            AccountMeta::new(keys.marinade_liq_pool_msol_leg, false),
            AccountMeta::new_readonly(keys.marinade_liq_pool_msol_leg_auth, false),
            AccountMeta::new(keys.marinade_reserve, false),
            AccountMeta::new_readonly(keys.msol_mint_authority, false),
        ]
    }
}
impl<'a> From<&MarinadeDepositSolAccounts<'_, 'a, 'a, 'a, 'a, 'a, 'a, 'a>>
    for [AccountInfo<'a>; MARINADE_DEPOSIT_SOL_IX_ACCOUNTS_LEN]
{
    fn from(accounts: &MarinadeDepositSolAccounts<'_, 'a, 'a, 'a, 'a, 'a, 'a, 'a>) -> Self {
        [
            accounts.marinade_program.clone(),
            accounts.marinade_state.clone(),
            accounts.marinade_liq_pool_sol_leg.clone(),
            accounts.marinade_liq_pool_msol_leg.clone(),
            accounts.marinade_liq_pool_msol_leg_auth.clone(),
            accounts.marinade_reserve.clone(),
            accounts.msol_mint_authority.clone(),
        ]
    }
}
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug)]
pub struct MarinadeDepositSolIxArgs {}
#[derive(Copy, Clone, Debug)]
pub struct MarinadeDepositSolIxData<'me>(pub &'me MarinadeDepositSolIxArgs);
pub const MARINADE_DEPOSIT_SOL_IX_DISCM: u8 = 2u8;
impl<'me> From<&'me MarinadeDepositSolIxArgs> for MarinadeDepositSolIxData<'me> {
    fn from(args: &'me MarinadeDepositSolIxArgs) -> Self {
        Self(args)
    }
}
impl BorshSerialize for MarinadeDepositSolIxData<'_> {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        writer.write_all(&[MARINADE_DEPOSIT_SOL_IX_DISCM])?;
        self.0.serialize(writer)
    }
}
pub fn marinade_deposit_sol_ix<
    K: Into<MarinadeDepositSolKeys>,
    A: Into<MarinadeDepositSolIxArgs>,
>(
    accounts: K,
    args: A,
) -> std::io::Result<Instruction> {
    let keys: MarinadeDepositSolKeys = accounts.into();
    let metas: [AccountMeta; MARINADE_DEPOSIT_SOL_IX_ACCOUNTS_LEN] = (&keys).into();
    let args_full: MarinadeDepositSolIxArgs = args.into();
    let data: MarinadeDepositSolIxData = (&args_full).into();
    Ok(Instruction {
        program_id: crate::ID,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn marinade_deposit_sol_invoke<'a, A: Into<MarinadeDepositSolIxArgs>>(
    accounts: &MarinadeDepositSolAccounts<'_, 'a, 'a, 'a, 'a, 'a, 'a, 'a>,
    args: A,
) -> ProgramResult {
    let ix = marinade_deposit_sol_ix(accounts, args)?;
    let account_info: [AccountInfo<'a>; MARINADE_DEPOSIT_SOL_IX_ACCOUNTS_LEN] = accounts.into();
    invoke(&ix, &account_info)
}
pub fn marinade_deposit_sol_invoke_signed<'a, A: Into<MarinadeDepositSolIxArgs>>(
    accounts: &MarinadeDepositSolAccounts<'_, 'a, 'a, 'a, 'a, 'a, 'a, 'a>,
    args: A,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let ix = marinade_deposit_sol_ix(accounts, args)?;
    let account_info: [AccountInfo<'a>; MARINADE_DEPOSIT_SOL_IX_ACCOUNTS_LEN] = accounts.into();
    invoke_signed(&ix, &account_info, seeds)
}
pub const SOCEAN_STAKE_POOL_DEPOSIT_SOL_IX_ACCOUNTS_LEN: usize = 6usize;
#[derive(Copy, Clone, Debug)]
pub struct SoceanStakePoolDepositSolAccounts<
    'me,
    'a0: 'me,
    'a1: 'me,
    'a2: 'me,
    'a3: 'me,
    'a4: 'me,
    'a5: 'me,
> {
    pub socean_stake_pool_program: &'me AccountInfo<'a0>,
    pub stake_pool: &'me AccountInfo<'a1>,
    pub stake_pool_withdraw_authority: &'me AccountInfo<'a2>,
    pub stake_pool_reserve_stake: &'me AccountInfo<'a3>,
    pub stake_pool_manager_fee: &'me AccountInfo<'a4>,
    pub clock: &'me AccountInfo<'a5>,
}
#[derive(Copy, Clone, Debug)]
pub struct SoceanStakePoolDepositSolKeys {
    pub socean_stake_pool_program: Pubkey,
    pub stake_pool: Pubkey,
    pub stake_pool_withdraw_authority: Pubkey,
    pub stake_pool_reserve_stake: Pubkey,
    pub stake_pool_manager_fee: Pubkey,
    pub clock: Pubkey,
}
impl<'me> From<&SoceanStakePoolDepositSolAccounts<'me, '_, '_, '_, '_, '_, '_>>
    for SoceanStakePoolDepositSolKeys
{
    fn from(accounts: &SoceanStakePoolDepositSolAccounts<'me, '_, '_, '_, '_, '_, '_>) -> Self {
        Self {
            socean_stake_pool_program: *accounts.socean_stake_pool_program.key,
            stake_pool: *accounts.stake_pool.key,
            stake_pool_withdraw_authority: *accounts.stake_pool_withdraw_authority.key,
            stake_pool_reserve_stake: *accounts.stake_pool_reserve_stake.key,
            stake_pool_manager_fee: *accounts.stake_pool_manager_fee.key,
            clock: *accounts.clock.key,
        }
    }
}
impl From<&SoceanStakePoolDepositSolKeys>
    for [AccountMeta; SOCEAN_STAKE_POOL_DEPOSIT_SOL_IX_ACCOUNTS_LEN]
{
    fn from(keys: &SoceanStakePoolDepositSolKeys) -> Self {
        [
            AccountMeta::new_readonly(keys.socean_stake_pool_program, false),
            AccountMeta::new(keys.stake_pool, false),
            AccountMeta::new_readonly(keys.stake_pool_withdraw_authority, false),
            AccountMeta::new(keys.stake_pool_reserve_stake, false),
            AccountMeta::new(keys.stake_pool_manager_fee, false),
            AccountMeta::new_readonly(keys.clock, false),
        ]
    }
}
impl<'a> From<&SoceanStakePoolDepositSolAccounts<'_, 'a, 'a, 'a, 'a, 'a, 'a>>
    for [AccountInfo<'a>; SOCEAN_STAKE_POOL_DEPOSIT_SOL_IX_ACCOUNTS_LEN]
{
    fn from(accounts: &SoceanStakePoolDepositSolAccounts<'_, 'a, 'a, 'a, 'a, 'a, 'a>) -> Self {
        [
            accounts.socean_stake_pool_program.clone(),
            accounts.stake_pool.clone(),
            accounts.stake_pool_withdraw_authority.clone(),
            accounts.stake_pool_reserve_stake.clone(),
            accounts.stake_pool_manager_fee.clone(),
            accounts.clock.clone(),
        ]
    }
}
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug)]
pub struct SoceanStakePoolDepositSolIxArgs {}
#[derive(Copy, Clone, Debug)]
pub struct SoceanStakePoolDepositSolIxData<'me>(pub &'me SoceanStakePoolDepositSolIxArgs);
pub const SOCEAN_STAKE_POOL_DEPOSIT_SOL_IX_DISCM: u8 = 3u8;
impl<'me> From<&'me SoceanStakePoolDepositSolIxArgs> for SoceanStakePoolDepositSolIxData<'me> {
    fn from(args: &'me SoceanStakePoolDepositSolIxArgs) -> Self {
        Self(args)
    }
}
impl BorshSerialize for SoceanStakePoolDepositSolIxData<'_> {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        writer.write_all(&[SOCEAN_STAKE_POOL_DEPOSIT_SOL_IX_DISCM])?;
        self.0.serialize(writer)
    }
}
pub fn socean_stake_pool_deposit_sol_ix<
    K: Into<SoceanStakePoolDepositSolKeys>,
    A: Into<SoceanStakePoolDepositSolIxArgs>,
>(
    accounts: K,
    args: A,
) -> std::io::Result<Instruction> {
    let keys: SoceanStakePoolDepositSolKeys = accounts.into();
    let metas: [AccountMeta; SOCEAN_STAKE_POOL_DEPOSIT_SOL_IX_ACCOUNTS_LEN] = (&keys).into();
    let args_full: SoceanStakePoolDepositSolIxArgs = args.into();
    let data: SoceanStakePoolDepositSolIxData = (&args_full).into();
    Ok(Instruction {
        program_id: crate::ID,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn socean_stake_pool_deposit_sol_invoke<'a, A: Into<SoceanStakePoolDepositSolIxArgs>>(
    accounts: &SoceanStakePoolDepositSolAccounts<'_, 'a, 'a, 'a, 'a, 'a, 'a>,
    args: A,
) -> ProgramResult {
    let ix = socean_stake_pool_deposit_sol_ix(accounts, args)?;
    let account_info: [AccountInfo<'a>; SOCEAN_STAKE_POOL_DEPOSIT_SOL_IX_ACCOUNTS_LEN] =
        accounts.into();
    invoke(&ix, &account_info)
}
pub fn socean_stake_pool_deposit_sol_invoke_signed<'a, A: Into<SoceanStakePoolDepositSolIxArgs>>(
    accounts: &SoceanStakePoolDepositSolAccounts<'_, 'a, 'a, 'a, 'a, 'a, 'a>,
    args: A,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let ix = socean_stake_pool_deposit_sol_ix(accounts, args)?;
    let account_info: [AccountInfo<'a>; SOCEAN_STAKE_POOL_DEPOSIT_SOL_IX_ACCOUNTS_LEN] =
        accounts.into();
    invoke_signed(&ix, &account_info, seeds)
}
pub const SPL_STAKE_POOL_DEPOSIT_SOL_IX_ACCOUNTS_LEN: usize = 5usize;
#[derive(Copy, Clone, Debug)]
pub struct SplStakePoolDepositSolAccounts<'me, 'a0: 'me, 'a1: 'me, 'a2: 'me, 'a3: 'me, 'a4: 'me> {
    pub spl_stake_pool_program: &'me AccountInfo<'a0>,
    pub stake_pool: &'me AccountInfo<'a1>,
    pub stake_pool_withdraw_authority: &'me AccountInfo<'a2>,
    pub stake_pool_reserve_stake: &'me AccountInfo<'a3>,
    pub stake_pool_manager_fee: &'me AccountInfo<'a4>,
}
#[derive(Copy, Clone, Debug)]
pub struct SplStakePoolDepositSolKeys {
    pub spl_stake_pool_program: Pubkey,
    pub stake_pool: Pubkey,
    pub stake_pool_withdraw_authority: Pubkey,
    pub stake_pool_reserve_stake: Pubkey,
    pub stake_pool_manager_fee: Pubkey,
}
impl<'me> From<&SplStakePoolDepositSolAccounts<'me, '_, '_, '_, '_, '_>>
    for SplStakePoolDepositSolKeys
{
    fn from(accounts: &SplStakePoolDepositSolAccounts<'me, '_, '_, '_, '_, '_>) -> Self {
        Self {
            spl_stake_pool_program: *accounts.spl_stake_pool_program.key,
            stake_pool: *accounts.stake_pool.key,
            stake_pool_withdraw_authority: *accounts.stake_pool_withdraw_authority.key,
            stake_pool_reserve_stake: *accounts.stake_pool_reserve_stake.key,
            stake_pool_manager_fee: *accounts.stake_pool_manager_fee.key,
        }
    }
}
impl From<&SplStakePoolDepositSolKeys>
    for [AccountMeta; SPL_STAKE_POOL_DEPOSIT_SOL_IX_ACCOUNTS_LEN]
{
    fn from(keys: &SplStakePoolDepositSolKeys) -> Self {
        [
            AccountMeta::new_readonly(keys.spl_stake_pool_program, false),
            AccountMeta::new(keys.stake_pool, false),
            AccountMeta::new_readonly(keys.stake_pool_withdraw_authority, false),
            AccountMeta::new(keys.stake_pool_reserve_stake, false),
            AccountMeta::new(keys.stake_pool_manager_fee, false),
        ]
    }
}
impl<'a> From<&SplStakePoolDepositSolAccounts<'_, 'a, 'a, 'a, 'a, 'a>>
    for [AccountInfo<'a>; SPL_STAKE_POOL_DEPOSIT_SOL_IX_ACCOUNTS_LEN]
{
    fn from(accounts: &SplStakePoolDepositSolAccounts<'_, 'a, 'a, 'a, 'a, 'a>) -> Self {
        [
            accounts.spl_stake_pool_program.clone(),
            accounts.stake_pool.clone(),
            accounts.stake_pool_withdraw_authority.clone(),
            accounts.stake_pool_reserve_stake.clone(),
            accounts.stake_pool_manager_fee.clone(),
        ]
    }
}
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug)]
pub struct SplStakePoolDepositSolIxArgs {}
#[derive(Copy, Clone, Debug)]
pub struct SplStakePoolDepositSolIxData<'me>(pub &'me SplStakePoolDepositSolIxArgs);
pub const SPL_STAKE_POOL_DEPOSIT_SOL_IX_DISCM: u8 = 4u8;
impl<'me> From<&'me SplStakePoolDepositSolIxArgs> for SplStakePoolDepositSolIxData<'me> {
    fn from(args: &'me SplStakePoolDepositSolIxArgs) -> Self {
        Self(args)
    }
}
impl BorshSerialize for SplStakePoolDepositSolIxData<'_> {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        writer.write_all(&[SPL_STAKE_POOL_DEPOSIT_SOL_IX_DISCM])?;
        self.0.serialize(writer)
    }
}
pub fn spl_stake_pool_deposit_sol_ix<
    K: Into<SplStakePoolDepositSolKeys>,
    A: Into<SplStakePoolDepositSolIxArgs>,
>(
    accounts: K,
    args: A,
) -> std::io::Result<Instruction> {
    let keys: SplStakePoolDepositSolKeys = accounts.into();
    let metas: [AccountMeta; SPL_STAKE_POOL_DEPOSIT_SOL_IX_ACCOUNTS_LEN] = (&keys).into();
    let args_full: SplStakePoolDepositSolIxArgs = args.into();
    let data: SplStakePoolDepositSolIxData = (&args_full).into();
    Ok(Instruction {
        program_id: crate::ID,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn spl_stake_pool_deposit_sol_invoke<'a, A: Into<SplStakePoolDepositSolIxArgs>>(
    accounts: &SplStakePoolDepositSolAccounts<'_, 'a, 'a, 'a, 'a, 'a>,
    args: A,
) -> ProgramResult {
    let ix = spl_stake_pool_deposit_sol_ix(accounts, args)?;
    let account_info: [AccountInfo<'a>; SPL_STAKE_POOL_DEPOSIT_SOL_IX_ACCOUNTS_LEN] =
        accounts.into();
    invoke(&ix, &account_info)
}
pub fn spl_stake_pool_deposit_sol_invoke_signed<'a, A: Into<SplStakePoolDepositSolIxArgs>>(
    accounts: &SplStakePoolDepositSolAccounts<'_, 'a, 'a, 'a, 'a, 'a>,
    args: A,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let ix = spl_stake_pool_deposit_sol_ix(accounts, args)?;
    let account_info: [AccountInfo<'a>; SPL_STAKE_POOL_DEPOSIT_SOL_IX_ACCOUNTS_LEN] =
        accounts.into();
    invoke_signed(&ix, &account_info, seeds)
}
