use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::{env, fs, os::unix::fs::PermissionsExt, path::PathBuf};

mod add_to_path;

use add_to_path::add_tracer_path_to_env;

#[derive(Serialize, Deserialize)]
pub struct TracerProjectConfig {
    pub api_key: String,
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
        let config = serde_json::from_str(&contents)
            .with_context(|| "Failed to deserialize the config file")?;
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
    };
    config.save()?;
    let path = TracerProjectConfig::get_path()?;
    println!("API key saved to {:?}", path);
    add_tracer_path_to_env().await?;
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
