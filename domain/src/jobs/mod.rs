use async_graphql::{InputObject, Object};
use mongodb::bson::{doc};
use serde::{Deserialize, Serialize};

use crate::RODomainItem;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Job {
    id: String,
    description: Option<String>,
}

#[Object]
impl Job {

    async fn id(&self) -> String {
        self.clone().id
    }

    async fn description(&self) -> Option<String> {
        self.clone().description
    }
}

#[derive(Clone, Debug, InputObject, Serialize, Deserialize)]
pub struct RegisterJobModel {
    description: Option<String>,
}

impl RODomainItem for Job {}
impl RODomainItem for RegisterJobModel {}
