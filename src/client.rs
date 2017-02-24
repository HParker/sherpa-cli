use chrono::{DateTime, UTC};
use config::Config;
use error::Error;
#[cfg(any(test, feature = "mock"))]
use mockito;
use reqwest::Client;
use reqwest::header::Authorization;
use std::collections::HashMap;
use std::io::Read;

#[cfg(any(test, feature = "mock"))]
const DEFAULT_BASE_URL: &'static str = mockito::SERVER_URL;
#[cfg(not(any(test, feature = "mock")))]
const DEFAULT_BASE_URL: &'static str = "https://sherpa.procoretech.com/api/v1";

#[derive(Debug, Deserialize)]
pub struct AuthenticateResponse {
    pub token: String,
    pub expires_at: DateTime<UTC>,
}

#[derive(Debug, Deserialize)]
pub struct CreateDeployResponse {
    pub messages: Vec<String>,
}

pub fn authenticate(base_url: Option<&str>,
                    github_handle: &str,
                    github_token: &str)
                    -> Result<AuthenticateResponse, Error> {
    let client = try!(Client::new());
    let base_url = base_url.unwrap_or(DEFAULT_BASE_URL);
    let url = base_url.to_owned() + "/token";

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

pub fn create_deploy(base_url: Option<&str>,
                     trekker: &str,
                     stage: &str,
                     branch: &str,
                     config: Config,
                     dry_run: bool)
                     -> Result<CreateDeployResponse, Error> {
    let config = try!(config.validate(base_url));

    let client = try!(Client::new());
    let base_url = base_url.unwrap_or(DEFAULT_BASE_URL);
    let url = base_url.to_owned() + "/deploys";

    let mut deploy = HashMap::new();
    deploy.insert("trekker_name", trekker);
    deploy.insert("stage_name", stage);
    deploy.insert("branch", branch);

    let mut body = HashMap::new();
    body.insert("deploy", deploy);

    if dry_run {
        println!("{:?}", url);
        println!("{:?}", body);
        Ok(CreateDeployResponse{messages: vec!["".to_string()]})
    } else {
        let request = client.post(&url)
            .header(Authorization(format!("Token token={}", config.token)))
            .json(&body);

        let mut response = try!(request.send());

        if response.status().is_success() {
            Ok(try!(response.json::<CreateDeployResponse>()))
        } else {
            let mut response_body = String::new();
            try!(response.read_to_string(&mut response_body));
            Err(Error::Http(response_body))
        }
    }
}
