windows_link::link!("quartz.dll" "system" fn AMGetErrorTextA(hr : windows_sys::core::HRESULT, pbuffer : windows_sys::core::PSTR, maxlen : u32) -> u32);
windows_link::link!("quartz.dll" "system" fn AMGetErrorTextW(hr : windows_sys::core::HRESULT, pbuffer : windows_sys::core::PWSTR, maxlen : u32) -> u32);
pub type AMGETERRORTEXTPROC = AMGETERRORTEXTPROCA;
pub type AMGETERRORTEXTPROCA = Option<unsafe extern "system" fn(param0: windows_sys::core::HRESULT, param1: *mut i8, param2: u32) -> windows_sys::core::BOOL>;
pub type AMGETERRORTEXTPROCW = Option<unsafe extern "system" fn(param0: windows_sys::core::HRESULT, param1: *mut u16, param2: u32) -> windows_sys::core::BOOL>;
pub const MAX_ERROR_TEXT_LEN: u32 = 160;
pub const VFW_FIRST_CODE: u32 = 512;
