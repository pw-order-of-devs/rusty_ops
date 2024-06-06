use domain::auth::roles::Role;
use persist::db_client::DbClient;

const ROLES_INDEX: &str = "roles";

pub async fn create_role(db: &DbClient, name: &str, description: &str, users: &[&str]) -> String {
    log::info!("creating `{name}` role: start");
    let role = Role {
        id: uuid::Uuid::new_v4().to_string(),
        name: name.to_string(),
        description: Some(description.to_string()),
        users: users.iter().map(|it| (*it).to_string()).collect(),
    };
    match db.create(ROLES_INDEX, &role).await {
        Ok(id) => {
            log::info!("creating `{name}` role: done");
            id
        }
        Err(err) => panic!("error while creating role `{name}`: `{err}`"),
    }
}
