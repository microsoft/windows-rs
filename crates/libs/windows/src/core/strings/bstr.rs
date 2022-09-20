use super::*;
use bindings::*;

#[repr(transparent)]
pub struct BSTR(*const u16);

impl BSTR {
    pub const fn new() -> Self {
        Self(core::ptr::null_mut())
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn len(&self) -> usize {
        if self.0.is_null() {
            0
        } else {
            unsafe { SysStringLen(self.0) as usize }
        }
    }

    pub fn from_wide(value: &[u16]) -> Self {
        if value.is_empty() {
            return Self(::core::ptr::null_mut());
        }

        unsafe { Self(SysAllocStringLen(value.as_ptr(), value.len() as _)) }
    }

    pub fn as_wide(&self) -> &[u16] {
        if self.0.is_null() {
            return &[];
        }

        unsafe { ::core::slice::from_raw_parts(self.0, self.len()) }
    }

    pub unsafe fn from_raw(raw: *const u16) -> Self {
        Self(raw)
    }

    pub fn into_raw(self) -> *const u16 {
        unsafe { std::mem::transmute(self) }
    }
}
impl ::core::clone::Clone for BSTR {
    fn clone(&self) -> Self {
        Self::from_wide(self.as_wide())
    }
}
impl ::core::convert::From<&str> for BSTR {
    fn from(value: &str) -> Self {
        let value: ::windows::core::alloc::vec::Vec<u16> = value.encode_utf16().collect();
        Self::from_wide(&value)
    }
}
impl ::core::convert::From<::windows::core::alloc::string::String> for BSTR {
    fn from(value: ::windows::core::alloc::string::String) -> Self {
        value.as_str().into()
    }
}
impl ::core::convert::From<&::windows::core::alloc::string::String> for BSTR {
    fn from(value: &::windows::core::alloc::string::String) -> Self {
        value.as_str().into()
    }
}
impl<'a> ::core::convert::TryFrom<&'a BSTR> for ::windows::core::alloc::string::String {
    type Error = ::windows::core::alloc::string::FromUtf16Error;

    fn try_from(value: &BSTR) -> ::core::result::Result<Self, Self::Error> {
        ::windows::core::alloc::string::String::from_utf16(value.as_wide())
    }
}
impl ::core::convert::TryFrom<BSTR> for ::windows::core::alloc::string::String {
    type Error = ::windows::core::alloc::string::FromUtf16Error;

    fn try_from(value: BSTR) -> ::core::result::Result<Self, Self::Error> {
        ::windows::core::alloc::string::String::try_from(&value)
    }
}
impl ::core::default::Default for BSTR {
    fn default() -> Self {
        Self(::core::ptr::null_mut())
    }
}
impl ::core::fmt::Display for BSTR {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        ::core::write!(f, "{}", ::windows::core::Decode(|| ::core::char::decode_utf16(self.as_wide().iter().cloned())))
    }
}
impl ::core::fmt::Debug for BSTR {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        ::core::write!(f, "{}", self)
    }
}
impl ::core::cmp::PartialEq for BSTR {
    fn eq(&self, other: &Self) -> bool {
        self.as_wide() == other.as_wide()
    }
}
impl ::core::cmp::Eq for BSTR {}
impl ::core::cmp::PartialEq<::windows::core::alloc::string::String> for BSTR {
    fn eq(&self, other: &::windows::core::alloc::string::String) -> bool {
        self == other.as_str()
    }
}
impl ::core::cmp::PartialEq<str> for BSTR {
    fn eq(&self, other: &str) -> bool {
        self == other
    }
}
impl ::core::cmp::PartialEq<&str> for BSTR {
    fn eq(&self, other: &&str) -> bool {
        self.as_wide().iter().copied().eq(other.encode_utf16())
    }
}

impl ::core::cmp::PartialEq<BSTR> for &str {
    fn eq(&self, other: &BSTR) -> bool {
        other == self
    }
}
impl ::core::ops::Drop for BSTR {
    fn drop(&mut self) {
        if !self.0.is_null() {
            unsafe { SysFreeString(self.0) }
        }
    }
}
unsafe impl ::windows::core::Abi for BSTR {
    type Abi = ::core::mem::ManuallyDrop<Self>;
}
