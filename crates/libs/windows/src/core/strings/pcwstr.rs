use super::*;

/// A pointer to a constant null-terminated string of 16-bit Unicode characters.
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct PCWSTR(pub *const u16);

impl PCWSTR {
    /// Construct a new `PCWSTR` from a raw pointer
    pub const fn from_raw(ptr: *const u16) -> Self {
        Self(ptr)
    }

    /// Construct a null `PCWSTR`
    pub fn null() -> Self {
        Self(core::ptr::null())
    }

    /// Returns a raw pointer to the `PCWSTR`
    pub fn as_ptr(&self) -> *const u16 {
        self.0
    }

    /// Checks whether the `PCWSTR` is null
    pub fn is_null(&self) -> bool {
        self.0.is_null()
    }

    /// String data without the trailing 0
    ///
    /// # Safety
    ///
    /// The `PCWSTR`'s pointer needs to be valid for reads up until and including the next `\0`.
    pub unsafe fn as_wide(&self) -> &[u16] {
        let len = super::wcslen(*self);
        std::slice::from_raw_parts(self.0, len)
    }

    /// Copy the `PCWSTR` into a Rust `String`.
    ///
    /// # Safety
    ///
    /// See the safety information for `PCWSTR::as_wide`.
    pub unsafe fn to_string(&self) -> core::result::Result<String, std::string::FromUtf16Error> {
        String::from_utf16(self.as_wide())
    }

    /// Allow this string to be displayed.
    ///
    /// # Safety
    ///
    /// See the safety information for `PCWSTR::as_wide`.
    pub unsafe fn display<'a>(&'a self) -> impl core::fmt::Display + 'a {
        Decode(move || core::char::decode_utf16(self.as_wide().iter().cloned()))
    }
}

unsafe impl Abi for PCWSTR {
    type Abi = Self;
}

impl From<&HSTRING> for PCWSTR {
    fn from(hstring: &HSTRING) -> Self {
        Self(hstring.as_ptr())
    }
}

// This just ensures that `None` can be used for optional PCWSTR parameters, which can be quite common
// with some Windows APIs.
impl From<Option<PCWSTR>> for PCWSTR {
    fn from(from: Option<PCWSTR>) -> Self {
        from.unwrap_or_else(Self::null)
    }
}
