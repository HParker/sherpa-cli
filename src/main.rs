// #![deny(warnings)]

extern crate clap;
extern crate reqwest;
extern crate serde;
#[macro_use] extern crate serde_derive;
extern crate serde_json;

mod authentication;
mod cli;
mod config;
mod deploy;
mod error;

use std::process::exit;
use config::load_config;
use std::error::Error as StdError;

fn main() {
    let matches = cli::build_cli().get_matches();
    let config = match load_config(None) {
        Some(config) => config,
        None => {
            println!("You must be authenticated to use Sherpa");
            exit(1);
        },
    };

    match matches.subcommand() {
        ("authenticate", Some(matches)) => authentication::run(matches),
        ("deploy", Some(matches)) => deploy::run(matches, &config),
        _ => unreachable!("The cli parser should prevent reaching here"),
    }
}

fn handle_error<E: StdError, T>(error: E) -> T {
    println!("{}", error);
    exit(1);
}
