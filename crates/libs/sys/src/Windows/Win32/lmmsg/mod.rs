windows_link::link!("netapi32.dll" "system" fn NetMessageBufferSend(servername : windows_sys::core::PCWSTR, msgname : windows_sys::core::PCWSTR, fromname : windows_sys::core::PCWSTR, buf : *const u8, buflen : u32) -> u32);
windows_link::link!("netapi32.dll" "system" fn NetMessageNameAdd(servername : windows_sys::core::PCWSTR, msgname : windows_sys::core::PCWSTR) -> u32);
windows_link::link!("netapi32.dll" "system" fn NetMessageNameDel(servername : windows_sys::core::PCWSTR, msgname : windows_sys::core::PCWSTR) -> u32);
#[cfg(feature = "Win32_minwindef")]
windows_link::link!("netapi32.dll" "system" fn NetMessageNameEnum(servername : windows_sys::core::PCWSTR, level : u32, bufptr : *const super::minwindef::LPBYTE, prefmaxlen : u32, entriesread : *mut u32, totalentries : *mut u32, resume_handle : *mut u32) -> u32);
#[cfg(feature = "Win32_minwindef")]
windows_link::link!("netapi32.dll" "system" fn NetMessageNameGetInfo(servername : windows_sys::core::PCWSTR, msgname : windows_sys::core::PCWSTR, level : u32, bufptr : *const super::minwindef::LPBYTE) -> u32);
pub type LPMSG_INFO_0 = *mut MSG_INFO_0;
pub type LPMSG_INFO_1 = *mut MSG_INFO_1;
pub const MSGNAME_FORWARDED_FROM: u32 = 16;
pub const MSGNAME_FORWARDED_TO: u32 = 4;
pub const MSGNAME_NOT_FORWARDED: u32 = 0;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct MSG_INFO_0 {
    pub msgi0_name: windows_sys::core::PWSTR,
}
impl Default for MSG_INFO_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct MSG_INFO_1 {
    pub msgi1_name: windows_sys::core::PWSTR,
    pub msgi1_forward_flag: u32,
    pub msgi1_forward: windows_sys::core::PWSTR,
}
impl Default for MSG_INFO_1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type PMSG_INFO_0 = *mut MSG_INFO_0;
pub type PMSG_INFO_1 = *mut MSG_INFO_1;
