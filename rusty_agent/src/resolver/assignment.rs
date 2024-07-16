use serde_json::Value;

use commons::errors::RustyError;

use crate::api::pipelines as api;

pub async fn assign_pipeline(uuid: &str, text: &str) -> Result<(), RustyError> {
    log::trace!("Obtained message: {text}");
    let message = serde_json::from_str::<Value>(text)?;
    if let Some(message) = message["payload"]["data"]["pipelines"].as_object() {
        if let Some(id) = message.get("id") {
            let res = api::assign_pipeline(id.as_str().unwrap_or_default(), uuid).await;
            log::trace!("assign pipeline result: {res:?}");
        } else {
            log::warn!("Error while parsing pipeline to assign - missing id");
        };
    } else {
        log::warn!("Error while parsing pipeline to assign");
    }
    Ok(())
}
