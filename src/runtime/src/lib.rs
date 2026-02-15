use std::alloc::{alloc, dealloc, Layout};

pub mod profiler;
pub mod gc;
mod gc_trace;
mod stack_alloc;
mod arc;
mod pin;
mod python_bridge;
mod shadow_stack;
mod finalizer;
mod scheduler;
mod channel;
mod blocking;
mod stdio;
mod panic;
mod driver;
pub mod ui;
pub mod cloud;
pub mod tensor;
mod gc_barrier;
mod lockfree;
mod coroutine;
mod async_rt;
mod spawn;

#[no_mangle]
pub extern "C" fn korlang_alloc(size: usize, align: usize) -> *mut u8 {
    if size == 0 {
        return std::ptr::null_mut();
    }
    let layout = Layout::from_size_align(size, align.max(1)).unwrap();
    unsafe { alloc(layout) }
}

#[no_mangle]
pub extern "C" fn korlang_free(ptr: *mut u8, size: usize, align: usize) {
    if ptr.is_null() || size == 0 {
        return;
    }
    let layout = Layout::from_size_align(size, align.max(1)).unwrap();
    unsafe { dealloc(ptr, layout) }
}

#[no_mangle]
pub extern "C" fn korlang_gc_alloc(size: usize, align: usize) -> *mut u8 {
    gc::GC.alloc(size, align)
}

#[no_mangle]
pub extern "C" fn korlang_gc_collect() {
    gc::GC.collect();
}

#[no_mangle]
pub extern "C" fn korlang_gc_add_root(ptr: *mut u8) {
    gc::GC.add_root(ptr);
}

#[no_mangle]
pub extern "C" fn korlang_gc_remove_root(ptr: *mut u8) {
    gc::GC.remove_root(ptr);
}

#[no_mangle]
pub extern "C" fn korlang_gc_set_concurrent(enabled: bool) {
    gc::GC.set_concurrent(enabled);
}

#[no_mangle]
pub extern "C" fn korlang_gc_register_tracer(f: gc_trace::TraceFn) {
    gc_trace::register_tracer(f);
}

#[no_mangle]
pub extern "C" fn korlang_shadow_stack_push(ptr: *mut u8) {
    shadow_stack::push_root(ptr);
}

#[no_mangle]
pub extern "C" fn korlang_shadow_stack_pop() -> *mut u8 {
    shadow_stack::pop_root()
}

#[no_mangle]
pub extern "C" fn korlang_finalizer_register(ptr: *mut u8, f: finalizer::Finalizer) {
    finalizer::register(ptr, f);
}

#[no_mangle]
pub extern "C" fn korlang_stack_alloc(size: usize) -> *mut u8 {
    stack_alloc::stack_alloc(size, 8)
}

#[no_mangle]
pub extern "C" fn korlang_stack_alloc_aligned(size: usize, align: usize) -> *mut u8 {
    stack_alloc::stack_alloc(size, align)
}

#[no_mangle]
pub extern "C" fn korlang_stack_push_frame() {
    stack_alloc::push_frame();
}

#[no_mangle]
pub extern "C" fn korlang_stack_pop_frame() {
    stack_alloc::pop_frame();
}

#[no_mangle]
pub extern "C" fn korlang_main(
    entry: extern "C" fn(i32, *const *const u8) -> i32,
    argc: i32,
    argv: *const *const u8,
) -> i32 {
    entry(argc, argv)
}

#[no_mangle]
pub extern "C" fn korlang_driver() {
    driver::korlang_driver();
}

pub fn korlang_arc_alloc_handle(size: usize) -> *mut std::ffi::c_void {
    arc::korlang_arc_alloc(size) as *mut std::ffi::c_void
}

pub fn korlang_arc_retain_handle(handle: *mut std::ffi::c_void) {
    arc::korlang_arc_retain(handle as *mut arc::ArcBuf);
}

pub fn korlang_arc_release_handle(handle: *mut std::ffi::c_void) {
    arc::korlang_arc_release(handle as *mut arc::ArcBuf);
}

#[no_mangle]
pub extern "C" fn korlang_int_add(a: i64, b: i64) -> i64 {
    a.wrapping_add(b)
}

#[no_mangle]
pub extern "C" fn korlang_int_sub(a: i64, b: i64) -> i64 {
    a.wrapping_sub(b)
}

#[no_mangle]
pub extern "C" fn korlang_int_mul(a: i64, b: i64) -> i64 {
    a.wrapping_mul(b)
}

#[no_mangle]
pub extern "C" fn korlang_int_div(a: i64, b: i64) -> i64 {
    if b == 0 { 0 } else { a / b }
}

#[no_mangle]
pub extern "C" fn korlang_float_add(a: f64, b: f64) -> f64 {
    a + b
}

#[no_mangle]
pub extern "C" fn korlang_float_sub(a: f64, b: f64) -> f64 {
    a - b
}

#[no_mangle]
pub extern "C" fn korlang_float_mul(a: f64, b: f64) -> f64 {
    a * b
}

#[no_mangle]
pub extern "C" fn korlang_float_div(a: f64, b: f64) -> f64 {
    if b == 0.0 { f64::NAN } else { a / b }
}
