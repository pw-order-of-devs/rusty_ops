use std::future::Future;

use commons::errors::ROError;
use domain::RODomainItem;

use crate::mongo::MongoDBClient;

pub mod mongo;

pub enum DbType {
    MongoDb,
}

impl DbType {

    pub fn parse() -> Self {
        let db_type = std::env::var("DATABASE_TYPE")
            .expect("DATABASE_TYPE variable is required")
            .to_lowercase();

        match db_type.as_str() {
            "mongodb" | "mongo_db" => DbType::MongoDb,
            _ => panic!("Unsupported database: {}", db_type)
        }
    }
}

#[allow(opaque_hidden_inferred_bound)]
pub trait PersistenceBuilder {
    type PersistentType: Persistence;

    fn build() -> impl Future<Output=Self> + Send;
}

pub trait Persistence: Send + Sync {

    fn get_all<T: RODomainItem>(&self, index: &str) -> impl Future<Output=Result<Vec<T>, ROError>> + Send;

    fn get_by_id<T: RODomainItem>(&self, index: &str, id: &str) -> impl Future<Output=Result<Option<T>, ROError>> + Send;

    fn create<T: RODomainItem>(&self, index: &str, item: &T) -> impl Future<Output=Result<String, ROError>> + Send;

    fn delete(&self, index: &str, id: &str) -> impl Future<Output=Result<u64, ROError>> + Send;
}

pub async fn init() -> impl Persistence + Send + Sync {
    match DbType::parse() {
        DbType::MongoDb => MongoDBClient::build().await
    }
}
