use lazy_static::lazy_static;
use solana_program::pubkey::Pubkey;
use stakedex_sdk_common::{
    cogent_stake_pool, daopool_stake_pool, jito_stake_pool, jpool_stake_pool, laine_stake_pool,
    mrgn_stake_pool, risklol_stake_pool, solblaze_stake_pool,
};
use std::collections::HashMap;

mod stakedex_traits;
pub use stakedex_traits::*;

lazy_static! {
    pub static ref SPL_STAKE_POOL_STATE_TO_LABEL: HashMap<Pubkey, &'static str> = {
        let mut m = HashMap::new();
        m.insert(cogent_stake_pool::ID, "Cogent");
        m.insert(daopool_stake_pool::ID, "DaoPool");
        m.insert(jito_stake_pool::ID, "Jito");
        m.insert(jpool_stake_pool::ID, "JPool");
        m.insert(laine_stake_pool::ID, "Laine");
        m.insert(risklol_stake_pool::ID, "Risk.lol");
        m.insert(solblaze_stake_pool::ID, "SolBlaze");
        m.insert(mrgn_stake_pool::ID, "mrgn");
        m
    };
}

#[cfg(test)]
mod tests {
    use crate::*;
    use stakedex_sdk_common::DepositSolWrapper;

    #[test]
    fn test_wrapper_impls_amm_correctly_compile_time() {
        // DepositSolWrapper<SplStakePoolDepositSol>
        // impls Amm
        let _sp = DepositSolWrapper(SplStakePoolStakedex::default());
    }
}
