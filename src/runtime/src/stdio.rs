use std::io::{self, Read, Write};

#[no_mangle]
pub extern "C" fn korlang_io_print(ptr: *const u8, len: usize) {
    if ptr.is_null() { return; }
    let slice = unsafe { std::slice::from_raw_parts(ptr, len) };
    let _ = io::stdout().write_all(slice);
}

#[no_mangle]
pub extern "C" fn korlang_io_println(ptr: *const u8, len: usize) {
    if ptr.is_null() { return; }
    let slice = unsafe { std::slice::from_raw_parts(ptr, len) };
    let mut out = io::stdout();
    let _ = out.write_all(slice);
    let _ = out.write_all(b"\n");
}

#[no_mangle]
pub extern "C" fn korlang_io_read_line(buf: *mut u8, max: usize) -> usize {
    if buf.is_null() || max == 0 { return 0; }
    let mut input = String::new();
    let _ = io::stdin().read_line(&mut input);
    let bytes = input.as_bytes();
    let n = bytes.len().min(max - 1);
    unsafe { std::ptr::copy_nonoverlapping(bytes.as_ptr(), buf, n); }
    unsafe { *buf.add(n) = 0; }
    n
}

