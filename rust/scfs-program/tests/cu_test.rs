#[cfg(test)]
mod tests {
    use std::{error, path::PathBuf, str::FromStr};

    // Use gadget-scfs to get interegate feature lists from clusters
    // must have `gadgets-scfs = "0.2.0" in Cargo.toml [dev-dependencies] to use
    use gadgets_scfs::{ScfsCriteria, ScfsMatrix, SCFS_DEVNET};
    use solana_client::rpc_client::RpcClient;
    use solana_program::{instruction::Instruction, message::Message, pubkey::Pubkey};
    use solana_sdk::{
        // Added in Solana 1.9.2
        compute_budget::ComputeBudgetInstruction,
        pubkey,
        signature::{Keypair, Signature},
        signer::Signer,
        transaction::Transaction,
    };
    // Extended in Solana 1.9.6
    use solana_test_validator::{TestValidator, TestValidatorGenesis};

    /// Location/Name of ProgramTestGenesis ledger
    const LEDGER_PATH: &str = "./.ledger";
    /// Path to BPF program (*.so)
    const PROG_PATH: &str = "target/deploy/";
    /// Program name from program Cargo.toml
    /// FILL IN WITH YOUR PROGRAM
    const PROG_NAME: &str = "scfs_program";
    /// Program public key
    /// FILL IN WITH YOUR PROGRAM'S PUBLIC KEY
    const PROG_KEY: Pubkey = pubkey!("PWDnx8LkjJUn9bAVzG6Fp6BuvB41x7DkBZdo9YLMGcc");
    /// 'transaction wide compute cap' public key
    const TXWIDE_LIMITS: Pubkey = pubkey!("5ekBxc8itEnPv4NzGJtr8BVVQLNMQuLMNQQj7pHoLNZ9");

    /// Setup the test validator with predefined properties
    pub fn setup_validator(
        invalidate_features: Vec<Pubkey>,
    ) -> Result<(TestValidator, Keypair), Box<dyn error::Error>> {
        // Extend environment variable to include our program location
        std::env::set_var("BPF_OUT_DIR", PROG_PATH);
        // Instantiate the test validator
        let mut test_validator = TestValidatorGenesis::default();
        // Once instantiated, TestValidatorGenesis configuration functions follow
        // a builder pattern enabling chaining of settings function calls
        let (test_validator, kp) = test_validator
            // Set the ledger path and name
            // maps to `solana-test-validator --ledger <DIR>`
            .ledger_path(LEDGER_PATH)
            // Load our program. Ignored if reusing ledger
            // maps to `solana-test-validator --bpf-program <ADDRESS_OR_PATH BPF_PROGRAM.SO>`
            .add_program(PROG_NAME, PROG_KEY)
            // Identify features to deactivate. Ignored if reusing ledger
            // maps to `solana-test-validator --deactivate-feature <FEATURE_PUBKEY>`
            .deactivate_features(&invalidate_features)
            // Start the test validator
            .start();
        Ok((test_validator, kp))
    }

    /// Convenience function to remove existing ledger before TestValidatorGenesis setup
    /// maps to `solana-test-validator ... --reset`
    pub fn clean_ledger_setup_validator(
        invalidate_features: Vec<Pubkey>,
    ) -> Result<(TestValidator, Keypair), Box<dyn error::Error>> {
        if PathBuf::from_str(LEDGER_PATH).unwrap().exists() {
            std::fs::remove_dir_all(LEDGER_PATH).unwrap();
        }
        setup_validator(invalidate_features)
    }

    /// Submits a transaction with programs instruction
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

    #[test]
    fn test_base_pass() {
        // Run with all features acticated (default for TestValidatorGenesis)
        let inv_feat = vec![];
        // Start validator with clean (new) ledger
        let (test_validator, main_payer) = clean_ledger_setup_validator(inv_feat).unwrap();
        // Get the RpcClient
        let connection = test_validator.get_rpc_client();
        // Capture our programs log statements
        solana_logger::setup_with_default("solana_runtime::message=debug");

        // This example doesn't require sending any accounts to program
        let accounts = &[];
        // Build instruction array and submit transaction
        let txn = submit_transaction(
            &connection,
            &main_payer,
            // Add two (2) instructions to transaction
            // Replace with instructions that make sense for your program
            [
                Instruction::new_with_borsh(PROG_KEY, &0u8, accounts.to_vec()),
                Instruction::new_with_borsh(PROG_KEY, &1u8, accounts.to_vec()),
            ]
            .to_vec(),
        );
        assert!(txn.is_ok());
    }

    #[test]
    fn test_deactivate_tx_cu_pass() {
        // Run with all features acticated except 'transaction wide compute cap'
        let inv_feat = vec![TXWIDE_LIMITS];
        let (test_validator, main_payer) = clean_ledger_setup_validator(inv_feat).unwrap();
        let connection = test_validator.get_rpc_client();
        solana_logger::setup_with_default("solana_runtime::message=debug");
        let accounts = &[];
        let txn = submit_transaction(
            &connection,
            &main_payer,
            [
                // This CU transaction budget instruction does nothing when we deactivate the feature
                ComputeBudgetInstruction::set_compute_unit_limit(400_000u32),
                // Add two (2) instructions to transaction
                // Replace with instructions that make sense for your program
                Instruction::new_with_borsh(PROG_KEY, &0u8, accounts.to_vec()),
                Instruction::new_with_borsh(PROG_KEY, &1u8, accounts.to_vec()),
            ]
            .to_vec(),
        );
        assert!(txn.is_ok());
    }

    #[test]
    fn test_devnet_parity_pass() {
        // Use gadget-scfs to get all deactivated features from devnet
        // must have `gadgets-scfs = "0.2.0" in Cargo.toml to use
        let mut my_matrix = ScfsMatrix::new(Some(ScfsCriteria {
            clusters: Some(vec![SCFS_DEVNET.to_string()]),
            ..Default::default()
        }))
        .unwrap();
        assert!(my_matrix.run().is_ok());
        let deactivated = my_matrix
            .get_features(Some(&ScfsMatrix::any_inactive))
            .unwrap();
        assert_ne!(deactivated.len(), 0);
        // Setup test validator and logging while deactivating all
        // features that are deactivated in devnet
        let (test_validator, main_payer) = clean_ledger_setup_validator(deactivated).unwrap();
        let connection = test_validator.get_rpc_client();
        solana_logger::setup_with_default("solana_runtime::message=debug");

        let accounts = &[];
        let txn = submit_transaction(
            &connection,
            &main_payer,
            [
                // Add two (2) instructions to transaction
                // Replace with instructions that make sense for your program
                Instruction::new_with_borsh(PROG_KEY, &0u8, accounts.to_vec()),
                Instruction::new_with_borsh(PROG_KEY, &1u8, accounts.to_vec()),
            ]
            .to_vec(),
        );
        assert!(txn.is_ok());
    }
    #[test]
    fn base_x_transaction_cu_test_dt() {
        let mut inv_vec = Vec::<Pubkey>::new();
        inv_vec.push(TXWIDE_LIMITS);
        inv_vec.push(pubkey!("HTW2pSyErTj4BV6KBM9NZ9VBUJVxt7sacNWcf76wtzb3"));
        inv_vec.push(TXWIDE_LIMITS); // These are redundant and set collection will eliminate
        inv_vec.push(pubkey!("HTW2pSyErTj4BV6KBM9NZ9VBUJVxt7sacNWcf76wtzb3"));
        inv_vec.push(PROG_KEY); // Not a feature so this will be rejected
        let (test_validator, main_payer) = clean_ledger_setup_validator(inv_vec).unwrap();
        let connection = test_validator.get_rpc_client();
        solana_logger::setup_with_default("solana_runtime::message=debug");

        let accounts = &[];
        let txn = submit_transaction(
            &connection,
            &main_payer,
            [
                // This CU transaction budget instruction does nothing when we deactivate the feature
                ComputeBudgetInstruction::set_compute_unit_limit(400_000u32),
                // Add two (2) instructions to transaction
                // Replace with instructions that make sense for your program
                Instruction::new_with_borsh(PROG_KEY, &0u8, accounts.to_vec()),
                Instruction::new_with_borsh(PROG_KEY, &1u8, accounts.to_vec()),
            ]
            .to_vec(),
        );
        println!("{:?}", txn);
    }
}
