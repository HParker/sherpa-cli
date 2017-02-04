extern crate clap;

use clap::{ArgMatches};

mod cli;

fn main() {

    let matches = cli::build_cli().get_matches();

    match matches.subcommand() {
        ("authenticate", Some(matches)) => run_setup_command(matches),
        _ => unreachable!("The cli parser should prevent reaching here"),
    }
}

fn run_setup_command(matches: &ArgMatches) {
    println!("{:?}", matches);
}
