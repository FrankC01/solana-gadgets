#!/usr/bin/env python3
# -*- coding: utf-8; py-indent-offset:4 -*-
"""sad

Command line tool that takes a binary data string from Solana
account along with a yaml file describing the data and
generates a deserialized output"""

import io
import base64
from cmdline import sad_cmd_parser
from datadeser import load_deserializer
from pathlib import Path
from solana.keypair import Keypair
from solana.publickey import PublicKey
from solana.rpc.api import Client
from solana.rpc.types import RPCResponse


def keypair_from_file(key_pair_file: str) -> Keypair:
    """Returns a KeyPair from a file"""
    with open(key_pair_file) as kpf:
        keypair = kpf.read()
        keypair = keypair.replace('[', '').replace(']', '')
        keypair = list(keypair.split(','))
        keypair = [int(i) for i in keypair]
    return Keypair(keypair[:32])


def account_info(client: Client, pubkey: PublicKey) -> RPCResponse:
    """Fetch account information"""
    return client.get_account_info(pubkey, 'configrmed')


def data_from_base64(client: Client, pubkey: PublicKey) -> bytes:
    """Fetches account info with data decoded into bytes"""
    try:
        acc_info = account_info(client, pubkey)
        return base64.urlsafe_b64decode(acc_info['result']['value']['data'][0])
    except Exception:
        print(f"RCP Connection error. Make sure you have access to Solana")
        return base64.urlsafe_b64decode("")


def get_public_key(args) -> PublicKey:
    """Returns a Public from either a keyfile or str on the command line"""
    return keypair_from_file(args.keypair).public_key


def main():
    args = sad_cmd_parser().parse_args()
    try:
        pubkey = get_public_key(args)
        # Make this optional via argparse
        client = Client("http://localhost:8899")
        dd = load_deserializer(Path(args.data_decl))
        decodedBytes = data_from_base64(client, pubkey)
        if decodedBytes:
            mystream = io.BytesIO(decodedBytes)
            print(dd.deser(mystream))
        else:
            print(f"Empty data for {pubkey}")
    except Exception as e:
        print(f"Terminating due to exception {e}")
    return 0


if __name__ == "__main__":
    main()
