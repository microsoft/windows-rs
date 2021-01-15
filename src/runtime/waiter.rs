use crate::*;

/// A simple blocking waiter used by the generated bindings and should not be used directly.
pub struct Waiter(RawPtr);
pub struct WaiterSignaler(RawPtr);

impl Waiter {
    pub fn new() -> (Waiter, WaiterSignaler) {
        unsafe {
            let handle = CreateEventW(std::ptr::null_mut(), 1, 0, std::ptr::null_mut());
            (Waiter(handle), WaiterSignaler(handle))
        }
    }
}

impl WaiterSignaler {
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

#[link(name = "kernel32")]
extern "system" {
    fn CreateEventW(security: RawPtr, manual: i32, state: i32, name: RawPtr) -> RawPtr;
    fn SetEvent(handle: RawPtr) -> i32;
    fn WaitForSingleObject(handle: RawPtr, milliseconds: u32) -> u32;
    fn CloseHandle(handle: RawPtr) -> i32;
}
