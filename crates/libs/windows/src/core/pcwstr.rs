use super::*;

/// A pointer to a constant null-terminated string of 16-bit Unicode characters.
#[repr(transparent)]
pub struct PCWSTR(pub *const u16);
impl PCWSTR {
    pub fn is_null(&self) -> bool {
        self.0.is_null()
    }
}
impl ::core::default::Default for PCWSTR {
    fn default() -> Self {
        Self(::core::ptr::null())
    }
}
impl ::core::clone::Clone for PCWSTR {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::marker::Copy for PCWSTR {}
impl ::core::cmp::PartialEq for PCWSTR {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PCWSTR {}
impl ::core::fmt::Debug for PCWSTR {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PCWSTR").field(&self.0).finish()
    }
}
unsafe impl Abi for PCWSTR {
    type Abi = Self;
}
