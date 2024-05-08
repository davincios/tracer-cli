use std::collections::HashSet;

use crate::metrics::Metrics;
use sysinfo::{Disks, System};

use super::MetricEntry;

pub struct DiskMetricsCollector {
    pub metrics: Metrics,
}

impl Default for DiskMetricsCollector {
    fn default() -> Self {
        Self::new()
    }
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
        let mut seen_disks = HashSet::new(); // Initialize the set here within the function scope

        for disk in &disks {
            let name = disk.name().to_string_lossy().into_owned();

            // Skip collecting if this disk name is already seen in this function execution
            if seen_disks.contains(&name) {
                println!("Skipping duplicate disk metric for: {}", name);
                continue;
            }

            // Mark this disk as collected within this function execution
            seen_disks.insert(name.clone());

            let total_space = disk.total_space();
            let available_space = disk.available_space();
            let used_space = total_space - available_space;
            let usage_percentage = (used_space as f64 / total_space as f64) * 100.0;

            let metric_message = format!("Disk Usage: {:.2}% for {}", usage_percentage, name);

            // Add the new metric entry
            self.metrics.add_metric(MetricEntry {
                message: metric_message,
                name,
                total_space,
                used_space,
                available_space,
                usage_percentage,
            });
        }
        // At the end of the function execution, `seen_disks` goes out of scope and is dropped.
    }
}
