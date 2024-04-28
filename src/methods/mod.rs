use anyhow::Result;
use serde_json::json;

use self::events::{send_event, EventStatus};

// Declare submodules as private
mod config;
mod events;
mod utils;

pub use config::AppConfig;

pub struct Tool {
    name: String,
    version: String,
    flags: Vec<String>,
}

pub async fn pipeline_init(config: &AppConfig, msg: &str) -> Result<()> {
    send_event(
        config,
        "init",
        &format!("Initialized pipeline run with name: {}", msg),
        Some(EventStatus::NewRun),
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
        "process_status",
        &format!("Tool process: {}", tool.name),
        Some(EventStatus::ToolExecution),
        Some(properties),
    )
    .await
}

pub async fn log_message(config: &AppConfig, message: &str) -> Result<()> {
    send_event(
        config,
        "log_message",
        message,
        Some(EventStatus::RunStatusMessage),
        None,
    )
    .await
}

pub async fn pipeline_finish(config: &AppConfig) -> Result<()> {
    send_event(
        config,
        "finished_installation",
        "Pipeline run concluded.",
        Some(EventStatus::FinishedRun),
        None,
    )
    .await
}
