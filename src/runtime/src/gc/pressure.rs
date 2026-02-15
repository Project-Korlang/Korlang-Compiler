use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::LazyLock;

pub struct MemoryPressureMonitor {
    allocated_bytes: AtomicUsize,
    pressure_threshold_bytes: usize,
}

impl MemoryPressureMonitor {
    pub fn new(threshold_mb: usize) -> Self {
        Self {
            allocated_bytes: AtomicUsize::new(0),
            pressure_threshold_bytes: threshold_mb * 1024 * 1024,
        }
    }

    pub fn record_allocation(&self, size: usize) {
        self.allocated_bytes.fetch_add(size, Ordering::Relaxed);
    }

    pub fn record_deallocation(&self, size: usize) {
        self.allocated_bytes.fetch_sub(size, Ordering::Relaxed);
    }

    pub fn get_pressure_level(&self) -> f64 {
        let current = self.allocated_bytes.load(Ordering::Relaxed);
        (current as f64) / (self.pressure_threshold_bytes as f64)
    }

    pub fn is_under_high_pressure(&self) -> bool {
        self.get_pressure_level() > 0.85
    }

    pub fn current_usage_mb(&self) -> usize {
        self.allocated_bytes.load(Ordering::Relaxed) / (1024 * 1024)
    }
}

pub static MONITOR: LazyLock<MemoryPressureMonitor> = LazyLock::new(|| {
    // Default to 100MB threshold for pressure alerts in this demo
    MemoryPressureMonitor::new(100)
});
