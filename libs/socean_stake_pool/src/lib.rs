mod stakedex_traits;
pub use stakedex_traits::*;

pub const SOCEAN_STAKE_POOL_LABEL: &str = "Socean";

#[cfg(test)]
mod tests {
    use crate::*;
    use stakedex_sdk_common::DepositSolWrapper;

    #[test]
    fn test_wrapper_impls_amm_correctly_compile_time() {
        // DepositSolWrapper<SplStakePoolDepositSol>
        // impls Amm
        let _sp = DepositSolWrapper(SoceanStakePoolStakedex::default());
    }
}
