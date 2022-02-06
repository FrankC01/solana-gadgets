#[cfg(test)]
mod tests {
    use std::{error, path::PathBuf, str::FromStr};

    use gadgets_scfs10::{ScfsCriteria, ScfsMatrix, SCFS_DEVNET};
    use solana_client::rpc_client::RpcClient;
    use solana_program::{instruction::Instruction, message::Message, pubkey::Pubkey};
    use solana_sdk::{
        compute_budget::ComputeBudgetInstruction,
        pubkey,
        signature::{Keypair, Signature},
        signer::Signer,
        transaction::Transaction,
    };
    use solana_test_validator::{TestValidator, TestValidatorGenesis};

    const LEDGER_PATH: &str = "./.ledger";
    const PROG_PATH: &str = "target/deploy/";
    const PROG_NAME: &str = "scfs_program";
    const PROG_KEY: Pubkey = pubkey!("PWDnx8LkjJUn9bAVzG6Fp6BuvB41x7DkBZdo9YLMGcc");

    const TXWIDE_LIMITS: Pubkey = pubkey!("5ekBxc8itEnPv4NzGJtr8BVVQLNMQuLMNQQj7pHoLNZ9");
    const BLAKE3_SYSCALL: Pubkey = pubkey!("HTW2pSyErTj4BV6KBM9NZ9VBUJVxt7sacNWcf76wtzb3");

    /// Setup the test validator with predefined properties
    pub fn setup_validator(
        invalidate_features: Vec<Pubkey>,
    ) -> Result<(TestValidator, Keypair), Box<dyn error::Error>> {
        std::env::set_var("BPF_OUT_DIR", PROG_PATH);
        let mut test_validator = TestValidatorGenesis::default();
        let (test_validator, kp) = test_validator
            .ledger_path(LEDGER_PATH)
            .add_program(PROG_NAME, PROG_KEY)
            .deactivate_features(&invalidate_features)
            .start();
        // test_validator.start_with_mint_address(vwallet.pubkey(), SocketAddrSpace::new(true))?;
        Ok((test_validator, kp))
    }

    /// Submits the program instruction as per the
    /// instruction definition
    fn submit_transaction(
        rpc_client: &RpcClient,
        wallet_signer: &dyn Signer,
        instructions: Vec<Instruction>,
    ) -> Result<Signature, Box<dyn std::error::Error>> {
        let mut transaction =
            Transaction::new_unsigned(Message::new(&instructions, Some(&wallet_signer.pubkey())));
        let recent_blockhash = rpc_client
            .get_latest_blockhash()
            .map_err(|err| format!("error: unable to get recent blockhash: {}", err))?;
        transaction
            .try_sign(&vec![wallet_signer], recent_blockhash)
            .map_err(|err| format!("error: failed to sign transaction: {}", err))?;

        let signature = rpc_client
            .send_and_confirm_transaction(&transaction)
            .map_err(|err| format!("error: send transaction: {}", err))?;
        Ok(signature)
    }

    /// Ensures an empty ledger before setting up the validator
    pub fn clean_ledger_setup_validator(
        invalidate_features: Vec<Pubkey>,
    ) -> Result<(TestValidator, Keypair), Box<dyn error::Error>> {
        if PathBuf::from_str(LEDGER_PATH).unwrap().exists() {
            std::fs::remove_dir_all(LEDGER_PATH).unwrap();
        }
        setup_validator(invalidate_features)
    }

    #[test]
    fn base_test() {
        let inv_feat = vec![];

        let (test_validator, main_payer) = clean_ledger_setup_validator(inv_feat).unwrap();
        let connection = test_validator.get_rpc_client();
        solana_logger::setup_with_default("solana_runtime::message=debug");

        let accounts = &[];
        let txn = submit_transaction(
            &connection,
            &main_payer,
            [
                ComputeBudgetInstruction::request_units(400_000u32),
                Instruction::new_with_borsh(PROG_KEY, &0u8, accounts.to_vec()),
                Instruction::new_with_borsh(PROG_KEY, &1u8, accounts.to_vec()),
            ]
            .to_vec(),
        );
        println!("{:?}", txn);
    }
    #[test]
    fn base_x_transaction_cu_test() {
        let inv_feat = vec![TXWIDE_LIMITS];
        let (test_validator, main_payer) = clean_ledger_setup_validator(inv_feat).unwrap();
        let connection = test_validator.get_rpc_client();
        solana_logger::setup_with_default("solana_runtime::message=debug");

        let accounts = &[];
        let instruction = Instruction::new_with_borsh(PROG_KEY, &0u8, accounts.to_vec());
        let bump_budget = ComputeBudgetInstruction::request_units(400_000u32);
        let txn = submit_transaction(
            &connection,
            &main_payer,
            [bump_budget, instruction.clone(), instruction.clone()].to_vec(),
        );
        println!("{:?}", txn);
    }
    #[test]
    fn base_x_transaction_cu_test_dt() {
        let mut inv_vec = Vec::<Pubkey>::new();
        inv_vec.push(TXWIDE_LIMITS);
        inv_vec.push(BLAKE3_SYSCALL);
        inv_vec.push(TXWIDE_LIMITS); // These are redundant and set collection will eliminate
        inv_vec.push(BLAKE3_SYSCALL);
        inv_vec.push(PROG_KEY); // Not a feature so this will be rejected
        let (test_validator, main_payer) = clean_ledger_setup_validator(inv_vec).unwrap();
        let connection = test_validator.get_rpc_client();
        solana_logger::setup_with_default("solana_runtime::message=debug");

        let accounts = &[];
        let instruction = Instruction::new_with_borsh(PROG_KEY, &0u8, accounts.to_vec());
        let txn = submit_transaction(
            &connection,
            &main_payer,
            [instruction.clone(), instruction.clone()].to_vec(),
        );
        println!("{:?}", txn);
    }

    #[test]
    fn test_devnet_filter_inactive_pass() {
        let mut cluster_vec = Vec::<String>::new();
        cluster_vec.push(SCFS_DEVNET.to_string());
        let mut my_matrix = ScfsMatrix::new(Some(ScfsCriteria {
            clusters: Some(cluster_vec),
            ..Default::default()
        }))
        .unwrap();
        assert!(my_matrix.run().is_ok());
        let inactives = my_matrix
            .get_features(Some(&ScfsMatrix::any_inactive))
            .unwrap();
        assert_ne!(inactives.len(), 0);
        let (test_validator, main_payer) = clean_ledger_setup_validator(inactives).unwrap();
        let connection = test_validator.get_rpc_client();
        solana_logger::setup_with_default("solana_runtime::message=debug");

        let accounts = &[];
        let instruction = Instruction::new_with_borsh(PROG_KEY, &0u8, accounts.to_vec());
        let txn = submit_transaction(
            &connection,
            &main_payer,
            [instruction.clone(), instruction.clone()].to_vec(),
        );
        println!("{:?}", txn);
    }
}
