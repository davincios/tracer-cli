use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::{env, fs, os::unix::fs::PermissionsExt, path::PathBuf};

#[derive(Serialize, Deserialize)]
pub struct TracerProjectConfig {
    pub api_key: String,
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
