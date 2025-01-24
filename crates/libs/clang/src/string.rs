use super::*;

#[repr(C)]
#[derive(Clone, Copy)]
pub struct CXString(*const c_void, u32);

impl Deref for CXString {
    type Target = str;

    fn deref(&self) -> &str {
        unsafe {
            let ptr = clang_getCString(*self);
            let len = strlen(ptr);
            std::str::from_utf8_unchecked(std::slice::from_raw_parts(ptr, len))
        }
    }
}

impl Debug for CXString {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "\"{}\"", self.deref())
    }
}

impl PartialEq<&str> for CXString {
    fn eq(&self, other: &&str) -> bool {
        self.deref() == *other
    }
}

impl Free for CXString {
    unsafe fn free(&mut self) {
        clang_disposeString(*self);
    }
}
