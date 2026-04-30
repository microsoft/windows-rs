#![doc = include_str!("../readme.md")]
#![cfg_attr(all(not(feature = "std")), no_std)]
#![expect(non_snake_case, non_camel_case_types, clippy::upper_case_acronyms)]

extern crate alloc;

#[cfg(windows)]
mod bindings;
#[cfg(windows)]
use bindings::*;

#[cfg(windows)]
mod pool;
#[cfg(windows)]
pub use pool::*;

#[cfg(windows)]
use alloc::boxed::Box;
#[cfg(windows)]
use core::ffi::c_void;

/// Submit the closure to the default thread pool.
///
/// * The closure must have `'static` lifetime as the thread may outlive the lifetime in which `submit` is called.
/// * The closure must be `Send` as it will be sent to another thread for execution.
///
/// On Windows, this submits the closure to the system-wide thread pool via
/// `TrySubmitThreadpoolCallback`. On other targets the closure runs on a freshly spawned
/// `std::thread` when the `std` feature is enabled (the default); without the `std` feature
/// on a non-Windows target this function panics, since there is no portable thread-pool
/// equivalent without `std`.
#[cfg(windows)]
pub fn submit<F: FnOnce() + Send + 'static>(f: F) {
    // This is safe because the closure has `'static` lifetime.
    unsafe {
        try_submit(core::ptr::null(), f);
    }
}

/// Submit the closure to the default thread pool.
///
/// See the Windows-targeted variant for the contract. On non-Windows targets with `std`
/// enabled, the closure runs on a freshly spawned `std::thread` — there is no shared pool.
#[cfg(all(not(windows), feature = "std"))]
pub fn submit<F: FnOnce() + Send + 'static>(f: F) {
    std::thread::spawn(f);
}

/// Submit the closure to the default thread pool.
///
/// On non-Windows targets without the `std` feature there is no thread-pool implementation
/// available, so calling this function panics. The function is still exposed at compile time
/// so that callers can be written portably and gated at runtime if needed.
#[cfg(all(not(windows), not(feature = "std")))]
pub fn submit<F: FnOnce() + Send + 'static>(_: F) {
    unimplemented!("windows_threading::submit is not implemented on non-Windows targets without the `std` feature");
}

/// Calls the closure on each element of the iterator in parallel, waiting for all closures to finish.
///
/// * The closure does not require `'static` lifetime since the `for_each` function bounds the lifetime of all submitted closures.
/// * The closure must be `Sync` as multiple threads will refer to it.
/// * The iterator items must be `Send` as they will be sent from one thread to another.
#[cfg(windows)]
pub fn for_each<I, F, T>(i: I, f: F)
where
    I: Iterator<Item = T>,
    F: Fn(T) + Sync,
    T: Send,
{
    Pool::with_scope(|pool| {
        for item in i {
            pool.submit(|| f(item));
        }
    });
}

/// The thread identifier of the calling thread.
#[cfg(windows)]
pub fn thread_id() -> u32 {
    unsafe { GetCurrentThreadId() }
}

/// Suspends the execution of the current thread until the time-out interval elapses.
#[cfg(windows)]
pub fn sleep(milliseconds: u32) {
    unsafe {
        Sleep(milliseconds);
    }
}

// When used correctly, the Windows thread pool APIs only fail when memory is exhausted. This function will cause such failures to `panic`.
#[cfg(windows)]
fn check<D: Default + PartialEq>(result: D) -> D {
    if result == D::default() {
        panic!("allocation failed");
    }

    result
}

// This function is `unsafe` as it cannot ensure that the lifetime of the closure is sufficient or
// whether the `environment` pointer is valid.
#[cfg(windows)]
unsafe fn try_submit<F: FnOnce() + Send>(environment: *const TP_CALLBACK_ENVIRON_V3, f: F) {
    unsafe extern "system" fn callback<F: FnOnce() + Send>(
        _: PTP_CALLBACK_INSTANCE,
        callback: *mut c_void,
    ) {
        unsafe {
            Box::from_raw(callback as *mut F)();
        }
    }

    unsafe {
        check(TrySubmitThreadpoolCallback(
            Some(callback::<F>),
            Box::into_raw(Box::new(f)) as _,
            environment,
        ));
    }
}
