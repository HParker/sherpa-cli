use sherpa::deploy::{branch, origin_remote_url, trekker_name};
use support::project;

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
