use {
    solana_client::rpc_client::RpcClient,
    solana_sdk::{account::Account, commitment_config::CommitmentConfig, pubkey::Pubkey},
};

/// gadgets common solana transactions
/// Checks for existence of account

pub fn account_for_key(
    rpc_client: &RpcClient,
    key: &Pubkey,
    commitment_config: CommitmentConfig,
) -> Option<Account> {
    rpc_client
        .get_account_with_commitment(key, commitment_config)
        .unwrap()
        .value
}
