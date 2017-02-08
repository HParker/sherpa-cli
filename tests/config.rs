use sherpa::config::{Config, load_config, save_config};
use tempdir::TempDir;

#[test]
fn test_save_config() {
    let github_handle = "mikeastock";
    let github_token = "some-github-token";
    let token = "some-sherpa-token";
    let expires_at = "some-timestamp";

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
