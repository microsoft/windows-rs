#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    #[doc = "*Required features: `Win32_System_Restore`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SRSetRestorePointA(prestoreptspec: *const RESTOREPOINTINFOA, psmgrstatus: *mut STATEMGRSTATUS) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_Restore`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SRSetRestorePointW(prestoreptspec: *const RESTOREPOINTINFOW, psmgrstatus: *mut STATEMGRSTATUS) -> super::super::Foundation::BOOL;
}
#[doc = "*Required features: `Win32_System_Restore`*"]
pub const ACCESSIBILITY_SETTING: u32 = 3u32;
#[doc = "*Required features: `Win32_System_Restore`*"]
pub const APPLICATION_RUN: u32 = 5u32;
#[doc = "*Required features: `Win32_System_Restore`*"]
pub const BACKUP: u32 = 15u32;
#[doc = "*Required features: `Win32_System_Restore`*"]
pub const BACKUP_RECOVERY: u32 = 14u32;
#[doc = "*Required features: `Win32_System_Restore`*"]
pub const BEGIN_NESTED_SYSTEM_CHANGE_NORP: u32 = 104u32;
#[doc = "*Required features: `Win32_System_Restore`*"]
pub const CHECKPOINT: u32 = 7u32;
#[doc = "*Required features: `Win32_System_Restore`*"]
pub const CRITICAL_UPDATE: u32 = 18u32;
#[doc = "*Required features: `Win32_System_Restore`*"]
pub const DESKTOP_SETTING: u32 = 2u32;
#[doc = "*Required features: `Win32_System_Restore`*"]
pub const FIRSTRUN: u32 = 11u32;
#[doc = "*Required features: `Win32_System_Restore`*"]
pub const MANUAL_CHECKPOINT: u32 = 16u32;
#[doc = "*Required features: `Win32_System_Restore`*"]
pub const MAX_DESC: u32 = 64u32;
#[doc = "*Required features: `Win32_System_Restore`*"]
pub const MAX_DESC_W: u32 = 256u32;
#[doc = "*Required features: `Win32_System_Restore`*"]
pub const MAX_EVENT: u32 = 104u32;
#[doc = "*Required features: `Win32_System_Restore`*"]
pub const MAX_RPT: u32 = 18u32;
#[doc = "*Required features: `Win32_System_Restore`*"]
pub const MIN_EVENT: u32 = 100u32;
#[doc = "*Required features: `Win32_System_Restore`*"]
pub const MIN_RPT: u32 = 0u32;
#[doc = "*Required features: `Win32_System_Restore`*"]
pub const OE_SETTING: u32 = 4u32;
#[doc = "*Required features: `Win32_System_Restore`*"]
pub const RESTORE: u32 = 6u32;
#[cfg(feature = "Win32_Foundation")]
pub struct RESTOREPOINTINFOA(i32);
pub struct RESTOREPOINTINFOW(i32);
pub struct RESTOREPOINTINFO_EVENT_TYPE(i32);
pub struct RESTOREPOINTINFO_TYPE(i32);
pub struct STATEMGRSTATUS(i32);
#[doc = "*Required features: `Win32_System_Restore`*"]
pub const WINDOWS_BOOT: u32 = 9u32;
#[doc = "*Required features: `Win32_System_Restore`*"]
pub const WINDOWS_SHUTDOWN: u32 = 8u32;
#[doc = "*Required features: `Win32_System_Restore`*"]
pub const WINDOWS_UPDATE: u32 = 17u32;
#[cfg(feature = "Win32_Foundation")]
pub struct _RESTOREPTINFOEX(i32);
