# scfsd - Solana Cluster Feature Set Diff

## Overview
`scfsd` gathers all feature sets and their status from the various Solana clusters

## Options
```bash
scfsd --help

scfsd 0.2.0
Solana Cluster Feature Set Diff

USAGE:
    scfsd [OPTIONS]

OPTIONS:
    -c, --cluster <cluster>         Clusters to analyze [default: all] [possible values: all, local,
                                    devnet, testnet, mainnet]
    -k, --keys-only-for-inactive    Generates list of inactivated feature keys for specific cluster
                                    (-c) of devnet, testnet or mainnet
    -t, --target-test-validator     Combined with -k, generates list of inactivated feature keys for
                                    input to solana-test-validator
    -h, --help                      Print help information
    -V, --version                   Print version information
```

## Sample default run
1. `cd solana-gadgets/rust/scfsd`
2. `cargo run`

You should see something similar to this (partial screen shot) where green is active and red it not.

## Variations
1. `cargo run -- -c devnet -c testnet` -> Fetch feature state from devnet and testnet only
2. `cargo run -- -c devnet -c testnet -c all` -> Defaults to all, ignoring other `-c` options
3. `cargo run -- -c devnet -k` -> Just prints the inactivated feature keys
4. `cargo run -- -c devnet -k -t` -> Just prints the inactivated feature keys with --deactivate-feature XXX prefix for solana-test-validator

Because for current local configuration `scfsd` doesn't even query `local` so you do not need to run `solana-test-validator` to use `scfsd`

![scfsd screen](images/screen1.png?raw=true "Screen")