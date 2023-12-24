pub mod unstake_it_program {
    // \xde\...\xc6 = unstakeit pool
    sanctum_macros::declare_program_keys!(
        "unpXTU2Ndrc7WWNyEhQWe4udTzSibLPi25SXv2xbCHQ",
        [
            ("sol_reserves", b"\xde\x91\xbbP4tnb;\xfb6\xb7=\xae\"\xa4\x83\xb7\xcf\'\xd2\xad\x83\xfa\x8cx\x05\xa6\xcc`+\xc6"),
            ("fee", b"\xde\x91\xbbP4tnb;\xfb6\xb7=\xae\"\xa4\x83\xb7\xcf\'\xd2\xad\x83\xfa\x8cx\x05\xa6\xcc`+\xc6", b"fee"),
            ("protocol_fee", b"protocol-fee")
        ]
    );
}

pub mod unstake_it_pool {
    sanctum_macros::declare_program_keys!("FypPtwbY3FUfzJUtXHSyVRokVKG2jKtH29FmK4ebxRSd", []);
}
