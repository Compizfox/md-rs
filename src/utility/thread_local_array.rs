use std::cell::{RefCell, RefMut};
use std::ops::Add;
use thread_local::ThreadLocal;

use crate::utility::add_arrays_into;

/// A struct encapsulating a thread-local array type.
/// Every thread has its own separate copy of the wrapped array.
pub struct ThreadLocalArray<T: Send, const N: usize> {
    inner: ThreadLocal<RefCell<[T; N]>>,
    init: T,
}

impl<T: Send + Copy, const N: usize> ThreadLocalArray<T, N> {
    /// Constructs a new `ThreadLocalArray`.
    /// * `init` - initial value for the elements to initialize the arrays with
    pub fn new(init: T) -> Self {
        Self {
            inner: ThreadLocal::new(),
            init,
        }
    }

    /// Mutually borrows the thread-local array, creating it if necessary.
    pub fn borrow_mut(&self) -> RefMut<[T; N]> {
        self.inner.get_or(|| RefCell::new([self.init; N]))
            .borrow_mut()
    }
}

impl<T: Send + Add<T, Output=T>, const N: usize> ThreadLocalArray<T, N> {
    /// Sums the thread-local arrays together element-wise, consuming the `ThreadLocalArray`.
    pub fn into_sum(self) -> [T; N] {
        self.inner.into_iter()
            .map(|x| x.into_inner())
            .reduce(|a, b| add_arrays_into(a, b))
            .unwrap()
    }
}