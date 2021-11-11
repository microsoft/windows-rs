#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    #[doc = "*Required features: `Win32_Storage_OfflineFiles`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn OfflineFilesEnable();
    #[doc = "*Required features: `Win32_Storage_OfflineFiles`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn OfflineFilesQueryStatus();
    #[doc = "*Required features: `Win32_Storage_OfflineFiles`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn OfflineFilesQueryStatusEx();
    #[doc = "*Required features: `Win32_Storage_OfflineFiles`*"]
    pub fn OfflineFilesStart();
}
