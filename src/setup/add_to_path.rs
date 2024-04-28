use anyhow::{Context, Result};
use std::{env, fs, io::Write, os::unix::fs::PermissionsExt, path::Path, process::Command};

/// Append a line to a config file if it does not already contain it.
fn append_to_config(config_path: &Path, entry: &str) -> Result<()> {
    let mut file = fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(config_path)
        .with_context(|| format!("Failed to open {:?}", config_path))?;

    let content =
        fs::read_to_string(config_path).with_context(|| "Failed to read shell config file")?;

    if !content.contains(entry) {
        writeln!(file, "{}", entry).with_context(|| "Failed to write to shell config file")?;
    }

    Ok(())
}

/// Add `tracer_cli` to the system path and set up necessary configurations.
pub async fn setup_tracer_cli_path() -> Result<()> {
    // Ensure /etc/tracer/ directory exists
    let tracer_dir = Path::new("/etc/tracer");
    fs::create_dir_all(tracer_dir)
        .with_context(|| format!("Failed to create directory {:?}", tracer_dir))?;

    // Copy `tracer_cli` to /etc/tracer/tracer_cli
    let tracer_cli_path = tracer_dir.join("tracer_cli");
    fs::copy("tracer_cli", &tracer_cli_path)
        .with_context(|| format!("Failed to copy `tracer_cli` to {:?}", tracer_cli_path))?;

    // Set executable permissions for tracer_cli
    fs::set_permissions(&tracer_cli_path, fs::Permissions::from_mode(0o755))?;

    // Determine which shell config file to modify (.bashrc or .zshrc)
    let home_dir = env::var("HOME").context("Failed to get HOME environment variable")?;
    let shell_config_path = if env::var("SHELL").map_or(false, |sh| sh.contains("zsh")) {
        Path::new(&home_dir).join(".zshrc")
    } else {
        Path::new(&home_dir).join(".bashrc")
    };

    // Prepare the entries to add to the shell config file
    let path_entry = "export PATH=\"$PATH:/etc/tracer\"";
    let alias_entry = "alias tracer=\"tracer_cli\"";

    append_to_config(&shell_config_path, path_entry)?;
    append_to_config(&shell_config_path, alias_entry)?;

    println!("Configurations updated in {:?}", shell_config_path);

    // Execute tracer --version to verify installation
    if let Ok(output) = Command::new("/etc/tracer/tracer_cli")
        .arg("--version")
        .output()
    {
        println!(
            "Tracer Version: {}",
            String::from_utf8_lossy(&output.stdout)
        );
    }

    Ok(())
}
