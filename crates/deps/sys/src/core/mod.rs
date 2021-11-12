#[must_use]
#[repr(transparent)]
#[allow(non_camel_case_types)]
pub struct HRESULT(pub u32);

#[repr(C)]
pub struct GUID {
    pub data1: u32,
    pub data2: u16,
    pub data3: u16,
    pub data4: [u8; 8],
}
