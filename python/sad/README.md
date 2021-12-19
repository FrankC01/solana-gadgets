# sad - Solana Account Data Deserializer

__*Work in Progress (WIP)!*__ ...

## Overview
The idea is to be able to take any program owned account's data and deserialize it based
on descriptors (YAML)

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
## Sample Examples

The Sample [Program](https://github.com/hashblock/solana-cli-program-template) has been loaded into Solana's 'devnet'

Both of the user accounts ('user1', 'user2') as well as the Programs keys have been copied to the `../samples/keys` folder
The data descriptor file for the Program account owned data is provided in the `../samples/yamldecls` folder

1. For the examples output below, the data came from `devnet` where first a 'mint' occured on `user1` and then the key/value was transferred to `user2`. To run locally, first follow instructions for 'minting' and 'transferring' in the above Program repo

2. To see a single account data, where account key is from keypair file. `user1`:
```bash
src/sad.py account -s user1
>> Account: A94wMjV54C8f8wn7zL8TxNCdNiGoq7XSN7vWGrtd4vwU [True, 32, {'ts key': 'ts first value'}]
```
3. To see `user2` account data where account key is from command line string:
```bash
src/sad.py account -s user2
>> Account: 5gMsBeLmPkwEKQ1H2AwceAPasXLyZ4tvWGCYR59qf47U [True, 27, {'Dev1': 'A new value'}]
```
4. To see all programed owned accounts data where program account key is from command line string:
```bash
src/sad.py program -s prog
>> Account: A94wMjV54C8f8wn7zL8TxNCdNiGoq7XSN7vWGrtd4vwU [True, 32, {'ts key': 'ts first value'}]
>> Account: 5gMsBeLmPkwEKQ1H2AwceAPasXLyZ4tvWGCYR59qf47U [True, 27, {'Dev1': 'A new value'}]
```
Depending on what you minted (key/value) your mileage may vary
