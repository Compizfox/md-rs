use std::cell::{RefCell, RefMut};
use std::ops::Add;

use thread_local::ThreadLocal;

use crate::utility::add_vecs_into;

/// A struct encapsulating a thread-local vector type.
/// Every thread has its own separate copy of the wrapped vector.
pub struct ThreadLocalVec<T: Send> {
    inner: ThreadLocal<RefCell<Vec<T>>>,
    init: T,
    n: usize,
}

impl<T: Send + Copy> ThreadLocalVec<T> {
    /// Constructs a new `ThreadLocalVec`.
    /// * `init` - initial value for the elements to initialize the vectors with
    pub fn new(init: T, n: usize) -> Self {
        Self {
            inner: ThreadLocal::new(),
            init,
            n,
        }
    }

    /// Mutually borrows the thread-local vector, creating it if necessary.
    pub fn borrow_mut(&self) -> RefMut<Vec<T>> {
        self.inner.get_or(|| RefCell::new(vec![self.init; self.n]))
            .borrow_mut()
    }
}

impl<T: Send + Add<T, Output=T>> ThreadLocalVec<T> {
    /// Sums the thread-local vectors together element-wise, consuming the `ThreadLocalVec`.
    pub fn into_sum(self) -> Vec<T> {
        self.inner.into_iter()
            .map(|x| x.into_inner())
            .reduce(|a, b| add_vecs_into(a, b))
            .unwrap()
    }
}
