// struct ValidatorRecord is 53 bytes long borsh serialized
// but marinade serializes it with 8-bytes padding so it's 61 bytes in accountinfo.data
pub const VALIDATOR_RECORD_BYTE_LENGTH: usize = 61;
