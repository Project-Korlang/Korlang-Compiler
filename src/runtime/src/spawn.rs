use crate::scheduler::{Scheduler, Task};
use std::sync::{Arc, LazyLock};

pub static GLOBAL_SCHEDULER: LazyLock<Arc<Scheduler>> = LazyLock::new(|| {
    let threads = std::thread::available_parallelism()
        .map(|p| p.get())
        .unwrap_or(4);
    Scheduler::new(threads)
});

#[no_mangle]
pub extern "C" fn korlang_spawn(task_ptr: *const std::ffi::c_void) {
    // In a real implementation, task_ptr would be a pointer to a closure or state machine
    // For now, we simulate task spawning
    GLOBAL_SCHEDULER.spawn(Box::new(move || {
        println!("[SPAWN] Task executing on worker thread.");
    }));
}

#[no_mangle]
pub extern "C" fn korlang_yield() {
    std::thread::yield_now();
}
