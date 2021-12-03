#!/usr/bin/env python3
# -*- coding: utf-8; py-indent-offset:4 -*-
"""sad

Command line tool that takes a binary data string from Solana
account along with a yaml file describing the data and
generates a deserialized output"""

import io
from cmdline import sad_cmd_parser
from config import Config
from datadeser import deserializer
from pathlib import Path
from sad_account import SadAccountInfo
from solana.keypair import Keypair
from solana.publickey import PublicKey
from solana.rpc.api import Client


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


def main():
    cfg = Config()
    args = sad_cmd_parser(cfg).parse_args()
    try:
        # Account's public key
        pubkey = None
        if not args.pkstr and not args.keyfile:
            raise Exception("Must specify either'-k' or '-p' arguments")
        if args.pkstr:
            pubkey = public_key_from_str(args.pkstr)
        else:
            pubkey = public_key_from_file(args.keyfile)

        # RPC URL to communicate with
        client = Client(args.url)
        # Data deserializer declarations
        data_desc = deserializer(Path(args.decl))
        accounts = None
        if args.action == 'account':
            accounts = SadAccountInfo.single_account(client, pubkey, args.conf)
        else:
            accounts = SadAccountInfo.program_accounts(
                client, pubkey, args.conf)
        if accounts:
            for acc in accounts:
                if acc.data:
                    print(
                        f"Account: {acc.account_key} {data_desc.deser(io.BytesIO(acc.data))}")
                else:
                    print(f"Empty data for {pubkey}")
    except Exception as e:
        print(f"Terminating due to exception {e}")
    return 0


if __name__ == "__main__":
    main()
