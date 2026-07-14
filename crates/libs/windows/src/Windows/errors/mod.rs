#[inline]
pub unsafe fn AMGetErrorTextA(hr: windows_core::HRESULT, pbuffer: &mut [u8]) -> u32 {
    windows_core::link!("quartz.dll" "system" fn AMGetErrorTextA(hr : windows_core::HRESULT, pbuffer : windows_core::PSTR, maxlen : u32) -> u32);
    unsafe { AMGetErrorTextA(hr, core::mem::transmute(pbuffer.as_mut_ptr()), pbuffer.len().try_into().unwrap()) }
}
#[inline]
pub unsafe fn AMGetErrorTextW(hr: windows_core::HRESULT, pbuffer: &mut [u16]) -> u32 {
    windows_core::link!("quartz.dll" "system" fn AMGetErrorTextW(hr : windows_core::HRESULT, pbuffer : windows_core::PWSTR, maxlen : u32) -> u32);
    unsafe { AMGetErrorTextW(hr, core::mem::transmute(pbuffer.as_mut_ptr()), pbuffer.len().try_into().unwrap()) }
}
pub type AMGETERRORTEXTPROC = AMGETERRORTEXTPROCA;
pub type AMGETERRORTEXTPROCA = Option<unsafe extern "system" fn(param0: windows_core::HRESULT, param1: *mut i8, param2: u32) -> windows_core::BOOL>;
pub type AMGETERRORTEXTPROCW = Option<unsafe extern "system" fn(param0: windows_core::HRESULT, param1: *mut u16, param2: u32) -> windows_core::BOOL>;
pub const MAX_ERROR_TEXT_LEN: u32 = 160;
pub const VFW_FIRST_CODE: u32 = 512;
