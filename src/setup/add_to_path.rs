use anyhow::{Context, Result};
use std::{env, fs, io::Write, path::Path};

pub async fn add_tracer_path_to_env() -> Result<()> {
    let shell_config_path = match env::var("SHELL") {
        Ok(shell) if shell.contains("zsh") => Path::new(&env::var("HOME")?).join(".zshrc"),
        _ => Path::new(&env::var("HOME")?).join(".bashrc"),
    };

    let path_entry = "export PATH=\"$PATH:/usr/local/bin\"";

    let mut file = fs::OpenOptions::new()
        .write(true)
        .append(true)
        .open(&shell_config_path)
        .with_context(|| format!("Failed to open {:?}", shell_config_path))?;

    // Check if `path_entry` is already in the file
    let content = fs::read_to_string(&shell_config_path)
        .with_context(|| "Failed to read shell config file")?;
    if !content.contains(path_entry) {
        writeln!(file, "\n{}", path_entry)
            .with_context(|| "Failed to write to shell config file")?;
    }

    println!("Path updated in {:?}", shell_config_path);

    Ok(())
}
