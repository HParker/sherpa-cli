use chrono::UTC;
use mockito::mock;
use support::ProjectBuilder;
use time::Duration;

#[test]
fn test_auth() {
    let new_token = "new-token-test-auth";
    let new_expires_at = UTC::now() + Duration::days(2);

    let stubbed_repsonse = json!({
        "token": new_token,
        "expires_at": new_expires_at.to_rfc3339(),
    });

    let mut mock = mock("POST", "/token");
    mock.with_status(201)
        .with_header("Content-Type", "accept/json")
        .with_body(&stubbed_repsonse.to_string())
        .create();

    let github_handle = "mikeastock";
    let github_token = "some-token";

    let project = ProjectBuilder::new("auth-test").build();
    let config_path = project.path().join("config");

    let result = project
        .sherpa_command(&format!("-c {} auth {} {}", config_path.to_str().unwrap(), github_handle, github_token))
        .run();

    assert!(result.is_success(), result.failure_message("command to succeed"));

    mock.remove();
}
