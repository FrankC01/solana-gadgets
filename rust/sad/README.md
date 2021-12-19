# sad - Solana Account Data Deserializer

__*Work in Progress (WIP)!*__ ...

## Overview
The idea is to be able to take any program owned account's data and deserialize it based
on descriptors (YAML)

This gadget presumes that data was serialized into accounts using `borsh` (little endian)

## Setup
In the `solana-gadgets/rust/sad` folder run: `cargo build`

## Execution
```bash
cargo run -- --help
```
## Sample Examples

The Sample [Program](https://github.com/hashblock/solana-cli-program-template) has been loaded into Solana's 'devnet'

Both of the user accounts ('user1', 'user2') as well as the Programs keys have been copied to the `../samples/keys` folder
The data descriptor file for the Program account owned data is provided in the `../samples/yamldecls` folder

1. For the examples output below, the data came from `devnet` where first a 'mint' occured on `user1` and then the key/value was transferred to `user2`. To run locally, first follow instructions for 'minting' and 'transferring' in the above Program repo

2. To see a single account data, where account key is from keypair file. `user1`:
