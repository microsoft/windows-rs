#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsNetworkVirtualization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WnvOpen() -> super::super::Foundation::HANDLE;
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsNetworkVirtualization`, `Win32_Foundation`, `Win32_System_IO`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
    pub fn WnvRequestNotification(wnvhandle: super::super::Foundation::HANDLE, notificationparam: *mut WNV_NOTIFICATION_PARAM, overlapped: *mut super::super::System::IO::OVERLAPPED, bytestransferred: *mut u32) -> u32;
}
