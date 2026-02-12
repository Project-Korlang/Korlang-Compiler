use std::sync::Mutex;

pub type TraceFn = fn(*mut u8);

static TRACERS: Mutex<Vec<TraceFn>> = Mutex::new(Vec::new());

pub fn register_tracer(f: TraceFn) {
    TRACERS.lock().unwrap().push(f);
}

pub fn trace(ptr: *mut u8) {
    for f in TRACERS.lock().unwrap().iter() {
        f(ptr);
    }
}

