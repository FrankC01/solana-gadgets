//! @brief command line setup and parse

use {
    clap::{crate_description, crate_name, crate_version, App, AppSettings, Arg, ArgMatches},
    solana_clap_utils::input_validators::{is_keypair, is_pubkey, is_url_or_moniker},
};

/// Construct the cli input model and parse command line
pub fn parse_command_line() -> ArgMatches<'static> {
    App::new(crate_name!())
        .about(crate_description!())
        .version(crate_version!())
        // .setting(AppSettings::SubcommandRequiredElseHelp)
        .setting(AppSettings::ArgRequiredElseHelp)
        .arg({
            let arg = Arg::with_name("config_file")
                .short("C")
                .long("config")
                .value_name("PATH")
                .takes_value(true)
                .global(true)
                .help("Configuration file to use");
            if let Some(ref config_file) = *solana_cli_config::CONFIG_FILE {
                arg.default_value(config_file)
            } else {
                arg
            }
        })
        .arg(
            Arg::with_name("verbose")
                .long("verbose")
                .short("v")
                .takes_value(false)
                .global(true)
                .help("Show additional information"),
        )
        .arg(
            Arg::with_name("json_rpc_url")
                .short("u")
                .long("url")
                .value_name("URL")
                .takes_value(true)
                .global(true)
                .validator(is_url_or_moniker)
                .help("JSON RPC URL for the cluster [default: value from configuration file]"),
        )
        .arg(
            Arg::with_name("decl")
                .display_order(2)
                .long("declfile")
                .short("d")
                .takes_value(true)
                .global(true)
                .help("YAML data deserialization declaration file"),
        )
        .arg(
            Arg::with_name("keypair")
                .conflicts_with("pkstr")
                .long("keypair")
                .global(true)
                .short("k")
                .validator(is_keypair)
                .takes_value(true)
                .help("Keypair to extract public key from. Mutually exclusive with '--pubkey'"),
        )
        .arg(
            Arg::with_name("pkstr")
                .long("pubkey")
                .global(true)
                .short("p")
                .validator(is_pubkey)
                .takes_value(true)
                .help("Publickey string. Mutually exclusive with '--keyfile'"),
        )
        .arg(
            Arg::with_name("output")
                .long("output")
                .short("o")
                .global(true)
                .takes_value(true)
                .possible_values(&["csv", "excel", "stdout"])
                .default_value("stdout")
                .help("Direct output to file"),
        )
        .arg(
            Arg::with_name("filename")
                .long("filename")
                .short("f")
                .global(true)
                .takes_value(true)
                .required_ifs(&[("output", "excel"), ("output", "csv")])
                .help("Filename for '-o excel' or '-o csv' output"),
        )
        .subcommand(App::new("account").about("Deserialize single account"))
        .subcommand(App::new("program").about("Deserialize all program owned accounts"))
        .get_matches()
}

#[cfg(test)]
mod tests {
    use clap::ErrorKind;

    use super::*;
    #[test]
    fn test_requiredifs_options_without_file_fail() {
        let res = App::new("prog")
            .arg(
                Arg::with_name("output")
                    .long("output")
                    .short("o")
                    .takes_value(true)
                    .possible_values(&["csv", "excel", "stdout"])
                    .default_value("stdout")
                    .help("Direct output to file"),
            )
            .arg(
                Arg::with_name("filename")
                    .long("filename")
                    .short("f")
                    .takes_value(true)
                    .required_ifs(&[("output", "excel"), ("output", "csv")])
                    .help("Filename for '-o excel' or '-o csv' output"),
            )
            .get_matches_from_safe(vec!["prog", "-o", "excel"]);
        assert!(res.is_err()); // We  used -o excel so -f <filename> is required
        assert_eq!(res.unwrap_err().kind, ErrorKind::MissingRequiredArgument);
    }
    #[test]
    fn test_requiredifs_options_without_output_should_fail() {
        let res = App::new("prog")
            .arg(
                Arg::with_name("output")
                    .long("output")
                    .short("o")
                    .takes_value(true)
                    .possible_values(&["csv", "excel", "stdout"])
                    .default_value("stdout")
                    .help("Direct output to file"),
            )
            .arg(
                Arg::with_name("filename")
                    .long("filename")
                    .short("f")
                    .takes_value(true)
                    .required_ifs(&[("output", "excel"), ("output", "csv")])
                    .help("Filename for '-o excel' or '-o csv' output"),
            )
            .get_matches_from_safe(vec!["prog", "-f", "filename"]);
        assert!(res.is_err()); // We  used -o excel so -f <filename> is required
        assert_eq!(res.unwrap_err().kind, ErrorKind::MissingRequiredArgument);
    }
    #[test]
    fn test_requiresif_options_without_file_should_fail() {
        let res = App::new("prog")
            .arg(
                Arg::with_name("output")
                    .long("output")
                    .short("o")
                    .takes_value(true)
                    .possible_values(&["csv", "excel", "stdout"])
                    .default_value("stdout")
                    .help("Direct output to file"),
            )
            .arg(
                Arg::with_name("filename")
                    .long("filename")
                    .short("f")
                    .takes_value(true)
                    .requires_ifs(&[("output", "excel"), ("output", "csv")])
                    .help("Filename for '-o excel' or '-o csv' output"),
            )
            .get_matches_from_safe(vec!["prog", "-o", "excel"]);
        assert!(res.is_err()); // We  used -o excel so -f <filename> is required
        assert_eq!(res.unwrap_err().kind, ErrorKind::MissingRequiredArgument);
    }
    #[test]
    fn test_requiresif_options_without_output_should_fail() {
        let res = App::new("prog")
            .arg(
                Arg::with_name("output")
                    .long("output")
                    .short("o")
                    .takes_value(true)
                    .possible_values(&["csv", "excel", "stdout"])
                    .default_value("stdout")
                    .help("Direct output to file"),
            )
            .arg(
                Arg::with_name("filename")
                    .long("filename")
                    .short("f")
                    .takes_value(true)
                    .requires_ifs(&[("output", "excel"), ("output", "csv")])
                    .help("Filename for '-o excel' or '-o csv' output"),
            )
            .get_matches_from_safe(vec!["prog", "-f", "somefile"]);
        assert!(res.is_err()); // We  used -o excel so -f <filename> is required
        assert_eq!(res.unwrap_err().kind, ErrorKind::MissingRequiredArgument);
    }
}
