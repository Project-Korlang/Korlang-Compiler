use std::cell::RefCell;

pub struct Arena<T> {
    chunks: RefCell<Vec<Vec<T>>>,
}

impl<T> Arena<T> {
    pub fn new() -> Self {
        Self {
            chunks: RefCell::new(vec![Vec::with_capacity(1024)]),
        }
    }

    pub fn alloc(&self, value: T) -> &T {
        let mut chunks = self.chunks.borrow_mut();
        if chunks.last().unwrap().len() == chunks.last().unwrap().capacity() {
            chunks.push(Vec::with_capacity(1024));
        }
        let chunk = chunks.last_mut().unwrap();
        chunk.push(value);
        let ptr = chunk.last().unwrap() as *const T;
        unsafe { &*ptr }
    }
}
