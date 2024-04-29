use anyhow::{Context, Result};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::env;
use std::fs::{self};
use std::path::PathBuf;

use std::path::Path;
use std::sync::Arc;

#[derive(Serialize, Deserialize)]
pub struct TracerConfig {
    pub installation_tracer_binary_path: PathBuf,
    pub user_shell_config_file_path: PathBuf,
    pub api_key: String,
    pub service_url: String,
    #[serde(skip)]
    pub http_client: Arc<Client>,
}

impl TracerConfig {
    pub fn new(api_key: String) -> Result<Self> {
        let home_dir = env::var("HOME").context("Failed to get HOME environment variable")?;
        let shell_config_file = match env::var("SHELL").unwrap_or_default().as_str() {
            "/bin/zsh" => ".zshrc",
            _ => ".bashrc",
        };

        Ok(Self {
            installation_tracer_binary_path: PathBuf::from("/tmp/tracer"),
            user_shell_config_file_path: Path::new(&home_dir).join(shell_config_file),
            api_key,
            service_url: "https://app.tracer.bio/api/fluent-bit-webhook".to_string(),
            http_client: Arc::new(Client::new()),
        })
    }

    pub fn save_config(&self) -> Result<()> {
        let config_file_path = self.installation_tracer_binary_path.join("config.json");
        fs::create_dir_all(&self.installation_tracer_binary_path)?;
        let config_data = serde_json::to_string(&self)?;
        fs::write(&config_file_path, config_data)?;
        Ok(())
    }

    pub fn load_config() -> Result<Self> {
        let config_file_path = PathBuf::from("/tmp/tracer/config.json");
        let config_data = fs::read_to_string(&config_file_path)?;
        let mut config: TracerConfig = serde_json::from_str(&config_data)?;
        config.http_client = Arc::new(Client::new()); // Reinitialize http_client
        Ok(config)
    }
}
