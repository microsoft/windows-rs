#[repr(C)]
#[derive(Clone, Copy)]
pub struct GUID {
    pub data1: u32,
    pub data2: u16,
    pub data3: u16,
    pub data4: [u8; 8],
}
impl GUID {
    pub const fn from_u128(uuid: u128) -> Self {
        Self {
            data1: (uuid >> 96) as u32,
            data2: (uuid >> 80 & 0xffff) as u16,
            data3: (uuid >> 64 & 0xffff) as u16,
            data4: (uuid as u64).to_be_bytes(),
        }
    }
}
pub type HRESULT = i32;
pub mod Windows {
    pub mod Win32 {
        pub mod System {
            pub mod Com {
                pub type CoCreateGuid =
                    unsafe extern "system" fn(
                        pguid: *mut super::super::super::super::GUID,
                    )
                        -> super::super::super::super::HRESULT;
                windows_link::link!("ole32.dll" "system" fn CoCreateGuid(pguid : *mut super::super::super::super::GUID) -> super::super::super::super::HRESULT);
            }
        }
    }
}
