use std::sync::atomic::{AtomicPtr, Ordering};
use std::ptr;

/// A lock-free Stack (Treiber stack)
pub struct LockFreeStack<T> {
    head: AtomicPtr<Node<T>>,
}

struct Node<T> {
    data: T,
    next: *mut Node<T>,
}

impl<T> LockFreeStack<T> {
    pub fn new() -> Self {
        Self {
            head: AtomicPtr::new(ptr::null_mut()),
        }
    }

    pub fn push(&self, data: T) {
        let new_node = Box::into_raw(Box::new(Node {
            data,
            next: ptr::null_mut(),
        }));

        loop {
            let head = self.head.load(Ordering::Relaxed);
            unsafe {
                (*new_node).next = head;
            }

            if self.head.compare_exchange(
                head,
                new_node,
                Ordering::Release,
                Ordering::Relaxed,
            ).is_ok() {
                break;
            }
        }
    }

    pub fn pop(&self) -> Option<T> {
        loop {
            let head = self.head.load(Ordering::Acquire);
            if head.is_null() {
                return None;
            }

            let next = unsafe { (*head).next };

            if self.head.compare_exchange(
                head,
                next,
                Ordering::Release,
                Ordering::Relaxed,
            ).is_ok() {
                let node = unsafe { Box::from_raw(head) };
                return Some(node.data);
            }
        }
    }
}

impl<T> Drop for LockFreeStack<T> {
    fn drop(&mut self) {
        while self.pop().is_some() {}
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;
    use std::thread;

    #[test]
    fn test_lockfree_stack_stress() {
        let stack = Arc::new(LockFreeStack::new());
        let total_items = 100_000;
        let num_threads = 10;
        let items_per_thread = total_items / num_threads;

        let mut threads = Vec::new();
        for _ in 0..num_threads {
            let s = Arc::clone(&stack);
            threads.push(thread::spawn(move || {
                for i in 0..items_per_thread {
                    s.push(i);
                }
            }));
        }

        for t in threads {
            t.join().unwrap();
        }

        let mut threads = Vec::new();
        let popped_count = Arc::new(AtomicUsize::new(0));
        for _ in 0..num_threads {
            let s = Arc::clone(&stack);
            let pc = Arc::clone(&popped_count);
            threads.push(thread::spawn(move || {
                while s.pop().is_some() {
                    pc.fetch_add(1, Ordering::Relaxed);
                }
            }));
        }

        for t in threads {
            t.join().unwrap();
        }

        assert_eq!(popped_count.load(Ordering::Acquire), total_items, "Loss of items in lock-free stack!");
    }
}
