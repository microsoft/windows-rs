use super::*;

/// A pointer to a null-terminated string of 8-bit Windows (ANSI) characters.
#[repr(transparent)]
pub struct PSTR(pub *mut u8);
impl PSTR {
    pub fn is_null(&self) -> bool {
        self.0.is_null()
    }
}
impl ::core::default::Default for PSTR {
    fn default() -> Self {
        Self(::core::ptr::null_mut())
    }
}
impl ::core::clone::Clone for PSTR {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::marker::Copy for PSTR {}
impl ::core::cmp::PartialEq for PSTR {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PSTR {}
impl ::core::fmt::Debug for PSTR {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PSTR").field(&self.0).finish()
    }
}
unsafe impl Abi for PSTR {
    type Abi = Self;
}
