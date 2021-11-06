import argparse
from config import Config


def sad_cmd_parser(in_cfg: Config):
    parser = argparse.ArgumentParser(
        formatter_class=argparse.ArgumentDefaultsHelpFormatter,
        description="Solana Account Decoder")
    parser.add_argument('-d', '--decl',
                        help='YAML data declaration file',
                        required=True,
                        dest='decl',
                        action='store')
    parser.add_argument('-u', '--url',
                        help="Override Solana config JSON RPC URL",
                        required=False,
                        default=in_cfg.rpc_url,
                        dest='url',
                        action='store')
    parser.add_argument('-c', '--conf',
                        help="Override Solana config commitment level",
                        required=False,
                        default=in_cfg.commitment,
                        choices=['processed', 'confirmed', 'finalized'],
                        dest='conf',
                        action='store')
    key_group = parser.add_mutually_exclusive_group()
    key_group.add_argument('-k',
                           "--key",
                           help="Override Solana configs default keypair",
                           required=False,
                           default=in_cfg.default_keypair,
                           dest='key',
                           action='store')
    key_group.add_argument('-a',
                           "--acc",
                           help="Account public key string",
                           required=False,
                           dest='acc',
                           action='store')
    granularity_group = parser.add_mutually_exclusive_group()
    granularity_group.add_argument('-p',
                                   "--program",
                                   help="Fetch all program accounts",
                                   required=False,
                                   dest='program',
                                   action='store_true')
    granularity_group.add_argument('-s',
                                   "--single",
                                   help="Fetches one program owned account",
                                   required=False,
                                   dest='single',
                                   action='store_true')
    return parser
