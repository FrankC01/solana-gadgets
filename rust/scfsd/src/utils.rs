//! @brief solana-features-diff utility functions

use solana_sdk::{
    account::Account, clock::Slot, feature, feature_set::FEATURE_NAMES, pubkey::Pubkey,
};
use std::collections::HashMap;

/// Get the public keys from the static feature sets
pub fn base_feature_pks() -> Vec<Pubkey> {
    let mut feature_set_pks = Vec::<Pubkey>::new();
    for (k, _) in &*FEATURE_NAMES {
        feature_set_pks.push(k.clone())
    }
    feature_set_pks
}

/// Easy url lookup map (name -> url)
/// Should be moved to a lazy static
pub fn url_lookups() -> HashMap<String, String> {
    let mut urls = HashMap::<String, String>::new();
    urls.insert(
        "devnet".to_string(),
        "https://api.devnet.solana.com".to_string(),
    );
    urls.insert(
        "testnet".to_string(),
        "https://api.testnet.solana.com".to_string(),
    );
    urls.insert(
        "mainnet-beta".to_string(),
        "https://api.mainnet-beta.solana.com".to_string(),
    );
    urls
}

#[derive(Debug)]
pub enum FeatureStatus {
    Inactive,
    Pending,
    Active(Slot),
}

/// Get the status of a particular feature account
pub fn status_from_account(account: Account) -> Option<FeatureStatus> {
    feature::from_account(&account).map(|feature| match feature.activated_at {
        None => FeatureStatus::Pending,
        Some(activation_slot) => FeatureStatus::Active(activation_slot),
    })
}

#[derive(Debug)]
pub struct FeatureState {
    pub id: String,
    pub description: String,
    pub status: FeatureStatus,
}

/// Builds the ALL ENABLED feature set states
pub fn build_local_state() -> Vec<FeatureState> {
    let mut local_state = Vec::<FeatureState>::new();
    for (k, desc) in &*FEATURE_NAMES {
        local_state.push(FeatureState {
            id: k.to_string(),
            description: desc.to_string(),
            status: FeatureStatus::Active(0),
        })
    }
    local_state
}
