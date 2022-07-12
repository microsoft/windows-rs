use super::*;

/// A pointer to a constant null-terminated string of 8-bit Windows (ANSI) characters.
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct PCSTR(pub *const u8);

impl PCSTR {
    /// Construct a new `PCSTR` from a raw pointer
    pub const fn from_raw(ptr: *const u8) -> Self {
        Self(ptr)
    }

    /// Construct a null `PCSTR`
    pub fn null() -> Self {
        Self(core::ptr::null())
    }

    /// Returns a raw pointer to the `PCSTR`
    pub fn as_ptr(&self) -> *const u8 {
        self.0
    }

    /// Checks whether the `PCSTR` is null
    pub fn is_null(&self) -> bool {
        self.0.is_null()
    }

    /// String data without the trailing 0
    ///
    /// # Safety
    ///
    /// The `PCSTR`'s pointer needs to be valid for reads up until and including the next `\0`.
    pub unsafe fn as_bytes(&self) -> &[u8] {
        let len = super::strlen(*self);
        std::slice::from_raw_parts(self.0, len)
    }

    /// Copy the `PCSTR` into a Rust `String`.
    ///
    /// # Safety
    ///
    /// See the safety information for `PCSTR::as_bytes`.
    pub unsafe fn to_string(&self) -> core::result::Result<String, std::string::FromUtf8Error> {
        String::from_utf8(self.as_bytes().into())
    }
}

unsafe impl Abi for PCSTR {
    type Abi = Self;
}

// This just ensures that `None` can be used for optional PCSTR parameters, which can be quite common
// with some Windows APIs.
impl From<Option<PCSTR>> for PCSTR {
    fn from(from: Option<PCSTR>) -> Self {
        from.unwrap_or_else(Self::null)
    }
}
