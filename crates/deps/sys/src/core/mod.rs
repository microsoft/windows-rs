#[must_use]
#[repr(transparent)]
#[allow(non_camel_case_types)]
pub struct HRESULT(pub u32);
impl ::core::marker::Copy for HRESULT {}
impl ::core::clone::Clone for HRESULT {
    fn clone(&self) -> Self {
        *self
    }
}

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
