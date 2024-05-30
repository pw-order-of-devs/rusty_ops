use rstest::rstest;
use serde_valid::Validate;

use domain::projects::{Project, RegisterProject};
use domain::RustyDomainItem;

#[test]
fn from_register_project_test() {
    let name = "test_01";
    let url = "http://dummy";
    let input = RegisterProject::new(name, url);
    let project = Project::from(&input);
    assert_eq!(36, project.get_id().len());
    assert_eq!(name.to_string(), project.name);
    assert!(project.url.is_some());
    assert_eq!(url.to_string(), project.url.unwrap());
}

#[rstest]
#[case(RegisterProject::new("new project", "http://dummy.ext"), true)]
#[case(RegisterProject::new("", "http://dummy.ext"), false)]
#[case(RegisterProject::new("new project", ""), false)]
#[case(RegisterProject::new("new project", "http"), false)]
#[case(RegisterProject::new("new project", "x://dummy.ext"), true)]
fn validate_project_test(#[case] project: RegisterProject, #[case] expected: bool) {
    assert_eq!(expected, project.validate().is_ok())
}
