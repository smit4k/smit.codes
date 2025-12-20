use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemInfo {
    pub os_name: Option<String>,
    pub os_version: Option<String>,
    pub kernel_version: Option<String>,
    pub hostname: Option<String>,
    pub cpu_brand: String,
    pub cpu_cores: usize,
    pub total_memory: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemMetrics {
    pub cpu_usage: f32,
    pub used_memory: u64,
    pub memory_percent: f32,
    pub uptime_seconds: u64,
}
