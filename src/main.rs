// src/main.rs
use anyhow::{Context, Result};
use clap::Parser;

mod cli;
use crate::cli::{Cli, Commands};

use tracer_cli::{
    log_message, pipeline_finish_run, pipeline_new_run, setup_tracer, tool_process, AppConfig,
    Tool, TracerProjectConfig,
};

#[tokio::main] // Adding the async entry point
async fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Setup { api_key } => setup_tracer(&api_key).await,
        Commands::Start => start().await,
        Commands::Log { r#type, message } => log(r#type, message).await,
        Commands::End => end().await,
        Commands::Tool { name, version } => tool(name, version).await,
    }
}

async fn start() -> Result<()> {
    println!("Starting new pipeline...");
    let config = TracerProjectConfig::load()?;
    let api_key = config.api_key;

    let app_config =
        AppConfig::new(Some(&api_key)).context("Failed to load configuration during start.")?;

    pipeline_new_run(&app_config, "[CLI] Starting pipeline run").await?;
    println!("Started pipeline run successfully...");

    Ok(())
}

async fn tool(name: String, version: String) -> Result<()> {
    println!("Processing tool...");
    let tool = Tool { name, version };
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
