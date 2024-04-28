use anyhow::Result;
use reqwest::Client;
use serde_json::json;
use std::{fs, io::Write}; // Using anyhow for error handling

#[derive(Debug)]
pub enum EventStatus {
    NewRun,
    FinishedRun,
    Error,
    UnknownRunStatus,
    RunStatusMessage,
    ToolProcess,
}

impl EventStatus {
    pub fn as_str(&self) -> &'static str {
        match self {
            EventStatus::NewRun => "new_run",
            EventStatus::FinishedRun => "finished_run",
            EventStatus::Error => "error",
            EventStatus::UnknownRunStatus => "unknown_run_status",
            EventStatus::RunStatusMessage => "run_status_message",
            EventStatus::ToolProcess => "tool_process",
        }
    }
}

pub async fn send_event(
    event_type: &str,
    message: &str,
    event_status: Option<EventStatus>,
    base_url: &str,
    api_key: &str,
) -> Result<()> {
    let client = Client::new();
    let status_str = event_status.map_or("", |status| status.as_str());
    let data = json!({
        "logs": [{
            "message": message,
            "event_type": event_type,
            "process_type": "pipeline",
            "process_status": status_str,
        }]
    });

    let response = client
        .post(base_url)
        .header("x-api-key", api_key)
        .header("Content-Type", "application/json")
        .json(&data)
        .send()
        .await?;

    let status_code = response.status();
    let response_text: String = response.text().await?;

    log_response(&response_text, &format!("./responses/{}.log", event_type))?;

    println!("Status Code: {}, Response: {}", status_code, response_text);
    Ok(())
}
pub async fn init_pipeline(run_name: String, base_url: &str, api_key: &str) -> Result<()> {
    send_event(
        "init",
        &format!("Initialized pipeline run with name: {}", run_name),
        Some(EventStatus::NewRun), // Using the enum
        base_url,
        api_key,
    )
    .await
}

pub async fn finish_pipeline(base_url: &str, api_key: &str) -> Result<()> {
    send_event(
        "finished_installation",
        "Pipeline run concluded.",
        Some(EventStatus::FinishedRun), // Using the enum
        base_url,
        api_key,
    )
    .await
}

pub fn log_response(response: &str, log_file: &str) -> Result<(), std::io::Error> {
    fs::create_dir_all(std::path::Path::new(log_file).parent().unwrap())?;
    let mut file = fs::OpenOptions::new()
        .append(true)
        .create(true)
        .open(log_file)?;
    writeln!(file, "{}", response)?;
    Ok(())
}
