#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsConnectionManager`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FreeInterfaceContextTable(interfacecontexttable: *const NET_INTERFACE_CONTEXT_TABLE);
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsConnectionManager`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetInterfaceContextTableForHostName(hostname: super::super::Foundation::PWSTR, proxyname: super::super::Foundation::PWSTR, flags: u32, connectionprofilefilterrawdata: *const u8, connectionprofilefilterrawdatasize: u32, interfacecontexttable: *mut *mut NET_INTERFACE_CONTEXT_TABLE) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsConnectionManager`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn OnDemandGetRoutingHint(destinationhostname: super::super::Foundation::PWSTR, interfaceindex: *mut u32) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsConnectionManager`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn OnDemandRegisterNotification(callback: ONDEMAND_NOTIFICATION_CALLBACK, callbackcontext: *const ::core::ffi::c_void, registrationhandle: *mut super::super::Foundation::HANDLE) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsConnectionManager`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn OnDemandUnRegisterNotification(registrationhandle: super::super::Foundation::HANDLE) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsConnectionManager`*"]
    pub fn WcmFreeMemory(pmemory: *mut ::core::ffi::c_void);
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsConnectionManager`*"]
    pub fn WcmGetProfileList(preserved: *mut ::core::ffi::c_void, ppprofilelist: *mut *mut WCM_PROFILE_INFO_LIST) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsConnectionManager`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WcmQueryProperty(pinterface: *const ::windows_sys::core::GUID, strprofilename: super::super::Foundation::PWSTR, property: WCM_PROPERTY, preserved: *mut ::core::ffi::c_void, pdwdatasize: *mut u32, ppdata: *mut *mut u8) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsConnectionManager`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WcmSetProfileList(pprofilelist: *const WCM_PROFILE_INFO_LIST, dwposition: u32, fignoreunknownprofiles: super::super::Foundation::BOOL, preserved: *mut ::core::ffi::c_void) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsConnectionManager`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WcmSetProperty(pinterface: *const ::windows_sys::core::GUID, strprofilename: super::super::Foundation::PWSTR, property: WCM_PROPERTY, preserved: *mut ::core::ffi::c_void, dwdatasize: u32, pbdata: *const u8) -> u32;
}
pub struct NET_INTERFACE_CONTEXT(i32);
pub struct NET_INTERFACE_CONTEXT_TABLE(i32);
#[doc = "*Required features: `Win32_NetworkManagement_WindowsConnectionManager`*"]
pub const NET_INTERFACE_FLAG_CONNECT_IF_NEEDED: u32 = 1u32;
#[doc = "*Required features: `Win32_NetworkManagement_WindowsConnectionManager`*"]
pub const NET_INTERFACE_FLAG_NONE: u32 = 0u32;
pub struct ONDEMAND_NOTIFICATION_CALLBACK(i32);
#[doc = "*Required features: `Win32_NetworkManagement_WindowsConnectionManager`*"]
pub const WCM_API_VERSION: u32 = 1u32;
#[doc = "*Required features: `Win32_NetworkManagement_WindowsConnectionManager`*"]
pub const WCM_API_VERSION_1_0: u32 = 1u32;
pub struct WCM_BILLING_CYCLE_INFO(i32);
pub struct WCM_CONNECTION_COST(i32);
pub struct WCM_CONNECTION_COST_DATA(i32);
pub struct WCM_CONNECTION_COST_SOURCE(i32);
pub struct WCM_DATAPLAN_STATUS(i32);
#[doc = "*Required features: `Win32_NetworkManagement_WindowsConnectionManager`*"]
pub const WCM_MAX_PROFILE_NAME: u32 = 256u32;
pub struct WCM_MEDIA_TYPE(i32);
pub struct WCM_POLICY_VALUE(i32);
pub struct WCM_PROFILE_INFO(i32);
pub struct WCM_PROFILE_INFO_LIST(i32);
pub struct WCM_PROPERTY(i32);
pub struct WCM_TIME_INTERVAL(i32);
#[doc = "*Required features: `Win32_NetworkManagement_WindowsConnectionManager`*"]
pub const WCM_UNKNOWN_DATAPLAN_STATUS: u32 = 4294967295u32;
pub struct WCM_USAGE_DATA(i32);
