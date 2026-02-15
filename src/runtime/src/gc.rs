use std::alloc::{alloc, dealloc, Layout};
use std::collections::{HashSet, VecDeque};
use std::sync::{Mutex, atomic::{AtomicBool, Ordering}, LazyLock};
use std::time::Instant;
use super::{gc_trace, finalizer};
use crate::profiler::PROFILER;

pub mod tuner;
pub mod cross_heap;
pub mod tiered;
pub mod pressure;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Color {
    White,
    Gray,
    Black,
}

#[derive(Debug, Clone, Copy)]
struct GcObject {
    ptr: usize,
    size: usize,
    align: usize,
    marked: bool,
    color: Color,
    age: u16,
    gen: u16, // 0 young, 1 old
}

pub struct GcHeap {
    objects: Mutex<Vec<GcObject>>,
    roots: Mutex<HashSet<usize>>,
    concurrent: AtomicBool,
}

impl GcHeap {
    pub fn new() -> Self {
        Self {
            objects: Mutex::new(Vec::new()),
            roots: Mutex::new(HashSet::new()),
            concurrent: AtomicBool::new(false),
        }
    }

    pub fn alloc(&self, size: usize, align: usize) -> *mut u8 {
        PROFILER.record_allocation(size);
        let layout = Layout::from_size_align(size, align.max(1)).unwrap();
        let ptr = unsafe { alloc(layout) };
        let obj = GcObject {
            ptr: ptr as usize,
            size,
            align,
            marked: false,
            color: Color::White,
            age: 0,
            gen: 0,
        };
        self.objects.lock().unwrap().push(obj);
        ptr
    }

    pub fn add_root(&self, ptr: *mut u8) {
        self.roots.lock().unwrap().insert(ptr as usize);
    }

    pub fn remove_root(&self, ptr: *mut u8) {
        self.roots.lock().unwrap().remove(&(ptr as usize));
    }

    pub fn predict_pause(&self) -> std::time::Duration {
        PROFILER.get_average_pause()
    }

    pub fn collect(&self) {
        let start = Instant::now();
        let roots = self.roots.lock().unwrap().clone();
        let mut objs = self.objects.lock().unwrap();

        let mut work = VecDeque::new();
        for obj in objs.iter_mut() {
            obj.marked = roots.contains(&obj.ptr);
            obj.color = if obj.marked { Color::Gray } else { Color::White };
            if obj.color == Color::Gray {
                work.push_back(obj.ptr as *mut u8);
            }
        }

        while let Some(ptr) = work.pop_front() {
            if let Some(obj) = objs.iter_mut().find(|o| o.ptr == ptr as usize) {
                if obj.color == Color::Black {
                    continue;
                }
                obj.color = Color::Black;
            }
            gc_trace::trace(ptr);
        }

        let mut i = 0;
        let mut updated_roots = roots.clone();
        while i < objs.len() {
            if !objs[i].marked {
                let obj = objs.remove(i);
                finalizer::run(obj.ptr as *mut u8);
                let layout = Layout::from_size_align(obj.size, obj.align.max(1)).unwrap();
                unsafe { dealloc(obj.ptr as *mut u8, layout) };
            } else {
                if objs[i].gen == 0 {
                    // Move young objects to reduce fragmentation (best-effort).
                    let layout = Layout::from_size_align(objs[i].size, objs[i].align.max(1)).unwrap();
                    let new_ptr = unsafe { alloc(layout) };
                    if !new_ptr.is_null() {
                        unsafe { std::ptr::copy_nonoverlapping(objs[i].ptr as *mut u8, new_ptr, objs[i].size); }
                        if updated_roots.remove(&objs[i].ptr) {
                            updated_roots.insert(new_ptr as usize);
                        }
                        unsafe { dealloc(objs[i].ptr as *mut u8, layout) };
                        objs[i].ptr = new_ptr as usize;
                    }
                }
                objs[i].marked = false;
                objs[i].age = objs[i].age.saturating_add(1);
                if objs[i].age >= 2 {
                    objs[i].gen = 1;
                }
                objs[i].color = Color::White;
                i += 1;
            }
        }
        *self.roots.lock().unwrap() = updated_roots;
        PROFILER.record_gc_pause(start.elapsed());
    }

    pub fn set_concurrent(&self, enabled: bool) {
        self.concurrent.store(enabled, Ordering::Release);
    }

    pub fn is_concurrent(&self) -> bool {
        self.concurrent.load(Ordering::Acquire)
    }
}

pub static GC: LazyLock<GcHeap> = LazyLock::new(GcHeap::new);
