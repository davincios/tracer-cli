// how can I import the metrics module function collect_disk metrics in src/lib.rs and src/main.rs?
// src/metrics/mod.rs
use serde_json::json;
use sysinfo::{Disks, System};

use crate::{log_message, methods::EventStatus, send_event};

pub struct Metrics {
    disk_usage: Vec<String>,
}

impl Metrics {
    pub fn new() -> Self {
        Metrics { disk_usage: vec![] }
    }

    pub fn add_metric(&mut self, metric: String) {
        self.disk_usage.push(metric);
    }
}

impl Default for Metrics {
    fn default() -> Self {
        Self::new()
    }
}

pub struct MetricsCollector {
    metrics: Metrics,
}
impl Default for MetricsCollector {
    fn default() -> Self {
        Self::new()
    }
}

impl MetricsCollector {
    pub fn new() -> Self {
        MetricsCollector {
            metrics: Metrics::new(),
        }
    }

    pub async fn collect_disk_usage_metrics(&mut self) {
        let mut sys = System::new_all(); // Create a new system object
        sys.refresh_all(); // Refresh all information

        let disks = Disks::new_with_refreshed_list();
        for disk in &disks {
            println!("{disk:?}");
            let name = disk.name().to_string_lossy().into_owned();
            let total_space = disk.total_space();
            let available_space = disk.available_space();
            let used_space = total_space - available_space;
            let usage_percentage = (used_space as f64 / total_space as f64) * 100.0;

            let metric = format!(
                "Disk: {}, Total Space: {} bytes, Used Space: {} bytes, Available Space: {} bytes, Usage: {:.2}%",
                name,
                total_space,
                used_space,
                available_space,
                usage_percentage
            );

            let config = crate::TracerAppConfig::load_config().unwrap();

            // Send the metric as an event
            log_message(&config, &metric).await.unwrap();
            // please fix the send event usage
            let attributes = json!({
                "name": name,
                "total_space": total_space,
                "used_space": used_space,
                "available_space": available_space,
                "usage_percentage": usage_percentage
            });

            send_event(
                &config,
                EventStatus::MetricEvent.as_str(),
                &metric,
                Some(attributes),
            )
            .await
            .unwrap();

            self.metrics.add_metric(metric);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_collect_disk_usage_metrics() {
        let mut collector = MetricsCollector::new();
        collector.collect_disk_usage_metrics().await;
        assert!(
            !collector.metrics.disk_usage.is_empty(),
            "Metrics should not be empty after collecting disk usage."
        );
    }
}
