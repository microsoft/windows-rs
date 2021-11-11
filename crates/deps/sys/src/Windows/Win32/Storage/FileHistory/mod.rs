#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    #[doc = "*Required features: `Win32_Storage_FileHistory`, `Win32_System_WindowsProgramming`*"]
    #[cfg(feature = "Win32_System_WindowsProgramming")]
    pub fn FhServiceBlockBackup();
    #[doc = "*Required features: `Win32_Storage_FileHistory`, `Win32_System_WindowsProgramming`*"]
    #[cfg(feature = "Win32_System_WindowsProgramming")]
    pub fn FhServiceClosePipe();
    #[doc = "*Required features: `Win32_Storage_FileHistory`, `Win32_Foundation`, `Win32_System_WindowsProgramming`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_WindowsProgramming"))]
    pub fn FhServiceOpenPipe();
    #[doc = "*Required features: `Win32_Storage_FileHistory`, `Win32_System_WindowsProgramming`*"]
    #[cfg(feature = "Win32_System_WindowsProgramming")]
    pub fn FhServiceReloadConfiguration();
    #[doc = "*Required features: `Win32_Storage_FileHistory`, `Win32_Foundation`, `Win32_System_WindowsProgramming`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_WindowsProgramming"))]
    pub fn FhServiceStartBackup();
    #[doc = "*Required features: `Win32_Storage_FileHistory`, `Win32_Foundation`, `Win32_System_WindowsProgramming`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_WindowsProgramming"))]
    pub fn FhServiceStopBackup();
    #[doc = "*Required features: `Win32_Storage_FileHistory`, `Win32_System_WindowsProgramming`*"]
    #[cfg(feature = "Win32_System_WindowsProgramming")]
    pub fn FhServiceUnblockBackup();
}
