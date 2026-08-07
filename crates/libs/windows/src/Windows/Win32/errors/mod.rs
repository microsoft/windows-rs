#[inline]
pub unsafe fn AMGetErrorTextA(hr: windows_core::HRESULT, pbuffer: windows_core::PSTR, maxlen: u32) -> u32 {
    windows_core::link!("quartz.dll" "system" fn AMGetErrorTextA(hr : windows_core::HRESULT, pbuffer : windows_core::PSTR, maxlen : u32) -> u32);
    unsafe { AMGetErrorTextA(hr, pbuffer, maxlen) }
}
#[inline]
pub unsafe fn AMGetErrorTextW(hr: windows_core::HRESULT, pbuffer: windows_core::PWSTR, maxlen: u32) -> u32 {
    windows_core::link!("quartz.dll" "system" fn AMGetErrorTextW(hr : windows_core::HRESULT, pbuffer : windows_core::PWSTR, maxlen : u32) -> u32);
    unsafe { AMGetErrorTextW(hr, pbuffer, maxlen) }
}
pub type AMGETERRORTEXTPROC = AMGETERRORTEXTPROCA;
pub type AMGETERRORTEXTPROCA = Option<unsafe extern "system" fn(param0: windows_core::HRESULT, param1: *mut i8, param2: u32) -> windows_core::BOOL>;
pub type AMGETERRORTEXTPROCW = Option<unsafe extern "system" fn(param0: windows_core::HRESULT, param1: *mut u16, param2: u32) -> windows_core::BOOL>;
pub const MAX_ERROR_TEXT_LEN: i32 = 160;
pub const VFW_FIRST_CODE: i32 = 512;
