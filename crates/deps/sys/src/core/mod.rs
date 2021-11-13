#[repr(C)]
pub struct GUID {
    pub data1: u32,
    pub data2: u16,
    pub data3: u16,
    pub data4: [u8; 8],
}
impl ::core::marker::Copy for GUID {}
impl ::core::clone::Clone for GUID {
    fn clone(&self) -> Self {
        *self
    }
}

#[must_use]
#[repr(transparent)]
#[allow(non_camel_case_types)]
pub struct HRESULT(pub u32);
impl core::marker::Copy for HRESULT {}
impl core::clone::Clone for HRESULT {
    fn clone(&self) -> Self {
        *self
    }
}

#[repr(transparent)]
pub struct HSTRING(*mut core::ffi::c_void);
impl ::core::marker::Copy for HSTRING {}
impl ::core::clone::Clone for HSTRING {
    fn clone(&self) -> Self {
        *self
    }
}

#[repr(transparent)]
pub struct IInspectable(*mut core::ffi::c_void);
impl ::core::marker::Copy for IInspectable {}
impl ::core::clone::Clone for IInspectable {
    fn clone(&self) -> Self {
        *self
    }
}

#[repr(transparent)]
pub struct IUnknown(*mut core::ffi::c_void);
impl ::core::marker::Copy for IUnknown {}
impl ::core::clone::Clone for IUnknown {
    fn clone(&self) -> Self {
        *self
    }
}
