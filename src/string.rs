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
fn duplicate(ptr: RawPtr) -> RawPtr {
    unsafe {
        let header = ptr as *const Header;
        if header.is_null() {
            std::ptr::null_mut()
        } else if (*header).flags & REFERENCE_FLAG == 0 {
            (*(header as *const SharedHeader)).count.addref();
            ptr
        } else {
            let copy = with_len((*header).len);
            std::ptr::copy_nonoverlapping((*header).ptr, (*copy).ptr as *mut u16, (*header).len as usize + 1);
            copy as RawPtr
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
    ptr: RawPtr,
}

impl String {
    pub fn new() -> String {
        String { ptr: std::ptr::null_mut() }
    }

    pub fn is_empty(&self) -> bool {
        self.ptr.is_null()
    }

    pub fn len(&self) -> usize {
        unsafe {
            if self.ptr.is_null() {
                0
            } else {
                (*(self.ptr as *const Header)).len as usize
            }
        }
    }

    pub fn as_chars(&self) -> &[u16] {
        unsafe {
            if self.ptr.is_null() {
                &[]
            } else {
                let header = self.ptr as *const Header;
                std::slice::from_raw_parts((*header).ptr, (*header).len as usize)
            }
        }
    }

    pub fn clear(&mut self) {
        unsafe {
            if !self.is_empty() {
                let header = self.ptr as *const SharedHeader;
                debug_assert!((*header).header.flags & REFERENCE_FLAG == 0);

                if 0 == (*header).count.release() {
                    HeapFree(GetProcessHeap(), 0, self.ptr);
                }

                self.ptr = std::ptr::null_mut();
            }
        }
    }
}

impl RuntimeType for String {
    type Abi = RawPtr;

    fn abi(&self) -> Self::Abi {
        self.ptr as RawPtr
    }

    fn set_abi(&mut self) -> *mut Self::Abi {
        self.clear();
        &mut self.ptr
    }
}

impl Default for String {
    fn default() -> Self {
        String::new()
    }
}

impl Clone for String {
    fn clone(&self) -> String {
        String { ptr: duplicate(self.ptr) }
    }
}

impl Drop for String {
    fn drop(&mut self) {
        self.clear();
    }
}

impl std::fmt::Display for String {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // TODO: how to format the wchar buffer directly to avoid an allocation?
        // Especially since `value.to_string()` relies on this... unless the formatter
        // can somehow move/forward the vector.
        write!(f, "{}", std::string::String::from_utf16(self.as_chars()).unwrap())
    }
}

impl From<&str> for String {
    fn from(value: &str) -> String {
        unsafe {
            let mut ptr = with_len(value.len() as u32);

            for (index, wide) in value.encode_utf16().enumerate() {
                *((*ptr).ptr.offset(index as isize) as *mut u16) = wide;
                (*ptr).len = index as u32 + 1;
            }

            *((*ptr).ptr.offset((*ptr).len as isize) as *mut u16) = 0;
            Self { ptr: ptr as RawPtr }
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
