use clap::ArgMatches;
use config::Config;
use error::Error;
use git2::Repository;
use std::env::current_dir;
use std::path::Path;

pub fn run(matches: &ArgMatches, config: Config, optional_path: Option<&Path>) -> Result<(), Error> {
    let default_path = try!(current_dir());
    let path = optional_path.unwrap_or(&default_path);
    let stage = matches.value_of("stage").expect("Expected required stage");
    let trekker = try!(trekker_name(path));

    println!("{:?}", config);
    println!("{}", stage);
    println!("{}", trekker);

    Ok(())
}

pub fn trekker_name(path: &Path) -> Result<String, Error> {
    let origin_remote_url = try!(origin_remote_url(path));
    origin_remote_url
        .split('/')
        .last()
        .map(|last_piece| last_piece.split('.'))
        .and_then(|last_piece_split| last_piece_split.rev().last())
        .map(|name| name.into())
        .ok_or(Error::GitRemoteUrl("Failed to find name from git remote".into()))
}

pub fn origin_remote_url(path: &Path) -> Result<String, Error> {
    let repo = try!(discover_repo(path));
    let origin = try!(repo.find_remote("origin"));

    origin
        .url()
        .map(|url| url.into())
        .ok_or(Error::GitRemoteUrl("No git remote origin found".into()))
}

fn discover_repo(path: &Path) -> Result<Repository, Error> {
    Repository::discover(path).map_err(Error::Git)
}
