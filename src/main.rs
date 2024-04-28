use anyhow::Result;
use clap::{Parser, Subcommand};
use tokio; // Assuming you are using Tokio for async runtime
use tracer_cli::{pipeline_new_run, AppConfig};

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
    /// Start tracing
    Start { api_key: String },
    /// Log a message with a type
    Log {
        #[clap(long)]
        r#type: String,

        message: String,
    },
    /// End tracing
    End,
}

#[tokio::main] // Adding the async entry point
async fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Start { api_key } => start(&api_key).await,
        Commands::Log { r#type, message } => log(r#type, message),
        Commands::End => end(),
    }
}

async fn start(api_key: &str) -> Result<()> {
    println!("Starting tracer...");
    let config = AppConfig::new(Some(api_key)).expect("Failed to load configuration.");

    pipeline_new_run(&config, "[CLI] Starting pipeline run").await?;
    println!("Started piepline run successfully...");

    Ok(())
}

fn log(log_type: String, message: String) -> Result<()> {
    println!("Logging a {} message: {}", log_type, message);
    Ok(())
}

fn end() -> Result<()> {
    println!("Ending tracer session...");
    Ok(())
}
