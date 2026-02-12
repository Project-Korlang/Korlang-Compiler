use std::sync::{mpsc, Arc, Mutex};
use std::thread;

pub type Job = Box<dyn FnOnce() + Send + 'static>;

pub struct BlockingPool {
    tx: mpsc::Sender<Job>,
}

impl BlockingPool {
    pub fn new(threads: usize) -> Arc<Self> {
        let (tx, rx) = mpsc::channel::<Job>();
        let rx = Arc::new(Mutex::new(rx));
        for _ in 0..threads.max(1) {
            let r = Arc::clone(&rx);
            thread::spawn(move || loop {
                let job = r.lock().unwrap().recv().unwrap();
                job();
            });
        }
        Arc::new(Self { tx })
    }

    pub fn spawn_blocking(&self, job: Job) {
        let _ = self.tx.send(job);
    }
}

