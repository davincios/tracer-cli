use anyhow::Result;
use serde_json::{json, Value};

use super::{config::AppConfig, utils::handle_response};

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
    event_type: &str,
    message: &str,
    event_status: Option<EventStatus>,
    properties: Option<Value>,
) -> Result<()> {
    let status_str = event_status.map_or("", |status| status.as_str());
    let mut data = json!({
        "logs": [{
            "message": message,
            "event_type": event_type,
            "process_type": "pipeline",
            "process_status": status_str,
        }]
    });

    // Add properties if provided
    if let Some(props) = properties {
        data["logs"][0]["properties"] = props;
    }

    let response = config
        .client
        .post(&config.base_url)
        .header("x-api-key", &config.api_key)
        .header("Content-Type", "application/json")
        .json(&data)
        .send()
        .await?;

    handle_response(response, event_type).await
}
