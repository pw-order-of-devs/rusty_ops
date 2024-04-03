use serde_json::Value;

use commons::errors::RustyError;
use domain::pipelines::Pipeline;

use crate::api::pipelines as api;

pub async fn assign_pipeline(uuid: &str, text: &str) -> Result<(), RustyError> {
    log::trace!("Obtained message: {text}");
    let message = serde_json::from_str::<Value>(text)?;
    let message = message["payload"]["data"]["pipelines"].clone();
    match serde_json::from_value::<Option<Pipeline>>(message) {
        Ok(pipeline) => {
            if let Some(pipeline) = pipeline {
                log::trace!("Parsed pipeline: {pipeline:?}");
                let res = api::assign_pipeline(&pipeline.id, uuid).await;
                log::trace!("assign pipeline result: {res:?}");
            } else {
                log::trace!("Received `None` instead of pipeline. Subscription mode might not be supported by server.");
            }
        }
        Err(err) => log::warn!("Error while parsing message: {err}"),
    };
    Ok(())
}
