use deploy::{origin_remote_url, trekker_name};

#[test]
fn test_trekker_name() {
    assert_eq!(trekker_name().unwrap(), "sherpa-cli".to_owned());
}

#[test]
fn test_origin_remote_url() {
    let expected_url = "git@github.com:mikeastock/sherpa-cli.git".to_owned();
    assert_eq!(origin_remote_url().unwrap(), expected_url);
}
