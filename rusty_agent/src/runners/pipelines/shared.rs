use serde_json::json;
use std::collections::HashMap;

use domain::pipelines::PipelineStatus;
use domain::templates::pipeline::{PipelineTemplate, Stage};
use messaging::mq_client::MqClient;

use crate::api::pipelines::{finalize, update_stage};

pub const WORKING_DIR: &str = "/tmp/rusty";

pub async fn cleanup(
    messaging: &MqClient,
    uuid: &str,
    pipeline_id: &str,
    stage_name: &str,
    status: PipelineStatus,
) {
    let _ = std::fs::remove_dir_all(format!("{WORKING_DIR}/{pipeline_id}"));
    let _ = update_stage(pipeline_id, uuid, stage_name, status).await;
    let _ = finalize(pipeline_id, uuid, status).await;
    let _ = messaging
        .publish(&format!("pipeline-logs-{pipeline_id}"), "EOF")
        .await;
}

pub async fn print_line(messaging: &MqClient, pipeline_id: &str, stage: &str, line: &str) {
    let _ = messaging
        .publish(
            &format!("pipeline-logs-{pipeline_id}"),
            &json!({ "stage": stage, "line": line }).to_string(),
        )
        .await;
    log::debug!("{line}");
}

pub fn prepare_env(template: &PipelineTemplate, stage: &Option<Stage>) -> HashMap<String, String> {
    let mut envs = HashMap::new();
    if let Some(env) = template.clone().env {
        for (k, v) in env {
            envs.insert(k, v);
        }
    }
    if let Some(stage) = stage {
        if let Some(env) = stage.clone().env {
            for (k, v) in env {
                envs.insert(k, v);
            }
        }
    }
    envs
}
