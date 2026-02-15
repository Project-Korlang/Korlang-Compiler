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
    // Monitoring
    sent_count: AtomicUsize,
    recv_count: AtomicUsize,
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
            sent_count: AtomicUsize::new(0),
            recv_count: AtomicUsize::new(0),
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
        self.sent_count.fetch_add(1, Ordering::Relaxed);
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
        self.recv_count.fetch_add(1, Ordering::Relaxed);
        Some(val)
    }

    pub fn get_metrics(&self) -> (usize, usize) {
        (self.sent_count.load(Ordering::Relaxed), self.recv_count.load(Ordering::Relaxed))
    }
}

// MPMC channel using a mutex fallback (not lock-free)
pub struct MpmcChannel<T> {
    q: Mutex<Vec<T>>,
    cv: Condvar,
    // Monitoring
    sent_count: AtomicUsize,
    recv_count: AtomicUsize,
}

#[cfg(test)]
mod tests {
    use super::SpscChannel;
    use std::sync::Arc;
    use std::thread;

    #[test]
    fn o3_5_wait_free_queue_integrity_under_contention() {
        let ch = Arc::new(SpscChannel::with_capacity(1024));
        let total = 200_000usize;
        let producer = {
            let ch = Arc::clone(&ch);
            thread::spawn(move || {
                for i in 0..total {
                    loop {
                        if ch.send(i).is_ok() {
                            break;
                        }
                        std::hint::spin_loop();
                    }
                }
            })
        };

        let consumer = {
            let ch = Arc::clone(&ch);
            thread::spawn(move || {
                let mut next = 0usize;
                while next < total {
                    if let Some(v) = ch.recv() {
                        assert_eq!(v, next, "out-of-order or corrupted queue element");
                        next += 1;
                    } else {
                        std::hint::spin_loop();
                    }
                }
            })
        };

        producer.join().unwrap();
        consumer.join().unwrap();
    }
}

impl<T> MpmcChannel<T> {
    pub fn new() -> Self {
        Self {
            q: Mutex::new(Vec::new()),
            cv: Condvar::new(),
            sent_count: AtomicUsize::new(0),
            recv_count: AtomicUsize::new(0),
        }
    }

    pub fn send(&self, value: T) {
        let mut q = self.q.lock().unwrap();
        q.push(value);
        self.sent_count.fetch_add(1, Ordering::Relaxed);
        self.cv.notify_one();
    }

    pub fn recv(&self) -> T {
        let mut q = self.q.lock().unwrap();
        loop {
            if let Some(v) = q.pop() {
                self.recv_count.fetch_add(1, Ordering::Relaxed);
                return v;
            }
            q = self.cv.wait(q).unwrap();
        }
    }

    pub fn get_metrics(&self) -> (usize, usize) {
        (self.sent_count.load(Ordering::Relaxed), self.recv_count.load(Ordering::Relaxed))
    }
}
