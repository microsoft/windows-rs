#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    #[doc = "*Required features: `Win32_NetworkManagement_Multicast`*"]
    pub fn McastApiCleanup();
    #[doc = "*Required features: `Win32_NetworkManagement_Multicast`*"]
    pub fn McastApiStartup();
    #[doc = "*Required features: `Win32_NetworkManagement_Multicast`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn McastEnumerateScopes();
    #[doc = "*Required features: `Win32_NetworkManagement_Multicast`*"]
    pub fn McastGenUID();
    #[doc = "*Required features: `Win32_NetworkManagement_Multicast`*"]
    pub fn McastReleaseAddress();
    #[doc = "*Required features: `Win32_NetworkManagement_Multicast`*"]
    pub fn McastRenewAddress();
    #[doc = "*Required features: `Win32_NetworkManagement_Multicast`*"]
    pub fn McastRequestAddress();
}
