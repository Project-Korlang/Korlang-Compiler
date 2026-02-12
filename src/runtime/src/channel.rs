use std::cell::UnsafeCell;
use std::mem::MaybeUninit;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Condvar, Mutex};

// Lock-free SPSC ring buffer
pub struct SpscChannel<T> {
    mask: usize,
    buffer: Vec<UnsafeCell<MaybeUninit<T>>>,
    head: AtomicUsize,
    tail: AtomicUsize,
}

unsafe impl<T: Send> Send for SpscChannel<T> {}
unsafe impl<T: Send> Sync for SpscChannel<T> {}

impl<T> SpscChannel<T> {
    pub fn with_capacity(cap: usize) -> Self {
        let cap = cap.next_power_of_two().max(2);
        let mut buf = Vec::with_capacity(cap);
        for _ in 0..cap {
            buf.push(UnsafeCell::new(MaybeUninit::uninit()));
        }
        Self {
            mask: cap - 1,
            buffer: buf,
            head: AtomicUsize::new(0),
            tail: AtomicUsize::new(0),
        }
    }

    pub fn send(&self, value: T) -> Result<(), T> {
        let head = self.head.load(Ordering::Relaxed);
        let next = (head + 1) & self.mask;
        let tail = self.tail.load(Ordering::Acquire);
        if next == tail {
            return Err(value);
        }
        unsafe { (*self.buffer[head].get()).write(value); }
        self.head.store(next, Ordering::Release);
        Ok(())
    }

    pub fn recv(&self) -> Option<T> {
        let tail = self.tail.load(Ordering::Relaxed);
        let head = self.head.load(Ordering::Acquire);
        if tail == head {
            return None;
        }
        let val = unsafe { (*self.buffer[tail].get()).assume_init_read() };
        self.tail.store((tail + 1) & self.mask, Ordering::Release);
        Some(val)
    }
}

// MPMC channel using a mutex fallback (not lock-free)
pub struct MpmcChannel<T> {
    q: Mutex<Vec<T>>,
    cv: Condvar,
}

impl<T> MpmcChannel<T> {
    pub fn new() -> Self {
        Self {
            q: Mutex::new(Vec::new()),
            cv: Condvar::new(),
        }
    }

    pub fn send(&self, value: T) {
        let mut q = self.q.lock().unwrap();
        q.push(value);
        self.cv.notify_one();
    }

    pub fn recv(&self) -> T {
        let mut q = self.q.lock().unwrap();
        loop {
            if let Some(v) = q.pop() {
                return v;
            }
            q = self.cv.wait(q).unwrap();
        }
    }
}

