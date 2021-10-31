#!/usr/bin/env python3
# -*- coding: utf-8; py-indent-offset:4 -*-
"""sad

Command line tool that takes a binary data string from Solana
account along with a yaml file describing the data and
generates a deserialized output"""

import io
import base64
from borsh_construct import U8, String, U32, Bool, HashMap
from cmdline import sad_cmd_parser
from datadeser import load_deserializer
from pathlib import Path
from solana.keypair import Keypair
from solana.publickey import PublicKey
from solana.rpc.api import Client
from solana.rpc.types import RPCResponse


def get_keypair_from_file(key_pair_file: str) -> Keypair:
    """Returns a KeyPair from a file"""
    with open(key_pair_file) as kpf:
        keypair = kpf.read()
        keypair = keypair.replace('[', '').replace(']', '')
        keypair = list(keypair.split(','))
        keypair = [int(i) for i in keypair]
    return Keypair(keypair[:32])


def get_account_info(client: Client, pubkey: PublicKey) -> RPCResponse:
    """Fetch account information"""
    return client.get_account_info(pubkey, 'configrmed')


def get_account_data_from_base64(client: Client, pubkey: PublicKey) -> bytes:
    """Fetches account info with data decoded into bytes"""
    try:
        acc_info = get_account_info(client, pubkey)
        return base64.urlsafe_b64decode(acc_info['result']['value']['data'][0])
    except Exception:
        print(f"RCP Connection error. Make sure you have access to Solana")
        # print(f"{repr(re)}")
        return base64.urlsafe_b64decode("")


def cmdline_get_public_key(args) -> PublicKey:
    """Returns a Public from either a keyfile or str on the command line"""
    return get_keypair_from_file(args.keypair).public_key


def main(args):
    # print(f"Args = {args}")

    try:
        pubkey = cmdline_get_public_key(args)
        client = Client("http://localhost:8899")
        dd = load_deserializer(Path(args.data_decl))
        dd.describe()

        # decodedBytes = get_account_data_from_base64(client, pubkey)
        pacc = 'ASUAAAABAAAABAAAAEFLZXkVAAAATWludGVkIGtleSB2YWx1ZSBwYWlyAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA=='
        decodedBytes = base64.urlsafe_b64decode(pacc)
        if decodedBytes:
            mystream = io.BytesIO(decodedBytes)
            print(dd.deser(mystream))
            # print(f"Initialized = {Bool.parse_stream(mystream)}")
            # print(f"CSP {mystream.tell()}")
            # y = U32.parse_stream(mystream)
            # print(f"CSP {mystream.tell()}")

            # print(
            #     f"{HashMap(String, String).parse(mystream.read1(y+mystream.tell()))}")
            # print(f"CSP {mystream.tell()}")
            # y = U32.parse_stream(mystream)
            # print(y)
            # print(f"CSP {mystream.tell()}")
            # mystream.seek(y+5, 1)
            # print(x)
            # print(f"Map = {myhm.parse(x)}")
        else:
            print(f"Empty data for {pubkey}")
    except Exception as e:
        print(f"Terminating due to exception {e}")
    return 0


if __name__ == "__main__":
    main(sad_cmd_parser().parse_args())
