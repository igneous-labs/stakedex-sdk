use jupiter_core::amm::{Amm, KeyedAccount};
use stakedex_eversol_stake_pool::EversolStakePoolStakedex;
use stakedex_interface::DepositSolType;
use stakedex_lido::LidoStakedex;
use stakedex_marinade::MarinadeStakedex;
use stakedex_sdk_common::{DepositSol, DepositSolWrapper, InitFromKeyedAccount};
use stakedex_socean_stake_pool::SoceanStakePoolStakedex;
use stakedex_spl_stake_pool::SplStakePoolStakedex;

pub fn load_deposit_sol(
    ty: &DepositSolType,
    main_account: &KeyedAccount,
) -> Result<Box<dyn Amm>, anyhow::Error> {
    match ty {
        DepositSolType::Eversol => load_pool::<EversolStakePoolStakedex>(main_account),
        DepositSolType::Lido => load_pool::<LidoStakedex>(main_account),
        DepositSolType::Marinade => load_pool::<MarinadeStakedex>(main_account),
        DepositSolType::Socean => load_pool::<SoceanStakePoolStakedex>(main_account),
        DepositSolType::Spl => load_pool::<SplStakePoolStakedex>(main_account),
    }
}

fn load_pool<P: InitFromKeyedAccount + DepositSol + Clone + Send + Sync + 'static>(
    main_account: &KeyedAccount,
) -> Result<Box<dyn Amm>, anyhow::Error> {
    Ok(Box::new(DepositSolWrapper(P::from_keyed_account(
        main_account,
    )?)))
}
