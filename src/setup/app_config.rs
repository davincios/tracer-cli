use anyhow::{Context, Result};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::fs;
use std::sync::Arc;

use crate::setup::paths::ConfigPaths; // Ensure the paths module is correctly imported

#[derive(Serialize, Deserialize)]
pub struct TracerAppConfig {
    pub api_key: String,
    pub service_url: String,
    #[serde(skip)]
    pub http_client: Arc<Client>,
}

impl TracerAppConfig {
    // Load configuration from a file, optionally specifying a file path
    pub fn load_config() -> Result<Self> {
        let tracer_config_file_path = ConfigPaths::tracer_config_file_path()
            .to_str()
            .unwrap()
            .to_string();

        let config_data = fs::read_to_string(&tracer_config_file_path).with_context(|| {
            format!(
                "Failed to read configuration file at: {}",
                tracer_config_file_path
            )
        })?;
        let mut config: TracerAppConfig = serde_json::from_str(&config_data)
            .with_context(|| "Failed to deserialize the configuration")?;

        // Additional trimming and setup
        config.api_key = config.api_key.trim().to_string();
        config.http_client = Arc::new(Client::new());

        // Optionally read service URL from an environment variable or fallback to default
        config.service_url = "https://app.tracer.bio/api/fluent-bit-webhook".to_string();

        Ok(config)
    }
}
