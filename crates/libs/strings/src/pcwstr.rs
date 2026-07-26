use super::*;

/// A pointer to a constant null-terminated string of 16-bit Unicode characters.
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct PCWSTR(pub *const u16);

impl PCWSTR {
    /// Constructs a `PCWSTR` from a raw pointer.
    pub const fn from_raw(ptr: *const u16) -> Self {
        Self(ptr)
    }

    /// Constructs a null `PCWSTR`.
    pub const fn null() -> Self {
        Self(core::ptr::null())
    }

    /// Returns the raw pointer.
    pub const fn as_ptr(&self) -> *const u16 {
        self.0
    }

    /// Returns whether the pointer is null.
    pub fn is_null(&self) -> bool {
        self.0.is_null()
    }

    /// Returns the string length without the trailing null.
    ///
    /// # Safety
    ///
    /// The `PCWSTR`'s pointer needs to be valid for reads up until and including the next `\0`.
    pub unsafe fn len(&self) -> usize {
        unsafe extern "C" {
            fn wcslen(s: *const u16) -> usize;
        }
        unsafe { wcslen(self.0) }
    }

    /// Returns whether the string is empty.
    ///
    /// # Safety
    ///
    /// The `PCWSTR`'s pointer needs to be valid for reads up until and including the next `\0`.
    pub unsafe fn is_empty(&self) -> bool {
        unsafe { self.len() == 0 }
    }

    /// Returns the string data without the trailing null.
    ///
    /// # Safety
    ///
    /// The `PCWSTR`'s pointer needs to be valid for reads up until and including the next `\0`.
    pub unsafe fn as_wide(&self) -> &[u16] {
        unsafe { core::slice::from_raw_parts(self.0, self.len()) }
    }

    /// Copies the string into a Rust `String`.
    ///
    /// # Safety
    ///
    /// See the safety information for `PCWSTR::as_wide`.
    pub unsafe fn to_string(&self) -> Result<String, alloc::string::FromUtf16Error> {
        unsafe { String::from_utf16(self.as_wide()) }
    }

    /// Copies the string into an `HSTRING`.
    ///
    /// # Safety
    ///
    /// See the safety information for `PCWSTR::as_wide`.
    pub unsafe fn to_hstring(&self) -> HSTRING {
        unsafe { HSTRING::from_wide(self.as_wide()) }
    }

    /// Returns a display adapter for the string.
    ///
    /// # Safety
    ///
    /// See the safety information for `PCWSTR::as_wide`.
    pub unsafe fn display(&self) -> impl core::fmt::Display + '_ {
        unsafe { Decode(move || core::char::decode_utf16(self.as_wide().iter().copied())) }
    }
}

impl Default for PCWSTR {
    fn default() -> Self {
        Self::null()
    }
}

impl AsRef<Self> for PCWSTR {
    fn as_ref(&self) -> &Self {
        self
    }
}

impl core::ops::Deref for PCWSTR {
    type Target = *const u16;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
