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

pub(crate) async fn create(db: &DbClient) {
    if let DbClient::PostgreSql(client) = db {
        log::info!("[postgresql] creating database schema: start");
        let init_script_path = var_or_default(
            "POSTGRESQL_INIT_SCRIPT_PATH",
            "/app/pg/init.sql".to_string(),
        );
        let sql = std::fs::read_to_string(&init_script_path)
            .expect("[postgresql] error while opening init script");
        match client.execute_sql(&sql).await {
            Ok(()) => log::info!("[postgresql] creating database schema: done"),
            Err(err) => panic!("[postgresql] error while initializing database schema: {err}"),
        };
    }
}
