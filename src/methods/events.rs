use anyhow::Result;
use serde_json::{json, Value};

use crate::AppConfig;

use super::utils::handle_response;

#[derive(Debug)]
pub enum EventStatus {
    NewRun,
    FinishedRun,
    RunStatusMessage,
    ToolExecution,
}

impl EventStatus {
    pub fn as_str(&self) -> &'static str {
        match self {
            EventStatus::NewRun => "new_run",
            EventStatus::FinishedRun => "finished_run",
            EventStatus::RunStatusMessage => "run_status_message",
            EventStatus::ToolExecution => "tool_execution",
        }
    }
}

pub async fn send_event(
    // keep send_event private
    config: &AppConfig,
    process_status: &str,
    message: &str,
    attributes: Option<Value>,
) -> Result<()> {
    let mut data = json!({
        "logs": [{
            "message": message,
            "event_type": "process_status",
            "process_type": "pipeline",
            "process_status": process_status,
        }]
    });

    // Add attributes if provided
    if let Some(props) = attributes {
        data["logs"][0]["attributes"] = props;
    }

    let response = config
        .client
        .post(&config.base_url)
        .header("x-api-key", &config.api_key)
        .header("Content-Type", "application/json")
        .json(&data)
        .send()
        .await?;

    handle_response(response, process_status).await
}
