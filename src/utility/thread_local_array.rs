use std::cell::{RefCell, RefMut};
use std::ops::Add;

use thread_local::ThreadLocal;

/// A struct encapsulating a thread-local array type.
/// Every thread has its own separate copy of the wrapped array.
pub struct ThreadLocalVec<T: Send> {
    inner: ThreadLocal<RefCell<Vec<T>>>,
    init: T,
    n: usize,
}

impl<T: Send + Copy> ThreadLocalVec<T> {
    /// Constructs a new `ThreadLocalArray`.
    /// * `init` - initial value for the elements to initialize the arrays with
    pub fn new(init: T, n: usize) -> Self {
        Self {
            inner: ThreadLocal::new(),
            init,
            n,
        }
    }

    /// Mutually borrows the thread-local array, creating it if necessary.
    pub fn borrow_mut(&self) -> RefMut<Vec<T>> {
        self.inner.get_or(|| RefCell::new(vec![self.init; self.n]))
            .borrow_mut()
    }
}

impl<T: Send + Add<T, Output=T>> ThreadLocalVec<T> {
    /// Sums the thread-local arrays together element-wise, consuming the `ThreadLocalArray`.
    pub fn into_sum(self) -> Vec<T> {
        self.inner.into_iter()
            .map(|x| x.into_inner())
            .reduce(|a, b| add_vecs_into(a, b))
            .unwrap()
    }
}

pub fn add_vecs_into<T: Add<T, Output=T>>(a: Vec<T>, b: Vec<T>) -> Vec<T> {
    a.
        into_iter()
        .zip(b)
        .map(|(a, b)| a + b)
        .collect()
}