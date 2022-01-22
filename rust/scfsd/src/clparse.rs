//! @brief command line setup and parse

use clap::{app_from_crate, App, AppSettings, Arg};

/// Builds command line argument parser
pub fn build_command_line_parser() -> App<'static> {
    app_from_crate!()
        .global_setting(AppSettings::DeriveDisplayOrder)
        .global_setting(AppSettings::ArgRequiredElseHelp)
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
    fn test_basic_pass() {
        let x = build_command_line_parser().try_get_matches_from([" "]);
        println!("{:?}", x);
    }
}
