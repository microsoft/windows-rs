//! Shared helpers for the `test_clang` integration tests.

use std::sync::{Mutex, MutexGuard};

/// Serializes libclang scrapes across the test harness's parallel threads.
///
/// libclang keeps process-global state (the LLVM target registry, `cl::opt` command-line
/// option table, and other `ManagedStatic` singletons) that is not safe to initialize or
/// tear down from several threads at once. The generated `clang.rs` suite runs ~80 fixtures
/// as parallel test threads, each loading libclang and parsing a translation unit - far more
/// concurrency than the three-arch parses the real tools run - and that intermittently faults
/// with `STATUS_ACCESS_VIOLATION`. Holding this lock around each scrape serializes the
/// libclang-touching section; the tests are fast enough that the lost parallelism does not
/// matter, and production behavior and generated output are unaffected.
///
/// A poisoned lock is recovered so a panicking fixture reports its own assertion failure
/// instead of poisoning every later test.
pub fn libclang_guard() -> MutexGuard<'static, ()> {
    static LOCK: Mutex<()> = Mutex::new(());
    LOCK.lock().unwrap_or_else(|poisoned| poisoned.into_inner())
}
