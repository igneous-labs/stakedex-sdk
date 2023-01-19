use std::collections::HashMap;

use anyhow::{anyhow, Result};
use jupiter_core::amm::{Amm, Quote, QuoteParams, SwapLegAndAccountMetas, SwapParams};
use rust_decimal::{
    prelude::{FromPrimitive, Zero},
    Decimal,
};
use solana_program::{clock::Clock, instruction::AccountMeta, pubkey::Pubkey, sysvar};
use stakedex_interface::{SwapViaStakeKeys, SWAP_VIA_STAKE_IX_ACCOUNTS_LEN};

use crate::{
    account_missing_err, apply_global_fee, find_bridge_stake, find_fee_token_acc,
    find_stake_pool_pair_amm_key, DepositStake, DepositStakeInfo, DepositStakeQuote, WithdrawStake,
    WithdrawStakeQuote,
};

pub fn first_avail_quote<W: WithdrawStake + ?Sized, D: DepositStake + ?Sized>(
    withdraw_amount: u64,
    withdraw_from: &W,
    deposit_to: &D,
) -> Option<(WithdrawStakeQuote, DepositStakeQuote)> {
    let mut withdraw_quote_iter = withdraw_from.withdraw_stake_quote_iter(withdraw_amount);
    while let Some(wsq) = withdraw_quote_iter.next(withdraw_from) {
        if wsq.is_zero_out() {
            continue;
        }
        let dsq = match deposit_to.get_deposit_stake_quote(wsq) {
            Some(r) => r,
            None => return None,
        };
        if !dsq.is_zero_out() {
            return Some((wsq, dsq));
        }
    }
    None
}

pub fn quote_pool_pair<W: WithdrawStake + ?Sized, D: DepositStake + ?Sized>(
    quote_params: &QuoteParams,
    withdraw_from: &W,
    deposit_to: &D,
) -> Result<Quote> {
    let (withdraw_quote, deposit_quote) =
        first_avail_quote(quote_params.in_amount, withdraw_from, deposit_to)
            .ok_or_else(|| anyhow!("No route found between pools"))?;

    let in_amount = quote_params.in_amount;
    let aft_global_fees = apply_global_fee(deposit_quote.tokens_out);
    let out_amount = aft_global_fees.remainder;
    // total fees is sum of
    // - withdraw_from's withdraw stake fees (input mint)
    // - deposit_to's deposit stake fees (output mint)
    // - stakedex's global fees (output mint)
    let mut total_fees = aft_global_fees.fee + deposit_quote.fee_amount;
    // withdraw fees pct = withdraw_fees_in_token / quote_params.in_amount
    // approx withdraw fees in terms of out tokens
    // = before_fees * (withdraw fees pct / (1.0 - withdraw fees pct))
    // = before_fees * withdraw_fees_in_token / (quote_params.in_amount - withdraw_fees_in_token)
    let out_before_fees = deposit_quote.tokens_out + deposit_quote.fee_amount;
    let denom = quote_params
        .in_amount
        .checked_sub(withdraw_quote.fee_amount)
        .ok_or_else(|| anyhow!("100% withdrawal fees"))?;
    let approx_withdraw_fees_out_token = (out_before_fees as u128)
        .checked_mul(withdraw_quote.fee_amount as u128)
        .and_then(|v| v.checked_div(denom as u128))
        .and_then(|v| u64::try_from(v).ok())
        .ok_or_else(|| anyhow!("Math error"))?;
    total_fees += approx_withdraw_fees_out_token;
    let before_fees = out_before_fees + approx_withdraw_fees_out_token;
    let fee_pct =
        Decimal::from_f64((total_fees as f64) / (before_fees as f64)).unwrap_or_else(Decimal::zero);
    Ok(Quote {
        in_amount,
        out_amount,
        fee_amount: total_fees,
        fee_pct,
        fee_mint: deposit_to.staked_sol_mint(),
        ..Quote::default()
    })
}

pub fn get_account_metas<W: WithdrawStake + ?Sized, D: DepositStake + ?Sized>(
    swap_params: &SwapParams,
    withdraw_from: &W,
    deposit_to: &D,
    bridge_stake_seed: u32,
) -> Result<Vec<AccountMeta>> {
    // TODO: this is doing the same computation as it did in quote, should we cache this somehow?
    let (withdraw_quote, deposit_quote) =
        first_avail_quote(swap_params.in_amount, withdraw_from, deposit_to)
            .ok_or_else(|| anyhow!("No route found between pools"))?;
    let bridge_stake_seed_le_bytes = bridge_stake_seed.to_le_bytes();
    let bridge_stake = find_bridge_stake(
        &swap_params.user_transfer_authority,
        &bridge_stake_seed_le_bytes,
    )
    .0;
    let deposit_stake_info = DepositStakeInfo { addr: bridge_stake };
    let mut metas = Vec::from(<[AccountMeta; SWAP_VIA_STAKE_IX_ACCOUNTS_LEN]>::from(
        &SwapViaStakeKeys {
            payer: swap_params.user_transfer_authority,
            user: swap_params.user_transfer_authority,
            src_token_from: swap_params.user_source_token_account,
            src_token_mint: swap_params.source_mint,
            dest_token_to: swap_params.user_destination_token_account,
            dest_token_mint: swap_params.destination_mint,
            dest_token_fee_token_account: find_fee_token_acc(&swap_params.destination_mint).0,
            bridge_stake,
        },
    ));
    let withdraw_stake_virtual_ix = withdraw_from.virtual_ix(&withdraw_quote)?;
    metas.extend(withdraw_stake_virtual_ix.accounts);
    let deposit_stake_virtual_ix = deposit_to.virtual_ix(&deposit_quote, &deposit_stake_info)?;
    metas.extend(deposit_stake_virtual_ix.accounts);
    Ok(metas)
}

#[derive(Clone)]
pub struct OneWayPoolPair<
    W: WithdrawStake + Clone + Send + Sync + 'static,
    D: DepositStake + Clone + Send + Sync + 'static,
> {
    pub withdraw: W,
    pub deposit: D,
    pub clock: Clock,
}

impl<W, D> Amm for OneWayPoolPair<W, D>
where
    W: WithdrawStake + Clone + Send + Sync,
    D: DepositStake + Clone + Send + Sync,
{
    fn label(&self) -> String {
        format!(
            "{}+{} (StakeDex)",
            self.withdraw.stake_pool_label(),
            self.deposit.stake_pool_label()
        )
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
            self.withdraw.get_accounts_to_update(),
            self.deposit.get_accounts_to_update(),
            vec![sysvar::clock::ID],
        ]
        .concat()
    }

    fn update(&mut self, accounts_map: &HashMap<Pubkey, Vec<u8>>) -> Result<()> {
        // TODO: not sure if should short-circuit and early return if first update() fails
        let rw = self.withdraw.update(accounts_map);
        let rd = self.deposit.update(accounts_map);
        let rc = accounts_map
            .get(&sysvar::clock::ID)
            .ok_or_else(|| account_missing_err(&sysvar::clock::ID))
            .map_or_else(Err, |acc| Ok(bincode::deserialize(acc)?))
            .map(|new_clock| self.clock = new_clock);
        rw.and(rd).and(rc)
    }

    fn quote(&self, quote_params: &QuoteParams) -> Result<Quote> {
        if quote_params.input_mint != self.withdraw.staked_sol_mint()
            || quote_params.output_mint != self.deposit.staked_sol_mint()
        {
            return Err(anyhow!(
                "Cannot handle {} -> {}",
                quote_params.input_mint,
                quote_params.output_mint
            ));
        }
        quote_pool_pair(quote_params, &self.withdraw, &self.deposit)
    }

    fn get_swap_leg_and_account_metas(
        &self,
        swap_params: &SwapParams,
    ) -> Result<jupiter_core::amm::SwapLegAndAccountMetas> {
        let bridge_stake_seed = (self.clock.slot % u64::from(u32::MAX)).try_into().unwrap();
        let _metas = get_account_metas(
            swap_params,
            &self.withdraw,
            &self.deposit,
            bridge_stake_seed,
        )?;
        // TODO: jupiter overrides
        Err(anyhow!("UNIMPLEMENTED"))
    }

    fn clone_amm(&self) -> Box<dyn Amm + Send + Sync> {
        Box::new(self.clone())
    }
}

#[derive(Clone)]
pub struct TwoWayPoolPair<
    P1: DepositStake + WithdrawStake + Clone + Send + Sync + 'static,
    P2: DepositStake + WithdrawStake + Clone + Send + Sync + 'static,
> {
    pub p1: P1,
    pub p2: P2,
    pub clock: Clock,
}

impl<P1, P2> Amm for TwoWayPoolPair<P1, P2>
where
    P1: DepositStake + WithdrawStake + Clone + Send + Sync,
    P2: DepositStake + WithdrawStake + Clone + Send + Sync,
{
    fn label(&self) -> String {
        format!(
            "{}+{} (StakeDex)",
            self.p1.stake_pool_label(),
            self.p2.stake_pool_label()
        )
    }

    fn key(&self) -> Pubkey {
        find_stake_pool_pair_amm_key(&self.p1.main_state_key(), &self.p2.main_state_key()).0
    }

    fn get_reserve_mints(&self) -> Vec<Pubkey> {
        Vec::from([self.p1.staked_sol_mint(), self.p2.staked_sol_mint()])
    }

    fn get_accounts_to_update(&self) -> Vec<Pubkey> {
        [
            self.p1.get_accounts_to_update(),
            self.p2.get_accounts_to_update(),
            vec![sysvar::clock::ID],
        ]
        .concat()
    }

    fn update(&mut self, accounts_map: &HashMap<Pubkey, Vec<u8>>) -> Result<()> {
        // TODO: not sure if should short-circuit and early return if first update() fails
        let r1 = self.p1.update(accounts_map);
        let r2 = self.p2.update(accounts_map);
        let rc = accounts_map
            .get(&sysvar::clock::ID)
            .ok_or_else(|| account_missing_err(&sysvar::clock::ID))
            .map_or_else(Err, |acc| Ok(bincode::deserialize(acc)?))
            .map(|new_clock| self.clock = new_clock);
        r1.and(r2).and(rc)
    }

    fn quote(&self, quote_params: &QuoteParams) -> Result<Quote> {
        if quote_params.input_mint == self.p1.staked_sol_mint()
            && quote_params.output_mint == self.p2.staked_sol_mint()
        {
            return quote_pool_pair(quote_params, &self.p1, &self.p2);
        } else if quote_params.input_mint == self.p2.staked_sol_mint()
            && quote_params.output_mint == self.p1.staked_sol_mint()
        {
            return quote_pool_pair(quote_params, &self.p2, &self.p1);
        }
        Err(anyhow!(
            "Cannot handle {} -> {}",
            quote_params.input_mint,
            quote_params.output_mint
        ))
    }

    fn get_swap_leg_and_account_metas(
        &self,
        swap_params: &SwapParams,
    ) -> Result<SwapLegAndAccountMetas> {
        let bridge_stake_seed = (self.clock.slot % u64::from(u32::MAX)).try_into().unwrap();
        let _metas = if swap_params.source_mint == self.p1.staked_sol_mint()
            && swap_params.destination_mint == self.p2.staked_sol_mint()
        {
            get_account_metas(swap_params, &self.p1, &self.p2, bridge_stake_seed)?
        } else if swap_params.source_mint == self.p2.staked_sol_mint()
            && swap_params.destination_mint == self.p1.staked_sol_mint()
        {
            get_account_metas(swap_params, &self.p2, &self.p1, bridge_stake_seed)?
        } else {
            return Err(anyhow!(
                "Cannot handle {} -> {}",
                swap_params.source_mint,
                swap_params.destination_mint
            ));
        };
        // TODO: jupiter overrides
        Err(anyhow!("UNIMPLEMENTED"))
    }

    fn clone_amm(&self) -> Box<dyn Amm + Send + Sync> {
        Box::new(self.clone())
    }
}
