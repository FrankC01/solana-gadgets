//! @brief solana-features-diff utility functions
use console::{style, StyledObject};
use gadgets_scfs::{
    SCFS_DESCRIPTION, SCFS_DEVNET, SCFS_FEATURE_PKS, SCFS_HEADER_LIST, SCFS_LOCAL, SCFS_MAINNET,
    SCFS_TESTNET, SCFS_URL_LOOKUPS,
};

use solana_client::rpc_client::RpcClient;
use solana_sdk::{
    account::Account, clock::Slot, feature, feature_set::FEATURE_NAMES, pubkey::Pubkey,
};
use std::collections::HashMap;

/// Cluster feature status
#[derive(Debug, Clone, PartialEq)]
pub enum FeatureStatus {
    Inactive,
    Pending,
    Active(Slot),
}

/// Container for feature status across multiple clusters
#[derive(Debug, Clone, PartialEq)]
pub struct FeatureState {
    pub description: String,
    pub status: [FeatureStatus; 4],
}

/// Grid for cluster feature state types
pub type ScfsdGrid = HashMap<Pubkey, FeatureState>;

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
}

/// Iterates through the feature results for a given cluster and
/// sets the grid entry accordingly
pub fn update_grid_for(
    position: usize,
    cluster_alias: &String,
    grid: &mut ScfsdGrid,
) -> Result<(), Box<dyn std::error::Error>> {
    let rcpclient = RpcClient::new(SCFS_URL_LOOKUPS.get(cluster_alias).unwrap().clone());
    for (index, account) in rcpclient
        .get_multiple_accounts(&SCFS_FEATURE_PKS)?
        .into_iter()
        .enumerate()
    {
        let apk = SCFS_FEATURE_PKS[index];
        // If account is valid, get status and update grid
        if let Some(acc) = account {
            if let Some(status) = status_from_account(acc) {
                update_grid_status_entry(grid, &apk, position, status);
                continue;
            }
        }
        // Defaults to Inactive, update grid
        update_grid_status_entry(grid, &apk, position, FeatureStatus::Inactive);
    }
    Ok(())
}

/// Transmuate state arrary to boolean array
fn states_to_bools(fstate: &FeatureState) -> Vec<bool> {
    fstate
        .status
        .iter()
        .fold(Vec::<bool>::new(), |mut acc, xs| {
            acc.push(match xs {
                FeatureStatus::Inactive | FeatureStatus::Pending => false,
                _ => true,
            });
            acc
        })
}

#[derive(Debug)]
struct ScfsdMatrixRow<'a> {
    key: &'a Pubkey,
    local_status: bool,
    dev_status: bool,
    test_status: bool,
    main_status: bool,
    desc: &'a String,
}

impl<'a> ScfsdMatrixRow<'a> {
    pub fn from_feature_state(pkey: &'a Pubkey, fstate: &'a FeatureState) -> Self {
        let fstob = states_to_bools(&fstate);
        Self {
            key: pkey,
            local_status: fstob[0],
            dev_status: fstob[1],
            test_status: fstob[2],
            main_status: fstob[3],
            desc: &fstate.description,
        }
    }
}

#[derive(Debug)]
pub struct ScfsdMatrix<'a> {
    rows: Vec<ScfsdMatrixRow<'a>>,
    includes: Option<Vec<String>>,
    header: Vec<String>,
}

impl<'a> ScfsdMatrix<'a> {
    pub fn from_grid(grid: &'a ScfsdGrid) -> Self {
        let mut matrix = Vec::<ScfsdMatrixRow>::new();
        for (pkey, state) in grid {
            matrix.push(ScfsdMatrixRow::from_feature_state(pkey, state))
        }
        Self {
            rows: matrix,
            includes: None,
            header: SCFS_HEADER_LIST.to_vec(),
        }
    }
    // pub fn from_includes(includes: Option<Vec<String>>) -> Self {

    // }
}

impl std::fmt::Display for ScfsdMatrix<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        fn fill_status(status: bool) -> StyledObject<String> {
            let yes = " ".to_string();
            let no = "  ".to_string();
            if status {
                style(yes).bg(console::Color::Green)
            } else {
                style(no).bg(console::Color::Red)
            }
        }
        writeln!(
            f,
            "{}",
            style(format!(
                "\n{:<44} | {:^8} | {:^8} |{:^8} |{:^8} | {:<95}",
                "Feature ID (PK)",
                *SCFS_LOCAL,
                *SCFS_DEVNET,
                *SCFS_TESTNET,
                *SCFS_MAINNET,
                *SCFS_DESCRIPTION
            ))
            .bold()
        )?;
        writeln!(
            f,
            "{}",
            style(format!(
                "{:-<44} | {:-^8} | {:-^8} |{:-^8} |{:-^8} | {:-<95}",
                "", "", "", "", "", ""
            )) // .bold()
        )?;
        for row in &self.rows {
            writeln!(
                f,
                "{:<44} | {:^8} | {:^8} |{:^8} |{:^8} | {}",
                row.key.to_string(),
                fill_status(row.local_status),
                fill_status(row.dev_status),
                fill_status(row.test_status),
                fill_status(row.main_status),
                row.desc,
            )?;
        }
        Ok(())
    }
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

    #[test]
    fn test_grid_formatting() {
        let grid = initialize_grid();
        let matrix = ScfsdMatrix::from_grid(&grid);
        println!("{}", matrix);
    }
}
