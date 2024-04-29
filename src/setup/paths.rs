// src/setup/paths.rs
use std::path::PathBuf;

pub struct ConfigPaths;

impl ConfigPaths {
    // Method to get the configuration directory path
    pub fn tracer_config_dir_path() -> PathBuf {
        PathBuf::from("/tmp/tracer")
    }

    // Method to get the full path to the configuration file
    pub fn tracer_config_file_path() -> PathBuf {
        Self::tracer_config_dir_path().join("config.json")
    }
}
