pub mod jobs;
pub mod pipelines;
pub mod projects;

pub trait RODomainItem: Send + Sync + std::fmt::Debug + Unpin
+ serde::ser::Serialize + for<'de> serde::de::Deserialize<'de> {}
