//! Live heap object counts, compiled in only when building with
//! `RUSTFLAGS=--cfg windows_cast_diagnostics`.
//!
//! Counts objects allocated by `#[implement]`, WinRT delegates, and weak reference tear-offs that
//! have not yet been freed. A count that stays above zero once an app has released everything it
//! created points to a reference-count leak. Static COM objects are never freed and not counted.

use core::sync::atomic::{AtomicUsize, Ordering};

/// The number of heap objects of each kind currently alive in this process.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct LiveObjects {
    /// `#[implement]` objects.
    pub objects: usize,
    /// WinRT delegates.
    pub delegates: usize,
    /// Weak reference tear-offs.
    pub weak_references: usize,
}

/// Returns the number of heap objects currently alive in this process.
pub fn live_objects() -> LiveObjects {
    LiveObjects {
        objects: OBJECTS.get(),
        delegates: DELEGATES.get(),
        weak_references: WEAK_REFERENCES.get(),
    }
}

pub(crate) struct Counter(AtomicUsize);

impl Counter {
    const fn new() -> Self {
        Self(AtomicUsize::new(0))
    }

    pub(crate) fn add(&self) {
        self.0.fetch_add(1, Ordering::Relaxed);
    }

    pub(crate) fn remove(&self) {
        self.0.fetch_sub(1, Ordering::Relaxed);
    }

    fn get(&self) -> usize {
        self.0.load(Ordering::Relaxed)
    }
}

pub(crate) static OBJECTS: Counter = Counter::new();
pub(crate) static DELEGATES: Counter = Counter::new();
pub(crate) static WEAK_REFERENCES: Counter = Counter::new();
