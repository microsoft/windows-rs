#![allow(dead_code)]

use crate::*;
use std::ptr;

#[repr(transparent)]
pub struct HString {
    ptr: Option<ptr::NonNull<Header>>,
}

impl HString {
    pub fn new() -> HString {
        HString { ptr: None }
    }

    pub fn is_empty(&self) -> bool {
        self.ptr.is_none()
    }

    pub fn len(&self) -> usize {
        match self.ptr {
            None => 0,
            Some(ptr) => unsafe { ptr.as_ref().len as usize },
        }
    }

    pub fn as_chars(&self) -> &[u16] {
        match self.ptr {
            None => &[],
            Some(ptr) => unsafe {
                let header = ptr.as_ref();
                std::slice::from_raw_parts(header.ptr, header.len as usize)
            },
        }
    }

    pub fn clear(&mut self) {
        if let Some(mut ptr) = self.ptr {
            let header = unsafe { ptr.as_mut() };
            debug_assert!(header.flags & REFERENCE_FLAG == 0);

            unsafe {
                if 0 == (*header.shared.as_mut_ptr()).count.release() {
                    HeapFree(GetProcessHeap(), 0, ptr.as_ptr() as *mut std::ffi::c_void);
                }
            }

            self.ptr = None;
        }
    }
}

impl RuntimeType for HString {
    type Abi = RawPtr;

    fn abi(&self) -> Self::Abi {
        self.ptr
            .map(|p| p.as_ptr())
            .unwrap_or_else(std::ptr::null_mut) as RawPtr
    }

    fn set_abi(&mut self) -> *mut Self::Abi {
        self.clear();
        &mut self.abi()
    }
}

impl Default for HString {
    fn default() -> Self {
        HString::new()
    }
}

impl Clone for HString {
    fn clone(&self) -> HString {
        let ptr = self.ptr.map(|mut p| unsafe { p.as_mut().duplicate() });
        HString { ptr }
    }
}

impl Drop for HString {
    fn drop(&mut self) {
        self.clear();
    }
}

impl std::fmt::Display for HString {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use std::fmt::Write;
        for c in std::char::decode_utf16(self.as_chars().iter().cloned()) {
            f.write_char(c.map_err(|_| std::fmt::Error)?)?
        }
        Ok(())
    }
}

impl std::fmt::Debug for HString {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        <Self as std::fmt::Display>::fmt(self, f)
    }
}

impl From<&str> for HString {
    fn from(value: &str) -> HString {
        let mut ptr = Header::alloc(value.len() as u32);
        unsafe {
            // place each utf-16 character into the buffer and
            // increase len as we go along
            for (index, wide) in value.encode_utf16().enumerate() {
                ptr::write((*ptr).ptr.add(index) as *mut _, wide);
                (*ptr).len = index as u32 + 1;
            }

            // write a 0 byte to the end of the buffer
            ptr::write((*ptr).ptr.offset((*ptr).len as isize) as *mut _, 0);
            Self {
                ptr: Some(ptr::NonNull::new_unchecked(ptr)),
            }
        }
    }
}

impl From<String> for HString {
    fn from(value: String) -> HString {
        value.as_str().into()
    }
}

impl From<&String> for HString {
    fn from(value: &String) -> HString {
        value.as_str().into()
    }
}

impl PartialEq for HString {
    fn eq(&self, other: &Self) -> bool {
        self.as_chars() == other.as_chars()
    }
}

impl PartialEq<String> for HString {
    fn eq(&self, other: &String) -> bool {
        self == other.as_str()
    }
}

impl PartialEq<str> for HString {
    fn eq(&self, other: &str) -> bool {
        self == other
    }
}

impl PartialEq<&str> for HString {
    fn eq(&self, other: &&str) -> bool {
        self.as_chars().iter().copied().eq(other.encode_utf16())
    }
}

impl<'a> From<&'a HString> for String {
    fn from(hstring: &HString) -> Self {
        String::from_utf16(hstring.as_chars()).unwrap()
    }
}

impl From<HString> for String {
    fn from(hstring: HString) -> Self {
        hstring.into()
    }
}

const REFERENCE_FLAG: u32 = 1;

#[repr(C)]
struct Header {
    flags: u32,
    len: u32,
    _0: u32,
    _1: u32,
    ptr: *const u16,
    shared: std::mem::MaybeUninit<Shared>,
}

#[repr(C)]
struct Shared {
    count: RefCount,
    buffer_start: u16,
}

impl Header {
    fn alloc(len: u32) -> *mut Header {
        debug_assert!(len != 0);
        // alloc enough space for header and two bytes per character
        let alloc_size = std::mem::size_of::<Header>() + 2 * len as usize;
        let header = unsafe { HeapAlloc(GetProcessHeap(), 0, alloc_size) as *mut Header };

        if header.is_null() {
            panic!("Could not successfully allocate for HString");
        }

        unsafe {
            (*header).flags = 0;
            (*header).len = len;
            (*header).ptr = &(*(*header).shared.as_ptr()).buffer_start;
            (*(*header).shared.as_mut_ptr()).count = RefCount::new(1);
        }
        header as *mut Header
    }

    fn duplicate(&mut self) -> ptr::NonNull<Header> {
        if self.flags & REFERENCE_FLAG == 0 {
            unsafe {
                (*self.shared.as_ptr()).count.addref();
                ptr::NonNull::new_unchecked(self)
            }
        } else {
            let copy = Header::alloc(self.len);
            unsafe {
                std::ptr::copy_nonoverlapping(
                    self.ptr,
                    (*copy).ptr as *mut u16,
                    self.len as usize + 1,
                );
                ptr::NonNull::new_unchecked(copy)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hstring_works() {
        let empty = HString::new();
        assert!(empty.is_empty());
        assert!(empty.len() == 0);

        let mut hello = HString::from("Hello");
        assert!(!hello.is_empty());
        assert!(hello.len() == 5);

        let rust = hello.to_string();
        assert!(rust == "Hello");
        assert!(rust.len() == 5);

        let hello2 = hello.clone();
        hello.clear();
        assert!(hello.len() == 0);
        hello.clear();
        assert!(hello.len() == 0);
        assert!(!hello2.is_empty());
        assert!(hello2.len() == 5);

        assert!(HString::from("Hello") == HString::from("Hello"));
        assert!(HString::from("Hello") != HString::from("World"));

        assert!(HString::from("Hello") == "Hello");
        assert!(HString::from("Hello") != "Hello ");
        assert!(HString::from("Hello") != "Hell");
        assert!(HString::from("Hello") != "World");

        assert!(HString::from("Hello").to_string() == String::from("Hello"));
    }

    #[test]
    fn display_format() {
        let value = HString::from("Hello world");
        assert!(format!("{}", value) == "Hello world");
    }

    #[test]
    fn debug_format() {
        let value = HString::from("Hello world");
        assert!(format!("{:#?}", value) == "Hello world");
    }
}
