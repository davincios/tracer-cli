// src/metrics/mod.rs
mod api;
mod disk;

pub use self::api::MetricsApiHandler;
pub use self::disk::DiskMetricsCollector;

pub struct Metrics {
    pub disk_usage: Vec<String>,
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
