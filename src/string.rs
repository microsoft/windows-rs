#![allow(dead_code)]

use crate::*;

#[repr(C)]
pub struct String {
    pub hstring: *const std::ffi::c_void,
}

impl String {
    pub fn new() -> String {
        String {
            hstring: std::ptr::null(),
        }
    }

    pub fn is_empty(&self) -> bool {
        self.hstring.is_null()
    }

    pub fn len(&self) -> usize {
        unsafe { WindowsGetStringLen(self.hstring) as usize }
    }

    pub fn as_chars(&self) -> &[u16] {
        unsafe {
            let mut len = 0;
            let wide = WindowsGetStringRawBuffer(self.hstring, &mut len);
            if len == 0 {
                &[]
            } else {
                std::slice::from_raw_parts(wide, len as usize)
            }
        }
    }

    pub fn as_raw_handle(&self) -> *const std::ffi::c_void {
        self.hstring
    }
}

impl Drop for String {
    fn drop(&mut self) {
        if !self.is_empty() {
            unsafe {
                WindowsDeleteString(self.hstring);
            }
        }
    }
}

impl std::fmt::Display for String {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // TODO: how to format the wchar buffer directly to avoid an allocation?
        // Especially since `value.to_string()` relies on this... unless the formatter
        // can somehow move/forward the string.
        write!(
            f,
            "{}",
            std::string::String::from_utf16(self.as_chars()).unwrap()
        )
    }
}

impl From<&str> for String {
    fn from(value: &str) -> String {
        unsafe {
            let len = value.encode_utf16().count() as u32;
            let mut buffer: *mut u16 = std::ptr::null_mut();
            let mut handle = std::ptr::null_mut();
            WindowsPreallocateStringBuffer(len, &mut buffer, &mut handle).unwrap();

            for (index, wide) in value.encode_utf16().enumerate() {
                *buffer.offset(index as isize) = wide;
            }

            let mut hstring = std::ptr::null_mut();
            WindowsPromoteStringBuffer(handle, &mut hstring).unwrap();
            String { hstring }
        }
    }
}
