use serde;
use serde_json;
use std::path::Path;
use std::env;
use std::fs::create_dir_all;

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq)]
pub struct Config {
    github_handle: String,
    github_token: String,
    token: String,
    expires_at: String,
}

impl Config {
    fn new(github_handle: &str, github_token: &str, token: &str, expires_at: &str) -> Config {
        Config {
            github_handle: github_handle.into(),
            github_token: github_token.into(),
            token: token.into(),
            expires_at: expires_at.into(),
        }
    }
}

pub fn save_config(config: Config, optional_path: Option<String>) {
    let path = match optional_path {
        Some(path) => path,
        None => default_path(),
    };

    match serde_json::to_string(&config) {
        Ok(json) => {
            json;
        },
        Err(error) => {
            println!("{:?}", error);
        },
    }
}

pub fn load_config(optional_path: Option<String>) -> Option<Config> {
    let path = match optional_path {
        Some(path) => path,
        None => default_path(),
    };

    None
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

        let tempdir = TempDir::new("test_save_config").unwrap();

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

        tempdir.close();
    }
}
