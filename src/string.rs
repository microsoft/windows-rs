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
    buffer: u16,
}

// TODO: inline these functions (duplicate & with_len) when done
fn duplicate(handle: *const Header) -> *const Header {
    unsafe {
        if handle.is_null() {
            std::ptr::null()
        } else if (*handle).flags & REFERENCE_FLAG == 0 {
            (*(handle as *const SharedHeader)).count.addref();
            handle
        } else {
            let result = with_len((*handle).len);
            std::ptr::copy_nonoverlapping((*handle).ptr, (*result).ptr as *mut u16, (*handle).len as usize + 1);
            result
        }
    }
}

fn with_len(len: u32) -> *mut Header {
    unsafe {
        debug_assert!(len != 0);

        let shared = HeapAlloc(GetProcessHeap(), 0, std::mem::size_of::<SharedHeader>() + 2 * len as usize) as *mut SharedHeader;

        if shared.is_null() {
            panic!();
        }

        (*shared).header.flags = 0;
        (*shared).header.len = len;
        (*shared).header.ptr = &(*shared).buffer;
        (*shared).count = RefCount::new(1);
        shared as *mut Header
    }
}

#[repr(C)]
pub struct String {
    handle: *const Header,
}

impl String {
    pub fn new() -> String {
        String { handle: std::ptr::null_mut() }
    }

    pub fn is_empty(&self) -> bool {
        self.handle.is_null()
    }

    pub fn len(&self) -> usize {
        unsafe {
            if self.handle.is_null() {
                0
            } else {
                (*self.handle).len as usize
            }
        }
    }

    pub fn as_chars(&self) -> &[u16] {
        unsafe {
            if self.handle.is_null() {
                &[]
            } else {
                std::slice::from_raw_parts((*self.handle).ptr, (*self.handle).len as usize)
            }
        }
    }
}

impl AsAbi for String {
    type In = *const std::ffi::c_void;
    type Out = *mut *mut std::ffi::c_void;

    fn as_abi_in(&self) -> Self::In {
        self.handle as Self::In
    }

    fn as_abi_out(&mut self) -> Self::Out {
        debug_assert!(self.is_empty());
        &mut (self.handle as *mut std::ffi::c_void)
    }

    fn detach_abi(&mut self) -> Self::In {
        let handle = self.as_abi_in();
        self.handle = std::ptr::null_mut();
        handle
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
                debug_assert!((*self.handle).flags & REFERENCE_FLAG == 0);

                if 0 == (*(self.handle as *const SharedHeader)).count.release() {
                    HeapFree(GetProcessHeap(), 0, self.handle as *const std::ffi::c_void);
                }
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
            let mut handle = with_len(value.len() as u32);

            for (index, wide) in value.encode_utf16().enumerate() {
                *((*handle).ptr.offset(index as isize) as *mut u16) = wide;
                (*handle).len = index as u32 + 1;
            }

            *((*handle).ptr.offset((*handle).len as isize) as *mut u16) = 0;
            Self { handle }
        }
    }
}

impl From<std::string::String> for String {
    fn from(value: std::string::String) -> String {
        value.as_str().into()
    }
}

impl From<&std::string::String> for String {
    fn from(value: &std::string::String) -> String {
        value.as_str().into()
    }
}

impl From<*mut std::ffi::c_void> for String {
    fn from(handle: *mut std::ffi::c_void) -> String {
        Self { handle: handle as *const Header }
    }
}



// impl AsRef<String> for String {
//     fn as_ref(&self) -> &String {
//         self
//     }
// }

// impl AsRef<String> for str {
//     fn as_ref(&self) -> &String {
//         self
//     }
// }


// pub enum Str<'a> {
//     Slice(&'a str),
//     Borrowed(&'a String),
//     Owned(String),
// }

// impl<'a> std::borrow::Borrow<Str<'a>> for String {
//     fn borrow(&self) -> &Str<'a> {
//         panic!();
//     }
// }

// impl<'a> std::borrow::BorrowMut<Str<'a>> for String {
//     fn borrow_mut(&mut self) -> &mut Str<'a> {
//         panic!();
//     }
// }