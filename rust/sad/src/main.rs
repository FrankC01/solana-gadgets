//! @brief Main entry poiint for CLI

use std::str::FromStr;

use {
    crate::{clparse::parse_command_line, datamap::DataMap},
    solana_clap_utils::{input_validators::normalize_to_url_if_moniker, keypair::DefaultSigner},
    solana_client::rpc_client::RpcClient,
    solana_remote_wallet::remote_wallet::RemoteWalletManager,
    solana_sdk::{commitment_config::CommitmentConfig, pubkey::Pubkey, signature::Signer},
    std::{path::Path, process::exit, sync::Arc},
};

/// sad main module
mod clparse;
mod datamap;

struct Config {
    commitment_config: CommitmentConfig,
    default_signer: Box<dyn Signer>,
    json_rpc_url: String,
    verbose: bool,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let app_matches = parse_command_line();
    let (sub_command, sub_matches) = app_matches.subcommand();
    let matches = sub_matches.unwrap();
    let mut wallet_manager: Option<Arc<RemoteWalletManager>> = None;
    let config = {
        let cli_config = if let Some(config_file) = matches.value_of("config_file") {
            solana_cli_config::Config::load(config_file).unwrap_or_default()
        } else {
            solana_cli_config::Config::default()
        };

        let default_signer = DefaultSigner::new(
            "keypair".to_string(),
            matches
                .value_of(&"keypair")
                .map(|s| s.to_string())
                .unwrap_or_else(|| cli_config.keypair_path.clone()),
        );

        Config {
            json_rpc_url: normalize_to_url_if_moniker(
                matches
                    .value_of("json_rpc_url")
                    .unwrap_or(&cli_config.json_rpc_url)
                    .to_string(),
            ),
            default_signer: default_signer
                .signer_from_path(matches, &mut wallet_manager)
                .unwrap_or_else(|err| {
                    eprintln!("error: {}", err);
                    exit(1);
                }),
            verbose: matches.is_present("verbose"),
            commitment_config: CommitmentConfig::confirmed(),
        }
    };
    solana_logger::setup_with_default("solana=info");

    if config.verbose {
        println!("JSON RPC URL: {}", config.json_rpc_url);
    }
    let rpc_client = RpcClient::new(config.json_rpc_url.clone());
    match (sub_command, sub_matches) {
        ("deser", Some(_arg_matchs)) => {
            let dfile = Path::new(matches.value_of("data-map").unwrap());
            let dmap = DataMap::new(dfile);
            let account: Pubkey = match matches.value_of("account") {
                Some(acc) => Pubkey::from_str(acc).unwrap(),
                None => config.default_signer.pubkey(),
            };
            dmap.map_accounts_data(&rpc_client, &account, config.commitment_config);
            println!("Deserializing data with {:?}", dmap);
        }
        _ => unreachable!(),
    }
    Ok(())
}
