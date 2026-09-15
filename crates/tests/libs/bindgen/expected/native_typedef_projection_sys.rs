windows_link::link!("test.dll" "system" fn GetTime(result : LPSYSTEMTIME));
windows_link::link!("test.dll" "system" fn GetValue(result : LPDWORD) -> HRESULT);
pub type DWORD = u32;
pub type HRESULT = i32;
pub type LPDWORD = *mut DWORD;
pub type LPSYSTEMTIME = *mut SYSTEMTIME;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct SYSTEMTIME {
    pub value: u32,
}
