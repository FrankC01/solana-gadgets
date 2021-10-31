import argparse


def sad_cmd_parser():
    parser = argparse.ArgumentParser(
        formatter_class=argparse.ArgumentDefaultsHelpFormatter,
        description="Solana Account Decoder")
    parser.add_argument('-d', '--datadecl',
                        help='YAML data declaration file for deserialization',
                        required=True,
                        dest='data_decl',
                        action='store')
    group = parser.add_mutually_exclusive_group()
    group.add_argument('-k',
                       "--key-pair",
                       help="Account keypair file",
                       required=False,
                       dest='keypair',
                       action='store')
    group.add_argument('-a',
                       "--account",
                       help="Account public key",
                       required=False,
                       dest='account',
                       action='store')
    return parser
