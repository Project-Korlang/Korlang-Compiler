use std::collections::HashSet;
use std::sync::{LazyLock, Mutex};

static PINNED: LazyLock<Mutex<HashSet<usize>>> = LazyLock::new(|| Mutex::new(HashSet::new()));

#[no_mangle]
pub extern "C" fn korlang_pin(ptr: *mut u8) {
    if ptr.is_null() { return; }
    PINNED.lock().unwrap().insert(ptr as usize);
}

#[no_mangle]
pub extern "C" fn korlang_unpin(ptr: *mut u8) {
    if ptr.is_null() { return; }
    PINNED.lock().unwrap().remove(&(ptr as usize));
}

#[no_mangle]
pub extern "C" fn korlang_is_pinned(ptr: *mut u8) -> bool {
    if ptr.is_null() { return false; }
    PINNED.lock().unwrap().contains(&(ptr as usize))
}
