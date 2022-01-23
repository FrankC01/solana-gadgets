//! @brief solana-features-diff utility functions

use lazy_static::*;
use solana_client::rpc_client::RpcClient;
use solana_sdk::{
    account::Account, clock::Slot, feature, feature_set::FEATURE_NAMES, pubkey::Pubkey,
};
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
pub enum FeatureStatus {
    Inactive,
    Pending,
    Active(Slot),
}

#[derive(Debug, Clone, PartialEq)]
pub struct FeatureState {
    pub description: String,
    pub status: [FeatureStatus; 4],
}

/// Grid for cluster feature state types
pub type ScfsdGrid = HashMap<Pubkey, FeatureState>;

lazy_static! {
    pub static ref SCFSD_LOCAL: String = "Local".to_string();
    pub static ref SCFSD_DEVNET: String = "Devnet".to_string();
    pub static ref SCFSD_TESTNET: String = "Testnet".to_string();
    pub static ref SCFSD_MAINNET: String = "Mainnet".to_string();
    /// Easy url lookup map (name -> url)
    pub static ref SCFSD_URL_LOOKUPS: HashMap<String, String> = {
        let mut urls = HashMap::<String, String>::new();
        urls.insert(
            SCFSD_LOCAL.clone(),
            "http://localhost:8899".to_string(),
        );
        urls.insert(
            SCFSD_DEVNET.clone(),
            "https://api.devnet.solana.com".to_string(),
        );
        urls.insert(
            SCFSD_TESTNET.clone(),
            "https://api.testnet.solana.com".to_string(),
        );
        urls.insert(
            SCFSD_MAINNET.clone(),
            "https://api.mainnet-beta.solana.com".to_string(),
        );
        urls
    };
    /// List of cluster aliases
    pub static ref SCFSD_CLUSTER_LIST: Vec<String> = {
        let mut clusters = Vec::<String>::new();
        clusters.push(SCFSD_LOCAL.clone());
        clusters.push(SCFSD_DEVNET.clone());
        clusters.push(SCFSD_TESTNET.clone());
        clusters.push(SCFSD_MAINNET.clone());
        clusters
    };

    /// Features public keys
    pub static ref SCFSD_FEATURE_PKS: Vec<Pubkey> = {
        FEATURE_NAMES.keys().cloned().collect::<Vec<Pubkey>>()
    };

}

/// Return a baseline clone which  includes the local state
pub fn initialize_grid() -> ScfsdGrid {
    let mut lstate = ScfsdGrid::new();
    for (pubkey, desc) in &*FEATURE_NAMES {
        lstate.insert(
            pubkey.clone(),
            FeatureState {
                description: desc.to_string(),
                status: [
                    FeatureStatus::Active(0),
                    FeatureStatus::Pending,
                    FeatureStatus::Pending,
                    FeatureStatus::Pending,
                ],
            },
        );
    }
    lstate
}

/// Get the status of a particular feature account
fn status_from_account(account: Account) -> Option<FeatureStatus> {
    feature::from_account(&account).map(|feature| match feature.activated_at {
        None => FeatureStatus::Pending,
        Some(activation_slot) => FeatureStatus::Active(activation_slot),
    })
}

/// Update a status at index in the grid entry
fn update_grid_status_entry(
    grid: &mut ScfsdGrid,
    akey: &Pubkey,
    index: usize,
    status: FeatureStatus,
) {
    grid.get_mut(akey).unwrap().status[index] = status;
    // let fset = &mut grid.get_mut(akey).unwrap().status;
    // fset[index] = status;
}

/// Iterates through the feature results for a given cluster and
/// sets the grid entry accordingly
pub fn update_grid_for(
    position: usize,
    cluster_alias: &String,
    grid: &mut ScfsdGrid,
) -> Result<(), Box<dyn std::error::Error>> {
    let rcpclient = RpcClient::new(SCFSD_URL_LOOKUPS.get(cluster_alias).unwrap().clone());
    for (index, account) in rcpclient
        .get_multiple_accounts(&SCFSD_FEATURE_PKS)?
        .into_iter()
        .enumerate()
    {
        let apk = SCFSD_FEATURE_PKS[index];
        if let Some(acc) = account {
            if let Some(status) = status_from_account(acc) {
                update_grid_status_entry(grid, &apk, position, status);
                continue;
            }
        }
        update_grid_status_entry(grid, &apk, position, FeatureStatus::Inactive);
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_update_grid_pass() {
        let mut grid = initialize_grid();
        let myurl = "Devnet".to_string();
        update_grid_for(1usize, &myurl, &mut grid).unwrap();
        for (p, v) in grid {
            println!("{:?} = {:?}", p, v);
        }
    }
}
