use async_graphql::indexmap::IndexMap;
use commons::errors::RustyError;
use serde::{Deserialize, Serialize};

/// Pipeline script
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Script {
    /// pipeline stage commands
    pub script: Vec<String>,
}

/// Pipeline stage
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Stage {
    /// pipeline stage docker image
    pub image: Option<String>,
    /// pipeline stage commands
    pub script: Vec<String>,
}

/// Pipeline template
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PipelineTemplate {
    /// pipeline docker image
    pub image: Option<String>,
    /// pipeline before stage
    pub before: Option<Script>,
    /// pipeline after stage
    pub after: Option<Script>,
    /// pipeline stages
    pub stages: IndexMap<String, Stage>,
}

impl PipelineTemplate {
    /// Validate pipeline from yaml
    ///
    /// # Errors
    ///
    /// This function can generate the following errors:
    ///
    /// * `RustyError` - If there was an error during the creation of the item.
    pub fn from_yaml(text: &str) -> Result<Self, RustyError> {
        let text = String::from_utf8(base64_url::decode(text)?)?;
        let result = serde_yaml::from_str::<Self>(&text)?;

        let mut errors: Vec<&str> = vec![];
        if result.stages.is_empty() {
            errors.push("Pipeline template: stages cannot be empty");
        } else {
            result.stages.iter().for_each(|(name, stage)| {
                if name.is_empty() {
                    errors.push("Pipeline template: stages.name cannot be empty");
                }
                if stage.script.is_empty() {
                    errors.push("Pipeline template: stages.script cannot be empty");
                }
            });
        }

        if let Some(before) = result.clone().before {
            if before.script.is_empty() {
                errors.push("Pipeline template: before.scripts cannot be empty");
            }
        }

        if let Some(after) = result.clone().after {
            if after.script.is_empty() {
                errors.push("Pipeline template: after_each.scripts cannot be empty");
            }
        }

        if errors.is_empty() {
            Ok(result)
        } else {
            Err(RustyError::SerializationError(errors.join(";")))
        }
    }
}
