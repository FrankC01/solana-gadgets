# sad - Solana Account Data Deserializer


## Overview
Take any program owned account's data and deserialize it based on descriptors (YAML) and
renders the output in JSON (to screen or file)

This gadget presumes that data was serialized into accounts using `borsh` (little endian)

## Setup
In the `solana-gadgets/rust/sad` folder run: `cargo build`

## Execution
```bash
cargo run -- --help

sad 0.1.0


USAGE:
    sad [FLAGS] [OPTIONS] [SUBCOMMAND]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information
    -v, --verbose    Show additional information

OPTIONS:
    -d, --declfile <decl>        YAML data deserialization declaration file
    -C, --config <PATH>          Configuration file to use [default:
                                 /Users/frankcastellucci/.config/solana/cli/config.yml]
    -f, --filename <filename>    Filename for '-o json' output
    -u, --url <URL>              JSON RPC URL for the cluster [default: value from configuration file]
    -k, --keypair <keypair>      Keypair to extract public key from
    -o, --output <output>        Direct output to file [default: stdout]  [possible values: json, stdout]
    -p, --pubkey <pkstr>         Publickey Base58 string
    -s, --samplekey <sampkey>    Account or program sample name [possible values: user1, user2, prog]

SUBCOMMANDS:
    account    Deserialize single account
    help       Prints this message or the help of the given subcommand(s)
    program    Deserialize all program owned accounts
```
## Sample

This [Program](https://github.com/hashblock/solana-cli-program-template) has been
loaded into Solana's 'devnet'

Both of the user accounts ('user1', 'user2') as well as the Programs key ('prog') have been copied
to the `../samples/keys` folder

The data descriptor file for the Program account owned data is provided in the `../samples/yamldecls` folder

For the examples output below, the data came from the program in `devnet`, where first a 'mint' occurred
on `user1` and then the key/value was transferred to `user2`. The output content may differ by the time you
run this:

Account User 1 data, output defaults to stdout:

```bash
cargo run -- account -s user1

[
  {
    "account_key": "A94wMjV54C8f8wn7zL8TxNCdNiGoq7XSN7vWGrtd4vwU",
    "account_program_key": "SampGgdt3wioaoMZhC6LTSbg4pnuvQnSfJpDYeuXQBv",
    "data": {
      "initialized": true,
      "map": {
        "Happy": "New Year!",
        "newKey": "A new value",
        "python key": "python value",
        "ts key": "ts first value"
      },
      "map_length": 109
    }
  }
]
```

Account User 2 data, output defaults to stdout:

```bash
cargo run -- account -s user2

[
  {
    "account_key": "5gMsBeLmPkwEKQ1H2AwceAPasXLyZ4tvWGCYR59qf47U",
    "account_program_key": "SampGgdt3wioaoMZhC6LTSbg4pnuvQnSfJpDYeuXQBv",
    "data": {
      "initialized": true,
      "map": {
        "Dev1": "A new value"
      },
      "map_length": 27
    }
  }
]
```

All accounts data owned by program, output defaults to stdout:

```bash
cargo run -- program -s prog

[
  {
    "account_key": "A94wMjV54C8f8wn7zL8TxNCdNiGoq7XSN7vWGrtd4vwU",
    "account_program_key": "SampGgdt3wioaoMZhC6LTSbg4pnuvQnSfJpDYeuXQBv",
    "data": {
      "initialized": true,
      "map": {
        "Happy": "New Year!",
        "newKey": "A new value",
        "python key": "python value",
        "ts key": "ts first value"
      },
      "map_length": 109
    }
  },
  {
    "account_key": "5gMsBeLmPkwEKQ1H2AwceAPasXLyZ4tvWGCYR59qf47U",
    "account_program_key": "SampGgdt3wioaoMZhC6LTSbg4pnuvQnSfJpDYeuXQBv",
    "data": {
      "initialized": true,
      "map": {
        "Dev1": "A new value"
      },
      "map_length": 27
    }
  }
]
```

Depending on what you minted (key/value) your mileage may vary