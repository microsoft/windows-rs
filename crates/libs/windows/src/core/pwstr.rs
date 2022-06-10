use super::*;

/// A pointer to a null-terminated string of 16-bit Unicode characters.
#[repr(transparent)]
pub struct PWSTR(pub *mut u16);
impl PWSTR {
    pub fn is_null(&self) -> bool {
        self.0.is_null()
    }
}
impl ::core::default::Default for PWSTR {
    fn default() -> Self {
        Self(::core::ptr::null_mut())
    }
}
impl ::core::clone::Clone for PWSTR {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::marker::Copy for PWSTR {}
impl ::core::cmp::PartialEq for PWSTR {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PWSTR {}
impl ::core::fmt::Debug for PWSTR {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PWSTR").field(&self.0).finish()
    }
}
unsafe impl Abi for PWSTR {
    type Abi = Self;
}
