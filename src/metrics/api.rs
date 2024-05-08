// src/metrics/api.rs
use crate::methods::EventStatus;
use crate::metrics::Metrics;
use crate::send_event;
use crate::TracerAppConfig;
use log::{error, info};
use serde_json::json; // Ensure you have the `log` crate and an appropriate logging implementation.

pub struct MetricsApiHandler;

impl MetricsApiHandler {
    pub async fn send_metrics(metrics: &Metrics) {
        // Check if there are any metrics available to send
        if metrics.entries.is_empty() {
            error!("No metrics collected to send to the API.");
            return;
        }

        // Load the tracer application configuration
        let config = match TracerAppConfig::load_config() {
            Ok(cfg) => cfg,
            Err(err) => {
                error!("Failed to load tracer config: {:?}", err);
                return;
            }
        };

        // Iterate through each collected metric and send it to the server
        for metric in &metrics.entries {
            let attributes = json!({
                "name": metric.name,
                "total_space": metric.total_space,
                "used_space": metric.used_space,
                "available_space": metric.available_space,
                "usage_percentage": metric.usage_percentage,
            });

            // Log which metric is being sent
            info!("Sending metric: {}", metric.message);

            // Attempt to send the event and handle errors appropriately
            match send_event(
                &config,
                EventStatus::MetricEvent.as_str(),
                &metric.message,
                Some(attributes),
                false,
            )
            .await
            {
                Ok(_) => info!("Metric sent successfully: {}", metric.message),
                Err(e) => error!("Failed to send metric: {}. Error: {:?}", metric.message, e),
            }
        }
    }
}
