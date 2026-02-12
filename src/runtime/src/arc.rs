use std::sync::atomic::{AtomicUsize, Ordering};

#[repr(C)]
pub struct ArcBuf {
    ptr: *mut u8,
    size: usize,
    mmap: bool,
    refcnt: AtomicUsize,
}

impl ArcBuf {
    fn new(ptr: *mut u8, size: usize, mmap: bool) -> *mut ArcBuf {
        let buf = Box::new(ArcBuf {
            ptr,
            size,
            mmap,
            refcnt: AtomicUsize::new(1),
        });
        Box::into_raw(buf)
    }
}

#[cfg(unix)]
fn mmap_alloc(size: usize) -> *mut u8 {
    use libc::{mmap, MAP_ANON, MAP_FAILED, MAP_PRIVATE, PROT_READ, PROT_WRITE};
    unsafe {
        let ptr = mmap(std::ptr::null_mut(), size, PROT_READ | PROT_WRITE, MAP_PRIVATE | MAP_ANON, -1, 0);
        if ptr == MAP_FAILED {
            std::ptr::null_mut()
        } else {
            ptr as *mut u8
        }
    }
}

#[cfg(unix)]
fn mmap_free(ptr: *mut u8, size: usize) {
    unsafe { libc::munmap(ptr as *mut _, size); }
}

#[cfg(not(unix))]
fn mmap_alloc(size: usize) -> *mut u8 {
    let mut v = Vec::<u8>::with_capacity(size);
    let ptr = v.as_mut_ptr();
    std::mem::forget(v);
    ptr
}

#[cfg(not(unix))]
fn mmap_free(ptr: *mut u8, size: usize) {
    unsafe { Vec::from_raw_parts(ptr, 0, size); }
}

#[no_mangle]
pub extern "C" fn korlang_arc_alloc(size: usize) -> *mut ArcBuf {
    if size == 0 {
        return std::ptr::null_mut();
    }
    let mut v = Vec::<u8>::with_capacity(size);
    let ptr = v.as_mut_ptr();
    std::mem::forget(v);
    ArcBuf::new(ptr, size, false)
}

#[no_mangle]
pub extern "C" fn korlang_arc_alloc_mmap(size: usize) -> *mut ArcBuf {
    if size == 0 {
        return std::ptr::null_mut();
    }
    let ptr = mmap_alloc(size);
    if ptr.is_null() {
        return std::ptr::null_mut();
    }
    ArcBuf::new(ptr, size, true)
}

#[no_mangle]
pub extern "C" fn korlang_arc_retain(buf: *mut ArcBuf) {
    if buf.is_null() { return; }
    unsafe { (*buf).refcnt.fetch_add(1, Ordering::Relaxed); }
}

#[no_mangle]
pub extern "C" fn korlang_arc_release(buf: *mut ArcBuf) {
    if buf.is_null() { return; }
    let should_free = unsafe { (*buf).refcnt.fetch_sub(1, Ordering::Release) == 1 };
    if should_free {
        std::sync::atomic::fence(Ordering::Acquire);
        unsafe {
            let ptr = (*buf).ptr;
            let size = (*buf).size;
            let mmap = (*buf).mmap;
            drop(Box::from_raw(buf));
            if mmap {
                mmap_free(ptr, size);
            } else {
                Vec::from_raw_parts(ptr, 0, size);
            }
        }
    }
}

#[no_mangle]
pub extern "C" fn korlang_arc_ptr(buf: *mut ArcBuf) -> *mut u8 {
    if buf.is_null() { return std::ptr::null_mut(); }
    unsafe { (*buf).ptr }
}

#[no_mangle]
pub extern "C" fn korlang_arc_size(buf: *mut ArcBuf) -> usize {
    if buf.is_null() { return 0; }
    unsafe { (*buf).size }
}

