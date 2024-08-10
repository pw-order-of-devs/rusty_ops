use std::collections::HashMap;

use async_graphql::indexmap::IndexMap;
use serde::{Deserialize, Serialize};

use commons::errors::RustyError;

/// Pipeline script
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Script {
    /// pipeline stage commands
    pub script: Vec<String>,
}

impl Script {
    /// constructor
    #[must_use]
    pub fn new(script: &[String]) -> Self {
        Self {
            script: script.to_vec(),
        }
    }
}

/// Pipeline stage
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Stage {
    /// pipeline stage environment variables
    pub env: Option<HashMap<String, String>>,
    /// pipeline stage commands
    pub script: Vec<String>,
    /// pipeline dependencies
    #[serde(rename(deserialize = "dependsOn", deserialize = "depends_on"))]
    pub depends_on: Option<Vec<String>>,
}

/// Pipeline template
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PipelineTemplate {
    /// pipeline docker image
    pub image: Option<String>,
    /// pipeline environment variables
    pub env: Option<HashMap<String, String>>,
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
            errors.push("stages cannot be empty");
        } else {
            let stage_names = result
                .stages
                .iter()
                .map(|(s, _)| s.to_string())
                .collect::<Vec<String>>();
            result.stages.iter().for_each(|(name, stage)| {
                if stage.script.is_empty() {
                    errors.push("stages.script cannot be empty");
                }
                if let Some(depends_on) = stage.clone().depends_on {
                    if depends_on.iter().any(|s| !stage_names.contains(s)) {
                        errors.push("stage depends on an unknown stage");
                    }
                    if depends_on.iter().any(|s| s == name) {
                        errors.push("stage cannot depend on itself");
                    }
                }
            });
        }

        if let Some(before) = result.clone().before {
            if before.script.is_empty() {
                errors.push("before.script cannot be empty");
            }
        }

        if let Some(after) = result.clone().after {
            if after.script.is_empty() {
                errors.push("after.script cannot be empty");
            }
        }

        if errors.is_empty() {
            Ok(result)
        } else {
            Err(RustyError::SerializationError(
                format!("Pipeline template: {errors:?}").replace('\"', ""),
            ))
        }
    }

    /// Build dependency tree of stages to run
    #[must_use]
    pub fn dependency_tree(&self) -> Vec<Vec<String>> {
        let mut stages = self.clone().stages;
        let mut results: Vec<Vec<String>> = vec![];

        let total = stages.len();
        while results.iter().flatten().count() != total {
            let deps_stage = stages
                .clone()
                .into_iter()
                .filter(|(_, stage)| {
                    let deps = stage.clone().depends_on.unwrap_or(vec![]);
                    deps.iter()
                        .all(|d| results.clone().into_iter().flatten().any(|r| r.contains(d)))
                })
                .map(|(name, _)| name)
                .collect::<Vec<String>>();
            results.push(deps_stage.clone());
            for dep in deps_stage.clone() {
                stages.shift_remove(&dep);
            }
        }

        results
    }
}
