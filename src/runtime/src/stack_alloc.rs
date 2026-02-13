use std::cell::RefCell;

#[derive(Default)]
struct StackState {
    buf: Vec<u8>,
    sp: usize,
    frames: Vec<usize>,
}

thread_local! {
    static STACK: RefCell<StackState> = RefCell::new(StackState::default());
}

pub fn push_frame() {
    STACK.with(|state| {
        let mut s = state.borrow_mut();
        let sp = s.sp;
        s.frames.push(sp);
    });
}

pub fn pop_frame() {
    STACK.with(|state| {
            let mut s = state.borrow_mut();
            if let Some(sp) = s.frames.pop() {
                s.sp = sp;
                let new_sp = s.sp;
                if new_sp < s.buf.len() {
                    s.buf.truncate(new_sp);
                }
            }
        });
}

pub fn stack_alloc(size: usize, align: usize) -> *mut u8 {
    if size == 0 {
        return std::ptr::null_mut();
    }
    STACK.with(|state| {
        let mut s = state.borrow_mut();
        let align = align.max(1);
        let aligned_sp = (s.sp + (align - 1)) & !(align - 1);
        let new_sp = aligned_sp + size;
        if new_sp > s.buf.len() {
            s.buf.resize(new_sp, 0);
        }
        s.sp = new_sp;
        unsafe { s.buf.as_mut_ptr().add(aligned_sp) }
    })
}

pub fn debug_snapshot() -> (usize, usize, usize) {
    STACK.with(|state| {
        let s = state.borrow();
        (s.sp, s.buf.len(), s.frames.len())
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn o3_6_fiber_stack_grow_and_shrink() {
        push_frame();
        let (_, initial_buf, _) = debug_snapshot();
        let p = stack_alloc(64 * 1024, 16);
        assert!(!p.is_null());
        let (sp_after_alloc, buf_after_alloc, _) = debug_snapshot();
        assert!(sp_after_alloc >= 64 * 1024);
        assert!(buf_after_alloc >= initial_buf);

        pop_frame();
        let (sp_after_pop, buf_after_pop, _) = debug_snapshot();
        assert_eq!(sp_after_pop, 0);
        assert!(buf_after_pop <= buf_after_alloc);
    }
}
