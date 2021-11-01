# sad - Solana Account Data Deserializer

__*Work in Progress (WIP)!*__ ...

## Overview
The idea is to be able to take any program owned account's data and deserialize it based
on descriptors (YAML)

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
## Example

See https://github.com/hashblock/solana-cli-program-template

One of the user accounts has been copied locally in the `keys` folder:

*keys/user1_account.json*

The data descriptor file for the sample program is provided in the `descriptors` folder:

*descriptors/SampGgdt3wioaoMZhC6LTSbg4pnuvQnSfJpDYeuXQBv.yml*

1. To run the example, first follow instructions for 'minting' in the above repo using the `solana-test-validator`
2. From this folder:
```bash
src/sad.py -k keys/user1_account.json -d descriptors/SampGgdt3wioaoMZhC6LTSbg4pnuvQnSfJpDYeuXQBv.yml
>> [True, 37. {'AKey': 'Minted key value pair'}]
```
Depending on what you minted (key/value) your mileage may vary
