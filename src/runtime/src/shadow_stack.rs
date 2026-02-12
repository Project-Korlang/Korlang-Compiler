use std::cell::RefCell;
use super::gc;

thread_local! {
    static ROOTS: RefCell<Vec<*mut u8>> = RefCell::new(Vec::new());
}

pub fn push_root(ptr: *mut u8) {
    if ptr.is_null() {
        return;
    }
    ROOTS.with(|r| r.borrow_mut().push(ptr));
    gc::GC.add_root(ptr);
}

pub fn pop_root() -> *mut u8 {
    let mut out = std::ptr::null_mut();
    ROOTS.with(|r| {
        let mut v = r.borrow_mut();
        if let Some(ptr) = v.pop() {
            gc::GC.remove_root(ptr);
            out = ptr;
        }
    });
    out
}

pub fn roots_snapshot() -> Vec<*mut u8> {
    ROOTS.with(|r| r.borrow().clone())
}

