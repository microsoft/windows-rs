#[cfg(feature = "shared")]
#[inline]
pub unsafe fn AMGetErrorTextA(hr: windows_core::HRESULT, pbuffer: windows_core::PSTR, maxlen: super::DWORD) -> super::DWORD {
    windows_core::link!("quartz.dll" "system" fn AMGetErrorTextA(hr : windows_core::HRESULT, pbuffer : windows_core::PSTR, maxlen : super::DWORD) -> super::DWORD);
    unsafe { AMGetErrorTextA(hr, pbuffer, maxlen) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn AMGetErrorTextW(hr: windows_core::HRESULT, pbuffer: windows_core::PWSTR, maxlen: super::DWORD) -> super::DWORD {
    windows_core::link!("quartz.dll" "system" fn AMGetErrorTextW(hr : windows_core::HRESULT, pbuffer : windows_core::PWSTR, maxlen : super::DWORD) -> super::DWORD);
    unsafe { AMGetErrorTextW(hr, pbuffer, maxlen) }
}
#[cfg(feature = "shared")]
pub type AMGETERRORTEXTPROC = Option<unsafe extern "system" fn(param0: windows_core::HRESULT, param1: *mut i8, param2: super::DWORD) -> windows_core::BOOL>;
#[cfg(feature = "shared")]
pub type AMGETERRORTEXTPROCA = Option<unsafe extern "system" fn(param0: windows_core::HRESULT, param1: *mut i8, param2: super::DWORD) -> windows_core::BOOL>;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type AMGETERRORTEXTPROCW = Option<unsafe extern "system" fn(param0: windows_core::HRESULT, param1: *mut super::WCHAR, param2: super::DWORD) -> windows_core::BOOL>;
pub const MAX_ERROR_TEXT_LEN: i32 = 160;
pub const VFW_FIRST_CODE: i32 = 512;
