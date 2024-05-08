use anyhow::{Context, Result};
use reqwest::StatusCode;
use std::{fs, io::Write};

/// Logs a given response string into the specified log file.
pub fn log_response(response: &str, log_file: &str) -> Result<()> {
    // Ensure the parent directory exists before writing
    fs::create_dir_all(std::path::Path::new(log_file).parent().unwrap())?;

    // Open file in append mode, or create if it doesn't exist
    let mut file = fs::OpenOptions::new()
        .append(true)
        .create(true)
        .open(log_file)?;

    // Write the response to the file
    writeln!(file, "{}", response)?;
    Ok(())
}

pub async fn handle_response(
    response: reqwest::Response,
    process_status: &str,
    should_print_response: bool, // Optional parameter to control response printing
) -> Result<()> {
    // Extract the status code and response text
    let status_code = response.status();
    let response_text = response.text().await?;

    // Log the response to a file
    log_response(
        &response_text,
        &format!("./responses/{}.log", process_status),
    )
    .context("Failed to log response")?;

    // Handle success or failure based on status code
    if status_code == StatusCode::OK {
        if should_print_response {
            println!("Status Code: {}, Response: {}", status_code, response_text);
        }

        Ok(())
    } else {
        println!("Status Code: {}, Response: {}", status_code, response_text);
        Err(anyhow::anyhow!(
            "Failed to send event: {} - {}",
            status_code,
            response_text
        ))
    }
}
