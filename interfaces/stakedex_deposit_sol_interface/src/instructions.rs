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
pub enum StakedexDepositSolProgramIx {
    MarinadeDepositSol,
    SoceanStakePoolDepositSol,
    SplStakePoolDepositSol,
}
impl StakedexDepositSolProgramIx {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm_buf = [0u8; 1];
        reader.read_exact(&mut maybe_discm_buf)?;
        let maybe_discm = maybe_discm_buf[0];
        match maybe_discm {
            MARINADE_DEPOSIT_SOL_IX_DISCM => Ok(Self::MarinadeDepositSol),
            SOCEAN_STAKE_POOL_DEPOSIT_SOL_IX_DISCM => Ok(Self::SoceanStakePoolDepositSol),
            SPL_STAKE_POOL_DEPOSIT_SOL_IX_DISCM => Ok(Self::SplStakePoolDepositSol),
            _ => Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!("discm {:?} not found", maybe_discm),
            )),
        }
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        match self {
            Self::MarinadeDepositSol => writer.write_all(&[MARINADE_DEPOSIT_SOL_IX_DISCM]),
            Self::SoceanStakePoolDepositSol => {
                writer.write_all(&[SOCEAN_STAKE_POOL_DEPOSIT_SOL_IX_DISCM])
            }
            Self::SplStakePoolDepositSol => {
                writer.write_all(&[SPL_STAKE_POOL_DEPOSIT_SOL_IX_DISCM])
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
pub const MARINADE_DEPOSIT_SOL_IX_ACCOUNTS_LEN: usize = 7;
#[derive(Copy, Clone, Debug)]
pub struct MarinadeDepositSolAccounts<'me, 'info> {
    pub marinade_program: &'me AccountInfo<'info>,
    pub marinade_state: &'me AccountInfo<'info>,
    pub marinade_liq_pool_sol_leg: &'me AccountInfo<'info>,
    pub marinade_liq_pool_msol_leg: &'me AccountInfo<'info>,
    pub marinade_liq_pool_msol_leg_auth: &'me AccountInfo<'info>,
    pub marinade_reserve: &'me AccountInfo<'info>,
    pub msol_mint_authority: &'me AccountInfo<'info>,
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
impl From<MarinadeDepositSolAccounts<'_, '_>> for MarinadeDepositSolKeys {
    fn from(accounts: MarinadeDepositSolAccounts) -> Self {
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
impl From<MarinadeDepositSolKeys> for [AccountMeta; MARINADE_DEPOSIT_SOL_IX_ACCOUNTS_LEN] {
    fn from(keys: MarinadeDepositSolKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.marinade_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.marinade_state,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.marinade_liq_pool_sol_leg,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.marinade_liq_pool_msol_leg,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.marinade_liq_pool_msol_leg_auth,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.marinade_reserve,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.msol_mint_authority,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; MARINADE_DEPOSIT_SOL_IX_ACCOUNTS_LEN]> for MarinadeDepositSolKeys {
    fn from(pubkeys: [Pubkey; MARINADE_DEPOSIT_SOL_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            marinade_program: pubkeys[0],
            marinade_state: pubkeys[1],
            marinade_liq_pool_sol_leg: pubkeys[2],
            marinade_liq_pool_msol_leg: pubkeys[3],
            marinade_liq_pool_msol_leg_auth: pubkeys[4],
            marinade_reserve: pubkeys[5],
            msol_mint_authority: pubkeys[6],
        }
    }
}
impl<'info> From<MarinadeDepositSolAccounts<'_, 'info>>
    for [AccountInfo<'info>; MARINADE_DEPOSIT_SOL_IX_ACCOUNTS_LEN]
{
    fn from(accounts: MarinadeDepositSolAccounts<'_, 'info>) -> Self {
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
impl<'me, 'info> From<&'me [AccountInfo<'info>; MARINADE_DEPOSIT_SOL_IX_ACCOUNTS_LEN]>
    for MarinadeDepositSolAccounts<'me, 'info>
{
    fn from(arr: &'me [AccountInfo<'info>; MARINADE_DEPOSIT_SOL_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            marinade_program: &arr[0],
            marinade_state: &arr[1],
            marinade_liq_pool_sol_leg: &arr[2],
            marinade_liq_pool_msol_leg: &arr[3],
            marinade_liq_pool_msol_leg_auth: &arr[4],
            marinade_reserve: &arr[5],
            msol_mint_authority: &arr[6],
        }
    }
}
pub const MARINADE_DEPOSIT_SOL_IX_DISCM: u8 = 0u8;
#[derive(Clone, Debug, PartialEq)]
pub struct MarinadeDepositSolIxData;
impl MarinadeDepositSolIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm_buf = [0u8; 1];
        reader.read_exact(&mut maybe_discm_buf)?;
        let maybe_discm = maybe_discm_buf[0];
        if maybe_discm != MARINADE_DEPOSIT_SOL_IX_DISCM {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    MARINADE_DEPOSIT_SOL_IX_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&[MARINADE_DEPOSIT_SOL_IX_DISCM])
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn marinade_deposit_sol_ix_with_program_id(
    program_id: Pubkey,
    keys: MarinadeDepositSolKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; MARINADE_DEPOSIT_SOL_IX_ACCOUNTS_LEN] = keys.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: MarinadeDepositSolIxData.try_to_vec()?,
    })
}
pub fn marinade_deposit_sol_ix(keys: MarinadeDepositSolKeys) -> std::io::Result<Instruction> {
    marinade_deposit_sol_ix_with_program_id(crate::ID, keys)
}
pub fn marinade_deposit_sol_invoke_with_program_id(
    program_id: Pubkey,
    accounts: MarinadeDepositSolAccounts<'_, '_>,
) -> ProgramResult {
    let keys: MarinadeDepositSolKeys = accounts.into();
    let ix = marinade_deposit_sol_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn marinade_deposit_sol_invoke(accounts: MarinadeDepositSolAccounts<'_, '_>) -> ProgramResult {
    marinade_deposit_sol_invoke_with_program_id(crate::ID, accounts)
}
pub fn marinade_deposit_sol_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: MarinadeDepositSolAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: MarinadeDepositSolKeys = accounts.into();
    let ix = marinade_deposit_sol_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn marinade_deposit_sol_invoke_signed(
    accounts: MarinadeDepositSolAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    marinade_deposit_sol_invoke_signed_with_program_id(crate::ID, accounts, seeds)
}
pub fn marinade_deposit_sol_verify_account_keys(
    accounts: MarinadeDepositSolAccounts<'_, '_>,
    keys: MarinadeDepositSolKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (accounts.marinade_program.key, &keys.marinade_program),
        (accounts.marinade_state.key, &keys.marinade_state),
        (
            accounts.marinade_liq_pool_sol_leg.key,
            &keys.marinade_liq_pool_sol_leg,
        ),
        (
            accounts.marinade_liq_pool_msol_leg.key,
            &keys.marinade_liq_pool_msol_leg,
        ),
        (
            accounts.marinade_liq_pool_msol_leg_auth.key,
            &keys.marinade_liq_pool_msol_leg_auth,
        ),
        (accounts.marinade_reserve.key, &keys.marinade_reserve),
        (accounts.msol_mint_authority.key, &keys.msol_mint_authority),
    ] {
        if actual != expected {
            return Err((*actual, *expected));
        }
    }
    Ok(())
}
pub fn marinade_deposit_sol_verify_writable_privileges<'me, 'info>(
    accounts: MarinadeDepositSolAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.marinade_state,
        accounts.marinade_liq_pool_sol_leg,
        accounts.marinade_liq_pool_msol_leg,
        accounts.marinade_reserve,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn marinade_deposit_sol_verify_account_privileges<'me, 'info>(
    accounts: MarinadeDepositSolAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    marinade_deposit_sol_verify_writable_privileges(accounts)?;
    Ok(())
}
pub const SOCEAN_STAKE_POOL_DEPOSIT_SOL_IX_ACCOUNTS_LEN: usize = 6;
#[derive(Copy, Clone, Debug)]
pub struct SoceanStakePoolDepositSolAccounts<'me, 'info> {
    pub socean_stake_pool_program: &'me AccountInfo<'info>,
    pub stake_pool: &'me AccountInfo<'info>,
    pub stake_pool_withdraw_authority: &'me AccountInfo<'info>,
    pub stake_pool_reserve_stake: &'me AccountInfo<'info>,
    pub stake_pool_manager_fee: &'me AccountInfo<'info>,
    pub clock: &'me AccountInfo<'info>,
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
impl From<SoceanStakePoolDepositSolAccounts<'_, '_>> for SoceanStakePoolDepositSolKeys {
    fn from(accounts: SoceanStakePoolDepositSolAccounts) -> Self {
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
impl From<SoceanStakePoolDepositSolKeys>
    for [AccountMeta; SOCEAN_STAKE_POOL_DEPOSIT_SOL_IX_ACCOUNTS_LEN]
{
    fn from(keys: SoceanStakePoolDepositSolKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.socean_stake_pool_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.stake_pool,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.stake_pool_withdraw_authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.stake_pool_reserve_stake,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.stake_pool_manager_fee,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.clock,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; SOCEAN_STAKE_POOL_DEPOSIT_SOL_IX_ACCOUNTS_LEN]>
    for SoceanStakePoolDepositSolKeys
{
    fn from(pubkeys: [Pubkey; SOCEAN_STAKE_POOL_DEPOSIT_SOL_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            socean_stake_pool_program: pubkeys[0],
            stake_pool: pubkeys[1],
            stake_pool_withdraw_authority: pubkeys[2],
            stake_pool_reserve_stake: pubkeys[3],
            stake_pool_manager_fee: pubkeys[4],
            clock: pubkeys[5],
        }
    }
}
impl<'info> From<SoceanStakePoolDepositSolAccounts<'_, 'info>>
    for [AccountInfo<'info>; SOCEAN_STAKE_POOL_DEPOSIT_SOL_IX_ACCOUNTS_LEN]
{
    fn from(accounts: SoceanStakePoolDepositSolAccounts<'_, 'info>) -> Self {
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
impl<'me, 'info> From<&'me [AccountInfo<'info>; SOCEAN_STAKE_POOL_DEPOSIT_SOL_IX_ACCOUNTS_LEN]>
    for SoceanStakePoolDepositSolAccounts<'me, 'info>
{
    fn from(arr: &'me [AccountInfo<'info>; SOCEAN_STAKE_POOL_DEPOSIT_SOL_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            socean_stake_pool_program: &arr[0],
            stake_pool: &arr[1],
            stake_pool_withdraw_authority: &arr[2],
            stake_pool_reserve_stake: &arr[3],
            stake_pool_manager_fee: &arr[4],
            clock: &arr[5],
        }
    }
}
pub const SOCEAN_STAKE_POOL_DEPOSIT_SOL_IX_DISCM: u8 = 1u8;
#[derive(Clone, Debug, PartialEq)]
pub struct SoceanStakePoolDepositSolIxData;
impl SoceanStakePoolDepositSolIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm_buf = [0u8; 1];
        reader.read_exact(&mut maybe_discm_buf)?;
        let maybe_discm = maybe_discm_buf[0];
        if maybe_discm != SOCEAN_STAKE_POOL_DEPOSIT_SOL_IX_DISCM {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    SOCEAN_STAKE_POOL_DEPOSIT_SOL_IX_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&[SOCEAN_STAKE_POOL_DEPOSIT_SOL_IX_DISCM])
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn socean_stake_pool_deposit_sol_ix_with_program_id(
    program_id: Pubkey,
    keys: SoceanStakePoolDepositSolKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; SOCEAN_STAKE_POOL_DEPOSIT_SOL_IX_ACCOUNTS_LEN] = keys.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: SoceanStakePoolDepositSolIxData.try_to_vec()?,
    })
}
pub fn socean_stake_pool_deposit_sol_ix(
    keys: SoceanStakePoolDepositSolKeys,
) -> std::io::Result<Instruction> {
    socean_stake_pool_deposit_sol_ix_with_program_id(crate::ID, keys)
}
pub fn socean_stake_pool_deposit_sol_invoke_with_program_id(
    program_id: Pubkey,
    accounts: SoceanStakePoolDepositSolAccounts<'_, '_>,
) -> ProgramResult {
    let keys: SoceanStakePoolDepositSolKeys = accounts.into();
    let ix = socean_stake_pool_deposit_sol_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn socean_stake_pool_deposit_sol_invoke(
    accounts: SoceanStakePoolDepositSolAccounts<'_, '_>,
) -> ProgramResult {
    socean_stake_pool_deposit_sol_invoke_with_program_id(crate::ID, accounts)
}
pub fn socean_stake_pool_deposit_sol_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: SoceanStakePoolDepositSolAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: SoceanStakePoolDepositSolKeys = accounts.into();
    let ix = socean_stake_pool_deposit_sol_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn socean_stake_pool_deposit_sol_invoke_signed(
    accounts: SoceanStakePoolDepositSolAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    socean_stake_pool_deposit_sol_invoke_signed_with_program_id(crate::ID, accounts, seeds)
}
pub fn socean_stake_pool_deposit_sol_verify_account_keys(
    accounts: SoceanStakePoolDepositSolAccounts<'_, '_>,
    keys: SoceanStakePoolDepositSolKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (
            accounts.socean_stake_pool_program.key,
            &keys.socean_stake_pool_program,
        ),
        (accounts.stake_pool.key, &keys.stake_pool),
        (
            accounts.stake_pool_withdraw_authority.key,
            &keys.stake_pool_withdraw_authority,
        ),
        (
            accounts.stake_pool_reserve_stake.key,
            &keys.stake_pool_reserve_stake,
        ),
        (
            accounts.stake_pool_manager_fee.key,
            &keys.stake_pool_manager_fee,
        ),
        (accounts.clock.key, &keys.clock),
    ] {
        if actual != expected {
            return Err((*actual, *expected));
        }
    }
    Ok(())
}
pub fn socean_stake_pool_deposit_sol_verify_writable_privileges<'me, 'info>(
    accounts: SoceanStakePoolDepositSolAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.stake_pool,
        accounts.stake_pool_reserve_stake,
        accounts.stake_pool_manager_fee,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn socean_stake_pool_deposit_sol_verify_account_privileges<'me, 'info>(
    accounts: SoceanStakePoolDepositSolAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    socean_stake_pool_deposit_sol_verify_writable_privileges(accounts)?;
    Ok(())
}
pub const SPL_STAKE_POOL_DEPOSIT_SOL_IX_ACCOUNTS_LEN: usize = 5;
#[derive(Copy, Clone, Debug)]
pub struct SplStakePoolDepositSolAccounts<'me, 'info> {
    pub spl_stake_pool_program: &'me AccountInfo<'info>,
    pub stake_pool: &'me AccountInfo<'info>,
    pub stake_pool_withdraw_authority: &'me AccountInfo<'info>,
    pub stake_pool_reserve_stake: &'me AccountInfo<'info>,
    pub stake_pool_manager_fee: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug)]
pub struct SplStakePoolDepositSolKeys {
    pub spl_stake_pool_program: Pubkey,
    pub stake_pool: Pubkey,
    pub stake_pool_withdraw_authority: Pubkey,
    pub stake_pool_reserve_stake: Pubkey,
    pub stake_pool_manager_fee: Pubkey,
}
impl From<SplStakePoolDepositSolAccounts<'_, '_>> for SplStakePoolDepositSolKeys {
    fn from(accounts: SplStakePoolDepositSolAccounts) -> Self {
        Self {
            spl_stake_pool_program: *accounts.spl_stake_pool_program.key,
            stake_pool: *accounts.stake_pool.key,
            stake_pool_withdraw_authority: *accounts.stake_pool_withdraw_authority.key,
            stake_pool_reserve_stake: *accounts.stake_pool_reserve_stake.key,
            stake_pool_manager_fee: *accounts.stake_pool_manager_fee.key,
        }
    }
}
impl From<SplStakePoolDepositSolKeys>
    for [AccountMeta; SPL_STAKE_POOL_DEPOSIT_SOL_IX_ACCOUNTS_LEN]
{
    fn from(keys: SplStakePoolDepositSolKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.spl_stake_pool_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.stake_pool,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.stake_pool_withdraw_authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.stake_pool_reserve_stake,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.stake_pool_manager_fee,
                is_signer: false,
                is_writable: true,
            },
        ]
    }
}
impl From<[Pubkey; SPL_STAKE_POOL_DEPOSIT_SOL_IX_ACCOUNTS_LEN]> for SplStakePoolDepositSolKeys {
    fn from(pubkeys: [Pubkey; SPL_STAKE_POOL_DEPOSIT_SOL_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            spl_stake_pool_program: pubkeys[0],
            stake_pool: pubkeys[1],
            stake_pool_withdraw_authority: pubkeys[2],
            stake_pool_reserve_stake: pubkeys[3],
            stake_pool_manager_fee: pubkeys[4],
        }
    }
}
impl<'info> From<SplStakePoolDepositSolAccounts<'_, 'info>>
    for [AccountInfo<'info>; SPL_STAKE_POOL_DEPOSIT_SOL_IX_ACCOUNTS_LEN]
{
    fn from(accounts: SplStakePoolDepositSolAccounts<'_, 'info>) -> Self {
        [
            accounts.spl_stake_pool_program.clone(),
            accounts.stake_pool.clone(),
            accounts.stake_pool_withdraw_authority.clone(),
            accounts.stake_pool_reserve_stake.clone(),
            accounts.stake_pool_manager_fee.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; SPL_STAKE_POOL_DEPOSIT_SOL_IX_ACCOUNTS_LEN]>
    for SplStakePoolDepositSolAccounts<'me, 'info>
{
    fn from(arr: &'me [AccountInfo<'info>; SPL_STAKE_POOL_DEPOSIT_SOL_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            spl_stake_pool_program: &arr[0],
            stake_pool: &arr[1],
            stake_pool_withdraw_authority: &arr[2],
            stake_pool_reserve_stake: &arr[3],
            stake_pool_manager_fee: &arr[4],
        }
    }
}
pub const SPL_STAKE_POOL_DEPOSIT_SOL_IX_DISCM: u8 = 2u8;
#[derive(Clone, Debug, PartialEq)]
pub struct SplStakePoolDepositSolIxData;
impl SplStakePoolDepositSolIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm_buf = [0u8; 1];
        reader.read_exact(&mut maybe_discm_buf)?;
        let maybe_discm = maybe_discm_buf[0];
        if maybe_discm != SPL_STAKE_POOL_DEPOSIT_SOL_IX_DISCM {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    SPL_STAKE_POOL_DEPOSIT_SOL_IX_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&[SPL_STAKE_POOL_DEPOSIT_SOL_IX_DISCM])
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn spl_stake_pool_deposit_sol_ix_with_program_id(
    program_id: Pubkey,
    keys: SplStakePoolDepositSolKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; SPL_STAKE_POOL_DEPOSIT_SOL_IX_ACCOUNTS_LEN] = keys.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: SplStakePoolDepositSolIxData.try_to_vec()?,
    })
}
pub fn spl_stake_pool_deposit_sol_ix(
    keys: SplStakePoolDepositSolKeys,
) -> std::io::Result<Instruction> {
    spl_stake_pool_deposit_sol_ix_with_program_id(crate::ID, keys)
}
pub fn spl_stake_pool_deposit_sol_invoke_with_program_id(
    program_id: Pubkey,
    accounts: SplStakePoolDepositSolAccounts<'_, '_>,
) -> ProgramResult {
    let keys: SplStakePoolDepositSolKeys = accounts.into();
    let ix = spl_stake_pool_deposit_sol_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn spl_stake_pool_deposit_sol_invoke(
    accounts: SplStakePoolDepositSolAccounts<'_, '_>,
) -> ProgramResult {
    spl_stake_pool_deposit_sol_invoke_with_program_id(crate::ID, accounts)
}
pub fn spl_stake_pool_deposit_sol_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: SplStakePoolDepositSolAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: SplStakePoolDepositSolKeys = accounts.into();
    let ix = spl_stake_pool_deposit_sol_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn spl_stake_pool_deposit_sol_invoke_signed(
    accounts: SplStakePoolDepositSolAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    spl_stake_pool_deposit_sol_invoke_signed_with_program_id(crate::ID, accounts, seeds)
}
pub fn spl_stake_pool_deposit_sol_verify_account_keys(
    accounts: SplStakePoolDepositSolAccounts<'_, '_>,
    keys: SplStakePoolDepositSolKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (
            accounts.spl_stake_pool_program.key,
            &keys.spl_stake_pool_program,
        ),
        (accounts.stake_pool.key, &keys.stake_pool),
        (
            accounts.stake_pool_withdraw_authority.key,
            &keys.stake_pool_withdraw_authority,
        ),
        (
            accounts.stake_pool_reserve_stake.key,
            &keys.stake_pool_reserve_stake,
        ),
        (
            accounts.stake_pool_manager_fee.key,
            &keys.stake_pool_manager_fee,
        ),
    ] {
        if actual != expected {
            return Err((*actual, *expected));
        }
    }
    Ok(())
}
pub fn spl_stake_pool_deposit_sol_verify_writable_privileges<'me, 'info>(
    accounts: SplStakePoolDepositSolAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.stake_pool,
        accounts.stake_pool_reserve_stake,
        accounts.stake_pool_manager_fee,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn spl_stake_pool_deposit_sol_verify_account_privileges<'me, 'info>(
    accounts: SplStakePoolDepositSolAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    spl_stake_pool_deposit_sol_verify_writable_privileges(accounts)?;
    Ok(())
}
