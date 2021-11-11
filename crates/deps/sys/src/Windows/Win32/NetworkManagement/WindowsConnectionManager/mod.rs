#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsConnectionManager`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FreeInterfaceContextTable();
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsConnectionManager`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetInterfaceContextTableForHostName();
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsConnectionManager`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn OnDemandGetRoutingHint();
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsConnectionManager`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn OnDemandRegisterNotification();
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsConnectionManager`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn OnDemandUnRegisterNotification();
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsConnectionManager`*"]
    pub fn WcmFreeMemory();
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsConnectionManager`*"]
    pub fn WcmGetProfileList();
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsConnectionManager`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WcmQueryProperty();
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsConnectionManager`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WcmSetProfileList();
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsConnectionManager`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WcmSetProperty();
}
