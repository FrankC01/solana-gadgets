"""sad_account

Class amd functions to call and process base64 data results"""

import base64
from solana.publickey import PublicKey
from solana.rpc.api import Client


class SadAccountInfo(object):

    def __init__(self, in_dict: dict, pubkey: PublicKey) -> None:
        self._account_key = pubkey
        self._owner = PublicKey(in_dict['owner'])
        self._lamports = in_dict['lamports']
        self._data = self._from_base64(in_dict['data'])
        self._executable = in_dict['executable']

    @property
    def account_key(self) -> PublicKey:
        return self._account_key

    @property
    def owner_key(self) -> PublicKey:
        return self._owner

    @property
    def lamports(self) -> int:
        return self._lamports

    @property
    def data(self) -> bytes:
        return self._data

    @property
    def executable(self) -> bool:
        return self._executable

    @classmethod
    def _from_base64(cls, data: list) -> bytes:
        """convert base64 string to bytes"""
        return base64.urlsafe_b64decode(data[0])

    @classmethod
    def _from_single(cls, data: dict, acc_key: PublicKey) -> list:
        """convert single base64 result map to a single SadAccountInfo object"""
        return [SadAccountInfo(data, acc_key)]

    @classmethod
    def _from_multiple(cls, data: list) -> list:
        """convert base64 result list to SadAccountInfo objects"""
        return [SadAccountInfo(x['account'], PublicKey(x['pubkey'])) for x in data]

    @classmethod
    def accounts_from(cls, rpc_result: dict, acc_key=None) -> list:
        """process rpc result to list of account info objects"""
        data = rpc_result['result']
        if isinstance(data, dict):
            return cls._from_single(data['value'], acc_key)
        elif isinstance(data, list):
            return cls._from_multiple(data)
        else:
            raise AttributeError(f'Unknown RPC result {data}')

    @classmethod
    def single_account(cls, client: Client, pubkey: PublicKey, confirmation: str) -> list:
        """fetch single account information"""
        res = client.get_account_info(pubkey, confirmation, encoding='base64')
        return cls.accounts_from(res, pubkey)

    @classmethod
    def program_accounts(cls, client: Client, prod_pk: PublicKey, confirmation: str) -> list:
        """gets all accounts associated to program key"""
        res = client.get_program_accounts(
            prod_pk, confirmation, encoding='base64')
        return cls.accounts_from(res)


if __name__ == "__main__":
    pass
    # Move this to integration testing
    # cfg = Config()
    # client = Client(cfg.rpc_url)
    # result = SadAccountInfo.single_account(client, PublicKey(
    #     '5gMsBeLmPkwEKQ1H2AwceAPasXLyZ4tvWGCYR59qf47U'), cfg.commitment)
    # print(result)
    # result = SadAccountInfo.program_accounts(client, PublicKey(
    #     'SampGgdt3wioaoMZhC6LTSbg4pnuvQnSfJpDYeuXQBv'), cfg.commitment)
    # print(result)
