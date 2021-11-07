"""command line argument parser
sad <OPTIONS> account <-k keyfile/-s pubkey> <-o outfile> <-f format>
sad <OPTIONS> program <-k keyfile/-s pubkey> <-o outfile> <-f format>
"""

import argparse
from config import Config


def sad_cmd_parser(in_cfg: Config):
    parser = argparse.ArgumentParser(
        formatter_class=argparse.ArgumentDefaultsHelpFormatter,
        description="Solana Account Decoder")

    subparsers = parser.add_subparsers(
        help='Data from', dest='action')
    parent_parser = argparse.ArgumentParser(add_help=False)

    parent_parser.add_argument('-d', '--declfile',
                               help='YAML data declaration file',
                               required=True,
                               dest='decl',
                               action='store')
    parent_parser.add_argument('-u', '--url',
                               help="Override Solana config JSON RPC URL",
                               required=False,
                               default=in_cfg.rpc_url,
                               dest='url',
                               action='store')
    parent_parser.add_argument('-c', '--conf',
                               help="Override Solana config confirmation level",
                               required=False,
                               default=in_cfg.commitment,
                               choices=['processed', 'confirmed', 'finalized'],
                               dest='conf',
                               action='store')
    key_group = parent_parser.add_mutually_exclusive_group()
    key_group.add_argument('-k',
                           "--keyfile",
                           help="Account or program keyfile",
                           required=False,
                           default=in_cfg.default_keypair,
                           dest='keyfile',
                           action='store')
    key_group.add_argument('-p',
                           "--pubkey",
                           help="Account or program publickey string",
                           required=False,
                           dest='pkstr',
                           action='store')
    parser_account = subparsers.add_parser("account", parents=[parent_parser],
                                           help='Single account')
    parser_program = subparsers.add_parser("program", parents=[parent_parser],
                                           help='Program owned accounts')
    return parser
