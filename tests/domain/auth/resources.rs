use domain::auth::resources::Resource;
use domain::RustyDomainItem;

#[test]
fn get_id_test() {
    let resource = Resource {
        id: uuid::Uuid::new_v4().to_string(),
        name: "resource".to_string(),
        rights: vec!["right".to_string()],
    };
    assert_eq!("resource", resource.get_id());
}
