use clap::{ArgMatches};
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

pub fn run(matches: &ArgMatches) {
    let github_handle = matches
        .value_of("handle")
        .expect("Expected required handle");

    let github_token = matches
        .value_of("token")
        .expect("Expected required token");

    let response = request_token(github_handle, github_token);
    println!("{:?}", response);
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
        response.read_to_string(&mut response_body);
        Err(Error::HttpError(response_body))
    }
}
