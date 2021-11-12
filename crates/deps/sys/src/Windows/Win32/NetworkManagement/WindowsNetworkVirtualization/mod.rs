#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    #[cfg(feature = "Win32_Foundation")]
    pub fn WnvOpen() -> super::super::Foundation::HANDLE;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
    pub fn WnvRequestNotification(wnvhandle: super::super::Foundation::HANDLE, notificationparam: *mut WNV_NOTIFICATION_PARAM, overlapped: *mut super::super::System::IO::OVERLAPPED, bytestransferred: *mut u32) -> u32;
}
pub const WNV_API_MAJOR_VERSION_1: u32 = 1u32;
pub const WNV_API_MINOR_VERSION_0: u32 = 0u32;
pub struct WNV_CA_NOTIFICATION_TYPE(i32);
#[cfg(all(feature = "Win32_NetworkManagement_WindowsFilteringPlatform", feature = "Win32_Networking_WinSock"))]
pub struct WNV_CUSTOMER_ADDRESS_CHANGE_PARAM(i32);
#[cfg(feature = "Win32_Networking_WinSock")]
pub struct WNV_IP_ADDRESS(i32);
pub struct WNV_NOTIFICATION_PARAM(i32);
pub struct WNV_NOTIFICATION_TYPE(i32);
#[cfg(all(feature = "Win32_NetworkManagement_WindowsFilteringPlatform", feature = "Win32_Networking_WinSock"))]
pub struct WNV_OBJECT_CHANGE_PARAM(i32);
pub struct WNV_OBJECT_HEADER(i32);
pub struct WNV_OBJECT_TYPE(i32);
#[cfg(feature = "Win32_Networking_WinSock")]
pub struct WNV_POLICY_MISMATCH_PARAM(i32);
#[cfg(feature = "Win32_Networking_WinSock")]
pub struct WNV_PROVIDER_ADDRESS_CHANGE_PARAM(i32);
#[cfg(feature = "Win32_Networking_WinSock")]
pub struct WNV_REDIRECT_PARAM(i32);
