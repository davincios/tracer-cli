use reqwest::Client;
use serde_json::json;
use std::{fs, io::Write};

// Constants for API configuration
const API_KEY: &str = "dDRE5rxJEjktQxCtzsYyz";

pub async fn send_event(
    event_type: &str,
    message: &str,
    event_status: Option<&str>,
    base_url: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new();
    let data = json!({
        "logs": [{
            "message": message,
            "event_type": event_type,
            "process_type": "pipeline",
            "process_status": event_status.unwrap_or(""),
        }]
    });

    let response = client
        .post(base_url)
        .header("x-api-key", API_KEY)
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

pub async fn init_pipeline(
    run_name: String,
    base_url: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    send_event(
        "init",
        &format!("Initialized pipeline run with name: {}", run_name),
        None,
        base_url,
    )
    .await
}

pub async fn finish_pipeline(base_url: &str) -> Result<(), Box<dyn std::error::Error>> {
    send_event(
        "finished_installation",
        "Pipeline run concluded.",
        None,
        base_url,
    )
    .await
}

pub fn log_response(response: &str, log_file: &str) -> Result<(), std::io::Error> {
    fs::create_dir_all(std::path::Path::new(log_file).parent().unwrap())?;
    let mut file = fs::OpenOptions::new()
        .append(true)
        .create(true)
        .open(log_file)?;
    writeln!(file, "{}", response)
}
