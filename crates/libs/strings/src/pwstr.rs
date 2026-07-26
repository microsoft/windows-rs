use super::*;

/// A pointer to a null-terminated string of 16-bit Unicode characters.
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct PWSTR(pub *mut u16);

impl PWSTR {
    /// Constructs a `PWSTR` from a raw pointer.
    pub const fn from_raw(ptr: *mut u16) -> Self {
        Self(ptr)
    }

    /// Constructs a null `PWSTR`.
    pub const fn null() -> Self {
        Self(core::ptr::null_mut())
    }

    /// Returns the raw pointer.
    pub const fn as_ptr(&self) -> *mut u16 {
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
    /// The `PWSTR`'s pointer needs to be valid for reads up until and including the next `\0`.
    pub unsafe fn len(&self) -> usize {
        unsafe { PCWSTR(self.0).len() }
    }

    /// Returns whether the string is empty.
    ///
    /// # Safety
    ///
    /// The `PWSTR`'s pointer needs to be valid for reads up until and including the next `\0`.
    pub unsafe fn is_empty(&self) -> bool {
        unsafe { self.len() == 0 }
    }

    /// Returns the string data without the trailing null.
    ///
    /// # Safety
    ///
    /// The `PWSTR`'s pointer needs to be valid for reads up until and including the next `\0`.
    pub unsafe fn as_wide(&self) -> &[u16] {
        unsafe { core::slice::from_raw_parts(self.0, self.len()) }
    }

    /// Copies the string into a Rust `String`.
    ///
    /// # Safety
    ///
    /// See the safety information for `PWSTR::as_wide`.
    pub unsafe fn to_string(&self) -> Result<String, alloc::string::FromUtf16Error> {
        unsafe { String::from_utf16(self.as_wide()) }
    }

    /// Copies the string into an `HSTRING`.
    ///
    /// # Safety
    ///
    /// See the safety information for `PWSTR::as_wide`.
    pub unsafe fn to_hstring(&self) -> HSTRING {
        unsafe { HSTRING::from_wide(self.as_wide()) }
    }

    /// Returns a display adapter for the string.
    ///
    /// # Safety
    ///
    /// See the safety information for `PWSTR::as_wide`.
    pub unsafe fn display(&self) -> impl core::fmt::Display + '_ {
        unsafe { Decode(move || core::char::decode_utf16(self.as_wide().iter().copied())) }
    }
}

impl Default for PWSTR {
    fn default() -> Self {
        Self::null()
    }
}
