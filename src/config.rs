use error::Error;
use serde_json;
use std::env;
use std::fs::File;
use std::fs::create_dir_all;
use std::io::{Write, Read, Error as IoError};
use std::path::Path;

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq)]
pub struct Config {
    github_handle: String,
    github_token: String,
    token: String,
    expires_at: String,
}

impl Config {
    pub fn new(github_handle: &str, github_token: &str, token: &str, expires_at: &str) -> Config {
        Config {
            github_handle: github_handle.into(),
            github_token: github_token.into(),
            token: token.into(),
            expires_at: expires_at.into(),
        }
    }
}

pub fn save_config(config: Config, optional_path: Option<String>) -> Result<(), Error> {
    let path = match optional_path {
        Some(path) => path,
        None => default_path(),
    };

    let json = try!(serde_json::to_string(&config));
    save_file(path, json).map_err(Error::Io)
}

pub fn load_config(optional_path: Option<String>) -> Option<Config> {
    let path = match optional_path {
        Some(path) => path,
        None => default_path(),
    };

    match load_file(path) {
        Ok(json) => {
            match serde_json::from_str::<Config>(&json) {
                Ok(config) => Some(config),
                Err(_) => None,
            }
        },
        Err(_) => None,
    }
}

fn save_file(path: String, json: String) -> Result<(), IoError> {
    let config_path = Path::new(&path).join("config");
    let mut file = try!(File::create(config_path));
    file.write_all(json.as_bytes())
}

fn load_file(path: String) -> Result<String, IoError> {
    let mut file = try!(File::open(path.clone()));
    let mut json = String::new();
    try!(file.read_to_string(&mut json));
    Ok(json)
}

fn default_path() -> String {
    let home_path = env::home_dir().unwrap();
    let path = home_path.join(".sherpa");

    if !path.exists() {
        create_dir_all(path.clone()).unwrap();
    }

    path.to_str().unwrap().into()
}

#[cfg(test)]
mod test {
    extern crate tempdir;

    use super::{Config, load_config, save_config};
    use self::tempdir::TempDir;

    #[test]
    fn test_save_config() {
        let github_handle = "mikeastock";
        let github_token = "some-github-token";
        let token = "some-sherpa-token";
        let expires_at = "some-timestamp";

        let config = Config::new(github_handle, github_token, token, expires_at);

        let tempdir = TempDir::new("test_save_config").expect("Create temp dir");

        let tempdir_path_string = tempdir
            .path()
            .to_str()
            .unwrap()
            .to_owned();

        save_config(config.clone(), Some(tempdir_path_string));

        let config_path_string = tempdir
            .path()
            .join("config")
            .to_str()
            .unwrap()
            .into();

        let expected_config = load_config(Some(config_path_string));

        assert_eq!(expected_config, Some(config));

        tempdir.close().expect("Remove temp dir");
    }
}
