// src/fluent-bit/mod.rs --> next to src/fluent-bit/fluent-bit.conf
mod app_config;
mod files;

use anyhow::{Result, Context};
use tokio::fs;
pub use app_config::TracerAppConfig;
use files::setup_tracer_configuration_files;

use crate::methods::{send_event, EventStatus};
mod paths;
pub use paths::ConfigPaths;

// move fluent-bit conf fluent-bit.conf 

pub async fn setup_fluent_bit(api_key: String) -> Result<()> {
    let tracer_config_dir_path = ConfigPaths::tracer_config_dir_path();

    // Clone api_key before passing it to the function that moves it
    let api_key_clone = api_key.clone();

    // Read the content of fluent-bit.conf
    let fluent_bit_conf_path = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("src/fluent-bit/fluent-bit.conf");
    let fluent_bit_conf_content = fs::read_to_string(&fluent_bit_conf_path)
        .await
        .with_context(|| format!("Failed to read fluent-bit.conf from {}", fluent_bit_conf_path.display()))?;

    // Write the content to the destination path in the tracer configuration directory
    let tracer_config_file_path = tracer_config_dir_path.join("fluent-bit.conf");
    fs::write(&tracer_config_file_path, fluent_bit_conf_content)
        .await
        .with_context(|| format!("Failed to write fluent-bit.conf to {}", tracer_config_file_path.display()))?;

    setup_tracer_configuration_files(
        api_key_clone,
        tracer_config_file_path,
        tracer_config_dir_path,
    )
    .await?;

    println!(
        "Fluent-bit configuration completed"
    );

    let config = TracerAppConfig::default(); // Assuming you have a default implementation
    send_event(
        &config,
        EventStatus::InstallationFinished.as_str(),
        "[TracerCLI setup] Installation completed",
        None,
        false,
    )
    .await?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_fluent_bit_tracer_success() {
        let api_key = "test_api_key".to_string(); // Replace with an appropriate test API key

        let result = setup_fluent_bit(api_key).await;

        if let Err(ref e) = result {
            println!("Setup failed with error: {}", e);
        }

        assert!(result.is_ok(), "Setup should succeed");
    }
}
