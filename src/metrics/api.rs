// src/metrics/api.rs
use crate::methods::EventStatus;
use crate::metrics::Metrics;
use crate::send_event;
use crate::TracerAppConfig;
use serde_json::json;

pub struct MetricsApiHandler;

impl MetricsApiHandler {
    pub async fn send_metrics(metrics: &Metrics) {
        let config = TracerAppConfig::load_config().unwrap();

        for metric in &metrics.disk_usage {
            let attributes = json!({
                "details": metric
            });

            send_event(
                &config,
                EventStatus::MetricEvent.as_str(),
                metric,
                Some(attributes),
            )
            .await
            .unwrap();
        }
    }
}
