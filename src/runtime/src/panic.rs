use std::ffi::{CStr, CString};
use std::os::raw::c_char;
use std::sync::Mutex;

static HOOK: Mutex<Option<extern "C" fn(*const c_char)>> = Mutex::new(None);

#[no_mangle]
pub extern "C" fn korlang_set_panic_hook(hook: extern "C" fn(*const c_char)) {
    *HOOK.lock().unwrap() = Some(hook);
}

#[no_mangle]
pub extern "C" fn korlang_panic(msg: *const c_char) -> ! {
    let cstr = if msg.is_null() {
        CStr::from_bytes_with_nul(b"panic\0").unwrap()
    } else {
        unsafe { CStr::from_ptr(msg) }
    };
    if let Some(h) = *HOOK.lock().unwrap() {
        h(cstr.as_ptr());
    } else {
        eprintln!("Korlang panic: {}", cstr.to_string_lossy());
    }
    std::process::abort()
}

#[no_mangle]
pub extern "C" fn korlang_panic_backtrace() -> *mut c_char {
    let s = CString::new("<stack trace unavailable>").unwrap();
    s.into_raw()
}

#[no_mangle]
pub extern "C" fn korlang_panic_free(s: *mut c_char) {
    if s.is_null() { return; }
    unsafe { CString::from_raw(s); }
}

