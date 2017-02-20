use mockito::mock;
use sherpa::deploy::{branch, origin_remote_url, trekker_name};
use support::{project, ProjectBuilder};

#[test]
fn test_branch() {
    let expected_branch = "master";
    let project = project("test-branch").build();

    assert_eq!(branch(&project.path()).unwrap(), expected_branch.to_owned());
}

#[test]
fn test_trekker_name() {
    let expected_trekker_name = "example-trekker-name";
    let project = project(expected_trekker_name).build();

    assert_eq!(trekker_name(&project.path()).unwrap(), expected_trekker_name.to_owned());
}

#[test]
fn test_origin_remote_url() {
    let expected_trekker_name = "example-trekker-name";
    let project = project(expected_trekker_name).build();

    let expected_url = "git@github.com:example/example-trekker-name.git".to_owned();
    assert_eq!(origin_remote_url(&project.path()).unwrap(), expected_url);
}

#[test]
fn test_deploy() {
    let stubbed_repsonse = json!({
        "messages": [],
    });

    let mut mock = mock("POST", "/deploys");
    mock.with_status(201)
        .with_header("Content-Type", "accept/json")
        .with_body(&stubbed_repsonse.to_string())
        .create();

    let mut project = ProjectBuilder::new("deploy-test").build();
    project.setup_config();

    let stage = "staging1";

    let result = project
        .sherpa_command(&format!("-c {} deploy {}", project.config_path(), stage))
        .run();

    assert!(result.is_success(), result.failure_message("command to succeed"));

    mock.remove();
}
