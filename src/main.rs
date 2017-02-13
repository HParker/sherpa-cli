extern crate sherpa;

use sherpa::authentication;
use sherpa::cli;
use sherpa::deploy;
use sherpa::{handle_error, load_config};

fn main() {
    let matches = cli::build_cli().get_matches();

    match matches.subcommand() {
        ("auth", Some(matches)) => authentication::run(matches).unwrap_or_else(handle_error),
        ("deploy", Some(matches)) => deploy::run(matches, load_config(), None).unwrap_or_else(handle_error),
        _ => unreachable!("The cli parser should prevent reaching here"),
    }
}
