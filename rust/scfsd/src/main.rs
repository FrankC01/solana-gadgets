//! @brief Diff the feature sets status between
//! Solana clusters (local, devnet, testnet, mainnet)

// Local will always have all features enabled when running,
// in solana-test-validator all features are enabled
use clparse::build_command_line_parser;
use gadgets_scfs::ScfsMatrix;
use utils::write_matrix_stdio;

mod clparse;
mod utils;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Command line args
    let matches = build_command_line_parser().get_matches();
    let matrix_result = ScfsMatrix::new(None);
    if matrix_result.is_ok() {
        let mut matrix = matrix_result.unwrap();
        let run_result = matrix.run();
        if run_result.is_ok() {
            match matches.value_of("filename") {
                Some(_output_filename) => todo!(),
                None => write_matrix_stdio(&matrix),
            }
        }
    }

    Ok(())
}
