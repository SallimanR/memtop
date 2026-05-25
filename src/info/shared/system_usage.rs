#[derive(Debug, Clone, Copy, Default)]
pub struct SystemUsage {
    pub total_cpu_usage: f32,
    pub total_gpu_usage: f32,
    pub total_memory_usage_bytes: u64,
}

pub trait SystemUsageFunctionality {
    fn update(system: &mut sysinfo::System) -> SystemUsage;
}
