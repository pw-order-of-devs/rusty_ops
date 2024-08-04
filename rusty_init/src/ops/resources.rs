use serde_valid::json::json;

use persist::db_client::DbClient;

const RESOURCES_INDEX: &str = "resources";

pub async fn create_resource(db: &DbClient, name: &str, rights: &[&str]) {
    log::info!("creating `{name}` resource: start");
    let resource = json!({
        "id": uuid::Uuid::new_v4().to_string(),
        "name": name.to_string(),
        "rights": rights.iter().map(|it| (*it).to_string()).collect::<Vec<String>>(),
    });
    if let Ok(None) = db
        .get_one(
            RESOURCES_INDEX,
            json!({
                "name": { "equals": name },
            }),
        )
        .await
    {
        match db.create(RESOURCES_INDEX, &resource).await {
            Ok(_) => log::info!("creating `{name}` resource: done"),
            Err(err) => panic!("error while creating resource `{name}`: `{err}`"),
        }
    } else {
        log::warn!("resource `{name}` already exists: skipping");
    }
}
