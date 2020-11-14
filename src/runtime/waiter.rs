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
