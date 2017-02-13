#![deny(warnings)]

extern crate chrono;
extern crate clap;
extern crate git2;
extern crate reqwest;
extern crate serde;
#[macro_use] extern crate serde_derive;
extern crate serde_json;

pub mod authentication;
pub mod cli;
pub mod client;
pub mod config;
pub mod deploy;
pub mod error;

use config::Config;
use std::error::Error as StdError;
use std::process::exit;

pub fn handle_error<E: StdError, T>(error: E) -> T {
    println!("{:?}", error);
    exit(1);
}

pub fn load_config() -> Config {
    match config::load_config(None) {
        Some(config) => config,
        None => {
            println!("You must be authenticated to use Sherpa\nPlease run `sherpa authenticate <github_handle> <github_token>`");
            exit(1);
        },
    }
}
