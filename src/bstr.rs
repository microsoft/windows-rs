use core::fmt;
use core::ptr;

type RawBStr = *mut u16;

extern "system" {
    fn SysStringLen(s: RawBStr) -> u32;
    fn SysFreeString(s: RawBStr);
}

/// A handle to a [COM Basic String](https://docs.microsoft.com/en-us/previous-versions/windows/desktop/automat/bstr).
///
/// This handle should be used in FFI to represent a COM BSTR.
#[repr(transparent)]
pub struct BStr {
    ptr: RawBStr,
}

impl BStr {
    // Returns the length of the string.
    pub fn len(&self) -> usize {
        unsafe { SysStringLen(self.ptr) as usize }
    }

    /// Check whether the BSTR is the empty string or not
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Get the string as 16-bit wide characters (wchars)
    pub fn as_wide(&self) -> &[u16] {
        if self.is_empty() {
            return &[];
        }
        let len = self.len();
        unsafe { std::slice::from_raw_parts(self.ptr, len) }
    }
}

impl Default for BStr {
    fn default() -> Self {
        BStr {
            ptr: ptr::null_mut(),
        }
    }
}

impl From<BStr> for String {
    fn from(s: BStr) -> String {
        String::from(&s)
    }
}

impl From<&BStr> for String {
    fn from(s: &BStr) -> String {
        if !s.ptr.is_null() {
            String::from_utf16_lossy(s.as_wide())
        } else {
            String::new()
        }
    }
}

impl Drop for BStr {
    fn drop(&mut self) {
        unsafe {
            // Freeing NULL is allowed, and will noop.
            SysFreeString(self.ptr);
        }
    }
}

impl fmt::Debug for BStr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s: String = self.into();
        write!(f, "{:?}", s)
    }
}
