use anyhow::{Context, Result};
use clap::Parser;
use std::io::Write;
use std::path::PathBuf; // Import `find_matches` function from the parent or another module if needed

/// Command line arguments definition using Clap.
#[derive(Parser)]
pub struct Cli {
    /// The pattern to look for
    pub pattern: String,
    /// The path to the file to read
    pub path: PathBuf,
}

/// Function to handle the command line interface logic
pub fn run() -> Result<()> {
    let args = Cli::parse();
    let content = std::fs::read_to_string(&args.path)
        .with_context(|| format!("could not read file `{}`", args.path.display()))?;

    find_matches(&content, &args.pattern, &mut std::io::stdout())?;
    Ok(())
}

/// Performs line-by-line matching to find lines containing the specified pattern.
pub fn find_matches(content: &str, pattern: &str, mut writer: impl Write) -> Result<()> {
    for line in content.lines() {
        if line.contains(pattern) {
            writeln!(writer, "{}", line).with_context(|| "Failed to write to output")?;
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn find_a_match() -> Result<()> {
        let mut result = Vec::new();
        find_matches("lorem ipsum\ndolor sit amet", "lorem", &mut result)?;
        assert_eq!(result, b"lorem ipsum\n");
        Ok(())
    }
}
