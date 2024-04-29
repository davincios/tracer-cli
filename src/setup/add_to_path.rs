use anyhow::{Context, Result};
use std::{
    env, fs,
    io::Write,
    os::unix::fs::PermissionsExt,
    path::{Path, PathBuf},
    process::Command,
};

/// Configuration for setting up the tracer.
struct TracerConfig {
    tracer_dir: PathBuf,
    shell_config: PathBuf,
}

impl TracerConfig {
    /// Initializes the configuration for the tracer setup.
    fn new() -> Result<Self> {
        let home_dir = env::var("HOME").context("Failed to get HOME environment variable")?;
        let shell = env::var("SHELL").unwrap_or_else(|_| String::from("/bin/bash"));
        let shell_config_file = if shell.contains("zsh") {
            ".zshrc"
        } else {
            ".bashrc"
        };

        Ok(Self {
            tracer_dir: PathBuf::from("/tmp/tracer"),
            shell_config: Path::new(&home_dir).join(shell_config_file),
        })
    }
}

/// Append a line to a config file if it does not already contain it.
fn append_to_config(config_path: &Path, entry: &str) -> Result<()> {
    let content = fs::read_to_string(config_path)
        .with_context(|| format!("Failed to read shell config file at {:?}", config_path))?;

    if content.contains(entry) {
        return Ok(());
    }

    let mut file = fs::OpenOptions::new()
        .append(true)
        .open(config_path)
        .with_context(|| format!("Failed to open {:?}", config_path))?;

    writeln!(file, "{}", entry).with_context(|| "Failed to write to shell config file")
}

/// Sets up tracer in the system.
pub async fn setup_config_tracer() -> Result<()> {
    let config = TracerConfig::new()?;
    fs::create_dir_all(&config.tracer_dir)
        .with_context(|| format!("Failed to create directory {:?}", config.tracer_dir))?;

    let tracer_path = config.tracer_dir.join("tracer");
    fs::copy("tracer", &tracer_path)
        .with_context(|| format!("Failed to copy `tracer` to {:?}", tracer_path))?;
    fs::set_permissions(&tracer_path, fs::Permissions::from_mode(0o755))?;

    let path_entry = format!("export PATH=\"{}:$PATH\"", config.tracer_dir.display());
    let alias_entry = "alias tracer=\"tracer\"";

    append_to_config(&config.shell_config, &path_entry)?;
    append_to_config(&config.shell_config, &alias_entry)?;

    println!("Configurations updated in {:?}", config.shell_config);

    // Verifying installation
    Command::new(tracer_path)
        .arg("--version")
        .output()
        .map(|output| {
            println!(
                "Tracer Version: {}",
                String::from_utf8_lossy(&output.stdout)
            );
            Ok(())
        })?
}
