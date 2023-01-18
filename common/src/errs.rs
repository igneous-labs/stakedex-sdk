use solana_program::pubkey::Pubkey;

pub fn account_missing_err(pk: &Pubkey) -> anyhow::Error {
    anyhow::anyhow!("{} missing in accounts_map", pk)
}
