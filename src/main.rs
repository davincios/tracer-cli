// src/main.rs
use clap::Parser;

use anyhow::{Context, Result};
pub mod cli;
use cli::find_matches;
use cli::Cli;

fn main() -> Result<()> {
    let args = Cli::parse();
    let content = std::fs::read_to_string(&args.path)
        .with_context(|| format!("could not read file `{}`", args.path.display()))?;

    find_matches(&content, &args.pattern, &mut std::io::stdout())?;
    Ok(())
}
