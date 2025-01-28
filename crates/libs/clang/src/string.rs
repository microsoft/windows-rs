use super::*;

#[repr(C)]
#[derive(Clone, Copy)]
pub struct CXString(*const std::ffi::c_void, u32);

impl Free for CXString {
    fn free(&mut self) {
        link!("libclang.dll" "system" fn clang_disposeString(_: CXString));
        unsafe { clang_disposeString(*self) }
    }
}

impl std::ops::Deref for CXString {
    type Target = str;

    fn deref(&self) -> &str {
        self.as_str()
    }
}

impl PartialEq<&str> for CXString {
    fn eq(&self, other: &&str) -> bool {
        self.as_str() == *other
    }
}

impl PartialEq<&str> for Owned<CXString> {
    fn eq(&self, other: &&str) -> bool {
        self.as_str() == *other
    }
}

impl std::fmt::Debug for CXString {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "\"{}\"", self.as_str())
    }
}

impl std::fmt::Display for CXString {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(fmt, "{}", self.as_str())
    }
}

impl CXString {
    pub fn as_str(&self) -> &str {
        link!("libclang.dll" "system" fn clang_getCString(_: CXString) -> *const u8);

        extern "C" {
            fn strlen(s: *const u8) -> usize;
        }

        unsafe {
            let ptr = clang_getCString(*self);
            let len = strlen(ptr);
            std::str::from_utf8_unchecked(std::slice::from_raw_parts(ptr, len))
        }
    }
}
