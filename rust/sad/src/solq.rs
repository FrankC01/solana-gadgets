//! @brief Solana queries

use {
    crate::{
        datamap::SadValue,
        desertree::Deseriaizer,
        errors::{SadAccountErrorType, SadAccountResult},
    },
    solana_client::rpc_client::RpcClient,
    solana_sdk::{
        account::{Account, ReadableAccount},
        pubkey::Pubkey,
    },
};

/// Retrieves a single account from RPC cluster
///
/// Presumes that the key is a program owned account
pub fn solana_account(rpc_client: &RpcClient, key: &Pubkey) -> SadAccountResult<Account> {
    match rpc_client.get_account(key) {
        Ok(acc) => Ok(acc),
        Err(e) => {
            eprintln!("{}", e);
            Err(SadAccountErrorType::FailedAccountGet)
        }
    }
}

/// Retrieves a list of accounts from RPC cluster
///
/// Presumes that the key is the Program key for which
/// multiple Program Owned Accounts exist
pub fn solana_program_accounts(
    rpc_client: &RpcClient,
    key: &Pubkey,
) -> SadAccountResult<Vec<Account>> {
    match rpc_client.get_program_accounts(key) {
        Ok(vec) => {
            let mut ovec = Vec::<Account>::new();
            for ivec in vec {
                ovec.push(ivec.1)
            }
            Ok(ovec)
        }
        Err(e) => {
            eprintln!("{}", e);
            Err(SadAccountErrorType::FailedProgramAccountGet)
        }
    }
}

/// Deserialize a single Account
pub fn deserialize_account(
    rpc_client: &RpcClient,
    key: &Pubkey,
    destree: &Deseriaizer,
) -> SadAccountResult<Vec<SadValue>> {
    let solacc = solana_account(rpc_client, key)?;
    match destree.deser(&mut solacc.data()) {
        Ok(res) => Ok(res),
        Err(_) => todo!(),
    }
}

/// Deserialize all Program Owned Accounts
pub fn deserialize_program_accounts(
    rpc_client: &RpcClient,
    key: &Pubkey,
    destree: &Deseriaizer,
) -> SadAccountResult<Vec<Vec<SadValue>>> {
    let solacc = solana_program_accounts(rpc_client, key)?;
    let mut ovec = Vec::<Vec<SadValue>>::new();
    for acc in solacc {
        match destree.deser(&mut acc.data()) {
            Ok(res) => ovec.push(res),
            Err(_) => todo!(),
        }
    }
    Ok(ovec)
}
