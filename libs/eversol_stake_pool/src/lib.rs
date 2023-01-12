mod stakedex_traits;
pub use stakedex_traits::*;

pub const EVERSOL_STAKE_POOL_LABEL: &str = "Eversol";

#[cfg(test)]
mod tests {
    use crate::*;
    use jupiter_core::amm::Amm;
    use stakedex_sdk_common::DepositSolWrapper;

    #[test]
    fn test_wrapper_impls_amm_correctly_compile_time() {
        // DepositSolWrapper<SplStakePoolDepositSol>
        // impls Amm
        let sp = DepositSolWrapper(EversolStakePoolStakedex::default());
        sp.clone_amm();
    }
}
