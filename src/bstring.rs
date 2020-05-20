use crate::runtime::*;
use crate::*;

#[repr(transparent)]
pub struct BString {
    ptr: RawPtr,
}

impl BString {
    pub fn new() -> BString {
        Self {
            ptr: std::ptr::null_mut(),
        }
    }

    pub fn set_abi(&mut self) -> *mut RawPtr {
        if !self.ptr.is_null() {
            unsafe {
                SysFreeString(self.ptr);
            }
            self.ptr = std::ptr::null_mut();
        }
        &mut self.ptr
    }
}

impl Drop for BString {
    fn drop(&mut self) {
        unsafe {
            SysFreeString(self.ptr);
        }
    }
}

impl From<BString> for String {
    fn from(from: BString) -> Self {
        if from.ptr.is_null() {
            return String::new();
        }

        unsafe {
            String::from_utf16(std::slice::from_raw_parts(
                from.ptr as *const u16,
                SysStringLen(from.ptr) as usize,
            ))
            .unwrap()
        }
    }
}
