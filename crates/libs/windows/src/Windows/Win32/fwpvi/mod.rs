#[cfg(all(feature = "fwpmtypes", feature = "fwptypes", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn FwpmCalloutAdd(enginehandle: super::HANDLE, callout: *const super::FWPM_CALLOUT0, sd: Option<super::PSECURITY_DESCRIPTOR>, id: Option<*mut super::UINT32>) -> super::DWORD {
    windows_core::link!("fwpuclnt.dll" "system" "FwpmCalloutAdd0" fn FwpmCalloutAdd(enginehandle : super::HANDLE, callout : *const super::FWPM_CALLOUT0, sd : super::PSECURITY_DESCRIPTOR, id : *mut super::UINT32) -> super::DWORD);
    unsafe { FwpmCalloutAdd(enginehandle, callout, sd.unwrap_or(core::mem::zeroed()) as _, id.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "fwpmtypes", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn FwpmCalloutCreateEnumHandle(enginehandle: super::HANDLE, enumtemplate: Option<*const super::FWPM_CALLOUT_ENUM_TEMPLATE0>, enumhandle: *mut super::HANDLE) -> super::DWORD {
    windows_core::link!("fwpuclnt.dll" "system" "FwpmCalloutCreateEnumHandle0" fn FwpmCalloutCreateEnumHandle(enginehandle : super::HANDLE, enumtemplate : *const super::FWPM_CALLOUT_ENUM_TEMPLATE0, enumhandle : *mut super::HANDLE) -> super::DWORD);
    unsafe { FwpmCalloutCreateEnumHandle(enginehandle, enumtemplate.unwrap_or(core::mem::zeroed()) as _, enumhandle as _) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn FwpmCalloutDeleteById(enginehandle: super::HANDLE, id: super::UINT32) -> super::DWORD {
    windows_core::link!("fwpuclnt.dll" "system" "FwpmCalloutDeleteById0" fn FwpmCalloutDeleteById(enginehandle : super::HANDLE, id : super::UINT32) -> super::DWORD);
    unsafe { FwpmCalloutDeleteById(enginehandle, id) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn FwpmCalloutDeleteByKey(enginehandle: super::HANDLE, key: *const windows_core::GUID) -> super::DWORD {
    windows_core::link!("fwpuclnt.dll" "system" "FwpmCalloutDeleteByKey0" fn FwpmCalloutDeleteByKey(enginehandle : super::HANDLE, key : *const windows_core::GUID) -> super::DWORD);
    unsafe { FwpmCalloutDeleteByKey(enginehandle, key) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn FwpmCalloutDestroyEnumHandle(enginehandle: super::HANDLE, enumhandle: super::HANDLE) -> super::DWORD {
    windows_core::link!("fwpuclnt.dll" "system" "FwpmCalloutDestroyEnumHandle0" fn FwpmCalloutDestroyEnumHandle(enginehandle : super::HANDLE, enumhandle : super::HANDLE) -> super::DWORD);
    unsafe { FwpmCalloutDestroyEnumHandle(enginehandle, enumhandle as _) }
}
#[cfg(all(feature = "fwpmtypes", feature = "fwptypes", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn FwpmCalloutEnum(enginehandle: super::HANDLE, enumhandle: super::HANDLE, numentriesrequested: super::UINT32, entries: *mut *mut *mut super::FWPM_CALLOUT0, numentriesreturned: *mut super::UINT32) -> super::DWORD {
    windows_core::link!("fwpuclnt.dll" "system" "FwpmCalloutEnum0" fn FwpmCalloutEnum(enginehandle : super::HANDLE, enumhandle : super::HANDLE, numentriesrequested : super::UINT32, entries : *mut *mut *mut super::FWPM_CALLOUT0, numentriesreturned : *mut super::UINT32) -> super::DWORD);
    unsafe { FwpmCalloutEnum(enginehandle, enumhandle, numentriesrequested, entries as _, numentriesreturned as _) }
}
#[cfg(all(feature = "fwpmtypes", feature = "fwptypes", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn FwpmCalloutGetById(enginehandle: super::HANDLE, id: super::UINT32, callout: *mut *mut super::FWPM_CALLOUT0) -> super::DWORD {
    windows_core::link!("fwpuclnt.dll" "system" "FwpmCalloutGetById0" fn FwpmCalloutGetById(enginehandle : super::HANDLE, id : super::UINT32, callout : *mut *mut super::FWPM_CALLOUT0) -> super::DWORD);
    unsafe { FwpmCalloutGetById(enginehandle, id, callout as _) }
}
#[cfg(all(feature = "fwpmtypes", feature = "fwptypes", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn FwpmCalloutGetByKey(enginehandle: super::HANDLE, key: *const windows_core::GUID, callout: *mut *mut super::FWPM_CALLOUT0) -> super::DWORD {
    windows_core::link!("fwpuclnt.dll" "system" "FwpmCalloutGetByKey0" fn FwpmCalloutGetByKey(enginehandle : super::HANDLE, key : *const windows_core::GUID, callout : *mut *mut super::FWPM_CALLOUT0) -> super::DWORD);
    unsafe { FwpmCalloutGetByKey(enginehandle, key, callout as _) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn FwpmCalloutGetSecurityInfoByKey(enginehandle: super::HANDLE, key: Option<*const windows_core::GUID>, securityinfo: super::SECURITY_INFORMATION, sidowner: *mut super::PSID, sidgroup: *mut super::PSID, dacl: *mut super::PACL, sacl: *mut super::PACL, securitydescriptor: *mut super::PSECURITY_DESCRIPTOR) -> super::DWORD {
    windows_core::link!("fwpuclnt.dll" "system" "FwpmCalloutGetSecurityInfoByKey0" fn FwpmCalloutGetSecurityInfoByKey(enginehandle : super::HANDLE, key : *const windows_core::GUID, securityinfo : super::SECURITY_INFORMATION, sidowner : *mut super::PSID, sidgroup : *mut super::PSID, dacl : *mut super::PACL, sacl : *mut super::PACL, securitydescriptor : *mut super::PSECURITY_DESCRIPTOR) -> super::DWORD);
    unsafe { FwpmCalloutGetSecurityInfoByKey(enginehandle, key.unwrap_or(core::mem::zeroed()) as _, securityinfo, sidowner as _, sidgroup as _, dacl as _, sacl as _, securitydescriptor as _) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn FwpmCalloutSetSecurityInfoByKey(enginehandle: super::HANDLE, key: Option<*const windows_core::GUID>, securityinfo: super::SECURITY_INFORMATION, sidowner: Option<*const super::SID>, sidgroup: Option<*const super::SID>, dacl: Option<*const super::ACL>, sacl: Option<*const super::ACL>) -> super::DWORD {
    windows_core::link!("fwpuclnt.dll" "system" "FwpmCalloutSetSecurityInfoByKey0" fn FwpmCalloutSetSecurityInfoByKey(enginehandle : super::HANDLE, key : *const windows_core::GUID, securityinfo : super::SECURITY_INFORMATION, sidowner : *const super::SID, sidgroup : *const super::SID, dacl : *const super::ACL, sacl : *const super::ACL) -> super::DWORD);
    unsafe { FwpmCalloutSetSecurityInfoByKey(enginehandle, key.unwrap_or(core::mem::zeroed()) as _, securityinfo, sidowner.unwrap_or(core::mem::zeroed()) as _, sidgroup.unwrap_or(core::mem::zeroed()) as _, dacl.unwrap_or(core::mem::zeroed()) as _, sacl.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "fwpmtypes", feature = "fwpmu", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn FwpmCalloutSubscribeChanges(enginehandle: super::HANDLE, subscription: *const super::FWPM_CALLOUT_SUBSCRIPTION0, callback: super::FWPM_CALLOUT_CHANGE_CALLBACK0, context: Option<*const core::ffi::c_void>, changehandle: *mut super::HANDLE) -> super::DWORD {
    windows_core::link!("fwpuclnt.dll" "system" "FwpmCalloutSubscribeChanges0" fn FwpmCalloutSubscribeChanges(enginehandle : super::HANDLE, subscription : *const super::FWPM_CALLOUT_SUBSCRIPTION0, callback : super::FWPM_CALLOUT_CHANGE_CALLBACK0, context : *const core::ffi::c_void, changehandle : *mut super::HANDLE) -> super::DWORD);
    unsafe { FwpmCalloutSubscribeChanges(enginehandle, subscription, callback, context.unwrap_or(core::mem::zeroed()) as _, changehandle as _) }
}
#[cfg(all(feature = "fwpmtypes", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn FwpmCalloutSubscriptionsGet(enginehandle: super::HANDLE, entries: *mut *mut *mut super::FWPM_CALLOUT_SUBSCRIPTION0, numentries: *mut super::UINT32) -> super::DWORD {
    windows_core::link!("fwpuclnt.dll" "system" "FwpmCalloutSubscriptionsGet0" fn FwpmCalloutSubscriptionsGet(enginehandle : super::HANDLE, entries : *mut *mut *mut super::FWPM_CALLOUT_SUBSCRIPTION0, numentries : *mut super::UINT32) -> super::DWORD);
    unsafe { FwpmCalloutSubscriptionsGet(enginehandle, entries as _, numentries as _) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn FwpmCalloutUnsubscribeChanges(enginehandle: super::HANDLE, changehandle: super::HANDLE) -> super::DWORD {
    windows_core::link!("fwpuclnt.dll" "system" "FwpmCalloutUnsubscribeChanges0" fn FwpmCalloutUnsubscribeChanges(enginehandle : super::HANDLE, changehandle : super::HANDLE) -> super::DWORD);
    unsafe { FwpmCalloutUnsubscribeChanges(enginehandle, changehandle as _) }
}
#[cfg(all(feature = "fwpmtypes", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn FwpmConnectionCreateEnumHandle(enginehandle: super::HANDLE, enumtemplate: Option<*const super::FWPM_CONNECTION_ENUM_TEMPLATE0>, enumhandle: *mut super::HANDLE) -> super::DWORD {
    windows_core::link!("fwpuclnt.dll" "system" "FwpmConnectionCreateEnumHandle0" fn FwpmConnectionCreateEnumHandle(enginehandle : super::HANDLE, enumtemplate : *const super::FWPM_CONNECTION_ENUM_TEMPLATE0, enumhandle : *mut super::HANDLE) -> super::DWORD);
    unsafe { FwpmConnectionCreateEnumHandle(enginehandle, enumtemplate.unwrap_or(core::mem::zeroed()) as _, enumhandle as _) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn FwpmConnectionDestroyEnumHandle(enginehandle: super::HANDLE, enumhandle: super::HANDLE) -> super::DWORD {
    windows_core::link!("fwpuclnt.dll" "system" "FwpmConnectionDestroyEnumHandle0" fn FwpmConnectionDestroyEnumHandle(enginehandle : super::HANDLE, enumhandle : super::HANDLE) -> super::DWORD);
    unsafe { FwpmConnectionDestroyEnumHandle(enginehandle, enumhandle) }
}
#[cfg(all(feature = "fwpmtypes", feature = "fwptypes", feature = "iketypes", feature = "ipsectypes", feature = "minwindef", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn FwpmConnectionEnum(enginehandle: super::HANDLE, enumhandle: super::HANDLE, numentriesrequested: super::UINT32, entries: *mut *mut *mut super::FWPM_CONNECTION0, numentriesreturned: *mut super::UINT32) -> super::DWORD {
    windows_core::link!("fwpuclnt.dll" "system" "FwpmConnectionEnum0" fn FwpmConnectionEnum(enginehandle : super::HANDLE, enumhandle : super::HANDLE, numentriesrequested : super::UINT32, entries : *mut *mut *mut super::FWPM_CONNECTION0, numentriesreturned : *mut super::UINT32) -> super::DWORD);
    unsafe { FwpmConnectionEnum(enginehandle, enumhandle, numentriesrequested, entries as _, numentriesreturned as _) }
}
#[cfg(all(feature = "fwpmtypes", feature = "fwptypes", feature = "iketypes", feature = "ipsectypes", feature = "minwindef", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn FwpmConnectionGetById(enginehandle: super::HANDLE, id: super::UINT64, connection: *mut *mut super::FWPM_CONNECTION0) -> super::DWORD {
    windows_core::link!("fwpuclnt.dll" "system" "FwpmConnectionGetById0" fn FwpmConnectionGetById(enginehandle : super::HANDLE, id : super::UINT64, connection : *mut *mut super::FWPM_CONNECTION0) -> super::DWORD);
    unsafe { FwpmConnectionGetById(enginehandle, id, connection as _) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn FwpmConnectionGetSecurityInfo(enginehandle: super::HANDLE, securityinfo: super::SECURITY_INFORMATION, sidowner: *mut super::PSID, sidgroup: *mut super::PSID, dacl: *mut super::PACL, sacl: *mut super::PACL, securitydescriptor: *mut super::PSECURITY_DESCRIPTOR) -> super::DWORD {
    windows_core::link!("fwpuclnt.dll" "system" "FwpmConnectionGetSecurityInfo0" fn FwpmConnectionGetSecurityInfo(enginehandle : super::HANDLE, securityinfo : super::SECURITY_INFORMATION, sidowner : *mut super::PSID, sidgroup : *mut super::PSID, dacl : *mut super::PACL, sacl : *mut super::PACL, securitydescriptor : *mut super::PSECURITY_DESCRIPTOR) -> super::DWORD);
    unsafe { FwpmConnectionGetSecurityInfo(enginehandle, securityinfo, sidowner as _, sidgroup as _, dacl as _, sacl as _, securitydescriptor as _) }
}
#[cfg(all(feature = "fwpmtypes", feature = "fwptypes", feature = "iketypes", feature = "ipsectypes", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn FwpmConnectionPolicyAdd(enginehandle: super::HANDLE, connectionpolicy: *const super::FWPM_PROVIDER_CONTEXT3, ipversion: super::FWP_IP_VERSION, weight: super::UINT64, filterconditions: &[super::FWPM_FILTER_CONDITION0], sd: Option<super::PSECURITY_DESCRIPTOR>) -> super::DWORD {
    windows_core::link!("fwpuclnt.dll" "system" "FwpmConnectionPolicyAdd0" fn FwpmConnectionPolicyAdd(enginehandle : super::HANDLE, connectionpolicy : *const super::FWPM_PROVIDER_CONTEXT3, ipversion : super::FWP_IP_VERSION, weight : super::UINT64, numfilterconditions : super::UINT32, filterconditions : *const super::FWPM_FILTER_CONDITION0, sd : super::PSECURITY_DESCRIPTOR) -> super::DWORD);
    unsafe { FwpmConnectionPolicyAdd(enginehandle, connectionpolicy, ipversion, weight, filterconditions.len().try_into().unwrap(), filterconditions.as_ptr(), sd.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn FwpmConnectionPolicyDeleteByKey(enginehandle: super::HANDLE, key: *const windows_core::GUID) -> super::DWORD {
    windows_core::link!("fwpuclnt.dll" "system" "FwpmConnectionPolicyDeleteByKey0" fn FwpmConnectionPolicyDeleteByKey(enginehandle : super::HANDLE, key : *const windows_core::GUID) -> super::DWORD);
    unsafe { FwpmConnectionPolicyDeleteByKey(enginehandle, key) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn FwpmConnectionSetSecurityInfo(enginehandle: super::HANDLE, securityinfo: super::SECURITY_INFORMATION, sidowner: Option<*const super::SID>, sidgroup: Option<*const super::SID>, dacl: Option<*const super::ACL>, sacl: Option<*const super::ACL>) -> super::DWORD {
    windows_core::link!("fwpuclnt.dll" "system" "FwpmConnectionSetSecurityInfo0" fn FwpmConnectionSetSecurityInfo(enginehandle : super::HANDLE, securityinfo : super::SECURITY_INFORMATION, sidowner : *const super::SID, sidgroup : *const super::SID, dacl : *const super::ACL, sacl : *const super::ACL) -> super::DWORD);
    unsafe { FwpmConnectionSetSecurityInfo(enginehandle, securityinfo, sidowner.unwrap_or(core::mem::zeroed()) as _, sidgroup.unwrap_or(core::mem::zeroed()) as _, dacl.unwrap_or(core::mem::zeroed()) as _, sacl.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "fwpmtypes", feature = "fwpmu", feature = "fwptypes", feature = "iketypes", feature = "ipsectypes", feature = "minwindef", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn FwpmConnectionSubscribe(enginehandle: super::HANDLE, subscription: *const super::FWPM_CONNECTION_SUBSCRIPTION0, callback: super::FWPM_CONNECTION_CALLBACK0, context: Option<*const core::ffi::c_void>, eventshandle: *mut super::HANDLE) -> super::DWORD {
    windows_core::link!("fwpuclnt.dll" "system" "FwpmConnectionSubscribe0" fn FwpmConnectionSubscribe(enginehandle : super::HANDLE, subscription : *const super::FWPM_CONNECTION_SUBSCRIPTION0, callback : super::FWPM_CONNECTION_CALLBACK0, context : *const core::ffi::c_void, eventshandle : *mut super::HANDLE) -> super::DWORD);
    unsafe { FwpmConnectionSubscribe(enginehandle, subscription, callback, context.unwrap_or(core::mem::zeroed()) as _, eventshandle as _) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn FwpmConnectionUnsubscribe(enginehandle: super::HANDLE, eventshandle: super::HANDLE) -> super::DWORD {
    windows_core::link!("fwpuclnt.dll" "system" "FwpmConnectionUnsubscribe0" fn FwpmConnectionUnsubscribe(enginehandle : super::HANDLE, eventshandle : super::HANDLE) -> super::DWORD);
    unsafe { FwpmConnectionUnsubscribe(enginehandle, eventshandle as _) }
}
#[cfg(all(feature = "fwpmu", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn FwpmDynamicKeywordSubscribe(flags: super::DWORD, callback: super::FWPM_DYNAMIC_KEYWORD_CALLBACK0, context: Option<*const core::ffi::c_void>, subscriptionhandle: *mut super::HANDLE) -> super::DWORD {
    windows_core::link!("fwpuclnt.dll" "system" "FwpmDynamicKeywordSubscribe0" fn FwpmDynamicKeywordSubscribe(flags : super::DWORD, callback : super::FWPM_DYNAMIC_KEYWORD_CALLBACK0, context : *const core::ffi::c_void, subscriptionhandle : *mut super::HANDLE) -> super::DWORD);
    unsafe { FwpmDynamicKeywordSubscribe(flags, callback, context.unwrap_or(core::mem::zeroed()) as _, subscriptionhandle as _) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn FwpmEngineClose(enginehandle: super::HANDLE) -> super::DWORD {
    windows_core::link!("fwpuclnt.dll" "system" "FwpmEngineClose0" fn FwpmEngineClose(enginehandle : super::HANDLE) -> super::DWORD);
    unsafe { FwpmEngineClose(enginehandle as _) }
}
#[cfg(all(feature = "fwpmtypes", feature = "fwptypes", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn FwpmEngineGetOption(enginehandle: super::HANDLE, option: super::FWPM_ENGINE_OPTION, value: *mut *mut super::FWP_VALUE0) -> super::DWORD {
    windows_core::link!("fwpuclnt.dll" "system" "FwpmEngineGetOption0" fn FwpmEngineGetOption(enginehandle : super::HANDLE, option : super::FWPM_ENGINE_OPTION, value : *mut *mut super::FWP_VALUE0) -> super::DWORD);
    unsafe { FwpmEngineGetOption(enginehandle, option, value as _) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn FwpmEngineGetSecurityInfo(enginehandle: super::HANDLE, securityinfo: super::SECURITY_INFORMATION, sidowner: *mut super::PSID, sidgroup: *mut super::PSID, dacl: *mut super::PACL, sacl: *mut super::PACL, securitydescriptor: *mut super::PSECURITY_DESCRIPTOR) -> super::DWORD {
    windows_core::link!("fwpuclnt.dll" "system" "FwpmEngineGetSecurityInfo0" fn FwpmEngineGetSecurityInfo(enginehandle : super::HANDLE, securityinfo : super::SECURITY_INFORMATION, sidowner : *mut super::PSID, sidgroup : *mut super::PSID, dacl : *mut super::PACL, sacl : *mut super::PACL, securitydescriptor : *mut super::PSECURITY_DESCRIPTOR) -> super::DWORD);
    unsafe { FwpmEngineGetSecurityInfo(enginehandle, securityinfo, sidowner as _, sidgroup as _, dacl as _, sacl as _, securitydescriptor as _) }
}
#[cfg(all(feature = "fwpmtypes", feature = "fwptypes", feature = "rpc", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn FwpmEngineOpen(servername: Option<*const u16>, authnservice: super::UINT32, authidentity: Option<*const super::SEC_WINNT_AUTH_IDENTITY_W>, session: Option<*const super::FWPM_SESSION0>, enginehandle: *mut super::HANDLE) -> super::DWORD {
    windows_core::link!("fwpuclnt.dll" "system" "FwpmEngineOpen0" fn FwpmEngineOpen(servername : *const u16, authnservice : super::UINT32, authidentity : *const super::SEC_WINNT_AUTH_IDENTITY_W, session : *const super::FWPM_SESSION0, enginehandle : *mut super::HANDLE) -> super::DWORD);
    unsafe { FwpmEngineOpen(servername.unwrap_or(core::mem::zeroed()) as _, authnservice, authidentity.unwrap_or(core::mem::zeroed()) as _, session.unwrap_or(core::mem::zeroed()) as _, enginehandle as _) }
}
#[cfg(all(feature = "fwpmtypes", feature = "fwptypes", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn FwpmEngineSetOption(enginehandle: super::HANDLE, option: super::FWPM_ENGINE_OPTION, newvalue: *const super::FWP_VALUE0) -> super::DWORD {
    windows_core::link!("fwpuclnt.dll" "system" "FwpmEngineSetOption0" fn FwpmEngineSetOption(enginehandle : super::HANDLE, option : super::FWPM_ENGINE_OPTION, newvalue : *const super::FWP_VALUE0) -> super::DWORD);
    unsafe { FwpmEngineSetOption(enginehandle, option, newvalue) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn FwpmEngineSetSecurityInfo(enginehandle: super::HANDLE, securityinfo: super::SECURITY_INFORMATION, sidowner: Option<*const super::SID>, sidgroup: Option<*const super::SID>, dacl: Option<*const super::ACL>, sacl: Option<*const super::ACL>) -> super::DWORD {
    windows_core::link!("fwpuclnt.dll" "system" "FwpmEngineSetSecurityInfo0" fn FwpmEngineSetSecurityInfo(enginehandle : super::HANDLE, securityinfo : super::SECURITY_INFORMATION, sidowner : *const super::SID, sidgroup : *const super::SID, dacl : *const super::ACL, sacl : *const super::ACL) -> super::DWORD);
    unsafe { FwpmEngineSetSecurityInfo(enginehandle, securityinfo, sidowner.unwrap_or(core::mem::zeroed()) as _, sidgroup.unwrap_or(core::mem::zeroed()) as _, dacl.unwrap_or(core::mem::zeroed()) as _, sacl.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "fwpmtypes", feature = "fwptypes", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn FwpmFilterAdd(enginehandle: super::HANDLE, filter: *const super::FWPM_FILTER0, sd: Option<super::PSECURITY_DESCRIPTOR>, id: Option<*mut super::UINT64>) -> super::DWORD {
    windows_core::link!("fwpuclnt.dll" "system" "FwpmFilterAdd0" fn FwpmFilterAdd(enginehandle : super::HANDLE, filter : *const super::FWPM_FILTER0, sd : super::PSECURITY_DESCRIPTOR, id : *mut super::UINT64) -> super::DWORD);
    unsafe { FwpmFilterAdd(enginehandle, filter, sd.unwrap_or(core::mem::zeroed()) as _, id.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "fwpmtypes", feature = "fwptypes", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn FwpmFilterCreateEnumHandle(enginehandle: super::HANDLE, enumtemplate: Option<*const super::FWPM_FILTER_ENUM_TEMPLATE0>, enumhandle: *mut super::HANDLE) -> super::DWORD {
    windows_core::link!("fwpuclnt.dll" "system" "FwpmFilterCreateEnumHandle0" fn FwpmFilterCreateEnumHandle(enginehandle : super::HANDLE, enumtemplate : *const super::FWPM_FILTER_ENUM_TEMPLATE0, enumhandle : *mut super::HANDLE) -> super::DWORD);
    unsafe { FwpmFilterCreateEnumHandle(enginehandle, enumtemplate.unwrap_or(core::mem::zeroed()) as _, enumhandle as _) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn FwpmFilterDeleteById(enginehandle: super::HANDLE, id: super::UINT64) -> super::DWORD {
    windows_core::link!("fwpuclnt.dll" "system" "FwpmFilterDeleteById0" fn FwpmFilterDeleteById(enginehandle : super::HANDLE, id : super::UINT64) -> super::DWORD);
    unsafe { FwpmFilterDeleteById(enginehandle, id) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn FwpmFilterDeleteByKey(enginehandle: super::HANDLE, key: *const windows_core::GUID) -> super::DWORD {
    windows_core::link!("fwpuclnt.dll" "system" "FwpmFilterDeleteByKey0" fn FwpmFilterDeleteByKey(enginehandle : super::HANDLE, key : *const windows_core::GUID) -> super::DWORD);
    unsafe { FwpmFilterDeleteByKey(enginehandle, key) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn FwpmFilterDestroyEnumHandle(enginehandle: super::HANDLE, enumhandle: super::HANDLE) -> super::DWORD {
    windows_core::link!("fwpuclnt.dll" "system" "FwpmFilterDestroyEnumHandle0" fn FwpmFilterDestroyEnumHandle(enginehandle : super::HANDLE, enumhandle : super::HANDLE) -> super::DWORD);
    unsafe { FwpmFilterDestroyEnumHandle(enginehandle, enumhandle as _) }
}
#[cfg(all(feature = "fwpmtypes", feature = "fwptypes", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn FwpmFilterEnum(enginehandle: super::HANDLE, enumhandle: super::HANDLE, numentriesrequested: super::UINT32, entries: *mut *mut *mut super::FWPM_FILTER0, numentriesreturned: *mut super::UINT32) -> super::DWORD {
    windows_core::link!("fwpuclnt.dll" "system" "FwpmFilterEnum0" fn FwpmFilterEnum(enginehandle : super::HANDLE, enumhandle : super::HANDLE, numentriesrequested : super::UINT32, entries : *mut *mut *mut super::FWPM_FILTER0, numentriesreturned : *mut super::UINT32) -> super::DWORD);
    unsafe { FwpmFilterEnum(enginehandle, enumhandle, numentriesrequested, entries as _, numentriesreturned as _) }
}
#[cfg(all(feature = "fwpmtypes", feature = "fwptypes", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn FwpmFilterGetById(enginehandle: super::HANDLE, id: super::UINT64, filter: *mut *mut super::FWPM_FILTER0) -> super::DWORD {
    windows_core::link!("fwpuclnt.dll" "system" "FwpmFilterGetById0" fn FwpmFilterGetById(enginehandle : super::HANDLE, id : super::UINT64, filter : *mut *mut super::FWPM_FILTER0) -> super::DWORD);
    unsafe { FwpmFilterGetById(enginehandle, id, filter as _) }
}
#[cfg(all(feature = "fwpmtypes", feature = "fwptypes", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn FwpmFilterGetByKey(enginehandle: super::HANDLE, key: *const windows_core::GUID, filter: *mut *mut super::FWPM_FILTER0) -> super::DWORD {
    windows_core::link!("fwpuclnt.dll" "system" "FwpmFilterGetByKey0" fn FwpmFilterGetByKey(enginehandle : super::HANDLE, key : *const windows_core::GUID, filter : *mut *mut super::FWPM_FILTER0) -> super::DWORD);
    unsafe { FwpmFilterGetByKey(enginehandle, key, filter as _) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn FwpmFilterGetSecurityInfoByKey(enginehandle: super::HANDLE, key: Option<*const windows_core::GUID>, securityinfo: super::SECURITY_INFORMATION, sidowner: *mut super::PSID, sidgroup: *mut super::PSID, dacl: *mut super::PACL, sacl: *mut super::PACL, securitydescriptor: *mut super::PSECURITY_DESCRIPTOR) -> super::DWORD {
    windows_core::link!("fwpuclnt.dll" "system" "FwpmFilterGetSecurityInfoByKey0" fn FwpmFilterGetSecurityInfoByKey(enginehandle : super::HANDLE, key : *const windows_core::GUID, securityinfo : super::SECURITY_INFORMATION, sidowner : *mut super::PSID, sidgroup : *mut super::PSID, dacl : *mut super::PACL, sacl : *mut super::PACL, securitydescriptor : *mut super::PSECURITY_DESCRIPTOR) -> super::DWORD);
    unsafe { FwpmFilterGetSecurityInfoByKey(enginehandle, key.unwrap_or(core::mem::zeroed()) as _, securityinfo, sidowner as _, sidgroup as _, dacl as _, sacl as _, securitydescriptor as _) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn FwpmFilterSetSecurityInfoByKey(enginehandle: super::HANDLE, key: Option<*const windows_core::GUID>, securityinfo: super::SECURITY_INFORMATION, sidowner: Option<*const super::SID>, sidgroup: Option<*const super::SID>, dacl: Option<*const super::ACL>, sacl: Option<*const super::ACL>) -> super::DWORD {
    windows_core::link!("fwpuclnt.dll" "system" "FwpmFilterSetSecurityInfoByKey0" fn FwpmFilterSetSecurityInfoByKey(enginehandle : super::HANDLE, key : *const windows_core::GUID, securityinfo : super::SECURITY_INFORMATION, sidowner : *const super::SID, sidgroup : *const super::SID, dacl : *const super::ACL, sacl : *const super::ACL) -> super::DWORD);
    unsafe { FwpmFilterSetSecurityInfoByKey(enginehandle, key.unwrap_or(core::mem::zeroed()) as _, securityinfo, sidowner.unwrap_or(core::mem::zeroed()) as _, sidgroup.unwrap_or(core::mem::zeroed()) as _, dacl.unwrap_or(core::mem::zeroed()) as _, sacl.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "fwpmtypes", feature = "fwpmu", feature = "fwptypes", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn FwpmFilterSubscribeChanges(enginehandle: super::HANDLE, subscription: *const super::FWPM_FILTER_SUBSCRIPTION0, callback: super::FWPM_FILTER_CHANGE_CALLBACK0, context: Option<*const core::ffi::c_void>, changehandle: *mut super::HANDLE) -> super::DWORD {
    windows_core::link!("fwpuclnt.dll" "system" "FwpmFilterSubscribeChanges0" fn FwpmFilterSubscribeChanges(enginehandle : super::HANDLE, subscription : *const super::FWPM_FILTER_SUBSCRIPTION0, callback : super::FWPM_FILTER_CHANGE_CALLBACK0, context : *const core::ffi::c_void, changehandle : *mut super::HANDLE) -> super::DWORD);
    unsafe { FwpmFilterSubscribeChanges(enginehandle, subscription, callback, context.unwrap_or(core::mem::zeroed()) as _, changehandle as _) }
}
#[cfg(all(feature = "fwpmtypes", feature = "fwptypes", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn FwpmFilterSubscriptionsGet(enginehandle: super::HANDLE, entries: *mut *mut *mut super::FWPM_FILTER_SUBSCRIPTION0, numentries: *mut super::UINT32) -> super::DWORD {
    windows_core::link!("fwpuclnt.dll" "system" "FwpmFilterSubscriptionsGet0" fn FwpmFilterSubscriptionsGet(enginehandle : super::HANDLE, entries : *mut *mut *mut super::FWPM_FILTER_SUBSCRIPTION0, numentries : *mut super::UINT32) -> super::DWORD);
    unsafe { FwpmFilterSubscriptionsGet(enginehandle, entries as _, numentries as _) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn FwpmFilterUnsubscribeChanges(enginehandle: super::HANDLE, changehandle: super::HANDLE) -> super::DWORD {
    windows_core::link!("fwpuclnt.dll" "system" "FwpmFilterUnsubscribeChanges0" fn FwpmFilterUnsubscribeChanges(enginehandle : super::HANDLE, changehandle : super::HANDLE) -> super::DWORD);
    unsafe { FwpmFilterUnsubscribeChanges(enginehandle, changehandle as _) }
}
#[inline]
pub unsafe fn FwpmFreeMemory(p: *mut *mut core::ffi::c_void) {
    windows_core::link!("fwpuclnt.dll" "system" "FwpmFreeMemory0" fn FwpmFreeMemory(p : *mut *mut core::ffi::c_void));
    unsafe { FwpmFreeMemory(p as _) }
}
#[cfg(all(feature = "fwptypes", feature = "shared"))]
#[inline]
pub unsafe fn FwpmGetAppIdFromFileName<P0>(filename: P0, appid: *mut *mut super::FWP_BYTE_BLOB) -> super::DWORD
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("fwpuclnt.dll" "system" "FwpmGetAppIdFromFileName0" fn FwpmGetAppIdFromFileName(filename : windows_core::PCWSTR, appid : *mut *mut super::FWP_BYTE_BLOB) -> super::DWORD);
    unsafe { FwpmGetAppIdFromFileName(filename.param().abi(), appid as _) }
}
#[cfg(all(feature = "fwpmtypes", feature = "fwptypes", feature = "iketypes", feature = "ipsectypes", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn FwpmIPsecTunnelAdd(enginehandle: super::HANDLE, flags: super::UINT32, mainmodepolicy: Option<*const super::FWPM_PROVIDER_CONTEXT3>, tunnelpolicy: *const super::FWPM_PROVIDER_CONTEXT3, filterconditions: &[super::FWPM_FILTER_CONDITION0], keymodkey: Option<*const windows_core::GUID>, sd: Option<super::PSECURITY_DESCRIPTOR>) -> super::DWORD {
    windows_core::link!("fwpuclnt.dll" "system" "FwpmIPsecTunnelAdd3" fn FwpmIPsecTunnelAdd(enginehandle : super::HANDLE, flags : super::UINT32, mainmodepolicy : *const super::FWPM_PROVIDER_CONTEXT3, tunnelpolicy : *const super::FWPM_PROVIDER_CONTEXT3, numfilterconditions : super::UINT32, filterconditions : *const super::FWPM_FILTER_CONDITION0, keymodkey : *const windows_core::GUID, sd : super::PSECURITY_DESCRIPTOR) -> super::DWORD);
    unsafe { FwpmIPsecTunnelAdd(enginehandle, flags, mainmodepolicy.unwrap_or(core::mem::zeroed()) as _, tunnelpolicy, filterconditions.len().try_into().unwrap(), filterconditions.as_ptr(), keymodkey.unwrap_or(core::mem::zeroed()) as _, sd.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn FwpmIPsecTunnelDeleteByKey(enginehandle: super::HANDLE, key: *const windows_core::GUID) -> super::DWORD {
    windows_core::link!("fwpuclnt.dll" "system" "FwpmIPsecTunnelDeleteByKey0" fn FwpmIPsecTunnelDeleteByKey(enginehandle : super::HANDLE, key : *const windows_core::GUID) -> super::DWORD);
    unsafe { FwpmIPsecTunnelDeleteByKey(enginehandle, key) }
}
#[cfg(all(feature = "fwpmtypes", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn FwpmLayerCreateEnumHandle(enginehandle: super::HANDLE, enumtemplate: Option<*const super::FWPM_LAYER_ENUM_TEMPLATE0>, enumhandle: *mut super::HANDLE) -> super::DWORD {
    windows_core::link!("fwpuclnt.dll" "system" "FwpmLayerCreateEnumHandle0" fn FwpmLayerCreateEnumHandle(enginehandle : super::HANDLE, enumtemplate : *const super::FWPM_LAYER_ENUM_TEMPLATE0, enumhandle : *mut super::HANDLE) -> super::DWORD);
    unsafe { FwpmLayerCreateEnumHandle(enginehandle, enumtemplate.unwrap_or(core::mem::zeroed()) as _, enumhandle as _) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn FwpmLayerDestroyEnumHandle(enginehandle: super::HANDLE, enumhandle: super::HANDLE) -> super::DWORD {
    windows_core::link!("fwpuclnt.dll" "system" "FwpmLayerDestroyEnumHandle0" fn FwpmLayerDestroyEnumHandle(enginehandle : super::HANDLE, enumhandle : super::HANDLE) -> super::DWORD);
    unsafe { FwpmLayerDestroyEnumHandle(enginehandle, enumhandle as _) }
}
#[cfg(all(feature = "fwpmtypes", feature = "fwptypes", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn FwpmLayerEnum(enginehandle: super::HANDLE, enumhandle: super::HANDLE, numentriesrequested: super::UINT32, entries: *mut *mut *mut super::FWPM_LAYER0, numentriesreturned: *mut super::UINT32) -> super::DWORD {
    windows_core::link!("fwpuclnt.dll" "system" "FwpmLayerEnum0" fn FwpmLayerEnum(enginehandle : super::HANDLE, enumhandle : super::HANDLE, numentriesrequested : super::UINT32, entries : *mut *mut *mut super::FWPM_LAYER0, numentriesreturned : *mut super::UINT32) -> super::DWORD);
    unsafe { FwpmLayerEnum(enginehandle, enumhandle, numentriesrequested, entries as _, numentriesreturned as _) }
}
#[cfg(all(feature = "fwpmtypes", feature = "fwptypes", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn FwpmLayerGetById(enginehandle: super::HANDLE, id: super::UINT16, layer: *mut *mut super::FWPM_LAYER0) -> super::DWORD {
    windows_core::link!("fwpuclnt.dll" "system" "FwpmLayerGetById0" fn FwpmLayerGetById(enginehandle : super::HANDLE, id : super::UINT16, layer : *mut *mut super::FWPM_LAYER0) -> super::DWORD);
    unsafe { FwpmLayerGetById(enginehandle, id, layer as _) }
}
#[cfg(all(feature = "fwpmtypes", feature = "fwptypes", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn FwpmLayerGetByKey(enginehandle: super::HANDLE, key: *const windows_core::GUID, layer: *mut *mut super::FWPM_LAYER0) -> super::DWORD {
    windows_core::link!("fwpuclnt.dll" "system" "FwpmLayerGetByKey0" fn FwpmLayerGetByKey(enginehandle : super::HANDLE, key : *const windows_core::GUID, layer : *mut *mut super::FWPM_LAYER0) -> super::DWORD);
    unsafe { FwpmLayerGetByKey(enginehandle, key, layer as _) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn FwpmLayerGetSecurityInfoByKey(enginehandle: super::HANDLE, key: Option<*const windows_core::GUID>, securityinfo: super::SECURITY_INFORMATION, sidowner: *mut super::PSID, sidgroup: *mut super::PSID, dacl: *mut super::PACL, sacl: *mut super::PACL, securitydescriptor: *mut super::PSECURITY_DESCRIPTOR) -> super::DWORD {
    windows_core::link!("fwpuclnt.dll" "system" "FwpmLayerGetSecurityInfoByKey0" fn FwpmLayerGetSecurityInfoByKey(enginehandle : super::HANDLE, key : *const windows_core::GUID, securityinfo : super::SECURITY_INFORMATION, sidowner : *mut super::PSID, sidgroup : *mut super::PSID, dacl : *mut super::PACL, sacl : *mut super::PACL, securitydescriptor : *mut super::PSECURITY_DESCRIPTOR) -> super::DWORD);
    unsafe { FwpmLayerGetSecurityInfoByKey(enginehandle, key.unwrap_or(core::mem::zeroed()) as _, securityinfo, sidowner as _, sidgroup as _, dacl as _, sacl as _, securitydescriptor as _) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn FwpmLayerSetSecurityInfoByKey(enginehandle: super::HANDLE, key: Option<*const windows_core::GUID>, securityinfo: super::SECURITY_INFORMATION, sidowner: Option<*const super::SID>, sidgroup: Option<*const super::SID>, dacl: Option<*const super::ACL>, sacl: Option<*const super::ACL>) -> super::DWORD {
    windows_core::link!("fwpuclnt.dll" "system" "FwpmLayerSetSecurityInfoByKey0" fn FwpmLayerSetSecurityInfoByKey(enginehandle : super::HANDLE, key : *const windows_core::GUID, securityinfo : super::SECURITY_INFORMATION, sidowner : *const super::SID, sidgroup : *const super::SID, dacl : *const super::ACL, sacl : *const super::ACL) -> super::DWORD);
    unsafe { FwpmLayerSetSecurityInfoByKey(enginehandle, key.unwrap_or(core::mem::zeroed()) as _, securityinfo, sidowner.unwrap_or(core::mem::zeroed()) as _, sidgroup.unwrap_or(core::mem::zeroed()) as _, dacl.unwrap_or(core::mem::zeroed()) as _, sacl.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "fwpmtypes", feature = "fwptypes", feature = "minwindef", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn FwpmNetEventCreateEnumHandle(enginehandle: super::HANDLE, enumtemplate: Option<*const super::FWPM_NET_EVENT_ENUM_TEMPLATE0>, enumhandle: *mut super::HANDLE) -> super::DWORD {
    windows_core::link!("fwpuclnt.dll" "system" "FwpmNetEventCreateEnumHandle0" fn FwpmNetEventCreateEnumHandle(enginehandle : super::HANDLE, enumtemplate : *const super::FWPM_NET_EVENT_ENUM_TEMPLATE0, enumhandle : *mut super::HANDLE) -> super::DWORD);
    unsafe { FwpmNetEventCreateEnumHandle(enginehandle, enumtemplate.unwrap_or(core::mem::zeroed()) as _, enumhandle as _) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn FwpmNetEventDestroyEnumHandle(enginehandle: super::HANDLE, enumhandle: super::HANDLE) -> super::DWORD {
    windows_core::link!("fwpuclnt.dll" "system" "FwpmNetEventDestroyEnumHandle0" fn FwpmNetEventDestroyEnumHandle(enginehandle : super::HANDLE, enumhandle : super::HANDLE) -> super::DWORD);
    unsafe { FwpmNetEventDestroyEnumHandle(enginehandle, enumhandle as _) }
}
#[cfg(all(feature = "fwpmtypes", feature = "fwptypes", feature = "iketypes", feature = "ipsectypes", feature = "minwindef", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn FwpmNetEventEnum(enginehandle: super::HANDLE, enumhandle: super::HANDLE, numentriesrequested: super::UINT32, entries: *mut *mut *mut super::FWPM_NET_EVENT5, numentriesreturned: *mut super::UINT32) -> super::DWORD {
    windows_core::link!("fwpuclnt.dll" "system" "FwpmNetEventEnum5" fn FwpmNetEventEnum(enginehandle : super::HANDLE, enumhandle : super::HANDLE, numentriesrequested : super::UINT32, entries : *mut *mut *mut super::FWPM_NET_EVENT5, numentriesreturned : *mut super::UINT32) -> super::DWORD);
    unsafe { FwpmNetEventEnum(enginehandle, enumhandle, numentriesrequested, entries as _, numentriesreturned as _) }
}
#[cfg(all(feature = "fwpmtypes", feature = "fwpmu", feature = "fwptypes", feature = "iketypes", feature = "ipsectypes", feature = "minwindef", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn FwpmNetEventSubscribe(enginehandle: super::HANDLE, subscription: *const super::FWPM_NET_EVENT_SUBSCRIPTION0, callback: super::FWPM_NET_EVENT_CALLBACK4, context: Option<*const core::ffi::c_void>, eventshandle: *mut super::HANDLE) -> super::DWORD {
    windows_core::link!("fwpuclnt.dll" "system" "FwpmNetEventSubscribe4" fn FwpmNetEventSubscribe(enginehandle : super::HANDLE, subscription : *const super::FWPM_NET_EVENT_SUBSCRIPTION0, callback : super::FWPM_NET_EVENT_CALLBACK4, context : *const core::ffi::c_void, eventshandle : *mut super::HANDLE) -> super::DWORD);
    unsafe { FwpmNetEventSubscribe(enginehandle, subscription, callback, context.unwrap_or(core::mem::zeroed()) as _, eventshandle as _) }
}
#[cfg(all(feature = "fwpmtypes", feature = "fwptypes", feature = "minwindef", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn FwpmNetEventSubscriptionsGet(enginehandle: super::HANDLE, entries: *mut *mut *mut super::FWPM_NET_EVENT_SUBSCRIPTION0, numentries: *mut super::UINT32) -> super::DWORD {
    windows_core::link!("fwpuclnt.dll" "system" "FwpmNetEventSubscriptionsGet0" fn FwpmNetEventSubscriptionsGet(enginehandle : super::HANDLE, entries : *mut *mut *mut super::FWPM_NET_EVENT_SUBSCRIPTION0, numentries : *mut super::UINT32) -> super::DWORD);
    unsafe { FwpmNetEventSubscriptionsGet(enginehandle, entries as _, numentries as _) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn FwpmNetEventUnsubscribe(enginehandle: super::HANDLE, eventshandle: super::HANDLE) -> super::DWORD {
    windows_core::link!("fwpuclnt.dll" "system" "FwpmNetEventUnsubscribe0" fn FwpmNetEventUnsubscribe(enginehandle : super::HANDLE, eventshandle : super::HANDLE) -> super::DWORD);
    unsafe { FwpmNetEventUnsubscribe(enginehandle, eventshandle as _) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn FwpmNetEventsGetSecurityInfo(enginehandle: super::HANDLE, securityinfo: super::SECURITY_INFORMATION, sidowner: *mut super::PSID, sidgroup: *mut super::PSID, dacl: *mut super::PACL, sacl: *mut super::PACL, securitydescriptor: *mut super::PSECURITY_DESCRIPTOR) -> super::DWORD {
    windows_core::link!("fwpuclnt.dll" "system" "FwpmNetEventsGetSecurityInfo0" fn FwpmNetEventsGetSecurityInfo(enginehandle : super::HANDLE, securityinfo : super::SECURITY_INFORMATION, sidowner : *mut super::PSID, sidgroup : *mut super::PSID, dacl : *mut super::PACL, sacl : *mut super::PACL, securitydescriptor : *mut super::PSECURITY_DESCRIPTOR) -> super::DWORD);
    unsafe { FwpmNetEventsGetSecurityInfo(enginehandle, securityinfo, sidowner as _, sidgroup as _, dacl as _, sacl as _, securitydescriptor as _) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn FwpmNetEventsSetSecurityInfo(enginehandle: super::HANDLE, securityinfo: super::SECURITY_INFORMATION, sidowner: Option<*const super::SID>, sidgroup: Option<*const super::SID>, dacl: Option<*const super::ACL>, sacl: Option<*const super::ACL>) -> super::DWORD {
    windows_core::link!("fwpuclnt.dll" "system" "FwpmNetEventsSetSecurityInfo0" fn FwpmNetEventsSetSecurityInfo(enginehandle : super::HANDLE, securityinfo : super::SECURITY_INFORMATION, sidowner : *const super::SID, sidgroup : *const super::SID, dacl : *const super::ACL, sacl : *const super::ACL) -> super::DWORD);
    unsafe { FwpmNetEventsSetSecurityInfo(enginehandle, securityinfo, sidowner.unwrap_or(core::mem::zeroed()) as _, sidgroup.unwrap_or(core::mem::zeroed()) as _, dacl.unwrap_or(core::mem::zeroed()) as _, sacl.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "fwpmtypes", feature = "fwptypes", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn FwpmProviderAdd(enginehandle: super::HANDLE, provider: *const super::FWPM_PROVIDER0, sd: Option<super::PSECURITY_DESCRIPTOR>) -> super::DWORD {
    windows_core::link!("fwpuclnt.dll" "system" "FwpmProviderAdd0" fn FwpmProviderAdd(enginehandle : super::HANDLE, provider : *const super::FWPM_PROVIDER0, sd : super::PSECURITY_DESCRIPTOR) -> super::DWORD);
    unsafe { FwpmProviderAdd(enginehandle, provider, sd.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "fwpmtypes", feature = "fwptypes", feature = "iketypes", feature = "ipsectypes", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn FwpmProviderContextAdd(enginehandle: super::HANDLE, providercontext: *const super::FWPM_PROVIDER_CONTEXT3, sd: Option<super::PSECURITY_DESCRIPTOR>, id: Option<*mut super::UINT64>) -> super::DWORD {
    windows_core::link!("fwpuclnt.dll" "system" "FwpmProviderContextAdd3" fn FwpmProviderContextAdd(enginehandle : super::HANDLE, providercontext : *const super::FWPM_PROVIDER_CONTEXT3, sd : super::PSECURITY_DESCRIPTOR, id : *mut super::UINT64) -> super::DWORD);
    unsafe { FwpmProviderContextAdd(enginehandle, providercontext, sd.unwrap_or(core::mem::zeroed()) as _, id.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "fwpmtypes", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn FwpmProviderContextCreateEnumHandle(enginehandle: super::HANDLE, enumtemplate: Option<*const super::FWPM_PROVIDER_CONTEXT_ENUM_TEMPLATE0>, enumhandle: *mut super::HANDLE) -> super::DWORD {
    windows_core::link!("fwpuclnt.dll" "system" "FwpmProviderContextCreateEnumHandle0" fn FwpmProviderContextCreateEnumHandle(enginehandle : super::HANDLE, enumtemplate : *const super::FWPM_PROVIDER_CONTEXT_ENUM_TEMPLATE0, enumhandle : *mut super::HANDLE) -> super::DWORD);
    unsafe { FwpmProviderContextCreateEnumHandle(enginehandle, enumtemplate.unwrap_or(core::mem::zeroed()) as _, enumhandle as _) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn FwpmProviderContextDeleteById(enginehandle: super::HANDLE, id: super::UINT64) -> super::DWORD {
    windows_core::link!("fwpuclnt.dll" "system" "FwpmProviderContextDeleteById0" fn FwpmProviderContextDeleteById(enginehandle : super::HANDLE, id : super::UINT64) -> super::DWORD);
    unsafe { FwpmProviderContextDeleteById(enginehandle, id) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn FwpmProviderContextDeleteByKey(enginehandle: super::HANDLE, key: *const windows_core::GUID) -> super::DWORD {
    windows_core::link!("fwpuclnt.dll" "system" "FwpmProviderContextDeleteByKey0" fn FwpmProviderContextDeleteByKey(enginehandle : super::HANDLE, key : *const windows_core::GUID) -> super::DWORD);
    unsafe { FwpmProviderContextDeleteByKey(enginehandle, key) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn FwpmProviderContextDestroyEnumHandle(enginehandle: super::HANDLE, enumhandle: super::HANDLE) -> super::DWORD {
    windows_core::link!("fwpuclnt.dll" "system" "FwpmProviderContextDestroyEnumHandle0" fn FwpmProviderContextDestroyEnumHandle(enginehandle : super::HANDLE, enumhandle : super::HANDLE) -> super::DWORD);
    unsafe { FwpmProviderContextDestroyEnumHandle(enginehandle, enumhandle as _) }
}
#[cfg(all(feature = "fwpmtypes", feature = "fwptypes", feature = "iketypes", feature = "ipsectypes", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn FwpmProviderContextEnum(enginehandle: super::HANDLE, enumhandle: super::HANDLE, numentriesrequested: super::UINT32, entries: *mut *mut *mut super::FWPM_PROVIDER_CONTEXT3, numentriesreturned: *mut super::UINT32) -> super::DWORD {
    windows_core::link!("fwpuclnt.dll" "system" "FwpmProviderContextEnum3" fn FwpmProviderContextEnum(enginehandle : super::HANDLE, enumhandle : super::HANDLE, numentriesrequested : super::UINT32, entries : *mut *mut *mut super::FWPM_PROVIDER_CONTEXT3, numentriesreturned : *mut super::UINT32) -> super::DWORD);
    unsafe { FwpmProviderContextEnum(enginehandle, enumhandle, numentriesrequested, entries as _, numentriesreturned as _) }
}
#[cfg(all(feature = "fwpmtypes", feature = "fwptypes", feature = "iketypes", feature = "ipsectypes", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn FwpmProviderContextGetById(enginehandle: super::HANDLE, id: super::UINT64, providercontext: *mut *mut super::FWPM_PROVIDER_CONTEXT3) -> super::DWORD {
    windows_core::link!("fwpuclnt.dll" "system" "FwpmProviderContextGetById3" fn FwpmProviderContextGetById(enginehandle : super::HANDLE, id : super::UINT64, providercontext : *mut *mut super::FWPM_PROVIDER_CONTEXT3) -> super::DWORD);
    unsafe { FwpmProviderContextGetById(enginehandle, id, providercontext as _) }
}
#[cfg(all(feature = "fwpmtypes", feature = "fwptypes", feature = "iketypes", feature = "ipsectypes", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn FwpmProviderContextGetByKey(enginehandle: super::HANDLE, key: *const windows_core::GUID, providercontext: *mut *mut super::FWPM_PROVIDER_CONTEXT3) -> super::DWORD {
    windows_core::link!("fwpuclnt.dll" "system" "FwpmProviderContextGetByKey3" fn FwpmProviderContextGetByKey(enginehandle : super::HANDLE, key : *const windows_core::GUID, providercontext : *mut *mut super::FWPM_PROVIDER_CONTEXT3) -> super::DWORD);
    unsafe { FwpmProviderContextGetByKey(enginehandle, key, providercontext as _) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn FwpmProviderContextGetSecurityInfoByKey(enginehandle: super::HANDLE, key: Option<*const windows_core::GUID>, securityinfo: super::SECURITY_INFORMATION, sidowner: *mut super::PSID, sidgroup: *mut super::PSID, dacl: *mut super::PACL, sacl: *mut super::PACL, securitydescriptor: *mut super::PSECURITY_DESCRIPTOR) -> super::DWORD {
    windows_core::link!("fwpuclnt.dll" "system" "FwpmProviderContextGetSecurityInfoByKey0" fn FwpmProviderContextGetSecurityInfoByKey(enginehandle : super::HANDLE, key : *const windows_core::GUID, securityinfo : super::SECURITY_INFORMATION, sidowner : *mut super::PSID, sidgroup : *mut super::PSID, dacl : *mut super::PACL, sacl : *mut super::PACL, securitydescriptor : *mut super::PSECURITY_DESCRIPTOR) -> super::DWORD);
    unsafe { FwpmProviderContextGetSecurityInfoByKey(enginehandle, key.unwrap_or(core::mem::zeroed()) as _, securityinfo, sidowner as _, sidgroup as _, dacl as _, sacl as _, securitydescriptor as _) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn FwpmProviderContextSetSecurityInfoByKey(enginehandle: super::HANDLE, key: Option<*const windows_core::GUID>, securityinfo: super::SECURITY_INFORMATION, sidowner: Option<*const super::SID>, sidgroup: Option<*const super::SID>, dacl: Option<*const super::ACL>, sacl: Option<*const super::ACL>) -> super::DWORD {
    windows_core::link!("fwpuclnt.dll" "system" "FwpmProviderContextSetSecurityInfoByKey0" fn FwpmProviderContextSetSecurityInfoByKey(enginehandle : super::HANDLE, key : *const windows_core::GUID, securityinfo : super::SECURITY_INFORMATION, sidowner : *const super::SID, sidgroup : *const super::SID, dacl : *const super::ACL, sacl : *const super::ACL) -> super::DWORD);
    unsafe { FwpmProviderContextSetSecurityInfoByKey(enginehandle, key.unwrap_or(core::mem::zeroed()) as _, securityinfo, sidowner.unwrap_or(core::mem::zeroed()) as _, sidgroup.unwrap_or(core::mem::zeroed()) as _, dacl.unwrap_or(core::mem::zeroed()) as _, sacl.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "fwpmtypes", feature = "fwpmu", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn FwpmProviderContextSubscribeChanges(enginehandle: super::HANDLE, subscription: *const super::FWPM_PROVIDER_CONTEXT_SUBSCRIPTION0, callback: super::FWPM_PROVIDER_CONTEXT_CHANGE_CALLBACK0, context: Option<*const core::ffi::c_void>, changehandle: *mut super::HANDLE) -> super::DWORD {
    windows_core::link!("fwpuclnt.dll" "system" "FwpmProviderContextSubscribeChanges0" fn FwpmProviderContextSubscribeChanges(enginehandle : super::HANDLE, subscription : *const super::FWPM_PROVIDER_CONTEXT_SUBSCRIPTION0, callback : super::FWPM_PROVIDER_CONTEXT_CHANGE_CALLBACK0, context : *const core::ffi::c_void, changehandle : *mut super::HANDLE) -> super::DWORD);
    unsafe { FwpmProviderContextSubscribeChanges(enginehandle, subscription, callback, context.unwrap_or(core::mem::zeroed()) as _, changehandle as _) }
}
#[cfg(all(feature = "fwpmtypes", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn FwpmProviderContextSubscriptionsGet(enginehandle: super::HANDLE, entries: *mut *mut *mut super::FWPM_PROVIDER_CONTEXT_SUBSCRIPTION0, numentries: *mut super::UINT32) -> super::DWORD {
    windows_core::link!("fwpuclnt.dll" "system" "FwpmProviderContextSubscriptionsGet0" fn FwpmProviderContextSubscriptionsGet(enginehandle : super::HANDLE, entries : *mut *mut *mut super::FWPM_PROVIDER_CONTEXT_SUBSCRIPTION0, numentries : *mut super::UINT32) -> super::DWORD);
    unsafe { FwpmProviderContextSubscriptionsGet(enginehandle, entries as _, numentries as _) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn FwpmProviderContextUnsubscribeChanges(enginehandle: super::HANDLE, changehandle: super::HANDLE) -> super::DWORD {
    windows_core::link!("fwpuclnt.dll" "system" "FwpmProviderContextUnsubscribeChanges0" fn FwpmProviderContextUnsubscribeChanges(enginehandle : super::HANDLE, changehandle : super::HANDLE) -> super::DWORD);
    unsafe { FwpmProviderContextUnsubscribeChanges(enginehandle, changehandle as _) }
}
#[cfg(all(feature = "fwpmtypes", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn FwpmProviderCreateEnumHandle(enginehandle: super::HANDLE, enumtemplate: Option<*const super::FWPM_PROVIDER_ENUM_TEMPLATE0>, enumhandle: *mut super::HANDLE) -> super::DWORD {
    windows_core::link!("fwpuclnt.dll" "system" "FwpmProviderCreateEnumHandle0" fn FwpmProviderCreateEnumHandle(enginehandle : super::HANDLE, enumtemplate : *const super::FWPM_PROVIDER_ENUM_TEMPLATE0, enumhandle : *mut super::HANDLE) -> super::DWORD);
    unsafe { FwpmProviderCreateEnumHandle(enginehandle, enumtemplate.unwrap_or(core::mem::zeroed()) as _, enumhandle as _) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn FwpmProviderDeleteByKey(enginehandle: super::HANDLE, key: *const windows_core::GUID) -> super::DWORD {
    windows_core::link!("fwpuclnt.dll" "system" "FwpmProviderDeleteByKey0" fn FwpmProviderDeleteByKey(enginehandle : super::HANDLE, key : *const windows_core::GUID) -> super::DWORD);
    unsafe { FwpmProviderDeleteByKey(enginehandle, key) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn FwpmProviderDestroyEnumHandle(enginehandle: super::HANDLE, enumhandle: super::HANDLE) -> super::DWORD {
    windows_core::link!("fwpuclnt.dll" "system" "FwpmProviderDestroyEnumHandle0" fn FwpmProviderDestroyEnumHandle(enginehandle : super::HANDLE, enumhandle : super::HANDLE) -> super::DWORD);
    unsafe { FwpmProviderDestroyEnumHandle(enginehandle, enumhandle as _) }
}
#[cfg(all(feature = "fwpmtypes", feature = "fwptypes", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn FwpmProviderEnum(enginehandle: super::HANDLE, enumhandle: super::HANDLE, numentriesrequested: super::UINT32, entries: *mut *mut *mut super::FWPM_PROVIDER0, numentriesreturned: *mut super::UINT32) -> super::DWORD {
    windows_core::link!("fwpuclnt.dll" "system" "FwpmProviderEnum0" fn FwpmProviderEnum(enginehandle : super::HANDLE, enumhandle : super::HANDLE, numentriesrequested : super::UINT32, entries : *mut *mut *mut super::FWPM_PROVIDER0, numentriesreturned : *mut super::UINT32) -> super::DWORD);
    unsafe { FwpmProviderEnum(enginehandle, enumhandle, numentriesrequested, entries as _, numentriesreturned as _) }
}
#[cfg(all(feature = "fwpmtypes", feature = "fwptypes", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn FwpmProviderGetByKey(enginehandle: super::HANDLE, key: *const windows_core::GUID, provider: *mut *mut super::FWPM_PROVIDER0) -> super::DWORD {
    windows_core::link!("fwpuclnt.dll" "system" "FwpmProviderGetByKey0" fn FwpmProviderGetByKey(enginehandle : super::HANDLE, key : *const windows_core::GUID, provider : *mut *mut super::FWPM_PROVIDER0) -> super::DWORD);
    unsafe { FwpmProviderGetByKey(enginehandle, key, provider as _) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn FwpmProviderGetSecurityInfoByKey(enginehandle: super::HANDLE, key: Option<*const windows_core::GUID>, securityinfo: super::SECURITY_INFORMATION, sidowner: *mut super::PSID, sidgroup: *mut super::PSID, dacl: *mut super::PACL, sacl: *mut super::PACL, securitydescriptor: *mut super::PSECURITY_DESCRIPTOR) -> super::DWORD {
    windows_core::link!("fwpuclnt.dll" "system" "FwpmProviderGetSecurityInfoByKey0" fn FwpmProviderGetSecurityInfoByKey(enginehandle : super::HANDLE, key : *const windows_core::GUID, securityinfo : super::SECURITY_INFORMATION, sidowner : *mut super::PSID, sidgroup : *mut super::PSID, dacl : *mut super::PACL, sacl : *mut super::PACL, securitydescriptor : *mut super::PSECURITY_DESCRIPTOR) -> super::DWORD);
    unsafe { FwpmProviderGetSecurityInfoByKey(enginehandle, key.unwrap_or(core::mem::zeroed()) as _, securityinfo, sidowner as _, sidgroup as _, dacl as _, sacl as _, securitydescriptor as _) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn FwpmProviderSetSecurityInfoByKey(enginehandle: super::HANDLE, key: Option<*const windows_core::GUID>, securityinfo: super::SECURITY_INFORMATION, sidowner: Option<*const super::SID>, sidgroup: Option<*const super::SID>, dacl: Option<*const super::ACL>, sacl: Option<*const super::ACL>) -> super::DWORD {
    windows_core::link!("fwpuclnt.dll" "system" "FwpmProviderSetSecurityInfoByKey0" fn FwpmProviderSetSecurityInfoByKey(enginehandle : super::HANDLE, key : *const windows_core::GUID, securityinfo : super::SECURITY_INFORMATION, sidowner : *const super::SID, sidgroup : *const super::SID, dacl : *const super::ACL, sacl : *const super::ACL) -> super::DWORD);
    unsafe { FwpmProviderSetSecurityInfoByKey(enginehandle, key.unwrap_or(core::mem::zeroed()) as _, securityinfo, sidowner.unwrap_or(core::mem::zeroed()) as _, sidgroup.unwrap_or(core::mem::zeroed()) as _, dacl.unwrap_or(core::mem::zeroed()) as _, sacl.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "fwpmtypes", feature = "fwpmu", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn FwpmProviderSubscribeChanges(enginehandle: super::HANDLE, subscription: *const super::FWPM_PROVIDER_SUBSCRIPTION0, callback: super::FWPM_PROVIDER_CHANGE_CALLBACK0, context: Option<*const core::ffi::c_void>, changehandle: *mut super::HANDLE) -> super::DWORD {
    windows_core::link!("fwpuclnt.dll" "system" "FwpmProviderSubscribeChanges0" fn FwpmProviderSubscribeChanges(enginehandle : super::HANDLE, subscription : *const super::FWPM_PROVIDER_SUBSCRIPTION0, callback : super::FWPM_PROVIDER_CHANGE_CALLBACK0, context : *const core::ffi::c_void, changehandle : *mut super::HANDLE) -> super::DWORD);
    unsafe { FwpmProviderSubscribeChanges(enginehandle, subscription, callback, context.unwrap_or(core::mem::zeroed()) as _, changehandle as _) }
}
#[cfg(all(feature = "fwpmtypes", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn FwpmProviderSubscriptionsGet(enginehandle: super::HANDLE, entries: *mut *mut *mut super::FWPM_PROVIDER_SUBSCRIPTION0, numentries: *mut super::UINT32) -> super::DWORD {
    windows_core::link!("fwpuclnt.dll" "system" "FwpmProviderSubscriptionsGet0" fn FwpmProviderSubscriptionsGet(enginehandle : super::HANDLE, entries : *mut *mut *mut super::FWPM_PROVIDER_SUBSCRIPTION0, numentries : *mut super::UINT32) -> super::DWORD);
    unsafe { FwpmProviderSubscriptionsGet(enginehandle, entries as _, numentries as _) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn FwpmProviderUnsubscribeChanges(enginehandle: super::HANDLE, changehandle: super::HANDLE) -> super::DWORD {
    windows_core::link!("fwpuclnt.dll" "system" "FwpmProviderUnsubscribeChanges0" fn FwpmProviderUnsubscribeChanges(enginehandle : super::HANDLE, changehandle : super::HANDLE) -> super::DWORD);
    unsafe { FwpmProviderUnsubscribeChanges(enginehandle, changehandle as _) }
}
#[cfg(all(feature = "fwpmtypes", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn FwpmSessionCreateEnumHandle(enginehandle: super::HANDLE, enumtemplate: Option<*const super::FWPM_SESSION_ENUM_TEMPLATE0>, enumhandle: *mut super::HANDLE) -> super::DWORD {
    windows_core::link!("fwpuclnt.dll" "system" "FwpmSessionCreateEnumHandle0" fn FwpmSessionCreateEnumHandle(enginehandle : super::HANDLE, enumtemplate : *const super::FWPM_SESSION_ENUM_TEMPLATE0, enumhandle : *mut super::HANDLE) -> super::DWORD);
    unsafe { FwpmSessionCreateEnumHandle(enginehandle, enumtemplate.unwrap_or(core::mem::zeroed()) as _, enumhandle as _) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn FwpmSessionDestroyEnumHandle(enginehandle: super::HANDLE, enumhandle: super::HANDLE) -> super::DWORD {
    windows_core::link!("fwpuclnt.dll" "system" "FwpmSessionDestroyEnumHandle0" fn FwpmSessionDestroyEnumHandle(enginehandle : super::HANDLE, enumhandle : super::HANDLE) -> super::DWORD);
    unsafe { FwpmSessionDestroyEnumHandle(enginehandle, enumhandle as _) }
}
#[cfg(all(feature = "fwpmtypes", feature = "fwptypes", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn FwpmSessionEnum(enginehandle: super::HANDLE, enumhandle: super::HANDLE, numentriesrequested: super::UINT32, entries: *mut *mut *mut super::FWPM_SESSION0, numentriesreturned: *mut super::UINT32) -> super::DWORD {
    windows_core::link!("fwpuclnt.dll" "system" "FwpmSessionEnum0" fn FwpmSessionEnum(enginehandle : super::HANDLE, enumhandle : super::HANDLE, numentriesrequested : super::UINT32, entries : *mut *mut *mut super::FWPM_SESSION0, numentriesreturned : *mut super::UINT32) -> super::DWORD);
    unsafe { FwpmSessionEnum(enginehandle, enumhandle, numentriesrequested, entries as _, numentriesreturned as _) }
}
#[cfg(all(feature = "fwpmtypes", feature = "fwptypes", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn FwpmSubLayerAdd(enginehandle: super::HANDLE, sublayer: *const super::FWPM_SUBLAYER0, sd: Option<super::PSECURITY_DESCRIPTOR>) -> super::DWORD {
    windows_core::link!("fwpuclnt.dll" "system" "FwpmSubLayerAdd0" fn FwpmSubLayerAdd(enginehandle : super::HANDLE, sublayer : *const super::FWPM_SUBLAYER0, sd : super::PSECURITY_DESCRIPTOR) -> super::DWORD);
    unsafe { FwpmSubLayerAdd(enginehandle, sublayer, sd.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "fwpmtypes", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn FwpmSubLayerCreateEnumHandle(enginehandle: super::HANDLE, enumtemplate: Option<*const super::FWPM_SUBLAYER_ENUM_TEMPLATE0>, enumhandle: *mut super::HANDLE) -> super::DWORD {
    windows_core::link!("fwpuclnt.dll" "system" "FwpmSubLayerCreateEnumHandle0" fn FwpmSubLayerCreateEnumHandle(enginehandle : super::HANDLE, enumtemplate : *const super::FWPM_SUBLAYER_ENUM_TEMPLATE0, enumhandle : *mut super::HANDLE) -> super::DWORD);
    unsafe { FwpmSubLayerCreateEnumHandle(enginehandle, enumtemplate.unwrap_or(core::mem::zeroed()) as _, enumhandle as _) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn FwpmSubLayerDeleteByKey(enginehandle: super::HANDLE, key: *const windows_core::GUID) -> super::DWORD {
    windows_core::link!("fwpuclnt.dll" "system" "FwpmSubLayerDeleteByKey0" fn FwpmSubLayerDeleteByKey(enginehandle : super::HANDLE, key : *const windows_core::GUID) -> super::DWORD);
    unsafe { FwpmSubLayerDeleteByKey(enginehandle, key) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn FwpmSubLayerDestroyEnumHandle(enginehandle: super::HANDLE, enumhandle: super::HANDLE) -> super::DWORD {
    windows_core::link!("fwpuclnt.dll" "system" "FwpmSubLayerDestroyEnumHandle0" fn FwpmSubLayerDestroyEnumHandle(enginehandle : super::HANDLE, enumhandle : super::HANDLE) -> super::DWORD);
    unsafe { FwpmSubLayerDestroyEnumHandle(enginehandle, enumhandle as _) }
}
#[cfg(all(feature = "fwpmtypes", feature = "fwptypes", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn FwpmSubLayerEnum(enginehandle: super::HANDLE, enumhandle: super::HANDLE, numentriesrequested: super::UINT32, entries: *mut *mut *mut super::FWPM_SUBLAYER0, numentriesreturned: *mut super::UINT32) -> super::DWORD {
    windows_core::link!("fwpuclnt.dll" "system" "FwpmSubLayerEnum0" fn FwpmSubLayerEnum(enginehandle : super::HANDLE, enumhandle : super::HANDLE, numentriesrequested : super::UINT32, entries : *mut *mut *mut super::FWPM_SUBLAYER0, numentriesreturned : *mut super::UINT32) -> super::DWORD);
    unsafe { FwpmSubLayerEnum(enginehandle, enumhandle, numentriesrequested, entries as _, numentriesreturned as _) }
}
#[cfg(all(feature = "fwpmtypes", feature = "fwptypes", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn FwpmSubLayerGetByKey(enginehandle: super::HANDLE, key: *const windows_core::GUID, sublayer: *mut *mut super::FWPM_SUBLAYER0) -> super::DWORD {
    windows_core::link!("fwpuclnt.dll" "system" "FwpmSubLayerGetByKey0" fn FwpmSubLayerGetByKey(enginehandle : super::HANDLE, key : *const windows_core::GUID, sublayer : *mut *mut super::FWPM_SUBLAYER0) -> super::DWORD);
    unsafe { FwpmSubLayerGetByKey(enginehandle, key, sublayer as _) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn FwpmSubLayerGetSecurityInfoByKey(enginehandle: super::HANDLE, key: Option<*const windows_core::GUID>, securityinfo: super::SECURITY_INFORMATION, sidowner: *mut super::PSID, sidgroup: *mut super::PSID, dacl: *mut super::PACL, sacl: *mut super::PACL, securitydescriptor: *mut super::PSECURITY_DESCRIPTOR) -> super::DWORD {
    windows_core::link!("fwpuclnt.dll" "system" "FwpmSubLayerGetSecurityInfoByKey0" fn FwpmSubLayerGetSecurityInfoByKey(enginehandle : super::HANDLE, key : *const windows_core::GUID, securityinfo : super::SECURITY_INFORMATION, sidowner : *mut super::PSID, sidgroup : *mut super::PSID, dacl : *mut super::PACL, sacl : *mut super::PACL, securitydescriptor : *mut super::PSECURITY_DESCRIPTOR) -> super::DWORD);
    unsafe { FwpmSubLayerGetSecurityInfoByKey(enginehandle, key.unwrap_or(core::mem::zeroed()) as _, securityinfo, sidowner as _, sidgroup as _, dacl as _, sacl as _, securitydescriptor as _) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn FwpmSubLayerSetSecurityInfoByKey(enginehandle: super::HANDLE, key: Option<*const windows_core::GUID>, securityinfo: super::SECURITY_INFORMATION, sidowner: Option<*const super::SID>, sidgroup: Option<*const super::SID>, dacl: Option<*const super::ACL>, sacl: Option<*const super::ACL>) -> super::DWORD {
    windows_core::link!("fwpuclnt.dll" "system" "FwpmSubLayerSetSecurityInfoByKey0" fn FwpmSubLayerSetSecurityInfoByKey(enginehandle : super::HANDLE, key : *const windows_core::GUID, securityinfo : super::SECURITY_INFORMATION, sidowner : *const super::SID, sidgroup : *const super::SID, dacl : *const super::ACL, sacl : *const super::ACL) -> super::DWORD);
    unsafe { FwpmSubLayerSetSecurityInfoByKey(enginehandle, key.unwrap_or(core::mem::zeroed()) as _, securityinfo, sidowner.unwrap_or(core::mem::zeroed()) as _, sidgroup.unwrap_or(core::mem::zeroed()) as _, dacl.unwrap_or(core::mem::zeroed()) as _, sacl.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "fwpmtypes", feature = "fwpmu", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn FwpmSubLayerSubscribeChanges(enginehandle: super::HANDLE, subscription: *const super::FWPM_SUBLAYER_SUBSCRIPTION0, callback: super::FWPM_SUBLAYER_CHANGE_CALLBACK0, context: Option<*const core::ffi::c_void>, changehandle: *mut super::HANDLE) -> super::DWORD {
    windows_core::link!("fwpuclnt.dll" "system" "FwpmSubLayerSubscribeChanges0" fn FwpmSubLayerSubscribeChanges(enginehandle : super::HANDLE, subscription : *const super::FWPM_SUBLAYER_SUBSCRIPTION0, callback : super::FWPM_SUBLAYER_CHANGE_CALLBACK0, context : *const core::ffi::c_void, changehandle : *mut super::HANDLE) -> super::DWORD);
    unsafe { FwpmSubLayerSubscribeChanges(enginehandle, subscription, callback, context.unwrap_or(core::mem::zeroed()) as _, changehandle as _) }
}
#[cfg(all(feature = "fwpmtypes", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn FwpmSubLayerSubscriptionsGet(enginehandle: super::HANDLE, entries: *mut *mut *mut super::FWPM_SUBLAYER_SUBSCRIPTION0, numentries: *mut super::UINT32) -> super::DWORD {
    windows_core::link!("fwpuclnt.dll" "system" "FwpmSubLayerSubscriptionsGet0" fn FwpmSubLayerSubscriptionsGet(enginehandle : super::HANDLE, entries : *mut *mut *mut super::FWPM_SUBLAYER_SUBSCRIPTION0, numentries : *mut super::UINT32) -> super::DWORD);
    unsafe { FwpmSubLayerSubscriptionsGet(enginehandle, entries as _, numentries as _) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn FwpmSubLayerUnsubscribeChanges(enginehandle: super::HANDLE, changehandle: super::HANDLE) -> super::DWORD {
    windows_core::link!("fwpuclnt.dll" "system" "FwpmSubLayerUnsubscribeChanges0" fn FwpmSubLayerUnsubscribeChanges(enginehandle : super::HANDLE, changehandle : super::HANDLE) -> super::DWORD);
    unsafe { FwpmSubLayerUnsubscribeChanges(enginehandle, changehandle as _) }
}
#[cfg(all(feature = "fwpmtypes", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn FwpmSystemPortsGet(enginehandle: Option<super::HANDLE>, sysports: *mut *mut super::FWPM_SYSTEM_PORTS0) -> super::DWORD {
    windows_core::link!("fwpuclnt.dll" "system" "FwpmSystemPortsGet0" fn FwpmSystemPortsGet(enginehandle : super::HANDLE, sysports : *mut *mut super::FWPM_SYSTEM_PORTS0) -> super::DWORD);
    unsafe { FwpmSystemPortsGet(enginehandle.unwrap_or(core::mem::zeroed()) as _, sysports as _) }
}
#[cfg(all(feature = "fwpmtypes", feature = "fwpmu", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn FwpmSystemPortsSubscribe(enginehandle: Option<super::HANDLE>, reserved: Option<*mut core::ffi::c_void>, callback: super::FWPM_SYSTEM_PORTS_CALLBACK0, context: Option<*const core::ffi::c_void>, sysportshandle: *mut super::HANDLE) -> super::DWORD {
    windows_core::link!("fwpuclnt.dll" "system" "FwpmSystemPortsSubscribe0" fn FwpmSystemPortsSubscribe(enginehandle : super::HANDLE, reserved : *mut core::ffi::c_void, callback : super::FWPM_SYSTEM_PORTS_CALLBACK0, context : *const core::ffi::c_void, sysportshandle : *mut super::HANDLE) -> super::DWORD);
    unsafe { FwpmSystemPortsSubscribe(enginehandle.unwrap_or(core::mem::zeroed()) as _, reserved.unwrap_or(core::mem::zeroed()) as _, callback, context.unwrap_or(core::mem::zeroed()) as _, sysportshandle as _) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn FwpmSystemPortsUnsubscribe(enginehandle: Option<super::HANDLE>, sysportshandle: super::HANDLE) -> super::DWORD {
    windows_core::link!("fwpuclnt.dll" "system" "FwpmSystemPortsUnsubscribe0" fn FwpmSystemPortsUnsubscribe(enginehandle : super::HANDLE, sysportshandle : super::HANDLE) -> super::DWORD);
    unsafe { FwpmSystemPortsUnsubscribe(enginehandle.unwrap_or(core::mem::zeroed()) as _, sysportshandle as _) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn FwpmTransactionAbort(enginehandle: super::HANDLE) -> super::DWORD {
    windows_core::link!("fwpuclnt.dll" "system" "FwpmTransactionAbort0" fn FwpmTransactionAbort(enginehandle : super::HANDLE) -> super::DWORD);
    unsafe { FwpmTransactionAbort(enginehandle) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn FwpmTransactionBegin(enginehandle: super::HANDLE, flags: super::UINT32) -> super::DWORD {
    windows_core::link!("fwpuclnt.dll" "system" "FwpmTransactionBegin0" fn FwpmTransactionBegin(enginehandle : super::HANDLE, flags : super::UINT32) -> super::DWORD);
    unsafe { FwpmTransactionBegin(enginehandle, flags) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn FwpmTransactionCommit(enginehandle: super::HANDLE) -> super::DWORD {
    windows_core::link!("fwpuclnt.dll" "system" "FwpmTransactionCommit0" fn FwpmTransactionCommit(enginehandle : super::HANDLE) -> super::DWORD);
    unsafe { FwpmTransactionCommit(enginehandle) }
}
#[cfg(all(feature = "fwpmtypes", feature = "fwpmu", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn FwpmvSwitchEventSubscribe(enginehandle: super::HANDLE, subscription: *const super::FWPM_VSWITCH_EVENT_SUBSCRIPTION0, callback: super::FWPM_VSWITCH_EVENT_CALLBACK0, context: Option<*const core::ffi::c_void>, subscriptionhandle: *mut super::HANDLE) -> super::DWORD {
    windows_core::link!("fwpuclnt.dll" "system" "FwpmvSwitchEventSubscribe0" fn FwpmvSwitchEventSubscribe(enginehandle : super::HANDLE, subscription : *const super::FWPM_VSWITCH_EVENT_SUBSCRIPTION0, callback : super::FWPM_VSWITCH_EVENT_CALLBACK0, context : *const core::ffi::c_void, subscriptionhandle : *mut super::HANDLE) -> super::DWORD);
    unsafe { FwpmvSwitchEventSubscribe(enginehandle, subscription, callback, context.unwrap_or(core::mem::zeroed()) as _, subscriptionhandle as _) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn FwpmvSwitchEventUnsubscribe(enginehandle: super::HANDLE, subscriptionhandle: super::HANDLE) -> super::DWORD {
    windows_core::link!("fwpuclnt.dll" "system" "FwpmvSwitchEventUnsubscribe0" fn FwpmvSwitchEventUnsubscribe(enginehandle : super::HANDLE, subscriptionhandle : super::HANDLE) -> super::DWORD);
    unsafe { FwpmvSwitchEventUnsubscribe(enginehandle, subscriptionhandle as _) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn FwpmvSwitchEventsGetSecurityInfo(enginehandle: super::HANDLE, securityinfo: super::SECURITY_INFORMATION, sidowner: *mut super::PSID, sidgroup: *mut super::PSID, dacl: *mut super::PACL, sacl: *mut super::PACL, securitydescriptor: *mut super::PSECURITY_DESCRIPTOR) -> super::DWORD {
    windows_core::link!("fwpuclnt.dll" "system" "FwpmvSwitchEventsGetSecurityInfo0" fn FwpmvSwitchEventsGetSecurityInfo(enginehandle : super::HANDLE, securityinfo : super::SECURITY_INFORMATION, sidowner : *mut super::PSID, sidgroup : *mut super::PSID, dacl : *mut super::PACL, sacl : *mut super::PACL, securitydescriptor : *mut super::PSECURITY_DESCRIPTOR) -> super::DWORD);
    unsafe { FwpmvSwitchEventsGetSecurityInfo(enginehandle, securityinfo, sidowner as _, sidgroup as _, dacl as _, sacl as _, securitydescriptor as _) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn FwpmvSwitchEventsSetSecurityInfo(enginehandle: super::HANDLE, securityinfo: super::SECURITY_INFORMATION, sidowner: Option<*const super::SID>, sidgroup: Option<*const super::SID>, dacl: Option<*const super::ACL>, sacl: Option<*const super::ACL>) -> super::DWORD {
    windows_core::link!("fwpuclnt.dll" "system" "FwpmvSwitchEventsSetSecurityInfo0" fn FwpmvSwitchEventsSetSecurityInfo(enginehandle : super::HANDLE, securityinfo : super::SECURITY_INFORMATION, sidowner : *const super::SID, sidgroup : *const super::SID, dacl : *const super::ACL, sacl : *const super::ACL) -> super::DWORD);
    unsafe { FwpmvSwitchEventsSetSecurityInfo(enginehandle, securityinfo, sidowner.unwrap_or(core::mem::zeroed()) as _, sidgroup.unwrap_or(core::mem::zeroed()) as _, dacl.unwrap_or(core::mem::zeroed()) as _, sacl.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn IPsecDospGetSecurityInfo(enginehandle: super::HANDLE, securityinfo: super::SECURITY_INFORMATION, sidowner: *mut super::PSID, sidgroup: *mut super::PSID, dacl: *mut super::PACL, sacl: *mut super::PACL, securitydescriptor: *mut super::PSECURITY_DESCRIPTOR) -> super::DWORD {
    windows_core::link!("fwpuclnt.dll" "system" "IPsecDospGetSecurityInfo0" fn IPsecDospGetSecurityInfo(enginehandle : super::HANDLE, securityinfo : super::SECURITY_INFORMATION, sidowner : *mut super::PSID, sidgroup : *mut super::PSID, dacl : *mut super::PACL, sacl : *mut super::PACL, securitydescriptor : *mut super::PSECURITY_DESCRIPTOR) -> super::DWORD);
    unsafe { IPsecDospGetSecurityInfo(enginehandle, securityinfo, sidowner as _, sidgroup as _, dacl as _, sacl as _, securitydescriptor as _) }
}
#[cfg(all(feature = "ipsectypes", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn IPsecDospGetStatistics(enginehandle: super::HANDLE, idpstatistics: *mut super::IPSEC_DOSP_STATISTICS0) -> super::DWORD {
    windows_core::link!("fwpuclnt.dll" "system" "IPsecDospGetStatistics0" fn IPsecDospGetStatistics(enginehandle : super::HANDLE, idpstatistics : *mut super::IPSEC_DOSP_STATISTICS0) -> super::DWORD);
    unsafe { IPsecDospGetStatistics(enginehandle, idpstatistics as _) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn IPsecDospSetSecurityInfo(enginehandle: super::HANDLE, securityinfo: super::SECURITY_INFORMATION, sidowner: Option<*const super::SID>, sidgroup: Option<*const super::SID>, dacl: Option<*const super::ACL>, sacl: Option<*const super::ACL>) -> super::DWORD {
    windows_core::link!("fwpuclnt.dll" "system" "IPsecDospSetSecurityInfo0" fn IPsecDospSetSecurityInfo(enginehandle : super::HANDLE, securityinfo : super::SECURITY_INFORMATION, sidowner : *const super::SID, sidgroup : *const super::SID, dacl : *const super::ACL, sacl : *const super::ACL) -> super::DWORD);
    unsafe { IPsecDospSetSecurityInfo(enginehandle, securityinfo, sidowner.unwrap_or(core::mem::zeroed()) as _, sidgroup.unwrap_or(core::mem::zeroed()) as _, dacl.unwrap_or(core::mem::zeroed()) as _, sacl.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "fwptypes", feature = "ipsectypes", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn IPsecDospStateCreateEnumHandle(enginehandle: super::HANDLE, enumtemplate: Option<*const super::IPSEC_DOSP_STATE_ENUM_TEMPLATE0>, enumhandle: *mut super::HANDLE) -> super::DWORD {
    windows_core::link!("fwpuclnt.dll" "system" "IPsecDospStateCreateEnumHandle0" fn IPsecDospStateCreateEnumHandle(enginehandle : super::HANDLE, enumtemplate : *const super::IPSEC_DOSP_STATE_ENUM_TEMPLATE0, enumhandle : *mut super::HANDLE) -> super::DWORD);
    unsafe { IPsecDospStateCreateEnumHandle(enginehandle, enumtemplate.unwrap_or(core::mem::zeroed()) as _, enumhandle as _) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn IPsecDospStateDestroyEnumHandle(enginehandle: super::HANDLE, enumhandle: super::HANDLE) -> super::DWORD {
    windows_core::link!("fwpuclnt.dll" "system" "IPsecDospStateDestroyEnumHandle0" fn IPsecDospStateDestroyEnumHandle(enginehandle : super::HANDLE, enumhandle : super::HANDLE) -> super::DWORD);
    unsafe { IPsecDospStateDestroyEnumHandle(enginehandle, enumhandle as _) }
}
#[cfg(all(feature = "ipsectypes", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn IPsecDospStateEnum(enginehandle: super::HANDLE, enumhandle: super::HANDLE, numentriesrequested: super::UINT32, entries: *mut *mut *mut super::IPSEC_DOSP_STATE0, numentries: *mut super::UINT32) -> super::DWORD {
    windows_core::link!("fwpuclnt.dll" "system" "IPsecDospStateEnum0" fn IPsecDospStateEnum(enginehandle : super::HANDLE, enumhandle : super::HANDLE, numentriesrequested : super::UINT32, entries : *mut *mut *mut super::IPSEC_DOSP_STATE0, numentries : *mut super::UINT32) -> super::DWORD);
    unsafe { IPsecDospStateEnum(enginehandle, enumhandle, numentriesrequested, entries as _, numentries as _) }
}
#[cfg(all(feature = "ipsectypes", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn IPsecGetStatistics(enginehandle: super::HANDLE, ipsecstatistics: *mut super::IPSEC_STATISTICS1) -> super::DWORD {
    windows_core::link!("fwpuclnt.dll" "system" "IPsecGetStatistics1" fn IPsecGetStatistics(enginehandle : super::HANDLE, ipsecstatistics : *mut super::IPSEC_STATISTICS1) -> super::DWORD);
    unsafe { IPsecGetStatistics(enginehandle, ipsecstatistics as _) }
}
#[cfg(all(feature = "fwpmtypes", feature = "fwpmu", feature = "fwptypes", feature = "iketypes", feature = "ipsectypes", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn IPsecKeyManagerAddAndRegister(enginehandle: super::HANDLE, keymanager: *const super::IPSEC_KEY_MANAGER0, keymanagercallbacks: *const super::IPSEC_KEY_MANAGER_CALLBACKS0, keymgmthandle: *mut super::HANDLE) -> super::DWORD {
    windows_core::link!("fwpuclnt.dll" "system" "IPsecKeyManagerAddAndRegister0" fn IPsecKeyManagerAddAndRegister(enginehandle : super::HANDLE, keymanager : *const super::IPSEC_KEY_MANAGER0, keymanagercallbacks : *const super::IPSEC_KEY_MANAGER_CALLBACKS0, keymgmthandle : *mut super::HANDLE) -> super::DWORD);
    unsafe { IPsecKeyManagerAddAndRegister(enginehandle, keymanager, keymanagercallbacks, keymgmthandle as _) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn IPsecKeyManagerGetSecurityInfoByKey(enginehandle: super::HANDLE, reserved: Option<*const core::ffi::c_void>, securityinfo: super::SECURITY_INFORMATION, sidowner: *mut super::PSID, sidgroup: *mut super::PSID, dacl: *mut super::PACL, sacl: *mut super::PACL, securitydescriptor: *mut super::PSECURITY_DESCRIPTOR) -> super::DWORD {
    windows_core::link!("fwpuclnt.dll" "system" "IPsecKeyManagerGetSecurityInfoByKey0" fn IPsecKeyManagerGetSecurityInfoByKey(enginehandle : super::HANDLE, reserved : *const core::ffi::c_void, securityinfo : super::SECURITY_INFORMATION, sidowner : *mut super::PSID, sidgroup : *mut super::PSID, dacl : *mut super::PACL, sacl : *mut super::PACL, securitydescriptor : *mut super::PSECURITY_DESCRIPTOR) -> super::DWORD);
    unsafe { IPsecKeyManagerGetSecurityInfoByKey(enginehandle, reserved.unwrap_or(core::mem::zeroed()) as _, securityinfo, sidowner as _, sidgroup as _, dacl as _, sacl as _, securitydescriptor as _) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn IPsecKeyManagerSetSecurityInfoByKey(enginehandle: super::HANDLE, reserved: Option<*const core::ffi::c_void>, securityinfo: super::SECURITY_INFORMATION, sidowner: Option<*const super::SID>, sidgroup: Option<*const super::SID>, dacl: Option<*const super::ACL>, sacl: Option<*const super::ACL>) -> super::DWORD {
    windows_core::link!("fwpuclnt.dll" "system" "IPsecKeyManagerSetSecurityInfoByKey0" fn IPsecKeyManagerSetSecurityInfoByKey(enginehandle : super::HANDLE, reserved : *const core::ffi::c_void, securityinfo : super::SECURITY_INFORMATION, sidowner : *const super::SID, sidgroup : *const super::SID, dacl : *const super::ACL, sacl : *const super::ACL) -> super::DWORD);
    unsafe { IPsecKeyManagerSetSecurityInfoByKey(enginehandle, reserved.unwrap_or(core::mem::zeroed()) as _, securityinfo, sidowner.unwrap_or(core::mem::zeroed()) as _, sidgroup.unwrap_or(core::mem::zeroed()) as _, dacl.unwrap_or(core::mem::zeroed()) as _, sacl.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn IPsecKeyManagerUnregisterAndDelete(enginehandle: super::HANDLE, keymgmthandle: super::HANDLE) -> super::DWORD {
    windows_core::link!("fwpuclnt.dll" "system" "IPsecKeyManagerUnregisterAndDelete0" fn IPsecKeyManagerUnregisterAndDelete(enginehandle : super::HANDLE, keymgmthandle : super::HANDLE) -> super::DWORD);
    unsafe { IPsecKeyManagerUnregisterAndDelete(enginehandle, keymgmthandle) }
}
#[cfg(all(feature = "fwptypes", feature = "ipsectypes", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn IPsecKeyManagersGet(enginehandle: super::HANDLE, entries: *mut *mut *mut super::IPSEC_KEY_MANAGER0, numentries: *mut super::UINT32) -> super::DWORD {
    windows_core::link!("fwpuclnt.dll" "system" "IPsecKeyManagersGet0" fn IPsecKeyManagersGet(enginehandle : super::HANDLE, entries : *mut *mut *mut super::IPSEC_KEY_MANAGER0, numentries : *mut super::UINT32) -> super::DWORD);
    unsafe { IPsecKeyManagersGet(enginehandle, entries as _, numentries as _) }
}
#[cfg(all(feature = "fwptypes", feature = "ipsectypes", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn IPsecSaContextAddInbound(enginehandle: super::HANDLE, id: super::UINT64, inboundbundle: *const super::IPSEC_SA_BUNDLE1) -> super::DWORD {
    windows_core::link!("fwpuclnt.dll" "system" "IPsecSaContextAddInbound1" fn IPsecSaContextAddInbound(enginehandle : super::HANDLE, id : super::UINT64, inboundbundle : *const super::IPSEC_SA_BUNDLE1) -> super::DWORD);
    unsafe { IPsecSaContextAddInbound(enginehandle, id, inboundbundle) }
}
#[cfg(all(feature = "fwptypes", feature = "ipsectypes", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn IPsecSaContextAddOutbound(enginehandle: super::HANDLE, id: super::UINT64, outboundbundle: *const super::IPSEC_SA_BUNDLE1) -> super::DWORD {
    windows_core::link!("fwpuclnt.dll" "system" "IPsecSaContextAddOutbound1" fn IPsecSaContextAddOutbound(enginehandle : super::HANDLE, id : super::UINT64, outboundbundle : *const super::IPSEC_SA_BUNDLE1) -> super::DWORD);
    unsafe { IPsecSaContextAddOutbound(enginehandle, id, outboundbundle) }
}
#[cfg(all(feature = "fwptypes", feature = "ipsectypes", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn IPsecSaContextCreate(enginehandle: super::HANDLE, outboundtraffic: *const super::IPSEC_TRAFFIC1, virtualiftunnelinfo: Option<*const super::IPSEC_VIRTUAL_IF_TUNNEL_INFO0>, inboundfilterid: Option<*mut super::UINT64>, id: *mut super::UINT64) -> super::DWORD {
    windows_core::link!("fwpuclnt.dll" "system" "IPsecSaContextCreate1" fn IPsecSaContextCreate(enginehandle : super::HANDLE, outboundtraffic : *const super::IPSEC_TRAFFIC1, virtualiftunnelinfo : *const super::IPSEC_VIRTUAL_IF_TUNNEL_INFO0, inboundfilterid : *mut super::UINT64, id : *mut super::UINT64) -> super::DWORD);
    unsafe { IPsecSaContextCreate(enginehandle, outboundtraffic, virtualiftunnelinfo.unwrap_or(core::mem::zeroed()) as _, inboundfilterid.unwrap_or(core::mem::zeroed()) as _, id as _) }
}
#[cfg(all(feature = "fwptypes", feature = "ipsectypes", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn IPsecSaContextCreateEnumHandle(enginehandle: super::HANDLE, enumtemplate: Option<*const super::IPSEC_SA_CONTEXT_ENUM_TEMPLATE0>, enumhandle: *mut super::HANDLE) -> super::DWORD {
    windows_core::link!("fwpuclnt.dll" "system" "IPsecSaContextCreateEnumHandle0" fn IPsecSaContextCreateEnumHandle(enginehandle : super::HANDLE, enumtemplate : *const super::IPSEC_SA_CONTEXT_ENUM_TEMPLATE0, enumhandle : *mut super::HANDLE) -> super::DWORD);
    unsafe { IPsecSaContextCreateEnumHandle(enginehandle, enumtemplate.unwrap_or(core::mem::zeroed()) as _, enumhandle as _) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn IPsecSaContextDeleteById(enginehandle: super::HANDLE, id: super::UINT64) -> super::DWORD {
    windows_core::link!("fwpuclnt.dll" "system" "IPsecSaContextDeleteById0" fn IPsecSaContextDeleteById(enginehandle : super::HANDLE, id : super::UINT64) -> super::DWORD);
    unsafe { IPsecSaContextDeleteById(enginehandle, id) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn IPsecSaContextDestroyEnumHandle(enginehandle: super::HANDLE, enumhandle: super::HANDLE) -> super::DWORD {
    windows_core::link!("fwpuclnt.dll" "system" "IPsecSaContextDestroyEnumHandle0" fn IPsecSaContextDestroyEnumHandle(enginehandle : super::HANDLE, enumhandle : super::HANDLE) -> super::DWORD);
    unsafe { IPsecSaContextDestroyEnumHandle(enginehandle, enumhandle as _) }
}
#[cfg(all(feature = "fwpmtypes", feature = "fwptypes", feature = "ipsectypes", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn IPsecSaContextEnum(enginehandle: super::HANDLE, enumhandle: super::HANDLE, numentriesrequested: super::UINT32, entries: *mut *mut *mut super::IPSEC_SA_CONTEXT1, numentriesreturned: *mut super::UINT32) -> super::DWORD {
    windows_core::link!("fwpuclnt.dll" "system" "IPsecSaContextEnum1" fn IPsecSaContextEnum(enginehandle : super::HANDLE, enumhandle : super::HANDLE, numentriesrequested : super::UINT32, entries : *mut *mut *mut super::IPSEC_SA_CONTEXT1, numentriesreturned : *mut super::UINT32) -> super::DWORD);
    unsafe { IPsecSaContextEnum(enginehandle, enumhandle, numentriesrequested, entries as _, numentriesreturned as _) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn IPsecSaContextExpire(enginehandle: super::HANDLE, id: super::UINT64) -> super::DWORD {
    windows_core::link!("fwpuclnt.dll" "system" "IPsecSaContextExpire0" fn IPsecSaContextExpire(enginehandle : super::HANDLE, id : super::UINT64) -> super::DWORD);
    unsafe { IPsecSaContextExpire(enginehandle, id) }
}
#[cfg(all(feature = "fwpmtypes", feature = "fwptypes", feature = "ipsectypes", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn IPsecSaContextGetById(enginehandle: super::HANDLE, id: super::UINT64, sacontext: *mut *mut super::IPSEC_SA_CONTEXT1) -> super::DWORD {
    windows_core::link!("fwpuclnt.dll" "system" "IPsecSaContextGetById1" fn IPsecSaContextGetById(enginehandle : super::HANDLE, id : super::UINT64, sacontext : *mut *mut super::IPSEC_SA_CONTEXT1) -> super::DWORD);
    unsafe { IPsecSaContextGetById(enginehandle, id, sacontext as _) }
}
#[cfg(all(feature = "fwptypes", feature = "ipsectypes", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn IPsecSaContextGetSpi(enginehandle: super::HANDLE, id: super::UINT64, getspi: *const super::IPSEC_GETSPI1, inboundspi: *mut super::IPSEC_SA_SPI) -> super::DWORD {
    windows_core::link!("fwpuclnt.dll" "system" "IPsecSaContextGetSpi1" fn IPsecSaContextGetSpi(enginehandle : super::HANDLE, id : super::UINT64, getspi : *const super::IPSEC_GETSPI1, inboundspi : *mut super::IPSEC_SA_SPI) -> super::DWORD);
    unsafe { IPsecSaContextGetSpi(enginehandle, id, getspi, inboundspi as _) }
}
#[cfg(all(feature = "fwptypes", feature = "ipsectypes", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn IPsecSaContextSetSpi(enginehandle: super::HANDLE, id: super::UINT64, getspi: *const super::IPSEC_GETSPI1, inboundspi: super::IPSEC_SA_SPI) -> super::DWORD {
    windows_core::link!("fwpuclnt.dll" "system" "IPsecSaContextSetSpi0" fn IPsecSaContextSetSpi(enginehandle : super::HANDLE, id : super::UINT64, getspi : *const super::IPSEC_GETSPI1, inboundspi : super::IPSEC_SA_SPI) -> super::DWORD);
    unsafe { IPsecSaContextSetSpi(enginehandle, id, getspi, inboundspi) }
}
#[cfg(all(feature = "fwpmu", feature = "fwptypes", feature = "ipsectypes", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn IPsecSaContextSubscribe(enginehandle: super::HANDLE, subscription: *const super::IPSEC_SA_CONTEXT_SUBSCRIPTION0, callback: super::IPSEC_SA_CONTEXT_CALLBACK0, context: Option<*const core::ffi::c_void>, eventshandle: *mut super::HANDLE) -> super::DWORD {
    windows_core::link!("fwpuclnt.dll" "system" "IPsecSaContextSubscribe0" fn IPsecSaContextSubscribe(enginehandle : super::HANDLE, subscription : *const super::IPSEC_SA_CONTEXT_SUBSCRIPTION0, callback : super::IPSEC_SA_CONTEXT_CALLBACK0, context : *const core::ffi::c_void, eventshandle : *mut super::HANDLE) -> super::DWORD);
    unsafe { IPsecSaContextSubscribe(enginehandle, subscription, callback, context.unwrap_or(core::mem::zeroed()) as _, eventshandle as _) }
}
#[cfg(all(feature = "fwptypes", feature = "ipsectypes", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn IPsecSaContextSubscriptionsGet(enginehandle: super::HANDLE, entries: *mut *mut *mut super::IPSEC_SA_CONTEXT_SUBSCRIPTION0, numentries: *mut super::UINT32) -> super::DWORD {
    windows_core::link!("fwpuclnt.dll" "system" "IPsecSaContextSubscriptionsGet0" fn IPsecSaContextSubscriptionsGet(enginehandle : super::HANDLE, entries : *mut *mut *mut super::IPSEC_SA_CONTEXT_SUBSCRIPTION0, numentries : *mut super::UINT32) -> super::DWORD);
    unsafe { IPsecSaContextSubscriptionsGet(enginehandle, entries as _, numentries as _) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn IPsecSaContextUnsubscribe(enginehandle: super::HANDLE, eventshandle: super::HANDLE) -> super::DWORD {
    windows_core::link!("fwpuclnt.dll" "system" "IPsecSaContextUnsubscribe0" fn IPsecSaContextUnsubscribe(enginehandle : super::HANDLE, eventshandle : super::HANDLE) -> super::DWORD);
    unsafe { IPsecSaContextUnsubscribe(enginehandle, eventshandle as _) }
}
#[cfg(all(feature = "fwpmtypes", feature = "fwptypes", feature = "ipsectypes", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn IPsecSaContextUpdate(enginehandle: super::HANDLE, flags: super::UINT64, newvalues: *const super::IPSEC_SA_CONTEXT1) -> super::DWORD {
    windows_core::link!("fwpuclnt.dll" "system" "IPsecSaContextUpdate0" fn IPsecSaContextUpdate(enginehandle : super::HANDLE, flags : super::UINT64, newvalues : *const super::IPSEC_SA_CONTEXT1) -> super::DWORD);
    unsafe { IPsecSaContextUpdate(enginehandle, flags, newvalues) }
}
#[cfg(all(feature = "fwptypes", feature = "ipsectypes", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn IPsecSaCreateEnumHandle(enginehandle: super::HANDLE, enumtemplate: Option<*const super::IPSEC_SA_ENUM_TEMPLATE0>, enumhandle: *mut super::HANDLE) -> super::DWORD {
    windows_core::link!("fwpuclnt.dll" "system" "IPsecSaCreateEnumHandle0" fn IPsecSaCreateEnumHandle(enginehandle : super::HANDLE, enumtemplate : *const super::IPSEC_SA_ENUM_TEMPLATE0, enumhandle : *mut super::HANDLE) -> super::DWORD);
    unsafe { IPsecSaCreateEnumHandle(enginehandle, enumtemplate.unwrap_or(core::mem::zeroed()) as _, enumhandle as _) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn IPsecSaDbGetSecurityInfo(enginehandle: super::HANDLE, securityinfo: super::SECURITY_INFORMATION, sidowner: *mut super::PSID, sidgroup: *mut super::PSID, dacl: *mut super::PACL, sacl: *mut super::PACL, securitydescriptor: *mut super::PSECURITY_DESCRIPTOR) -> super::DWORD {
    windows_core::link!("fwpuclnt.dll" "system" "IPsecSaDbGetSecurityInfo0" fn IPsecSaDbGetSecurityInfo(enginehandle : super::HANDLE, securityinfo : super::SECURITY_INFORMATION, sidowner : *mut super::PSID, sidgroup : *mut super::PSID, dacl : *mut super::PACL, sacl : *mut super::PACL, securitydescriptor : *mut super::PSECURITY_DESCRIPTOR) -> super::DWORD);
    unsafe { IPsecSaDbGetSecurityInfo(enginehandle, securityinfo, sidowner as _, sidgroup as _, dacl as _, sacl as _, securitydescriptor as _) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn IPsecSaDbSetSecurityInfo(enginehandle: super::HANDLE, securityinfo: super::SECURITY_INFORMATION, sidowner: Option<*const super::SID>, sidgroup: Option<*const super::SID>, dacl: Option<*const super::ACL>, sacl: Option<*const super::ACL>) -> super::DWORD {
    windows_core::link!("fwpuclnt.dll" "system" "IPsecSaDbSetSecurityInfo0" fn IPsecSaDbSetSecurityInfo(enginehandle : super::HANDLE, securityinfo : super::SECURITY_INFORMATION, sidowner : *const super::SID, sidgroup : *const super::SID, dacl : *const super::ACL, sacl : *const super::ACL) -> super::DWORD);
    unsafe { IPsecSaDbSetSecurityInfo(enginehandle, securityinfo, sidowner.unwrap_or(core::mem::zeroed()) as _, sidgroup.unwrap_or(core::mem::zeroed()) as _, dacl.unwrap_or(core::mem::zeroed()) as _, sacl.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn IPsecSaDestroyEnumHandle(enginehandle: super::HANDLE, enumhandle: super::HANDLE) -> super::DWORD {
    windows_core::link!("fwpuclnt.dll" "system" "IPsecSaDestroyEnumHandle0" fn IPsecSaDestroyEnumHandle(enginehandle : super::HANDLE, enumhandle : super::HANDLE) -> super::DWORD);
    unsafe { IPsecSaDestroyEnumHandle(enginehandle, enumhandle as _) }
}
#[cfg(all(feature = "fwpmtypes", feature = "fwptypes", feature = "ipsectypes", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn IPsecSaEnum(enginehandle: super::HANDLE, enumhandle: super::HANDLE, numentriesrequested: super::UINT32, entries: *mut *mut *mut super::IPSEC_SA_DETAILS1, numentriesreturned: *mut super::UINT32) -> super::DWORD {
    windows_core::link!("fwpuclnt.dll" "system" "IPsecSaEnum1" fn IPsecSaEnum(enginehandle : super::HANDLE, enumhandle : super::HANDLE, numentriesrequested : super::UINT32, entries : *mut *mut *mut super::IPSEC_SA_DETAILS1, numentriesreturned : *mut super::UINT32) -> super::DWORD);
    unsafe { IPsecSaEnum(enginehandle, enumhandle, numentriesrequested, entries as _, numentriesreturned as _) }
}
#[cfg(all(feature = "iketypes", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn IkeextGetStatistics(enginehandle: super::HANDLE, ikeextstatistics: *mut super::IKEEXT_STATISTICS1) -> super::DWORD {
    windows_core::link!("fwpuclnt.dll" "system" "IkeextGetStatistics1" fn IkeextGetStatistics(enginehandle : super::HANDLE, ikeextstatistics : *mut super::IKEEXT_STATISTICS1) -> super::DWORD);
    unsafe { IkeextGetStatistics(enginehandle, ikeextstatistics as _) }
}
#[cfg(all(feature = "fwptypes", feature = "iketypes", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn IkeextSaCreateEnumHandle(enginehandle: super::HANDLE, enumtemplate: Option<*const super::IKEEXT_SA_ENUM_TEMPLATE0>, enumhandle: *mut super::HANDLE) -> super::DWORD {
    windows_core::link!("fwpuclnt.dll" "system" "IkeextSaCreateEnumHandle0" fn IkeextSaCreateEnumHandle(enginehandle : super::HANDLE, enumtemplate : *const super::IKEEXT_SA_ENUM_TEMPLATE0, enumhandle : *mut super::HANDLE) -> super::DWORD);
    unsafe { IkeextSaCreateEnumHandle(enginehandle, enumtemplate.unwrap_or(core::mem::zeroed()) as _, enumhandle as _) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn IkeextSaDbGetSecurityInfo(enginehandle: super::HANDLE, securityinfo: super::SECURITY_INFORMATION, sidowner: *mut super::PSID, sidgroup: *mut super::PSID, dacl: *mut super::PACL, sacl: *mut super::PACL, securitydescriptor: *mut super::PSECURITY_DESCRIPTOR) -> super::DWORD {
    windows_core::link!("fwpuclnt.dll" "system" "IkeextSaDbGetSecurityInfo0" fn IkeextSaDbGetSecurityInfo(enginehandle : super::HANDLE, securityinfo : super::SECURITY_INFORMATION, sidowner : *mut super::PSID, sidgroup : *mut super::PSID, dacl : *mut super::PACL, sacl : *mut super::PACL, securitydescriptor : *mut super::PSECURITY_DESCRIPTOR) -> super::DWORD);
    unsafe { IkeextSaDbGetSecurityInfo(enginehandle, securityinfo, sidowner as _, sidgroup as _, dacl as _, sacl as _, securitydescriptor as _) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn IkeextSaDbSetSecurityInfo(enginehandle: super::HANDLE, securityinfo: super::SECURITY_INFORMATION, sidowner: Option<*const super::SID>, sidgroup: Option<*const super::SID>, dacl: Option<*const super::ACL>, sacl: Option<*const super::ACL>) -> super::DWORD {
    windows_core::link!("fwpuclnt.dll" "system" "IkeextSaDbSetSecurityInfo0" fn IkeextSaDbSetSecurityInfo(enginehandle : super::HANDLE, securityinfo : super::SECURITY_INFORMATION, sidowner : *const super::SID, sidgroup : *const super::SID, dacl : *const super::ACL, sacl : *const super::ACL) -> super::DWORD);
    unsafe { IkeextSaDbSetSecurityInfo(enginehandle, securityinfo, sidowner.unwrap_or(core::mem::zeroed()) as _, sidgroup.unwrap_or(core::mem::zeroed()) as _, dacl.unwrap_or(core::mem::zeroed()) as _, sacl.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn IkeextSaDeleteById(enginehandle: super::HANDLE, id: super::UINT64) -> super::DWORD {
    windows_core::link!("fwpuclnt.dll" "system" "IkeextSaDeleteById0" fn IkeextSaDeleteById(enginehandle : super::HANDLE, id : super::UINT64) -> super::DWORD);
    unsafe { IkeextSaDeleteById(enginehandle, id) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn IkeextSaDestroyEnumHandle(enginehandle: super::HANDLE, enumhandle: super::HANDLE) -> super::DWORD {
    windows_core::link!("fwpuclnt.dll" "system" "IkeextSaDestroyEnumHandle0" fn IkeextSaDestroyEnumHandle(enginehandle : super::HANDLE, enumhandle : super::HANDLE) -> super::DWORD);
    unsafe { IkeextSaDestroyEnumHandle(enginehandle, enumhandle as _) }
}
#[cfg(all(feature = "fwptypes", feature = "iketypes", feature = "ipsectypes", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn IkeextSaEnum(enginehandle: super::HANDLE, enumhandle: super::HANDLE, numentriesrequested: super::UINT32, entries: *mut *mut *mut super::IKEEXT_SA_DETAILS2, numentriesreturned: *mut super::UINT32) -> super::DWORD {
    windows_core::link!("fwpuclnt.dll" "system" "IkeextSaEnum2" fn IkeextSaEnum(enginehandle : super::HANDLE, enumhandle : super::HANDLE, numentriesrequested : super::UINT32, entries : *mut *mut *mut super::IKEEXT_SA_DETAILS2, numentriesreturned : *mut super::UINT32) -> super::DWORD);
    unsafe { IkeextSaEnum(enginehandle, enumhandle, numentriesrequested, entries as _, numentriesreturned as _) }
}
#[cfg(all(feature = "fwptypes", feature = "iketypes", feature = "ipsectypes", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn IkeextSaGetById(enginehandle: super::HANDLE, id: super::UINT64, salookupcontext: Option<*const windows_core::GUID>, sa: *mut *mut super::IKEEXT_SA_DETAILS2) -> super::DWORD {
    windows_core::link!("fwpuclnt.dll" "system" "IkeextSaGetById2" fn IkeextSaGetById(enginehandle : super::HANDLE, id : super::UINT64, salookupcontext : *const windows_core::GUID, sa : *mut *mut super::IKEEXT_SA_DETAILS2) -> super::DWORD);
    unsafe { IkeextSaGetById(enginehandle, id, salookupcontext.unwrap_or(core::mem::zeroed()) as _, sa as _) }
}
