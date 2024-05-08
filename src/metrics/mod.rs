// src/metrics/mod.rs
mod api;
mod disk;

pub use self::api::MetricsApiHandler;
pub use self::disk::DiskMetricsCollector;

pub struct MetricEntry {
    pub message: String,
    pub name: String,
    pub total_space: u64,
    pub used_space: u64,
    pub available_space: u64,
    pub usage_percentage: f64,
}

pub struct Metrics {
    pub entries: Vec<MetricEntry>,
    pub disk_usage: Vec<String>, // If this is used elsewhere and needs clearing, consider this in implementation.
}

impl Metrics {
    pub fn new() -> Self {
        Metrics {
            entries: vec![],
            disk_usage: vec![],
        }
    }

    pub fn add_metric(&mut self, entry: MetricEntry) {
        self.entries.push(entry);
    }

    /// Sends all collected metrics and clears the entries afterwards.
    pub async fn send_metrics(&mut self) {
        if self.entries.is_empty() {
            log::warn!("Attempted to send metrics, but no metrics are available.");
            return;
        }

        MetricsApiHandler::send_metrics(self).await;
        println!("Metrics sent!");

        // Clear the entries after sending them to ensure they are not sent again.
        self.entries.clear();
        self.disk_usage.clear(); // Clearing disk_usage if it's no longer needed. Remove if it has another purpose.
    }
}

impl Default for Metrics {
    fn default() -> Self {
        Self::new()
    }
}
