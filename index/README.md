# stakedex_index

Serde format for storing and decoding on-chain xSOL swaps data

## Overview

To allow dynamic discovery of swap pairs, data required to construct the stake pool structs for each pair is stored on-chain via the `RecordDex` instruction that creates an immutable record of this data on-chain in a `DexRecord` account.