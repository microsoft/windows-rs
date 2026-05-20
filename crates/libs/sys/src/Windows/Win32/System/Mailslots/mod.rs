#[cfg(feature = "Win32_Security")]
pub type CreateMailslotA = unsafe extern "system" fn(lpname: windows_sys::core::PCSTR, nmaxmessagesize: u32, lreadtimeout: u32, lpsecurityattributes: *const super::super::Security::SECURITY_ATTRIBUTES) -> super::super::Foundation::HANDLE;
#[cfg(feature = "Win32_Security")]
windows_link::link!("kernel32.dll" "system" fn CreateMailslotA(lpname : windows_sys::core::PCSTR, nmaxmessagesize : u32, lreadtimeout : u32, lpsecurityattributes : *const super::super::Security::SECURITY_ATTRIBUTES) -> super::super::Foundation::HANDLE);
#[cfg(feature = "Win32_Security")]
pub type CreateMailslotW = unsafe extern "system" fn(lpname: windows_sys::core::PCWSTR, nmaxmessagesize: u32, lreadtimeout: u32, lpsecurityattributes: *const super::super::Security::SECURITY_ATTRIBUTES) -> super::super::Foundation::HANDLE;
#[cfg(feature = "Win32_Security")]
windows_link::link!("kernel32.dll" "system" fn CreateMailslotW(lpname : windows_sys::core::PCWSTR, nmaxmessagesize : u32, lreadtimeout : u32, lpsecurityattributes : *const super::super::Security::SECURITY_ATTRIBUTES) -> super::super::Foundation::HANDLE);
pub type GetMailslotInfo = unsafe extern "system" fn(hmailslot: super::super::Foundation::HANDLE, lpmaxmessagesize: *mut u32, lpnextsize: *mut u32, lpmessagecount: *mut u32, lpreadtimeout: *mut u32) -> windows_sys::core::BOOL;
windows_link::link!("kernel32.dll" "system" fn GetMailslotInfo(hmailslot : super::super::Foundation::HANDLE, lpmaxmessagesize : *mut u32, lpnextsize : *mut u32, lpmessagecount : *mut u32, lpreadtimeout : *mut u32) -> windows_sys::core::BOOL);
pub type SetMailslotInfo = unsafe extern "system" fn(hmailslot: super::super::Foundation::HANDLE, lreadtimeout: u32) -> windows_sys::core::BOOL;
windows_link::link!("kernel32.dll" "system" fn SetMailslotInfo(hmailslot : super::super::Foundation::HANDLE, lreadtimeout : u32) -> windows_sys::core::BOOL);
