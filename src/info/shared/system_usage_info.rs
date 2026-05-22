#[derive(Debug, Clone, Copy, Default)]
pub struct SystemUsageInfo {
    pub total_cpu_usage: f32,
    pub total_gpu_usage: f32,
    pub total_memory_usage_mb: u64,
}

pub trait SystemUsageInfoFunctionality {
    fn update(system: &mut sysinfo::System) -> SystemUsageInfo;
}
