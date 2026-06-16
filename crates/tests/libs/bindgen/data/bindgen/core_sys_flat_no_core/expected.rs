windows_link::link!("ole32.dll" "system" fn CoCreateGuid(pguid : *mut GUID) -> HRESULT);
#[repr(C)]
#[derive(Clone, Copy)]
pub struct GUID {
    pub data1: u32,
    pub data2: u16,
    pub data3: u16,
    pub data4: [u8; 8],
}
pub type HRESULT = i32;
