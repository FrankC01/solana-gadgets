# sad - Solana Account Data Deserializer

## Overview
Take any program owned account's data and deserialize it based on descriptors (YAML) and
renders the output in JSON (to screen or file)

This gadget presumes that data was serialized into accounts using `borsh` (little endian)

## Setup
First setup a python virtual environment in this folder
```bash
python3 -m venv env
source env/bin/activate
pip install -r requirements.txt
```

## Execution
```bash
src/sad.py --help
```
## Sample

This [Program](https://github.com/hashblock/solana-cli-program-template) has been
loaded into Solana's 'devnet'

Both of the user accounts ('user1', 'user2') as well as the Programs keys have been copied
to the `../samples/keys` folder

The data descriptor file for the Program account owned data is provided in the `../samples/yamldecls` folder

For the examples output below, the data came from the program in `devnet`, where first a 'mint' occured
on `user1` and then the key/value was transferred to `user2`. The output content may differ by the time you
run this:

Account User 1 data, output defaults to stdout:

```bash
src/sad.py account -s user1

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
src/sad.py account -s user2
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
