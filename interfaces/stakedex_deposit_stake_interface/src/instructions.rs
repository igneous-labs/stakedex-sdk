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
pub enum StakedexDepositStakeProgramIx {
    SplStakePoolDepositStake,
    MarinadeDepositStake,
    UnstakeItDepositStake,
    SanctumSplStakePoolDepositStake,
    SanctumSplMultiStakePoolDepositStake,
}
impl StakedexDepositStakeProgramIx {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm_buf = [0u8; 1];
        reader.read_exact(&mut maybe_discm_buf)?;
        let maybe_discm = maybe_discm_buf[0];
        match maybe_discm {
            SPL_STAKE_POOL_DEPOSIT_STAKE_IX_DISCM => Ok(Self::SplStakePoolDepositStake),
            MARINADE_DEPOSIT_STAKE_IX_DISCM => Ok(Self::MarinadeDepositStake),
            UNSTAKE_IT_DEPOSIT_STAKE_IX_DISCM => Ok(Self::UnstakeItDepositStake),
            SANCTUM_SPL_STAKE_POOL_DEPOSIT_STAKE_IX_DISCM => {
                Ok(Self::SanctumSplStakePoolDepositStake)
            }
            SANCTUM_SPL_MULTI_STAKE_POOL_DEPOSIT_STAKE_IX_DISCM => {
                Ok(Self::SanctumSplMultiStakePoolDepositStake)
            }
            _ => Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!("discm {:?} not found", maybe_discm),
            )),
        }
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        match self {
            Self::SplStakePoolDepositStake => {
                writer.write_all(&[SPL_STAKE_POOL_DEPOSIT_STAKE_IX_DISCM])
            }
            Self::MarinadeDepositStake => writer.write_all(&[MARINADE_DEPOSIT_STAKE_IX_DISCM]),
            Self::UnstakeItDepositStake => writer.write_all(&[UNSTAKE_IT_DEPOSIT_STAKE_IX_DISCM]),
            Self::SanctumSplStakePoolDepositStake => {
                writer.write_all(&[SANCTUM_SPL_STAKE_POOL_DEPOSIT_STAKE_IX_DISCM])
            }
            Self::SanctumSplMultiStakePoolDepositStake => {
                writer.write_all(&[SANCTUM_SPL_MULTI_STAKE_POOL_DEPOSIT_STAKE_IX_DISCM])
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
pub const SPL_STAKE_POOL_DEPOSIT_STAKE_IX_ACCOUNTS_LEN: usize = 12;
#[derive(Copy, Clone, Debug)]
pub struct SplStakePoolDepositStakeAccounts<'me, 'info> {
    pub spl_stake_pool_program: &'me AccountInfo<'info>,
    pub deposit_stake_spl_stake_pool: &'me AccountInfo<'info>,
    pub deposit_stake_validator_list: &'me AccountInfo<'info>,
    pub deposit_stake_deposit_authority: &'me AccountInfo<'info>,
    pub deposit_stake_withdraw_authority: &'me AccountInfo<'info>,
    pub deposit_stake_validator_stake: &'me AccountInfo<'info>,
    pub deposit_stake_reserve_stake: &'me AccountInfo<'info>,
    pub deposit_stake_manager_fee: &'me AccountInfo<'info>,
    pub clock: &'me AccountInfo<'info>,
    pub stake_history: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
    pub stake_program: &'me AccountInfo<'info>,
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
impl From<SplStakePoolDepositStakeAccounts<'_, '_>> for SplStakePoolDepositStakeKeys {
    fn from(accounts: SplStakePoolDepositStakeAccounts) -> Self {
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
impl From<SplStakePoolDepositStakeKeys>
    for [AccountMeta; SPL_STAKE_POOL_DEPOSIT_STAKE_IX_ACCOUNTS_LEN]
{
    fn from(keys: SplStakePoolDepositStakeKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.spl_stake_pool_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.deposit_stake_spl_stake_pool,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.deposit_stake_validator_list,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.deposit_stake_deposit_authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.deposit_stake_withdraw_authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.deposit_stake_validator_stake,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.deposit_stake_reserve_stake,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.deposit_stake_manager_fee,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.clock,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.stake_history,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.stake_program,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; SPL_STAKE_POOL_DEPOSIT_STAKE_IX_ACCOUNTS_LEN]> for SplStakePoolDepositStakeKeys {
    fn from(pubkeys: [Pubkey; SPL_STAKE_POOL_DEPOSIT_STAKE_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            spl_stake_pool_program: pubkeys[0],
            deposit_stake_spl_stake_pool: pubkeys[1],
            deposit_stake_validator_list: pubkeys[2],
            deposit_stake_deposit_authority: pubkeys[3],
            deposit_stake_withdraw_authority: pubkeys[4],
            deposit_stake_validator_stake: pubkeys[5],
            deposit_stake_reserve_stake: pubkeys[6],
            deposit_stake_manager_fee: pubkeys[7],
            clock: pubkeys[8],
            stake_history: pubkeys[9],
            token_program: pubkeys[10],
            stake_program: pubkeys[11],
        }
    }
}
impl<'info> From<SplStakePoolDepositStakeAccounts<'_, 'info>>
    for [AccountInfo<'info>; SPL_STAKE_POOL_DEPOSIT_STAKE_IX_ACCOUNTS_LEN]
{
    fn from(accounts: SplStakePoolDepositStakeAccounts<'_, 'info>) -> Self {
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
impl<'me, 'info> From<&'me [AccountInfo<'info>; SPL_STAKE_POOL_DEPOSIT_STAKE_IX_ACCOUNTS_LEN]>
    for SplStakePoolDepositStakeAccounts<'me, 'info>
{
    fn from(arr: &'me [AccountInfo<'info>; SPL_STAKE_POOL_DEPOSIT_STAKE_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            spl_stake_pool_program: &arr[0],
            deposit_stake_spl_stake_pool: &arr[1],
            deposit_stake_validator_list: &arr[2],
            deposit_stake_deposit_authority: &arr[3],
            deposit_stake_withdraw_authority: &arr[4],
            deposit_stake_validator_stake: &arr[5],
            deposit_stake_reserve_stake: &arr[6],
            deposit_stake_manager_fee: &arr[7],
            clock: &arr[8],
            stake_history: &arr[9],
            token_program: &arr[10],
            stake_program: &arr[11],
        }
    }
}
pub const SPL_STAKE_POOL_DEPOSIT_STAKE_IX_DISCM: u8 = 1u8;
#[derive(Clone, Debug, PartialEq)]
pub struct SplStakePoolDepositStakeIxData;
impl SplStakePoolDepositStakeIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm_buf = [0u8; 1];
        reader.read_exact(&mut maybe_discm_buf)?;
        let maybe_discm = maybe_discm_buf[0];
        if maybe_discm != SPL_STAKE_POOL_DEPOSIT_STAKE_IX_DISCM {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    SPL_STAKE_POOL_DEPOSIT_STAKE_IX_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&[SPL_STAKE_POOL_DEPOSIT_STAKE_IX_DISCM])
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn spl_stake_pool_deposit_stake_ix_with_program_id(
    program_id: Pubkey,
    keys: SplStakePoolDepositStakeKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; SPL_STAKE_POOL_DEPOSIT_STAKE_IX_ACCOUNTS_LEN] = keys.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: SplStakePoolDepositStakeIxData.try_to_vec()?,
    })
}
pub fn spl_stake_pool_deposit_stake_ix(
    keys: SplStakePoolDepositStakeKeys,
) -> std::io::Result<Instruction> {
    spl_stake_pool_deposit_stake_ix_with_program_id(crate::ID, keys)
}
pub fn spl_stake_pool_deposit_stake_invoke_with_program_id(
    program_id: Pubkey,
    accounts: SplStakePoolDepositStakeAccounts<'_, '_>,
) -> ProgramResult {
    let keys: SplStakePoolDepositStakeKeys = accounts.into();
    let ix = spl_stake_pool_deposit_stake_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn spl_stake_pool_deposit_stake_invoke(
    accounts: SplStakePoolDepositStakeAccounts<'_, '_>,
) -> ProgramResult {
    spl_stake_pool_deposit_stake_invoke_with_program_id(crate::ID, accounts)
}
pub fn spl_stake_pool_deposit_stake_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: SplStakePoolDepositStakeAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: SplStakePoolDepositStakeKeys = accounts.into();
    let ix = spl_stake_pool_deposit_stake_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn spl_stake_pool_deposit_stake_invoke_signed(
    accounts: SplStakePoolDepositStakeAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    spl_stake_pool_deposit_stake_invoke_signed_with_program_id(crate::ID, accounts, seeds)
}
pub fn spl_stake_pool_deposit_stake_verify_account_keys(
    accounts: SplStakePoolDepositStakeAccounts<'_, '_>,
    keys: SplStakePoolDepositStakeKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (
            accounts.spl_stake_pool_program.key,
            &keys.spl_stake_pool_program,
        ),
        (
            accounts.deposit_stake_spl_stake_pool.key,
            &keys.deposit_stake_spl_stake_pool,
        ),
        (
            accounts.deposit_stake_validator_list.key,
            &keys.deposit_stake_validator_list,
        ),
        (
            accounts.deposit_stake_deposit_authority.key,
            &keys.deposit_stake_deposit_authority,
        ),
        (
            accounts.deposit_stake_withdraw_authority.key,
            &keys.deposit_stake_withdraw_authority,
        ),
        (
            accounts.deposit_stake_validator_stake.key,
            &keys.deposit_stake_validator_stake,
        ),
        (
            accounts.deposit_stake_reserve_stake.key,
            &keys.deposit_stake_reserve_stake,
        ),
        (
            accounts.deposit_stake_manager_fee.key,
            &keys.deposit_stake_manager_fee,
        ),
        (accounts.clock.key, &keys.clock),
        (accounts.stake_history.key, &keys.stake_history),
        (accounts.token_program.key, &keys.token_program),
        (accounts.stake_program.key, &keys.stake_program),
    ] {
        if actual != expected {
            return Err((*actual, *expected));
        }
    }
    Ok(())
}
pub fn spl_stake_pool_deposit_stake_verify_writable_privileges<'me, 'info>(
    accounts: SplStakePoolDepositStakeAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.deposit_stake_spl_stake_pool,
        accounts.deposit_stake_validator_list,
        accounts.deposit_stake_validator_stake,
        accounts.deposit_stake_reserve_stake,
        accounts.deposit_stake_manager_fee,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn spl_stake_pool_deposit_stake_verify_account_privileges<'me, 'info>(
    accounts: SplStakePoolDepositStakeAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    spl_stake_pool_deposit_stake_verify_writable_privileges(accounts)?;
    Ok(())
}
pub const MARINADE_DEPOSIT_STAKE_IX_ACCOUNTS_LEN: usize = 11;
#[derive(Copy, Clone, Debug)]
pub struct MarinadeDepositStakeAccounts<'me, 'info> {
    pub marinade_program: &'me AccountInfo<'info>,
    pub deposit_stake_marinade_state: &'me AccountInfo<'info>,
    pub deposit_stake_validator_list: &'me AccountInfo<'info>,
    pub deposit_stake_stake_list: &'me AccountInfo<'info>,
    pub deposit_stake_duplication_flag: &'me AccountInfo<'info>,
    pub deposit_stake_msol_mint_auth: &'me AccountInfo<'info>,
    pub clock: &'me AccountInfo<'info>,
    pub rent: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
    pub stake_program: &'me AccountInfo<'info>,
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
impl From<MarinadeDepositStakeAccounts<'_, '_>> for MarinadeDepositStakeKeys {
    fn from(accounts: MarinadeDepositStakeAccounts) -> Self {
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
impl From<MarinadeDepositStakeKeys> for [AccountMeta; MARINADE_DEPOSIT_STAKE_IX_ACCOUNTS_LEN] {
    fn from(keys: MarinadeDepositStakeKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.marinade_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.deposit_stake_marinade_state,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.deposit_stake_validator_list,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.deposit_stake_stake_list,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.deposit_stake_duplication_flag,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.deposit_stake_msol_mint_auth,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.clock,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.rent,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.system_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.stake_program,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; MARINADE_DEPOSIT_STAKE_IX_ACCOUNTS_LEN]> for MarinadeDepositStakeKeys {
    fn from(pubkeys: [Pubkey; MARINADE_DEPOSIT_STAKE_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            marinade_program: pubkeys[0],
            deposit_stake_marinade_state: pubkeys[1],
            deposit_stake_validator_list: pubkeys[2],
            deposit_stake_stake_list: pubkeys[3],
            deposit_stake_duplication_flag: pubkeys[4],
            deposit_stake_msol_mint_auth: pubkeys[5],
            clock: pubkeys[6],
            rent: pubkeys[7],
            system_program: pubkeys[8],
            token_program: pubkeys[9],
            stake_program: pubkeys[10],
        }
    }
}
impl<'info> From<MarinadeDepositStakeAccounts<'_, 'info>>
    for [AccountInfo<'info>; MARINADE_DEPOSIT_STAKE_IX_ACCOUNTS_LEN]
{
    fn from(accounts: MarinadeDepositStakeAccounts<'_, 'info>) -> Self {
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
impl<'me, 'info> From<&'me [AccountInfo<'info>; MARINADE_DEPOSIT_STAKE_IX_ACCOUNTS_LEN]>
    for MarinadeDepositStakeAccounts<'me, 'info>
{
    fn from(arr: &'me [AccountInfo<'info>; MARINADE_DEPOSIT_STAKE_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            marinade_program: &arr[0],
            deposit_stake_marinade_state: &arr[1],
            deposit_stake_validator_list: &arr[2],
            deposit_stake_stake_list: &arr[3],
            deposit_stake_duplication_flag: &arr[4],
            deposit_stake_msol_mint_auth: &arr[5],
            clock: &arr[6],
            rent: &arr[7],
            system_program: &arr[8],
            token_program: &arr[9],
            stake_program: &arr[10],
        }
    }
}
pub const MARINADE_DEPOSIT_STAKE_IX_DISCM: u8 = 2u8;
#[derive(Clone, Debug, PartialEq)]
pub struct MarinadeDepositStakeIxData;
impl MarinadeDepositStakeIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm_buf = [0u8; 1];
        reader.read_exact(&mut maybe_discm_buf)?;
        let maybe_discm = maybe_discm_buf[0];
        if maybe_discm != MARINADE_DEPOSIT_STAKE_IX_DISCM {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    MARINADE_DEPOSIT_STAKE_IX_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&[MARINADE_DEPOSIT_STAKE_IX_DISCM])
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn marinade_deposit_stake_ix_with_program_id(
    program_id: Pubkey,
    keys: MarinadeDepositStakeKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; MARINADE_DEPOSIT_STAKE_IX_ACCOUNTS_LEN] = keys.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: MarinadeDepositStakeIxData.try_to_vec()?,
    })
}
pub fn marinade_deposit_stake_ix(keys: MarinadeDepositStakeKeys) -> std::io::Result<Instruction> {
    marinade_deposit_stake_ix_with_program_id(crate::ID, keys)
}
pub fn marinade_deposit_stake_invoke_with_program_id(
    program_id: Pubkey,
    accounts: MarinadeDepositStakeAccounts<'_, '_>,
) -> ProgramResult {
    let keys: MarinadeDepositStakeKeys = accounts.into();
    let ix = marinade_deposit_stake_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn marinade_deposit_stake_invoke(
    accounts: MarinadeDepositStakeAccounts<'_, '_>,
) -> ProgramResult {
    marinade_deposit_stake_invoke_with_program_id(crate::ID, accounts)
}
pub fn marinade_deposit_stake_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: MarinadeDepositStakeAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: MarinadeDepositStakeKeys = accounts.into();
    let ix = marinade_deposit_stake_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn marinade_deposit_stake_invoke_signed(
    accounts: MarinadeDepositStakeAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    marinade_deposit_stake_invoke_signed_with_program_id(crate::ID, accounts, seeds)
}
pub fn marinade_deposit_stake_verify_account_keys(
    accounts: MarinadeDepositStakeAccounts<'_, '_>,
    keys: MarinadeDepositStakeKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (accounts.marinade_program.key, &keys.marinade_program),
        (
            accounts.deposit_stake_marinade_state.key,
            &keys.deposit_stake_marinade_state,
        ),
        (
            accounts.deposit_stake_validator_list.key,
            &keys.deposit_stake_validator_list,
        ),
        (
            accounts.deposit_stake_stake_list.key,
            &keys.deposit_stake_stake_list,
        ),
        (
            accounts.deposit_stake_duplication_flag.key,
            &keys.deposit_stake_duplication_flag,
        ),
        (
            accounts.deposit_stake_msol_mint_auth.key,
            &keys.deposit_stake_msol_mint_auth,
        ),
        (accounts.clock.key, &keys.clock),
        (accounts.rent.key, &keys.rent),
        (accounts.system_program.key, &keys.system_program),
        (accounts.token_program.key, &keys.token_program),
        (accounts.stake_program.key, &keys.stake_program),
    ] {
        if actual != expected {
            return Err((*actual, *expected));
        }
    }
    Ok(())
}
pub fn marinade_deposit_stake_verify_writable_privileges<'me, 'info>(
    accounts: MarinadeDepositStakeAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.deposit_stake_marinade_state,
        accounts.deposit_stake_validator_list,
        accounts.deposit_stake_stake_list,
        accounts.deposit_stake_duplication_flag,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn marinade_deposit_stake_verify_account_privileges<'me, 'info>(
    accounts: MarinadeDepositStakeAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    marinade_deposit_stake_verify_writable_privileges(accounts)?;
    Ok(())
}
pub const UNSTAKE_IT_DEPOSIT_STAKE_IX_ACCOUNTS_LEN: usize = 11;
#[derive(Copy, Clone, Debug)]
pub struct UnstakeItDepositStakeAccounts<'me, 'info> {
    pub unstakeit_program: &'me AccountInfo<'info>,
    pub deposit_stake_unstake_pool: &'me AccountInfo<'info>,
    pub deposit_stake_pool_sol_reserves: &'me AccountInfo<'info>,
    pub deposit_stake_unstake_fee: &'me AccountInfo<'info>,
    pub deposit_stake_stake_acc_record: &'me AccountInfo<'info>,
    pub deposit_stake_protocol_fee: &'me AccountInfo<'info>,
    pub deposit_stake_protocol_fee_dest: &'me AccountInfo<'info>,
    pub clock: &'me AccountInfo<'info>,
    pub stake_program: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
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
impl From<UnstakeItDepositStakeAccounts<'_, '_>> for UnstakeItDepositStakeKeys {
    fn from(accounts: UnstakeItDepositStakeAccounts) -> Self {
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
impl From<UnstakeItDepositStakeKeys> for [AccountMeta; UNSTAKE_IT_DEPOSIT_STAKE_IX_ACCOUNTS_LEN] {
    fn from(keys: UnstakeItDepositStakeKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.unstakeit_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.deposit_stake_unstake_pool,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.deposit_stake_pool_sol_reserves,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.deposit_stake_unstake_fee,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.deposit_stake_stake_acc_record,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.deposit_stake_protocol_fee,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.deposit_stake_protocol_fee_dest,
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
            AccountMeta {
                pubkey: keys.token_program,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; UNSTAKE_IT_DEPOSIT_STAKE_IX_ACCOUNTS_LEN]> for UnstakeItDepositStakeKeys {
    fn from(pubkeys: [Pubkey; UNSTAKE_IT_DEPOSIT_STAKE_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            unstakeit_program: pubkeys[0],
            deposit_stake_unstake_pool: pubkeys[1],
            deposit_stake_pool_sol_reserves: pubkeys[2],
            deposit_stake_unstake_fee: pubkeys[3],
            deposit_stake_stake_acc_record: pubkeys[4],
            deposit_stake_protocol_fee: pubkeys[5],
            deposit_stake_protocol_fee_dest: pubkeys[6],
            clock: pubkeys[7],
            stake_program: pubkeys[8],
            system_program: pubkeys[9],
            token_program: pubkeys[10],
        }
    }
}
impl<'info> From<UnstakeItDepositStakeAccounts<'_, 'info>>
    for [AccountInfo<'info>; UNSTAKE_IT_DEPOSIT_STAKE_IX_ACCOUNTS_LEN]
{
    fn from(accounts: UnstakeItDepositStakeAccounts<'_, 'info>) -> Self {
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
impl<'me, 'info> From<&'me [AccountInfo<'info>; UNSTAKE_IT_DEPOSIT_STAKE_IX_ACCOUNTS_LEN]>
    for UnstakeItDepositStakeAccounts<'me, 'info>
{
    fn from(arr: &'me [AccountInfo<'info>; UNSTAKE_IT_DEPOSIT_STAKE_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            unstakeit_program: &arr[0],
            deposit_stake_unstake_pool: &arr[1],
            deposit_stake_pool_sol_reserves: &arr[2],
            deposit_stake_unstake_fee: &arr[3],
            deposit_stake_stake_acc_record: &arr[4],
            deposit_stake_protocol_fee: &arr[5],
            deposit_stake_protocol_fee_dest: &arr[6],
            clock: &arr[7],
            stake_program: &arr[8],
            system_program: &arr[9],
            token_program: &arr[10],
        }
    }
}
pub const UNSTAKE_IT_DEPOSIT_STAKE_IX_DISCM: u8 = 3u8;
#[derive(Clone, Debug, PartialEq)]
pub struct UnstakeItDepositStakeIxData;
impl UnstakeItDepositStakeIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm_buf = [0u8; 1];
        reader.read_exact(&mut maybe_discm_buf)?;
        let maybe_discm = maybe_discm_buf[0];
        if maybe_discm != UNSTAKE_IT_DEPOSIT_STAKE_IX_DISCM {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    UNSTAKE_IT_DEPOSIT_STAKE_IX_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&[UNSTAKE_IT_DEPOSIT_STAKE_IX_DISCM])
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn unstake_it_deposit_stake_ix_with_program_id(
    program_id: Pubkey,
    keys: UnstakeItDepositStakeKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; UNSTAKE_IT_DEPOSIT_STAKE_IX_ACCOUNTS_LEN] = keys.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: UnstakeItDepositStakeIxData.try_to_vec()?,
    })
}
pub fn unstake_it_deposit_stake_ix(
    keys: UnstakeItDepositStakeKeys,
) -> std::io::Result<Instruction> {
    unstake_it_deposit_stake_ix_with_program_id(crate::ID, keys)
}
pub fn unstake_it_deposit_stake_invoke_with_program_id(
    program_id: Pubkey,
    accounts: UnstakeItDepositStakeAccounts<'_, '_>,
) -> ProgramResult {
    let keys: UnstakeItDepositStakeKeys = accounts.into();
    let ix = unstake_it_deposit_stake_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn unstake_it_deposit_stake_invoke(
    accounts: UnstakeItDepositStakeAccounts<'_, '_>,
) -> ProgramResult {
    unstake_it_deposit_stake_invoke_with_program_id(crate::ID, accounts)
}
pub fn unstake_it_deposit_stake_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: UnstakeItDepositStakeAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: UnstakeItDepositStakeKeys = accounts.into();
    let ix = unstake_it_deposit_stake_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn unstake_it_deposit_stake_invoke_signed(
    accounts: UnstakeItDepositStakeAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    unstake_it_deposit_stake_invoke_signed_with_program_id(crate::ID, accounts, seeds)
}
pub fn unstake_it_deposit_stake_verify_account_keys(
    accounts: UnstakeItDepositStakeAccounts<'_, '_>,
    keys: UnstakeItDepositStakeKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (accounts.unstakeit_program.key, &keys.unstakeit_program),
        (
            accounts.deposit_stake_unstake_pool.key,
            &keys.deposit_stake_unstake_pool,
        ),
        (
            accounts.deposit_stake_pool_sol_reserves.key,
            &keys.deposit_stake_pool_sol_reserves,
        ),
        (
            accounts.deposit_stake_unstake_fee.key,
            &keys.deposit_stake_unstake_fee,
        ),
        (
            accounts.deposit_stake_stake_acc_record.key,
            &keys.deposit_stake_stake_acc_record,
        ),
        (
            accounts.deposit_stake_protocol_fee.key,
            &keys.deposit_stake_protocol_fee,
        ),
        (
            accounts.deposit_stake_protocol_fee_dest.key,
            &keys.deposit_stake_protocol_fee_dest,
        ),
        (accounts.clock.key, &keys.clock),
        (accounts.stake_program.key, &keys.stake_program),
        (accounts.system_program.key, &keys.system_program),
        (accounts.token_program.key, &keys.token_program),
    ] {
        if actual != expected {
            return Err((*actual, *expected));
        }
    }
    Ok(())
}
pub fn unstake_it_deposit_stake_verify_writable_privileges<'me, 'info>(
    accounts: UnstakeItDepositStakeAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.deposit_stake_unstake_pool,
        accounts.deposit_stake_pool_sol_reserves,
        accounts.deposit_stake_stake_acc_record,
        accounts.deposit_stake_protocol_fee_dest,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn unstake_it_deposit_stake_verify_account_privileges<'me, 'info>(
    accounts: UnstakeItDepositStakeAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    unstake_it_deposit_stake_verify_writable_privileges(accounts)?;
    Ok(())
}
pub const SANCTUM_SPL_STAKE_POOL_DEPOSIT_STAKE_IX_ACCOUNTS_LEN: usize = 12;
#[derive(Copy, Clone, Debug)]
pub struct SanctumSplStakePoolDepositStakeAccounts<'me, 'info> {
    pub sanctum_spl_stake_pool_program: &'me AccountInfo<'info>,
    pub deposit_stake_spl_stake_pool: &'me AccountInfo<'info>,
    pub deposit_stake_validator_list: &'me AccountInfo<'info>,
    pub deposit_stake_deposit_authority: &'me AccountInfo<'info>,
    pub deposit_stake_withdraw_authority: &'me AccountInfo<'info>,
    pub deposit_stake_validator_stake: &'me AccountInfo<'info>,
    pub deposit_stake_reserve_stake: &'me AccountInfo<'info>,
    pub deposit_stake_manager_fee: &'me AccountInfo<'info>,
    pub clock: &'me AccountInfo<'info>,
    pub stake_history: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
    pub stake_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug)]
pub struct SanctumSplStakePoolDepositStakeKeys {
    pub sanctum_spl_stake_pool_program: Pubkey,
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
impl From<SanctumSplStakePoolDepositStakeAccounts<'_, '_>> for SanctumSplStakePoolDepositStakeKeys {
    fn from(accounts: SanctumSplStakePoolDepositStakeAccounts) -> Self {
        Self {
            sanctum_spl_stake_pool_program: *accounts.sanctum_spl_stake_pool_program.key,
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
impl From<SanctumSplStakePoolDepositStakeKeys>
    for [AccountMeta; SANCTUM_SPL_STAKE_POOL_DEPOSIT_STAKE_IX_ACCOUNTS_LEN]
{
    fn from(keys: SanctumSplStakePoolDepositStakeKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.sanctum_spl_stake_pool_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.deposit_stake_spl_stake_pool,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.deposit_stake_validator_list,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.deposit_stake_deposit_authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.deposit_stake_withdraw_authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.deposit_stake_validator_stake,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.deposit_stake_reserve_stake,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.deposit_stake_manager_fee,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.clock,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.stake_history,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.stake_program,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; SANCTUM_SPL_STAKE_POOL_DEPOSIT_STAKE_IX_ACCOUNTS_LEN]>
    for SanctumSplStakePoolDepositStakeKeys
{
    fn from(pubkeys: [Pubkey; SANCTUM_SPL_STAKE_POOL_DEPOSIT_STAKE_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            sanctum_spl_stake_pool_program: pubkeys[0],
            deposit_stake_spl_stake_pool: pubkeys[1],
            deposit_stake_validator_list: pubkeys[2],
            deposit_stake_deposit_authority: pubkeys[3],
            deposit_stake_withdraw_authority: pubkeys[4],
            deposit_stake_validator_stake: pubkeys[5],
            deposit_stake_reserve_stake: pubkeys[6],
            deposit_stake_manager_fee: pubkeys[7],
            clock: pubkeys[8],
            stake_history: pubkeys[9],
            token_program: pubkeys[10],
            stake_program: pubkeys[11],
        }
    }
}
impl<'info> From<SanctumSplStakePoolDepositStakeAccounts<'_, 'info>>
    for [AccountInfo<'info>; SANCTUM_SPL_STAKE_POOL_DEPOSIT_STAKE_IX_ACCOUNTS_LEN]
{
    fn from(accounts: SanctumSplStakePoolDepositStakeAccounts<'_, 'info>) -> Self {
        [
            accounts.sanctum_spl_stake_pool_program.clone(),
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
impl<'me, 'info>
    From<&'me [AccountInfo<'info>; SANCTUM_SPL_STAKE_POOL_DEPOSIT_STAKE_IX_ACCOUNTS_LEN]>
    for SanctumSplStakePoolDepositStakeAccounts<'me, 'info>
{
    fn from(
        arr: &'me [AccountInfo<'info>; SANCTUM_SPL_STAKE_POOL_DEPOSIT_STAKE_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            sanctum_spl_stake_pool_program: &arr[0],
            deposit_stake_spl_stake_pool: &arr[1],
            deposit_stake_validator_list: &arr[2],
            deposit_stake_deposit_authority: &arr[3],
            deposit_stake_withdraw_authority: &arr[4],
            deposit_stake_validator_stake: &arr[5],
            deposit_stake_reserve_stake: &arr[6],
            deposit_stake_manager_fee: &arr[7],
            clock: &arr[8],
            stake_history: &arr[9],
            token_program: &arr[10],
            stake_program: &arr[11],
        }
    }
}
pub const SANCTUM_SPL_STAKE_POOL_DEPOSIT_STAKE_IX_DISCM: u8 = 4u8;
#[derive(Clone, Debug, PartialEq)]
pub struct SanctumSplStakePoolDepositStakeIxData;
impl SanctumSplStakePoolDepositStakeIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm_buf = [0u8; 1];
        reader.read_exact(&mut maybe_discm_buf)?;
        let maybe_discm = maybe_discm_buf[0];
        if maybe_discm != SANCTUM_SPL_STAKE_POOL_DEPOSIT_STAKE_IX_DISCM {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    SANCTUM_SPL_STAKE_POOL_DEPOSIT_STAKE_IX_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&[SANCTUM_SPL_STAKE_POOL_DEPOSIT_STAKE_IX_DISCM])
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn sanctum_spl_stake_pool_deposit_stake_ix_with_program_id(
    program_id: Pubkey,
    keys: SanctumSplStakePoolDepositStakeKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; SANCTUM_SPL_STAKE_POOL_DEPOSIT_STAKE_IX_ACCOUNTS_LEN] = keys.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: SanctumSplStakePoolDepositStakeIxData.try_to_vec()?,
    })
}
pub fn sanctum_spl_stake_pool_deposit_stake_ix(
    keys: SanctumSplStakePoolDepositStakeKeys,
) -> std::io::Result<Instruction> {
    sanctum_spl_stake_pool_deposit_stake_ix_with_program_id(crate::ID, keys)
}
pub fn sanctum_spl_stake_pool_deposit_stake_invoke_with_program_id(
    program_id: Pubkey,
    accounts: SanctumSplStakePoolDepositStakeAccounts<'_, '_>,
) -> ProgramResult {
    let keys: SanctumSplStakePoolDepositStakeKeys = accounts.into();
    let ix = sanctum_spl_stake_pool_deposit_stake_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn sanctum_spl_stake_pool_deposit_stake_invoke(
    accounts: SanctumSplStakePoolDepositStakeAccounts<'_, '_>,
) -> ProgramResult {
    sanctum_spl_stake_pool_deposit_stake_invoke_with_program_id(crate::ID, accounts)
}
pub fn sanctum_spl_stake_pool_deposit_stake_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: SanctumSplStakePoolDepositStakeAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: SanctumSplStakePoolDepositStakeKeys = accounts.into();
    let ix = sanctum_spl_stake_pool_deposit_stake_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn sanctum_spl_stake_pool_deposit_stake_invoke_signed(
    accounts: SanctumSplStakePoolDepositStakeAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    sanctum_spl_stake_pool_deposit_stake_invoke_signed_with_program_id(crate::ID, accounts, seeds)
}
pub fn sanctum_spl_stake_pool_deposit_stake_verify_account_keys(
    accounts: SanctumSplStakePoolDepositStakeAccounts<'_, '_>,
    keys: SanctumSplStakePoolDepositStakeKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (
            accounts.sanctum_spl_stake_pool_program.key,
            &keys.sanctum_spl_stake_pool_program,
        ),
        (
            accounts.deposit_stake_spl_stake_pool.key,
            &keys.deposit_stake_spl_stake_pool,
        ),
        (
            accounts.deposit_stake_validator_list.key,
            &keys.deposit_stake_validator_list,
        ),
        (
            accounts.deposit_stake_deposit_authority.key,
            &keys.deposit_stake_deposit_authority,
        ),
        (
            accounts.deposit_stake_withdraw_authority.key,
            &keys.deposit_stake_withdraw_authority,
        ),
        (
            accounts.deposit_stake_validator_stake.key,
            &keys.deposit_stake_validator_stake,
        ),
        (
            accounts.deposit_stake_reserve_stake.key,
            &keys.deposit_stake_reserve_stake,
        ),
        (
            accounts.deposit_stake_manager_fee.key,
            &keys.deposit_stake_manager_fee,
        ),
        (accounts.clock.key, &keys.clock),
        (accounts.stake_history.key, &keys.stake_history),
        (accounts.token_program.key, &keys.token_program),
        (accounts.stake_program.key, &keys.stake_program),
    ] {
        if actual != expected {
            return Err((*actual, *expected));
        }
    }
    Ok(())
}
pub fn sanctum_spl_stake_pool_deposit_stake_verify_writable_privileges<'me, 'info>(
    accounts: SanctumSplStakePoolDepositStakeAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.deposit_stake_spl_stake_pool,
        accounts.deposit_stake_validator_list,
        accounts.deposit_stake_validator_stake,
        accounts.deposit_stake_reserve_stake,
        accounts.deposit_stake_manager_fee,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn sanctum_spl_stake_pool_deposit_stake_verify_account_privileges<'me, 'info>(
    accounts: SanctumSplStakePoolDepositStakeAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    sanctum_spl_stake_pool_deposit_stake_verify_writable_privileges(accounts)?;
    Ok(())
}
pub const SANCTUM_SPL_MULTI_STAKE_POOL_DEPOSIT_STAKE_IX_ACCOUNTS_LEN: usize = 12;
#[derive(Copy, Clone, Debug)]
pub struct SanctumSplMultiStakePoolDepositStakeAccounts<'me, 'info> {
    pub sanctum_spl_multi_stake_pool_program: &'me AccountInfo<'info>,
    pub deposit_stake_spl_stake_pool: &'me AccountInfo<'info>,
    pub deposit_stake_validator_list: &'me AccountInfo<'info>,
    pub deposit_stake_deposit_authority: &'me AccountInfo<'info>,
    pub deposit_stake_withdraw_authority: &'me AccountInfo<'info>,
    pub deposit_stake_validator_stake: &'me AccountInfo<'info>,
    pub deposit_stake_reserve_stake: &'me AccountInfo<'info>,
    pub deposit_stake_manager_fee: &'me AccountInfo<'info>,
    pub clock: &'me AccountInfo<'info>,
    pub stake_history: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
    pub stake_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug)]
pub struct SanctumSplMultiStakePoolDepositStakeKeys {
    pub sanctum_spl_multi_stake_pool_program: Pubkey,
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
impl From<SanctumSplMultiStakePoolDepositStakeAccounts<'_, '_>>
    for SanctumSplMultiStakePoolDepositStakeKeys
{
    fn from(accounts: SanctumSplMultiStakePoolDepositStakeAccounts) -> Self {
        Self {
            sanctum_spl_multi_stake_pool_program: *accounts
                .sanctum_spl_multi_stake_pool_program
                .key,
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
impl From<SanctumSplMultiStakePoolDepositStakeKeys>
    for [AccountMeta; SANCTUM_SPL_MULTI_STAKE_POOL_DEPOSIT_STAKE_IX_ACCOUNTS_LEN]
{
    fn from(keys: SanctumSplMultiStakePoolDepositStakeKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.sanctum_spl_multi_stake_pool_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.deposit_stake_spl_stake_pool,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.deposit_stake_validator_list,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.deposit_stake_deposit_authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.deposit_stake_withdraw_authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.deposit_stake_validator_stake,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.deposit_stake_reserve_stake,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.deposit_stake_manager_fee,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.clock,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.stake_history,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.stake_program,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; SANCTUM_SPL_MULTI_STAKE_POOL_DEPOSIT_STAKE_IX_ACCOUNTS_LEN]>
    for SanctumSplMultiStakePoolDepositStakeKeys
{
    fn from(pubkeys: [Pubkey; SANCTUM_SPL_MULTI_STAKE_POOL_DEPOSIT_STAKE_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            sanctum_spl_multi_stake_pool_program: pubkeys[0],
            deposit_stake_spl_stake_pool: pubkeys[1],
            deposit_stake_validator_list: pubkeys[2],
            deposit_stake_deposit_authority: pubkeys[3],
            deposit_stake_withdraw_authority: pubkeys[4],
            deposit_stake_validator_stake: pubkeys[5],
            deposit_stake_reserve_stake: pubkeys[6],
            deposit_stake_manager_fee: pubkeys[7],
            clock: pubkeys[8],
            stake_history: pubkeys[9],
            token_program: pubkeys[10],
            stake_program: pubkeys[11],
        }
    }
}
impl<'info> From<SanctumSplMultiStakePoolDepositStakeAccounts<'_, 'info>>
    for [AccountInfo<'info>; SANCTUM_SPL_MULTI_STAKE_POOL_DEPOSIT_STAKE_IX_ACCOUNTS_LEN]
{
    fn from(accounts: SanctumSplMultiStakePoolDepositStakeAccounts<'_, 'info>) -> Self {
        [
            accounts.sanctum_spl_multi_stake_pool_program.clone(),
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
impl<'me, 'info>
    From<&'me [AccountInfo<'info>; SANCTUM_SPL_MULTI_STAKE_POOL_DEPOSIT_STAKE_IX_ACCOUNTS_LEN]>
    for SanctumSplMultiStakePoolDepositStakeAccounts<'me, 'info>
{
    fn from(
        arr: &'me [AccountInfo<'info>; SANCTUM_SPL_MULTI_STAKE_POOL_DEPOSIT_STAKE_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            sanctum_spl_multi_stake_pool_program: &arr[0],
            deposit_stake_spl_stake_pool: &arr[1],
            deposit_stake_validator_list: &arr[2],
            deposit_stake_deposit_authority: &arr[3],
            deposit_stake_withdraw_authority: &arr[4],
            deposit_stake_validator_stake: &arr[5],
            deposit_stake_reserve_stake: &arr[6],
            deposit_stake_manager_fee: &arr[7],
            clock: &arr[8],
            stake_history: &arr[9],
            token_program: &arr[10],
            stake_program: &arr[11],
        }
    }
}
pub const SANCTUM_SPL_MULTI_STAKE_POOL_DEPOSIT_STAKE_IX_DISCM: u8 = 5u8;
#[derive(Clone, Debug, PartialEq)]
pub struct SanctumSplMultiStakePoolDepositStakeIxData;
impl SanctumSplMultiStakePoolDepositStakeIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm_buf = [0u8; 1];
        reader.read_exact(&mut maybe_discm_buf)?;
        let maybe_discm = maybe_discm_buf[0];
        if maybe_discm != SANCTUM_SPL_MULTI_STAKE_POOL_DEPOSIT_STAKE_IX_DISCM {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    SANCTUM_SPL_MULTI_STAKE_POOL_DEPOSIT_STAKE_IX_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&[SANCTUM_SPL_MULTI_STAKE_POOL_DEPOSIT_STAKE_IX_DISCM])
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn sanctum_spl_multi_stake_pool_deposit_stake_ix_with_program_id(
    program_id: Pubkey,
    keys: SanctumSplMultiStakePoolDepositStakeKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; SANCTUM_SPL_MULTI_STAKE_POOL_DEPOSIT_STAKE_IX_ACCOUNTS_LEN] =
        keys.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: SanctumSplMultiStakePoolDepositStakeIxData.try_to_vec()?,
    })
}
pub fn sanctum_spl_multi_stake_pool_deposit_stake_ix(
    keys: SanctumSplMultiStakePoolDepositStakeKeys,
) -> std::io::Result<Instruction> {
    sanctum_spl_multi_stake_pool_deposit_stake_ix_with_program_id(crate::ID, keys)
}
pub fn sanctum_spl_multi_stake_pool_deposit_stake_invoke_with_program_id(
    program_id: Pubkey,
    accounts: SanctumSplMultiStakePoolDepositStakeAccounts<'_, '_>,
) -> ProgramResult {
    let keys: SanctumSplMultiStakePoolDepositStakeKeys = accounts.into();
    let ix = sanctum_spl_multi_stake_pool_deposit_stake_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn sanctum_spl_multi_stake_pool_deposit_stake_invoke(
    accounts: SanctumSplMultiStakePoolDepositStakeAccounts<'_, '_>,
) -> ProgramResult {
    sanctum_spl_multi_stake_pool_deposit_stake_invoke_with_program_id(crate::ID, accounts)
}
pub fn sanctum_spl_multi_stake_pool_deposit_stake_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: SanctumSplMultiStakePoolDepositStakeAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: SanctumSplMultiStakePoolDepositStakeKeys = accounts.into();
    let ix = sanctum_spl_multi_stake_pool_deposit_stake_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn sanctum_spl_multi_stake_pool_deposit_stake_invoke_signed(
    accounts: SanctumSplMultiStakePoolDepositStakeAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    sanctum_spl_multi_stake_pool_deposit_stake_invoke_signed_with_program_id(
        crate::ID,
        accounts,
        seeds,
    )
}
pub fn sanctum_spl_multi_stake_pool_deposit_stake_verify_account_keys(
    accounts: SanctumSplMultiStakePoolDepositStakeAccounts<'_, '_>,
    keys: SanctumSplMultiStakePoolDepositStakeKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (
            accounts.sanctum_spl_multi_stake_pool_program.key,
            &keys.sanctum_spl_multi_stake_pool_program,
        ),
        (
            accounts.deposit_stake_spl_stake_pool.key,
            &keys.deposit_stake_spl_stake_pool,
        ),
        (
            accounts.deposit_stake_validator_list.key,
            &keys.deposit_stake_validator_list,
        ),
        (
            accounts.deposit_stake_deposit_authority.key,
            &keys.deposit_stake_deposit_authority,
        ),
        (
            accounts.deposit_stake_withdraw_authority.key,
            &keys.deposit_stake_withdraw_authority,
        ),
        (
            accounts.deposit_stake_validator_stake.key,
            &keys.deposit_stake_validator_stake,
        ),
        (
            accounts.deposit_stake_reserve_stake.key,
            &keys.deposit_stake_reserve_stake,
        ),
        (
            accounts.deposit_stake_manager_fee.key,
            &keys.deposit_stake_manager_fee,
        ),
        (accounts.clock.key, &keys.clock),
        (accounts.stake_history.key, &keys.stake_history),
        (accounts.token_program.key, &keys.token_program),
        (accounts.stake_program.key, &keys.stake_program),
    ] {
        if actual != expected {
            return Err((*actual, *expected));
        }
    }
    Ok(())
}
pub fn sanctum_spl_multi_stake_pool_deposit_stake_verify_writable_privileges<'me, 'info>(
    accounts: SanctumSplMultiStakePoolDepositStakeAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.deposit_stake_spl_stake_pool,
        accounts.deposit_stake_validator_list,
        accounts.deposit_stake_validator_stake,
        accounts.deposit_stake_reserve_stake,
        accounts.deposit_stake_manager_fee,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn sanctum_spl_multi_stake_pool_deposit_stake_verify_account_privileges<'me, 'info>(
    accounts: SanctumSplMultiStakePoolDepositStakeAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    sanctum_spl_multi_stake_pool_deposit_stake_verify_writable_privileges(accounts)?;
    Ok(())
}
