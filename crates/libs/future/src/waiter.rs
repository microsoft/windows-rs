use super::*;
use alloc::sync::Arc;

// Owns the Win32 event handle and closes it on drop. Shared between `Waiter` and `WaiterSignaler`
// via `Arc` so that the handle is guaranteed to remain valid as long as either side is alive.
struct EventHandle(HANDLE);

// SAFETY: A Win32 event handle can safely be used from any thread.
unsafe impl Send for EventHandle {}
unsafe impl Sync for EventHandle {}

impl Drop for EventHandle {
    fn drop(&mut self) {
        unsafe {
            CloseHandle(self.0);
        }
    }
}

pub struct Waiter(Arc<EventHandle>);
pub struct WaiterSignaler(Arc<EventHandle>);

impl Waiter {
    pub fn new() -> crate::Result<(Self, WaiterSignaler)> {
        let handle = unsafe { CreateEventW(core::ptr::null(), 1, 0, core::ptr::null()) };
        if handle.is_null() {
            Err(crate::Error::from_thread())
        } else {
            let handle = Arc::new(EventHandle(handle));
            Ok((Self(handle.clone()), WaiterSignaler(handle)))
        }
    }

    // Waits for the `WaiterSignaler` to signal. The event handle is closed when the last `Arc`
    // reference (either `Waiter` or `WaiterSignaler`) is dropped.
    pub fn wait(self) {
        unsafe {
            WaitForSingleObject(self.0 .0, 0xFFFFFFFF);
        }
    }
}

impl WaiterSignaler {
    // Signals the `Waiter`. The underlying event handle is kept alive by the `Arc` so this is
    // always safe to call regardless of whether the `Waiter` has been dropped.
    pub fn signal(&self) {
        // https://github.com/microsoft/windows-rs/pull/374#discussion_r535313344
        unsafe {
            SetEvent(self.0 .0);
        }
    }
}
