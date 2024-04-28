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
        // Initialize the configuration builder and set default values
        let mut settings = Config::builder()
            .set_default("base_url", "https://app.tracer.bio/api/fluent-bit-webhook")?;

        // Add configuration file source which may override defaults
        settings = settings.add_source(
            File::with_name("Settings")
                .required(false)
                .format(FileFormat::Toml),
        );

        // Build the final config, applying file settings over defaults
        let settings = settings.build()?;

        // Deserialize the configuration into AppConfig, excluding the client
        let mut config: AppConfig = settings.try_deserialize()?;

        // Optionally override the api_key if one is provided
        if let Some(key) = api_key {
            config.api_key = key.to_string();
        }

        // Create the client and include it in the returned AppConfig
        let client = Arc::new(Client::new());
        config.client = client;

        Ok(config)
    }
}
