#[cfg(all(feature = "fwpmtypes", feature = "fwptypes", feature = "winnt"))]
windows_link::link!("fwpuclnt.dll" "system" fn FwpmCalloutAdd0(enginehandle : super::HANDLE, callout : *const super::FWPM_CALLOUT0, sd : super::PSECURITY_DESCRIPTOR, id : *mut u32) -> u32);
#[cfg(all(feature = "fwpmtypes", feature = "winnt"))]
windows_link::link!("fwpuclnt.dll" "system" fn FwpmCalloutCreateEnumHandle0(enginehandle : super::HANDLE, enumtemplate : *const super::FWPM_CALLOUT_ENUM_TEMPLATE0, enumhandle : *mut super::HANDLE) -> u32);
#[cfg(feature = "winnt")]
windows_link::link!("fwpuclnt.dll" "system" fn FwpmCalloutDeleteById0(enginehandle : super::HANDLE, id : u32) -> u32);
#[cfg(feature = "winnt")]
windows_link::link!("fwpuclnt.dll" "system" fn FwpmCalloutDeleteByKey0(enginehandle : super::HANDLE, key : *const windows_sys::core::GUID) -> u32);
#[cfg(feature = "winnt")]
windows_link::link!("fwpuclnt.dll" "system" fn FwpmCalloutDestroyEnumHandle0(enginehandle : super::HANDLE, enumhandle : super::HANDLE) -> u32);
#[cfg(all(feature = "fwpmtypes", feature = "fwptypes", feature = "winnt"))]
windows_link::link!("fwpuclnt.dll" "system" fn FwpmCalloutEnum0(enginehandle : super::HANDLE, enumhandle : super::HANDLE, numentriesrequested : u32, entries : *mut *mut *mut super::FWPM_CALLOUT0, numentriesreturned : *mut u32) -> u32);
#[cfg(all(feature = "fwpmtypes", feature = "fwptypes", feature = "winnt"))]
windows_link::link!("fwpuclnt.dll" "system" fn FwpmCalloutGetById0(enginehandle : super::HANDLE, id : u32, callout : *mut *mut super::FWPM_CALLOUT0) -> u32);
#[cfg(all(feature = "fwpmtypes", feature = "fwptypes", feature = "winnt"))]
windows_link::link!("fwpuclnt.dll" "system" fn FwpmCalloutGetByKey0(enginehandle : super::HANDLE, key : *const windows_sys::core::GUID, callout : *mut *mut super::FWPM_CALLOUT0) -> u32);
#[cfg(feature = "winnt")]
windows_link::link!("fwpuclnt.dll" "system" fn FwpmCalloutGetSecurityInfoByKey0(enginehandle : super::HANDLE, key : *const windows_sys::core::GUID, securityinfo : super::SECURITY_INFORMATION, sidowner : *mut super::PSID, sidgroup : *mut super::PSID, dacl : *mut super::PACL, sacl : *mut super::PACL, securitydescriptor : *mut super::PSECURITY_DESCRIPTOR) -> u32);
#[cfg(feature = "winnt")]
windows_link::link!("fwpuclnt.dll" "system" fn FwpmCalloutSetSecurityInfoByKey0(enginehandle : super::HANDLE, key : *const windows_sys::core::GUID, securityinfo : super::SECURITY_INFORMATION, sidowner : *const super::SID, sidgroup : *const super::SID, dacl : *const super::ACL, sacl : *const super::ACL) -> u32);
#[cfg(all(feature = "fwpmtypes", feature = "winnt"))]
windows_link::link!("fwpuclnt.dll" "system" fn FwpmCalloutSubscribeChanges0(enginehandle : super::HANDLE, subscription : *const super::FWPM_CALLOUT_SUBSCRIPTION0, callback : FWPM_CALLOUT_CHANGE_CALLBACK0, context : *const core::ffi::c_void, changehandle : *mut super::HANDLE) -> u32);
#[cfg(all(feature = "fwpmtypes", feature = "winnt"))]
windows_link::link!("fwpuclnt.dll" "system" fn FwpmCalloutSubscriptionsGet0(enginehandle : super::HANDLE, entries : *mut *mut *mut super::FWPM_CALLOUT_SUBSCRIPTION0, numentries : *mut u32) -> u32);
#[cfg(feature = "winnt")]
windows_link::link!("fwpuclnt.dll" "system" fn FwpmCalloutUnsubscribeChanges0(enginehandle : super::HANDLE, changehandle : super::HANDLE) -> u32);
#[cfg(all(feature = "fwpmtypes", feature = "winnt"))]
windows_link::link!("fwpuclnt.dll" "system" fn FwpmConnectionCreateEnumHandle0(enginehandle : super::HANDLE, enumtemplate : *const super::FWPM_CONNECTION_ENUM_TEMPLATE0, enumhandle : *mut super::HANDLE) -> u32);
#[cfg(feature = "winnt")]
windows_link::link!("fwpuclnt.dll" "system" fn FwpmConnectionDestroyEnumHandle0(enginehandle : super::HANDLE, enumhandle : super::HANDLE) -> u32);
#[cfg(all(feature = "fwpmtypes", feature = "fwptypes", feature = "iketypes", feature = "ipsectypes", feature = "minwindef", feature = "winnt"))]
windows_link::link!("fwpuclnt.dll" "system" fn FwpmConnectionEnum0(enginehandle : super::HANDLE, enumhandle : super::HANDLE, numentriesrequested : u32, entries : *mut *mut *mut super::FWPM_CONNECTION0, numentriesreturned : *mut u32) -> u32);
#[cfg(all(feature = "fwpmtypes", feature = "fwptypes", feature = "iketypes", feature = "ipsectypes", feature = "minwindef", feature = "winnt"))]
windows_link::link!("fwpuclnt.dll" "system" fn FwpmConnectionGetById0(enginehandle : super::HANDLE, id : u64, connection : *mut *mut super::FWPM_CONNECTION0) -> u32);
#[cfg(feature = "winnt")]
windows_link::link!("fwpuclnt.dll" "system" fn FwpmConnectionGetSecurityInfo0(enginehandle : super::HANDLE, securityinfo : super::SECURITY_INFORMATION, sidowner : *mut super::PSID, sidgroup : *mut super::PSID, dacl : *mut super::PACL, sacl : *mut super::PACL, securitydescriptor : *mut super::PSECURITY_DESCRIPTOR) -> u32);
#[cfg(all(feature = "fwpmtypes", feature = "fwptypes", feature = "iketypes", feature = "ipsectypes", feature = "winnt"))]
windows_link::link!("fwpuclnt.dll" "system" fn FwpmConnectionPolicyAdd0(enginehandle : super::HANDLE, connectionpolicy : *const super::FWPM_PROVIDER_CONTEXT3, ipversion : super::FWP_IP_VERSION, weight : u64, numfilterconditions : u32, filterconditions : *const super::FWPM_FILTER_CONDITION0, sd : super::PSECURITY_DESCRIPTOR) -> u32);
#[cfg(feature = "winnt")]
windows_link::link!("fwpuclnt.dll" "system" fn FwpmConnectionPolicyDeleteByKey0(enginehandle : super::HANDLE, key : *const windows_sys::core::GUID) -> u32);
#[cfg(feature = "winnt")]
windows_link::link!("fwpuclnt.dll" "system" fn FwpmConnectionSetSecurityInfo0(enginehandle : super::HANDLE, securityinfo : super::SECURITY_INFORMATION, sidowner : *const super::SID, sidgroup : *const super::SID, dacl : *const super::ACL, sacl : *const super::ACL) -> u32);
#[cfg(all(feature = "fwpmtypes", feature = "fwptypes", feature = "iketypes", feature = "ipsectypes", feature = "minwindef", feature = "winnt"))]
windows_link::link!("fwpuclnt.dll" "system" fn FwpmConnectionSubscribe0(enginehandle : super::HANDLE, subscription : *const super::FWPM_CONNECTION_SUBSCRIPTION0, callback : FWPM_CONNECTION_CALLBACK0, context : *const core::ffi::c_void, eventshandle : *mut super::HANDLE) -> u32);
#[cfg(feature = "winnt")]
windows_link::link!("fwpuclnt.dll" "system" fn FwpmConnectionUnsubscribe0(enginehandle : super::HANDLE, eventshandle : super::HANDLE) -> u32);
#[cfg(feature = "winnt")]
windows_link::link!("fwpuclnt.dll" "system" fn FwpmDynamicKeywordSubscribe0(flags : u32, callback : FWPM_DYNAMIC_KEYWORD_CALLBACK0, context : *const core::ffi::c_void, subscriptionhandle : *mut super::HANDLE) -> u32);
#[cfg(feature = "winnt")]
windows_link::link!("fwpuclnt.dll" "system" fn FwpmDynamicKeywordUnsubscribe0(subscriptionhandle : super::HANDLE) -> u32);
#[cfg(feature = "winnt")]
windows_link::link!("fwpuclnt.dll" "system" fn FwpmEngineClose0(enginehandle : super::HANDLE) -> u32);
#[cfg(all(feature = "fwpmtypes", feature = "fwptypes", feature = "winnt"))]
windows_link::link!("fwpuclnt.dll" "system" fn FwpmEngineGetOption0(enginehandle : super::HANDLE, option : super::FWPM_ENGINE_OPTION, value : *mut *mut super::FWP_VALUE0) -> u32);
#[cfg(feature = "winnt")]
windows_link::link!("fwpuclnt.dll" "system" fn FwpmEngineGetSecurityInfo0(enginehandle : super::HANDLE, securityinfo : super::SECURITY_INFORMATION, sidowner : *mut super::PSID, sidgroup : *mut super::PSID, dacl : *mut super::PACL, sacl : *mut super::PACL, securitydescriptor : *mut super::PSECURITY_DESCRIPTOR) -> u32);
#[cfg(all(feature = "fwpmtypes", feature = "fwptypes", feature = "rpc", feature = "winnt"))]
windows_link::link!("fwpuclnt.dll" "system" fn FwpmEngineOpen0(servername : *const u16, authnservice : u32, authidentity : *const super::SEC_WINNT_AUTH_IDENTITY_W, session : *const super::FWPM_SESSION0, enginehandle : *mut super::HANDLE) -> u32);
#[cfg(all(feature = "fwpmtypes", feature = "fwptypes", feature = "winnt"))]
windows_link::link!("fwpuclnt.dll" "system" fn FwpmEngineSetOption0(enginehandle : super::HANDLE, option : super::FWPM_ENGINE_OPTION, newvalue : *const super::FWP_VALUE0) -> u32);
#[cfg(feature = "winnt")]
windows_link::link!("fwpuclnt.dll" "system" fn FwpmEngineSetSecurityInfo0(enginehandle : super::HANDLE, securityinfo : super::SECURITY_INFORMATION, sidowner : *const super::SID, sidgroup : *const super::SID, dacl : *const super::ACL, sacl : *const super::ACL) -> u32);
#[cfg(all(feature = "fwpmtypes", feature = "fwptypes", feature = "winnt"))]
windows_link::link!("fwpuclnt.dll" "system" fn FwpmFilterAdd0(enginehandle : super::HANDLE, filter : *const super::FWPM_FILTER0, sd : super::PSECURITY_DESCRIPTOR, id : *mut u64) -> u32);
#[cfg(all(feature = "fwpmtypes", feature = "fwptypes", feature = "winnt"))]
windows_link::link!("fwpuclnt.dll" "system" fn FwpmFilterCreateEnumHandle0(enginehandle : super::HANDLE, enumtemplate : *const super::FWPM_FILTER_ENUM_TEMPLATE0, enumhandle : *mut super::HANDLE) -> u32);
#[cfg(feature = "winnt")]
windows_link::link!("fwpuclnt.dll" "system" fn FwpmFilterDeleteById0(enginehandle : super::HANDLE, id : u64) -> u32);
#[cfg(feature = "winnt")]
windows_link::link!("fwpuclnt.dll" "system" fn FwpmFilterDeleteByKey0(enginehandle : super::HANDLE, key : *const windows_sys::core::GUID) -> u32);
#[cfg(feature = "winnt")]
windows_link::link!("fwpuclnt.dll" "system" fn FwpmFilterDestroyEnumHandle0(enginehandle : super::HANDLE, enumhandle : super::HANDLE) -> u32);
#[cfg(all(feature = "fwpmtypes", feature = "fwptypes", feature = "winnt"))]
windows_link::link!("fwpuclnt.dll" "system" fn FwpmFilterEnum0(enginehandle : super::HANDLE, enumhandle : super::HANDLE, numentriesrequested : u32, entries : *mut *mut *mut super::FWPM_FILTER0, numentriesreturned : *mut u32) -> u32);
#[cfg(all(feature = "fwpmtypes", feature = "fwptypes", feature = "winnt"))]
windows_link::link!("fwpuclnt.dll" "system" fn FwpmFilterGetById0(enginehandle : super::HANDLE, id : u64, filter : *mut *mut super::FWPM_FILTER0) -> u32);
#[cfg(all(feature = "fwpmtypes", feature = "fwptypes", feature = "winnt"))]
windows_link::link!("fwpuclnt.dll" "system" fn FwpmFilterGetByKey0(enginehandle : super::HANDLE, key : *const windows_sys::core::GUID, filter : *mut *mut super::FWPM_FILTER0) -> u32);
#[cfg(feature = "winnt")]
windows_link::link!("fwpuclnt.dll" "system" fn FwpmFilterGetSecurityInfoByKey0(enginehandle : super::HANDLE, key : *const windows_sys::core::GUID, securityinfo : super::SECURITY_INFORMATION, sidowner : *mut super::PSID, sidgroup : *mut super::PSID, dacl : *mut super::PACL, sacl : *mut super::PACL, securitydescriptor : *mut super::PSECURITY_DESCRIPTOR) -> u32);
#[cfg(feature = "winnt")]
windows_link::link!("fwpuclnt.dll" "system" fn FwpmFilterSetSecurityInfoByKey0(enginehandle : super::HANDLE, key : *const windows_sys::core::GUID, securityinfo : super::SECURITY_INFORMATION, sidowner : *const super::SID, sidgroup : *const super::SID, dacl : *const super::ACL, sacl : *const super::ACL) -> u32);
#[cfg(all(feature = "fwpmtypes", feature = "fwptypes", feature = "winnt"))]
windows_link::link!("fwpuclnt.dll" "system" fn FwpmFilterSubscribeChanges0(enginehandle : super::HANDLE, subscription : *const super::FWPM_FILTER_SUBSCRIPTION0, callback : FWPM_FILTER_CHANGE_CALLBACK0, context : *const core::ffi::c_void, changehandle : *mut super::HANDLE) -> u32);
#[cfg(all(feature = "fwpmtypes", feature = "fwptypes", feature = "winnt"))]
windows_link::link!("fwpuclnt.dll" "system" fn FwpmFilterSubscriptionsGet0(enginehandle : super::HANDLE, entries : *mut *mut *mut super::FWPM_FILTER_SUBSCRIPTION0, numentries : *mut u32) -> u32);
#[cfg(feature = "winnt")]
windows_link::link!("fwpuclnt.dll" "system" fn FwpmFilterUnsubscribeChanges0(enginehandle : super::HANDLE, changehandle : super::HANDLE) -> u32);
windows_link::link!("fwpuclnt.dll" "system" fn FwpmFreeMemory0(p : *mut *mut core::ffi::c_void));
#[cfg(feature = "fwptypes")]
windows_link::link!("fwpuclnt.dll" "system" fn FwpmGetAppIdFromFileName0(filename : windows_sys::core::PCWSTR, appid : *mut *mut super::FWP_BYTE_BLOB) -> u32);
#[cfg(all(feature = "fwpmtypes", feature = "fwptypes", feature = "iketypes", feature = "ipsectypes", feature = "winnt"))]
windows_link::link!("fwpuclnt.dll" "system" fn FwpmIPsecTunnelAdd0(enginehandle : super::HANDLE, flags : u32, mainmodepolicy : *const super::FWPM_PROVIDER_CONTEXT0, tunnelpolicy : *const super::FWPM_PROVIDER_CONTEXT0, numfilterconditions : u32, filterconditions : *const super::FWPM_FILTER_CONDITION0, sd : super::PSECURITY_DESCRIPTOR) -> u32);
#[cfg(all(feature = "fwpmtypes", feature = "fwptypes", feature = "iketypes", feature = "ipsectypes", feature = "winnt"))]
windows_link::link!("fwpuclnt.dll" "system" fn FwpmIPsecTunnelAdd1(enginehandle : super::HANDLE, flags : u32, mainmodepolicy : *const super::FWPM_PROVIDER_CONTEXT1, tunnelpolicy : *const super::FWPM_PROVIDER_CONTEXT1, numfilterconditions : u32, filterconditions : *const super::FWPM_FILTER_CONDITION0, keymodkey : *const windows_sys::core::GUID, sd : super::PSECURITY_DESCRIPTOR) -> u32);
#[cfg(all(feature = "fwpmtypes", feature = "fwptypes", feature = "iketypes", feature = "ipsectypes", feature = "winnt"))]
windows_link::link!("fwpuclnt.dll" "system" fn FwpmIPsecTunnelAdd2(enginehandle : super::HANDLE, flags : u32, mainmodepolicy : *const super::FWPM_PROVIDER_CONTEXT2, tunnelpolicy : *const super::FWPM_PROVIDER_CONTEXT2, numfilterconditions : u32, filterconditions : *const super::FWPM_FILTER_CONDITION0, keymodkey : *const windows_sys::core::GUID, sd : super::PSECURITY_DESCRIPTOR) -> u32);
#[cfg(all(feature = "fwpmtypes", feature = "fwptypes", feature = "iketypes", feature = "ipsectypes", feature = "winnt"))]
windows_link::link!("fwpuclnt.dll" "system" fn FwpmIPsecTunnelAdd3(enginehandle : super::HANDLE, flags : u32, mainmodepolicy : *const super::FWPM_PROVIDER_CONTEXT3, tunnelpolicy : *const super::FWPM_PROVIDER_CONTEXT3, numfilterconditions : u32, filterconditions : *const super::FWPM_FILTER_CONDITION0, keymodkey : *const windows_sys::core::GUID, sd : super::PSECURITY_DESCRIPTOR) -> u32);
#[cfg(feature = "winnt")]
windows_link::link!("fwpuclnt.dll" "system" fn FwpmIPsecTunnelDeleteByKey0(enginehandle : super::HANDLE, key : *const windows_sys::core::GUID) -> u32);
#[cfg(all(feature = "fwpmtypes", feature = "winnt"))]
windows_link::link!("fwpuclnt.dll" "system" fn FwpmLayerCreateEnumHandle0(enginehandle : super::HANDLE, enumtemplate : *const super::FWPM_LAYER_ENUM_TEMPLATE0, enumhandle : *mut super::HANDLE) -> u32);
#[cfg(feature = "winnt")]
windows_link::link!("fwpuclnt.dll" "system" fn FwpmLayerDestroyEnumHandle0(enginehandle : super::HANDLE, enumhandle : super::HANDLE) -> u32);
#[cfg(all(feature = "fwpmtypes", feature = "fwptypes", feature = "winnt"))]
windows_link::link!("fwpuclnt.dll" "system" fn FwpmLayerEnum0(enginehandle : super::HANDLE, enumhandle : super::HANDLE, numentriesrequested : u32, entries : *mut *mut *mut super::FWPM_LAYER0, numentriesreturned : *mut u32) -> u32);
#[cfg(all(feature = "fwpmtypes", feature = "fwptypes", feature = "winnt"))]
windows_link::link!("fwpuclnt.dll" "system" fn FwpmLayerGetById0(enginehandle : super::HANDLE, id : u16, layer : *mut *mut super::FWPM_LAYER0) -> u32);
#[cfg(all(feature = "fwpmtypes", feature = "fwptypes", feature = "winnt"))]
windows_link::link!("fwpuclnt.dll" "system" fn FwpmLayerGetByKey0(enginehandle : super::HANDLE, key : *const windows_sys::core::GUID, layer : *mut *mut super::FWPM_LAYER0) -> u32);
#[cfg(feature = "winnt")]
windows_link::link!("fwpuclnt.dll" "system" fn FwpmLayerGetSecurityInfoByKey0(enginehandle : super::HANDLE, key : *const windows_sys::core::GUID, securityinfo : super::SECURITY_INFORMATION, sidowner : *mut super::PSID, sidgroup : *mut super::PSID, dacl : *mut super::PACL, sacl : *mut super::PACL, securitydescriptor : *mut super::PSECURITY_DESCRIPTOR) -> u32);
#[cfg(feature = "winnt")]
windows_link::link!("fwpuclnt.dll" "system" fn FwpmLayerSetSecurityInfoByKey0(enginehandle : super::HANDLE, key : *const windows_sys::core::GUID, securityinfo : super::SECURITY_INFORMATION, sidowner : *const super::SID, sidgroup : *const super::SID, dacl : *const super::ACL, sacl : *const super::ACL) -> u32);
#[cfg(all(feature = "fwpmtypes", feature = "fwptypes", feature = "minwindef", feature = "winnt"))]
windows_link::link!("fwpuclnt.dll" "system" fn FwpmNetEventCreateEnumHandle0(enginehandle : super::HANDLE, enumtemplate : *const super::FWPM_NET_EVENT_ENUM_TEMPLATE0, enumhandle : *mut super::HANDLE) -> u32);
#[cfg(feature = "winnt")]
windows_link::link!("fwpuclnt.dll" "system" fn FwpmNetEventDestroyEnumHandle0(enginehandle : super::HANDLE, enumhandle : super::HANDLE) -> u32);
#[cfg(all(feature = "fwpmtypes", feature = "fwptypes", feature = "iketypes", feature = "ipsectypes", feature = "minwindef", feature = "winnt"))]
windows_link::link!("fwpuclnt.dll" "system" fn FwpmNetEventEnum0(enginehandle : super::HANDLE, enumhandle : super::HANDLE, numentriesrequested : u32, entries : *mut *mut *mut super::FWPM_NET_EVENT0, numentriesreturned : *mut u32) -> u32);
#[cfg(all(feature = "fwpmtypes", feature = "fwptypes", feature = "iketypes", feature = "ipsectypes", feature = "minwindef", feature = "winnt"))]
windows_link::link!("fwpuclnt.dll" "system" fn FwpmNetEventEnum1(enginehandle : super::HANDLE, enumhandle : super::HANDLE, numentriesrequested : u32, entries : *mut *mut *mut super::FWPM_NET_EVENT1, numentriesreturned : *mut u32) -> u32);
#[cfg(all(feature = "fwpmtypes", feature = "fwptypes", feature = "iketypes", feature = "ipsectypes", feature = "minwindef", feature = "winnt"))]
windows_link::link!("fwpuclnt.dll" "system" fn FwpmNetEventEnum2(enginehandle : super::HANDLE, enumhandle : super::HANDLE, numentriesrequested : u32, entries : *mut *mut *mut super::FWPM_NET_EVENT2, numentriesreturned : *mut u32) -> u32);
#[cfg(all(feature = "fwpmtypes", feature = "fwptypes", feature = "iketypes", feature = "ipsectypes", feature = "minwindef", feature = "winnt"))]
windows_link::link!("fwpuclnt.dll" "system" fn FwpmNetEventEnum3(enginehandle : super::HANDLE, enumhandle : super::HANDLE, numentriesrequested : u32, entries : *mut *mut *mut super::FWPM_NET_EVENT3, numentriesreturned : *mut u32) -> u32);
#[cfg(all(feature = "fwpmtypes", feature = "fwptypes", feature = "iketypes", feature = "ipsectypes", feature = "minwindef", feature = "winnt"))]
windows_link::link!("fwpuclnt.dll" "system" fn FwpmNetEventEnum4(enginehandle : super::HANDLE, enumhandle : super::HANDLE, numentriesrequested : u32, entries : *mut *mut *mut super::FWPM_NET_EVENT4, numentriesreturned : *mut u32) -> u32);
#[cfg(all(feature = "fwpmtypes", feature = "fwptypes", feature = "iketypes", feature = "ipsectypes", feature = "minwindef", feature = "winnt"))]
windows_link::link!("fwpuclnt.dll" "system" fn FwpmNetEventEnum5(enginehandle : super::HANDLE, enumhandle : super::HANDLE, numentriesrequested : u32, entries : *mut *mut *mut super::FWPM_NET_EVENT5, numentriesreturned : *mut u32) -> u32);
#[cfg(all(feature = "fwpmtypes", feature = "fwptypes", feature = "iketypes", feature = "ipsectypes", feature = "minwindef", feature = "winnt"))]
windows_link::link!("fwpuclnt.dll" "system" fn FwpmNetEventSubscribe0(enginehandle : super::HANDLE, subscription : *const super::FWPM_NET_EVENT_SUBSCRIPTION0, callback : FWPM_NET_EVENT_CALLBACK0, context : *const core::ffi::c_void, eventshandle : *mut super::HANDLE) -> u32);
#[cfg(all(feature = "fwpmtypes", feature = "fwptypes", feature = "iketypes", feature = "ipsectypes", feature = "minwindef", feature = "winnt"))]
windows_link::link!("fwpuclnt.dll" "system" fn FwpmNetEventSubscribe1(enginehandle : super::HANDLE, subscription : *const super::FWPM_NET_EVENT_SUBSCRIPTION0, callback : FWPM_NET_EVENT_CALLBACK1, context : *const core::ffi::c_void, eventshandle : *mut super::HANDLE) -> u32);
#[cfg(all(feature = "fwpmtypes", feature = "fwptypes", feature = "iketypes", feature = "ipsectypes", feature = "minwindef", feature = "winnt"))]
windows_link::link!("fwpuclnt.dll" "system" fn FwpmNetEventSubscribe2(enginehandle : super::HANDLE, subscription : *const super::FWPM_NET_EVENT_SUBSCRIPTION0, callback : FWPM_NET_EVENT_CALLBACK2, context : *const core::ffi::c_void, eventshandle : *mut super::HANDLE) -> u32);
#[cfg(all(feature = "fwpmtypes", feature = "fwptypes", feature = "iketypes", feature = "ipsectypes", feature = "minwindef", feature = "winnt"))]
windows_link::link!("fwpuclnt.dll" "system" fn FwpmNetEventSubscribe3(enginehandle : super::HANDLE, subscription : *const super::FWPM_NET_EVENT_SUBSCRIPTION0, callback : FWPM_NET_EVENT_CALLBACK3, context : *const core::ffi::c_void, eventshandle : *mut super::HANDLE) -> u32);
#[cfg(all(feature = "fwpmtypes", feature = "fwptypes", feature = "iketypes", feature = "ipsectypes", feature = "minwindef", feature = "winnt"))]
windows_link::link!("fwpuclnt.dll" "system" fn FwpmNetEventSubscribe4(enginehandle : super::HANDLE, subscription : *const super::FWPM_NET_EVENT_SUBSCRIPTION0, callback : FWPM_NET_EVENT_CALLBACK4, context : *const core::ffi::c_void, eventshandle : *mut super::HANDLE) -> u32);
#[cfg(all(feature = "fwpmtypes", feature = "fwptypes", feature = "minwindef", feature = "winnt"))]
windows_link::link!("fwpuclnt.dll" "system" fn FwpmNetEventSubscriptionsGet0(enginehandle : super::HANDLE, entries : *mut *mut *mut super::FWPM_NET_EVENT_SUBSCRIPTION0, numentries : *mut u32) -> u32);
#[cfg(feature = "winnt")]
windows_link::link!("fwpuclnt.dll" "system" fn FwpmNetEventUnsubscribe0(enginehandle : super::HANDLE, eventshandle : super::HANDLE) -> u32);
#[cfg(feature = "winnt")]
windows_link::link!("fwpuclnt.dll" "system" fn FwpmNetEventsGetSecurityInfo0(enginehandle : super::HANDLE, securityinfo : super::SECURITY_INFORMATION, sidowner : *mut super::PSID, sidgroup : *mut super::PSID, dacl : *mut super::PACL, sacl : *mut super::PACL, securitydescriptor : *mut super::PSECURITY_DESCRIPTOR) -> u32);
#[cfg(feature = "winnt")]
windows_link::link!("fwpuclnt.dll" "system" fn FwpmNetEventsSetSecurityInfo0(enginehandle : super::HANDLE, securityinfo : super::SECURITY_INFORMATION, sidowner : *const super::SID, sidgroup : *const super::SID, dacl : *const super::ACL, sacl : *const super::ACL) -> u32);
#[cfg(all(feature = "fwpmtypes", feature = "fwptypes", feature = "winnt"))]
windows_link::link!("fwpuclnt.dll" "system" fn FwpmProviderAdd0(enginehandle : super::HANDLE, provider : *const super::FWPM_PROVIDER0, sd : super::PSECURITY_DESCRIPTOR) -> u32);
#[cfg(all(feature = "fwpmtypes", feature = "fwptypes", feature = "iketypes", feature = "ipsectypes", feature = "winnt"))]
windows_link::link!("fwpuclnt.dll" "system" fn FwpmProviderContextAdd0(enginehandle : super::HANDLE, providercontext : *const super::FWPM_PROVIDER_CONTEXT0, sd : super::PSECURITY_DESCRIPTOR, id : *mut u64) -> u32);
#[cfg(all(feature = "fwpmtypes", feature = "fwptypes", feature = "iketypes", feature = "ipsectypes", feature = "winnt"))]
windows_link::link!("fwpuclnt.dll" "system" fn FwpmProviderContextAdd1(enginehandle : super::HANDLE, providercontext : *const super::FWPM_PROVIDER_CONTEXT1, sd : super::PSECURITY_DESCRIPTOR, id : *mut u64) -> u32);
#[cfg(all(feature = "fwpmtypes", feature = "fwptypes", feature = "iketypes", feature = "ipsectypes", feature = "winnt"))]
windows_link::link!("fwpuclnt.dll" "system" fn FwpmProviderContextAdd2(enginehandle : super::HANDLE, providercontext : *const super::FWPM_PROVIDER_CONTEXT2, sd : super::PSECURITY_DESCRIPTOR, id : *mut u64) -> u32);
#[cfg(all(feature = "fwpmtypes", feature = "fwptypes", feature = "iketypes", feature = "ipsectypes", feature = "winnt"))]
windows_link::link!("fwpuclnt.dll" "system" fn FwpmProviderContextAdd3(enginehandle : super::HANDLE, providercontext : *const super::FWPM_PROVIDER_CONTEXT3, sd : super::PSECURITY_DESCRIPTOR, id : *mut u64) -> u32);
#[cfg(all(feature = "fwpmtypes", feature = "winnt"))]
windows_link::link!("fwpuclnt.dll" "system" fn FwpmProviderContextCreateEnumHandle0(enginehandle : super::HANDLE, enumtemplate : *const super::FWPM_PROVIDER_CONTEXT_ENUM_TEMPLATE0, enumhandle : *mut super::HANDLE) -> u32);
#[cfg(feature = "winnt")]
windows_link::link!("fwpuclnt.dll" "system" fn FwpmProviderContextDeleteById0(enginehandle : super::HANDLE, id : u64) -> u32);
#[cfg(feature = "winnt")]
windows_link::link!("fwpuclnt.dll" "system" fn FwpmProviderContextDeleteByKey0(enginehandle : super::HANDLE, key : *const windows_sys::core::GUID) -> u32);
#[cfg(feature = "winnt")]
windows_link::link!("fwpuclnt.dll" "system" fn FwpmProviderContextDestroyEnumHandle0(enginehandle : super::HANDLE, enumhandle : super::HANDLE) -> u32);
#[cfg(all(feature = "fwpmtypes", feature = "fwptypes", feature = "iketypes", feature = "ipsectypes", feature = "winnt"))]
windows_link::link!("fwpuclnt.dll" "system" fn FwpmProviderContextEnum0(enginehandle : super::HANDLE, enumhandle : super::HANDLE, numentriesrequested : u32, entries : *mut *mut *mut super::FWPM_PROVIDER_CONTEXT0, numentriesreturned : *mut u32) -> u32);
#[cfg(all(feature = "fwpmtypes", feature = "fwptypes", feature = "iketypes", feature = "ipsectypes", feature = "winnt"))]
windows_link::link!("fwpuclnt.dll" "system" fn FwpmProviderContextEnum1(enginehandle : super::HANDLE, enumhandle : super::HANDLE, numentriesrequested : u32, entries : *mut *mut *mut super::FWPM_PROVIDER_CONTEXT1, numentriesreturned : *mut u32) -> u32);
#[cfg(all(feature = "fwpmtypes", feature = "fwptypes", feature = "iketypes", feature = "ipsectypes", feature = "winnt"))]
windows_link::link!("fwpuclnt.dll" "system" fn FwpmProviderContextEnum2(enginehandle : super::HANDLE, enumhandle : super::HANDLE, numentriesrequested : u32, entries : *mut *mut *mut super::FWPM_PROVIDER_CONTEXT2, numentriesreturned : *mut u32) -> u32);
#[cfg(all(feature = "fwpmtypes", feature = "fwptypes", feature = "iketypes", feature = "ipsectypes", feature = "winnt"))]
windows_link::link!("fwpuclnt.dll" "system" fn FwpmProviderContextEnum3(enginehandle : super::HANDLE, enumhandle : super::HANDLE, numentriesrequested : u32, entries : *mut *mut *mut super::FWPM_PROVIDER_CONTEXT3, numentriesreturned : *mut u32) -> u32);
#[cfg(all(feature = "fwpmtypes", feature = "fwptypes", feature = "iketypes", feature = "ipsectypes", feature = "winnt"))]
windows_link::link!("fwpuclnt.dll" "system" fn FwpmProviderContextGetById0(enginehandle : super::HANDLE, id : u64, providercontext : *mut *mut super::FWPM_PROVIDER_CONTEXT0) -> u32);
#[cfg(all(feature = "fwpmtypes", feature = "fwptypes", feature = "iketypes", feature = "ipsectypes", feature = "winnt"))]
windows_link::link!("fwpuclnt.dll" "system" fn FwpmProviderContextGetById1(enginehandle : super::HANDLE, id : u64, providercontext : *mut *mut super::FWPM_PROVIDER_CONTEXT1) -> u32);
#[cfg(all(feature = "fwpmtypes", feature = "fwptypes", feature = "iketypes", feature = "ipsectypes", feature = "winnt"))]
windows_link::link!("fwpuclnt.dll" "system" fn FwpmProviderContextGetById2(enginehandle : super::HANDLE, id : u64, providercontext : *mut *mut super::FWPM_PROVIDER_CONTEXT2) -> u32);
#[cfg(all(feature = "fwpmtypes", feature = "fwptypes", feature = "iketypes", feature = "ipsectypes", feature = "winnt"))]
windows_link::link!("fwpuclnt.dll" "system" fn FwpmProviderContextGetById3(enginehandle : super::HANDLE, id : u64, providercontext : *mut *mut super::FWPM_PROVIDER_CONTEXT3) -> u32);
#[cfg(all(feature = "fwpmtypes", feature = "fwptypes", feature = "iketypes", feature = "ipsectypes", feature = "winnt"))]
windows_link::link!("fwpuclnt.dll" "system" fn FwpmProviderContextGetByKey0(enginehandle : super::HANDLE, key : *const windows_sys::core::GUID, providercontext : *mut *mut super::FWPM_PROVIDER_CONTEXT0) -> u32);
#[cfg(all(feature = "fwpmtypes", feature = "fwptypes", feature = "iketypes", feature = "ipsectypes", feature = "winnt"))]
windows_link::link!("fwpuclnt.dll" "system" fn FwpmProviderContextGetByKey1(enginehandle : super::HANDLE, key : *const windows_sys::core::GUID, providercontext : *mut *mut super::FWPM_PROVIDER_CONTEXT1) -> u32);
#[cfg(all(feature = "fwpmtypes", feature = "fwptypes", feature = "iketypes", feature = "ipsectypes", feature = "winnt"))]
windows_link::link!("fwpuclnt.dll" "system" fn FwpmProviderContextGetByKey2(enginehandle : super::HANDLE, key : *const windows_sys::core::GUID, providercontext : *mut *mut super::FWPM_PROVIDER_CONTEXT2) -> u32);
#[cfg(all(feature = "fwpmtypes", feature = "fwptypes", feature = "iketypes", feature = "ipsectypes", feature = "winnt"))]
windows_link::link!("fwpuclnt.dll" "system" fn FwpmProviderContextGetByKey3(enginehandle : super::HANDLE, key : *const windows_sys::core::GUID, providercontext : *mut *mut super::FWPM_PROVIDER_CONTEXT3) -> u32);
#[cfg(feature = "winnt")]
windows_link::link!("fwpuclnt.dll" "system" fn FwpmProviderContextGetSecurityInfoByKey0(enginehandle : super::HANDLE, key : *const windows_sys::core::GUID, securityinfo : super::SECURITY_INFORMATION, sidowner : *mut super::PSID, sidgroup : *mut super::PSID, dacl : *mut super::PACL, sacl : *mut super::PACL, securitydescriptor : *mut super::PSECURITY_DESCRIPTOR) -> u32);
#[cfg(feature = "winnt")]
windows_link::link!("fwpuclnt.dll" "system" fn FwpmProviderContextSetSecurityInfoByKey0(enginehandle : super::HANDLE, key : *const windows_sys::core::GUID, securityinfo : super::SECURITY_INFORMATION, sidowner : *const super::SID, sidgroup : *const super::SID, dacl : *const super::ACL, sacl : *const super::ACL) -> u32);
#[cfg(all(feature = "fwpmtypes", feature = "winnt"))]
windows_link::link!("fwpuclnt.dll" "system" fn FwpmProviderContextSubscribeChanges0(enginehandle : super::HANDLE, subscription : *const super::FWPM_PROVIDER_CONTEXT_SUBSCRIPTION0, callback : FWPM_PROVIDER_CONTEXT_CHANGE_CALLBACK0, context : *const core::ffi::c_void, changehandle : *mut super::HANDLE) -> u32);
#[cfg(all(feature = "fwpmtypes", feature = "winnt"))]
windows_link::link!("fwpuclnt.dll" "system" fn FwpmProviderContextSubscriptionsGet0(enginehandle : super::HANDLE, entries : *mut *mut *mut super::FWPM_PROVIDER_CONTEXT_SUBSCRIPTION0, numentries : *mut u32) -> u32);
#[cfg(feature = "winnt")]
windows_link::link!("fwpuclnt.dll" "system" fn FwpmProviderContextUnsubscribeChanges0(enginehandle : super::HANDLE, changehandle : super::HANDLE) -> u32);
#[cfg(all(feature = "fwpmtypes", feature = "winnt"))]
windows_link::link!("fwpuclnt.dll" "system" fn FwpmProviderCreateEnumHandle0(enginehandle : super::HANDLE, enumtemplate : *const super::FWPM_PROVIDER_ENUM_TEMPLATE0, enumhandle : *mut super::HANDLE) -> u32);
#[cfg(feature = "winnt")]
windows_link::link!("fwpuclnt.dll" "system" fn FwpmProviderDeleteByKey0(enginehandle : super::HANDLE, key : *const windows_sys::core::GUID) -> u32);
#[cfg(feature = "winnt")]
windows_link::link!("fwpuclnt.dll" "system" fn FwpmProviderDestroyEnumHandle0(enginehandle : super::HANDLE, enumhandle : super::HANDLE) -> u32);
#[cfg(all(feature = "fwpmtypes", feature = "fwptypes", feature = "winnt"))]
windows_link::link!("fwpuclnt.dll" "system" fn FwpmProviderEnum0(enginehandle : super::HANDLE, enumhandle : super::HANDLE, numentriesrequested : u32, entries : *mut *mut *mut super::FWPM_PROVIDER0, numentriesreturned : *mut u32) -> u32);
#[cfg(all(feature = "fwpmtypes", feature = "fwptypes", feature = "winnt"))]
windows_link::link!("fwpuclnt.dll" "system" fn FwpmProviderGetByKey0(enginehandle : super::HANDLE, key : *const windows_sys::core::GUID, provider : *mut *mut super::FWPM_PROVIDER0) -> u32);
#[cfg(feature = "winnt")]
windows_link::link!("fwpuclnt.dll" "system" fn FwpmProviderGetSecurityInfoByKey0(enginehandle : super::HANDLE, key : *const windows_sys::core::GUID, securityinfo : super::SECURITY_INFORMATION, sidowner : *mut super::PSID, sidgroup : *mut super::PSID, dacl : *mut super::PACL, sacl : *mut super::PACL, securitydescriptor : *mut super::PSECURITY_DESCRIPTOR) -> u32);
#[cfg(feature = "winnt")]
windows_link::link!("fwpuclnt.dll" "system" fn FwpmProviderSetSecurityInfoByKey0(enginehandle : super::HANDLE, key : *const windows_sys::core::GUID, securityinfo : super::SECURITY_INFORMATION, sidowner : *const super::SID, sidgroup : *const super::SID, dacl : *const super::ACL, sacl : *const super::ACL) -> u32);
#[cfg(all(feature = "fwpmtypes", feature = "winnt"))]
windows_link::link!("fwpuclnt.dll" "system" fn FwpmProviderSubscribeChanges0(enginehandle : super::HANDLE, subscription : *const super::FWPM_PROVIDER_SUBSCRIPTION0, callback : FWPM_PROVIDER_CHANGE_CALLBACK0, context : *const core::ffi::c_void, changehandle : *mut super::HANDLE) -> u32);
#[cfg(all(feature = "fwpmtypes", feature = "winnt"))]
windows_link::link!("fwpuclnt.dll" "system" fn FwpmProviderSubscriptionsGet0(enginehandle : super::HANDLE, entries : *mut *mut *mut super::FWPM_PROVIDER_SUBSCRIPTION0, numentries : *mut u32) -> u32);
#[cfg(feature = "winnt")]
windows_link::link!("fwpuclnt.dll" "system" fn FwpmProviderUnsubscribeChanges0(enginehandle : super::HANDLE, changehandle : super::HANDLE) -> u32);
#[cfg(all(feature = "fwpmtypes", feature = "winnt"))]
windows_link::link!("fwpuclnt.dll" "system" fn FwpmSessionCreateEnumHandle0(enginehandle : super::HANDLE, enumtemplate : *const super::FWPM_SESSION_ENUM_TEMPLATE0, enumhandle : *mut super::HANDLE) -> u32);
#[cfg(feature = "winnt")]
windows_link::link!("fwpuclnt.dll" "system" fn FwpmSessionDestroyEnumHandle0(enginehandle : super::HANDLE, enumhandle : super::HANDLE) -> u32);
#[cfg(all(feature = "fwpmtypes", feature = "fwptypes", feature = "winnt"))]
windows_link::link!("fwpuclnt.dll" "system" fn FwpmSessionEnum0(enginehandle : super::HANDLE, enumhandle : super::HANDLE, numentriesrequested : u32, entries : *mut *mut *mut super::FWPM_SESSION0, numentriesreturned : *mut u32) -> u32);
#[cfg(all(feature = "fwpmtypes", feature = "fwptypes", feature = "winnt"))]
windows_link::link!("fwpuclnt.dll" "system" fn FwpmSubLayerAdd0(enginehandle : super::HANDLE, sublayer : *const super::FWPM_SUBLAYER0, sd : super::PSECURITY_DESCRIPTOR) -> u32);
#[cfg(all(feature = "fwpmtypes", feature = "winnt"))]
windows_link::link!("fwpuclnt.dll" "system" fn FwpmSubLayerCreateEnumHandle0(enginehandle : super::HANDLE, enumtemplate : *const super::FWPM_SUBLAYER_ENUM_TEMPLATE0, enumhandle : *mut super::HANDLE) -> u32);
#[cfg(feature = "winnt")]
windows_link::link!("fwpuclnt.dll" "system" fn FwpmSubLayerDeleteByKey0(enginehandle : super::HANDLE, key : *const windows_sys::core::GUID) -> u32);
#[cfg(feature = "winnt")]
windows_link::link!("fwpuclnt.dll" "system" fn FwpmSubLayerDestroyEnumHandle0(enginehandle : super::HANDLE, enumhandle : super::HANDLE) -> u32);
#[cfg(all(feature = "fwpmtypes", feature = "fwptypes", feature = "winnt"))]
windows_link::link!("fwpuclnt.dll" "system" fn FwpmSubLayerEnum0(enginehandle : super::HANDLE, enumhandle : super::HANDLE, numentriesrequested : u32, entries : *mut *mut *mut super::FWPM_SUBLAYER0, numentriesreturned : *mut u32) -> u32);
#[cfg(all(feature = "fwpmtypes", feature = "fwptypes", feature = "winnt"))]
windows_link::link!("fwpuclnt.dll" "system" fn FwpmSubLayerGetByKey0(enginehandle : super::HANDLE, key : *const windows_sys::core::GUID, sublayer : *mut *mut super::FWPM_SUBLAYER0) -> u32);
#[cfg(feature = "winnt")]
windows_link::link!("fwpuclnt.dll" "system" fn FwpmSubLayerGetSecurityInfoByKey0(enginehandle : super::HANDLE, key : *const windows_sys::core::GUID, securityinfo : super::SECURITY_INFORMATION, sidowner : *mut super::PSID, sidgroup : *mut super::PSID, dacl : *mut super::PACL, sacl : *mut super::PACL, securitydescriptor : *mut super::PSECURITY_DESCRIPTOR) -> u32);
#[cfg(feature = "winnt")]
windows_link::link!("fwpuclnt.dll" "system" fn FwpmSubLayerSetSecurityInfoByKey0(enginehandle : super::HANDLE, key : *const windows_sys::core::GUID, securityinfo : super::SECURITY_INFORMATION, sidowner : *const super::SID, sidgroup : *const super::SID, dacl : *const super::ACL, sacl : *const super::ACL) -> u32);
#[cfg(all(feature = "fwpmtypes", feature = "winnt"))]
windows_link::link!("fwpuclnt.dll" "system" fn FwpmSubLayerSubscribeChanges0(enginehandle : super::HANDLE, subscription : *const super::FWPM_SUBLAYER_SUBSCRIPTION0, callback : FWPM_SUBLAYER_CHANGE_CALLBACK0, context : *const core::ffi::c_void, changehandle : *mut super::HANDLE) -> u32);
#[cfg(all(feature = "fwpmtypes", feature = "winnt"))]
windows_link::link!("fwpuclnt.dll" "system" fn FwpmSubLayerSubscriptionsGet0(enginehandle : super::HANDLE, entries : *mut *mut *mut super::FWPM_SUBLAYER_SUBSCRIPTION0, numentries : *mut u32) -> u32);
#[cfg(feature = "winnt")]
windows_link::link!("fwpuclnt.dll" "system" fn FwpmSubLayerUnsubscribeChanges0(enginehandle : super::HANDLE, changehandle : super::HANDLE) -> u32);
#[cfg(all(feature = "fwpmtypes", feature = "winnt"))]
windows_link::link!("fwpuclnt.dll" "system" fn FwpmSystemPortsGet0(enginehandle : super::HANDLE, sysports : *mut *mut super::FWPM_SYSTEM_PORTS0) -> u32);
#[cfg(all(feature = "fwpmtypes", feature = "winnt"))]
windows_link::link!("fwpuclnt.dll" "system" fn FwpmSystemPortsSubscribe0(enginehandle : super::HANDLE, reserved : *const core::ffi::c_void, callback : FWPM_SYSTEM_PORTS_CALLBACK0, context : *const core::ffi::c_void, sysportshandle : *mut super::HANDLE) -> u32);
#[cfg(feature = "winnt")]
windows_link::link!("fwpuclnt.dll" "system" fn FwpmSystemPortsUnsubscribe0(enginehandle : super::HANDLE, sysportshandle : super::HANDLE) -> u32);
#[cfg(feature = "winnt")]
windows_link::link!("fwpuclnt.dll" "system" fn FwpmTransactionAbort0(enginehandle : super::HANDLE) -> u32);
#[cfg(feature = "winnt")]
windows_link::link!("fwpuclnt.dll" "system" fn FwpmTransactionBegin0(enginehandle : super::HANDLE, flags : u32) -> u32);
#[cfg(feature = "winnt")]
windows_link::link!("fwpuclnt.dll" "system" fn FwpmTransactionCommit0(enginehandle : super::HANDLE) -> u32);
#[cfg(all(feature = "fwpmtypes", feature = "winnt"))]
windows_link::link!("fwpuclnt.dll" "system" fn FwpmvSwitchEventSubscribe0(enginehandle : super::HANDLE, subscription : *const super::FWPM_VSWITCH_EVENT_SUBSCRIPTION0, callback : FWPM_VSWITCH_EVENT_CALLBACK0, context : *const core::ffi::c_void, subscriptionhandle : *mut super::HANDLE) -> u32);
#[cfg(feature = "winnt")]
windows_link::link!("fwpuclnt.dll" "system" fn FwpmvSwitchEventUnsubscribe0(enginehandle : super::HANDLE, subscriptionhandle : super::HANDLE) -> u32);
#[cfg(feature = "winnt")]
windows_link::link!("fwpuclnt.dll" "system" fn FwpmvSwitchEventsGetSecurityInfo0(enginehandle : super::HANDLE, securityinfo : super::SECURITY_INFORMATION, sidowner : *mut super::PSID, sidgroup : *mut super::PSID, dacl : *mut super::PACL, sacl : *mut super::PACL, securitydescriptor : *mut super::PSECURITY_DESCRIPTOR) -> u32);
#[cfg(feature = "winnt")]
windows_link::link!("fwpuclnt.dll" "system" fn FwpmvSwitchEventsSetSecurityInfo0(enginehandle : super::HANDLE, securityinfo : super::SECURITY_INFORMATION, sidowner : *const super::SID, sidgroup : *const super::SID, dacl : *const super::ACL, sacl : *const super::ACL) -> u32);
#[cfg(feature = "winnt")]
windows_link::link!("fwpuclnt.dll" "system" fn IPsecDospGetSecurityInfo0(enginehandle : super::HANDLE, securityinfo : super::SECURITY_INFORMATION, sidowner : *mut super::PSID, sidgroup : *mut super::PSID, dacl : *mut super::PACL, sacl : *mut super::PACL, securitydescriptor : *mut super::PSECURITY_DESCRIPTOR) -> u32);
#[cfg(all(feature = "ipsectypes", feature = "winnt"))]
windows_link::link!("fwpuclnt.dll" "system" fn IPsecDospGetStatistics0(enginehandle : super::HANDLE, idpstatistics : *mut super::IPSEC_DOSP_STATISTICS0) -> u32);
#[cfg(feature = "winnt")]
windows_link::link!("fwpuclnt.dll" "system" fn IPsecDospSetSecurityInfo0(enginehandle : super::HANDLE, securityinfo : super::SECURITY_INFORMATION, sidowner : *const super::SID, sidgroup : *const super::SID, dacl : *const super::ACL, sacl : *const super::ACL) -> u32);
#[cfg(all(feature = "fwptypes", feature = "ipsectypes", feature = "winnt"))]
windows_link::link!("fwpuclnt.dll" "system" fn IPsecDospStateCreateEnumHandle0(enginehandle : super::HANDLE, enumtemplate : *const super::IPSEC_DOSP_STATE_ENUM_TEMPLATE0, enumhandle : *mut super::HANDLE) -> u32);
#[cfg(feature = "winnt")]
windows_link::link!("fwpuclnt.dll" "system" fn IPsecDospStateDestroyEnumHandle0(enginehandle : super::HANDLE, enumhandle : super::HANDLE) -> u32);
#[cfg(all(feature = "ipsectypes", feature = "winnt"))]
windows_link::link!("fwpuclnt.dll" "system" fn IPsecDospStateEnum0(enginehandle : super::HANDLE, enumhandle : super::HANDLE, numentriesrequested : u32, entries : *mut *mut *mut super::IPSEC_DOSP_STATE0, numentries : *mut u32) -> u32);
#[cfg(all(feature = "ipsectypes", feature = "winnt"))]
windows_link::link!("fwpuclnt.dll" "system" fn IPsecGetStatistics0(enginehandle : super::HANDLE, ipsecstatistics : *mut super::IPSEC_STATISTICS0) -> u32);
#[cfg(all(feature = "ipsectypes", feature = "winnt"))]
windows_link::link!("fwpuclnt.dll" "system" fn IPsecGetStatistics1(enginehandle : super::HANDLE, ipsecstatistics : *mut super::IPSEC_STATISTICS1) -> u32);
#[cfg(all(feature = "fwpmtypes", feature = "fwptypes", feature = "iketypes", feature = "ipsectypes", feature = "winnt"))]
windows_link::link!("fwpuclnt.dll" "system" fn IPsecKeyManagerAddAndRegister0(enginehandle : super::HANDLE, keymanager : *const super::IPSEC_KEY_MANAGER0, keymanagercallbacks : *const IPSEC_KEY_MANAGER_CALLBACKS0, keymgmthandle : *mut super::HANDLE) -> u32);
#[cfg(feature = "winnt")]
windows_link::link!("fwpuclnt.dll" "system" fn IPsecKeyManagerGetSecurityInfoByKey0(enginehandle : super::HANDLE, reserved : *const core::ffi::c_void, securityinfo : super::SECURITY_INFORMATION, sidowner : *mut super::PSID, sidgroup : *mut super::PSID, dacl : *mut super::PACL, sacl : *mut super::PACL, securitydescriptor : *mut super::PSECURITY_DESCRIPTOR) -> u32);
#[cfg(feature = "winnt")]
windows_link::link!("fwpuclnt.dll" "system" fn IPsecKeyManagerSetSecurityInfoByKey0(enginehandle : super::HANDLE, reserved : *const core::ffi::c_void, securityinfo : super::SECURITY_INFORMATION, sidowner : *const super::SID, sidgroup : *const super::SID, dacl : *const super::ACL, sacl : *const super::ACL) -> u32);
#[cfg(feature = "winnt")]
windows_link::link!("fwpuclnt.dll" "system" fn IPsecKeyManagerUnregisterAndDelete0(enginehandle : super::HANDLE, keymgmthandle : super::HANDLE) -> u32);
#[cfg(all(feature = "fwptypes", feature = "ipsectypes", feature = "winnt"))]
windows_link::link!("fwpuclnt.dll" "system" fn IPsecKeyManagersGet0(enginehandle : super::HANDLE, entries : *mut *mut *mut super::IPSEC_KEY_MANAGER0, numentries : *mut u32) -> u32);
#[cfg(all(feature = "fwptypes", feature = "ipsectypes", feature = "winnt"))]
windows_link::link!("fwpuclnt.dll" "system" fn IPsecSaContextAddInbound0(enginehandle : super::HANDLE, id : u64, inboundbundle : *const super::IPSEC_SA_BUNDLE0) -> u32);
#[cfg(all(feature = "fwptypes", feature = "ipsectypes", feature = "winnt"))]
windows_link::link!("fwpuclnt.dll" "system" fn IPsecSaContextAddInbound1(enginehandle : super::HANDLE, id : u64, inboundbundle : *const super::IPSEC_SA_BUNDLE1) -> u32);
#[cfg(all(feature = "fwptypes", feature = "ipsectypes", feature = "winnt"))]
windows_link::link!("fwpuclnt.dll" "system" fn IPsecSaContextAddOutbound0(enginehandle : super::HANDLE, id : u64, outboundbundle : *const super::IPSEC_SA_BUNDLE0) -> u32);
#[cfg(all(feature = "fwptypes", feature = "ipsectypes", feature = "winnt"))]
windows_link::link!("fwpuclnt.dll" "system" fn IPsecSaContextAddOutbound1(enginehandle : super::HANDLE, id : u64, outboundbundle : *const super::IPSEC_SA_BUNDLE1) -> u32);
#[cfg(all(feature = "fwptypes", feature = "ipsectypes", feature = "winnt"))]
windows_link::link!("fwpuclnt.dll" "system" fn IPsecSaContextCreate0(enginehandle : super::HANDLE, outboundtraffic : *const super::IPSEC_TRAFFIC0, inboundfilterid : *mut u64, id : *mut u64) -> u32);
#[cfg(all(feature = "fwptypes", feature = "ipsectypes", feature = "winnt"))]
windows_link::link!("fwpuclnt.dll" "system" fn IPsecSaContextCreate1(enginehandle : super::HANDLE, outboundtraffic : *const super::IPSEC_TRAFFIC1, virtualiftunnelinfo : *const super::IPSEC_VIRTUAL_IF_TUNNEL_INFO0, inboundfilterid : *mut u64, id : *mut u64) -> u32);
#[cfg(all(feature = "fwptypes", feature = "ipsectypes", feature = "winnt"))]
windows_link::link!("fwpuclnt.dll" "system" fn IPsecSaContextCreateEnumHandle0(enginehandle : super::HANDLE, enumtemplate : *const super::IPSEC_SA_CONTEXT_ENUM_TEMPLATE0, enumhandle : *mut super::HANDLE) -> u32);
#[cfg(feature = "winnt")]
windows_link::link!("fwpuclnt.dll" "system" fn IPsecSaContextDeleteById0(enginehandle : super::HANDLE, id : u64) -> u32);
#[cfg(feature = "winnt")]
windows_link::link!("fwpuclnt.dll" "system" fn IPsecSaContextDestroyEnumHandle0(enginehandle : super::HANDLE, enumhandle : super::HANDLE) -> u32);
#[cfg(all(feature = "fwpmtypes", feature = "fwptypes", feature = "ipsectypes", feature = "winnt"))]
windows_link::link!("fwpuclnt.dll" "system" fn IPsecSaContextEnum0(enginehandle : super::HANDLE, enumhandle : super::HANDLE, numentriesrequested : u32, entries : *mut *mut *mut super::IPSEC_SA_CONTEXT0, numentriesreturned : *mut u32) -> u32);
#[cfg(all(feature = "fwpmtypes", feature = "fwptypes", feature = "ipsectypes", feature = "winnt"))]
windows_link::link!("fwpuclnt.dll" "system" fn IPsecSaContextEnum1(enginehandle : super::HANDLE, enumhandle : super::HANDLE, numentriesrequested : u32, entries : *mut *mut *mut super::IPSEC_SA_CONTEXT1, numentriesreturned : *mut u32) -> u32);
#[cfg(feature = "winnt")]
windows_link::link!("fwpuclnt.dll" "system" fn IPsecSaContextExpire0(enginehandle : super::HANDLE, id : u64) -> u32);
#[cfg(all(feature = "fwpmtypes", feature = "fwptypes", feature = "ipsectypes", feature = "winnt"))]
windows_link::link!("fwpuclnt.dll" "system" fn IPsecSaContextGetById0(enginehandle : super::HANDLE, id : u64, sacontext : *mut *mut super::IPSEC_SA_CONTEXT0) -> u32);
#[cfg(all(feature = "fwpmtypes", feature = "fwptypes", feature = "ipsectypes", feature = "winnt"))]
windows_link::link!("fwpuclnt.dll" "system" fn IPsecSaContextGetById1(enginehandle : super::HANDLE, id : u64, sacontext : *mut *mut super::IPSEC_SA_CONTEXT1) -> u32);
#[cfg(all(feature = "fwptypes", feature = "ipsectypes", feature = "winnt"))]
windows_link::link!("fwpuclnt.dll" "system" fn IPsecSaContextGetSpi0(enginehandle : super::HANDLE, id : u64, getspi : *const super::IPSEC_GETSPI0, inboundspi : *mut super::IPSEC_SA_SPI) -> u32);
#[cfg(all(feature = "fwptypes", feature = "ipsectypes", feature = "winnt"))]
windows_link::link!("fwpuclnt.dll" "system" fn IPsecSaContextGetSpi1(enginehandle : super::HANDLE, id : u64, getspi : *const super::IPSEC_GETSPI1, inboundspi : *mut super::IPSEC_SA_SPI) -> u32);
#[cfg(all(feature = "fwptypes", feature = "ipsectypes", feature = "winnt"))]
windows_link::link!("fwpuclnt.dll" "system" fn IPsecSaContextSetSpi0(enginehandle : super::HANDLE, id : u64, getspi : *const super::IPSEC_GETSPI1, inboundspi : super::IPSEC_SA_SPI) -> u32);
#[cfg(all(feature = "fwptypes", feature = "ipsectypes", feature = "winnt"))]
windows_link::link!("fwpuclnt.dll" "system" fn IPsecSaContextSubscribe0(enginehandle : super::HANDLE, subscription : *const super::IPSEC_SA_CONTEXT_SUBSCRIPTION0, callback : IPSEC_SA_CONTEXT_CALLBACK0, context : *const core::ffi::c_void, eventshandle : *mut super::HANDLE) -> u32);
#[cfg(all(feature = "fwptypes", feature = "ipsectypes", feature = "winnt"))]
windows_link::link!("fwpuclnt.dll" "system" fn IPsecSaContextSubscriptionsGet0(enginehandle : super::HANDLE, entries : *mut *mut *mut super::IPSEC_SA_CONTEXT_SUBSCRIPTION0, numentries : *mut u32) -> u32);
#[cfg(feature = "winnt")]
windows_link::link!("fwpuclnt.dll" "system" fn IPsecSaContextUnsubscribe0(enginehandle : super::HANDLE, eventshandle : super::HANDLE) -> u32);
#[cfg(all(feature = "fwpmtypes", feature = "fwptypes", feature = "ipsectypes", feature = "winnt"))]
windows_link::link!("fwpuclnt.dll" "system" fn IPsecSaContextUpdate0(enginehandle : super::HANDLE, flags : u64, newvalues : *const super::IPSEC_SA_CONTEXT1) -> u32);
#[cfg(all(feature = "fwptypes", feature = "ipsectypes", feature = "winnt"))]
windows_link::link!("fwpuclnt.dll" "system" fn IPsecSaCreateEnumHandle0(enginehandle : super::HANDLE, enumtemplate : *const super::IPSEC_SA_ENUM_TEMPLATE0, enumhandle : *mut super::HANDLE) -> u32);
#[cfg(feature = "winnt")]
windows_link::link!("fwpuclnt.dll" "system" fn IPsecSaDbGetSecurityInfo0(enginehandle : super::HANDLE, securityinfo : super::SECURITY_INFORMATION, sidowner : *mut super::PSID, sidgroup : *mut super::PSID, dacl : *mut super::PACL, sacl : *mut super::PACL, securitydescriptor : *mut super::PSECURITY_DESCRIPTOR) -> u32);
#[cfg(feature = "winnt")]
windows_link::link!("fwpuclnt.dll" "system" fn IPsecSaDbSetSecurityInfo0(enginehandle : super::HANDLE, securityinfo : super::SECURITY_INFORMATION, sidowner : *const super::SID, sidgroup : *const super::SID, dacl : *const super::ACL, sacl : *const super::ACL) -> u32);
#[cfg(feature = "winnt")]
windows_link::link!("fwpuclnt.dll" "system" fn IPsecSaDestroyEnumHandle0(enginehandle : super::HANDLE, enumhandle : super::HANDLE) -> u32);
#[cfg(all(feature = "fwpmtypes", feature = "fwptypes", feature = "ipsectypes", feature = "winnt"))]
windows_link::link!("fwpuclnt.dll" "system" fn IPsecSaEnum0(enginehandle : super::HANDLE, enumhandle : super::HANDLE, numentriesrequested : u32, entries : *mut *mut *mut super::IPSEC_SA_DETAILS0, numentriesreturned : *mut u32) -> u32);
#[cfg(all(feature = "fwpmtypes", feature = "fwptypes", feature = "ipsectypes", feature = "winnt"))]
windows_link::link!("fwpuclnt.dll" "system" fn IPsecSaEnum1(enginehandle : super::HANDLE, enumhandle : super::HANDLE, numentriesrequested : u32, entries : *mut *mut *mut super::IPSEC_SA_DETAILS1, numentriesreturned : *mut u32) -> u32);
#[cfg(all(feature = "iketypes", feature = "winnt"))]
windows_link::link!("fwpuclnt.dll" "system" fn IkeextGetStatistics0(enginehandle : super::HANDLE, ikeextstatistics : *mut super::IKEEXT_STATISTICS0) -> u32);
#[cfg(all(feature = "iketypes", feature = "winnt"))]
windows_link::link!("fwpuclnt.dll" "system" fn IkeextGetStatistics1(enginehandle : super::HANDLE, ikeextstatistics : *mut super::IKEEXT_STATISTICS1) -> u32);
#[cfg(all(feature = "fwptypes", feature = "iketypes", feature = "winnt"))]
windows_link::link!("fwpuclnt.dll" "system" fn IkeextSaCreateEnumHandle0(enginehandle : super::HANDLE, enumtemplate : *const super::IKEEXT_SA_ENUM_TEMPLATE0, enumhandle : *mut super::HANDLE) -> u32);
#[cfg(feature = "winnt")]
windows_link::link!("fwpuclnt.dll" "system" fn IkeextSaDbGetSecurityInfo0(enginehandle : super::HANDLE, securityinfo : super::SECURITY_INFORMATION, sidowner : *mut super::PSID, sidgroup : *mut super::PSID, dacl : *mut super::PACL, sacl : *mut super::PACL, securitydescriptor : *mut super::PSECURITY_DESCRIPTOR) -> u32);
#[cfg(feature = "winnt")]
windows_link::link!("fwpuclnt.dll" "system" fn IkeextSaDbSetSecurityInfo0(enginehandle : super::HANDLE, securityinfo : super::SECURITY_INFORMATION, sidowner : *const super::SID, sidgroup : *const super::SID, dacl : *const super::ACL, sacl : *const super::ACL) -> u32);
#[cfg(feature = "winnt")]
windows_link::link!("fwpuclnt.dll" "system" fn IkeextSaDeleteById0(enginehandle : super::HANDLE, id : u64) -> u32);
#[cfg(feature = "winnt")]
windows_link::link!("fwpuclnt.dll" "system" fn IkeextSaDestroyEnumHandle0(enginehandle : super::HANDLE, enumhandle : super::HANDLE) -> u32);
#[cfg(all(feature = "fwptypes", feature = "iketypes", feature = "ipsectypes", feature = "winnt"))]
windows_link::link!("fwpuclnt.dll" "system" fn IkeextSaEnum0(enginehandle : super::HANDLE, enumhandle : super::HANDLE, numentriesrequested : u32, entries : *mut *mut *mut super::IKEEXT_SA_DETAILS0, numentriesreturned : *mut u32) -> u32);
#[cfg(all(feature = "fwptypes", feature = "iketypes", feature = "ipsectypes", feature = "winnt"))]
windows_link::link!("fwpuclnt.dll" "system" fn IkeextSaEnum1(enginehandle : super::HANDLE, enumhandle : super::HANDLE, numentriesrequested : u32, entries : *mut *mut *mut super::IKEEXT_SA_DETAILS1, numentriesreturned : *mut u32) -> u32);
#[cfg(all(feature = "fwptypes", feature = "iketypes", feature = "ipsectypes", feature = "winnt"))]
windows_link::link!("fwpuclnt.dll" "system" fn IkeextSaEnum2(enginehandle : super::HANDLE, enumhandle : super::HANDLE, numentriesrequested : u32, entries : *mut *mut *mut super::IKEEXT_SA_DETAILS2, numentriesreturned : *mut u32) -> u32);
#[cfg(all(feature = "fwptypes", feature = "iketypes", feature = "ipsectypes", feature = "winnt"))]
windows_link::link!("fwpuclnt.dll" "system" fn IkeextSaGetById0(enginehandle : super::HANDLE, id : u64, sa : *mut *mut super::IKEEXT_SA_DETAILS0) -> u32);
#[cfg(all(feature = "fwptypes", feature = "iketypes", feature = "ipsectypes", feature = "winnt"))]
windows_link::link!("fwpuclnt.dll" "system" fn IkeextSaGetById1(enginehandle : super::HANDLE, id : u64, salookupcontext : *const windows_sys::core::GUID, sa : *mut *mut super::IKEEXT_SA_DETAILS1) -> u32);
#[cfg(all(feature = "fwptypes", feature = "iketypes", feature = "ipsectypes", feature = "winnt"))]
windows_link::link!("fwpuclnt.dll" "system" fn IkeextSaGetById2(enginehandle : super::HANDLE, id : u64, salookupcontext : *const windows_sys::core::GUID, sa : *mut *mut super::IKEEXT_SA_DETAILS2) -> u32);
pub const FWPM_ACTRL_ADD: u32 = 1;
pub const FWPM_ACTRL_ADD_LINK: u32 = 2;
pub const FWPM_ACTRL_BEGIN_READ_TXN: u32 = 4;
pub const FWPM_ACTRL_BEGIN_WRITE_TXN: u32 = 8;
pub const FWPM_ACTRL_CLASSIFY: u32 = 16;
pub const FWPM_ACTRL_ENUM: u32 = 32;
pub const FWPM_ACTRL_OPEN: u32 = 64;
pub const FWPM_ACTRL_READ: u32 = 128;
pub const FWPM_ACTRL_READ_STATS: u32 = 256;
pub const FWPM_ACTRL_SUBSCRIBE: u32 = 512;
pub const FWPM_ACTRL_WRITE: u32 = 1024;
pub const FWPM_AUTO_WEIGHT_BITS: u32 = 60;
pub const FWPM_AUTO_WEIGHT_MAX: i32 = -1;
pub const FWPM_CALLOUT_BUILT_IN_RESERVED_1: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x779719a4_e695_47b6_a199_7999fec9163b);
pub const FWPM_CALLOUT_BUILT_IN_RESERVED_2: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xef9661b6_7c5e_48fd_a130_96678ceacc41);
pub const FWPM_CALLOUT_BUILT_IN_RESERVED_3: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x18729c7a_2f62_4be0_966f_974b21b86df1);
pub const FWPM_CALLOUT_BUILT_IN_RESERVED_4: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x6c3fb801_daff_40e9_91e6_f7ff7e52f7d9);
#[cfg(feature = "fwpmtypes")]
pub type FWPM_CALLOUT_CHANGE_CALLBACK0 = Option<unsafe extern "system" fn(context: *mut core::ffi::c_void, change: *const super::FWPM_CALLOUT_CHANGE0)>;
pub const FWPM_CALLOUT_EDGE_TRAVERSAL_ALE_LISTEN_V4: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x33486ab5_6d5e_4e65_a00b_a7afed0ba9a1);
pub const FWPM_CALLOUT_EDGE_TRAVERSAL_ALE_RESOURCE_ASSIGNMENT_V4: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x079b1010_f1c5_4fcd_ae05_da41107abd0b);
pub const FWPM_CALLOUT_HTTP_TEMPLATE_SSL_HANDSHAKE: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xb3423249_8d09_4858_9210_95c7fda8e30f);
pub const FWPM_CALLOUT_IPSEC_ALE_CONNECT_V4: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x6ac141fc_f75d_4203_b9c8_48e6149c2712);
pub const FWPM_CALLOUT_IPSEC_ALE_CONNECT_V6: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x4c0dda05_e31f_4666_90b0_b3dfad34129a);
pub const FWPM_CALLOUT_IPSEC_DOSP_FORWARD_V4: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x2fcb56ec_cd37_4b4f_b108_62c2b1850a0c);
pub const FWPM_CALLOUT_IPSEC_DOSP_FORWARD_V6: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x6d08a342_db9e_4fbe_9ed2_57374ce89f79);
pub const FWPM_CALLOUT_IPSEC_FORWARD_INBOUND_TUNNEL_V4: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x28829633_c4f0_4e66_873f_844db2a899c7);
pub const FWPM_CALLOUT_IPSEC_FORWARD_INBOUND_TUNNEL_V6: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xaf50bec2_c686_429a_884d_b74443e7b0b4);
pub const FWPM_CALLOUT_IPSEC_FORWARD_OUTBOUND_TUNNEL_V4: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xfb532136_15cb_440b_937c_1717ca320c40);
pub const FWPM_CALLOUT_IPSEC_FORWARD_OUTBOUND_TUNNEL_V6: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xdae640cc_e021_4bee_9eb6_a48b275c8c1d);
pub const FWPM_CALLOUT_IPSEC_INBOUND_INITIATE_SECURE_V4: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x7dff309b_ba7d_4aba_91aa_ae5c6640c944);
pub const FWPM_CALLOUT_IPSEC_INBOUND_INITIATE_SECURE_V6: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xa9a0d6d9_c58c_474e_8aeb_3cfe99d6d53d);
pub const FWPM_CALLOUT_IPSEC_INBOUND_TRANSPORT_V4: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x5132900d_5e84_4b5f_80e4_01741e81ff10);
pub const FWPM_CALLOUT_IPSEC_INBOUND_TRANSPORT_V6: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x49d3ac92_2a6c_4dcf_955f_1c3be009dd99);
pub const FWPM_CALLOUT_IPSEC_INBOUND_TUNNEL_ALE_ACCEPT_V4: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x3df6e7de_fd20_48f2_9f26_f854444cba79);
pub const FWPM_CALLOUT_IPSEC_INBOUND_TUNNEL_ALE_ACCEPT_V6: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xa1e392d3_72ac_47bb_87a7_0122c69434ab);
pub const FWPM_CALLOUT_IPSEC_INBOUND_TUNNEL_V4: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x191a8a46_0bf8_46cf_b045_4b45dfa6a324);
pub const FWPM_CALLOUT_IPSEC_INBOUND_TUNNEL_V6: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x80c342e3_1e53_4d6f_9b44_03df5aeee154);
pub const FWPM_CALLOUT_IPSEC_OUTBOUND_TRANSPORT_V4: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x4b46bf0a_4523_4e57_aa38_a87987c910d9);
pub const FWPM_CALLOUT_IPSEC_OUTBOUND_TRANSPORT_V6: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x38d87722_ad83_4f11_a91f_df0fb077225b);
pub const FWPM_CALLOUT_IPSEC_OUTBOUND_TUNNEL_V4: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x70a4196c_835b_4fb0_98e8_075f4d977d46);
pub const FWPM_CALLOUT_IPSEC_OUTBOUND_TUNNEL_V6: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xf1835363_a6a5_4e62_b180_23db789d8da6);
pub const FWPM_CALLOUT_OUTBOUND_NETWORK_CONNECTION_POLICY_LAYER_V4: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x103090d4_8e28_4fd6_9894_d1d67d6b10c9);
pub const FWPM_CALLOUT_OUTBOUND_NETWORK_CONNECTION_POLICY_LAYER_V6: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x4ed3446d_8dc7_459b_b09f_c1cb7a8f8689);
pub const FWPM_CALLOUT_POLICY_SILENT_MODE_AUTH_CONNECT_LAYER_V4: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x5fbfc31d_a51c_44dc_acb6_0624a030a700);
pub const FWPM_CALLOUT_POLICY_SILENT_MODE_AUTH_CONNECT_LAYER_V6: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x5fbfc31d_a51c_44dc_acb6_0624a030a701);
pub const FWPM_CALLOUT_POLICY_SILENT_MODE_AUTH_RECV_ACCEPT_LAYER_V4: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x5fbfc31d_a51c_44dc_acb6_0624a030a702);
pub const FWPM_CALLOUT_POLICY_SILENT_MODE_AUTH_RECV_ACCEPT_LAYER_V6: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x5fbfc31d_a51c_44dc_acb6_0624a030a703);
pub const FWPM_CALLOUT_RESERVED_AUTH_CONNECT_LAYER_V4: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x288b524d_0566_4e19_b612_8f441a2e5949);
pub const FWPM_CALLOUT_RESERVED_AUTH_CONNECT_LAYER_V6: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x00b84b92_2b5e_4b71_ab0e_aaca43e387e6);
pub const FWPM_CALLOUT_SET_OPTIONS_AUTH_CONNECT_LAYER_V4: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xbc582280_1677_41e9_94ab_c2fcb15c2eeb);
pub const FWPM_CALLOUT_SET_OPTIONS_AUTH_CONNECT_LAYER_V6: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x98e5373c_b884_490f_b65f_2f6a4a575195);
pub const FWPM_CALLOUT_SET_OPTIONS_AUTH_RECV_ACCEPT_LAYER_V4: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x2d55f008_0c01_4f92_b26e_a08a94569b8d);
pub const FWPM_CALLOUT_SET_OPTIONS_AUTH_RECV_ACCEPT_LAYER_V6: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x63018537_f281_4dc4_83d3_8dec18b7ade2);
pub const FWPM_CALLOUT_TCP_CHIMNEY_ACCEPT_LAYER_V4: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xe183ecb2_3a7f_4b54_8ad9_76050ed880ca);
pub const FWPM_CALLOUT_TCP_CHIMNEY_ACCEPT_LAYER_V6: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x0378cf41_bf98_4603_81f2_7f12586079f6);
pub const FWPM_CALLOUT_TCP_CHIMNEY_CONNECT_LAYER_V4: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xf3e10ab3_2c25_4279_ac36_c30fc181bec4);
pub const FWPM_CALLOUT_TCP_CHIMNEY_CONNECT_LAYER_V6: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x39e22085_a341_42fc_a279_aec94e689c56);
pub const FWPM_CALLOUT_TCP_TEMPLATES_ACCEPT_LAYER_V4: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x2f23f5d0_40c4_4c41_a254_46d8dba8957c);
pub const FWPM_CALLOUT_TCP_TEMPLATES_ACCEPT_LAYER_V6: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xb25152f0_991c_4f53_bbe7_d24b45fe632c);
pub const FWPM_CALLOUT_TCP_TEMPLATES_CONNECT_LAYER_V4: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x215a0b39_4b7e_4eda_8ce4_179679df6224);
pub const FWPM_CALLOUT_TCP_TEMPLATES_CONNECT_LAYER_V6: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x838b37a1_5c12_4d34_8b38_078728b2d25c);
pub const FWPM_CALLOUT_TEREDO_ALE_LISTEN_V6: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x81a434e7_f60c_4378_bab8_c625a30f0197);
pub const FWPM_CALLOUT_TEREDO_ALE_RESOURCE_ASSIGNMENT_V6: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x31b95392_066e_42a2_b7db_92f8acdd56f9);
pub const FWPM_CALLOUT_WFP_TRANSPORT_LAYER_V4_SILENT_DROP: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xeda08606_2494_4d78_89bc_67837c03b969);
pub const FWPM_CALLOUT_WFP_TRANSPORT_LAYER_V6_SILENT_DROP: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x8693cc74_a075_4156_b476_9286eece814e);
pub const FWPM_CONDITION_ALE_APP_ID: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xd78e1e87_8644_4ea5_9437_d809ecefc971);
pub const FWPM_CONDITION_ALE_EFFECTIVE_NAME: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xb1277b9a_b781_40fc_9671_e5f1b989f34e);
pub const FWPM_CONDITION_ALE_NAP_CONTEXT: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x46275a9d_c03f_4d77_b784_1c57f4d02753);
pub const FWPM_CONDITION_ALE_ORIGINAL_APP_ID: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x0e6cd086_e1fb_4212_842f_8a9f993fb3f6);
pub const FWPM_CONDITION_ALE_PACKAGE_FAMILY_NAME: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x81bc78fb_f28d_4886_a604_6acc261f261b);
pub const FWPM_CONDITION_ALE_PACKAGE_ID: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x71bc78fa_f17c_4997_a602_6abb261f351c);
pub const FWPM_CONDITION_ALE_PROMISCUOUS_MODE: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x1c974776_7182_46e9_afd3_b02910e30334);
pub const FWPM_CONDITION_ALE_REAUTH_REASON: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xb482d227_1979_4a98_8044_18bbe6237542);
pub const FWPM_CONDITION_ALE_REMOTE_MACHINE_ID: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x1aa47f51_7f93_4508_a271_81abb00c9cab);
pub const FWPM_CONDITION_ALE_REMOTE_USER_ID: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xf63073b7_0189_4ab0_95a4_6123cbfab862);
pub const FWPM_CONDITION_ALE_SECURITY_ATTRIBUTE_FQBN_VALUE: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x37a57699_5883_4963_92b8_3e704688b0ad);
pub const FWPM_CONDITION_ALE_SIO_FIREWALL_SYSTEM_PORT: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xb9f4e088_cb98_4efb_a2c7_ad07332643db);
pub const FWPM_CONDITION_ALE_USER_ID: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xaf043a0a_b34d_4f86_979c_c90371af6e66);
pub const FWPM_CONDITION_ARRIVAL_INTERFACE_INDEX: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xcc088db3_1792_4a71_b0f9_037d21cd828b);
pub const FWPM_CONDITION_ARRIVAL_INTERFACE_PROFILE_ID: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xcdfe6aab_c083_4142_8679_c08f95329c61);
pub const FWPM_CONDITION_ARRIVAL_INTERFACE_TYPE: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x89f990de_e798_4e6d_ab76_7c9558292e6f);
pub const FWPM_CONDITION_ARRIVAL_TUNNEL_TYPE: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x511166dc_7a8c_4aa7_b533_95ab59fb0340);
pub const FWPM_CONDITION_AUTHENTICATION_TYPE: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xeb458cd5_da7b_4ef9_8d43_7b0a840332f2);
pub const FWPM_CONDITION_CLIENT_CERT_KEY_LENGTH: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xa3ec00c7_05f4_4df7_91f2_5f60d91ff443);
pub const FWPM_CONDITION_CLIENT_CERT_OID: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xc491ad5e_f882_4283_b916_436b103ff4ad);
pub const FWPM_CONDITION_CLIENT_TOKEN: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xc228fc1e_403a_4478_be05_c9baa4c05ace);
pub const FWPM_CONDITION_COMPARTMENT_ID: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x35a791ab_04ac_4ff2_a6bb_da6cfac71806);
pub const FWPM_CONDITION_CURRENT_PROFILE_ID: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xab3033c9_c0e3_4759_937d_5758c65d4ae3);
pub const FWPM_CONDITION_DCOM_APP_ID: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xff2e7b4d_3112_4770_b636_4d24ae3a6af2);
pub const FWPM_CONDITION_DESTINATION_INTERFACE_INDEX: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x35cf6522_4139_45ee_a0d5_67b80949d879);
pub const FWPM_CONDITION_DESTINATION_SUB_INTERFACE_INDEX: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x2b7d4399_d4c7_4738_a2f5_e994b43da388);
pub const FWPM_CONDITION_DIRECTION: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x8784c146_ca97_44d6_9fd1_19fb1840cbf7);
pub const FWPM_CONDITION_EMBEDDED_LOCAL_ADDRESS_TYPE: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x4672a468_8a0a_4202_abb4_849e92e66809);
pub const FWPM_CONDITION_EMBEDDED_LOCAL_PORT: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xbfca394d_acdb_484e_b8e6_2aff79757345);
pub const FWPM_CONDITION_EMBEDDED_PROTOCOL: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x07784107_a29e_4c7b_9ec7_29c44afafdbc);
pub const FWPM_CONDITION_EMBEDDED_REMOTE_ADDRESS: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x77ee4b39_3273_4671_b63b_ab6feb66eeb6);
pub const FWPM_CONDITION_EMBEDDED_REMOTE_PORT: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xcae4d6a1_2968_40ed_a4ce_547160dda88d);
pub const FWPM_CONDITION_ETHER_TYPE: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xfd08948d_a219_4d52_bb98_1a5540ee7b4e);
pub const FWPM_CONDITION_FLAGS: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x632ce23b_5167_435c_86d7_e903684aa80c);
pub const FWPM_CONDITION_IMAGE_NAME: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xd024de4d_deaa_4317_9c85_e40ef6e140c3);
pub const FWPM_CONDITION_INTERFACE_INDEX: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x667fd755_d695_434a_8af5_d3835a1259bc);
pub const FWPM_CONDITION_INTERFACE_MAC_ADDRESS: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xf6e63dce_1f4b_4c6b_b6ef_1165e71f8ee7);
pub const FWPM_CONDITION_INTERFACE_QUARANTINE_EPOCH: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xcce68d5e_053b_43a8_9a6f_33384c28e4f6);
pub const FWPM_CONDITION_INTERFACE_TYPE: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xdaf8cd14_e09e_4c93_a5ae_c5c13b73ffca);
pub const FWPM_CONDITION_IPSEC_POLICY_KEY: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xad37dee3_722f_45cc_a4e3_068048124452);
pub const FWPM_CONDITION_IPSEC_SECURITY_REALM_ID: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x37a57700_5884_4964_92b8_3e704688b0ad);
pub const FWPM_CONDITION_IP_ARRIVAL_INTERFACE: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x618a9b6d_386b_4136_ad6e_b51587cfb1cd);
pub const FWPM_CONDITION_IP_DESTINATION_ADDRESS: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x2d79133b_b390_45c6_8699_acaceaafed33);
pub const FWPM_CONDITION_IP_DESTINATION_ADDRESS_TYPE: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x1ec1b7c9_4eea_4f5e_b9ef_76beaaaf17ee);
pub const FWPM_CONDITION_IP_DESTINATION_PORT: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xce6def45_60fb_4a7b_a304_af30a117000e);
pub const FWPM_CONDITION_IP_FORWARD_INTERFACE: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x1076b8a5_6323_4c5e_9810_e8d3fc9e6136);
pub const FWPM_CONDITION_IP_LOCAL_ADDRESS: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xd9ee00de_c1ef_4617_bfe3_ffd8f5a08957);
pub const FWPM_CONDITION_IP_LOCAL_ADDRESS_TYPE: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x6ec7f6c4_376b_45d7_9e9c_d337cedcd237);
pub const FWPM_CONDITION_IP_LOCAL_ADDRESS_V4: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x03a629cb_6e52_49f8_9c41_5709633c09cf);
pub const FWPM_CONDITION_IP_LOCAL_ADDRESS_V6: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x2381be84_7524_45b3_a05b_1e637d9c7a6a);
pub const FWPM_CONDITION_IP_LOCAL_INTERFACE: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x4cd62a49_59c3_4969_b7f3_bda5d32890a4);
pub const FWPM_CONDITION_IP_LOCAL_PORT: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x0c1ba1af_5765_453f_af22_a8f791ac775b);
pub const FWPM_CONDITION_IP_NEXTHOP_ADDRESS: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xeabe448a_a711_4d64_85b7_3f76b65299c7);
pub const FWPM_CONDITION_IP_NEXTHOP_INTERFACE: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x93ae8f5b_7f6f_4719_98c8_14e97429ef04);
pub const FWPM_CONDITION_IP_PHYSICAL_ARRIVAL_INTERFACE: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xda50d5c8_fa0d_4c89_b032_6e62136d1e96);
pub const FWPM_CONDITION_IP_PHYSICAL_NEXTHOP_INTERFACE: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xf09bd5ce_5150_48be_b098_c25152fb1f92);
pub const FWPM_CONDITION_IP_PROTOCOL: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x3971ef2b_623e_4f9a_8cb1_6e79b806b9a7);
pub const FWPM_CONDITION_IP_REMOTE_ADDRESS: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xb235ae9a_1d64_49b8_a44c_5ff3d9095045);
pub const FWPM_CONDITION_IP_REMOTE_ADDRESS_V4: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x1febb610_3bcc_45e1_bc36_2e067e2cb186);
pub const FWPM_CONDITION_IP_REMOTE_ADDRESS_V6: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x246e1d8c_8bee_4018_9b98_31d4582f3361);
pub const FWPM_CONDITION_IP_REMOTE_PORT: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xc35a604d_d22b_4e1a_91b4_68f674ee674b);
pub const FWPM_CONDITION_IP_SOURCE_ADDRESS: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xae96897e_2e94_4bc9_b313_b27ee80e574d);
pub const FWPM_CONDITION_IP_SOURCE_PORT: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xa6afef91_3df4_4730_a214_f5426aebf821);
pub const FWPM_CONDITION_KM_AUTH_NAP_CONTEXT: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x35d0ea0e_15ca_492b_900e_97fd46352cce);
pub const FWPM_CONDITION_KM_MODE: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xfeef4582_ef8f_4f7b_858b_9077d122de47);
pub const FWPM_CONDITION_KM_TYPE: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xff0f5f49_0ceb_481b_8638_1479791f3f2c);
pub const FWPM_CONDITION_L2_FLAGS: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x7bc43cbf_37ba_45f1_b74a_82ff518eeb10);
pub const FWPM_CONDITION_LOCAL_INTERFACE_PROFILE_ID: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x4ebf7562_9f18_4d06_9941_a7a625744d71);
pub const FWPM_CONDITION_MAC_DESTINATION_ADDRESS: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x04ea2a93_858c_4027_b613_b43180c7859e);
pub const FWPM_CONDITION_MAC_DESTINATION_ADDRESS_TYPE: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xae052932_ef42_4e99_b129_f3b3139e34f7);
pub const FWPM_CONDITION_MAC_LOCAL_ADDRESS: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xd999e981_7948_4c83_b742_c84e3b678f8f);
pub const FWPM_CONDITION_MAC_LOCAL_ADDRESS_TYPE: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xcc31355c_3073_4ffb_a14f_79415cb1ead1);
pub const FWPM_CONDITION_MAC_REMOTE_ADDRESS: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x408f2ed4_3a70_4b4d_92a6_415ac20e2f12);
pub const FWPM_CONDITION_MAC_REMOTE_ADDRESS_TYPE: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x027fedb4_f1c1_4030_b564_ee777fd867ea);
pub const FWPM_CONDITION_MAC_SOURCE_ADDRESS: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x7b795451_f1f6_4d05_b7cb_21779d802336);
pub const FWPM_CONDITION_MAC_SOURCE_ADDRESS_TYPE: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x5c1b72e4_299e_4437_a298_bc3f014b3dc2);
pub const FWPM_CONDITION_NDIS_MEDIA_TYPE: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xcb31cef1_791d_473b_89d1_61c5984304a0);
pub const FWPM_CONDITION_NDIS_PHYSICAL_MEDIA_TYPE: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x34c79823_c229_44f2_b83c_74020882ae77);
pub const FWPM_CONDITION_NDIS_PORT: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xdb7bb42b_2dac_4cd4_a59a_e0bdce1e6834);
pub const FWPM_CONDITION_NET_EVENT_TYPE: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x206e9996_490e_40cf_b831_b38641eb6fcb);
pub const FWPM_CONDITION_NEXTHOP_INTERFACE_INDEX: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x138e6888_7ab8_4d65_9ee8_0591bcf6a494);
pub const FWPM_CONDITION_NEXTHOP_INTERFACE_PROFILE_ID: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xd7ff9a56_cdaa_472b_84db_d23963c1d1bf);
pub const FWPM_CONDITION_NEXTHOP_INTERFACE_TYPE: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x97537c6c_d9a3_4767_a381_e942675cd920);
pub const FWPM_CONDITION_NEXTHOP_SUB_INTERFACE_INDEX: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xef8a6122_0577_45a7_9aaf_825fbeb4fb95);
pub const FWPM_CONDITION_NEXTHOP_TUNNEL_TYPE: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x72b1a111_987b_4720_99dd_c7c576fa2d4c);
pub const FWPM_CONDITION_ORIGINAL_ICMP_TYPE: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x076dfdbe_c56c_4f72_ae8a_2cfe7e5c8286);
pub const FWPM_CONDITION_ORIGINAL_PROFILE_ID: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x46ea1551_2255_492b_8019_aabeee349f40);
pub const FWPM_CONDITION_PEER_NAME: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x9b539082_eb90_4186_a6cc_de5b63235016);
pub const FWPM_CONDITION_PIPE: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x1bd0741d_e3df_4e24_8634_762046eef6eb);
pub const FWPM_CONDITION_PROCESS_WITH_RPC_IF_UUID: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xe31180a8_bbbd_4d14_a65e_7157b06233bb);
pub const FWPM_CONDITION_QM_MODE: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xf64fc6d1_f9cb_43d2_8a5f_e13bc894f265);
pub const FWPM_CONDITION_REAUTHORIZE_REASON: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x11205e8c_11ae_457a_8a44_477026dd764a);
pub const FWPM_CONDITION_REMOTE_ID: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xf68166fd_0682_4c89_b8f5_86436c7ef9b7);
pub const FWPM_CONDITION_REMOTE_USER_TOKEN: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x9bf0ee66_06c9_41b9_84da_288cb43af51f);
pub const FWPM_CONDITION_RESERVED0: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x678f4deb_45af_4882_93fe_19d4729d9834);
pub const FWPM_CONDITION_RESERVED1: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xd818f827_5c69_48eb_bf80_d86b17755f97);
pub const FWPM_CONDITION_RESERVED10: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xb979e282_d621_4c8c_b184_b105a61c36ce);
pub const FWPM_CONDITION_RESERVED11: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x2d62ee4d_023d_411f_9582_43acbb795975);
pub const FWPM_CONDITION_RESERVED12: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xa3677c32_7e35_4ddc_93da_e8c33fc923c7);
pub const FWPM_CONDITION_RESERVED13: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x335a3e90_84aa_42f5_9e6f_59309536a44c);
pub const FWPM_CONDITION_RESERVED14: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x30e44da2_2f1a_4116_a559_f907de83604a);
pub const FWPM_CONDITION_RESERVED15: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xbab8340f_afe0_43d1_80d8_5ca456962de3);
pub const FWPM_CONDITION_RESERVED2: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x53d4123d_e15b_4e84_b7a8_dce16f7b62d9);
pub const FWPM_CONDITION_RESERVED3: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x7f6e8ca3_6606_4932_97c7_e1f20710af3b);
pub const FWPM_CONDITION_RESERVED4: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x5f58e642_b937_495e_a94b_f6b051a49250);
pub const FWPM_CONDITION_RESERVED5: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x9ba8f6cd_f77c_43e6_8847_11939dc5db5a);
pub const FWPM_CONDITION_RESERVED6: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xf13d84bd_59d5_44c4_8817_5ecdae1805bd);
pub const FWPM_CONDITION_RESERVED7: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x65a0f930_45dd_4983_aa33_efc7b611af08);
pub const FWPM_CONDITION_RESERVED8: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x4f424974_0c12_4816_9b47_9a547db39a32);
pub const FWPM_CONDITION_RESERVED9: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xce78e10f_13ff_4c70_8643_36ad1879afa3);
pub const FWPM_CONDITION_RPC_AUTH_LEVEL: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xe5a0aed5_59ac_46ea_be05_a5f05ecf446e);
pub const FWPM_CONDITION_RPC_AUTH_TYPE: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xdaba74ab_0d67_43e7_986e_75b84f82f594);
pub const FWPM_CONDITION_RPC_EP_FLAGS: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x218b814a_0a39_49b8_8e71_c20c39c7dd2e);
pub const FWPM_CONDITION_RPC_EP_VALUE: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xdccea0b9_0886_4360_9c6a_ab043a24fba9);
pub const FWPM_CONDITION_RPC_IF_FLAG: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x238a8a32_3199_467d_871c_272621ab3896);
pub const FWPM_CONDITION_RPC_IF_UUID: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x7c9c7d9f_0075_4d35_a0d1_8311c4cf6af1);
pub const FWPM_CONDITION_RPC_IF_VERSION: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xeabfd9b7_1262_4a2e_adaa_5f96f6fe326d);
pub const FWPM_CONDITION_RPC_OPNUM: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xd58efb76_aab7_4148_a87e_9581134129b9);
pub const FWPM_CONDITION_RPC_PROTOCOL: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x2717bc74_3a35_4ce7_b7ef_c838fabdec45);
pub const FWPM_CONDITION_RPC_PROXY_AUTH_TYPE: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x40953fe2_8565_4759_8488_1771b4b4b5db);
pub const FWPM_CONDITION_RPC_SERVER_NAME: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xb605a225_c3b3_48c7_9833_7aefa9527546);
pub const FWPM_CONDITION_RPC_SERVER_PORT: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x8090f645_9ad5_4e3b_9f9f_8023ca097909);
pub const FWPM_CONDITION_SEC_ENCRYPT_ALGORITHM: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x0d306ef0_e974_4f74_b5c7_591b0da7d562);
pub const FWPM_CONDITION_SEC_KEY_SIZE: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x4772183b_ccf8_4aeb_bce1_c6c6161c8fe4);
pub const FWPM_CONDITION_SOURCE_INTERFACE_INDEX: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x2311334d_c92d_45bf_9496_edf447820e2d);
pub const FWPM_CONDITION_SOURCE_SUB_INTERFACE_INDEX: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x055edd9d_acd2_4361_8dab_f9525d97662f);
pub const FWPM_CONDITION_SUB_INTERFACE_INDEX: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x0cd42473_d621_4be3_ae8c_72a348d283e1);
pub const FWPM_CONDITION_TUNNEL_TYPE: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x77a40437_8779_4868_a261_f5a902f1c0cd);
pub const FWPM_CONDITION_VLAN_ID: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x938eab21_3618_4e64_9ca5_2141ebda1ca2);
pub const FWPM_CONDITION_VSWITCH_DESTINATION_INTERFACE_ID: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x8ed48be4_c926_49f6_a4f6_ef3030e3fc16);
pub const FWPM_CONDITION_VSWITCH_DESTINATION_INTERFACE_TYPE: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xfa9b3f06_2f1a_4c57_9e68_a7098b28dbfe);
pub const FWPM_CONDITION_VSWITCH_DESTINATION_VM_ID: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x6106aace_4de1_4c84_9671_3637f8bcf731);
pub const FWPM_CONDITION_VSWITCH_ID: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xc4a414ba_437b_4de6_9946_d99c1b95b312);
pub const FWPM_CONDITION_VSWITCH_NETWORK_TYPE: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x11d48b4b_e77a_40b4_9155_392c906c2608);
pub const FWPM_CONDITION_VSWITCH_SOURCE_INTERFACE_ID: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x7f4ef24b_b2c1_4938_ba33_a1ecbed512ba);
pub const FWPM_CONDITION_VSWITCH_SOURCE_INTERFACE_TYPE: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xe6b040a2_edaf_4c36_908b_f2f58ae43807);
pub const FWPM_CONDITION_VSWITCH_SOURCE_VM_ID: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x9c2a9ec2_9fc6_42bc_bdd8_406d4da0be64);
pub const FWPM_CONDITION_VSWITCH_TENANT_NETWORK_ID: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xdc04843c_79e6_4e44_a025_65b9bb0f9f94);
#[cfg(all(feature = "fwpmtypes", feature = "fwptypes", feature = "iketypes", feature = "ipsectypes", feature = "minwindef"))]
pub type FWPM_CONNECTION_CALLBACK0 = Option<unsafe extern "system" fn(context: *mut core::ffi::c_void, eventtype: super::FWPM_CONNECTION_EVENT_TYPE, connection: *const super::FWPM_CONNECTION0)>;
pub const FWPM_CONTEXT_ALE_ALLOW_AUTH_FW: u32 = 32;
pub const FWPM_CONTEXT_ALE_SET_CONNECTION_ALLOW_FIRST_INBOUND_PKT_UNENCRYPTED: u32 = 16;
pub const FWPM_CONTEXT_ALE_SET_CONNECTION_LAZY_SD_EVALUATION: u32 = 4;
pub const FWPM_CONTEXT_ALE_SET_CONNECTION_REQUIRE_IPSEC_ENCRYPTION: u32 = 8;
pub const FWPM_CONTEXT_ALE_SET_CONNECTION_REQUIRE_IPSEC_SECURITY: u32 = 2;
pub const FWPM_CONTEXT_IPSEC_INBOUND_PASSTHRU: u32 = 1;
pub const FWPM_CONTEXT_IPSEC_INBOUND_PERSIST_CONNECTION_SECURITY: u32 = 2;
pub const FWPM_CONTEXT_IPSEC_INBOUND_RESERVED: u32 = 0;
pub const FWPM_CONTEXT_IPSEC_INBOUND_SECURITY_REALM_ID: u32 = 4;
pub const FWPM_CONTEXT_IPSEC_OUTBOUNDBOUND_SECURITY_REALM_ID: u32 = 4;
pub const FWPM_CONTEXT_IPSEC_OUTBOUND_NEGOTIATE_DISCOVER: u32 = 1;
pub const FWPM_CONTEXT_IPSEC_OUTBOUND_SUPPRESS_NEGOTIATION: u32 = 2;
pub const FWPM_CONTEXT_RPC_AUDIT_BUFFER_ENABLED: u32 = 2;
pub const FWPM_CONTEXT_RPC_AUDIT_ENABLED: u32 = 1;
pub const FWPM_CONTEXT_TCP_CHIMNEY_OFFLOAD_DISABLE: u32 = 2;
pub const FWPM_CONTEXT_TCP_CHIMNEY_OFFLOAD_ENABLE: u32 = 1;
pub type FWPM_DYNAMIC_KEYWORD_CALLBACK0 = Option<unsafe extern "system" fn(notification: *mut core::ffi::c_void, context: *mut core::ffi::c_void)>;
#[cfg(feature = "fwpmtypes")]
pub type FWPM_FILTER_CHANGE_CALLBACK0 = Option<unsafe extern "system" fn(context: *mut core::ffi::c_void, change: *const super::FWPM_FILTER_CHANGE0)>;
pub const FWPM_GENERIC_ALL: u32 = 985087;
pub const FWPM_GENERIC_EXECUTE: u32 = 131616;
pub const FWPM_GENERIC_READ: u32 = 131540;
pub const FWPM_GENERIC_WRITE: u32 = 197643;
pub const FWPM_KEYING_MODULE_AUTHIP: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x11e3dae0_dd26_4590_857d_ab4b28d1a095);
pub const FWPM_KEYING_MODULE_IKE: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xa9bbf787_82a8_45bb_a400_5d7e5952c7a9);
pub const FWPM_KEYING_MODULE_IKEV2: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x041792cc_8f07_419d_a394_716968cb1647);
pub const FWPM_LAYER_ALE_AUTH_CONNECT_V4: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xc38d57d1_05a7_4c33_904f_7fbceee60e82);
pub const FWPM_LAYER_ALE_AUTH_CONNECT_V4_DISCARD: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xd632a801_f5ba_4ad6_96e3_607017d9836a);
pub const FWPM_LAYER_ALE_AUTH_CONNECT_V6: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x4a72393b_319f_44bc_84c3_ba54dcb3b6b4);
pub const FWPM_LAYER_ALE_AUTH_CONNECT_V6_DISCARD: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xc97bc3b8_c9a3_4e33_8695_8e17aad4de09);
pub const FWPM_LAYER_ALE_AUTH_LISTEN_V4: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x88bb5dad_76d7_4227_9c71_df0a3ed7be7e);
pub const FWPM_LAYER_ALE_AUTH_LISTEN_V4_DISCARD: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x371dfada_9f26_45fd_b4eb_c29eb212893f);
pub const FWPM_LAYER_ALE_AUTH_LISTEN_V6: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x7ac9de24_17dd_4814_b4bd_a9fbc95a321b);
pub const FWPM_LAYER_ALE_AUTH_LISTEN_V6_DISCARD: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x60703b07_63c8_48e9_ada3_12b1af40a617);
pub const FWPM_LAYER_ALE_AUTH_RECV_ACCEPT_V4: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xe1cd9fe7_f4b5_4273_96c0_592e487b8650);
pub const FWPM_LAYER_ALE_AUTH_RECV_ACCEPT_V4_DISCARD: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x9eeaa99b_bd22_4227_919f_0073c63357b1);
pub const FWPM_LAYER_ALE_AUTH_RECV_ACCEPT_V6: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xa3b42c97_9f04_4672_b87e_cee9c483257f);
pub const FWPM_LAYER_ALE_AUTH_RECV_ACCEPT_V6_DISCARD: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x89455b97_dbe1_453f_a224_13da895af396);
pub const FWPM_LAYER_ALE_BIND_REDIRECT_V4: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x66978cad_c704_42ac_86ac_7c1a231bd253);
pub const FWPM_LAYER_ALE_BIND_REDIRECT_V6: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xbef02c9c_606b_4536_8c26_1c2fc7b631d4);
pub const FWPM_LAYER_ALE_CONNECT_REDIRECT_V4: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xc6e63c8c_b784_4562_aa7d_0a67cfcaf9a3);
pub const FWPM_LAYER_ALE_CONNECT_REDIRECT_V6: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x587e54a7_8046_42ba_a0aa_b716250fc7fd);
pub const FWPM_LAYER_ALE_ENDPOINT_CLOSURE_V4: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xb4766427_e2a2_467a_bd7e_dbcd1bd85a09);
pub const FWPM_LAYER_ALE_ENDPOINT_CLOSURE_V6: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xbb536ccd_4755_4ba9_9ff7_f9edf8699c7b);
pub const FWPM_LAYER_ALE_FLOW_ESTABLISHED_V4: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xaf80470a_5596_4c13_9992_539e6fe57967);
pub const FWPM_LAYER_ALE_FLOW_ESTABLISHED_V4_DISCARD: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x146ae4a9_a1d2_4d43_a31a_4c42682b8e4f);
pub const FWPM_LAYER_ALE_FLOW_ESTABLISHED_V6: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x7021d2b3_dfa4_406e_afeb_6afaf7e70efd);
pub const FWPM_LAYER_ALE_FLOW_ESTABLISHED_V6_DISCARD: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x46928636_bbca_4b76_941d_0fa7f5d7d372);
pub const FWPM_LAYER_ALE_RESOURCE_ASSIGNMENT_V4: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x1247d66d_0b60_4a15_8d44_7155d0f53a0c);
pub const FWPM_LAYER_ALE_RESOURCE_ASSIGNMENT_V4_DISCARD: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x0b5812a2_c3ff_4eca_b88d_c79e20ac6322);
pub const FWPM_LAYER_ALE_RESOURCE_ASSIGNMENT_V6: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x55a650e1_5f0a_4eca_a653_88f53b26aa8c);
pub const FWPM_LAYER_ALE_RESOURCE_ASSIGNMENT_V6_DISCARD: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xcbc998bb_c51f_4c1a_bb4f_9775fcacab2f);
pub const FWPM_LAYER_ALE_RESOURCE_RELEASE_V4: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x74365cce_ccb0_401a_bfc1_b89934ad7e15);
pub const FWPM_LAYER_ALE_RESOURCE_RELEASE_V6: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xf4e5ce80_edcc_4e13_8a2f_b91454bb057b);
pub const FWPM_LAYER_DATAGRAM_DATA_V4: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x3d08bf4e_45f6_4930_a922_417098e20027);
pub const FWPM_LAYER_DATAGRAM_DATA_V4_DISCARD: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x18e330c6_7248_4e52_aaab_472ed67704fd);
pub const FWPM_LAYER_DATAGRAM_DATA_V6: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xfa45fe2f_3cba_4427_87fc_57b9a4b10d00);
pub const FWPM_LAYER_DATAGRAM_DATA_V6_DISCARD: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x09d1dfe1_9b86_4a42_be9d_8c315b92a5d0);
pub const FWPM_LAYER_EGRESS_VSWITCH_ETHERNET: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x86c872b0_76fa_4b79_93a4_0750530ae292);
pub const FWPM_LAYER_EGRESS_VSWITCH_TRANSPORT_V4: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xb92350b6_91f0_46b6_bdc4_871dfd4a7c98);
pub const FWPM_LAYER_EGRESS_VSWITCH_TRANSPORT_V6: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x1b2def23_1881_40bd_82f4_4254e63141cb);
pub const FWPM_LAYER_IKEEXT_V4: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xb14b7bdb_dbbd_473e_bed4_8b4708d4f270);
pub const FWPM_LAYER_IKEEXT_V6: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xb64786b3_f687_4eb9_89d2_8ef32acdabe2);
pub const FWPM_LAYER_INBOUND_ICMP_ERROR_V4: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x61499990_3cb6_4e84_b950_53b94b6964f3);
pub const FWPM_LAYER_INBOUND_ICMP_ERROR_V4_DISCARD: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xa6b17075_ebaf_4053_a4e7_213c8121ede5);
pub const FWPM_LAYER_INBOUND_ICMP_ERROR_V6: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x65f9bdff_3b2d_4e5d_b8c6_c720651fe898);
pub const FWPM_LAYER_INBOUND_ICMP_ERROR_V6_DISCARD: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xa6e7ccc0_08fb_468d_a472_9771d5595e09);
pub const FWPM_LAYER_INBOUND_IPPACKET_V4: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xc86fd1bf_21cd_497e_a0bb_17425c885c58);
pub const FWPM_LAYER_INBOUND_IPPACKET_V4_DISCARD: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xb5a230d0_a8c0_44f2_916e_991b53ded1f7);
pub const FWPM_LAYER_INBOUND_IPPACKET_V6: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xf52032cb_991c_46e7_971d_2601459a91ca);
pub const FWPM_LAYER_INBOUND_IPPACKET_V6_DISCARD: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xbb24c279_93b4_47a2_83ad_ae1698b50885);
pub const FWPM_LAYER_INBOUND_MAC_FRAME_ETHERNET: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xeffb7edb_0055_4f9a_a231_4ff8131ad191);
pub const FWPM_LAYER_INBOUND_MAC_FRAME_NATIVE: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xd4220bd3_62ce_4f08_ae88_b56e8526df50);
pub const FWPM_LAYER_INBOUND_MAC_FRAME_NATIVE_FAST: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x853aaa8e_2b78_4d24_a804_36db08b29711);
pub const FWPM_LAYER_INBOUND_RESERVED2: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xf4fb8d55_c076_46d8_a2c7_6a4c722ca4ed);
pub const FWPM_LAYER_INBOUND_TRANSPORT_FAST: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xe41d2719_05c7_40f0_8983_ea8d17bbc2f6);
pub const FWPM_LAYER_INBOUND_TRANSPORT_V4: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x5926dfc8_e3cf_4426_a283_dc393f5d0f9d);
pub const FWPM_LAYER_INBOUND_TRANSPORT_V4_DISCARD: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xac4a9833_f69d_4648_b261_6dc84835ef39);
pub const FWPM_LAYER_INBOUND_TRANSPORT_V6: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x634a869f_fc23_4b90_b0c1_bf620a36ae6f);
pub const FWPM_LAYER_INBOUND_TRANSPORT_V6_DISCARD: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x2a6ff955_3b2b_49d2_9848_ad9d72dcaab7);
pub const FWPM_LAYER_INGRESS_VSWITCH_ETHERNET: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x7d98577a_9a87_41ec_9718_7cf589c9f32d);
pub const FWPM_LAYER_INGRESS_VSWITCH_TRANSPORT_V4: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xb2696ff6_774f_4554_9f7d_3da3945f8e85);
pub const FWPM_LAYER_INGRESS_VSWITCH_TRANSPORT_V6: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x5ee314fc_7d8a_47f4_b7e3_291a36da4e12);
pub const FWPM_LAYER_IPFORWARD_V4: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xa82acc24_4ee1_4ee1_b465_fd1d25cb10a4);
pub const FWPM_LAYER_IPFORWARD_V4_DISCARD: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x9e9ea773_2fae_4210_8f17_34129ef369eb);
pub const FWPM_LAYER_IPFORWARD_V6: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x7b964818_19c7_493a_b71f_832c3684d28c);
pub const FWPM_LAYER_IPFORWARD_V6_DISCARD: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x31524a5d_1dfe_472f_bb93_518ee945d8a2);
pub const FWPM_LAYER_IPSEC_KM_DEMUX_V4: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xf02b1526_a459_4a51_b9e3_759de52b9d2c);
pub const FWPM_LAYER_IPSEC_KM_DEMUX_V6: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x2f755cf6_2fd4_4e88_b3e4_a91bca495235);
pub const FWPM_LAYER_IPSEC_V4: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xeda65c74_610d_4bc5_948f_3c4f89556867);
pub const FWPM_LAYER_IPSEC_V6: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x13c48442_8d87_4261_9a29_59d2abc348b4);
pub const FWPM_LAYER_KM_AUTHORIZATION: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x4aa226e9_9020_45fb_956a_c0249d841195);
pub const FWPM_LAYER_NAME_RESOLUTION_CACHE_V4: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x0c2aa681_905b_4ccd_a467_4dd811d07b7b);
pub const FWPM_LAYER_NAME_RESOLUTION_CACHE_V6: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x92d592fa_6b01_434a_9dea_d1e96ea97da9);
pub const FWPM_LAYER_OUTBOUND_ICMP_ERROR_V4: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x41390100_564c_4b32_bc1d_718048354d7c);
pub const FWPM_LAYER_OUTBOUND_ICMP_ERROR_V4_DISCARD: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xb3598d36_0561_4588_a6bf_e955e3f6264b);
pub const FWPM_LAYER_OUTBOUND_ICMP_ERROR_V6: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x7fb03b60_7b8d_4dfa_badd_980176fc4e12);
pub const FWPM_LAYER_OUTBOUND_ICMP_ERROR_V6_DISCARD: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x65f2e647_8d0c_4f47_b19b_33a4d3f1357c);
pub const FWPM_LAYER_OUTBOUND_IPPACKET_V4: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x1e5c9fae_8a84_4135_a331_950b54229ecd);
pub const FWPM_LAYER_OUTBOUND_IPPACKET_V4_DISCARD: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x08e4bcb5_b647_48f3_953c_e5ddbd03937e);
pub const FWPM_LAYER_OUTBOUND_IPPACKET_V6: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xa3b3ab6b_3564_488c_9117_f34e82142763);
pub const FWPM_LAYER_OUTBOUND_IPPACKET_V6_DISCARD: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x9513d7c4_a934_49dc_91a7_6ccb80cc02e3);
pub const FWPM_LAYER_OUTBOUND_MAC_FRAME_ETHERNET: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x694673bc_d6db_4870_adee_0acdbdb7f4b2);
pub const FWPM_LAYER_OUTBOUND_MAC_FRAME_NATIVE: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x94c44912_9d6f_4ebf_b995_05ab8a088d1b);
pub const FWPM_LAYER_OUTBOUND_MAC_FRAME_NATIVE_FAST: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x470df946_c962_486f_9446_8293cbc75eb8);
pub const FWPM_LAYER_OUTBOUND_NETWORK_CONNECTION_POLICY_V4: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x037f317a_d696_494a_bba5_bffc265e6052);
pub const FWPM_LAYER_OUTBOUND_NETWORK_CONNECTION_POLICY_V6: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x22a4fdb1_6d7e_48ae_ae77_3742525c3119);
pub const FWPM_LAYER_OUTBOUND_TRANSPORT_FAST: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x13ed4388_a070_4815_9935_7a9be6408b78);
pub const FWPM_LAYER_OUTBOUND_TRANSPORT_V4: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x09e61aea_d214_46e2_9b21_b26b0b2f28c8);
pub const FWPM_LAYER_OUTBOUND_TRANSPORT_V4_DISCARD: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xc5f10551_bdb0_43d7_a313_50e211f4d68a);
pub const FWPM_LAYER_OUTBOUND_TRANSPORT_V6: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xe1735bde_013f_4655_b351_a49e15762df0);
pub const FWPM_LAYER_OUTBOUND_TRANSPORT_V6_DISCARD: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xf433df69_ccbd_482e_b9b2_57165658c3b3);
pub const FWPM_LAYER_RPC_EPMAP: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x9247bc61_eb07_47ee_872c_bfd78bfd1616);
pub const FWPM_LAYER_RPC_EP_ADD: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x618dffc7_c450_4943_95db_99b4c16a55d4);
pub const FWPM_LAYER_RPC_PROXY_CONN: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x94a4b50b_ba5c_4f27_907a_229fac0c2a7a);
pub const FWPM_LAYER_RPC_PROXY_IF: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xf8a38615_e12c_41ac_98df_121ad981aade);
pub const FWPM_LAYER_RPC_UM: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x75a89dda_95e4_40f3_adc7_7688a9c847e1);
pub const FWPM_LAYER_STREAM_PACKET_V4: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xaf52d8ec_cb2d_44e5_ad92_f8dc38d2eb29);
pub const FWPM_LAYER_STREAM_PACKET_V6: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x779a8ca3_f099_468f_b5d4_83535c461c02);
pub const FWPM_LAYER_STREAM_V4: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x3b89653c_c170_49e4_b1cd_e0eeeee19a3e);
pub const FWPM_LAYER_STREAM_V4_DISCARD: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x25c4c2c2_25ff_4352_82f9_c54a4a4726dc);
pub const FWPM_LAYER_STREAM_V6: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x47c9137a_7ec4_46b3_b6e4_48e926b1eda4);
pub const FWPM_LAYER_STREAM_V6_DISCARD: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x10a59fc7_b628_4c41_9eb8_cf37d55103cf);
#[cfg(all(feature = "fwpmtypes", feature = "fwptypes", feature = "iketypes", feature = "ipsectypes", feature = "minwindef", feature = "winnt"))]
pub type FWPM_NET_EVENT_CALLBACK0 = Option<unsafe extern "system" fn(context: *mut core::ffi::c_void, event: *const super::FWPM_NET_EVENT1)>;
#[cfg(all(feature = "fwpmtypes", feature = "fwptypes", feature = "iketypes", feature = "ipsectypes", feature = "minwindef", feature = "winnt"))]
pub type FWPM_NET_EVENT_CALLBACK1 = Option<unsafe extern "system" fn(context: *mut core::ffi::c_void, event: *const super::FWPM_NET_EVENT2)>;
#[cfg(all(feature = "fwpmtypes", feature = "fwptypes", feature = "iketypes", feature = "ipsectypes", feature = "minwindef", feature = "winnt"))]
pub type FWPM_NET_EVENT_CALLBACK2 = Option<unsafe extern "system" fn(context: *mut core::ffi::c_void, event: *const super::FWPM_NET_EVENT3)>;
#[cfg(all(feature = "fwpmtypes", feature = "fwptypes", feature = "iketypes", feature = "ipsectypes", feature = "minwindef", feature = "winnt"))]
pub type FWPM_NET_EVENT_CALLBACK3 = Option<unsafe extern "system" fn(context: *mut core::ffi::c_void, event: *const super::FWPM_NET_EVENT4)>;
#[cfg(all(feature = "fwpmtypes", feature = "fwptypes", feature = "iketypes", feature = "ipsectypes", feature = "minwindef", feature = "winnt"))]
pub type FWPM_NET_EVENT_CALLBACK4 = Option<unsafe extern "system" fn(context: *mut core::ffi::c_void, event: *const super::FWPM_NET_EVENT5)>;
pub const FWPM_NOTIFY_ADDRESSES_ALL: u32 = 3;
pub const FWPM_NOTIFY_ADDRESSES_AUTO_RESOLVE: u32 = 1;
pub const FWPM_NOTIFY_ADDRESSES_NON_AUTO_RESOLVE: u32 = 2;
pub const FWPM_NOTIFY_GRANULAR: u32 = 4;
#[cfg(feature = "fwpmtypes")]
pub type FWPM_PROVIDER_CHANGE_CALLBACK0 = Option<unsafe extern "system" fn(context: *mut core::ffi::c_void, change: *const super::FWPM_PROVIDER_CHANGE0)>;
#[cfg(feature = "fwpmtypes")]
pub type FWPM_PROVIDER_CONTEXT_CHANGE_CALLBACK0 = Option<unsafe extern "system" fn(context: *mut core::ffi::c_void, change: *const super::FWPM_PROVIDER_CONTEXT_CHANGE0)>;
pub const FWPM_PROVIDER_CONTEXT_SECURE_SOCKET_AUTHIP: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xb25ea800_0d02_46ed_92bd_7fa84bb73e9d);
pub const FWPM_PROVIDER_CONTEXT_SECURE_SOCKET_IPSEC: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x8c2d4144_f8e0_42c0_94ce_7ccfc63b2f9b);
pub const FWPM_PROVIDER_IKEEXT: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x10ad9216_ccde_456c_8b16_e9f04e60a90b);
pub const FWPM_PROVIDER_IPSEC_DOSP_CONFIG: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x3c6c05a9_c05c_4bb9_8338_2327814ce8bf);
pub const FWPM_PROVIDER_MPSSVC_APP_ISOLATION: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x3cc2631f_2d5d_43a0_b174_614837d863a1);
pub const FWPM_PROVIDER_MPSSVC_EDP: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xa90296f7_46b8_4457_8f84_b05e05d3c622);
pub const FWPM_PROVIDER_MPSSVC_TENANT_RESTRICTIONS: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xd0718ff9_44da_4f50_9dc2_c963a4247613);
pub const FWPM_PROVIDER_MPSSVC_WF: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xdecc16ca_3f33_4346_be1e_8fb4ae0f3d62);
pub const FWPM_PROVIDER_MPSSVC_WSH: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x4b153735_1049_4480_aab4_d1b9bdc03710);
pub const FWPM_PROVIDER_TCP_CHIMNEY_OFFLOAD: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x896aa19e_9a34_4bcb_ae79_beb9127c84b9);
pub const FWPM_PROVIDER_TCP_TEMPLATES: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x76cfcd30_3394_432d_bed3_441ae50e63c3);
#[cfg(feature = "fwpmtypes")]
pub type FWPM_SUBLAYER_CHANGE_CALLBACK0 = Option<unsafe extern "system" fn(context: *mut core::ffi::c_void, change: *const super::FWPM_SUBLAYER_CHANGE0)>;
pub const FWPM_SUBLAYER_INSPECTION: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x877519e1_e6a9_41a5_81b4_8c4f118e4a60);
pub const FWPM_SUBLAYER_IPSEC_DOSP: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xe076d572_5d3d_48ef_802b_909eddb098bd);
pub const FWPM_SUBLAYER_IPSEC_FORWARD_OUTBOUND_TUNNEL: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xa5082e73_8f71_4559_8a9a_101cea04ef87);
pub const FWPM_SUBLAYER_IPSEC_SECURITY_REALM: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x37a57701_5884_4964_92b8_3e704688b0ad);
pub const FWPM_SUBLAYER_IPSEC_TUNNEL: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x83f299ed_9ff4_4967_aff4_c309f4dab827);
pub const FWPM_SUBLAYER_LIPS: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x1b75c0ce_ff60_4711_a70f_b4958cc3b2d0);
pub const FWPM_SUBLAYER_MPSSVC_APP_ISOLATION: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xffe221c3_92a8_4564_a59f_dafb70756020);
pub const FWPM_SUBLAYER_MPSSVC_EDP: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x09a47e38_fa97_471b_b123_18bcd7e65071);
pub const FWPM_SUBLAYER_MPSSVC_QUARANTINE: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xb3cdd441_af90_41ba_a745_7c6008ff2302);
pub const FWPM_SUBLAYER_MPSSVC_TENANT_RESTRICTIONS: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x1ec6c7e1_fdd9_478a_b55f_ff8ba1d2c17d);
pub const FWPM_SUBLAYER_MPSSVC_WF: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xb3cdd441_af90_41ba_a745_7c6008ff2301);
pub const FWPM_SUBLAYER_MPSSVC_WSH: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xb3cdd441_af90_41ba_a745_7c6008ff2300);
pub const FWPM_SUBLAYER_RPC_AUDIT: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x758c84f4_fb48_4de9_9aeb_3ed9551ab1fd);
pub const FWPM_SUBLAYER_SECURE_SOCKET: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x15a66e17_3f3c_4f7b_aa6c_812aa613dd82);
pub const FWPM_SUBLAYER_TCP_CHIMNEY_OFFLOAD: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x337608b9_b7d5_4d5f_82f9_3618618bc058);
pub const FWPM_SUBLAYER_TCP_TEMPLATES: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x24421dcf_0ac5_4caa_9e14_50f6e3636af0);
pub const FWPM_SUBLAYER_TEREDO: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xba69dc66_5176_4979_9c89_26a7b46a8327);
pub const FWPM_SUBLAYER_UNIVERSAL: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xeebecc03_ced4_4380_819a_2734397b2b74);
#[cfg(feature = "fwpmtypes")]
pub type FWPM_SYSTEM_PORTS_CALLBACK0 = Option<unsafe extern "system" fn(context: *mut core::ffi::c_void, sysports: *const super::FWPM_SYSTEM_PORTS0)>;
pub const FWPM_TUNNEL_FLAG_ENABLE_VIRTUAL_IF_TUNNELING: u32 = 2;
pub const FWPM_TUNNEL_FLAG_POINT_TO_POINT: u32 = 1;
pub const FWPM_TUNNEL_FLAG_RESERVED0: u32 = 4;
pub const FWPM_TXN_READ_ONLY: u32 = 1;
#[cfg(feature = "fwpmtypes")]
pub type FWPM_VSWITCH_EVENT_CALLBACK0 = Option<unsafe extern "system" fn(context: *mut core::ffi::c_void, vswitchevent: *const super::FWPM_VSWITCH_EVENT0) -> u32>;
pub const FWPM_WEIGHT_RANGE_IKE_EXEMPTIONS: u32 = 12;
pub const FWPM_WEIGHT_RANGE_IPSEC: u32 = 0;
pub const FWPM_WEIGHT_RANGE_MAX: u32 = 15;
#[repr(C)]
#[cfg(all(feature = "fwpmtypes", feature = "fwptypes", feature = "iketypes", feature = "ipsectypes", feature = "winnt"))]
#[derive(Clone, Copy, Default)]
pub struct IPSEC_KEY_MANAGER_CALLBACKS0 {
    pub reserved: windows_sys::core::GUID,
    pub flags: u32,
    pub keyDictationCheck: IPSEC_KEY_MANAGER_KEY_DICTATION_CHECK0,
    pub keyDictation: IPSEC_KEY_MANAGER_DICTATE_KEY0,
    pub keyNotify: IPSEC_KEY_MANAGER_NOTIFY_KEY0,
}
#[cfg(all(feature = "fwpmtypes", feature = "fwptypes", feature = "ipsectypes", feature = "winnt"))]
pub type IPSEC_KEY_MANAGER_DICTATE_KEY0 = Option<unsafe extern "system" fn(inboundsadetails: *mut super::IPSEC_SA_DETAILS1, outboundsadetails: *mut super::IPSEC_SA_DETAILS1, keyingmodulegenkey: *mut windows_sys::core::BOOL) -> u32>;
#[cfg(all(feature = "fwptypes", feature = "iketypes"))]
pub type IPSEC_KEY_MANAGER_KEY_DICTATION_CHECK0 = Option<unsafe extern "system" fn(iketraffic: *const super::IKEEXT_TRAFFIC0, willdictatekey: *mut windows_sys::core::BOOL, weight: *mut u32)>;
#[cfg(all(feature = "fwpmtypes", feature = "fwptypes", feature = "ipsectypes", feature = "winnt"))]
pub type IPSEC_KEY_MANAGER_NOTIFY_KEY0 = Option<unsafe extern "system" fn(inboundsa: *const super::IPSEC_SA_DETAILS1, outboundsa: *const super::IPSEC_SA_DETAILS1)>;
pub const IPSEC_SA_BUNDLE_UPDATE_FLAGS: u32 = 4;
pub const IPSEC_SA_BUNDLE_UPDATE_KEY_MODULE_STATE: u32 = 16;
pub const IPSEC_SA_BUNDLE_UPDATE_MM_SA_ID: u32 = 64;
pub const IPSEC_SA_BUNDLE_UPDATE_NAP_CONTEXT: u32 = 8;
pub const IPSEC_SA_BUNDLE_UPDATE_PEER_V4_PRIVATE_ADDRESS: u32 = 32;
#[cfg(feature = "ipsectypes")]
pub type IPSEC_SA_CONTEXT_CALLBACK0 = Option<unsafe extern "system" fn(context: *mut core::ffi::c_void, change: *const super::IPSEC_SA_CONTEXT_CHANGE0)>;
pub const IPSEC_SA_DETAILS_UPDATE_TRAFFIC: u32 = 1;
pub const IPSEC_SA_DETAILS_UPDATE_UDP_ENCAPSULATION: u32 = 2;
