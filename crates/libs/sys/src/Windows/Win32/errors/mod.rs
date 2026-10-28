#[cfg(feature = "shared")]
windows_link::link!("quartz.dll" "system" fn AMGetErrorTextA(hr : windows_sys::core::HRESULT, pbuffer : windows_sys::core::PSTR, maxlen : super::DWORD) -> super::DWORD);
#[cfg(feature = "shared")]
windows_link::link!("quartz.dll" "system" fn AMGetErrorTextW(hr : windows_sys::core::HRESULT, pbuffer : windows_sys::core::PWSTR, maxlen : super::DWORD) -> super::DWORD);
#[cfg(feature = "shared")]
pub type AMGETERRORTEXTPROC = Option<unsafe extern "system" fn(param0: windows_sys::core::HRESULT, param1: *mut i8, param2: super::DWORD) -> windows_sys::core::BOOL>;
#[cfg(feature = "shared")]
pub type AMGETERRORTEXTPROCA = Option<unsafe extern "system" fn(param0: windows_sys::core::HRESULT, param1: *mut i8, param2: super::DWORD) -> windows_sys::core::BOOL>;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type AMGETERRORTEXTPROCW = Option<unsafe extern "system" fn(param0: windows_sys::core::HRESULT, param1: *mut super::WCHAR, param2: super::DWORD) -> windows_sys::core::BOOL>;
pub const MAX_ERROR_TEXT_LEN: i32 = 160;
pub const VFW_FIRST_CODE: i32 = 512;
