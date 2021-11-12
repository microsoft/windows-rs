#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[link(name = "windows")]
extern "system" {
    #[cfg(feature = "Win32_Foundation")]
    pub fn FreeInterfaceContextTable(interfacecontexttable: *const NET_INTERFACE_CONTEXT_TABLE);
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetInterfaceContextTableForHostName(hostname: super::super::Foundation::PWSTR, proxyname: super::super::Foundation::PWSTR, flags: u32, connectionprofilefilterrawdata: *const u8, connectionprofilefilterrawdatasize: u32, interfacecontexttable: *mut *mut NET_INTERFACE_CONTEXT_TABLE) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn OnDemandGetRoutingHint(destinationhostname: super::super::Foundation::PWSTR, interfaceindex: *mut u32) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn OnDemandRegisterNotification(callback: ONDEMAND_NOTIFICATION_CALLBACK, callbackcontext: *const ::core::ffi::c_void, registrationhandle: *mut super::super::Foundation::HANDLE) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn OnDemandUnRegisterNotification(registrationhandle: super::super::Foundation::HANDLE) -> ::windows_sys::core::HRESULT;
    pub fn WcmFreeMemory(pmemory: *mut ::core::ffi::c_void);
    pub fn WcmGetProfileList(preserved: *mut ::core::ffi::c_void, ppprofilelist: *mut *mut WCM_PROFILE_INFO_LIST) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn WcmQueryProperty(pinterface: *const ::windows_sys::core::GUID, strprofilename: super::super::Foundation::PWSTR, property: WCM_PROPERTY, preserved: *mut ::core::ffi::c_void, pdwdatasize: *mut u32, ppdata: *mut *mut u8) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn WcmSetProfileList(pprofilelist: *const WCM_PROFILE_INFO_LIST, dwposition: u32, fignoreunknownprofiles: super::super::Foundation::BOOL, preserved: *mut ::core::ffi::c_void) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn WcmSetProperty(pinterface: *const ::windows_sys::core::GUID, strprofilename: super::super::Foundation::PWSTR, property: WCM_PROPERTY, preserved: *mut ::core::ffi::c_void, dwdatasize: u32, pbdata: *const u8) -> u32;
}
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct NET_INTERFACE_CONTEXT(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct NET_INTERFACE_CONTEXT_TABLE(i32);
pub const NET_INTERFACE_FLAG_CONNECT_IF_NEEDED: u32 = 1u32;
pub const NET_INTERFACE_FLAG_NONE: u32 = 0u32;
pub type ONDEMAND_NOTIFICATION_CALLBACK = unsafe extern "system" fn(param0: *const ::core::ffi::c_void);
pub const WCM_API_VERSION: u32 = 1u32;
pub const WCM_API_VERSION_1_0: u32 = 1u32;
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct WCM_BILLING_CYCLE_INFO(i32);
#[repr(transparent)]
pub struct WCM_CONNECTION_COST(pub i32);
pub const WCM_CONNECTION_COST_UNKNOWN: WCM_CONNECTION_COST = WCM_CONNECTION_COST(0i32);
pub const WCM_CONNECTION_COST_UNRESTRICTED: WCM_CONNECTION_COST = WCM_CONNECTION_COST(1i32);
pub const WCM_CONNECTION_COST_FIXED: WCM_CONNECTION_COST = WCM_CONNECTION_COST(2i32);
pub const WCM_CONNECTION_COST_VARIABLE: WCM_CONNECTION_COST = WCM_CONNECTION_COST(4i32);
pub const WCM_CONNECTION_COST_OVERDATALIMIT: WCM_CONNECTION_COST = WCM_CONNECTION_COST(65536i32);
pub const WCM_CONNECTION_COST_CONGESTED: WCM_CONNECTION_COST = WCM_CONNECTION_COST(131072i32);
pub const WCM_CONNECTION_COST_ROAMING: WCM_CONNECTION_COST = WCM_CONNECTION_COST(262144i32);
pub const WCM_CONNECTION_COST_APPROACHINGDATALIMIT: WCM_CONNECTION_COST = WCM_CONNECTION_COST(524288i32);
#[repr(C)]
pub struct WCM_CONNECTION_COST_DATA(i32);
#[repr(transparent)]
pub struct WCM_CONNECTION_COST_SOURCE(pub i32);
pub const WCM_CONNECTION_COST_SOURCE_DEFAULT: WCM_CONNECTION_COST_SOURCE = WCM_CONNECTION_COST_SOURCE(0i32);
pub const WCM_CONNECTION_COST_SOURCE_GP: WCM_CONNECTION_COST_SOURCE = WCM_CONNECTION_COST_SOURCE(1i32);
pub const WCM_CONNECTION_COST_SOURCE_USER: WCM_CONNECTION_COST_SOURCE = WCM_CONNECTION_COST_SOURCE(2i32);
pub const WCM_CONNECTION_COST_SOURCE_OPERATOR: WCM_CONNECTION_COST_SOURCE = WCM_CONNECTION_COST_SOURCE(3i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct WCM_DATAPLAN_STATUS(i32);
pub const WCM_MAX_PROFILE_NAME: u32 = 256u32;
#[repr(transparent)]
pub struct WCM_MEDIA_TYPE(pub i32);
pub const wcm_media_unknown: WCM_MEDIA_TYPE = WCM_MEDIA_TYPE(0i32);
pub const wcm_media_ethernet: WCM_MEDIA_TYPE = WCM_MEDIA_TYPE(1i32);
pub const wcm_media_wlan: WCM_MEDIA_TYPE = WCM_MEDIA_TYPE(2i32);
pub const wcm_media_mbn: WCM_MEDIA_TYPE = WCM_MEDIA_TYPE(3i32);
pub const wcm_media_invalid: WCM_MEDIA_TYPE = WCM_MEDIA_TYPE(4i32);
pub const wcm_media_max: WCM_MEDIA_TYPE = WCM_MEDIA_TYPE(5i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct WCM_POLICY_VALUE(i32);
#[repr(C)]
pub struct WCM_PROFILE_INFO(i32);
#[repr(C)]
pub struct WCM_PROFILE_INFO_LIST(i32);
#[repr(transparent)]
pub struct WCM_PROPERTY(pub i32);
pub const wcm_global_property_domain_policy: WCM_PROPERTY = WCM_PROPERTY(0i32);
pub const wcm_global_property_minimize_policy: WCM_PROPERTY = WCM_PROPERTY(1i32);
pub const wcm_global_property_roaming_policy: WCM_PROPERTY = WCM_PROPERTY(2i32);
pub const wcm_global_property_powermanagement_policy: WCM_PROPERTY = WCM_PROPERTY(3i32);
pub const wcm_intf_property_connection_cost: WCM_PROPERTY = WCM_PROPERTY(4i32);
pub const wcm_intf_property_dataplan_status: WCM_PROPERTY = WCM_PROPERTY(5i32);
pub const wcm_intf_property_hotspot_profile: WCM_PROPERTY = WCM_PROPERTY(6i32);
#[repr(C)]
pub struct WCM_TIME_INTERVAL(i32);
pub const WCM_UNKNOWN_DATAPLAN_STATUS: u32 = 4294967295u32;
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct WCM_USAGE_DATA(i32);
