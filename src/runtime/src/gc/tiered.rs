use std::alloc::{alloc, dealloc, Layout};
use crate::profiler::PROFILER;

pub enum AllocationTier {
    Stack,
    Heap,
    Managed,
}

pub struct TieredAllocator;

impl TieredAllocator {
    pub fn alloc(size: usize, align: usize, tier: AllocationTier) -> *mut u8 {
        match tier {
            AllocationTier::Stack => {
                // Stack allocation is handled by the stack_alloc module
                unsafe { crate::stack_alloc::stack_alloc(size, align) }
            }
            AllocationTier::Heap => {
                let layout = Layout::from_size_align(size, align.max(1)).unwrap();
                unsafe { alloc(layout) }
            }
            AllocationTier::Managed => {
                crate::gc::GC.alloc(size, align)
            }
        }
    }

    pub fn free(ptr: *mut u8, size: usize, align: usize, tier: AllocationTier) {
        match tier {
            AllocationTier::Stack => {
                // No-op, handled by frame pop
            }
            AllocationTier::Heap => {
                let layout = Layout::from_size_align(size, align.max(1)).unwrap();
                unsafe { dealloc(ptr, layout) }
            }
            AllocationTier::Managed => {
                // Handled by GC collection
            }
        }
    }
}
