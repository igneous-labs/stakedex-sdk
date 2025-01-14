pub mod stakedex_program {
    sanctum_macros::declare_program_keys!(
        "stkitrT1Uoy18Dk1fTrgPw8W6MVzoCfYoAFT4MLsmhq",
        [
            ("sol-bridge-out", b"sol_bridge_out"),
            ("prefunder", b"prefunder"),
            ("wsol-fee-token-account", b"fee", b"\x06\x9b\x88W\xfe\xab\x81\x84\xfbh\x7fcF\x18\xc05\xda\xc49\xdc\x1a\xeb;U\x98\xa0\xf0\x00\x00\x00\x00\x01")
        ]
    );
}

pub mod wsol_bridge_in {
    sanctum_macros::create_with_seed!(
        "75jTZDE78xpBJokeB2BcimRNY5BZ7U45bWhpgUrTzWZC",
        "wsol_bridge_in",
        "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA",
    );
}
