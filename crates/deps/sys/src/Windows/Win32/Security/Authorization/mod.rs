#![allow(non_snake_case, non_camel_case_types)]
#[cfg(feature = "Win32_Security_Authorization_UI")]
pub mod UI;
#[link(name = "windows")]
extern "system" {
    #[doc = "*Required features: `Win32_Security_Authorization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn AuthzAccessCheck(flags: AUTHZ_ACCESS_CHECK_FLAGS, hauthzclientcontext: AUTHZ_CLIENT_CONTEXT_HANDLE, prequest: *const AUTHZ_ACCESS_REQUEST, hauditevent: AUTHZ_AUDIT_EVENT_HANDLE, psecuritydescriptor: *const super::SECURITY_DESCRIPTOR, optionalsecuritydescriptorarray: *const *const super::SECURITY_DESCRIPTOR, optionalsecuritydescriptorcount: u32, preply: *mut AUTHZ_ACCESS_REPLY, phaccesscheckresults: *mut isize) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Security_Authorization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn AuthzAddSidsToContext(hauthzclientcontext: AUTHZ_CLIENT_CONTEXT_HANDLE, sids: *const super::SID_AND_ATTRIBUTES, sidcount: u32, restrictedsids: *const super::SID_AND_ATTRIBUTES, restrictedsidcount: u32, phnewauthzclientcontext: *mut isize) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Security_Authorization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn AuthzCachedAccessCheck(flags: u32, haccesscheckresults: AUTHZ_ACCESS_CHECK_RESULTS_HANDLE, prequest: *const AUTHZ_ACCESS_REQUEST, hauditevent: AUTHZ_AUDIT_EVENT_HANDLE, preply: *mut AUTHZ_ACCESS_REPLY) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Security_Authorization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn AuthzEnumerateSecurityEventSources(dwflags: u32, buffer: *mut AUTHZ_SOURCE_SCHEMA_REGISTRATION, pdwcount: *mut u32, pdwlength: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Security_Authorization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn AuthzEvaluateSacl(authzclientcontext: AUTHZ_CLIENT_CONTEXT_HANDLE, prequest: *const AUTHZ_ACCESS_REQUEST, sacl: *const super::ACL, grantedaccess: u32, accessgranted: super::super::Foundation::BOOL, pbgenerateaudit: *mut super::super::Foundation::BOOL) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Security_Authorization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn AuthzFreeAuditEvent(hauditevent: AUTHZ_AUDIT_EVENT_HANDLE) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Security_Authorization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn AuthzFreeCentralAccessPolicyCache() -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Security_Authorization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn AuthzFreeContext(hauthzclientcontext: AUTHZ_CLIENT_CONTEXT_HANDLE) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Security_Authorization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn AuthzFreeHandle(haccesscheckresults: AUTHZ_ACCESS_CHECK_RESULTS_HANDLE) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Security_Authorization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn AuthzFreeResourceManager(hauthzresourcemanager: AUTHZ_RESOURCE_MANAGER_HANDLE) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Security_Authorization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn AuthzGetInformationFromContext(hauthzclientcontext: AUTHZ_CLIENT_CONTEXT_HANDLE, infoclass: AUTHZ_CONTEXT_INFORMATION_CLASS, buffersize: u32, psizerequired: *mut u32, buffer: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Security_Authorization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn AuthzInitializeCompoundContext(usercontext: AUTHZ_CLIENT_CONTEXT_HANDLE, devicecontext: AUTHZ_CLIENT_CONTEXT_HANDLE, phcompoundcontext: *mut isize) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Security_Authorization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn AuthzInitializeContextFromAuthzContext(flags: u32, hauthzclientcontext: AUTHZ_CLIENT_CONTEXT_HANDLE, pexpirationtime: *const i64, identifier: super::super::Foundation::LUID, dynamicgroupargs: *const ::core::ffi::c_void, phnewauthzclientcontext: *mut isize) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Security_Authorization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn AuthzInitializeContextFromSid(flags: u32, usersid: super::super::Foundation::PSID, hauthzresourcemanager: AUTHZ_RESOURCE_MANAGER_HANDLE, pexpirationtime: *const i64, identifier: super::super::Foundation::LUID, dynamicgroupargs: *const ::core::ffi::c_void, phauthzclientcontext: *mut isize) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Security_Authorization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn AuthzInitializeContextFromToken(flags: u32, tokenhandle: super::super::Foundation::HANDLE, hauthzresourcemanager: AUTHZ_RESOURCE_MANAGER_HANDLE, pexpirationtime: *const i64, identifier: super::super::Foundation::LUID, dynamicgroupargs: *const ::core::ffi::c_void, phauthzclientcontext: *mut isize) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Security_Authorization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn AuthzInitializeObjectAccessAuditEvent(flags: AUTHZ_INITIALIZE_OBJECT_ACCESS_AUDIT_EVENT_FLAGS, hauditeventtype: AUTHZ_AUDIT_EVENT_TYPE_HANDLE, szoperationtype: super::super::Foundation::PWSTR, szobjecttype: super::super::Foundation::PWSTR, szobjectname: super::super::Foundation::PWSTR, szadditionalinfo: super::super::Foundation::PWSTR, phauditevent: *mut isize, dwadditionalparametercount: u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Security_Authorization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn AuthzInitializeObjectAccessAuditEvent2(flags: u32, hauditeventtype: AUTHZ_AUDIT_EVENT_TYPE_HANDLE, szoperationtype: super::super::Foundation::PWSTR, szobjecttype: super::super::Foundation::PWSTR, szobjectname: super::super::Foundation::PWSTR, szadditionalinfo: super::super::Foundation::PWSTR, szadditionalinfo2: super::super::Foundation::PWSTR, phauditevent: *mut isize, dwadditionalparametercount: u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Security_Authorization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn AuthzInitializeRemoteResourceManager(prpcinitinfo: *const AUTHZ_RPC_INIT_INFO_CLIENT, phauthzresourcemanager: *mut AUTHZ_RESOURCE_MANAGER_HANDLE) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Security_Authorization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn AuthzInitializeResourceManager(flags: u32, pfndynamicaccesscheck: PFN_AUTHZ_DYNAMIC_ACCESS_CHECK, pfncomputedynamicgroups: PFN_AUTHZ_COMPUTE_DYNAMIC_GROUPS, pfnfreedynamicgroups: PFN_AUTHZ_FREE_DYNAMIC_GROUPS, szresourcemanagername: super::super::Foundation::PWSTR, phauthzresourcemanager: *mut AUTHZ_RESOURCE_MANAGER_HANDLE) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Security_Authorization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn AuthzInitializeResourceManagerEx(flags: AUTHZ_RESOURCE_MANAGER_FLAGS, pauthzinitinfo: *const AUTHZ_INIT_INFO, phauthzresourcemanager: *mut AUTHZ_RESOURCE_MANAGER_HANDLE) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Security_Authorization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn AuthzInstallSecurityEventSource(dwflags: u32, pregistration: *const AUTHZ_SOURCE_SCHEMA_REGISTRATION) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Security_Authorization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn AuthzModifyClaims(hauthzclientcontext: AUTHZ_CLIENT_CONTEXT_HANDLE, claimclass: AUTHZ_CONTEXT_INFORMATION_CLASS, pclaimoperations: *const AUTHZ_SECURITY_ATTRIBUTE_OPERATION, pclaims: *const AUTHZ_SECURITY_ATTRIBUTES_INFORMATION) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Security_Authorization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn AuthzModifySecurityAttributes(hauthzclientcontext: AUTHZ_CLIENT_CONTEXT_HANDLE, poperations: *const AUTHZ_SECURITY_ATTRIBUTE_OPERATION, pattributes: *const AUTHZ_SECURITY_ATTRIBUTES_INFORMATION) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Security_Authorization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn AuthzModifySids(hauthzclientcontext: AUTHZ_CLIENT_CONTEXT_HANDLE, sidclass: AUTHZ_CONTEXT_INFORMATION_CLASS, psidoperations: *const AUTHZ_SID_OPERATION, psids: *const super::TOKEN_GROUPS) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Security_Authorization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn AuthzOpenObjectAudit(flags: u32, hauthzclientcontext: AUTHZ_CLIENT_CONTEXT_HANDLE, prequest: *const AUTHZ_ACCESS_REQUEST, hauditevent: AUTHZ_AUDIT_EVENT_HANDLE, psecuritydescriptor: *const super::SECURITY_DESCRIPTOR, optionalsecuritydescriptorarray: *const *const super::SECURITY_DESCRIPTOR, optionalsecuritydescriptorcount: u32, preply: *const AUTHZ_ACCESS_REPLY) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Security_Authorization`, `Win32_Foundation`, `Win32_System_Threading`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Threading"))]
    pub fn AuthzRegisterCapChangeNotification(phcapchangesubscription: *mut *mut AUTHZ_CAP_CHANGE_SUBSCRIPTION_HANDLE__, pfncapchangecallback: super::super::System::Threading::LPTHREAD_START_ROUTINE, pcallbackcontext: *const ::core::ffi::c_void) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Security_Authorization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn AuthzRegisterSecurityEventSource(dwflags: u32, szeventsourcename: super::super::Foundation::PWSTR, pheventprovider: *mut isize) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Security_Authorization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn AuthzReportSecurityEvent(dwflags: u32, heventprovider: AUTHZ_SECURITY_EVENT_PROVIDER_HANDLE, dwauditid: u32, pusersid: super::super::Foundation::PSID, dwcount: u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Security_Authorization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn AuthzReportSecurityEventFromParams(dwflags: u32, heventprovider: AUTHZ_SECURITY_EVENT_PROVIDER_HANDLE, dwauditid: u32, pusersid: super::super::Foundation::PSID, pparams: *const AUDIT_PARAMS) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Security_Authorization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn AuthzSetAppContainerInformation(hauthzclientcontext: AUTHZ_CLIENT_CONTEXT_HANDLE, pappcontainersid: super::super::Foundation::PSID, capabilitycount: u32, pcapabilitysids: *const super::SID_AND_ATTRIBUTES) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Security_Authorization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn AuthzUninstallSecurityEventSource(dwflags: u32, szeventsourcename: super::super::Foundation::PWSTR) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Security_Authorization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn AuthzUnregisterCapChangeNotification(hcapchangesubscription: *const AUTHZ_CAP_CHANGE_SUBSCRIPTION_HANDLE__) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Security_Authorization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn AuthzUnregisterSecurityEventSource(dwflags: u32, pheventprovider: *mut isize) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Security_Authorization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn BuildExplicitAccessWithNameA(pexplicitaccess: *mut EXPLICIT_ACCESS_A, ptrusteename: super::super::Foundation::PSTR, accesspermissions: u32, accessmode: ACCESS_MODE, inheritance: super::ACE_FLAGS);
    #[doc = "*Required features: `Win32_Security_Authorization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn BuildExplicitAccessWithNameW(pexplicitaccess: *mut EXPLICIT_ACCESS_W, ptrusteename: super::super::Foundation::PWSTR, accesspermissions: u32, accessmode: ACCESS_MODE, inheritance: super::ACE_FLAGS);
    #[doc = "*Required features: `Win32_Security_Authorization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn BuildImpersonateExplicitAccessWithNameA(pexplicitaccess: *mut EXPLICIT_ACCESS_A, ptrusteename: super::super::Foundation::PSTR, ptrustee: *const TRUSTEE_A, accesspermissions: u32, accessmode: ACCESS_MODE, inheritance: u32);
    #[doc = "*Required features: `Win32_Security_Authorization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn BuildImpersonateExplicitAccessWithNameW(pexplicitaccess: *mut EXPLICIT_ACCESS_W, ptrusteename: super::super::Foundation::PWSTR, ptrustee: *const TRUSTEE_W, accesspermissions: u32, accessmode: ACCESS_MODE, inheritance: u32);
    #[doc = "*Required features: `Win32_Security_Authorization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn BuildImpersonateTrusteeA(ptrustee: *mut TRUSTEE_A, pimpersonatetrustee: *const TRUSTEE_A);
    #[doc = "*Required features: `Win32_Security_Authorization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn BuildImpersonateTrusteeW(ptrustee: *mut TRUSTEE_W, pimpersonatetrustee: *const TRUSTEE_W);
    #[doc = "*Required features: `Win32_Security_Authorization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn BuildSecurityDescriptorA(powner: *const TRUSTEE_A, pgroup: *const TRUSTEE_A, ccountofaccessentries: u32, plistofaccessentries: *const EXPLICIT_ACCESS_A, ccountofauditentries: u32, plistofauditentries: *const EXPLICIT_ACCESS_A, poldsd: *const super::SECURITY_DESCRIPTOR, psizenewsd: *mut u32, pnewsd: *mut *mut super::SECURITY_DESCRIPTOR) -> u32;
    #[doc = "*Required features: `Win32_Security_Authorization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn BuildSecurityDescriptorW(powner: *const TRUSTEE_W, pgroup: *const TRUSTEE_W, ccountofaccessentries: u32, plistofaccessentries: *const EXPLICIT_ACCESS_W, ccountofauditentries: u32, plistofauditentries: *const EXPLICIT_ACCESS_W, poldsd: *const super::SECURITY_DESCRIPTOR, psizenewsd: *mut u32, pnewsd: *mut *mut super::SECURITY_DESCRIPTOR) -> u32;
    #[doc = "*Required features: `Win32_Security_Authorization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn BuildTrusteeWithNameA(ptrustee: *mut TRUSTEE_A, pname: super::super::Foundation::PSTR);
    #[doc = "*Required features: `Win32_Security_Authorization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn BuildTrusteeWithNameW(ptrustee: *mut TRUSTEE_W, pname: super::super::Foundation::PWSTR);
    #[doc = "*Required features: `Win32_Security_Authorization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn BuildTrusteeWithObjectsAndNameA(ptrustee: *mut TRUSTEE_A, pobjname: *const OBJECTS_AND_NAME_A, objecttype: SE_OBJECT_TYPE, objecttypename: super::super::Foundation::PSTR, inheritedobjecttypename: super::super::Foundation::PSTR, name: super::super::Foundation::PSTR);
    #[doc = "*Required features: `Win32_Security_Authorization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn BuildTrusteeWithObjectsAndNameW(ptrustee: *mut TRUSTEE_W, pobjname: *const OBJECTS_AND_NAME_W, objecttype: SE_OBJECT_TYPE, objecttypename: super::super::Foundation::PWSTR, inheritedobjecttypename: super::super::Foundation::PWSTR, name: super::super::Foundation::PWSTR);
    #[doc = "*Required features: `Win32_Security_Authorization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn BuildTrusteeWithObjectsAndSidA(ptrustee: *mut TRUSTEE_A, pobjsid: *const OBJECTS_AND_SID, pobjectguid: *const ::windows_sys::core::GUID, pinheritedobjectguid: *const ::windows_sys::core::GUID, psid: super::super::Foundation::PSID);
    #[doc = "*Required features: `Win32_Security_Authorization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn BuildTrusteeWithObjectsAndSidW(ptrustee: *mut TRUSTEE_W, pobjsid: *const OBJECTS_AND_SID, pobjectguid: *const ::windows_sys::core::GUID, pinheritedobjectguid: *const ::windows_sys::core::GUID, psid: super::super::Foundation::PSID);
    #[doc = "*Required features: `Win32_Security_Authorization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn BuildTrusteeWithSidA(ptrustee: *mut TRUSTEE_A, psid: super::super::Foundation::PSID);
    #[doc = "*Required features: `Win32_Security_Authorization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn BuildTrusteeWithSidW(ptrustee: *mut TRUSTEE_W, psid: super::super::Foundation::PSID);
    #[doc = "*Required features: `Win32_Security_Authorization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ConvertSecurityDescriptorToStringSecurityDescriptorA(securitydescriptor: *const super::SECURITY_DESCRIPTOR, requestedstringsdrevision: u32, securityinformation: u32, stringsecuritydescriptor: *mut super::super::Foundation::PSTR, stringsecuritydescriptorlen: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Security_Authorization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ConvertSecurityDescriptorToStringSecurityDescriptorW(securitydescriptor: *const super::SECURITY_DESCRIPTOR, requestedstringsdrevision: u32, securityinformation: u32, stringsecuritydescriptor: *mut super::super::Foundation::PWSTR, stringsecuritydescriptorlen: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Security_Authorization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ConvertSidToStringSidA(sid: super::super::Foundation::PSID, stringsid: *mut super::super::Foundation::PSTR) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Security_Authorization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ConvertSidToStringSidW(sid: super::super::Foundation::PSID, stringsid: *mut super::super::Foundation::PWSTR) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Security_Authorization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ConvertStringSecurityDescriptorToSecurityDescriptorA(stringsecuritydescriptor: super::super::Foundation::PSTR, stringsdrevision: u32, securitydescriptor: *mut *mut super::SECURITY_DESCRIPTOR, securitydescriptorsize: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Security_Authorization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ConvertStringSecurityDescriptorToSecurityDescriptorW(stringsecuritydescriptor: super::super::Foundation::PWSTR, stringsdrevision: u32, securitydescriptor: *mut *mut super::SECURITY_DESCRIPTOR, securitydescriptorsize: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Security_Authorization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ConvertStringSidToSidA(stringsid: super::super::Foundation::PSTR, sid: *mut super::super::Foundation::PSID) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Security_Authorization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ConvertStringSidToSidW(stringsid: super::super::Foundation::PWSTR, sid: *mut super::super::Foundation::PSID) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Security_Authorization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FreeInheritedFromArray(pinheritarray: *const INHERITED_FROMW, acecnt: u16, pfnarray: *const FN_OBJECT_MGR_FUNCTIONS) -> u32;
    #[doc = "*Required features: `Win32_Security_Authorization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetAuditedPermissionsFromAclA(pacl: *const super::ACL, ptrustee: *const TRUSTEE_A, psuccessfulauditedrights: *mut u32, pfailedauditrights: *mut u32) -> u32;
    #[doc = "*Required features: `Win32_Security_Authorization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetAuditedPermissionsFromAclW(pacl: *const super::ACL, ptrustee: *const TRUSTEE_W, psuccessfulauditedrights: *mut u32, pfailedauditrights: *mut u32) -> u32;
    #[doc = "*Required features: `Win32_Security_Authorization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetEffectiveRightsFromAclA(pacl: *const super::ACL, ptrustee: *const TRUSTEE_A, paccessrights: *mut u32) -> u32;
    #[doc = "*Required features: `Win32_Security_Authorization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetEffectiveRightsFromAclW(pacl: *const super::ACL, ptrustee: *const TRUSTEE_W, paccessrights: *mut u32) -> u32;
    #[doc = "*Required features: `Win32_Security_Authorization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetExplicitEntriesFromAclA(pacl: *const super::ACL, pccountofexplicitentries: *mut u32, plistofexplicitentries: *mut *mut EXPLICIT_ACCESS_A) -> u32;
    #[doc = "*Required features: `Win32_Security_Authorization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetExplicitEntriesFromAclW(pacl: *const super::ACL, pccountofexplicitentries: *mut u32, plistofexplicitentries: *mut *mut EXPLICIT_ACCESS_W) -> u32;
    #[doc = "*Required features: `Win32_Security_Authorization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetInheritanceSourceA(pobjectname: super::super::Foundation::PSTR, objecttype: SE_OBJECT_TYPE, securityinfo: u32, container: super::super::Foundation::BOOL, pobjectclassguids: *const *const ::windows_sys::core::GUID, guidcount: u32, pacl: *const super::ACL, pfnarray: *const FN_OBJECT_MGR_FUNCTIONS, pgenericmapping: *const super::GENERIC_MAPPING, pinheritarray: *mut INHERITED_FROMA) -> u32;
    #[doc = "*Required features: `Win32_Security_Authorization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetInheritanceSourceW(pobjectname: super::super::Foundation::PWSTR, objecttype: SE_OBJECT_TYPE, securityinfo: u32, container: super::super::Foundation::BOOL, pobjectclassguids: *const *const ::windows_sys::core::GUID, guidcount: u32, pacl: *const super::ACL, pfnarray: *const FN_OBJECT_MGR_FUNCTIONS, pgenericmapping: *const super::GENERIC_MAPPING, pinheritarray: *mut INHERITED_FROMW) -> u32;
    #[doc = "*Required features: `Win32_Security_Authorization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetMultipleTrusteeA(ptrustee: *const TRUSTEE_A) -> *mut TRUSTEE_A;
    #[doc = "*Required features: `Win32_Security_Authorization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetMultipleTrusteeOperationA(ptrustee: *const TRUSTEE_A) -> MULTIPLE_TRUSTEE_OPERATION;
    #[doc = "*Required features: `Win32_Security_Authorization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetMultipleTrusteeOperationW(ptrustee: *const TRUSTEE_W) -> MULTIPLE_TRUSTEE_OPERATION;
    #[doc = "*Required features: `Win32_Security_Authorization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetMultipleTrusteeW(ptrustee: *const TRUSTEE_W) -> *mut TRUSTEE_W;
    #[doc = "*Required features: `Win32_Security_Authorization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetNamedSecurityInfoA(pobjectname: super::super::Foundation::PSTR, objecttype: SE_OBJECT_TYPE, securityinfo: super::OBJECT_SECURITY_INFORMATION, ppsidowner: *mut super::super::Foundation::PSID, ppsidgroup: *mut super::super::Foundation::PSID, ppdacl: *mut *mut super::ACL, ppsacl: *mut *mut super::ACL, ppsecuritydescriptor: *mut *mut super::SECURITY_DESCRIPTOR) -> u32;
    #[doc = "*Required features: `Win32_Security_Authorization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetNamedSecurityInfoW(pobjectname: super::super::Foundation::PWSTR, objecttype: SE_OBJECT_TYPE, securityinfo: super::OBJECT_SECURITY_INFORMATION, ppsidowner: *mut super::super::Foundation::PSID, ppsidgroup: *mut super::super::Foundation::PSID, ppdacl: *mut *mut super::ACL, ppsacl: *mut *mut super::ACL, ppsecuritydescriptor: *mut *mut super::SECURITY_DESCRIPTOR) -> u32;
    #[doc = "*Required features: `Win32_Security_Authorization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetSecurityInfo(handle: super::super::Foundation::HANDLE, objecttype: SE_OBJECT_TYPE, securityinfo: u32, ppsidowner: *mut super::super::Foundation::PSID, ppsidgroup: *mut super::super::Foundation::PSID, ppdacl: *mut *mut super::ACL, ppsacl: *mut *mut super::ACL, ppsecuritydescriptor: *mut *mut super::SECURITY_DESCRIPTOR) -> u32;
    #[doc = "*Required features: `Win32_Security_Authorization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetTrusteeFormA(ptrustee: *const TRUSTEE_A) -> TRUSTEE_FORM;
    #[doc = "*Required features: `Win32_Security_Authorization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetTrusteeFormW(ptrustee: *const TRUSTEE_W) -> TRUSTEE_FORM;
    #[doc = "*Required features: `Win32_Security_Authorization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetTrusteeNameA(ptrustee: *const TRUSTEE_A) -> super::super::Foundation::PSTR;
    #[doc = "*Required features: `Win32_Security_Authorization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetTrusteeNameW(ptrustee: *const TRUSTEE_W) -> super::super::Foundation::PWSTR;
    #[doc = "*Required features: `Win32_Security_Authorization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetTrusteeTypeA(ptrustee: *const TRUSTEE_A) -> TRUSTEE_TYPE;
    #[doc = "*Required features: `Win32_Security_Authorization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetTrusteeTypeW(ptrustee: *const TRUSTEE_W) -> TRUSTEE_TYPE;
    #[doc = "*Required features: `Win32_Security_Authorization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn LookupSecurityDescriptorPartsA(ppowner: *mut *mut TRUSTEE_A, ppgroup: *mut *mut TRUSTEE_A, pccountofaccessentries: *mut u32, pplistofaccessentries: *mut *mut EXPLICIT_ACCESS_A, pccountofauditentries: *mut u32, pplistofauditentries: *mut *mut EXPLICIT_ACCESS_A, psd: *const super::SECURITY_DESCRIPTOR) -> u32;
    #[doc = "*Required features: `Win32_Security_Authorization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn LookupSecurityDescriptorPartsW(ppowner: *mut *mut TRUSTEE_W, ppgroup: *mut *mut TRUSTEE_W, pccountofaccessentries: *mut u32, pplistofaccessentries: *mut *mut EXPLICIT_ACCESS_W, pccountofauditentries: *mut u32, pplistofauditentries: *mut *mut EXPLICIT_ACCESS_W, psd: *const super::SECURITY_DESCRIPTOR) -> u32;
    #[doc = "*Required features: `Win32_Security_Authorization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetEntriesInAclA(ccountofexplicitentries: u32, plistofexplicitentries: *const EXPLICIT_ACCESS_A, oldacl: *const super::ACL, newacl: *mut *mut super::ACL) -> u32;
    #[doc = "*Required features: `Win32_Security_Authorization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetEntriesInAclW(ccountofexplicitentries: u32, plistofexplicitentries: *const EXPLICIT_ACCESS_W, oldacl: *const super::ACL, newacl: *mut *mut super::ACL) -> u32;
    #[doc = "*Required features: `Win32_Security_Authorization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetNamedSecurityInfoA(pobjectname: super::super::Foundation::PSTR, objecttype: SE_OBJECT_TYPE, securityinfo: super::OBJECT_SECURITY_INFORMATION, psidowner: super::super::Foundation::PSID, psidgroup: super::super::Foundation::PSID, pdacl: *const super::ACL, psacl: *const super::ACL) -> u32;
    #[doc = "*Required features: `Win32_Security_Authorization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetNamedSecurityInfoW(pobjectname: super::super::Foundation::PWSTR, objecttype: SE_OBJECT_TYPE, securityinfo: super::OBJECT_SECURITY_INFORMATION, psidowner: super::super::Foundation::PSID, psidgroup: super::super::Foundation::PSID, pdacl: *const super::ACL, psacl: *const super::ACL) -> u32;
    #[doc = "*Required features: `Win32_Security_Authorization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetSecurityInfo(handle: super::super::Foundation::HANDLE, objecttype: SE_OBJECT_TYPE, securityinfo: u32, psidowner: super::super::Foundation::PSID, psidgroup: super::super::Foundation::PSID, pdacl: *const super::ACL, psacl: *const super::ACL) -> u32;
    #[doc = "*Required features: `Win32_Security_Authorization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn TreeResetNamedSecurityInfoA(pobjectname: super::super::Foundation::PSTR, objecttype: SE_OBJECT_TYPE, securityinfo: u32, powner: super::super::Foundation::PSID, pgroup: super::super::Foundation::PSID, pdacl: *const super::ACL, psacl: *const super::ACL, keepexplicit: super::super::Foundation::BOOL, fnprogress: FN_PROGRESS, progressinvokesetting: PROG_INVOKE_SETTING, args: *const ::core::ffi::c_void) -> u32;
    #[doc = "*Required features: `Win32_Security_Authorization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn TreeResetNamedSecurityInfoW(pobjectname: super::super::Foundation::PWSTR, objecttype: SE_OBJECT_TYPE, securityinfo: u32, powner: super::super::Foundation::PSID, pgroup: super::super::Foundation::PSID, pdacl: *const super::ACL, psacl: *const super::ACL, keepexplicit: super::super::Foundation::BOOL, fnprogress: FN_PROGRESS, progressinvokesetting: PROG_INVOKE_SETTING, args: *const ::core::ffi::c_void) -> u32;
    #[doc = "*Required features: `Win32_Security_Authorization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn TreeSetNamedSecurityInfoA(pobjectname: super::super::Foundation::PSTR, objecttype: SE_OBJECT_TYPE, securityinfo: u32, powner: super::super::Foundation::PSID, pgroup: super::super::Foundation::PSID, pdacl: *const super::ACL, psacl: *const super::ACL, dwaction: TREE_SEC_INFO, fnprogress: FN_PROGRESS, progressinvokesetting: PROG_INVOKE_SETTING, args: *const ::core::ffi::c_void) -> u32;
    #[doc = "*Required features: `Win32_Security_Authorization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn TreeSetNamedSecurityInfoW(pobjectname: super::super::Foundation::PWSTR, objecttype: SE_OBJECT_TYPE, securityinfo: u32, powner: super::super::Foundation::PSID, pgroup: super::super::Foundation::PSID, pdacl: *const super::ACL, psacl: *const super::ACL, dwaction: TREE_SEC_INFO, fnprogress: FN_PROGRESS, progressinvokesetting: PROG_INVOKE_SETTING, args: *const ::core::ffi::c_void) -> u32;
}
pub struct ACCESS_MODE(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct ACTRL_ACCESSA(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct ACTRL_ACCESSW(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct ACTRL_ACCESS_ENTRYA(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct ACTRL_ACCESS_ENTRYW(i32);
pub struct ACTRL_ACCESS_ENTRY_ACCESS_FLAGS(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct ACTRL_ACCESS_ENTRY_LISTA(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct ACTRL_ACCESS_ENTRY_LISTW(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct ACTRL_ACCESS_INFOA(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct ACTRL_ACCESS_INFOW(i32);
#[doc = "*Required features: `Win32_Security_Authorization`*"]
pub const ACTRL_ACCESS_NO_OPTIONS: u32 = 0u32;
#[doc = "*Required features: `Win32_Security_Authorization`*"]
pub const ACTRL_ACCESS_PROTECTED: u32 = 1u32;
#[doc = "*Required features: `Win32_Security_Authorization`*"]
pub const ACTRL_ACCESS_SUPPORTS_OBJECT_ENTRIES: u32 = 1u32;
#[doc = "*Required features: `Win32_Security_Authorization`*"]
pub const ACTRL_CHANGE_ACCESS: u32 = 536870912u32;
#[doc = "*Required features: `Win32_Security_Authorization`*"]
pub const ACTRL_CHANGE_OWNER: u32 = 1073741824u32;
#[cfg(feature = "Win32_Foundation")]
pub struct ACTRL_CONTROL_INFOA(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct ACTRL_CONTROL_INFOW(i32);
#[doc = "*Required features: `Win32_Security_Authorization`*"]
pub const ACTRL_DELETE: u32 = 134217728u32;
#[doc = "*Required features: `Win32_Security_Authorization`*"]
pub const ACTRL_DIR_CREATE_CHILD: u32 = 4u32;
#[doc = "*Required features: `Win32_Security_Authorization`*"]
pub const ACTRL_DIR_CREATE_OBJECT: u32 = 2u32;
#[doc = "*Required features: `Win32_Security_Authorization`*"]
pub const ACTRL_DIR_DELETE_CHILD: u32 = 64u32;
#[doc = "*Required features: `Win32_Security_Authorization`*"]
pub const ACTRL_DIR_LIST: u32 = 1u32;
#[doc = "*Required features: `Win32_Security_Authorization`*"]
pub const ACTRL_DIR_TRAVERSE: u32 = 32u32;
#[doc = "*Required features: `Win32_Security_Authorization`*"]
pub const ACTRL_FILE_APPEND: u32 = 4u32;
#[doc = "*Required features: `Win32_Security_Authorization`*"]
pub const ACTRL_FILE_CREATE_PIPE: u32 = 512u32;
#[doc = "*Required features: `Win32_Security_Authorization`*"]
pub const ACTRL_FILE_EXECUTE: u32 = 32u32;
#[doc = "*Required features: `Win32_Security_Authorization`*"]
pub const ACTRL_FILE_READ: u32 = 1u32;
#[doc = "*Required features: `Win32_Security_Authorization`*"]
pub const ACTRL_FILE_READ_ATTRIB: u32 = 128u32;
#[doc = "*Required features: `Win32_Security_Authorization`*"]
pub const ACTRL_FILE_READ_PROP: u32 = 8u32;
#[doc = "*Required features: `Win32_Security_Authorization`*"]
pub const ACTRL_FILE_WRITE: u32 = 2u32;
#[doc = "*Required features: `Win32_Security_Authorization`*"]
pub const ACTRL_FILE_WRITE_ATTRIB: u32 = 256u32;
#[doc = "*Required features: `Win32_Security_Authorization`*"]
pub const ACTRL_FILE_WRITE_PROP: u32 = 16u32;
#[doc = "*Required features: `Win32_Security_Authorization`*"]
pub const ACTRL_KERNEL_ALERT: u32 = 1024u32;
#[doc = "*Required features: `Win32_Security_Authorization`*"]
pub const ACTRL_KERNEL_CONTROL: u32 = 512u32;
#[doc = "*Required features: `Win32_Security_Authorization`*"]
pub const ACTRL_KERNEL_DIMPERSONATE: u32 = 32768u32;
#[doc = "*Required features: `Win32_Security_Authorization`*"]
pub const ACTRL_KERNEL_DUP_HANDLE: u32 = 32u32;
#[doc = "*Required features: `Win32_Security_Authorization`*"]
pub const ACTRL_KERNEL_GET_CONTEXT: u32 = 2048u32;
#[doc = "*Required features: `Win32_Security_Authorization`*"]
pub const ACTRL_KERNEL_GET_INFO: u32 = 256u32;
#[doc = "*Required features: `Win32_Security_Authorization`*"]
pub const ACTRL_KERNEL_IMPERSONATE: u32 = 16384u32;
#[doc = "*Required features: `Win32_Security_Authorization`*"]
pub const ACTRL_KERNEL_PROCESS: u32 = 64u32;
#[doc = "*Required features: `Win32_Security_Authorization`*"]
pub const ACTRL_KERNEL_SET_CONTEXT: u32 = 4096u32;
#[doc = "*Required features: `Win32_Security_Authorization`*"]
pub const ACTRL_KERNEL_SET_INFO: u32 = 128u32;
#[doc = "*Required features: `Win32_Security_Authorization`*"]
pub const ACTRL_KERNEL_TERMINATE: u32 = 1u32;
#[doc = "*Required features: `Win32_Security_Authorization`*"]
pub const ACTRL_KERNEL_THREAD: u32 = 2u32;
#[doc = "*Required features: `Win32_Security_Authorization`*"]
pub const ACTRL_KERNEL_TOKEN: u32 = 8192u32;
#[doc = "*Required features: `Win32_Security_Authorization`*"]
pub const ACTRL_KERNEL_VM: u32 = 4u32;
#[doc = "*Required features: `Win32_Security_Authorization`*"]
pub const ACTRL_KERNEL_VM_READ: u32 = 8u32;
#[doc = "*Required features: `Win32_Security_Authorization`*"]
pub const ACTRL_KERNEL_VM_WRITE: u32 = 16u32;
#[cfg(feature = "Win32_Foundation")]
pub struct ACTRL_OVERLAPPED(i32);
#[doc = "*Required features: `Win32_Security_Authorization`*"]
pub const ACTRL_PERM_1: u32 = 1u32;
#[doc = "*Required features: `Win32_Security_Authorization`*"]
pub const ACTRL_PERM_10: u32 = 512u32;
#[doc = "*Required features: `Win32_Security_Authorization`*"]
pub const ACTRL_PERM_11: u32 = 1024u32;
#[doc = "*Required features: `Win32_Security_Authorization`*"]
pub const ACTRL_PERM_12: u32 = 2048u32;
#[doc = "*Required features: `Win32_Security_Authorization`*"]
pub const ACTRL_PERM_13: u32 = 4096u32;
#[doc = "*Required features: `Win32_Security_Authorization`*"]
pub const ACTRL_PERM_14: u32 = 8192u32;
#[doc = "*Required features: `Win32_Security_Authorization`*"]
pub const ACTRL_PERM_15: u32 = 16384u32;
#[doc = "*Required features: `Win32_Security_Authorization`*"]
pub const ACTRL_PERM_16: u32 = 32768u32;
#[doc = "*Required features: `Win32_Security_Authorization`*"]
pub const ACTRL_PERM_17: u32 = 65536u32;
#[doc = "*Required features: `Win32_Security_Authorization`*"]
pub const ACTRL_PERM_18: u32 = 131072u32;
#[doc = "*Required features: `Win32_Security_Authorization`*"]
pub const ACTRL_PERM_19: u32 = 262144u32;
#[doc = "*Required features: `Win32_Security_Authorization`*"]
pub const ACTRL_PERM_2: u32 = 2u32;
#[doc = "*Required features: `Win32_Security_Authorization`*"]
pub const ACTRL_PERM_20: u32 = 524288u32;
#[doc = "*Required features: `Win32_Security_Authorization`*"]
pub const ACTRL_PERM_3: u32 = 4u32;
#[doc = "*Required features: `Win32_Security_Authorization`*"]
pub const ACTRL_PERM_4: u32 = 8u32;
#[doc = "*Required features: `Win32_Security_Authorization`*"]
pub const ACTRL_PERM_5: u32 = 16u32;
#[doc = "*Required features: `Win32_Security_Authorization`*"]
pub const ACTRL_PERM_6: u32 = 32u32;
#[doc = "*Required features: `Win32_Security_Authorization`*"]
pub const ACTRL_PERM_7: u32 = 64u32;
#[doc = "*Required features: `Win32_Security_Authorization`*"]
pub const ACTRL_PERM_8: u32 = 128u32;
#[doc = "*Required features: `Win32_Security_Authorization`*"]
pub const ACTRL_PERM_9: u32 = 256u32;
#[doc = "*Required features: `Win32_Security_Authorization`*"]
pub const ACTRL_PRINT_JADMIN: u32 = 16u32;
#[doc = "*Required features: `Win32_Security_Authorization`*"]
pub const ACTRL_PRINT_PADMIN: u32 = 4u32;
#[doc = "*Required features: `Win32_Security_Authorization`*"]
pub const ACTRL_PRINT_PUSE: u32 = 8u32;
#[doc = "*Required features: `Win32_Security_Authorization`*"]
pub const ACTRL_PRINT_SADMIN: u32 = 1u32;
#[doc = "*Required features: `Win32_Security_Authorization`*"]
pub const ACTRL_PRINT_SLIST: u32 = 2u32;
#[cfg(feature = "Win32_Foundation")]
pub struct ACTRL_PROPERTY_ENTRYA(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct ACTRL_PROPERTY_ENTRYW(i32);
#[doc = "*Required features: `Win32_Security_Authorization`*"]
pub const ACTRL_READ_CONTROL: u32 = 268435456u32;
#[doc = "*Required features: `Win32_Security_Authorization`*"]
pub const ACTRL_REG_CREATE_CHILD: u32 = 4u32;
#[doc = "*Required features: `Win32_Security_Authorization`*"]
pub const ACTRL_REG_LINK: u32 = 32u32;
#[doc = "*Required features: `Win32_Security_Authorization`*"]
pub const ACTRL_REG_LIST: u32 = 8u32;
#[doc = "*Required features: `Win32_Security_Authorization`*"]
pub const ACTRL_REG_NOTIFY: u32 = 16u32;
#[doc = "*Required features: `Win32_Security_Authorization`*"]
pub const ACTRL_REG_QUERY: u32 = 1u32;
#[doc = "*Required features: `Win32_Security_Authorization`*"]
pub const ACTRL_REG_SET: u32 = 2u32;
#[doc = "*Required features: `Win32_Security_Authorization`*"]
pub const ACTRL_RESERVED: u32 = 0u32;
#[doc = "*Required features: `Win32_Security_Authorization`*"]
pub const ACTRL_STD_RIGHTS_ALL: u32 = 4160749568u32;
#[doc = "*Required features: `Win32_Security_Authorization`*"]
pub const ACTRL_SVC_GET_INFO: u32 = 1u32;
#[doc = "*Required features: `Win32_Security_Authorization`*"]
pub const ACTRL_SVC_INTERROGATE: u32 = 128u32;
#[doc = "*Required features: `Win32_Security_Authorization`*"]
pub const ACTRL_SVC_LIST: u32 = 8u32;
#[doc = "*Required features: `Win32_Security_Authorization`*"]
pub const ACTRL_SVC_PAUSE: u32 = 64u32;
#[doc = "*Required features: `Win32_Security_Authorization`*"]
pub const ACTRL_SVC_SET_INFO: u32 = 2u32;
#[doc = "*Required features: `Win32_Security_Authorization`*"]
pub const ACTRL_SVC_START: u32 = 16u32;
#[doc = "*Required features: `Win32_Security_Authorization`*"]
pub const ACTRL_SVC_STATUS: u32 = 4u32;
#[doc = "*Required features: `Win32_Security_Authorization`*"]
pub const ACTRL_SVC_STOP: u32 = 32u32;
#[doc = "*Required features: `Win32_Security_Authorization`*"]
pub const ACTRL_SVC_UCONTROL: u32 = 256u32;
#[doc = "*Required features: `Win32_Security_Authorization`*"]
pub const ACTRL_SYNCHRONIZE: u32 = 2147483648u32;
#[doc = "*Required features: `Win32_Security_Authorization`*"]
pub const ACTRL_SYSTEM_ACCESS: u32 = 67108864u32;
#[doc = "*Required features: `Win32_Security_Authorization`*"]
pub const ACTRL_WIN_CLIPBRD: u32 = 1u32;
#[doc = "*Required features: `Win32_Security_Authorization`*"]
pub const ACTRL_WIN_CREATE: u32 = 4u32;
#[doc = "*Required features: `Win32_Security_Authorization`*"]
pub const ACTRL_WIN_EXIT: u32 = 256u32;
#[doc = "*Required features: `Win32_Security_Authorization`*"]
pub const ACTRL_WIN_GLOBAL_ATOMS: u32 = 2u32;
#[doc = "*Required features: `Win32_Security_Authorization`*"]
pub const ACTRL_WIN_LIST: u32 = 16u32;
#[doc = "*Required features: `Win32_Security_Authorization`*"]
pub const ACTRL_WIN_LIST_DESK: u32 = 8u32;
#[doc = "*Required features: `Win32_Security_Authorization`*"]
pub const ACTRL_WIN_READ_ATTRIBS: u32 = 32u32;
#[doc = "*Required features: `Win32_Security_Authorization`*"]
pub const ACTRL_WIN_SCREEN: u32 = 128u32;
#[doc = "*Required features: `Win32_Security_Authorization`*"]
pub const ACTRL_WIN_WRITE_ATTRIBS: u32 = 64u32;
#[doc = "*Required features: `Win32_Security_Authorization`*"]
pub const APF_AuditFailure: u32 = 0u32;
#[doc = "*Required features: `Win32_Security_Authorization`*"]
pub const APF_AuditSuccess: u32 = 1u32;
#[doc = "*Required features: `Win32_Security_Authorization`*"]
pub const APF_ValidFlags: u32 = 1u32;
#[doc = "*Required features: `Win32_Security_Authorization`*"]
pub const AP_ParamTypeBits: u32 = 8u32;
#[doc = "*Required features: `Win32_Security_Authorization`*"]
pub const AP_ParamTypeMask: i32 = 255i32;
pub struct AUDIT_IP_ADDRESS(i32);
pub struct AUDIT_OBJECT_TYPE(i32);
pub struct AUDIT_OBJECT_TYPES(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct AUDIT_PARAM(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct AUDIT_PARAMS(i32);
pub struct AUDIT_PARAM_TYPE(i32);
#[doc = "*Required features: `Win32_Security_Authorization`*"]
pub const AUDIT_TYPE_LEGACY: u32 = 1u32;
#[doc = "*Required features: `Win32_Security_Authorization`*"]
pub const AUDIT_TYPE_WMI: u32 = 2u32;
#[doc = "*Required features: `Win32_Security_Authorization`*"]
pub const AUTHZP_WPD_EVENT: u32 = 16u32;
pub struct AUTHZ_ACCESS_CHECK_FLAGS(i32);
pub struct AUTHZ_ACCESS_CHECK_RESULTS_HANDLE(i32);
pub struct AUTHZ_ACCESS_REPLY(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct AUTHZ_ACCESS_REQUEST(i32);
#[doc = "*Required features: `Win32_Security_Authorization`*"]
pub const AUTHZ_ALLOW_MULTIPLE_SOURCE_INSTANCES: u32 = 1u32;
pub struct AUTHZ_AUDIT_EVENT_HANDLE(i32);
pub struct AUTHZ_AUDIT_EVENT_INFORMATION_CLASS(i32);
pub struct AUTHZ_AUDIT_EVENT_TYPE_HANDLE(i32);
pub struct AUTHZ_AUDIT_EVENT_TYPE_LEGACY(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct AUTHZ_AUDIT_EVENT_TYPE_OLD(i32);
pub struct AUTHZ_AUDIT_EVENT_TYPE_UNION(i32);
#[doc = "*Required features: `Win32_Security_Authorization`*"]
pub const AUTHZ_AUDIT_INSTANCE_INFORMATION: u32 = 2u32;
pub struct AUTHZ_CAP_CHANGE_SUBSCRIPTION_HANDLE__(i32);
pub struct AUTHZ_CLIENT_CONTEXT_HANDLE(i32);
#[doc = "*Required features: `Win32_Security_Authorization`*"]
pub const AUTHZ_COMPUTE_PRIVILEGES: u32 = 8u32;
pub struct AUTHZ_CONTEXT_INFORMATION_CLASS(i32);
#[doc = "*Required features: `Win32_Security_Authorization`*"]
pub const AUTHZ_FLAG_ALLOW_MULTIPLE_SOURCE_INSTANCES: u32 = 1u32;
pub struct AUTHZ_GENERATE_RESULTS(i32);
pub struct AUTHZ_INITIALIZE_OBJECT_ACCESS_AUDIT_EVENT_FLAGS(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct AUTHZ_INIT_INFO(i32);
#[doc = "*Required features: `Win32_Security_Authorization`*"]
pub const AUTHZ_INIT_INFO_VERSION_V1: u32 = 1u32;
#[doc = "*Required features: `Win32_Security_Authorization`*"]
pub const AUTHZ_MIGRATED_LEGACY_PUBLISHER: u32 = 2u32;
#[cfg(feature = "Win32_Foundation")]
pub struct AUTHZ_REGISTRATION_OBJECT_TYPE_NAME_OFFSET(i32);
#[doc = "*Required features: `Win32_Security_Authorization`*"]
pub const AUTHZ_REQUIRE_S4U_LOGON: u32 = 4u32;
pub struct AUTHZ_RESOURCE_MANAGER_FLAGS(i32);
pub struct AUTHZ_RESOURCE_MANAGER_HANDLE(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct AUTHZ_RPC_INIT_INFO_CLIENT(i32);
#[doc = "*Required features: `Win32_Security_Authorization`*"]
pub const AUTHZ_RPC_INIT_INFO_CLIENT_VERSION_V1: u32 = 1u32;
#[cfg(feature = "Win32_Foundation")]
pub struct AUTHZ_SECURITY_ATTRIBUTES_INFORMATION(i32);
#[doc = "*Required features: `Win32_Security_Authorization`*"]
pub const AUTHZ_SECURITY_ATTRIBUTES_INFORMATION_VERSION: u32 = 1u32;
#[doc = "*Required features: `Win32_Security_Authorization`*"]
pub const AUTHZ_SECURITY_ATTRIBUTES_INFORMATION_VERSION_V1: u32 = 1u32;
pub struct AUTHZ_SECURITY_ATTRIBUTE_FLAGS(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct AUTHZ_SECURITY_ATTRIBUTE_FQBN_VALUE(i32);
pub struct AUTHZ_SECURITY_ATTRIBUTE_OCTET_STRING_VALUE(i32);
pub struct AUTHZ_SECURITY_ATTRIBUTE_OPERATION(i32);
#[doc = "*Required features: `Win32_Security_Authorization`*"]
pub const AUTHZ_SECURITY_ATTRIBUTE_TYPE_BOOLEAN: u32 = 6u32;
#[doc = "*Required features: `Win32_Security_Authorization`*"]
pub const AUTHZ_SECURITY_ATTRIBUTE_TYPE_FQBN: u32 = 4u32;
#[doc = "*Required features: `Win32_Security_Authorization`*"]
pub const AUTHZ_SECURITY_ATTRIBUTE_TYPE_INT64: u32 = 1u32;
#[doc = "*Required features: `Win32_Security_Authorization`*"]
pub const AUTHZ_SECURITY_ATTRIBUTE_TYPE_INVALID: u32 = 0u32;
#[doc = "*Required features: `Win32_Security_Authorization`*"]
pub const AUTHZ_SECURITY_ATTRIBUTE_TYPE_OCTET_STRING: u32 = 16u32;
#[doc = "*Required features: `Win32_Security_Authorization`*"]
pub const AUTHZ_SECURITY_ATTRIBUTE_TYPE_SID: u32 = 5u32;
#[doc = "*Required features: `Win32_Security_Authorization`*"]
pub const AUTHZ_SECURITY_ATTRIBUTE_TYPE_STRING: u32 = 3u32;
#[doc = "*Required features: `Win32_Security_Authorization`*"]
pub const AUTHZ_SECURITY_ATTRIBUTE_TYPE_UINT64: u32 = 2u32;
#[cfg(feature = "Win32_Foundation")]
pub struct AUTHZ_SECURITY_ATTRIBUTE_V1(i32);
pub struct AUTHZ_SECURITY_EVENT_PROVIDER_HANDLE(i32);
pub struct AUTHZ_SID_OPERATION(i32);
#[doc = "*Required features: `Win32_Security_Authorization`*"]
pub const AUTHZ_SKIP_TOKEN_GROUPS: u32 = 2u32;
#[cfg(feature = "Win32_Foundation")]
pub struct AUTHZ_SOURCE_SCHEMA_REGISTRATION(i32);
#[doc = "*Required features: `Win32_Security_Authorization`*"]
pub const AUTHZ_WPD_CATEGORY_FLAG: u32 = 16u32;
pub struct AZ_PROP_CONSTANTS(i32);
pub struct AzAuthorizationStore(i32);
pub struct AzBizRuleContext(i32);
pub struct AzPrincipalLocator(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct EXPLICIT_ACCESS_A(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct EXPLICIT_ACCESS_W(i32);
pub struct FN_OBJECT_MGR_FUNCTIONS(i32);
pub struct FN_PROGRESS(i32);
pub struct IAzApplication(pub *mut ::core::ffi::c_void);
pub struct IAzApplication2(pub *mut ::core::ffi::c_void);
pub struct IAzApplication3(pub *mut ::core::ffi::c_void);
pub struct IAzApplicationGroup(pub *mut ::core::ffi::c_void);
pub struct IAzApplicationGroup2(pub *mut ::core::ffi::c_void);
pub struct IAzApplicationGroups(pub *mut ::core::ffi::c_void);
pub struct IAzApplications(pub *mut ::core::ffi::c_void);
pub struct IAzAuthorizationStore(pub *mut ::core::ffi::c_void);
pub struct IAzAuthorizationStore2(pub *mut ::core::ffi::c_void);
pub struct IAzAuthorizationStore3(pub *mut ::core::ffi::c_void);
pub struct IAzBizRuleContext(pub *mut ::core::ffi::c_void);
pub struct IAzBizRuleInterfaces(pub *mut ::core::ffi::c_void);
pub struct IAzBizRuleParameters(pub *mut ::core::ffi::c_void);
pub struct IAzClientContext(pub *mut ::core::ffi::c_void);
pub struct IAzClientContext2(pub *mut ::core::ffi::c_void);
pub struct IAzClientContext3(pub *mut ::core::ffi::c_void);
pub struct IAzNameResolver(pub *mut ::core::ffi::c_void);
pub struct IAzObjectPicker(pub *mut ::core::ffi::c_void);
pub struct IAzOperation(pub *mut ::core::ffi::c_void);
pub struct IAzOperation2(pub *mut ::core::ffi::c_void);
pub struct IAzOperations(pub *mut ::core::ffi::c_void);
pub struct IAzPrincipalLocator(pub *mut ::core::ffi::c_void);
pub struct IAzRole(pub *mut ::core::ffi::c_void);
pub struct IAzRoleAssignment(pub *mut ::core::ffi::c_void);
pub struct IAzRoleAssignments(pub *mut ::core::ffi::c_void);
pub struct IAzRoleDefinition(pub *mut ::core::ffi::c_void);
pub struct IAzRoleDefinitions(pub *mut ::core::ffi::c_void);
pub struct IAzRoles(pub *mut ::core::ffi::c_void);
pub struct IAzScope(pub *mut ::core::ffi::c_void);
pub struct IAzScope2(pub *mut ::core::ffi::c_void);
pub struct IAzScopes(pub *mut ::core::ffi::c_void);
pub struct IAzTask(pub *mut ::core::ffi::c_void);
pub struct IAzTask2(pub *mut ::core::ffi::c_void);
pub struct IAzTasks(pub *mut ::core::ffi::c_void);
#[doc = "*Required features: `Win32_Security_Authorization`*"]
pub const INHERITED_ACCESS_ENTRY: u32 = 16u32;
#[cfg(feature = "Win32_Foundation")]
pub struct INHERITED_FROMA(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct INHERITED_FROMW(i32);
#[doc = "*Required features: `Win32_Security_Authorization`*"]
pub const INHERITED_GRANDPARENT: u32 = 536870912u32;
#[doc = "*Required features: `Win32_Security_Authorization`*"]
pub const INHERITED_PARENT: u32 = 268435456u32;
pub struct MULTIPLE_TRUSTEE_OPERATION(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct OBJECTS_AND_NAME_A(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct OBJECTS_AND_NAME_W(i32);
pub struct OBJECTS_AND_SID(i32);
#[doc = "*Required features: `Win32_Security_Authorization`*"]
pub const OLESCRIPT_E_SYNTAX: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147352319i32 as _);
pub struct PFN_AUTHZ_COMPUTE_DYNAMIC_GROUPS(i32);
pub struct PFN_AUTHZ_DYNAMIC_ACCESS_CHECK(i32);
pub struct PFN_AUTHZ_FREE_CENTRAL_ACCESS_POLICY(i32);
pub struct PFN_AUTHZ_FREE_DYNAMIC_GROUPS(i32);
pub struct PFN_AUTHZ_GET_CENTRAL_ACCESS_POLICY(i32);
pub struct PROG_INVOKE_SETTING(i32);
#[doc = "*Required features: `Win32_Security_Authorization`*"]
pub const SDDL_ALIAS_SIZE: u32 = 2u32;
#[doc = "*Required features: `Win32_Security_Authorization`*"]
pub const SDDL_REVISION: u32 = 1u32;
#[doc = "*Required features: `Win32_Security_Authorization`*"]
pub const SDDL_REVISION_1: u32 = 1u32;
pub struct SE_OBJECT_TYPE(i32);
pub struct TREE_SEC_INFO(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct TRUSTEE_A(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct TRUSTEE_ACCESSA(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct TRUSTEE_ACCESSW(i32);
#[doc = "*Required features: `Win32_Security_Authorization`*"]
pub const TRUSTEE_ACCESS_ALL: i32 = -1i32;
#[doc = "*Required features: `Win32_Security_Authorization`*"]
pub const TRUSTEE_ACCESS_ALLOWED: i32 = 1i32;
#[doc = "*Required features: `Win32_Security_Authorization`*"]
pub const TRUSTEE_ACCESS_EXPLICIT: i32 = 1i32;
#[doc = "*Required features: `Win32_Security_Authorization`*"]
pub const TRUSTEE_ACCESS_READ: i32 = 2i32;
#[doc = "*Required features: `Win32_Security_Authorization`*"]
pub const TRUSTEE_ACCESS_WRITE: i32 = 4i32;
pub struct TRUSTEE_FORM(i32);
pub struct TRUSTEE_TYPE(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct TRUSTEE_W(i32);
#[doc = "*Required features: `Win32_Security_Authorization`*"]
pub const _AUTHZ_SS_MAXSIZE: u32 = 128u32;
