//! @brief command line setup and parse

use clap::{app_from_crate, App, AppSettings, Arg};

/// Builds command line argument parser
pub fn build_command_line_parser() -> App<'static> {
    app_from_crate!()
        .global_setting(AppSettings::DeriveDisplayOrder)
        // Limit cluster fetching
        .arg(
            Arg::new("cluster")
                .long("cluster")
                .short('c')
                .takes_value(true)
                .multiple_occurrences(true)
                .possible_values(&["all", "local", "devnet", "testnet", "mainnet"])
                .default_value("all")
                .help("Clusters to analyze"),
        )
        .arg(
            Arg::new("keys_only_for_inactive")
                .long("keys-only-for-inactive")
                .multiple_occurrences(false)
                .short('k')
                .help("Generates list of inactivated feature keys for specific cluster (-c) of devnet, testnet or mainnet"),
        )
        .arg(
            Arg::new("target-test-validator")
                .long("target-test-validator")
                .multiple_occurrences(false)
                .short('t')
                .help("Generates list of inactivated feature keys for input to solana-test-validator"),
        )
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use super::*;

    #[test]
    fn test_empty_pass() {
        let x = build_command_line_parser().try_get_matches_from([" "]);
        assert!(x.is_ok());
    }
    #[test]
    fn test_default_all_check_pass() {
        let matches = build_command_line_parser()
            .try_get_matches_from(["fing"])
            .unwrap();
        let mut inc_set = HashSet::<&str>::new();
        inc_set.extend(matches.values_of("cluster").unwrap());
        assert!(inc_set.contains("all"));
    }
    #[test]
    fn test_devnet_check_pass() {
        let matches = build_command_line_parser()
            .try_get_matches_from(["fing", "-c", "devnet"])
            .unwrap();
        let mut inc_set = HashSet::<&str>::new();
        inc_set.extend(matches.values_of("cluster").unwrap());
        assert_eq!(matches.occurrences_of("cluster"), 1);
        assert_eq!(matches.occurrences_of("cluster") as usize, inc_set.len());
        assert!(inc_set.contains("devnet"));
    }
    #[test]
    fn test_keys_inactive_pass() {
        let match_res =
            build_command_line_parser().try_get_matches_from(["fing", "-k", "-c", "devnet"]);
        assert!(match_res.is_ok());
        let matches = match_res.unwrap();
        let mut inc_set = HashSet::<&str>::new();
        inc_set.extend(matches.values_of("cluster").unwrap());
        let safe_inacts_only = if matches.is_present("keys_only_for_inactive") && inc_set.len() == 1
        {
            if inc_set.contains("all") || inc_set.contains("local") {
                false
            } else {
                true
            }
        } else {
            false
        };
        assert!(safe_inacts_only);
    }
    #[test]
    fn test_keys_no_inactive_pass() {
        let match_res = build_command_line_parser().try_get_matches_from(["fing"]);
        assert!(match_res.is_ok());
        let matches = match_res.unwrap();
        assert_eq!(matches.is_present("keys_only_for_inactive"), false);
    }

    #[test]
    fn test_keys_inactive_fail() {
        let match_res = build_command_line_parser()
            .try_get_matches_from(["fing", "-k", "-c", "devnet", "-c", "testnet"]);
        assert!(match_res.is_ok());
        let matches = match_res.unwrap();
        let mut inc_set = HashSet::<&str>::new();
        inc_set.extend(matches.values_of("cluster").unwrap());
        let safe_inacts_only = if matches.is_present("keys_only_for_inactive") && inc_set.len() == 1
        {
            if inc_set.contains("all") || inc_set.contains("local") {
                false
            } else {
                true
            }
        } else {
            false
        };
        assert!(!safe_inacts_only);
    }
}
