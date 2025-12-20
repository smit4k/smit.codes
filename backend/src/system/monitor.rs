use crate::system::models::{SystemInfo, SystemMetrics};
use sysinfo::{Components, Disks, Networks, System};

pub fn get_system_info() -> SystemInfo {
    let mut sys = System::new_all();
    sys.refresh_all();

    SystemInfo {
        os_name: System::name(),
        os_version: System::os_version(),
        kernel_version: System::kernel_version(),
        hostname: System::host_name(),
        cpu_brand: sys
            .cpus()
            .first()
            .map(|cpu| cpu.brand().to_string())
            .unwrap_or_else(|| "Unknown".to_string()),
        cpu_cores: System::physical_core_count().unwrap(),
        total_memory: sys.total_memory(),
    }
}

pub fn get_system_metrics() -> SystemMetrics {
    let mut sys = System::new_all();
    sys.refresh_cpu_usage();

    std::thread::sleep(sysinfo::MINIMUM_CPU_UPDATE_INTERVAL);

    sys.refresh_cpu_usage();
    sys.refresh_memory();

    let cpu_usage = sys.global_cpu_usage();
    let used_memory = sys.used_memory();
    let total_memory = sys.total_memory();
    let memory_percent = (used_memory as f32 / total_memory as f32) * 100.0;

    SystemMetrics {
        cpu_usage,
        used_memory,
        memory_percent,
        uptime_seconds: System::uptime(),
    }
}
