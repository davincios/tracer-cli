use config::{ConfigBuilder, ConfigError, File, FileFormat};
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
    pub fn new() -> Result<Self, ConfigError> {
        // Explicitly starting with the default state
        let builder = ConfigBuilder::<config::builder::DefaultState>::default().add_source(
            File::with_name("Settings")
                .required(false)
                .format(FileFormat::Toml),
        );
        let settings = builder.build()?;
        let config: AppConfig = settings.try_deserialize()?;

        // Create the client and include it in the returned AppConfig
        let client = Arc::new(Client::new());
        Ok(AppConfig {
            base_url: config.base_url,
            api_key: config.api_key,
            client,
        })
    }
}
