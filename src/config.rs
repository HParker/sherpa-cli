use chrono::{DateTime, UTC};
use error::Error;
use serde_json;
use std::env;
use std::fs::File;
use std::fs::create_dir_all;
use std::io::{Write, Read, Error as IoError};
use std::path::Path;
use client;

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq)]
pub struct Config {
    pub github_handle: String,
    pub github_token: String,
    pub token: String,
    pub expires_at: DateTime<UTC>,
}

impl Config {
    pub fn new(github_handle: &str, github_token: &str, token: &str, expires_at: DateTime<UTC>) -> Config {
        Config {
            github_handle: github_handle.into(),
            github_token: github_token.into(),
            token: token.into(),
            expires_at: expires_at,
        }
    }

    pub fn validate(self, base_url: Option<&str>, path: Option<String>) -> Result<Config, Error> {
        if self.is_expired() {
            let response = try!(client::authenticate(base_url, &self.github_handle, &self.github_token));
            let config = Config::new(
                &self.github_handle,
                &self.github_token,
                &response.token,
                response.expires_at);
            save_config(config, path)
        } else {
            Ok(self)
        }
    }

    pub fn is_expired(&self) -> bool {
        UTC::now() > self.expires_at
    }
}

pub fn save_config(config: Config, path: Option<String>) -> Result<Config, Error> {
    let path = path.unwrap_or(default_path());

    let json = try!(serde_json::to_string(&config));
    try!(save_file(path, json));
    Ok(config)
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
    let mut file = try!(File::open(Path::new(&path).join("config").clone()));
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
    extern crate time;

    use chrono::UTC;
    use super::Config;
    use self::time::Duration;

    #[test]
    fn test_is_expired() {
        let github_handle = "mikeastock";
        let github_token = "some-github-token";
        let token = "some-expired-token";
        let expires_at = UTC::now() - Duration::days(2);

        let config = Config::new(github_handle, github_token, token, expires_at);

        assert!(config.is_expired())
    }
}
