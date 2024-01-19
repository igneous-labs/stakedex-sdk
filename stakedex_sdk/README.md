# stakedex_sdk

The main SDK crate.

## Test

To run the find-vsa test to figure out which VSAs are use to SwapViaStake between stake pools:

`cargo test find_vsa -- --nocapture`

To run the test-fixtures test:

`cargo test`

To run the tests against mainnet:

`SOLANA_RPC_URL=<RPC-URL> cargo test --features mainnet-test -- --nocapture`
