#[cfg(feature = "shared")]
windows_link::link!("netapi32.dll" "system" fn NetMessageBufferSend(servername : windows_sys::core::PCWSTR, msgname : windows_sys::core::PCWSTR, fromname : windows_sys::core::PCWSTR, buf : super::LPBYTE, buflen : super::DWORD) -> super::DWORD);
#[cfg(feature = "shared")]
windows_link::link!("netapi32.dll" "system" fn NetMessageNameAdd(servername : windows_sys::core::PCWSTR, msgname : windows_sys::core::PCWSTR) -> super::DWORD);
#[cfg(feature = "shared")]
windows_link::link!("netapi32.dll" "system" fn NetMessageNameDel(servername : windows_sys::core::PCWSTR, msgname : windows_sys::core::PCWSTR) -> super::DWORD);
#[cfg(all(feature = "minwindef", feature = "shared"))]
windows_link::link!("netapi32.dll" "system" fn NetMessageNameEnum(servername : windows_sys::core::PCWSTR, level : super::DWORD, bufptr : *const super::LPBYTE, prefmaxlen : super::DWORD, entriesread : super::LPDWORD, totalentries : super::LPDWORD, resume_handle : super::LPDWORD) -> super::DWORD);
#[cfg(feature = "shared")]
windows_link::link!("netapi32.dll" "system" fn NetMessageNameGetInfo(servername : windows_sys::core::PCWSTR, msgname : windows_sys::core::PCWSTR, level : super::DWORD, bufptr : *const super::LPBYTE) -> super::DWORD);
pub type LPMSG_INFO_0 = *mut MSG_INFO_0;
#[cfg(feature = "shared")]
pub type LPMSG_INFO_1 = *mut MSG_INFO_1;
pub const MSGNAME_FORWARDED_FROM: i32 = 16;
pub const MSGNAME_FORWARDED_TO: i32 = 4;
pub const MSGNAME_NOT_FORWARDED: i32 = 0;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct MSG_INFO_0 {
    pub msgi0_name: windows_sys::core::PWSTR,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct MSG_INFO_1 {
    pub msgi1_name: windows_sys::core::PWSTR,
    pub msgi1_forward_flag: super::DWORD,
    pub msgi1_forward: windows_sys::core::PWSTR,
}
pub type PMSG_INFO_0 = *mut MSG_INFO_0;
#[cfg(feature = "shared")]
pub type PMSG_INFO_1 = *mut MSG_INFO_1;
