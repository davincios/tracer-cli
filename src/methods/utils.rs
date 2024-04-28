use anyhow::{Context, Result};
use reqwest::StatusCode;
use std::{fs, io::Write};

pub fn log_response(response: &str, log_file: &str) -> Result<()> {
    fs::create_dir_all(std::path::Path::new(log_file).parent().unwrap())?;
    let mut file = fs::OpenOptions::new()
        .append(true)
        .create(true)
        .open(log_file)?;
    writeln!(file, "{}", response)?;
    Ok(())
}

pub async fn handle_response(response: reqwest::Response, process_status: &str) -> Result<()> {
    let status_code = response.status();
    let response_text = response.text().await?;

    log_response(
        &response_text,
        &format!("./responses/{}.log", process_status),
    )
    .context("Failed to log response")?;

    if status_code == StatusCode::OK {
        println!("Status Code: {}, Response: {}", status_code, response_text);
        Ok(())
    } else {
        Err(anyhow::anyhow!(
            "Failed to send event: {} - {}",
            status_code,
            response_text
        ))
    }
}
