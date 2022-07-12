use super::*;

/// A pointer to a constant null-terminated string of 8-bit Windows (ANSI) characters.
#[repr(transparent)]
pub struct PCSTR(pub *const u8);

impl PCSTR {
    pub fn is_null(&self) -> bool {
        self.0.is_null()
    }
}

impl ::core::default::Default for PCSTR {
    fn default() -> Self {
        Self(::core::ptr::null())
    }
}

impl ::core::clone::Clone for PCSTR {
    fn clone(&self) -> Self {
        *self
    }
}

impl ::core::marker::Copy for PCSTR {}

impl ::core::cmp::PartialEq for PCSTR {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl ::core::cmp::Eq for PCSTR {}

impl ::core::fmt::Debug for PCSTR {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PCSTR").field(&self.0).finish()
    }
}

unsafe impl Abi for PCSTR {
    type Abi = Self;
}

// This just ensures that `None` can be used for optional PCSTR parameters, which can be quite common
// with some Windows APIs.
impl From<Option<PCSTR>> for PCSTR {
    fn from(from: Option<PCSTR>) -> Self {
        from.unwrap_or_default()
    }
}
