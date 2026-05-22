use sysinfo::CpuRefreshKind;

use crate::info::shared::system_usage_info::{SystemUsageInfo, SystemUsageInfoFunctionality};

impl SystemUsageInfoFunctionality for SystemUsageInfo {
    fn update(system: &mut sysinfo::System) -> SystemUsageInfo {
        system.refresh_cpu_specifics(CpuRefreshKind::everything());
        system.refresh_memory();
        SystemUsageInfo {
            total_cpu_usage: system.global_cpu_usage(),
            total_gpu_usage: 0.0,
            total_memory_usage_mb: system.used_memory(),
        }
    }
}
