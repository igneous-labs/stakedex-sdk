use anyhow::{anyhow, Result};
use jupiter_amm_interface::{
    AccountMap, Amm, AmmContext, KeyedAccount, Quote, QuoteParams, Swap, SwapAndAccountMetas,
    SwapParams,
};
use solana_sdk::{instruction::AccountMeta, pubkey::Pubkey};
use spl_token::native_mint;
use stakedex_interface::{WithdrawWrappedSolKeys, WITHDRAW_WRAPPED_SOL_IX_ACCOUNTS_LEN};
use stakedex_sdk_common::{
    find_fee_token_acc, find_withdraw_stake_amm_key, InitFromKeyedAccount, WithdrawSol,
    TEMPORARY_JUP_AMM_LABEL,
};

use crate::jupiter_stakedex_interface::STAKEDEX_ACCOUNT_META;

// newtype pattern in order to impl external trait (Amm) on external generic (WithdrawSol)
#[derive(Clone)]
pub struct WithdrawSolWrapper<T: WithdrawSol + Clone + Send + Sync + 'static>(pub T);

impl<T> Amm for WithdrawSolWrapper<T>
where
    T: WithdrawSol + InitFromKeyedAccount + Clone + Send + Sync,
{
    fn from_keyed_account(keyed_account: &KeyedAccount, amm_context: &AmmContext) -> Result<Self> {
        T::from_keyed_account(keyed_account, amm_context).map(|t| Self(t))
    }

    fn label(&self) -> String {
        TEMPORARY_JUP_AMM_LABEL.to_owned()
    }

    // To avoid key clashes with existing stake pools on jup (Marinade),
    // we can use a PDA like this
    fn key(&self) -> Pubkey {
        find_withdraw_stake_amm_key(&self.0.main_state_key()).0
    }

    fn get_reserve_mints(&self) -> Vec<Pubkey> {
        // TODO: confirm this
        Vec::from([native_mint::ID, self.0.staked_sol_mint()])
    }

    fn get_accounts_to_update(&self) -> Vec<Pubkey> {
        self.0.get_accounts_to_update()
    }

    fn update(&mut self, accounts_map: &AccountMap) -> Result<()> {
        self.0.update(accounts_map)
    }

    fn quote(&self, quote_params: &QuoteParams) -> Result<Quote> {
        if quote_params.input_mint != self.0.staked_sol_mint()
            || quote_params.output_mint != native_mint::ID
        // TODO: confirm output_mint is wrapped sol
        {
            return Err(anyhow!(
                "Cannot handle {} -> {}",
                quote_params.input_mint,
                quote_params.output_mint
            ));
        }
        let withdraw_sol_quote = self.0.get_withdraw_sol_quote(quote_params.amount)?;
        let quote = self.0.convert_quote(withdraw_sol_quote);
        Ok(quote)
    }

    fn get_swap_and_account_metas(&self, swap_params: &SwapParams) -> Result<SwapAndAccountMetas> {
        let mut account_metas = vec![STAKEDEX_ACCOUNT_META.clone()];
        account_metas.extend(<[AccountMeta; WITHDRAW_WRAPPED_SOL_IX_ACCOUNTS_LEN]>::from(
            WithdrawWrappedSolKeys {
                user: swap_params.token_transfer_authority,
                src_token_from: swap_params.source_token_account,
                wsol_to: swap_params.destination_token_account,
                wsol_fee_token_account: find_fee_token_acc(&swap_params.destination_mint).0,
                src_token_mint: swap_params.source_mint,
                wsol_mint: swap_params.destination_mint,
                token_program: spl_token::ID,
            },
        ));

        let withdraw_sol_virtual_ix = self.0.virtual_ix()?;
        account_metas.extend(withdraw_sol_virtual_ix.accounts);
        account_metas.push(swap_params.placeholder_account_meta());
        Ok(SwapAndAccountMetas {
            // TODO: what should be here?
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

    fn get_accounts_len(&self) -> usize {
        1 + WITHDRAW_WRAPPED_SOL_IX_ACCOUNTS_LEN + self.0.accounts_len()
    }

    fn program_dependencies(&self) -> Vec<(Pubkey, String)> {
        vec![(
            self.0.program_id(),
            self.0.stake_pool_label().to_lowercase(),
        )]
    }
}
