extern crate sherpa;

use sherpa::authentication;
use sherpa::cli;
use sherpa::deploy;
use sherpa::{handle_error, load_config};
use sherpa::config;

fn main() {
    let matches = cli::build_cli().get_matches();

    let config_path = matches.value_of("config")
        .map(|config| config.to_owned())
        .unwrap_or(config::default_path());

    match matches.subcommand() {
        ("auth", Some(matches)) => {
            authentication::run(matches, config_path).unwrap_or_else(handle_error)
        }
        ("deploy", Some(matches)) => {
            deploy::run(matches, load_config(config_path), None).unwrap_or_else(handle_error)
        }
        _ => unreachable!("The cli parser should prevent reaching here"),
    }
}
