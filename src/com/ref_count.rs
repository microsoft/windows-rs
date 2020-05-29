use std::sync::atomic::{self, AtomicI32, Ordering};

/// A thread-safe reference count for use with COM implementations.
#[repr(transparent)]
pub struct RefCount {
    value: AtomicI32,
}

impl RefCount {
    /// Creates a new `RefCount` with an initial value of `1`.
    pub fn new() -> RefCount {
        RefCount {
            value: AtomicI32::new(1),
        }
    }

    /// Increments the reference count, returning the new value.
    pub fn add_ref(&self) -> u32 {
        (self.value.fetch_add(1, Ordering::Relaxed) + 1) as u32
    }

    /// Decrements the reference count, returning the new value.
    ///
    /// This operation inserts an `Acquire` fence when the reference count reaches zero.
    /// This prevents reordering before the object is destroyed.
    pub fn release(&self) -> u32 {
        let remaining = self.value.fetch_sub(1, Ordering::Release) - 1;

        if remaining == 0 {
            atomic::fence(Ordering::Acquire);
        } else if remaining < 0 {
            panic!("Object has been over-released.");
        }

        remaining as u32
    }
}
