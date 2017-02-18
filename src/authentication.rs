use clap::ArgMatches;
use config::{Config, save_config};
use error::Error;
use client::authenticate;

pub fn run(matches: &ArgMatches, config_path: String) -> Result<(), Error> {
    let github_handle = matches
        .value_of("handle")
        .expect("Expected required handle");

    let github_token = matches
        .value_of("token")
        .expect("Expected required token");

    let response = try!(authenticate(None, github_handle, github_token));
    let config = Config::new(
        github_handle,
        github_token,
        &response.token,
        response.expires_at);

    try!(save_config(config, config_path));

    Ok(())
}
