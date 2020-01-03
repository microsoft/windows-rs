#![allow(dead_code)]

use crate::*;

const REFERENCE_FLAG: u32 = 1;

#[repr(C)]
struct Header {
    flags: u32,
    len: u32,
    __0: u32,
    __1: u32,
    ptr: *const u16,
}

#[repr(C)]
struct SharedHeader {
    header: Header,
    count: RefCount,
    buffer: *mut u16,
}

#[repr(C)]
pub struct String {
    pub ptr: *mut std::ffi::c_void,
}

impl String {
    pub fn new() -> String {
        String { ptr: std::ptr::null_mut() }
    }

    pub fn is_empty(&self) -> bool {
        self.ptr.is_null()
    }

    pub fn len(&self) -> usize {
        unsafe { WindowsGetStringLen(self.ptr) as usize }
    }

    pub fn as_chars(&self) -> &[u16] {
        unsafe {
            let mut len = 0;
            let wide = WindowsGetStringRawBuffer(self.ptr, &mut len);
            if len == 0 {
                &[]
            } else {
                std::slice::from_raw_parts(wide, len as usize)
            }
        }
    }
}

impl AsAbi for String {
    type In = *const std::ffi::c_void;
    type Out = *mut *mut std::ffi::c_void;

    fn as_abi_in(&self) -> Self::In {
        self.ptr
    }

    fn as_abi_out(&mut self) -> Self::Out {
        debug_assert!(self.is_empty());
        &mut self.ptr
    }
}

impl Default for String {
    fn default() -> Self {
        String::new()
    }
}

impl Drop for String {
    fn drop(&mut self) {
        if !self.is_empty() {
            unsafe {
                WindowsDeleteString(self.ptr);
            }
        }
    }
}

impl std::fmt::Display for String {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // TODO: how to format the wchar buffer directly to avoid an allocation?
        // Especially since `value.to_string()` relies on this... unless the formatter
        // can somehow move/forward the string.
        write!(f, "{}", std::string::String::from_utf16(self.as_chars()).unwrap())
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

            let mut ptr = std::ptr::null_mut();
            WindowsPromoteStringBuffer(handle, &mut ptr).unwrap();
            String { ptr }
        }
    }
}

impl From<*mut std::ffi::c_void> for String {
    fn from(ptr: *mut std::ffi::c_void) -> String {
        Self { ptr: ptr }
    }
}
