use anyhow::{anyhow, Result};
use jupiter_amm_interface::{Quote, QuoteParams, SwapParams};
use rust_decimal::prelude::*;
use solana_sdk::{instruction::AccountMeta, pubkey::Pubkey, stake, system_program, sysvar};
use spl_token::native_mint;
use stakedex_interface::{
    DepositStakeKeys, PrefundSwapViaStakeKeys, PrefundWithdrawStakeKeys,
    DEPOSIT_STAKE_IX_ACCOUNTS_LEN, PREFUND_SWAP_VIA_STAKE_IX_ACCOUNTS_LEN,
    PREFUND_WITHDRAW_STAKE_IX_ACCOUNTS_LEN,
};
use stakedex_sdk_common::{
    apply_global_fee, find_bridge_stake, find_fee_token_acc, slumdog_stake_create_with_seed,
    stakedex_program, unstake_it_pool, unstake_it_program, DepositStake, DepositStakeInfo,
    DepositStakeQuote, SwapViaStakeQuoteErr, WithdrawStake, WithdrawStakeQuote,
    WithdrawStakeQuoteErr, DEPOSIT_STAKE_DST_TOKEN_MINT_IDX,
    PREFUND_WITHDRAW_STAKE_SRC_TOKEN_MINT_IDX, STAKE_ACCOUNT_RENT_EXEMPT_LAMPORTS,
    SWAP_VIA_STAKE_DST_TOKEN_MINT_ACCOUNT_INDEX, SWAP_VIA_STAKE_SRC_TOKEN_MINT_ACCOUNT_INDEX,
};
use std::collections::HashSet;

use crate::PrefundRepayParams;

/// Due to CPI restrictions, PrefundSwapViaStake cannot be CPI'd directly and needs to be
/// split into 2 CPIs.
///
/// Returns:
/// [
///    ...PrefundWithdrawStake metas,
///    jup_v6_program_id as separator,
///    ...DepositStake metas
/// ]
pub fn manual_concat_get_account_metas<W: WithdrawStake + ?Sized, D: DepositStake + ?Sized>(
    swap_params: &SwapParams,
    prefund_repay_params: &PrefundRepayParams,
    withdraw_from: &W,
    deposit_to: &D,
    bridge_stake_seed: u32,
) -> Result<Vec<AccountMeta>> {
    // TODO: this is doing the same computation as it did in quote, should we cache this somehow?
    let prefund_split_lamports = prefund_repay_params.prefund_split_lamports()?;
    let (withdraw_quote, deposit_quote) = first_avail_prefund_quote(
        swap_params.in_amount,
        prefund_split_lamports,
        withdraw_from,
        deposit_to,
    )?;
    let bridge_stake_seed_le_bytes = bridge_stake_seed.to_le_bytes();
    let bridge_stake = find_bridge_stake(
        &swap_params.token_transfer_authority,
        &bridge_stake_seed_le_bytes,
    )
    .0;
    let slumdog_stake = slumdog_stake_create_with_seed(&bridge_stake)?;
    let deposit_stake_info = DepositStakeInfo { addr: bridge_stake };
    let mut prefund_withdraw_prefix =
        <[AccountMeta; PREFUND_WITHDRAW_STAKE_IX_ACCOUNTS_LEN]>::from(PrefundWithdrawStakeKeys {
            user: swap_params.token_transfer_authority,
            src_token_from: swap_params.source_token_account,
            bridge_stake,
            src_token_mint: swap_params.source_mint,
            prefunder: stakedex_program::PREFUNDER_ID,
            slumdog_stake,
            unstakeit_program: unstake_it_program::ID,
            unstake_pool: unstake_it_pool::ID,
            pool_sol_reserves: unstake_it_program::SOL_RESERVES_ID,
            unstake_fee: unstake_it_program::FEE_ID,
            slumdog_stake_acc_record: find_stake_account_record(&slumdog_stake).0,
            unstake_protocol_fee: unstake_it_program::PROTOCOL_FEE_ID,
            unstake_protocol_fee_dest: prefund_repay_params.protocol_fee_dest,
            clock: sysvar::clock::ID,
            stake_program: stake::program::ID,
            system_program: system_program::ID,
        });
    if prefund_withdraw_prefix[PREFUND_WITHDRAW_STAKE_SRC_TOKEN_MINT_IDX].pubkey == native_mint::ID
    {
        prefund_withdraw_prefix[PREFUND_WITHDRAW_STAKE_SRC_TOKEN_MINT_IDX].is_writable = false;
    }
    let mut deposit_prefix =
        <[AccountMeta; DEPOSIT_STAKE_IX_ACCOUNTS_LEN]>::from(DepositStakeKeys {
            user: swap_params.token_transfer_authority,
            stake_account: bridge_stake,
            dest_token_to: swap_params.destination_token_account,
            dest_token_fee_token_account: find_fee_token_acc(&swap_params.destination_mint).0,
            dest_token_mint: swap_params.destination_mint,
        });
    if deposit_prefix[DEPOSIT_STAKE_DST_TOKEN_MINT_IDX].pubkey == native_mint::ID {
        deposit_prefix[DEPOSIT_STAKE_DST_TOKEN_MINT_IDX].is_writable = false;
    }
    Ok(prefund_withdraw_prefix
        .into_iter()
        .chain(withdraw_from.virtual_ix(&withdraw_quote)?.accounts)
        .chain(std::iter::once(swap_params.placeholder_account_meta()))
        .chain(deposit_prefix)
        .chain(
            deposit_to
                .virtual_ix(&deposit_quote, &deposit_stake_info)?
                .accounts,
        )
        .collect())
}

pub fn prefund_get_account_metas<W: WithdrawStake + ?Sized, D: DepositStake + ?Sized>(
    swap_params: &SwapParams,
    prefund_repay_params: &PrefundRepayParams,
    withdraw_from: &W,
    deposit_to: &D,
    bridge_stake_seed: u32,
) -> Result<Vec<AccountMeta>> {
    // TODO: this is doing the same computation as it did in quote, should we cache this somehow?
    let prefund_split_lamports = prefund_repay_params.prefund_split_lamports()?;
    let (withdraw_quote, deposit_quote) = first_avail_prefund_quote(
        swap_params.in_amount,
        prefund_split_lamports,
        withdraw_from,
        deposit_to,
    )?;
    let bridge_stake_seed_le_bytes = bridge_stake_seed.to_le_bytes();
    let bridge_stake = find_bridge_stake(
        &swap_params.token_transfer_authority,
        &bridge_stake_seed_le_bytes,
    )
    .0;
    let slumdog_stake = slumdog_stake_create_with_seed(&bridge_stake)?;
    let deposit_stake_info = DepositStakeInfo { addr: bridge_stake };
    let mut metas = Vec::from(
        <[AccountMeta; PREFUND_SWAP_VIA_STAKE_IX_ACCOUNTS_LEN]>::from(PrefundSwapViaStakeKeys {
            user: swap_params.token_transfer_authority,
            src_token_from: swap_params.source_token_account,
            src_token_mint: swap_params.source_mint,
            dest_token_to: swap_params.destination_token_account,
            dest_token_mint: swap_params.destination_mint,
            dest_token_fee_token_account: find_fee_token_acc(&swap_params.destination_mint).0,
            bridge_stake,
            prefunder: stakedex_program::PREFUNDER_ID,
            slumdog_stake,
            unstakeit_program: unstake_it_program::ID,
            unstake_pool: unstake_it_pool::ID,
            pool_sol_reserves: unstake_it_program::SOL_RESERVES_ID,
            unstake_fee: unstake_it_program::FEE_ID,
            slumdog_stake_acc_record: find_stake_account_record(&slumdog_stake).0,
            unstake_protocol_fee: unstake_it_program::PROTOCOL_FEE_ID,
            unstake_protocol_fee_dest: prefund_repay_params.protocol_fee_dest,
            clock: sysvar::clock::ID,
            stake_program: stake::program::ID,
            system_program: system_program::ID,
        }),
    );
    for mint_idx in [
        SWAP_VIA_STAKE_SRC_TOKEN_MINT_ACCOUNT_INDEX,
        SWAP_VIA_STAKE_DST_TOKEN_MINT_ACCOUNT_INDEX,
    ] {
        if metas[mint_idx].pubkey == native_mint::ID {
            metas[mint_idx].is_writable = false;
        }
    }
    let withdraw_stake_virtual_ix = withdraw_from.virtual_ix(&withdraw_quote)?;
    metas.extend(withdraw_stake_virtual_ix.accounts);
    let deposit_stake_virtual_ix = deposit_to.virtual_ix(&deposit_quote, &deposit_stake_info)?;
    metas.extend(deposit_stake_virtual_ix.accounts);
    Ok(metas)
}

pub fn quote_pool_pair<W: WithdrawStake + ?Sized, D: DepositStake + ?Sized>(
    quote_params: &QuoteParams,
    prefund_repay_params: &PrefundRepayParams,
    withdraw_from: &W,
    deposit_to: &D,
) -> Result<Quote> {
    let prefund_split_lamports = prefund_repay_params.prefund_split_lamports()?;
    let (withdraw_quote, deposit_quote) = first_avail_prefund_quote(
        quote_params.amount,
        prefund_split_lamports,
        withdraw_from,
        deposit_to,
    )?;

    let in_amount = quote_params.amount;
    let aft_global_fees = apply_global_fee(deposit_quote.tokens_out);
    let out_amount = aft_global_fees.remainder;
    // total fees is sum of following fees in sequence:
    // 1. withdraw_from's withdraw stake fees (input mint)
    // 2. instant unstake fee for slumdog stake to repay prefund (SOL)
    // 3. deposit_to's deposit stake fees (output mint)
    // 4. stakedex's global fees (output mint)

    // in terms of output mint
    let mut approx_total_fees = aft_global_fees.fee + deposit_quote.fee_amount;
    // before global fees + deposit stake fees, after prefund repay, in terms of out token
    let mut approx_before_fees = deposit_quote.tokens_out + deposit_quote.fee_amount;

    let approx_prefund_fee_out_token = approx_fees_charged_out_token(
        approx_before_fees,
        prefund_split_lamports,
        withdraw_quote.lamports_out,
    )?;
    approx_total_fees += approx_prefund_fee_out_token;
    // approx before global fees + deposit stake fees + prefund repay fees, after withdraw stake fees, in terms of out token
    approx_before_fees += approx_prefund_fee_out_token;

    let approx_withdraw_stake_fee_out_token = approx_fees_charged_out_token(
        approx_before_fees,
        withdraw_quote.fee_amount,
        quote_params.amount,
    )?;
    approx_total_fees += approx_withdraw_stake_fee_out_token;
    approx_before_fees += approx_withdraw_stake_fee_out_token;

    let fee_pct = Decimal::from_f64((approx_total_fees as f64) / (approx_before_fees as f64))
        .unwrap_or_else(Decimal::zero);
    Ok(Quote {
        in_amount,
        out_amount,
        fee_amount: approx_total_fees,
        fee_pct,
        fee_mint: deposit_to.staked_sol_mint(),
        ..Quote::default()
    })
}

pub(crate) fn prepare_underlying_liquidities(
    underlying_liquidities: &[Option<&Pubkey>],
) -> Option<HashSet<Pubkey>> {
    let uls = HashSet::from_iter(underlying_liquidities.iter().filter_map(|ul| ul.cloned()));
    if !uls.is_empty() {
        Some(uls)
    } else {
        None
    }
}

/// Returns
/// (
///   withdraw_stake_quote before splitting off prefund lamports,
///   deposit_stake_quote,
///)
pub fn first_avail_prefund_quote<W: WithdrawStake + ?Sized, D: DepositStake + ?Sized>(
    withdraw_amount: u64,
    prefund_split_lamports: u64,
    withdraw_from: &W,
    deposit_to: &D,
) -> Result<(WithdrawStakeQuote, DepositStakeQuote), SwapViaStakeQuoteErr> {
    if !withdraw_from.can_accept_stake_withdrawals() {
        return Err(WithdrawStakeQuoteErr::CannotAcceptStakeWithdrawals.into());
    }
    let withdraw_quote_iter = withdraw_from.withdraw_stake_quote_iter_dyn(withdraw_amount);
    for wsq in withdraw_quote_iter {
        let wsq = prefund_transform_wsq(wsq);
        let wsq_after_repaying_prefund = wsq_post_prefund_repay(wsq, prefund_split_lamports);
        if !wsq_after_repaying_prefund.is_rent_exempt() {
            continue;
        }
        let dsq = deposit_to.get_deposit_stake_quote(wsq_after_repaying_prefund)?;

        if !dsq.is_zero_out() {
            return Ok((wsq, dsq));
        }
    }
    Err(SwapViaStakeQuoteErr::NoRouteFound)
}

/// Returns the state of the stake account with rent-exempt lamports prefunded.
///
/// This is basically adding [`STAKE_ACCOUNT_RENT_EXEMPT_LAMPORTS`] non-staked lamports to the stake account
/// while treating its original [`WithdrawStakeQuote::lamports_out`] as the [`WithdrawStakeQuote::lamports_staked`]
pub fn prefund_transform_wsq(mut wsq: WithdrawStakeQuote) -> WithdrawStakeQuote {
    wsq.lamports_staked = wsq.lamports_out;
    wsq.lamports_out += STAKE_ACCOUNT_RENT_EXEMPT_LAMPORTS;
    wsq
}

/// Returns the state of the stake account after `prefund_split_lamports` is split
/// from it to repay the prefund flash loan
pub fn wsq_post_prefund_repay(
    mut wsq: WithdrawStakeQuote,
    prefund_split_lamports: u64,
) -> WithdrawStakeQuote {
    wsq.lamports_out = wsq.lamports_out.saturating_sub(prefund_split_lamports);
    wsq.lamports_staked = wsq.lamports_staked.saturating_sub(prefund_split_lamports);
    wsq
}

/// Calculates approximate fees charged in terms of out token given known
/// amt_after_fee in terms of out token and fee ratio
fn approx_fees_charged_out_token(amt_after_fee: u64, fee_num: u64, fee_denom: u64) -> Result<u64> {
    // fee_rate = fee_num / fee_denom
    // (1.0 - fee_rate) * amt_before_fee = amt_after_fee
    // amt_before_fee = amt_after_fee / (1.0 - fee_rate)
    // = fee_denom * amt_after_fee / (fee_denom - fee_num)
    //
    // fees_charged = amt_before_fee - amt_after_fee
    // = fee_denom * amt_after_fee / (fee_denom - fee_num) - amt_after_fee
    // = amt_after_fee (fee_denom / (fee_denom - fee_num) - 1)
    // = amt_after_fee * fee_num / (fee_denom - fee_num)
    let denom = fee_denom
        .checked_sub(fee_num)
        .ok_or_else(|| anyhow!("100% withdrawal fees"))?;
    (amt_after_fee as u128 * fee_num as u128)
        .checked_div(denom as u128)
        .and_then(|v| u64::try_from(v).ok())
        .ok_or_else(|| anyhow!("Math error"))
}

/// TODO: this should really be in unstake-lib instead.
/// Duplicating from stakedex_unstake_it to avoid additional dep
fn find_stake_account_record(stake_account: &Pubkey) -> (Pubkey, u8) {
    Pubkey::find_program_address(
        &[&unstake_it_pool::ID.to_bytes(), &stake_account.to_bytes()],
        &unstake_it_program::ID,
    )
}
