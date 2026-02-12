use std::collections::VecDeque;
use std::sync::{Arc, Condvar, Mutex};
use std::thread;

pub type Task = Box<dyn FnOnce() + Send + 'static>;

struct Worker {
    queue: Mutex<VecDeque<Task>>,
    cv: Condvar,
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

