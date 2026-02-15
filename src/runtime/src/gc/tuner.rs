use crate::profiler::PROFILER;
use std::sync::Mutex;

pub struct GcTuner {
    heap_limit: Mutex<usize>,
    growth_factor: f64,
}

impl GcTuner {
    pub fn new(initial_limit: usize) -> Self {
        Self {
            heap_limit: Mutex::new(initial_limit),
            growth_factor: 1.5,
        }
    }

    pub fn adjust(&self, live_size: usize) {
        let mut limit = self.heap_limit.lock().unwrap();
        let pressure = crate::gc::pressure::MONITOR.get_pressure_level();
        
        // If high pressure or live size > 70% of limit, grow limit or trigger aggressive GC
        if pressure > 0.9 {
            // Aggressive mode: slow down growth, trigger more frequent collections
            *limit = (*limit as f64 * 1.1) as usize; 
        } else if live_size as f64 > (*limit as f64 * 0.7) {
            *limit = (*limit as f64 * self.growth_factor) as usize;
        }
    }

    pub fn get_limit(&self) -> usize {
        *self.heap_limit.lock().unwrap()
    }
    
    pub fn report(&self) -> String {
        format!("Heap Limit: {} bytes", self.get_limit())
    }
}

pub static TUNER: std::sync::LazyLock<GcTuner> = std::sync::LazyLock::new(|| GcTuner::new(1024 * 1024 * 10)); // 10MB start
