# stakedex_interface

Crate generated using [solores](https://github.com/igneous-labs/solores) v0.3.0.

To regenerate, run `solores -o interfaces/ interfaces/stakedex_interface/idl.json -b workspace=true -s workspace=true` in base directory.

IDL is generated using `gen-idl.sh` in the stakedex repo.

This is the on-chain program's interface. However the `StakeWrappedSol` and `SwapViaStake` instructions have incomplete account inputs:

- For a complete `StakeWrappedSol` instruction, append one of the instruction accounts from `stakedex_deposit_sol_interface` to the end of the instruction's accounts.

    For example, to stake wrapped SOL to the Socean stake pool, you would append the `SoceanStakePoolDepositSol` instruction accounts to a `StakeWrappedSol` instruction.

- For a complete `SwapViaStake` instruction, append one of the instruction accounts from `stakedex_withdraw_stake_interface` to the end of the instruction's accounts, then one of the instruction accounts from `stakedex_deposit_stake_interface` to that.

    For example, to swap from scnSOL to laineSOL by withdrawing stake from socean then depositing the withdrawn stake to laine stake pool, you would append the `SoceanStakePoolWithdrawStake` instruction accounts, followed by the `SplStakePoolWithdrawStake` instruction accounts to a `SwapViaStake` instruction.