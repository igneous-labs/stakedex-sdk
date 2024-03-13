use anyhow::{anyhow, Result};
use jupiter_amm_interface::{
    AccountMap, Amm, KeyedAccount, Quote, QuoteParams, Swap, SwapAndAccountMetas, SwapParams,
};
use solana_sdk::{clock::Clock, pubkey::Pubkey, sysvar};
use stakedex_interface::PREFUND_SWAP_VIA_STAKE_IX_ACCOUNTS_LEN;
use stakedex_sdk_common::{
    account_missing_err, find_stake_pool_pair_amm_key, unstake_it_program, DepositStake,
    WithdrawStake, TEMPORARY_JUP_AMM_LABEL,
};
use std::collections::HashSet;

use crate::{
    get_account_metas, jupiter_stakedex_interface::STAKEDEX_ACCOUNT_META,
    prepare_underlying_liquidities, quote_pool_pair, PrefundRepayParams,
};

#[derive(Clone)]
pub struct TwoWayPoolPair<
    P1: DepositStake + WithdrawStake + Clone + Send + Sync + 'static,
    P2: DepositStake + WithdrawStake + Clone + Send + Sync + 'static,
> {
    pub p1: P1,
    pub p2: P2,
    clock: Clock,
    prefund_repay_params: Option<PrefundRepayParams>,
    underlying_liquidities: Option<HashSet<Pubkey>>,
}

impl<P1, P2> TwoWayPoolPair<P1, P2>
where
    P1: DepositStake + WithdrawStake + Clone + Send + Sync,
    P2: DepositStake + WithdrawStake + Clone + Send + Sync,
{
    pub fn new(p1: P1, p2: P2) -> Self {
        let underlying_liquidities = prepare_underlying_liquidities(&[
            DepositStake::underlying_liquidity(&p1),
            <dyn WithdrawStake>::underlying_liquidity(&p1),
            DepositStake::underlying_liquidity(&p2),
            <dyn WithdrawStake>::underlying_liquidity(&p2),
        ]);
        Self {
            p1,
            p2,
            clock: Clock::default(),
            prefund_repay_params: None,
            underlying_liquidities,
        }
    }

    pub fn prefund_repay_params_checked(&self) -> Result<&PrefundRepayParams> {
        self.prefund_repay_params
            .as_ref()
            .ok_or_else(|| anyhow!("prefund_repay_params not initialized"))
    }
}

impl<P1, P2> Amm for TwoWayPoolPair<P1, P2>
where
    P1: DepositStake + WithdrawStake + Clone + Send + Sync,
    P2: DepositStake + WithdrawStake + Clone + Send + Sync,
{
    fn from_keyed_account(_keyed_account: &KeyedAccount) -> Result<Self> {
        panic!(); // TODO: Assess this code smell
    }

    fn label(&self) -> String {
        TEMPORARY_JUP_AMM_LABEL.to_owned()
    }

    fn key(&self) -> Pubkey {
        find_stake_pool_pair_amm_key(&self.p1.main_state_key(), &self.p2.main_state_key()).0
    }

    fn get_reserve_mints(&self) -> Vec<Pubkey> {
        Vec::from([self.p1.staked_sol_mint(), self.p2.staked_sol_mint()])
    }

    fn get_accounts_to_update(&self) -> Vec<Pubkey> {
        [
            self.p1.get_accounts_to_update().as_slice(),
            self.p2.get_accounts_to_update().as_slice(),
            &[sysvar::clock::ID],
            PrefundRepayParams::ACCOUNTS_TO_UPDATE.as_slice(),
        ]
        .concat()
    }

    fn update(&mut self, account_map: &AccountMap) -> Result<()> {
        // TODO: not sure if should short-circuit and early return if first update() fails
        let r1 = self.p1.update(account_map);
        let r2 = self.p2.update(account_map);
        let rc = account_map
            .get(&sysvar::clock::ID)
            .ok_or_else(|| account_missing_err(&sysvar::clock::ID))
            .map_or_else(Err, |acc| Ok(bincode::deserialize(&acc.data)?))
            .map(|new_clock| self.clock = new_clock);
        let rp = match self.prefund_repay_params.as_mut() {
            None => {
                let init_res = PrefundRepayParams::try_init(account_map);
                init_res.map(|p| {
                    self.prefund_repay_params = Some(p);
                })
            }
            Some(p) => p.update(account_map),
        };
        r1.and(r2).and(rc).and(rp)
    }

    fn quote(&self, quote_params: &QuoteParams) -> Result<Quote> {
        if quote_params.input_mint == self.p1.staked_sol_mint()
            && quote_params.output_mint == self.p2.staked_sol_mint()
        {
            quote_pool_pair(
                quote_params,
                self.prefund_repay_params_checked()?,
                &self.p1,
                &self.p2,
            )
        } else if quote_params.input_mint == self.p2.staked_sol_mint()
            && quote_params.output_mint == self.p1.staked_sol_mint()
        {
            quote_pool_pair(
                quote_params,
                self.prefund_repay_params_checked()?,
                &self.p2,
                &self.p1,
            )
        } else {
            Err(anyhow!(
                "Cannot handle {} -> {}",
                quote_params.input_mint,
                quote_params.output_mint
            ))
        }
    }

    fn get_swap_and_account_metas(&self, swap_params: &SwapParams) -> Result<SwapAndAccountMetas> {
        let bridge_stake_seed = rand::random();
        let mut account_metas = vec![STAKEDEX_ACCOUNT_META.clone()];
        let other_account_metas = if swap_params.source_mint == self.p1.staked_sol_mint()
            && swap_params.destination_mint == self.p2.staked_sol_mint()
        {
            get_account_metas(
                swap_params,
                self.prefund_repay_params_checked()?,
                &self.p1,
                &self.p2,
                bridge_stake_seed,
            )?
        } else if swap_params.source_mint == self.p2.staked_sol_mint()
            && swap_params.destination_mint == self.p1.staked_sol_mint()
        {
            get_account_metas(
                swap_params,
                self.prefund_repay_params_checked()?,
                &self.p2,
                &self.p1,
                bridge_stake_seed,
            )?
        } else {
            return Err(anyhow!(
                "Cannot handle {} -> {}",
                swap_params.source_mint,
                swap_params.destination_mint
            ));
        };
        account_metas.extend(other_account_metas);
        account_metas.push(swap_params.placeholder_account_meta());
        Ok(SwapAndAccountMetas {
            swap: Swap::StakeDexSwapViaStake { bridge_stake_seed },
            account_metas,
        })
    }

    fn clone_amm(&self) -> Box<dyn Amm + Send + Sync> {
        Box::new(self.clone())
    }

    fn program_id(&self) -> Pubkey {
        stakedex_interface::ID
    }

    fn get_accounts_len(&self) -> usize {
        // Pick a single direction
        1 + PREFUND_SWAP_VIA_STAKE_IX_ACCOUNTS_LEN
            + <dyn WithdrawStake>::accounts_len(&self.p1)
            + DepositStake::accounts_len(&self.p2)
            + 1
    }

    fn underlying_liquidities(&self) -> Option<HashSet<Pubkey>> {
        self.underlying_liquidities.clone()
    }

    fn program_dependencies(&self) -> Vec<(Pubkey, String)> {
        vec![
            (
                self.p1.program_id(),
                self.p1.stake_pool_label().to_lowercase(),
            ),
            (
                self.p2.program_id(),
                self.p2.stake_pool_label().to_lowercase(),
            ),
            (unstake_it_program::ID, "unstake.it".to_owned()),
        ]
    }
}
