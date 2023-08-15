impl WIN32_ERROR {
    #[inline]
    pub const fn is_ok(self) -> bool {
        self.0 == 0
    }
    #[inline]
    pub const fn is_err(self) -> bool {
        !self.is_ok()
    }
    #[inline]
    pub const fn to_hresult(self) -> ::windows_core::HRESULT {
        ::windows_core::HRESULT(if self.0 == 0 { self.0 } else { (self.0 & 0x0000_FFFF) | (7 << 16) | 0x8000_0000 } as i32)
    }
    #[inline]
    pub fn from_error(error: &::windows_core::Error) -> ::core::option::Option<Self> {
        let hresult = error.code().0 as u32;
        if ((hresult >> 16) & 0x7FF) == 7 {
            Some(Self(hresult & 0xFFFF))
        } else {
            None
        }
    }
    #[inline]
    pub fn ok(self) -> ::windows_core::Result<()> {
        if self.is_ok() {
            Ok(())
        } else {
            Err(self.to_hresult().into())
        }
    }
}
impl ::core::convert::From<WIN32_ERROR> for ::windows_core::HRESULT {
    fn from(value: WIN32_ERROR) -> Self {
        value.to_hresult()
    }
}
impl ::core::convert::From<WIN32_ERROR> for ::windows_core::Error {
    fn from(value: WIN32_ERROR) -> Self {
        value.to_hresult().into()
    }
}
