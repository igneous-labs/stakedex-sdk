pub mod stakedex_program {
    sanctum_macros::declare_program_keys!(
        "stkitrT1Uoy18Dk1fTrgPw8W6MVzoCfYoAFT4MLsmhq",
        [
            ("sol-bridge-out", b"sol_bridge_out"),
            ("prefunder", b"prefunder"),
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
