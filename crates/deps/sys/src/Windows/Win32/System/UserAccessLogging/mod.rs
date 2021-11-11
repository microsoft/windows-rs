#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    #[doc = "*Required features: `Win32_System_UserAccessLogging`, `Win32_Foundation`, `Win32_Networking_WinSock`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
    pub fn UalInstrument();
    #[doc = "*Required features: `Win32_System_UserAccessLogging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn UalRegisterProduct();
    #[doc = "*Required features: `Win32_System_UserAccessLogging`, `Win32_Foundation`, `Win32_Networking_WinSock`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
    pub fn UalStart();
    #[doc = "*Required features: `Win32_System_UserAccessLogging`, `Win32_Foundation`, `Win32_Networking_WinSock`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
    pub fn UalStop();
}
