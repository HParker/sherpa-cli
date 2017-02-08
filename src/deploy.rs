use clap::ArgMatches;
use config::Config;
use error::Error;
use git2::Repository;
use std::env::current_dir;

pub fn run(matches: &ArgMatches, config: Config) -> Result<(), Error> {
    let stage = matches.value_of("stage").expect("Expected required stage");
    let trekker = try!(trekker_name());

    println!("{:?}", config);
    println!("{}", stage);
    println!("{}", trekker);

    Ok(())
}

pub fn trekker_name() -> Result<String, Error> {
    let origin_remote_url = try!(origin_remote_url());
    origin_remote_url
        .split('/')
        .last()
        .map(|last_piece| last_piece.split('.'))
        .and_then(|last_piece_split| last_piece_split.rev().last())
        .map(|name| name.into())
        .ok_or(Error::GitRemoteUrl("Failed to find name from git remote".into()))
}

pub fn origin_remote_url() -> Result<String, Error> {
    let repo = try!(discover_repo());
    let origin = try!(repo.find_remote("origin"));

    origin
        .url()
        .map(|url| url.into())
        .ok_or(Error::GitRemoteUrl("No git remote origin found".into()))
}

fn discover_repo() -> Result<Repository, Error> {
    let current_dir = try!(current_dir());
    Repository::discover(current_dir).map_err(Error::Git)
}
