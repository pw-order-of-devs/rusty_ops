pub mod jobs;

pub trait RODomainItem: Send + Sync + std::fmt::Debug
+ serde::ser::Serialize + for<'de> serde::de::Deserialize<'de> {}
