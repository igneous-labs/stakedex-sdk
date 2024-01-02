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
pub enum StakedexWithdrawStakeProgramIx {
    SoceanStakePoolWithdrawStake,
    SplStakePoolWithdrawStake,
    LidoWithdrawStake,
    MarinadeWithdrawStake,
}
impl StakedexWithdrawStakeProgramIx {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm_buf = [0u8; 1];
        reader.read_exact(&mut maybe_discm_buf)?;
        let maybe_discm = maybe_discm_buf[0];
        match maybe_discm {
            SOCEAN_STAKE_POOL_WITHDRAW_STAKE_IX_DISCM => Ok(Self::SoceanStakePoolWithdrawStake),
            SPL_STAKE_POOL_WITHDRAW_STAKE_IX_DISCM => Ok(Self::SplStakePoolWithdrawStake),
            LIDO_WITHDRAW_STAKE_IX_DISCM => Ok(Self::LidoWithdrawStake),
            MARINADE_WITHDRAW_STAKE_IX_DISCM => Ok(Self::MarinadeWithdrawStake),
            _ => Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!("discm {:?} not found", maybe_discm),
            )),
        }
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        match self {
            Self::SoceanStakePoolWithdrawStake => {
                writer.write_all(&[SOCEAN_STAKE_POOL_WITHDRAW_STAKE_IX_DISCM])
            }
            Self::SplStakePoolWithdrawStake => {
                writer.write_all(&[SPL_STAKE_POOL_WITHDRAW_STAKE_IX_DISCM])
            }
            Self::LidoWithdrawStake => writer.write_all(&[LIDO_WITHDRAW_STAKE_IX_DISCM]),
            Self::MarinadeWithdrawStake => writer.write_all(&[MARINADE_WITHDRAW_STAKE_IX_DISCM]),
        }
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const SOCEAN_STAKE_POOL_WITHDRAW_STAKE_IX_ACCOUNTS_LEN: usize = 10;
#[derive(Copy, Clone, Debug)]
pub struct SoceanStakePoolWithdrawStakeAccounts<'me, 'info> {
    pub socean_stake_pool_program: &'me AccountInfo<'info>,
    pub withdraw_stake_spl_stake_pool: &'me AccountInfo<'info>,
    pub withdraw_stake_validator_list: &'me AccountInfo<'info>,
    pub withdraw_stake_withdraw_authority: &'me AccountInfo<'info>,
    pub withdraw_stake_stake_to_split: &'me AccountInfo<'info>,
    pub withdraw_stake_manager_fee: &'me AccountInfo<'info>,
    pub clock: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
    pub stake_program: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
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
impl From<SoceanStakePoolWithdrawStakeAccounts<'_, '_>> for SoceanStakePoolWithdrawStakeKeys {
    fn from(accounts: SoceanStakePoolWithdrawStakeAccounts) -> Self {
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
impl From<SoceanStakePoolWithdrawStakeKeys>
    for [AccountMeta; SOCEAN_STAKE_POOL_WITHDRAW_STAKE_IX_ACCOUNTS_LEN]
{
    fn from(keys: SoceanStakePoolWithdrawStakeKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.socean_stake_pool_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.withdraw_stake_spl_stake_pool,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.withdraw_stake_validator_list,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.withdraw_stake_withdraw_authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.withdraw_stake_stake_to_split,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.withdraw_stake_manager_fee,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.clock,
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
            AccountMeta {
                pubkey: keys.system_program,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; SOCEAN_STAKE_POOL_WITHDRAW_STAKE_IX_ACCOUNTS_LEN]>
    for SoceanStakePoolWithdrawStakeKeys
{
    fn from(pubkeys: [Pubkey; SOCEAN_STAKE_POOL_WITHDRAW_STAKE_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            socean_stake_pool_program: pubkeys[0],
            withdraw_stake_spl_stake_pool: pubkeys[1],
            withdraw_stake_validator_list: pubkeys[2],
            withdraw_stake_withdraw_authority: pubkeys[3],
            withdraw_stake_stake_to_split: pubkeys[4],
            withdraw_stake_manager_fee: pubkeys[5],
            clock: pubkeys[6],
            token_program: pubkeys[7],
            stake_program: pubkeys[8],
            system_program: pubkeys[9],
        }
    }
}
impl<'info> From<SoceanStakePoolWithdrawStakeAccounts<'_, 'info>>
    for [AccountInfo<'info>; SOCEAN_STAKE_POOL_WITHDRAW_STAKE_IX_ACCOUNTS_LEN]
{
    fn from(accounts: SoceanStakePoolWithdrawStakeAccounts<'_, 'info>) -> Self {
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
impl<'me, 'info> From<&'me [AccountInfo<'info>; SOCEAN_STAKE_POOL_WITHDRAW_STAKE_IX_ACCOUNTS_LEN]>
    for SoceanStakePoolWithdrawStakeAccounts<'me, 'info>
{
    fn from(
        arr: &'me [AccountInfo<'info>; SOCEAN_STAKE_POOL_WITHDRAW_STAKE_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            socean_stake_pool_program: &arr[0],
            withdraw_stake_spl_stake_pool: &arr[1],
            withdraw_stake_validator_list: &arr[2],
            withdraw_stake_withdraw_authority: &arr[3],
            withdraw_stake_stake_to_split: &arr[4],
            withdraw_stake_manager_fee: &arr[5],
            clock: &arr[6],
            token_program: &arr[7],
            stake_program: &arr[8],
            system_program: &arr[9],
        }
    }
}
pub const SOCEAN_STAKE_POOL_WITHDRAW_STAKE_IX_DISCM: u8 = 0u8;
#[derive(Clone, Debug, PartialEq)]
pub struct SoceanStakePoolWithdrawStakeIxData;
impl SoceanStakePoolWithdrawStakeIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm_buf = [0u8; 1];
        reader.read_exact(&mut maybe_discm_buf)?;
        let maybe_discm = maybe_discm_buf[0];
        if maybe_discm != SOCEAN_STAKE_POOL_WITHDRAW_STAKE_IX_DISCM {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    SOCEAN_STAKE_POOL_WITHDRAW_STAKE_IX_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&[SOCEAN_STAKE_POOL_WITHDRAW_STAKE_IX_DISCM])
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn socean_stake_pool_withdraw_stake_ix<K: Into<SoceanStakePoolWithdrawStakeKeys>>(
    accounts: K,
) -> std::io::Result<Instruction> {
    let keys: SoceanStakePoolWithdrawStakeKeys = accounts.into();
    let metas: [AccountMeta; SOCEAN_STAKE_POOL_WITHDRAW_STAKE_IX_ACCOUNTS_LEN] = keys.into();
    Ok(Instruction {
        program_id: crate::ID,
        accounts: Vec::from(metas),
        data: SoceanStakePoolWithdrawStakeIxData.try_to_vec()?,
    })
}
pub fn socean_stake_pool_withdraw_stake_invoke<'info>(
    accounts: SoceanStakePoolWithdrawStakeAccounts<'_, 'info>,
) -> ProgramResult {
    let ix = socean_stake_pool_withdraw_stake_ix(accounts)?;
    let account_info: [AccountInfo<'info>; SOCEAN_STAKE_POOL_WITHDRAW_STAKE_IX_ACCOUNTS_LEN] =
        accounts.into();
    invoke(&ix, &account_info)
}
pub fn socean_stake_pool_withdraw_stake_invoke_signed<'info>(
    accounts: SoceanStakePoolWithdrawStakeAccounts<'_, 'info>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let ix = socean_stake_pool_withdraw_stake_ix(accounts)?;
    let account_info: [AccountInfo<'info>; SOCEAN_STAKE_POOL_WITHDRAW_STAKE_IX_ACCOUNTS_LEN] =
        accounts.into();
    invoke_signed(&ix, &account_info, seeds)
}
pub fn socean_stake_pool_withdraw_stake_verify_account_keys(
    accounts: SoceanStakePoolWithdrawStakeAccounts<'_, '_>,
    keys: SoceanStakePoolWithdrawStakeKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (
            accounts.socean_stake_pool_program.key,
            &keys.socean_stake_pool_program,
        ),
        (
            accounts.withdraw_stake_spl_stake_pool.key,
            &keys.withdraw_stake_spl_stake_pool,
        ),
        (
            accounts.withdraw_stake_validator_list.key,
            &keys.withdraw_stake_validator_list,
        ),
        (
            accounts.withdraw_stake_withdraw_authority.key,
            &keys.withdraw_stake_withdraw_authority,
        ),
        (
            accounts.withdraw_stake_stake_to_split.key,
            &keys.withdraw_stake_stake_to_split,
        ),
        (
            accounts.withdraw_stake_manager_fee.key,
            &keys.withdraw_stake_manager_fee,
        ),
        (accounts.clock.key, &keys.clock),
        (accounts.token_program.key, &keys.token_program),
        (accounts.stake_program.key, &keys.stake_program),
        (accounts.system_program.key, &keys.system_program),
    ] {
        if actual != expected {
            return Err((*actual, *expected));
        }
    }
    Ok(())
}
pub fn socean_stake_pool_withdraw_stake_verify_account_privileges<'me, 'info>(
    accounts: SoceanStakePoolWithdrawStakeAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.withdraw_stake_spl_stake_pool,
        accounts.withdraw_stake_validator_list,
        accounts.withdraw_stake_stake_to_split,
        accounts.withdraw_stake_manager_fee,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub const SPL_STAKE_POOL_WITHDRAW_STAKE_IX_ACCOUNTS_LEN: usize = 10;
#[derive(Copy, Clone, Debug)]
pub struct SplStakePoolWithdrawStakeAccounts<'me, 'info> {
    pub spl_stake_pool_program: &'me AccountInfo<'info>,
    pub withdraw_stake_spl_stake_pool: &'me AccountInfo<'info>,
    pub withdraw_stake_validator_list: &'me AccountInfo<'info>,
    pub withdraw_stake_withdraw_authority: &'me AccountInfo<'info>,
    pub withdraw_stake_stake_to_split: &'me AccountInfo<'info>,
    pub withdraw_stake_manager_fee: &'me AccountInfo<'info>,
    pub clock: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
    pub stake_program: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
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
impl From<SplStakePoolWithdrawStakeAccounts<'_, '_>> for SplStakePoolWithdrawStakeKeys {
    fn from(accounts: SplStakePoolWithdrawStakeAccounts) -> Self {
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
impl From<SplStakePoolWithdrawStakeKeys>
    for [AccountMeta; SPL_STAKE_POOL_WITHDRAW_STAKE_IX_ACCOUNTS_LEN]
{
    fn from(keys: SplStakePoolWithdrawStakeKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.spl_stake_pool_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.withdraw_stake_spl_stake_pool,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.withdraw_stake_validator_list,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.withdraw_stake_withdraw_authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.withdraw_stake_stake_to_split,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.withdraw_stake_manager_fee,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.clock,
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
            AccountMeta {
                pubkey: keys.system_program,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; SPL_STAKE_POOL_WITHDRAW_STAKE_IX_ACCOUNTS_LEN]>
    for SplStakePoolWithdrawStakeKeys
{
    fn from(pubkeys: [Pubkey; SPL_STAKE_POOL_WITHDRAW_STAKE_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            spl_stake_pool_program: pubkeys[0],
            withdraw_stake_spl_stake_pool: pubkeys[1],
            withdraw_stake_validator_list: pubkeys[2],
            withdraw_stake_withdraw_authority: pubkeys[3],
            withdraw_stake_stake_to_split: pubkeys[4],
            withdraw_stake_manager_fee: pubkeys[5],
            clock: pubkeys[6],
            token_program: pubkeys[7],
            stake_program: pubkeys[8],
            system_program: pubkeys[9],
        }
    }
}
impl<'info> From<SplStakePoolWithdrawStakeAccounts<'_, 'info>>
    for [AccountInfo<'info>; SPL_STAKE_POOL_WITHDRAW_STAKE_IX_ACCOUNTS_LEN]
{
    fn from(accounts: SplStakePoolWithdrawStakeAccounts<'_, 'info>) -> Self {
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
impl<'me, 'info> From<&'me [AccountInfo<'info>; SPL_STAKE_POOL_WITHDRAW_STAKE_IX_ACCOUNTS_LEN]>
    for SplStakePoolWithdrawStakeAccounts<'me, 'info>
{
    fn from(arr: &'me [AccountInfo<'info>; SPL_STAKE_POOL_WITHDRAW_STAKE_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            spl_stake_pool_program: &arr[0],
            withdraw_stake_spl_stake_pool: &arr[1],
            withdraw_stake_validator_list: &arr[2],
            withdraw_stake_withdraw_authority: &arr[3],
            withdraw_stake_stake_to_split: &arr[4],
            withdraw_stake_manager_fee: &arr[5],
            clock: &arr[6],
            token_program: &arr[7],
            stake_program: &arr[8],
            system_program: &arr[9],
        }
    }
}
pub const SPL_STAKE_POOL_WITHDRAW_STAKE_IX_DISCM: u8 = 1u8;
#[derive(Clone, Debug, PartialEq)]
pub struct SplStakePoolWithdrawStakeIxData;
impl SplStakePoolWithdrawStakeIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm_buf = [0u8; 1];
        reader.read_exact(&mut maybe_discm_buf)?;
        let maybe_discm = maybe_discm_buf[0];
        if maybe_discm != SPL_STAKE_POOL_WITHDRAW_STAKE_IX_DISCM {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    SPL_STAKE_POOL_WITHDRAW_STAKE_IX_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&[SPL_STAKE_POOL_WITHDRAW_STAKE_IX_DISCM])
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn spl_stake_pool_withdraw_stake_ix<K: Into<SplStakePoolWithdrawStakeKeys>>(
    accounts: K,
) -> std::io::Result<Instruction> {
    let keys: SplStakePoolWithdrawStakeKeys = accounts.into();
    let metas: [AccountMeta; SPL_STAKE_POOL_WITHDRAW_STAKE_IX_ACCOUNTS_LEN] = keys.into();
    Ok(Instruction {
        program_id: crate::ID,
        accounts: Vec::from(metas),
        data: SplStakePoolWithdrawStakeIxData.try_to_vec()?,
    })
}
pub fn spl_stake_pool_withdraw_stake_invoke<'info>(
    accounts: SplStakePoolWithdrawStakeAccounts<'_, 'info>,
) -> ProgramResult {
    let ix = spl_stake_pool_withdraw_stake_ix(accounts)?;
    let account_info: [AccountInfo<'info>; SPL_STAKE_POOL_WITHDRAW_STAKE_IX_ACCOUNTS_LEN] =
        accounts.into();
    invoke(&ix, &account_info)
}
pub fn spl_stake_pool_withdraw_stake_invoke_signed<'info>(
    accounts: SplStakePoolWithdrawStakeAccounts<'_, 'info>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let ix = spl_stake_pool_withdraw_stake_ix(accounts)?;
    let account_info: [AccountInfo<'info>; SPL_STAKE_POOL_WITHDRAW_STAKE_IX_ACCOUNTS_LEN] =
        accounts.into();
    invoke_signed(&ix, &account_info, seeds)
}
pub fn spl_stake_pool_withdraw_stake_verify_account_keys(
    accounts: SplStakePoolWithdrawStakeAccounts<'_, '_>,
    keys: SplStakePoolWithdrawStakeKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (
            accounts.spl_stake_pool_program.key,
            &keys.spl_stake_pool_program,
        ),
        (
            accounts.withdraw_stake_spl_stake_pool.key,
            &keys.withdraw_stake_spl_stake_pool,
        ),
        (
            accounts.withdraw_stake_validator_list.key,
            &keys.withdraw_stake_validator_list,
        ),
        (
            accounts.withdraw_stake_withdraw_authority.key,
            &keys.withdraw_stake_withdraw_authority,
        ),
        (
            accounts.withdraw_stake_stake_to_split.key,
            &keys.withdraw_stake_stake_to_split,
        ),
        (
            accounts.withdraw_stake_manager_fee.key,
            &keys.withdraw_stake_manager_fee,
        ),
        (accounts.clock.key, &keys.clock),
        (accounts.token_program.key, &keys.token_program),
        (accounts.stake_program.key, &keys.stake_program),
        (accounts.system_program.key, &keys.system_program),
    ] {
        if actual != expected {
            return Err((*actual, *expected));
        }
    }
    Ok(())
}
pub fn spl_stake_pool_withdraw_stake_verify_account_privileges<'me, 'info>(
    accounts: SplStakePoolWithdrawStakeAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.withdraw_stake_spl_stake_pool,
        accounts.withdraw_stake_validator_list,
        accounts.withdraw_stake_stake_to_split,
        accounts.withdraw_stake_manager_fee,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub const LIDO_WITHDRAW_STAKE_IX_ACCOUNTS_LEN: usize = 10;
#[derive(Copy, Clone, Debug)]
pub struct LidoWithdrawStakeAccounts<'me, 'info> {
    pub lido_program: &'me AccountInfo<'info>,
    pub withdraw_stake_solido: &'me AccountInfo<'info>,
    pub withdraw_stake_voter: &'me AccountInfo<'info>,
    pub withdraw_stake_stake_to_split: &'me AccountInfo<'info>,
    pub withdraw_stake_stake_authority: &'me AccountInfo<'info>,
    pub withdraw_stake_validator_list: &'me AccountInfo<'info>,
    pub clock: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
    pub stake_program: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
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
impl From<LidoWithdrawStakeAccounts<'_, '_>> for LidoWithdrawStakeKeys {
    fn from(accounts: LidoWithdrawStakeAccounts) -> Self {
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
impl From<LidoWithdrawStakeKeys> for [AccountMeta; LIDO_WITHDRAW_STAKE_IX_ACCOUNTS_LEN] {
    fn from(keys: LidoWithdrawStakeKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.lido_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.withdraw_stake_solido,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.withdraw_stake_voter,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.withdraw_stake_stake_to_split,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.withdraw_stake_stake_authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.withdraw_stake_validator_list,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.clock,
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
            AccountMeta {
                pubkey: keys.system_program,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; LIDO_WITHDRAW_STAKE_IX_ACCOUNTS_LEN]> for LidoWithdrawStakeKeys {
    fn from(pubkeys: [Pubkey; LIDO_WITHDRAW_STAKE_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            lido_program: pubkeys[0],
            withdraw_stake_solido: pubkeys[1],
            withdraw_stake_voter: pubkeys[2],
            withdraw_stake_stake_to_split: pubkeys[3],
            withdraw_stake_stake_authority: pubkeys[4],
            withdraw_stake_validator_list: pubkeys[5],
            clock: pubkeys[6],
            token_program: pubkeys[7],
            stake_program: pubkeys[8],
            system_program: pubkeys[9],
        }
    }
}
impl<'info> From<LidoWithdrawStakeAccounts<'_, 'info>>
    for [AccountInfo<'info>; LIDO_WITHDRAW_STAKE_IX_ACCOUNTS_LEN]
{
    fn from(accounts: LidoWithdrawStakeAccounts<'_, 'info>) -> Self {
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
impl<'me, 'info> From<&'me [AccountInfo<'info>; LIDO_WITHDRAW_STAKE_IX_ACCOUNTS_LEN]>
    for LidoWithdrawStakeAccounts<'me, 'info>
{
    fn from(arr: &'me [AccountInfo<'info>; LIDO_WITHDRAW_STAKE_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            lido_program: &arr[0],
            withdraw_stake_solido: &arr[1],
            withdraw_stake_voter: &arr[2],
            withdraw_stake_stake_to_split: &arr[3],
            withdraw_stake_stake_authority: &arr[4],
            withdraw_stake_validator_list: &arr[5],
            clock: &arr[6],
            token_program: &arr[7],
            stake_program: &arr[8],
            system_program: &arr[9],
        }
    }
}
pub const LIDO_WITHDRAW_STAKE_IX_DISCM: u8 = 2u8;
#[derive(Clone, Debug, PartialEq)]
pub struct LidoWithdrawStakeIxData;
impl LidoWithdrawStakeIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm_buf = [0u8; 1];
        reader.read_exact(&mut maybe_discm_buf)?;
        let maybe_discm = maybe_discm_buf[0];
        if maybe_discm != LIDO_WITHDRAW_STAKE_IX_DISCM {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    LIDO_WITHDRAW_STAKE_IX_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&[LIDO_WITHDRAW_STAKE_IX_DISCM])
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn lido_withdraw_stake_ix<K: Into<LidoWithdrawStakeKeys>>(
    accounts: K,
) -> std::io::Result<Instruction> {
    let keys: LidoWithdrawStakeKeys = accounts.into();
    let metas: [AccountMeta; LIDO_WITHDRAW_STAKE_IX_ACCOUNTS_LEN] = keys.into();
    Ok(Instruction {
        program_id: crate::ID,
        accounts: Vec::from(metas),
        data: LidoWithdrawStakeIxData.try_to_vec()?,
    })
}
pub fn lido_withdraw_stake_invoke<'info>(
    accounts: LidoWithdrawStakeAccounts<'_, 'info>,
) -> ProgramResult {
    let ix = lido_withdraw_stake_ix(accounts)?;
    let account_info: [AccountInfo<'info>; LIDO_WITHDRAW_STAKE_IX_ACCOUNTS_LEN] = accounts.into();
    invoke(&ix, &account_info)
}
pub fn lido_withdraw_stake_invoke_signed<'info>(
    accounts: LidoWithdrawStakeAccounts<'_, 'info>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let ix = lido_withdraw_stake_ix(accounts)?;
    let account_info: [AccountInfo<'info>; LIDO_WITHDRAW_STAKE_IX_ACCOUNTS_LEN] = accounts.into();
    invoke_signed(&ix, &account_info, seeds)
}
pub fn lido_withdraw_stake_verify_account_keys(
    accounts: LidoWithdrawStakeAccounts<'_, '_>,
    keys: LidoWithdrawStakeKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (accounts.lido_program.key, &keys.lido_program),
        (
            accounts.withdraw_stake_solido.key,
            &keys.withdraw_stake_solido,
        ),
        (
            accounts.withdraw_stake_voter.key,
            &keys.withdraw_stake_voter,
        ),
        (
            accounts.withdraw_stake_stake_to_split.key,
            &keys.withdraw_stake_stake_to_split,
        ),
        (
            accounts.withdraw_stake_stake_authority.key,
            &keys.withdraw_stake_stake_authority,
        ),
        (
            accounts.withdraw_stake_validator_list.key,
            &keys.withdraw_stake_validator_list,
        ),
        (accounts.clock.key, &keys.clock),
        (accounts.token_program.key, &keys.token_program),
        (accounts.stake_program.key, &keys.stake_program),
        (accounts.system_program.key, &keys.system_program),
    ] {
        if actual != expected {
            return Err((*actual, *expected));
        }
    }
    Ok(())
}
pub fn lido_withdraw_stake_verify_account_privileges<'me, 'info>(
    accounts: LidoWithdrawStakeAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.withdraw_stake_solido,
        accounts.withdraw_stake_stake_to_split,
        accounts.withdraw_stake_validator_list,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub const MARINADE_WITHDRAW_STAKE_IX_ACCOUNTS_LEN: usize = 12;
#[derive(Copy, Clone, Debug)]
pub struct MarinadeWithdrawStakeAccounts<'me, 'info> {
    pub marinade_program: &'me AccountInfo<'info>,
    pub withdraw_stake_marinade_state: &'me AccountInfo<'info>,
    pub withdraw_stake_marinade_treasury: &'me AccountInfo<'info>,
    pub withdraw_stake_validator_list: &'me AccountInfo<'info>,
    pub withdraw_stake_stake_to_split: &'me AccountInfo<'info>,
    pub withdraw_stake_stake_list: &'me AccountInfo<'info>,
    pub withdraw_stake_withdraw_authority: &'me AccountInfo<'info>,
    pub withdraw_stake_deposit_authority: &'me AccountInfo<'info>,
    pub clock: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
    pub stake_program: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug)]
pub struct MarinadeWithdrawStakeKeys {
    pub marinade_program: Pubkey,
    pub withdraw_stake_marinade_state: Pubkey,
    pub withdraw_stake_marinade_treasury: Pubkey,
    pub withdraw_stake_validator_list: Pubkey,
    pub withdraw_stake_stake_to_split: Pubkey,
    pub withdraw_stake_stake_list: Pubkey,
    pub withdraw_stake_withdraw_authority: Pubkey,
    pub withdraw_stake_deposit_authority: Pubkey,
    pub clock: Pubkey,
    pub token_program: Pubkey,
    pub stake_program: Pubkey,
    pub system_program: Pubkey,
}
impl From<MarinadeWithdrawStakeAccounts<'_, '_>> for MarinadeWithdrawStakeKeys {
    fn from(accounts: MarinadeWithdrawStakeAccounts) -> Self {
        Self {
            marinade_program: *accounts.marinade_program.key,
            withdraw_stake_marinade_state: *accounts.withdraw_stake_marinade_state.key,
            withdraw_stake_marinade_treasury: *accounts.withdraw_stake_marinade_treasury.key,
            withdraw_stake_validator_list: *accounts.withdraw_stake_validator_list.key,
            withdraw_stake_stake_to_split: *accounts.withdraw_stake_stake_to_split.key,
            withdraw_stake_stake_list: *accounts.withdraw_stake_stake_list.key,
            withdraw_stake_withdraw_authority: *accounts.withdraw_stake_withdraw_authority.key,
            withdraw_stake_deposit_authority: *accounts.withdraw_stake_deposit_authority.key,
            clock: *accounts.clock.key,
            token_program: *accounts.token_program.key,
            stake_program: *accounts.stake_program.key,
            system_program: *accounts.system_program.key,
        }
    }
}
impl From<MarinadeWithdrawStakeKeys> for [AccountMeta; MARINADE_WITHDRAW_STAKE_IX_ACCOUNTS_LEN] {
    fn from(keys: MarinadeWithdrawStakeKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.marinade_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.withdraw_stake_marinade_state,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.withdraw_stake_marinade_treasury,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.withdraw_stake_validator_list,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.withdraw_stake_stake_to_split,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.withdraw_stake_stake_list,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.withdraw_stake_withdraw_authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.withdraw_stake_deposit_authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.clock,
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
            AccountMeta {
                pubkey: keys.system_program,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; MARINADE_WITHDRAW_STAKE_IX_ACCOUNTS_LEN]> for MarinadeWithdrawStakeKeys {
    fn from(pubkeys: [Pubkey; MARINADE_WITHDRAW_STAKE_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            marinade_program: pubkeys[0],
            withdraw_stake_marinade_state: pubkeys[1],
            withdraw_stake_marinade_treasury: pubkeys[2],
            withdraw_stake_validator_list: pubkeys[3],
            withdraw_stake_stake_to_split: pubkeys[4],
            withdraw_stake_stake_list: pubkeys[5],
            withdraw_stake_withdraw_authority: pubkeys[6],
            withdraw_stake_deposit_authority: pubkeys[7],
            clock: pubkeys[8],
            token_program: pubkeys[9],
            stake_program: pubkeys[10],
            system_program: pubkeys[11],
        }
    }
}
impl<'info> From<MarinadeWithdrawStakeAccounts<'_, 'info>>
    for [AccountInfo<'info>; MARINADE_WITHDRAW_STAKE_IX_ACCOUNTS_LEN]
{
    fn from(accounts: MarinadeWithdrawStakeAccounts<'_, 'info>) -> Self {
        [
            accounts.marinade_program.clone(),
            accounts.withdraw_stake_marinade_state.clone(),
            accounts.withdraw_stake_marinade_treasury.clone(),
            accounts.withdraw_stake_validator_list.clone(),
            accounts.withdraw_stake_stake_to_split.clone(),
            accounts.withdraw_stake_stake_list.clone(),
            accounts.withdraw_stake_withdraw_authority.clone(),
            accounts.withdraw_stake_deposit_authority.clone(),
            accounts.clock.clone(),
            accounts.token_program.clone(),
            accounts.stake_program.clone(),
            accounts.system_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; MARINADE_WITHDRAW_STAKE_IX_ACCOUNTS_LEN]>
    for MarinadeWithdrawStakeAccounts<'me, 'info>
{
    fn from(arr: &'me [AccountInfo<'info>; MARINADE_WITHDRAW_STAKE_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            marinade_program: &arr[0],
            withdraw_stake_marinade_state: &arr[1],
            withdraw_stake_marinade_treasury: &arr[2],
            withdraw_stake_validator_list: &arr[3],
            withdraw_stake_stake_to_split: &arr[4],
            withdraw_stake_stake_list: &arr[5],
            withdraw_stake_withdraw_authority: &arr[6],
            withdraw_stake_deposit_authority: &arr[7],
            clock: &arr[8],
            token_program: &arr[9],
            stake_program: &arr[10],
            system_program: &arr[11],
        }
    }
}
pub const MARINADE_WITHDRAW_STAKE_IX_DISCM: u8 = 3u8;
#[derive(Clone, Debug, PartialEq)]
pub struct MarinadeWithdrawStakeIxData;
impl MarinadeWithdrawStakeIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm_buf = [0u8; 1];
        reader.read_exact(&mut maybe_discm_buf)?;
        let maybe_discm = maybe_discm_buf[0];
        if maybe_discm != MARINADE_WITHDRAW_STAKE_IX_DISCM {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    MARINADE_WITHDRAW_STAKE_IX_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&[MARINADE_WITHDRAW_STAKE_IX_DISCM])
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn marinade_withdraw_stake_ix<K: Into<MarinadeWithdrawStakeKeys>>(
    accounts: K,
) -> std::io::Result<Instruction> {
    let keys: MarinadeWithdrawStakeKeys = accounts.into();
    let metas: [AccountMeta; MARINADE_WITHDRAW_STAKE_IX_ACCOUNTS_LEN] = keys.into();
    Ok(Instruction {
        program_id: crate::ID,
        accounts: Vec::from(metas),
        data: MarinadeWithdrawStakeIxData.try_to_vec()?,
    })
}
pub fn marinade_withdraw_stake_invoke<'info>(
    accounts: MarinadeWithdrawStakeAccounts<'_, 'info>,
) -> ProgramResult {
    let ix = marinade_withdraw_stake_ix(accounts)?;
    let account_info: [AccountInfo<'info>; MARINADE_WITHDRAW_STAKE_IX_ACCOUNTS_LEN] =
        accounts.into();
    invoke(&ix, &account_info)
}
pub fn marinade_withdraw_stake_invoke_signed<'info>(
    accounts: MarinadeWithdrawStakeAccounts<'_, 'info>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let ix = marinade_withdraw_stake_ix(accounts)?;
    let account_info: [AccountInfo<'info>; MARINADE_WITHDRAW_STAKE_IX_ACCOUNTS_LEN] =
        accounts.into();
    invoke_signed(&ix, &account_info, seeds)
}
pub fn marinade_withdraw_stake_verify_account_keys(
    accounts: MarinadeWithdrawStakeAccounts<'_, '_>,
    keys: MarinadeWithdrawStakeKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (accounts.marinade_program.key, &keys.marinade_program),
        (
            accounts.withdraw_stake_marinade_state.key,
            &keys.withdraw_stake_marinade_state,
        ),
        (
            accounts.withdraw_stake_marinade_treasury.key,
            &keys.withdraw_stake_marinade_treasury,
        ),
        (
            accounts.withdraw_stake_validator_list.key,
            &keys.withdraw_stake_validator_list,
        ),
        (
            accounts.withdraw_stake_stake_to_split.key,
            &keys.withdraw_stake_stake_to_split,
        ),
        (
            accounts.withdraw_stake_stake_list.key,
            &keys.withdraw_stake_stake_list,
        ),
        (
            accounts.withdraw_stake_withdraw_authority.key,
            &keys.withdraw_stake_withdraw_authority,
        ),
        (
            accounts.withdraw_stake_deposit_authority.key,
            &keys.withdraw_stake_deposit_authority,
        ),
        (accounts.clock.key, &keys.clock),
        (accounts.token_program.key, &keys.token_program),
        (accounts.stake_program.key, &keys.stake_program),
        (accounts.system_program.key, &keys.system_program),
    ] {
        if actual != expected {
            return Err((*actual, *expected));
        }
    }
    Ok(())
}
pub fn marinade_withdraw_stake_verify_account_privileges<'me, 'info>(
    accounts: MarinadeWithdrawStakeAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.withdraw_stake_marinade_state,
        accounts.withdraw_stake_marinade_treasury,
        accounts.withdraw_stake_validator_list,
        accounts.withdraw_stake_stake_to_split,
        accounts.withdraw_stake_stake_list,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
