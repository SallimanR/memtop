use sysinfo::CpuRefreshKind;

use crate::info::shared::system_usage::{SystemUsage, SystemUsageFunctionality};

impl SystemUsageFunctionality for SystemUsage {
    fn update(system: &mut sysinfo::System) -> SystemUsage {
        system.refresh_cpu_specifics(CpuRefreshKind::everything());
        system.refresh_memory();
        SystemUsage {
            total_cpu_usage: system.global_cpu_usage(),
            total_gpu_usage: 0.0,
            total_memory_usage_bytes: system.used_memory(),
        }
    }
}
