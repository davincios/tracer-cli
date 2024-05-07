use crate::metrics::Metrics;
use sysinfo::{Disks, System};

pub struct DiskMetricsCollector {
    metrics: Metrics,
}

impl DiskMetricsCollector {
    pub fn new() -> Self {
        DiskMetricsCollector {
            metrics: Metrics::new(),
        }
    }

    pub async fn collect_disk_usage_metrics(&mut self) {
        let mut sys = System::new_all();
        sys.refresh_all();

        let disks = Disks::new_with_refreshed_list();
        for disk in &disks {
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

            self.metrics.add_metric(metric);
        }
    }
}
