// Implementation of a weak reference count and tearoff

// Somehow try to bake everything in here so we can keep the implement macro simple.

use std::sync::atomic::{fence, AtomicI32, Ordering};

/// A thread-safe reference count for use with COM weak reference implementations.
#[repr(transparent)]
#[derive(Default)]
pub struct WeakRefCount(AtomicI32);

impl WeakRefCount {
    pub fn new() -> Self {
        Self(AtomicI32::new(1))
    }

    pub fn add_ref(&self) -> u32 {
        (self.0.fetch_add(1, Ordering::Relaxed) + 1) as u32
    }

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
