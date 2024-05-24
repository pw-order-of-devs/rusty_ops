use commons::env::var_or_default;
use persist::db_client::DbClient;

pub(crate) async fn purge_db(db: &DbClient) {
    if var_or_default("WIPE_DATA", false) {
        log::info!("cleaning up database: start");
        match db.purge().await {
            Ok(()) => log::info!("cleaning up database: done"),
            Err(err) => panic!("error while cleaning up database: {err}"),
        };
    }
}

pub(crate) async fn execute_sql(db: &DbClient, version: &str) {
    if let DbClient::PostgreSql(client) = db {
        let script_path = var_or_default("POSTGRESQL_SCRIPTS_PATH", "/app/pg/sql".to_string());
        let script_path = format!("{script_path}/v{version}.sql");
        log::info!("[postgresql] executing `{script_path}` script: start");
        let sql = std::fs::read_to_string(&script_path)
            .expect("[postgresql] error while opening init script");
        match client.execute_sql(&sql).await {
            Ok(()) => log::info!("[postgresql] executing `{script_path}` script: done"),
            Err(err) => panic!("[postgresql] error while initializing database schema: {err}"),
        };
    }
}
