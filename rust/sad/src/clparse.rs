//! @brief command line setup and parse

use {
    clap::{
        crate_description, crate_name, crate_version, App, AppSettings, Arg, ArgGroup, ArgMatches,
    },
    lazy_static::*,
    solana_clap_utils::input_validators::{is_keypair, is_pubkey, is_url_or_moniker},
    solana_sdk::{pubkey::Pubkey, signature::read_keypair_file, signer::Signer},
    std::{collections::HashMap, str::FromStr},
};

/// Construct the cli input model and parse command line
pub fn parse_command_line() -> ArgMatches<'static> {
    App::new(crate_name!())
        .about(crate_description!())
        .version(crate_version!())
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
                .long("keypair")
                .short("k")
                .global(true)
                .validator(is_keypair)
                .conflicts_with_all(&["pkstr", "sampkey"])
                .takes_value(true)
                .help("Keypair to extract public key from"),
        )
        .arg(
            Arg::with_name("pkstr")
                .long("pubkey")
                .short("p")
                .global(true)
                .conflicts_with("sampkey")
                .validator(is_pubkey)
                .takes_value(true)
                .help("Publickey Base58 string"),
        )
        .arg(
            Arg::with_name("sampkey")
                .long("samplekey")
                .short("s")
                .global(true)
                .possible_values(&["user1", "user2", "prog"])
                .takes_value(true)
                .help("Account or program sample name"),
        )
        .arg(
            Arg::with_name("output")
                .long("output")
                .short("o")
                .global(true)
                .takes_value(true)
                .possible_values(&["json", "stdout"])
                .default_value("stdout")
                .requires_ifs(&[("json", "filename")])
                .help("Direct output to file"),
        )
        .arg(
            Arg::with_name("filename")
                .long("filename")
                .short("f")
                .global(true)
                .takes_value(true)
                .requires("output")
                .help("Filename for '-o json' output"),
        )
        .subcommand(App::new("account").about("Deserialize single account"))
        .subcommand(App::new("program").about("Deserialize all program owned accounts"))
        .group(
            ArgGroup::with_name("key_flags").args(&["keypair", "pkstr", "sampkey"]), // .required(true),
        )
        .get_matches()
}

lazy_static! {
    static ref SAMPLE_KEYS_MAP: HashMap<&'static str, &'static str> = {
        let mut jump_table = HashMap::<&str, &str>::new();
        if std::env::current_dir().unwrap().ends_with("sad") {
            jump_table.insert("user1", "../../samples/keys/user1_account.json");
            jump_table.insert("user2", "../../samples/keys/user2_account.json");
            jump_table.insert(
                "prog",
                "../../samples/keys/SampGgdt3wioaoMZhC6LTSbg4pnuvQnSfJpDYeuXQBv.json",
            );
            jump_table
        } else {
            jump_table.insert("user1", "../samples/keys/user1_account.json");
            jump_table.insert("user2", "../samples/keys/user2_account.json");
            jump_table.insert(
                "prog",
                "../samples/keys/SampGgdt3wioaoMZhC6LTSbg4pnuvQnSfJpDYeuXQBv.json",
            );
            jump_table
        }
    };
}

/// Get correct public key from command line
pub fn get_target_publickey(matches: &ArgMatches) -> Option<Pubkey> {
    let (kp, ks, ss) = (
        matches.is_present("keypair"),
        matches.is_present("pkstr"),
        matches.is_present("sampkey"),
    );
    match (kp, ks, ss) {
        (true, _, _) => Some(
            read_keypair_file(matches.value_of("keypair").unwrap())
                .unwrap()
                .pubkey(),
        ),
        (_, true, _) => Some(Pubkey::from_str(matches.value_of("pkstr").unwrap()).unwrap()),
        (_, _, true) => Some(
            read_keypair_file(
                SAMPLE_KEYS_MAP
                    .get(matches.value_of("sampkey").unwrap())
                    .unwrap(),
            )
            .unwrap()
            .pubkey(),
        ),
        _ => unreachable!(),
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use clap::ErrorKind;

    fn argsetup(faux_cmd_line: Vec<&str>) -> Result<ArgMatches, clap::Error> {
        App::new("prog")
            .arg(
                Arg::with_name("output")
                    .long("output")
                    .short("o")
                    .takes_value(true)
                    .possible_values(&["json", "stdout"])
                    .requires_ifs(&[("json", "filename")])
                    // .default_value("stdout")
                    .help("Direct output to file"),
            )
            .arg(
                Arg::with_name("filename")
                    .long("filename")
                    .short("f")
                    .takes_value(true)
                    .requires("output")
                    .help("Filename for '-o json' output"),
            )
            .get_matches_from_safe(faux_cmd_line)
    }

    #[test]
    fn test_requiredifs_options_without_output_should_pass() {
        let res = argsetup(vec!["prog", "-o", "json", "-f", "filename"]);
        assert!(res.is_ok());
    }
    #[test]
    fn test_requiredifs_options_without_output_should_fail() {
        let res = argsetup(vec!["prog", "-f", "filename"]);
        assert!(res.is_err()); // We  used -o excel so -f <filename> is required
        assert_eq!(res.unwrap_err().kind, ErrorKind::MissingRequiredArgument);
    }
    #[test]
    fn test_requiresif_options_without_file_should_fail() {
        let res = argsetup(vec!["prog", "-o", "json"]);
        assert!(res.is_err()); // We  used -o excel so -f <filename> is required
        assert_eq!(res.unwrap_err().kind, ErrorKind::MissingRequiredArgument);
    }

    // Setup for which key (accounnt or program)
    fn keysetup(faux_cmd_line: Vec<&str>) -> Result<ArgMatches, clap::Error> {
        App::new("prog")
            .arg(
                Arg::with_name("keypair")
                    .long("keypair")
                    .short("k")
                    .validator(is_keypair)
                    .takes_value(true)
                    .help("Keypair to extract public key from. Mutually exclusive with '--pubkey'"),
            )
            .arg(
                Arg::with_name("pkstr")
                    .long("pubkey")
                    .short("p")
                    .validator(is_pubkey)
                    .takes_value(true)
                    .help("Publickey string. Mutually exclusive with '--keyfile'"),
            )
            .arg(
                Arg::with_name("sampkey")
                    .long("samplekey")
                    .short("s")
                    .possible_values(&["user1", "user2", "prog"])
                    .takes_value(true)
                    .help("Account or program sample name"),
            )
            .group(
                ArgGroup::with_name("key_flags")
                    .required(true)
                    .args(&["keypair", "pkstr", "sampkey"]),
            )
            .get_matches_from_safe(faux_cmd_line)
    }

    #[test]
    fn test_keyfile_pass() {
        let matches = keysetup(vec!["prog", "-k", SAMPLE_KEYS_MAP.get("user2").unwrap()]).unwrap();
        let pkey = get_target_publickey(&matches);
        assert!(pkey.is_some());
    }
    #[test]
    fn test_keystr_pass() {
        let matches = keysetup(vec![
            "prog",
            "-p",
            "SampGgdt3wioaoMZhC6LTSbg4pnuvQnSfJpDYeuXQBv",
        ])
        .unwrap();
        let pkey = get_target_publickey(&matches);
        assert!(pkey.is_some());
    }
    #[test]
    fn test_sampkey_pass() {
        let matches = keysetup(vec!["prog", "-s", "user2"]).unwrap();
        let pkey = get_target_publickey(&matches);
        assert!(pkey.is_some());
    }
    #[test]
    fn test_sampkey_options_fail() {
        let matches = keysetup(vec!["prog"]);
        assert!(matches.is_err());
    }
}
