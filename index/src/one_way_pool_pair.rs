use jupiter_core::amm::{Amm, KeyedAccount};
use solana_program::clock::Clock;
use stakedex_eversol_stake_pool::EversolStakePoolStakedex;
use stakedex_interface::{DepositStakeType, WithdrawStakeType};
use stakedex_lido::LidoStakedex;
use stakedex_marinade::MarinadeStakedex;
use stakedex_sdk_common::{DepositStake, InitFromKeyedAccount, OneWayPoolPair, WithdrawStake};
use stakedex_socean_stake_pool::SoceanStakePoolStakedex;
use stakedex_spl_stake_pool::SplStakePoolStakedex;
use stakedex_unstake_it::UnstakeItStakedex;

macro_rules! match_deposit {
    ($w: ty, $withdraw_stake_main_account: expr, $deposit_stake_ty: expr, $deposit_stake_main_account: expr, $clock: expr) => {
        match $deposit_stake_ty {
            DepositStakeType::Eversol => load_pool_pair::<$w, EversolStakePoolStakedex>(
                $withdraw_stake_main_account,
                $deposit_stake_main_account,
                $clock,
            ),
            DepositStakeType::Marinade => load_pool_pair::<$w, MarinadeStakedex>(
                $withdraw_stake_main_account,
                $deposit_stake_main_account,
                $clock,
            ),
            DepositStakeType::Socean => load_pool_pair::<$w, SoceanStakePoolStakedex>(
                $withdraw_stake_main_account,
                $deposit_stake_main_account,
                $clock,
            ),
            DepositStakeType::Spl => load_pool_pair::<$w, SplStakePoolStakedex>(
                $withdraw_stake_main_account,
                $deposit_stake_main_account,
                $clock,
            ),
            DepositStakeType::Unstakeit => load_pool_pair::<$w, UnstakeItStakedex>(
                $withdraw_stake_main_account,
                $deposit_stake_main_account,
                $clock,
            ),
        }
    };
}

pub fn load_one_way_pool_pair(
    withdraw_stake_ty: &WithdrawStakeType,
    withdraw_stake_main_account: &KeyedAccount,
    deposit_stake_ty: &DepositStakeType,
    deposit_stake_main_account: &KeyedAccount,
    clock: Clock,
) -> Result<Box<dyn Amm>, anyhow::Error> {
    match withdraw_stake_ty {
        WithdrawStakeType::Eversol => match_deposit!(
            EversolStakePoolStakedex,
            withdraw_stake_main_account,
            deposit_stake_ty,
            deposit_stake_main_account,
            clock
        ),
        WithdrawStakeType::Lido => match_deposit!(
            LidoStakedex,
            withdraw_stake_main_account,
            deposit_stake_ty,
            deposit_stake_main_account,
            clock
        ),
        WithdrawStakeType::Socean => match_deposit!(
            SoceanStakePoolStakedex,
            withdraw_stake_main_account,
            deposit_stake_ty,
            deposit_stake_main_account,
            clock
        ),
        WithdrawStakeType::Spl => match_deposit!(
            SplStakePoolStakedex,
            withdraw_stake_main_account,
            deposit_stake_ty,
            deposit_stake_main_account,
            clock
        ),
    }
}

fn load_pool_pair<
    W: InitFromKeyedAccount + WithdrawStake + Clone + Send + Sync + 'static,
    D: InitFromKeyedAccount + DepositStake + Clone + Send + Sync + 'static,
>(
    withdraw_stake_main_account: &KeyedAccount,
    deposit_stake_main_account: &KeyedAccount,
    clock: Clock,
) -> Result<Box<dyn Amm>, anyhow::Error> {
    Ok(Box::new(OneWayPoolPair {
        withdraw: W::from_keyed_account(withdraw_stake_main_account)?,
        deposit: D::from_keyed_account(deposit_stake_main_account)?,
        clock,
    }))
}
