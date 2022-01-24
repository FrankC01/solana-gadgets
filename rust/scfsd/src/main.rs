//! @brief Diff the feature sets status between
//! Solana clusters (local, devnet, testnet, mainnet)

// Local will always have all features enabled when running,
// in solana-test-validator all features are enabled
use clparse::build_command_line_parser;
use utils::{initialize_grid, update_grid_for, ScfsdMatrix, SCFSD_CLUSTER_LIST};

mod clparse;
mod utils;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Get all feature public keys
    let matches = build_command_line_parser().get_matches();
    let mut grid = initialize_grid();

    // Populate the position
    let mut index: usize = 1;
    for cluster_name in &*SCFSD_CLUSTER_LIST {
        match cluster_name.as_str() {
            "Local" => (),
            "Devnet" | "Testnet" | "Mainnet" => {
                let indx = index;
                index += 1;
                update_grid_for(indx, cluster_name, &mut grid)?
            }
            _ => unreachable!(),
        };
    }

    match matches.value_of("filename") {
        Some(_output_filename) => todo!(),
        None => {
            println!("{}", ScfsdMatrix::from_grid(&grid))
        }
    }

    Ok(())
}
