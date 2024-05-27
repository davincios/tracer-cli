// src/main.rs
use anyhow::Result;
use clap::Parser;

mod cli;
use crate::cli::{Cli, Commands};

use tracer::{
    log_message, metrics::DiskMetricsCollector, pipeline_finish_run, pipeline_new_run,
    setup_tracer, tool_process, Tool, TracerAppConfig,
};

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Setup { api_key } => setup(api_key).await,
        Commands::Update => update_tracer_cli().await,
        Commands::Start => start().await,
        Commands::Log { message } => log(message).await,
        Commands::Metrics => metrics().await,
        Commands::End => end().await,
        Commands::Info => {
            let config = TracerAppConfig::load_config()?;
            let api_key = config.api_key;

            println!(
                "Tracer CLI version {} is setup with {}",
                env!("CARGO_PKG_VERSION"),
                api_key
            );
            Ok(())
        }
        Commands::Tool { name, version } => tool(name, version).await,
        Commands::Version => {
            println!("Tracer version: {}", env!("CARGO_PKG_VERSION"));
            Ok(())
        }
    }
}

async fn metrics() -> Result<()> {
    let mut collector = DiskMetricsCollector::new();

    tokio::spawn(async move {
        collector.collect_disk_usage_metrics().await; // Ensure these are async and await them.
        collector.metrics.send_metrics().await; // Ensure send_metrics is properly awaited.
    });
    Ok(())
}

async fn setup(api_key: String) -> Result<()> {
    setup_tracer(api_key.clone()).await?;

    let config = TracerAppConfig::load_config()?;

    assert_eq!(api_key.clone(), config.api_key);
    metrics().await?;

    setup_fluent_bit(api_key.clone())


    Ok(())
}

async fn start() -> Result<()> {
    println!("Starting new pipeline...");
    let config = TracerAppConfig::load_config()?;

    metrics().await?;
    pipeline_new_run(&config, "[CLI] Starting pipeline run").await?;
    println!("Started pipeline run successfully...");

    Ok(())
}

async fn tool(name: String, version: String) -> Result<()> {
    println!("Processing tool...");
    let tool = Tool { name, version };
    let config = TracerAppConfig::load_config()?;

    metrics().await?;
    tool_process(&config, &tool).await?;
    println!("Tool processed successfully...");

    Ok(())
}

async fn log(message: String) -> Result<()> {
    println!("Logging a message: {}", message);
    let config = TracerAppConfig::load_config()?;

    metrics().await?;
    log_message(&config, &message).await?;

    Ok(())
}

async fn end() -> Result<()> {
    println!("Ending tracer session...");
    let config = TracerAppConfig::load_config()?;

    metrics().await?;
    pipeline_finish_run(&config).await?;
    Ok(())
}

// Import std::process::Command for executing shell commands
use std::process::Command;

async fn update_tracer_cli() -> Result<()> {
    println!("Updating Tracer CLI...");

    // Load the configuration to retrieve the API key
    let config = TracerAppConfig::load_config()?;
    let api_key = config.api_key;

    // Construct the shell command with the appropriate API key
    let install_command = format!(
        "curl -sSL https://raw.githubusercontent.com/davincios/tracer-cli/master/install-tracer.sh | bash -s -- --api-key {} && source ~/.bashrc && tracer help",
        api_key
    );

    // Execute the command using the `sh` shell
    let output = Command::new("sh")
        .arg("-c")
        .arg(&install_command)
        .output()
        .expect("Failed to execute the update command");

    // Print the standard output and error for debugging purposes
    println!("{}", String::from_utf8_lossy(&output.stdout));
    eprintln!("{}", String::from_utf8_lossy(&output.stderr));

    Ok(())
}
