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
pub struct OneWayPoolPair<
    W: WithdrawStake + Clone + Send + Sync + 'static,
    D: DepositStake + Clone + Send + Sync + 'static,
> {
    pub withdraw: W,
    pub deposit: D,
    clock: Clock,
    prefund_repay_params: Option<PrefundRepayParams>,
    underlying_liquidities: Option<HashSet<Pubkey>>,
}

impl<W, D> OneWayPoolPair<W, D>
where
    W: WithdrawStake + Clone + Send + Sync,
    D: DepositStake + Clone + Send + Sync,
{
    pub fn new(withdraw: W, deposit: D) -> Self {
        let underlying_liquidities = prepare_underlying_liquidities(&[
            withdraw.underlying_liquidity(),
            deposit.underlying_liquidity(),
        ]);
        Self {
            withdraw,
            deposit,
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

impl<W, D> Amm for OneWayPoolPair<W, D>
where
    W: WithdrawStake + Clone + Send + Sync,
    D: DepositStake + Clone + Send + Sync,
{
    fn from_keyed_account(_keyed_account: &KeyedAccount) -> Result<Self> {
        todo!() // TODO: Assess this code smell
    }

    fn label(&self) -> String {
        TEMPORARY_JUP_AMM_LABEL.to_owned()
    }

    fn key(&self) -> Pubkey {
        find_stake_pool_pair_amm_key(
            &self.withdraw.main_state_key(),
            &self.deposit.main_state_key(),
        )
        .0
    }

    fn get_reserve_mints(&self) -> Vec<Pubkey> {
        Vec::from([
            self.withdraw.staked_sol_mint(),
            self.deposit.staked_sol_mint(),
        ])
    }

    fn get_accounts_to_update(&self) -> Vec<Pubkey> {
        [
            self.withdraw.get_accounts_to_update().as_slice(),
            self.deposit.get_accounts_to_update().as_slice(),
            &[sysvar::clock::ID],
            PrefundRepayParams::ACCOUNTS_TO_UPDATE.as_slice(),
        ]
        .concat()
    }

    fn update(&mut self, account_map: &AccountMap) -> Result<()> {
        // TODO: not sure if should short-circuit and early return if first update() fails
        let rw = self.withdraw.update(account_map);
        let rd = self.deposit.update(account_map);
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
        rw.and(rd).and(rc).and(rp)
    }

    fn quote(&self, quote_params: &QuoteParams) -> Result<Quote> {
        if quote_params.input_mint != self.withdraw.staked_sol_mint()
            || quote_params.output_mint != self.deposit.staked_sol_mint()
        {
            Err(anyhow!(
                "Cannot handle {} -> {}",
                quote_params.input_mint,
                quote_params.output_mint
            ))
        } else {
            quote_pool_pair(
                quote_params,
                self.prefund_repay_params_checked()?,
                &self.withdraw,
                &self.deposit,
            )
        }
    }

    fn get_swap_and_account_metas(&self, swap_params: &SwapParams) -> Result<SwapAndAccountMetas> {
        let bridge_stake_seed = rand::random();
        let mut account_metas = vec![STAKEDEX_ACCOUNT_META.clone()];
        account_metas.extend(get_account_metas(
            swap_params,
            self.prefund_repay_params_checked()?,
            &self.withdraw,
            &self.deposit,
            bridge_stake_seed,
        )?);
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

    fn unidirectional(&self) -> bool {
        true
    }

    fn get_accounts_len(&self) -> usize {
        1 + PREFUND_SWAP_VIA_STAKE_IX_ACCOUNTS_LEN
            + self.withdraw.accounts_len()
            + self.deposit.accounts_len()
    }

    fn underlying_liquidities(&self) -> Option<HashSet<Pubkey>> {
        self.underlying_liquidities.clone()
    }

    fn program_dependencies(&self) -> Vec<(Pubkey, String)> {
        vec![
            (
                self.withdraw.program_id(),
                self.withdraw.stake_pool_label().to_lowercase(),
            ),
            (
                self.deposit.program_id(),
                self.deposit.stake_pool_label().to_lowercase(),
            ),
            (unstake_it_program::ID, "unstake.it".to_owned()),
        ]
    }
}
