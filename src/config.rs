use chrono::{DateTime, UTC};
use client;
use error::Error;
use serde_json;
use std::env;
use std::fs::File;
use std::fs::create_dir_all;
use std::io::{Write, Read, Error as IoError};
use std::path::Path;

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

    pub fn validate(self, base_url: Option<&str>, path: String) -> Result<Config, Error> {
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

pub fn save_config(config: Config, path: String) -> Result<Config, Error> {
    let json = try!(serde_json::to_string(&config));
    try!(save_file(path, json));
    Ok(config)
}

pub fn load_config(path: String) -> Option<Config> {
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

pub fn default_path() -> String {
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
    extern crate time;

    use chrono::UTC;
    use mockito;
    use self::tempdir::TempDir;
    use self::time::Duration;
    use super::{Config, save_config, load_config};

    #[test]
    fn test_is_expired() {
        let github_handle = "mikeastock";
        let github_token = "some-github-token";
        let token = "some-expired-token";
        let expires_at = UTC::now() - Duration::days(2);

        let config = Config::new(github_handle, github_token, token, expires_at);

        assert!(config.is_expired())
    }

    #[test]
    fn test_validate() {
        let tempdir = TempDir::new("test_validate").expect("Create temp dir");
        let tempdir_path_string = tempdir
            .path()
            .to_str()
            .unwrap()
            .to_owned();

        let mocked_base_url = Some(mockito::SERVER_URL);

        let new_token = "brand-new-token";
        let new_expires_at = UTC::now() + Duration::days(2);

        let stubbed_repsonse = json!({
            "token": new_token,
            "expires_at": new_expires_at.to_rfc3339(),
        });

        let mut mock = mockito::mock("POST", "/token");
        mock.with_status(201)
            .with_header("Content-Type", "accept/json")
            .with_body(&stubbed_repsonse.to_string())
            .create();

        let github_handle = "mikeastock";
        let github_token = "some-github-token";
        let token = "some-expired-token";
        let expires_at = UTC::now() - Duration::days(2);

        let config = Config::new(github_handle, github_token, token, expires_at);

        let new_config = config.validate(mocked_base_url, Some(tempdir_path_string)).unwrap();

        assert_eq!(new_config.token, new_token);
        assert_eq!(new_config.expires_at, new_expires_at);

        tempdir.close().expect("Remove temp dir");
        mock.remove();
    }

    #[test]
    fn test_save_config() {
        let github_handle = "mikeastock";
        let github_token = "some-github-token";
        let token = "some-sherpa-token";
        let expires_at = UTC::now();

        let config = Config::new(github_handle, github_token, token, expires_at);

        let tempdir = TempDir::new("test_save_config").expect("Create temp dir");

        let tempdir_path_string = tempdir
            .path()
            .to_str()
            .unwrap()
            .to_owned();

        save_config(config.clone(), Some(tempdir_path_string)).unwrap();

        let config_path_string = tempdir
            .path()
            .to_str()
            .unwrap()
            .into();

        let expected_config = load_config(Some(config_path_string));

        assert_eq!(expected_config, Some(config));

        tempdir.close().expect("Remove temp dir");
    }
}
