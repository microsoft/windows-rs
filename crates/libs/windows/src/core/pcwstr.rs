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

    unsafe fn from_abi(abi: Self::Abi) -> Result<Self> {
        Ok(abi)
    }

    #[cfg(feature = "alloc")]
    unsafe fn drop_param(param: &mut Param<'_, Self>) {
        if let Param::Boxed(value) = param {
            if !value.is_null() {
                heap_free(value.0 as _);
            }
        }
    }
}
#[cfg(feature = "alloc")]
impl<'a> IntoParam<'a, PCWSTR> for &str {
    fn into_param(self) -> Param<'a, PCWSTR> {
        Param::Boxed(PCWSTR(alloc_from_iter(self.encode_utf16().chain(core::iter::once(0)), self.len() + 1)))
    }
}
#[cfg(feature = "alloc")]
impl<'a> IntoParam<'a, PCWSTR> for alloc::string::String {
    fn into_param(self) -> Param<'a, PCWSTR> {
        IntoParam::into_param(self.as_str())
    }
}
#[cfg(feature = "alloc")]
impl<'a> IntoParam<'a, PCWSTR> for &::std::ffi::OsStr {
    fn into_param(self) -> Param<'a, PCWSTR> {
        use ::std::os::windows::ffi::OsStrExt;
        Param::Boxed(PCWSTR(alloc_from_iter(self.encode_wide().chain(core::iter::once(0)), self.len() + 1)))
    }
}
#[cfg(feature = "alloc")]
impl<'a> IntoParam<'a, PCWSTR> for ::std::ffi::OsString {
    fn into_param(self) -> Param<'a, PCWSTR> {
        IntoParam::into_param(self.as_os_str())
    }
}
