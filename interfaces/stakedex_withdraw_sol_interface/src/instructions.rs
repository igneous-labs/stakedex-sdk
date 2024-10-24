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
pub enum StakedexWithdrawSolProgramIx {
    SplStakePoolWithdrawSol,
}
impl StakedexWithdrawSolProgramIx {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm_buf = [0u8; 1];
        reader.read_exact(&mut maybe_discm_buf)?;
        let maybe_discm = maybe_discm_buf[0];
        match maybe_discm {
            SPL_STAKE_POOL_WITHDRAW_SOL_IX_DISCM => Ok(Self::SplStakePoolWithdrawSol),
            _ => Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!("discm {:?} not found", maybe_discm),
            )),
        }
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        match self {
            Self::SplStakePoolWithdrawSol => {
                writer.write_all(&[SPL_STAKE_POOL_WITHDRAW_SOL_IX_DISCM])
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
pub const SPL_STAKE_POOL_WITHDRAW_SOL_IX_ACCOUNTS_LEN: usize = 9;
#[derive(Copy, Clone, Debug)]
pub struct SplStakePoolWithdrawSolAccounts<'me, 'info> {
    pub spl_stake_pool_program: &'me AccountInfo<'info>,
    pub withdraw_sol_spl_stake_pool: &'me AccountInfo<'info>,
    pub withdraw_sol_withdraw_authority: &'me AccountInfo<'info>,
    pub withdraw_sol_reserve_stake: &'me AccountInfo<'info>,
    pub withdraw_sol_manager_fee: &'me AccountInfo<'info>,
    pub clock: &'me AccountInfo<'info>,
    pub stake_history: &'me AccountInfo<'info>,
    pub stake_program: &'me AccountInfo<'info>,
    ///possible duplicate to account for token-22 stake pools
    pub withdraw_sol_token_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug)]
pub struct SplStakePoolWithdrawSolKeys {
    pub spl_stake_pool_program: Pubkey,
    pub withdraw_sol_spl_stake_pool: Pubkey,
    pub withdraw_sol_withdraw_authority: Pubkey,
    pub withdraw_sol_reserve_stake: Pubkey,
    pub withdraw_sol_manager_fee: Pubkey,
    pub clock: Pubkey,
    pub stake_history: Pubkey,
    pub stake_program: Pubkey,
    ///possible duplicate to account for token-22 stake pools
    pub withdraw_sol_token_program: Pubkey,
}
impl From<SplStakePoolWithdrawSolAccounts<'_, '_>> for SplStakePoolWithdrawSolKeys {
    fn from(accounts: SplStakePoolWithdrawSolAccounts) -> Self {
        Self {
            spl_stake_pool_program: *accounts.spl_stake_pool_program.key,
            withdraw_sol_spl_stake_pool: *accounts.withdraw_sol_spl_stake_pool.key,
            withdraw_sol_withdraw_authority: *accounts.withdraw_sol_withdraw_authority.key,
            withdraw_sol_reserve_stake: *accounts.withdraw_sol_reserve_stake.key,
            withdraw_sol_manager_fee: *accounts.withdraw_sol_manager_fee.key,
            clock: *accounts.clock.key,
            stake_history: *accounts.stake_history.key,
            stake_program: *accounts.stake_program.key,
            withdraw_sol_token_program: *accounts.withdraw_sol_token_program.key,
        }
    }
}
impl From<SplStakePoolWithdrawSolKeys>
    for [AccountMeta; SPL_STAKE_POOL_WITHDRAW_SOL_IX_ACCOUNTS_LEN]
{
    fn from(keys: SplStakePoolWithdrawSolKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.spl_stake_pool_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.withdraw_sol_spl_stake_pool,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.withdraw_sol_withdraw_authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.withdraw_sol_reserve_stake,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.withdraw_sol_manager_fee,
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
                pubkey: keys.stake_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.withdraw_sol_token_program,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; SPL_STAKE_POOL_WITHDRAW_SOL_IX_ACCOUNTS_LEN]> for SplStakePoolWithdrawSolKeys {
    fn from(pubkeys: [Pubkey; SPL_STAKE_POOL_WITHDRAW_SOL_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            spl_stake_pool_program: pubkeys[0],
            withdraw_sol_spl_stake_pool: pubkeys[1],
            withdraw_sol_withdraw_authority: pubkeys[2],
            withdraw_sol_reserve_stake: pubkeys[3],
            withdraw_sol_manager_fee: pubkeys[4],
            clock: pubkeys[5],
            stake_history: pubkeys[6],
            stake_program: pubkeys[7],
            withdraw_sol_token_program: pubkeys[8],
        }
    }
}
impl<'info> From<SplStakePoolWithdrawSolAccounts<'_, 'info>>
    for [AccountInfo<'info>; SPL_STAKE_POOL_WITHDRAW_SOL_IX_ACCOUNTS_LEN]
{
    fn from(accounts: SplStakePoolWithdrawSolAccounts<'_, 'info>) -> Self {
        [
            accounts.spl_stake_pool_program.clone(),
            accounts.withdraw_sol_spl_stake_pool.clone(),
            accounts.withdraw_sol_withdraw_authority.clone(),
            accounts.withdraw_sol_reserve_stake.clone(),
            accounts.withdraw_sol_manager_fee.clone(),
            accounts.clock.clone(),
            accounts.stake_history.clone(),
            accounts.stake_program.clone(),
            accounts.withdraw_sol_token_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; SPL_STAKE_POOL_WITHDRAW_SOL_IX_ACCOUNTS_LEN]>
    for SplStakePoolWithdrawSolAccounts<'me, 'info>
{
    fn from(arr: &'me [AccountInfo<'info>; SPL_STAKE_POOL_WITHDRAW_SOL_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            spl_stake_pool_program: &arr[0],
            withdraw_sol_spl_stake_pool: &arr[1],
            withdraw_sol_withdraw_authority: &arr[2],
            withdraw_sol_reserve_stake: &arr[3],
            withdraw_sol_manager_fee: &arr[4],
            clock: &arr[5],
            stake_history: &arr[6],
            stake_program: &arr[7],
            withdraw_sol_token_program: &arr[8],
        }
    }
}
pub const SPL_STAKE_POOL_WITHDRAW_SOL_IX_DISCM: u8 = 1u8;
#[derive(Clone, Debug, PartialEq)]
pub struct SplStakePoolWithdrawSolIxData;
impl SplStakePoolWithdrawSolIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm_buf = [0u8; 1];
        reader.read_exact(&mut maybe_discm_buf)?;
        let maybe_discm = maybe_discm_buf[0];
        if maybe_discm != SPL_STAKE_POOL_WITHDRAW_SOL_IX_DISCM {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    SPL_STAKE_POOL_WITHDRAW_SOL_IX_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&[SPL_STAKE_POOL_WITHDRAW_SOL_IX_DISCM])
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn spl_stake_pool_withdraw_sol_ix_with_program_id(
    program_id: Pubkey,
    keys: SplStakePoolWithdrawSolKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; SPL_STAKE_POOL_WITHDRAW_SOL_IX_ACCOUNTS_LEN] = keys.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: SplStakePoolWithdrawSolIxData.try_to_vec()?,
    })
}
pub fn spl_stake_pool_withdraw_sol_ix(
    keys: SplStakePoolWithdrawSolKeys,
) -> std::io::Result<Instruction> {
    spl_stake_pool_withdraw_sol_ix_with_program_id(crate::ID, keys)
}
pub fn spl_stake_pool_withdraw_sol_invoke_with_program_id(
    program_id: Pubkey,
    accounts: SplStakePoolWithdrawSolAccounts<'_, '_>,
) -> ProgramResult {
    let keys: SplStakePoolWithdrawSolKeys = accounts.into();
    let ix = spl_stake_pool_withdraw_sol_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn spl_stake_pool_withdraw_sol_invoke(
    accounts: SplStakePoolWithdrawSolAccounts<'_, '_>,
) -> ProgramResult {
    spl_stake_pool_withdraw_sol_invoke_with_program_id(crate::ID, accounts)
}
pub fn spl_stake_pool_withdraw_sol_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: SplStakePoolWithdrawSolAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: SplStakePoolWithdrawSolKeys = accounts.into();
    let ix = spl_stake_pool_withdraw_sol_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn spl_stake_pool_withdraw_sol_invoke_signed(
    accounts: SplStakePoolWithdrawSolAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    spl_stake_pool_withdraw_sol_invoke_signed_with_program_id(crate::ID, accounts, seeds)
}
pub fn spl_stake_pool_withdraw_sol_verify_account_keys(
    accounts: SplStakePoolWithdrawSolAccounts<'_, '_>,
    keys: SplStakePoolWithdrawSolKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (
            accounts.spl_stake_pool_program.key,
            &keys.spl_stake_pool_program,
        ),
        (
            accounts.withdraw_sol_spl_stake_pool.key,
            &keys.withdraw_sol_spl_stake_pool,
        ),
        (
            accounts.withdraw_sol_withdraw_authority.key,
            &keys.withdraw_sol_withdraw_authority,
        ),
        (
            accounts.withdraw_sol_reserve_stake.key,
            &keys.withdraw_sol_reserve_stake,
        ),
        (
            accounts.withdraw_sol_manager_fee.key,
            &keys.withdraw_sol_manager_fee,
        ),
        (accounts.clock.key, &keys.clock),
        (accounts.stake_history.key, &keys.stake_history),
        (accounts.stake_program.key, &keys.stake_program),
        (
            accounts.withdraw_sol_token_program.key,
            &keys.withdraw_sol_token_program,
        ),
    ] {
        if actual != expected {
            return Err((*actual, *expected));
        }
    }
    Ok(())
}
pub fn spl_stake_pool_withdraw_sol_verify_writable_privileges<'me, 'info>(
    accounts: SplStakePoolWithdrawSolAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.withdraw_sol_spl_stake_pool,
        accounts.withdraw_sol_reserve_stake,
        accounts.withdraw_sol_manager_fee,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn spl_stake_pool_withdraw_sol_verify_account_privileges<'me, 'info>(
    accounts: SplStakePoolWithdrawSolAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    spl_stake_pool_withdraw_sol_verify_writable_privileges(accounts)?;
    Ok(())
}
