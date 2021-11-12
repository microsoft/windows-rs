#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    #[cfg(feature = "Win32_Foundation")]
    pub fn AbortSystemShutdownA(lpmachinename: super::super::Foundation::PSTR) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn AbortSystemShutdownW(lpmachinename: super::super::Foundation::PWSTR) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn CheckForHiberboot(phiberboot: *mut super::super::Foundation::BOOLEAN, bclearflag: super::super::Foundation::BOOLEAN) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn ExitWindowsEx(uflags: EXIT_WINDOWS_FLAGS, dwreason: u32) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn InitiateShutdownA(lpmachinename: super::super::Foundation::PSTR, lpmessage: super::super::Foundation::PSTR, dwgraceperiod: u32, dwshutdownflags: SHUTDOWN_FLAGS, dwreason: SHUTDOWN_REASON) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn InitiateShutdownW(lpmachinename: super::super::Foundation::PWSTR, lpmessage: super::super::Foundation::PWSTR, dwgraceperiod: u32, dwshutdownflags: SHUTDOWN_FLAGS, dwreason: SHUTDOWN_REASON) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn InitiateSystemShutdownA(lpmachinename: super::super::Foundation::PSTR, lpmessage: super::super::Foundation::PSTR, dwtimeout: u32, bforceappsclosed: super::super::Foundation::BOOL, brebootaftershutdown: super::super::Foundation::BOOL) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn InitiateSystemShutdownExA(lpmachinename: super::super::Foundation::PSTR, lpmessage: super::super::Foundation::PSTR, dwtimeout: u32, bforceappsclosed: super::super::Foundation::BOOL, brebootaftershutdown: super::super::Foundation::BOOL, dwreason: SHUTDOWN_REASON) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn InitiateSystemShutdownExW(lpmachinename: super::super::Foundation::PWSTR, lpmessage: super::super::Foundation::PWSTR, dwtimeout: u32, bforceappsclosed: super::super::Foundation::BOOL, brebootaftershutdown: super::super::Foundation::BOOL, dwreason: SHUTDOWN_REASON) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn InitiateSystemShutdownW(lpmachinename: super::super::Foundation::PWSTR, lpmessage: super::super::Foundation::PWSTR, dwtimeout: u32, bforceappsclosed: super::super::Foundation::BOOL, brebootaftershutdown: super::super::Foundation::BOOL) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn LockWorkStation() -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn ShutdownBlockReasonCreate(hwnd: super::super::Foundation::HWND, pwszreason: super::super::Foundation::PWSTR) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn ShutdownBlockReasonDestroy(hwnd: super::super::Foundation::HWND) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn ShutdownBlockReasonQuery(hwnd: super::super::Foundation::HWND, pwszbuff: super::super::Foundation::PWSTR, pcchbuff: *mut u32) -> super::super::Foundation::BOOL;
}
pub struct EXIT_WINDOWS_FLAGS(i32);
pub const MAX_NUM_REASONS: u32 = 256u32;
pub const MAX_REASON_BUGID_LEN: u32 = 32u32;
pub const MAX_REASON_COMMENT_LEN: u32 = 512u32;
pub const MAX_REASON_DESC_LEN: u32 = 256u32;
pub const MAX_REASON_NAME_LEN: u32 = 64u32;
pub const POLICY_SHOWREASONUI_ALWAYS: u32 = 1u32;
pub const POLICY_SHOWREASONUI_NEVER: u32 = 0u32;
pub const POLICY_SHOWREASONUI_SERVERONLY: u32 = 3u32;
pub const POLICY_SHOWREASONUI_WORKSTATIONONLY: u32 = 2u32;
pub struct SHUTDOWN_FLAGS(i32);
pub struct SHUTDOWN_REASON(i32);
pub const SHUTDOWN_TYPE_LEN: u32 = 32u32;
pub const SNAPSHOT_POLICY_ALWAYS: u32 = 1u32;
pub const SNAPSHOT_POLICY_NEVER: u32 = 0u32;
pub const SNAPSHOT_POLICY_UNPLANNED: u32 = 2u32;
