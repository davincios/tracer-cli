use anyhow::Result;
use serde_json::json;

use crate::TracerConfig;

use self::events::{send_event, EventStatus};

// Declare submodules as private
mod events;
mod utils;

pub struct Tool {
    pub name: String,
    pub version: String,
}

pub async fn pipeline_new_run(config: &TracerConfig, msg: &str) -> Result<()> {
    send_event(
        config,
        EventStatus::NewRun.as_str(),
        &format!("Initialized pipeline run with name: {}", msg),
        None,
    )
    .await
}

pub async fn tool_process(config: &TracerConfig, tool: &Tool) -> Result<()> {
    let properties = json!({
        "tool_version": tool.version
    });

    send_event(
        config,
        EventStatus::ToolExecution.as_str(),
        &format!("Tool process: {}", tool.name),
        Some(properties),
    )
    .await
}

pub async fn log_message(config: &TracerConfig, message: &str) -> Result<()> {
    send_event(
        config,
        EventStatus::RunStatusMessage.as_str(),
        message,
        None,
    )
    .await
}

pub async fn pipeline_finish_run(config: &TracerConfig) -> Result<()> {
    send_event(
        config,
        EventStatus::FinishedRun.as_str(),
        "Pipeline run concluded successfully",
        None,
    )
    .await
}
