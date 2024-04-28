use anyhow::Result;
use serde_json::json;

use self::events::{send_event, EventStatus};

// Declare submodules as private
mod config;
mod events;
mod utils;

pub use config::AppConfig;

pub struct Tool {
    pub name: String,
    pub version: String,
    pub flags: Vec<String>,
}

pub async fn pipeline_init(config: &AppConfig, msg: &str) -> Result<()> {
    send_event(
        config,
        EventStatus::NewRun.as_str(),
        &format!("Initialized pipeline run with name: {}", msg),
        None,
    )
    .await
}

pub async fn tool_process(config: &AppConfig, tool: &Tool) -> Result<()> {
    let properties = json!({
        "tool_version": tool.version,
        "flags": tool.flags
    });

    send_event(
        config,
        EventStatus::ToolExecution.as_str(),
        &format!("Tool process: {}", tool.name),
        Some(properties),
    )
    .await
}

pub async fn log_message(config: &AppConfig, message: &str) -> Result<()> {
    send_event(
        config,
        EventStatus::RunStatusMessage.as_str(),
        message,
        None,
    )
    .await
}

pub async fn pipeline_finish(config: &AppConfig) -> Result<()> {
    send_event(
        config,
        EventStatus::FinishedRun.as_str(),
        "Pipeline run concluded successfully",
        None,
    )
    .await
}
