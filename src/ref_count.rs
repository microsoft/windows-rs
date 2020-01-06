use std::sync::atomic::*;

#[repr(C)]
pub struct RefCount {
    value: AtomicU32,
}

impl RefCount {
    pub fn new(value: u32) -> RefCount {
        RefCount { value: AtomicU32::new(value) }
    }

    pub fn addref(&self) -> u32 {
        self.value.fetch_add(1, Ordering::Relaxed) + 1
    }

    pub fn release(&self) -> u32 {
        let remaining = self.value.fetch_sub(1, Ordering::Release) - 1;

        if remaining == 0 {
            fence(Ordering::Acquire);
        }

        remaining
    }
}
