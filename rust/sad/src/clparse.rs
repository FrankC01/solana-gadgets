//! @brief command line setup and parse

use {
    clap::{
        crate_description, crate_name, crate_version, App, AppSettings, Arg, ArgMatches, SubCommand,
    },
    solana_clap_utils::input_validators::{is_url_or_moniker, is_valid_pubkey, is_valid_signer},
};

/// Construct the cli input model and parse command line
pub fn parse_command_line() -> ArgMatches<'static> {
    App::new(crate_name!())
        .about(crate_description!())
        .version(crate_version!())
        .setting(AppSettings::SubcommandRequiredElseHelp)
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
            Arg::with_name("keypair")
                .long("keypair")
                .value_name("KEYPAIR")
                .short("k")
                .validator(is_valid_signer)
                .takes_value(true)
                .global(true)
                .help("Filepath or URL to a keypair [default: client keypair]"),
        )
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
        .subcommand(
            App::new("account").about("Deserialize single account").arg(
                Arg::with_name("pkstr")
                    .display_order(1)
                    .long("pubkey")
                    .short("p")
                    .validator(is_valid_pubkey)
                    .required(false)
                    .takes_value(true)
                    .help("Account publickey string"),
            ),
        )
        .subcommand(
            App::new("program")
                .about("Deserialize all program owned accounts")
                .arg(
                    Arg::with_name("pkstr")
                        .display_order(1)
                        .long("pubkey")
                        .short("p")
                        .validator(is_valid_pubkey)
                        .required(false)
                        .takes_value(true)
                        .help("Program publickey string"),
                ),
        )
        .get_matches()
}
