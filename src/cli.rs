// src/cli.rs
use clap::{Parser, Subcommand};

// Define the CLI structure using `clap`
#[derive(Parser)]
#[clap(
    name = "tracer",
    about = "A tool for monitoring bioinformatics applications",
    version = env!("CARGO_PKG_VERSION") // Automatically use the version from Cargo.toml
)]
pub struct Cli {
    #[clap(subcommand)]
    pub command: Commands,
}

// Define the subcommands
#[derive(Subcommand)]
pub enum Commands {
    Setup {
        api_key: String,
    },
    Start,
    Log {
        #[clap(long)]
        r#type: String,
        message: String,
    },
    Tool {
        name: String,
        version: String,
    },
    End,
}

// Tests
#[cfg(test)]
mod tests {
    use assert_cmd::Command;
    use predicates::prelude::*; // This is used for writing assertions

    #[test]
    fn test_version() {
        let mut cmd = Command::cargo_bin("tracer").unwrap();
        cmd.arg("--version");
        cmd.assert()
            .success()
            .stdout(predicate::str::contains(env!("CARGO_PKG_VERSION")));
    }
}
