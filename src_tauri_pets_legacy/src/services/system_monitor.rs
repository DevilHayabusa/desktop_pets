use sysinfo::{System, Cpu};
use serde::Serialize;

// Data structure to be sent to the React frontend
#[derive(Debug, Serialize)]
pub struct SystemResources {
    pub cpu_usage_percentage: f32,
    pub used_memory_mb: u64,      // Changed from available to used
    pub total_memory_mb: u64,
}

pub struct HardwareMonitor {
    sys: System,
}

impl HardwareMonitor {
    pub fn new() -> Self {
        HardwareMonitor {
            sys: System::new_all(),
        }
    }

    pub fn get_current_resources(&mut self) -> SystemResources {
        // Refresh system metrics
        self.sys.refresh_cpu_usage();
        self.sys.refresh_memory();

        // Calculate average CPU usage across all cores
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

        // Memory calculations: Windows reports 'available', but users prefer 'used'
        let total_bytes = self.sys.total_memory();
        let available_bytes = self.sys.available_memory();
        
        // Logical Used Memory = Total - Available
        let used_bytes = total_bytes - available_bytes;

        let used_mb = used_bytes / 1024 / 1024;
        let total_mb = total_bytes / 1024 / 1024;

        SystemResources {
            cpu_usage_percentage: average_cpu_usage,
            used_memory_mb: used_mb,
            total_memory_mb: total_mb,
        }
    }
}