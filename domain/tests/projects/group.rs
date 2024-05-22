use domain::projects::{Group, RegisterGroup};

#[test]
fn from_register_project_test() {
    let name = "test_group_01";
    let input = RegisterGroup::new(name);
    let group = Group::from(&input);
    assert_eq!(36, group.id.len());
    assert_eq!(name.to_string(), group.name);
}
