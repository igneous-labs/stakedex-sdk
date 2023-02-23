use jupiter_core::amm::{Amm, KeyedAccount};
use solana_program::clock::Clock;
use stakedex_eversol_stake_pool::EversolStakePoolStakedex;
use stakedex_interface::DepositWithdrawStakeType;
use stakedex_sdk_common::{DepositStake, InitFromKeyedAccount, TwoWayPoolPair, WithdrawStake};
use stakedex_socean_stake_pool::SoceanStakePoolStakedex;
use stakedex_spl_stake_pool::SplStakePoolStakedex;

macro_rules! match_b {
    ($a: ty, $a_main_account: expr, $b_ty: expr, $b_main_account: expr, $clock: expr) => {
        match $b_ty {
            DepositWithdrawStakeType::Eversol => load_pool_pair::<$a, EversolStakePoolStakedex>(
                $a_main_account,
                $b_main_account,
                $clock,
            ),
            DepositWithdrawStakeType::Socean => load_pool_pair::<$a, SoceanStakePoolStakedex>(
                $a_main_account,
                $b_main_account,
                $clock,
            ),
            DepositWithdrawStakeType::Spl => {
                load_pool_pair::<$a, SplStakePoolStakedex>($a_main_account, $b_main_account, $clock)
            }
        }
    };
}

pub fn load_two_way_pool_pair(
    a_ty: &DepositWithdrawStakeType,
    a_main_account: &KeyedAccount,
    b_ty: &DepositWithdrawStakeType,
    b_main_account: &KeyedAccount,
    clock: Clock,
) -> Result<Box<dyn Amm>, anyhow::Error> {
    match a_ty {
        DepositWithdrawStakeType::Eversol => match_b!(
            EversolStakePoolStakedex,
            a_main_account,
            b_ty,
            b_main_account,
            clock
        ),
        DepositWithdrawStakeType::Socean => match_b!(
            SoceanStakePoolStakedex,
            a_main_account,
            b_ty,
            b_main_account,
            clock
        ),
        DepositWithdrawStakeType::Spl => match_b!(
            SplStakePoolStakedex,
            a_main_account,
            b_ty,
            b_main_account,
            clock
        ),
    }
}

fn load_pool_pair<
    P1: InitFromKeyedAccount + DepositStake + WithdrawStake + Clone + Send + Sync + 'static,
    P2: InitFromKeyedAccount + DepositStake + WithdrawStake + Clone + Send + Sync + 'static,
>(
    main_account_withdraw: &KeyedAccount,
    main_account_deposit: &KeyedAccount,
    clock: Clock,
) -> Result<Box<dyn Amm>, anyhow::Error> {
    Ok(Box::new(TwoWayPoolPair {
        p1: P1::from_keyed_account(main_account_withdraw)?,
        p2: P2::from_keyed_account(main_account_deposit)?,
        clock,
    }))
}
