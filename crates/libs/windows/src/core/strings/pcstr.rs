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

    /// Allow this string to be displayed.
    ///
    /// # Safety
    ///
    /// See the safety information for `PCSTR::as_bytes`.
    pub unsafe fn display<'a>(&'a self) -> impl core::fmt::Display + 'a {
        Decode(move || decode_utf8(self.as_bytes()))
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_display() {
        // 💖 followed by an invalid byte sequence and then an incomplete one
        let s = vec![240, 159, 146, 150, 255, 240, 159, 0];
        let s = PCSTR::from_raw(s.as_ptr());
        assert_eq!("💖�", format!("{}", unsafe { s.display() }));
    }
}
