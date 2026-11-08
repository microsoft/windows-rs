use core::sync::atomic::{AtomicI32, Ordering, fence};

#[repr(transparent)]
#[derive(Default)]
pub struct RefCount(pub(crate) AtomicI32);

impl RefCount {
    pub(crate) const MAX: u32 = i32::MAX as u32;

    /// Creates a new `RefCount` with the given initial value.
    pub const fn new(count: u32) -> Self {
        assert!(count <= Self::MAX);
        Self(AtomicI32::new(count as i32))
    }

    /// Increments the reference count, returning the new value.
    pub fn add_ref(&self) -> u32 {
        let previous = self.0.fetch_add(1, Ordering::Relaxed);
        if previous < 0 || previous == Self::MAX as i32 {
            Self::ref_count_overflow();
        }
        (previous + 1) as u32
    }

    pub(crate) fn try_add_ref(&self) -> bool {
        self.0
            .try_update(Ordering::Acquire, Ordering::Relaxed, |count| {
                if count == 0 {
                    return None;
                }
                if count < 0 || count == Self::MAX as i32 {
                    Self::ref_count_overflow();
                }
                Some(count + 1)
            })
            .is_ok()
    }

    // The non-unwinding ABI makes overflow fatal even when the caller catches panics. In particular,
    // `add_ref` must not return after its atomic increment has crossed the supported range.
    #[cold]
    pub(crate) extern "C" fn ref_count_overflow() -> ! {
        panic!("COM reference count overflow.");
    }

    /// Decrements the reference count, returning the new value.
    ///
    /// This operation inserts an `Acquire` fence when the reference count reaches zero.
    /// This prevents reordering before the object is destroyed.
    pub fn release(&self) -> u32 {
        let remaining = self.0.fetch_sub(1, Ordering::Release) - 1;

        match remaining.cmp(&0) {
            core::cmp::Ordering::Equal => fence(Ordering::Acquire),
            core::cmp::Ordering::Less => panic!("Object has been over-released."),
            core::cmp::Ordering::Greater => {}
        }

        remaining as u32
    }
}
