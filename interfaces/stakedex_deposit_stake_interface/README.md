# stakedex_deposit_stake_interface

This is a virtual IDL, there's no actual on-chain program, but we use the generated accounts/instructions structs for convenience.

## Generate

In workspace root:

```sh
solores \
    -o ./interfaces \
    --solana-program-vers "workspace=true" \
    --borsh-vers "workspace=true" \
    --thiserror-vers "workspace=true" \
    --num-derive-vers "workspace=true" \
    --num-traits-vers "workspace=true" \
    --serde-vers "workspace=true" \
    interfaces/stakedex_deposit_stake_interface/idl.json
```
