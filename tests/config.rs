use mockito;
use mockito::mock;
use time::Duration;
use chrono::UTC;
use sherpa::config::{Config, load_config, save_config};
use tempdir::TempDir;

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

#[test]
fn test_validate() {
    let mocked_base_url = Some(mockito::SERVER_URL);

    let new_token = "brand-new-token";
    let new_expires_at = UTC::now() + Duration::days(2);

    let stubbed_repsonse = json!({
        "token": new_token,
        "expires_at": new_expires_at.to_rfc3339(),
    });

    mock("POST", "/token")
        .with_status(201)
        .with_header("Content-Type", "accept/json")
        .with_body(&stubbed_repsonse.to_string())
        .create();

    let github_handle = "mikeastock";
    let github_token = "some-github-token";
    let token = "some-expired-token";
    let expires_at = UTC::now() - Duration::days(2);

    let config = Config::new(github_handle, github_token, token, expires_at);

    let new_config = config.validate(mocked_base_url).unwrap();

    assert_eq!(new_config.token, new_token);
    assert_eq!(new_config.expires_at, new_expires_at);
}
