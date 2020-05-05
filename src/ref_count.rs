use std::sync::atomic::{self, AtomicU32, Ordering};

#[repr(transparent)]
pub struct RefCount {
    value: AtomicU32,
}

impl RefCount {
    pub fn new(value: u32) -> RefCount {
        RefCount {
            value: AtomicU32::new(value),
        }
    }

    pub fn add_ref(&self) -> u32 {
        self.value.fetch_add(1, Ordering::Relaxed) + 1
    }

    pub fn release(&self) -> u32 {
        let remaining = self.value.fetch_sub(1, Ordering::Release) - 1;

        if remaining == 0 {
            atomic::fence(Ordering::Acquire);
        }

        remaining
    }
}
