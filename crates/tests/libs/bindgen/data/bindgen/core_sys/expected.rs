#[repr(C)]
#[derive(Clone, Copy)]
pub struct GUID {
    pub data1: u32,
    pub data2: u16,
    pub data3: u16,
    pub data4: [u8; 8],
}
pub type HRESULT = i32;
pub mod Windows {
    pub mod Win32 {
        pub mod System {
            pub mod Com {
                windows_link::link!("ole32.dll" "system" fn CoCreateGuid(pguid : *mut super::super::super::super::GUID) -> super::super::super::super::HRESULT);
            }
        }
    }
}
