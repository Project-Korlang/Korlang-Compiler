use std::collections::VecDeque;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Arc, Condvar, Mutex};
use std::thread;

pub type Task = Box<dyn FnOnce() + Send + 'static>;

struct Worker {
    queue: Mutex<VecDeque<Task>>,
    cv: Condvar,
}

thread_local! {
    static WORKER_ID: AtomicUsize = const { AtomicUsize::new(usize::MAX) };
}

pub fn current_worker_id() -> Option<usize> {
    WORKER_ID.with(|id| {
        let v = id.load(Ordering::Relaxed);
        if v == usize::MAX { None } else { Some(v) }
    })
}

pub struct Scheduler {
    workers: Vec<Arc<Worker>>,
    next: Mutex<usize>,
}

impl Scheduler {
    pub fn new(threads: usize) -> Arc<Self> {
        let mut workers = Vec::new();
        for _ in 0..threads.max(1) {
            workers.push(Arc::new(Worker {
                queue: Mutex::new(VecDeque::new()),
                cv: Condvar::new(),
            }));
        }

        let sched = Arc::new(Self {
            workers,
            next: Mutex::new(0),
        });

        for idx in 0..sched.workers.len() {
            let s = Arc::clone(&sched);
            thread::spawn(move || worker_loop(s, idx));
        }

        sched
    }

    pub fn spawn(self: &Arc<Self>, task: Task) {
        let idx = {
            let mut n = self.next.lock().unwrap();
            let idx = *n % self.workers.len();
            *n = idx + 1;
            idx
        };
        let w = &self.workers[idx];
        let mut q = w.queue.lock().unwrap();
        q.push_back(task);
        w.cv.notify_one();
    }
}

fn worker_loop(s: Arc<Scheduler>, idx: usize) {
    WORKER_ID.with(|id| id.store(idx, Ordering::Relaxed));
    loop {
        let task = {
            let w = &s.workers[idx];
            let mut q = w.queue.lock().unwrap();
            loop {
                if let Some(t) = q.pop_front() {
                    break t;
                }
                if let Some(t) = steal(&s, idx) {
                    break t;
                }
                q = w.cv.wait(q).unwrap();
            }
        };
        task();
    }
}

fn steal(s: &Arc<Scheduler>, idx: usize) -> Option<Task> {
    for (i, w) in s.workers.iter().enumerate() {
        if i == idx {
            continue;
        }
        if let Ok(mut q) = w.queue.try_lock() {
            if let Some(t) = q.pop_back() {
                return Some(t);
            }
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::{AtomicUsize, Ordering};
    use std::time::{Duration, Instant};

    #[test]
    fn o3_1_scheduler_handles_100k_tasks() {
        let sched = Scheduler::new(8);
        let done = Arc::new(AtomicUsize::new(0));
        for _ in 0..100_000 {
            let done_c = Arc::clone(&done);
            sched.spawn(Box::new(move || {
                done_c.fetch_add(1, Ordering::Relaxed);
            }));
        }

        let start = Instant::now();
        while done.load(Ordering::Relaxed) < 100_000 && start.elapsed() < Duration::from_secs(20) {
            thread::sleep(Duration::from_millis(5));
        }
        assert_eq!(done.load(Ordering::Relaxed), 100_000, "not all tasks executed in time");
    }

    #[test]
    fn o3_2_work_stealing_uses_multiple_workers() {
        let sched = Scheduler::new(6);
        let done = Arc::new(AtomicUsize::new(0));
        let mask = Arc::new(AtomicUsize::new(0));

        for _ in 0..20_000 {
            let done_c = Arc::clone(&done);
            let mask_c = Arc::clone(&mask);
            sched.spawn(Box::new(move || {
                if let Some(id) = current_worker_id() {
                    let bit = 1usize << (id % (usize::BITS as usize - 1));
                    mask_c.fetch_or(bit, Ordering::Relaxed);
                }
                done_c.fetch_add(1, Ordering::Relaxed);
            }));
        }

        let start = Instant::now();
        while done.load(Ordering::Relaxed) < 20_000 && start.elapsed() < Duration::from_secs(15) {
            thread::sleep(Duration::from_millis(5));
        }
        assert_eq!(done.load(Ordering::Relaxed), 20_000, "tasks stalled");
        assert!(
            mask.load(Ordering::Relaxed).count_ones() >= 2,
            "work was not distributed across workers"
        );
    }
}
