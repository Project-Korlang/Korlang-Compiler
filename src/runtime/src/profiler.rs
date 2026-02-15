use std::time::{Instant, Duration};
use std::sync::Mutex;
use std::collections::VecDeque;

pub struct Profiler {
    gc_pauses: Mutex<VecDeque<Duration>>,
    allocation_rate: Mutex<usize>, // bytes per second
    last_collection: Mutex<Instant>,
}

impl Profiler {
    pub fn new() -> Self {
        Self {
            gc_pauses: Mutex::new(VecDeque::new()),
            allocation_rate: Mutex::new(0),
            last_collection: Mutex::new(Instant::now()),
        }
    }

    pub fn record_gc_pause(&self, duration: Duration) {
        let mut pauses = self.gc_pauses.lock().unwrap();
        if pauses.len() >= 100 {
            pauses.pop_front();
        }
        pauses.push_back(duration);
    }

    pub fn record_allocation(&self, bytes: usize) {
        let mut rate = self.allocation_rate.lock().unwrap();
        *rate += bytes;
    }

    pub fn get_average_pause(&self) -> Duration {
        let pauses = self.gc_pauses.lock().unwrap();
        if pauses.is_empty() {
            return Duration::from_millis(0);
        }
        let total: Duration = pauses.iter().sum();
        total / (pauses.len() as u32)
    }

    pub fn dump_visualization(&self) -> String {
        let pauses = self.gc_pauses.lock().unwrap();
        let mut out = String::from("GC Pauses (ms): [");
        for (i, p) in pauses.iter().enumerate() {
            if i > 0 { out.push_str(", "); }
            out.push_str(&format!("{:.2}", p.as_secs_f64() * 1000.0));
        }
        out.push(']');
        out
    }
}

pub static PROFILER: std::sync::LazyLock<Profiler> = std::sync::LazyLock::new(Profiler::new);
