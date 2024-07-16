mod literals;

#[doc(hidden)]
pub use literals::*;

pub type HRESULT = i32;
pub type PSTR = *mut u8;
pub type PWSTR = *mut u16;
pub type PCSTR = *const u8;
pub type PCWSTR = *const u16;
pub type BSTR = *const u16;

#[repr(C)]
pub struct GUID {
    pub data1: u32,
    pub data2: u16,
    pub data3: u16,
    pub data4: [u8; 8],
}

impl Copy for GUID {}

impl Clone for GUID {
    fn clone(&self) -> Self {
        *self
    }
}

impl GUID {
    pub const fn from_u128(uuid: u128) -> Self {
        Self { data1: (uuid >> 96) as u32, data2: (uuid >> 80 & 0xffff) as u16, data3: (uuid >> 64 & 0xffff) as u16, data4: (uuid as u64).to_be_bytes() }
    }

    pub const fn to_u128(&self) -> u128 {
        (self.data1 as u128) << 96 | (self.data2 as u128) << 80 | (self.data3 as u128) << 64 | u64::from_be_bytes(self.data4) as u128
    }
}

impl From<u128> for GUID {
    fn from(uuid: u128) -> Self {
        GUID::from_u128(uuid)
    }
}

impl From<GUID> for u128 {
    fn from(guid: GUID) -> Self {
        guid.to_u128()
    }
}
