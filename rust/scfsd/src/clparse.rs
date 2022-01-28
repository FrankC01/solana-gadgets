//! @brief command line setup and parse

use clap::{app_from_crate, App, AppSettings, Arg};

/// Builds command line argument parser
pub fn build_command_line_parser() -> App<'static> {
    app_from_crate!()
        .global_setting(AppSettings::DeriveDisplayOrder)
        // Limit cluster fetching
        .arg(
            Arg::new("include")
                .long("include")
                .short('i')
                .takes_value(true)
                .help("Selective choices"),
        )
        // Output to CSV
        .arg(
            Arg::new("filename")
                .long("filename")
                .short('f')
                .takes_value(true)
                .help("Output to filename in csv format"),
        )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_pass() {
        let x = build_command_line_parser().try_get_matches_from([" "]);
        assert!(x.is_ok());
    }
    #[test]
    fn test_fileoutput_check_pass() {
        let x = build_command_line_parser().try_get_matches_from(["fing", "-f", "foo"]);
        assert!(x.is_ok());
        let matches = x.unwrap();
        assert!(matches.value_of("filename").is_some());
    }
    #[test]
    fn test_stdoutput_check_pass() {
        let x = build_command_line_parser().try_get_matches_from(["fing"]);
        assert!(x.is_ok());
        let matches = x.unwrap();
        assert!(matches.value_of("filename").is_none());
    }
}
