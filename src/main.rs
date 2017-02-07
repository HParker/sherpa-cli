#![deny(warnings)]

extern crate clap;
extern crate git2;
extern crate reqwest;
extern crate serde;
#[macro_use] extern crate serde_derive;
extern crate serde_json;

mod authentication;
mod cli;
mod client;
mod config;
mod deploy;
mod error;

use config::Config;
use std::error::Error as StdError;
use std::process::exit;

fn main() {
    let matches = cli::build_cli().get_matches();

    match matches.subcommand() {
        ("authenticate", Some(matches)) => authentication::run(matches).unwrap_or_else(handle_error),
        ("deploy", Some(matches)) => deploy::run(matches, load_config()),
        _ => unreachable!("The cli parser should prevent reaching here"),
    }
}

fn handle_error<E: StdError, T>(error: E) -> T {
    println!("{:?}", error);
    exit(1);
}

fn load_config() -> Config {
    match config::load_config(None) {
        Some(config) => config,
        None => {
            println!("You must be authenticated to use Sherpa\nPlease run `sherpa authenticate <github_handle> <github_token>`");
            exit(1);
        },
    }
}
