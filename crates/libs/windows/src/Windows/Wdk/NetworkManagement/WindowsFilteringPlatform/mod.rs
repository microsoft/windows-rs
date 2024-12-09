#[cfg(feature = "Win32_NetworkManagement_WindowsFilteringPlatform")]
#[inline]
pub unsafe fn FwpmBfeStateGet0() -> super::super::super::Win32::NetworkManagement::WindowsFilteringPlatform::FWPM_SERVICE_STATE {
    windows_targets::link!("fwpkclnt.sys" "system" fn FwpmBfeStateGet0() -> super::super::super::Win32::NetworkManagement::WindowsFilteringPlatform:: FWPM_SERVICE_STATE);
    FwpmBfeStateGet0()
}
#[cfg(feature = "Win32_NetworkManagement_WindowsFilteringPlatform")]
#[inline]
pub unsafe fn FwpmBfeStateSubscribeChanges0(deviceobject: *mut core::ffi::c_void, callback: FWPM_SERVICE_STATE_CHANGE_CALLBACK0, context: Option<*const core::ffi::c_void>, changehandle: *mut super::super::super::Win32::Foundation::HANDLE) -> super::super::super::Win32::Foundation::NTSTATUS {
    windows_targets::link!("fwpkclnt.sys" "system" fn FwpmBfeStateSubscribeChanges0(deviceobject : *mut core::ffi::c_void, callback : FWPM_SERVICE_STATE_CHANGE_CALLBACK0, context : *const core::ffi::c_void, changehandle : *mut super::super::super::Win32::Foundation:: HANDLE) -> super::super::super::Win32::Foundation:: NTSTATUS);
    FwpmBfeStateSubscribeChanges0(core::mem::transmute(deviceobject), core::mem::transmute(callback), core::mem::transmute(context.unwrap_or(core::mem::zeroed())), core::mem::transmute(changehandle))
}
#[inline]
pub unsafe fn FwpmBfeStateUnsubscribeChanges0(changehandle: super::super::super::Win32::Foundation::HANDLE) -> super::super::super::Win32::Foundation::NTSTATUS {
    windows_targets::link!("fwpkclnt.sys" "system" fn FwpmBfeStateUnsubscribeChanges0(changehandle : super::super::super::Win32::Foundation:: HANDLE) -> super::super::super::Win32::Foundation:: NTSTATUS);
    FwpmBfeStateUnsubscribeChanges0(core::mem::transmute(changehandle))
}
#[cfg(all(feature = "Win32_NetworkManagement_WindowsFilteringPlatform", feature = "Win32_Security"))]
#[inline]
pub unsafe fn FwpmCalloutAdd0(enginehandle: super::super::super::Win32::Foundation::HANDLE, callout: *const super::super::super::Win32::NetworkManagement::WindowsFilteringPlatform::FWPM_CALLOUT0, sd: Option<super::super::super::Win32::Security::PSECURITY_DESCRIPTOR>, id: Option<*mut u32>) -> super::super::super::Win32::Foundation::NTSTATUS {
    windows_targets::link!("fwpuclnt.dll" "system" fn FwpmCalloutAdd0(enginehandle : super::super::super::Win32::Foundation:: HANDLE, callout : *const super::super::super::Win32::NetworkManagement::WindowsFilteringPlatform:: FWPM_CALLOUT0, sd : super::super::super::Win32::Security:: PSECURITY_DESCRIPTOR, id : *mut u32) -> super::super::super::Win32::Foundation:: NTSTATUS);
    FwpmCalloutAdd0(core::mem::transmute(enginehandle), core::mem::transmute(callout), core::mem::transmute(sd.unwrap_or(core::mem::zeroed())), core::mem::transmute(id.unwrap_or(core::mem::zeroed())))
}
#[cfg(feature = "Win32_NetworkManagement_WindowsFilteringPlatform")]
#[inline]
pub unsafe fn FwpmCalloutCreateEnumHandle0(enginehandle: super::super::super::Win32::Foundation::HANDLE, enumtemplate: Option<*const super::super::super::Win32::NetworkManagement::WindowsFilteringPlatform::FWPM_CALLOUT_ENUM_TEMPLATE0>, enumhandle: *mut super::super::super::Win32::Foundation::HANDLE) -> super::super::super::Win32::Foundation::NTSTATUS {
    windows_targets::link!("fwpuclnt.dll" "system" fn FwpmCalloutCreateEnumHandle0(enginehandle : super::super::super::Win32::Foundation:: HANDLE, enumtemplate : *const super::super::super::Win32::NetworkManagement::WindowsFilteringPlatform:: FWPM_CALLOUT_ENUM_TEMPLATE0, enumhandle : *mut super::super::super::Win32::Foundation:: HANDLE) -> super::super::super::Win32::Foundation:: NTSTATUS);
    FwpmCalloutCreateEnumHandle0(core::mem::transmute(enginehandle), core::mem::transmute(enumtemplate.unwrap_or(core::mem::zeroed())), core::mem::transmute(enumhandle))
}
#[inline]
pub unsafe fn FwpmCalloutDeleteById0(enginehandle: super::super::super::Win32::Foundation::HANDLE, id: u32) -> super::super::super::Win32::Foundation::NTSTATUS {
    windows_targets::link!("fwpuclnt.dll" "system" fn FwpmCalloutDeleteById0(enginehandle : super::super::super::Win32::Foundation:: HANDLE, id : u32) -> super::super::super::Win32::Foundation:: NTSTATUS);
    FwpmCalloutDeleteById0(core::mem::transmute(enginehandle), core::mem::transmute(id))
}
#[inline]
pub unsafe fn FwpmCalloutDeleteByKey0(enginehandle: super::super::super::Win32::Foundation::HANDLE, key: *const windows_core::GUID) -> super::super::super::Win32::Foundation::NTSTATUS {
    windows_targets::link!("fwpuclnt.dll" "system" fn FwpmCalloutDeleteByKey0(enginehandle : super::super::super::Win32::Foundation:: HANDLE, key : *const windows_core::GUID) -> super::super::super::Win32::Foundation:: NTSTATUS);
    FwpmCalloutDeleteByKey0(core::mem::transmute(enginehandle), core::mem::transmute(key))
}
#[inline]
pub unsafe fn FwpmCalloutDestroyEnumHandle0(enginehandle: super::super::super::Win32::Foundation::HANDLE, enumhandle: super::super::super::Win32::Foundation::HANDLE) -> super::super::super::Win32::Foundation::NTSTATUS {
    windows_targets::link!("fwpuclnt.dll" "system" fn FwpmCalloutDestroyEnumHandle0(enginehandle : super::super::super::Win32::Foundation:: HANDLE, enumhandle : super::super::super::Win32::Foundation:: HANDLE) -> super::super::super::Win32::Foundation:: NTSTATUS);
    FwpmCalloutDestroyEnumHandle0(core::mem::transmute(enginehandle), core::mem::transmute(enumhandle))
}
#[cfg(feature = "Win32_NetworkManagement_WindowsFilteringPlatform")]
#[inline]
pub unsafe fn FwpmCalloutEnum0(enginehandle: super::super::super::Win32::Foundation::HANDLE, enumhandle: super::super::super::Win32::Foundation::HANDLE, numentriesrequested: u32, entries: *mut *mut *mut super::super::super::Win32::NetworkManagement::WindowsFilteringPlatform::FWPM_CALLOUT0, numentriesreturned: *mut u32) -> super::super::super::Win32::Foundation::NTSTATUS {
    windows_targets::link!("fwpuclnt.dll" "system" fn FwpmCalloutEnum0(enginehandle : super::super::super::Win32::Foundation:: HANDLE, enumhandle : super::super::super::Win32::Foundation:: HANDLE, numentriesrequested : u32, entries : *mut *mut *mut super::super::super::Win32::NetworkManagement::WindowsFilteringPlatform:: FWPM_CALLOUT0, numentriesreturned : *mut u32) -> super::super::super::Win32::Foundation:: NTSTATUS);
    FwpmCalloutEnum0(core::mem::transmute(enginehandle), core::mem::transmute(enumhandle), core::mem::transmute(numentriesrequested), core::mem::transmute(entries), core::mem::transmute(numentriesreturned))
}
#[cfg(feature = "Win32_NetworkManagement_WindowsFilteringPlatform")]
#[inline]
pub unsafe fn FwpmCalloutGetById0(enginehandle: super::super::super::Win32::Foundation::HANDLE, id: u32, callout: *mut *mut super::super::super::Win32::NetworkManagement::WindowsFilteringPlatform::FWPM_CALLOUT0) -> super::super::super::Win32::Foundation::NTSTATUS {
    windows_targets::link!("fwpuclnt.dll" "system" fn FwpmCalloutGetById0(enginehandle : super::super::super::Win32::Foundation:: HANDLE, id : u32, callout : *mut *mut super::super::super::Win32::NetworkManagement::WindowsFilteringPlatform:: FWPM_CALLOUT0) -> super::super::super::Win32::Foundation:: NTSTATUS);
    FwpmCalloutGetById0(core::mem::transmute(enginehandle), core::mem::transmute(id), core::mem::transmute(callout))
}
#[cfg(feature = "Win32_NetworkManagement_WindowsFilteringPlatform")]
#[inline]
pub unsafe fn FwpmCalloutGetByKey0(enginehandle: super::super::super::Win32::Foundation::HANDLE, key: *const windows_core::GUID, callout: *mut *mut super::super::super::Win32::NetworkManagement::WindowsFilteringPlatform::FWPM_CALLOUT0) -> super::super::super::Win32::Foundation::NTSTATUS {
    windows_targets::link!("fwpuclnt.dll" "system" fn FwpmCalloutGetByKey0(enginehandle : super::super::super::Win32::Foundation:: HANDLE, key : *const windows_core::GUID, callout : *mut *mut super::super::super::Win32::NetworkManagement::WindowsFilteringPlatform:: FWPM_CALLOUT0) -> super::super::super::Win32::Foundation:: NTSTATUS);
    FwpmCalloutGetByKey0(core::mem::transmute(enginehandle), core::mem::transmute(key), core::mem::transmute(callout))
}
#[cfg(feature = "Win32_Security")]
#[inline]
pub unsafe fn FwpmCalloutGetSecurityInfoByKey0(enginehandle: super::super::super::Win32::Foundation::HANDLE, key: Option<*const windows_core::GUID>, securityinfo: u32, sidowner: *mut super::super::super::Win32::Security::PSID, sidgroup: *mut super::super::super::Win32::Security::PSID, dacl: *mut *mut super::super::super::Win32::Security::ACL, sacl: *mut *mut super::super::super::Win32::Security::ACL, securitydescriptor: *mut super::super::super::Win32::Security::PSECURITY_DESCRIPTOR) -> super::super::super::Win32::Foundation::NTSTATUS {
    windows_targets::link!("fwpuclnt.dll" "system" fn FwpmCalloutGetSecurityInfoByKey0(enginehandle : super::super::super::Win32::Foundation:: HANDLE, key : *const windows_core::GUID, securityinfo : u32, sidowner : *mut super::super::super::Win32::Security:: PSID, sidgroup : *mut super::super::super::Win32::Security:: PSID, dacl : *mut *mut super::super::super::Win32::Security:: ACL, sacl : *mut *mut super::super::super::Win32::Security:: ACL, securitydescriptor : *mut super::super::super::Win32::Security:: PSECURITY_DESCRIPTOR) -> super::super::super::Win32::Foundation:: NTSTATUS);
    FwpmCalloutGetSecurityInfoByKey0(core::mem::transmute(enginehandle), core::mem::transmute(key.unwrap_or(core::mem::zeroed())), core::mem::transmute(securityinfo), core::mem::transmute(sidowner), core::mem::transmute(sidgroup), core::mem::transmute(dacl), core::mem::transmute(sacl), core::mem::transmute(securitydescriptor))
}
#[cfg(feature = "Win32_Security")]
#[inline]
pub unsafe fn FwpmCalloutSetSecurityInfoByKey0(enginehandle: super::super::super::Win32::Foundation::HANDLE, key: Option<*const windows_core::GUID>, securityinfo: u32, sidowner: Option<*const super::super::super::Win32::Security::SID>, sidgroup: Option<*const super::super::super::Win32::Security::SID>, dacl: Option<*const super::super::super::Win32::Security::ACL>, sacl: Option<*const super::super::super::Win32::Security::ACL>) -> super::super::super::Win32::Foundation::NTSTATUS {
    windows_targets::link!("fwpuclnt.dll" "system" fn FwpmCalloutSetSecurityInfoByKey0(enginehandle : super::super::super::Win32::Foundation:: HANDLE, key : *const windows_core::GUID, securityinfo : u32, sidowner : *const super::super::super::Win32::Security:: SID, sidgroup : *const super::super::super::Win32::Security:: SID, dacl : *const super::super::super::Win32::Security:: ACL, sacl : *const super::super::super::Win32::Security:: ACL) -> super::super::super::Win32::Foundation:: NTSTATUS);
    FwpmCalloutSetSecurityInfoByKey0(core::mem::transmute(enginehandle), core::mem::transmute(key.unwrap_or(core::mem::zeroed())), core::mem::transmute(securityinfo), core::mem::transmute(sidowner.unwrap_or(core::mem::zeroed())), core::mem::transmute(sidgroup.unwrap_or(core::mem::zeroed())), core::mem::transmute(dacl.unwrap_or(core::mem::zeroed())), core::mem::transmute(sacl.unwrap_or(core::mem::zeroed())))
}
#[cfg(feature = "Win32_NetworkManagement_WindowsFilteringPlatform")]
#[inline]
pub unsafe fn FwpmConnectionCreateEnumHandle0(enginehandle: super::super::super::Win32::Foundation::HANDLE, enumtemplate: Option<*const super::super::super::Win32::NetworkManagement::WindowsFilteringPlatform::FWPM_CONNECTION_ENUM_TEMPLATE0>, enumhandle: *mut super::super::super::Win32::Foundation::HANDLE) -> super::super::super::Win32::Foundation::NTSTATUS {
    windows_targets::link!("fwpuclnt.dll" "system" fn FwpmConnectionCreateEnumHandle0(enginehandle : super::super::super::Win32::Foundation:: HANDLE, enumtemplate : *const super::super::super::Win32::NetworkManagement::WindowsFilteringPlatform:: FWPM_CONNECTION_ENUM_TEMPLATE0, enumhandle : *mut super::super::super::Win32::Foundation:: HANDLE) -> super::super::super::Win32::Foundation:: NTSTATUS);
    FwpmConnectionCreateEnumHandle0(core::mem::transmute(enginehandle), core::mem::transmute(enumtemplate.unwrap_or(core::mem::zeroed())), core::mem::transmute(enumhandle))
}
#[inline]
pub unsafe fn FwpmConnectionDestroyEnumHandle0(enginehandle: super::super::super::Win32::Foundation::HANDLE, enumhandle: super::super::super::Win32::Foundation::HANDLE) -> super::super::super::Win32::Foundation::NTSTATUS {
    windows_targets::link!("fwpuclnt.dll" "system" fn FwpmConnectionDestroyEnumHandle0(enginehandle : super::super::super::Win32::Foundation:: HANDLE, enumhandle : super::super::super::Win32::Foundation:: HANDLE) -> super::super::super::Win32::Foundation:: NTSTATUS);
    FwpmConnectionDestroyEnumHandle0(core::mem::transmute(enginehandle), core::mem::transmute(enumhandle))
}
#[cfg(feature = "Win32_NetworkManagement_WindowsFilteringPlatform")]
#[inline]
pub unsafe fn FwpmConnectionEnum0(enginehandle: super::super::super::Win32::Foundation::HANDLE, enumhandle: super::super::super::Win32::Foundation::HANDLE, numentriesrequested: u32, entries: *mut *mut *mut super::super::super::Win32::NetworkManagement::WindowsFilteringPlatform::FWPM_CONNECTION0, numentriesreturned: *mut u32) -> super::super::super::Win32::Foundation::NTSTATUS {
    windows_targets::link!("fwpuclnt.dll" "system" fn FwpmConnectionEnum0(enginehandle : super::super::super::Win32::Foundation:: HANDLE, enumhandle : super::super::super::Win32::Foundation:: HANDLE, numentriesrequested : u32, entries : *mut *mut *mut super::super::super::Win32::NetworkManagement::WindowsFilteringPlatform:: FWPM_CONNECTION0, numentriesreturned : *mut u32) -> super::super::super::Win32::Foundation:: NTSTATUS);
    FwpmConnectionEnum0(core::mem::transmute(enginehandle), core::mem::transmute(enumhandle), core::mem::transmute(numentriesrequested), core::mem::transmute(entries), core::mem::transmute(numentriesreturned))
}
#[cfg(feature = "Win32_NetworkManagement_WindowsFilteringPlatform")]
#[inline]
pub unsafe fn FwpmConnectionGetById0(enginehandle: super::super::super::Win32::Foundation::HANDLE, id: u64, connection: *mut *mut super::super::super::Win32::NetworkManagement::WindowsFilteringPlatform::FWPM_CONNECTION0) -> super::super::super::Win32::Foundation::NTSTATUS {
    windows_targets::link!("fwpuclnt.dll" "system" fn FwpmConnectionGetById0(enginehandle : super::super::super::Win32::Foundation:: HANDLE, id : u64, connection : *mut *mut super::super::super::Win32::NetworkManagement::WindowsFilteringPlatform:: FWPM_CONNECTION0) -> super::super::super::Win32::Foundation:: NTSTATUS);
    FwpmConnectionGetById0(core::mem::transmute(enginehandle), core::mem::transmute(id), core::mem::transmute(connection))
}
#[cfg(feature = "Win32_Security")]
#[inline]
pub unsafe fn FwpmConnectionGetSecurityInfo0(enginehandle: super::super::super::Win32::Foundation::HANDLE, securityinfo: u32, sidowner: *mut super::super::super::Win32::Security::PSID, sidgroup: *mut super::super::super::Win32::Security::PSID, dacl: *mut *mut super::super::super::Win32::Security::ACL, sacl: *mut *mut super::super::super::Win32::Security::ACL, securitydescriptor: *mut super::super::super::Win32::Security::PSECURITY_DESCRIPTOR) -> super::super::super::Win32::Foundation::NTSTATUS {
    windows_targets::link!("fwpuclnt.dll" "system" fn FwpmConnectionGetSecurityInfo0(enginehandle : super::super::super::Win32::Foundation:: HANDLE, securityinfo : u32, sidowner : *mut super::super::super::Win32::Security:: PSID, sidgroup : *mut super::super::super::Win32::Security:: PSID, dacl : *mut *mut super::super::super::Win32::Security:: ACL, sacl : *mut *mut super::super::super::Win32::Security:: ACL, securitydescriptor : *mut super::super::super::Win32::Security:: PSECURITY_DESCRIPTOR) -> super::super::super::Win32::Foundation:: NTSTATUS);
    FwpmConnectionGetSecurityInfo0(core::mem::transmute(enginehandle), core::mem::transmute(securityinfo), core::mem::transmute(sidowner), core::mem::transmute(sidgroup), core::mem::transmute(dacl), core::mem::transmute(sacl), core::mem::transmute(securitydescriptor))
}
#[cfg(feature = "Win32_Security")]
#[inline]
pub unsafe fn FwpmConnectionSetSecurityInfo0(enginehandle: super::super::super::Win32::Foundation::HANDLE, securityinfo: u32, sidowner: Option<*const super::super::super::Win32::Security::SID>, sidgroup: Option<*const super::super::super::Win32::Security::SID>, dacl: Option<*const super::super::super::Win32::Security::ACL>, sacl: Option<*const super::super::super::Win32::Security::ACL>) -> super::super::super::Win32::Foundation::NTSTATUS {
    windows_targets::link!("fwpuclnt.dll" "system" fn FwpmConnectionSetSecurityInfo0(enginehandle : super::super::super::Win32::Foundation:: HANDLE, securityinfo : u32, sidowner : *const super::super::super::Win32::Security:: SID, sidgroup : *const super::super::super::Win32::Security:: SID, dacl : *const super::super::super::Win32::Security:: ACL, sacl : *const super::super::super::Win32::Security:: ACL) -> super::super::super::Win32::Foundation:: NTSTATUS);
    FwpmConnectionSetSecurityInfo0(core::mem::transmute(enginehandle), core::mem::transmute(securityinfo), core::mem::transmute(sidowner.unwrap_or(core::mem::zeroed())), core::mem::transmute(sidgroup.unwrap_or(core::mem::zeroed())), core::mem::transmute(dacl.unwrap_or(core::mem::zeroed())), core::mem::transmute(sacl.unwrap_or(core::mem::zeroed())))
}
#[inline]
pub unsafe fn FwpmEngineClose0(enginehandle: super::super::super::Win32::Foundation::HANDLE) -> super::super::super::Win32::Foundation::NTSTATUS {
    windows_targets::link!("fwpuclnt.dll" "system" fn FwpmEngineClose0(enginehandle : super::super::super::Win32::Foundation:: HANDLE) -> super::super::super::Win32::Foundation:: NTSTATUS);
    FwpmEngineClose0(core::mem::transmute(enginehandle))
}
#[cfg(all(feature = "Win32_NetworkManagement_WindowsFilteringPlatform", feature = "Win32_Security"))]
#[inline]
pub unsafe fn FwpmEngineGetOption0(enginehandle: super::super::super::Win32::Foundation::HANDLE, option: super::super::super::Win32::NetworkManagement::WindowsFilteringPlatform::FWPM_ENGINE_OPTION, value: *mut *mut super::super::super::Win32::NetworkManagement::WindowsFilteringPlatform::FWP_VALUE0) -> super::super::super::Win32::Foundation::NTSTATUS {
    windows_targets::link!("fwpuclnt.dll" "system" fn FwpmEngineGetOption0(enginehandle : super::super::super::Win32::Foundation:: HANDLE, option : super::super::super::Win32::NetworkManagement::WindowsFilteringPlatform:: FWPM_ENGINE_OPTION, value : *mut *mut super::super::super::Win32::NetworkManagement::WindowsFilteringPlatform:: FWP_VALUE0) -> super::super::super::Win32::Foundation:: NTSTATUS);
    FwpmEngineGetOption0(core::mem::transmute(enginehandle), core::mem::transmute(option), core::mem::transmute(value))
}
#[cfg(feature = "Win32_Security")]
#[inline]
pub unsafe fn FwpmEngineGetSecurityInfo0(enginehandle: super::super::super::Win32::Foundation::HANDLE, securityinfo: u32, sidowner: *mut super::super::super::Win32::Security::PSID, sidgroup: *mut super::super::super::Win32::Security::PSID, dacl: *mut *mut super::super::super::Win32::Security::ACL, sacl: *mut *mut super::super::super::Win32::Security::ACL, securitydescriptor: *mut super::super::super::Win32::Security::PSECURITY_DESCRIPTOR) -> super::super::super::Win32::Foundation::NTSTATUS {
    windows_targets::link!("fwpuclnt.dll" "system" fn FwpmEngineGetSecurityInfo0(enginehandle : super::super::super::Win32::Foundation:: HANDLE, securityinfo : u32, sidowner : *mut super::super::super::Win32::Security:: PSID, sidgroup : *mut super::super::super::Win32::Security:: PSID, dacl : *mut *mut super::super::super::Win32::Security:: ACL, sacl : *mut *mut super::super::super::Win32::Security:: ACL, securitydescriptor : *mut super::super::super::Win32::Security:: PSECURITY_DESCRIPTOR) -> super::super::super::Win32::Foundation:: NTSTATUS);
    FwpmEngineGetSecurityInfo0(core::mem::transmute(enginehandle), core::mem::transmute(securityinfo), core::mem::transmute(sidowner), core::mem::transmute(sidgroup), core::mem::transmute(dacl), core::mem::transmute(sacl), core::mem::transmute(securitydescriptor))
}
#[cfg(all(feature = "Win32_NetworkManagement_WindowsFilteringPlatform", feature = "Win32_Security", feature = "Win32_System_Rpc"))]
#[inline]
pub unsafe fn FwpmEngineOpen0<P0>(servername: P0, authnservice: u32, authidentity: Option<*const super::super::super::Win32::System::Rpc::SEC_WINNT_AUTH_IDENTITY_W>, session: Option<*const super::super::super::Win32::NetworkManagement::WindowsFilteringPlatform::FWPM_SESSION0>, enginehandle: *mut super::super::super::Win32::Foundation::HANDLE) -> super::super::super::Win32::Foundation::NTSTATUS
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("fwpuclnt.dll" "system" fn FwpmEngineOpen0(servername : windows_core::PCWSTR, authnservice : u32, authidentity : *const super::super::super::Win32::System::Rpc:: SEC_WINNT_AUTH_IDENTITY_W, session : *const super::super::super::Win32::NetworkManagement::WindowsFilteringPlatform:: FWPM_SESSION0, enginehandle : *mut super::super::super::Win32::Foundation:: HANDLE) -> super::super::super::Win32::Foundation:: NTSTATUS);
    FwpmEngineOpen0(servername.param().abi(), core::mem::transmute(authnservice), core::mem::transmute(authidentity.unwrap_or(core::mem::zeroed())), core::mem::transmute(session.unwrap_or(core::mem::zeroed())), core::mem::transmute(enginehandle))
}
#[cfg(all(feature = "Win32_NetworkManagement_WindowsFilteringPlatform", feature = "Win32_Security"))]
#[inline]
pub unsafe fn FwpmEngineSetOption0(enginehandle: super::super::super::Win32::Foundation::HANDLE, option: super::super::super::Win32::NetworkManagement::WindowsFilteringPlatform::FWPM_ENGINE_OPTION, newvalue: *const super::super::super::Win32::NetworkManagement::WindowsFilteringPlatform::FWP_VALUE0) -> super::super::super::Win32::Foundation::NTSTATUS {
    windows_targets::link!("fwpuclnt.dll" "system" fn FwpmEngineSetOption0(enginehandle : super::super::super::Win32::Foundation:: HANDLE, option : super::super::super::Win32::NetworkManagement::WindowsFilteringPlatform:: FWPM_ENGINE_OPTION, newvalue : *const super::super::super::Win32::NetworkManagement::WindowsFilteringPlatform:: FWP_VALUE0) -> super::super::super::Win32::Foundation:: NTSTATUS);
    FwpmEngineSetOption0(core::mem::transmute(enginehandle), core::mem::transmute(option), core::mem::transmute(newvalue))
}
#[cfg(feature = "Win32_Security")]
#[inline]
pub unsafe fn FwpmEngineSetSecurityInfo0(enginehandle: super::super::super::Win32::Foundation::HANDLE, securityinfo: u32, sidowner: Option<*const super::super::super::Win32::Security::SID>, sidgroup: Option<*const super::super::super::Win32::Security::SID>, dacl: Option<*const super::super::super::Win32::Security::ACL>, sacl: Option<*const super::super::super::Win32::Security::ACL>) -> super::super::super::Win32::Foundation::NTSTATUS {
    windows_targets::link!("fwpuclnt.dll" "system" fn FwpmEngineSetSecurityInfo0(enginehandle : super::super::super::Win32::Foundation:: HANDLE, securityinfo : u32, sidowner : *const super::super::super::Win32::Security:: SID, sidgroup : *const super::super::super::Win32::Security:: SID, dacl : *const super::super::super::Win32::Security:: ACL, sacl : *const super::super::super::Win32::Security:: ACL) -> super::super::super::Win32::Foundation:: NTSTATUS);
    FwpmEngineSetSecurityInfo0(core::mem::transmute(enginehandle), core::mem::transmute(securityinfo), core::mem::transmute(sidowner.unwrap_or(core::mem::zeroed())), core::mem::transmute(sidgroup.unwrap_or(core::mem::zeroed())), core::mem::transmute(dacl.unwrap_or(core::mem::zeroed())), core::mem::transmute(sacl.unwrap_or(core::mem::zeroed())))
}
#[cfg(all(feature = "Win32_NetworkManagement_WindowsFilteringPlatform", feature = "Win32_Security"))]
#[inline]
pub unsafe fn FwpmFilterAdd0(enginehandle: super::super::super::Win32::Foundation::HANDLE, filter: *const super::super::super::Win32::NetworkManagement::WindowsFilteringPlatform::FWPM_FILTER0, sd: Option<super::super::super::Win32::Security::PSECURITY_DESCRIPTOR>, id: Option<*mut u64>) -> super::super::super::Win32::Foundation::NTSTATUS {
    windows_targets::link!("fwpuclnt.dll" "system" fn FwpmFilterAdd0(enginehandle : super::super::super::Win32::Foundation:: HANDLE, filter : *const super::super::super::Win32::NetworkManagement::WindowsFilteringPlatform:: FWPM_FILTER0, sd : super::super::super::Win32::Security:: PSECURITY_DESCRIPTOR, id : *mut u64) -> super::super::super::Win32::Foundation:: NTSTATUS);
    FwpmFilterAdd0(core::mem::transmute(enginehandle), core::mem::transmute(filter), core::mem::transmute(sd.unwrap_or(core::mem::zeroed())), core::mem::transmute(id.unwrap_or(core::mem::zeroed())))
}
#[cfg(all(feature = "Win32_NetworkManagement_WindowsFilteringPlatform", feature = "Win32_Security"))]
#[inline]
pub unsafe fn FwpmFilterCreateEnumHandle0(enginehandle: super::super::super::Win32::Foundation::HANDLE, enumtemplate: Option<*const super::super::super::Win32::NetworkManagement::WindowsFilteringPlatform::FWPM_FILTER_ENUM_TEMPLATE0>, enumhandle: *mut super::super::super::Win32::Foundation::HANDLE) -> super::super::super::Win32::Foundation::NTSTATUS {
    windows_targets::link!("fwpuclnt.dll" "system" fn FwpmFilterCreateEnumHandle0(enginehandle : super::super::super::Win32::Foundation:: HANDLE, enumtemplate : *const super::super::super::Win32::NetworkManagement::WindowsFilteringPlatform:: FWPM_FILTER_ENUM_TEMPLATE0, enumhandle : *mut super::super::super::Win32::Foundation:: HANDLE) -> super::super::super::Win32::Foundation:: NTSTATUS);
    FwpmFilterCreateEnumHandle0(core::mem::transmute(enginehandle), core::mem::transmute(enumtemplate.unwrap_or(core::mem::zeroed())), core::mem::transmute(enumhandle))
}
#[inline]
pub unsafe fn FwpmFilterDeleteById0(enginehandle: super::super::super::Win32::Foundation::HANDLE, id: u64) -> super::super::super::Win32::Foundation::NTSTATUS {
    windows_targets::link!("fwpuclnt.dll" "system" fn FwpmFilterDeleteById0(enginehandle : super::super::super::Win32::Foundation:: HANDLE, id : u64) -> super::super::super::Win32::Foundation:: NTSTATUS);
    FwpmFilterDeleteById0(core::mem::transmute(enginehandle), core::mem::transmute(id))
}
#[inline]
pub unsafe fn FwpmFilterDeleteByKey0(enginehandle: super::super::super::Win32::Foundation::HANDLE, key: *const windows_core::GUID) -> super::super::super::Win32::Foundation::NTSTATUS {
    windows_targets::link!("fwpuclnt.dll" "system" fn FwpmFilterDeleteByKey0(enginehandle : super::super::super::Win32::Foundation:: HANDLE, key : *const windows_core::GUID) -> super::super::super::Win32::Foundation:: NTSTATUS);
    FwpmFilterDeleteByKey0(core::mem::transmute(enginehandle), core::mem::transmute(key))
}
#[inline]
pub unsafe fn FwpmFilterDestroyEnumHandle0(enginehandle: super::super::super::Win32::Foundation::HANDLE, enumhandle: super::super::super::Win32::Foundation::HANDLE) -> super::super::super::Win32::Foundation::NTSTATUS {
    windows_targets::link!("fwpuclnt.dll" "system" fn FwpmFilterDestroyEnumHandle0(enginehandle : super::super::super::Win32::Foundation:: HANDLE, enumhandle : super::super::super::Win32::Foundation:: HANDLE) -> super::super::super::Win32::Foundation:: NTSTATUS);
    FwpmFilterDestroyEnumHandle0(core::mem::transmute(enginehandle), core::mem::transmute(enumhandle))
}
#[cfg(all(feature = "Win32_NetworkManagement_WindowsFilteringPlatform", feature = "Win32_Security"))]
#[inline]
pub unsafe fn FwpmFilterEnum0(enginehandle: super::super::super::Win32::Foundation::HANDLE, enumhandle: super::super::super::Win32::Foundation::HANDLE, numentriesrequested: u32, entries: *mut *mut *mut super::super::super::Win32::NetworkManagement::WindowsFilteringPlatform::FWPM_FILTER0, numentriesreturned: *mut u32) -> super::super::super::Win32::Foundation::NTSTATUS {
    windows_targets::link!("fwpuclnt.dll" "system" fn FwpmFilterEnum0(enginehandle : super::super::super::Win32::Foundation:: HANDLE, enumhandle : super::super::super::Win32::Foundation:: HANDLE, numentriesrequested : u32, entries : *mut *mut *mut super::super::super::Win32::NetworkManagement::WindowsFilteringPlatform:: FWPM_FILTER0, numentriesreturned : *mut u32) -> super::super::super::Win32::Foundation:: NTSTATUS);
    FwpmFilterEnum0(core::mem::transmute(enginehandle), core::mem::transmute(enumhandle), core::mem::transmute(numentriesrequested), core::mem::transmute(entries), core::mem::transmute(numentriesreturned))
}
#[cfg(all(feature = "Win32_NetworkManagement_WindowsFilteringPlatform", feature = "Win32_Security"))]
#[inline]
pub unsafe fn FwpmFilterGetById0(enginehandle: super::super::super::Win32::Foundation::HANDLE, id: u64, filter: *mut *mut super::super::super::Win32::NetworkManagement::WindowsFilteringPlatform::FWPM_FILTER0) -> super::super::super::Win32::Foundation::NTSTATUS {
    windows_targets::link!("fwpuclnt.dll" "system" fn FwpmFilterGetById0(enginehandle : super::super::super::Win32::Foundation:: HANDLE, id : u64, filter : *mut *mut super::super::super::Win32::NetworkManagement::WindowsFilteringPlatform:: FWPM_FILTER0) -> super::super::super::Win32::Foundation:: NTSTATUS);
    FwpmFilterGetById0(core::mem::transmute(enginehandle), core::mem::transmute(id), core::mem::transmute(filter))
}
#[cfg(all(feature = "Win32_NetworkManagement_WindowsFilteringPlatform", feature = "Win32_Security"))]
#[inline]
pub unsafe fn FwpmFilterGetByKey0(enginehandle: super::super::super::Win32::Foundation::HANDLE, key: *const windows_core::GUID, filter: *mut *mut super::super::super::Win32::NetworkManagement::WindowsFilteringPlatform::FWPM_FILTER0) -> super::super::super::Win32::Foundation::NTSTATUS {
    windows_targets::link!("fwpuclnt.dll" "system" fn FwpmFilterGetByKey0(enginehandle : super::super::super::Win32::Foundation:: HANDLE, key : *const windows_core::GUID, filter : *mut *mut super::super::super::Win32::NetworkManagement::WindowsFilteringPlatform:: FWPM_FILTER0) -> super::super::super::Win32::Foundation:: NTSTATUS);
    FwpmFilterGetByKey0(core::mem::transmute(enginehandle), core::mem::transmute(key), core::mem::transmute(filter))
}
#[cfg(feature = "Win32_Security")]
#[inline]
pub unsafe fn FwpmFilterGetSecurityInfoByKey0(enginehandle: super::super::super::Win32::Foundation::HANDLE, key: Option<*const windows_core::GUID>, securityinfo: u32, sidowner: *mut super::super::super::Win32::Security::PSID, sidgroup: *mut super::super::super::Win32::Security::PSID, dacl: *mut *mut super::super::super::Win32::Security::ACL, sacl: *mut *mut super::super::super::Win32::Security::ACL, securitydescriptor: *mut super::super::super::Win32::Security::PSECURITY_DESCRIPTOR) -> super::super::super::Win32::Foundation::NTSTATUS {
    windows_targets::link!("fwpuclnt.dll" "system" fn FwpmFilterGetSecurityInfoByKey0(enginehandle : super::super::super::Win32::Foundation:: HANDLE, key : *const windows_core::GUID, securityinfo : u32, sidowner : *mut super::super::super::Win32::Security:: PSID, sidgroup : *mut super::super::super::Win32::Security:: PSID, dacl : *mut *mut super::super::super::Win32::Security:: ACL, sacl : *mut *mut super::super::super::Win32::Security:: ACL, securitydescriptor : *mut super::super::super::Win32::Security:: PSECURITY_DESCRIPTOR) -> super::super::super::Win32::Foundation:: NTSTATUS);
    FwpmFilterGetSecurityInfoByKey0(core::mem::transmute(enginehandle), core::mem::transmute(key.unwrap_or(core::mem::zeroed())), core::mem::transmute(securityinfo), core::mem::transmute(sidowner), core::mem::transmute(sidgroup), core::mem::transmute(dacl), core::mem::transmute(sacl), core::mem::transmute(securitydescriptor))
}
#[cfg(feature = "Win32_Security")]
#[inline]
pub unsafe fn FwpmFilterSetSecurityInfoByKey0(enginehandle: super::super::super::Win32::Foundation::HANDLE, key: Option<*const windows_core::GUID>, securityinfo: u32, sidowner: Option<*const super::super::super::Win32::Security::SID>, sidgroup: Option<*const super::super::super::Win32::Security::SID>, dacl: Option<*const super::super::super::Win32::Security::ACL>, sacl: Option<*const super::super::super::Win32::Security::ACL>) -> super::super::super::Win32::Foundation::NTSTATUS {
    windows_targets::link!("fwpuclnt.dll" "system" fn FwpmFilterSetSecurityInfoByKey0(enginehandle : super::super::super::Win32::Foundation:: HANDLE, key : *const windows_core::GUID, securityinfo : u32, sidowner : *const super::super::super::Win32::Security:: SID, sidgroup : *const super::super::super::Win32::Security:: SID, dacl : *const super::super::super::Win32::Security:: ACL, sacl : *const super::super::super::Win32::Security:: ACL) -> super::super::super::Win32::Foundation:: NTSTATUS);
    FwpmFilterSetSecurityInfoByKey0(core::mem::transmute(enginehandle), core::mem::transmute(key.unwrap_or(core::mem::zeroed())), core::mem::transmute(securityinfo), core::mem::transmute(sidowner.unwrap_or(core::mem::zeroed())), core::mem::transmute(sidgroup.unwrap_or(core::mem::zeroed())), core::mem::transmute(dacl.unwrap_or(core::mem::zeroed())), core::mem::transmute(sacl.unwrap_or(core::mem::zeroed())))
}
#[inline]
pub unsafe fn FwpmFreeMemory0(p: *mut *mut core::ffi::c_void) {
    windows_targets::link!("fwpuclnt.dll" "system" fn FwpmFreeMemory0(p : *mut *mut core::ffi::c_void));
    FwpmFreeMemory0(core::mem::transmute(p))
}
#[cfg(all(feature = "Win32_NetworkManagement_WindowsFilteringPlatform", feature = "Win32_Security"))]
#[inline]
pub unsafe fn FwpmIPsecTunnelAdd0(enginehandle: super::super::super::Win32::Foundation::HANDLE, flags: u32, mainmodepolicy: Option<*const super::super::super::Win32::NetworkManagement::WindowsFilteringPlatform::FWPM_PROVIDER_CONTEXT0>, tunnelpolicy: *const super::super::super::Win32::NetworkManagement::WindowsFilteringPlatform::FWPM_PROVIDER_CONTEXT0, filterconditions: &[super::super::super::Win32::NetworkManagement::WindowsFilteringPlatform::FWPM_FILTER_CONDITION0], sd: Option<super::super::super::Win32::Security::PSECURITY_DESCRIPTOR>) -> super::super::super::Win32::Foundation::NTSTATUS {
    windows_targets::link!("fwpuclnt.dll" "system" fn FwpmIPsecTunnelAdd0(enginehandle : super::super::super::Win32::Foundation:: HANDLE, flags : u32, mainmodepolicy : *const super::super::super::Win32::NetworkManagement::WindowsFilteringPlatform:: FWPM_PROVIDER_CONTEXT0, tunnelpolicy : *const super::super::super::Win32::NetworkManagement::WindowsFilteringPlatform:: FWPM_PROVIDER_CONTEXT0, numfilterconditions : u32, filterconditions : *const super::super::super::Win32::NetworkManagement::WindowsFilteringPlatform:: FWPM_FILTER_CONDITION0, sd : super::super::super::Win32::Security:: PSECURITY_DESCRIPTOR) -> super::super::super::Win32::Foundation:: NTSTATUS);
    FwpmIPsecTunnelAdd0(core::mem::transmute(enginehandle), core::mem::transmute(flags), core::mem::transmute(mainmodepolicy.unwrap_or(core::mem::zeroed())), core::mem::transmute(tunnelpolicy), filterconditions.len().try_into().unwrap(), core::mem::transmute(filterconditions.as_ptr()), core::mem::transmute(sd.unwrap_or(core::mem::zeroed())))
}
#[cfg(all(feature = "Win32_NetworkManagement_WindowsFilteringPlatform", feature = "Win32_Security"))]
#[inline]
pub unsafe fn FwpmIPsecTunnelAdd1(enginehandle: super::super::super::Win32::Foundation::HANDLE, flags: u32, mainmodepolicy: Option<*const super::super::super::Win32::NetworkManagement::WindowsFilteringPlatform::FWPM_PROVIDER_CONTEXT1>, tunnelpolicy: *const super::super::super::Win32::NetworkManagement::WindowsFilteringPlatform::FWPM_PROVIDER_CONTEXT1, filterconditions: &[super::super::super::Win32::NetworkManagement::WindowsFilteringPlatform::FWPM_FILTER_CONDITION0], keymodkey: Option<*const windows_core::GUID>, sd: Option<super::super::super::Win32::Security::PSECURITY_DESCRIPTOR>) -> super::super::super::Win32::Foundation::NTSTATUS {
    windows_targets::link!("fwpuclnt.dll" "system" fn FwpmIPsecTunnelAdd1(enginehandle : super::super::super::Win32::Foundation:: HANDLE, flags : u32, mainmodepolicy : *const super::super::super::Win32::NetworkManagement::WindowsFilteringPlatform:: FWPM_PROVIDER_CONTEXT1, tunnelpolicy : *const super::super::super::Win32::NetworkManagement::WindowsFilteringPlatform:: FWPM_PROVIDER_CONTEXT1, numfilterconditions : u32, filterconditions : *const super::super::super::Win32::NetworkManagement::WindowsFilteringPlatform:: FWPM_FILTER_CONDITION0, keymodkey : *const windows_core::GUID, sd : super::super::super::Win32::Security:: PSECURITY_DESCRIPTOR) -> super::super::super::Win32::Foundation:: NTSTATUS);
    FwpmIPsecTunnelAdd1(core::mem::transmute(enginehandle), core::mem::transmute(flags), core::mem::transmute(mainmodepolicy.unwrap_or(core::mem::zeroed())), core::mem::transmute(tunnelpolicy), filterconditions.len().try_into().unwrap(), core::mem::transmute(filterconditions.as_ptr()), core::mem::transmute(keymodkey.unwrap_or(core::mem::zeroed())), core::mem::transmute(sd.unwrap_or(core::mem::zeroed())))
}
#[cfg(all(feature = "Win32_NetworkManagement_WindowsFilteringPlatform", feature = "Win32_Security"))]
#[inline]
pub unsafe fn FwpmIPsecTunnelAdd2(enginehandle: super::super::super::Win32::Foundation::HANDLE, flags: u32, mainmodepolicy: Option<*const super::super::super::Win32::NetworkManagement::WindowsFilteringPlatform::FWPM_PROVIDER_CONTEXT2>, tunnelpolicy: *const super::super::super::Win32::NetworkManagement::WindowsFilteringPlatform::FWPM_PROVIDER_CONTEXT2, filterconditions: &[super::super::super::Win32::NetworkManagement::WindowsFilteringPlatform::FWPM_FILTER_CONDITION0], keymodkey: Option<*const windows_core::GUID>, sd: Option<super::super::super::Win32::Security::PSECURITY_DESCRIPTOR>) -> super::super::super::Win32::Foundation::NTSTATUS {
    windows_targets::link!("fwpuclnt.dll" "system" fn FwpmIPsecTunnelAdd2(enginehandle : super::super::super::Win32::Foundation:: HANDLE, flags : u32, mainmodepolicy : *const super::super::super::Win32::NetworkManagement::WindowsFilteringPlatform:: FWPM_PROVIDER_CONTEXT2, tunnelpolicy : *const super::super::super::Win32::NetworkManagement::WindowsFilteringPlatform:: FWPM_PROVIDER_CONTEXT2, numfilterconditions : u32, filterconditions : *const super::super::super::Win32::NetworkManagement::WindowsFilteringPlatform:: FWPM_FILTER_CONDITION0, keymodkey : *const windows_core::GUID, sd : super::super::super::Win32::Security:: PSECURITY_DESCRIPTOR) -> super::super::super::Win32::Foundation:: NTSTATUS);
    FwpmIPsecTunnelAdd2(core::mem::transmute(enginehandle), core::mem::transmute(flags), core::mem::transmute(mainmodepolicy.unwrap_or(core::mem::zeroed())), core::mem::transmute(tunnelpolicy), filterconditions.len().try_into().unwrap(), core::mem::transmute(filterconditions.as_ptr()), core::mem::transmute(keymodkey.unwrap_or(core::mem::zeroed())), core::mem::transmute(sd.unwrap_or(core::mem::zeroed())))
}
#[cfg(all(feature = "Win32_NetworkManagement_WindowsFilteringPlatform", feature = "Win32_Security"))]
#[inline]
pub unsafe fn FwpmIPsecTunnelAdd3(enginehandle: super::super::super::Win32::Foundation::HANDLE, flags: u32, mainmodepolicy: Option<*const super::super::super::Win32::NetworkManagement::WindowsFilteringPlatform::FWPM_PROVIDER_CONTEXT3>, tunnelpolicy: *const super::super::super::Win32::NetworkManagement::WindowsFilteringPlatform::FWPM_PROVIDER_CONTEXT3, filterconditions: &[super::super::super::Win32::NetworkManagement::WindowsFilteringPlatform::FWPM_FILTER_CONDITION0], keymodkey: Option<*const windows_core::GUID>, sd: Option<super::super::super::Win32::Security::PSECURITY_DESCRIPTOR>) -> super::super::super::Win32::Foundation::NTSTATUS {
    windows_targets::link!("fwpuclnt.dll" "system" fn FwpmIPsecTunnelAdd3(enginehandle : super::super::super::Win32::Foundation:: HANDLE, flags : u32, mainmodepolicy : *const super::super::super::Win32::NetworkManagement::WindowsFilteringPlatform:: FWPM_PROVIDER_CONTEXT3, tunnelpolicy : *const super::super::super::Win32::NetworkManagement::WindowsFilteringPlatform:: FWPM_PROVIDER_CONTEXT3, numfilterconditions : u32, filterconditions : *const super::super::super::Win32::NetworkManagement::WindowsFilteringPlatform:: FWPM_FILTER_CONDITION0, keymodkey : *const windows_core::GUID, sd : super::super::super::Win32::Security:: PSECURITY_DESCRIPTOR) -> super::super::super::Win32::Foundation:: NTSTATUS);
    FwpmIPsecTunnelAdd3(core::mem::transmute(enginehandle), core::mem::transmute(flags), core::mem::transmute(mainmodepolicy.unwrap_or(core::mem::zeroed())), core::mem::transmute(tunnelpolicy), filterconditions.len().try_into().unwrap(), core::mem::transmute(filterconditions.as_ptr()), core::mem::transmute(keymodkey.unwrap_or(core::mem::zeroed())), core::mem::transmute(sd.unwrap_or(core::mem::zeroed())))
}
#[inline]
pub unsafe fn FwpmIPsecTunnelDeleteByKey0(enginehandle: super::super::super::Win32::Foundation::HANDLE, key: *const windows_core::GUID) -> super::super::super::Win32::Foundation::NTSTATUS {
    windows_targets::link!("fwpuclnt.dll" "system" fn FwpmIPsecTunnelDeleteByKey0(enginehandle : super::super::super::Win32::Foundation:: HANDLE, key : *const windows_core::GUID) -> super::super::super::Win32::Foundation:: NTSTATUS);
    FwpmIPsecTunnelDeleteByKey0(core::mem::transmute(enginehandle), core::mem::transmute(key))
}
#[cfg(feature = "Win32_NetworkManagement_WindowsFilteringPlatform")]
#[inline]
pub unsafe fn FwpmLayerCreateEnumHandle0(enginehandle: super::super::super::Win32::Foundation::HANDLE, enumtemplate: Option<*const super::super::super::Win32::NetworkManagement::WindowsFilteringPlatform::FWPM_LAYER_ENUM_TEMPLATE0>, enumhandle: *mut super::super::super::Win32::Foundation::HANDLE) -> super::super::super::Win32::Foundation::NTSTATUS {
    windows_targets::link!("fwpuclnt.dll" "system" fn FwpmLayerCreateEnumHandle0(enginehandle : super::super::super::Win32::Foundation:: HANDLE, enumtemplate : *const super::super::super::Win32::NetworkManagement::WindowsFilteringPlatform:: FWPM_LAYER_ENUM_TEMPLATE0, enumhandle : *mut super::super::super::Win32::Foundation:: HANDLE) -> super::super::super::Win32::Foundation:: NTSTATUS);
    FwpmLayerCreateEnumHandle0(core::mem::transmute(enginehandle), core::mem::transmute(enumtemplate.unwrap_or(core::mem::zeroed())), core::mem::transmute(enumhandle))
}
#[inline]
pub unsafe fn FwpmLayerDestroyEnumHandle0(enginehandle: super::super::super::Win32::Foundation::HANDLE, enumhandle: super::super::super::Win32::Foundation::HANDLE) -> super::super::super::Win32::Foundation::NTSTATUS {
    windows_targets::link!("fwpuclnt.dll" "system" fn FwpmLayerDestroyEnumHandle0(enginehandle : super::super::super::Win32::Foundation:: HANDLE, enumhandle : super::super::super::Win32::Foundation:: HANDLE) -> super::super::super::Win32::Foundation:: NTSTATUS);
    FwpmLayerDestroyEnumHandle0(core::mem::transmute(enginehandle), core::mem::transmute(enumhandle))
}
#[cfg(feature = "Win32_NetworkManagement_WindowsFilteringPlatform")]
#[inline]
pub unsafe fn FwpmLayerEnum0(enginehandle: super::super::super::Win32::Foundation::HANDLE, enumhandle: super::super::super::Win32::Foundation::HANDLE, numentriesrequested: u32, entries: *mut *mut *mut super::super::super::Win32::NetworkManagement::WindowsFilteringPlatform::FWPM_LAYER0, numentriesreturned: *mut u32) -> super::super::super::Win32::Foundation::NTSTATUS {
    windows_targets::link!("fwpuclnt.dll" "system" fn FwpmLayerEnum0(enginehandle : super::super::super::Win32::Foundation:: HANDLE, enumhandle : super::super::super::Win32::Foundation:: HANDLE, numentriesrequested : u32, entries : *mut *mut *mut super::super::super::Win32::NetworkManagement::WindowsFilteringPlatform:: FWPM_LAYER0, numentriesreturned : *mut u32) -> super::super::super::Win32::Foundation:: NTSTATUS);
    FwpmLayerEnum0(core::mem::transmute(enginehandle), core::mem::transmute(enumhandle), core::mem::transmute(numentriesrequested), core::mem::transmute(entries), core::mem::transmute(numentriesreturned))
}
#[cfg(feature = "Win32_NetworkManagement_WindowsFilteringPlatform")]
#[inline]
pub unsafe fn FwpmLayerGetById0(enginehandle: super::super::super::Win32::Foundation::HANDLE, id: u16, layer: *mut *mut super::super::super::Win32::NetworkManagement::WindowsFilteringPlatform::FWPM_LAYER0) -> super::super::super::Win32::Foundation::NTSTATUS {
    windows_targets::link!("fwpuclnt.dll" "system" fn FwpmLayerGetById0(enginehandle : super::super::super::Win32::Foundation:: HANDLE, id : u16, layer : *mut *mut super::super::super::Win32::NetworkManagement::WindowsFilteringPlatform:: FWPM_LAYER0) -> super::super::super::Win32::Foundation:: NTSTATUS);
    FwpmLayerGetById0(core::mem::transmute(enginehandle), core::mem::transmute(id), core::mem::transmute(layer))
}
#[cfg(feature = "Win32_NetworkManagement_WindowsFilteringPlatform")]
#[inline]
pub unsafe fn FwpmLayerGetByKey0(enginehandle: super::super::super::Win32::Foundation::HANDLE, key: *const windows_core::GUID, layer: *mut *mut super::super::super::Win32::NetworkManagement::WindowsFilteringPlatform::FWPM_LAYER0) -> super::super::super::Win32::Foundation::NTSTATUS {
    windows_targets::link!("fwpuclnt.dll" "system" fn FwpmLayerGetByKey0(enginehandle : super::super::super::Win32::Foundation:: HANDLE, key : *const windows_core::GUID, layer : *mut *mut super::super::super::Win32::NetworkManagement::WindowsFilteringPlatform:: FWPM_LAYER0) -> super::super::super::Win32::Foundation:: NTSTATUS);
    FwpmLayerGetByKey0(core::mem::transmute(enginehandle), core::mem::transmute(key), core::mem::transmute(layer))
}
#[cfg(feature = "Win32_Security")]
#[inline]
pub unsafe fn FwpmLayerGetSecurityInfoByKey0(enginehandle: super::super::super::Win32::Foundation::HANDLE, key: Option<*const windows_core::GUID>, securityinfo: u32, sidowner: *mut super::super::super::Win32::Security::PSID, sidgroup: *mut super::super::super::Win32::Security::PSID, dacl: *mut *mut super::super::super::Win32::Security::ACL, sacl: *mut *mut super::super::super::Win32::Security::ACL, securitydescriptor: *mut super::super::super::Win32::Security::PSECURITY_DESCRIPTOR) -> super::super::super::Win32::Foundation::NTSTATUS {
    windows_targets::link!("fwpuclnt.dll" "system" fn FwpmLayerGetSecurityInfoByKey0(enginehandle : super::super::super::Win32::Foundation:: HANDLE, key : *const windows_core::GUID, securityinfo : u32, sidowner : *mut super::super::super::Win32::Security:: PSID, sidgroup : *mut super::super::super::Win32::Security:: PSID, dacl : *mut *mut super::super::super::Win32::Security:: ACL, sacl : *mut *mut super::super::super::Win32::Security:: ACL, securitydescriptor : *mut super::super::super::Win32::Security:: PSECURITY_DESCRIPTOR) -> super::super::super::Win32::Foundation:: NTSTATUS);
    FwpmLayerGetSecurityInfoByKey0(core::mem::transmute(enginehandle), core::mem::transmute(key.unwrap_or(core::mem::zeroed())), core::mem::transmute(securityinfo), core::mem::transmute(sidowner), core::mem::transmute(sidgroup), core::mem::transmute(dacl), core::mem::transmute(sacl), core::mem::transmute(securitydescriptor))
}
#[cfg(feature = "Win32_Security")]
#[inline]
pub unsafe fn FwpmLayerSetSecurityInfoByKey0(enginehandle: super::super::super::Win32::Foundation::HANDLE, key: Option<*const windows_core::GUID>, securityinfo: u32, sidowner: Option<*const super::super::super::Win32::Security::SID>, sidgroup: Option<*const super::super::super::Win32::Security::SID>, dacl: Option<*const super::super::super::Win32::Security::ACL>, sacl: Option<*const super::super::super::Win32::Security::ACL>) -> super::super::super::Win32::Foundation::NTSTATUS {
    windows_targets::link!("fwpuclnt.dll" "system" fn FwpmLayerSetSecurityInfoByKey0(enginehandle : super::super::super::Win32::Foundation:: HANDLE, key : *const windows_core::GUID, securityinfo : u32, sidowner : *const super::super::super::Win32::Security:: SID, sidgroup : *const super::super::super::Win32::Security:: SID, dacl : *const super::super::super::Win32::Security:: ACL, sacl : *const super::super::super::Win32::Security:: ACL) -> super::super::super::Win32::Foundation:: NTSTATUS);
    FwpmLayerSetSecurityInfoByKey0(core::mem::transmute(enginehandle), core::mem::transmute(key.unwrap_or(core::mem::zeroed())), core::mem::transmute(securityinfo), core::mem::transmute(sidowner.unwrap_or(core::mem::zeroed())), core::mem::transmute(sidgroup.unwrap_or(core::mem::zeroed())), core::mem::transmute(dacl.unwrap_or(core::mem::zeroed())), core::mem::transmute(sacl.unwrap_or(core::mem::zeroed())))
}
#[cfg(all(feature = "Win32_NetworkManagement_WindowsFilteringPlatform", feature = "Win32_Security"))]
#[inline]
pub unsafe fn FwpmNetEventCreateEnumHandle0(enginehandle: super::super::super::Win32::Foundation::HANDLE, enumtemplate: Option<*const super::super::super::Win32::NetworkManagement::WindowsFilteringPlatform::FWPM_NET_EVENT_ENUM_TEMPLATE0>, enumhandle: *mut super::super::super::Win32::Foundation::HANDLE) -> super::super::super::Win32::Foundation::NTSTATUS {
    windows_targets::link!("fwpuclnt.dll" "system" fn FwpmNetEventCreateEnumHandle0(enginehandle : super::super::super::Win32::Foundation:: HANDLE, enumtemplate : *const super::super::super::Win32::NetworkManagement::WindowsFilteringPlatform:: FWPM_NET_EVENT_ENUM_TEMPLATE0, enumhandle : *mut super::super::super::Win32::Foundation:: HANDLE) -> super::super::super::Win32::Foundation:: NTSTATUS);
    FwpmNetEventCreateEnumHandle0(core::mem::transmute(enginehandle), core::mem::transmute(enumtemplate.unwrap_or(core::mem::zeroed())), core::mem::transmute(enumhandle))
}
#[inline]
pub unsafe fn FwpmNetEventDestroyEnumHandle0(enginehandle: super::super::super::Win32::Foundation::HANDLE, enumhandle: super::super::super::Win32::Foundation::HANDLE) -> super::super::super::Win32::Foundation::NTSTATUS {
    windows_targets::link!("fwpuclnt.dll" "system" fn FwpmNetEventDestroyEnumHandle0(enginehandle : super::super::super::Win32::Foundation:: HANDLE, enumhandle : super::super::super::Win32::Foundation:: HANDLE) -> super::super::super::Win32::Foundation:: NTSTATUS);
    FwpmNetEventDestroyEnumHandle0(core::mem::transmute(enginehandle), core::mem::transmute(enumhandle))
}
#[cfg(all(feature = "Win32_NetworkManagement_WindowsFilteringPlatform", feature = "Win32_Security"))]
#[inline]
pub unsafe fn FwpmNetEventEnum0(enginehandle: super::super::super::Win32::Foundation::HANDLE, enumhandle: super::super::super::Win32::Foundation::HANDLE, numentriesrequested: u32, entries: *mut *mut *mut super::super::super::Win32::NetworkManagement::WindowsFilteringPlatform::FWPM_NET_EVENT0, numentriesreturned: *mut u32) -> super::super::super::Win32::Foundation::NTSTATUS {
    windows_targets::link!("fwpuclnt.dll" "system" fn FwpmNetEventEnum0(enginehandle : super::super::super::Win32::Foundation:: HANDLE, enumhandle : super::super::super::Win32::Foundation:: HANDLE, numentriesrequested : u32, entries : *mut *mut *mut super::super::super::Win32::NetworkManagement::WindowsFilteringPlatform:: FWPM_NET_EVENT0, numentriesreturned : *mut u32) -> super::super::super::Win32::Foundation:: NTSTATUS);
    FwpmNetEventEnum0(core::mem::transmute(enginehandle), core::mem::transmute(enumhandle), core::mem::transmute(numentriesrequested), core::mem::transmute(entries), core::mem::transmute(numentriesreturned))
}
#[cfg(all(feature = "Win32_NetworkManagement_WindowsFilteringPlatform", feature = "Win32_Security"))]
#[inline]
pub unsafe fn FwpmNetEventEnum1(enginehandle: super::super::super::Win32::Foundation::HANDLE, enumhandle: super::super::super::Win32::Foundation::HANDLE, numentriesrequested: u32, entries: *mut *mut *mut super::super::super::Win32::NetworkManagement::WindowsFilteringPlatform::FWPM_NET_EVENT1, numentriesreturned: *mut u32) -> super::super::super::Win32::Foundation::NTSTATUS {
    windows_targets::link!("fwpuclnt.dll" "system" fn FwpmNetEventEnum1(enginehandle : super::super::super::Win32::Foundation:: HANDLE, enumhandle : super::super::super::Win32::Foundation:: HANDLE, numentriesrequested : u32, entries : *mut *mut *mut super::super::super::Win32::NetworkManagement::WindowsFilteringPlatform:: FWPM_NET_EVENT1, numentriesreturned : *mut u32) -> super::super::super::Win32::Foundation:: NTSTATUS);
    FwpmNetEventEnum1(core::mem::transmute(enginehandle), core::mem::transmute(enumhandle), core::mem::transmute(numentriesrequested), core::mem::transmute(entries), core::mem::transmute(numentriesreturned))
}
#[cfg(all(feature = "Win32_NetworkManagement_WindowsFilteringPlatform", feature = "Win32_Security"))]
#[inline]
pub unsafe fn FwpmNetEventEnum2(enginehandle: super::super::super::Win32::Foundation::HANDLE, enumhandle: super::super::super::Win32::Foundation::HANDLE, numentriesrequested: u32, entries: *mut *mut *mut super::super::super::Win32::NetworkManagement::WindowsFilteringPlatform::FWPM_NET_EVENT2, numentriesreturned: *mut u32) -> super::super::super::Win32::Foundation::NTSTATUS {
    windows_targets::link!("fwpuclnt.dll" "system" fn FwpmNetEventEnum2(enginehandle : super::super::super::Win32::Foundation:: HANDLE, enumhandle : super::super::super::Win32::Foundation:: HANDLE, numentriesrequested : u32, entries : *mut *mut *mut super::super::super::Win32::NetworkManagement::WindowsFilteringPlatform:: FWPM_NET_EVENT2, numentriesreturned : *mut u32) -> super::super::super::Win32::Foundation:: NTSTATUS);
    FwpmNetEventEnum2(core::mem::transmute(enginehandle), core::mem::transmute(enumhandle), core::mem::transmute(numentriesrequested), core::mem::transmute(entries), core::mem::transmute(numentriesreturned))
}
#[cfg(all(feature = "Win32_NetworkManagement_WindowsFilteringPlatform", feature = "Win32_Security"))]
#[inline]
pub unsafe fn FwpmNetEventEnum3(enginehandle: super::super::super::Win32::Foundation::HANDLE, enumhandle: super::super::super::Win32::Foundation::HANDLE, numentriesrequested: u32, entries: *mut *mut *mut super::super::super::Win32::NetworkManagement::WindowsFilteringPlatform::FWPM_NET_EVENT3, numentriesreturned: *mut u32) -> super::super::super::Win32::Foundation::NTSTATUS {
    windows_targets::link!("fwpuclnt.dll" "system" fn FwpmNetEventEnum3(enginehandle : super::super::super::Win32::Foundation:: HANDLE, enumhandle : super::super::super::Win32::Foundation:: HANDLE, numentriesrequested : u32, entries : *mut *mut *mut super::super::super::Win32::NetworkManagement::WindowsFilteringPlatform:: FWPM_NET_EVENT3, numentriesreturned : *mut u32) -> super::super::super::Win32::Foundation:: NTSTATUS);
    FwpmNetEventEnum3(core::mem::transmute(enginehandle), core::mem::transmute(enumhandle), core::mem::transmute(numentriesrequested), core::mem::transmute(entries), core::mem::transmute(numentriesreturned))
}
#[cfg(all(feature = "Win32_NetworkManagement_WindowsFilteringPlatform", feature = "Win32_Security"))]
#[inline]
pub unsafe fn FwpmNetEventEnum4(enginehandle: super::super::super::Win32::Foundation::HANDLE, enumhandle: super::super::super::Win32::Foundation::HANDLE, numentriesrequested: u32, entries: *mut *mut *mut super::super::super::Win32::NetworkManagement::WindowsFilteringPlatform::FWPM_NET_EVENT4, numentriesreturned: *mut u32) -> super::super::super::Win32::Foundation::NTSTATUS {
    windows_targets::link!("fwpuclnt.dll" "system" fn FwpmNetEventEnum4(enginehandle : super::super::super::Win32::Foundation:: HANDLE, enumhandle : super::super::super::Win32::Foundation:: HANDLE, numentriesrequested : u32, entries : *mut *mut *mut super::super::super::Win32::NetworkManagement::WindowsFilteringPlatform:: FWPM_NET_EVENT4, numentriesreturned : *mut u32) -> super::super::super::Win32::Foundation:: NTSTATUS);
    FwpmNetEventEnum4(core::mem::transmute(enginehandle), core::mem::transmute(enumhandle), core::mem::transmute(numentriesrequested), core::mem::transmute(entries), core::mem::transmute(numentriesreturned))
}
#[cfg(all(feature = "Win32_NetworkManagement_WindowsFilteringPlatform", feature = "Win32_Security"))]
#[inline]
pub unsafe fn FwpmNetEventEnum5(enginehandle: super::super::super::Win32::Foundation::HANDLE, enumhandle: super::super::super::Win32::Foundation::HANDLE, numentriesrequested: u32, entries: *mut *mut *mut super::super::super::Win32::NetworkManagement::WindowsFilteringPlatform::FWPM_NET_EVENT5, numentriesreturned: *mut u32) -> super::super::super::Win32::Foundation::NTSTATUS {
    windows_targets::link!("fwpuclnt.dll" "system" fn FwpmNetEventEnum5(enginehandle : super::super::super::Win32::Foundation:: HANDLE, enumhandle : super::super::super::Win32::Foundation:: HANDLE, numentriesrequested : u32, entries : *mut *mut *mut super::super::super::Win32::NetworkManagement::WindowsFilteringPlatform:: FWPM_NET_EVENT5, numentriesreturned : *mut u32) -> super::super::super::Win32::Foundation:: NTSTATUS);
    FwpmNetEventEnum5(core::mem::transmute(enginehandle), core::mem::transmute(enumhandle), core::mem::transmute(numentriesrequested), core::mem::transmute(entries), core::mem::transmute(numentriesreturned))
}
#[cfg(feature = "Win32_Security")]
#[inline]
pub unsafe fn FwpmNetEventsGetSecurityInfo0(enginehandle: super::super::super::Win32::Foundation::HANDLE, securityinfo: u32, sidowner: *mut super::super::super::Win32::Security::PSID, sidgroup: *mut super::super::super::Win32::Security::PSID, dacl: *mut *mut super::super::super::Win32::Security::ACL, sacl: *mut *mut super::super::super::Win32::Security::ACL, securitydescriptor: *mut super::super::super::Win32::Security::PSECURITY_DESCRIPTOR) -> super::super::super::Win32::Foundation::NTSTATUS {
    windows_targets::link!("fwpuclnt.dll" "system" fn FwpmNetEventsGetSecurityInfo0(enginehandle : super::super::super::Win32::Foundation:: HANDLE, securityinfo : u32, sidowner : *mut super::super::super::Win32::Security:: PSID, sidgroup : *mut super::super::super::Win32::Security:: PSID, dacl : *mut *mut super::super::super::Win32::Security:: ACL, sacl : *mut *mut super::super::super::Win32::Security:: ACL, securitydescriptor : *mut super::super::super::Win32::Security:: PSECURITY_DESCRIPTOR) -> super::super::super::Win32::Foundation:: NTSTATUS);
    FwpmNetEventsGetSecurityInfo0(core::mem::transmute(enginehandle), core::mem::transmute(securityinfo), core::mem::transmute(sidowner), core::mem::transmute(sidgroup), core::mem::transmute(dacl), core::mem::transmute(sacl), core::mem::transmute(securitydescriptor))
}
#[cfg(feature = "Win32_Security")]
#[inline]
pub unsafe fn FwpmNetEventsSetSecurityInfo0(enginehandle: super::super::super::Win32::Foundation::HANDLE, securityinfo: u32, sidowner: Option<*const super::super::super::Win32::Security::SID>, sidgroup: Option<*const super::super::super::Win32::Security::SID>, dacl: Option<*const super::super::super::Win32::Security::ACL>, sacl: Option<*const super::super::super::Win32::Security::ACL>) -> super::super::super::Win32::Foundation::NTSTATUS {
    windows_targets::link!("fwpuclnt.dll" "system" fn FwpmNetEventsSetSecurityInfo0(enginehandle : super::super::super::Win32::Foundation:: HANDLE, securityinfo : u32, sidowner : *const super::super::super::Win32::Security:: SID, sidgroup : *const super::super::super::Win32::Security:: SID, dacl : *const super::super::super::Win32::Security:: ACL, sacl : *const super::super::super::Win32::Security:: ACL) -> super::super::super::Win32::Foundation:: NTSTATUS);
    FwpmNetEventsSetSecurityInfo0(core::mem::transmute(enginehandle), core::mem::transmute(securityinfo), core::mem::transmute(sidowner.unwrap_or(core::mem::zeroed())), core::mem::transmute(sidgroup.unwrap_or(core::mem::zeroed())), core::mem::transmute(dacl.unwrap_or(core::mem::zeroed())), core::mem::transmute(sacl.unwrap_or(core::mem::zeroed())))
}
#[cfg(all(feature = "Win32_NetworkManagement_WindowsFilteringPlatform", feature = "Win32_Security"))]
#[inline]
pub unsafe fn FwpmProviderAdd0(enginehandle: super::super::super::Win32::Foundation::HANDLE, provider: *const super::super::super::Win32::NetworkManagement::WindowsFilteringPlatform::FWPM_PROVIDER0, sd: Option<super::super::super::Win32::Security::PSECURITY_DESCRIPTOR>) -> super::super::super::Win32::Foundation::NTSTATUS {
    windows_targets::link!("fwpuclnt.dll" "system" fn FwpmProviderAdd0(enginehandle : super::super::super::Win32::Foundation:: HANDLE, provider : *const super::super::super::Win32::NetworkManagement::WindowsFilteringPlatform:: FWPM_PROVIDER0, sd : super::super::super::Win32::Security:: PSECURITY_DESCRIPTOR) -> super::super::super::Win32::Foundation:: NTSTATUS);
    FwpmProviderAdd0(core::mem::transmute(enginehandle), core::mem::transmute(provider), core::mem::transmute(sd.unwrap_or(core::mem::zeroed())))
}
#[cfg(all(feature = "Win32_NetworkManagement_WindowsFilteringPlatform", feature = "Win32_Security"))]
#[inline]
pub unsafe fn FwpmProviderContextAdd0(enginehandle: super::super::super::Win32::Foundation::HANDLE, providercontext: *const super::super::super::Win32::NetworkManagement::WindowsFilteringPlatform::FWPM_PROVIDER_CONTEXT0, sd: Option<super::super::super::Win32::Security::PSECURITY_DESCRIPTOR>, id: Option<*mut u64>) -> super::super::super::Win32::Foundation::NTSTATUS {
    windows_targets::link!("fwpuclnt.dll" "system" fn FwpmProviderContextAdd0(enginehandle : super::super::super::Win32::Foundation:: HANDLE, providercontext : *const super::super::super::Win32::NetworkManagement::WindowsFilteringPlatform:: FWPM_PROVIDER_CONTEXT0, sd : super::super::super::Win32::Security:: PSECURITY_DESCRIPTOR, id : *mut u64) -> super::super::super::Win32::Foundation:: NTSTATUS);
    FwpmProviderContextAdd0(core::mem::transmute(enginehandle), core::mem::transmute(providercontext), core::mem::transmute(sd.unwrap_or(core::mem::zeroed())), core::mem::transmute(id.unwrap_or(core::mem::zeroed())))
}
#[cfg(all(feature = "Win32_NetworkManagement_WindowsFilteringPlatform", feature = "Win32_Security"))]
#[inline]
pub unsafe fn FwpmProviderContextAdd1(enginehandle: super::super::super::Win32::Foundation::HANDLE, providercontext: *const super::super::super::Win32::NetworkManagement::WindowsFilteringPlatform::FWPM_PROVIDER_CONTEXT1, sd: Option<super::super::super::Win32::Security::PSECURITY_DESCRIPTOR>, id: Option<*mut u64>) -> super::super::super::Win32::Foundation::NTSTATUS {
    windows_targets::link!("fwpuclnt.dll" "system" fn FwpmProviderContextAdd1(enginehandle : super::super::super::Win32::Foundation:: HANDLE, providercontext : *const super::super::super::Win32::NetworkManagement::WindowsFilteringPlatform:: FWPM_PROVIDER_CONTEXT1, sd : super::super::super::Win32::Security:: PSECURITY_DESCRIPTOR, id : *mut u64) -> super::super::super::Win32::Foundation:: NTSTATUS);
    FwpmProviderContextAdd1(core::mem::transmute(enginehandle), core::mem::transmute(providercontext), core::mem::transmute(sd.unwrap_or(core::mem::zeroed())), core::mem::transmute(id.unwrap_or(core::mem::zeroed())))
}
#[cfg(all(feature = "Win32_NetworkManagement_WindowsFilteringPlatform", feature = "Win32_Security"))]
#[inline]
pub unsafe fn FwpmProviderContextAdd2(enginehandle: super::super::super::Win32::Foundation::HANDLE, providercontext: *const super::super::super::Win32::NetworkManagement::WindowsFilteringPlatform::FWPM_PROVIDER_CONTEXT2, sd: Option<super::super::super::Win32::Security::PSECURITY_DESCRIPTOR>, id: Option<*mut u64>) -> super::super::super::Win32::Foundation::NTSTATUS {
    windows_targets::link!("fwpuclnt.dll" "system" fn FwpmProviderContextAdd2(enginehandle : super::super::super::Win32::Foundation:: HANDLE, providercontext : *const super::super::super::Win32::NetworkManagement::WindowsFilteringPlatform:: FWPM_PROVIDER_CONTEXT2, sd : super::super::super::Win32::Security:: PSECURITY_DESCRIPTOR, id : *mut u64) -> super::super::super::Win32::Foundation:: NTSTATUS);
    FwpmProviderContextAdd2(core::mem::transmute(enginehandle), core::mem::transmute(providercontext), core::mem::transmute(sd.unwrap_or(core::mem::zeroed())), core::mem::transmute(id.unwrap_or(core::mem::zeroed())))
}
#[cfg(all(feature = "Win32_NetworkManagement_WindowsFilteringPlatform", feature = "Win32_Security"))]
#[inline]
pub unsafe fn FwpmProviderContextAdd3(enginehandle: super::super::super::Win32::Foundation::HANDLE, providercontext: *const super::super::super::Win32::NetworkManagement::WindowsFilteringPlatform::FWPM_PROVIDER_CONTEXT3, sd: Option<super::super::super::Win32::Security::PSECURITY_DESCRIPTOR>, id: Option<*mut u64>) -> super::super::super::Win32::Foundation::NTSTATUS {
    windows_targets::link!("fwpuclnt.dll" "system" fn FwpmProviderContextAdd3(enginehandle : super::super::super::Win32::Foundation:: HANDLE, providercontext : *const super::super::super::Win32::NetworkManagement::WindowsFilteringPlatform:: FWPM_PROVIDER_CONTEXT3, sd : super::super::super::Win32::Security:: PSECURITY_DESCRIPTOR, id : *mut u64) -> super::super::super::Win32::Foundation:: NTSTATUS);
    FwpmProviderContextAdd3(core::mem::transmute(enginehandle), core::mem::transmute(providercontext), core::mem::transmute(sd.unwrap_or(core::mem::zeroed())), core::mem::transmute(id.unwrap_or(core::mem::zeroed())))
}
#[cfg(feature = "Win32_NetworkManagement_WindowsFilteringPlatform")]
#[inline]
pub unsafe fn FwpmProviderContextCreateEnumHandle0(enginehandle: super::super::super::Win32::Foundation::HANDLE, enumtemplate: Option<*const super::super::super::Win32::NetworkManagement::WindowsFilteringPlatform::FWPM_PROVIDER_CONTEXT_ENUM_TEMPLATE0>, enumhandle: *mut super::super::super::Win32::Foundation::HANDLE) -> super::super::super::Win32::Foundation::NTSTATUS {
    windows_targets::link!("fwpuclnt.dll" "system" fn FwpmProviderContextCreateEnumHandle0(enginehandle : super::super::super::Win32::Foundation:: HANDLE, enumtemplate : *const super::super::super::Win32::NetworkManagement::WindowsFilteringPlatform:: FWPM_PROVIDER_CONTEXT_ENUM_TEMPLATE0, enumhandle : *mut super::super::super::Win32::Foundation:: HANDLE) -> super::super::super::Win32::Foundation:: NTSTATUS);
    FwpmProviderContextCreateEnumHandle0(core::mem::transmute(enginehandle), core::mem::transmute(enumtemplate.unwrap_or(core::mem::zeroed())), core::mem::transmute(enumhandle))
}
#[inline]
pub unsafe fn FwpmProviderContextDeleteById0(enginehandle: super::super::super::Win32::Foundation::HANDLE, id: u64) -> super::super::super::Win32::Foundation::NTSTATUS {
    windows_targets::link!("fwpuclnt.dll" "system" fn FwpmProviderContextDeleteById0(enginehandle : super::super::super::Win32::Foundation:: HANDLE, id : u64) -> super::super::super::Win32::Foundation:: NTSTATUS);
    FwpmProviderContextDeleteById0(core::mem::transmute(enginehandle), core::mem::transmute(id))
}
#[inline]
pub unsafe fn FwpmProviderContextDeleteByKey0(enginehandle: super::super::super::Win32::Foundation::HANDLE, key: *const windows_core::GUID) -> super::super::super::Win32::Foundation::NTSTATUS {
    windows_targets::link!("fwpuclnt.dll" "system" fn FwpmProviderContextDeleteByKey0(enginehandle : super::super::super::Win32::Foundation:: HANDLE, key : *const windows_core::GUID) -> super::super::super::Win32::Foundation:: NTSTATUS);
    FwpmProviderContextDeleteByKey0(core::mem::transmute(enginehandle), core::mem::transmute(key))
}
#[inline]
pub unsafe fn FwpmProviderContextDestroyEnumHandle0(enginehandle: super::super::super::Win32::Foundation::HANDLE, enumhandle: super::super::super::Win32::Foundation::HANDLE) -> super::super::super::Win32::Foundation::NTSTATUS {
    windows_targets::link!("fwpuclnt.dll" "system" fn FwpmProviderContextDestroyEnumHandle0(enginehandle : super::super::super::Win32::Foundation:: HANDLE, enumhandle : super::super::super::Win32::Foundation:: HANDLE) -> super::super::super::Win32::Foundation:: NTSTATUS);
    FwpmProviderContextDestroyEnumHandle0(core::mem::transmute(enginehandle), core::mem::transmute(enumhandle))
}
#[cfg(all(feature = "Win32_NetworkManagement_WindowsFilteringPlatform", feature = "Win32_Security"))]
#[inline]
pub unsafe fn FwpmProviderContextEnum0(enginehandle: super::super::super::Win32::Foundation::HANDLE, enumhandle: super::super::super::Win32::Foundation::HANDLE, numentriesrequested: u32, entries: *mut *mut *mut super::super::super::Win32::NetworkManagement::WindowsFilteringPlatform::FWPM_PROVIDER_CONTEXT0, numentriesreturned: *mut u32) -> super::super::super::Win32::Foundation::NTSTATUS {
    windows_targets::link!("fwpuclnt.dll" "system" fn FwpmProviderContextEnum0(enginehandle : super::super::super::Win32::Foundation:: HANDLE, enumhandle : super::super::super::Win32::Foundation:: HANDLE, numentriesrequested : u32, entries : *mut *mut *mut super::super::super::Win32::NetworkManagement::WindowsFilteringPlatform:: FWPM_PROVIDER_CONTEXT0, numentriesreturned : *mut u32) -> super::super::super::Win32::Foundation:: NTSTATUS);
    FwpmProviderContextEnum0(core::mem::transmute(enginehandle), core::mem::transmute(enumhandle), core::mem::transmute(numentriesrequested), core::mem::transmute(entries), core::mem::transmute(numentriesreturned))
}
#[cfg(all(feature = "Win32_NetworkManagement_WindowsFilteringPlatform", feature = "Win32_Security"))]
#[inline]
pub unsafe fn FwpmProviderContextEnum1(enginehandle: super::super::super::Win32::Foundation::HANDLE, enumhandle: super::super::super::Win32::Foundation::HANDLE, numentriesrequested: u32, entries: *mut *mut *mut super::super::super::Win32::NetworkManagement::WindowsFilteringPlatform::FWPM_PROVIDER_CONTEXT1, numentriesreturned: *mut u32) -> super::super::super::Win32::Foundation::NTSTATUS {
    windows_targets::link!("fwpuclnt.dll" "system" fn FwpmProviderContextEnum1(enginehandle : super::super::super::Win32::Foundation:: HANDLE, enumhandle : super::super::super::Win32::Foundation:: HANDLE, numentriesrequested : u32, entries : *mut *mut *mut super::super::super::Win32::NetworkManagement::WindowsFilteringPlatform:: FWPM_PROVIDER_CONTEXT1, numentriesreturned : *mut u32) -> super::super::super::Win32::Foundation:: NTSTATUS);
    FwpmProviderContextEnum1(core::mem::transmute(enginehandle), core::mem::transmute(enumhandle), core::mem::transmute(numentriesrequested), core::mem::transmute(entries), core::mem::transmute(numentriesreturned))
}
#[cfg(all(feature = "Win32_NetworkManagement_WindowsFilteringPlatform", feature = "Win32_Security"))]
#[inline]
pub unsafe fn FwpmProviderContextEnum2(enginehandle: super::super::super::Win32::Foundation::HANDLE, enumhandle: super::super::super::Win32::Foundation::HANDLE, numentriesrequested: u32, entries: *mut *mut *mut super::super::super::Win32::NetworkManagement::WindowsFilteringPlatform::FWPM_PROVIDER_CONTEXT2, numentriesreturned: *mut u32) -> super::super::super::Win32::Foundation::NTSTATUS {
    windows_targets::link!("fwpuclnt.dll" "system" fn FwpmProviderContextEnum2(enginehandle : super::super::super::Win32::Foundation:: HANDLE, enumhandle : super::super::super::Win32::Foundation:: HANDLE, numentriesrequested : u32, entries : *mut *mut *mut super::super::super::Win32::NetworkManagement::WindowsFilteringPlatform:: FWPM_PROVIDER_CONTEXT2, numentriesreturned : *mut u32) -> super::super::super::Win32::Foundation:: NTSTATUS);
    FwpmProviderContextEnum2(core::mem::transmute(enginehandle), core::mem::transmute(enumhandle), core::mem::transmute(numentriesrequested), core::mem::transmute(entries), core::mem::transmute(numentriesreturned))
}
#[cfg(all(feature = "Win32_NetworkManagement_WindowsFilteringPlatform", feature = "Win32_Security"))]
#[inline]
pub unsafe fn FwpmProviderContextEnum3(enginehandle: super::super::super::Win32::Foundation::HANDLE, enumhandle: super::super::super::Win32::Foundation::HANDLE, numentriesrequested: u32, entries: *mut *mut *mut super::super::super::Win32::NetworkManagement::WindowsFilteringPlatform::FWPM_PROVIDER_CONTEXT3, numentriesreturned: *mut u32) -> super::super::super::Win32::Foundation::NTSTATUS {
    windows_targets::link!("fwpuclnt.dll" "system" fn FwpmProviderContextEnum3(enginehandle : super::super::super::Win32::Foundation:: HANDLE, enumhandle : super::super::super::Win32::Foundation:: HANDLE, numentriesrequested : u32, entries : *mut *mut *mut super::super::super::Win32::NetworkManagement::WindowsFilteringPlatform:: FWPM_PROVIDER_CONTEXT3, numentriesreturned : *mut u32) -> super::super::super::Win32::Foundation:: NTSTATUS);
    FwpmProviderContextEnum3(core::mem::transmute(enginehandle), core::mem::transmute(enumhandle), core::mem::transmute(numentriesrequested), core::mem::transmute(entries), core::mem::transmute(numentriesreturned))
}
#[cfg(all(feature = "Win32_NetworkManagement_WindowsFilteringPlatform", feature = "Win32_Security"))]
#[inline]
pub unsafe fn FwpmProviderContextGetById0(enginehandle: super::super::super::Win32::Foundation::HANDLE, id: u64, providercontext: *mut *mut super::super::super::Win32::NetworkManagement::WindowsFilteringPlatform::FWPM_PROVIDER_CONTEXT0) -> super::super::super::Win32::Foundation::NTSTATUS {
    windows_targets::link!("fwpuclnt.dll" "system" fn FwpmProviderContextGetById0(enginehandle : super::super::super::Win32::Foundation:: HANDLE, id : u64, providercontext : *mut *mut super::super::super::Win32::NetworkManagement::WindowsFilteringPlatform:: FWPM_PROVIDER_CONTEXT0) -> super::super::super::Win32::Foundation:: NTSTATUS);
    FwpmProviderContextGetById0(core::mem::transmute(enginehandle), core::mem::transmute(id), core::mem::transmute(providercontext))
}
#[cfg(all(feature = "Win32_NetworkManagement_WindowsFilteringPlatform", feature = "Win32_Security"))]
#[inline]
pub unsafe fn FwpmProviderContextGetById1(enginehandle: super::super::super::Win32::Foundation::HANDLE, id: u64, providercontext: *mut *mut super::super::super::Win32::NetworkManagement::WindowsFilteringPlatform::FWPM_PROVIDER_CONTEXT1) -> super::super::super::Win32::Foundation::NTSTATUS {
    windows_targets::link!("fwpuclnt.dll" "system" fn FwpmProviderContextGetById1(enginehandle : super::super::super::Win32::Foundation:: HANDLE, id : u64, providercontext : *mut *mut super::super::super::Win32::NetworkManagement::WindowsFilteringPlatform:: FWPM_PROVIDER_CONTEXT1) -> super::super::super::Win32::Foundation:: NTSTATUS);
    FwpmProviderContextGetById1(core::mem::transmute(enginehandle), core::mem::transmute(id), core::mem::transmute(providercontext))
}
#[cfg(all(feature = "Win32_NetworkManagement_WindowsFilteringPlatform", feature = "Win32_Security"))]
#[inline]
pub unsafe fn FwpmProviderContextGetById2(enginehandle: super::super::super::Win32::Foundation::HANDLE, id: u64, providercontext: *mut *mut super::super::super::Win32::NetworkManagement::WindowsFilteringPlatform::FWPM_PROVIDER_CONTEXT2) -> super::super::super::Win32::Foundation::NTSTATUS {
    windows_targets::link!("fwpuclnt.dll" "system" fn FwpmProviderContextGetById2(enginehandle : super::super::super::Win32::Foundation:: HANDLE, id : u64, providercontext : *mut *mut super::super::super::Win32::NetworkManagement::WindowsFilteringPlatform:: FWPM_PROVIDER_CONTEXT2) -> super::super::super::Win32::Foundation:: NTSTATUS);
    FwpmProviderContextGetById2(core::mem::transmute(enginehandle), core::mem::transmute(id), core::mem::transmute(providercontext))
}
#[cfg(all(feature = "Win32_NetworkManagement_WindowsFilteringPlatform", feature = "Win32_Security"))]
#[inline]
pub unsafe fn FwpmProviderContextGetById3(enginehandle: super::super::super::Win32::Foundation::HANDLE, id: u64, providercontext: *mut *mut super::super::super::Win32::NetworkManagement::WindowsFilteringPlatform::FWPM_PROVIDER_CONTEXT3) -> super::super::super::Win32::Foundation::NTSTATUS {
    windows_targets::link!("fwpuclnt.dll" "system" fn FwpmProviderContextGetById3(enginehandle : super::super::super::Win32::Foundation:: HANDLE, id : u64, providercontext : *mut *mut super::super::super::Win32::NetworkManagement::WindowsFilteringPlatform:: FWPM_PROVIDER_CONTEXT3) -> super::super::super::Win32::Foundation:: NTSTATUS);
    FwpmProviderContextGetById3(core::mem::transmute(enginehandle), core::mem::transmute(id), core::mem::transmute(providercontext))
}
#[cfg(all(feature = "Win32_NetworkManagement_WindowsFilteringPlatform", feature = "Win32_Security"))]
#[inline]
pub unsafe fn FwpmProviderContextGetByKey0(enginehandle: super::super::super::Win32::Foundation::HANDLE, key: *const windows_core::GUID, providercontext: *mut *mut super::super::super::Win32::NetworkManagement::WindowsFilteringPlatform::FWPM_PROVIDER_CONTEXT0) -> super::super::super::Win32::Foundation::NTSTATUS {
    windows_targets::link!("fwpuclnt.dll" "system" fn FwpmProviderContextGetByKey0(enginehandle : super::super::super::Win32::Foundation:: HANDLE, key : *const windows_core::GUID, providercontext : *mut *mut super::super::super::Win32::NetworkManagement::WindowsFilteringPlatform:: FWPM_PROVIDER_CONTEXT0) -> super::super::super::Win32::Foundation:: NTSTATUS);
    FwpmProviderContextGetByKey0(core::mem::transmute(enginehandle), core::mem::transmute(key), core::mem::transmute(providercontext))
}
#[cfg(all(feature = "Win32_NetworkManagement_WindowsFilteringPlatform", feature = "Win32_Security"))]
#[inline]
pub unsafe fn FwpmProviderContextGetByKey1(enginehandle: super::super::super::Win32::Foundation::HANDLE, key: *const windows_core::GUID, providercontext: *mut *mut super::super::super::Win32::NetworkManagement::WindowsFilteringPlatform::FWPM_PROVIDER_CONTEXT1) -> super::super::super::Win32::Foundation::NTSTATUS {
    windows_targets::link!("fwpuclnt.dll" "system" fn FwpmProviderContextGetByKey1(enginehandle : super::super::super::Win32::Foundation:: HANDLE, key : *const windows_core::GUID, providercontext : *mut *mut super::super::super::Win32::NetworkManagement::WindowsFilteringPlatform:: FWPM_PROVIDER_CONTEXT1) -> super::super::super::Win32::Foundation:: NTSTATUS);
    FwpmProviderContextGetByKey1(core::mem::transmute(enginehandle), core::mem::transmute(key), core::mem::transmute(providercontext))
}
#[cfg(all(feature = "Win32_NetworkManagement_WindowsFilteringPlatform", feature = "Win32_Security"))]
#[inline]
pub unsafe fn FwpmProviderContextGetByKey2(enginehandle: super::super::super::Win32::Foundation::HANDLE, key: *const windows_core::GUID, providercontext: *mut *mut super::super::super::Win32::NetworkManagement::WindowsFilteringPlatform::FWPM_PROVIDER_CONTEXT2) -> super::super::super::Win32::Foundation::NTSTATUS {
    windows_targets::link!("fwpuclnt.dll" "system" fn FwpmProviderContextGetByKey2(enginehandle : super::super::super::Win32::Foundation:: HANDLE, key : *const windows_core::GUID, providercontext : *mut *mut super::super::super::Win32::NetworkManagement::WindowsFilteringPlatform:: FWPM_PROVIDER_CONTEXT2) -> super::super::super::Win32::Foundation:: NTSTATUS);
    FwpmProviderContextGetByKey2(core::mem::transmute(enginehandle), core::mem::transmute(key), core::mem::transmute(providercontext))
}
#[cfg(all(feature = "Win32_NetworkManagement_WindowsFilteringPlatform", feature = "Win32_Security"))]
#[inline]
pub unsafe fn FwpmProviderContextGetByKey3(enginehandle: super::super::super::Win32::Foundation::HANDLE, key: *const windows_core::GUID, providercontext: *mut *mut super::super::super::Win32::NetworkManagement::WindowsFilteringPlatform::FWPM_PROVIDER_CONTEXT3) -> super::super::super::Win32::Foundation::NTSTATUS {
    windows_targets::link!("fwpuclnt.dll" "system" fn FwpmProviderContextGetByKey3(enginehandle : super::super::super::Win32::Foundation:: HANDLE, key : *const windows_core::GUID, providercontext : *mut *mut super::super::super::Win32::NetworkManagement::WindowsFilteringPlatform:: FWPM_PROVIDER_CONTEXT3) -> super::super::super::Win32::Foundation:: NTSTATUS);
    FwpmProviderContextGetByKey3(core::mem::transmute(enginehandle), core::mem::transmute(key), core::mem::transmute(providercontext))
}
#[cfg(feature = "Win32_Security")]
#[inline]
pub unsafe fn FwpmProviderContextGetSecurityInfoByKey0(enginehandle: super::super::super::Win32::Foundation::HANDLE, key: Option<*const windows_core::GUID>, securityinfo: u32, sidowner: *mut super::super::super::Win32::Security::PSID, sidgroup: *mut super::super::super::Win32::Security::PSID, dacl: *mut *mut super::super::super::Win32::Security::ACL, sacl: *mut *mut super::super::super::Win32::Security::ACL, securitydescriptor: *mut super::super::super::Win32::Security::PSECURITY_DESCRIPTOR) -> super::super::super::Win32::Foundation::NTSTATUS {
    windows_targets::link!("fwpuclnt.dll" "system" fn FwpmProviderContextGetSecurityInfoByKey0(enginehandle : super::super::super::Win32::Foundation:: HANDLE, key : *const windows_core::GUID, securityinfo : u32, sidowner : *mut super::super::super::Win32::Security:: PSID, sidgroup : *mut super::super::super::Win32::Security:: PSID, dacl : *mut *mut super::super::super::Win32::Security:: ACL, sacl : *mut *mut super::super::super::Win32::Security:: ACL, securitydescriptor : *mut super::super::super::Win32::Security:: PSECURITY_DESCRIPTOR) -> super::super::super::Win32::Foundation:: NTSTATUS);
    FwpmProviderContextGetSecurityInfoByKey0(core::mem::transmute(enginehandle), core::mem::transmute(key.unwrap_or(core::mem::zeroed())), core::mem::transmute(securityinfo), core::mem::transmute(sidowner), core::mem::transmute(sidgroup), core::mem::transmute(dacl), core::mem::transmute(sacl), core::mem::transmute(securitydescriptor))
}
#[cfg(feature = "Win32_Security")]
#[inline]
pub unsafe fn FwpmProviderContextSetSecurityInfoByKey0(enginehandle: super::super::super::Win32::Foundation::HANDLE, key: Option<*const windows_core::GUID>, securityinfo: u32, sidowner: Option<*const super::super::super::Win32::Security::SID>, sidgroup: Option<*const super::super::super::Win32::Security::SID>, dacl: Option<*const super::super::super::Win32::Security::ACL>, sacl: Option<*const super::super::super::Win32::Security::ACL>) -> super::super::super::Win32::Foundation::NTSTATUS {
    windows_targets::link!("fwpuclnt.dll" "system" fn FwpmProviderContextSetSecurityInfoByKey0(enginehandle : super::super::super::Win32::Foundation:: HANDLE, key : *const windows_core::GUID, securityinfo : u32, sidowner : *const super::super::super::Win32::Security:: SID, sidgroup : *const super::super::super::Win32::Security:: SID, dacl : *const super::super::super::Win32::Security:: ACL, sacl : *const super::super::super::Win32::Security:: ACL) -> super::super::super::Win32::Foundation:: NTSTATUS);
    FwpmProviderContextSetSecurityInfoByKey0(core::mem::transmute(enginehandle), core::mem::transmute(key.unwrap_or(core::mem::zeroed())), core::mem::transmute(securityinfo), core::mem::transmute(sidowner.unwrap_or(core::mem::zeroed())), core::mem::transmute(sidgroup.unwrap_or(core::mem::zeroed())), core::mem::transmute(dacl.unwrap_or(core::mem::zeroed())), core::mem::transmute(sacl.unwrap_or(core::mem::zeroed())))
}
#[cfg(feature = "Win32_NetworkManagement_WindowsFilteringPlatform")]
#[inline]
pub unsafe fn FwpmProviderCreateEnumHandle0(enginehandle: super::super::super::Win32::Foundation::HANDLE, enumtemplate: Option<*const super::super::super::Win32::NetworkManagement::WindowsFilteringPlatform::FWPM_PROVIDER_ENUM_TEMPLATE0>, enumhandle: *mut super::super::super::Win32::Foundation::HANDLE) -> super::super::super::Win32::Foundation::NTSTATUS {
    windows_targets::link!("fwpuclnt.dll" "system" fn FwpmProviderCreateEnumHandle0(enginehandle : super::super::super::Win32::Foundation:: HANDLE, enumtemplate : *const super::super::super::Win32::NetworkManagement::WindowsFilteringPlatform:: FWPM_PROVIDER_ENUM_TEMPLATE0, enumhandle : *mut super::super::super::Win32::Foundation:: HANDLE) -> super::super::super::Win32::Foundation:: NTSTATUS);
    FwpmProviderCreateEnumHandle0(core::mem::transmute(enginehandle), core::mem::transmute(enumtemplate.unwrap_or(core::mem::zeroed())), core::mem::transmute(enumhandle))
}
#[inline]
pub unsafe fn FwpmProviderDeleteByKey0(enginehandle: super::super::super::Win32::Foundation::HANDLE, key: *const windows_core::GUID) -> super::super::super::Win32::Foundation::NTSTATUS {
    windows_targets::link!("fwpuclnt.dll" "system" fn FwpmProviderDeleteByKey0(enginehandle : super::super::super::Win32::Foundation:: HANDLE, key : *const windows_core::GUID) -> super::super::super::Win32::Foundation:: NTSTATUS);
    FwpmProviderDeleteByKey0(core::mem::transmute(enginehandle), core::mem::transmute(key))
}
#[inline]
pub unsafe fn FwpmProviderDestroyEnumHandle0(enginehandle: super::super::super::Win32::Foundation::HANDLE, enumhandle: super::super::super::Win32::Foundation::HANDLE) -> super::super::super::Win32::Foundation::NTSTATUS {
    windows_targets::link!("fwpuclnt.dll" "system" fn FwpmProviderDestroyEnumHandle0(enginehandle : super::super::super::Win32::Foundation:: HANDLE, enumhandle : super::super::super::Win32::Foundation:: HANDLE) -> super::super::super::Win32::Foundation:: NTSTATUS);
    FwpmProviderDestroyEnumHandle0(core::mem::transmute(enginehandle), core::mem::transmute(enumhandle))
}
#[cfg(feature = "Win32_NetworkManagement_WindowsFilteringPlatform")]
#[inline]
pub unsafe fn FwpmProviderEnum0(enginehandle: super::super::super::Win32::Foundation::HANDLE, enumhandle: super::super::super::Win32::Foundation::HANDLE, numentriesrequested: u32, entries: *mut *mut *mut super::super::super::Win32::NetworkManagement::WindowsFilteringPlatform::FWPM_PROVIDER0, numentriesreturned: *mut u32) -> super::super::super::Win32::Foundation::NTSTATUS {
    windows_targets::link!("fwpuclnt.dll" "system" fn FwpmProviderEnum0(enginehandle : super::super::super::Win32::Foundation:: HANDLE, enumhandle : super::super::super::Win32::Foundation:: HANDLE, numentriesrequested : u32, entries : *mut *mut *mut super::super::super::Win32::NetworkManagement::WindowsFilteringPlatform:: FWPM_PROVIDER0, numentriesreturned : *mut u32) -> super::super::super::Win32::Foundation:: NTSTATUS);
    FwpmProviderEnum0(core::mem::transmute(enginehandle), core::mem::transmute(enumhandle), core::mem::transmute(numentriesrequested), core::mem::transmute(entries), core::mem::transmute(numentriesreturned))
}
#[cfg(feature = "Win32_NetworkManagement_WindowsFilteringPlatform")]
#[inline]
pub unsafe fn FwpmProviderGetByKey0(enginehandle: super::super::super::Win32::Foundation::HANDLE, key: *const windows_core::GUID, provider: *mut *mut super::super::super::Win32::NetworkManagement::WindowsFilteringPlatform::FWPM_PROVIDER0) -> super::super::super::Win32::Foundation::NTSTATUS {
    windows_targets::link!("fwpuclnt.dll" "system" fn FwpmProviderGetByKey0(enginehandle : super::super::super::Win32::Foundation:: HANDLE, key : *const windows_core::GUID, provider : *mut *mut super::super::super::Win32::NetworkManagement::WindowsFilteringPlatform:: FWPM_PROVIDER0) -> super::super::super::Win32::Foundation:: NTSTATUS);
    FwpmProviderGetByKey0(core::mem::transmute(enginehandle), core::mem::transmute(key), core::mem::transmute(provider))
}
#[cfg(feature = "Win32_Security")]
#[inline]
pub unsafe fn FwpmProviderGetSecurityInfoByKey0(enginehandle: super::super::super::Win32::Foundation::HANDLE, key: Option<*const windows_core::GUID>, securityinfo: u32, sidowner: *mut super::super::super::Win32::Security::PSID, sidgroup: *mut super::super::super::Win32::Security::PSID, dacl: *mut *mut super::super::super::Win32::Security::ACL, sacl: *mut *mut super::super::super::Win32::Security::ACL, securitydescriptor: *mut super::super::super::Win32::Security::PSECURITY_DESCRIPTOR) -> super::super::super::Win32::Foundation::NTSTATUS {
    windows_targets::link!("fwpuclnt.dll" "system" fn FwpmProviderGetSecurityInfoByKey0(enginehandle : super::super::super::Win32::Foundation:: HANDLE, key : *const windows_core::GUID, securityinfo : u32, sidowner : *mut super::super::super::Win32::Security:: PSID, sidgroup : *mut super::super::super::Win32::Security:: PSID, dacl : *mut *mut super::super::super::Win32::Security:: ACL, sacl : *mut *mut super::super::super::Win32::Security:: ACL, securitydescriptor : *mut super::super::super::Win32::Security:: PSECURITY_DESCRIPTOR) -> super::super::super::Win32::Foundation:: NTSTATUS);
    FwpmProviderGetSecurityInfoByKey0(core::mem::transmute(enginehandle), core::mem::transmute(key.unwrap_or(core::mem::zeroed())), core::mem::transmute(securityinfo), core::mem::transmute(sidowner), core::mem::transmute(sidgroup), core::mem::transmute(dacl), core::mem::transmute(sacl), core::mem::transmute(securitydescriptor))
}
#[cfg(feature = "Win32_Security")]
#[inline]
pub unsafe fn FwpmProviderSetSecurityInfoByKey0(enginehandle: super::super::super::Win32::Foundation::HANDLE, key: Option<*const windows_core::GUID>, securityinfo: u32, sidowner: Option<*const super::super::super::Win32::Security::SID>, sidgroup: Option<*const super::super::super::Win32::Security::SID>, dacl: Option<*const super::super::super::Win32::Security::ACL>, sacl: Option<*const super::super::super::Win32::Security::ACL>) -> super::super::super::Win32::Foundation::NTSTATUS {
    windows_targets::link!("fwpuclnt.dll" "system" fn FwpmProviderSetSecurityInfoByKey0(enginehandle : super::super::super::Win32::Foundation:: HANDLE, key : *const windows_core::GUID, securityinfo : u32, sidowner : *const super::super::super::Win32::Security:: SID, sidgroup : *const super::super::super::Win32::Security:: SID, dacl : *const super::super::super::Win32::Security:: ACL, sacl : *const super::super::super::Win32::Security:: ACL) -> super::super::super::Win32::Foundation:: NTSTATUS);
    FwpmProviderSetSecurityInfoByKey0(core::mem::transmute(enginehandle), core::mem::transmute(key.unwrap_or(core::mem::zeroed())), core::mem::transmute(securityinfo), core::mem::transmute(sidowner.unwrap_or(core::mem::zeroed())), core::mem::transmute(sidgroup.unwrap_or(core::mem::zeroed())), core::mem::transmute(dacl.unwrap_or(core::mem::zeroed())), core::mem::transmute(sacl.unwrap_or(core::mem::zeroed())))
}
#[cfg(feature = "Win32_NetworkManagement_WindowsFilteringPlatform")]
#[inline]
pub unsafe fn FwpmSessionCreateEnumHandle0(enginehandle: super::super::super::Win32::Foundation::HANDLE, enumtemplate: Option<*const super::super::super::Win32::NetworkManagement::WindowsFilteringPlatform::FWPM_SESSION_ENUM_TEMPLATE0>, enumhandle: *mut super::super::super::Win32::Foundation::HANDLE) -> super::super::super::Win32::Foundation::NTSTATUS {
    windows_targets::link!("fwpuclnt.dll" "system" fn FwpmSessionCreateEnumHandle0(enginehandle : super::super::super::Win32::Foundation:: HANDLE, enumtemplate : *const super::super::super::Win32::NetworkManagement::WindowsFilteringPlatform:: FWPM_SESSION_ENUM_TEMPLATE0, enumhandle : *mut super::super::super::Win32::Foundation:: HANDLE) -> super::super::super::Win32::Foundation:: NTSTATUS);
    FwpmSessionCreateEnumHandle0(core::mem::transmute(enginehandle), core::mem::transmute(enumtemplate.unwrap_or(core::mem::zeroed())), core::mem::transmute(enumhandle))
}
#[inline]
pub unsafe fn FwpmSessionDestroyEnumHandle0(enginehandle: super::super::super::Win32::Foundation::HANDLE, enumhandle: super::super::super::Win32::Foundation::HANDLE) -> super::super::super::Win32::Foundation::NTSTATUS {
    windows_targets::link!("fwpuclnt.dll" "system" fn FwpmSessionDestroyEnumHandle0(enginehandle : super::super::super::Win32::Foundation:: HANDLE, enumhandle : super::super::super::Win32::Foundation:: HANDLE) -> super::super::super::Win32::Foundation:: NTSTATUS);
    FwpmSessionDestroyEnumHandle0(core::mem::transmute(enginehandle), core::mem::transmute(enumhandle))
}
#[cfg(all(feature = "Win32_NetworkManagement_WindowsFilteringPlatform", feature = "Win32_Security"))]
#[inline]
pub unsafe fn FwpmSessionEnum0(enginehandle: super::super::super::Win32::Foundation::HANDLE, enumhandle: super::super::super::Win32::Foundation::HANDLE, numentriesrequested: u32, entries: *mut *mut *mut super::super::super::Win32::NetworkManagement::WindowsFilteringPlatform::FWPM_SESSION0, numentriesreturned: *mut u32) -> super::super::super::Win32::Foundation::NTSTATUS {
    windows_targets::link!("fwpuclnt.dll" "system" fn FwpmSessionEnum0(enginehandle : super::super::super::Win32::Foundation:: HANDLE, enumhandle : super::super::super::Win32::Foundation:: HANDLE, numentriesrequested : u32, entries : *mut *mut *mut super::super::super::Win32::NetworkManagement::WindowsFilteringPlatform:: FWPM_SESSION0, numentriesreturned : *mut u32) -> super::super::super::Win32::Foundation:: NTSTATUS);
    FwpmSessionEnum0(core::mem::transmute(enginehandle), core::mem::transmute(enumhandle), core::mem::transmute(numentriesrequested), core::mem::transmute(entries), core::mem::transmute(numentriesreturned))
}
#[cfg(all(feature = "Win32_NetworkManagement_WindowsFilteringPlatform", feature = "Win32_Security"))]
#[inline]
pub unsafe fn FwpmSubLayerAdd0(enginehandle: super::super::super::Win32::Foundation::HANDLE, sublayer: *const super::super::super::Win32::NetworkManagement::WindowsFilteringPlatform::FWPM_SUBLAYER0, sd: Option<super::super::super::Win32::Security::PSECURITY_DESCRIPTOR>) -> super::super::super::Win32::Foundation::NTSTATUS {
    windows_targets::link!("fwpuclnt.dll" "system" fn FwpmSubLayerAdd0(enginehandle : super::super::super::Win32::Foundation:: HANDLE, sublayer : *const super::super::super::Win32::NetworkManagement::WindowsFilteringPlatform:: FWPM_SUBLAYER0, sd : super::super::super::Win32::Security:: PSECURITY_DESCRIPTOR) -> super::super::super::Win32::Foundation:: NTSTATUS);
    FwpmSubLayerAdd0(core::mem::transmute(enginehandle), core::mem::transmute(sublayer), core::mem::transmute(sd.unwrap_or(core::mem::zeroed())))
}
#[cfg(feature = "Win32_NetworkManagement_WindowsFilteringPlatform")]
#[inline]
pub unsafe fn FwpmSubLayerCreateEnumHandle0(enginehandle: super::super::super::Win32::Foundation::HANDLE, enumtemplate: Option<*const super::super::super::Win32::NetworkManagement::WindowsFilteringPlatform::FWPM_SUBLAYER_ENUM_TEMPLATE0>, enumhandle: *mut super::super::super::Win32::Foundation::HANDLE) -> super::super::super::Win32::Foundation::NTSTATUS {
    windows_targets::link!("fwpuclnt.dll" "system" fn FwpmSubLayerCreateEnumHandle0(enginehandle : super::super::super::Win32::Foundation:: HANDLE, enumtemplate : *const super::super::super::Win32::NetworkManagement::WindowsFilteringPlatform:: FWPM_SUBLAYER_ENUM_TEMPLATE0, enumhandle : *mut super::super::super::Win32::Foundation:: HANDLE) -> super::super::super::Win32::Foundation:: NTSTATUS);
    FwpmSubLayerCreateEnumHandle0(core::mem::transmute(enginehandle), core::mem::transmute(enumtemplate.unwrap_or(core::mem::zeroed())), core::mem::transmute(enumhandle))
}
#[inline]
pub unsafe fn FwpmSubLayerDeleteByKey0(enginehandle: super::super::super::Win32::Foundation::HANDLE, key: *const windows_core::GUID) -> super::super::super::Win32::Foundation::NTSTATUS {
    windows_targets::link!("fwpuclnt.dll" "system" fn FwpmSubLayerDeleteByKey0(enginehandle : super::super::super::Win32::Foundation:: HANDLE, key : *const windows_core::GUID) -> super::super::super::Win32::Foundation:: NTSTATUS);
    FwpmSubLayerDeleteByKey0(core::mem::transmute(enginehandle), core::mem::transmute(key))
}
#[inline]
pub unsafe fn FwpmSubLayerDestroyEnumHandle0(enginehandle: super::super::super::Win32::Foundation::HANDLE, enumhandle: super::super::super::Win32::Foundation::HANDLE) -> super::super::super::Win32::Foundation::NTSTATUS {
    windows_targets::link!("fwpuclnt.dll" "system" fn FwpmSubLayerDestroyEnumHandle0(enginehandle : super::super::super::Win32::Foundation:: HANDLE, enumhandle : super::super::super::Win32::Foundation:: HANDLE) -> super::super::super::Win32::Foundation:: NTSTATUS);
    FwpmSubLayerDestroyEnumHandle0(core::mem::transmute(enginehandle), core::mem::transmute(enumhandle))
}
#[cfg(feature = "Win32_NetworkManagement_WindowsFilteringPlatform")]
#[inline]
pub unsafe fn FwpmSubLayerEnum0(enginehandle: super::super::super::Win32::Foundation::HANDLE, enumhandle: super::super::super::Win32::Foundation::HANDLE, numentriesrequested: u32, entries: *mut *mut *mut super::super::super::Win32::NetworkManagement::WindowsFilteringPlatform::FWPM_SUBLAYER0, numentriesreturned: *mut u32) -> super::super::super::Win32::Foundation::NTSTATUS {
    windows_targets::link!("fwpuclnt.dll" "system" fn FwpmSubLayerEnum0(enginehandle : super::super::super::Win32::Foundation:: HANDLE, enumhandle : super::super::super::Win32::Foundation:: HANDLE, numentriesrequested : u32, entries : *mut *mut *mut super::super::super::Win32::NetworkManagement::WindowsFilteringPlatform:: FWPM_SUBLAYER0, numentriesreturned : *mut u32) -> super::super::super::Win32::Foundation:: NTSTATUS);
    FwpmSubLayerEnum0(core::mem::transmute(enginehandle), core::mem::transmute(enumhandle), core::mem::transmute(numentriesrequested), core::mem::transmute(entries), core::mem::transmute(numentriesreturned))
}
#[cfg(feature = "Win32_NetworkManagement_WindowsFilteringPlatform")]
#[inline]
pub unsafe fn FwpmSubLayerGetByKey0(enginehandle: super::super::super::Win32::Foundation::HANDLE, key: *const windows_core::GUID, sublayer: *mut *mut super::super::super::Win32::NetworkManagement::WindowsFilteringPlatform::FWPM_SUBLAYER0) -> super::super::super::Win32::Foundation::NTSTATUS {
    windows_targets::link!("fwpuclnt.dll" "system" fn FwpmSubLayerGetByKey0(enginehandle : super::super::super::Win32::Foundation:: HANDLE, key : *const windows_core::GUID, sublayer : *mut *mut super::super::super::Win32::NetworkManagement::WindowsFilteringPlatform:: FWPM_SUBLAYER0) -> super::super::super::Win32::Foundation:: NTSTATUS);
    FwpmSubLayerGetByKey0(core::mem::transmute(enginehandle), core::mem::transmute(key), core::mem::transmute(sublayer))
}
#[cfg(feature = "Win32_Security")]
#[inline]
pub unsafe fn FwpmSubLayerGetSecurityInfoByKey0(enginehandle: super::super::super::Win32::Foundation::HANDLE, key: Option<*const windows_core::GUID>, securityinfo: u32, sidowner: *mut super::super::super::Win32::Security::PSID, sidgroup: *mut super::super::super::Win32::Security::PSID, dacl: *mut *mut super::super::super::Win32::Security::ACL, sacl: *mut *mut super::super::super::Win32::Security::ACL, securitydescriptor: *mut super::super::super::Win32::Security::PSECURITY_DESCRIPTOR) -> super::super::super::Win32::Foundation::NTSTATUS {
    windows_targets::link!("fwpuclnt.dll" "system" fn FwpmSubLayerGetSecurityInfoByKey0(enginehandle : super::super::super::Win32::Foundation:: HANDLE, key : *const windows_core::GUID, securityinfo : u32, sidowner : *mut super::super::super::Win32::Security:: PSID, sidgroup : *mut super::super::super::Win32::Security:: PSID, dacl : *mut *mut super::super::super::Win32::Security:: ACL, sacl : *mut *mut super::super::super::Win32::Security:: ACL, securitydescriptor : *mut super::super::super::Win32::Security:: PSECURITY_DESCRIPTOR) -> super::super::super::Win32::Foundation:: NTSTATUS);
    FwpmSubLayerGetSecurityInfoByKey0(core::mem::transmute(enginehandle), core::mem::transmute(key.unwrap_or(core::mem::zeroed())), core::mem::transmute(securityinfo), core::mem::transmute(sidowner), core::mem::transmute(sidgroup), core::mem::transmute(dacl), core::mem::transmute(sacl), core::mem::transmute(securitydescriptor))
}
#[cfg(feature = "Win32_Security")]
#[inline]
pub unsafe fn FwpmSubLayerSetSecurityInfoByKey0(enginehandle: super::super::super::Win32::Foundation::HANDLE, key: Option<*const windows_core::GUID>, securityinfo: u32, sidowner: Option<*const super::super::super::Win32::Security::SID>, sidgroup: Option<*const super::super::super::Win32::Security::SID>, dacl: Option<*const super::super::super::Win32::Security::ACL>, sacl: Option<*const super::super::super::Win32::Security::ACL>) -> super::super::super::Win32::Foundation::NTSTATUS {
    windows_targets::link!("fwpuclnt.dll" "system" fn FwpmSubLayerSetSecurityInfoByKey0(enginehandle : super::super::super::Win32::Foundation:: HANDLE, key : *const windows_core::GUID, securityinfo : u32, sidowner : *const super::super::super::Win32::Security:: SID, sidgroup : *const super::super::super::Win32::Security:: SID, dacl : *const super::super::super::Win32::Security:: ACL, sacl : *const super::super::super::Win32::Security:: ACL) -> super::super::super::Win32::Foundation:: NTSTATUS);
    FwpmSubLayerSetSecurityInfoByKey0(core::mem::transmute(enginehandle), core::mem::transmute(key.unwrap_or(core::mem::zeroed())), core::mem::transmute(securityinfo), core::mem::transmute(sidowner.unwrap_or(core::mem::zeroed())), core::mem::transmute(sidgroup.unwrap_or(core::mem::zeroed())), core::mem::transmute(dacl.unwrap_or(core::mem::zeroed())), core::mem::transmute(sacl.unwrap_or(core::mem::zeroed())))
}
#[inline]
pub unsafe fn FwpmTransactionAbort0(enginehandle: super::super::super::Win32::Foundation::HANDLE) -> super::super::super::Win32::Foundation::NTSTATUS {
    windows_targets::link!("fwpuclnt.dll" "system" fn FwpmTransactionAbort0(enginehandle : super::super::super::Win32::Foundation:: HANDLE) -> super::super::super::Win32::Foundation:: NTSTATUS);
    FwpmTransactionAbort0(core::mem::transmute(enginehandle))
}
#[inline]
pub unsafe fn FwpmTransactionBegin0(enginehandle: super::super::super::Win32::Foundation::HANDLE, flags: u32) -> super::super::super::Win32::Foundation::NTSTATUS {
    windows_targets::link!("fwpuclnt.dll" "system" fn FwpmTransactionBegin0(enginehandle : super::super::super::Win32::Foundation:: HANDLE, flags : u32) -> super::super::super::Win32::Foundation:: NTSTATUS);
    FwpmTransactionBegin0(core::mem::transmute(enginehandle), core::mem::transmute(flags))
}
#[inline]
pub unsafe fn FwpmTransactionCommit0(enginehandle: super::super::super::Win32::Foundation::HANDLE) -> super::super::super::Win32::Foundation::NTSTATUS {
    windows_targets::link!("fwpuclnt.dll" "system" fn FwpmTransactionCommit0(enginehandle : super::super::super::Win32::Foundation:: HANDLE) -> super::super::super::Win32::Foundation:: NTSTATUS);
    FwpmTransactionCommit0(core::mem::transmute(enginehandle))
}
#[cfg(feature = "Win32_Security")]
#[inline]
pub unsafe fn FwpmvSwitchEventsGetSecurityInfo0(enginehandle: super::super::super::Win32::Foundation::HANDLE, securityinfo: u32, sidowner: *mut super::super::super::Win32::Security::PSID, sidgroup: *mut super::super::super::Win32::Security::PSID, dacl: *mut *mut super::super::super::Win32::Security::ACL, sacl: *mut *mut super::super::super::Win32::Security::ACL, securitydescriptor: *mut super::super::super::Win32::Security::PSECURITY_DESCRIPTOR) -> super::super::super::Win32::Foundation::NTSTATUS {
    windows_targets::link!("fwpuclnt.dll" "system" fn FwpmvSwitchEventsGetSecurityInfo0(enginehandle : super::super::super::Win32::Foundation:: HANDLE, securityinfo : u32, sidowner : *mut super::super::super::Win32::Security:: PSID, sidgroup : *mut super::super::super::Win32::Security:: PSID, dacl : *mut *mut super::super::super::Win32::Security:: ACL, sacl : *mut *mut super::super::super::Win32::Security:: ACL, securitydescriptor : *mut super::super::super::Win32::Security:: PSECURITY_DESCRIPTOR) -> super::super::super::Win32::Foundation:: NTSTATUS);
    FwpmvSwitchEventsGetSecurityInfo0(core::mem::transmute(enginehandle), core::mem::transmute(securityinfo), core::mem::transmute(sidowner), core::mem::transmute(sidgroup), core::mem::transmute(dacl), core::mem::transmute(sacl), core::mem::transmute(securitydescriptor))
}
#[cfg(feature = "Win32_Security")]
#[inline]
pub unsafe fn FwpmvSwitchEventsSetSecurityInfo0(enginehandle: super::super::super::Win32::Foundation::HANDLE, securityinfo: u32, sidowner: Option<*const super::super::super::Win32::Security::SID>, sidgroup: Option<*const super::super::super::Win32::Security::SID>, dacl: Option<*const super::super::super::Win32::Security::ACL>, sacl: Option<*const super::super::super::Win32::Security::ACL>) -> super::super::super::Win32::Foundation::NTSTATUS {
    windows_targets::link!("fwpuclnt.dll" "system" fn FwpmvSwitchEventsSetSecurityInfo0(enginehandle : super::super::super::Win32::Foundation:: HANDLE, securityinfo : u32, sidowner : *const super::super::super::Win32::Security:: SID, sidgroup : *const super::super::super::Win32::Security:: SID, dacl : *const super::super::super::Win32::Security:: ACL, sacl : *const super::super::super::Win32::Security:: ACL) -> super::super::super::Win32::Foundation:: NTSTATUS);
    FwpmvSwitchEventsSetSecurityInfo0(core::mem::transmute(enginehandle), core::mem::transmute(securityinfo), core::mem::transmute(sidowner.unwrap_or(core::mem::zeroed())), core::mem::transmute(sidgroup.unwrap_or(core::mem::zeroed())), core::mem::transmute(dacl.unwrap_or(core::mem::zeroed())), core::mem::transmute(sacl.unwrap_or(core::mem::zeroed())))
}
#[cfg(feature = "Win32_Security")]
#[inline]
pub unsafe fn IPsecDospGetSecurityInfo0(enginehandle: super::super::super::Win32::Foundation::HANDLE, securityinfo: u32, sidowner: *mut super::super::super::Win32::Security::PSID, sidgroup: *mut super::super::super::Win32::Security::PSID, dacl: *mut *mut super::super::super::Win32::Security::ACL, sacl: *mut *mut super::super::super::Win32::Security::ACL, securitydescriptor: *mut super::super::super::Win32::Security::PSECURITY_DESCRIPTOR) -> super::super::super::Win32::Foundation::NTSTATUS {
    windows_targets::link!("fwpuclnt.dll" "system" fn IPsecDospGetSecurityInfo0(enginehandle : super::super::super::Win32::Foundation:: HANDLE, securityinfo : u32, sidowner : *mut super::super::super::Win32::Security:: PSID, sidgroup : *mut super::super::super::Win32::Security:: PSID, dacl : *mut *mut super::super::super::Win32::Security:: ACL, sacl : *mut *mut super::super::super::Win32::Security:: ACL, securitydescriptor : *mut super::super::super::Win32::Security:: PSECURITY_DESCRIPTOR) -> super::super::super::Win32::Foundation:: NTSTATUS);
    IPsecDospGetSecurityInfo0(core::mem::transmute(enginehandle), core::mem::transmute(securityinfo), core::mem::transmute(sidowner), core::mem::transmute(sidgroup), core::mem::transmute(dacl), core::mem::transmute(sacl), core::mem::transmute(securitydescriptor))
}
#[cfg(feature = "Win32_NetworkManagement_WindowsFilteringPlatform")]
#[inline]
pub unsafe fn IPsecDospGetStatistics0(enginehandle: super::super::super::Win32::Foundation::HANDLE, idpstatistics: *mut super::super::super::Win32::NetworkManagement::WindowsFilteringPlatform::IPSEC_DOSP_STATISTICS0) -> super::super::super::Win32::Foundation::NTSTATUS {
    windows_targets::link!("fwpuclnt.dll" "system" fn IPsecDospGetStatistics0(enginehandle : super::super::super::Win32::Foundation:: HANDLE, idpstatistics : *mut super::super::super::Win32::NetworkManagement::WindowsFilteringPlatform:: IPSEC_DOSP_STATISTICS0) -> super::super::super::Win32::Foundation:: NTSTATUS);
    IPsecDospGetStatistics0(core::mem::transmute(enginehandle), core::mem::transmute(idpstatistics))
}
#[cfg(feature = "Win32_Security")]
#[inline]
pub unsafe fn IPsecDospSetSecurityInfo0(enginehandle: super::super::super::Win32::Foundation::HANDLE, securityinfo: u32, sidowner: Option<*const super::super::super::Win32::Security::SID>, sidgroup: Option<*const super::super::super::Win32::Security::SID>, dacl: Option<*const super::super::super::Win32::Security::ACL>, sacl: Option<*const super::super::super::Win32::Security::ACL>) -> super::super::super::Win32::Foundation::NTSTATUS {
    windows_targets::link!("fwpuclnt.dll" "system" fn IPsecDospSetSecurityInfo0(enginehandle : super::super::super::Win32::Foundation:: HANDLE, securityinfo : u32, sidowner : *const super::super::super::Win32::Security:: SID, sidgroup : *const super::super::super::Win32::Security:: SID, dacl : *const super::super::super::Win32::Security:: ACL, sacl : *const super::super::super::Win32::Security:: ACL) -> super::super::super::Win32::Foundation:: NTSTATUS);
    IPsecDospSetSecurityInfo0(core::mem::transmute(enginehandle), core::mem::transmute(securityinfo), core::mem::transmute(sidowner.unwrap_or(core::mem::zeroed())), core::mem::transmute(sidgroup.unwrap_or(core::mem::zeroed())), core::mem::transmute(dacl.unwrap_or(core::mem::zeroed())), core::mem::transmute(sacl.unwrap_or(core::mem::zeroed())))
}
#[cfg(feature = "Win32_NetworkManagement_WindowsFilteringPlatform")]
#[inline]
pub unsafe fn IPsecDospStateCreateEnumHandle0(enginehandle: super::super::super::Win32::Foundation::HANDLE, enumtemplate: Option<*const super::super::super::Win32::NetworkManagement::WindowsFilteringPlatform::IPSEC_DOSP_STATE_ENUM_TEMPLATE0>, enumhandle: *mut super::super::super::Win32::Foundation::HANDLE) -> super::super::super::Win32::Foundation::NTSTATUS {
    windows_targets::link!("fwpuclnt.dll" "system" fn IPsecDospStateCreateEnumHandle0(enginehandle : super::super::super::Win32::Foundation:: HANDLE, enumtemplate : *const super::super::super::Win32::NetworkManagement::WindowsFilteringPlatform:: IPSEC_DOSP_STATE_ENUM_TEMPLATE0, enumhandle : *mut super::super::super::Win32::Foundation:: HANDLE) -> super::super::super::Win32::Foundation:: NTSTATUS);
    IPsecDospStateCreateEnumHandle0(core::mem::transmute(enginehandle), core::mem::transmute(enumtemplate.unwrap_or(core::mem::zeroed())), core::mem::transmute(enumhandle))
}
#[inline]
pub unsafe fn IPsecDospStateDestroyEnumHandle0(enginehandle: super::super::super::Win32::Foundation::HANDLE, enumhandle: super::super::super::Win32::Foundation::HANDLE) -> super::super::super::Win32::Foundation::NTSTATUS {
    windows_targets::link!("fwpuclnt.dll" "system" fn IPsecDospStateDestroyEnumHandle0(enginehandle : super::super::super::Win32::Foundation:: HANDLE, enumhandle : super::super::super::Win32::Foundation:: HANDLE) -> super::super::super::Win32::Foundation:: NTSTATUS);
    IPsecDospStateDestroyEnumHandle0(core::mem::transmute(enginehandle), core::mem::transmute(enumhandle))
}
#[cfg(feature = "Win32_NetworkManagement_WindowsFilteringPlatform")]
#[inline]
pub unsafe fn IPsecDospStateEnum0(enginehandle: super::super::super::Win32::Foundation::HANDLE, enumhandle: super::super::super::Win32::Foundation::HANDLE, numentriesrequested: u32, entries: *mut *mut *mut super::super::super::Win32::NetworkManagement::WindowsFilteringPlatform::IPSEC_DOSP_STATE0, numentries: *mut u32) -> super::super::super::Win32::Foundation::NTSTATUS {
    windows_targets::link!("fwpuclnt.dll" "system" fn IPsecDospStateEnum0(enginehandle : super::super::super::Win32::Foundation:: HANDLE, enumhandle : super::super::super::Win32::Foundation:: HANDLE, numentriesrequested : u32, entries : *mut *mut *mut super::super::super::Win32::NetworkManagement::WindowsFilteringPlatform:: IPSEC_DOSP_STATE0, numentries : *mut u32) -> super::super::super::Win32::Foundation:: NTSTATUS);
    IPsecDospStateEnum0(core::mem::transmute(enginehandle), core::mem::transmute(enumhandle), core::mem::transmute(numentriesrequested), core::mem::transmute(entries), core::mem::transmute(numentries))
}
#[cfg(feature = "Win32_NetworkManagement_WindowsFilteringPlatform")]
#[inline]
pub unsafe fn IPsecGetStatistics0(enginehandle: super::super::super::Win32::Foundation::HANDLE, ipsecstatistics: *mut super::super::super::Win32::NetworkManagement::WindowsFilteringPlatform::IPSEC_STATISTICS0) -> super::super::super::Win32::Foundation::NTSTATUS {
    windows_targets::link!("fwpuclnt.dll" "system" fn IPsecGetStatistics0(enginehandle : super::super::super::Win32::Foundation:: HANDLE, ipsecstatistics : *mut super::super::super::Win32::NetworkManagement::WindowsFilteringPlatform:: IPSEC_STATISTICS0) -> super::super::super::Win32::Foundation:: NTSTATUS);
    IPsecGetStatistics0(core::mem::transmute(enginehandle), core::mem::transmute(ipsecstatistics))
}
#[cfg(feature = "Win32_NetworkManagement_WindowsFilteringPlatform")]
#[inline]
pub unsafe fn IPsecGetStatistics1(enginehandle: super::super::super::Win32::Foundation::HANDLE, ipsecstatistics: *mut super::super::super::Win32::NetworkManagement::WindowsFilteringPlatform::IPSEC_STATISTICS1) -> super::super::super::Win32::Foundation::NTSTATUS {
    windows_targets::link!("fwpuclnt.dll" "system" fn IPsecGetStatistics1(enginehandle : super::super::super::Win32::Foundation:: HANDLE, ipsecstatistics : *mut super::super::super::Win32::NetworkManagement::WindowsFilteringPlatform:: IPSEC_STATISTICS1) -> super::super::super::Win32::Foundation:: NTSTATUS);
    IPsecGetStatistics1(core::mem::transmute(enginehandle), core::mem::transmute(ipsecstatistics))
}
#[cfg(feature = "Win32_NetworkManagement_WindowsFilteringPlatform")]
#[inline]
pub unsafe fn IPsecSaContextAddInbound0(enginehandle: super::super::super::Win32::Foundation::HANDLE, id: u64, inboundbundle: *const super::super::super::Win32::NetworkManagement::WindowsFilteringPlatform::IPSEC_SA_BUNDLE0) -> super::super::super::Win32::Foundation::NTSTATUS {
    windows_targets::link!("fwpuclnt.dll" "system" fn IPsecSaContextAddInbound0(enginehandle : super::super::super::Win32::Foundation:: HANDLE, id : u64, inboundbundle : *const super::super::super::Win32::NetworkManagement::WindowsFilteringPlatform:: IPSEC_SA_BUNDLE0) -> super::super::super::Win32::Foundation:: NTSTATUS);
    IPsecSaContextAddInbound0(core::mem::transmute(enginehandle), core::mem::transmute(id), core::mem::transmute(inboundbundle))
}
#[cfg(feature = "Win32_NetworkManagement_WindowsFilteringPlatform")]
#[inline]
pub unsafe fn IPsecSaContextAddInbound1(enginehandle: super::super::super::Win32::Foundation::HANDLE, id: u64, inboundbundle: *const super::super::super::Win32::NetworkManagement::WindowsFilteringPlatform::IPSEC_SA_BUNDLE1) -> super::super::super::Win32::Foundation::NTSTATUS {
    windows_targets::link!("fwpuclnt.dll" "system" fn IPsecSaContextAddInbound1(enginehandle : super::super::super::Win32::Foundation:: HANDLE, id : u64, inboundbundle : *const super::super::super::Win32::NetworkManagement::WindowsFilteringPlatform:: IPSEC_SA_BUNDLE1) -> super::super::super::Win32::Foundation:: NTSTATUS);
    IPsecSaContextAddInbound1(core::mem::transmute(enginehandle), core::mem::transmute(id), core::mem::transmute(inboundbundle))
}
#[cfg(feature = "Win32_NetworkManagement_WindowsFilteringPlatform")]
#[inline]
pub unsafe fn IPsecSaContextAddOutbound0(enginehandle: super::super::super::Win32::Foundation::HANDLE, id: u64, outboundbundle: *const super::super::super::Win32::NetworkManagement::WindowsFilteringPlatform::IPSEC_SA_BUNDLE0) -> super::super::super::Win32::Foundation::NTSTATUS {
    windows_targets::link!("fwpuclnt.dll" "system" fn IPsecSaContextAddOutbound0(enginehandle : super::super::super::Win32::Foundation:: HANDLE, id : u64, outboundbundle : *const super::super::super::Win32::NetworkManagement::WindowsFilteringPlatform:: IPSEC_SA_BUNDLE0) -> super::super::super::Win32::Foundation:: NTSTATUS);
    IPsecSaContextAddOutbound0(core::mem::transmute(enginehandle), core::mem::transmute(id), core::mem::transmute(outboundbundle))
}
#[cfg(feature = "Win32_NetworkManagement_WindowsFilteringPlatform")]
#[inline]
pub unsafe fn IPsecSaContextAddOutbound1(enginehandle: super::super::super::Win32::Foundation::HANDLE, id: u64, outboundbundle: *const super::super::super::Win32::NetworkManagement::WindowsFilteringPlatform::IPSEC_SA_BUNDLE1) -> super::super::super::Win32::Foundation::NTSTATUS {
    windows_targets::link!("fwpuclnt.dll" "system" fn IPsecSaContextAddOutbound1(enginehandle : super::super::super::Win32::Foundation:: HANDLE, id : u64, outboundbundle : *const super::super::super::Win32::NetworkManagement::WindowsFilteringPlatform:: IPSEC_SA_BUNDLE1) -> super::super::super::Win32::Foundation:: NTSTATUS);
    IPsecSaContextAddOutbound1(core::mem::transmute(enginehandle), core::mem::transmute(id), core::mem::transmute(outboundbundle))
}
#[cfg(feature = "Win32_NetworkManagement_WindowsFilteringPlatform")]
#[inline]
pub unsafe fn IPsecSaContextCreate0(enginehandle: super::super::super::Win32::Foundation::HANDLE, outboundtraffic: *const super::super::super::Win32::NetworkManagement::WindowsFilteringPlatform::IPSEC_TRAFFIC0, inboundfilterid: Option<*mut u64>, id: *mut u64) -> super::super::super::Win32::Foundation::NTSTATUS {
    windows_targets::link!("fwpuclnt.dll" "system" fn IPsecSaContextCreate0(enginehandle : super::super::super::Win32::Foundation:: HANDLE, outboundtraffic : *const super::super::super::Win32::NetworkManagement::WindowsFilteringPlatform:: IPSEC_TRAFFIC0, inboundfilterid : *mut u64, id : *mut u64) -> super::super::super::Win32::Foundation:: NTSTATUS);
    IPsecSaContextCreate0(core::mem::transmute(enginehandle), core::mem::transmute(outboundtraffic), core::mem::transmute(inboundfilterid.unwrap_or(core::mem::zeroed())), core::mem::transmute(id))
}
#[cfg(feature = "Win32_NetworkManagement_WindowsFilteringPlatform")]
#[inline]
pub unsafe fn IPsecSaContextCreate1(enginehandle: super::super::super::Win32::Foundation::HANDLE, outboundtraffic: *const super::super::super::Win32::NetworkManagement::WindowsFilteringPlatform::IPSEC_TRAFFIC1, virtualiftunnelinfo: Option<*const super::super::super::Win32::NetworkManagement::WindowsFilteringPlatform::IPSEC_VIRTUAL_IF_TUNNEL_INFO0>, inboundfilterid: Option<*mut u64>, id: *mut u64) -> super::super::super::Win32::Foundation::NTSTATUS {
    windows_targets::link!("fwpuclnt.dll" "system" fn IPsecSaContextCreate1(enginehandle : super::super::super::Win32::Foundation:: HANDLE, outboundtraffic : *const super::super::super::Win32::NetworkManagement::WindowsFilteringPlatform:: IPSEC_TRAFFIC1, virtualiftunnelinfo : *const super::super::super::Win32::NetworkManagement::WindowsFilteringPlatform:: IPSEC_VIRTUAL_IF_TUNNEL_INFO0, inboundfilterid : *mut u64, id : *mut u64) -> super::super::super::Win32::Foundation:: NTSTATUS);
    IPsecSaContextCreate1(core::mem::transmute(enginehandle), core::mem::transmute(outboundtraffic), core::mem::transmute(virtualiftunnelinfo.unwrap_or(core::mem::zeroed())), core::mem::transmute(inboundfilterid.unwrap_or(core::mem::zeroed())), core::mem::transmute(id))
}
#[cfg(all(feature = "Win32_NetworkManagement_WindowsFilteringPlatform", feature = "Win32_Security"))]
#[inline]
pub unsafe fn IPsecSaContextCreateEnumHandle0(enginehandle: super::super::super::Win32::Foundation::HANDLE, enumtemplate: Option<*const super::super::super::Win32::NetworkManagement::WindowsFilteringPlatform::IPSEC_SA_CONTEXT_ENUM_TEMPLATE0>, enumhandle: *mut super::super::super::Win32::Foundation::HANDLE) -> super::super::super::Win32::Foundation::NTSTATUS {
    windows_targets::link!("fwpuclnt.dll" "system" fn IPsecSaContextCreateEnumHandle0(enginehandle : super::super::super::Win32::Foundation:: HANDLE, enumtemplate : *const super::super::super::Win32::NetworkManagement::WindowsFilteringPlatform:: IPSEC_SA_CONTEXT_ENUM_TEMPLATE0, enumhandle : *mut super::super::super::Win32::Foundation:: HANDLE) -> super::super::super::Win32::Foundation:: NTSTATUS);
    IPsecSaContextCreateEnumHandle0(core::mem::transmute(enginehandle), core::mem::transmute(enumtemplate.unwrap_or(core::mem::zeroed())), core::mem::transmute(enumhandle))
}
#[inline]
pub unsafe fn IPsecSaContextDeleteById0(enginehandle: super::super::super::Win32::Foundation::HANDLE, id: u64) -> super::super::super::Win32::Foundation::NTSTATUS {
    windows_targets::link!("fwpuclnt.dll" "system" fn IPsecSaContextDeleteById0(enginehandle : super::super::super::Win32::Foundation:: HANDLE, id : u64) -> super::super::super::Win32::Foundation:: NTSTATUS);
    IPsecSaContextDeleteById0(core::mem::transmute(enginehandle), core::mem::transmute(id))
}
#[inline]
pub unsafe fn IPsecSaContextDestroyEnumHandle0(enginehandle: super::super::super::Win32::Foundation::HANDLE, enumhandle: super::super::super::Win32::Foundation::HANDLE) -> super::super::super::Win32::Foundation::NTSTATUS {
    windows_targets::link!("fwpuclnt.dll" "system" fn IPsecSaContextDestroyEnumHandle0(enginehandle : super::super::super::Win32::Foundation:: HANDLE, enumhandle : super::super::super::Win32::Foundation:: HANDLE) -> super::super::super::Win32::Foundation:: NTSTATUS);
    IPsecSaContextDestroyEnumHandle0(core::mem::transmute(enginehandle), core::mem::transmute(enumhandle))
}
#[cfg(all(feature = "Win32_NetworkManagement_WindowsFilteringPlatform", feature = "Win32_Security"))]
#[inline]
pub unsafe fn IPsecSaContextEnum0(enginehandle: super::super::super::Win32::Foundation::HANDLE, enumhandle: super::super::super::Win32::Foundation::HANDLE, numentriesrequested: u32, entries: *mut *mut *mut super::super::super::Win32::NetworkManagement::WindowsFilteringPlatform::IPSEC_SA_CONTEXT0, numentriesreturned: *mut u32) -> super::super::super::Win32::Foundation::NTSTATUS {
    windows_targets::link!("fwpuclnt.dll" "system" fn IPsecSaContextEnum0(enginehandle : super::super::super::Win32::Foundation:: HANDLE, enumhandle : super::super::super::Win32::Foundation:: HANDLE, numentriesrequested : u32, entries : *mut *mut *mut super::super::super::Win32::NetworkManagement::WindowsFilteringPlatform:: IPSEC_SA_CONTEXT0, numentriesreturned : *mut u32) -> super::super::super::Win32::Foundation:: NTSTATUS);
    IPsecSaContextEnum0(core::mem::transmute(enginehandle), core::mem::transmute(enumhandle), core::mem::transmute(numentriesrequested), core::mem::transmute(entries), core::mem::transmute(numentriesreturned))
}
#[cfg(all(feature = "Win32_NetworkManagement_WindowsFilteringPlatform", feature = "Win32_Security"))]
#[inline]
pub unsafe fn IPsecSaContextEnum1(enginehandle: super::super::super::Win32::Foundation::HANDLE, enumhandle: super::super::super::Win32::Foundation::HANDLE, numentriesrequested: u32, entries: *mut *mut *mut super::super::super::Win32::NetworkManagement::WindowsFilteringPlatform::IPSEC_SA_CONTEXT1, numentriesreturned: *mut u32) -> super::super::super::Win32::Foundation::NTSTATUS {
    windows_targets::link!("fwpuclnt.dll" "system" fn IPsecSaContextEnum1(enginehandle : super::super::super::Win32::Foundation:: HANDLE, enumhandle : super::super::super::Win32::Foundation:: HANDLE, numentriesrequested : u32, entries : *mut *mut *mut super::super::super::Win32::NetworkManagement::WindowsFilteringPlatform:: IPSEC_SA_CONTEXT1, numentriesreturned : *mut u32) -> super::super::super::Win32::Foundation:: NTSTATUS);
    IPsecSaContextEnum1(core::mem::transmute(enginehandle), core::mem::transmute(enumhandle), core::mem::transmute(numentriesrequested), core::mem::transmute(entries), core::mem::transmute(numentriesreturned))
}
#[inline]
pub unsafe fn IPsecSaContextExpire0(enginehandle: super::super::super::Win32::Foundation::HANDLE, id: u64) -> super::super::super::Win32::Foundation::NTSTATUS {
    windows_targets::link!("fwpuclnt.dll" "system" fn IPsecSaContextExpire0(enginehandle : super::super::super::Win32::Foundation:: HANDLE, id : u64) -> super::super::super::Win32::Foundation:: NTSTATUS);
    IPsecSaContextExpire0(core::mem::transmute(enginehandle), core::mem::transmute(id))
}
#[cfg(all(feature = "Win32_NetworkManagement_WindowsFilteringPlatform", feature = "Win32_Security"))]
#[inline]
pub unsafe fn IPsecSaContextGetById0(enginehandle: super::super::super::Win32::Foundation::HANDLE, id: u64, sacontext: *mut *mut super::super::super::Win32::NetworkManagement::WindowsFilteringPlatform::IPSEC_SA_CONTEXT0) -> super::super::super::Win32::Foundation::NTSTATUS {
    windows_targets::link!("fwpuclnt.dll" "system" fn IPsecSaContextGetById0(enginehandle : super::super::super::Win32::Foundation:: HANDLE, id : u64, sacontext : *mut *mut super::super::super::Win32::NetworkManagement::WindowsFilteringPlatform:: IPSEC_SA_CONTEXT0) -> super::super::super::Win32::Foundation:: NTSTATUS);
    IPsecSaContextGetById0(core::mem::transmute(enginehandle), core::mem::transmute(id), core::mem::transmute(sacontext))
}
#[cfg(all(feature = "Win32_NetworkManagement_WindowsFilteringPlatform", feature = "Win32_Security"))]
#[inline]
pub unsafe fn IPsecSaContextGetById1(enginehandle: super::super::super::Win32::Foundation::HANDLE, id: u64, sacontext: *mut *mut super::super::super::Win32::NetworkManagement::WindowsFilteringPlatform::IPSEC_SA_CONTEXT1) -> super::super::super::Win32::Foundation::NTSTATUS {
    windows_targets::link!("fwpuclnt.dll" "system" fn IPsecSaContextGetById1(enginehandle : super::super::super::Win32::Foundation:: HANDLE, id : u64, sacontext : *mut *mut super::super::super::Win32::NetworkManagement::WindowsFilteringPlatform:: IPSEC_SA_CONTEXT1) -> super::super::super::Win32::Foundation:: NTSTATUS);
    IPsecSaContextGetById1(core::mem::transmute(enginehandle), core::mem::transmute(id), core::mem::transmute(sacontext))
}
#[cfg(feature = "Win32_NetworkManagement_WindowsFilteringPlatform")]
#[inline]
pub unsafe fn IPsecSaContextGetSpi0(enginehandle: super::super::super::Win32::Foundation::HANDLE, id: u64, getspi: *const super::super::super::Win32::NetworkManagement::WindowsFilteringPlatform::IPSEC_GETSPI0, inboundspi: *mut u32) -> super::super::super::Win32::Foundation::NTSTATUS {
    windows_targets::link!("fwpuclnt.dll" "system" fn IPsecSaContextGetSpi0(enginehandle : super::super::super::Win32::Foundation:: HANDLE, id : u64, getspi : *const super::super::super::Win32::NetworkManagement::WindowsFilteringPlatform:: IPSEC_GETSPI0, inboundspi : *mut u32) -> super::super::super::Win32::Foundation:: NTSTATUS);
    IPsecSaContextGetSpi0(core::mem::transmute(enginehandle), core::mem::transmute(id), core::mem::transmute(getspi), core::mem::transmute(inboundspi))
}
#[cfg(feature = "Win32_NetworkManagement_WindowsFilteringPlatform")]
#[inline]
pub unsafe fn IPsecSaContextGetSpi1(enginehandle: super::super::super::Win32::Foundation::HANDLE, id: u64, getspi: *const super::super::super::Win32::NetworkManagement::WindowsFilteringPlatform::IPSEC_GETSPI1, inboundspi: *mut u32) -> super::super::super::Win32::Foundation::NTSTATUS {
    windows_targets::link!("fwpuclnt.dll" "system" fn IPsecSaContextGetSpi1(enginehandle : super::super::super::Win32::Foundation:: HANDLE, id : u64, getspi : *const super::super::super::Win32::NetworkManagement::WindowsFilteringPlatform:: IPSEC_GETSPI1, inboundspi : *mut u32) -> super::super::super::Win32::Foundation:: NTSTATUS);
    IPsecSaContextGetSpi1(core::mem::transmute(enginehandle), core::mem::transmute(id), core::mem::transmute(getspi), core::mem::transmute(inboundspi))
}
#[cfg(feature = "Win32_NetworkManagement_WindowsFilteringPlatform")]
#[inline]
pub unsafe fn IPsecSaContextSetSpi0(enginehandle: super::super::super::Win32::Foundation::HANDLE, id: u64, getspi: *const super::super::super::Win32::NetworkManagement::WindowsFilteringPlatform::IPSEC_GETSPI1, inboundspi: u32) -> super::super::super::Win32::Foundation::NTSTATUS {
    windows_targets::link!("fwpuclnt.dll" "system" fn IPsecSaContextSetSpi0(enginehandle : super::super::super::Win32::Foundation:: HANDLE, id : u64, getspi : *const super::super::super::Win32::NetworkManagement::WindowsFilteringPlatform:: IPSEC_GETSPI1, inboundspi : u32) -> super::super::super::Win32::Foundation:: NTSTATUS);
    IPsecSaContextSetSpi0(core::mem::transmute(enginehandle), core::mem::transmute(id), core::mem::transmute(getspi), core::mem::transmute(inboundspi))
}
#[cfg(all(feature = "Win32_NetworkManagement_WindowsFilteringPlatform", feature = "Win32_Security"))]
#[inline]
pub unsafe fn IPsecSaContextUpdate0(enginehandle: super::super::super::Win32::Foundation::HANDLE, flags: u64, newvalues: *const super::super::super::Win32::NetworkManagement::WindowsFilteringPlatform::IPSEC_SA_CONTEXT1) -> super::super::super::Win32::Foundation::NTSTATUS {
    windows_targets::link!("fwpuclnt.dll" "system" fn IPsecSaContextUpdate0(enginehandle : super::super::super::Win32::Foundation:: HANDLE, flags : u64, newvalues : *const super::super::super::Win32::NetworkManagement::WindowsFilteringPlatform:: IPSEC_SA_CONTEXT1) -> super::super::super::Win32::Foundation:: NTSTATUS);
    IPsecSaContextUpdate0(core::mem::transmute(enginehandle), core::mem::transmute(flags), core::mem::transmute(newvalues))
}
#[cfg(feature = "Win32_NetworkManagement_WindowsFilteringPlatform")]
#[inline]
pub unsafe fn IPsecSaCreateEnumHandle0(enginehandle: super::super::super::Win32::Foundation::HANDLE, enumtemplate: Option<*const super::super::super::Win32::NetworkManagement::WindowsFilteringPlatform::IPSEC_SA_ENUM_TEMPLATE0>, enumhandle: *mut super::super::super::Win32::Foundation::HANDLE) -> super::super::super::Win32::Foundation::NTSTATUS {
    windows_targets::link!("fwpuclnt.dll" "system" fn IPsecSaCreateEnumHandle0(enginehandle : super::super::super::Win32::Foundation:: HANDLE, enumtemplate : *const super::super::super::Win32::NetworkManagement::WindowsFilteringPlatform:: IPSEC_SA_ENUM_TEMPLATE0, enumhandle : *mut super::super::super::Win32::Foundation:: HANDLE) -> super::super::super::Win32::Foundation:: NTSTATUS);
    IPsecSaCreateEnumHandle0(core::mem::transmute(enginehandle), core::mem::transmute(enumtemplate.unwrap_or(core::mem::zeroed())), core::mem::transmute(enumhandle))
}
#[cfg(feature = "Win32_Security")]
#[inline]
pub unsafe fn IPsecSaDbGetSecurityInfo0(enginehandle: super::super::super::Win32::Foundation::HANDLE, securityinfo: u32, sidowner: *mut super::super::super::Win32::Security::PSID, sidgroup: *mut super::super::super::Win32::Security::PSID, dacl: *mut *mut super::super::super::Win32::Security::ACL, sacl: *mut *mut super::super::super::Win32::Security::ACL, securitydescriptor: *mut super::super::super::Win32::Security::PSECURITY_DESCRIPTOR) -> super::super::super::Win32::Foundation::NTSTATUS {
    windows_targets::link!("fwpuclnt.dll" "system" fn IPsecSaDbGetSecurityInfo0(enginehandle : super::super::super::Win32::Foundation:: HANDLE, securityinfo : u32, sidowner : *mut super::super::super::Win32::Security:: PSID, sidgroup : *mut super::super::super::Win32::Security:: PSID, dacl : *mut *mut super::super::super::Win32::Security:: ACL, sacl : *mut *mut super::super::super::Win32::Security:: ACL, securitydescriptor : *mut super::super::super::Win32::Security:: PSECURITY_DESCRIPTOR) -> super::super::super::Win32::Foundation:: NTSTATUS);
    IPsecSaDbGetSecurityInfo0(core::mem::transmute(enginehandle), core::mem::transmute(securityinfo), core::mem::transmute(sidowner), core::mem::transmute(sidgroup), core::mem::transmute(dacl), core::mem::transmute(sacl), core::mem::transmute(securitydescriptor))
}
#[cfg(feature = "Win32_Security")]
#[inline]
pub unsafe fn IPsecSaDbSetSecurityInfo0(enginehandle: super::super::super::Win32::Foundation::HANDLE, securityinfo: u32, sidowner: Option<*const super::super::super::Win32::Security::SID>, sidgroup: Option<*const super::super::super::Win32::Security::SID>, dacl: Option<*const super::super::super::Win32::Security::ACL>, sacl: Option<*const super::super::super::Win32::Security::ACL>) -> super::super::super::Win32::Foundation::NTSTATUS {
    windows_targets::link!("fwpuclnt.dll" "system" fn IPsecSaDbSetSecurityInfo0(enginehandle : super::super::super::Win32::Foundation:: HANDLE, securityinfo : u32, sidowner : *const super::super::super::Win32::Security:: SID, sidgroup : *const super::super::super::Win32::Security:: SID, dacl : *const super::super::super::Win32::Security:: ACL, sacl : *const super::super::super::Win32::Security:: ACL) -> super::super::super::Win32::Foundation:: NTSTATUS);
    IPsecSaDbSetSecurityInfo0(core::mem::transmute(enginehandle), core::mem::transmute(securityinfo), core::mem::transmute(sidowner.unwrap_or(core::mem::zeroed())), core::mem::transmute(sidgroup.unwrap_or(core::mem::zeroed())), core::mem::transmute(dacl.unwrap_or(core::mem::zeroed())), core::mem::transmute(sacl.unwrap_or(core::mem::zeroed())))
}
#[inline]
pub unsafe fn IPsecSaDestroyEnumHandle0(enginehandle: super::super::super::Win32::Foundation::HANDLE, enumhandle: super::super::super::Win32::Foundation::HANDLE) -> super::super::super::Win32::Foundation::NTSTATUS {
    windows_targets::link!("fwpuclnt.dll" "system" fn IPsecSaDestroyEnumHandle0(enginehandle : super::super::super::Win32::Foundation:: HANDLE, enumhandle : super::super::super::Win32::Foundation:: HANDLE) -> super::super::super::Win32::Foundation:: NTSTATUS);
    IPsecSaDestroyEnumHandle0(core::mem::transmute(enginehandle), core::mem::transmute(enumhandle))
}
#[cfg(all(feature = "Win32_NetworkManagement_WindowsFilteringPlatform", feature = "Win32_Security"))]
#[inline]
pub unsafe fn IPsecSaEnum0(enginehandle: super::super::super::Win32::Foundation::HANDLE, enumhandle: super::super::super::Win32::Foundation::HANDLE, numentriesrequested: u32, entries: *mut *mut *mut super::super::super::Win32::NetworkManagement::WindowsFilteringPlatform::IPSEC_SA_DETAILS0, numentriesreturned: *mut u32) -> super::super::super::Win32::Foundation::NTSTATUS {
    windows_targets::link!("fwpuclnt.dll" "system" fn IPsecSaEnum0(enginehandle : super::super::super::Win32::Foundation:: HANDLE, enumhandle : super::super::super::Win32::Foundation:: HANDLE, numentriesrequested : u32, entries : *mut *mut *mut super::super::super::Win32::NetworkManagement::WindowsFilteringPlatform:: IPSEC_SA_DETAILS0, numentriesreturned : *mut u32) -> super::super::super::Win32::Foundation:: NTSTATUS);
    IPsecSaEnum0(core::mem::transmute(enginehandle), core::mem::transmute(enumhandle), core::mem::transmute(numentriesrequested), core::mem::transmute(entries), core::mem::transmute(numentriesreturned))
}
#[cfg(all(feature = "Win32_NetworkManagement_WindowsFilteringPlatform", feature = "Win32_Security"))]
#[inline]
pub unsafe fn IPsecSaEnum1(enginehandle: super::super::super::Win32::Foundation::HANDLE, enumhandle: super::super::super::Win32::Foundation::HANDLE, numentriesrequested: u32, entries: *mut *mut *mut super::super::super::Win32::NetworkManagement::WindowsFilteringPlatform::IPSEC_SA_DETAILS1, numentriesreturned: *mut u32) -> super::super::super::Win32::Foundation::NTSTATUS {
    windows_targets::link!("fwpuclnt.dll" "system" fn IPsecSaEnum1(enginehandle : super::super::super::Win32::Foundation:: HANDLE, enumhandle : super::super::super::Win32::Foundation:: HANDLE, numentriesrequested : u32, entries : *mut *mut *mut super::super::super::Win32::NetworkManagement::WindowsFilteringPlatform:: IPSEC_SA_DETAILS1, numentriesreturned : *mut u32) -> super::super::super::Win32::Foundation:: NTSTATUS);
    IPsecSaEnum1(core::mem::transmute(enginehandle), core::mem::transmute(enumhandle), core::mem::transmute(numentriesrequested), core::mem::transmute(entries), core::mem::transmute(numentriesreturned))
}
#[cfg(feature = "Win32_NetworkManagement_WindowsFilteringPlatform")]
#[inline]
pub unsafe fn IkeextGetStatistics0(enginehandle: super::super::super::Win32::Foundation::HANDLE, ikeextstatistics: *mut super::super::super::Win32::NetworkManagement::WindowsFilteringPlatform::IKEEXT_STATISTICS0) -> super::super::super::Win32::Foundation::NTSTATUS {
    windows_targets::link!("fwpuclnt.dll" "system" fn IkeextGetStatistics0(enginehandle : super::super::super::Win32::Foundation:: HANDLE, ikeextstatistics : *mut super::super::super::Win32::NetworkManagement::WindowsFilteringPlatform:: IKEEXT_STATISTICS0) -> super::super::super::Win32::Foundation:: NTSTATUS);
    IkeextGetStatistics0(core::mem::transmute(enginehandle), core::mem::transmute(ikeextstatistics))
}
#[cfg(feature = "Win32_NetworkManagement_WindowsFilteringPlatform")]
#[inline]
pub unsafe fn IkeextGetStatistics1(enginehandle: super::super::super::Win32::Foundation::HANDLE, ikeextstatistics: *mut super::super::super::Win32::NetworkManagement::WindowsFilteringPlatform::IKEEXT_STATISTICS1) -> super::super::super::Win32::Foundation::NTSTATUS {
    windows_targets::link!("fwpuclnt.dll" "system" fn IkeextGetStatistics1(enginehandle : super::super::super::Win32::Foundation:: HANDLE, ikeextstatistics : *mut super::super::super::Win32::NetworkManagement::WindowsFilteringPlatform:: IKEEXT_STATISTICS1) -> super::super::super::Win32::Foundation:: NTSTATUS);
    IkeextGetStatistics1(core::mem::transmute(enginehandle), core::mem::transmute(ikeextstatistics))
}
#[cfg(all(feature = "Win32_NetworkManagement_WindowsFilteringPlatform", feature = "Win32_Security"))]
#[inline]
pub unsafe fn IkeextSaCreateEnumHandle0(enginehandle: super::super::super::Win32::Foundation::HANDLE, enumtemplate: Option<*const super::super::super::Win32::NetworkManagement::WindowsFilteringPlatform::IKEEXT_SA_ENUM_TEMPLATE0>, enumhandle: *mut super::super::super::Win32::Foundation::HANDLE) -> super::super::super::Win32::Foundation::NTSTATUS {
    windows_targets::link!("fwpuclnt.dll" "system" fn IkeextSaCreateEnumHandle0(enginehandle : super::super::super::Win32::Foundation:: HANDLE, enumtemplate : *const super::super::super::Win32::NetworkManagement::WindowsFilteringPlatform:: IKEEXT_SA_ENUM_TEMPLATE0, enumhandle : *mut super::super::super::Win32::Foundation:: HANDLE) -> super::super::super::Win32::Foundation:: NTSTATUS);
    IkeextSaCreateEnumHandle0(core::mem::transmute(enginehandle), core::mem::transmute(enumtemplate.unwrap_or(core::mem::zeroed())), core::mem::transmute(enumhandle))
}
#[cfg(feature = "Win32_Security")]
#[inline]
pub unsafe fn IkeextSaDbGetSecurityInfo0(enginehandle: super::super::super::Win32::Foundation::HANDLE, securityinfo: u32, sidowner: *mut super::super::super::Win32::Security::PSID, sidgroup: *mut super::super::super::Win32::Security::PSID, dacl: *mut *mut super::super::super::Win32::Security::ACL, sacl: *mut *mut super::super::super::Win32::Security::ACL, securitydescriptor: *mut super::super::super::Win32::Security::PSECURITY_DESCRIPTOR) -> super::super::super::Win32::Foundation::NTSTATUS {
    windows_targets::link!("fwpuclnt.dll" "system" fn IkeextSaDbGetSecurityInfo0(enginehandle : super::super::super::Win32::Foundation:: HANDLE, securityinfo : u32, sidowner : *mut super::super::super::Win32::Security:: PSID, sidgroup : *mut super::super::super::Win32::Security:: PSID, dacl : *mut *mut super::super::super::Win32::Security:: ACL, sacl : *mut *mut super::super::super::Win32::Security:: ACL, securitydescriptor : *mut super::super::super::Win32::Security:: PSECURITY_DESCRIPTOR) -> super::super::super::Win32::Foundation:: NTSTATUS);
    IkeextSaDbGetSecurityInfo0(core::mem::transmute(enginehandle), core::mem::transmute(securityinfo), core::mem::transmute(sidowner), core::mem::transmute(sidgroup), core::mem::transmute(dacl), core::mem::transmute(sacl), core::mem::transmute(securitydescriptor))
}
#[cfg(feature = "Win32_Security")]
#[inline]
pub unsafe fn IkeextSaDbSetSecurityInfo0(enginehandle: super::super::super::Win32::Foundation::HANDLE, securityinfo: u32, sidowner: Option<*const super::super::super::Win32::Security::SID>, sidgroup: Option<*const super::super::super::Win32::Security::SID>, dacl: Option<*const super::super::super::Win32::Security::ACL>, sacl: Option<*const super::super::super::Win32::Security::ACL>) -> super::super::super::Win32::Foundation::NTSTATUS {
    windows_targets::link!("fwpuclnt.dll" "system" fn IkeextSaDbSetSecurityInfo0(enginehandle : super::super::super::Win32::Foundation:: HANDLE, securityinfo : u32, sidowner : *const super::super::super::Win32::Security:: SID, sidgroup : *const super::super::super::Win32::Security:: SID, dacl : *const super::super::super::Win32::Security:: ACL, sacl : *const super::super::super::Win32::Security:: ACL) -> super::super::super::Win32::Foundation:: NTSTATUS);
    IkeextSaDbSetSecurityInfo0(core::mem::transmute(enginehandle), core::mem::transmute(securityinfo), core::mem::transmute(sidowner.unwrap_or(core::mem::zeroed())), core::mem::transmute(sidgroup.unwrap_or(core::mem::zeroed())), core::mem::transmute(dacl.unwrap_or(core::mem::zeroed())), core::mem::transmute(sacl.unwrap_or(core::mem::zeroed())))
}
#[inline]
pub unsafe fn IkeextSaDeleteById0(enginehandle: super::super::super::Win32::Foundation::HANDLE, id: u64) -> super::super::super::Win32::Foundation::NTSTATUS {
    windows_targets::link!("fwpuclnt.dll" "system" fn IkeextSaDeleteById0(enginehandle : super::super::super::Win32::Foundation:: HANDLE, id : u64) -> super::super::super::Win32::Foundation:: NTSTATUS);
    IkeextSaDeleteById0(core::mem::transmute(enginehandle), core::mem::transmute(id))
}
#[inline]
pub unsafe fn IkeextSaDestroyEnumHandle0(enginehandle: super::super::super::Win32::Foundation::HANDLE, enumhandle: super::super::super::Win32::Foundation::HANDLE) -> super::super::super::Win32::Foundation::NTSTATUS {
    windows_targets::link!("fwpuclnt.dll" "system" fn IkeextSaDestroyEnumHandle0(enginehandle : super::super::super::Win32::Foundation:: HANDLE, enumhandle : super::super::super::Win32::Foundation:: HANDLE) -> super::super::super::Win32::Foundation:: NTSTATUS);
    IkeextSaDestroyEnumHandle0(core::mem::transmute(enginehandle), core::mem::transmute(enumhandle))
}
#[cfg(feature = "Win32_NetworkManagement_WindowsFilteringPlatform")]
#[inline]
pub unsafe fn IkeextSaEnum0(enginehandle: super::super::super::Win32::Foundation::HANDLE, enumhandle: super::super::super::Win32::Foundation::HANDLE, numentriesrequested: u32, entries: *mut *mut *mut super::super::super::Win32::NetworkManagement::WindowsFilteringPlatform::IKEEXT_SA_DETAILS0, numentriesreturned: *mut u32) -> super::super::super::Win32::Foundation::NTSTATUS {
    windows_targets::link!("fwpuclnt.dll" "system" fn IkeextSaEnum0(enginehandle : super::super::super::Win32::Foundation:: HANDLE, enumhandle : super::super::super::Win32::Foundation:: HANDLE, numentriesrequested : u32, entries : *mut *mut *mut super::super::super::Win32::NetworkManagement::WindowsFilteringPlatform:: IKEEXT_SA_DETAILS0, numentriesreturned : *mut u32) -> super::super::super::Win32::Foundation:: NTSTATUS);
    IkeextSaEnum0(core::mem::transmute(enginehandle), core::mem::transmute(enumhandle), core::mem::transmute(numentriesrequested), core::mem::transmute(entries), core::mem::transmute(numentriesreturned))
}
#[cfg(feature = "Win32_NetworkManagement_WindowsFilteringPlatform")]
#[inline]
pub unsafe fn IkeextSaEnum1(enginehandle: super::super::super::Win32::Foundation::HANDLE, enumhandle: super::super::super::Win32::Foundation::HANDLE, numentriesrequested: u32, entries: *mut *mut *mut super::super::super::Win32::NetworkManagement::WindowsFilteringPlatform::IKEEXT_SA_DETAILS1, numentriesreturned: *mut u32) -> super::super::super::Win32::Foundation::NTSTATUS {
    windows_targets::link!("fwpuclnt.dll" "system" fn IkeextSaEnum1(enginehandle : super::super::super::Win32::Foundation:: HANDLE, enumhandle : super::super::super::Win32::Foundation:: HANDLE, numentriesrequested : u32, entries : *mut *mut *mut super::super::super::Win32::NetworkManagement::WindowsFilteringPlatform:: IKEEXT_SA_DETAILS1, numentriesreturned : *mut u32) -> super::super::super::Win32::Foundation:: NTSTATUS);
    IkeextSaEnum1(core::mem::transmute(enginehandle), core::mem::transmute(enumhandle), core::mem::transmute(numentriesrequested), core::mem::transmute(entries), core::mem::transmute(numentriesreturned))
}
#[cfg(feature = "Win32_NetworkManagement_WindowsFilteringPlatform")]
#[inline]
pub unsafe fn IkeextSaEnum2(enginehandle: super::super::super::Win32::Foundation::HANDLE, enumhandle: super::super::super::Win32::Foundation::HANDLE, numentriesrequested: u32, entries: *mut *mut *mut super::super::super::Win32::NetworkManagement::WindowsFilteringPlatform::IKEEXT_SA_DETAILS2, numentriesreturned: *mut u32) -> super::super::super::Win32::Foundation::NTSTATUS {
    windows_targets::link!("fwpuclnt.dll" "system" fn IkeextSaEnum2(enginehandle : super::super::super::Win32::Foundation:: HANDLE, enumhandle : super::super::super::Win32::Foundation:: HANDLE, numentriesrequested : u32, entries : *mut *mut *mut super::super::super::Win32::NetworkManagement::WindowsFilteringPlatform:: IKEEXT_SA_DETAILS2, numentriesreturned : *mut u32) -> super::super::super::Win32::Foundation:: NTSTATUS);
    IkeextSaEnum2(core::mem::transmute(enginehandle), core::mem::transmute(enumhandle), core::mem::transmute(numentriesrequested), core::mem::transmute(entries), core::mem::transmute(numentriesreturned))
}
#[cfg(feature = "Win32_NetworkManagement_WindowsFilteringPlatform")]
#[inline]
pub unsafe fn IkeextSaGetById0(enginehandle: super::super::super::Win32::Foundation::HANDLE, id: u64, sa: *mut *mut super::super::super::Win32::NetworkManagement::WindowsFilteringPlatform::IKEEXT_SA_DETAILS0) -> super::super::super::Win32::Foundation::NTSTATUS {
    windows_targets::link!("fwpuclnt.dll" "system" fn IkeextSaGetById0(enginehandle : super::super::super::Win32::Foundation:: HANDLE, id : u64, sa : *mut *mut super::super::super::Win32::NetworkManagement::WindowsFilteringPlatform:: IKEEXT_SA_DETAILS0) -> super::super::super::Win32::Foundation:: NTSTATUS);
    IkeextSaGetById0(core::mem::transmute(enginehandle), core::mem::transmute(id), core::mem::transmute(sa))
}
#[cfg(feature = "Win32_NetworkManagement_WindowsFilteringPlatform")]
#[inline]
pub unsafe fn IkeextSaGetById1(enginehandle: super::super::super::Win32::Foundation::HANDLE, id: u64, salookupcontext: Option<*const windows_core::GUID>, sa: *mut *mut super::super::super::Win32::NetworkManagement::WindowsFilteringPlatform::IKEEXT_SA_DETAILS1) -> super::super::super::Win32::Foundation::NTSTATUS {
    windows_targets::link!("fwpuclnt.dll" "system" fn IkeextSaGetById1(enginehandle : super::super::super::Win32::Foundation:: HANDLE, id : u64, salookupcontext : *const windows_core::GUID, sa : *mut *mut super::super::super::Win32::NetworkManagement::WindowsFilteringPlatform:: IKEEXT_SA_DETAILS1) -> super::super::super::Win32::Foundation:: NTSTATUS);
    IkeextSaGetById1(core::mem::transmute(enginehandle), core::mem::transmute(id), core::mem::transmute(salookupcontext.unwrap_or(core::mem::zeroed())), core::mem::transmute(sa))
}
#[cfg(feature = "Win32_NetworkManagement_WindowsFilteringPlatform")]
#[inline]
pub unsafe fn IkeextSaGetById2(enginehandle: super::super::super::Win32::Foundation::HANDLE, id: u64, salookupcontext: Option<*const windows_core::GUID>, sa: *mut *mut super::super::super::Win32::NetworkManagement::WindowsFilteringPlatform::IKEEXT_SA_DETAILS2) -> super::super::super::Win32::Foundation::NTSTATUS {
    windows_targets::link!("fwpuclnt.dll" "system" fn IkeextSaGetById2(enginehandle : super::super::super::Win32::Foundation:: HANDLE, id : u64, salookupcontext : *const windows_core::GUID, sa : *mut *mut super::super::super::Win32::NetworkManagement::WindowsFilteringPlatform:: IKEEXT_SA_DETAILS2) -> super::super::super::Win32::Foundation:: NTSTATUS);
    IkeextSaGetById2(core::mem::transmute(enginehandle), core::mem::transmute(id), core::mem::transmute(salookupcontext.unwrap_or(core::mem::zeroed())), core::mem::transmute(sa))
}
#[cfg(feature = "Win32_NetworkManagement_WindowsFilteringPlatform")]
pub type FWPM_SERVICE_STATE_CHANGE_CALLBACK0 = Option<unsafe extern "system" fn(context: *mut core::ffi::c_void, newstate: super::super::super::Win32::NetworkManagement::WindowsFilteringPlatform::FWPM_SERVICE_STATE)>;
