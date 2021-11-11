#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsNetworkVirtualization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WnvOpen();
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsNetworkVirtualization`, `Win32_Foundation`, `Win32_System_IO`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
    pub fn WnvRequestNotification();
}
