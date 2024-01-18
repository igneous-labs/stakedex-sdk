# test-fixtures

Accounts cloned from mainnnet or mock accounts to load into `ProgramTest`.

## Cloning from mainnet

Set solana cli config to mainnet, then in workspace root:

```sh
solana account -o test-fixtures/<NEW-FILENAME>.json --output json <ACCOUNT-PUBKEY>
```

## Current data

Epoch of collection recorded in `tests/common.rs`
