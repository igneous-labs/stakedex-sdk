pub mod lido_program {
    // .\xc3...xd1 = lido state
    sanctum_macros::declare_program_keys!(
        "CrX7kMhLC3cSsXJdT7JDgqrRVWGnUpX3gfEfxxU2NVLi",
        [
            ("stake_authority", b".\xc3\x8e\xfaG\x07\x0e\x1f\x83\r)\xbc%\xb8\x18\xa5U`rD\x01{\xdf\x9e\"\x9d\xfab\x18\xa2Y\xd1", b"stake_authority")
        ]
    );
}

pub mod lido_state {
    sanctum_macros::declare_program_keys!("49Yi1TKkNyYjPAFdR9LBvoHcUjuPX4Df5T5yv39w2XTn", []);
}

pub mod lido_validator_list {
    sanctum_macros::declare_program_keys!("GL9kqRNUTUosW3RsDoXHCuXUZn73SgQQmBvtp1ng2co4", []);
}

pub mod stsol {
    sanctum_macros::declare_program_keys!("7dHbWXmci3dT8UFYWYZweBLXgycu7Y3iL6trKn1Y7ARj", []);
}
