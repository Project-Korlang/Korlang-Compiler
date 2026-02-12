use std::ffi::{CStr, CString};
use std::os::raw::c_char;

#[repr(C)]
pub struct PyObjectHandle {
    ptr: *mut u8,
}

#[no_mangle]
pub extern "C" fn korlang_py_init() -> bool {
    // Placeholder: real implementation will call into the Python C API.
    true
}

#[no_mangle]
pub extern "C" fn korlang_py_eval(code: *const c_char) -> PyObjectHandle {
    if code.is_null() {
        return PyObjectHandle { ptr: std::ptr::null_mut() };
    }
    let _ = unsafe { CStr::from_ptr(code) };
    // Placeholder: return null handle until Python VM is integrated.
    PyObjectHandle { ptr: std::ptr::null_mut() }
}

#[no_mangle]
pub extern "C" fn korlang_py_call(fn_handle: PyObjectHandle, _argc: usize, _argv: *const PyObjectHandle) -> PyObjectHandle {
    let _ = fn_handle.ptr;
    PyObjectHandle { ptr: std::ptr::null_mut() }
}

#[no_mangle]
pub extern "C" fn korlang_py_str(handle: PyObjectHandle) -> *mut c_char {
    let _ = handle.ptr;
    let s = CString::new("<pyobject>").unwrap();
    s.into_raw()
}

#[no_mangle]
pub extern "C" fn korlang_py_free_str(s: *mut c_char) {
    if s.is_null() { return; }
    unsafe { CString::from_raw(s); }
}

