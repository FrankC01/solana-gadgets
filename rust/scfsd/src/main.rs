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
    let mut matrix_result = if inc_set.contains("all") {
        ScfsMatrix::new(None)
    } else {
        ScfsMatrix::new(Some(ScfsCriteria {
            clusters: Some(inc_set.iter().map(|cluster| cluster.to_string()).collect()),
            ..Default::default()
        }))
    }?;

    matrix_result.run()?;
    if matches.is_present("keys_only_for_inactive") && inc_set.len() == 1 {
        let test_for: HashSet<_> = ["all", "local"].iter().cloned().collect();
        let intersection: HashSet<_> = test_for.intersection(&inc_set).collect();
        if intersection.len() == 0 {
            // Fetch the keys only
            let inactives = matrix_result.get_features(Some(&ScfsMatrix::all_inactive))?;
            let istr: Vec<String> = inactives.iter().map(|f| f.to_string()).collect();
            // If target is input to solana-test-validator
            if matches.is_present("target-test-validator") {
                let mut rvec = vec!["--deactivate-feature".to_string()];
                rvec.push(istr.join(" --deactivate-feature "));
                println!("{}", rvec.join(" "));
            } else {
                println!("{}", istr.join(" "));
            }
        } else {
            println!("Error: -k can only be used with -c devnet OR -c testnet OR -c mainnet. Found -c {:?}", intersection);
        }
    } else {
        write_matrix_stdio(&matrix_result);
    }

    Ok(())
}
