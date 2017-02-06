use clap::ArgMatches;
use config::{Config, save_config};
use error::Error;
use reqwest::Client;
use std::collections::HashMap;
use std::io::Read;

const BASE_URL: &'static str = "https://sherpa.procoretech.com/api/v1";

#[derive(Debug, Deserialize)]
struct TokenResponse {
    token: String,
    expires_at: String,
}

pub fn run(matches: &ArgMatches) -> Result<(), Error> {
    let github_handle = matches
        .value_of("handle")
        .expect("Expected required handle");

    let github_token = matches
        .value_of("token")
        .expect("Expected required token");

    let token_response = try!(request_token(github_handle, github_token));
    let config = Config::new(
        github_handle,
        github_token,
        &token_response.token,
        &token_response.expires_at);

    save_config(config, None)
}

fn request_token(github_handle: &str, github_token: &str) -> Result<TokenResponse, Error> {
    let client = try!(Client::new());
    let url = BASE_URL.to_owned() + "/token";

    let mut nested_body = HashMap::new();
    nested_body.insert("handle", github_handle);
    nested_body.insert("access_token", github_token);

    let mut body = HashMap::new();
    body.insert("github", nested_body);

    let mut response = try!(client.post(&url).json(&body).send());

    if response.status().is_success() {
        Ok(try!(response.json::<TokenResponse>()))
    } else {
        let mut response_body = String::new();
        try!(response.read_to_string(&mut response_body));
        Err(Error::HttpError(response_body))
    }
}
