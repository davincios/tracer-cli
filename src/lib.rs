// src/lib.rs

use anyhow::{Context, Result};
use std::io::Write;

pub mod events;
pub use events::{finish_pipeline, init_pipeline, send_event};

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
