use error::Error;

#[derive(Debug)]
pub struct Config {
    github_handle: String,
    github_token: String,
    token: String,
}

pub fn create_or_update_config(github_handle: &str, github_token: &str, token: &str) {
}

pub fn load_config() -> Result<Config, Error> {
    let config = Config {
        github_handle: "mikeastock".into(),
        github_token: "foobar".into(),
        token: "token".into(),
    };

    Ok(config)
}
