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

fn trekker_name() -> Result<String, Error> {
    Ok("sherpa-cli".into())
}

fn origin_remote_url() -> Result<String, Error> {
    let repo = try!(discover_repo());
    let origin = try!(repo.find_remote("origin"));

    match origin.url() {
        Some(url) => Ok(url.into()),
        None => Err(Error::GitRemoteUrl("Failed".into())),
    }
}

fn discover_repo() -> Result<Repository, Error> {
    let current_dir = try!(current_dir());
    Repository::discover(current_dir).map_err(Error::Git)
}

#[cfg(test)]
mod test {
    use super::{origin_remote_url, trekker_name};

    #[test]
    fn test_trekker_name() {
        assert_eq!(trekker_name().unwrap(), "sherpa-cli".to_owned());
    }

    #[test]
    fn test_origin_remote_url() {
        let expected_url = "git@github.com:mikeastock/sherpa-cli.git".to_owned();
        assert_eq!(origin_remote_url().unwrap(), expected_url);
    }
}
