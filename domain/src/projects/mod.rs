use async_graphql::{InputObject, SimpleObject};
use serde::{Deserialize, Serialize};

use crate::jobs::Job;
use crate::RustyDomainItem;

/// A struct representing a project.
#[derive(Clone, Debug, SimpleObject, Serialize, Deserialize)]
pub struct Project {
    /// project id
    pub id: String,
    /// project name
    pub name: String,
    url: Option<String>,
    jobs: Option<Vec<Job>>,
}

/// A struct representing the registration of a project.
#[derive(Clone, Debug, InputObject, Serialize, Deserialize)]
pub struct RegisterProject {
    name: String,
    url: String,
}

impl From<&RegisterProject> for Project {
    fn from(value: &RegisterProject) -> Self {
        Self {
            id: Self::generate_id(),
            name: value.clone().name,
            url: Some(value.clone().url),
            jobs: None,
        }
    }
}

impl RustyDomainItem for Project {

    fn id(&self) -> String {
        self.clone().id
    }
}
impl RustyDomainItem for RegisterProject {

    fn id(&self) -> String {
        todo!()
    }
}