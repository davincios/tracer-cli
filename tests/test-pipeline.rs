use config::{Config, File, FileFormat};

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::Result;
    use tokio;
    use tracer::methods::Tool;
    use tracer::{log_message, pipeline_finish_run, pipeline_new_run, tool_process, TracerConfig};

    async fn setup() -> Result<TracerConfig> {
        let settings = Config::builder()
            .add_source(
                File::with_name("Settings")
                    .required(false)
                    .format(FileFormat::Toml),
            )
            .build()?;

        let config: TracerConfig = settings.try_deserialize()?;
        Ok(config)
    }

    #[tokio::test]
    async fn test_full_pipeline_sequence() -> Result<()> {
        let config = setup().await?;

        // Init pipeline
        pipeline_new_run(&config, "Starting pipeline run").await?;

        // Process using tool
        let tool = Tool {
            name: "STAR".to_string(),
            version: "2.7.11b".to_string(),
        };
        tool_process(&config, &tool).await?;

        // Log message
        log_message(&config, "QC mapping reads GC content below 61% threshold").await?;

        // Finish pipeline
        pipeline_finish_run(&config).await?;

        Ok(())
    }
}
