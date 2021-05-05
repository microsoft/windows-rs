use std::sync::atomic::{fence, AtomicI32, Ordering};

/// A thread-safe reference count for use with COM/HSTRING implementations.
#[repr(transparent)]
#[derive(Default, Debug)]
pub struct RefCount(pub(crate) AtomicI32);

impl RefCount {
    /// Creates a new `RefCount` with an initial value of `count`.
    pub fn new(count: u32) -> Self {
        Self(AtomicI32::new(count as _))
    }

    /// Increments the reference count, returning the new value.
    pub fn add_ref(&self) -> u32 {
        (self.0.fetch_add(1, Ordering::Relaxed) + 1) as u32
    }

    /// Decrements the reference count, returning the new value.
    ///
    /// This operation inserts an `Acquire` fence when the reference count reaches zero.
    /// This prevents reordering before the object is destroyed.
    pub fn release(&self) -> u32 {
        let remaining = self.0.fetch_sub(1, Ordering::Release) - 1;

        if remaining == 0 {
            fence(Ordering::Acquire);
        } else if remaining < 0 {
            panic!("Object has been over-released.");
        }

        remaining as u32
    }
}
