#[cfg(test)]
mod tests {
    use korlang_rt::{
        korlang_arc_alloc_handle, korlang_arc_release_handle, korlang_arc_retain_handle,
        korlang_gc_add_root, korlang_gc_alloc, korlang_gc_collect, korlang_gc_remove_root,
    };

    #[test]
    fn gc_handles_pressure_without_crash() {
        let mut roots = Vec::new();
        for i in 0..20_000usize {
            let p = unsafe { korlang_gc_alloc(64, 8) };
            assert!(!p.is_null(), "gc allocation returned null at iteration {i}");
            unsafe {
                p.write((i & 0xFF) as u8);
                korlang_gc_add_root(p);
            }
            roots.push(p);
        }

        for p in roots.iter().step_by(2) {
            unsafe { korlang_gc_remove_root(*p) };
        }

        for _ in 0..8 {
            unsafe { korlang_gc_collect() };
        }

        for p in roots.iter().skip(1).step_by(2) {
            unsafe { korlang_gc_remove_root(*p) };
        }
        unsafe { korlang_gc_collect() };
    }

    #[test]
    fn repeated_gc_cycles_remain_stable() {
        for i in 0..5_000usize {
            let p = unsafe { korlang_gc_alloc(128, 8) };
            assert!(!p.is_null(), "gc allocation returned null at iteration {i}");
            unsafe {
                p.write((i & 0xFF) as u8);
                korlang_gc_add_root(p);
                korlang_gc_remove_root(p);
            }
            if i % 64 == 0 {
                unsafe { korlang_gc_collect() };
            }
        }
        unsafe { korlang_gc_collect() };
    }

    #[test]
    fn arc_tier3_reference_counting_stress() {
        for i in 0..10_000usize {
            let handle = korlang_arc_alloc_handle(256);
            assert!(
                !handle.is_null(),
                "arc allocation returned null at iteration {i}"
            );
            for _ in 0..16 {
                korlang_arc_retain_handle(handle);
            }
            for _ in 0..16 {
                korlang_arc_release_handle(handle);
            }
            korlang_arc_release_handle(handle);
        }
    }
}
