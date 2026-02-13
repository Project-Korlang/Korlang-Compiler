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

#[cfg(test)]
mod tests {
    use super::*;
    use std::thread;

    #[test]
    fn o3_7_tls_isolation_between_threads() {
        let a = Box::into_raw(Box::new(11u8)) as usize;
        let b = Box::into_raw(Box::new(22u8)) as usize;

        let t1 = thread::spawn(move || {
            let a = a as *mut u8;
            push_root(a);
            assert_eq!(roots_snapshot().len(), 1);
            let p = pop_root();
            assert_eq!(p, a);
            unsafe { drop(Box::from_raw(a)); }
        });
        let t2 = thread::spawn(move || {
            let b = b as *mut u8;
            push_root(b);
            assert_eq!(roots_snapshot().len(), 1);
            let p = pop_root();
            assert_eq!(p, b);
            unsafe { drop(Box::from_raw(b)); }
        });

        t1.join().unwrap();
        t2.join().unwrap();
        assert!(roots_snapshot().is_empty());
    }
}
