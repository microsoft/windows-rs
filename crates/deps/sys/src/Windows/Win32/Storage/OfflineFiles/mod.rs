#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    #[doc = "*Required features: `Win32_Storage_OfflineFiles`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn OfflineFilesEnable(benable: super::super::Foundation::BOOL, pbrebootrequired: *mut super::super::Foundation::BOOL) -> u32;
    #[doc = "*Required features: `Win32_Storage_OfflineFiles`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn OfflineFilesQueryStatus(pbactive: *mut super::super::Foundation::BOOL, pbenabled: *mut super::super::Foundation::BOOL) -> u32;
    #[doc = "*Required features: `Win32_Storage_OfflineFiles`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn OfflineFilesQueryStatusEx(pbactive: *mut super::super::Foundation::BOOL, pbenabled: *mut super::super::Foundation::BOOL, pbavailable: *mut super::super::Foundation::BOOL) -> u32;
    #[doc = "*Required features: `Win32_Storage_OfflineFiles`*"]
    pub fn OfflineFilesStart() -> u32;
}
