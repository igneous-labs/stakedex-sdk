use lazy_static::lazy_static;
use solana_program::pubkey::Pubkey;
use stakedex_sdk_common::{
    daopool_stake_pool, jito_stake_pool, jpool_stake_pool, laine_stake_pool, solblaze_stake_pool,
};
use std::collections::HashMap;

mod stakedex_traits;
pub use stakedex_traits::*;

lazy_static! {
    pub static ref SPL_STAKE_POOL_STATE_TO_LABEL: HashMap<Pubkey, &'static str> = {
        let mut m = HashMap::new();
        m.insert(daopool_stake_pool::ID, "DaoPool");
        m.insert(jito_stake_pool::ID, "Jito");
        m.insert(jpool_stake_pool::ID, "JPool");
        m.insert(laine_stake_pool::ID, "Laine");
        m.insert(solblaze_stake_pool::ID, "SolBlaze");
        m
    };
}

#[cfg(test)]
mod tests {
    use crate::*;
    use jupiter_core::amm::Amm;
    use stakedex_sdk_common::DepositSolWrapper;

    #[test]
    fn test_wrapper_impls_amm_correctly_compile_time() {
        // DepositSolWrapper<SplStakePoolDepositSol>
        // impls Amm
        let sp = DepositSolWrapper(SplStakePoolStakedex::default());
        sp.clone_amm();
    }
}
