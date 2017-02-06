use error::Error;
use reqwest::Client;
use std::collections::HashMap;
use std::io::Read;

const BASE_URL: &'static str = "https://sherpa.procoretech.com/api/v1";

#[derive(Debug, Deserialize)]
pub struct AuthenticateResponse {
    pub token: String,
    pub expires_at: String,
}

pub fn authenticate(github_handle: &str, github_token: &str) -> Result<AuthenticateResponse, Error> {
    let client = try!(Client::new());
    let url = BASE_URL.to_owned() + "/token";

    let mut nested_body = HashMap::new();
    nested_body.insert("handle", github_handle);
    nested_body.insert("access_token", github_token);

    let mut body = HashMap::new();
    body.insert("github", nested_body);

    let mut response = try!(client.post(&url).json(&body).send());

    if response.status().is_success() {
        Ok(try!(response.json::<AuthenticateResponse>()))
    } else {
        let mut response_body = String::new();
        try!(response.read_to_string(&mut response_body));
        Err(Error::Http(response_body))
    }
}
