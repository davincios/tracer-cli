// src/cli.rs
use clap::{Parser, Subcommand};

// Define the CLI structure using `clap`
#[derive(Parser)]
#[clap(
    name = "tracer",
    about = "A tool for tracing application commands",
    version = "1.0"
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
