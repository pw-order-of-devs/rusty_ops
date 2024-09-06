use rstest::rstest;
use serde_valid::Validate;

use domain::projects::{Project, RegisterProject, Source};

#[test]
fn from_register_project_test() {
    let name = "test_01";
    let url = "http://dummy";
    let input = RegisterProject::new(&Source::Internal, &Some(name.to_string()), url);
    let project = Project::from(&input);
    assert_eq!(36, project.id.len());
    assert_eq!(Some(name.to_string()), project.name);
    assert!(project.url.is_some());
    assert_eq!(url.to_string(), project.url.unwrap());
}

#[rstest]
#[case(RegisterProject::new(&Source::Internal, &Some("new project".to_string()), "http://dummy.ext"), true)]
#[case(RegisterProject::new(&Source::Internal, &Some(String::new()), "http://dummy.ext"), false)]
#[case(RegisterProject::new(&Source::Internal, &Some("new project".to_string()), ""), false)]
#[case(RegisterProject::new(&Source::Internal, &Some("new project".to_string()), "http"), false)]
#[case(RegisterProject::new(&Source::Internal, &Some("new project".to_string()), "x://dummy.ext"), true)]
#[case(RegisterProject::new(&Source::GitHub, &None, "x://dummy.ext"), true)]
fn validate_project_test(#[case] project: RegisterProject, #[case] expected: bool) {
    assert_eq!(expected, project.validate().is_ok())
}
