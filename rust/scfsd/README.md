# scfsd - Solana Cluster Feature Set Diff

## Overview
`scfsd` gathers all feature sets and their status from the various Solana clusters

## Options
```bash
scfsd -h

scfsd 0.1.0
Solana Cluster Feature Set Diff

USAGE:
    scfsd [OPTIONS]

OPTIONS:
    -c, --cluster <cluster>    Clusters to analyze [default: all] [possible values: all, local,
                               devnet, testnet, mainnet]
    -h, --help                 Print help information
    -V, --version              Print version information
```

## Sample default run
1. `cd solana-gadgets/rust/scfsd`
2. `cargo run`

You should see something similar to this (partial screen shot) where green is active and red it not.

## Variations
1. `cargo run -- -c devnet -c testnet` -> Fetch feature state from devnet and testnet only
2. `cargo run -- -c devnet -c testnet -c all` -> Defaults to all, ignoring other `-c` options


Note: Local features are **_all_** active as `solana-test-validator` loads them all by default.
Because of this `scfsd` doesn't even query `local` so you do not need to run `solana-test-validator` to use `scfsd`

![scfsd screen](images/screen1.png?raw=true "Screen")