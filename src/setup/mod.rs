use anyhow::{anyhow, Result};
use std::fs;
use std::os::unix::fs::PermissionsExt;
use std::path::PathBuf;
use std::process::Command;

mod config;
pub use config::TracerConfig;

// Helper function to create a directory with permissions
fn create_config_directory(path: &PathBuf) -> Result<()> {
    if !path.exists() {
        fs::create_dir_all(path)?;
        fs::set_permissions(path, fs::Permissions::from_mode(0o700))?; // Permissions for owner only
    }
    Ok(())
}

// Helper function to setup configuration
fn setup_configuration(api_key: String, binary_path: PathBuf) -> Result<TracerConfig> {
    let mut config = TracerConfig::new(api_key)?;
    config.installation_tracer_binary_path = binary_path;
    config.save_config()?;
    Ok(config)
}

// Helper function to check tracer version
fn check_tracer_version(tracer_path: &PathBuf) -> Result<String> {
    let output = Command::new(tracer_path).arg("--version").output()?;
    Ok(String::from_utf8_lossy(&output.stdout).to_string())
}

pub async fn setup_tracer(api_key: String) -> Result<()> {
    let tracer_initial_binary_path = PathBuf::from("/usr/local/bin/tracer");
    let tracer_config_dir = PathBuf::from("/etc/tracer");

    create_config_directory(&tracer_config_dir)?;
    let config = setup_configuration(api_key, tracer_initial_binary_path)?;
    let version = check_tracer_version(&config.installation_tracer_binary_path)?;
    println!("Tracer Version: {}", version);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_create_config_directory() -> Result<()> {
        let test_dir = PathBuf::from("/tmp/test_tracer");
        create_config_directory(&test_dir)?;
        assert!(test_dir.exists());
        fs::remove_dir_all(test_dir)?; // Cleanup
        Ok(())
    }

    #[tokio::test]
    async fn test_setup_configuration() -> Result<()> {
        let test_binary_path = PathBuf::from("/usr/local/bin/tracer");
        let config = setup_configuration("test_api_key".to_string(), test_binary_path)?;
        assert_eq!(config.api_key, "test_api_key");
        Ok(())
    }

    #[tokio::test]
    async fn test_check_tracer_version() -> Result<()> {
        let test_binary_path = PathBuf::from("/usr/local/bin/tracer");
        let version = check_tracer_version(&test_binary_path)?;
        assert!(!version.is_empty());
        Ok(())
    }

    #[tokio::test]
    async fn test_tracer_setup() -> Result<()> {
        setup_tracer("test_api_key".to_string()).await?;
        Ok(())
    }
}
