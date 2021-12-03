# sad - Solana Account Data Deserializer

__*Work in Progress (WIP)!*__ ...

## Overview
The idea is to be able to take any program owned account's data and deserialize it based
on descriptors (YAML)

This gadget presumes that data was serialized into accounts using `borsh` (little endian)

## Setup
In the `solana-gadgets/rust` folder run: `cargo build`

## Example
These examples presume your Solana configuration is pointing to devnet where the program [Solana CLI Program](https://github.com/hashblock/solana-cli-program-template) was deployed.

For said program, and included in this repos `samples` folder, are the program account keys:

```
$ ls ../samples/keys/
user1_account.json	user2_account.json
```

as well as the YAML data declaration file:
```
$ ls ../samples/yamldecls/
SampGgdt3wioaoMZhC6LTSbg4pnuvQnSfJpDYeuXQBv.yml
```
