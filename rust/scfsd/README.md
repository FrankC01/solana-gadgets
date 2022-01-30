# scfsd - Solana Cluster Feature Set Diff

## Overview
`scfsd` gathers all feature sets and their status from the various Solana clusters

## Options (output to file not yet supported)
```bash
scfsd -h

scfsd 0.1.0
Solana Cluster Feature Set Diff

USAGE:
    scfsd [OPTIONS]

OPTIONS:
    -f, --filename <filename>    Output to filename in csv format
    -h, --help                   Print help information
    -V, --version                Print version information
```

## Running
1. `cd solana-gadgets/rust/scfsd`
1. `cargo run`

You should see something similar to this (partial screen shot) where green is active and red it not.

Note: Local features are **_all_** active as `solana-test-validator` loads them all by default.
Because of this `scfsd` doesn't even query `local` so you do not need to run `solana-test-validator` to use `scfsd`

![scfsd screen](images/screen1.png?raw=true "Screen")