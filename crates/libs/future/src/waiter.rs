#[cfg(windows)]
mod imp {
    use super::super::*;

    pub struct Waiter(HANDLE);
    pub struct WaiterSignaler(HANDLE);
    unsafe impl Send for WaiterSignaler {}

    impl Waiter {
        pub fn new() -> crate::Result<(Self, WaiterSignaler)> {
            unsafe {
                let handle = CreateEventW(core::ptr::null(), 1, 0, core::ptr::null());
                if handle.is_null() {
                    Err(crate::Error::from_thread())
                } else {
                    Ok((Self(handle), WaiterSignaler(handle)))
                }
            }
        }

        // Waits for the `WaiterSignaler` to signal and then closes the handle.
        pub fn wait(self) {
            unsafe {
                WaitForSingleObject(self.0, 0xFFFFFFFF);
            }
        }
    }

    impl WaiterSignaler {
        /// # Safety
        /// Signals the `Waiter`. This is unsafe because the lifetime of `WaiterSignaler` is not tied
        /// to the lifetime of the `Waiter`. This is not possible in this case because the `Waiter`
        /// is used to signal a WinRT async completion and the compiler doesn't know that the lifetime
        /// of the delegate is bounded by the calling function.
        pub unsafe fn signal(&self) {
            // https://github.com/microsoft/windows-rs/pull/374#discussion_r535313344
            unsafe {
                SetEvent(self.0);
            }
        }
    }

    impl Drop for Waiter {
        fn drop(&mut self) {
            unsafe {
                CloseHandle(self.0);
            }
        }
    }
}

// Non-Windows fallback using std primitives. Requires the `std` feature, which is the only
// configuration in which `join` is reachable on non-Windows targets (the `spawn`/`ready`
// implementations themselves are gated on `std`).
#[cfg(all(not(windows), feature = "std"))]
mod imp {
    pub struct Waiter(std::sync::Arc<(std::sync::Mutex<bool>, std::sync::Condvar)>);

    pub struct WaiterSignaler(std::sync::Arc<(std::sync::Mutex<bool>, std::sync::Condvar)>);

    impl Waiter {
        pub fn new() -> crate::Result<(Self, WaiterSignaler)> {
            let state =
                std::sync::Arc::new((std::sync::Mutex::new(false), std::sync::Condvar::new()));
            Ok((Self(state.clone()), WaiterSignaler(state)))
        }

        pub fn wait(self) {
            let (lock, cvar) = &*self.0;
            let mut signaled = lock.lock().unwrap();
            while !*signaled {
                signaled = cvar.wait(signaled).unwrap();
            }
        }
    }

    impl WaiterSignaler {
        /// # Safety
        /// Matches the Windows signature. The non-Windows implementation is itself safe because the
        /// state is reference counted, but the function is kept `unsafe` for API parity.
        pub unsafe fn signal(&self) {
            let (lock, cvar) = &*self.0;
            let mut signaled = lock.lock().unwrap();
            *signaled = true;
            cvar.notify_all();
        }
    }
}

#[cfg(any(windows, feature = "std"))]
pub use imp::*;
