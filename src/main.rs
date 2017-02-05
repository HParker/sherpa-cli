#![deny(warnings)]
#![feature(custom_attribute, custom_derive)]

extern crate clap;
#[macro_use] extern crate diesel;
extern crate dotenv;
extern crate reqwest;
extern crate serde;
#[macro_use] extern crate serde_derive;
extern crate serde_json;

mod authentication;
mod cli;
mod error;
mod schema;
mod user;

use dotenv::dotenv;

fn main() {
    dotenv().ok();
    let matches = cli::build_cli().get_matches();

    match matches.subcommand() {
        ("authenticate", Some(matches)) => authentication::run(matches),
        _ => unreachable!("The cli parser should prevent reaching here"),
    }
}
