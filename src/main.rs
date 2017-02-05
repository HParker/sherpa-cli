// #![deny(warnings)]

extern crate clap;
extern crate reqwest;
extern crate serde;
#[macro_use] extern crate serde_derive;
extern crate serde_json;

mod authentication;
mod cli;
mod config;
mod error;

fn main() {
    let matches = cli::build_cli().get_matches();

    match matches.subcommand() {
        ("authenticate", Some(matches)) => authentication::run(matches),
        _ => unreachable!("The cli parser should prevent reaching here"),
    }
}
