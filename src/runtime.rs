#![allow(dead_code)]

#[link(name = "onecore")]
extern "C" {
    pub fn WindowsCreateString(value: PCWSTR, length: u32, result: &mut HSTRING) -> HRESULT;
    pub fn WindowsGetStringRawBuffer(value: HSTRING, length: &mut u32) -> PCWSTR;

    pub fn RoInitialize(apartment: ApartmentType) -> HRESULT;
    pub fn RoGetActivationFactory(class: HSTRING, interface: &GUID, result: *mut *mut VOID) -> HRESULT;
}

pub enum VOID {}
pub type PCWSTR = *const u16;
pub type HRESULT = i32;
pub type HSTRING = *mut VOID;

#[repr(C)]
pub enum ApartmentType {
    SingleThreaded,
    MultiThreaded,
}

#[repr(C)]
pub struct GUID {
    data1: u32,
    data2: u16,
    data3: u16,
    data4: [u8; 8],
}

impl GUID {
    pub fn from_values(data1: u32, data2: u16, data3: u16, data4: &[u8; 8]) -> GUID {
        GUID { data1, data2, data3, data4: *data4 }
    }
}
