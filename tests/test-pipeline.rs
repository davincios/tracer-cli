#[cfg(test)]
mod tests {
    use anyhow::Result;
    use tracer_cli::methods::{AppConfig, Tool};
    use tracer_cli::{log_message, pipeline_finish_run, pipeline_new_run, tool_process};

    async fn setup() -> AppConfig {
        AppConfig::new().expect("Failed to load configuration for test.")
    }

    #[tokio::test]
    async fn test_full_pipeline_sequence() -> Result<()> {
        let config = setup().await;

        // Init pipeline
        pipeline_new_run(&config, "Starting pipeline run").await?;

        // Process using tool
        let tool = Tool {
            name: "STAR".to_string(),
            version: "2.7.11b".to_string(),
            flags: vec!["--flag1".to_string()],
        };
        tool_process(&config, &tool).await?;

        // Log message
        log_message(&config, "Pipeline run status").await?;

        // Finish pipeline
        pipeline_finish_run(&config).await?;

        Ok(())
    }
}
