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

    #[cfg(feature = "alloc")]
    unsafe fn drop_param(param: &mut Param<'_, Self>) {
        if let Param::Boxed(value) = param {
            if !value.is_null() {
                alloc::boxed::Box::from_raw(value.0 as *mut u8);
            }
        }
    }
}
#[cfg(feature = "alloc")]
impl<'a> IntoParam<'a, PCSTR> for &str {
    fn into_param(self) -> Param<'a, PCSTR> {
        Param::Boxed(PCSTR(alloc::boxed::Box::<[u8]>::into_raw(self.bytes().chain(::core::iter::once(0)).collect::<alloc::vec::Vec<u8>>().into_boxed_slice()) as _))
    }
}
#[cfg(feature = "alloc")]
impl<'a> IntoParam<'a, PCSTR> for alloc::string::String {
    fn into_param(self) -> Param<'a, PCSTR> {
        IntoParam::into_param(self.as_str())
    }
}
