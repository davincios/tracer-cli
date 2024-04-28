use anyhow::Result;
use config::{ConfigBuilder, ConfigError, File, FileFormat};
use tracer_cli::methods::AppConfig;

impl AppConfig {
    pub fn new() -> Result<Self, ConfigError> {
        // Explicitly starting with the default state
        let builder = ConfigBuilder::<config::builder::DefaultState>::default().add_source(
            File::with_name("Settings")
                .required(false)
                .format(FileFormat::Toml),
        );
        let settings = builder.build()?;
        let config: AppConfig = settings.try_deserialize()?;

        // Create the client and include it in the returned AppConfig
        let client = Arc::new(Client::new());
        Ok(AppConfig {
            base_url: config.base_url,
            api_key: config.api_key,
            client,
        })
    }
}
#[cfg(test)]
mod tests {
    use anyhow::Error;
    use tracer_cli::methods::AppConfig;
    use tracer_cli::methods::Tool;
    use tracer_cli::{log_message, pipeline_finish, pipeline_init, tool_process}; // Add this import statement // Add this import statement // Add this import statement

    #[tokio::test]
    async fn pipeline_test() -> Result<(), Error> {
        let config = AppConfig::new().expect("Failed to load configuration for test.");
        let tool = Tool {
            name: "Sample Tool".to_string(),
            version: "1.0.0".to_string(),
            flags: vec!["--flag1".to_string()],
        };

        pipeline_init(&config, "Starting pipeline run").await?;
        tool_process(&config, &tool).await?;
        log_message(&config, "Pipeline run status").await?;
        pipeline_finish(&config).await?;

        Ok(())
    }
}
