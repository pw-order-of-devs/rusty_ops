use domain::auth::resources::Resource;
use domain::RustyDomainItem;

#[test]
fn get_id_test() {
    let resource = Resource {
        name: "resource".to_string(),
        rights: vec!["right".to_string()],
    };
    assert_eq!("resource", resource.get_id());
}
