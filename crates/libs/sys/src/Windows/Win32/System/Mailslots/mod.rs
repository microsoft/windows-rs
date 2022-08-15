#[cfg_attr(windows, link(name = "windows"))]
extern "system" {
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub fn CreateMailslotA(lpname: ::windows_sys::core::PCSTR, nmaxmessagesize: u32, lreadtimeout: u32, lpsecurityattributes: *const super::super::Security::SECURITY_ATTRIBUTES) -> super::super::Foundation::HANDLE;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub fn CreateMailslotW(lpname: ::windows_sys::core::PCWSTR, nmaxmessagesize: u32, lreadtimeout: u32, lpsecurityattributes: *const super::super::Security::SECURITY_ATTRIBUTES) -> super::super::Foundation::HANDLE;
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetMailslotInfo(hmailslot: super::super::Foundation::HANDLE, lpmaxmessagesize: *mut u32, lpnextsize: *mut u32, lpmessagecount: *mut u32, lpreadtimeout: *mut u32) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetMailslotInfo(hmailslot: super::super::Foundation::HANDLE, lreadtimeout: u32) -> super::super::Foundation::BOOL;
}
