#!/bin/sh
# Run in repo root

readonly DIR="test-fixtures"

solana account -o $DIR/stsol-mint.json --output json 7dHbWXmci3dT8UFYWYZweBLXgycu7Y3iL6trKn1Y7ARj
solana account -o $DIR/lido-list.json --output json GL9kqRNUTUosW3RsDoXHCuXUZn73SgQQmBvtp1ng2co4
solana account -o $DIR/lido-state.json --output json 49Yi1TKkNyYjPAFdR9LBvoHcUjuPX4Df5T5yv39w2XTn
