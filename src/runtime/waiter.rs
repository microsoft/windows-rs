use crate::*;

pub struct Waiter(RawPtr);

impl Waiter {
    pub fn new() -> Self {
        unsafe {
            Self(CreateEventW(
                std::ptr::null_mut(),
                1,
                0,
                std::ptr::null_mut(),
            ))
        }
    }

    pub fn signal(&self) {
        unsafe {
            SetEvent(self.0);
        }
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
