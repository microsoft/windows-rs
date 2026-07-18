#[cfg(all(feature = "rpc", feature = "winnt"))]
windows_link::link!("ntdsapi.dll" "system" fn DsAddSidHistoryA(hds : super::HANDLE, flags : u32, srcdomain : windows_sys::core::PCSTR, srcprincipal : windows_sys::core::PCSTR, srcdomaincontroller : windows_sys::core::PCSTR, srcdomaincreds : super::RPC_AUTH_IDENTITY_HANDLE, dstdomain : windows_sys::core::PCSTR, dstprincipal : windows_sys::core::PCSTR) -> u32);
#[cfg(all(feature = "rpc", feature = "winnt"))]
windows_link::link!("ntdsapi.dll" "system" fn DsAddSidHistoryW(hds : super::HANDLE, flags : u32, srcdomain : windows_sys::core::PCWSTR, srcprincipal : windows_sys::core::PCWSTR, srcdomaincontroller : windows_sys::core::PCWSTR, srcdomaincreds : super::RPC_AUTH_IDENTITY_HANDLE, dstdomain : windows_sys::core::PCWSTR, dstprincipal : windows_sys::core::PCWSTR) -> u32);
#[cfg(feature = "winnt")]
windows_link::link!("ntdsapi.dll" "system" fn DsBindA(domaincontrollername : windows_sys::core::PCSTR, dnsdomainname : windows_sys::core::PCSTR, phds : *mut super::HANDLE) -> u32);
#[cfg(all(feature = "rpc", feature = "winnt"))]
windows_link::link!("ntdsapi.dll" "system" fn DsBindByInstanceA(servername : windows_sys::core::PCSTR, annotation : windows_sys::core::PCSTR, instanceguid : *const windows_sys::core::GUID, dnsdomainname : windows_sys::core::PCSTR, authidentity : super::RPC_AUTH_IDENTITY_HANDLE, serviceprincipalname : windows_sys::core::PCSTR, bindflags : u32, phds : *mut super::HANDLE) -> u32);
#[cfg(all(feature = "rpc", feature = "winnt"))]
windows_link::link!("ntdsapi.dll" "system" fn DsBindByInstanceW(servername : windows_sys::core::PCWSTR, annotation : windows_sys::core::PCWSTR, instanceguid : *const windows_sys::core::GUID, dnsdomainname : windows_sys::core::PCWSTR, authidentity : super::RPC_AUTH_IDENTITY_HANDLE, serviceprincipalname : windows_sys::core::PCWSTR, bindflags : u32, phds : *mut super::HANDLE) -> u32);
#[cfg(feature = "winnt")]
windows_link::link!("ntdsapi.dll" "system" fn DsBindToISTGA(sitename : windows_sys::core::PCSTR, phds : *mut super::HANDLE) -> u32);
#[cfg(feature = "winnt")]
windows_link::link!("ntdsapi.dll" "system" fn DsBindToISTGW(sitename : windows_sys::core::PCWSTR, phds : *mut super::HANDLE) -> u32);
#[cfg(feature = "winnt")]
windows_link::link!("ntdsapi.dll" "system" fn DsBindW(domaincontrollername : windows_sys::core::PCWSTR, dnsdomainname : windows_sys::core::PCWSTR, phds : *mut super::HANDLE) -> u32);
#[cfg(all(feature = "rpc", feature = "winnt"))]
windows_link::link!("ntdsapi.dll" "system" fn DsBindWithCredA(domaincontrollername : windows_sys::core::PCSTR, dnsdomainname : windows_sys::core::PCSTR, authidentity : super::RPC_AUTH_IDENTITY_HANDLE, phds : *mut super::HANDLE) -> u32);
#[cfg(all(feature = "rpc", feature = "winnt"))]
windows_link::link!("ntdsapi.dll" "system" fn DsBindWithCredW(domaincontrollername : windows_sys::core::PCWSTR, dnsdomainname : windows_sys::core::PCWSTR, authidentity : super::RPC_AUTH_IDENTITY_HANDLE, phds : *mut super::HANDLE) -> u32);
#[cfg(all(feature = "rpc", feature = "winnt"))]
windows_link::link!("ntdsapi.dll" "system" fn DsBindWithSpnA(domaincontrollername : windows_sys::core::PCSTR, dnsdomainname : windows_sys::core::PCSTR, authidentity : super::RPC_AUTH_IDENTITY_HANDLE, serviceprincipalname : windows_sys::core::PCSTR, phds : *mut super::HANDLE) -> u32);
#[cfg(all(feature = "rpc", feature = "winnt"))]
windows_link::link!("ntdsapi.dll" "system" fn DsBindWithSpnExA(domaincontrollername : windows_sys::core::PCSTR, dnsdomainname : windows_sys::core::PCSTR, authidentity : super::RPC_AUTH_IDENTITY_HANDLE, serviceprincipalname : windows_sys::core::PCSTR, bindflags : u32, phds : *mut super::HANDLE) -> u32);
#[cfg(all(feature = "rpc", feature = "winnt"))]
windows_link::link!("ntdsapi.dll" "system" fn DsBindWithSpnExW(domaincontrollername : windows_sys::core::PCWSTR, dnsdomainname : windows_sys::core::PCWSTR, authidentity : super::RPC_AUTH_IDENTITY_HANDLE, serviceprincipalname : windows_sys::core::PCWSTR, bindflags : u32, phds : *mut super::HANDLE) -> u32);
#[cfg(all(feature = "rpc", feature = "winnt"))]
windows_link::link!("ntdsapi.dll" "system" fn DsBindWithSpnW(domaincontrollername : windows_sys::core::PCWSTR, dnsdomainname : windows_sys::core::PCWSTR, authidentity : super::RPC_AUTH_IDENTITY_HANDLE, serviceprincipalname : windows_sys::core::PCWSTR, phds : *mut super::HANDLE) -> u32);
#[cfg(feature = "winnt")]
windows_link::link!("ntdsapi.dll" "system" fn DsBindingSetTimeout(hds : super::HANDLE, ctimeoutsecs : u32) -> u32);
windows_link::link!("ntdsapi.dll" "system" fn DsClientMakeSpnForTargetServerA(serviceclass : windows_sys::core::PCSTR, servicename : windows_sys::core::PCSTR, pcspnlength : *mut u32, pszspn : windows_sys::core::PSTR) -> u32);
windows_link::link!("ntdsapi.dll" "system" fn DsClientMakeSpnForTargetServerW(serviceclass : windows_sys::core::PCWSTR, servicename : windows_sys::core::PCWSTR, pcspnlength : *mut u32, pszspn : windows_sys::core::PWSTR) -> u32);
#[cfg(feature = "winnt")]
windows_link::link!("ntdsapi.dll" "system" fn DsCrackNamesA(hds : super::HANDLE, flags : DS_NAME_FLAGS, formatoffered : DS_NAME_FORMAT, formatdesired : DS_NAME_FORMAT, cnames : u32, rpnames : *const windows_sys::core::PCSTR, ppresult : *mut PDS_NAME_RESULTA) -> u32);
#[cfg(feature = "winnt")]
windows_link::link!("ntdsapi.dll" "system" fn DsCrackNamesW(hds : super::HANDLE, flags : DS_NAME_FLAGS, formatoffered : DS_NAME_FORMAT, formatdesired : DS_NAME_FORMAT, cnames : u32, rpnames : *const windows_sys::core::PCWSTR, ppresult : *mut PDS_NAME_RESULTW) -> u32);
windows_link::link!("ntdsapi.dll" "system" fn DsFreeDomainControllerInfoA(infolevel : u32, cinfo : u32, pinfo : *const core::ffi::c_void));
windows_link::link!("ntdsapi.dll" "system" fn DsFreeDomainControllerInfoW(infolevel : u32, cinfo : u32, pinfo : *const core::ffi::c_void));
windows_link::link!("ntdsapi.dll" "system" fn DsFreeNameResultA(presult : *const DS_NAME_RESULTA));
windows_link::link!("ntdsapi.dll" "system" fn DsFreeNameResultW(presult : *const DS_NAME_RESULTW));
#[cfg(feature = "rpc")]
windows_link::link!("ntdsapi.dll" "system" fn DsFreePasswordCredentials(authidentity : super::RPC_AUTH_IDENTITY_HANDLE));
windows_link::link!("ntdsapi.dll" "system" fn DsFreeSchemaGuidMapA(pguidmap : *const DS_SCHEMA_GUID_MAPA));
windows_link::link!("ntdsapi.dll" "system" fn DsFreeSchemaGuidMapW(pguidmap : *const DS_SCHEMA_GUID_MAPW));
windows_link::link!("ntdsapi.dll" "system" fn DsFreeSpnArrayA(cspn : u32, rpszspn : *mut windows_sys::core::PSTR));
windows_link::link!("ntdsapi.dll" "system" fn DsFreeSpnArrayW(cspn : u32, rpszspn : *mut windows_sys::core::PWSTR));
#[cfg(feature = "winnt")]
windows_link::link!("ntdsapi.dll" "system" fn DsGetDomainControllerInfoA(hds : super::HANDLE, domainname : windows_sys::core::PCSTR, infolevel : u32, pcout : *mut u32, ppinfo : *mut *mut core::ffi::c_void) -> u32);
#[cfg(feature = "winnt")]
windows_link::link!("ntdsapi.dll" "system" fn DsGetDomainControllerInfoW(hds : super::HANDLE, domainname : windows_sys::core::PCWSTR, infolevel : u32, pcout : *mut u32, ppinfo : *mut *mut core::ffi::c_void) -> u32);
windows_link::link!("ntdsapi.dll" "system" fn DsGetSpnA(servicetype : DS_SPN_NAME_TYPE, serviceclass : windows_sys::core::PCSTR, servicename : windows_sys::core::PCSTR, instanceport : u16, cinstancenames : u16, pinstancenames : *const windows_sys::core::PCSTR, pinstanceports : *const u16, pcspn : *mut u32, prpszspn : *mut *mut windows_sys::core::PSTR) -> u32);
windows_link::link!("ntdsapi.dll" "system" fn DsGetSpnW(servicetype : DS_SPN_NAME_TYPE, serviceclass : windows_sys::core::PCWSTR, servicename : windows_sys::core::PCWSTR, instanceport : u16, cinstancenames : u16, pinstancenames : *const windows_sys::core::PCWSTR, pinstanceports : *const u16, pcspn : *mut u32, prpszspn : *mut *mut windows_sys::core::PWSTR) -> u32);
#[cfg(feature = "winnt")]
windows_link::link!("ntdsapi.dll" "system" fn DsInheritSecurityIdentityA(hds : super::HANDLE, flags : u32, srcprincipal : windows_sys::core::PCSTR, dstprincipal : windows_sys::core::PCSTR) -> u32);
#[cfg(feature = "winnt")]
windows_link::link!("ntdsapi.dll" "system" fn DsInheritSecurityIdentityW(hds : super::HANDLE, flags : u32, srcprincipal : windows_sys::core::PCWSTR, dstprincipal : windows_sys::core::PCWSTR) -> u32);
#[cfg(feature = "winnt")]
windows_link::link!("ntdsapi.dll" "system" fn DsListDomainsInSiteA(hds : super::HANDLE, site : windows_sys::core::PCSTR, ppdomains : *mut PDS_NAME_RESULTA) -> u32);
#[cfg(feature = "winnt")]
windows_link::link!("ntdsapi.dll" "system" fn DsListDomainsInSiteW(hds : super::HANDLE, site : windows_sys::core::PCWSTR, ppdomains : *mut PDS_NAME_RESULTW) -> u32);
#[cfg(feature = "winnt")]
windows_link::link!("ntdsapi.dll" "system" fn DsListInfoForServerA(hds : super::HANDLE, server : windows_sys::core::PCSTR, ppinfo : *mut PDS_NAME_RESULTA) -> u32);
#[cfg(feature = "winnt")]
windows_link::link!("ntdsapi.dll" "system" fn DsListInfoForServerW(hds : super::HANDLE, server : windows_sys::core::PCWSTR, ppinfo : *mut PDS_NAME_RESULTW) -> u32);
#[cfg(feature = "winnt")]
windows_link::link!("ntdsapi.dll" "system" fn DsListRolesA(hds : super::HANDLE, pproles : *mut PDS_NAME_RESULTA) -> u32);
#[cfg(feature = "winnt")]
windows_link::link!("ntdsapi.dll" "system" fn DsListRolesW(hds : super::HANDLE, pproles : *mut PDS_NAME_RESULTW) -> u32);
#[cfg(feature = "winnt")]
windows_link::link!("ntdsapi.dll" "system" fn DsListServersForDomainInSiteA(hds : super::HANDLE, domain : windows_sys::core::PCSTR, site : windows_sys::core::PCSTR, ppservers : *mut PDS_NAME_RESULTA) -> u32);
#[cfg(feature = "winnt")]
windows_link::link!("ntdsapi.dll" "system" fn DsListServersForDomainInSiteW(hds : super::HANDLE, domain : windows_sys::core::PCWSTR, site : windows_sys::core::PCWSTR, ppservers : *mut PDS_NAME_RESULTW) -> u32);
#[cfg(feature = "winnt")]
windows_link::link!("ntdsapi.dll" "system" fn DsListServersInSiteA(hds : super::HANDLE, site : windows_sys::core::PCSTR, ppservers : *mut PDS_NAME_RESULTA) -> u32);
#[cfg(feature = "winnt")]
windows_link::link!("ntdsapi.dll" "system" fn DsListServersInSiteW(hds : super::HANDLE, site : windows_sys::core::PCWSTR, ppservers : *mut PDS_NAME_RESULTW) -> u32);
#[cfg(feature = "winnt")]
windows_link::link!("ntdsapi.dll" "system" fn DsListSitesA(hds : super::HANDLE, ppsites : *mut PDS_NAME_RESULTA) -> u32);
#[cfg(feature = "winnt")]
windows_link::link!("ntdsapi.dll" "system" fn DsListSitesW(hds : super::HANDLE, ppsites : *mut PDS_NAME_RESULTW) -> u32);
#[cfg(feature = "rpc")]
windows_link::link!("ntdsapi.dll" "system" fn DsMakePasswordCredentialsA(user : windows_sys::core::PCSTR, domain : windows_sys::core::PCSTR, password : windows_sys::core::PCSTR, pauthidentity : *mut super::RPC_AUTH_IDENTITY_HANDLE) -> u32);
#[cfg(feature = "rpc")]
windows_link::link!("ntdsapi.dll" "system" fn DsMakePasswordCredentialsW(user : windows_sys::core::PCWSTR, domain : windows_sys::core::PCWSTR, password : windows_sys::core::PCWSTR, pauthidentity : *mut super::RPC_AUTH_IDENTITY_HANDLE) -> u32);
#[cfg(feature = "winnt")]
windows_link::link!("ntdsapi.dll" "system" fn DsMapSchemaGuidsA(hds : super::HANDLE, cguids : u32, rguids : *const windows_sys::core::GUID, ppguidmap : *mut *mut DS_SCHEMA_GUID_MAPA) -> u32);
#[cfg(feature = "winnt")]
windows_link::link!("ntdsapi.dll" "system" fn DsMapSchemaGuidsW(hds : super::HANDLE, cguids : u32, rguids : *const windows_sys::core::GUID, ppguidmap : *mut *mut DS_SCHEMA_GUID_MAPW) -> u32);
#[cfg(feature = "winnt")]
windows_link::link!("ntdsapi.dll" "system" fn DsQuerySitesByCostA(hds : super::HANDLE, pszfromsite : windows_sys::core::PCSTR, rgsztosites : *const windows_sys::core::PCSTR, ctosites : u32, dwflags : u32, prgsiteinfo : *mut PDS_SITE_COST_INFO) -> u32);
#[cfg(feature = "winnt")]
windows_link::link!("ntdsapi.dll" "system" fn DsQuerySitesByCostW(hds : super::HANDLE, pwszfromsite : windows_sys::core::PCWSTR, rgwsztosites : *const windows_sys::core::PCWSTR, ctosites : u32, dwflags : u32, prgsiteinfo : *mut PDS_SITE_COST_INFO) -> u32);
windows_link::link!("ntdsapi.dll" "system" fn DsQuerySitesFree(rgsiteinfo : *const DS_SITE_COST_INFO));
#[cfg(feature = "winnt")]
windows_link::link!("ntdsapi.dll" "system" fn DsRemoveDsDomainA(hds : super::HANDLE, domaindn : windows_sys::core::PCSTR) -> u32);
#[cfg(feature = "winnt")]
windows_link::link!("ntdsapi.dll" "system" fn DsRemoveDsDomainW(hds : super::HANDLE, domaindn : windows_sys::core::PCWSTR) -> u32);
#[cfg(feature = "winnt")]
windows_link::link!("ntdsapi.dll" "system" fn DsRemoveDsServerA(hds : super::HANDLE, serverdn : windows_sys::core::PCSTR, domaindn : windows_sys::core::PCSTR, flastdcindomain : *mut windows_sys::core::BOOL, fcommit : windows_sys::core::BOOL) -> u32);
#[cfg(feature = "winnt")]
windows_link::link!("ntdsapi.dll" "system" fn DsRemoveDsServerW(hds : super::HANDLE, serverdn : windows_sys::core::PCWSTR, domaindn : windows_sys::core::PCWSTR, flastdcindomain : *mut windows_sys::core::BOOL, fcommit : windows_sys::core::BOOL) -> u32);
#[cfg(all(feature = "schedule", feature = "winnt"))]
windows_link::link!("ntdsapi.dll" "system" fn DsReplicaAddA(hds : super::HANDLE, namecontext : windows_sys::core::PCSTR, sourcedsadn : windows_sys::core::PCSTR, transportdn : windows_sys::core::PCSTR, sourcedsaaddress : windows_sys::core::PCSTR, pschedule : *const super::SCHEDULE, options : u32) -> u32);
#[cfg(all(feature = "schedule", feature = "winnt"))]
windows_link::link!("ntdsapi.dll" "system" fn DsReplicaAddW(hds : super::HANDLE, namecontext : windows_sys::core::PCWSTR, sourcedsadn : windows_sys::core::PCWSTR, transportdn : windows_sys::core::PCWSTR, sourcedsaaddress : windows_sys::core::PCWSTR, pschedule : *const super::SCHEDULE, options : u32) -> u32);
#[cfg(feature = "winnt")]
windows_link::link!("ntdsapi.dll" "system" fn DsReplicaConsistencyCheck(hds : super::HANDLE, taskid : DS_KCC_TASKID, dwflags : u32) -> u32);
#[cfg(feature = "winnt")]
windows_link::link!("ntdsapi.dll" "system" fn DsReplicaDelA(hds : super::HANDLE, namecontext : windows_sys::core::PCSTR, dsasrc : windows_sys::core::PCSTR, options : u32) -> u32);
#[cfg(feature = "winnt")]
windows_link::link!("ntdsapi.dll" "system" fn DsReplicaDelW(hds : super::HANDLE, namecontext : windows_sys::core::PCWSTR, dsasrc : windows_sys::core::PCWSTR, options : u32) -> u32);
windows_link::link!("ntdsapi.dll" "system" fn DsReplicaFreeInfo(infotype : DS_REPL_INFO_TYPE, pinfo : *const core::ffi::c_void));
#[cfg(feature = "winnt")]
windows_link::link!("ntdsapi.dll" "system" fn DsReplicaGetInfo2W(hds : super::HANDLE, infotype : DS_REPL_INFO_TYPE, pszobject : windows_sys::core::PCWSTR, puuidforsourcedsaobjguid : *const windows_sys::core::GUID, pszattributename : windows_sys::core::PCWSTR, pszvalue : windows_sys::core::PCWSTR, dwflags : u32, dwenumerationcontext : u32, ppinfo : *mut *mut core::ffi::c_void) -> u32);
#[cfg(feature = "winnt")]
windows_link::link!("ntdsapi.dll" "system" fn DsReplicaGetInfoW(hds : super::HANDLE, infotype : DS_REPL_INFO_TYPE, pszobject : windows_sys::core::PCWSTR, puuidforsourcedsaobjguid : *const windows_sys::core::GUID, ppinfo : *mut *mut core::ffi::c_void) -> u32);
#[cfg(all(feature = "schedule", feature = "winnt"))]
windows_link::link!("ntdsapi.dll" "system" fn DsReplicaModifyA(hds : super::HANDLE, namecontext : windows_sys::core::PCSTR, puuidsourcedsa : *const windows_sys::core::GUID, transportdn : windows_sys::core::PCSTR, sourcedsaaddress : windows_sys::core::PCSTR, pschedule : *const super::SCHEDULE, replicaflags : u32, modifyfields : u32, options : u32) -> u32);
#[cfg(all(feature = "schedule", feature = "winnt"))]
windows_link::link!("ntdsapi.dll" "system" fn DsReplicaModifyW(hds : super::HANDLE, namecontext : windows_sys::core::PCWSTR, puuidsourcedsa : *const windows_sys::core::GUID, transportdn : windows_sys::core::PCWSTR, sourcedsaaddress : windows_sys::core::PCWSTR, pschedule : *const super::SCHEDULE, replicaflags : u32, modifyfields : u32, options : u32) -> u32);
#[cfg(feature = "winnt")]
windows_link::link!("ntdsapi.dll" "system" fn DsReplicaSyncA(hds : super::HANDLE, namecontext : windows_sys::core::PCSTR, puuiddsasrc : *const windows_sys::core::GUID, options : u32) -> u32);
#[cfg(feature = "winnt")]
windows_link::link!("ntdsapi.dll" "system" fn DsReplicaSyncAllA(hds : super::HANDLE, psznamecontext : windows_sys::core::PCSTR, ulflags : u32, pfncallback : *mut u8, pcallbackdata : *const core::ffi::c_void, perrors : *mut *mut PDS_REPSYNCALL_ERRINFOA) -> u32);
#[cfg(feature = "winnt")]
windows_link::link!("ntdsapi.dll" "system" fn DsReplicaSyncAllW(hds : super::HANDLE, psznamecontext : windows_sys::core::PCWSTR, ulflags : u32, pfncallback : *mut u8, pcallbackdata : *const core::ffi::c_void, perrors : *mut *mut PDS_REPSYNCALL_ERRINFOW) -> u32);
#[cfg(feature = "winnt")]
windows_link::link!("ntdsapi.dll" "system" fn DsReplicaSyncW(hds : super::HANDLE, namecontext : windows_sys::core::PCWSTR, puuiddsasrc : *const windows_sys::core::GUID, options : u32) -> u32);
#[cfg(feature = "winnt")]
windows_link::link!("ntdsapi.dll" "system" fn DsReplicaUpdateRefsA(hds : super::HANDLE, namecontext : windows_sys::core::PCSTR, dsadest : windows_sys::core::PCSTR, puuiddsadest : *const windows_sys::core::GUID, options : u32) -> u32);
#[cfg(feature = "winnt")]
windows_link::link!("ntdsapi.dll" "system" fn DsReplicaUpdateRefsW(hds : super::HANDLE, namecontext : windows_sys::core::PCWSTR, dsadest : windows_sys::core::PCWSTR, puuiddsadest : *const windows_sys::core::GUID, options : u32) -> u32);
#[cfg(feature = "winnt")]
windows_link::link!("ntdsapi.dll" "system" fn DsReplicaVerifyObjectsA(hds : super::HANDLE, namecontext : windows_sys::core::PCSTR, puuiddsasrc : *const windows_sys::core::GUID, uloptions : u32) -> u32);
#[cfg(feature = "winnt")]
windows_link::link!("ntdsapi.dll" "system" fn DsReplicaVerifyObjectsW(hds : super::HANDLE, namecontext : windows_sys::core::PCWSTR, puuiddsasrc : *const windows_sys::core::GUID, uloptions : u32) -> u32);
windows_link::link!("ntdsapi.dll" "system" fn DsServerRegisterSpnA(operation : DS_SPN_WRITE_OP, serviceclass : windows_sys::core::PCSTR, userobjectdn : windows_sys::core::PCSTR) -> u32);
windows_link::link!("ntdsapi.dll" "system" fn DsServerRegisterSpnW(operation : DS_SPN_WRITE_OP, serviceclass : windows_sys::core::PCWSTR, userobjectdn : windows_sys::core::PCWSTR) -> u32);
#[cfg(feature = "winnt")]
windows_link::link!("ntdsapi.dll" "system" fn DsUnBindA(phds : *const super::HANDLE) -> u32);
#[cfg(feature = "winnt")]
windows_link::link!("ntdsapi.dll" "system" fn DsUnBindW(phds : *const super::HANDLE) -> u32);
#[cfg(feature = "winnt")]
windows_link::link!("ntdsapi.dll" "system" fn DsWriteAccountSpnA(hds : super::HANDLE, operation : DS_SPN_WRITE_OP, pszaccount : windows_sys::core::PCSTR, cspn : u32, rpszspn : *const windows_sys::core::PCSTR) -> u32);
#[cfg(feature = "winnt")]
windows_link::link!("ntdsapi.dll" "system" fn DsWriteAccountSpnW(hds : super::HANDLE, operation : DS_SPN_WRITE_OP, pszaccount : windows_sys::core::PCWSTR, cspn : u32, rpszspn : *const windows_sys::core::PCWSTR) -> u32);
pub const ADAM_REPL_AUTHENTICATION_MODE_MUTUAL_AUTH_REQUIRED: u32 = 2;
pub const ADAM_REPL_AUTHENTICATION_MODE_NEGOTIATE: u32 = 1;
pub const ADAM_REPL_AUTHENTICATION_MODE_NEGOTIATE_PASS_THROUGH: u32 = 0;
pub const ADAM_SCP_FSMO_NAMING_STRING: windows_sys::core::PCSTR = windows_sys::core::s!("naming");
pub const ADAM_SCP_FSMO_NAMING_STRING_W: windows_sys::core::PCWSTR = windows_sys::core::w!("naming");
pub const ADAM_SCP_FSMO_SCHEMA_STRING: windows_sys::core::PCSTR = windows_sys::core::s!("schema");
pub const ADAM_SCP_FSMO_SCHEMA_STRING_W: windows_sys::core::PCWSTR = windows_sys::core::w!("schema");
pub const ADAM_SCP_FSMO_STRING: windows_sys::core::PCSTR = windows_sys::core::s!("fsmo:");
pub const ADAM_SCP_FSMO_STRING_W: windows_sys::core::PCWSTR = windows_sys::core::w!("fsmo:");
pub const ADAM_SCP_INSTANCE_NAME_STRING: windows_sys::core::PCSTR = windows_sys::core::s!("instance:");
pub const ADAM_SCP_INSTANCE_NAME_STRING_W: windows_sys::core::PCWSTR = windows_sys::core::w!("instance:");
pub const ADAM_SCP_PARTITION_STRING: windows_sys::core::PCSTR = windows_sys::core::s!("partition:");
pub const ADAM_SCP_PARTITION_STRING_W: windows_sys::core::PCWSTR = windows_sys::core::w!("partition:");
pub const ADAM_SCP_SITE_NAME_STRING: windows_sys::core::PCSTR = windows_sys::core::s!("site:");
pub const ADAM_SCP_SITE_NAME_STRING_W: windows_sys::core::PCWSTR = windows_sys::core::w!("site:");
pub const DS_BEHAVIOR_LONGHORN: u32 = 3;
pub const DS_BEHAVIOR_WIN2000: u32 = 0;
pub const DS_BEHAVIOR_WIN2003: u32 = 2;
pub const DS_BEHAVIOR_WIN2003_WITH_MIXED_DOMAINS: u32 = 1;
pub const DS_BEHAVIOR_WIN2008: u32 = 3;
pub const DS_BEHAVIOR_WIN2008R2: u32 = 4;
pub const DS_BEHAVIOR_WIN2012: u32 = 5;
pub const DS_BEHAVIOR_WIN2012R2: u32 = 6;
pub const DS_BEHAVIOR_WIN2016: u32 = 7;
pub const DS_BEHAVIOR_WIN2025: u32 = 10;
pub const DS_BEHAVIOR_WIN7: u32 = 4;
pub const DS_BEHAVIOR_WIN8: u32 = 5;
pub const DS_BEHAVIOR_WINBLUE: u32 = 6;
pub const DS_BEHAVIOR_WINTHRESHOLD: u32 = 7;
pub const DS_CANONICAL_NAME: DS_NAME_FORMAT = 7;
pub const DS_CANONICAL_NAME_EX: DS_NAME_FORMAT = 9;
pub const DS_DEFAULT_LOCALE: u32 = 1033;
pub const DS_DEFAULT_LOCALE_COMPARE_FLAGS: u32 = 200707;
pub const DS_DISPLAY_NAME: DS_NAME_FORMAT = 3;
pub const DS_DNS_DOMAIN_NAME: DS_NAME_FORMAT = 12;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct DS_DOMAIN_CONTROLLER_INFO_1A {
    pub NetbiosName: windows_sys::core::PSTR,
    pub DnsHostName: windows_sys::core::PSTR,
    pub SiteName: windows_sys::core::PSTR,
    pub ComputerObjectName: windows_sys::core::PSTR,
    pub ServerObjectName: windows_sys::core::PSTR,
    pub fIsPdc: windows_sys::core::BOOL,
    pub fDsEnabled: windows_sys::core::BOOL,
}
impl Default for DS_DOMAIN_CONTROLLER_INFO_1A {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct DS_DOMAIN_CONTROLLER_INFO_1W {
    pub NetbiosName: windows_sys::core::PWSTR,
    pub DnsHostName: windows_sys::core::PWSTR,
    pub SiteName: windows_sys::core::PWSTR,
    pub ComputerObjectName: windows_sys::core::PWSTR,
    pub ServerObjectName: windows_sys::core::PWSTR,
    pub fIsPdc: windows_sys::core::BOOL,
    pub fDsEnabled: windows_sys::core::BOOL,
}
impl Default for DS_DOMAIN_CONTROLLER_INFO_1W {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct DS_DOMAIN_CONTROLLER_INFO_2A {
    pub NetbiosName: windows_sys::core::PSTR,
    pub DnsHostName: windows_sys::core::PSTR,
    pub SiteName: windows_sys::core::PSTR,
    pub SiteObjectName: windows_sys::core::PSTR,
    pub ComputerObjectName: windows_sys::core::PSTR,
    pub ServerObjectName: windows_sys::core::PSTR,
    pub NtdsDsaObjectName: windows_sys::core::PSTR,
    pub fIsPdc: windows_sys::core::BOOL,
    pub fDsEnabled: windows_sys::core::BOOL,
    pub fIsGc: windows_sys::core::BOOL,
    pub SiteObjectGuid: windows_sys::core::GUID,
    pub ComputerObjectGuid: windows_sys::core::GUID,
    pub ServerObjectGuid: windows_sys::core::GUID,
    pub NtdsDsaObjectGuid: windows_sys::core::GUID,
}
impl Default for DS_DOMAIN_CONTROLLER_INFO_2A {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct DS_DOMAIN_CONTROLLER_INFO_2W {
    pub NetbiosName: windows_sys::core::PWSTR,
    pub DnsHostName: windows_sys::core::PWSTR,
    pub SiteName: windows_sys::core::PWSTR,
    pub SiteObjectName: windows_sys::core::PWSTR,
    pub ComputerObjectName: windows_sys::core::PWSTR,
    pub ServerObjectName: windows_sys::core::PWSTR,
    pub NtdsDsaObjectName: windows_sys::core::PWSTR,
    pub fIsPdc: windows_sys::core::BOOL,
    pub fDsEnabled: windows_sys::core::BOOL,
    pub fIsGc: windows_sys::core::BOOL,
    pub SiteObjectGuid: windows_sys::core::GUID,
    pub ComputerObjectGuid: windows_sys::core::GUID,
    pub ServerObjectGuid: windows_sys::core::GUID,
    pub NtdsDsaObjectGuid: windows_sys::core::GUID,
}
impl Default for DS_DOMAIN_CONTROLLER_INFO_2W {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct DS_DOMAIN_CONTROLLER_INFO_3A {
    pub NetbiosName: windows_sys::core::PSTR,
    pub DnsHostName: windows_sys::core::PSTR,
    pub SiteName: windows_sys::core::PSTR,
    pub SiteObjectName: windows_sys::core::PSTR,
    pub ComputerObjectName: windows_sys::core::PSTR,
    pub ServerObjectName: windows_sys::core::PSTR,
    pub NtdsDsaObjectName: windows_sys::core::PSTR,
    pub fIsPdc: windows_sys::core::BOOL,
    pub fDsEnabled: windows_sys::core::BOOL,
    pub fIsGc: windows_sys::core::BOOL,
    pub fIsRodc: windows_sys::core::BOOL,
    pub SiteObjectGuid: windows_sys::core::GUID,
    pub ComputerObjectGuid: windows_sys::core::GUID,
    pub ServerObjectGuid: windows_sys::core::GUID,
    pub NtdsDsaObjectGuid: windows_sys::core::GUID,
}
impl Default for DS_DOMAIN_CONTROLLER_INFO_3A {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct DS_DOMAIN_CONTROLLER_INFO_3W {
    pub NetbiosName: windows_sys::core::PWSTR,
    pub DnsHostName: windows_sys::core::PWSTR,
    pub SiteName: windows_sys::core::PWSTR,
    pub SiteObjectName: windows_sys::core::PWSTR,
    pub ComputerObjectName: windows_sys::core::PWSTR,
    pub ServerObjectName: windows_sys::core::PWSTR,
    pub NtdsDsaObjectName: windows_sys::core::PWSTR,
    pub fIsPdc: windows_sys::core::BOOL,
    pub fDsEnabled: windows_sys::core::BOOL,
    pub fIsGc: windows_sys::core::BOOL,
    pub fIsRodc: windows_sys::core::BOOL,
    pub SiteObjectGuid: windows_sys::core::GUID,
    pub ComputerObjectGuid: windows_sys::core::GUID,
    pub ServerObjectGuid: windows_sys::core::GUID,
    pub NtdsDsaObjectGuid: windows_sys::core::GUID,
}
impl Default for DS_DOMAIN_CONTROLLER_INFO_3W {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const DS_DOMAIN_SIMPLE_NAME: u32 = 8;
pub const DS_ENTERPRISE_SIMPLE_NAME: u32 = 8;
pub const DS_EXIST_ADVISORY_MODE: u32 = 1;
pub const DS_FQDN_1779_NAME: DS_NAME_FORMAT = 1;
pub const DS_INSTANCETYPE_IS_NC_HEAD: u32 = 1;
pub const DS_INSTANCETYPE_NC_COMING: u32 = 16;
pub const DS_INSTANCETYPE_NC_GOING: u32 = 32;
pub const DS_INSTANCETYPE_NC_IS_WRITEABLE: u32 = 4;
pub const DS_KCC_FLAG_ASYNC_OP: u32 = 1;
pub const DS_KCC_FLAG_DAMPED: u32 = 2;
pub type DS_KCC_TASKID = i32;
pub const DS_KCC_TASKID_UPDATE_TOPOLOGY: DS_KCC_TASKID = 0;
pub const DS_LIST_ACCOUNT_OBJECT_FOR_SERVER: u32 = 2;
pub const DS_LIST_DNS_HOST_NAME_FOR_SERVER: u32 = 1;
pub const DS_LIST_DSA_OBJECT_FOR_SERVER: u32 = 0;
pub type DS_NAME_ERROR = i32;
pub const DS_NAME_ERROR_DOMAIN_ONLY: DS_NAME_ERROR = 5;
pub const DS_NAME_ERROR_NOT_FOUND: DS_NAME_ERROR = 2;
pub const DS_NAME_ERROR_NOT_UNIQUE: DS_NAME_ERROR = 3;
pub const DS_NAME_ERROR_NO_MAPPING: DS_NAME_ERROR = 4;
pub const DS_NAME_ERROR_NO_SYNTACTICAL_MAPPING: DS_NAME_ERROR = 6;
pub const DS_NAME_ERROR_RESOLVING: DS_NAME_ERROR = 1;
pub const DS_NAME_ERROR_TRUST_REFERRAL: DS_NAME_ERROR = 7;
pub type DS_NAME_FLAGS = i32;
pub const DS_NAME_FLAG_EVAL_AT_DC: DS_NAME_FLAGS = 2;
pub const DS_NAME_FLAG_GCVERIFY: DS_NAME_FLAGS = 4;
pub const DS_NAME_FLAG_SYNTACTICAL_ONLY: DS_NAME_FLAGS = 1;
pub const DS_NAME_FLAG_TRUST_REFERRAL: DS_NAME_FLAGS = 8;
pub type DS_NAME_FORMAT = i32;
pub const DS_NAME_LEGAL_FLAGS: u32 = 1;
pub const DS_NAME_NO_ERROR: DS_NAME_ERROR = 0;
pub const DS_NAME_NO_FLAGS: DS_NAME_FLAGS = 0;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct DS_NAME_RESULTA {
    pub cItems: u32,
    pub rItems: PDS_NAME_RESULT_ITEMA,
}
impl Default for DS_NAME_RESULTA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct DS_NAME_RESULTW {
    pub cItems: u32,
    pub rItems: PDS_NAME_RESULT_ITEMW,
}
impl Default for DS_NAME_RESULTW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct DS_NAME_RESULT_ITEMA {
    pub status: u32,
    pub pDomain: windows_sys::core::PSTR,
    pub pName: windows_sys::core::PSTR,
}
impl Default for DS_NAME_RESULT_ITEMA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct DS_NAME_RESULT_ITEMW {
    pub status: u32,
    pub pDomain: windows_sys::core::PWSTR,
    pub pName: windows_sys::core::PWSTR,
}
impl Default for DS_NAME_RESULT_ITEMW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const DS_NT4_ACCOUNT_NAME: DS_NAME_FORMAT = 2;
pub const DS_REPADD_ASYNCHRONOUS_OPERATION: u32 = 1;
pub const DS_REPADD_ASYNCHRONOUS_REPLICA: u32 = 32;
pub const DS_REPADD_CRITICAL: u32 = 2048;
pub const DS_REPADD_DISABLE_NOTIFICATION: u32 = 64;
pub const DS_REPADD_DISABLE_PERIODIC: u32 = 128;
pub const DS_REPADD_INITIAL: u32 = 4;
pub const DS_REPADD_INTERSITE_MESSAGING: u32 = 16;
pub const DS_REPADD_NEVER_NOTIFY: u32 = 512;
pub const DS_REPADD_NONGC_RO_REPLICA: u32 = 16777216;
pub const DS_REPADD_PERIODIC: u32 = 8;
pub const DS_REPADD_SELECT_SECRETS: u32 = 4096;
pub const DS_REPADD_TWO_WAY: u32 = 1024;
pub const DS_REPADD_USE_COMPRESSION: u32 = 256;
pub const DS_REPADD_WRITEABLE: u32 = 2;
pub const DS_REPDEL_ASYNCHRONOUS_OPERATION: u32 = 1;
pub const DS_REPDEL_IGNORE_ERRORS: u32 = 8;
pub const DS_REPDEL_INTERSITE_MESSAGING: u32 = 4;
pub const DS_REPDEL_LOCAL_ONLY: u32 = 16;
pub const DS_REPDEL_NO_SOURCE: u32 = 32;
pub const DS_REPDEL_REF_OK: u32 = 64;
pub const DS_REPDEL_WRITEABLE: u32 = 2;
#[repr(C)]
#[cfg(all(feature = "minwindef", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct DS_REPL_ATTR_META_DATA {
    pub pszAttributeName: windows_sys::core::PWSTR,
    pub dwVersion: u32,
    pub ftimeLastOriginatingChange: super::FILETIME,
    pub uuidLastOriginatingDsaInvocationID: windows_sys::core::GUID,
    pub usnOriginatingChange: super::USN,
    pub usnLocalChange: super::USN,
}
#[cfg(all(feature = "minwindef", feature = "winnt"))]
impl Default for DS_REPL_ATTR_META_DATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "minwindef", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct DS_REPL_ATTR_META_DATA_2 {
    pub pszAttributeName: windows_sys::core::PWSTR,
    pub dwVersion: u32,
    pub ftimeLastOriginatingChange: super::FILETIME,
    pub uuidLastOriginatingDsaInvocationID: windows_sys::core::GUID,
    pub usnOriginatingChange: super::USN,
    pub usnLocalChange: super::USN,
    pub pszLastOriginatingDsaDN: windows_sys::core::PWSTR,
}
#[cfg(all(feature = "minwindef", feature = "winnt"))]
impl Default for DS_REPL_ATTR_META_DATA_2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "minwindef", feature = "winnt"))]
#[derive(Clone, Copy, Default)]
pub struct DS_REPL_ATTR_META_DATA_BLOB {
    pub oszAttributeName: u32,
    pub dwVersion: u32,
    pub ftimeLastOriginatingChange: super::FILETIME,
    pub uuidLastOriginatingDsaInvocationID: windows_sys::core::GUID,
    pub usnOriginatingChange: super::USN,
    pub usnLocalChange: super::USN,
    pub oszLastOriginatingDsaDN: u32,
}
#[repr(C)]
#[cfg(all(feature = "minwindef", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct DS_REPL_ATTR_VALUE_META_DATA {
    pub cNumEntries: u32,
    pub dwEnumerationContext: u32,
    pub rgMetaData: [DS_REPL_VALUE_META_DATA; 1],
}
#[cfg(all(feature = "minwindef", feature = "winnt"))]
impl Default for DS_REPL_ATTR_VALUE_META_DATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "minwindef", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct DS_REPL_ATTR_VALUE_META_DATA_2 {
    pub cNumEntries: u32,
    pub dwEnumerationContext: u32,
    pub rgMetaData: [DS_REPL_VALUE_META_DATA_2; 1],
}
#[cfg(all(feature = "minwindef", feature = "winnt"))]
impl Default for DS_REPL_ATTR_VALUE_META_DATA_2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "minwindef", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct DS_REPL_ATTR_VALUE_META_DATA_EXT {
    pub cNumEntries: u32,
    pub dwEnumerationContext: u32,
    pub rgMetaData: [DS_REPL_VALUE_META_DATA_EXT; 1],
}
#[cfg(all(feature = "minwindef", feature = "winnt"))]
impl Default for DS_REPL_ATTR_VALUE_META_DATA_EXT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy, Default)]
pub struct DS_REPL_CURSOR {
    pub uuidSourceDsaInvocationID: windows_sys::core::GUID,
    pub usnAttributeFilter: super::USN,
}
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy)]
pub struct DS_REPL_CURSORS {
    pub cNumCursors: u32,
    pub dwReserved: u32,
    pub rgCursor: [DS_REPL_CURSOR; 1],
}
#[cfg(feature = "winnt")]
impl Default for DS_REPL_CURSORS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "minwindef", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct DS_REPL_CURSORS_2 {
    pub cNumCursors: u32,
    pub dwEnumerationContext: u32,
    pub rgCursor: [DS_REPL_CURSOR_2; 1],
}
#[cfg(all(feature = "minwindef", feature = "winnt"))]
impl Default for DS_REPL_CURSORS_2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "minwindef", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct DS_REPL_CURSORS_3W {
    pub cNumCursors: u32,
    pub dwEnumerationContext: u32,
    pub rgCursor: [DS_REPL_CURSOR_3W; 1],
}
#[cfg(all(feature = "minwindef", feature = "winnt"))]
impl Default for DS_REPL_CURSORS_3W {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "minwindef", feature = "winnt"))]
#[derive(Clone, Copy, Default)]
pub struct DS_REPL_CURSOR_2 {
    pub uuidSourceDsaInvocationID: windows_sys::core::GUID,
    pub usnAttributeFilter: super::USN,
    pub ftimeLastSyncSuccess: super::FILETIME,
}
#[repr(C)]
#[cfg(all(feature = "minwindef", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct DS_REPL_CURSOR_3W {
    pub uuidSourceDsaInvocationID: windows_sys::core::GUID,
    pub usnAttributeFilter: super::USN,
    pub ftimeLastSyncSuccess: super::FILETIME,
    pub pszSourceDsaDN: windows_sys::core::PWSTR,
}
#[cfg(all(feature = "minwindef", feature = "winnt"))]
impl Default for DS_REPL_CURSOR_3W {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "minwindef", feature = "winnt"))]
#[derive(Clone, Copy, Default)]
pub struct DS_REPL_CURSOR_BLOB {
    pub uuidSourceDsaInvocationID: windows_sys::core::GUID,
    pub usnAttributeFilter: super::USN,
    pub ftimeLastSyncSuccess: super::FILETIME,
    pub oszSourceDsaDN: u32,
}
pub const DS_REPL_INFO_CURSORS_2_FOR_NC: DS_REPL_INFO_TYPE = 7;
pub const DS_REPL_INFO_CURSORS_3_FOR_NC: DS_REPL_INFO_TYPE = 8;
pub const DS_REPL_INFO_CURSORS_FOR_NC: DS_REPL_INFO_TYPE = 1;
pub const DS_REPL_INFO_FLAG_IMPROVE_LINKED_ATTRS: u32 = 1;
pub const DS_REPL_INFO_KCC_DSA_CONNECT_FAILURES: DS_REPL_INFO_TYPE = 3;
pub const DS_REPL_INFO_KCC_DSA_LINK_FAILURES: DS_REPL_INFO_TYPE = 4;
pub const DS_REPL_INFO_METADATA_2_FOR_ATTR_VALUE: DS_REPL_INFO_TYPE = 10;
pub const DS_REPL_INFO_METADATA_2_FOR_OBJ: DS_REPL_INFO_TYPE = 9;
pub const DS_REPL_INFO_METADATA_EXT_FOR_ATTR_VALUE: DS_REPL_INFO_TYPE = 11;
pub const DS_REPL_INFO_METADATA_FOR_ATTR_VALUE: DS_REPL_INFO_TYPE = 6;
pub const DS_REPL_INFO_METADATA_FOR_OBJ: DS_REPL_INFO_TYPE = 2;
pub const DS_REPL_INFO_NEIGHBORS: DS_REPL_INFO_TYPE = 0;
pub const DS_REPL_INFO_PENDING_OPS: DS_REPL_INFO_TYPE = 5;
pub type DS_REPL_INFO_TYPE = i32;
pub const DS_REPL_INFO_TYPE_MAX: DS_REPL_INFO_TYPE = 12;
#[repr(C)]
#[cfg(feature = "minwindef")]
#[derive(Clone, Copy)]
pub struct DS_REPL_KCC_DSA_FAILURESW {
    pub cNumEntries: u32,
    pub dwReserved: u32,
    pub rgDsaFailure: [DS_REPL_KCC_DSA_FAILUREW; 1],
}
#[cfg(feature = "minwindef")]
impl Default for DS_REPL_KCC_DSA_FAILURESW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "minwindef")]
#[derive(Clone, Copy)]
pub struct DS_REPL_KCC_DSA_FAILUREW {
    pub pszDsaDN: windows_sys::core::PWSTR,
    pub uuidDsaObjGuid: windows_sys::core::GUID,
    pub ftimeFirstFailure: super::FILETIME,
    pub cNumFailures: u32,
    pub dwLastResult: u32,
}
#[cfg(feature = "minwindef")]
impl Default for DS_REPL_KCC_DSA_FAILUREW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "minwindef")]
#[derive(Clone, Copy, Default)]
pub struct DS_REPL_KCC_DSA_FAILUREW_BLOB {
    pub oszDsaDN: u32,
    pub uuidDsaObjGuid: windows_sys::core::GUID,
    pub ftimeFirstFailure: super::FILETIME,
    pub cNumFailures: u32,
    pub dwLastResult: u32,
}
pub const DS_REPL_NBR_COMPRESS_CHANGES: u32 = 268435456;
pub const DS_REPL_NBR_DISABLE_SCHEDULED_SYNC: u32 = 134217728;
pub const DS_REPL_NBR_DO_SCHEDULED_SYNCS: u32 = 64;
pub const DS_REPL_NBR_FULL_SYNC_IN_PROGRESS: u32 = 65536;
pub const DS_REPL_NBR_FULL_SYNC_NEXT_PACKET: u32 = 131072;
pub const DS_REPL_NBR_GCSPN: u32 = 1048576;
pub const DS_REPL_NBR_IGNORE_CHANGE_NOTIFICATIONS: u32 = 67108864;
pub const DS_REPL_NBR_MODIFIABLE_MASK: u32 = 1006633568;
pub const DS_REPL_NBR_NEVER_SYNCED: u32 = 2097152;
pub const DS_REPL_NBR_NONGC_RO_REPLICA: u32 = 1024;
pub const DS_REPL_NBR_NO_CHANGE_NOTIFICATIONS: u32 = 536870912;
pub const DS_REPL_NBR_PARTIAL_ATTRIBUTE_SET: u32 = 1073741824;
pub const DS_REPL_NBR_PREEMPTED: u32 = 16777216;
pub const DS_REPL_NBR_RETURN_OBJECT_PARENTS: u32 = 2048;
pub const DS_REPL_NBR_SELECT_SECRETS: u32 = 4096;
pub const DS_REPL_NBR_SYNC_ON_STARTUP: u32 = 32;
pub const DS_REPL_NBR_TWO_WAY_SYNC: u32 = 512;
pub const DS_REPL_NBR_USE_ASYNC_INTERSITE_TRANSPORT: u32 = 128;
pub const DS_REPL_NBR_WRITEABLE: u32 = 16;
#[repr(C)]
#[cfg(all(feature = "minwindef", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct DS_REPL_NEIGHBORSW {
    pub cNumNeighbors: u32,
    pub dwReserved: u32,
    pub rgNeighbor: [DS_REPL_NEIGHBORW; 1],
}
#[cfg(all(feature = "minwindef", feature = "winnt"))]
impl Default for DS_REPL_NEIGHBORSW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "minwindef", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct DS_REPL_NEIGHBORW {
    pub pszNamingContext: windows_sys::core::PWSTR,
    pub pszSourceDsaDN: windows_sys::core::PWSTR,
    pub pszSourceDsaAddress: windows_sys::core::PWSTR,
    pub pszAsyncIntersiteTransportDN: windows_sys::core::PWSTR,
    pub dwReplicaFlags: u32,
    pub dwReserved: u32,
    pub uuidNamingContextObjGuid: windows_sys::core::GUID,
    pub uuidSourceDsaObjGuid: windows_sys::core::GUID,
    pub uuidSourceDsaInvocationID: windows_sys::core::GUID,
    pub uuidAsyncIntersiteTransportObjGuid: windows_sys::core::GUID,
    pub usnLastObjChangeSynced: super::USN,
    pub usnAttributeFilter: super::USN,
    pub ftimeLastSyncSuccess: super::FILETIME,
    pub ftimeLastSyncAttempt: super::FILETIME,
    pub dwLastSyncResult: u32,
    pub cNumConsecutiveSyncFailures: u32,
}
#[cfg(all(feature = "minwindef", feature = "winnt"))]
impl Default for DS_REPL_NEIGHBORW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "minwindef", feature = "winnt"))]
#[derive(Clone, Copy, Default)]
pub struct DS_REPL_NEIGHBORW_BLOB {
    pub oszNamingContext: u32,
    pub oszSourceDsaDN: u32,
    pub oszSourceDsaAddress: u32,
    pub oszAsyncIntersiteTransportDN: u32,
    pub dwReplicaFlags: u32,
    pub dwReserved: u32,
    pub uuidNamingContextObjGuid: windows_sys::core::GUID,
    pub uuidSourceDsaObjGuid: windows_sys::core::GUID,
    pub uuidSourceDsaInvocationID: windows_sys::core::GUID,
    pub uuidAsyncIntersiteTransportObjGuid: windows_sys::core::GUID,
    pub usnLastObjChangeSynced: super::USN,
    pub usnAttributeFilter: super::USN,
    pub ftimeLastSyncSuccess: super::FILETIME,
    pub ftimeLastSyncAttempt: super::FILETIME,
    pub dwLastSyncResult: u32,
    pub cNumConsecutiveSyncFailures: u32,
}
#[repr(C)]
#[cfg(all(feature = "minwindef", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct DS_REPL_OBJ_META_DATA {
    pub cNumEntries: u32,
    pub dwReserved: u32,
    pub rgMetaData: [DS_REPL_ATTR_META_DATA; 1],
}
#[cfg(all(feature = "minwindef", feature = "winnt"))]
impl Default for DS_REPL_OBJ_META_DATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "minwindef", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct DS_REPL_OBJ_META_DATA_2 {
    pub cNumEntries: u32,
    pub dwReserved: u32,
    pub rgMetaData: [DS_REPL_ATTR_META_DATA_2; 1],
}
#[cfg(all(feature = "minwindef", feature = "winnt"))]
impl Default for DS_REPL_OBJ_META_DATA_2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "minwindef")]
#[derive(Clone, Copy)]
pub struct DS_REPL_OPW {
    pub ftimeEnqueued: super::FILETIME,
    pub ulSerialNumber: u32,
    pub ulPriority: u32,
    pub OpType: DS_REPL_OP_TYPE,
    pub ulOptions: u32,
    pub pszNamingContext: windows_sys::core::PWSTR,
    pub pszDsaDN: windows_sys::core::PWSTR,
    pub pszDsaAddress: windows_sys::core::PWSTR,
    pub uuidNamingContextObjGuid: windows_sys::core::GUID,
    pub uuidDsaObjGuid: windows_sys::core::GUID,
}
#[cfg(feature = "minwindef")]
impl Default for DS_REPL_OPW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "minwindef")]
#[derive(Clone, Copy, Default)]
pub struct DS_REPL_OPW_BLOB {
    pub ftimeEnqueued: super::FILETIME,
    pub ulSerialNumber: u32,
    pub ulPriority: u32,
    pub OpType: DS_REPL_OP_TYPE,
    pub ulOptions: u32,
    pub oszNamingContext: u32,
    pub oszDsaDN: u32,
    pub oszDsaAddress: u32,
    pub uuidNamingContextObjGuid: windows_sys::core::GUID,
    pub uuidDsaObjGuid: windows_sys::core::GUID,
}
pub type DS_REPL_OP_TYPE = i32;
pub const DS_REPL_OP_TYPE_ADD: DS_REPL_OP_TYPE = 1;
pub const DS_REPL_OP_TYPE_DELETE: DS_REPL_OP_TYPE = 2;
pub const DS_REPL_OP_TYPE_MODIFY: DS_REPL_OP_TYPE = 3;
pub const DS_REPL_OP_TYPE_SYNC: DS_REPL_OP_TYPE = 0;
pub const DS_REPL_OP_TYPE_UPDATE_REFS: DS_REPL_OP_TYPE = 4;
#[repr(C)]
#[cfg(feature = "minwindef")]
#[derive(Clone, Copy)]
pub struct DS_REPL_PENDING_OPSW {
    pub ftimeCurrentOpStarted: super::FILETIME,
    pub cNumPendingOps: u32,
    pub rgPendingOp: [DS_REPL_OPW; 1],
}
#[cfg(feature = "minwindef")]
impl Default for DS_REPL_PENDING_OPSW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "minwindef")]
#[derive(Clone, Copy, Default)]
pub struct DS_REPL_QUEUE_STATISTICSW {
    pub ftimeCurrentOpStarted: super::FILETIME,
    pub cNumPendingOps: u32,
    pub ftimeOldestSync: super::FILETIME,
    pub ftimeOldestAdd: super::FILETIME,
    pub ftimeOldestMod: super::FILETIME,
    pub ftimeOldestDel: super::FILETIME,
    pub ftimeOldestUpdRefs: super::FILETIME,
}
#[cfg(feature = "minwindef")]
pub type DS_REPL_QUEUE_STATISTICSW_BLOB = DS_REPL_QUEUE_STATISTICSW;
#[repr(C)]
#[cfg(all(feature = "minwindef", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct DS_REPL_VALUE_META_DATA {
    pub pszAttributeName: windows_sys::core::PWSTR,
    pub pszObjectDn: windows_sys::core::PWSTR,
    pub cbData: u32,
    pub pbData: *mut u8,
    pub ftimeDeleted: super::FILETIME,
    pub ftimeCreated: super::FILETIME,
    pub dwVersion: u32,
    pub ftimeLastOriginatingChange: super::FILETIME,
    pub uuidLastOriginatingDsaInvocationID: windows_sys::core::GUID,
    pub usnOriginatingChange: super::USN,
    pub usnLocalChange: super::USN,
}
#[cfg(all(feature = "minwindef", feature = "winnt"))]
impl Default for DS_REPL_VALUE_META_DATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "minwindef", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct DS_REPL_VALUE_META_DATA_2 {
    pub pszAttributeName: windows_sys::core::PWSTR,
    pub pszObjectDn: windows_sys::core::PWSTR,
    pub cbData: u32,
    pub pbData: *mut u8,
    pub ftimeDeleted: super::FILETIME,
    pub ftimeCreated: super::FILETIME,
    pub dwVersion: u32,
    pub ftimeLastOriginatingChange: super::FILETIME,
    pub uuidLastOriginatingDsaInvocationID: windows_sys::core::GUID,
    pub usnOriginatingChange: super::USN,
    pub usnLocalChange: super::USN,
    pub pszLastOriginatingDsaDN: windows_sys::core::PWSTR,
}
#[cfg(all(feature = "minwindef", feature = "winnt"))]
impl Default for DS_REPL_VALUE_META_DATA_2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "minwindef", feature = "winnt"))]
#[derive(Clone, Copy, Default)]
pub struct DS_REPL_VALUE_META_DATA_BLOB {
    pub oszAttributeName: u32,
    pub oszObjectDn: u32,
    pub cbData: u32,
    pub obData: u32,
    pub ftimeDeleted: super::FILETIME,
    pub ftimeCreated: super::FILETIME,
    pub dwVersion: u32,
    pub ftimeLastOriginatingChange: super::FILETIME,
    pub uuidLastOriginatingDsaInvocationID: windows_sys::core::GUID,
    pub usnOriginatingChange: super::USN,
    pub usnLocalChange: super::USN,
    pub oszLastOriginatingDsaDN: u32,
}
#[repr(C)]
#[cfg(all(feature = "minwindef", feature = "winnt"))]
#[derive(Clone, Copy, Default)]
pub struct DS_REPL_VALUE_META_DATA_BLOB_EXT {
    pub oszAttributeName: u32,
    pub oszObjectDn: u32,
    pub cbData: u32,
    pub obData: u32,
    pub ftimeDeleted: super::FILETIME,
    pub ftimeCreated: super::FILETIME,
    pub dwVersion: u32,
    pub ftimeLastOriginatingChange: super::FILETIME,
    pub uuidLastOriginatingDsaInvocationID: windows_sys::core::GUID,
    pub usnOriginatingChange: super::USN,
    pub usnLocalChange: super::USN,
    pub oszLastOriginatingDsaDN: u32,
    pub dwUserIdentifier: u32,
    pub dwPriorLinkState: u32,
    pub dwCurrentLinkState: u32,
}
#[repr(C)]
#[cfg(all(feature = "minwindef", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct DS_REPL_VALUE_META_DATA_EXT {
    pub pszAttributeName: windows_sys::core::PWSTR,
    pub pszObjectDn: windows_sys::core::PWSTR,
    pub cbData: u32,
    pub pbData: *mut u8,
    pub ftimeDeleted: super::FILETIME,
    pub ftimeCreated: super::FILETIME,
    pub dwVersion: u32,
    pub ftimeLastOriginatingChange: super::FILETIME,
    pub uuidLastOriginatingDsaInvocationID: windows_sys::core::GUID,
    pub usnOriginatingChange: super::USN,
    pub usnLocalChange: super::USN,
    pub pszLastOriginatingDsaDN: windows_sys::core::PWSTR,
    pub dwUserIdentifier: u32,
    pub dwPriorLinkState: u32,
    pub dwCurrentLinkState: u32,
}
#[cfg(all(feature = "minwindef", feature = "winnt"))]
impl Default for DS_REPL_VALUE_META_DATA_EXT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const DS_REPMOD_ASYNCHRONOUS_OPERATION: u32 = 1;
pub const DS_REPMOD_UPDATE_ADDRESS: u32 = 2;
pub const DS_REPMOD_UPDATE_FLAGS: u32 = 1;
pub const DS_REPMOD_UPDATE_INSTANCE: u32 = 2;
pub const DS_REPMOD_UPDATE_RESULT: u32 = 8;
pub const DS_REPMOD_UPDATE_SCHEDULE: u32 = 4;
pub const DS_REPMOD_UPDATE_TRANSPORT: u32 = 16;
pub const DS_REPMOD_WRITEABLE: u32 = 2;
pub const DS_REPSYNCALL_ABORT_IF_SERVER_UNAVAILABLE: u32 = 1;
pub const DS_REPSYNCALL_CROSS_SITE_BOUNDARIES: u32 = 64;
pub const DS_REPSYNCALL_DO_NOT_SYNC: u32 = 8;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct DS_REPSYNCALL_ERRINFOA {
    pub pszSvrId: windows_sys::core::PSTR,
    pub error: DS_REPSYNCALL_ERROR,
    pub dwWin32Err: u32,
    pub pszSrcId: windows_sys::core::PSTR,
}
impl Default for DS_REPSYNCALL_ERRINFOA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct DS_REPSYNCALL_ERRINFOW {
    pub pszSvrId: windows_sys::core::PWSTR,
    pub error: DS_REPSYNCALL_ERROR,
    pub dwWin32Err: u32,
    pub pszSrcId: windows_sys::core::PWSTR,
}
impl Default for DS_REPSYNCALL_ERRINFOW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type DS_REPSYNCALL_ERROR = i32;
pub type DS_REPSYNCALL_EVENT = i32;
pub const DS_REPSYNCALL_EVENT_ERROR: DS_REPSYNCALL_EVENT = 0;
pub const DS_REPSYNCALL_EVENT_FINISHED: DS_REPSYNCALL_EVENT = 3;
pub const DS_REPSYNCALL_EVENT_SYNC_COMPLETED: DS_REPSYNCALL_EVENT = 2;
pub const DS_REPSYNCALL_EVENT_SYNC_STARTED: DS_REPSYNCALL_EVENT = 1;
pub const DS_REPSYNCALL_ID_SERVERS_BY_DN: u32 = 4;
pub const DS_REPSYNCALL_NO_OPTIONS: u32 = 0;
pub const DS_REPSYNCALL_PUSH_CHANGES_OUTWARD: u32 = 32;
pub const DS_REPSYNCALL_SERVER_UNREACHABLE: DS_REPSYNCALL_ERROR = 2;
pub const DS_REPSYNCALL_SKIP_INITIAL_CHECK: u32 = 16;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct DS_REPSYNCALL_SYNCA {
    pub pszSrcId: windows_sys::core::PSTR,
    pub pszDstId: windows_sys::core::PSTR,
    pub pszNC: windows_sys::core::PSTR,
    pub pguidSrc: *mut windows_sys::core::GUID,
    pub pguidDst: *mut windows_sys::core::GUID,
}
impl Default for DS_REPSYNCALL_SYNCA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct DS_REPSYNCALL_SYNCW {
    pub pszSrcId: windows_sys::core::PWSTR,
    pub pszDstId: windows_sys::core::PWSTR,
    pub pszNC: windows_sys::core::PWSTR,
    pub pguidSrc: *mut windows_sys::core::GUID,
    pub pguidDst: *mut windows_sys::core::GUID,
}
impl Default for DS_REPSYNCALL_SYNCW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const DS_REPSYNCALL_SYNC_ADJACENT_SERVERS_ONLY: u32 = 2;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct DS_REPSYNCALL_UPDATEA {
    pub event: DS_REPSYNCALL_EVENT,
    pub pErrInfo: *mut DS_REPSYNCALL_ERRINFOA,
    pub pSync: *mut DS_REPSYNCALL_SYNCA,
}
impl Default for DS_REPSYNCALL_UPDATEA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct DS_REPSYNCALL_UPDATEW {
    pub event: DS_REPSYNCALL_EVENT,
    pub pErrInfo: *mut DS_REPSYNCALL_ERRINFOW,
    pub pSync: *mut DS_REPSYNCALL_SYNCW,
}
impl Default for DS_REPSYNCALL_UPDATEW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const DS_REPSYNCALL_WIN32_ERROR_CONTACTING_SERVER: DS_REPSYNCALL_ERROR = 0;
pub const DS_REPSYNCALL_WIN32_ERROR_REPLICATING: DS_REPSYNCALL_ERROR = 1;
pub const DS_REPSYNC_ABANDONED: u32 = 32768;
pub const DS_REPSYNC_ADD_REFERENCE: u32 = 512;
pub const DS_REPSYNC_ASYNCHRONOUS_OPERATION: u32 = 1;
pub const DS_REPSYNC_ASYNCHRONOUS_REPLICA: u32 = 1048576;
pub const DS_REPSYNC_CRITICAL: u32 = 2097152;
pub const DS_REPSYNC_FORCE: u32 = 256;
pub const DS_REPSYNC_FULL: u32 = 32;
pub const DS_REPSYNC_FULL_IN_PROGRESS: u32 = 4194304;
pub const DS_REPSYNC_INITIAL: u32 = 8192;
pub const DS_REPSYNC_INITIAL_IN_PROGRESS: u32 = 65536;
pub const DS_REPSYNC_INTERSITE_MESSAGING: u32 = 8;
pub const DS_REPSYNC_NEVER_COMPLETED: u32 = 1024;
pub const DS_REPSYNC_NEVER_NOTIFY: u32 = 4096;
pub const DS_REPSYNC_NONGC_RO_REPLICA: u32 = 16777216;
pub const DS_REPSYNC_NOTIFICATION: u32 = 524288;
pub const DS_REPSYNC_NO_DISCARD: u32 = 128;
pub const DS_REPSYNC_PARTIAL_ATTRIBUTE_SET: u32 = 131072;
pub const DS_REPSYNC_PERIODIC: u32 = 4;
pub const DS_REPSYNC_PREEMPTED: u32 = 8388608;
pub const DS_REPSYNC_REQUEUE: u32 = 262144;
pub const DS_REPSYNC_SELECT_SECRETS: u32 = 32768;
pub const DS_REPSYNC_TWO_WAY: u32 = 2048;
pub const DS_REPSYNC_URGENT: u32 = 64;
pub const DS_REPSYNC_USE_COMPRESSION: u32 = 16384;
pub const DS_REPSYNC_WRITEABLE: u32 = 2;
pub const DS_REPUPD_ADD_REFERENCE: u32 = 4;
pub const DS_REPUPD_ASYNCHRONOUS_OPERATION: u32 = 1;
pub const DS_REPUPD_DELETE_REFERENCE: u32 = 8;
pub const DS_REPUPD_REFERENCE_GCSPN: u32 = 16;
pub const DS_REPUPD_WRITEABLE: u32 = 2;
pub const DS_ROLE_DOMAIN_OWNER: u32 = 1;
pub const DS_ROLE_INFRASTRUCTURE_OWNER: u32 = 4;
pub const DS_ROLE_PDC_OWNER: u32 = 2;
pub const DS_ROLE_RID_OWNER: u32 = 3;
pub const DS_ROLE_SCHEMA_OWNER: u32 = 0;
pub const DS_SCHEMA_GUID_ATTR: u32 = 1;
pub const DS_SCHEMA_GUID_ATTR_SET: u32 = 2;
pub const DS_SCHEMA_GUID_CLASS: u32 = 3;
pub const DS_SCHEMA_GUID_CONTROL_RIGHT: u32 = 4;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct DS_SCHEMA_GUID_MAPA {
    pub guid: windows_sys::core::GUID,
    pub guidType: u32,
    pub pName: windows_sys::core::PSTR,
}
impl Default for DS_SCHEMA_GUID_MAPA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct DS_SCHEMA_GUID_MAPW {
    pub guid: windows_sys::core::GUID,
    pub guidType: u32,
    pub pName: windows_sys::core::PWSTR,
}
impl Default for DS_SCHEMA_GUID_MAPW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const DS_SCHEMA_GUID_NOT_FOUND: u32 = 0;
pub const DS_SERVICE_PRINCIPAL_NAME: DS_NAME_FORMAT = 10;
pub const DS_SID_OR_SID_HISTORY_NAME: DS_NAME_FORMAT = 11;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct DS_SITE_COST_INFO {
    pub errorCode: u32,
    pub cost: u32,
}
pub const DS_SPN_ADD_SPN_OP: DS_SPN_WRITE_OP = 0;
pub const DS_SPN_DELETE_SPN_OP: DS_SPN_WRITE_OP = 2;
pub const DS_SPN_DNS_HOST: DS_SPN_NAME_TYPE = 0;
pub const DS_SPN_DN_HOST: DS_SPN_NAME_TYPE = 1;
pub const DS_SPN_DOMAIN: DS_SPN_NAME_TYPE = 3;
pub type DS_SPN_NAME_TYPE = i32;
pub const DS_SPN_NB_DOMAIN: DS_SPN_NAME_TYPE = 4;
pub const DS_SPN_NB_HOST: DS_SPN_NAME_TYPE = 2;
pub const DS_SPN_REPLACE_SPN_OP: DS_SPN_WRITE_OP = 1;
pub const DS_SPN_SERVICE: DS_SPN_NAME_TYPE = 5;
pub type DS_SPN_WRITE_OP = i32;
pub const DS_SYNCED_EVENT_NAME: windows_sys::core::PCSTR = windows_sys::core::s!("NTDSInitialSyncsCompleted");
pub const DS_SYNCED_EVENT_NAME_W: windows_sys::core::PCWSTR = windows_sys::core::w!("NTDSInitialSyncsCompleted");
pub const DS_UNIQUE_ID_NAME: DS_NAME_FORMAT = 6;
pub const DS_UNKNOWN_NAME: DS_NAME_FORMAT = 0;
pub const DS_USER_PRINCIPAL_NAME: DS_NAME_FORMAT = 8;
pub const FLAG_DISABLABLE_OPTIONAL_FEATURE: u32 = 4;
pub const FLAG_DOMAIN_OPTIONAL_FEATURE: u32 = 2;
pub const FLAG_FOREST_OPTIONAL_FEATURE: u32 = 1;
pub const FLAG_SERVER_OPTIONAL_FEATURE: u32 = 8;
pub const FRSCONN_MAX_PRIORITY: u32 = 8;
pub const FRSCONN_PRIORITY_MASK: u32 = 1879048192;
pub const GUID_COMPUTRS_CONTAINER_A: windows_sys::core::PCSTR = windows_sys::core::s!("aa312825768811d1aded00c04fd8d5cd");
pub const GUID_COMPUTRS_CONTAINER_W: windows_sys::core::PCWSTR = windows_sys::core::w!("aa312825768811d1aded00c04fd8d5cd");
pub const GUID_DATABASE_32K_PAGES_OPTIONAL_FEATURE_A: windows_sys::core::PCSTR = windows_sys::core::s!("c62a9852731e4f75ae2473ae2775aab8");
pub const GUID_DATABASE_32K_PAGES_OPTIONAL_FEATURE_W: windows_sys::core::PCWSTR = windows_sys::core::w!("c62a9852731e4f75ae2473ae2775aab8");
pub const GUID_DELETED_OBJECTS_CONTAINER_A: windows_sys::core::PCSTR = windows_sys::core::s!("18e2ea80684f11d2b9aa00c04f79f805");
pub const GUID_DELETED_OBJECTS_CONTAINER_W: windows_sys::core::PCWSTR = windows_sys::core::w!("18e2ea80684f11d2b9aa00c04f79f805");
pub const GUID_DOMAIN_CONTROLLERS_CONTAINER_A: windows_sys::core::PCSTR = windows_sys::core::s!("a361b2ffffd211d1aa4b00c04fd7d83a");
pub const GUID_DOMAIN_CONTROLLERS_CONTAINER_W: windows_sys::core::PCWSTR = windows_sys::core::w!("a361b2ffffd211d1aa4b00c04fd7d83a");
pub const GUID_FOREIGNSECURITYPRINCIPALS_CONTAINER_A: windows_sys::core::PCSTR = windows_sys::core::s!("22b70c67d56e4efb91e9300fca3dc1aa");
pub const GUID_FOREIGNSECURITYPRINCIPALS_CONTAINER_W: windows_sys::core::PCWSTR = windows_sys::core::w!("22b70c67d56e4efb91e9300fca3dc1aa");
pub const GUID_INFRASTRUCTURE_CONTAINER_A: windows_sys::core::PCSTR = windows_sys::core::s!("2fbac1870ade11d297c400c04fd8d5cd");
pub const GUID_INFRASTRUCTURE_CONTAINER_W: windows_sys::core::PCWSTR = windows_sys::core::w!("2fbac1870ade11d297c400c04fd8d5cd");
pub const GUID_KEYS_CONTAINER_W: windows_sys::core::PCWSTR = windows_sys::core::w!("683A24E2E8164BD3AF86AC3C2CF3F981");
pub const GUID_LOSTANDFOUND_CONTAINER_A: windows_sys::core::PCSTR = windows_sys::core::s!("ab8153b7768811d1aded00c04fd8d5cd");
pub const GUID_LOSTANDFOUND_CONTAINER_W: windows_sys::core::PCWSTR = windows_sys::core::w!("ab8153b7768811d1aded00c04fd8d5cd");
pub const GUID_MANAGED_SERVICE_ACCOUNTS_CONTAINER_W: windows_sys::core::PCWSTR = windows_sys::core::w!("1EB93889E40C45DF9F0C64D23BBB6237");
pub const GUID_MICROSOFT_PROGRAM_DATA_CONTAINER_A: windows_sys::core::PCSTR = windows_sys::core::s!("f4be92a4c777485e878e9421d53087db");
pub const GUID_MICROSOFT_PROGRAM_DATA_CONTAINER_W: windows_sys::core::PCWSTR = windows_sys::core::w!("f4be92a4c777485e878e9421d53087db");
pub const GUID_NTDS_QUOTAS_CONTAINER_A: windows_sys::core::PCSTR = windows_sys::core::s!("6227f0af1fc2410d8e3bb10615bb5b0f");
pub const GUID_NTDS_QUOTAS_CONTAINER_W: windows_sys::core::PCWSTR = windows_sys::core::w!("6227f0af1fc2410d8e3bb10615bb5b0f");
pub const GUID_PRIVILEGED_ACCESS_MANAGEMENT_OPTIONAL_FEATURE_A: windows_sys::core::PCSTR = windows_sys::core::s!("73e843ece8cc4046b4ab07ffe4ab5bcd");
pub const GUID_PRIVILEGED_ACCESS_MANAGEMENT_OPTIONAL_FEATURE_W: windows_sys::core::PCWSTR = windows_sys::core::w!("73e843ece8cc4046b4ab07ffe4ab5bcd");
pub const GUID_PROGRAM_DATA_CONTAINER_A: windows_sys::core::PCSTR = windows_sys::core::s!("09460c08ae1e4a4ea0f64aee7daa1e5a");
pub const GUID_PROGRAM_DATA_CONTAINER_W: windows_sys::core::PCWSTR = windows_sys::core::w!("09460c08ae1e4a4ea0f64aee7daa1e5a");
pub const GUID_RECYCLE_BIN_OPTIONAL_FEATURE_A: windows_sys::core::PCSTR = windows_sys::core::s!("d8dc6d76d0ac5e44f3b9a7f9b6744f2a");
pub const GUID_RECYCLE_BIN_OPTIONAL_FEATURE_W: windows_sys::core::PCWSTR = windows_sys::core::w!("d8dc6d76d0ac5e44f3b9a7f9b6744f2a");
pub const GUID_SYSTEMS_CONTAINER_A: windows_sys::core::PCSTR = windows_sys::core::s!("ab1d30f3768811d1aded00c04fd8d5cd");
pub const GUID_SYSTEMS_CONTAINER_W: windows_sys::core::PCWSTR = windows_sys::core::w!("ab1d30f3768811d1aded00c04fd8d5cd");
pub const GUID_USERS_CONTAINER_A: windows_sys::core::PCSTR = windows_sys::core::s!("a9d1ca15768811d1aded00c04fd8d5cd");
pub const GUID_USERS_CONTAINER_W: windows_sys::core::PCWSTR = windows_sys::core::w!("a9d1ca15768811d1aded00c04fd8d5cd");
pub const NTDSAPI_BIND_ALLOW_DELEGATION: u32 = 1;
pub const NTDSAPI_BIND_FIND_BINDING: u32 = 2;
pub const NTDSAPI_BIND_FORCE_KERBEROS: u32 = 4;
pub const NTDSCONN_KCC_GC_TOPOLOGY: u32 = 1;
pub const NTDSCONN_KCC_INTERSITE_GC_TOPOLOGY: u32 = 32;
pub const NTDSCONN_KCC_INTERSITE_TOPOLOGY: u32 = 64;
pub const NTDSCONN_KCC_MINIMIZE_HOPS_TOPOLOGY: u32 = 4;
pub const NTDSCONN_KCC_NO_REASON: u32 = 0;
pub const NTDSCONN_KCC_OSCILLATING_CONNECTION_TOPOLOGY: u32 = 16;
pub const NTDSCONN_KCC_REDUNDANT_SERVER_TOPOLOGY: u32 = 512;
pub const NTDSCONN_KCC_RING_TOPOLOGY: u32 = 2;
pub const NTDSCONN_KCC_SERVER_FAILOVER_TOPOLOGY: u32 = 128;
pub const NTDSCONN_KCC_SITE_FAILOVER_TOPOLOGY: u32 = 256;
pub const NTDSCONN_KCC_STALE_SERVERS_TOPOLOGY: u32 = 8;
pub const NTDSCONN_OPT_DISABLE_INTERSITE_COMPRESSION: u32 = 16;
pub const NTDSCONN_OPT_IGNORE_SCHEDULE_MASK: u32 = 2147483648;
pub const NTDSCONN_OPT_IS_GENERATED: u32 = 1;
pub const NTDSCONN_OPT_OVERRIDE_NOTIFY_DEFAULT: u32 = 4;
pub const NTDSCONN_OPT_RODC_TOPOLOGY: u32 = 64;
pub const NTDSCONN_OPT_TWOWAY_SYNC: u32 = 2;
pub const NTDSCONN_OPT_USER_OWNED_SCHEDULE: u32 = 32;
pub const NTDSCONN_OPT_USE_NOTIFY: u32 = 8;
pub const NTDSDSA_OPT_BLOCK_RPC: u32 = 64;
pub const NTDSDSA_OPT_DISABLE_INBOUND_REPL: u32 = 2;
pub const NTDSDSA_OPT_DISABLE_NTDSCONN_XLATE: u32 = 8;
pub const NTDSDSA_OPT_DISABLE_OUTBOUND_REPL: u32 = 4;
pub const NTDSDSA_OPT_DISABLE_SPN_REGISTRATION: u32 = 16;
pub const NTDSDSA_OPT_GENERATE_OWN_TOPO: u32 = 32;
pub const NTDSDSA_OPT_IS_GC: u32 = 1;
pub const NTDSSETTINGS_DEFAULT_SERVER_REDUNDANCY: u32 = 2;
pub const NTDSSETTINGS_OPT_FORCE_KCC_W2K_ELECTION: u32 = 128;
pub const NTDSSETTINGS_OPT_FORCE_KCC_WHISTLER_BEHAVIOR: u32 = 64;
pub const NTDSSETTINGS_OPT_IS_AUTO_TOPOLOGY_DISABLED: u32 = 1;
pub const NTDSSETTINGS_OPT_IS_GROUP_CACHING_ENABLED: u32 = 32;
pub const NTDSSETTINGS_OPT_IS_INTER_SITE_AUTO_TOPOLOGY_DISABLED: u32 = 16;
pub const NTDSSETTINGS_OPT_IS_RAND_BH_SELECTION_DISABLED: u32 = 256;
pub const NTDSSETTINGS_OPT_IS_REDUNDANT_SERVER_TOPOLOGY_ENABLED: u32 = 1024;
pub const NTDSSETTINGS_OPT_IS_SCHEDULE_HASHING_ENABLED: u32 = 512;
pub const NTDSSETTINGS_OPT_IS_TOPL_CLEANUP_DISABLED: u32 = 2;
pub const NTDSSETTINGS_OPT_IS_TOPL_DETECT_STALE_DISABLED: u32 = 8;
pub const NTDSSETTINGS_OPT_IS_TOPL_MIN_HOPS_DISABLED: u32 = 4;
pub const NTDSSETTINGS_OPT_W2K3_BRIDGES_REQUIRED: u32 = 4096;
pub const NTDSSETTINGS_OPT_W2K3_IGNORE_SCHEDULES: u32 = 2048;
pub const NTDSSITECONN_OPT_DISABLE_COMPRESSION: u32 = 4;
pub const NTDSSITECONN_OPT_TWOWAY_SYNC: u32 = 2;
pub const NTDSSITECONN_OPT_USE_NOTIFY: u32 = 1;
pub const NTDSSITELINK_OPT_DISABLE_COMPRESSION: u32 = 4;
pub const NTDSSITELINK_OPT_TWOWAY_SYNC: u32 = 2;
pub const NTDSSITELINK_OPT_USE_NOTIFY: u32 = 1;
pub const NTDSTRANSPORT_OPT_BRIDGES_REQUIRED: u32 = 2;
pub const NTDSTRANSPORT_OPT_IGNORE_SCHEDULES: u32 = 1;
pub type PDS_DOMAIN_CONTROLLER_INFO_1A = *mut DS_DOMAIN_CONTROLLER_INFO_1A;
pub type PDS_DOMAIN_CONTROLLER_INFO_1W = *mut DS_DOMAIN_CONTROLLER_INFO_1W;
pub type PDS_DOMAIN_CONTROLLER_INFO_2A = *mut DS_DOMAIN_CONTROLLER_INFO_2A;
pub type PDS_DOMAIN_CONTROLLER_INFO_2W = *mut DS_DOMAIN_CONTROLLER_INFO_2W;
pub type PDS_DOMAIN_CONTROLLER_INFO_3A = *mut DS_DOMAIN_CONTROLLER_INFO_3A;
pub type PDS_DOMAIN_CONTROLLER_INFO_3W = *mut DS_DOMAIN_CONTROLLER_INFO_3W;
pub type PDS_NAME_RESULTA = *mut DS_NAME_RESULTA;
pub type PDS_NAME_RESULTW = *mut DS_NAME_RESULTW;
pub type PDS_NAME_RESULT_ITEMA = *mut DS_NAME_RESULT_ITEMA;
pub type PDS_NAME_RESULT_ITEMW = *mut DS_NAME_RESULT_ITEMW;
pub type PDS_REPSYNCALL_ERRINFOA = *mut DS_REPSYNCALL_ERRINFOA;
pub type PDS_REPSYNCALL_ERRINFOW = *mut DS_REPSYNCALL_ERRINFOW;
pub type PDS_REPSYNCALL_SYNCA = *mut DS_REPSYNCALL_SYNCA;
pub type PDS_REPSYNCALL_SYNCW = *mut DS_REPSYNCALL_SYNCW;
pub type PDS_REPSYNCALL_UPDATEA = *mut DS_REPSYNCALL_UPDATEA;
pub type PDS_REPSYNCALL_UPDATEW = *mut DS_REPSYNCALL_UPDATEW;
pub type PDS_SCHEMA_GUID_MAPA = *mut DS_SCHEMA_GUID_MAPA;
pub type PDS_SCHEMA_GUID_MAPW = *mut DS_SCHEMA_GUID_MAPW;
pub type PDS_SITE_COST_INFO = *mut DS_SITE_COST_INFO;
