use super::*;
use bindings::*;

#[repr(transparent)]
pub struct BSTR(*const u16);

impl BSTR {
    pub const fn new() -> Self {
        Self(std::ptr::null_mut())
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

    pub fn from_wide(value: &[u16]) -> Result<Self> {
        if value.is_empty() {
            return Ok(Self::new());
        }

        let result = unsafe { Self(SysAllocStringLen(value.as_ptr(), value.len() as _)) };

        if result.is_empty() {
            Err(E_OUTOFMEMORY.into())
        } else {
            Ok(result)
        }
    }

    pub fn as_wide(&self) -> &[u16] {
        if self.0.is_null() {
            return &[];
        }

        unsafe { std::slice::from_raw_parts(self.0, self.len()) }
    }

    /// # Safety
    pub unsafe fn from_raw(raw: *const u16) -> Self {
        Self(raw)
    }

    pub fn into_raw(self) -> *const u16 {
        unsafe { std::mem::transmute(self) }
    }
}
impl std::clone::Clone for BSTR {
    fn clone(&self) -> Self {
        Self::from_wide(self.as_wide()).unwrap()
    }
}
impl std::convert::From<&str> for BSTR {
    fn from(value: &str) -> Self {
        let value: windows::core::alloc::vec::Vec<u16> = value.encode_utf16().collect();
        Self::from_wide(&value).unwrap()
    }
}
impl std::convert::From<windows::core::alloc::string::String> for BSTR {
    fn from(value: windows::core::alloc::string::String) -> Self {
        value.as_str().into()
    }
}
impl std::convert::From<&windows::core::alloc::string::String> for BSTR {
    fn from(value: &windows::core::alloc::string::String) -> Self {
        value.as_str().into()
    }
}
impl<'a> std::convert::TryFrom<&'a BSTR> for windows::core::alloc::string::String {
    type Error = windows::core::alloc::string::FromUtf16Error;

    fn try_from(value: &BSTR) -> std::result::Result<Self, Self::Error> {
        windows::core::alloc::string::String::from_utf16(value.as_wide())
    }
}
impl std::convert::TryFrom<BSTR> for windows::core::alloc::string::String {
    type Error = windows::core::alloc::string::FromUtf16Error;

    fn try_from(value: BSTR) -> std::result::Result<Self, Self::Error> {
        windows::core::alloc::string::String::try_from(&value)
    }
}
impl std::default::Default for BSTR {
    fn default() -> Self {
        Self(std::ptr::null_mut())
    }
}
impl std::fmt::Display for BSTR {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::write!(f, "{}", windows::core::Decode(|| std::char::decode_utf16(self.as_wide().iter().cloned())))
    }
}
impl std::fmt::Debug for BSTR {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::write!(f, "{self}")
    }
}
impl std::cmp::PartialEq for BSTR {
    fn eq(&self, other: &Self) -> bool {
        self.as_wide() == other.as_wide()
    }
}
impl std::cmp::Eq for BSTR {}

impl std::cmp::PartialEq<BSTR> for &str {
    fn eq(&self, other: &BSTR) -> bool {
        other == self
    }
}

impl std::cmp::PartialEq<BSTR> for String {
    fn eq(&self, other: &BSTR) -> bool {
        other == self
    }
}

impl<T: AsRef<str> + ?Sized> std::cmp::PartialEq<T> for BSTR {
    fn eq(&self, other: &T) -> bool {
        self.as_wide().iter().copied().eq(other.as_ref().encode_utf16())
    }
}

impl std::ops::Drop for BSTR {
    fn drop(&mut self) {
        if !self.0.is_null() {
            unsafe { SysFreeString(self.0) }
        }
    }
}
unsafe impl windows::core::Abi for BSTR {
    type Abi = *mut std::ffi::c_void;
}
