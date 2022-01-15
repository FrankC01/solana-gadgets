//! @brief Solana queries

use {
    crate::{
        desertree::Deseriaizer,
        errors::{SadAccountErrorType, SadAccountResult},
        sadtypes::SadValue,
    },
    solana_client::rpc_client::RpcClient,
    solana_sdk::{
        account::{Account, ReadableAccount},
        pubkey::Pubkey,
    },
};

/// Identifies type of processing for deserialization
#[derive(Debug, PartialEq)]
pub enum ResultForKeyType {
    SingleAccount,
    ProgramAccount(Pubkey),
}

/// Context of deserialization
#[derive(Debug)]
pub struct AccountResultContext {
    key: Pubkey,
    account: Account,
    deserialized: Vec<SadValue>,
}

impl AccountResultContext {
    pub fn new(pkey: Pubkey, acc: Account, deser: Vec<SadValue>) -> Self {
        Self {
            key: pkey,
            account: acc,
            deserialized: deser,
        }
    }

    pub fn pubkey(&self) -> &Pubkey {
        &self.key
    }

    pub fn account(&self) -> &Account {
        &self.account
    }

    pub fn deserialize_list(&self) -> &Vec<SadValue> {
        &self.deserialized
    }
}

/// Generalized deserialization result
#[derive(Debug)]
pub struct DeserializationResult {
    account_type: ResultForKeyType,
    contexts: Vec<AccountResultContext>,
}

impl DeserializationResult {
    pub fn account_type(&self) -> &ResultForKeyType {
        &self.account_type
    }

    pub fn context_count(&self) -> usize {
        self.contexts.len()
    }

    pub fn context_vec(&self) -> &Vec<AccountResultContext> {
        &self.contexts
    }
}
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
) -> SadAccountResult<Vec<(Pubkey, Account)>> {
    let vaccount = solana_account(rpc_client, key)?;
    if vaccount.executable() != true {
        return Err(SadAccountErrorType::NotProgramKeyError);
    }
    match rpc_client.get_program_accounts(key) {
        Ok(vec) => Ok(vec),
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
) -> SadAccountResult<DeserializationResult> {
    let solacc = solana_account(rpc_client, key)?;
    if solacc.executable() == true {
        return Err(SadAccountErrorType::AccountIsExecutableError);
    }
    let mut resvec = Vec::<AccountResultContext>::new();
    match destree.deser(&mut solacc.data()) {
        Ok(res) => {
            resvec.push(AccountResultContext::new(key.clone(), solacc, res));
            Ok(DeserializationResult {
                account_type: ResultForKeyType::SingleAccount,
                contexts: resvec,
            })
        }
        Err(_) => todo!(),
    }
}

/// Deserialize all Program Owned Accounts
pub fn deserialize_program_accounts(
    rpc_client: &RpcClient,
    key: &Pubkey,
    destree: &Deseriaizer,
) -> SadAccountResult<DeserializationResult> {
    let solacc = solana_program_accounts(rpc_client, key)?;
    let mut resvec = Vec::<AccountResultContext>::new();
    for acc in solacc {
        // println!("{:?}", encode(acc.1.data()));
        match destree.deser(&mut acc.1.data()) {
            Ok(res) => resvec.push(AccountResultContext::new(acc.0.clone(), acc.1, res)),
            Err(_) => todo!(),
        }
    }
    Ok(DeserializationResult {
        account_type: ResultForKeyType::ProgramAccount(key.clone()),
        contexts: resvec,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::str::FromStr;

    use gadgets_common::load_yaml_file;
    use solana_cli_config::*;
    // Presume solana-cli-program accounts
    const SCLI: &str = "../../samples/yamldecls/SampGgdt3wioaoMZhC6LTSbg4pnuvQnSfJpDYeuXQBv.yml";

    fn get_config_rpcclient() -> SadAccountResult<(Config, RpcClient)> {
        if let Some(x) = &*CONFIG_FILE {
            let cfg = match Config::load(&x) {
                Ok(config) => config,
                Err(e) => {
                    eprintln!("{}", e);
                    return Err(SadAccountErrorType::ConfigFileError);
                }
            };
            let rpc_client = RpcClient::new(cfg.json_rpc_url.clone());
            Ok((cfg, rpc_client))
        } else {
            Err(SadAccountErrorType::ConfigFileError)
        }
    }

    #[test]
    fn test_fetch_pda_pass() {
        let (_, rpc_client) = get_config_rpcclient().unwrap();
        // Presume solana-cli-program accounts are created and run either locally or devnet
        let pubkey = Pubkey::from_str("SampGgdt3wioaoMZhC6LTSbg4pnuvQnSfJpDYeuXQBv").unwrap();
        let pda = Pubkey::create_program_address(&[b"test"], &pubkey).unwrap();
        println!("PDA PubKey {:?}", pda);
        println!("{:?}", rpc_client.get_account(&pda));
    }

    #[test]
    fn test_fetch_singleaccount_pass() {
        let (_, rpc_client) = get_config_rpcclient().unwrap();
        // Presume solana-cli-program accounts are created and run either locally or devnet
        let pubkey = Pubkey::from_str("5gMsBeLmPkwEKQ1H2AwceAPasXLyZ4tvWGCYR59qf47U").unwrap();
        let x = solana_account(&rpc_client, &pubkey);
        assert!(x.is_ok());
        println!("{:?}", x.unwrap());
    }
    #[test]
    fn test_fetch_programaccounts_pass() {
        let (_, rpc_client) = get_config_rpcclient().unwrap();
        // Presume solana-cli-program accounts are created and run either locally or devnet
        let pubkey = Pubkey::from_str("SampGgdt3wioaoMZhC6LTSbg4pnuvQnSfJpDYeuXQBv").unwrap();
        // let pubkey = Pubkey::from_str("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA").unwrap();
        let x = solana_program_accounts(&rpc_client, &pubkey);
        assert!(x.is_ok());
        println!("{:?}", x.unwrap());
    }

    #[test]
    fn test_deserialize_singleaccount_pass() {
        let (_, rpc_client) = get_config_rpcclient().unwrap();
        // Presume solana-cli-program accounts are created and run either locally or devnet
        let pubkey = Pubkey::from_str("5gMsBeLmPkwEKQ1H2AwceAPasXLyZ4tvWGCYR59qf47U").unwrap();
        let yamldecl = load_yaml_file(SCLI).unwrap();
        let deser =
            deserialize_account(&rpc_client, &pubkey, &Deseriaizer::new(&yamldecl[0])).unwrap();
        assert_eq!(deser.context_count(), 1);
        assert_eq!(deser.account_type(), &ResultForKeyType::SingleAccount);
        let oneresult = deser.context_vec().first().unwrap();
        assert_eq!(oneresult.pubkey(), &pubkey);
        println!("{:?}", oneresult.deserialize_list());
    }
    #[test]
    fn test_deserialize_programaccount_pass() {
        let (_, rpc_client) = get_config_rpcclient().unwrap();
        // Presume solana-cli-program accounts are created and run either locally or devnet
        let pubkey = Pubkey::from_str("SampGgdt3wioaoMZhC6LTSbg4pnuvQnSfJpDYeuXQBv").unwrap();
        let onekey = Pubkey::from_str("A94wMjV54C8f8wn7zL8TxNCdNiGoq7XSN7vWGrtd4vwU").unwrap();
        let twokey = Pubkey::from_str("5gMsBeLmPkwEKQ1H2AwceAPasXLyZ4tvWGCYR59qf47U").unwrap();
        let yamldecl = load_yaml_file(SCLI).unwrap();
        let data_declaration = Deseriaizer::new(&yamldecl[0]);
        let deser = deserialize_program_accounts(&rpc_client, &pubkey, &data_declaration).unwrap();
        assert_eq!(deser.context_count(), 2);
        assert_eq!(
            deser.account_type(),
            &ResultForKeyType::ProgramAccount(pubkey)
        );

        let key_list = [&onekey, &twokey];
        let cvec = deser.context_vec();
        for i in 0..2 {
            assert_eq!(
                key_list.iter().find(|x| *x == &cvec[i].pubkey()),
                Some(&cvec[i].pubkey())
            );
        }
        for c in deser.context_vec() {
            println!("HL Elements {}", c.deserialize_list().len());
            println!("Data {:?}", c.deserialize_list());
        }
    }
}
