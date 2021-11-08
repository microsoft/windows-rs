use super::*;

use bindings::{
    Windows::Win32::Foundation::{CloseHandle, HANDLE},
    Windows::Win32::System::Threading::{CreateEventA, SetEvent, WaitForSingleObject},
};

#[doc(hidden)]
pub struct Waiter(HANDLE);
pub struct WaiterSignaler(HANDLE);

impl Waiter {
    pub fn new() -> (Waiter, WaiterSignaler) {
        unsafe {
            let handle = CreateEventA(core::ptr::null_mut(), true, false, None);
            (Waiter(handle), WaiterSignaler(handle))
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
        SetEvent(self.0);
    }
}

impl Drop for Waiter {
    fn drop(&mut self) {
        unsafe {
            WaitForSingleObject(self.0, 0xFFFFFFFF);
            CloseHandle(self.0);
        }
    }
}
