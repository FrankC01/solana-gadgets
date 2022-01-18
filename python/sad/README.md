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
>> Account: A94wMjV54C8f8wn7zL8TxNCdNiGoq7XSN7vWGrtd4vwU [True, 32, {'ts key': 'ts first value'}]
```

Account User 2 data, output defaults to stdout:

```bash
src/sad.py account -s user2
>> Account: 5gMsBeLmPkwEKQ1H2AwceAPasXLyZ4tvWGCYR59qf47U [True, 27, {'Dev1': 'A new value'}]
```

To see all accounts owned by a program:
```bash
src/sad.py program -s prog
>> Account: A94wMjV54C8f8wn7zL8TxNCdNiGoq7XSN7vWGrtd4vwU [True, 32, {'ts key': 'ts first value'}]
>> Account: 5gMsBeLmPkwEKQ1H2AwceAPasXLyZ4tvWGCYR59qf47U [True, 27, {'Dev1': 'A new value'}]
```
Depending on what you minted (key/value) your mileage may vary
