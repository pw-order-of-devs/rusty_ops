use persist::db_client::DbClient;
use serde_valid::json::json;

static VERSIONS_INDEX: &str = "versions";

pub async fn is_installed(db: &DbClient, version: &str) -> bool {
    db.get_one(VERSIONS_INDEX, json!({ "version": { "equals": version } }))
        .await
        .unwrap_or(None)
        .is_some()
}

pub async fn insert(db: &DbClient, version: &str) {
    let _ = db
        .create(
            VERSIONS_INDEX,
            &json!({
                "id": uuid::Uuid::new_v4().to_string(),
                "version": version,
            }),
        )
        .await;
}
