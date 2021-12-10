#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
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
pub type EXIT_WINDOWS_FLAGS = u32;
pub const EWX_HYBRID_SHUTDOWN: EXIT_WINDOWS_FLAGS = 4194304u32;
pub const EWX_LOGOFF: EXIT_WINDOWS_FLAGS = 0u32;
pub const EWX_POWEROFF: EXIT_WINDOWS_FLAGS = 8u32;
pub const EWX_REBOOT: EXIT_WINDOWS_FLAGS = 2u32;
pub const EWX_RESTARTAPPS: EXIT_WINDOWS_FLAGS = 64u32;
pub const EWX_SHUTDOWN: EXIT_WINDOWS_FLAGS = 1u32;
pub const MAX_NUM_REASONS: u32 = 256u32;
pub const MAX_REASON_BUGID_LEN: u32 = 32u32;
pub const MAX_REASON_COMMENT_LEN: u32 = 512u32;
pub const MAX_REASON_DESC_LEN: u32 = 256u32;
pub const MAX_REASON_NAME_LEN: u32 = 64u32;
pub const POLICY_SHOWREASONUI_ALWAYS: u32 = 1u32;
pub const POLICY_SHOWREASONUI_NEVER: u32 = 0u32;
pub const POLICY_SHOWREASONUI_SERVERONLY: u32 = 3u32;
pub const POLICY_SHOWREASONUI_WORKSTATIONONLY: u32 = 2u32;
pub type SHUTDOWN_FLAGS = u32;
pub const SHUTDOWN_FORCE_OTHERS: SHUTDOWN_FLAGS = 1u32;
pub const SHUTDOWN_FORCE_SELF: SHUTDOWN_FLAGS = 2u32;
pub const SHUTDOWN_RESTART: SHUTDOWN_FLAGS = 4u32;
pub const SHUTDOWN_POWEROFF: SHUTDOWN_FLAGS = 8u32;
pub const SHUTDOWN_NOREBOOT: SHUTDOWN_FLAGS = 16u32;
pub const SHUTDOWN_GRACE_OVERRIDE: SHUTDOWN_FLAGS = 32u32;
pub const SHUTDOWN_INSTALL_UPDATES: SHUTDOWN_FLAGS = 64u32;
pub const SHUTDOWN_RESTARTAPPS: SHUTDOWN_FLAGS = 128u32;
pub const SHUTDOWN_SKIP_SVC_PRESHUTDOWN: SHUTDOWN_FLAGS = 256u32;
pub const SHUTDOWN_HYBRID: SHUTDOWN_FLAGS = 512u32;
pub const SHUTDOWN_RESTART_BOOTOPTIONS: SHUTDOWN_FLAGS = 1024u32;
pub const SHUTDOWN_SOFT_REBOOT: SHUTDOWN_FLAGS = 2048u32;
pub const SHUTDOWN_MOBILE_UI: SHUTDOWN_FLAGS = 4096u32;
pub const SHUTDOWN_ARSO: SHUTDOWN_FLAGS = 8192u32;
pub const SHUTDOWN_CHECK_SAFE_FOR_SERVER: SHUTDOWN_FLAGS = 16384u32;
pub const SHUTDOWN_VAIL_CONTAINER: SHUTDOWN_FLAGS = 32768u32;
pub const SHUTDOWN_SYSTEM_INITIATED: SHUTDOWN_FLAGS = 65536u32;
pub type SHUTDOWN_REASON = u32;
pub const SHTDN_REASON_NONE: SHUTDOWN_REASON = 0u32;
pub const SHTDN_REASON_FLAG_COMMENT_REQUIRED: SHUTDOWN_REASON = 16777216u32;
pub const SHTDN_REASON_FLAG_DIRTY_PROBLEM_ID_REQUIRED: SHUTDOWN_REASON = 33554432u32;
pub const SHTDN_REASON_FLAG_CLEAN_UI: SHUTDOWN_REASON = 67108864u32;
pub const SHTDN_REASON_FLAG_DIRTY_UI: SHUTDOWN_REASON = 134217728u32;
pub const SHTDN_REASON_FLAG_MOBILE_UI_RESERVED: SHUTDOWN_REASON = 268435456u32;
pub const SHTDN_REASON_FLAG_USER_DEFINED: SHUTDOWN_REASON = 1073741824u32;
pub const SHTDN_REASON_FLAG_PLANNED: SHUTDOWN_REASON = 2147483648u32;
pub const SHTDN_REASON_MAJOR_OTHER: SHUTDOWN_REASON = 0u32;
pub const SHTDN_REASON_MAJOR_NONE: SHUTDOWN_REASON = 0u32;
pub const SHTDN_REASON_MAJOR_HARDWARE: SHUTDOWN_REASON = 65536u32;
pub const SHTDN_REASON_MAJOR_OPERATINGSYSTEM: SHUTDOWN_REASON = 131072u32;
pub const SHTDN_REASON_MAJOR_SOFTWARE: SHUTDOWN_REASON = 196608u32;
pub const SHTDN_REASON_MAJOR_APPLICATION: SHUTDOWN_REASON = 262144u32;
pub const SHTDN_REASON_MAJOR_SYSTEM: SHUTDOWN_REASON = 327680u32;
pub const SHTDN_REASON_MAJOR_POWER: SHUTDOWN_REASON = 393216u32;
pub const SHTDN_REASON_MAJOR_LEGACY_API: SHUTDOWN_REASON = 458752u32;
pub const SHTDN_REASON_MINOR_OTHER: SHUTDOWN_REASON = 0u32;
pub const SHTDN_REASON_MINOR_NONE: SHUTDOWN_REASON = 255u32;
pub const SHTDN_REASON_MINOR_MAINTENANCE: SHUTDOWN_REASON = 1u32;
pub const SHTDN_REASON_MINOR_INSTALLATION: SHUTDOWN_REASON = 2u32;
pub const SHTDN_REASON_MINOR_UPGRADE: SHUTDOWN_REASON = 3u32;
pub const SHTDN_REASON_MINOR_RECONFIG: SHUTDOWN_REASON = 4u32;
pub const SHTDN_REASON_MINOR_HUNG: SHUTDOWN_REASON = 5u32;
pub const SHTDN_REASON_MINOR_UNSTABLE: SHUTDOWN_REASON = 6u32;
pub const SHTDN_REASON_MINOR_DISK: SHUTDOWN_REASON = 7u32;
pub const SHTDN_REASON_MINOR_PROCESSOR: SHUTDOWN_REASON = 8u32;
pub const SHTDN_REASON_MINOR_NETWORKCARD: SHUTDOWN_REASON = 9u32;
pub const SHTDN_REASON_MINOR_POWER_SUPPLY: SHUTDOWN_REASON = 10u32;
pub const SHTDN_REASON_MINOR_CORDUNPLUGGED: SHUTDOWN_REASON = 11u32;
pub const SHTDN_REASON_MINOR_ENVIRONMENT: SHUTDOWN_REASON = 12u32;
pub const SHTDN_REASON_MINOR_HARDWARE_DRIVER: SHUTDOWN_REASON = 13u32;
pub const SHTDN_REASON_MINOR_OTHERDRIVER: SHUTDOWN_REASON = 14u32;
pub const SHTDN_REASON_MINOR_BLUESCREEN: SHUTDOWN_REASON = 15u32;
pub const SHTDN_REASON_MINOR_SERVICEPACK: SHUTDOWN_REASON = 16u32;
pub const SHTDN_REASON_MINOR_HOTFIX: SHUTDOWN_REASON = 17u32;
pub const SHTDN_REASON_MINOR_SECURITYFIX: SHUTDOWN_REASON = 18u32;
pub const SHTDN_REASON_MINOR_SECURITY: SHUTDOWN_REASON = 19u32;
pub const SHTDN_REASON_MINOR_NETWORK_CONNECTIVITY: SHUTDOWN_REASON = 20u32;
pub const SHTDN_REASON_MINOR_WMI: SHUTDOWN_REASON = 21u32;
pub const SHTDN_REASON_MINOR_SERVICEPACK_UNINSTALL: SHUTDOWN_REASON = 22u32;
pub const SHTDN_REASON_MINOR_HOTFIX_UNINSTALL: SHUTDOWN_REASON = 23u32;
pub const SHTDN_REASON_MINOR_SECURITYFIX_UNINSTALL: SHUTDOWN_REASON = 24u32;
pub const SHTDN_REASON_MINOR_MMC: SHUTDOWN_REASON = 25u32;
pub const SHTDN_REASON_MINOR_SYSTEMRESTORE: SHUTDOWN_REASON = 26u32;
pub const SHTDN_REASON_MINOR_TERMSRV: SHUTDOWN_REASON = 32u32;
pub const SHTDN_REASON_MINOR_DC_PROMOTION: SHUTDOWN_REASON = 33u32;
pub const SHTDN_REASON_MINOR_DC_DEMOTION: SHUTDOWN_REASON = 34u32;
pub const SHTDN_REASON_UNKNOWN: SHUTDOWN_REASON = 255u32;
pub const SHTDN_REASON_LEGACY_API: SHUTDOWN_REASON = 2147942400u32;
pub const SHTDN_REASON_VALID_BIT_MASK: SHUTDOWN_REASON = 3238002687u32;
pub const SHUTDOWN_TYPE_LEN: u32 = 32u32;
pub const SNAPSHOT_POLICY_ALWAYS: u32 = 1u32;
pub const SNAPSHOT_POLICY_NEVER: u32 = 0u32;
pub const SNAPSHOT_POLICY_UNPLANNED: u32 = 2u32;
