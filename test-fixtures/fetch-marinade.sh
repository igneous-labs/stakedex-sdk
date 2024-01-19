#!/bin/sh
# Run in repo root

readonly DIR="test-fixtures"

solana account -o $DIR/msol-mint.json --output json mSoLzYCxHdYgdzU16g5QSh3i5K3z3KZK7ytfqcJm7So
solana account -o $DIR/marinade-state.json --output json 8szGkuLTAux9XMgZ2vtY39jVSowEcpBfFfD8hXSEqdGC
solana account -o $DIR/marinade-stake-list.json --output json Anv3XE7e5saNdm16MU6bniYS59Mpv7DzQXHAhxJUmAKW
solana account -o $DIR/marinade-validator-list.json --output json DwFYJNnhLmw19FBTrVaLWZ8SZJpxdPoSYVSJaio9tjbY
solana account -o $DIR/marinade-liq-pool-sol-leg.json --output json UefNb6z6yvArqe4cJHTXCqStRsKmWhGxnZzuHbikP5Q
solana account -o $DIR/marinade-liq-pool-msol-leg.json --output json 7GgPYjS5Dza89wV6FpZ23kUJRG5vbQ1GM25ezspYFSoE
solana account -o $DIR/marinade-reserves.json --output json Du3Ysj1wKbxPKkuPPnvzQLQh8oMSVifs3jGZjJWXFmHN
solana account -o $DIR/marinade-msol-treasury.json --output json B1aLzaNMeFVAyQ6f3XbbUyKcH2YPHu2fqiEagmiF23VR
