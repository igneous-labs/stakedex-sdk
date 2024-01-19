#!/bin/sh
# Run in repo root

readonly DIR="test-fixtures"

solana account -o $DIR/scnsol-mint.json --output json 5oVNBeEEQvYi1cX3ir8Dx5n1P7pdxydbGF2X4TxVusJm
solana account -o $DIR/socean-reserves.json --output json 4sDXGroVt7ba45rzXtNto97QjG1rHm8Py3v56Mgg16Nc
solana account -o $DIR/socean-pool.json --output json 5oc4nmbNTda9fx8Tw57ShLD132aqDK65vuHH4RU1K4LZ
solana account -o $DIR/socean-list.json --output json 8pTa29ovYHxjQgX7gjxGi395GAo8DSXCRTKJZvwMc6MR
solana account -o $DIR/socean-manager-fee-dest.json --output json 4nvTrY3KdYCVEtzfopCDZ2NuL8u6ZaHgL7xcUnQDQpHe
