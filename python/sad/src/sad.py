#!/usr/bin/env python3
# -*- coding: utf-8; py-indent-offset:4 -*-
"""sad

Command line tool that takes a binary data string from Solana
account along with a yaml file describing the data and
generates a deserialized output"""

import io
import base64
from cmdline import sad_cmd_parser
from config import Config
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


def public_key_from_file(file_name: str) -> PublicKey:
    """Creates public key from keypair file"""
    return keypair_from_file(file_name).public_key


def public_key_from_str(in_str: str) -> PublicKey:
    """Creates public key from string"""
    return PublicKey(in_str)


def account_info(client: Client, pubkey: PublicKey, confirmation: str) -> RPCResponse:
    """Fetch account information"""
    return client.get_account_info(pubkey, confirmation)


def data_from_base64(client: Client, pubkey: PublicKey, confirmation: str) -> bytes:
    """Fetches account info with data decoded into bytes"""
    try:
        acc_info = account_info(client, pubkey, confirmation)
        return base64.urlsafe_b64decode(acc_info['result']['value']['data'][0])
    except Exception:
        print(f"RCP Connection error. Make sure you have access to Solana")
        return base64.urlsafe_b64decode("")


def main():
    cfg = Config()
    args = sad_cmd_parser(cfg).parse_args()

    try:
        # Account's public key
        if args.acc:
            public_key_from_str(args.acc)
        else:
            pubkey = public_key_from_file(args.key)

        # RPC URL to communicate with
        client = Client(args.url)

        dd = load_deserializer(Path(args.decl))
        decodedBytes = data_from_base64(client, pubkey, args.conf)
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
