use clap::{ArgMatches};
use reqwest::{Client, Error as ReqwestError};
use std::collections::HashMap;

const BASE_URL: &'static str = "https://sherpa.procoretech.com/api/v1";

#[derive(Debug, Deserialize)]
struct TokenResponse {
    token: String,
    expires_at: String,
}

pub fn run(matches: &ArgMatches) {
    let github_token = matches
        .value_of("token")
        .expect("Expected required token");

    let response = request_token(github_token);
    println!("{:?}", response);
}

fn request_token(github_token: &str) -> Result<TokenResponse, ReqwestError> {
    let client = try!(Client::new());
    let url = BASE_URL.to_owned() + "/token";

    let mut nested_body = HashMap::new();
    nested_body.insert("access_token", github_token);

    let mut body = HashMap::new();
    body.insert("github", nested_body);

    let response = client.post(&url).json(&body).send();

    try!(response.map(|mut res| res.json::<TokenResponse>()))
}
