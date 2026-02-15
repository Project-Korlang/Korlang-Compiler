use std::pin::Pin;
use std::task::{Context, Poll};
use std::future::Future;

pub enum CoroutineResult<T, E> {
    Finished(Result<T, E>),
    Yielded,
}

pub struct Coroutine<T, E> {
    future: Pin<Box<dyn Future<Output = Result<T, E>> + Send>>,
}

impl<T, E> Coroutine<T, E> {
    pub fn new<F>(f: F) -> Self 
    where 
        F: Future<Output = Result<T, E>> + Send + 'static
    {
        Self {
            future: Box::pin(f),
        }
    }

    pub fn poll(&mut self, cx: &mut Context<'_>) -> CoroutineResult<T, E> {
        match self.future.as_mut().poll(cx) {
            Poll::Ready(res) => CoroutineResult::Finished(res),
            Poll::Pending => CoroutineResult::Yielded,
        }
    }
}

pub mod error {
    #[derive(Debug)]
    pub enum CoroutineError {
        Panic(String),
        Timeout,
        Cancelled,
        Custom(String),
    }

    impl std::fmt::Display for CoroutineError {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self)
        }
    }

    impl std::error::Error for CoroutineError {}
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::future::ready;
    use std::task::{Context, Wake};
    use std::sync::Arc;

    struct NoopWaker;
    impl Wake for NoopWaker {
        fn wake(self: Arc<Self>) {}
    }

    #[test]
    fn test_coroutine_basic() {
        let mut coro = Coroutine::new(ready(Ok::<i32, String>(42)));
        let waker = Arc::new(NoopWaker).into();
        let mut cx = Context::from_waker(&waker);

        match coro.poll(&mut cx) {
            CoroutineResult::Finished(Ok(v)) => assert_eq!(v, 42),
            _ => panic!("Expected Finished(Ok(42))"),
        }
    }
}
