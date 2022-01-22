//! @brief Diff the feature sets status between
//! Solana clusters (local, devnet, testnet, mainnet)

// Local will always have all features enabled when running,
// for example, in solana-test-validator
use clparse::build_command_line_parser;
use solana_client::rpc_client::RpcClient;
use solana_sdk::feature_set::FEATURE_NAMES;
use std::collections::HashMap;
use utils::{
    base_feature_pks, build_local_state, status_from_account, url_lookups, FeatureState,
    FeatureStatus,
};

mod clparse;
mod utils;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Get all feature public keys
    let matches = build_command_line_parser().get_matches();
    let feature_set_pks = base_feature_pks();
    let url_map = url_lookups();
    let mut features_by_cluster = HashMap::<String, Vec<FeatureState>>::new();
    features_by_cluster.insert("Local".to_string(), build_local_state());
    for (stub, url) in url_lookups() {
        let rcpclient = RpcClient::new(url);
        let mut features: Vec<FeatureState> = vec![];
        for (i, account) in rcpclient
            .get_multiple_accounts(&feature_set_pks)?
            .into_iter()
            .enumerate()
        {
            let feature_id = &feature_set_pks[i];
            let feature_name = FEATURE_NAMES.get(feature_id).unwrap();
            if let Some(account) = account {
                if let Some(feature_status) = status_from_account(account) {
                    features.push(FeatureState {
                        id: feature_id.to_string(),
                        description: feature_name.to_string(),
                        status: feature_status,
                    });
                    continue;
                }
            }
            features.push(FeatureState {
                id: feature_id.to_string(),
                description: feature_name.to_string(),
                status: FeatureStatus::Inactive,
            });
        }
        features_by_cluster.insert(stub.clone(), features);
    }

    for (stub, cluster_features) in features_by_cluster {
        println!("For {}", &url_map.get(&stub).unwrap());
        for cvs in cluster_features {
            println!("{} -> {:?}", cvs.id, cvs.status)
        }
    }

    Ok(())
}
