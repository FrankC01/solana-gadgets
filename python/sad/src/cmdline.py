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
    group = parser.add_mutually_exclusive_group()
    group.add_argument('-k',
                       "--key",
                       help="Override Solana configs default keypair",
                       required=False,
                       default=in_cfg.default_keypair,
                       dest='key',
                       action='store')
    group.add_argument('-a',
                       "--acc",
                       help="Account public key string",
                       required=False,
                       dest='acc',
                       action='store')
    return parser
