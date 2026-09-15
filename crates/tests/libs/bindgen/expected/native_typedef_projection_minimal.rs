windows_core::link!("test.dll" "system" fn GetTime(result : LPSYSTEMTIME));
windows_core::link!("test.dll" "system" fn GetValue(result : LPDWORD) -> windows_core::HRESULT);
pub type DWORD = u32;
pub type LPDWORD = *mut DWORD;
pub type LPSYSTEMTIME = *mut SYSTEMTIME;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SYSTEMTIME {
    pub value: u32,
}
