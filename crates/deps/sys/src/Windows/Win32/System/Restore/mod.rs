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
