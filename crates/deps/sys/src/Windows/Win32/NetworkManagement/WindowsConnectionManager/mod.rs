#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsConnectionManager`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FreeInterfaceContextTable(interfacecontexttable: *const NET_INTERFACE_CONTEXT_TABLE);
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsConnectionManager`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetInterfaceContextTableForHostName(hostname: super::super::Foundation::PWSTR, proxyname: super::super::Foundation::PWSTR, flags: u32, connectionprofilefilterrawdata: *const u8, connectionprofilefilterrawdatasize: u32, interfacecontexttable: *mut *mut NET_INTERFACE_CONTEXT_TABLE) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsConnectionManager`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn OnDemandGetRoutingHint(destinationhostname: super::super::Foundation::PWSTR, interfaceindex: *mut u32) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsConnectionManager`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn OnDemandRegisterNotification(callback: ::windows::runtime::RawPtr, callbackcontext: *const ::core::ffi::c_void, registrationhandle: *mut super::super::Foundation::HANDLE) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsConnectionManager`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn OnDemandUnRegisterNotification(registrationhandle: super::super::Foundation::HANDLE) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsConnectionManager`*"]
    pub fn WcmFreeMemory(pmemory: *mut ::core::ffi::c_void);
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsConnectionManager`*"]
    pub fn WcmGetProfileList(preserved: *mut ::core::ffi::c_void, ppprofilelist: *mut *mut WCM_PROFILE_INFO_LIST) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsConnectionManager`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WcmQueryProperty(pinterface: *const ::windows::runtime::GUID, strprofilename: super::super::Foundation::PWSTR, property: WCM_PROPERTY, preserved: *mut ::core::ffi::c_void, pdwdatasize: *mut u32, ppdata: *mut *mut u8) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsConnectionManager`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WcmSetProfileList(pprofilelist: *const WCM_PROFILE_INFO_LIST, dwposition: u32, fignoreunknownprofiles: super::super::Foundation::BOOL, preserved: *mut ::core::ffi::c_void) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsConnectionManager`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WcmSetProperty(pinterface: *const ::windows::runtime::GUID, strprofilename: super::super::Foundation::PWSTR, property: WCM_PROPERTY, preserved: *mut ::core::ffi::c_void, dwdatasize: u32, pbdata: *const u8) -> u32;
}
