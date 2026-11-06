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
use core::time::Duration;

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

/// The order in which the thread pool runs queued callbacks.
///
/// Callbacks with a higher priority start before queued callbacks with a lower priority. Priority
/// does not change the priority of the thread that runs the callback.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub enum Priority {
    /// Runs before normal and low priority callbacks, such as for work the user is waiting on.
    High,
    /// The priority used by `submit`.
    #[default]
    Normal,
    /// Runs after high and normal priority callbacks, such as for background indexing.
    Low,
}

impl Priority {
    fn to_raw(self) -> TP_CALLBACK_PRIORITY {
        match self {
            Self::High => TP_CALLBACK_PRIORITY_HIGH,
            Self::Normal => TP_CALLBACK_PRIORITY_NORMAL,
            Self::Low => TP_CALLBACK_PRIORITY_LOW,
        }
    }
}

/// Submits a closure to the default thread pool with the given priority.
///
/// The closure must be `Send + 'static`.
pub fn submit_with_priority<F: FnOnce() + Send + 'static>(priority: Priority, f: F) {
    let environment = environment(core::ptr::null_mut(), core::ptr::null_mut(), priority);

    // SAFETY: the closure has `'static` lifetime, and the thread pool copies the environment.
    assert!(
        unsafe { submit_with_environment(&environment, f) },
        "allocation failed"
    );
}

/// Submits a closure to the default thread pool to run once `delay` has elapsed.
///
/// The closure must be `Send + 'static`. The delay is a minimum: the closure may run later if
/// the system is busy, and timers are only as precise as the system timer resolution, which is
/// often about 15 milliseconds.
pub fn submit_after<F: FnOnce() + Send + 'static>(delay: Duration, f: F) {
    unsafe extern "system" fn callback<F: FnOnce() + Send>(
        _: PTP_CALLBACK_INSTANCE,
        callback: *mut c_void,
        timer: PTP_TIMER,
    ) {
        unsafe {
            Box::from_raw(callback as *mut F)();
            CloseThreadpoolTimer(timer);
        }
    }

    let context = Box::into_raw(Box::new(f));
    let timer =
        unsafe { CreateThreadpoolTimer(Some(callback::<F>), context as _, core::ptr::null_mut()) };

    if timer.is_null() {
        // SAFETY: the thread pool did not take ownership of the closure.
        drop(unsafe { Box::from_raw(context) });
        panic!("allocation failed");
    }

    // A negative due time is relative to now, in 100-nanosecond units.
    let ticks = delay.as_nanos().div_ceil(100).min(i64::MAX as u128) as i64;
    let due = (-ticks) as u64;
    let mut due = FILETIME {
        dwLowDateTime: due as u32,
        dwHighDateTime: (due >> 32) as u32,
    };

    unsafe { SetThreadpoolTimer(timer, &mut due, 0, 0) };
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

fn environment(
    pool: PTP_POOL,
    cleanup_group: PTP_CLEANUP_GROUP,
    priority: Priority,
) -> TP_CALLBACK_ENVIRON_V3 {
    TP_CALLBACK_ENVIRON_V3 {
        Version: 3,
        Pool: pool,
        CleanupGroup: cleanup_group,
        CallbackPriority: priority.to_raw(),
        Size: size_of::<TP_CALLBACK_ENVIRON_V3>() as u32,
        ..Default::default()
    }
}

// This function is `unsafe` as it cannot ensure that the lifetime of the closure is sufficient or
// whether the `environment` pointer is valid.
unsafe fn submit_with_environment<F: FnOnce() + Send>(
    environment: *const TP_CALLBACK_ENVIRON_V3,
    f: F,
) -> bool {
    unsafe extern "system" fn callback<F: FnOnce() + Send>(
        _: PTP_CALLBACK_INSTANCE,
        callback: *mut c_void,
    ) {
        unsafe {
            Box::from_raw(callback as *mut F)();
        }
    }

    let context = Box::into_raw(Box::new(f));
    if unsafe {
        TrySubmitThreadpoolCallback(Some(callback::<F>), context as _, environment.cast_mut())
    } != 0
    {
        true
    } else {
        // SAFETY: the thread pool rejected the callback and did not take ownership.
        drop(unsafe { Box::from_raw(context) });
        false
    }
}
