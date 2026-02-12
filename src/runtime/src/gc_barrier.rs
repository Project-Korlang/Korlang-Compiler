#[inline]
pub fn write_barrier(_obj: *mut u8, _field: *mut u8) {
    // Fast path no-op for now; hook for generational barrier.
}

#[inline]
pub fn read_barrier(_obj: *mut u8) {
    // Fast path no-op for now; hook for concurrent marking.
}

#[no_mangle]
pub extern "C" fn korlang_write_barrier(obj: *mut u8, field: *mut u8) {
    write_barrier(obj, field)
}

#[no_mangle]
pub extern "C" fn korlang_read_barrier(obj: *mut u8) {
    read_barrier(obj)
}

