use config::{Config, ConfigError, File, FileFormat};
use reqwest::Client;
use serde::Deserialize;
use std::sync::Arc;

#[derive(Deserialize)]
pub struct AppConfig {
    pub base_url: String,
    pub api_key: String,
    #[serde(skip)]
    pub client: Arc<Client>, // Using Arc to easily share the client across threads
}

impl AppConfig {
    pub fn new(api_key: Option<&str>) -> Result<Self, ConfigError> {
        // Start by loading the configuration from a file
        let settings = Config::builder()
            .add_source(
                File::with_name("Settings")
                    .required(false)
                    .format(FileFormat::Toml),
            )
            .build()?;

        // Deserialize the configuration into AppConfig, excluding the client
        let mut config: AppConfig = settings.try_deserialize()?;

        // Optionally override the api_key if one is provided
        if let Some(key) = api_key {
            config.api_key = key.to_string();
        }

        config.base_url = "https://app.tracer.bio/api/fluent-bit-webhook".to_string();

        // Create the client and include it in the returned AppConfig
        let client = Arc::new(Client::new());
        config.client = client;

        Ok(config)
    }
}
