#!/usr/bin/env python3
# -*- coding: utf-8; py-indent-offset:4 -*-
"""sad

Command line tool that takes a binary data string from Solana
account along with a yaml file describing the data and
generates a deserialized output"""

import io
from cmdline import sad_command_line
from sad_account import SadAccountInfo


def main():
    try:
        setup = sad_command_line()
        # Deserialize
        accounts = None
        if setup['action'] == 'account':
            accounts = SadAccountInfo.single_account(
                setup['client'], setup['public_key'], setup['confirmation'])
        else:
            accounts = SadAccountInfo.program_accounts(
                setup['client'], setup['public_key'], setup['confirmation'])
        if accounts:
            for acc in accounts:
                if acc.data:
                    print(
                        f"Account: {acc.account_key} {setup['data_desc'].deser(io.BytesIO(acc.data))}")
                else:
                    print(f"Empty data for {setup['public_key']}")
    except Exception as e:
        print(f"Terminating due to exception {e}")
    return 0


if __name__ == "__main__":
    main()
