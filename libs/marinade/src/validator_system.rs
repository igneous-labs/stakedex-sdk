use marinade_finance_interface::ValidatorRecord;
use solana_program::pubkey::Pubkey;
use stakedex_sdk_common::marinade_program;

pub struct ValidatorRecordWrapper<'a>(pub &'a ValidatorRecord);

impl<'a> ValidatorRecordWrapper<'a> {
    pub const DUPLICATE_FLAG_SEED: &'static [u8] = b"unique_validator";

    pub fn find_duplication_flag(state: &Pubkey, validator_account: &Pubkey) -> (Pubkey, u8) {
        Pubkey::find_program_address(
            &[
                &state.to_bytes()[..32],
                Self::DUPLICATE_FLAG_SEED,
                &validator_account.to_bytes()[..32],
            ],
            &marinade_program::ID,
        )
    }
}
