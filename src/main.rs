// src/main.rs
use anyhow::Result;
use clap::Parser;

mod cli;
use crate::cli::{Cli, Commands};

use tracer::{
    log_message, pipeline_finish_run, pipeline_new_run, setup_tracer, tool_process, Tool,
    TracerAppConfig,
};

#[tokio::main] // Adding the async entry point
async fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Setup { api_key } => setup(api_key).await,
        Commands::Start => start().await,
        Commands::Log { r#type, message } => log(r#type, message).await,
        Commands::End => end().await,
        Commands::Tool { name, version } => tool(name, version).await,
    }
}

async fn setup(api_key: String) -> Result<()> {
    println!("Setting up tracer with API key");
    setup_tracer(api_key.clone()).await?;

    let config = TracerAppConfig::load_config()?;

    assert_eq!(api_key.clone(), config.api_key);
    println!(
        "Tracer setup completed successfully with API key: {}",
        config.api_key
    );

    Ok(())
}

async fn start() -> Result<()> {
    println!("Starting new pipeline...");
    let config = TracerAppConfig::load_config()?;

    pipeline_new_run(&config, "[CLI] Starting pipeline run").await?;
    println!("Started pipeline run successfully...");

    Ok(())
}

async fn tool(name: String, version: String) -> Result<()> {
    println!("Processing tool...");
    let tool = Tool { name, version };
    let config = TracerAppConfig::load_config()?;

    tool_process(&config, &tool).await?;
    println!("Tool processed successfully...");

    Ok(())
}

async fn log(log_type: String, message: String) -> Result<()> {
    println!("Logging a {} message: {}", log_type, message);
    let config = TracerAppConfig::load_config()?;

    log_message(&config, &message).await?;
    Ok(())
}

async fn end() -> Result<()> {
    println!("Ending tracer session...");
    let config = TracerAppConfig::load_config()?;

    pipeline_finish_run(&config).await?;
    Ok(())
}
