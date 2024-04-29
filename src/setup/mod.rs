// src/setup/mod.rs
mod app_config;
mod files;

use anyhow::Result;
pub use app_config::TracerAppConfig;
use files::setup_tracer_configuration_files;

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

    assert_eq!(
        api_key, // use the original api_key here since it has not been moved
        config.api_key,
        "API key mismatch detected."
    );
    println!(
        "Tracer setup completed successfully with API key: {}",
        config.api_key
    );

    Ok(())
}
