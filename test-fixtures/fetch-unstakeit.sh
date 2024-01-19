#!/bin/sh
# Run in repo root

readonly DIR="test-fixtures"

solana account -o $DIR/unstake-pool.json --output json FypPtwbY3FUfzJUtXHSyVRokVKG2jKtH29FmK4ebxRSd
solana account -o $DIR/unstake-reserves.json --output json 3rBnnH9TTgd3xwu48rnzGsaQkSr1hR64nY71DrDt6VrQ
solana account -o $DIR/unstake-fees.json --output json 5Pcu8WeQa3VbBz2vdBT49Rj4gbS4hsnfzuL1LmuRaKFY
solana account -o $DIR/unstake-protocol-fees.json --output json 2hN9UhvRFVfPYKL6rZJ5YiLEPCLTpN755pgwDJHWgFbU
solana account -o $DIR/unstake-protocol-fees-dest.json --output json GnRGTBrFuEwb85Zs4zeZWUzQYfTwmPxCPYmQQodDzYUK
