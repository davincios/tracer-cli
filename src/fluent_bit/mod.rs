// src/fluent-bit/mod.rs

use anyhow::{Context, Result};
use tokio::fs;
use tokio::process::Command;

use crate::methods::{send_event, EventStatus};
use crate::ConfigPaths;
use crate::TracerAppConfig;

// Function to generate the fluent-bit.conf content as a string
fn generate_fluent_bit_conf(api_key: &str) -> String {
    format!(
        r#"[SERVICE]
    flush        20
    daemon       Off
    log_level    info
    parsers_file parsers.conf
    plugins_file plugins.conf
    http_server  Off
    http_listen  0.0.0.0
    http_port    2020
    storage.metrics on

[INPUT]
    name cpu
    tag  cpu.local
    interval_sec 30

[INPUT]
    name            mem
    tag             mem.local
    interval_sec    30

[INPUT]
    name          netif
    tag           netif
    interval_Sec  30
    interval_NSec 0
    interface     eth0

[INPUT]
    name            disk
    tag             disk.local
    interval_sec    30

[OUTPUT]
    name            http
    match           *
    host            app.tracer.bio
    port            443
    uri             /api/fluent-bit-webhook-without-logs
    format          json
    tls             On
    tls.verify      Off
    header          Content-Type application/json
    header          X-Api-Key {api_key}
"#,
        api_key = api_key
    )
}

pub async fn start_fluent_bit(tracer_config_dir_path: &std::path::Path) -> Result<()> {
    // Path to the generated fluent-bit.conf file
    let tracer_config_file_path = tracer_config_dir_path.join("fluent-bit.conf");

    // Start Fluent Bit in the background
    let mut child = Command::new("fluent-bit")
        .arg("-c")
        .arg(tracer_config_file_path)
        .spawn()
        .context("Failed to start Fluent Bit")?;

    // Detach the process to run in the background
    tokio::spawn(async move {
        let _ = child.wait().await;
    });

    Ok(())
}

pub async fn setup_fluent_bit(api_key: String, config: &TracerAppConfig) -> Result<()> {
    let tracer_config_dir_path = ConfigPaths::tracer_config_dir_path();

    // Generate the content of fluent-bit.conf
    let fluent_bit_conf_content = generate_fluent_bit_conf(&api_key);

    // Write the generated content to the destination path in the tracer configuration directory
    let tracer_config_file_path = tracer_config_dir_path.join("fluent-bit.conf");
    fs::write(&tracer_config_file_path, fluent_bit_conf_content)
        .await
        .with_context(|| {
            format!(
                "Failed to write fluent-bit.conf to {}",
                tracer_config_file_path.display()
            )
        })?;

    println!("Fluent-bit configuration completed");

    send_event(
        config,
        EventStatus::InstallationFinished.as_str(),
        "[TracerCLI setup] Installation completed",
        None,
        false,
    )
    .await?;

    // Start Fluent Bit
    start_fluent_bit(&tracer_config_dir_path).await?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::TracerAppConfig;

    #[tokio::test]
    async fn test_fluent_bit_tracer_success() -> Result<()> {
        let config = TracerAppConfig::load_config()?; // Assuming load_config returns Result<TracerAppConfig>

        let result = setup_fluent_bit(config.api_key.clone(), &config).await;

        if let Err(ref e) = result {
            println!("Setup failed with error: {}", e);
        }

        assert!(result.is_ok(), "Setup should succeed");
        Ok(())
    }
}
