use domain::projects::{Project, RegisterProject};

#[test]
fn from_register_project_test() {
    let name = "test_01";
    let url = "http://dummy";
    let input = RegisterProject::new(name, url);
    let project = Project::from(&input);
    assert_eq!(36, project.id.len());
    assert_eq!(name.to_string(), project.name);
    assert!(project.url.is_some());
    assert_eq!(url.to_string(), project.url.unwrap());
}
