// src/setup/mod.rs
mod app_config;
mod files;

use anyhow::Result;
pub use app_config::TracerAppConfig;
use files::setup_tracer_configuration_files;

use crate::methods::{send_event, EventStatus};
mod paths;
pub use paths::ConfigPaths;

pub async fn setup_tracer(api_key: String) -> Result<()> {
    let tracer_config_file_path = ConfigPaths::tracer_config_file_path();
    let tracer_config_dir_path = ConfigPaths::tracer_config_dir_path();

    // Clone api_key before passing it to the function that moves it
    let api_key_clone = api_key.clone();
    setup_tracer_configuration_files(
        api_key_clone,
        tracer_config_file_path,
        tracer_config_dir_path,
    )
    .await?;

    // Load the configuration to verify if the API_KEY is loadable from the file
    let config = TracerAppConfig::load_config()?;

    assert_eq!(api_key, config.api_key, "API key mismatch detected.");
    println!(
        "Tracer setup completed successfully with API key: {}",
        config.api_key
    );

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
    async fn test_setup_tracer_success() {
        let api_key = "dIdd4HI9ixcQtw7xsulnv".to_string();

        let result = setup_tracer(api_key.clone()).await;

        if let Err(ref e) = result {
            println!("Setup failed with error: {}", e);
        }

        assert!(result.is_ok(), "Setup should succeed");
    }
}
