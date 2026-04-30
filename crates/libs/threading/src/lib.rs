#![doc = include_str!("../readme.md")]
#![no_std]
#![cfg_attr(
    windows,
    expect(non_snake_case, non_camel_case_types, clippy::upper_case_acronyms)
)]

#[cfg(windows)]
mod bindings;
#[cfg(windows)]
use bindings::*;

#[cfg(windows)]
mod pool;
#[cfg(windows)]
pub use pool::*;

extern crate alloc;
#[cfg(windows)]
use alloc::boxed::Box;
#[cfg(windows)]
use core::ffi::c_void;

#[cfg(all(not(windows), feature = "std"))]
extern crate std;

/// Submit the closure to the default thread pool.
///
/// * The closure must have `'static` lifetime as the thread may outlive the lifetime in which `submit` is called.
/// * The closure must be `Send` as it will be sent to another thread for execution.
///
/// On Windows this dispatches through the Win32 thread pool
/// (`TrySubmitThreadpoolCallback`), which reuses worker threads and is
/// dramatically cheaper than spawning a new OS thread per call. On non-Windows
/// targets it falls back to `std::thread::spawn` when the `std` feature is
/// enabled (the default); without `std` on a non-Windows target it panics, as
/// no portable thread primitive is available.
#[cfg(windows)]
pub fn submit<F: FnOnce() + Send + 'static>(f: F) {
    // This is safe because the closure has `'static` lifetime.
    unsafe {
        try_submit(core::ptr::null(), f);
    }
}

/// Submit the closure to the default thread pool.
///
/// * The closure must have `'static` lifetime as the thread may outlive the lifetime in which `submit` is called.
/// * The closure must be `Send` as it will be sent to another thread for execution.
#[cfg(all(not(windows), feature = "std"))]
pub fn submit<F: FnOnce() + Send + 'static>(f: F) {
    // The handle is dropped immediately; matches the fire-and-forget
    // semantics of the Windows path.
    let _ = std::thread::spawn(f);
}

/// Submit the closure to the default thread pool.
///
/// * The closure must have `'static` lifetime as the thread may outlive the lifetime in which `submit` is called.
/// * The closure must be `Send` as it will be sent to another thread for execution.
#[cfg(all(not(windows), not(feature = "std")))]
pub fn submit<F: FnOnce() + Send + 'static>(_f: F) {
    unimplemented!("windows_threading::submit requires the `std` feature on non-Windows targets");
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
