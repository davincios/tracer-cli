use anyhow::{Context, Result};
use serde_json::{json, to_string_pretty};
use std::fs;
use std::io;
use std::io::Write;
use std::os::unix::fs::PermissionsExt;
use std::path::PathBuf;

// Helper function to create a directory with permissions
fn create_config_directory(path: &PathBuf) -> Result<()> {
    if !path.exists() {
        fs::create_dir_all(path)
            .with_context(|| format!("Failed to create directory: {:?}", path))?;
        fs::set_permissions(path, fs::Permissions::from_mode(0o700))
            .with_context(|| format!("Failed to set permissions for directory: {:?}", path))?;
    }
    Ok(())
}

fn create_configuration_file(config_json_file_path: &PathBuf) -> Result<()> {
    if !config_json_file_path.exists() {
        fs::File::create(config_json_file_path)
            .with_context(|| format!("Failed to create file: {:?}", config_json_file_path))?;
        fs::set_permissions(config_json_file_path, fs::Permissions::from_mode(0o600))
            .with_context(|| {
                format!(
                    "Failed to set permissions for file: {:?}",
                    config_json_file_path
                )
            })?;
    }
    Ok(())
}

fn write_to_configuration_file(api_key: &str, config_json_file_path: &PathBuf) -> io::Result<()> {
    // Create a JSON object for the API key
    let api_key_data = json!({ "api_key": api_key });

    // Serialize the JSON object to a pretty string
    let api_key_json = to_string_pretty(&api_key_data).expect("Failed to serialize API key");

    // Write the JSON string to the file
    let mut config = fs::File::create(config_json_file_path)?;
    writeln!(config, "{}", api_key_json)?;

    Ok(())
}

pub async fn setup_tracer_configuration_files(
    api_key: String,
    tracer_config_file_path: PathBuf,
    tracer_config_dir_path: PathBuf,
) -> Result<PathBuf> {
    create_config_directory(&tracer_config_dir_path)?;
    create_configuration_file(&tracer_config_file_path)?;
    write_to_configuration_file(&api_key, &tracer_config_file_path)?;
    println!(
        "Configuration saved to: {}",
        tracer_config_file_path.to_str().unwrap()
    );

    Ok(tracer_config_file_path)
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::Value;
    use std::io::Read;

    #[tokio::test]
    async fn test_create_config_directory() -> Result<()> {
        let test_dir = PathBuf::from("/tmp/test_tracer");
        create_config_directory(&test_dir)?;
        assert!(test_dir.exists());
        fs::remove_dir_all(&test_dir)?;
        Ok(())
    }

    #[tokio::test]
    async fn test_create_configuration_file() -> Result<()> {
        let test_file_path = PathBuf::from("/tmp/test_tracer_config.json");
        create_configuration_file(&test_file_path)?;
        assert!(test_file_path.exists());
        assert_eq!(
            fs::metadata(&test_file_path)?.permissions().mode() & 0o777,
            0o600
        );
        fs::remove_file(test_file_path)?;
        Ok(())
    }

    #[tokio::test]
    async fn test_write_to_configuration_file() -> Result<()> {
        let test_file_path = PathBuf::from("/tmp/test_tracer_write.json");
        write_to_configuration_file("test_api_key_123", &test_file_path)?;
        assert!(test_file_path.exists());
        fs::remove_file(test_file_path)?;
        Ok(())
    }

    #[tokio::test]
    async fn test_api_key_write_and_read() -> Result<()> {
        let test_file_path = PathBuf::from("/tmp/test_tracer_api_key.json");
        let test_api_key = "test_api_key_123";

        write_to_configuration_file(test_api_key, &test_file_path)?;

        // Read back the contents of the file
        let mut file = fs::File::open(&test_file_path)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;

        // Parse the JSON to get the API key value
        let parsed_json: Value = serde_json::from_str(&contents)?;
        let read_api_key = parsed_json["api_key"].as_str().unwrap();

        // Clean up: remove the test file
        fs::remove_file(test_file_path)?;

        // Check that the contents match the API key written
        assert_eq!(read_api_key, test_api_key);
        Ok(())
    }
}
