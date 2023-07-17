use std::collections::HashMap;

use anyhow::{anyhow, Result};
use jupiter_amm_interface::{
    Amm, KeyedAccount, Quote, QuoteParams, Swap, SwapAndAccountMetas, SwapParams,
};
use rust_decimal::{
    prelude::{FromPrimitive, Zero},
    Decimal,
};
use solana_program::{
    instruction::{AccountMeta, Instruction},
    pubkey::Pubkey,
    system_program,
};
use solana_sdk::account::Account;
use spl_token::native_mint;
use stakedex_interface::{StakeWrappedSolKeys, STAKE_WRAPPED_SOL_IX_ACCOUNTS_LEN};

use crate::{
    init_from_keyed_account::InitFromKeyedAccount,
    jupiter_stakedex_interface::STAKEDEX_ACCOUNT_META,
    pda::{
        cws_wsol_bridge_in, find_deposit_stake_amm_key, find_fee_token_acc, find_sol_bridge_out,
    },
    BaseStakePoolAmm, DepositSolQuoteError, TEMPORARY_JUP_AMM_LABEL,
};

#[derive(Copy, Clone, Debug)]
pub struct DepositSolQuote {
    pub in_amount: u64,

    /// After subtracting deposit fees
    pub out_amount: u64,

    /// Deposit fees, in staked_sol_mint
    pub fee_amount: u64,
}

pub trait DepositSol: BaseStakePoolAmm {
    fn can_accept_sol_deposits(&self) -> bool;

    /// This should only include the stake pool's fees, not stakedex's global fees
    fn get_deposit_sol_quote_unchecked(&self, lamports: u64) -> Result<DepositSolQuote>;

    fn virtual_ix(&self) -> Result<Instruction>;

    fn get_deposit_sol_quote(&self, lamports: u64) -> Result<DepositSolQuote> {
        if !self.can_accept_sol_deposits() {
            return Err(DepositSolQuoteError::CannotAcceptSolDeposits.into());
        }
        self.get_deposit_sol_quote_unchecked(lamports)
    }

    fn convert_quote(&self, deposit_sol_quote: DepositSolQuote) -> Quote {
        // global fee has been removed for depositsol
        //let aft_global_fees = apply_global_fee(deposit_sol_quote.out_amount);
        let total_fees = deposit_sol_quote.fee_amount; // + aft_global_fees.fee;
        let final_out_amount = deposit_sol_quote.out_amount; //aft_global_fees.remainder;
        let before_fees = (final_out_amount + total_fees) as f64;
        // Decimal::from_f64() returns None if infinite or NaN (before_fees = 0)
        let fee_pct =
            Decimal::from_f64((total_fees as f64) / before_fees).unwrap_or_else(Decimal::zero);
        Quote {
            in_amount: deposit_sol_quote.in_amount,
            out_amount: final_out_amount,
            fee_amount: total_fees,
            fee_pct,
            // TODO: fee_mint == staked_sol_mint is true for all stake pools for now
            fee_mint: self.staked_sol_mint(),
            ..Quote::default()
        }
    }
}

// newtype pattern in order to impl external trait on internal generic
#[derive(Clone)]
pub struct DepositSolWrapper<T: DepositSol + Clone + Send + Sync + 'static>(pub T);

impl<T> Amm for DepositSolWrapper<T>
where
    T: DepositSol + InitFromKeyedAccount + Clone + Send + Sync,
{
    fn from_keyed_account(keyed_account: &KeyedAccount) -> Result<Self> {
        T::from_keyed_account(keyed_account).map(|t| Self(t))
    }

    fn label(&self) -> String {
        TEMPORARY_JUP_AMM_LABEL.to_owned()
    }

    // To avoid key clashes with existing stake pools on jup (Marinade),
    // we can use a PDA like this
    fn key(&self) -> Pubkey {
        find_deposit_stake_amm_key(&self.0.main_state_key()).0
    }

    fn get_reserve_mints(&self) -> Vec<Pubkey> {
        Vec::from([native_mint::ID, self.0.staked_sol_mint()])
    }

    fn get_accounts_to_update(&self) -> Vec<Pubkey> {
        self.0.get_accounts_to_update()
    }

    fn update(&mut self, accounts_map: &HashMap<Pubkey, Account>) -> Result<()> {
        self.0.update(accounts_map)
    }

    fn quote(&self, quote_params: &QuoteParams) -> Result<Quote> {
        if quote_params.input_mint != native_mint::ID
            || quote_params.output_mint != self.0.staked_sol_mint()
        {
            return Err(anyhow!(
                "Cannot handle {} -> {}",
                quote_params.input_mint,
                quote_params.output_mint
            ));
        }
        let deposit_sol_quote = self.0.get_deposit_sol_quote(quote_params.in_amount)?;
        let quote = self.0.convert_quote(deposit_sol_quote);
        Ok(quote)
    }

    fn get_swap_and_account_metas(&self, swap_params: &SwapParams) -> Result<SwapAndAccountMetas> {
        let (sol_bridge_out, _) = find_sol_bridge_out();
        let mut account_metas = vec![STAKEDEX_ACCOUNT_META.clone()];
        account_metas.extend(<[AccountMeta; STAKE_WRAPPED_SOL_IX_ACCOUNTS_LEN]>::from(
            &StakeWrappedSolKeys {
                user: swap_params.token_transfer_authority,
                wsol_from: swap_params.source_token_account,
                dest_token_to: swap_params.destination_token_account,
                wsol_mint: swap_params.source_mint,
                dest_token_mint: swap_params.destination_mint,
                token_program: spl_token::ID,
                system_program: system_program::ID,
                wsol_bridge_in: cws_wsol_bridge_in(&sol_bridge_out),
                sol_bridge_out,
                dest_token_fee_token_account: find_fee_token_acc(&swap_params.destination_mint).0,
            },
        ));
        let deposit_sol_virtual_ix = self.0.virtual_ix()?;
        account_metas.extend(deposit_sol_virtual_ix.accounts);
        account_metas.push(swap_params.placeholder_account_meta());
        Ok(SwapAndAccountMetas {
            swap: Swap::StakeDexStakeWrappedSol,
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

    fn program_dependencies(&self) -> Vec<(Pubkey, String)> {
        use crate::{eversol_program, lido_program, marinade_program, socean_program};
        use solana_sdk::pubkey;

        let mut stake_pool_label = self.0.stake_pool_label();
        let stake_pool_program = match stake_pool_label {
            "Eversol" => eversol_program::ID,
            "Socean" => socean_program::ID,
            "Marinade" => marinade_program::ID,
            "Lido" => lido_program::ID,
            "Cogent" | "DaoPool" | "Jito" | "Laine" | "SolBlaze" => {
                stake_pool_label = "spl_stake_pool";
                pubkey!("SPoo1Ku8WFXoNDMHPsrGSTSG1Y47rzgn41SLUNakuHy")
            }
            _ => {
                println!("Label not recognized: {}", stake_pool_label);
                return vec![];
            }
        };
        vec![(stake_pool_program, stake_pool_label.into())]
    }
}
