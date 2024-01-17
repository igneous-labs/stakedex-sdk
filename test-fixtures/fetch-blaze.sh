#!/bin/sh
# Run in repo root

readonly DIR="test-fixtures"

solana account -o $DIR/bsol-mint.json --output json bSo13r4TkiE4KumL71LsHTPpL2euBYLFx6h9HP3piy1
solana account -o $DIR/blaze-reserves.json --output json rsrxDvYUXjH1RQj2Ke36LNZEVqGztATxFkqNukERqFT
solana account -o $DIR/blaze-pool.json --output json stk9ApL5HeVAwPLr3TLhDXdZS8ptVu7zp6ov8HFDuMi
solana account -o $DIR/blaze-list.json --output json 1istpXjy8BM7Vd5vPfA485frrV7SRJhgq5vs3sskWmc
