# dex

A simple DEX program on Solana (Anchor), made for fun to understand how decentralized exchanges work on-chain.

## What's here

- a Rust/Anchor program with an orderbook (bid/ask)
- basic structures: buy offer (bid), sell offer (ask), orderbooks for both
- tests in `programs/dex/tests`

## Why

Not a product, just a learning project - to see firsthand how placing offers, matching orders, and keeping state on a blockchain actually works.

## How to run

```
anchor build
anchor test
```
