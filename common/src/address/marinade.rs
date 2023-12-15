pub mod marinade_program {
    // u\x11...\xf1 = marinade state addr
    sanctum_macros::declare_program_keys!(
        "MarBmsSgKXdrN1egZf5sqe1TMai9K1rChYNDJgjq7aD",
        [
            ("liq_pool_sol_leg", b"u\x11\x9b1u\x80u\x86\xe3\xf4\xa7\xe5\xcd\x0f\x89\x0e\x96\xa7S\xb1\x0f\xcc\xc7h\x1e\x94s\xa0\x082p\xf1", b"liq_sol"),
            ("liq_pool_msol_leg_authority", b"u\x11\x9b1u\x80u\x86\xe3\xf4\xa7\xe5\xcd\x0f\x89\x0e\x96\xa7S\xb1\x0f\xcc\xc7h\x1e\x94s\xa0\x082p\xf1", b"liq_st_sol_authority"),
            ("msol_mint_auth", b"u\x11\x9b1u\x80u\x86\xe3\xf4\xa7\xe5\xcd\x0f\x89\x0e\x96\xa7S\xb1\x0f\xcc\xc7h\x1e\x94s\xa0\x082p\xf1", b"st_mint"),
            ("reserve", b"u\x11\x9b1u\x80u\x86\xe3\xf4\xa7\xe5\xcd\x0f\x89\x0e\x96\xa7S\xb1\x0f\xcc\xc7h\x1e\x94s\xa0\x082p\xf1", b"reserve"),
        ]
    );
}

pub mod marinade_state {
    sanctum_macros::declare_program_keys!("8szGkuLTAux9XMgZ2vtY39jVSowEcpBfFfD8hXSEqdGC", []);
}

pub mod msol {
    sanctum_macros::declare_program_keys!("mSoLzYCxHdYgdzU16g5QSh3i5K3z3KZK7ytfqcJm7So", []);
}
