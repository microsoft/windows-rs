#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[link(name = "windows")]
extern "system" {
    #[cfg(feature = "Win32_Foundation")]
    pub fn WnvOpen() -> super::super::Foundation::HANDLE;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
    pub fn WnvRequestNotification(wnvhandle: super::super::Foundation::HANDLE, notificationparam: *mut WNV_NOTIFICATION_PARAM, overlapped: *mut super::super::System::IO::OVERLAPPED, bytestransferred: *mut u32) -> u32;
}
pub const WNV_API_MAJOR_VERSION_1: u32 = 1u32;
pub const WNV_API_MINOR_VERSION_0: u32 = 0u32;
#[repr(transparent)]
pub struct WNV_CA_NOTIFICATION_TYPE(pub i32);
pub const WnvCustomerAddressAdded: WNV_CA_NOTIFICATION_TYPE = WNV_CA_NOTIFICATION_TYPE(0i32);
pub const WnvCustomerAddressDeleted: WNV_CA_NOTIFICATION_TYPE = WNV_CA_NOTIFICATION_TYPE(1i32);
pub const WnvCustomerAddressMoved: WNV_CA_NOTIFICATION_TYPE = WNV_CA_NOTIFICATION_TYPE(2i32);
pub const WnvCustomerAddressMax: WNV_CA_NOTIFICATION_TYPE = WNV_CA_NOTIFICATION_TYPE(3i32);
#[cfg(all(feature = "Win32_NetworkManagement_WindowsFilteringPlatform", feature = "Win32_Networking_WinSock"))]
#[repr(C)]
pub struct WNV_CUSTOMER_ADDRESS_CHANGE_PARAM(i32);
#[cfg(feature = "Win32_Networking_WinSock")]
#[repr(C)]
pub struct WNV_IP_ADDRESS(i32);
#[repr(C)]
pub struct WNV_NOTIFICATION_PARAM(i32);
#[repr(transparent)]
pub struct WNV_NOTIFICATION_TYPE(pub i32);
pub const WnvPolicyMismatchType: WNV_NOTIFICATION_TYPE = WNV_NOTIFICATION_TYPE(0i32);
pub const WnvRedirectType: WNV_NOTIFICATION_TYPE = WNV_NOTIFICATION_TYPE(1i32);
pub const WnvObjectChangeType: WNV_NOTIFICATION_TYPE = WNV_NOTIFICATION_TYPE(2i32);
pub const WnvNotificationTypeMax: WNV_NOTIFICATION_TYPE = WNV_NOTIFICATION_TYPE(3i32);
#[cfg(all(feature = "Win32_NetworkManagement_WindowsFilteringPlatform", feature = "Win32_Networking_WinSock"))]
#[repr(C)]
pub struct WNV_OBJECT_CHANGE_PARAM(i32);
#[repr(C)]
pub struct WNV_OBJECT_HEADER(i32);
#[repr(transparent)]
pub struct WNV_OBJECT_TYPE(pub i32);
pub const WnvProviderAddressType: WNV_OBJECT_TYPE = WNV_OBJECT_TYPE(0i32);
pub const WnvCustomerAddressType: WNV_OBJECT_TYPE = WNV_OBJECT_TYPE(1i32);
pub const WnvObjectTypeMax: WNV_OBJECT_TYPE = WNV_OBJECT_TYPE(2i32);
#[cfg(feature = "Win32_Networking_WinSock")]
#[repr(C)]
pub struct WNV_POLICY_MISMATCH_PARAM(i32);
#[cfg(feature = "Win32_Networking_WinSock")]
#[repr(C)]
pub struct WNV_PROVIDER_ADDRESS_CHANGE_PARAM(i32);
#[cfg(feature = "Win32_Networking_WinSock")]
#[repr(C)]
pub struct WNV_REDIRECT_PARAM(i32);
