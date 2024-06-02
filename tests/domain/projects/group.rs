use domain::projects::{Group, RegisterGroup};
use domain::RustyDomainItem;

#[test]
fn from_register_project_test() {
    let name = "test_group_01";
    let input = RegisterGroup::new(name);
    let group = Group::from(&input);
    assert_eq!(36, group.get_id().len());
    assert_eq!(name.to_string(), group.name);
}
