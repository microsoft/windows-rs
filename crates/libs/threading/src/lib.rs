#![doc = include_str!("../readme.md")]
#![no_std]
#![expect(non_snake_case, non_camel_case_types, clippy::upper_case_acronyms)]

mod bindings;
use bindings::*;

mod pool;
pub use pool::*;

extern crate alloc;
use alloc::boxed::Box;
use core::ffi::c_void;

/// Submits a closure to the default thread pool.
///
/// The closure must be `Send + 'static`.
///
/// This uses `TrySubmitThreadpoolCallback` and reuses worker threads.
pub fn submit<F: FnOnce() + Send + 'static>(f: F) {
    // SAFETY: the closure has `'static` lifetime.
    assert!(
        unsafe { submit_with_environment(core::ptr::null(), f) },
        "allocation failed"
    );
}

/// Calls the closure on each item in parallel and waits for completion.
///
/// The closure must be `Sync`, and iterator items must be `Send`.
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
pub fn thread_id() -> u32 {
    unsafe { GetCurrentThreadId() }
}

/// Suspends the execution of the current thread until the time-out interval elapses.
pub fn sleep(milliseconds: u32) {
    unsafe {
        Sleep(milliseconds);
    }
}

// Thread-pool allocation failures are unrecoverable here.
fn check<D: Default + PartialEq>(result: D) -> D {
    assert!(result != D::default(), "allocation failed");

    result
}

// As with `check`, but for the thread pool APIs that report failure by returning a null pointer.
fn check_ptr<T>(result: *mut T) -> *mut T {
    assert!(!result.is_null(), "allocation failed");

    result
}

// This function is `unsafe` as it cannot ensure that the lifetime of the closure is sufficient or
// whether the `environment` pointer is valid.
unsafe fn submit_with_environment<F: FnOnce() + Send>(
    environment: *const TP_CALLBACK_ENVIRON_V3,
    f: F,
) -> bool {
    unsafe extern "system" fn callback<F: FnOnce() + Send>(
        _: *mut TP_CALLBACK_INSTANCE,
        callback: *mut c_void,
    ) {
        unsafe {
            Box::from_raw(callback as *mut F)();
        }
    }

    let context = Box::into_raw(Box::new(f));
    if unsafe { TrySubmitThreadpoolCallback(Some(callback::<F>), context as _, environment) } != 0 {
        true
    } else {
        // SAFETY: the thread pool rejected the callback and did not take ownership.
        drop(unsafe { Box::from_raw(context) });
        false
    }
}
