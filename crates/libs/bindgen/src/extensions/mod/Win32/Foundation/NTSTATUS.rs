impl NTSTATUS {
    #[inline]
    pub const fn is_ok(self) -> bool {
        self.0 >= 0
    }
    #[inline]
    pub const fn is_err(self) -> bool {
        !self.is_ok()
    }
    #[inline]
    pub const fn to_hresult(self) -> ::windows_core::HRESULT {
        ::windows_core::HRESULT(self.0 | 0x1000_0000)
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
impl ::core::convert::From<NTSTATUS> for ::windows_core::HRESULT {
    fn from(value: NTSTATUS) -> Self {
        value.to_hresult()
    }
}
impl ::core::convert::From<NTSTATUS> for ::windows_core::Error {
    fn from(value: NTSTATUS) -> Self {
        value.to_hresult().into()
    }
}
