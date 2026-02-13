#[cfg(test)]
mod tests {
    use korlang_rt::{korlang_alloc, korlang_free};

    #[test]
    fn native_allocator_stress_small_blocks() {
        let mut ptrs = Vec::new();
        for i in 1..=50_000usize {
            let size = (i % 128) + 1;
            let p = unsafe { korlang_alloc(size, 8) };
            assert!(!p.is_null(), "allocation failed at iteration {i}");
            unsafe {
                std::ptr::write_bytes(p, 0xA5, size);
            }
            ptrs.push((p, size));
        }

        for (p, size) in ptrs {
            unsafe { korlang_free(p, size, 8) };
        }
    }

    #[test]
    fn native_allocator_stress_large_blocks() {
        let mut ptrs = Vec::new();
        for i in 0..2_000usize {
            let size = 4096 + (i % 1024);
            let p = unsafe { korlang_alloc(size, 16) };
            assert!(!p.is_null(), "large allocation failed at iteration {i}");
            unsafe {
                std::ptr::write_bytes(p, 0x3C, size);
            }
            ptrs.push((p, size));
        }

        for (p, size) in ptrs.into_iter().rev() {
            unsafe { korlang_free(p, size, 16) };
        }
    }
}
