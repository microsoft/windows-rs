use crate::*;
use std::sync::atomic::{fence, AtomicIsize, AtomicU32, Ordering};

/// A thread-safe reference count for use with COM weak reference implementations.
#[repr(transparent)]
#[derive(Default)]
pub struct WeakRefCount(AtomicIsize);

impl WeakRefCount {
    pub fn new() -> Self {
        Self(AtomicIsize::new(1))
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

struct TearOff {
    object: RawPtr,
    source_vtable: *const IWeakReferenceSource_vtable,
    weak_vtable: *const IWeakReference_vtable,
    strong_count: AtomicU32,
    weak_count: AtomicU32,
}

// impl TearOff {
//     fn new(object: RawPtr, strong_count: u32) -> IWeakReferenceSource {
//         let com = TearOff {
//             object,
//             source_vtable: &Self::SOURCE_VTABLE,
//             weak_vtable: &Self::WEAK_VTABLE,
//             strong_count,
//             weak_count: 1,
//         };
//         unsafe {
//             std::mem::transmute(::std::boxed::Box::new(com))
//         }
//     }
// }
