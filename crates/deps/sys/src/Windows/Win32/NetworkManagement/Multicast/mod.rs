#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    #[doc = "*Required features: `Win32_NetworkManagement_Multicast`*"]
    pub fn McastApiCleanup();
    #[doc = "*Required features: `Win32_NetworkManagement_Multicast`*"]
    pub fn McastApiStartup(version: *mut u32) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_Multicast`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn McastEnumerateScopes(addrfamily: u16, requery: super::super::Foundation::BOOL, pscopelist: *mut MCAST_SCOPE_ENTRY, pscopelen: *mut u32, pscopecount: *mut u32) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_Multicast`*"]
    pub fn McastGenUID(prequestid: *mut MCAST_CLIENT_UID) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_Multicast`*"]
    pub fn McastReleaseAddress(addrfamily: u16, prequestid: *mut MCAST_CLIENT_UID, preleaserequest: *mut MCAST_LEASE_REQUEST) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_Multicast`*"]
    pub fn McastRenewAddress(addrfamily: u16, prequestid: *mut MCAST_CLIENT_UID, prenewrequest: *mut MCAST_LEASE_REQUEST, prenewresponse: *mut MCAST_LEASE_RESPONSE) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_Multicast`*"]
    pub fn McastRequestAddress(addrfamily: u16, prequestid: *mut MCAST_CLIENT_UID, pscopectx: *mut MCAST_SCOPE_CTX, paddrrequest: *mut MCAST_LEASE_REQUEST, paddrresponse: *mut MCAST_LEASE_RESPONSE) -> u32;
}
