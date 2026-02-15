use crate::scheduler::Scheduler;
use crate::coroutine::Coroutine;
use std::sync::Arc;
use std::future::Future;

pub struct AsyncRuntime {
    scheduler: Arc<Scheduler>,
}

impl AsyncRuntime {
    pub fn new(threads: usize) -> Self {
        Self {
            scheduler: Scheduler::new(threads),
        }
    }

    pub fn spawn<F, T, E>(&self, future: F) 
    where 
        F: Future<Output = Result<T, E>> + Send + 'static,
        T: Send + 'static,
        E: Send + 'static + std::fmt::Debug,
    {
        use std::task::{Waker, RawWaker, RawWakerVTable};

        unsafe fn dummy_waker_clone(_: *const ()) -> RawWaker { dummy_raw_waker() }
        unsafe fn dummy_waker_wake(_: *const ()) {}
        unsafe fn dummy_waker_wake_by_ref(_: *const ()) {}
        unsafe fn dummy_waker_drop(_: *const ()) {}

        static DUMMY_VTABLE: RawWakerVTable = RawWakerVTable::new(
            dummy_waker_clone,
            dummy_waker_wake,
            dummy_waker_wake_by_ref,
            dummy_waker_drop,
        );

        fn dummy_raw_waker() -> RawWaker {
            RawWaker::new(std::ptr::null(), &DUMMY_VTABLE)
        }

        let mut coro = Coroutine::new(future);
        self.scheduler.spawn(Box::new(move || {
            let start = std::time::Instant::now();
            let waker = unsafe { Waker::from_raw(dummy_raw_waker()) };
            let mut cx = std::task::Context::from_waker(&waker);
            match coro.poll(&mut cx) {
                crate::coroutine::CoroutineResult::Finished(res) => {
                    let duration = start.elapsed();
                    if let Err(e) = res {
                        eprintln!("[ASYNCRUNTIME-DIAGNOSTIC] Task finished with error in {}us: {:?}", duration.as_micros(), e);
                    } else {
                        println!("[ASYNCRUNTIME-DIAGNOSTIC] Task completed successfully in {}us", duration.as_micros());
                    }
                }
                crate::coroutine::CoroutineResult::Yielded => {
                    // Simplification: In a real system, we'd wait for notification
                }
            }
        }));
    }
}

pub mod diag {
    pub struct TaskInfo {
        pub id: usize,
        pub state: String,
        pub duration_us: u64,
    }

    pub fn report_diagnostic(info: TaskInfo) {
        println!("[ASYNC-DIAG] Task {}: {} (took {}us)", info.id, info.state, info.duration_us);
    }
}
