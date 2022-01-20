#!/usr/bin/env python3
# -*- coding: utf-8; py-indent-offset:4 -*-
"""sad

Command line tool that takes a binary data string from Solana
account along with a yaml file describing the data and
generates a deserialized output"""

from cmdline import sad_command_line
import io
import json
from sad_account import SadAccountInfo


def main():
    try:
        setup = sad_command_line()

        if setup['version']:
            print("sad 0.1.0")
            return 0
        # Deserialize
        accounts = None
        # If this is for a single account, the public_key is
        # for the account
        if setup['action'] == 'account':
            accounts = SadAccountInfo.single_account(
                setup['client'], setup['public_key'], setup['confirmation'])
        # If this is for a program account, the public_key is
        # for the program
        else:
            accounts = SadAccountInfo.program_accounts(
                setup['client'], setup['public_key'], setup['confirmation'])
        if accounts:
            acc_list = []
            for acc in accounts:
                if acc.data:
                    acc_list.append(setup['data_desc'].deser(
                        str(acc.account_key), str(acc.owner_key), io.BytesIO(acc.data)))
                else:
                    print(f"Empty data for {setup['public_key']}")

            if setup['output_type'] == 'stdout':
                print(json.dumps(acc_list, indent=2))
            else:
                json.dump(acc_list, setup['output_file'], indent=2)
                setup['output_file'].close()

    except Exception as e:
        print(f"Exception {e}")
    return 0


if __name__ == "__main__":
    main()
