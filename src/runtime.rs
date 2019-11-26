#![allow(dead_code)]

pub enum VOID {}
pub type PCWSTR = *const u16;
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

#[repr(C)]
pub struct HRESULT(pub i32);

#[link(name = "onecore")]
extern "C" {
    pub fn WindowsCreateString(value: PCWSTR, length: u32, result: &mut HSTRING) -> HRESULT;
    pub fn WindowsGetStringRawBuffer(value: HSTRING, length: &mut u32) -> PCWSTR;

    pub fn CoIncrementMTAUsage(cookie: *mut *mut VOID) -> HRESULT;
    pub fn RoGetActivationFactory(
        class: HSTRING,
        interface: &GUID,
        result: *mut *mut VOID,
    ) -> HRESULT;
}

impl GUID {
    pub fn from_values(data1: u32, data2: u16, data3: u16, data4: &[u8; 8]) -> GUID {
        GUID {
            data1,
            data2,
            data3,
            data4: *data4,
        }
    }
}

impl HRESULT {
    pub fn is_ok(&self) -> bool {
        self.0 >= 0
    }

    pub fn is_err(&self) -> bool {
        self.0 < 0
    }

    pub fn unwrap(&self) {
        assert!(self.is_ok(), "HRESULT 0x{:X}", self.0);
    }
}
