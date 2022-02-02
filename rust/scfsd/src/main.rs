//! @brief Diff the feature sets status between
//! Solana clusters (local, devnet, testnet, mainnet)

use std::collections::HashSet;

// Local will always have all features enabled when running,
// in solana-test-validator all features are enabled
use clparse::build_command_line_parser;
use gadgets_scfs::{ScfsCriteria, ScfsMatrix};
use utils::write_matrix_stdio;

mod clparse;
mod utils;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Command line args
    let matches = build_command_line_parser().get_matches();
    let mut inc_set = HashSet::<&str>::new();
    inc_set.extend(matches.values_of("cluster").unwrap());
    let matrix_result = if inc_set.contains("all") {
        ScfsMatrix::new(None)
    } else {
        ScfsMatrix::new(Some(ScfsCriteria {
            clusters: Some(inc_set.iter().map(|cluster| cluster.to_string()).collect()),
            ..Default::default()
        }))
    };

    if matrix_result.is_ok() {
        let mut matrix = matrix_result.unwrap();
        let run_result = matrix.run();
        if run_result.is_ok() {
            write_matrix_stdio(&matrix)
        }
    }

    Ok(())
}
