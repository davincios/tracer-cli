/// main.rs
use anyhow::{Context, Result};
use clap::{Parser, Subcommand};

// use serde::{Deserialize, Serialize};
// use std::{fs, path::PathBuf};
use tracer_cli::{
    log_message, pipeline_finish_run, pipeline_new_run, setup_tracer, tool_process, AppConfig,
    Tool, TracerProjectConfig,
};

// Define the CLI structure using `clap`
#[derive(Parser)]
#[clap(
    name = "tracer",
    about = "A tool for tracing application commands",
    version = "1.0"
)]
struct Cli {
    #[clap(subcommand)]
    command: Commands,
}

// Define the subcommands
#[derive(Subcommand)]
enum Commands {
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
        flags: Vec<String>,
    },
    End,
}

#[tokio::main] // Adding the async entry point
async fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Setup { api_key } => setup_tracer(&api_key).await,
        Commands::Start => start().await,
        Commands::Log { r#type, message } => log(r#type, message).await,
        Commands::End => end().await,
        Commands::Tool {
            name,
            version,
            flags,
        } => tool(name, version, flags).await,
    }
}

async fn start() -> Result<()> {
    println!("Starting tracer...");
    let config = TracerProjectConfig::load()?;
    let api_key = config.api_key;

    let app_config =
        AppConfig::new(Some(&api_key)).context("Failed to load configuration during start.")?;

    pipeline_new_run(&app_config, "[CLI] Starting pipeline run").await?;
    println!("Started pipeline run successfully...");

    Ok(())
}

async fn tool(name: String, version: String, flags: Vec<String>) -> Result<()> {
    println!("Processing tool...");
    let tool = Tool {
        name,
        version,
        flags,
    };
    let config = TracerProjectConfig::load()?;
    let api_key = config.api_key;

    let app_config =
        AppConfig::new(Some(&api_key)).context("Failed to load configuration for tool.")?;

    tool_process(&app_config, &tool).await?;
    println!("Tool processed successfully...");

    Ok(())
}

async fn log(log_type: String, message: String) -> Result<()> {
    println!("Logging a {} message: {}", log_type, message);
    let config = TracerProjectConfig::load()?;
    let api_key = config.api_key;
    let app_config =
        AppConfig::new(Some(&api_key)).context("Failed to load configuration during end.")?;

    log_message(&app_config, &message).await?;
    Ok(())
}

async fn end() -> Result<()> {
    println!("Ending tracer session...");
    let config = TracerProjectConfig::load()?;
    let api_key = config.api_key;

    let app_config =
        AppConfig::new(Some(&api_key)).context("Failed to load configuration during end.")?;

    pipeline_finish_run(&app_config).await?;
    Ok(())
}
