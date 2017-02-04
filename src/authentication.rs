use clap::{ArgMatches};
use reqwest;
use std::collections::HashMap;

const BASE_URL: &'static str = "https://sherpa.procoretech.com/api/v1";

pub fn run(matches: &ArgMatches) {
    let github_token = matches
        .value_of("token")
        .expect("Expected required token");

    let token = request_token(github_token);
    println!("{:?}", token);
}

fn request_token(github_token: &str) -> Result<HashMap<String, String>, reqwest::Error> {
    let client = try!(reqwest::Client::new());
    let url = BASE_URL.to_owned() + "/token";

    let mut nested_body = HashMap::new();
    nested_body.insert("access_token", github_token);

    let mut body = HashMap::new();
    body.insert("github", nested_body);

    let response = client.post(&url).json(&body).send();

    try!(response.map(|mut res| res.json()))
}
