use anyhow::{Context, Result};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use std::{env, fs, os::unix::fs::PermissionsExt, path::PathBuf};

mod add_to_path;

use add_to_path::setup_config_tracer;

#[derive(Serialize, Deserialize)]
pub struct TracerProjectConfig {
    pub api_key: String,
    pub base_url: String,
    #[serde(skip)]
    pub client: Arc<Client>, // Using Arc to easily share the client across threads
}

impl TracerProjectConfig {
    pub fn get_path() -> Result<PathBuf> {
        let base_path = env::var("TRACER_CONFIG_DIR").unwrap_or_else(|_| "/tmp/tracer".to_string());
        let mut path = PathBuf::from(base_path);
        fs::create_dir_all(&path)?; // Ensure the directory exists
        path.push("config.json");
        Ok(path)
    }

    pub fn load() -> Result<Self> {
        let path = TracerProjectConfig::get_path()?;
        let contents = fs::read_to_string(&path)
            .with_context(|| format!("Failed to read config file at {:?}", path))?;
        let config: TracerProjectConfig = serde_json::from_str(&contents)
            .with_context(|| "Failed to deserialize the config file")?;

        let client = Arc::new(Client::new());
        let mut config = config;
        config.client = client;
        Ok(config)
    }

    pub fn save(&self) -> Result<()> {
        let path = TracerProjectConfig::get_path()?;
        let contents =
            serde_json::to_string(&self).with_context(|| "Failed to serialize config")?;
        fs::write(&path, contents)
            .with_context(|| format!("Failed to write config file at {:?}", path))?;
        // Set file permissions to read/write for owner, and read for group (e.g., chmod 640)
        let mut perms = fs::metadata(&path)?.permissions();
        perms.set_mode(0o640); // Owner read/write, group read, others no access
        fs::set_permissions(&path, perms)?;
        Ok(())
    }
}

pub async fn setup_tracer(api_key: &str) -> Result<()> {
    let config = TracerProjectConfig {
        api_key: api_key.to_string(),
        base_url: "https://app.tracer.bio/api/fluent-bit-webhook".to_string(),
        client: Arc::new(Client::new()),
    };
    config.save()?;
    let path = TracerProjectConfig::get_path()?;
    println!("API key saved to {:?}", path);
    setup_config_tracer().await?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio;

    #[tokio::test]
    async fn test_setup_sets_api_key() -> Result<()> {
        let test_api_key = "dDRE5rxJEjktQxCtzsYyz";
        setup_tracer(test_api_key).await?;

        let config = TracerProjectConfig::load()?;
        assert_eq!(
            config.api_key, test_api_key,
            "The API key set by setup does not match the expected test API key."
        );

        let path = TracerProjectConfig::get_path()?;
        print!("API key saved to {:?}", path);

        Ok(())
    }
}
