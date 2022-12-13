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
pub struct StakeWrappedSolEversolStakePoolAccounts<
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
    'a12: 'me,
    'a13: 'me,
    'a14: 'me,
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
    pub eversol_stake_pool_program: &'me AccountInfo<'a10>,
    pub stake_pool: &'me AccountInfo<'a11>,
    pub stake_pool_withdraw_authority: &'me AccountInfo<'a12>,
    pub stake_pool_reserve_stake: &'me AccountInfo<'a13>,
    pub stake_pool_manager_fee: &'me AccountInfo<'a14>,
}
#[derive(Copy, Clone, Debug)]
pub struct StakeWrappedSolEversolStakePoolKeys {
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
    pub eversol_stake_pool_program: Pubkey,
    pub stake_pool: Pubkey,
    pub stake_pool_withdraw_authority: Pubkey,
    pub stake_pool_reserve_stake: Pubkey,
    pub stake_pool_manager_fee: Pubkey,
}
impl<'me>
    From<
        &StakeWrappedSolEversolStakePoolAccounts<
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
            '_,
            '_,
            '_,
        >,
    > for StakeWrappedSolEversolStakePoolKeys
{
    fn from(
        accounts: &StakeWrappedSolEversolStakePoolAccounts<
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
            '_,
            '_,
            '_,
        >,
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
            eversol_stake_pool_program: *accounts.eversol_stake_pool_program.key,
            stake_pool: *accounts.stake_pool.key,
            stake_pool_withdraw_authority: *accounts.stake_pool_withdraw_authority.key,
            stake_pool_reserve_stake: *accounts.stake_pool_reserve_stake.key,
            stake_pool_manager_fee: *accounts.stake_pool_manager_fee.key,
        }
    }
}
impl From<&StakeWrappedSolEversolStakePoolKeys> for [AccountMeta; 15] {
    fn from(keys: &StakeWrappedSolEversolStakePoolKeys) -> Self {
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
            AccountMeta::new_readonly(keys.eversol_stake_pool_program, false),
            AccountMeta::new(keys.stake_pool, false),
            AccountMeta::new_readonly(keys.stake_pool_withdraw_authority, false),
            AccountMeta::new(keys.stake_pool_reserve_stake, false),
            AccountMeta::new(keys.stake_pool_manager_fee, false),
        ]
    }
}
impl<'a>
    From<
        &StakeWrappedSolEversolStakePoolAccounts<
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
            'a,
            'a,
            'a,
        >,
    > for [AccountInfo<'a>; 15]
{
    fn from(
        accounts: &StakeWrappedSolEversolStakePoolAccounts<
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
            'a,
            'a,
            'a,
        >,
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
            accounts.eversol_stake_pool_program.clone(),
            accounts.stake_pool.clone(),
            accounts.stake_pool_withdraw_authority.clone(),
            accounts.stake_pool_reserve_stake.clone(),
            accounts.stake_pool_manager_fee.clone(),
        ]
    }
}
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug)]
pub struct StakeWrappedSolEversolStakePoolIxArgs {
    pub stake_wrapped_sol_args: StakeWrappedSolArgs,
}
#[derive(Copy, Clone, Debug)]
pub struct StakeWrappedSolEversolStakePoolIxData<'me>(
    pub &'me StakeWrappedSolEversolStakePoolIxArgs,
);
pub const STAKE_WRAPPED_SOL_EVERSOL_STAKE_POOL_IX_DISCM: u8 = 0u8;
impl<'me> From<&'me StakeWrappedSolEversolStakePoolIxArgs>
    for StakeWrappedSolEversolStakePoolIxData<'me>
{
    fn from(args: &'me StakeWrappedSolEversolStakePoolIxArgs) -> Self {
        Self(args)
    }
}
impl BorshSerialize for StakeWrappedSolEversolStakePoolIxData<'_> {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        writer.write_all(&[STAKE_WRAPPED_SOL_EVERSOL_STAKE_POOL_IX_DISCM])?;
        self.0.serialize(writer)
    }
}
pub fn stake_wrapped_sol_eversol_stake_pool_ix<
    K: Into<StakeWrappedSolEversolStakePoolKeys>,
    A: Into<StakeWrappedSolEversolStakePoolIxArgs>,
>(
    accounts: K,
    args: A,
) -> std::io::Result<Instruction> {
    let keys: StakeWrappedSolEversolStakePoolKeys = accounts.into();
    let metas: [AccountMeta; 15] = (&keys).into();
    let args_full: StakeWrappedSolEversolStakePoolIxArgs = args.into();
    let data: StakeWrappedSolEversolStakePoolIxData = (&args_full).into();
    Ok(Instruction {
        program_id: crate::ID,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn stake_wrapped_sol_eversol_stake_pool_invoke<
    'a,
    A: Into<StakeWrappedSolEversolStakePoolIxArgs>,
>(
    accounts: &StakeWrappedSolEversolStakePoolAccounts<
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
        'a,
        'a,
        'a,
    >,
    args: A,
) -> ProgramResult {
    let ix = stake_wrapped_sol_eversol_stake_pool_ix(accounts, args)?;
    let account_info: [AccountInfo<'a>; 15] = accounts.into();
    invoke(&ix, &account_info)
}
pub fn stake_wrapped_sol_eversol_stake_pool_invoke_signed<
    'a,
    A: Into<StakeWrappedSolEversolStakePoolIxArgs>,
>(
    accounts: &StakeWrappedSolEversolStakePoolAccounts<
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
        'a,
        'a,
        'a,
    >,
    args: A,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let ix = stake_wrapped_sol_eversol_stake_pool_ix(accounts, args)?;
    let account_info: [AccountInfo<'a>; 15] = accounts.into();
    invoke_signed(&ix, &account_info, seeds)
}
#[derive(Copy, Clone, Debug)]
pub struct StakeWrappedSolLidoAccounts<
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
    'a12: 'me,
    'a13: 'me,
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
    pub lido_program: &'me AccountInfo<'a10>,
    pub solido: &'me AccountInfo<'a11>,
    pub lido_reserve: &'me AccountInfo<'a12>,
    pub stsol_mint_authority: &'me AccountInfo<'a13>,
}
#[derive(Copy, Clone, Debug)]
pub struct StakeWrappedSolLidoKeys {
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
    pub lido_program: Pubkey,
    pub solido: Pubkey,
    pub lido_reserve: Pubkey,
    pub stsol_mint_authority: Pubkey,
}
impl<'me>
    From<&StakeWrappedSolLidoAccounts<'me, '_, '_, '_, '_, '_, '_, '_, '_, '_, '_, '_, '_, '_, '_>>
    for StakeWrappedSolLidoKeys
{
    fn from(
        accounts: &StakeWrappedSolLidoAccounts<
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
            '_,
            '_,
        >,
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
            lido_program: *accounts.lido_program.key,
            solido: *accounts.solido.key,
            lido_reserve: *accounts.lido_reserve.key,
            stsol_mint_authority: *accounts.stsol_mint_authority.key,
        }
    }
}
impl From<&StakeWrappedSolLidoKeys> for [AccountMeta; 14] {
    fn from(keys: &StakeWrappedSolLidoKeys) -> Self {
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
            AccountMeta::new_readonly(keys.lido_program, false),
            AccountMeta::new(keys.solido, false),
            AccountMeta::new(keys.lido_reserve, false),
            AccountMeta::new_readonly(keys.stsol_mint_authority, false),
        ]
    }
}
impl<'a>
    From<&StakeWrappedSolLidoAccounts<'_, 'a, 'a, 'a, 'a, 'a, 'a, 'a, 'a, 'a, 'a, 'a, 'a, 'a, 'a>>
    for [AccountInfo<'a>; 14]
{
    fn from(
        accounts: &StakeWrappedSolLidoAccounts<
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
            'a,
            'a,
        >,
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
            accounts.lido_program.clone(),
            accounts.solido.clone(),
            accounts.lido_reserve.clone(),
            accounts.stsol_mint_authority.clone(),
        ]
    }
}
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug)]
pub struct StakeWrappedSolLidoIxArgs {
    pub stake_wrapped_sol_args: StakeWrappedSolArgs,
}
#[derive(Copy, Clone, Debug)]
pub struct StakeWrappedSolLidoIxData<'me>(pub &'me StakeWrappedSolLidoIxArgs);
pub const STAKE_WRAPPED_SOL_LIDO_IX_DISCM: u8 = 0u8;
impl<'me> From<&'me StakeWrappedSolLidoIxArgs> for StakeWrappedSolLidoIxData<'me> {
    fn from(args: &'me StakeWrappedSolLidoIxArgs) -> Self {
        Self(args)
    }
}
impl BorshSerialize for StakeWrappedSolLidoIxData<'_> {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        writer.write_all(&[STAKE_WRAPPED_SOL_LIDO_IX_DISCM])?;
        self.0.serialize(writer)
    }
}
pub fn stake_wrapped_sol_lido_ix<
    K: Into<StakeWrappedSolLidoKeys>,
    A: Into<StakeWrappedSolLidoIxArgs>,
>(
    accounts: K,
    args: A,
) -> std::io::Result<Instruction> {
    let keys: StakeWrappedSolLidoKeys = accounts.into();
    let metas: [AccountMeta; 14] = (&keys).into();
    let args_full: StakeWrappedSolLidoIxArgs = args.into();
    let data: StakeWrappedSolLidoIxData = (&args_full).into();
    Ok(Instruction {
        program_id: crate::ID,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn stake_wrapped_sol_lido_invoke<'a, A: Into<StakeWrappedSolLidoIxArgs>>(
    accounts: &StakeWrappedSolLidoAccounts<
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
        'a,
        'a,
    >,
    args: A,
) -> ProgramResult {
    let ix = stake_wrapped_sol_lido_ix(accounts, args)?;
    let account_info: [AccountInfo<'a>; 14] = accounts.into();
    invoke(&ix, &account_info)
}
pub fn stake_wrapped_sol_lido_invoke_signed<'a, A: Into<StakeWrappedSolLidoIxArgs>>(
    accounts: &StakeWrappedSolLidoAccounts<
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
        'a,
        'a,
    >,
    args: A,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let ix = stake_wrapped_sol_lido_ix(accounts, args)?;
    let account_info: [AccountInfo<'a>; 14] = accounts.into();
    invoke_signed(&ix, &account_info, seeds)
}
#[derive(Copy, Clone, Debug)]
pub struct StakeWrappedSolMarinadeAccounts<
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
    'a12: 'me,
    'a13: 'me,
    'a14: 'me,
    'a15: 'me,
    'a16: 'me,
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
    pub marinade_program: &'me AccountInfo<'a10>,
    pub marinade_state: &'me AccountInfo<'a11>,
    pub marinade_liq_pool_sol_leg: &'me AccountInfo<'a12>,
    pub marinade_liq_pool_msol_leg: &'me AccountInfo<'a13>,
    pub marinade_liq_pool_msol_leg_auth: &'me AccountInfo<'a14>,
    pub marinade_reserve: &'me AccountInfo<'a15>,
    pub msol_mint_authority: &'me AccountInfo<'a16>,
}
#[derive(Copy, Clone, Debug)]
pub struct StakeWrappedSolMarinadeKeys {
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
    pub marinade_program: Pubkey,
    pub marinade_state: Pubkey,
    pub marinade_liq_pool_sol_leg: Pubkey,
    pub marinade_liq_pool_msol_leg: Pubkey,
    pub marinade_liq_pool_msol_leg_auth: Pubkey,
    pub marinade_reserve: Pubkey,
    pub msol_mint_authority: Pubkey,
}
impl<'me>
    From<
        &StakeWrappedSolMarinadeAccounts<
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
            '_,
            '_,
            '_,
            '_,
            '_,
        >,
    > for StakeWrappedSolMarinadeKeys
{
    fn from(
        accounts: &StakeWrappedSolMarinadeAccounts<
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
            '_,
            '_,
            '_,
            '_,
            '_,
        >,
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
impl From<&StakeWrappedSolMarinadeKeys> for [AccountMeta; 17] {
    fn from(keys: &StakeWrappedSolMarinadeKeys) -> Self {
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
impl<'a>
    From<
        &StakeWrappedSolMarinadeAccounts<
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
            'a,
            'a,
            'a,
            'a,
            'a,
        >,
    > for [AccountInfo<'a>; 17]
{
    fn from(
        accounts: &StakeWrappedSolMarinadeAccounts<
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
            'a,
            'a,
            'a,
            'a,
            'a,
        >,
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
pub struct StakeWrappedSolMarinadeIxArgs {
    pub stake_wrapped_sol_args: StakeWrappedSolArgs,
}
#[derive(Copy, Clone, Debug)]
pub struct StakeWrappedSolMarinadeIxData<'me>(pub &'me StakeWrappedSolMarinadeIxArgs);
pub const STAKE_WRAPPED_SOL_MARINADE_IX_DISCM: u8 = 0u8;
impl<'me> From<&'me StakeWrappedSolMarinadeIxArgs> for StakeWrappedSolMarinadeIxData<'me> {
    fn from(args: &'me StakeWrappedSolMarinadeIxArgs) -> Self {
        Self(args)
    }
}
impl BorshSerialize for StakeWrappedSolMarinadeIxData<'_> {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        writer.write_all(&[STAKE_WRAPPED_SOL_MARINADE_IX_DISCM])?;
        self.0.serialize(writer)
    }
}
pub fn stake_wrapped_sol_marinade_ix<
    K: Into<StakeWrappedSolMarinadeKeys>,
    A: Into<StakeWrappedSolMarinadeIxArgs>,
>(
    accounts: K,
    args: A,
) -> std::io::Result<Instruction> {
    let keys: StakeWrappedSolMarinadeKeys = accounts.into();
    let metas: [AccountMeta; 17] = (&keys).into();
    let args_full: StakeWrappedSolMarinadeIxArgs = args.into();
    let data: StakeWrappedSolMarinadeIxData = (&args_full).into();
    Ok(Instruction {
        program_id: crate::ID,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn stake_wrapped_sol_marinade_invoke<'a, A: Into<StakeWrappedSolMarinadeIxArgs>>(
    accounts: &StakeWrappedSolMarinadeAccounts<
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
        'a,
        'a,
        'a,
        'a,
        'a,
    >,
    args: A,
) -> ProgramResult {
    let ix = stake_wrapped_sol_marinade_ix(accounts, args)?;
    let account_info: [AccountInfo<'a>; 17] = accounts.into();
    invoke(&ix, &account_info)
}
pub fn stake_wrapped_sol_marinade_invoke_signed<'a, A: Into<StakeWrappedSolMarinadeIxArgs>>(
    accounts: &StakeWrappedSolMarinadeAccounts<
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
        'a,
        'a,
        'a,
        'a,
        'a,
    >,
    args: A,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let ix = stake_wrapped_sol_marinade_ix(accounts, args)?;
    let account_info: [AccountInfo<'a>; 17] = accounts.into();
    invoke_signed(&ix, &account_info, seeds)
}
#[derive(Copy, Clone, Debug)]
pub struct StakeWrappedSolSoceanStakePoolAccounts<
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
    'a12: 'me,
    'a13: 'me,
    'a14: 'me,
    'a15: 'me,
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
    pub socean_stake_pool_program: &'me AccountInfo<'a10>,
    pub stake_pool: &'me AccountInfo<'a11>,
    pub stake_pool_withdraw_authority: &'me AccountInfo<'a12>,
    pub stake_pool_reserve_stake: &'me AccountInfo<'a13>,
    pub stake_pool_manager_fee: &'me AccountInfo<'a14>,
    pub clock: &'me AccountInfo<'a15>,
}
#[derive(Copy, Clone, Debug)]
pub struct StakeWrappedSolSoceanStakePoolKeys {
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
    pub socean_stake_pool_program: Pubkey,
    pub stake_pool: Pubkey,
    pub stake_pool_withdraw_authority: Pubkey,
    pub stake_pool_reserve_stake: Pubkey,
    pub stake_pool_manager_fee: Pubkey,
    pub clock: Pubkey,
}
impl<'me>
    From<
        &StakeWrappedSolSoceanStakePoolAccounts<
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
            '_,
            '_,
            '_,
            '_,
        >,
    > for StakeWrappedSolSoceanStakePoolKeys
{
    fn from(
        accounts: &StakeWrappedSolSoceanStakePoolAccounts<
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
            '_,
            '_,
            '_,
            '_,
        >,
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
            socean_stake_pool_program: *accounts.socean_stake_pool_program.key,
            stake_pool: *accounts.stake_pool.key,
            stake_pool_withdraw_authority: *accounts.stake_pool_withdraw_authority.key,
            stake_pool_reserve_stake: *accounts.stake_pool_reserve_stake.key,
            stake_pool_manager_fee: *accounts.stake_pool_manager_fee.key,
            clock: *accounts.clock.key,
        }
    }
}
impl From<&StakeWrappedSolSoceanStakePoolKeys> for [AccountMeta; 16] {
    fn from(keys: &StakeWrappedSolSoceanStakePoolKeys) -> Self {
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
            AccountMeta::new_readonly(keys.socean_stake_pool_program, false),
            AccountMeta::new(keys.stake_pool, false),
            AccountMeta::new_readonly(keys.stake_pool_withdraw_authority, false),
            AccountMeta::new(keys.stake_pool_reserve_stake, false),
            AccountMeta::new(keys.stake_pool_manager_fee, false),
            AccountMeta::new_readonly(keys.clock, false),
        ]
    }
}
impl<'a>
    From<
        &StakeWrappedSolSoceanStakePoolAccounts<
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
            'a,
            'a,
            'a,
            'a,
        >,
    > for [AccountInfo<'a>; 16]
{
    fn from(
        accounts: &StakeWrappedSolSoceanStakePoolAccounts<
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
            'a,
            'a,
            'a,
            'a,
        >,
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
pub struct StakeWrappedSolSoceanStakePoolIxArgs {
    pub stake_wrapped_sol_args: StakeWrappedSolArgs,
}
#[derive(Copy, Clone, Debug)]
pub struct StakeWrappedSolSoceanStakePoolIxData<'me>(pub &'me StakeWrappedSolSoceanStakePoolIxArgs);
pub const STAKE_WRAPPED_SOL_SOCEAN_STAKE_POOL_IX_DISCM: u8 = 0u8;
impl<'me> From<&'me StakeWrappedSolSoceanStakePoolIxArgs>
    for StakeWrappedSolSoceanStakePoolIxData<'me>
{
    fn from(args: &'me StakeWrappedSolSoceanStakePoolIxArgs) -> Self {
        Self(args)
    }
}
impl BorshSerialize for StakeWrappedSolSoceanStakePoolIxData<'_> {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        writer.write_all(&[STAKE_WRAPPED_SOL_SOCEAN_STAKE_POOL_IX_DISCM])?;
        self.0.serialize(writer)
    }
}
pub fn stake_wrapped_sol_socean_stake_pool_ix<
    K: Into<StakeWrappedSolSoceanStakePoolKeys>,
    A: Into<StakeWrappedSolSoceanStakePoolIxArgs>,
>(
    accounts: K,
    args: A,
) -> std::io::Result<Instruction> {
    let keys: StakeWrappedSolSoceanStakePoolKeys = accounts.into();
    let metas: [AccountMeta; 16] = (&keys).into();
    let args_full: StakeWrappedSolSoceanStakePoolIxArgs = args.into();
    let data: StakeWrappedSolSoceanStakePoolIxData = (&args_full).into();
    Ok(Instruction {
        program_id: crate::ID,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn stake_wrapped_sol_socean_stake_pool_invoke<
    'a,
    A: Into<StakeWrappedSolSoceanStakePoolIxArgs>,
>(
    accounts: &StakeWrappedSolSoceanStakePoolAccounts<
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
        'a,
        'a,
        'a,
        'a,
    >,
    args: A,
) -> ProgramResult {
    let ix = stake_wrapped_sol_socean_stake_pool_ix(accounts, args)?;
    let account_info: [AccountInfo<'a>; 16] = accounts.into();
    invoke(&ix, &account_info)
}
pub fn stake_wrapped_sol_socean_stake_pool_invoke_signed<
    'a,
    A: Into<StakeWrappedSolSoceanStakePoolIxArgs>,
>(
    accounts: &StakeWrappedSolSoceanStakePoolAccounts<
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
        'a,
        'a,
        'a,
        'a,
    >,
    args: A,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let ix = stake_wrapped_sol_socean_stake_pool_ix(accounts, args)?;
    let account_info: [AccountInfo<'a>; 16] = accounts.into();
    invoke_signed(&ix, &account_info, seeds)
}
#[derive(Copy, Clone, Debug)]
pub struct StakeWrappedSolSplStakePoolAccounts<
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
    'a12: 'me,
    'a13: 'me,
    'a14: 'me,
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
    pub spl_stake_pool_program: &'me AccountInfo<'a10>,
    pub stake_pool: &'me AccountInfo<'a11>,
    pub stake_pool_withdraw_authority: &'me AccountInfo<'a12>,
    pub stake_pool_reserve_stake: &'me AccountInfo<'a13>,
    pub stake_pool_manager_fee: &'me AccountInfo<'a14>,
}
#[derive(Copy, Clone, Debug)]
pub struct StakeWrappedSolSplStakePoolKeys {
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
    pub spl_stake_pool_program: Pubkey,
    pub stake_pool: Pubkey,
    pub stake_pool_withdraw_authority: Pubkey,
    pub stake_pool_reserve_stake: Pubkey,
    pub stake_pool_manager_fee: Pubkey,
}
impl<'me>
    From<
        &StakeWrappedSolSplStakePoolAccounts<
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
            '_,
            '_,
            '_,
        >,
    > for StakeWrappedSolSplStakePoolKeys
{
    fn from(
        accounts: &StakeWrappedSolSplStakePoolAccounts<
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
            '_,
            '_,
            '_,
        >,
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
            spl_stake_pool_program: *accounts.spl_stake_pool_program.key,
            stake_pool: *accounts.stake_pool.key,
            stake_pool_withdraw_authority: *accounts.stake_pool_withdraw_authority.key,
            stake_pool_reserve_stake: *accounts.stake_pool_reserve_stake.key,
            stake_pool_manager_fee: *accounts.stake_pool_manager_fee.key,
        }
    }
}
impl From<&StakeWrappedSolSplStakePoolKeys> for [AccountMeta; 15] {
    fn from(keys: &StakeWrappedSolSplStakePoolKeys) -> Self {
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
            AccountMeta::new_readonly(keys.spl_stake_pool_program, false),
            AccountMeta::new(keys.stake_pool, false),
            AccountMeta::new_readonly(keys.stake_pool_withdraw_authority, false),
            AccountMeta::new(keys.stake_pool_reserve_stake, false),
            AccountMeta::new(keys.stake_pool_manager_fee, false),
        ]
    }
}
impl<'a>
    From<
        &StakeWrappedSolSplStakePoolAccounts<
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
            'a,
            'a,
            'a,
        >,
    > for [AccountInfo<'a>; 15]
{
    fn from(
        accounts: &StakeWrappedSolSplStakePoolAccounts<
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
            'a,
            'a,
            'a,
        >,
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
            accounts.spl_stake_pool_program.clone(),
            accounts.stake_pool.clone(),
            accounts.stake_pool_withdraw_authority.clone(),
            accounts.stake_pool_reserve_stake.clone(),
            accounts.stake_pool_manager_fee.clone(),
        ]
    }
}
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug)]
pub struct StakeWrappedSolSplStakePoolIxArgs {
    pub stake_wrapped_sol_args: StakeWrappedSolArgs,
}
#[derive(Copy, Clone, Debug)]
pub struct StakeWrappedSolSplStakePoolIxData<'me>(pub &'me StakeWrappedSolSplStakePoolIxArgs);
pub const STAKE_WRAPPED_SOL_SPL_STAKE_POOL_IX_DISCM: u8 = 0u8;
impl<'me> From<&'me StakeWrappedSolSplStakePoolIxArgs> for StakeWrappedSolSplStakePoolIxData<'me> {
    fn from(args: &'me StakeWrappedSolSplStakePoolIxArgs) -> Self {
        Self(args)
    }
}
impl BorshSerialize for StakeWrappedSolSplStakePoolIxData<'_> {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        writer.write_all(&[STAKE_WRAPPED_SOL_SPL_STAKE_POOL_IX_DISCM])?;
        self.0.serialize(writer)
    }
}
pub fn stake_wrapped_sol_spl_stake_pool_ix<
    K: Into<StakeWrappedSolSplStakePoolKeys>,
    A: Into<StakeWrappedSolSplStakePoolIxArgs>,
>(
    accounts: K,
    args: A,
) -> std::io::Result<Instruction> {
    let keys: StakeWrappedSolSplStakePoolKeys = accounts.into();
    let metas: [AccountMeta; 15] = (&keys).into();
    let args_full: StakeWrappedSolSplStakePoolIxArgs = args.into();
    let data: StakeWrappedSolSplStakePoolIxData = (&args_full).into();
    Ok(Instruction {
        program_id: crate::ID,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn stake_wrapped_sol_spl_stake_pool_invoke<'a, A: Into<StakeWrappedSolSplStakePoolIxArgs>>(
    accounts: &StakeWrappedSolSplStakePoolAccounts<
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
        'a,
        'a,
        'a,
    >,
    args: A,
) -> ProgramResult {
    let ix = stake_wrapped_sol_spl_stake_pool_ix(accounts, args)?;
    let account_info: [AccountInfo<'a>; 15] = accounts.into();
    invoke(&ix, &account_info)
}
pub fn stake_wrapped_sol_spl_stake_pool_invoke_signed<
    'a,
    A: Into<StakeWrappedSolSplStakePoolIxArgs>,
>(
    accounts: &StakeWrappedSolSplStakePoolAccounts<
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
        'a,
        'a,
        'a,
    >,
    args: A,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let ix = stake_wrapped_sol_spl_stake_pool_ix(accounts, args)?;
    let account_info: [AccountInfo<'a>; 15] = accounts.into();
    invoke_signed(&ix, &account_info, seeds)
}
#[derive(Copy, Clone, Debug)]
pub struct SwapViaStakeEversolStakePoolSoceanStakePoolAccounts<
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
    'a12: 'me,
    'a13: 'me,
    'a14: 'me,
    'a15: 'me,
    'a16: 'me,
    'a17: 'me,
    'a18: 'me,
    'a19: 'me,
    'a20: 'me,
    'a21: 'me,
    'a22: 'me,
    'a23: 'me,
    'a24: 'me,
    'a25: 'me,
    'a26: 'me,
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
    pub eversol_stake_pool_program: &'me AccountInfo<'a8>,
    pub withdraw_stake_spl_stake_pool: &'me AccountInfo<'a9>,
    pub withdraw_stake_validator_list: &'me AccountInfo<'a10>,
    pub withdraw_stake_withdraw_authority: &'me AccountInfo<'a11>,
    pub withdraw_stake_stake_to_split: &'me AccountInfo<'a12>,
    pub withdraw_stake_manager_fee: &'me AccountInfo<'a13>,
    pub clock: &'me AccountInfo<'a14>,
    pub token_program: &'me AccountInfo<'a15>,
    pub stake_program: &'me AccountInfo<'a16>,
    pub system_program: &'me AccountInfo<'a17>,
    pub socean_stake_pool_program: &'me AccountInfo<'a18>,
    pub deposit_stake_spl_stake_pool: &'me AccountInfo<'a19>,
    pub deposit_stake_validator_list: &'me AccountInfo<'a20>,
    pub deposit_stake_deposit_authority: &'me AccountInfo<'a21>,
    pub deposit_stake_withdraw_authority: &'me AccountInfo<'a22>,
    pub deposit_stake_validator_stake: &'me AccountInfo<'a23>,
    pub deposit_stake_reserve_stake: &'me AccountInfo<'a24>,
    pub deposit_stake_manager_fee: &'me AccountInfo<'a25>,
    pub stake_history: &'me AccountInfo<'a26>,
}
#[derive(Copy, Clone, Debug)]
pub struct SwapViaStakeEversolStakePoolSoceanStakePoolKeys {
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
    pub socean_stake_pool_program: Pubkey,
    pub deposit_stake_spl_stake_pool: Pubkey,
    pub deposit_stake_validator_list: Pubkey,
    pub deposit_stake_deposit_authority: Pubkey,
    pub deposit_stake_withdraw_authority: Pubkey,
    pub deposit_stake_validator_stake: Pubkey,
    pub deposit_stake_reserve_stake: Pubkey,
    pub deposit_stake_manager_fee: Pubkey,
    pub stake_history: Pubkey,
}
impl<'me>
    From<
        &SwapViaStakeEversolStakePoolSoceanStakePoolAccounts<
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
            '_,
            '_,
            '_,
        >,
    > for SwapViaStakeEversolStakePoolSoceanStakePoolKeys
{
    fn from(
        accounts: &SwapViaStakeEversolStakePoolSoceanStakePoolAccounts<
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
            '_,
            '_,
            '_,
        >,
    ) -> Self {
        Self {
            payer: *accounts.payer.key,
            user: *accounts.user.key,
            src_token_from: *accounts.src_token_from.key,
            dest_token_to: *accounts.dest_token_to.key,
            bridge_stake: *accounts.bridge_stake.key,
            dest_token_fee_token_account: *accounts.dest_token_fee_token_account.key,
            src_token_mint: *accounts.src_token_mint.key,
            dest_token_mint: *accounts.dest_token_mint.key,
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
            socean_stake_pool_program: *accounts.socean_stake_pool_program.key,
            deposit_stake_spl_stake_pool: *accounts.deposit_stake_spl_stake_pool.key,
            deposit_stake_validator_list: *accounts.deposit_stake_validator_list.key,
            deposit_stake_deposit_authority: *accounts.deposit_stake_deposit_authority.key,
            deposit_stake_withdraw_authority: *accounts.deposit_stake_withdraw_authority.key,
            deposit_stake_validator_stake: *accounts.deposit_stake_validator_stake.key,
            deposit_stake_reserve_stake: *accounts.deposit_stake_reserve_stake.key,
            deposit_stake_manager_fee: *accounts.deposit_stake_manager_fee.key,
            stake_history: *accounts.stake_history.key,
        }
    }
}
impl From<&SwapViaStakeEversolStakePoolSoceanStakePoolKeys> for [AccountMeta; 30] {
    fn from(keys: &SwapViaStakeEversolStakePoolSoceanStakePoolKeys) -> Self {
        [
            AccountMeta::new(keys.payer, true),
            AccountMeta::new_readonly(keys.user, true),
            AccountMeta::new(keys.src_token_from, false),
            AccountMeta::new(keys.dest_token_to, false),
            AccountMeta::new(keys.bridge_stake, false),
            AccountMeta::new(keys.dest_token_fee_token_account, false),
            AccountMeta::new(keys.src_token_mint, false),
            AccountMeta::new(keys.dest_token_mint, false),
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
    From<
        &SwapViaStakeEversolStakePoolSoceanStakePoolAccounts<
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
            'a,
            'a,
            'a,
        >,
    > for [AccountInfo<'a>; 30]
{
    fn from(
        accounts: &SwapViaStakeEversolStakePoolSoceanStakePoolAccounts<
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
            'a,
            'a,
            'a,
        >,
    ) -> Self {
        [
            accounts.payer.clone(),
            accounts.user.clone(),
            accounts.src_token_from.clone(),
            accounts.dest_token_to.clone(),
            accounts.bridge_stake.clone(),
            accounts.dest_token_fee_token_account.clone(),
            accounts.src_token_mint.clone(),
            accounts.dest_token_mint.clone(),
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
pub struct SwapViaStakeEversolStakePoolSoceanStakePoolIxArgs {
    pub swap_via_stake_args: SwapViaStakeArgs,
}
#[derive(Copy, Clone, Debug)]
pub struct SwapViaStakeEversolStakePoolSoceanStakePoolIxData<'me>(
    pub &'me SwapViaStakeEversolStakePoolSoceanStakePoolIxArgs,
);
pub const SWAP_VIA_STAKE_EVERSOL_STAKE_POOL_SOCEAN_STAKE_POOL_IX_DISCM: u8 = 1u8;
impl<'me> From<&'me SwapViaStakeEversolStakePoolSoceanStakePoolIxArgs>
    for SwapViaStakeEversolStakePoolSoceanStakePoolIxData<'me>
{
    fn from(args: &'me SwapViaStakeEversolStakePoolSoceanStakePoolIxArgs) -> Self {
        Self(args)
    }
}
impl BorshSerialize for SwapViaStakeEversolStakePoolSoceanStakePoolIxData<'_> {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        writer.write_all(&[SWAP_VIA_STAKE_EVERSOL_STAKE_POOL_SOCEAN_STAKE_POOL_IX_DISCM])?;
        self.0.serialize(writer)
    }
}
pub fn swap_via_stake_eversol_stake_pool_socean_stake_pool_ix<
    K: Into<SwapViaStakeEversolStakePoolSoceanStakePoolKeys>,
    A: Into<SwapViaStakeEversolStakePoolSoceanStakePoolIxArgs>,
>(
    accounts: K,
    args: A,
) -> std::io::Result<Instruction> {
    let keys: SwapViaStakeEversolStakePoolSoceanStakePoolKeys = accounts.into();
    let metas: [AccountMeta; 30] = (&keys).into();
    let args_full: SwapViaStakeEversolStakePoolSoceanStakePoolIxArgs = args.into();
    let data: SwapViaStakeEversolStakePoolSoceanStakePoolIxData = (&args_full).into();
    Ok(Instruction {
        program_id: crate::ID,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn swap_via_stake_eversol_stake_pool_socean_stake_pool_invoke<
    'a,
    A: Into<SwapViaStakeEversolStakePoolSoceanStakePoolIxArgs>,
>(
    accounts: &SwapViaStakeEversolStakePoolSoceanStakePoolAccounts<
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
        'a,
        'a,
        'a,
    >,
    args: A,
) -> ProgramResult {
    let ix = swap_via_stake_eversol_stake_pool_socean_stake_pool_ix(accounts, args)?;
    let account_info: [AccountInfo<'a>; 30] = accounts.into();
    invoke(&ix, &account_info)
}
pub fn swap_via_stake_eversol_stake_pool_socean_stake_pool_invoke_signed<
    'a,
    A: Into<SwapViaStakeEversolStakePoolSoceanStakePoolIxArgs>,
>(
    accounts: &SwapViaStakeEversolStakePoolSoceanStakePoolAccounts<
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
        'a,
        'a,
        'a,
    >,
    args: A,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let ix = swap_via_stake_eversol_stake_pool_socean_stake_pool_ix(accounts, args)?;
    let account_info: [AccountInfo<'a>; 30] = accounts.into();
    invoke_signed(&ix, &account_info, seeds)
}
#[derive(Copy, Clone, Debug)]
pub struct SwapViaStakeEversolStakePoolSplStakePoolAccounts<
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
    'a12: 'me,
    'a13: 'me,
    'a14: 'me,
    'a15: 'me,
    'a16: 'me,
    'a17: 'me,
    'a18: 'me,
    'a19: 'me,
    'a20: 'me,
    'a21: 'me,
    'a22: 'me,
    'a23: 'me,
    'a24: 'me,
    'a25: 'me,
    'a26: 'me,
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
    pub eversol_stake_pool_program: &'me AccountInfo<'a8>,
    pub withdraw_stake_spl_stake_pool: &'me AccountInfo<'a9>,
    pub withdraw_stake_validator_list: &'me AccountInfo<'a10>,
    pub withdraw_stake_withdraw_authority: &'me AccountInfo<'a11>,
    pub withdraw_stake_stake_to_split: &'me AccountInfo<'a12>,
    pub withdraw_stake_manager_fee: &'me AccountInfo<'a13>,
    pub clock: &'me AccountInfo<'a14>,
    pub token_program: &'me AccountInfo<'a15>,
    pub stake_program: &'me AccountInfo<'a16>,
    pub system_program: &'me AccountInfo<'a17>,
    pub spl_stake_pool_program: &'me AccountInfo<'a18>,
    pub deposit_stake_spl_stake_pool: &'me AccountInfo<'a19>,
    pub deposit_stake_validator_list: &'me AccountInfo<'a20>,
    pub deposit_stake_deposit_authority: &'me AccountInfo<'a21>,
    pub deposit_stake_withdraw_authority: &'me AccountInfo<'a22>,
    pub deposit_stake_validator_stake: &'me AccountInfo<'a23>,
    pub deposit_stake_reserve_stake: &'me AccountInfo<'a24>,
    pub deposit_stake_manager_fee: &'me AccountInfo<'a25>,
    pub stake_history: &'me AccountInfo<'a26>,
}
#[derive(Copy, Clone, Debug)]
pub struct SwapViaStakeEversolStakePoolSplStakePoolKeys {
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
    pub spl_stake_pool_program: Pubkey,
    pub deposit_stake_spl_stake_pool: Pubkey,
    pub deposit_stake_validator_list: Pubkey,
    pub deposit_stake_deposit_authority: Pubkey,
    pub deposit_stake_withdraw_authority: Pubkey,
    pub deposit_stake_validator_stake: Pubkey,
    pub deposit_stake_reserve_stake: Pubkey,
    pub deposit_stake_manager_fee: Pubkey,
    pub stake_history: Pubkey,
}
impl<'me>
    From<
        &SwapViaStakeEversolStakePoolSplStakePoolAccounts<
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
            '_,
            '_,
            '_,
        >,
    > for SwapViaStakeEversolStakePoolSplStakePoolKeys
{
    fn from(
        accounts: &SwapViaStakeEversolStakePoolSplStakePoolAccounts<
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
            '_,
            '_,
            '_,
        >,
    ) -> Self {
        Self {
            payer: *accounts.payer.key,
            user: *accounts.user.key,
            src_token_from: *accounts.src_token_from.key,
            dest_token_to: *accounts.dest_token_to.key,
            bridge_stake: *accounts.bridge_stake.key,
            dest_token_fee_token_account: *accounts.dest_token_fee_token_account.key,
            src_token_mint: *accounts.src_token_mint.key,
            dest_token_mint: *accounts.dest_token_mint.key,
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
            spl_stake_pool_program: *accounts.spl_stake_pool_program.key,
            deposit_stake_spl_stake_pool: *accounts.deposit_stake_spl_stake_pool.key,
            deposit_stake_validator_list: *accounts.deposit_stake_validator_list.key,
            deposit_stake_deposit_authority: *accounts.deposit_stake_deposit_authority.key,
            deposit_stake_withdraw_authority: *accounts.deposit_stake_withdraw_authority.key,
            deposit_stake_validator_stake: *accounts.deposit_stake_validator_stake.key,
            deposit_stake_reserve_stake: *accounts.deposit_stake_reserve_stake.key,
            deposit_stake_manager_fee: *accounts.deposit_stake_manager_fee.key,
            stake_history: *accounts.stake_history.key,
        }
    }
}
impl From<&SwapViaStakeEversolStakePoolSplStakePoolKeys> for [AccountMeta; 30] {
    fn from(keys: &SwapViaStakeEversolStakePoolSplStakePoolKeys) -> Self {
        [
            AccountMeta::new(keys.payer, true),
            AccountMeta::new_readonly(keys.user, true),
            AccountMeta::new(keys.src_token_from, false),
            AccountMeta::new(keys.dest_token_to, false),
            AccountMeta::new(keys.bridge_stake, false),
            AccountMeta::new(keys.dest_token_fee_token_account, false),
            AccountMeta::new(keys.src_token_mint, false),
            AccountMeta::new(keys.dest_token_mint, false),
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
impl<'a>
    From<
        &SwapViaStakeEversolStakePoolSplStakePoolAccounts<
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
            'a,
            'a,
            'a,
        >,
    > for [AccountInfo<'a>; 30]
{
    fn from(
        accounts: &SwapViaStakeEversolStakePoolSplStakePoolAccounts<
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
            'a,
            'a,
            'a,
        >,
    ) -> Self {
        [
            accounts.payer.clone(),
            accounts.user.clone(),
            accounts.src_token_from.clone(),
            accounts.dest_token_to.clone(),
            accounts.bridge_stake.clone(),
            accounts.dest_token_fee_token_account.clone(),
            accounts.src_token_mint.clone(),
            accounts.dest_token_mint.clone(),
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
pub struct SwapViaStakeEversolStakePoolSplStakePoolIxArgs {
    pub swap_via_stake_args: SwapViaStakeArgs,
}
#[derive(Copy, Clone, Debug)]
pub struct SwapViaStakeEversolStakePoolSplStakePoolIxData<'me>(
    pub &'me SwapViaStakeEversolStakePoolSplStakePoolIxArgs,
);
pub const SWAP_VIA_STAKE_EVERSOL_STAKE_POOL_SPL_STAKE_POOL_IX_DISCM: u8 = 1u8;
impl<'me> From<&'me SwapViaStakeEversolStakePoolSplStakePoolIxArgs>
    for SwapViaStakeEversolStakePoolSplStakePoolIxData<'me>
{
    fn from(args: &'me SwapViaStakeEversolStakePoolSplStakePoolIxArgs) -> Self {
        Self(args)
    }
}
impl BorshSerialize for SwapViaStakeEversolStakePoolSplStakePoolIxData<'_> {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        writer.write_all(&[SWAP_VIA_STAKE_EVERSOL_STAKE_POOL_SPL_STAKE_POOL_IX_DISCM])?;
        self.0.serialize(writer)
    }
}
pub fn swap_via_stake_eversol_stake_pool_spl_stake_pool_ix<
    K: Into<SwapViaStakeEversolStakePoolSplStakePoolKeys>,
    A: Into<SwapViaStakeEversolStakePoolSplStakePoolIxArgs>,
>(
    accounts: K,
    args: A,
) -> std::io::Result<Instruction> {
    let keys: SwapViaStakeEversolStakePoolSplStakePoolKeys = accounts.into();
    let metas: [AccountMeta; 30] = (&keys).into();
    let args_full: SwapViaStakeEversolStakePoolSplStakePoolIxArgs = args.into();
    let data: SwapViaStakeEversolStakePoolSplStakePoolIxData = (&args_full).into();
    Ok(Instruction {
        program_id: crate::ID,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn swap_via_stake_eversol_stake_pool_spl_stake_pool_invoke<
    'a,
    A: Into<SwapViaStakeEversolStakePoolSplStakePoolIxArgs>,
>(
    accounts: &SwapViaStakeEversolStakePoolSplStakePoolAccounts<
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
        'a,
        'a,
        'a,
    >,
    args: A,
) -> ProgramResult {
    let ix = swap_via_stake_eversol_stake_pool_spl_stake_pool_ix(accounts, args)?;
    let account_info: [AccountInfo<'a>; 30] = accounts.into();
    invoke(&ix, &account_info)
}
pub fn swap_via_stake_eversol_stake_pool_spl_stake_pool_invoke_signed<
    'a,
    A: Into<SwapViaStakeEversolStakePoolSplStakePoolIxArgs>,
>(
    accounts: &SwapViaStakeEversolStakePoolSplStakePoolAccounts<
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
        'a,
        'a,
        'a,
    >,
    args: A,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let ix = swap_via_stake_eversol_stake_pool_spl_stake_pool_ix(accounts, args)?;
    let account_info: [AccountInfo<'a>; 30] = accounts.into();
    invoke_signed(&ix, &account_info, seeds)
}
#[derive(Copy, Clone, Debug)]
pub struct SwapViaStakeEversolStakePoolMarinadeAccounts<
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
    'a12: 'me,
    'a13: 'me,
    'a14: 'me,
    'a15: 'me,
    'a16: 'me,
    'a17: 'me,
    'a18: 'me,
    'a19: 'me,
    'a20: 'me,
    'a21: 'me,
    'a22: 'me,
    'a23: 'me,
    'a24: 'me,
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
    pub eversol_stake_pool_program: &'me AccountInfo<'a8>,
    pub withdraw_stake_spl_stake_pool: &'me AccountInfo<'a9>,
    pub withdraw_stake_validator_list: &'me AccountInfo<'a10>,
    pub withdraw_stake_withdraw_authority: &'me AccountInfo<'a11>,
    pub withdraw_stake_stake_to_split: &'me AccountInfo<'a12>,
    pub withdraw_stake_manager_fee: &'me AccountInfo<'a13>,
    pub clock: &'me AccountInfo<'a14>,
    pub token_program: &'me AccountInfo<'a15>,
    pub stake_program: &'me AccountInfo<'a16>,
    pub system_program: &'me AccountInfo<'a17>,
    pub marinade_program: &'me AccountInfo<'a18>,
    pub deposit_stake_marinade_state: &'me AccountInfo<'a19>,
    pub deposit_stake_validator_list: &'me AccountInfo<'a20>,
    pub deposit_stake_stake_list: &'me AccountInfo<'a21>,
    pub deposit_stake_duplication_flag: &'me AccountInfo<'a22>,
    pub deposit_stake_msol_mint_auth: &'me AccountInfo<'a23>,
    pub rent: &'me AccountInfo<'a24>,
}
#[derive(Copy, Clone, Debug)]
pub struct SwapViaStakeEversolStakePoolMarinadeKeys {
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
    pub marinade_program: Pubkey,
    pub deposit_stake_marinade_state: Pubkey,
    pub deposit_stake_validator_list: Pubkey,
    pub deposit_stake_stake_list: Pubkey,
    pub deposit_stake_duplication_flag: Pubkey,
    pub deposit_stake_msol_mint_auth: Pubkey,
    pub rent: Pubkey,
}
impl<'me>
    From<
        &SwapViaStakeEversolStakePoolMarinadeAccounts<
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
            '_,
        >,
    > for SwapViaStakeEversolStakePoolMarinadeKeys
{
    fn from(
        accounts: &SwapViaStakeEversolStakePoolMarinadeAccounts<
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
            '_,
        >,
    ) -> Self {
        Self {
            payer: *accounts.payer.key,
            user: *accounts.user.key,
            src_token_from: *accounts.src_token_from.key,
            dest_token_to: *accounts.dest_token_to.key,
            bridge_stake: *accounts.bridge_stake.key,
            dest_token_fee_token_account: *accounts.dest_token_fee_token_account.key,
            src_token_mint: *accounts.src_token_mint.key,
            dest_token_mint: *accounts.dest_token_mint.key,
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
            marinade_program: *accounts.marinade_program.key,
            deposit_stake_marinade_state: *accounts.deposit_stake_marinade_state.key,
            deposit_stake_validator_list: *accounts.deposit_stake_validator_list.key,
            deposit_stake_stake_list: *accounts.deposit_stake_stake_list.key,
            deposit_stake_duplication_flag: *accounts.deposit_stake_duplication_flag.key,
            deposit_stake_msol_mint_auth: *accounts.deposit_stake_msol_mint_auth.key,
            rent: *accounts.rent.key,
        }
    }
}
impl From<&SwapViaStakeEversolStakePoolMarinadeKeys> for [AccountMeta; 29] {
    fn from(keys: &SwapViaStakeEversolStakePoolMarinadeKeys) -> Self {
        [
            AccountMeta::new(keys.payer, true),
            AccountMeta::new_readonly(keys.user, true),
            AccountMeta::new(keys.src_token_from, false),
            AccountMeta::new(keys.dest_token_to, false),
            AccountMeta::new(keys.bridge_stake, false),
            AccountMeta::new(keys.dest_token_fee_token_account, false),
            AccountMeta::new(keys.src_token_mint, false),
            AccountMeta::new(keys.dest_token_mint, false),
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
impl<'a>
    From<
        &SwapViaStakeEversolStakePoolMarinadeAccounts<
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
            'a,
        >,
    > for [AccountInfo<'a>; 29]
{
    fn from(
        accounts: &SwapViaStakeEversolStakePoolMarinadeAccounts<
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
            'a,
        >,
    ) -> Self {
        [
            accounts.payer.clone(),
            accounts.user.clone(),
            accounts.src_token_from.clone(),
            accounts.dest_token_to.clone(),
            accounts.bridge_stake.clone(),
            accounts.dest_token_fee_token_account.clone(),
            accounts.src_token_mint.clone(),
            accounts.dest_token_mint.clone(),
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
pub struct SwapViaStakeEversolStakePoolMarinadeIxArgs {
    pub swap_via_stake_args: SwapViaStakeArgs,
}
#[derive(Copy, Clone, Debug)]
pub struct SwapViaStakeEversolStakePoolMarinadeIxData<'me>(
    pub &'me SwapViaStakeEversolStakePoolMarinadeIxArgs,
);
pub const SWAP_VIA_STAKE_EVERSOL_STAKE_POOL_MARINADE_IX_DISCM: u8 = 1u8;
impl<'me> From<&'me SwapViaStakeEversolStakePoolMarinadeIxArgs>
    for SwapViaStakeEversolStakePoolMarinadeIxData<'me>
{
    fn from(args: &'me SwapViaStakeEversolStakePoolMarinadeIxArgs) -> Self {
        Self(args)
    }
}
impl BorshSerialize for SwapViaStakeEversolStakePoolMarinadeIxData<'_> {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        writer.write_all(&[SWAP_VIA_STAKE_EVERSOL_STAKE_POOL_MARINADE_IX_DISCM])?;
        self.0.serialize(writer)
    }
}
pub fn swap_via_stake_eversol_stake_pool_marinade_ix<
    K: Into<SwapViaStakeEversolStakePoolMarinadeKeys>,
    A: Into<SwapViaStakeEversolStakePoolMarinadeIxArgs>,
>(
    accounts: K,
    args: A,
) -> std::io::Result<Instruction> {
    let keys: SwapViaStakeEversolStakePoolMarinadeKeys = accounts.into();
    let metas: [AccountMeta; 29] = (&keys).into();
    let args_full: SwapViaStakeEversolStakePoolMarinadeIxArgs = args.into();
    let data: SwapViaStakeEversolStakePoolMarinadeIxData = (&args_full).into();
    Ok(Instruction {
        program_id: crate::ID,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn swap_via_stake_eversol_stake_pool_marinade_invoke<
    'a,
    A: Into<SwapViaStakeEversolStakePoolMarinadeIxArgs>,
>(
    accounts: &SwapViaStakeEversolStakePoolMarinadeAccounts<
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
        'a,
    >,
    args: A,
) -> ProgramResult {
    let ix = swap_via_stake_eversol_stake_pool_marinade_ix(accounts, args)?;
    let account_info: [AccountInfo<'a>; 29] = accounts.into();
    invoke(&ix, &account_info)
}
pub fn swap_via_stake_eversol_stake_pool_marinade_invoke_signed<
    'a,
    A: Into<SwapViaStakeEversolStakePoolMarinadeIxArgs>,
>(
    accounts: &SwapViaStakeEversolStakePoolMarinadeAccounts<
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
        'a,
    >,
    args: A,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let ix = swap_via_stake_eversol_stake_pool_marinade_ix(accounts, args)?;
    let account_info: [AccountInfo<'a>; 29] = accounts.into();
    invoke_signed(&ix, &account_info, seeds)
}
#[derive(Copy, Clone, Debug)]
pub struct SwapViaStakeEversolStakePoolUnstakeItAccounts<
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
    'a12: 'me,
    'a13: 'me,
    'a14: 'me,
    'a15: 'me,
    'a16: 'me,
    'a17: 'me,
    'a18: 'me,
    'a19: 'me,
    'a20: 'me,
    'a21: 'me,
    'a22: 'me,
    'a23: 'me,
    'a24: 'me,
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
    pub eversol_stake_pool_program: &'me AccountInfo<'a8>,
    pub withdraw_stake_spl_stake_pool: &'me AccountInfo<'a9>,
    pub withdraw_stake_validator_list: &'me AccountInfo<'a10>,
    pub withdraw_stake_withdraw_authority: &'me AccountInfo<'a11>,
    pub withdraw_stake_stake_to_split: &'me AccountInfo<'a12>,
    pub withdraw_stake_manager_fee: &'me AccountInfo<'a13>,
    pub clock: &'me AccountInfo<'a14>,
    pub token_program: &'me AccountInfo<'a15>,
    pub stake_program: &'me AccountInfo<'a16>,
    pub system_program: &'me AccountInfo<'a17>,
    pub unstakeit_program: &'me AccountInfo<'a18>,
    pub deposit_stake_unstake_pool: &'me AccountInfo<'a19>,
    pub deposit_stake_pool_sol_reserves: &'me AccountInfo<'a20>,
    pub deposit_stake_unstake_fee: &'me AccountInfo<'a21>,
    pub deposit_stake_stake_acc_record: &'me AccountInfo<'a22>,
    pub deposit_stake_protocol_fee: &'me AccountInfo<'a23>,
    pub deposit_stake_protocol_fee_dest: &'me AccountInfo<'a24>,
}
#[derive(Copy, Clone, Debug)]
pub struct SwapViaStakeEversolStakePoolUnstakeItKeys {
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
    pub unstakeit_program: Pubkey,
    pub deposit_stake_unstake_pool: Pubkey,
    pub deposit_stake_pool_sol_reserves: Pubkey,
    pub deposit_stake_unstake_fee: Pubkey,
    pub deposit_stake_stake_acc_record: Pubkey,
    pub deposit_stake_protocol_fee: Pubkey,
    pub deposit_stake_protocol_fee_dest: Pubkey,
}
impl<'me>
    From<
        &SwapViaStakeEversolStakePoolUnstakeItAccounts<
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
            '_,
        >,
    > for SwapViaStakeEversolStakePoolUnstakeItKeys
{
    fn from(
        accounts: &SwapViaStakeEversolStakePoolUnstakeItAccounts<
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
            '_,
        >,
    ) -> Self {
        Self {
            payer: *accounts.payer.key,
            user: *accounts.user.key,
            src_token_from: *accounts.src_token_from.key,
            dest_token_to: *accounts.dest_token_to.key,
            bridge_stake: *accounts.bridge_stake.key,
            dest_token_fee_token_account: *accounts.dest_token_fee_token_account.key,
            src_token_mint: *accounts.src_token_mint.key,
            dest_token_mint: *accounts.dest_token_mint.key,
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
            unstakeit_program: *accounts.unstakeit_program.key,
            deposit_stake_unstake_pool: *accounts.deposit_stake_unstake_pool.key,
            deposit_stake_pool_sol_reserves: *accounts.deposit_stake_pool_sol_reserves.key,
            deposit_stake_unstake_fee: *accounts.deposit_stake_unstake_fee.key,
            deposit_stake_stake_acc_record: *accounts.deposit_stake_stake_acc_record.key,
            deposit_stake_protocol_fee: *accounts.deposit_stake_protocol_fee.key,
            deposit_stake_protocol_fee_dest: *accounts.deposit_stake_protocol_fee_dest.key,
        }
    }
}
impl From<&SwapViaStakeEversolStakePoolUnstakeItKeys> for [AccountMeta; 29] {
    fn from(keys: &SwapViaStakeEversolStakePoolUnstakeItKeys) -> Self {
        [
            AccountMeta::new(keys.payer, true),
            AccountMeta::new_readonly(keys.user, true),
            AccountMeta::new(keys.src_token_from, false),
            AccountMeta::new(keys.dest_token_to, false),
            AccountMeta::new(keys.bridge_stake, false),
            AccountMeta::new(keys.dest_token_fee_token_account, false),
            AccountMeta::new(keys.src_token_mint, false),
            AccountMeta::new(keys.dest_token_mint, false),
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
impl<'a>
    From<
        &SwapViaStakeEversolStakePoolUnstakeItAccounts<
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
            'a,
        >,
    > for [AccountInfo<'a>; 29]
{
    fn from(
        accounts: &SwapViaStakeEversolStakePoolUnstakeItAccounts<
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
            'a,
        >,
    ) -> Self {
        [
            accounts.payer.clone(),
            accounts.user.clone(),
            accounts.src_token_from.clone(),
            accounts.dest_token_to.clone(),
            accounts.bridge_stake.clone(),
            accounts.dest_token_fee_token_account.clone(),
            accounts.src_token_mint.clone(),
            accounts.dest_token_mint.clone(),
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
pub struct SwapViaStakeEversolStakePoolUnstakeItIxArgs {
    pub swap_via_stake_args: SwapViaStakeArgs,
}
#[derive(Copy, Clone, Debug)]
pub struct SwapViaStakeEversolStakePoolUnstakeItIxData<'me>(
    pub &'me SwapViaStakeEversolStakePoolUnstakeItIxArgs,
);
pub const SWAP_VIA_STAKE_EVERSOL_STAKE_POOL_UNSTAKE_IT_IX_DISCM: u8 = 1u8;
impl<'me> From<&'me SwapViaStakeEversolStakePoolUnstakeItIxArgs>
    for SwapViaStakeEversolStakePoolUnstakeItIxData<'me>
{
    fn from(args: &'me SwapViaStakeEversolStakePoolUnstakeItIxArgs) -> Self {
        Self(args)
    }
}
impl BorshSerialize for SwapViaStakeEversolStakePoolUnstakeItIxData<'_> {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        writer.write_all(&[SWAP_VIA_STAKE_EVERSOL_STAKE_POOL_UNSTAKE_IT_IX_DISCM])?;
        self.0.serialize(writer)
    }
}
pub fn swap_via_stake_eversol_stake_pool_unstake_it_ix<
    K: Into<SwapViaStakeEversolStakePoolUnstakeItKeys>,
    A: Into<SwapViaStakeEversolStakePoolUnstakeItIxArgs>,
>(
    accounts: K,
    args: A,
) -> std::io::Result<Instruction> {
    let keys: SwapViaStakeEversolStakePoolUnstakeItKeys = accounts.into();
    let metas: [AccountMeta; 29] = (&keys).into();
    let args_full: SwapViaStakeEversolStakePoolUnstakeItIxArgs = args.into();
    let data: SwapViaStakeEversolStakePoolUnstakeItIxData = (&args_full).into();
    Ok(Instruction {
        program_id: crate::ID,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn swap_via_stake_eversol_stake_pool_unstake_it_invoke<
    'a,
    A: Into<SwapViaStakeEversolStakePoolUnstakeItIxArgs>,
>(
    accounts: &SwapViaStakeEversolStakePoolUnstakeItAccounts<
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
        'a,
    >,
    args: A,
) -> ProgramResult {
    let ix = swap_via_stake_eversol_stake_pool_unstake_it_ix(accounts, args)?;
    let account_info: [AccountInfo<'a>; 29] = accounts.into();
    invoke(&ix, &account_info)
}
pub fn swap_via_stake_eversol_stake_pool_unstake_it_invoke_signed<
    'a,
    A: Into<SwapViaStakeEversolStakePoolUnstakeItIxArgs>,
>(
    accounts: &SwapViaStakeEversolStakePoolUnstakeItAccounts<
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
        'a,
    >,
    args: A,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let ix = swap_via_stake_eversol_stake_pool_unstake_it_ix(accounts, args)?;
    let account_info: [AccountInfo<'a>; 29] = accounts.into();
    invoke_signed(&ix, &account_info, seeds)
}
#[derive(Copy, Clone, Debug)]
pub struct SwapViaStakeSoceanStakePoolEversolStakePoolAccounts<
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
    'a12: 'me,
    'a13: 'me,
    'a14: 'me,
    'a15: 'me,
    'a16: 'me,
    'a17: 'me,
    'a18: 'me,
    'a19: 'me,
    'a20: 'me,
    'a21: 'me,
    'a22: 'me,
    'a23: 'me,
    'a24: 'me,
    'a25: 'me,
    'a26: 'me,
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
    pub socean_stake_pool_program: &'me AccountInfo<'a8>,
    pub withdraw_stake_spl_stake_pool: &'me AccountInfo<'a9>,
    pub withdraw_stake_validator_list: &'me AccountInfo<'a10>,
    pub withdraw_stake_withdraw_authority: &'me AccountInfo<'a11>,
    pub withdraw_stake_stake_to_split: &'me AccountInfo<'a12>,
    pub withdraw_stake_manager_fee: &'me AccountInfo<'a13>,
    pub clock: &'me AccountInfo<'a14>,
    pub token_program: &'me AccountInfo<'a15>,
    pub stake_program: &'me AccountInfo<'a16>,
    pub system_program: &'me AccountInfo<'a17>,
    pub eversol_stake_pool_program: &'me AccountInfo<'a18>,
    pub deposit_stake_spl_stake_pool: &'me AccountInfo<'a19>,
    pub deposit_stake_validator_list: &'me AccountInfo<'a20>,
    pub deposit_stake_deposit_authority: &'me AccountInfo<'a21>,
    pub deposit_stake_withdraw_authority: &'me AccountInfo<'a22>,
    pub deposit_stake_validator_stake: &'me AccountInfo<'a23>,
    pub deposit_stake_reserve_stake: &'me AccountInfo<'a24>,
    pub deposit_stake_manager_fee: &'me AccountInfo<'a25>,
    pub stake_history: &'me AccountInfo<'a26>,
}
#[derive(Copy, Clone, Debug)]
pub struct SwapViaStakeSoceanStakePoolEversolStakePoolKeys {
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
    pub eversol_stake_pool_program: Pubkey,
    pub deposit_stake_spl_stake_pool: Pubkey,
    pub deposit_stake_validator_list: Pubkey,
    pub deposit_stake_deposit_authority: Pubkey,
    pub deposit_stake_withdraw_authority: Pubkey,
    pub deposit_stake_validator_stake: Pubkey,
    pub deposit_stake_reserve_stake: Pubkey,
    pub deposit_stake_manager_fee: Pubkey,
    pub stake_history: Pubkey,
}
impl<'me>
    From<
        &SwapViaStakeSoceanStakePoolEversolStakePoolAccounts<
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
            '_,
            '_,
            '_,
        >,
    > for SwapViaStakeSoceanStakePoolEversolStakePoolKeys
{
    fn from(
        accounts: &SwapViaStakeSoceanStakePoolEversolStakePoolAccounts<
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
            '_,
            '_,
            '_,
        >,
    ) -> Self {
        Self {
            payer: *accounts.payer.key,
            user: *accounts.user.key,
            src_token_from: *accounts.src_token_from.key,
            dest_token_to: *accounts.dest_token_to.key,
            bridge_stake: *accounts.bridge_stake.key,
            dest_token_fee_token_account: *accounts.dest_token_fee_token_account.key,
            src_token_mint: *accounts.src_token_mint.key,
            dest_token_mint: *accounts.dest_token_mint.key,
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
            eversol_stake_pool_program: *accounts.eversol_stake_pool_program.key,
            deposit_stake_spl_stake_pool: *accounts.deposit_stake_spl_stake_pool.key,
            deposit_stake_validator_list: *accounts.deposit_stake_validator_list.key,
            deposit_stake_deposit_authority: *accounts.deposit_stake_deposit_authority.key,
            deposit_stake_withdraw_authority: *accounts.deposit_stake_withdraw_authority.key,
            deposit_stake_validator_stake: *accounts.deposit_stake_validator_stake.key,
            deposit_stake_reserve_stake: *accounts.deposit_stake_reserve_stake.key,
            deposit_stake_manager_fee: *accounts.deposit_stake_manager_fee.key,
            stake_history: *accounts.stake_history.key,
        }
    }
}
impl From<&SwapViaStakeSoceanStakePoolEversolStakePoolKeys> for [AccountMeta; 30] {
    fn from(keys: &SwapViaStakeSoceanStakePoolEversolStakePoolKeys) -> Self {
        [
            AccountMeta::new(keys.payer, true),
            AccountMeta::new_readonly(keys.user, true),
            AccountMeta::new(keys.src_token_from, false),
            AccountMeta::new(keys.dest_token_to, false),
            AccountMeta::new(keys.bridge_stake, false),
            AccountMeta::new(keys.dest_token_fee_token_account, false),
            AccountMeta::new(keys.src_token_mint, false),
            AccountMeta::new(keys.dest_token_mint, false),
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
    From<
        &SwapViaStakeSoceanStakePoolEversolStakePoolAccounts<
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
            'a,
            'a,
            'a,
        >,
    > for [AccountInfo<'a>; 30]
{
    fn from(
        accounts: &SwapViaStakeSoceanStakePoolEversolStakePoolAccounts<
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
            'a,
            'a,
            'a,
        >,
    ) -> Self {
        [
            accounts.payer.clone(),
            accounts.user.clone(),
            accounts.src_token_from.clone(),
            accounts.dest_token_to.clone(),
            accounts.bridge_stake.clone(),
            accounts.dest_token_fee_token_account.clone(),
            accounts.src_token_mint.clone(),
            accounts.dest_token_mint.clone(),
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
pub struct SwapViaStakeSoceanStakePoolEversolStakePoolIxArgs {
    pub swap_via_stake_args: SwapViaStakeArgs,
}
#[derive(Copy, Clone, Debug)]
pub struct SwapViaStakeSoceanStakePoolEversolStakePoolIxData<'me>(
    pub &'me SwapViaStakeSoceanStakePoolEversolStakePoolIxArgs,
);
pub const SWAP_VIA_STAKE_SOCEAN_STAKE_POOL_EVERSOL_STAKE_POOL_IX_DISCM: u8 = 1u8;
impl<'me> From<&'me SwapViaStakeSoceanStakePoolEversolStakePoolIxArgs>
    for SwapViaStakeSoceanStakePoolEversolStakePoolIxData<'me>
{
    fn from(args: &'me SwapViaStakeSoceanStakePoolEversolStakePoolIxArgs) -> Self {
        Self(args)
    }
}
impl BorshSerialize for SwapViaStakeSoceanStakePoolEversolStakePoolIxData<'_> {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        writer.write_all(&[SWAP_VIA_STAKE_SOCEAN_STAKE_POOL_EVERSOL_STAKE_POOL_IX_DISCM])?;
        self.0.serialize(writer)
    }
}
pub fn swap_via_stake_socean_stake_pool_eversol_stake_pool_ix<
    K: Into<SwapViaStakeSoceanStakePoolEversolStakePoolKeys>,
    A: Into<SwapViaStakeSoceanStakePoolEversolStakePoolIxArgs>,
>(
    accounts: K,
    args: A,
) -> std::io::Result<Instruction> {
    let keys: SwapViaStakeSoceanStakePoolEversolStakePoolKeys = accounts.into();
    let metas: [AccountMeta; 30] = (&keys).into();
    let args_full: SwapViaStakeSoceanStakePoolEversolStakePoolIxArgs = args.into();
    let data: SwapViaStakeSoceanStakePoolEversolStakePoolIxData = (&args_full).into();
    Ok(Instruction {
        program_id: crate::ID,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn swap_via_stake_socean_stake_pool_eversol_stake_pool_invoke<
    'a,
    A: Into<SwapViaStakeSoceanStakePoolEversolStakePoolIxArgs>,
>(
    accounts: &SwapViaStakeSoceanStakePoolEversolStakePoolAccounts<
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
        'a,
        'a,
        'a,
    >,
    args: A,
) -> ProgramResult {
    let ix = swap_via_stake_socean_stake_pool_eversol_stake_pool_ix(accounts, args)?;
    let account_info: [AccountInfo<'a>; 30] = accounts.into();
    invoke(&ix, &account_info)
}
pub fn swap_via_stake_socean_stake_pool_eversol_stake_pool_invoke_signed<
    'a,
    A: Into<SwapViaStakeSoceanStakePoolEversolStakePoolIxArgs>,
>(
    accounts: &SwapViaStakeSoceanStakePoolEversolStakePoolAccounts<
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
        'a,
        'a,
        'a,
    >,
    args: A,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let ix = swap_via_stake_socean_stake_pool_eversol_stake_pool_ix(accounts, args)?;
    let account_info: [AccountInfo<'a>; 30] = accounts.into();
    invoke_signed(&ix, &account_info, seeds)
}
#[derive(Copy, Clone, Debug)]
pub struct SwapViaStakeSoceanStakePoolSplStakePoolAccounts<
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
    'a12: 'me,
    'a13: 'me,
    'a14: 'me,
    'a15: 'me,
    'a16: 'me,
    'a17: 'me,
    'a18: 'me,
    'a19: 'me,
    'a20: 'me,
    'a21: 'me,
    'a22: 'me,
    'a23: 'me,
    'a24: 'me,
    'a25: 'me,
    'a26: 'me,
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
    pub socean_stake_pool_program: &'me AccountInfo<'a8>,
    pub withdraw_stake_spl_stake_pool: &'me AccountInfo<'a9>,
    pub withdraw_stake_validator_list: &'me AccountInfo<'a10>,
    pub withdraw_stake_withdraw_authority: &'me AccountInfo<'a11>,
    pub withdraw_stake_stake_to_split: &'me AccountInfo<'a12>,
    pub withdraw_stake_manager_fee: &'me AccountInfo<'a13>,
    pub clock: &'me AccountInfo<'a14>,
    pub token_program: &'me AccountInfo<'a15>,
    pub stake_program: &'me AccountInfo<'a16>,
    pub system_program: &'me AccountInfo<'a17>,
    pub spl_stake_pool_program: &'me AccountInfo<'a18>,
    pub deposit_stake_spl_stake_pool: &'me AccountInfo<'a19>,
    pub deposit_stake_validator_list: &'me AccountInfo<'a20>,
    pub deposit_stake_deposit_authority: &'me AccountInfo<'a21>,
    pub deposit_stake_withdraw_authority: &'me AccountInfo<'a22>,
    pub deposit_stake_validator_stake: &'me AccountInfo<'a23>,
    pub deposit_stake_reserve_stake: &'me AccountInfo<'a24>,
    pub deposit_stake_manager_fee: &'me AccountInfo<'a25>,
    pub stake_history: &'me AccountInfo<'a26>,
}
#[derive(Copy, Clone, Debug)]
pub struct SwapViaStakeSoceanStakePoolSplStakePoolKeys {
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
    pub spl_stake_pool_program: Pubkey,
    pub deposit_stake_spl_stake_pool: Pubkey,
    pub deposit_stake_validator_list: Pubkey,
    pub deposit_stake_deposit_authority: Pubkey,
    pub deposit_stake_withdraw_authority: Pubkey,
    pub deposit_stake_validator_stake: Pubkey,
    pub deposit_stake_reserve_stake: Pubkey,
    pub deposit_stake_manager_fee: Pubkey,
    pub stake_history: Pubkey,
}
impl<'me>
    From<
        &SwapViaStakeSoceanStakePoolSplStakePoolAccounts<
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
            '_,
            '_,
            '_,
        >,
    > for SwapViaStakeSoceanStakePoolSplStakePoolKeys
{
    fn from(
        accounts: &SwapViaStakeSoceanStakePoolSplStakePoolAccounts<
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
            '_,
            '_,
            '_,
        >,
    ) -> Self {
        Self {
            payer: *accounts.payer.key,
            user: *accounts.user.key,
            src_token_from: *accounts.src_token_from.key,
            dest_token_to: *accounts.dest_token_to.key,
            bridge_stake: *accounts.bridge_stake.key,
            dest_token_fee_token_account: *accounts.dest_token_fee_token_account.key,
            src_token_mint: *accounts.src_token_mint.key,
            dest_token_mint: *accounts.dest_token_mint.key,
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
            spl_stake_pool_program: *accounts.spl_stake_pool_program.key,
            deposit_stake_spl_stake_pool: *accounts.deposit_stake_spl_stake_pool.key,
            deposit_stake_validator_list: *accounts.deposit_stake_validator_list.key,
            deposit_stake_deposit_authority: *accounts.deposit_stake_deposit_authority.key,
            deposit_stake_withdraw_authority: *accounts.deposit_stake_withdraw_authority.key,
            deposit_stake_validator_stake: *accounts.deposit_stake_validator_stake.key,
            deposit_stake_reserve_stake: *accounts.deposit_stake_reserve_stake.key,
            deposit_stake_manager_fee: *accounts.deposit_stake_manager_fee.key,
            stake_history: *accounts.stake_history.key,
        }
    }
}
impl From<&SwapViaStakeSoceanStakePoolSplStakePoolKeys> for [AccountMeta; 30] {
    fn from(keys: &SwapViaStakeSoceanStakePoolSplStakePoolKeys) -> Self {
        [
            AccountMeta::new(keys.payer, true),
            AccountMeta::new_readonly(keys.user, true),
            AccountMeta::new(keys.src_token_from, false),
            AccountMeta::new(keys.dest_token_to, false),
            AccountMeta::new(keys.bridge_stake, false),
            AccountMeta::new(keys.dest_token_fee_token_account, false),
            AccountMeta::new(keys.src_token_mint, false),
            AccountMeta::new(keys.dest_token_mint, false),
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
impl<'a>
    From<
        &SwapViaStakeSoceanStakePoolSplStakePoolAccounts<
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
            'a,
            'a,
            'a,
        >,
    > for [AccountInfo<'a>; 30]
{
    fn from(
        accounts: &SwapViaStakeSoceanStakePoolSplStakePoolAccounts<
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
            'a,
            'a,
            'a,
        >,
    ) -> Self {
        [
            accounts.payer.clone(),
            accounts.user.clone(),
            accounts.src_token_from.clone(),
            accounts.dest_token_to.clone(),
            accounts.bridge_stake.clone(),
            accounts.dest_token_fee_token_account.clone(),
            accounts.src_token_mint.clone(),
            accounts.dest_token_mint.clone(),
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
pub struct SwapViaStakeSoceanStakePoolSplStakePoolIxArgs {
    pub swap_via_stake_args: SwapViaStakeArgs,
}
#[derive(Copy, Clone, Debug)]
pub struct SwapViaStakeSoceanStakePoolSplStakePoolIxData<'me>(
    pub &'me SwapViaStakeSoceanStakePoolSplStakePoolIxArgs,
);
pub const SWAP_VIA_STAKE_SOCEAN_STAKE_POOL_SPL_STAKE_POOL_IX_DISCM: u8 = 1u8;
impl<'me> From<&'me SwapViaStakeSoceanStakePoolSplStakePoolIxArgs>
    for SwapViaStakeSoceanStakePoolSplStakePoolIxData<'me>
{
    fn from(args: &'me SwapViaStakeSoceanStakePoolSplStakePoolIxArgs) -> Self {
        Self(args)
    }
}
impl BorshSerialize for SwapViaStakeSoceanStakePoolSplStakePoolIxData<'_> {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        writer.write_all(&[SWAP_VIA_STAKE_SOCEAN_STAKE_POOL_SPL_STAKE_POOL_IX_DISCM])?;
        self.0.serialize(writer)
    }
}
pub fn swap_via_stake_socean_stake_pool_spl_stake_pool_ix<
    K: Into<SwapViaStakeSoceanStakePoolSplStakePoolKeys>,
    A: Into<SwapViaStakeSoceanStakePoolSplStakePoolIxArgs>,
>(
    accounts: K,
    args: A,
) -> std::io::Result<Instruction> {
    let keys: SwapViaStakeSoceanStakePoolSplStakePoolKeys = accounts.into();
    let metas: [AccountMeta; 30] = (&keys).into();
    let args_full: SwapViaStakeSoceanStakePoolSplStakePoolIxArgs = args.into();
    let data: SwapViaStakeSoceanStakePoolSplStakePoolIxData = (&args_full).into();
    Ok(Instruction {
        program_id: crate::ID,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn swap_via_stake_socean_stake_pool_spl_stake_pool_invoke<
    'a,
    A: Into<SwapViaStakeSoceanStakePoolSplStakePoolIxArgs>,
>(
    accounts: &SwapViaStakeSoceanStakePoolSplStakePoolAccounts<
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
        'a,
        'a,
        'a,
    >,
    args: A,
) -> ProgramResult {
    let ix = swap_via_stake_socean_stake_pool_spl_stake_pool_ix(accounts, args)?;
    let account_info: [AccountInfo<'a>; 30] = accounts.into();
    invoke(&ix, &account_info)
}
pub fn swap_via_stake_socean_stake_pool_spl_stake_pool_invoke_signed<
    'a,
    A: Into<SwapViaStakeSoceanStakePoolSplStakePoolIxArgs>,
>(
    accounts: &SwapViaStakeSoceanStakePoolSplStakePoolAccounts<
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
        'a,
        'a,
        'a,
    >,
    args: A,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let ix = swap_via_stake_socean_stake_pool_spl_stake_pool_ix(accounts, args)?;
    let account_info: [AccountInfo<'a>; 30] = accounts.into();
    invoke_signed(&ix, &account_info, seeds)
}
#[derive(Copy, Clone, Debug)]
pub struct SwapViaStakeSoceanStakePoolMarinadeAccounts<
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
    'a12: 'me,
    'a13: 'me,
    'a14: 'me,
    'a15: 'me,
    'a16: 'me,
    'a17: 'me,
    'a18: 'me,
    'a19: 'me,
    'a20: 'me,
    'a21: 'me,
    'a22: 'me,
    'a23: 'me,
    'a24: 'me,
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
    pub socean_stake_pool_program: &'me AccountInfo<'a8>,
    pub withdraw_stake_spl_stake_pool: &'me AccountInfo<'a9>,
    pub withdraw_stake_validator_list: &'me AccountInfo<'a10>,
    pub withdraw_stake_withdraw_authority: &'me AccountInfo<'a11>,
    pub withdraw_stake_stake_to_split: &'me AccountInfo<'a12>,
    pub withdraw_stake_manager_fee: &'me AccountInfo<'a13>,
    pub clock: &'me AccountInfo<'a14>,
    pub token_program: &'me AccountInfo<'a15>,
    pub stake_program: &'me AccountInfo<'a16>,
    pub system_program: &'me AccountInfo<'a17>,
    pub marinade_program: &'me AccountInfo<'a18>,
    pub deposit_stake_marinade_state: &'me AccountInfo<'a19>,
    pub deposit_stake_validator_list: &'me AccountInfo<'a20>,
    pub deposit_stake_stake_list: &'me AccountInfo<'a21>,
    pub deposit_stake_duplication_flag: &'me AccountInfo<'a22>,
    pub deposit_stake_msol_mint_auth: &'me AccountInfo<'a23>,
    pub rent: &'me AccountInfo<'a24>,
}
#[derive(Copy, Clone, Debug)]
pub struct SwapViaStakeSoceanStakePoolMarinadeKeys {
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
    pub marinade_program: Pubkey,
    pub deposit_stake_marinade_state: Pubkey,
    pub deposit_stake_validator_list: Pubkey,
    pub deposit_stake_stake_list: Pubkey,
    pub deposit_stake_duplication_flag: Pubkey,
    pub deposit_stake_msol_mint_auth: Pubkey,
    pub rent: Pubkey,
}
impl<'me>
    From<
        &SwapViaStakeSoceanStakePoolMarinadeAccounts<
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
            '_,
        >,
    > for SwapViaStakeSoceanStakePoolMarinadeKeys
{
    fn from(
        accounts: &SwapViaStakeSoceanStakePoolMarinadeAccounts<
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
            '_,
        >,
    ) -> Self {
        Self {
            payer: *accounts.payer.key,
            user: *accounts.user.key,
            src_token_from: *accounts.src_token_from.key,
            dest_token_to: *accounts.dest_token_to.key,
            bridge_stake: *accounts.bridge_stake.key,
            dest_token_fee_token_account: *accounts.dest_token_fee_token_account.key,
            src_token_mint: *accounts.src_token_mint.key,
            dest_token_mint: *accounts.dest_token_mint.key,
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
            marinade_program: *accounts.marinade_program.key,
            deposit_stake_marinade_state: *accounts.deposit_stake_marinade_state.key,
            deposit_stake_validator_list: *accounts.deposit_stake_validator_list.key,
            deposit_stake_stake_list: *accounts.deposit_stake_stake_list.key,
            deposit_stake_duplication_flag: *accounts.deposit_stake_duplication_flag.key,
            deposit_stake_msol_mint_auth: *accounts.deposit_stake_msol_mint_auth.key,
            rent: *accounts.rent.key,
        }
    }
}
impl From<&SwapViaStakeSoceanStakePoolMarinadeKeys> for [AccountMeta; 29] {
    fn from(keys: &SwapViaStakeSoceanStakePoolMarinadeKeys) -> Self {
        [
            AccountMeta::new(keys.payer, true),
            AccountMeta::new_readonly(keys.user, true),
            AccountMeta::new(keys.src_token_from, false),
            AccountMeta::new(keys.dest_token_to, false),
            AccountMeta::new(keys.bridge_stake, false),
            AccountMeta::new(keys.dest_token_fee_token_account, false),
            AccountMeta::new(keys.src_token_mint, false),
            AccountMeta::new(keys.dest_token_mint, false),
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
impl<'a>
    From<
        &SwapViaStakeSoceanStakePoolMarinadeAccounts<
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
            'a,
        >,
    > for [AccountInfo<'a>; 29]
{
    fn from(
        accounts: &SwapViaStakeSoceanStakePoolMarinadeAccounts<
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
            'a,
        >,
    ) -> Self {
        [
            accounts.payer.clone(),
            accounts.user.clone(),
            accounts.src_token_from.clone(),
            accounts.dest_token_to.clone(),
            accounts.bridge_stake.clone(),
            accounts.dest_token_fee_token_account.clone(),
            accounts.src_token_mint.clone(),
            accounts.dest_token_mint.clone(),
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
pub struct SwapViaStakeSoceanStakePoolMarinadeIxArgs {
    pub swap_via_stake_args: SwapViaStakeArgs,
}
#[derive(Copy, Clone, Debug)]
pub struct SwapViaStakeSoceanStakePoolMarinadeIxData<'me>(
    pub &'me SwapViaStakeSoceanStakePoolMarinadeIxArgs,
);
pub const SWAP_VIA_STAKE_SOCEAN_STAKE_POOL_MARINADE_IX_DISCM: u8 = 1u8;
impl<'me> From<&'me SwapViaStakeSoceanStakePoolMarinadeIxArgs>
    for SwapViaStakeSoceanStakePoolMarinadeIxData<'me>
{
    fn from(args: &'me SwapViaStakeSoceanStakePoolMarinadeIxArgs) -> Self {
        Self(args)
    }
}
impl BorshSerialize for SwapViaStakeSoceanStakePoolMarinadeIxData<'_> {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        writer.write_all(&[SWAP_VIA_STAKE_SOCEAN_STAKE_POOL_MARINADE_IX_DISCM])?;
        self.0.serialize(writer)
    }
}
pub fn swap_via_stake_socean_stake_pool_marinade_ix<
    K: Into<SwapViaStakeSoceanStakePoolMarinadeKeys>,
    A: Into<SwapViaStakeSoceanStakePoolMarinadeIxArgs>,
>(
    accounts: K,
    args: A,
) -> std::io::Result<Instruction> {
    let keys: SwapViaStakeSoceanStakePoolMarinadeKeys = accounts.into();
    let metas: [AccountMeta; 29] = (&keys).into();
    let args_full: SwapViaStakeSoceanStakePoolMarinadeIxArgs = args.into();
    let data: SwapViaStakeSoceanStakePoolMarinadeIxData = (&args_full).into();
    Ok(Instruction {
        program_id: crate::ID,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn swap_via_stake_socean_stake_pool_marinade_invoke<
    'a,
    A: Into<SwapViaStakeSoceanStakePoolMarinadeIxArgs>,
>(
    accounts: &SwapViaStakeSoceanStakePoolMarinadeAccounts<
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
        'a,
    >,
    args: A,
) -> ProgramResult {
    let ix = swap_via_stake_socean_stake_pool_marinade_ix(accounts, args)?;
    let account_info: [AccountInfo<'a>; 29] = accounts.into();
    invoke(&ix, &account_info)
}
pub fn swap_via_stake_socean_stake_pool_marinade_invoke_signed<
    'a,
    A: Into<SwapViaStakeSoceanStakePoolMarinadeIxArgs>,
>(
    accounts: &SwapViaStakeSoceanStakePoolMarinadeAccounts<
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
        'a,
    >,
    args: A,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let ix = swap_via_stake_socean_stake_pool_marinade_ix(accounts, args)?;
    let account_info: [AccountInfo<'a>; 29] = accounts.into();
    invoke_signed(&ix, &account_info, seeds)
}
#[derive(Copy, Clone, Debug)]
pub struct SwapViaStakeSoceanStakePoolUnstakeItAccounts<
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
    'a12: 'me,
    'a13: 'me,
    'a14: 'me,
    'a15: 'me,
    'a16: 'me,
    'a17: 'me,
    'a18: 'me,
    'a19: 'me,
    'a20: 'me,
    'a21: 'me,
    'a22: 'me,
    'a23: 'me,
    'a24: 'me,
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
    pub socean_stake_pool_program: &'me AccountInfo<'a8>,
    pub withdraw_stake_spl_stake_pool: &'me AccountInfo<'a9>,
    pub withdraw_stake_validator_list: &'me AccountInfo<'a10>,
    pub withdraw_stake_withdraw_authority: &'me AccountInfo<'a11>,
    pub withdraw_stake_stake_to_split: &'me AccountInfo<'a12>,
    pub withdraw_stake_manager_fee: &'me AccountInfo<'a13>,
    pub clock: &'me AccountInfo<'a14>,
    pub token_program: &'me AccountInfo<'a15>,
    pub stake_program: &'me AccountInfo<'a16>,
    pub system_program: &'me AccountInfo<'a17>,
    pub unstakeit_program: &'me AccountInfo<'a18>,
    pub deposit_stake_unstake_pool: &'me AccountInfo<'a19>,
    pub deposit_stake_pool_sol_reserves: &'me AccountInfo<'a20>,
    pub deposit_stake_unstake_fee: &'me AccountInfo<'a21>,
    pub deposit_stake_stake_acc_record: &'me AccountInfo<'a22>,
    pub deposit_stake_protocol_fee: &'me AccountInfo<'a23>,
    pub deposit_stake_protocol_fee_dest: &'me AccountInfo<'a24>,
}
#[derive(Copy, Clone, Debug)]
pub struct SwapViaStakeSoceanStakePoolUnstakeItKeys {
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
    pub unstakeit_program: Pubkey,
    pub deposit_stake_unstake_pool: Pubkey,
    pub deposit_stake_pool_sol_reserves: Pubkey,
    pub deposit_stake_unstake_fee: Pubkey,
    pub deposit_stake_stake_acc_record: Pubkey,
    pub deposit_stake_protocol_fee: Pubkey,
    pub deposit_stake_protocol_fee_dest: Pubkey,
}
impl<'me>
    From<
        &SwapViaStakeSoceanStakePoolUnstakeItAccounts<
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
            '_,
        >,
    > for SwapViaStakeSoceanStakePoolUnstakeItKeys
{
    fn from(
        accounts: &SwapViaStakeSoceanStakePoolUnstakeItAccounts<
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
            '_,
        >,
    ) -> Self {
        Self {
            payer: *accounts.payer.key,
            user: *accounts.user.key,
            src_token_from: *accounts.src_token_from.key,
            dest_token_to: *accounts.dest_token_to.key,
            bridge_stake: *accounts.bridge_stake.key,
            dest_token_fee_token_account: *accounts.dest_token_fee_token_account.key,
            src_token_mint: *accounts.src_token_mint.key,
            dest_token_mint: *accounts.dest_token_mint.key,
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
            unstakeit_program: *accounts.unstakeit_program.key,
            deposit_stake_unstake_pool: *accounts.deposit_stake_unstake_pool.key,
            deposit_stake_pool_sol_reserves: *accounts.deposit_stake_pool_sol_reserves.key,
            deposit_stake_unstake_fee: *accounts.deposit_stake_unstake_fee.key,
            deposit_stake_stake_acc_record: *accounts.deposit_stake_stake_acc_record.key,
            deposit_stake_protocol_fee: *accounts.deposit_stake_protocol_fee.key,
            deposit_stake_protocol_fee_dest: *accounts.deposit_stake_protocol_fee_dest.key,
        }
    }
}
impl From<&SwapViaStakeSoceanStakePoolUnstakeItKeys> for [AccountMeta; 29] {
    fn from(keys: &SwapViaStakeSoceanStakePoolUnstakeItKeys) -> Self {
        [
            AccountMeta::new(keys.payer, true),
            AccountMeta::new_readonly(keys.user, true),
            AccountMeta::new(keys.src_token_from, false),
            AccountMeta::new(keys.dest_token_to, false),
            AccountMeta::new(keys.bridge_stake, false),
            AccountMeta::new(keys.dest_token_fee_token_account, false),
            AccountMeta::new(keys.src_token_mint, false),
            AccountMeta::new(keys.dest_token_mint, false),
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
impl<'a>
    From<
        &SwapViaStakeSoceanStakePoolUnstakeItAccounts<
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
            'a,
        >,
    > for [AccountInfo<'a>; 29]
{
    fn from(
        accounts: &SwapViaStakeSoceanStakePoolUnstakeItAccounts<
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
            'a,
        >,
    ) -> Self {
        [
            accounts.payer.clone(),
            accounts.user.clone(),
            accounts.src_token_from.clone(),
            accounts.dest_token_to.clone(),
            accounts.bridge_stake.clone(),
            accounts.dest_token_fee_token_account.clone(),
            accounts.src_token_mint.clone(),
            accounts.dest_token_mint.clone(),
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
pub struct SwapViaStakeSoceanStakePoolUnstakeItIxArgs {
    pub swap_via_stake_args: SwapViaStakeArgs,
}
#[derive(Copy, Clone, Debug)]
pub struct SwapViaStakeSoceanStakePoolUnstakeItIxData<'me>(
    pub &'me SwapViaStakeSoceanStakePoolUnstakeItIxArgs,
);
pub const SWAP_VIA_STAKE_SOCEAN_STAKE_POOL_UNSTAKE_IT_IX_DISCM: u8 = 1u8;
impl<'me> From<&'me SwapViaStakeSoceanStakePoolUnstakeItIxArgs>
    for SwapViaStakeSoceanStakePoolUnstakeItIxData<'me>
{
    fn from(args: &'me SwapViaStakeSoceanStakePoolUnstakeItIxArgs) -> Self {
        Self(args)
    }
}
impl BorshSerialize for SwapViaStakeSoceanStakePoolUnstakeItIxData<'_> {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        writer.write_all(&[SWAP_VIA_STAKE_SOCEAN_STAKE_POOL_UNSTAKE_IT_IX_DISCM])?;
        self.0.serialize(writer)
    }
}
pub fn swap_via_stake_socean_stake_pool_unstake_it_ix<
    K: Into<SwapViaStakeSoceanStakePoolUnstakeItKeys>,
    A: Into<SwapViaStakeSoceanStakePoolUnstakeItIxArgs>,
>(
    accounts: K,
    args: A,
) -> std::io::Result<Instruction> {
    let keys: SwapViaStakeSoceanStakePoolUnstakeItKeys = accounts.into();
    let metas: [AccountMeta; 29] = (&keys).into();
    let args_full: SwapViaStakeSoceanStakePoolUnstakeItIxArgs = args.into();
    let data: SwapViaStakeSoceanStakePoolUnstakeItIxData = (&args_full).into();
    Ok(Instruction {
        program_id: crate::ID,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn swap_via_stake_socean_stake_pool_unstake_it_invoke<
    'a,
    A: Into<SwapViaStakeSoceanStakePoolUnstakeItIxArgs>,
>(
    accounts: &SwapViaStakeSoceanStakePoolUnstakeItAccounts<
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
        'a,
    >,
    args: A,
) -> ProgramResult {
    let ix = swap_via_stake_socean_stake_pool_unstake_it_ix(accounts, args)?;
    let account_info: [AccountInfo<'a>; 29] = accounts.into();
    invoke(&ix, &account_info)
}
pub fn swap_via_stake_socean_stake_pool_unstake_it_invoke_signed<
    'a,
    A: Into<SwapViaStakeSoceanStakePoolUnstakeItIxArgs>,
>(
    accounts: &SwapViaStakeSoceanStakePoolUnstakeItAccounts<
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
        'a,
    >,
    args: A,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let ix = swap_via_stake_socean_stake_pool_unstake_it_ix(accounts, args)?;
    let account_info: [AccountInfo<'a>; 29] = accounts.into();
    invoke_signed(&ix, &account_info, seeds)
}
#[derive(Copy, Clone, Debug)]
pub struct SwapViaStakeSplStakePoolEversolStakePoolAccounts<
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
    'a12: 'me,
    'a13: 'me,
    'a14: 'me,
    'a15: 'me,
    'a16: 'me,
    'a17: 'me,
    'a18: 'me,
    'a19: 'me,
    'a20: 'me,
    'a21: 'me,
    'a22: 'me,
    'a23: 'me,
    'a24: 'me,
    'a25: 'me,
    'a26: 'me,
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
    pub spl_stake_pool_program: &'me AccountInfo<'a8>,
    pub withdraw_stake_spl_stake_pool: &'me AccountInfo<'a9>,
    pub withdraw_stake_validator_list: &'me AccountInfo<'a10>,
    pub withdraw_stake_withdraw_authority: &'me AccountInfo<'a11>,
    pub withdraw_stake_stake_to_split: &'me AccountInfo<'a12>,
    pub withdraw_stake_manager_fee: &'me AccountInfo<'a13>,
    pub clock: &'me AccountInfo<'a14>,
    pub token_program: &'me AccountInfo<'a15>,
    pub stake_program: &'me AccountInfo<'a16>,
    pub system_program: &'me AccountInfo<'a17>,
    pub eversol_stake_pool_program: &'me AccountInfo<'a18>,
    pub deposit_stake_spl_stake_pool: &'me AccountInfo<'a19>,
    pub deposit_stake_validator_list: &'me AccountInfo<'a20>,
    pub deposit_stake_deposit_authority: &'me AccountInfo<'a21>,
    pub deposit_stake_withdraw_authority: &'me AccountInfo<'a22>,
    pub deposit_stake_validator_stake: &'me AccountInfo<'a23>,
    pub deposit_stake_reserve_stake: &'me AccountInfo<'a24>,
    pub deposit_stake_manager_fee: &'me AccountInfo<'a25>,
    pub stake_history: &'me AccountInfo<'a26>,
}
#[derive(Copy, Clone, Debug)]
pub struct SwapViaStakeSplStakePoolEversolStakePoolKeys {
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
    pub eversol_stake_pool_program: Pubkey,
    pub deposit_stake_spl_stake_pool: Pubkey,
    pub deposit_stake_validator_list: Pubkey,
    pub deposit_stake_deposit_authority: Pubkey,
    pub deposit_stake_withdraw_authority: Pubkey,
    pub deposit_stake_validator_stake: Pubkey,
    pub deposit_stake_reserve_stake: Pubkey,
    pub deposit_stake_manager_fee: Pubkey,
    pub stake_history: Pubkey,
}
impl<'me>
    From<
        &SwapViaStakeSplStakePoolEversolStakePoolAccounts<
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
            '_,
            '_,
            '_,
        >,
    > for SwapViaStakeSplStakePoolEversolStakePoolKeys
{
    fn from(
        accounts: &SwapViaStakeSplStakePoolEversolStakePoolAccounts<
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
            '_,
            '_,
            '_,
        >,
    ) -> Self {
        Self {
            payer: *accounts.payer.key,
            user: *accounts.user.key,
            src_token_from: *accounts.src_token_from.key,
            dest_token_to: *accounts.dest_token_to.key,
            bridge_stake: *accounts.bridge_stake.key,
            dest_token_fee_token_account: *accounts.dest_token_fee_token_account.key,
            src_token_mint: *accounts.src_token_mint.key,
            dest_token_mint: *accounts.dest_token_mint.key,
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
            eversol_stake_pool_program: *accounts.eversol_stake_pool_program.key,
            deposit_stake_spl_stake_pool: *accounts.deposit_stake_spl_stake_pool.key,
            deposit_stake_validator_list: *accounts.deposit_stake_validator_list.key,
            deposit_stake_deposit_authority: *accounts.deposit_stake_deposit_authority.key,
            deposit_stake_withdraw_authority: *accounts.deposit_stake_withdraw_authority.key,
            deposit_stake_validator_stake: *accounts.deposit_stake_validator_stake.key,
            deposit_stake_reserve_stake: *accounts.deposit_stake_reserve_stake.key,
            deposit_stake_manager_fee: *accounts.deposit_stake_manager_fee.key,
            stake_history: *accounts.stake_history.key,
        }
    }
}
impl From<&SwapViaStakeSplStakePoolEversolStakePoolKeys> for [AccountMeta; 30] {
    fn from(keys: &SwapViaStakeSplStakePoolEversolStakePoolKeys) -> Self {
        [
            AccountMeta::new(keys.payer, true),
            AccountMeta::new_readonly(keys.user, true),
            AccountMeta::new(keys.src_token_from, false),
            AccountMeta::new(keys.dest_token_to, false),
            AccountMeta::new(keys.bridge_stake, false),
            AccountMeta::new(keys.dest_token_fee_token_account, false),
            AccountMeta::new(keys.src_token_mint, false),
            AccountMeta::new(keys.dest_token_mint, false),
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
    From<
        &SwapViaStakeSplStakePoolEversolStakePoolAccounts<
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
            'a,
            'a,
            'a,
        >,
    > for [AccountInfo<'a>; 30]
{
    fn from(
        accounts: &SwapViaStakeSplStakePoolEversolStakePoolAccounts<
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
            'a,
            'a,
            'a,
        >,
    ) -> Self {
        [
            accounts.payer.clone(),
            accounts.user.clone(),
            accounts.src_token_from.clone(),
            accounts.dest_token_to.clone(),
            accounts.bridge_stake.clone(),
            accounts.dest_token_fee_token_account.clone(),
            accounts.src_token_mint.clone(),
            accounts.dest_token_mint.clone(),
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
pub struct SwapViaStakeSplStakePoolEversolStakePoolIxArgs {
    pub swap_via_stake_args: SwapViaStakeArgs,
}
#[derive(Copy, Clone, Debug)]
pub struct SwapViaStakeSplStakePoolEversolStakePoolIxData<'me>(
    pub &'me SwapViaStakeSplStakePoolEversolStakePoolIxArgs,
);
pub const SWAP_VIA_STAKE_SPL_STAKE_POOL_EVERSOL_STAKE_POOL_IX_DISCM: u8 = 1u8;
impl<'me> From<&'me SwapViaStakeSplStakePoolEversolStakePoolIxArgs>
    for SwapViaStakeSplStakePoolEversolStakePoolIxData<'me>
{
    fn from(args: &'me SwapViaStakeSplStakePoolEversolStakePoolIxArgs) -> Self {
        Self(args)
    }
}
impl BorshSerialize for SwapViaStakeSplStakePoolEversolStakePoolIxData<'_> {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        writer.write_all(&[SWAP_VIA_STAKE_SPL_STAKE_POOL_EVERSOL_STAKE_POOL_IX_DISCM])?;
        self.0.serialize(writer)
    }
}
pub fn swap_via_stake_spl_stake_pool_eversol_stake_pool_ix<
    K: Into<SwapViaStakeSplStakePoolEversolStakePoolKeys>,
    A: Into<SwapViaStakeSplStakePoolEversolStakePoolIxArgs>,
>(
    accounts: K,
    args: A,
) -> std::io::Result<Instruction> {
    let keys: SwapViaStakeSplStakePoolEversolStakePoolKeys = accounts.into();
    let metas: [AccountMeta; 30] = (&keys).into();
    let args_full: SwapViaStakeSplStakePoolEversolStakePoolIxArgs = args.into();
    let data: SwapViaStakeSplStakePoolEversolStakePoolIxData = (&args_full).into();
    Ok(Instruction {
        program_id: crate::ID,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn swap_via_stake_spl_stake_pool_eversol_stake_pool_invoke<
    'a,
    A: Into<SwapViaStakeSplStakePoolEversolStakePoolIxArgs>,
>(
    accounts: &SwapViaStakeSplStakePoolEversolStakePoolAccounts<
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
        'a,
        'a,
        'a,
    >,
    args: A,
) -> ProgramResult {
    let ix = swap_via_stake_spl_stake_pool_eversol_stake_pool_ix(accounts, args)?;
    let account_info: [AccountInfo<'a>; 30] = accounts.into();
    invoke(&ix, &account_info)
}
pub fn swap_via_stake_spl_stake_pool_eversol_stake_pool_invoke_signed<
    'a,
    A: Into<SwapViaStakeSplStakePoolEversolStakePoolIxArgs>,
>(
    accounts: &SwapViaStakeSplStakePoolEversolStakePoolAccounts<
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
        'a,
        'a,
        'a,
    >,
    args: A,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let ix = swap_via_stake_spl_stake_pool_eversol_stake_pool_ix(accounts, args)?;
    let account_info: [AccountInfo<'a>; 30] = accounts.into();
    invoke_signed(&ix, &account_info, seeds)
}
#[derive(Copy, Clone, Debug)]
pub struct SwapViaStakeSplStakePoolSoceanStakePoolAccounts<
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
    'a12: 'me,
    'a13: 'me,
    'a14: 'me,
    'a15: 'me,
    'a16: 'me,
    'a17: 'me,
    'a18: 'me,
    'a19: 'me,
    'a20: 'me,
    'a21: 'me,
    'a22: 'me,
    'a23: 'me,
    'a24: 'me,
    'a25: 'me,
    'a26: 'me,
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
    pub spl_stake_pool_program: &'me AccountInfo<'a8>,
    pub withdraw_stake_spl_stake_pool: &'me AccountInfo<'a9>,
    pub withdraw_stake_validator_list: &'me AccountInfo<'a10>,
    pub withdraw_stake_withdraw_authority: &'me AccountInfo<'a11>,
    pub withdraw_stake_stake_to_split: &'me AccountInfo<'a12>,
    pub withdraw_stake_manager_fee: &'me AccountInfo<'a13>,
    pub clock: &'me AccountInfo<'a14>,
    pub token_program: &'me AccountInfo<'a15>,
    pub stake_program: &'me AccountInfo<'a16>,
    pub system_program: &'me AccountInfo<'a17>,
    pub socean_stake_pool_program: &'me AccountInfo<'a18>,
    pub deposit_stake_spl_stake_pool: &'me AccountInfo<'a19>,
    pub deposit_stake_validator_list: &'me AccountInfo<'a20>,
    pub deposit_stake_deposit_authority: &'me AccountInfo<'a21>,
    pub deposit_stake_withdraw_authority: &'me AccountInfo<'a22>,
    pub deposit_stake_validator_stake: &'me AccountInfo<'a23>,
    pub deposit_stake_reserve_stake: &'me AccountInfo<'a24>,
    pub deposit_stake_manager_fee: &'me AccountInfo<'a25>,
    pub stake_history: &'me AccountInfo<'a26>,
}
#[derive(Copy, Clone, Debug)]
pub struct SwapViaStakeSplStakePoolSoceanStakePoolKeys {
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
    pub socean_stake_pool_program: Pubkey,
    pub deposit_stake_spl_stake_pool: Pubkey,
    pub deposit_stake_validator_list: Pubkey,
    pub deposit_stake_deposit_authority: Pubkey,
    pub deposit_stake_withdraw_authority: Pubkey,
    pub deposit_stake_validator_stake: Pubkey,
    pub deposit_stake_reserve_stake: Pubkey,
    pub deposit_stake_manager_fee: Pubkey,
    pub stake_history: Pubkey,
}
impl<'me>
    From<
        &SwapViaStakeSplStakePoolSoceanStakePoolAccounts<
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
            '_,
            '_,
            '_,
        >,
    > for SwapViaStakeSplStakePoolSoceanStakePoolKeys
{
    fn from(
        accounts: &SwapViaStakeSplStakePoolSoceanStakePoolAccounts<
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
            '_,
            '_,
            '_,
        >,
    ) -> Self {
        Self {
            payer: *accounts.payer.key,
            user: *accounts.user.key,
            src_token_from: *accounts.src_token_from.key,
            dest_token_to: *accounts.dest_token_to.key,
            bridge_stake: *accounts.bridge_stake.key,
            dest_token_fee_token_account: *accounts.dest_token_fee_token_account.key,
            src_token_mint: *accounts.src_token_mint.key,
            dest_token_mint: *accounts.dest_token_mint.key,
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
            socean_stake_pool_program: *accounts.socean_stake_pool_program.key,
            deposit_stake_spl_stake_pool: *accounts.deposit_stake_spl_stake_pool.key,
            deposit_stake_validator_list: *accounts.deposit_stake_validator_list.key,
            deposit_stake_deposit_authority: *accounts.deposit_stake_deposit_authority.key,
            deposit_stake_withdraw_authority: *accounts.deposit_stake_withdraw_authority.key,
            deposit_stake_validator_stake: *accounts.deposit_stake_validator_stake.key,
            deposit_stake_reserve_stake: *accounts.deposit_stake_reserve_stake.key,
            deposit_stake_manager_fee: *accounts.deposit_stake_manager_fee.key,
            stake_history: *accounts.stake_history.key,
        }
    }
}
impl From<&SwapViaStakeSplStakePoolSoceanStakePoolKeys> for [AccountMeta; 30] {
    fn from(keys: &SwapViaStakeSplStakePoolSoceanStakePoolKeys) -> Self {
        [
            AccountMeta::new(keys.payer, true),
            AccountMeta::new_readonly(keys.user, true),
            AccountMeta::new(keys.src_token_from, false),
            AccountMeta::new(keys.dest_token_to, false),
            AccountMeta::new(keys.bridge_stake, false),
            AccountMeta::new(keys.dest_token_fee_token_account, false),
            AccountMeta::new(keys.src_token_mint, false),
            AccountMeta::new(keys.dest_token_mint, false),
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
    From<
        &SwapViaStakeSplStakePoolSoceanStakePoolAccounts<
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
            'a,
            'a,
            'a,
        >,
    > for [AccountInfo<'a>; 30]
{
    fn from(
        accounts: &SwapViaStakeSplStakePoolSoceanStakePoolAccounts<
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
            'a,
            'a,
            'a,
        >,
    ) -> Self {
        [
            accounts.payer.clone(),
            accounts.user.clone(),
            accounts.src_token_from.clone(),
            accounts.dest_token_to.clone(),
            accounts.bridge_stake.clone(),
            accounts.dest_token_fee_token_account.clone(),
            accounts.src_token_mint.clone(),
            accounts.dest_token_mint.clone(),
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
pub struct SwapViaStakeSplStakePoolSoceanStakePoolIxArgs {
    pub swap_via_stake_args: SwapViaStakeArgs,
}
#[derive(Copy, Clone, Debug)]
pub struct SwapViaStakeSplStakePoolSoceanStakePoolIxData<'me>(
    pub &'me SwapViaStakeSplStakePoolSoceanStakePoolIxArgs,
);
pub const SWAP_VIA_STAKE_SPL_STAKE_POOL_SOCEAN_STAKE_POOL_IX_DISCM: u8 = 1u8;
impl<'me> From<&'me SwapViaStakeSplStakePoolSoceanStakePoolIxArgs>
    for SwapViaStakeSplStakePoolSoceanStakePoolIxData<'me>
{
    fn from(args: &'me SwapViaStakeSplStakePoolSoceanStakePoolIxArgs) -> Self {
        Self(args)
    }
}
impl BorshSerialize for SwapViaStakeSplStakePoolSoceanStakePoolIxData<'_> {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        writer.write_all(&[SWAP_VIA_STAKE_SPL_STAKE_POOL_SOCEAN_STAKE_POOL_IX_DISCM])?;
        self.0.serialize(writer)
    }
}
pub fn swap_via_stake_spl_stake_pool_socean_stake_pool_ix<
    K: Into<SwapViaStakeSplStakePoolSoceanStakePoolKeys>,
    A: Into<SwapViaStakeSplStakePoolSoceanStakePoolIxArgs>,
>(
    accounts: K,
    args: A,
) -> std::io::Result<Instruction> {
    let keys: SwapViaStakeSplStakePoolSoceanStakePoolKeys = accounts.into();
    let metas: [AccountMeta; 30] = (&keys).into();
    let args_full: SwapViaStakeSplStakePoolSoceanStakePoolIxArgs = args.into();
    let data: SwapViaStakeSplStakePoolSoceanStakePoolIxData = (&args_full).into();
    Ok(Instruction {
        program_id: crate::ID,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn swap_via_stake_spl_stake_pool_socean_stake_pool_invoke<
    'a,
    A: Into<SwapViaStakeSplStakePoolSoceanStakePoolIxArgs>,
>(
    accounts: &SwapViaStakeSplStakePoolSoceanStakePoolAccounts<
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
        'a,
        'a,
        'a,
    >,
    args: A,
) -> ProgramResult {
    let ix = swap_via_stake_spl_stake_pool_socean_stake_pool_ix(accounts, args)?;
    let account_info: [AccountInfo<'a>; 30] = accounts.into();
    invoke(&ix, &account_info)
}
pub fn swap_via_stake_spl_stake_pool_socean_stake_pool_invoke_signed<
    'a,
    A: Into<SwapViaStakeSplStakePoolSoceanStakePoolIxArgs>,
>(
    accounts: &SwapViaStakeSplStakePoolSoceanStakePoolAccounts<
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
        'a,
        'a,
        'a,
    >,
    args: A,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let ix = swap_via_stake_spl_stake_pool_socean_stake_pool_ix(accounts, args)?;
    let account_info: [AccountInfo<'a>; 30] = accounts.into();
    invoke_signed(&ix, &account_info, seeds)
}
#[derive(Copy, Clone, Debug)]
pub struct SwapViaStakeSplStakePoolSplStakePoolAccounts<
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
    'a12: 'me,
    'a13: 'me,
    'a14: 'me,
    'a15: 'me,
    'a16: 'me,
    'a17: 'me,
    'a18: 'me,
    'a19: 'me,
    'a20: 'me,
    'a21: 'me,
    'a22: 'me,
    'a23: 'me,
    'a24: 'me,
    'a25: 'me,
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
    pub spl_stake_pool_program: &'me AccountInfo<'a8>,
    pub withdraw_stake_spl_stake_pool: &'me AccountInfo<'a9>,
    pub withdraw_stake_validator_list: &'me AccountInfo<'a10>,
    pub withdraw_stake_withdraw_authority: &'me AccountInfo<'a11>,
    pub withdraw_stake_stake_to_split: &'me AccountInfo<'a12>,
    pub withdraw_stake_manager_fee: &'me AccountInfo<'a13>,
    pub clock: &'me AccountInfo<'a14>,
    pub token_program: &'me AccountInfo<'a15>,
    pub stake_program: &'me AccountInfo<'a16>,
    pub system_program: &'me AccountInfo<'a17>,
    pub deposit_stake_spl_stake_pool: &'me AccountInfo<'a18>,
    pub deposit_stake_validator_list: &'me AccountInfo<'a19>,
    pub deposit_stake_deposit_authority: &'me AccountInfo<'a20>,
    pub deposit_stake_withdraw_authority: &'me AccountInfo<'a21>,
    pub deposit_stake_validator_stake: &'me AccountInfo<'a22>,
    pub deposit_stake_reserve_stake: &'me AccountInfo<'a23>,
    pub deposit_stake_manager_fee: &'me AccountInfo<'a24>,
    pub stake_history: &'me AccountInfo<'a25>,
}
#[derive(Copy, Clone, Debug)]
pub struct SwapViaStakeSplStakePoolSplStakePoolKeys {
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
    pub deposit_stake_spl_stake_pool: Pubkey,
    pub deposit_stake_validator_list: Pubkey,
    pub deposit_stake_deposit_authority: Pubkey,
    pub deposit_stake_withdraw_authority: Pubkey,
    pub deposit_stake_validator_stake: Pubkey,
    pub deposit_stake_reserve_stake: Pubkey,
    pub deposit_stake_manager_fee: Pubkey,
    pub stake_history: Pubkey,
}
impl<'me>
    From<
        &SwapViaStakeSplStakePoolSplStakePoolAccounts<
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
            '_,
            '_,
        >,
    > for SwapViaStakeSplStakePoolSplStakePoolKeys
{
    fn from(
        accounts: &SwapViaStakeSplStakePoolSplStakePoolAccounts<
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
            '_,
            '_,
        >,
    ) -> Self {
        Self {
            payer: *accounts.payer.key,
            user: *accounts.user.key,
            src_token_from: *accounts.src_token_from.key,
            dest_token_to: *accounts.dest_token_to.key,
            bridge_stake: *accounts.bridge_stake.key,
            dest_token_fee_token_account: *accounts.dest_token_fee_token_account.key,
            src_token_mint: *accounts.src_token_mint.key,
            dest_token_mint: *accounts.dest_token_mint.key,
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
            deposit_stake_spl_stake_pool: *accounts.deposit_stake_spl_stake_pool.key,
            deposit_stake_validator_list: *accounts.deposit_stake_validator_list.key,
            deposit_stake_deposit_authority: *accounts.deposit_stake_deposit_authority.key,
            deposit_stake_withdraw_authority: *accounts.deposit_stake_withdraw_authority.key,
            deposit_stake_validator_stake: *accounts.deposit_stake_validator_stake.key,
            deposit_stake_reserve_stake: *accounts.deposit_stake_reserve_stake.key,
            deposit_stake_manager_fee: *accounts.deposit_stake_manager_fee.key,
            stake_history: *accounts.stake_history.key,
        }
    }
}
impl From<&SwapViaStakeSplStakePoolSplStakePoolKeys> for [AccountMeta; 30] {
    fn from(keys: &SwapViaStakeSplStakePoolSplStakePoolKeys) -> Self {
        [
            AccountMeta::new(keys.payer, true),
            AccountMeta::new_readonly(keys.user, true),
            AccountMeta::new(keys.src_token_from, false),
            AccountMeta::new(keys.dest_token_to, false),
            AccountMeta::new(keys.bridge_stake, false),
            AccountMeta::new(keys.dest_token_fee_token_account, false),
            AccountMeta::new(keys.src_token_mint, false),
            AccountMeta::new(keys.dest_token_mint, false),
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
impl<'a>
    From<
        &SwapViaStakeSplStakePoolSplStakePoolAccounts<
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
            'a,
            'a,
        >,
    > for [AccountInfo<'a>; 30]
{
    fn from(
        accounts: &SwapViaStakeSplStakePoolSplStakePoolAccounts<
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
            'a,
            'a,
        >,
    ) -> Self {
        [
            accounts.payer.clone(),
            accounts.user.clone(),
            accounts.src_token_from.clone(),
            accounts.dest_token_to.clone(),
            accounts.bridge_stake.clone(),
            accounts.dest_token_fee_token_account.clone(),
            accounts.src_token_mint.clone(),
            accounts.dest_token_mint.clone(),
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
pub struct SwapViaStakeSplStakePoolSplStakePoolIxArgs {
    pub swap_via_stake_args: SwapViaStakeArgs,
}
#[derive(Copy, Clone, Debug)]
pub struct SwapViaStakeSplStakePoolSplStakePoolIxData<'me>(
    pub &'me SwapViaStakeSplStakePoolSplStakePoolIxArgs,
);
pub const SWAP_VIA_STAKE_SPL_STAKE_POOL_SPL_STAKE_POOL_IX_DISCM: u8 = 1u8;
impl<'me> From<&'me SwapViaStakeSplStakePoolSplStakePoolIxArgs>
    for SwapViaStakeSplStakePoolSplStakePoolIxData<'me>
{
    fn from(args: &'me SwapViaStakeSplStakePoolSplStakePoolIxArgs) -> Self {
        Self(args)
    }
}
impl BorshSerialize for SwapViaStakeSplStakePoolSplStakePoolIxData<'_> {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        writer.write_all(&[SWAP_VIA_STAKE_SPL_STAKE_POOL_SPL_STAKE_POOL_IX_DISCM])?;
        self.0.serialize(writer)
    }
}
pub fn swap_via_stake_spl_stake_pool_spl_stake_pool_ix<
    K: Into<SwapViaStakeSplStakePoolSplStakePoolKeys>,
    A: Into<SwapViaStakeSplStakePoolSplStakePoolIxArgs>,
>(
    accounts: K,
    args: A,
) -> std::io::Result<Instruction> {
    let keys: SwapViaStakeSplStakePoolSplStakePoolKeys = accounts.into();
    let metas: [AccountMeta; 30] = (&keys).into();
    let args_full: SwapViaStakeSplStakePoolSplStakePoolIxArgs = args.into();
    let data: SwapViaStakeSplStakePoolSplStakePoolIxData = (&args_full).into();
    Ok(Instruction {
        program_id: crate::ID,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn swap_via_stake_spl_stake_pool_spl_stake_pool_invoke<
    'a,
    A: Into<SwapViaStakeSplStakePoolSplStakePoolIxArgs>,
>(
    accounts: &SwapViaStakeSplStakePoolSplStakePoolAccounts<
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
        'a,
        'a,
    >,
    args: A,
) -> ProgramResult {
    let ix = swap_via_stake_spl_stake_pool_spl_stake_pool_ix(accounts, args)?;
    let account_info: [AccountInfo<'a>; 30] = accounts.into();
    invoke(&ix, &account_info)
}
pub fn swap_via_stake_spl_stake_pool_spl_stake_pool_invoke_signed<
    'a,
    A: Into<SwapViaStakeSplStakePoolSplStakePoolIxArgs>,
>(
    accounts: &SwapViaStakeSplStakePoolSplStakePoolAccounts<
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
        'a,
        'a,
    >,
    args: A,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let ix = swap_via_stake_spl_stake_pool_spl_stake_pool_ix(accounts, args)?;
    let account_info: [AccountInfo<'a>; 30] = accounts.into();
    invoke_signed(&ix, &account_info, seeds)
}
#[derive(Copy, Clone, Debug)]
pub struct SwapViaStakeSplStakePoolMarinadeAccounts<
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
    'a12: 'me,
    'a13: 'me,
    'a14: 'me,
    'a15: 'me,
    'a16: 'me,
    'a17: 'me,
    'a18: 'me,
    'a19: 'me,
    'a20: 'me,
    'a21: 'me,
    'a22: 'me,
    'a23: 'me,
    'a24: 'me,
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
    pub spl_stake_pool_program: &'me AccountInfo<'a8>,
    pub withdraw_stake_spl_stake_pool: &'me AccountInfo<'a9>,
    pub withdraw_stake_validator_list: &'me AccountInfo<'a10>,
    pub withdraw_stake_withdraw_authority: &'me AccountInfo<'a11>,
    pub withdraw_stake_stake_to_split: &'me AccountInfo<'a12>,
    pub withdraw_stake_manager_fee: &'me AccountInfo<'a13>,
    pub clock: &'me AccountInfo<'a14>,
    pub token_program: &'me AccountInfo<'a15>,
    pub stake_program: &'me AccountInfo<'a16>,
    pub system_program: &'me AccountInfo<'a17>,
    pub marinade_program: &'me AccountInfo<'a18>,
    pub deposit_stake_marinade_state: &'me AccountInfo<'a19>,
    pub deposit_stake_validator_list: &'me AccountInfo<'a20>,
    pub deposit_stake_stake_list: &'me AccountInfo<'a21>,
    pub deposit_stake_duplication_flag: &'me AccountInfo<'a22>,
    pub deposit_stake_msol_mint_auth: &'me AccountInfo<'a23>,
    pub rent: &'me AccountInfo<'a24>,
}
#[derive(Copy, Clone, Debug)]
pub struct SwapViaStakeSplStakePoolMarinadeKeys {
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
    pub marinade_program: Pubkey,
    pub deposit_stake_marinade_state: Pubkey,
    pub deposit_stake_validator_list: Pubkey,
    pub deposit_stake_stake_list: Pubkey,
    pub deposit_stake_duplication_flag: Pubkey,
    pub deposit_stake_msol_mint_auth: Pubkey,
    pub rent: Pubkey,
}
impl<'me>
    From<
        &SwapViaStakeSplStakePoolMarinadeAccounts<
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
            '_,
        >,
    > for SwapViaStakeSplStakePoolMarinadeKeys
{
    fn from(
        accounts: &SwapViaStakeSplStakePoolMarinadeAccounts<
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
            '_,
        >,
    ) -> Self {
        Self {
            payer: *accounts.payer.key,
            user: *accounts.user.key,
            src_token_from: *accounts.src_token_from.key,
            dest_token_to: *accounts.dest_token_to.key,
            bridge_stake: *accounts.bridge_stake.key,
            dest_token_fee_token_account: *accounts.dest_token_fee_token_account.key,
            src_token_mint: *accounts.src_token_mint.key,
            dest_token_mint: *accounts.dest_token_mint.key,
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
            marinade_program: *accounts.marinade_program.key,
            deposit_stake_marinade_state: *accounts.deposit_stake_marinade_state.key,
            deposit_stake_validator_list: *accounts.deposit_stake_validator_list.key,
            deposit_stake_stake_list: *accounts.deposit_stake_stake_list.key,
            deposit_stake_duplication_flag: *accounts.deposit_stake_duplication_flag.key,
            deposit_stake_msol_mint_auth: *accounts.deposit_stake_msol_mint_auth.key,
            rent: *accounts.rent.key,
        }
    }
}
impl From<&SwapViaStakeSplStakePoolMarinadeKeys> for [AccountMeta; 29] {
    fn from(keys: &SwapViaStakeSplStakePoolMarinadeKeys) -> Self {
        [
            AccountMeta::new(keys.payer, true),
            AccountMeta::new_readonly(keys.user, true),
            AccountMeta::new(keys.src_token_from, false),
            AccountMeta::new(keys.dest_token_to, false),
            AccountMeta::new(keys.bridge_stake, false),
            AccountMeta::new(keys.dest_token_fee_token_account, false),
            AccountMeta::new(keys.src_token_mint, false),
            AccountMeta::new(keys.dest_token_mint, false),
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
impl<'a>
    From<
        &SwapViaStakeSplStakePoolMarinadeAccounts<
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
            'a,
        >,
    > for [AccountInfo<'a>; 29]
{
    fn from(
        accounts: &SwapViaStakeSplStakePoolMarinadeAccounts<
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
            'a,
        >,
    ) -> Self {
        [
            accounts.payer.clone(),
            accounts.user.clone(),
            accounts.src_token_from.clone(),
            accounts.dest_token_to.clone(),
            accounts.bridge_stake.clone(),
            accounts.dest_token_fee_token_account.clone(),
            accounts.src_token_mint.clone(),
            accounts.dest_token_mint.clone(),
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
pub struct SwapViaStakeSplStakePoolMarinadeIxArgs {
    pub swap_via_stake_args: SwapViaStakeArgs,
}
#[derive(Copy, Clone, Debug)]
pub struct SwapViaStakeSplStakePoolMarinadeIxData<'me>(
    pub &'me SwapViaStakeSplStakePoolMarinadeIxArgs,
);
pub const SWAP_VIA_STAKE_SPL_STAKE_POOL_MARINADE_IX_DISCM: u8 = 1u8;
impl<'me> From<&'me SwapViaStakeSplStakePoolMarinadeIxArgs>
    for SwapViaStakeSplStakePoolMarinadeIxData<'me>
{
    fn from(args: &'me SwapViaStakeSplStakePoolMarinadeIxArgs) -> Self {
        Self(args)
    }
}
impl BorshSerialize for SwapViaStakeSplStakePoolMarinadeIxData<'_> {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        writer.write_all(&[SWAP_VIA_STAKE_SPL_STAKE_POOL_MARINADE_IX_DISCM])?;
        self.0.serialize(writer)
    }
}
pub fn swap_via_stake_spl_stake_pool_marinade_ix<
    K: Into<SwapViaStakeSplStakePoolMarinadeKeys>,
    A: Into<SwapViaStakeSplStakePoolMarinadeIxArgs>,
>(
    accounts: K,
    args: A,
) -> std::io::Result<Instruction> {
    let keys: SwapViaStakeSplStakePoolMarinadeKeys = accounts.into();
    let metas: [AccountMeta; 29] = (&keys).into();
    let args_full: SwapViaStakeSplStakePoolMarinadeIxArgs = args.into();
    let data: SwapViaStakeSplStakePoolMarinadeIxData = (&args_full).into();
    Ok(Instruction {
        program_id: crate::ID,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn swap_via_stake_spl_stake_pool_marinade_invoke<
    'a,
    A: Into<SwapViaStakeSplStakePoolMarinadeIxArgs>,
>(
    accounts: &SwapViaStakeSplStakePoolMarinadeAccounts<
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
        'a,
    >,
    args: A,
) -> ProgramResult {
    let ix = swap_via_stake_spl_stake_pool_marinade_ix(accounts, args)?;
    let account_info: [AccountInfo<'a>; 29] = accounts.into();
    invoke(&ix, &account_info)
}
pub fn swap_via_stake_spl_stake_pool_marinade_invoke_signed<
    'a,
    A: Into<SwapViaStakeSplStakePoolMarinadeIxArgs>,
>(
    accounts: &SwapViaStakeSplStakePoolMarinadeAccounts<
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
        'a,
    >,
    args: A,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let ix = swap_via_stake_spl_stake_pool_marinade_ix(accounts, args)?;
    let account_info: [AccountInfo<'a>; 29] = accounts.into();
    invoke_signed(&ix, &account_info, seeds)
}
#[derive(Copy, Clone, Debug)]
pub struct SwapViaStakeSplStakePoolUnstakeItAccounts<
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
    'a12: 'me,
    'a13: 'me,
    'a14: 'me,
    'a15: 'me,
    'a16: 'me,
    'a17: 'me,
    'a18: 'me,
    'a19: 'me,
    'a20: 'me,
    'a21: 'me,
    'a22: 'me,
    'a23: 'me,
    'a24: 'me,
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
    pub spl_stake_pool_program: &'me AccountInfo<'a8>,
    pub withdraw_stake_spl_stake_pool: &'me AccountInfo<'a9>,
    pub withdraw_stake_validator_list: &'me AccountInfo<'a10>,
    pub withdraw_stake_withdraw_authority: &'me AccountInfo<'a11>,
    pub withdraw_stake_stake_to_split: &'me AccountInfo<'a12>,
    pub withdraw_stake_manager_fee: &'me AccountInfo<'a13>,
    pub clock: &'me AccountInfo<'a14>,
    pub token_program: &'me AccountInfo<'a15>,
    pub stake_program: &'me AccountInfo<'a16>,
    pub system_program: &'me AccountInfo<'a17>,
    pub unstakeit_program: &'me AccountInfo<'a18>,
    pub deposit_stake_unstake_pool: &'me AccountInfo<'a19>,
    pub deposit_stake_pool_sol_reserves: &'me AccountInfo<'a20>,
    pub deposit_stake_unstake_fee: &'me AccountInfo<'a21>,
    pub deposit_stake_stake_acc_record: &'me AccountInfo<'a22>,
    pub deposit_stake_protocol_fee: &'me AccountInfo<'a23>,
    pub deposit_stake_protocol_fee_dest: &'me AccountInfo<'a24>,
}
#[derive(Copy, Clone, Debug)]
pub struct SwapViaStakeSplStakePoolUnstakeItKeys {
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
    pub unstakeit_program: Pubkey,
    pub deposit_stake_unstake_pool: Pubkey,
    pub deposit_stake_pool_sol_reserves: Pubkey,
    pub deposit_stake_unstake_fee: Pubkey,
    pub deposit_stake_stake_acc_record: Pubkey,
    pub deposit_stake_protocol_fee: Pubkey,
    pub deposit_stake_protocol_fee_dest: Pubkey,
}
impl<'me>
    From<
        &SwapViaStakeSplStakePoolUnstakeItAccounts<
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
            '_,
        >,
    > for SwapViaStakeSplStakePoolUnstakeItKeys
{
    fn from(
        accounts: &SwapViaStakeSplStakePoolUnstakeItAccounts<
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
            '_,
        >,
    ) -> Self {
        Self {
            payer: *accounts.payer.key,
            user: *accounts.user.key,
            src_token_from: *accounts.src_token_from.key,
            dest_token_to: *accounts.dest_token_to.key,
            bridge_stake: *accounts.bridge_stake.key,
            dest_token_fee_token_account: *accounts.dest_token_fee_token_account.key,
            src_token_mint: *accounts.src_token_mint.key,
            dest_token_mint: *accounts.dest_token_mint.key,
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
            unstakeit_program: *accounts.unstakeit_program.key,
            deposit_stake_unstake_pool: *accounts.deposit_stake_unstake_pool.key,
            deposit_stake_pool_sol_reserves: *accounts.deposit_stake_pool_sol_reserves.key,
            deposit_stake_unstake_fee: *accounts.deposit_stake_unstake_fee.key,
            deposit_stake_stake_acc_record: *accounts.deposit_stake_stake_acc_record.key,
            deposit_stake_protocol_fee: *accounts.deposit_stake_protocol_fee.key,
            deposit_stake_protocol_fee_dest: *accounts.deposit_stake_protocol_fee_dest.key,
        }
    }
}
impl From<&SwapViaStakeSplStakePoolUnstakeItKeys> for [AccountMeta; 29] {
    fn from(keys: &SwapViaStakeSplStakePoolUnstakeItKeys) -> Self {
        [
            AccountMeta::new(keys.payer, true),
            AccountMeta::new_readonly(keys.user, true),
            AccountMeta::new(keys.src_token_from, false),
            AccountMeta::new(keys.dest_token_to, false),
            AccountMeta::new(keys.bridge_stake, false),
            AccountMeta::new(keys.dest_token_fee_token_account, false),
            AccountMeta::new(keys.src_token_mint, false),
            AccountMeta::new(keys.dest_token_mint, false),
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
impl<'a>
    From<
        &SwapViaStakeSplStakePoolUnstakeItAccounts<
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
            'a,
        >,
    > for [AccountInfo<'a>; 29]
{
    fn from(
        accounts: &SwapViaStakeSplStakePoolUnstakeItAccounts<
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
            'a,
        >,
    ) -> Self {
        [
            accounts.payer.clone(),
            accounts.user.clone(),
            accounts.src_token_from.clone(),
            accounts.dest_token_to.clone(),
            accounts.bridge_stake.clone(),
            accounts.dest_token_fee_token_account.clone(),
            accounts.src_token_mint.clone(),
            accounts.dest_token_mint.clone(),
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
pub struct SwapViaStakeSplStakePoolUnstakeItIxArgs {
    pub swap_via_stake_args: SwapViaStakeArgs,
}
#[derive(Copy, Clone, Debug)]
pub struct SwapViaStakeSplStakePoolUnstakeItIxData<'me>(
    pub &'me SwapViaStakeSplStakePoolUnstakeItIxArgs,
);
pub const SWAP_VIA_STAKE_SPL_STAKE_POOL_UNSTAKE_IT_IX_DISCM: u8 = 1u8;
impl<'me> From<&'me SwapViaStakeSplStakePoolUnstakeItIxArgs>
    for SwapViaStakeSplStakePoolUnstakeItIxData<'me>
{
    fn from(args: &'me SwapViaStakeSplStakePoolUnstakeItIxArgs) -> Self {
        Self(args)
    }
}
impl BorshSerialize for SwapViaStakeSplStakePoolUnstakeItIxData<'_> {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        writer.write_all(&[SWAP_VIA_STAKE_SPL_STAKE_POOL_UNSTAKE_IT_IX_DISCM])?;
        self.0.serialize(writer)
    }
}
pub fn swap_via_stake_spl_stake_pool_unstake_it_ix<
    K: Into<SwapViaStakeSplStakePoolUnstakeItKeys>,
    A: Into<SwapViaStakeSplStakePoolUnstakeItIxArgs>,
>(
    accounts: K,
    args: A,
) -> std::io::Result<Instruction> {
    let keys: SwapViaStakeSplStakePoolUnstakeItKeys = accounts.into();
    let metas: [AccountMeta; 29] = (&keys).into();
    let args_full: SwapViaStakeSplStakePoolUnstakeItIxArgs = args.into();
    let data: SwapViaStakeSplStakePoolUnstakeItIxData = (&args_full).into();
    Ok(Instruction {
        program_id: crate::ID,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn swap_via_stake_spl_stake_pool_unstake_it_invoke<
    'a,
    A: Into<SwapViaStakeSplStakePoolUnstakeItIxArgs>,
>(
    accounts: &SwapViaStakeSplStakePoolUnstakeItAccounts<
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
        'a,
    >,
    args: A,
) -> ProgramResult {
    let ix = swap_via_stake_spl_stake_pool_unstake_it_ix(accounts, args)?;
    let account_info: [AccountInfo<'a>; 29] = accounts.into();
    invoke(&ix, &account_info)
}
pub fn swap_via_stake_spl_stake_pool_unstake_it_invoke_signed<
    'a,
    A: Into<SwapViaStakeSplStakePoolUnstakeItIxArgs>,
>(
    accounts: &SwapViaStakeSplStakePoolUnstakeItAccounts<
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
        'a,
    >,
    args: A,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let ix = swap_via_stake_spl_stake_pool_unstake_it_ix(accounts, args)?;
    let account_info: [AccountInfo<'a>; 29] = accounts.into();
    invoke_signed(&ix, &account_info, seeds)
}
#[derive(Copy, Clone, Debug)]
pub struct SwapViaStakeLidoEversolStakePoolAccounts<
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
    'a12: 'me,
    'a13: 'me,
    'a14: 'me,
    'a15: 'me,
    'a16: 'me,
    'a17: 'me,
    'a18: 'me,
    'a19: 'me,
    'a20: 'me,
    'a21: 'me,
    'a22: 'me,
    'a23: 'me,
    'a24: 'me,
    'a25: 'me,
    'a26: 'me,
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
    pub lido_program: &'me AccountInfo<'a8>,
    pub withdraw_stake_solido: &'me AccountInfo<'a9>,
    pub withdraw_stake_voter: &'me AccountInfo<'a10>,
    pub withdraw_stake_stake_to_split: &'me AccountInfo<'a11>,
    pub withdraw_stake_stake_authority: &'me AccountInfo<'a12>,
    pub withdraw_stake_validator_list: &'me AccountInfo<'a13>,
    pub clock: &'me AccountInfo<'a14>,
    pub token_program: &'me AccountInfo<'a15>,
    pub stake_program: &'me AccountInfo<'a16>,
    pub system_program: &'me AccountInfo<'a17>,
    pub eversol_stake_pool_program: &'me AccountInfo<'a18>,
    pub deposit_stake_spl_stake_pool: &'me AccountInfo<'a19>,
    pub deposit_stake_validator_list: &'me AccountInfo<'a20>,
    pub deposit_stake_deposit_authority: &'me AccountInfo<'a21>,
    pub deposit_stake_withdraw_authority: &'me AccountInfo<'a22>,
    pub deposit_stake_validator_stake: &'me AccountInfo<'a23>,
    pub deposit_stake_reserve_stake: &'me AccountInfo<'a24>,
    pub deposit_stake_manager_fee: &'me AccountInfo<'a25>,
    pub stake_history: &'me AccountInfo<'a26>,
}
#[derive(Copy, Clone, Debug)]
pub struct SwapViaStakeLidoEversolStakePoolKeys {
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
    pub eversol_stake_pool_program: Pubkey,
    pub deposit_stake_spl_stake_pool: Pubkey,
    pub deposit_stake_validator_list: Pubkey,
    pub deposit_stake_deposit_authority: Pubkey,
    pub deposit_stake_withdraw_authority: Pubkey,
    pub deposit_stake_validator_stake: Pubkey,
    pub deposit_stake_reserve_stake: Pubkey,
    pub deposit_stake_manager_fee: Pubkey,
    pub stake_history: Pubkey,
}
impl<'me>
    From<
        &SwapViaStakeLidoEversolStakePoolAccounts<
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
            '_,
            '_,
            '_,
        >,
    > for SwapViaStakeLidoEversolStakePoolKeys
{
    fn from(
        accounts: &SwapViaStakeLidoEversolStakePoolAccounts<
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
            '_,
            '_,
            '_,
        >,
    ) -> Self {
        Self {
            payer: *accounts.payer.key,
            user: *accounts.user.key,
            src_token_from: *accounts.src_token_from.key,
            dest_token_to: *accounts.dest_token_to.key,
            bridge_stake: *accounts.bridge_stake.key,
            dest_token_fee_token_account: *accounts.dest_token_fee_token_account.key,
            src_token_mint: *accounts.src_token_mint.key,
            dest_token_mint: *accounts.dest_token_mint.key,
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
            eversol_stake_pool_program: *accounts.eversol_stake_pool_program.key,
            deposit_stake_spl_stake_pool: *accounts.deposit_stake_spl_stake_pool.key,
            deposit_stake_validator_list: *accounts.deposit_stake_validator_list.key,
            deposit_stake_deposit_authority: *accounts.deposit_stake_deposit_authority.key,
            deposit_stake_withdraw_authority: *accounts.deposit_stake_withdraw_authority.key,
            deposit_stake_validator_stake: *accounts.deposit_stake_validator_stake.key,
            deposit_stake_reserve_stake: *accounts.deposit_stake_reserve_stake.key,
            deposit_stake_manager_fee: *accounts.deposit_stake_manager_fee.key,
            stake_history: *accounts.stake_history.key,
        }
    }
}
impl From<&SwapViaStakeLidoEversolStakePoolKeys> for [AccountMeta; 30] {
    fn from(keys: &SwapViaStakeLidoEversolStakePoolKeys) -> Self {
        [
            AccountMeta::new(keys.payer, true),
            AccountMeta::new_readonly(keys.user, true),
            AccountMeta::new(keys.src_token_from, false),
            AccountMeta::new(keys.dest_token_to, false),
            AccountMeta::new(keys.bridge_stake, false),
            AccountMeta::new(keys.dest_token_fee_token_account, false),
            AccountMeta::new(keys.src_token_mint, false),
            AccountMeta::new(keys.dest_token_mint, false),
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
    From<
        &SwapViaStakeLidoEversolStakePoolAccounts<
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
            'a,
            'a,
            'a,
        >,
    > for [AccountInfo<'a>; 30]
{
    fn from(
        accounts: &SwapViaStakeLidoEversolStakePoolAccounts<
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
            'a,
            'a,
            'a,
        >,
    ) -> Self {
        [
            accounts.payer.clone(),
            accounts.user.clone(),
            accounts.src_token_from.clone(),
            accounts.dest_token_to.clone(),
            accounts.bridge_stake.clone(),
            accounts.dest_token_fee_token_account.clone(),
            accounts.src_token_mint.clone(),
            accounts.dest_token_mint.clone(),
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
pub struct SwapViaStakeLidoEversolStakePoolIxArgs {
    pub swap_via_stake_args: SwapViaStakeArgs,
}
#[derive(Copy, Clone, Debug)]
pub struct SwapViaStakeLidoEversolStakePoolIxData<'me>(
    pub &'me SwapViaStakeLidoEversolStakePoolIxArgs,
);
pub const SWAP_VIA_STAKE_LIDO_EVERSOL_STAKE_POOL_IX_DISCM: u8 = 1u8;
impl<'me> From<&'me SwapViaStakeLidoEversolStakePoolIxArgs>
    for SwapViaStakeLidoEversolStakePoolIxData<'me>
{
    fn from(args: &'me SwapViaStakeLidoEversolStakePoolIxArgs) -> Self {
        Self(args)
    }
}
impl BorshSerialize for SwapViaStakeLidoEversolStakePoolIxData<'_> {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        writer.write_all(&[SWAP_VIA_STAKE_LIDO_EVERSOL_STAKE_POOL_IX_DISCM])?;
        self.0.serialize(writer)
    }
}
pub fn swap_via_stake_lido_eversol_stake_pool_ix<
    K: Into<SwapViaStakeLidoEversolStakePoolKeys>,
    A: Into<SwapViaStakeLidoEversolStakePoolIxArgs>,
>(
    accounts: K,
    args: A,
) -> std::io::Result<Instruction> {
    let keys: SwapViaStakeLidoEversolStakePoolKeys = accounts.into();
    let metas: [AccountMeta; 30] = (&keys).into();
    let args_full: SwapViaStakeLidoEversolStakePoolIxArgs = args.into();
    let data: SwapViaStakeLidoEversolStakePoolIxData = (&args_full).into();
    Ok(Instruction {
        program_id: crate::ID,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn swap_via_stake_lido_eversol_stake_pool_invoke<
    'a,
    A: Into<SwapViaStakeLidoEversolStakePoolIxArgs>,
>(
    accounts: &SwapViaStakeLidoEversolStakePoolAccounts<
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
        'a,
        'a,
        'a,
    >,
    args: A,
) -> ProgramResult {
    let ix = swap_via_stake_lido_eversol_stake_pool_ix(accounts, args)?;
    let account_info: [AccountInfo<'a>; 30] = accounts.into();
    invoke(&ix, &account_info)
}
pub fn swap_via_stake_lido_eversol_stake_pool_invoke_signed<
    'a,
    A: Into<SwapViaStakeLidoEversolStakePoolIxArgs>,
>(
    accounts: &SwapViaStakeLidoEversolStakePoolAccounts<
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
        'a,
        'a,
        'a,
    >,
    args: A,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let ix = swap_via_stake_lido_eversol_stake_pool_ix(accounts, args)?;
    let account_info: [AccountInfo<'a>; 30] = accounts.into();
    invoke_signed(&ix, &account_info, seeds)
}
#[derive(Copy, Clone, Debug)]
pub struct SwapViaStakeLidoSoceanStakePoolAccounts<
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
    'a12: 'me,
    'a13: 'me,
    'a14: 'me,
    'a15: 'me,
    'a16: 'me,
    'a17: 'me,
    'a18: 'me,
    'a19: 'me,
    'a20: 'me,
    'a21: 'me,
    'a22: 'me,
    'a23: 'me,
    'a24: 'me,
    'a25: 'me,
    'a26: 'me,
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
    pub lido_program: &'me AccountInfo<'a8>,
    pub withdraw_stake_solido: &'me AccountInfo<'a9>,
    pub withdraw_stake_voter: &'me AccountInfo<'a10>,
    pub withdraw_stake_stake_to_split: &'me AccountInfo<'a11>,
    pub withdraw_stake_stake_authority: &'me AccountInfo<'a12>,
    pub withdraw_stake_validator_list: &'me AccountInfo<'a13>,
    pub clock: &'me AccountInfo<'a14>,
    pub token_program: &'me AccountInfo<'a15>,
    pub stake_program: &'me AccountInfo<'a16>,
    pub system_program: &'me AccountInfo<'a17>,
    pub socean_stake_pool_program: &'me AccountInfo<'a18>,
    pub deposit_stake_spl_stake_pool: &'me AccountInfo<'a19>,
    pub deposit_stake_validator_list: &'me AccountInfo<'a20>,
    pub deposit_stake_deposit_authority: &'me AccountInfo<'a21>,
    pub deposit_stake_withdraw_authority: &'me AccountInfo<'a22>,
    pub deposit_stake_validator_stake: &'me AccountInfo<'a23>,
    pub deposit_stake_reserve_stake: &'me AccountInfo<'a24>,
    pub deposit_stake_manager_fee: &'me AccountInfo<'a25>,
    pub stake_history: &'me AccountInfo<'a26>,
}
#[derive(Copy, Clone, Debug)]
pub struct SwapViaStakeLidoSoceanStakePoolKeys {
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
    pub socean_stake_pool_program: Pubkey,
    pub deposit_stake_spl_stake_pool: Pubkey,
    pub deposit_stake_validator_list: Pubkey,
    pub deposit_stake_deposit_authority: Pubkey,
    pub deposit_stake_withdraw_authority: Pubkey,
    pub deposit_stake_validator_stake: Pubkey,
    pub deposit_stake_reserve_stake: Pubkey,
    pub deposit_stake_manager_fee: Pubkey,
    pub stake_history: Pubkey,
}
impl<'me>
    From<
        &SwapViaStakeLidoSoceanStakePoolAccounts<
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
            '_,
            '_,
            '_,
        >,
    > for SwapViaStakeLidoSoceanStakePoolKeys
{
    fn from(
        accounts: &SwapViaStakeLidoSoceanStakePoolAccounts<
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
            '_,
            '_,
            '_,
        >,
    ) -> Self {
        Self {
            payer: *accounts.payer.key,
            user: *accounts.user.key,
            src_token_from: *accounts.src_token_from.key,
            dest_token_to: *accounts.dest_token_to.key,
            bridge_stake: *accounts.bridge_stake.key,
            dest_token_fee_token_account: *accounts.dest_token_fee_token_account.key,
            src_token_mint: *accounts.src_token_mint.key,
            dest_token_mint: *accounts.dest_token_mint.key,
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
            socean_stake_pool_program: *accounts.socean_stake_pool_program.key,
            deposit_stake_spl_stake_pool: *accounts.deposit_stake_spl_stake_pool.key,
            deposit_stake_validator_list: *accounts.deposit_stake_validator_list.key,
            deposit_stake_deposit_authority: *accounts.deposit_stake_deposit_authority.key,
            deposit_stake_withdraw_authority: *accounts.deposit_stake_withdraw_authority.key,
            deposit_stake_validator_stake: *accounts.deposit_stake_validator_stake.key,
            deposit_stake_reserve_stake: *accounts.deposit_stake_reserve_stake.key,
            deposit_stake_manager_fee: *accounts.deposit_stake_manager_fee.key,
            stake_history: *accounts.stake_history.key,
        }
    }
}
impl From<&SwapViaStakeLidoSoceanStakePoolKeys> for [AccountMeta; 30] {
    fn from(keys: &SwapViaStakeLidoSoceanStakePoolKeys) -> Self {
        [
            AccountMeta::new(keys.payer, true),
            AccountMeta::new_readonly(keys.user, true),
            AccountMeta::new(keys.src_token_from, false),
            AccountMeta::new(keys.dest_token_to, false),
            AccountMeta::new(keys.bridge_stake, false),
            AccountMeta::new(keys.dest_token_fee_token_account, false),
            AccountMeta::new(keys.src_token_mint, false),
            AccountMeta::new(keys.dest_token_mint, false),
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
    From<
        &SwapViaStakeLidoSoceanStakePoolAccounts<
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
            'a,
            'a,
            'a,
        >,
    > for [AccountInfo<'a>; 30]
{
    fn from(
        accounts: &SwapViaStakeLidoSoceanStakePoolAccounts<
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
            'a,
            'a,
            'a,
        >,
    ) -> Self {
        [
            accounts.payer.clone(),
            accounts.user.clone(),
            accounts.src_token_from.clone(),
            accounts.dest_token_to.clone(),
            accounts.bridge_stake.clone(),
            accounts.dest_token_fee_token_account.clone(),
            accounts.src_token_mint.clone(),
            accounts.dest_token_mint.clone(),
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
pub struct SwapViaStakeLidoSoceanStakePoolIxArgs {
    pub swap_via_stake_args: SwapViaStakeArgs,
}
#[derive(Copy, Clone, Debug)]
pub struct SwapViaStakeLidoSoceanStakePoolIxData<'me>(
    pub &'me SwapViaStakeLidoSoceanStakePoolIxArgs,
);
pub const SWAP_VIA_STAKE_LIDO_SOCEAN_STAKE_POOL_IX_DISCM: u8 = 1u8;
impl<'me> From<&'me SwapViaStakeLidoSoceanStakePoolIxArgs>
    for SwapViaStakeLidoSoceanStakePoolIxData<'me>
{
    fn from(args: &'me SwapViaStakeLidoSoceanStakePoolIxArgs) -> Self {
        Self(args)
    }
}
impl BorshSerialize for SwapViaStakeLidoSoceanStakePoolIxData<'_> {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        writer.write_all(&[SWAP_VIA_STAKE_LIDO_SOCEAN_STAKE_POOL_IX_DISCM])?;
        self.0.serialize(writer)
    }
}
pub fn swap_via_stake_lido_socean_stake_pool_ix<
    K: Into<SwapViaStakeLidoSoceanStakePoolKeys>,
    A: Into<SwapViaStakeLidoSoceanStakePoolIxArgs>,
>(
    accounts: K,
    args: A,
) -> std::io::Result<Instruction> {
    let keys: SwapViaStakeLidoSoceanStakePoolKeys = accounts.into();
    let metas: [AccountMeta; 30] = (&keys).into();
    let args_full: SwapViaStakeLidoSoceanStakePoolIxArgs = args.into();
    let data: SwapViaStakeLidoSoceanStakePoolIxData = (&args_full).into();
    Ok(Instruction {
        program_id: crate::ID,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn swap_via_stake_lido_socean_stake_pool_invoke<
    'a,
    A: Into<SwapViaStakeLidoSoceanStakePoolIxArgs>,
>(
    accounts: &SwapViaStakeLidoSoceanStakePoolAccounts<
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
        'a,
        'a,
        'a,
    >,
    args: A,
) -> ProgramResult {
    let ix = swap_via_stake_lido_socean_stake_pool_ix(accounts, args)?;
    let account_info: [AccountInfo<'a>; 30] = accounts.into();
    invoke(&ix, &account_info)
}
pub fn swap_via_stake_lido_socean_stake_pool_invoke_signed<
    'a,
    A: Into<SwapViaStakeLidoSoceanStakePoolIxArgs>,
>(
    accounts: &SwapViaStakeLidoSoceanStakePoolAccounts<
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
        'a,
        'a,
        'a,
    >,
    args: A,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let ix = swap_via_stake_lido_socean_stake_pool_ix(accounts, args)?;
    let account_info: [AccountInfo<'a>; 30] = accounts.into();
    invoke_signed(&ix, &account_info, seeds)
}
#[derive(Copy, Clone, Debug)]
pub struct SwapViaStakeLidoSplStakePoolAccounts<
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
    'a12: 'me,
    'a13: 'me,
    'a14: 'me,
    'a15: 'me,
    'a16: 'me,
    'a17: 'me,
    'a18: 'me,
    'a19: 'me,
    'a20: 'me,
    'a21: 'me,
    'a22: 'me,
    'a23: 'me,
    'a24: 'me,
    'a25: 'me,
    'a26: 'me,
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
    pub lido_program: &'me AccountInfo<'a8>,
    pub withdraw_stake_solido: &'me AccountInfo<'a9>,
    pub withdraw_stake_voter: &'me AccountInfo<'a10>,
    pub withdraw_stake_stake_to_split: &'me AccountInfo<'a11>,
    pub withdraw_stake_stake_authority: &'me AccountInfo<'a12>,
    pub withdraw_stake_validator_list: &'me AccountInfo<'a13>,
    pub clock: &'me AccountInfo<'a14>,
    pub token_program: &'me AccountInfo<'a15>,
    pub stake_program: &'me AccountInfo<'a16>,
    pub system_program: &'me AccountInfo<'a17>,
    pub spl_stake_pool_program: &'me AccountInfo<'a18>,
    pub deposit_stake_spl_stake_pool: &'me AccountInfo<'a19>,
    pub deposit_stake_validator_list: &'me AccountInfo<'a20>,
    pub deposit_stake_deposit_authority: &'me AccountInfo<'a21>,
    pub deposit_stake_withdraw_authority: &'me AccountInfo<'a22>,
    pub deposit_stake_validator_stake: &'me AccountInfo<'a23>,
    pub deposit_stake_reserve_stake: &'me AccountInfo<'a24>,
    pub deposit_stake_manager_fee: &'me AccountInfo<'a25>,
    pub stake_history: &'me AccountInfo<'a26>,
}
#[derive(Copy, Clone, Debug)]
pub struct SwapViaStakeLidoSplStakePoolKeys {
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
    pub spl_stake_pool_program: Pubkey,
    pub deposit_stake_spl_stake_pool: Pubkey,
    pub deposit_stake_validator_list: Pubkey,
    pub deposit_stake_deposit_authority: Pubkey,
    pub deposit_stake_withdraw_authority: Pubkey,
    pub deposit_stake_validator_stake: Pubkey,
    pub deposit_stake_reserve_stake: Pubkey,
    pub deposit_stake_manager_fee: Pubkey,
    pub stake_history: Pubkey,
}
impl<'me>
    From<
        &SwapViaStakeLidoSplStakePoolAccounts<
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
            '_,
            '_,
            '_,
        >,
    > for SwapViaStakeLidoSplStakePoolKeys
{
    fn from(
        accounts: &SwapViaStakeLidoSplStakePoolAccounts<
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
            '_,
            '_,
            '_,
        >,
    ) -> Self {
        Self {
            payer: *accounts.payer.key,
            user: *accounts.user.key,
            src_token_from: *accounts.src_token_from.key,
            dest_token_to: *accounts.dest_token_to.key,
            bridge_stake: *accounts.bridge_stake.key,
            dest_token_fee_token_account: *accounts.dest_token_fee_token_account.key,
            src_token_mint: *accounts.src_token_mint.key,
            dest_token_mint: *accounts.dest_token_mint.key,
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
            spl_stake_pool_program: *accounts.spl_stake_pool_program.key,
            deposit_stake_spl_stake_pool: *accounts.deposit_stake_spl_stake_pool.key,
            deposit_stake_validator_list: *accounts.deposit_stake_validator_list.key,
            deposit_stake_deposit_authority: *accounts.deposit_stake_deposit_authority.key,
            deposit_stake_withdraw_authority: *accounts.deposit_stake_withdraw_authority.key,
            deposit_stake_validator_stake: *accounts.deposit_stake_validator_stake.key,
            deposit_stake_reserve_stake: *accounts.deposit_stake_reserve_stake.key,
            deposit_stake_manager_fee: *accounts.deposit_stake_manager_fee.key,
            stake_history: *accounts.stake_history.key,
        }
    }
}
impl From<&SwapViaStakeLidoSplStakePoolKeys> for [AccountMeta; 30] {
    fn from(keys: &SwapViaStakeLidoSplStakePoolKeys) -> Self {
        [
            AccountMeta::new(keys.payer, true),
            AccountMeta::new_readonly(keys.user, true),
            AccountMeta::new(keys.src_token_from, false),
            AccountMeta::new(keys.dest_token_to, false),
            AccountMeta::new(keys.bridge_stake, false),
            AccountMeta::new(keys.dest_token_fee_token_account, false),
            AccountMeta::new(keys.src_token_mint, false),
            AccountMeta::new(keys.dest_token_mint, false),
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
impl<'a>
    From<
        &SwapViaStakeLidoSplStakePoolAccounts<
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
            'a,
            'a,
            'a,
        >,
    > for [AccountInfo<'a>; 30]
{
    fn from(
        accounts: &SwapViaStakeLidoSplStakePoolAccounts<
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
            'a,
            'a,
            'a,
        >,
    ) -> Self {
        [
            accounts.payer.clone(),
            accounts.user.clone(),
            accounts.src_token_from.clone(),
            accounts.dest_token_to.clone(),
            accounts.bridge_stake.clone(),
            accounts.dest_token_fee_token_account.clone(),
            accounts.src_token_mint.clone(),
            accounts.dest_token_mint.clone(),
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
pub struct SwapViaStakeLidoSplStakePoolIxArgs {
    pub swap_via_stake_args: SwapViaStakeArgs,
}
#[derive(Copy, Clone, Debug)]
pub struct SwapViaStakeLidoSplStakePoolIxData<'me>(pub &'me SwapViaStakeLidoSplStakePoolIxArgs);
pub const SWAP_VIA_STAKE_LIDO_SPL_STAKE_POOL_IX_DISCM: u8 = 1u8;
impl<'me> From<&'me SwapViaStakeLidoSplStakePoolIxArgs>
    for SwapViaStakeLidoSplStakePoolIxData<'me>
{
    fn from(args: &'me SwapViaStakeLidoSplStakePoolIxArgs) -> Self {
        Self(args)
    }
}
impl BorshSerialize for SwapViaStakeLidoSplStakePoolIxData<'_> {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        writer.write_all(&[SWAP_VIA_STAKE_LIDO_SPL_STAKE_POOL_IX_DISCM])?;
        self.0.serialize(writer)
    }
}
pub fn swap_via_stake_lido_spl_stake_pool_ix<
    K: Into<SwapViaStakeLidoSplStakePoolKeys>,
    A: Into<SwapViaStakeLidoSplStakePoolIxArgs>,
>(
    accounts: K,
    args: A,
) -> std::io::Result<Instruction> {
    let keys: SwapViaStakeLidoSplStakePoolKeys = accounts.into();
    let metas: [AccountMeta; 30] = (&keys).into();
    let args_full: SwapViaStakeLidoSplStakePoolIxArgs = args.into();
    let data: SwapViaStakeLidoSplStakePoolIxData = (&args_full).into();
    Ok(Instruction {
        program_id: crate::ID,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn swap_via_stake_lido_spl_stake_pool_invoke<
    'a,
    A: Into<SwapViaStakeLidoSplStakePoolIxArgs>,
>(
    accounts: &SwapViaStakeLidoSplStakePoolAccounts<
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
        'a,
        'a,
        'a,
    >,
    args: A,
) -> ProgramResult {
    let ix = swap_via_stake_lido_spl_stake_pool_ix(accounts, args)?;
    let account_info: [AccountInfo<'a>; 30] = accounts.into();
    invoke(&ix, &account_info)
}
pub fn swap_via_stake_lido_spl_stake_pool_invoke_signed<
    'a,
    A: Into<SwapViaStakeLidoSplStakePoolIxArgs>,
>(
    accounts: &SwapViaStakeLidoSplStakePoolAccounts<
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
        'a,
        'a,
        'a,
    >,
    args: A,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let ix = swap_via_stake_lido_spl_stake_pool_ix(accounts, args)?;
    let account_info: [AccountInfo<'a>; 30] = accounts.into();
    invoke_signed(&ix, &account_info, seeds)
}
#[derive(Copy, Clone, Debug)]
pub struct SwapViaStakeLidoMarinadeAccounts<
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
    'a12: 'me,
    'a13: 'me,
    'a14: 'me,
    'a15: 'me,
    'a16: 'me,
    'a17: 'me,
    'a18: 'me,
    'a19: 'me,
    'a20: 'me,
    'a21: 'me,
    'a22: 'me,
    'a23: 'me,
    'a24: 'me,
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
    pub lido_program: &'me AccountInfo<'a8>,
    pub withdraw_stake_solido: &'me AccountInfo<'a9>,
    pub withdraw_stake_voter: &'me AccountInfo<'a10>,
    pub withdraw_stake_stake_to_split: &'me AccountInfo<'a11>,
    pub withdraw_stake_stake_authority: &'me AccountInfo<'a12>,
    pub withdraw_stake_validator_list: &'me AccountInfo<'a13>,
    pub clock: &'me AccountInfo<'a14>,
    pub token_program: &'me AccountInfo<'a15>,
    pub stake_program: &'me AccountInfo<'a16>,
    pub system_program: &'me AccountInfo<'a17>,
    pub marinade_program: &'me AccountInfo<'a18>,
    pub deposit_stake_marinade_state: &'me AccountInfo<'a19>,
    pub deposit_stake_validator_list: &'me AccountInfo<'a20>,
    pub deposit_stake_stake_list: &'me AccountInfo<'a21>,
    pub deposit_stake_duplication_flag: &'me AccountInfo<'a22>,
    pub deposit_stake_msol_mint_auth: &'me AccountInfo<'a23>,
    pub rent: &'me AccountInfo<'a24>,
}
#[derive(Copy, Clone, Debug)]
pub struct SwapViaStakeLidoMarinadeKeys {
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
    pub marinade_program: Pubkey,
    pub deposit_stake_marinade_state: Pubkey,
    pub deposit_stake_validator_list: Pubkey,
    pub deposit_stake_stake_list: Pubkey,
    pub deposit_stake_duplication_flag: Pubkey,
    pub deposit_stake_msol_mint_auth: Pubkey,
    pub rent: Pubkey,
}
impl<'me>
    From<
        &SwapViaStakeLidoMarinadeAccounts<
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
            '_,
        >,
    > for SwapViaStakeLidoMarinadeKeys
{
    fn from(
        accounts: &SwapViaStakeLidoMarinadeAccounts<
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
            '_,
        >,
    ) -> Self {
        Self {
            payer: *accounts.payer.key,
            user: *accounts.user.key,
            src_token_from: *accounts.src_token_from.key,
            dest_token_to: *accounts.dest_token_to.key,
            bridge_stake: *accounts.bridge_stake.key,
            dest_token_fee_token_account: *accounts.dest_token_fee_token_account.key,
            src_token_mint: *accounts.src_token_mint.key,
            dest_token_mint: *accounts.dest_token_mint.key,
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
            marinade_program: *accounts.marinade_program.key,
            deposit_stake_marinade_state: *accounts.deposit_stake_marinade_state.key,
            deposit_stake_validator_list: *accounts.deposit_stake_validator_list.key,
            deposit_stake_stake_list: *accounts.deposit_stake_stake_list.key,
            deposit_stake_duplication_flag: *accounts.deposit_stake_duplication_flag.key,
            deposit_stake_msol_mint_auth: *accounts.deposit_stake_msol_mint_auth.key,
            rent: *accounts.rent.key,
        }
    }
}
impl From<&SwapViaStakeLidoMarinadeKeys> for [AccountMeta; 29] {
    fn from(keys: &SwapViaStakeLidoMarinadeKeys) -> Self {
        [
            AccountMeta::new(keys.payer, true),
            AccountMeta::new_readonly(keys.user, true),
            AccountMeta::new(keys.src_token_from, false),
            AccountMeta::new(keys.dest_token_to, false),
            AccountMeta::new(keys.bridge_stake, false),
            AccountMeta::new(keys.dest_token_fee_token_account, false),
            AccountMeta::new(keys.src_token_mint, false),
            AccountMeta::new(keys.dest_token_mint, false),
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
impl<'a>
    From<
        &SwapViaStakeLidoMarinadeAccounts<
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
            'a,
        >,
    > for [AccountInfo<'a>; 29]
{
    fn from(
        accounts: &SwapViaStakeLidoMarinadeAccounts<
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
            'a,
        >,
    ) -> Self {
        [
            accounts.payer.clone(),
            accounts.user.clone(),
            accounts.src_token_from.clone(),
            accounts.dest_token_to.clone(),
            accounts.bridge_stake.clone(),
            accounts.dest_token_fee_token_account.clone(),
            accounts.src_token_mint.clone(),
            accounts.dest_token_mint.clone(),
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
pub struct SwapViaStakeLidoMarinadeIxArgs {
    pub swap_via_stake_args: SwapViaStakeArgs,
}
#[derive(Copy, Clone, Debug)]
pub struct SwapViaStakeLidoMarinadeIxData<'me>(pub &'me SwapViaStakeLidoMarinadeIxArgs);
pub const SWAP_VIA_STAKE_LIDO_MARINADE_IX_DISCM: u8 = 1u8;
impl<'me> From<&'me SwapViaStakeLidoMarinadeIxArgs> for SwapViaStakeLidoMarinadeIxData<'me> {
    fn from(args: &'me SwapViaStakeLidoMarinadeIxArgs) -> Self {
        Self(args)
    }
}
impl BorshSerialize for SwapViaStakeLidoMarinadeIxData<'_> {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        writer.write_all(&[SWAP_VIA_STAKE_LIDO_MARINADE_IX_DISCM])?;
        self.0.serialize(writer)
    }
}
pub fn swap_via_stake_lido_marinade_ix<
    K: Into<SwapViaStakeLidoMarinadeKeys>,
    A: Into<SwapViaStakeLidoMarinadeIxArgs>,
>(
    accounts: K,
    args: A,
) -> std::io::Result<Instruction> {
    let keys: SwapViaStakeLidoMarinadeKeys = accounts.into();
    let metas: [AccountMeta; 29] = (&keys).into();
    let args_full: SwapViaStakeLidoMarinadeIxArgs = args.into();
    let data: SwapViaStakeLidoMarinadeIxData = (&args_full).into();
    Ok(Instruction {
        program_id: crate::ID,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn swap_via_stake_lido_marinade_invoke<'a, A: Into<SwapViaStakeLidoMarinadeIxArgs>>(
    accounts: &SwapViaStakeLidoMarinadeAccounts<
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
        'a,
    >,
    args: A,
) -> ProgramResult {
    let ix = swap_via_stake_lido_marinade_ix(accounts, args)?;
    let account_info: [AccountInfo<'a>; 29] = accounts.into();
    invoke(&ix, &account_info)
}
pub fn swap_via_stake_lido_marinade_invoke_signed<'a, A: Into<SwapViaStakeLidoMarinadeIxArgs>>(
    accounts: &SwapViaStakeLidoMarinadeAccounts<
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
        'a,
    >,
    args: A,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let ix = swap_via_stake_lido_marinade_ix(accounts, args)?;
    let account_info: [AccountInfo<'a>; 29] = accounts.into();
    invoke_signed(&ix, &account_info, seeds)
}
#[derive(Copy, Clone, Debug)]
pub struct SwapViaStakeLidoUnstakeItAccounts<
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
    'a12: 'me,
    'a13: 'me,
    'a14: 'me,
    'a15: 'me,
    'a16: 'me,
    'a17: 'me,
    'a18: 'me,
    'a19: 'me,
    'a20: 'me,
    'a21: 'me,
    'a22: 'me,
    'a23: 'me,
    'a24: 'me,
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
    pub lido_program: &'me AccountInfo<'a8>,
    pub withdraw_stake_solido: &'me AccountInfo<'a9>,
    pub withdraw_stake_voter: &'me AccountInfo<'a10>,
    pub withdraw_stake_stake_to_split: &'me AccountInfo<'a11>,
    pub withdraw_stake_stake_authority: &'me AccountInfo<'a12>,
    pub withdraw_stake_validator_list: &'me AccountInfo<'a13>,
    pub clock: &'me AccountInfo<'a14>,
    pub token_program: &'me AccountInfo<'a15>,
    pub stake_program: &'me AccountInfo<'a16>,
    pub system_program: &'me AccountInfo<'a17>,
    pub unstakeit_program: &'me AccountInfo<'a18>,
    pub deposit_stake_unstake_pool: &'me AccountInfo<'a19>,
    pub deposit_stake_pool_sol_reserves: &'me AccountInfo<'a20>,
    pub deposit_stake_unstake_fee: &'me AccountInfo<'a21>,
    pub deposit_stake_stake_acc_record: &'me AccountInfo<'a22>,
    pub deposit_stake_protocol_fee: &'me AccountInfo<'a23>,
    pub deposit_stake_protocol_fee_dest: &'me AccountInfo<'a24>,
}
#[derive(Copy, Clone, Debug)]
pub struct SwapViaStakeLidoUnstakeItKeys {
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
    pub unstakeit_program: Pubkey,
    pub deposit_stake_unstake_pool: Pubkey,
    pub deposit_stake_pool_sol_reserves: Pubkey,
    pub deposit_stake_unstake_fee: Pubkey,
    pub deposit_stake_stake_acc_record: Pubkey,
    pub deposit_stake_protocol_fee: Pubkey,
    pub deposit_stake_protocol_fee_dest: Pubkey,
}
impl<'me>
    From<
        &SwapViaStakeLidoUnstakeItAccounts<
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
            '_,
        >,
    > for SwapViaStakeLidoUnstakeItKeys
{
    fn from(
        accounts: &SwapViaStakeLidoUnstakeItAccounts<
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
            '_,
        >,
    ) -> Self {
        Self {
            payer: *accounts.payer.key,
            user: *accounts.user.key,
            src_token_from: *accounts.src_token_from.key,
            dest_token_to: *accounts.dest_token_to.key,
            bridge_stake: *accounts.bridge_stake.key,
            dest_token_fee_token_account: *accounts.dest_token_fee_token_account.key,
            src_token_mint: *accounts.src_token_mint.key,
            dest_token_mint: *accounts.dest_token_mint.key,
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
            unstakeit_program: *accounts.unstakeit_program.key,
            deposit_stake_unstake_pool: *accounts.deposit_stake_unstake_pool.key,
            deposit_stake_pool_sol_reserves: *accounts.deposit_stake_pool_sol_reserves.key,
            deposit_stake_unstake_fee: *accounts.deposit_stake_unstake_fee.key,
            deposit_stake_stake_acc_record: *accounts.deposit_stake_stake_acc_record.key,
            deposit_stake_protocol_fee: *accounts.deposit_stake_protocol_fee.key,
            deposit_stake_protocol_fee_dest: *accounts.deposit_stake_protocol_fee_dest.key,
        }
    }
}
impl From<&SwapViaStakeLidoUnstakeItKeys> for [AccountMeta; 29] {
    fn from(keys: &SwapViaStakeLidoUnstakeItKeys) -> Self {
        [
            AccountMeta::new(keys.payer, true),
            AccountMeta::new_readonly(keys.user, true),
            AccountMeta::new(keys.src_token_from, false),
            AccountMeta::new(keys.dest_token_to, false),
            AccountMeta::new(keys.bridge_stake, false),
            AccountMeta::new(keys.dest_token_fee_token_account, false),
            AccountMeta::new(keys.src_token_mint, false),
            AccountMeta::new(keys.dest_token_mint, false),
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
impl<'a>
    From<
        &SwapViaStakeLidoUnstakeItAccounts<
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
            'a,
        >,
    > for [AccountInfo<'a>; 29]
{
    fn from(
        accounts: &SwapViaStakeLidoUnstakeItAccounts<
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
            'a,
        >,
    ) -> Self {
        [
            accounts.payer.clone(),
            accounts.user.clone(),
            accounts.src_token_from.clone(),
            accounts.dest_token_to.clone(),
            accounts.bridge_stake.clone(),
            accounts.dest_token_fee_token_account.clone(),
            accounts.src_token_mint.clone(),
            accounts.dest_token_mint.clone(),
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
pub struct SwapViaStakeLidoUnstakeItIxArgs {
    pub swap_via_stake_args: SwapViaStakeArgs,
}
#[derive(Copy, Clone, Debug)]
pub struct SwapViaStakeLidoUnstakeItIxData<'me>(pub &'me SwapViaStakeLidoUnstakeItIxArgs);
pub const SWAP_VIA_STAKE_LIDO_UNSTAKE_IT_IX_DISCM: u8 = 1u8;
impl<'me> From<&'me SwapViaStakeLidoUnstakeItIxArgs> for SwapViaStakeLidoUnstakeItIxData<'me> {
    fn from(args: &'me SwapViaStakeLidoUnstakeItIxArgs) -> Self {
        Self(args)
    }
}
impl BorshSerialize for SwapViaStakeLidoUnstakeItIxData<'_> {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        writer.write_all(&[SWAP_VIA_STAKE_LIDO_UNSTAKE_IT_IX_DISCM])?;
        self.0.serialize(writer)
    }
}
pub fn swap_via_stake_lido_unstake_it_ix<
    K: Into<SwapViaStakeLidoUnstakeItKeys>,
    A: Into<SwapViaStakeLidoUnstakeItIxArgs>,
>(
    accounts: K,
    args: A,
) -> std::io::Result<Instruction> {
    let keys: SwapViaStakeLidoUnstakeItKeys = accounts.into();
    let metas: [AccountMeta; 29] = (&keys).into();
    let args_full: SwapViaStakeLidoUnstakeItIxArgs = args.into();
    let data: SwapViaStakeLidoUnstakeItIxData = (&args_full).into();
    Ok(Instruction {
        program_id: crate::ID,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn swap_via_stake_lido_unstake_it_invoke<'a, A: Into<SwapViaStakeLidoUnstakeItIxArgs>>(
    accounts: &SwapViaStakeLidoUnstakeItAccounts<
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
        'a,
    >,
    args: A,
) -> ProgramResult {
    let ix = swap_via_stake_lido_unstake_it_ix(accounts, args)?;
    let account_info: [AccountInfo<'a>; 29] = accounts.into();
    invoke(&ix, &account_info)
}
pub fn swap_via_stake_lido_unstake_it_invoke_signed<
    'a,
    A: Into<SwapViaStakeLidoUnstakeItIxArgs>,
>(
    accounts: &SwapViaStakeLidoUnstakeItAccounts<
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
        'a,
    >,
    args: A,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let ix = swap_via_stake_lido_unstake_it_ix(accounts, args)?;
    let account_info: [AccountInfo<'a>; 29] = accounts.into();
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
