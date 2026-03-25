use sysinfo::{CpuExt, System, SystemExt};
use serde::Serialize;

// Represents the current state of hardware resources to be sent to the frontend
#[derive(Debug, Serialize)]
pub struct SystemResources {
    pub cpu_usage_percentage: f32,
    pub available_memory_mb: u64,
    pub total_memory_mb: u64,
}

// Encapsulates the system monitor to prevent memory leaks and redundant instantiations
pub struct HardwareMonitor {
    sys: System,
}

impl HardwareMonitor {
    // Initializes the monitor with existing system metrics
    pub fn new() -> Self {
        HardwareMonitor {
            sys: System::new_all(),
        }
    }

    // Refreshes hardware data and calculates current averages
    pub fn get_current_resources(&mut self) -> SystemResources {
        self.sys.refresh_cpu();
        self.sys.refresh_memory();

        let cpus = self.sys.cpus();
        let mut total_cpu_usage = 0.0;
        
        for cpu in cpus {
            total_cpu_usage += cpu.cpu_usage();
        }
        
        let average_cpu_usage = if cpus.is_empty() {
            0.0
        } else {
            total_cpu_usage / cpus.len() as f32
        };

        // Convert bytes to Megabytes for easier frontend consumption
        let available_mem = self.sys.available_memory() / 1024 / 1024;
        let total_mem = self.sys.total_memory() / 1024 / 1024;

        SystemResources {
            cpu_usage_percentage: average_cpu_usage,
            available_memory_mb: available_mem,
            total_memory_mb: total_mem,
        }
    }
}