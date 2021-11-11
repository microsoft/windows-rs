#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFilteringPlatform`, `Win32_Foundation`, `Win32_Security`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub fn FwpmCalloutAdd0(enginehandle: super::super::Foundation::HANDLE, callout: *const FWPM_CALLOUT0, sd: *const super::super::Security::SECURITY_DESCRIPTOR, id: *mut u32) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFilteringPlatform`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FwpmCalloutCreateEnumHandle0(enginehandle: super::super::Foundation::HANDLE, enumtemplate: *const FWPM_CALLOUT_ENUM_TEMPLATE0, enumhandle: *mut super::super::Foundation::HANDLE) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFilteringPlatform`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FwpmCalloutDeleteById0(enginehandle: super::super::Foundation::HANDLE, id: u32) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFilteringPlatform`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FwpmCalloutDeleteByKey0(enginehandle: super::super::Foundation::HANDLE, key: *const ::windows::runtime::GUID) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFilteringPlatform`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FwpmCalloutDestroyEnumHandle0(enginehandle: super::super::Foundation::HANDLE, enumhandle: super::super::Foundation::HANDLE) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFilteringPlatform`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FwpmCalloutEnum0(enginehandle: super::super::Foundation::HANDLE, enumhandle: super::super::Foundation::HANDLE, numentriesrequested: u32, entries: *mut *mut *mut FWPM_CALLOUT0, numentriesreturned: *mut u32) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFilteringPlatform`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FwpmCalloutGetById0(enginehandle: super::super::Foundation::HANDLE, id: u32, callout: *mut *mut FWPM_CALLOUT0) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFilteringPlatform`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FwpmCalloutGetByKey0(enginehandle: super::super::Foundation::HANDLE, key: *const ::windows::runtime::GUID, callout: *mut *mut FWPM_CALLOUT0) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFilteringPlatform`, `Win32_Foundation`, `Win32_Security`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub fn FwpmCalloutGetSecurityInfoByKey0(enginehandle: super::super::Foundation::HANDLE, key: *const ::windows::runtime::GUID, securityinfo: u32, sidowner: *mut super::super::Foundation::PSID, sidgroup: *mut super::super::Foundation::PSID, dacl: *mut *mut super::super::Security::ACL, sacl: *mut *mut super::super::Security::ACL, securitydescriptor: *mut *mut super::super::Security::SECURITY_DESCRIPTOR) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFilteringPlatform`, `Win32_Foundation`, `Win32_Security`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub fn FwpmCalloutSetSecurityInfoByKey0(enginehandle: super::super::Foundation::HANDLE, key: *const ::windows::runtime::GUID, securityinfo: u32, sidowner: *const super::super::Security::SID, sidgroup: *const super::super::Security::SID, dacl: *const super::super::Security::ACL, sacl: *const super::super::Security::ACL) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFilteringPlatform`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FwpmCalloutSubscribeChanges0(enginehandle: super::super::Foundation::HANDLE, subscription: *const FWPM_CALLOUT_SUBSCRIPTION0, callback: ::windows::runtime::RawPtr, context: *const ::core::ffi::c_void, changehandle: *mut super::super::Foundation::HANDLE) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFilteringPlatform`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FwpmCalloutSubscriptionsGet0(enginehandle: super::super::Foundation::HANDLE, entries: *mut *mut *mut FWPM_CALLOUT_SUBSCRIPTION0, numentries: *mut u32) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFilteringPlatform`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FwpmCalloutUnsubscribeChanges0(enginehandle: super::super::Foundation::HANDLE, changehandle: super::super::Foundation::HANDLE) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFilteringPlatform`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FwpmConnectionCreateEnumHandle0(enginehandle: super::super::Foundation::HANDLE, enumtemplate: *const FWPM_CONNECTION_ENUM_TEMPLATE0, enumhandle: *mut super::super::Foundation::HANDLE) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFilteringPlatform`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FwpmConnectionDestroyEnumHandle0(enginehandle: super::super::Foundation::HANDLE, enumhandle: super::super::Foundation::HANDLE) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFilteringPlatform`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FwpmConnectionEnum0(enginehandle: super::super::Foundation::HANDLE, enumhandle: super::super::Foundation::HANDLE, numentriesrequested: u32, entries: *mut *mut *mut FWPM_CONNECTION0, numentriesreturned: *mut u32) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFilteringPlatform`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FwpmConnectionGetById0(enginehandle: super::super::Foundation::HANDLE, id: u64, connection: *mut *mut FWPM_CONNECTION0) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFilteringPlatform`, `Win32_Foundation`, `Win32_Security`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub fn FwpmConnectionGetSecurityInfo0(enginehandle: super::super::Foundation::HANDLE, securityinfo: u32, sidowner: *mut super::super::Foundation::PSID, sidgroup: *mut super::super::Foundation::PSID, dacl: *mut *mut super::super::Security::ACL, sacl: *mut *mut super::super::Security::ACL, securitydescriptor: *mut *mut super::super::Security::SECURITY_DESCRIPTOR) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFilteringPlatform`, `Win32_Foundation`, `Win32_Security`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub fn FwpmConnectionSetSecurityInfo0(enginehandle: super::super::Foundation::HANDLE, securityinfo: u32, sidowner: *const super::super::Security::SID, sidgroup: *const super::super::Security::SID, dacl: *const super::super::Security::ACL, sacl: *const super::super::Security::ACL) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFilteringPlatform`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FwpmConnectionSubscribe0(enginehandle: super::super::Foundation::HANDLE, subscription: *const FWPM_CONNECTION_SUBSCRIPTION0, callback: ::windows::runtime::RawPtr, context: *const ::core::ffi::c_void, eventshandle: *mut super::super::Foundation::HANDLE) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFilteringPlatform`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FwpmConnectionUnsubscribe0(enginehandle: super::super::Foundation::HANDLE, eventshandle: super::super::Foundation::HANDLE) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFilteringPlatform`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FwpmDynamicKeywordSubscribe0(flags: u32, callback: ::windows::runtime::RawPtr, context: *const ::core::ffi::c_void, subscriptionhandle: *mut super::super::Foundation::HANDLE) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFilteringPlatform`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FwpmDynamicKeywordUnsubscribe0(subscriptionhandle: super::super::Foundation::HANDLE) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFilteringPlatform`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FwpmEngineClose0(enginehandle: super::super::Foundation::HANDLE) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFilteringPlatform`, `Win32_Foundation`, `Win32_Security`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub fn FwpmEngineGetOption0(enginehandle: super::super::Foundation::HANDLE, option: FWPM_ENGINE_OPTION, value: *mut *mut FWP_VALUE0) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFilteringPlatform`, `Win32_Foundation`, `Win32_Security`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub fn FwpmEngineGetSecurityInfo0(enginehandle: super::super::Foundation::HANDLE, securityinfo: u32, sidowner: *mut super::super::Foundation::PSID, sidgroup: *mut super::super::Foundation::PSID, dacl: *mut *mut super::super::Security::ACL, sacl: *mut *mut super::super::Security::ACL, securitydescriptor: *mut *mut super::super::Security::SECURITY_DESCRIPTOR) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFilteringPlatform`, `Win32_Foundation`, `Win32_Security`, `Win32_System_Rpc`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security", feature = "Win32_System_Rpc"))]
    pub fn FwpmEngineOpen0(servername: super::super::Foundation::PWSTR, authnservice: u32, authidentity: *const super::super::System::Rpc::SEC_WINNT_AUTH_IDENTITY_W, session: *const FWPM_SESSION0, enginehandle: *mut super::super::Foundation::HANDLE) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFilteringPlatform`, `Win32_Foundation`, `Win32_Security`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub fn FwpmEngineSetOption0(enginehandle: super::super::Foundation::HANDLE, option: FWPM_ENGINE_OPTION, newvalue: *const FWP_VALUE0) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFilteringPlatform`, `Win32_Foundation`, `Win32_Security`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub fn FwpmEngineSetSecurityInfo0(enginehandle: super::super::Foundation::HANDLE, securityinfo: u32, sidowner: *const super::super::Security::SID, sidgroup: *const super::super::Security::SID, dacl: *const super::super::Security::ACL, sacl: *const super::super::Security::ACL) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFilteringPlatform`, `Win32_Foundation`, `Win32_Security`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub fn FwpmFilterAdd0(enginehandle: super::super::Foundation::HANDLE, filter: *const FWPM_FILTER0, sd: *const super::super::Security::SECURITY_DESCRIPTOR, id: *mut u64) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFilteringPlatform`, `Win32_Foundation`, `Win32_Security`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub fn FwpmFilterCreateEnumHandle0(enginehandle: super::super::Foundation::HANDLE, enumtemplate: *const FWPM_FILTER_ENUM_TEMPLATE0, enumhandle: *mut super::super::Foundation::HANDLE) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFilteringPlatform`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FwpmFilterDeleteById0(enginehandle: super::super::Foundation::HANDLE, id: u64) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFilteringPlatform`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FwpmFilterDeleteByKey0(enginehandle: super::super::Foundation::HANDLE, key: *const ::windows::runtime::GUID) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFilteringPlatform`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FwpmFilterDestroyEnumHandle0(enginehandle: super::super::Foundation::HANDLE, enumhandle: super::super::Foundation::HANDLE) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFilteringPlatform`, `Win32_Foundation`, `Win32_Security`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub fn FwpmFilterEnum0(enginehandle: super::super::Foundation::HANDLE, enumhandle: super::super::Foundation::HANDLE, numentriesrequested: u32, entries: *mut *mut *mut FWPM_FILTER0, numentriesreturned: *mut u32) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFilteringPlatform`, `Win32_Foundation`, `Win32_Security`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub fn FwpmFilterGetById0(enginehandle: super::super::Foundation::HANDLE, id: u64, filter: *mut *mut FWPM_FILTER0) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFilteringPlatform`, `Win32_Foundation`, `Win32_Security`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub fn FwpmFilterGetByKey0(enginehandle: super::super::Foundation::HANDLE, key: *const ::windows::runtime::GUID, filter: *mut *mut FWPM_FILTER0) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFilteringPlatform`, `Win32_Foundation`, `Win32_Security`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub fn FwpmFilterGetSecurityInfoByKey0(enginehandle: super::super::Foundation::HANDLE, key: *const ::windows::runtime::GUID, securityinfo: u32, sidowner: *mut super::super::Foundation::PSID, sidgroup: *mut super::super::Foundation::PSID, dacl: *mut *mut super::super::Security::ACL, sacl: *mut *mut super::super::Security::ACL, securitydescriptor: *mut *mut super::super::Security::SECURITY_DESCRIPTOR) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFilteringPlatform`, `Win32_Foundation`, `Win32_Security`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub fn FwpmFilterSetSecurityInfoByKey0(enginehandle: super::super::Foundation::HANDLE, key: *const ::windows::runtime::GUID, securityinfo: u32, sidowner: *const super::super::Security::SID, sidgroup: *const super::super::Security::SID, dacl: *const super::super::Security::ACL, sacl: *const super::super::Security::ACL) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFilteringPlatform`, `Win32_Foundation`, `Win32_Security`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub fn FwpmFilterSubscribeChanges0(enginehandle: super::super::Foundation::HANDLE, subscription: *const FWPM_FILTER_SUBSCRIPTION0, callback: ::windows::runtime::RawPtr, context: *const ::core::ffi::c_void, changehandle: *mut super::super::Foundation::HANDLE) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFilteringPlatform`, `Win32_Foundation`, `Win32_Security`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub fn FwpmFilterSubscriptionsGet0(enginehandle: super::super::Foundation::HANDLE, entries: *mut *mut *mut FWPM_FILTER_SUBSCRIPTION0, numentries: *mut u32) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFilteringPlatform`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FwpmFilterUnsubscribeChanges0(enginehandle: super::super::Foundation::HANDLE, changehandle: super::super::Foundation::HANDLE) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFilteringPlatform`*"]
    pub fn FwpmFreeMemory0(p: *mut *mut ::core::ffi::c_void);
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFilteringPlatform`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FwpmGetAppIdFromFileName0(filename: super::super::Foundation::PWSTR, appid: *mut *mut FWP_BYTE_BLOB) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFilteringPlatform`, `Win32_Foundation`, `Win32_Security`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub fn FwpmIPsecTunnelAdd0(enginehandle: super::super::Foundation::HANDLE, flags: u32, mainmodepolicy: *const FWPM_PROVIDER_CONTEXT0, tunnelpolicy: *const FWPM_PROVIDER_CONTEXT0, numfilterconditions: u32, filterconditions: *const FWPM_FILTER_CONDITION0, sd: *const super::super::Security::SECURITY_DESCRIPTOR) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFilteringPlatform`, `Win32_Foundation`, `Win32_Security`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub fn FwpmIPsecTunnelAdd1(enginehandle: super::super::Foundation::HANDLE, flags: u32, mainmodepolicy: *const FWPM_PROVIDER_CONTEXT1, tunnelpolicy: *const FWPM_PROVIDER_CONTEXT1, numfilterconditions: u32, filterconditions: *const FWPM_FILTER_CONDITION0, keymodkey: *const ::windows::runtime::GUID, sd: *const super::super::Security::SECURITY_DESCRIPTOR) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFilteringPlatform`, `Win32_Foundation`, `Win32_Security`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub fn FwpmIPsecTunnelAdd2(enginehandle: super::super::Foundation::HANDLE, flags: u32, mainmodepolicy: *const FWPM_PROVIDER_CONTEXT2, tunnelpolicy: *const FWPM_PROVIDER_CONTEXT2, numfilterconditions: u32, filterconditions: *const FWPM_FILTER_CONDITION0, keymodkey: *const ::windows::runtime::GUID, sd: *const super::super::Security::SECURITY_DESCRIPTOR) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFilteringPlatform`, `Win32_Foundation`, `Win32_Security`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub fn FwpmIPsecTunnelAdd3(enginehandle: super::super::Foundation::HANDLE, flags: u32, mainmodepolicy: *const FWPM_PROVIDER_CONTEXT3_, tunnelpolicy: *const FWPM_PROVIDER_CONTEXT3_, numfilterconditions: u32, filterconditions: *const FWPM_FILTER_CONDITION0, keymodkey: *const ::windows::runtime::GUID, sd: *const super::super::Security::SECURITY_DESCRIPTOR) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFilteringPlatform`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FwpmIPsecTunnelDeleteByKey0(enginehandle: super::super::Foundation::HANDLE, key: *const ::windows::runtime::GUID) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFilteringPlatform`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FwpmLayerCreateEnumHandle0(enginehandle: super::super::Foundation::HANDLE, enumtemplate: *const FWPM_LAYER_ENUM_TEMPLATE0, enumhandle: *mut super::super::Foundation::HANDLE) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFilteringPlatform`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FwpmLayerDestroyEnumHandle0(enginehandle: super::super::Foundation::HANDLE, enumhandle: super::super::Foundation::HANDLE) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFilteringPlatform`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FwpmLayerEnum0(enginehandle: super::super::Foundation::HANDLE, enumhandle: super::super::Foundation::HANDLE, numentriesrequested: u32, entries: *mut *mut *mut FWPM_LAYER0, numentriesreturned: *mut u32) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFilteringPlatform`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FwpmLayerGetById0(enginehandle: super::super::Foundation::HANDLE, id: u16, layer: *mut *mut FWPM_LAYER0) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFilteringPlatform`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FwpmLayerGetByKey0(enginehandle: super::super::Foundation::HANDLE, key: *const ::windows::runtime::GUID, layer: *mut *mut FWPM_LAYER0) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFilteringPlatform`, `Win32_Foundation`, `Win32_Security`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub fn FwpmLayerGetSecurityInfoByKey0(enginehandle: super::super::Foundation::HANDLE, key: *const ::windows::runtime::GUID, securityinfo: u32, sidowner: *mut super::super::Foundation::PSID, sidgroup: *mut super::super::Foundation::PSID, dacl: *mut *mut super::super::Security::ACL, sacl: *mut *mut super::super::Security::ACL, securitydescriptor: *mut *mut super::super::Security::SECURITY_DESCRIPTOR) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFilteringPlatform`, `Win32_Foundation`, `Win32_Security`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub fn FwpmLayerSetSecurityInfoByKey0(enginehandle: super::super::Foundation::HANDLE, key: *const ::windows::runtime::GUID, securityinfo: u32, sidowner: *const super::super::Security::SID, sidgroup: *const super::super::Security::SID, dacl: *const super::super::Security::ACL, sacl: *const super::super::Security::ACL) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFilteringPlatform`, `Win32_Foundation`, `Win32_Security`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub fn FwpmNetEventCreateEnumHandle0(enginehandle: super::super::Foundation::HANDLE, enumtemplate: *const FWPM_NET_EVENT_ENUM_TEMPLATE0, enumhandle: *mut super::super::Foundation::HANDLE) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFilteringPlatform`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FwpmNetEventDestroyEnumHandle0(enginehandle: super::super::Foundation::HANDLE, enumhandle: super::super::Foundation::HANDLE) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFilteringPlatform`, `Win32_Foundation`, `Win32_Security`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub fn FwpmNetEventEnum0(enginehandle: super::super::Foundation::HANDLE, enumhandle: super::super::Foundation::HANDLE, numentriesrequested: u32, entries: *mut *mut *mut FWPM_NET_EVENT0, numentriesreturned: *mut u32) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFilteringPlatform`, `Win32_Foundation`, `Win32_Security`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub fn FwpmNetEventEnum1(enginehandle: super::super::Foundation::HANDLE, enumhandle: super::super::Foundation::HANDLE, numentriesrequested: u32, entries: *mut *mut *mut FWPM_NET_EVENT1, numentriesreturned: *mut u32) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFilteringPlatform`, `Win32_Foundation`, `Win32_Security`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub fn FwpmNetEventEnum2(enginehandle: super::super::Foundation::HANDLE, enumhandle: super::super::Foundation::HANDLE, numentriesrequested: u32, entries: *mut *mut *mut FWPM_NET_EVENT2, numentriesreturned: *mut u32) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFilteringPlatform`, `Win32_Foundation`, `Win32_Security`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub fn FwpmNetEventEnum3(enginehandle: super::super::Foundation::HANDLE, enumhandle: super::super::Foundation::HANDLE, numentriesrequested: u32, entries: *mut *mut *mut FWPM_NET_EVENT3, numentriesreturned: *mut u32) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFilteringPlatform`, `Win32_Foundation`, `Win32_Security`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub fn FwpmNetEventEnum4(enginehandle: super::super::Foundation::HANDLE, enumhandle: super::super::Foundation::HANDLE, numentriesrequested: u32, entries: *mut *mut *mut FWPM_NET_EVENT4_, numentriesreturned: *mut u32) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFilteringPlatform`, `Win32_Foundation`, `Win32_Security`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub fn FwpmNetEventEnum5(enginehandle: super::super::Foundation::HANDLE, enumhandle: super::super::Foundation::HANDLE, numentriesrequested: u32, entries: *mut *mut *mut FWPM_NET_EVENT5_, numentriesreturned: *mut u32) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFilteringPlatform`, `Win32_Foundation`, `Win32_Security`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub fn FwpmNetEventSubscribe0(enginehandle: super::super::Foundation::HANDLE, subscription: *const FWPM_NET_EVENT_SUBSCRIPTION0, callback: ::windows::runtime::RawPtr, context: *const ::core::ffi::c_void, eventshandle: *mut super::super::Foundation::HANDLE) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFilteringPlatform`, `Win32_Foundation`, `Win32_Security`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub fn FwpmNetEventSubscribe1(enginehandle: super::super::Foundation::HANDLE, subscription: *const FWPM_NET_EVENT_SUBSCRIPTION0, callback: ::windows::runtime::RawPtr, context: *const ::core::ffi::c_void, eventshandle: *mut super::super::Foundation::HANDLE) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFilteringPlatform`, `Win32_Foundation`, `Win32_Security`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub fn FwpmNetEventSubscribe2(enginehandle: super::super::Foundation::HANDLE, subscription: *const FWPM_NET_EVENT_SUBSCRIPTION0, callback: ::windows::runtime::RawPtr, context: *const ::core::ffi::c_void, eventshandle: *mut super::super::Foundation::HANDLE) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFilteringPlatform`, `Win32_Foundation`, `Win32_Security`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub fn FwpmNetEventSubscribe3(enginehandle: super::super::Foundation::HANDLE, subscription: *const FWPM_NET_EVENT_SUBSCRIPTION0, callback: ::windows::runtime::RawPtr, context: *const ::core::ffi::c_void, eventshandle: *mut super::super::Foundation::HANDLE) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFilteringPlatform`, `Win32_Foundation`, `Win32_Security`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub fn FwpmNetEventSubscribe4(enginehandle: super::super::Foundation::HANDLE, subscription: *const FWPM_NET_EVENT_SUBSCRIPTION0, callback: ::windows::runtime::RawPtr, context: *const ::core::ffi::c_void, eventshandle: *mut super::super::Foundation::HANDLE) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFilteringPlatform`, `Win32_Foundation`, `Win32_Security`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub fn FwpmNetEventSubscriptionsGet0(enginehandle: super::super::Foundation::HANDLE, entries: *mut *mut *mut FWPM_NET_EVENT_SUBSCRIPTION0, numentries: *mut u32) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFilteringPlatform`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FwpmNetEventUnsubscribe0(enginehandle: super::super::Foundation::HANDLE, eventshandle: super::super::Foundation::HANDLE) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFilteringPlatform`, `Win32_Foundation`, `Win32_Security`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub fn FwpmNetEventsGetSecurityInfo0(enginehandle: super::super::Foundation::HANDLE, securityinfo: u32, sidowner: *mut super::super::Foundation::PSID, sidgroup: *mut super::super::Foundation::PSID, dacl: *mut *mut super::super::Security::ACL, sacl: *mut *mut super::super::Security::ACL, securitydescriptor: *mut *mut super::super::Security::SECURITY_DESCRIPTOR) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFilteringPlatform`, `Win32_Foundation`, `Win32_Security`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub fn FwpmNetEventsSetSecurityInfo0(enginehandle: super::super::Foundation::HANDLE, securityinfo: u32, sidowner: *const super::super::Security::SID, sidgroup: *const super::super::Security::SID, dacl: *const super::super::Security::ACL, sacl: *const super::super::Security::ACL) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFilteringPlatform`, `Win32_Foundation`, `Win32_Security`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub fn FwpmProviderAdd0(enginehandle: super::super::Foundation::HANDLE, provider: *const FWPM_PROVIDER0, sd: *const super::super::Security::SECURITY_DESCRIPTOR) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFilteringPlatform`, `Win32_Foundation`, `Win32_Security`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub fn FwpmProviderContextAdd0(enginehandle: super::super::Foundation::HANDLE, providercontext: *const FWPM_PROVIDER_CONTEXT0, sd: *const super::super::Security::SECURITY_DESCRIPTOR, id: *mut u64) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFilteringPlatform`, `Win32_Foundation`, `Win32_Security`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub fn FwpmProviderContextAdd1(enginehandle: super::super::Foundation::HANDLE, providercontext: *const FWPM_PROVIDER_CONTEXT1, sd: *const super::super::Security::SECURITY_DESCRIPTOR, id: *mut u64) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFilteringPlatform`, `Win32_Foundation`, `Win32_Security`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub fn FwpmProviderContextAdd2(enginehandle: super::super::Foundation::HANDLE, providercontext: *const FWPM_PROVIDER_CONTEXT2, sd: *const super::super::Security::SECURITY_DESCRIPTOR, id: *mut u64) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFilteringPlatform`, `Win32_Foundation`, `Win32_Security`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub fn FwpmProviderContextAdd3(enginehandle: super::super::Foundation::HANDLE, providercontext: *const FWPM_PROVIDER_CONTEXT3_, sd: *const super::super::Security::SECURITY_DESCRIPTOR, id: *mut u64) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFilteringPlatform`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FwpmProviderContextCreateEnumHandle0(enginehandle: super::super::Foundation::HANDLE, enumtemplate: *const FWPM_PROVIDER_CONTEXT_ENUM_TEMPLATE0, enumhandle: *mut super::super::Foundation::HANDLE) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFilteringPlatform`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FwpmProviderContextDeleteById0(enginehandle: super::super::Foundation::HANDLE, id: u64) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFilteringPlatform`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FwpmProviderContextDeleteByKey0(enginehandle: super::super::Foundation::HANDLE, key: *const ::windows::runtime::GUID) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFilteringPlatform`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FwpmProviderContextDestroyEnumHandle0(enginehandle: super::super::Foundation::HANDLE, enumhandle: super::super::Foundation::HANDLE) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFilteringPlatform`, `Win32_Foundation`, `Win32_Security`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub fn FwpmProviderContextEnum0(enginehandle: super::super::Foundation::HANDLE, enumhandle: super::super::Foundation::HANDLE, numentriesrequested: u32, entries: *mut *mut *mut FWPM_PROVIDER_CONTEXT0, numentriesreturned: *mut u32) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFilteringPlatform`, `Win32_Foundation`, `Win32_Security`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub fn FwpmProviderContextEnum1(enginehandle: super::super::Foundation::HANDLE, enumhandle: super::super::Foundation::HANDLE, numentriesrequested: u32, entries: *mut *mut *mut FWPM_PROVIDER_CONTEXT1, numentriesreturned: *mut u32) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFilteringPlatform`, `Win32_Foundation`, `Win32_Security`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub fn FwpmProviderContextEnum2(enginehandle: super::super::Foundation::HANDLE, enumhandle: super::super::Foundation::HANDLE, numentriesrequested: u32, entries: *mut *mut *mut FWPM_PROVIDER_CONTEXT2, numentriesreturned: *mut u32) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFilteringPlatform`, `Win32_Foundation`, `Win32_Security`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub fn FwpmProviderContextEnum3(enginehandle: super::super::Foundation::HANDLE, enumhandle: super::super::Foundation::HANDLE, numentriesrequested: u32, entries: *mut *mut *mut FWPM_PROVIDER_CONTEXT3_, numentriesreturned: *mut u32) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFilteringPlatform`, `Win32_Foundation`, `Win32_Security`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub fn FwpmProviderContextGetById0(enginehandle: super::super::Foundation::HANDLE, id: u64, providercontext: *mut *mut FWPM_PROVIDER_CONTEXT0) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFilteringPlatform`, `Win32_Foundation`, `Win32_Security`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub fn FwpmProviderContextGetById1(enginehandle: super::super::Foundation::HANDLE, id: u64, providercontext: *mut *mut FWPM_PROVIDER_CONTEXT1) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFilteringPlatform`, `Win32_Foundation`, `Win32_Security`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub fn FwpmProviderContextGetById2(enginehandle: super::super::Foundation::HANDLE, id: u64, providercontext: *mut *mut FWPM_PROVIDER_CONTEXT2) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFilteringPlatform`, `Win32_Foundation`, `Win32_Security`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub fn FwpmProviderContextGetById3(enginehandle: super::super::Foundation::HANDLE, id: u64, providercontext: *mut *mut FWPM_PROVIDER_CONTEXT3_) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFilteringPlatform`, `Win32_Foundation`, `Win32_Security`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub fn FwpmProviderContextGetByKey0(enginehandle: super::super::Foundation::HANDLE, key: *const ::windows::runtime::GUID, providercontext: *mut *mut FWPM_PROVIDER_CONTEXT0) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFilteringPlatform`, `Win32_Foundation`, `Win32_Security`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub fn FwpmProviderContextGetByKey1(enginehandle: super::super::Foundation::HANDLE, key: *const ::windows::runtime::GUID, providercontext: *mut *mut FWPM_PROVIDER_CONTEXT1) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFilteringPlatform`, `Win32_Foundation`, `Win32_Security`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub fn FwpmProviderContextGetByKey2(enginehandle: super::super::Foundation::HANDLE, key: *const ::windows::runtime::GUID, providercontext: *mut *mut FWPM_PROVIDER_CONTEXT2) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFilteringPlatform`, `Win32_Foundation`, `Win32_Security`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub fn FwpmProviderContextGetByKey3(enginehandle: super::super::Foundation::HANDLE, key: *const ::windows::runtime::GUID, providercontext: *mut *mut FWPM_PROVIDER_CONTEXT3_) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFilteringPlatform`, `Win32_Foundation`, `Win32_Security`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub fn FwpmProviderContextGetSecurityInfoByKey0(enginehandle: super::super::Foundation::HANDLE, key: *const ::windows::runtime::GUID, securityinfo: u32, sidowner: *mut super::super::Foundation::PSID, sidgroup: *mut super::super::Foundation::PSID, dacl: *mut *mut super::super::Security::ACL, sacl: *mut *mut super::super::Security::ACL, securitydescriptor: *mut *mut super::super::Security::SECURITY_DESCRIPTOR) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFilteringPlatform`, `Win32_Foundation`, `Win32_Security`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub fn FwpmProviderContextSetSecurityInfoByKey0(enginehandle: super::super::Foundation::HANDLE, key: *const ::windows::runtime::GUID, securityinfo: u32, sidowner: *const super::super::Security::SID, sidgroup: *const super::super::Security::SID, dacl: *const super::super::Security::ACL, sacl: *const super::super::Security::ACL) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFilteringPlatform`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FwpmProviderContextSubscribeChanges0(enginehandle: super::super::Foundation::HANDLE, subscription: *const FWPM_PROVIDER_CONTEXT_SUBSCRIPTION0, callback: ::windows::runtime::RawPtr, context: *const ::core::ffi::c_void, changehandle: *mut super::super::Foundation::HANDLE) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFilteringPlatform`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FwpmProviderContextSubscriptionsGet0(enginehandle: super::super::Foundation::HANDLE, entries: *mut *mut *mut FWPM_PROVIDER_CONTEXT_SUBSCRIPTION0, numentries: *mut u32) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFilteringPlatform`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FwpmProviderContextUnsubscribeChanges0(enginehandle: super::super::Foundation::HANDLE, changehandle: super::super::Foundation::HANDLE) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFilteringPlatform`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FwpmProviderCreateEnumHandle0(enginehandle: super::super::Foundation::HANDLE, enumtemplate: *const FWPM_PROVIDER_ENUM_TEMPLATE0, enumhandle: *mut super::super::Foundation::HANDLE) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFilteringPlatform`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FwpmProviderDeleteByKey0(enginehandle: super::super::Foundation::HANDLE, key: *const ::windows::runtime::GUID) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFilteringPlatform`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FwpmProviderDestroyEnumHandle0(enginehandle: super::super::Foundation::HANDLE, enumhandle: super::super::Foundation::HANDLE) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFilteringPlatform`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FwpmProviderEnum0(enginehandle: super::super::Foundation::HANDLE, enumhandle: super::super::Foundation::HANDLE, numentriesrequested: u32, entries: *mut *mut *mut FWPM_PROVIDER0, numentriesreturned: *mut u32) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFilteringPlatform`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FwpmProviderGetByKey0(enginehandle: super::super::Foundation::HANDLE, key: *const ::windows::runtime::GUID, provider: *mut *mut FWPM_PROVIDER0) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFilteringPlatform`, `Win32_Foundation`, `Win32_Security`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub fn FwpmProviderGetSecurityInfoByKey0(enginehandle: super::super::Foundation::HANDLE, key: *const ::windows::runtime::GUID, securityinfo: u32, sidowner: *mut super::super::Foundation::PSID, sidgroup: *mut super::super::Foundation::PSID, dacl: *mut *mut super::super::Security::ACL, sacl: *mut *mut super::super::Security::ACL, securitydescriptor: *mut *mut super::super::Security::SECURITY_DESCRIPTOR) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFilteringPlatform`, `Win32_Foundation`, `Win32_Security`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub fn FwpmProviderSetSecurityInfoByKey0(enginehandle: super::super::Foundation::HANDLE, key: *const ::windows::runtime::GUID, securityinfo: u32, sidowner: *const super::super::Security::SID, sidgroup: *const super::super::Security::SID, dacl: *const super::super::Security::ACL, sacl: *const super::super::Security::ACL) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFilteringPlatform`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FwpmProviderSubscribeChanges0(enginehandle: super::super::Foundation::HANDLE, subscription: *const FWPM_PROVIDER_SUBSCRIPTION0, callback: ::windows::runtime::RawPtr, context: *const ::core::ffi::c_void, changehandle: *mut super::super::Foundation::HANDLE) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFilteringPlatform`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FwpmProviderSubscriptionsGet0(enginehandle: super::super::Foundation::HANDLE, entries: *mut *mut *mut FWPM_PROVIDER_SUBSCRIPTION0, numentries: *mut u32) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFilteringPlatform`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FwpmProviderUnsubscribeChanges0(enginehandle: super::super::Foundation::HANDLE, changehandle: super::super::Foundation::HANDLE) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFilteringPlatform`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FwpmSessionCreateEnumHandle0(enginehandle: super::super::Foundation::HANDLE, enumtemplate: *const FWPM_SESSION_ENUM_TEMPLATE0, enumhandle: *mut super::super::Foundation::HANDLE) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFilteringPlatform`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FwpmSessionDestroyEnumHandle0(enginehandle: super::super::Foundation::HANDLE, enumhandle: super::super::Foundation::HANDLE) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFilteringPlatform`, `Win32_Foundation`, `Win32_Security`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub fn FwpmSessionEnum0(enginehandle: super::super::Foundation::HANDLE, enumhandle: super::super::Foundation::HANDLE, numentriesrequested: u32, entries: *mut *mut *mut FWPM_SESSION0, numentriesreturned: *mut u32) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFilteringPlatform`, `Win32_Foundation`, `Win32_Security`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub fn FwpmSubLayerAdd0(enginehandle: super::super::Foundation::HANDLE, sublayer: *const FWPM_SUBLAYER0, sd: *const super::super::Security::SECURITY_DESCRIPTOR) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFilteringPlatform`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FwpmSubLayerCreateEnumHandle0(enginehandle: super::super::Foundation::HANDLE, enumtemplate: *const FWPM_SUBLAYER_ENUM_TEMPLATE0, enumhandle: *mut super::super::Foundation::HANDLE) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFilteringPlatform`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FwpmSubLayerDeleteByKey0(enginehandle: super::super::Foundation::HANDLE, key: *const ::windows::runtime::GUID) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFilteringPlatform`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FwpmSubLayerDestroyEnumHandle0(enginehandle: super::super::Foundation::HANDLE, enumhandle: super::super::Foundation::HANDLE) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFilteringPlatform`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FwpmSubLayerEnum0(enginehandle: super::super::Foundation::HANDLE, enumhandle: super::super::Foundation::HANDLE, numentriesrequested: u32, entries: *mut *mut *mut FWPM_SUBLAYER0, numentriesreturned: *mut u32) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFilteringPlatform`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FwpmSubLayerGetByKey0(enginehandle: super::super::Foundation::HANDLE, key: *const ::windows::runtime::GUID, sublayer: *mut *mut FWPM_SUBLAYER0) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFilteringPlatform`, `Win32_Foundation`, `Win32_Security`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub fn FwpmSubLayerGetSecurityInfoByKey0(enginehandle: super::super::Foundation::HANDLE, key: *const ::windows::runtime::GUID, securityinfo: u32, sidowner: *mut super::super::Foundation::PSID, sidgroup: *mut super::super::Foundation::PSID, dacl: *mut *mut super::super::Security::ACL, sacl: *mut *mut super::super::Security::ACL, securitydescriptor: *mut *mut super::super::Security::SECURITY_DESCRIPTOR) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFilteringPlatform`, `Win32_Foundation`, `Win32_Security`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub fn FwpmSubLayerSetSecurityInfoByKey0(enginehandle: super::super::Foundation::HANDLE, key: *const ::windows::runtime::GUID, securityinfo: u32, sidowner: *const super::super::Security::SID, sidgroup: *const super::super::Security::SID, dacl: *const super::super::Security::ACL, sacl: *const super::super::Security::ACL) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFilteringPlatform`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FwpmSubLayerSubscribeChanges0(enginehandle: super::super::Foundation::HANDLE, subscription: *const FWPM_SUBLAYER_SUBSCRIPTION0, callback: ::windows::runtime::RawPtr, context: *const ::core::ffi::c_void, changehandle: *mut super::super::Foundation::HANDLE) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFilteringPlatform`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FwpmSubLayerSubscriptionsGet0(enginehandle: super::super::Foundation::HANDLE, entries: *mut *mut *mut FWPM_SUBLAYER_SUBSCRIPTION0, numentries: *mut u32) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFilteringPlatform`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FwpmSubLayerUnsubscribeChanges0(enginehandle: super::super::Foundation::HANDLE, changehandle: super::super::Foundation::HANDLE) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFilteringPlatform`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FwpmSystemPortsGet0(enginehandle: super::super::Foundation::HANDLE, sysports: *mut *mut FWPM_SYSTEM_PORTS0) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFilteringPlatform`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FwpmSystemPortsSubscribe0(enginehandle: super::super::Foundation::HANDLE, reserved: *mut ::core::ffi::c_void, callback: ::windows::runtime::RawPtr, context: *const ::core::ffi::c_void, sysportshandle: *mut super::super::Foundation::HANDLE) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFilteringPlatform`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FwpmSystemPortsUnsubscribe0(enginehandle: super::super::Foundation::HANDLE, sysportshandle: super::super::Foundation::HANDLE) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFilteringPlatform`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FwpmTransactionAbort0(enginehandle: super::super::Foundation::HANDLE) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFilteringPlatform`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FwpmTransactionBegin0(enginehandle: super::super::Foundation::HANDLE, flags: u32) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFilteringPlatform`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FwpmTransactionCommit0(enginehandle: super::super::Foundation::HANDLE) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFilteringPlatform`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FwpmvSwitchEventSubscribe0(enginehandle: super::super::Foundation::HANDLE, subscription: *const FWPM_VSWITCH_EVENT_SUBSCRIPTION0, callback: ::windows::runtime::RawPtr, context: *const ::core::ffi::c_void, subscriptionhandle: *mut super::super::Foundation::HANDLE) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFilteringPlatform`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FwpmvSwitchEventUnsubscribe0(enginehandle: super::super::Foundation::HANDLE, subscriptionhandle: super::super::Foundation::HANDLE) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFilteringPlatform`, `Win32_Foundation`, `Win32_Security`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub fn FwpmvSwitchEventsGetSecurityInfo0(enginehandle: super::super::Foundation::HANDLE, securityinfo: u32, sidowner: *mut super::super::Foundation::PSID, sidgroup: *mut super::super::Foundation::PSID, dacl: *mut *mut super::super::Security::ACL, sacl: *mut *mut super::super::Security::ACL, securitydescriptor: *mut *mut super::super::Security::SECURITY_DESCRIPTOR) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFilteringPlatform`, `Win32_Foundation`, `Win32_Security`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub fn FwpmvSwitchEventsSetSecurityInfo0(enginehandle: super::super::Foundation::HANDLE, securityinfo: u32, sidowner: *const super::super::Security::SID, sidgroup: *const super::super::Security::SID, dacl: *const super::super::Security::ACL, sacl: *const super::super::Security::ACL) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFilteringPlatform`, `Win32_Foundation`, `Win32_Security`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub fn IPsecDospGetSecurityInfo0(enginehandle: super::super::Foundation::HANDLE, securityinfo: u32, sidowner: *mut super::super::Foundation::PSID, sidgroup: *mut super::super::Foundation::PSID, dacl: *mut *mut super::super::Security::ACL, sacl: *mut *mut super::super::Security::ACL, securitydescriptor: *mut *mut super::super::Security::SECURITY_DESCRIPTOR) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFilteringPlatform`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn IPsecDospGetStatistics0(enginehandle: super::super::Foundation::HANDLE, idpstatistics: *mut IPSEC_DOSP_STATISTICS0) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFilteringPlatform`, `Win32_Foundation`, `Win32_Security`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub fn IPsecDospSetSecurityInfo0(enginehandle: super::super::Foundation::HANDLE, securityinfo: u32, sidowner: *const super::super::Security::SID, sidgroup: *const super::super::Security::SID, dacl: *const super::super::Security::ACL, sacl: *const super::super::Security::ACL) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFilteringPlatform`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn IPsecDospStateCreateEnumHandle0(enginehandle: super::super::Foundation::HANDLE, enumtemplate: *const IPSEC_DOSP_STATE_ENUM_TEMPLATE0, enumhandle: *mut super::super::Foundation::HANDLE) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFilteringPlatform`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn IPsecDospStateDestroyEnumHandle0(enginehandle: super::super::Foundation::HANDLE, enumhandle: super::super::Foundation::HANDLE) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFilteringPlatform`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn IPsecDospStateEnum0(enginehandle: super::super::Foundation::HANDLE, enumhandle: super::super::Foundation::HANDLE, numentriesrequested: u32, entries: *mut *mut *mut IPSEC_DOSP_STATE0, numentries: *mut u32) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFilteringPlatform`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn IPsecGetStatistics0(enginehandle: super::super::Foundation::HANDLE, ipsecstatistics: *mut IPSEC_STATISTICS0) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFilteringPlatform`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn IPsecGetStatistics1(enginehandle: super::super::Foundation::HANDLE, ipsecstatistics: *mut IPSEC_STATISTICS1) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFilteringPlatform`, `Win32_Foundation`, `Win32_Security`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub fn IPsecKeyManagerAddAndRegister0(enginehandle: super::super::Foundation::HANDLE, keymanager: *const IPSEC_KEY_MANAGER0, keymanagercallbacks: *const ::core::mem::ManuallyDrop<IPSEC_KEY_MANAGER_CALLBACKS0>, keymgmthandle: *mut super::super::Foundation::HANDLE) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFilteringPlatform`, `Win32_Foundation`, `Win32_Security`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub fn IPsecKeyManagerGetSecurityInfoByKey0(enginehandle: super::super::Foundation::HANDLE, reserved: *const ::core::ffi::c_void, securityinfo: u32, sidowner: *mut super::super::Foundation::PSID, sidgroup: *mut super::super::Foundation::PSID, dacl: *mut *mut super::super::Security::ACL, sacl: *mut *mut super::super::Security::ACL, securitydescriptor: *mut *mut super::super::Security::SECURITY_DESCRIPTOR) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFilteringPlatform`, `Win32_Foundation`, `Win32_Security`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub fn IPsecKeyManagerSetSecurityInfoByKey0(enginehandle: super::super::Foundation::HANDLE, reserved: *const ::core::ffi::c_void, securityinfo: u32, sidowner: *const super::super::Security::SID, sidgroup: *const super::super::Security::SID, dacl: *const super::super::Security::ACL, sacl: *const super::super::Security::ACL) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFilteringPlatform`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn IPsecKeyManagerUnregisterAndDelete0(enginehandle: super::super::Foundation::HANDLE, keymgmthandle: super::super::Foundation::HANDLE) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFilteringPlatform`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn IPsecKeyManagersGet0(enginehandle: super::super::Foundation::HANDLE, entries: *mut *mut *mut IPSEC_KEY_MANAGER0, numentries: *mut u32) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFilteringPlatform`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn IPsecSaContextAddInbound0(enginehandle: super::super::Foundation::HANDLE, id: u64, inboundbundle: *const IPSEC_SA_BUNDLE0) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFilteringPlatform`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn IPsecSaContextAddInbound1(enginehandle: super::super::Foundation::HANDLE, id: u64, inboundbundle: *const IPSEC_SA_BUNDLE1) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFilteringPlatform`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn IPsecSaContextAddOutbound0(enginehandle: super::super::Foundation::HANDLE, id: u64, outboundbundle: *const IPSEC_SA_BUNDLE0) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFilteringPlatform`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn IPsecSaContextAddOutbound1(enginehandle: super::super::Foundation::HANDLE, id: u64, outboundbundle: *const IPSEC_SA_BUNDLE1) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFilteringPlatform`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn IPsecSaContextCreate0(enginehandle: super::super::Foundation::HANDLE, outboundtraffic: *const IPSEC_TRAFFIC0, inboundfilterid: *mut u64, id: *mut u64) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFilteringPlatform`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn IPsecSaContextCreate1(enginehandle: super::super::Foundation::HANDLE, outboundtraffic: *const IPSEC_TRAFFIC1, virtualiftunnelinfo: *const IPSEC_VIRTUAL_IF_TUNNEL_INFO0, inboundfilterid: *mut u64, id: *mut u64) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFilteringPlatform`, `Win32_Foundation`, `Win32_Security`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub fn IPsecSaContextCreateEnumHandle0(enginehandle: super::super::Foundation::HANDLE, enumtemplate: *const IPSEC_SA_CONTEXT_ENUM_TEMPLATE0, enumhandle: *mut super::super::Foundation::HANDLE) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFilteringPlatform`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn IPsecSaContextDeleteById0(enginehandle: super::super::Foundation::HANDLE, id: u64) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFilteringPlatform`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn IPsecSaContextDestroyEnumHandle0(enginehandle: super::super::Foundation::HANDLE, enumhandle: super::super::Foundation::HANDLE) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFilteringPlatform`, `Win32_Foundation`, `Win32_Security`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub fn IPsecSaContextEnum0(enginehandle: super::super::Foundation::HANDLE, enumhandle: super::super::Foundation::HANDLE, numentriesrequested: u32, entries: *mut *mut *mut IPSEC_SA_CONTEXT0, numentriesreturned: *mut u32) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFilteringPlatform`, `Win32_Foundation`, `Win32_Security`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub fn IPsecSaContextEnum1(enginehandle: super::super::Foundation::HANDLE, enumhandle: super::super::Foundation::HANDLE, numentriesrequested: u32, entries: *mut *mut *mut IPSEC_SA_CONTEXT1, numentriesreturned: *mut u32) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFilteringPlatform`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn IPsecSaContextExpire0(enginehandle: super::super::Foundation::HANDLE, id: u64) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFilteringPlatform`, `Win32_Foundation`, `Win32_Security`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub fn IPsecSaContextGetById0(enginehandle: super::super::Foundation::HANDLE, id: u64, sacontext: *mut *mut IPSEC_SA_CONTEXT0) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFilteringPlatform`, `Win32_Foundation`, `Win32_Security`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub fn IPsecSaContextGetById1(enginehandle: super::super::Foundation::HANDLE, id: u64, sacontext: *mut *mut IPSEC_SA_CONTEXT1) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFilteringPlatform`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn IPsecSaContextGetSpi0(enginehandle: super::super::Foundation::HANDLE, id: u64, getspi: *const IPSEC_GETSPI0, inboundspi: *mut u32) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFilteringPlatform`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn IPsecSaContextGetSpi1(enginehandle: super::super::Foundation::HANDLE, id: u64, getspi: *const IPSEC_GETSPI1, inboundspi: *mut u32) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFilteringPlatform`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn IPsecSaContextSetSpi0(enginehandle: super::super::Foundation::HANDLE, id: u64, getspi: *const IPSEC_GETSPI1, inboundspi: u32) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFilteringPlatform`, `Win32_Foundation`, `Win32_Security`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub fn IPsecSaContextSubscribe0(enginehandle: super::super::Foundation::HANDLE, subscription: *const IPSEC_SA_CONTEXT_SUBSCRIPTION0, callback: ::windows::runtime::RawPtr, context: *const ::core::ffi::c_void, eventshandle: *mut super::super::Foundation::HANDLE) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFilteringPlatform`, `Win32_Foundation`, `Win32_Security`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub fn IPsecSaContextSubscriptionsGet0(enginehandle: super::super::Foundation::HANDLE, entries: *mut *mut *mut IPSEC_SA_CONTEXT_SUBSCRIPTION0, numentries: *mut u32) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFilteringPlatform`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn IPsecSaContextUnsubscribe0(enginehandle: super::super::Foundation::HANDLE, eventshandle: super::super::Foundation::HANDLE) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFilteringPlatform`, `Win32_Foundation`, `Win32_Security`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub fn IPsecSaContextUpdate0(enginehandle: super::super::Foundation::HANDLE, flags: u64, newvalues: *const IPSEC_SA_CONTEXT1) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFilteringPlatform`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn IPsecSaCreateEnumHandle0(enginehandle: super::super::Foundation::HANDLE, enumtemplate: *const IPSEC_SA_ENUM_TEMPLATE0, enumhandle: *mut super::super::Foundation::HANDLE) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFilteringPlatform`, `Win32_Foundation`, `Win32_Security`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub fn IPsecSaDbGetSecurityInfo0(enginehandle: super::super::Foundation::HANDLE, securityinfo: u32, sidowner: *mut super::super::Foundation::PSID, sidgroup: *mut super::super::Foundation::PSID, dacl: *mut *mut super::super::Security::ACL, sacl: *mut *mut super::super::Security::ACL, securitydescriptor: *mut *mut super::super::Security::SECURITY_DESCRIPTOR) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFilteringPlatform`, `Win32_Foundation`, `Win32_Security`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub fn IPsecSaDbSetSecurityInfo0(enginehandle: super::super::Foundation::HANDLE, securityinfo: u32, sidowner: *const super::super::Security::SID, sidgroup: *const super::super::Security::SID, dacl: *const super::super::Security::ACL, sacl: *const super::super::Security::ACL) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFilteringPlatform`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn IPsecSaDestroyEnumHandle0(enginehandle: super::super::Foundation::HANDLE, enumhandle: super::super::Foundation::HANDLE) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFilteringPlatform`, `Win32_Foundation`, `Win32_Security`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub fn IPsecSaEnum0(enginehandle: super::super::Foundation::HANDLE, enumhandle: super::super::Foundation::HANDLE, numentriesrequested: u32, entries: *mut *mut *mut IPSEC_SA_DETAILS0, numentriesreturned: *mut u32) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFilteringPlatform`, `Win32_Foundation`, `Win32_Security`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub fn IPsecSaEnum1(enginehandle: super::super::Foundation::HANDLE, enumhandle: super::super::Foundation::HANDLE, numentriesrequested: u32, entries: *mut *mut *mut IPSEC_SA_DETAILS1, numentriesreturned: *mut u32) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFilteringPlatform`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn IkeextGetStatistics0(enginehandle: super::super::Foundation::HANDLE, ikeextstatistics: *mut IKEEXT_STATISTICS0) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFilteringPlatform`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn IkeextGetStatistics1(enginehandle: super::super::Foundation::HANDLE, ikeextstatistics: *mut IKEEXT_STATISTICS1) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFilteringPlatform`, `Win32_Foundation`, `Win32_Security`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub fn IkeextSaCreateEnumHandle0(enginehandle: super::super::Foundation::HANDLE, enumtemplate: *const IKEEXT_SA_ENUM_TEMPLATE0, enumhandle: *mut super::super::Foundation::HANDLE) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFilteringPlatform`, `Win32_Foundation`, `Win32_Security`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub fn IkeextSaDbGetSecurityInfo0(enginehandle: super::super::Foundation::HANDLE, securityinfo: u32, sidowner: *mut super::super::Foundation::PSID, sidgroup: *mut super::super::Foundation::PSID, dacl: *mut *mut super::super::Security::ACL, sacl: *mut *mut super::super::Security::ACL, securitydescriptor: *mut *mut super::super::Security::SECURITY_DESCRIPTOR) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFilteringPlatform`, `Win32_Foundation`, `Win32_Security`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub fn IkeextSaDbSetSecurityInfo0(enginehandle: super::super::Foundation::HANDLE, securityinfo: u32, sidowner: *const super::super::Security::SID, sidgroup: *const super::super::Security::SID, dacl: *const super::super::Security::ACL, sacl: *const super::super::Security::ACL) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFilteringPlatform`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn IkeextSaDeleteById0(enginehandle: super::super::Foundation::HANDLE, id: u64) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFilteringPlatform`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn IkeextSaDestroyEnumHandle0(enginehandle: super::super::Foundation::HANDLE, enumhandle: super::super::Foundation::HANDLE) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFilteringPlatform`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn IkeextSaEnum0(enginehandle: super::super::Foundation::HANDLE, enumhandle: super::super::Foundation::HANDLE, numentriesrequested: u32, entries: *mut *mut *mut IKEEXT_SA_DETAILS0, numentriesreturned: *mut u32) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFilteringPlatform`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn IkeextSaEnum1(enginehandle: super::super::Foundation::HANDLE, enumhandle: super::super::Foundation::HANDLE, numentriesrequested: u32, entries: *mut *mut *mut IKEEXT_SA_DETAILS1, numentriesreturned: *mut u32) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFilteringPlatform`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn IkeextSaEnum2(enginehandle: super::super::Foundation::HANDLE, enumhandle: super::super::Foundation::HANDLE, numentriesrequested: u32, entries: *mut *mut *mut IKEEXT_SA_DETAILS2, numentriesreturned: *mut u32) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFilteringPlatform`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn IkeextSaGetById0(enginehandle: super::super::Foundation::HANDLE, id: u64, sa: *mut *mut IKEEXT_SA_DETAILS0) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFilteringPlatform`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn IkeextSaGetById1(enginehandle: super::super::Foundation::HANDLE, id: u64, salookupcontext: *const ::windows::runtime::GUID, sa: *mut *mut IKEEXT_SA_DETAILS1) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFilteringPlatform`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn IkeextSaGetById2(enginehandle: super::super::Foundation::HANDLE, id: u64, salookupcontext: *const ::windows::runtime::GUID, sa: *mut *mut IKEEXT_SA_DETAILS2) -> u32;
}
