"""config

Attempts to fetch `solana config` from underlying system
"""
from subprocess import Popen, PIPE


class Config():
    """Encapsulates the solana config settings"""

    # Solana CLI command line
    _SOLANA_CONFIG_CMDLINE = 'solana config get'

    # URL For Solana Networks
    _SAD_URL_LOOKUP = {
        "localhost": "http://localhost:8899",
        "devnet": "https://api.devnet.solana.com",
        "testnet": "https://api.testnet.solana.com",
        "mainnet": "https://api.mainnet-beta.solana.com"
    }

    def __init__(self) -> None:
        self._config_file = None
        self._rpc_url = None
        self._rpc_name = None
        self._websocket_url = None
        self._default_keypair = None
        self._commitment = None
        self._solana_config()

    def _solana_config(self) -> None:
        """Internal config loader subprocess"""
        cmd = self._SOLANA_CONFIG_CMDLINE.split()
        process = Popen(cmd,
                        shell=False,
                        stdout=PIPE,
                        stderr=PIPE,
                        text=True,
                        universal_newlines=True,)
        stdout, stderr = process.communicate()

        if process.returncode > 0:
            raise ValueError(stderr)

        solana_config = dict([tuple(y for y in x.split(':', 1))
                              for x in stdout.splitlines()])
        self._config_file = solana_config['Config File'].strip()
        self._rpc_url = solana_config['RPC URL'].strip()
        # list out keys and values separately
        key_list = list(self._SAD_URL_LOOKUP.keys())
        val_list = list(self._SAD_URL_LOOKUP.values())
        self._rpc_name = key_list[val_list.index(self._rpc_url)]
        self._websocket_url = solana_config['WebSocket URL'].strip().split(' ')[
            0]
        self._default_keypair = solana_config['Keypair Path'].strip()
        self._commitment = solana_config['Commitment'].strip()

    @ property
    def config_file(self) -> str:
        return self._config_file

    @ property
    def rpc_url(self) -> str:
        return self._rpc_url

    @ property
    def rpc_name(self) -> str:
        return self._rpc_name

    def rpc_names(self) -> list:
        return self._SAD_URL_LOOKUP.keys()

    def rpc_for_name(self, name: str) -> str:
        return self._SAD_URL_LOOKUP[name]

    @ property
    def websocket_url(self) -> str:
        return self._websocket_url

    @ property
    def default_keypair(self) -> str:
        return self._default_keypair

    @ property
    def commitment(self) -> str:
        return self._commitment


if __name__ == "__main__":
    for x, y in Config().__dict__.items():
        print(f"Field {x} = {y}")
