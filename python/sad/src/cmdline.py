"""command line argument parser
sad <OPTIONS> account <-k keyfile/-s pubkey> <-o output_type> <-f filename>
sad <OPTIONS> program <-k keyfile/-s pubkey> <-o output_type> <-f filename>
"""

import argparse
from config import Config
from datadeser import deserializer
from pathlib import Path
from solana.keypair import Keypair
from solana.publickey import PublicKey
from solana.rpc.api import Client
import sys
from typing import Any


def _keypair_from_file(key_pair_file: str) -> Keypair:
    """Returns a Solana KeyPair from a file"""
    with open(key_pair_file) as kpf:
        keypair = kpf.read()
        keypair = keypair.replace('[', '').replace(']', '')
        keypair = list(keypair.split(','))
        keypair = [int(i) for i in keypair]
    return Keypair(keypair[:32])


def _public_key_from_file(file_name: str) -> PublicKey:
    """Creates Solana public key from keypair file"""
    return _keypair_from_file(file_name).public_key


def _public_key_from_str(in_str: str) -> PublicKey:
    """Creates Solana public key from string"""
    return PublicKey(in_str)

# argparse


def _sad_cmd_parser(in_cfg: Config):
    """Build command line parser"""

    parser = argparse.ArgumentParser(
        formatter_class=argparse.ArgumentDefaultsHelpFormatter,
        description="Solana Account Decoder")

    parser.add_argument('-V', '--version',
                        help='Prints version information',
                        action='store_true')

    subparsers = parser.add_subparsers(
        help='Data from', dest='action')
    parent_parser = argparse.ArgumentParser(add_help=False)

    parent_parser.add_argument('-o', '--output',
                               help='Sets output',
                               choices=['stdout', 'json'],
                               default='stdout',
                               dest='output',
                               required=False,
                               action='store')

    parent_parser.add_argument('-f', '--filename',
                               help='Set filename for output if json',
                               dest='filename',
                               required=False,
                               action='store')

    parent_parser.add_argument('-d', '--declfile',
                               help='YAML data declaration file',
                               required=False,
                               dest='decl',
                               action='store')
    parent_parser.add_argument('-u', '--url',
                               help="Override Solana config JSON RPC URL",
                               required=False,
                               choices=in_cfg.rpc_names(),
                               default=in_cfg.rpc_name,
                               dest='url',
                               action='store')
    parent_parser.add_argument('-c', '--conf',
                               help="Override Solana config confirmation level",
                               required=False,
                               default=in_cfg.commitment,
                               choices=['processed', 'confirmed', 'finalized'],
                               dest='conf',
                               action='store')
    key_group = parent_parser.add_mutually_exclusive_group(required=True)
    key_group.add_argument('-k',
                           "--keyfile",
                           help="Account or program keyfile",
                           dest='keyfile',
                           action='store')
    key_group.add_argument('-p',
                           "--pubkey",
                           help="Account or program publickey string",
                           dest='pkstr',
                           action='store')
    key_group.add_argument('-s',
                           "--samplekey",
                           help="Account or program sample name",
                           choices=['user1', 'user2', 'prog'],
                           dest='sampkey',
                           action='store')
    parser_account = subparsers.add_parser("account", parents=[parent_parser],
                                           help='Deserialize a ingle account')
    parser_program = subparsers.add_parser("program", parents=[parent_parser],
                                           help='Deserialize all program owned accounts')
    return parser

# Sample data definitions


_SAMPLE_KEY_MAP = {
    "user1": "../../samples/keys/user1_account.json",
    "user2": "../../samples/keys/user2_account.json",
    "prog": "../../samples/keys/SampGgdt3wioaoMZhC6LTSbg4pnuvQnSfJpDYeuXQBv.json"
}

_SAMP_DESC = "../../samples/yamldecls/SampGgdt3wioaoMZhC6LTSbg4pnuvQnSfJpDYeuXQBv.yml"


def sad_command_line() -> dict:
    """
    Parse command line and fixup arguments
    @returns dict
    """
    cfg = Config()
    cmd_target = dict()
    cmdparser = _sad_cmd_parser(cfg)
    args = cmdparser.parse_args(args=None if sys.argv[1:] else ['--help'])

    # print(args)

    # Version is quick exit
    if args.version:
        cmd_target['version'] = True
        return cmd_target
    else:
        cmd_target['version'] = False

    # Check that appropriate key is provided given the context
    # Account's public key
    pubkey = None
    if args.pkstr:
        pubkey = _public_key_from_str(args.pkstr)
    elif args.sampkey:
        pubkey = _public_key_from_file(_SAMPLE_KEY_MAP[args.sampkey])
    else:
        pubkey = _public_key_from_file(args.keyfile)

    cmd_target['public_key'] = pubkey

    # Do a URL swap if needed
    url = None
    if args.url is not None:
        url = cfg.rpc_for_name(args.url)
    else:
        raise ValueError("Expected '-u' or '-url' argument")
    cmd_target['client'] = Client(url)

    # Confirmation
    cmd_target['confirmation'] = args.conf

    # Descriptor
    if args.sampkey is not None and args.decl is None:
        cmd_target['data_desc'] = deserializer(Path(_SAMP_DESC))
    else:
        cmd_target['data_desc'] = deserializer(Path(args.decl))

    # Action
    cmd_target['action'] = args.action

    cmd_target['output_type'] = 'stdout'
    cmd_target['output_file'] = None

    # Output
    if args.output == 'json':
        if args.filename is None:
            raise ValueError(
                "Expected '-f' or '-filename' argument when using -o json")
        else:
            cmd_target['output_type'] = 'json'
            cmd_target['output_file'] = args.filename

    return cmd_target


if __name__ == "__main__":
    pass
