//! @brief Create for retrieving cluster feature status
//! supporting a myriad of parametizations
//!
//! A ScfsMatrix consists of rows, where each row has:
//!     Feature ID (Pubkey of feature) that is consistent across clusters - Always included
//!     A per feature status (ScfsStatus) type for devnet if included in criteria
//!     A per feature status (ScfsStatus) type for testnet if included in criteria
//!     A per feature status (ScfsStatus) type for mainnet if included in criteria
//!     A per feature Description (String) if included in criteria
//!
//! Options/Criteria for a ScfsMatrix build
//! 1. Return all feature status across all clusters and all fields poplated as noted for ScfsMatrix (predefined)
//! 2. User configured criteria allowing:
//!     2.1 Identifying which cluster(s) to sample
//!         2.1.1 Boolean indicating whether to keep the row if status from cluster is Active or other
//!     2.2 Identifying which elements of the rows to return
//!         2.2.1 Feature ID is always included
//!     2.3 A list of feature IDs to sample for the cluster status
//!

use lazy_static::*;
use scfs_errors::{ScfsError, ScfsResult};
use solana_client::rpc_client::RpcClient;
use solana_sdk::{
    account::Account, clock::Slot, feature, feature_set::FEATURE_NAMES, pubkey::Pubkey,
};
use std::collections::HashMap;

pub mod scfs_errors;

lazy_static! {
    /// Easy cluster aliases and misc string constants
    pub static ref SCFS_FEATURE_ID: String = "feature ID (pk)".to_string();
    pub static ref SCFS_LOCAL: String = "local".to_string();
    pub static ref SCFS_DEVNET: String = "devnet".to_string();
    pub static ref SCFS_TESTNET: String = "testnet".to_string();
    pub static ref SCFS_MAINNET: String = "mainnet".to_string();
    pub static ref SCFS_DESCRIPTION: String = "description".to_string();

    /// Easy url lookup map (name -> url)
    /// subject to change! Alternative would be to
    /// cycle changing the configuration and interogatting the Rcp URL
    pub static ref SCFS_URL_LOOKUPS: HashMap<String, String> = {
        let mut urls = HashMap::<String, String>::new();
        urls.insert(
            SCFS_LOCAL.clone(),
            "http://localhost:8899".to_string(),
        );
        urls.insert(
            SCFS_DEVNET.clone(),
            "https://api.devnet.solana.com".to_string(),
        );
        urls.insert(
            SCFS_TESTNET.clone(),
            "https://api.testnet.solana.com".to_string(),
        );
        urls.insert(
            SCFS_MAINNET.clone(),
            "https://api.mainnet-beta.solana.com".to_string(),
        );
        urls
    };
    /// List of cluster aliases
    pub static ref SCFS_CLUSTER_LIST: Vec<String> = {
        let mut clusters = Vec::<String>::new();
        clusters.push(SCFS_LOCAL.clone());
        clusters.push(SCFS_DEVNET.clone());
        clusters.push(SCFS_TESTNET.clone());
        clusters.push(SCFS_MAINNET.clone());
        clusters
    };

    /// Header Default List
    pub static ref SCFS_HEADER_LIST: Vec<String> = {
        let mut headers = SCFS_CLUSTER_LIST.to_vec();
        headers.insert(0, SCFS_FEATURE_ID.clone());
        headers.push(SCFS_DESCRIPTION.clone());
        headers
    };
    /// Features public keys
    pub static ref SCFS_FEATURE_PKS: Vec<Pubkey> = {
        FEATURE_NAMES.keys().cloned().collect::<Vec<Pubkey>>()
    };
}

#[derive(Clone, Debug, PartialEq)]
/// Criteria for processing feature set statusing
pub struct ScfsCriteria {
    pub features: Option<Vec<Pubkey>>, // Limits the feature to query status on, defaults to all
    pub clusters: Option<Vec<String>>, // Limits what clusters to query the features on, defaults to all
}

impl ScfsCriteria {
    fn get_clusters(&self) -> &Option<Vec<String>> {
        &self.clusters
    }
}

impl Default for ScfsCriteria {
    fn default() -> Self {
        Self {
            features: Some(SCFS_FEATURE_PKS.to_vec()),
            clusters: Some(SCFS_CLUSTER_LIST.to_vec()),
        }
    }
}

/// Cluster feature status indicator
#[derive(Debug, Clone, PartialEq)]
pub enum ScfsStatus {
    Inactive,
    Pending,
    Active(Slot),
}

#[derive(Debug)]
pub struct ScfsRow {
    feature_key: Pubkey,
    feature_status: Vec<ScfsStatus>,
    feature_description: String,
}

impl ScfsRow {
    /// New ScfsRow with key and description
    fn new(feature_key: Pubkey, feature_description: String) -> Self {
        Self {
            feature_key,
            feature_description: feature_description,
            feature_status: Vec::<ScfsStatus>::new(),
        }
    }
    pub fn key(&self) -> &Pubkey {
        &self.feature_key
    }
    pub fn status(&self) -> &Vec<ScfsStatus> {
        &self.feature_status
    }
    pub fn desc(&self) -> &String {
        &self.feature_description
    }
    // Borrow the feature status
    fn push_feature_status(&mut self, status: ScfsStatus) {
        self.feature_status.push(status)
    }
}

#[derive(Debug)]
pub struct ScfsMatrix {
    criteria: ScfsCriteria,
    rows: Vec<ScfsRow>,
    query_set: Vec<Pubkey>,
}

impl ScfsMatrix {
    /// Creates a new ScfsMatrix with either the default
    /// ScfsCriteria (if None passed in) or configures
    /// to the provided ScfsCriteria after validating
    pub fn new(in_criteria: Option<ScfsCriteria>) -> ScfsResult<Self> {
        let criteria = if let Some(c) = in_criteria {
            Self::validate_and_complete_criteria(&c)?
        } else {
            ScfsCriteria::default()
        };
        let (rows, query_set) = Self::build_rows(&criteria);
        Ok(Self {
            criteria,
            rows,
            query_set,
        })
    }

    // Prebuild rows and vector of publickeys to query by cluster
    fn build_rows(criteria: &ScfsCriteria) -> (Vec<ScfsRow>, Vec<Pubkey>) {
        let mut query_set = Vec::<Pubkey>::new();
        let rows: Vec<_> = criteria
            .features
            .as_ref()
            .unwrap()
            .iter()
            .map(|f| {
                let pk = f.clone();
                query_set.push(pk.clone());
                ScfsRow::new(pk, (&*FEATURE_NAMES.get(f).unwrap()).to_string())
            })
            .collect();
        (rows, query_set)
    }

    /// Validate the criteria for building the matrix
    /// TODO - Build filter predicates
    fn validate_and_complete_criteria(in_criteria: &ScfsCriteria) -> ScfsResult<ScfsCriteria> {
        if in_criteria.features.is_none() {
            Err(ScfsError::NoCriteriaFeaturesError)
        } else {
            let mut bad_elements = Vec::<String>::new();
            // Its ok to not have clusters but they must be
            // a recognized cluster name
            if let Some(clusters) = &in_criteria.clusters {
                let matching = clusters
                    .iter()
                    .filter(|predicate| {
                        if SCFS_URL_LOOKUPS.get(*predicate).is_some() {
                            true
                        } else {
                            bad_elements.push(predicate.to_string());
                            false
                        }
                    })
                    .count();
                if matching != clusters.len() {
                    return Err(ScfsError::UnrecognizedCriteriaTypeError {
                        element: bad_elements,
                        ctype: "cluster",
                    });
                }
            }
            // Must have features and must match from system
            // master list
            if let Some(features) = &in_criteria.features {
                let matching = features
                    .iter()
                    .filter(|predicate| {
                        if SCFS_FEATURE_PKS.contains(predicate) {
                            true
                        } else {
                            bad_elements.push(predicate.to_string());
                            false
                        }
                    })
                    .count();
                if matching != features.len() {
                    return Err(ScfsError::UnrecognizedCriteriaTypeError {
                        element: bad_elements,
                        ctype: "feature",
                    });
                }
            } else {
                return Err(ScfsError::UnrecognizedCriteriaTypeError {
                    element: vec!["empty".to_string()],
                    ctype: "No features",
                });
            }
            Ok(in_criteria.clone())
        }
    }

    // Update the status for a row
    fn push_to_row(&mut self, row_index: usize, status: ScfsStatus) {
        let row = &mut self.rows[row_index];
        row.push_feature_status(status);
    }

    /// Get the status of a particular feature account
    fn status_from_account(account: Account) -> Option<ScfsStatus> {
        feature::from_account(&account).map(|feature| match feature.activated_at {
            None => ScfsStatus::Pending,
            Some(activation_slot) => ScfsStatus::Active(activation_slot),
        })
    }

    /// Get account state and add to row
    fn set_status_for_row(&mut self, row_index: usize, account: Option<Account>) {
        let status = match account {
            Some(a) => match ScfsMatrix::status_from_account(a) {
                Some(s) => s,
                None => ScfsStatus::Inactive,
            },
            None => ScfsStatus::Inactive,
        };
        self.push_to_row(row_index, status)
    }

    /// Populate rows from cluster statusing
    fn process_cluster(
        &mut self,
        query_set: &Vec<Pubkey>,
        cluster_ref: &Option<Vec<String>>,
    ) -> ScfsResult<()> {
        if let Some(clusters) = cluster_ref {
            for cluster in clusters {
                match cluster.as_str() {
                    "local" => {
                        let mut index = 0usize;
                        for _ in query_set {
                            self.push_to_row(index, ScfsStatus::Active(0));
                            index += 1
                        }
                    }
                    _ => {
                        let rcpclient =
                            RpcClient::new(SCFS_URL_LOOKUPS.get(cluster).unwrap().clone());
                        for (index, account) in rcpclient
                            .get_multiple_accounts(&query_set)
                            .unwrap()
                            .into_iter()
                            .enumerate()
                        {
                            self.set_status_for_row(index, account);
                        }
                    }
                }
            }
        }
        Ok(())
    }

    /// Gets the internal query set
    fn get_query_set(&self) -> &Vec<Pubkey> {
        &self.query_set
    }

    /// Run the matrix
    pub fn run(&mut self) -> ScfsResult<()> {
        let qs = self.get_query_set().clone();
        let csref = self.get_criteria().get_clusters().clone();
        self.process_cluster(&qs, &csref)
    }

    /// Retrieve criteria used in processing
    pub fn get_criteria(&self) -> &ScfsCriteria {
        &self.criteria
    }

    /// Retrieve rows
    pub fn get_result_rows(&self) -> &Vec<ScfsRow> {
        &self.rows
    }
}

#[cfg(test)]
mod tests {
    use solana_sdk::pubkey::Pubkey;

    use crate::{
        ScfsCriteria, ScfsMatrix, SCFS_CLUSTER_LIST, SCFS_DEVNET, SCFS_FEATURE_PKS, SCFS_LOCAL,
    };

    #[test]
    fn full_empty_criteria_pass() {
        let mut my_matrix = ScfsMatrix::new(None).unwrap();
        if let Some(c) = &my_matrix.get_criteria().features {
            assert_eq!(c.len(), SCFS_FEATURE_PKS.len());
        }
        if let Some(c) = &my_matrix.get_criteria().clusters {
            assert_eq!(c.len(), SCFS_CLUSTER_LIST.len());
        }
        assert!(my_matrix.run().is_ok());
        for res_row in my_matrix.get_result_rows() {
            println!(
                "{} Local {:?} Dev {:?}",
                res_row.feature_key, res_row.feature_status[0], res_row.feature_status[1]
            );
        }
    }

    #[test]
    fn test_localnet_pass() {
        let mut cluster_vec = Vec::<String>::new();
        cluster_vec.push(SCFS_LOCAL.to_string());
        let mut my_matrix = ScfsMatrix::new(Some(ScfsCriteria {
            clusters: Some(cluster_vec),
            ..Default::default()
        }))
        .unwrap();
        assert!(my_matrix.run().is_ok());
        for res_row in my_matrix.get_result_rows() {
            println!("{:?}", res_row)
        }
    }
    #[test]
    fn test_devnet_pass() {
        let mut cluster_vec = Vec::<String>::new();
        cluster_vec.push(SCFS_DEVNET.to_string());
        let mut my_matrix = ScfsMatrix::new(Some(ScfsCriteria {
            clusters: Some(cluster_vec),
            ..Default::default()
        }))
        .unwrap();
        assert!(my_matrix.run().is_ok());
        for res_row in my_matrix.get_result_rows() {
            println!("{:?}", res_row)
        }
    }

    #[test]
    fn bad_features_fail() {
        let mut base_criteria = ScfsCriteria::default();
        base_criteria.features = None;
        let my_matrix = ScfsMatrix::new(Some(base_criteria));
        assert!(my_matrix.is_err());
        println!("{:?}", my_matrix);
        let mut base_criteria = ScfsCriteria::default();
        let faux_pkey = Pubkey::default();
        let mut faux_vec = Vec::<Pubkey>::new();
        faux_vec.push(faux_pkey);
        base_criteria.features = Some(faux_vec);
        let my_matrix = ScfsMatrix::new(Some(base_criteria));
        assert!(my_matrix.is_err());
        println!("{:?}", my_matrix);
    }
    #[test]
    fn bad_clusters_fail() {
        let faux_field = "funny_business".to_string();
        let mut faux_vec = Vec::<String>::new();
        faux_vec.push(faux_field);
        let my_matrix = ScfsMatrix::new(Some(ScfsCriteria {
            clusters: Some(faux_vec),
            ..Default::default()
        }));
        assert!(my_matrix.is_err());
        println!("{:?}", my_matrix);
    }
}
