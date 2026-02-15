use std::collections::HashMap;
use std::sync::Mutex;

pub struct CrossHeapManager {
    // Map from object ptr to thread ID
    refs: Mutex<HashMap<usize, usize>>,
}

impl CrossHeapManager {
    pub fn new() -> Self {
        Self {
            refs: Mutex::new(HashMap::new()),
        }
    }

    pub fn register_ref(&self, ptr: usize, owner_thread: usize) {
        self.refs.lock().unwrap().insert(ptr, owner_thread);
    }

    pub fn check_access(&self, ptr: usize, current_thread: usize) -> bool {
        let refs = self.refs.lock().unwrap();
        if let Some(&owner) = refs.get(&ptr) {
            return owner == current_thread;
        }
        true // Not tracked, assume safe or local
    }
    
    pub fn dump_debug_info(&self) -> String {
        let refs = self.refs.lock().unwrap();
        format!("Cross-heap refs: {}", refs.len())
    }
}

pub static CROSS_HEAP: std::sync::LazyLock<CrossHeapManager> = std::sync::LazyLock::new(CrossHeapManager::new);
