# stakedex_interface

This is the on-chain program's interface. However the `StakeWrappedSol` and `SwapViaStake` instructions have incomplete account inputs:

- For a complete `StakeWrappedSol` instruction, append one of the instruction accounts from `stakedex_deposit_sol_interface` to the end of the instruction's accounts.

  For example, to stake wrapped SOL to the Socean stake pool, you would append the `SoceanStakePoolDepositSol` instruction accounts to a `StakeWrappedSol` instruction.

- For a complete `SwapViaStake` instruction, append one of the instruction accounts from `stakedex_withdraw_stake_interface` to the end of the instruction's accounts, then one of the instruction accounts from `stakedex_deposit_stake_interface` to that.

  For example, to swap from scnSOL to laineSOL by withdrawing stake from socean then depositing the withdrawn stake to laine stake pool, you would append the `SoceanStakePoolWithdrawStake` instruction accounts, followed by the `SplStakePoolWithdrawStake` instruction accounts to a `SwapViaStake` instruction.

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
    interfaces/stakedex_interface/idl.json
```
