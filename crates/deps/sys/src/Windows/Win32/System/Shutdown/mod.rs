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
pub const EWX_HYBRID_SHUTDOWN: u32 = 4194304u32;
pub const EWX_LOGOFF: u32 = 0u32;
pub const EWX_POWEROFF: u32 = 8u32;
pub const EWX_REBOOT: u32 = 2u32;
pub const EWX_RESTARTAPPS: u32 = 64u32;
pub const EWX_SHUTDOWN: u32 = 1u32;
pub const MAX_NUM_REASONS: u32 = 256u32;
pub const MAX_REASON_BUGID_LEN: u32 = 32u32;
pub const MAX_REASON_COMMENT_LEN: u32 = 512u32;
pub const MAX_REASON_DESC_LEN: u32 = 256u32;
pub const MAX_REASON_NAME_LEN: u32 = 64u32;
pub const POLICY_SHOWREASONUI_ALWAYS: u32 = 1u32;
pub const POLICY_SHOWREASONUI_NEVER: u32 = 0u32;
pub const POLICY_SHOWREASONUI_SERVERONLY: u32 = 3u32;
pub const POLICY_SHOWREASONUI_WORKSTATIONONLY: u32 = 2u32;
pub const SHUTDOWN_FORCE_OTHERS: u32 = 1u32;
pub const SHUTDOWN_FORCE_SELF: u32 = 2u32;
pub const SHUTDOWN_RESTART: u32 = 4u32;
pub const SHUTDOWN_POWEROFF: u32 = 8u32;
pub const SHUTDOWN_NOREBOOT: u32 = 16u32;
pub const SHUTDOWN_GRACE_OVERRIDE: u32 = 32u32;
pub const SHUTDOWN_INSTALL_UPDATES: u32 = 64u32;
pub const SHUTDOWN_RESTARTAPPS: u32 = 128u32;
pub const SHUTDOWN_SKIP_SVC_PRESHUTDOWN: u32 = 256u32;
pub const SHUTDOWN_HYBRID: u32 = 512u32;
pub const SHUTDOWN_RESTART_BOOTOPTIONS: u32 = 1024u32;
pub const SHUTDOWN_SOFT_REBOOT: u32 = 2048u32;
pub const SHUTDOWN_MOBILE_UI: u32 = 4096u32;
pub const SHUTDOWN_ARSO: u32 = 8192u32;
pub const SHUTDOWN_CHECK_SAFE_FOR_SERVER: u32 = 16384u32;
pub const SHUTDOWN_VAIL_CONTAINER: u32 = 32768u32;
pub const SHUTDOWN_SYSTEM_INITIATED: u32 = 65536u32;
pub const SHTDN_REASON_NONE: u32 = 0u32;
pub const SHTDN_REASON_FLAG_COMMENT_REQUIRED: u32 = 16777216u32;
pub const SHTDN_REASON_FLAG_DIRTY_PROBLEM_ID_REQUIRED: u32 = 33554432u32;
pub const SHTDN_REASON_FLAG_CLEAN_UI: u32 = 67108864u32;
pub const SHTDN_REASON_FLAG_DIRTY_UI: u32 = 134217728u32;
pub const SHTDN_REASON_FLAG_MOBILE_UI_RESERVED: u32 = 268435456u32;
pub const SHTDN_REASON_FLAG_USER_DEFINED: u32 = 1073741824u32;
pub const SHTDN_REASON_FLAG_PLANNED: u32 = 2147483648u32;
pub const SHTDN_REASON_MAJOR_OTHER: u32 = 0u32;
pub const SHTDN_REASON_MAJOR_NONE: u32 = 0u32;
pub const SHTDN_REASON_MAJOR_HARDWARE: u32 = 65536u32;
pub const SHTDN_REASON_MAJOR_OPERATINGSYSTEM: u32 = 131072u32;
pub const SHTDN_REASON_MAJOR_SOFTWARE: u32 = 196608u32;
pub const SHTDN_REASON_MAJOR_APPLICATION: u32 = 262144u32;
pub const SHTDN_REASON_MAJOR_SYSTEM: u32 = 327680u32;
pub const SHTDN_REASON_MAJOR_POWER: u32 = 393216u32;
pub const SHTDN_REASON_MAJOR_LEGACY_API: u32 = 458752u32;
pub const SHTDN_REASON_MINOR_OTHER: u32 = 0u32;
pub const SHTDN_REASON_MINOR_NONE: u32 = 255u32;
pub const SHTDN_REASON_MINOR_MAINTENANCE: u32 = 1u32;
pub const SHTDN_REASON_MINOR_INSTALLATION: u32 = 2u32;
pub const SHTDN_REASON_MINOR_UPGRADE: u32 = 3u32;
pub const SHTDN_REASON_MINOR_RECONFIG: u32 = 4u32;
pub const SHTDN_REASON_MINOR_HUNG: u32 = 5u32;
pub const SHTDN_REASON_MINOR_UNSTABLE: u32 = 6u32;
pub const SHTDN_REASON_MINOR_DISK: u32 = 7u32;
pub const SHTDN_REASON_MINOR_PROCESSOR: u32 = 8u32;
pub const SHTDN_REASON_MINOR_NETWORKCARD: u32 = 9u32;
pub const SHTDN_REASON_MINOR_POWER_SUPPLY: u32 = 10u32;
pub const SHTDN_REASON_MINOR_CORDUNPLUGGED: u32 = 11u32;
pub const SHTDN_REASON_MINOR_ENVIRONMENT: u32 = 12u32;
pub const SHTDN_REASON_MINOR_HARDWARE_DRIVER: u32 = 13u32;
pub const SHTDN_REASON_MINOR_OTHERDRIVER: u32 = 14u32;
pub const SHTDN_REASON_MINOR_BLUESCREEN: u32 = 15u32;
pub const SHTDN_REASON_MINOR_SERVICEPACK: u32 = 16u32;
pub const SHTDN_REASON_MINOR_HOTFIX: u32 = 17u32;
pub const SHTDN_REASON_MINOR_SECURITYFIX: u32 = 18u32;
pub const SHTDN_REASON_MINOR_SECURITY: u32 = 19u32;
pub const SHTDN_REASON_MINOR_NETWORK_CONNECTIVITY: u32 = 20u32;
pub const SHTDN_REASON_MINOR_WMI: u32 = 21u32;
pub const SHTDN_REASON_MINOR_SERVICEPACK_UNINSTALL: u32 = 22u32;
pub const SHTDN_REASON_MINOR_HOTFIX_UNINSTALL: u32 = 23u32;
pub const SHTDN_REASON_MINOR_SECURITYFIX_UNINSTALL: u32 = 24u32;
pub const SHTDN_REASON_MINOR_MMC: u32 = 25u32;
pub const SHTDN_REASON_MINOR_SYSTEMRESTORE: u32 = 26u32;
pub const SHTDN_REASON_MINOR_TERMSRV: u32 = 32u32;
pub const SHTDN_REASON_MINOR_DC_PROMOTION: u32 = 33u32;
pub const SHTDN_REASON_MINOR_DC_DEMOTION: u32 = 34u32;
pub const SHTDN_REASON_UNKNOWN: u32 = 255u32;
pub const SHTDN_REASON_LEGACY_API: u32 = 2147942400u32;
pub const SHTDN_REASON_VALID_BIT_MASK: u32 = 3238002687u32;
pub const SHUTDOWN_TYPE_LEN: u32 = 32u32;
pub const SNAPSHOT_POLICY_ALWAYS: u32 = 1u32;
pub const SNAPSHOT_POLICY_NEVER: u32 = 0u32;
pub const SNAPSHOT_POLICY_UNPLANNED: u32 = 2u32;
