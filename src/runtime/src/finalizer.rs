use std::collections::HashMap;
use std::sync::{LazyLock, Mutex};

pub type Finalizer = fn(*mut u8);

static FINALIZERS: LazyLock<Mutex<HashMap<usize, Finalizer>>> = LazyLock::new(|| Mutex::new(HashMap::new()));

pub fn register(ptr: *mut u8, f: Finalizer) {
    if ptr.is_null() { return; }
    FINALIZERS.lock().unwrap().insert(ptr as usize, f);
}

pub fn take(ptr: *mut u8) -> Option<Finalizer> {
    FINALIZERS.lock().unwrap().remove(&(ptr as usize))
}

pub fn run(ptr: *mut u8) {
    if let Some(f) = take(ptr) {
        f(ptr);
    }
}
