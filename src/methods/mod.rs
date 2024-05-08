use anyhow::Result;
use serde_json::json;

use crate::TracerAppConfig;

pub use self::events::{send_event, EventStatus};

// Declare submodules as private
mod events;
mod utils;

pub struct Tool {
    pub name: String,
    pub version: String,
}

pub async fn pipeline_new_run(config: &TracerAppConfig, msg: &str) -> Result<()> {
    send_event(
        config,
        EventStatus::NewRun.as_str(),
        &format!("Initialized pipeline run with name: {}", msg),
        None,
        false,
    )
    .await
}

pub async fn tool_process(config: &TracerAppConfig, tool: &Tool) -> Result<()> {
    let properties = json!({
        "tool_version": tool.version,
        "tool_name": tool.name,
    });

    send_event(
        config,
        EventStatus::ToolExecution.as_str(),
        &format!("Tool process: {}", tool.name),
        Some(properties),
        false,
    )
    .await
}

pub async fn log_message(config: &TracerAppConfig, message: &str) -> Result<()> {
    send_event(
        config,
        EventStatus::RunStatusMessage.as_str(),
        message,
        None,
        false,
    )
    .await
}

pub async fn pipeline_finish_run(config: &TracerAppConfig) -> Result<()> {
    send_event(
        config,
        EventStatus::FinishedRun.as_str(),
        "Pipeline run concluded successfully",
        None,
        false,
    )
    .await
}
