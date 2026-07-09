#[cfg(feature = "winnt")]
windows_link::link!("advapi32.dll" "system" fn AccessCheck(psecuritydescriptor : super::winnt::PSECURITY_DESCRIPTOR, clienttoken : super::winnt::HANDLE, desiredaccess : u32, genericmapping : *const super::winnt::GENERIC_MAPPING, privilegeset : *mut super::winnt::PRIVILEGE_SET, privilegesetlength : *mut u32, grantedaccess : *mut u32, accessstatus : *mut windows_sys::core::BOOL) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("advapi32.dll" "system" fn AccessCheckAndAuditAlarmW(subsystemname : windows_sys::core::PCWSTR, handleid : *const core::ffi::c_void, objecttypename : windows_sys::core::PCWSTR, objectname : windows_sys::core::PCWSTR, securitydescriptor : super::winnt::PSECURITY_DESCRIPTOR, desiredaccess : u32, genericmapping : *const super::winnt::GENERIC_MAPPING, objectcreation : windows_sys::core::BOOL, grantedaccess : *mut u32, accessstatus : *mut windows_sys::core::BOOL, pfgenerateonclose : *mut windows_sys::core::BOOL) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("advapi32.dll" "system" fn AccessCheckByType(psecuritydescriptor : super::winnt::PSECURITY_DESCRIPTOR, principalselfsid : super::winnt::PSID, clienttoken : super::winnt::HANDLE, desiredaccess : u32, objecttypelist : *mut super::winnt::OBJECT_TYPE_LIST, objecttypelistlength : u32, genericmapping : *const super::winnt::GENERIC_MAPPING, privilegeset : *mut super::winnt::PRIVILEGE_SET, privilegesetlength : *mut u32, grantedaccess : *mut u32, accessstatus : *mut windows_sys::core::BOOL) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("advapi32.dll" "system" fn AccessCheckByTypeAndAuditAlarmW(subsystemname : windows_sys::core::PCWSTR, handleid : *const core::ffi::c_void, objecttypename : windows_sys::core::PCWSTR, objectname : windows_sys::core::PCWSTR, securitydescriptor : super::winnt::PSECURITY_DESCRIPTOR, principalselfsid : super::winnt::PSID, desiredaccess : u32, audittype : super::winnt::AUDIT_EVENT_TYPE, flags : u32, objecttypelist : *mut super::winnt::OBJECT_TYPE_LIST, objecttypelistlength : u32, genericmapping : *const super::winnt::GENERIC_MAPPING, objectcreation : windows_sys::core::BOOL, grantedaccess : *mut u32, accessstatus : *mut windows_sys::core::BOOL, pfgenerateonclose : *mut windows_sys::core::BOOL) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("advapi32.dll" "system" fn AccessCheckByTypeResultList(psecuritydescriptor : super::winnt::PSECURITY_DESCRIPTOR, principalselfsid : super::winnt::PSID, clienttoken : super::winnt::HANDLE, desiredaccess : u32, objecttypelist : *mut super::winnt::OBJECT_TYPE_LIST, objecttypelistlength : u32, genericmapping : *const super::winnt::GENERIC_MAPPING, privilegeset : *mut super::winnt::PRIVILEGE_SET, privilegesetlength : *mut u32, grantedaccesslist : *mut u32, accessstatuslist : *mut u32) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("advapi32.dll" "system" fn AccessCheckByTypeResultListAndAuditAlarmByHandleW(subsystemname : windows_sys::core::PCWSTR, handleid : *const core::ffi::c_void, clienttoken : super::winnt::HANDLE, objecttypename : windows_sys::core::PCWSTR, objectname : windows_sys::core::PCWSTR, securitydescriptor : super::winnt::PSECURITY_DESCRIPTOR, principalselfsid : super::winnt::PSID, desiredaccess : u32, audittype : super::winnt::AUDIT_EVENT_TYPE, flags : u32, objecttypelist : *mut super::winnt::OBJECT_TYPE_LIST, objecttypelistlength : u32, genericmapping : *const super::winnt::GENERIC_MAPPING, objectcreation : windows_sys::core::BOOL, grantedaccesslist : *mut u32, accessstatuslist : *mut u32, pfgenerateonclose : *mut windows_sys::core::BOOL) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("advapi32.dll" "system" fn AccessCheckByTypeResultListAndAuditAlarmW(subsystemname : windows_sys::core::PCWSTR, handleid : *const core::ffi::c_void, objecttypename : windows_sys::core::PCWSTR, objectname : windows_sys::core::PCWSTR, securitydescriptor : super::winnt::PSECURITY_DESCRIPTOR, principalselfsid : super::winnt::PSID, desiredaccess : u32, audittype : super::winnt::AUDIT_EVENT_TYPE, flags : u32, objecttypelist : *mut super::winnt::OBJECT_TYPE_LIST, objecttypelistlength : u32, genericmapping : *const super::winnt::GENERIC_MAPPING, objectcreation : windows_sys::core::BOOL, grantedaccesslist : *mut u32, accessstatuslist : *mut u32, pfgenerateonclose : *mut windows_sys::core::BOOL) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("advapi32.dll" "system" fn AddAccessAllowedAce(pacl : *mut super::winnt::ACL, dwacerevision : u32, accessmask : u32, psid : super::winnt::PSID) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("advapi32.dll" "system" fn AddAccessAllowedAceEx(pacl : *mut super::winnt::ACL, dwacerevision : u32, aceflags : u32, accessmask : u32, psid : super::winnt::PSID) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("advapi32.dll" "system" fn AddAccessAllowedObjectAce(pacl : *mut super::winnt::ACL, dwacerevision : u32, aceflags : u32, accessmask : u32, objecttypeguid : *const windows_sys::core::GUID, inheritedobjecttypeguid : *const windows_sys::core::GUID, psid : super::winnt::PSID) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("advapi32.dll" "system" fn AddAccessDeniedAce(pacl : *mut super::winnt::ACL, dwacerevision : u32, accessmask : u32, psid : super::winnt::PSID) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("advapi32.dll" "system" fn AddAccessDeniedAceEx(pacl : *mut super::winnt::ACL, dwacerevision : u32, aceflags : u32, accessmask : u32, psid : super::winnt::PSID) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("advapi32.dll" "system" fn AddAccessDeniedObjectAce(pacl : *mut super::winnt::ACL, dwacerevision : u32, aceflags : u32, accessmask : u32, objecttypeguid : *const windows_sys::core::GUID, inheritedobjecttypeguid : *const windows_sys::core::GUID, psid : super::winnt::PSID) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("advapi32.dll" "system" fn AddAce(pacl : *mut super::winnt::ACL, dwacerevision : u32, dwstartingaceindex : u32, pacelist : *const core::ffi::c_void, nacelistlength : u32) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("advapi32.dll" "system" fn AddAuditAccessAce(pacl : *mut super::winnt::ACL, dwacerevision : u32, dwaccessmask : u32, psid : super::winnt::PSID, bauditsuccess : windows_sys::core::BOOL, bauditfailure : windows_sys::core::BOOL) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("advapi32.dll" "system" fn AddAuditAccessAceEx(pacl : *mut super::winnt::ACL, dwacerevision : u32, aceflags : u32, dwaccessmask : u32, psid : super::winnt::PSID, bauditsuccess : windows_sys::core::BOOL, bauditfailure : windows_sys::core::BOOL) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("advapi32.dll" "system" fn AddAuditAccessObjectAce(pacl : *mut super::winnt::ACL, dwacerevision : u32, aceflags : u32, accessmask : u32, objecttypeguid : *const windows_sys::core::GUID, inheritedobjecttypeguid : *const windows_sys::core::GUID, psid : super::winnt::PSID, bauditsuccess : windows_sys::core::BOOL, bauditfailure : windows_sys::core::BOOL) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("advapi32.dll" "system" fn AddMandatoryAce(pacl : *mut super::winnt::ACL, dwacerevision : u32, aceflags : u32, mandatorypolicy : u32, plabelsid : super::winnt::PSID) -> windows_sys::core::BOOL);
#[cfg(all(feature = "basetsd", feature = "winnt"))]
windows_link::link!("kernel32.dll" "system" fn AddResourceAttributeAce(pacl : *mut super::winnt::ACL, dwacerevision : u32, aceflags : u32, accessmask : u32, psid : super::winnt::PSID, pattributeinfo : *const super::winnt::CLAIM_SECURITY_ATTRIBUTES_INFORMATION, preturnlength : *mut u32) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("kernel32.dll" "system" fn AddScopedPolicyIDAce(pacl : *mut super::winnt::ACL, dwacerevision : u32, aceflags : u32, accessmask : u32, psid : super::winnt::PSID) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("advapi32.dll" "system" fn AdjustTokenGroups(tokenhandle : super::winnt::HANDLE, resettodefault : windows_sys::core::BOOL, newstate : *const super::winnt::TOKEN_GROUPS, bufferlength : u32, previousstate : *mut super::winnt::TOKEN_GROUPS, returnlength : *mut u32) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("advapi32.dll" "system" fn AdjustTokenPrivileges(tokenhandle : super::winnt::HANDLE, disableallprivileges : windows_sys::core::BOOL, newstate : *const super::winnt::TOKEN_PRIVILEGES, bufferlength : u32, previousstate : *mut super::winnt::TOKEN_PRIVILEGES, returnlength : *mut u32) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("advapi32.dll" "system" fn AllocateAndInitializeSid(pidentifierauthority : *const super::winnt::SID_IDENTIFIER_AUTHORITY, nsubauthoritycount : u8, nsubauthority0 : u32, nsubauthority1 : u32, nsubauthority2 : u32, nsubauthority3 : u32, nsubauthority4 : u32, nsubauthority5 : u32, nsubauthority6 : u32, nsubauthority7 : u32, psid : *mut super::winnt::PSID) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("advapi32.dll" "system" fn AllocateLocallyUniqueId(luid : *mut super::winnt::LUID) -> windows_sys::core::BOOL);
windows_link::link!("advapi32.dll" "system" fn AreAllAccessesGranted(grantedaccess : u32, desiredaccess : u32) -> windows_sys::core::BOOL);
windows_link::link!("advapi32.dll" "system" fn AreAnyAccessesGranted(grantedaccess : u32, desiredaccess : u32) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("kernel32.dll" "system" fn CheckTokenCapability(tokenhandle : super::winnt::HANDLE, capabilitysidtocheck : super::winnt::PSID, hascapability : *mut windows_sys::core::BOOL) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("advapi32.dll" "system" fn CheckTokenMembership(tokenhandle : super::winnt::HANDLE, sidtocheck : super::winnt::PSID, ismember : *mut windows_sys::core::BOOL) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("kernel32.dll" "system" fn CheckTokenMembershipEx(tokenhandle : super::winnt::HANDLE, sidtocheck : super::winnt::PSID, flags : u32, ismember : *mut windows_sys::core::BOOL) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("advapi32.dll" "system" fn ConvertToAutoInheritPrivateObjectSecurity(parentdescriptor : super::winnt::PSECURITY_DESCRIPTOR, currentsecuritydescriptor : super::winnt::PSECURITY_DESCRIPTOR, newsecuritydescriptor : *mut super::winnt::PSECURITY_DESCRIPTOR, objecttype : *const windows_sys::core::GUID, isdirectoryobject : bool, genericmapping : *const super::winnt::GENERIC_MAPPING) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("advapi32.dll" "system" fn CopySid(ndestinationsidlength : u32, pdestinationsid : super::winnt::PSID, psourcesid : super::winnt::PSID) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("advapi32.dll" "system" fn CreatePrivateObjectSecurity(parentdescriptor : super::winnt::PSECURITY_DESCRIPTOR, creatordescriptor : super::winnt::PSECURITY_DESCRIPTOR, newdescriptor : *mut super::winnt::PSECURITY_DESCRIPTOR, isdirectoryobject : windows_sys::core::BOOL, token : super::winnt::HANDLE, genericmapping : *const super::winnt::GENERIC_MAPPING) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("advapi32.dll" "system" fn CreatePrivateObjectSecurityEx(parentdescriptor : super::winnt::PSECURITY_DESCRIPTOR, creatordescriptor : super::winnt::PSECURITY_DESCRIPTOR, newdescriptor : *mut super::winnt::PSECURITY_DESCRIPTOR, objecttype : *const windows_sys::core::GUID, iscontainerobject : windows_sys::core::BOOL, autoinheritflags : u32, token : super::winnt::HANDLE, genericmapping : *const super::winnt::GENERIC_MAPPING) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("advapi32.dll" "system" fn CreatePrivateObjectSecurityWithMultipleInheritance(parentdescriptor : super::winnt::PSECURITY_DESCRIPTOR, creatordescriptor : super::winnt::PSECURITY_DESCRIPTOR, newdescriptor : *mut super::winnt::PSECURITY_DESCRIPTOR, objecttypes : *const *const windows_sys::core::GUID, guidcount : u32, iscontainerobject : windows_sys::core::BOOL, autoinheritflags : u32, token : super::winnt::HANDLE, genericmapping : *const super::winnt::GENERIC_MAPPING) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("advapi32.dll" "system" fn CreateRestrictedToken(existingtokenhandle : super::winnt::HANDLE, flags : u32, disablesidcount : u32, sidstodisable : *const super::winnt::SID_AND_ATTRIBUTES, deleteprivilegecount : u32, privilegestodelete : *const super::winnt::LUID_AND_ATTRIBUTES, restrictedsidcount : u32, sidstorestrict : *const super::winnt::SID_AND_ATTRIBUTES, newtokenhandle : *mut super::winnt::HANDLE) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("advapi32.dll" "system" fn CreateWellKnownSid(wellknownsidtype : super::winnt::WELL_KNOWN_SID_TYPE, domainsid : super::winnt::PSID, psid : super::winnt::PSID, cbsid : *mut u32) -> windows_sys::core::BOOL);
windows_link::link!("advapi32.dll" "system" fn CveEventWrite(cveid : windows_sys::core::PCWSTR, additionaldetails : windows_sys::core::PCWSTR) -> i32);
#[cfg(feature = "winnt")]
windows_link::link!("advapi32.dll" "system" fn DeleteAce(pacl : *mut super::winnt::ACL, dwaceindex : u32) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("api-ms-win-security-base-l1-2-2.dll" "system" fn DeriveCapabilitySidsFromName(capname : windows_sys::core::PCWSTR, capabilitygroupsids : *mut *mut super::winnt::PSID, capabilitygroupsidcount : *mut u32, capabilitysids : *mut *mut super::winnt::PSID, capabilitysidcount : *mut u32) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("advapi32.dll" "system" fn DestroyPrivateObjectSecurity(objectdescriptor : *mut super::winnt::PSECURITY_DESCRIPTOR) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("advapi32.dll" "system" fn DuplicateToken(existingtokenhandle : super::winnt::HANDLE, impersonationlevel : super::winnt::SECURITY_IMPERSONATION_LEVEL, duplicatetokenhandle : *mut super::winnt::HANDLE) -> windows_sys::core::BOOL);
#[cfg(all(feature = "minwinbase", feature = "winnt"))]
windows_link::link!("advapi32.dll" "system" fn DuplicateTokenEx(hexistingtoken : super::winnt::HANDLE, dwdesiredaccess : u32, lptokenattributes : *const super::minwinbase::SECURITY_ATTRIBUTES, impersonationlevel : super::winnt::SECURITY_IMPERSONATION_LEVEL, tokentype : super::winnt::TOKEN_TYPE, phnewtoken : *mut super::winnt::HANDLE) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("advapi32.dll" "system" fn EqualDomainSid(psid1 : super::winnt::PSID, psid2 : super::winnt::PSID, pfequal : *mut windows_sys::core::BOOL) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("advapi32.dll" "system" fn EqualPrefixSid(psid1 : super::winnt::PSID, psid2 : super::winnt::PSID) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("advapi32.dll" "system" fn EqualSid(psid1 : super::winnt::PSID, psid2 : super::winnt::PSID) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("advapi32.dll" "system" fn FindFirstFreeAce(pacl : *const super::winnt::ACL, pace : *mut *mut core::ffi::c_void) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("advapi32.dll" "system" fn FreeSid(psid : super::winnt::PSID) -> *mut core::ffi::c_void);
#[cfg(feature = "winnt")]
windows_link::link!("advapi32.dll" "system" fn GetAce(pacl : *const super::winnt::ACL, dwaceindex : u32, pace : *mut *mut core::ffi::c_void) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("advapi32.dll" "system" fn GetAclInformation(pacl : *const super::winnt::ACL, paclinformation : *mut core::ffi::c_void, naclinformationlength : u32, dwaclinformationclass : super::winnt::ACL_INFORMATION_CLASS) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("kernel32.dll" "system" fn GetAppContainerAce(acl : *const super::winnt::ACL, startingaceindex : u32, appcontainerace : *mut *mut core::ffi::c_void, appcontaineraceindex : *mut u32) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("kernel32.dll" "system" fn GetCachedSigningLevel(file : super::winnt::HANDLE, flags : *mut u32, signinglevel : *mut u32, thumbprint : *mut u8, thumbprintsize : *mut u32, thumbprintalgorithm : *mut u32) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("advapi32.dll" "system" fn GetFileSecurityW(lpfilename : windows_sys::core::PCWSTR, requestedinformation : super::winnt::SECURITY_INFORMATION, psecuritydescriptor : super::winnt::PSECURITY_DESCRIPTOR, nlength : u32, lpnlengthneeded : *mut u32) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("advapi32.dll" "system" fn GetKernelObjectSecurity(handle : super::winnt::HANDLE, requestedinformation : super::winnt::SECURITY_INFORMATION, psecuritydescriptor : super::winnt::PSECURITY_DESCRIPTOR, nlength : u32, lpnlengthneeded : *mut u32) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("advapi32.dll" "system" fn GetLengthSid(psid : super::winnt::PSID) -> u32);
#[cfg(feature = "winnt")]
windows_link::link!("advapi32.dll" "system" fn GetPrivateObjectSecurity(objectdescriptor : super::winnt::PSECURITY_DESCRIPTOR, securityinformation : super::winnt::SECURITY_INFORMATION, resultantdescriptor : super::winnt::PSECURITY_DESCRIPTOR, descriptorlength : u32, returnlength : *mut u32) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("advapi32.dll" "system" fn GetSecurityDescriptorControl(psecuritydescriptor : super::winnt::PSECURITY_DESCRIPTOR, pcontrol : *mut u16, lpdwrevision : *mut u32) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("advapi32.dll" "system" fn GetSecurityDescriptorDacl(psecuritydescriptor : super::winnt::PSECURITY_DESCRIPTOR, lpbdaclpresent : *mut windows_sys::core::BOOL, pdacl : *mut super::winnt::PACL, lpbdacldefaulted : *mut windows_sys::core::BOOL) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("advapi32.dll" "system" fn GetSecurityDescriptorGroup(psecuritydescriptor : super::winnt::PSECURITY_DESCRIPTOR, pgroup : *mut super::winnt::PSID, lpbgroupdefaulted : *mut windows_sys::core::BOOL) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("advapi32.dll" "system" fn GetSecurityDescriptorLength(psecuritydescriptor : super::winnt::PSECURITY_DESCRIPTOR) -> u32);
#[cfg(feature = "winnt")]
windows_link::link!("advapi32.dll" "system" fn GetSecurityDescriptorOwner(psecuritydescriptor : super::winnt::PSECURITY_DESCRIPTOR, powner : *mut super::winnt::PSID, lpbownerdefaulted : *mut windows_sys::core::BOOL) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("advapi32.dll" "system" fn GetSecurityDescriptorRMControl(securitydescriptor : super::winnt::PSECURITY_DESCRIPTOR, rmcontrol : *mut u8) -> u32);
#[cfg(feature = "winnt")]
windows_link::link!("advapi32.dll" "system" fn GetSecurityDescriptorSacl(psecuritydescriptor : super::winnt::PSECURITY_DESCRIPTOR, lpbsaclpresent : *mut windows_sys::core::BOOL, psacl : *mut super::winnt::PACL, lpbsacldefaulted : *mut windows_sys::core::BOOL) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("advapi32.dll" "system" fn GetSidIdentifierAuthority(psid : super::winnt::PSID) -> super::winnt::PSID_IDENTIFIER_AUTHORITY);
windows_link::link!("advapi32.dll" "system" fn GetSidLengthRequired(nsubauthoritycount : u8) -> u32);
#[cfg(all(feature = "minwindef", feature = "winnt"))]
windows_link::link!("advapi32.dll" "system" fn GetSidSubAuthority(psid : super::winnt::PSID, nsubauthority : u32) -> super::minwindef::PDWORD);
#[cfg(all(feature = "minwindef", feature = "winnt"))]
windows_link::link!("advapi32.dll" "system" fn GetSidSubAuthorityCount(psid : super::winnt::PSID) -> super::minwindef::PUCHAR);
#[cfg(feature = "winnt")]
windows_link::link!("advapi32.dll" "system" fn GetTokenInformation(tokenhandle : super::winnt::HANDLE, tokeninformationclass : super::winnt::TOKEN_INFORMATION_CLASS, tokeninformation : *mut core::ffi::c_void, tokeninformationlength : u32, returnlength : *mut u32) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("advapi32.dll" "system" fn GetWindowsAccountDomainSid(psid : super::winnt::PSID, pdomainsid : super::winnt::PSID, cbdomainsid : *mut u32) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("advapi32.dll" "system" fn ImpersonateAnonymousToken(threadhandle : super::winnt::HANDLE) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("advapi32.dll" "system" fn ImpersonateLoggedOnUser(htoken : super::winnt::HANDLE) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("advapi32.dll" "system" fn ImpersonateSelf(impersonationlevel : super::winnt::SECURITY_IMPERSONATION_LEVEL) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("advapi32.dll" "system" fn InitializeAcl(pacl : *mut super::winnt::ACL, nacllength : u32, dwaclrevision : u32) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("advapi32.dll" "system" fn InitializeSecurityDescriptor(psecuritydescriptor : super::winnt::PSECURITY_DESCRIPTOR, dwrevision : u32) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("advapi32.dll" "system" fn InitializeSid(sid : super::winnt::PSID, pidentifierauthority : *const super::winnt::SID_IDENTIFIER_AUTHORITY, nsubauthoritycount : u8) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("advapi32.dll" "system" fn IsTokenRestricted(tokenhandle : super::winnt::HANDLE) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("advapi32.dll" "system" fn IsValidAcl(pacl : *const super::winnt::ACL) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("advapi32.dll" "system" fn IsValidSecurityDescriptor(psecuritydescriptor : super::winnt::PSECURITY_DESCRIPTOR) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("advapi32.dll" "system" fn IsValidSid(psid : super::winnt::PSID) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("advapi32.dll" "system" fn IsWellKnownSid(psid : super::winnt::PSID, wellknownsidtype : super::winnt::WELL_KNOWN_SID_TYPE) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("advapi32.dll" "system" fn MakeAbsoluteSD(pselfrelativesecuritydescriptor : super::winnt::PSECURITY_DESCRIPTOR, pabsolutesecuritydescriptor : super::winnt::PSECURITY_DESCRIPTOR, lpdwabsolutesecuritydescriptorsize : *mut u32, pdacl : *mut super::winnt::ACL, lpdwdaclsize : *mut u32, psacl : *mut super::winnt::ACL, lpdwsaclsize : *mut u32, powner : super::winnt::PSID, lpdwownersize : *mut u32, pprimarygroup : super::winnt::PSID, lpdwprimarygroupsize : *mut u32) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("advapi32.dll" "system" fn MakeSelfRelativeSD(pabsolutesecuritydescriptor : super::winnt::PSECURITY_DESCRIPTOR, pselfrelativesecuritydescriptor : super::winnt::PSECURITY_DESCRIPTOR, lpdwbufferlength : *mut u32) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("advapi32.dll" "system" fn MapGenericMask(accessmask : *mut u32, genericmapping : *const super::winnt::GENERIC_MAPPING));
windows_link::link!("advapi32.dll" "system" fn ObjectCloseAuditAlarmW(subsystemname : windows_sys::core::PCWSTR, handleid : *const core::ffi::c_void, generateonclose : windows_sys::core::BOOL) -> windows_sys::core::BOOL);
windows_link::link!("advapi32.dll" "system" fn ObjectDeleteAuditAlarmW(subsystemname : windows_sys::core::PCWSTR, handleid : *const core::ffi::c_void, generateonclose : windows_sys::core::BOOL) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("advapi32.dll" "system" fn ObjectOpenAuditAlarmW(subsystemname : windows_sys::core::PCWSTR, handleid : *const core::ffi::c_void, objecttypename : windows_sys::core::PCWSTR, objectname : windows_sys::core::PCWSTR, psecuritydescriptor : super::winnt::PSECURITY_DESCRIPTOR, clienttoken : super::winnt::HANDLE, desiredaccess : u32, grantedaccess : u32, privileges : *const super::winnt::PRIVILEGE_SET, objectcreation : windows_sys::core::BOOL, accessgranted : windows_sys::core::BOOL, generateonclose : *mut windows_sys::core::BOOL) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("advapi32.dll" "system" fn ObjectPrivilegeAuditAlarmW(subsystemname : windows_sys::core::PCWSTR, handleid : *const core::ffi::c_void, clienttoken : super::winnt::HANDLE, desiredaccess : u32, privileges : *const super::winnt::PRIVILEGE_SET, accessgranted : windows_sys::core::BOOL) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("advapi32.dll" "system" fn PrivilegeCheck(clienttoken : super::winnt::HANDLE, requiredprivileges : *mut super::winnt::PRIVILEGE_SET, pfresult : *mut windows_sys::core::BOOL) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("advapi32.dll" "system" fn PrivilegedServiceAuditAlarmW(subsystemname : windows_sys::core::PCWSTR, servicename : windows_sys::core::PCWSTR, clienttoken : super::winnt::HANDLE, privileges : *const super::winnt::PRIVILEGE_SET, accessgranted : windows_sys::core::BOOL) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("advapi32.dll" "system" fn QuerySecurityAccessMask(securityinformation : super::winnt::SECURITY_INFORMATION, desiredaccess : *mut u32));
windows_link::link!("advapi32.dll" "system" fn RevertToSelf() -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("advapi32.dll" "system" fn SetAclInformation(pacl : *mut super::winnt::ACL, paclinformation : *const core::ffi::c_void, naclinformationlength : u32, dwaclinformationclass : super::winnt::ACL_INFORMATION_CLASS) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("kernel32.dll" "system" fn SetCachedSigningLevel(sourcefiles : *const super::winnt::HANDLE, sourcefilecount : u32, flags : u32, targetfile : super::winnt::HANDLE) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("advapi32.dll" "system" fn SetFileSecurityW(lpfilename : windows_sys::core::PCWSTR, securityinformation : super::winnt::SECURITY_INFORMATION, psecuritydescriptor : super::winnt::PSECURITY_DESCRIPTOR) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("advapi32.dll" "system" fn SetKernelObjectSecurity(handle : super::winnt::HANDLE, securityinformation : super::winnt::SECURITY_INFORMATION, securitydescriptor : super::winnt::PSECURITY_DESCRIPTOR) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("advapi32.dll" "system" fn SetPrivateObjectSecurity(securityinformation : super::winnt::SECURITY_INFORMATION, modificationdescriptor : super::winnt::PSECURITY_DESCRIPTOR, objectssecuritydescriptor : *mut super::winnt::PSECURITY_DESCRIPTOR, genericmapping : *const super::winnt::GENERIC_MAPPING, token : super::winnt::HANDLE) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("advapi32.dll" "system" fn SetPrivateObjectSecurityEx(securityinformation : super::winnt::SECURITY_INFORMATION, modificationdescriptor : super::winnt::PSECURITY_DESCRIPTOR, objectssecuritydescriptor : *mut super::winnt::PSECURITY_DESCRIPTOR, autoinheritflags : u32, genericmapping : *const super::winnt::GENERIC_MAPPING, token : super::winnt::HANDLE) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("advapi32.dll" "system" fn SetSecurityAccessMask(securityinformation : super::winnt::SECURITY_INFORMATION, desiredaccess : *mut u32));
#[cfg(feature = "winnt")]
windows_link::link!("advapi32.dll" "system" fn SetSecurityDescriptorControl(psecuritydescriptor : super::winnt::PSECURITY_DESCRIPTOR, controlbitsofinterest : super::winnt::SECURITY_DESCRIPTOR_CONTROL, controlbitstoset : super::winnt::SECURITY_DESCRIPTOR_CONTROL) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("advapi32.dll" "system" fn SetSecurityDescriptorDacl(psecuritydescriptor : super::winnt::PSECURITY_DESCRIPTOR, bdaclpresent : windows_sys::core::BOOL, pdacl : *const super::winnt::ACL, bdacldefaulted : windows_sys::core::BOOL) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("advapi32.dll" "system" fn SetSecurityDescriptorGroup(psecuritydescriptor : super::winnt::PSECURITY_DESCRIPTOR, pgroup : super::winnt::PSID, bgroupdefaulted : windows_sys::core::BOOL) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("advapi32.dll" "system" fn SetSecurityDescriptorOwner(psecuritydescriptor : super::winnt::PSECURITY_DESCRIPTOR, powner : super::winnt::PSID, bownerdefaulted : windows_sys::core::BOOL) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("advapi32.dll" "system" fn SetSecurityDescriptorRMControl(securitydescriptor : super::winnt::PSECURITY_DESCRIPTOR, rmcontrol : *const u8) -> u32);
#[cfg(feature = "winnt")]
windows_link::link!("advapi32.dll" "system" fn SetSecurityDescriptorSacl(psecuritydescriptor : super::winnt::PSECURITY_DESCRIPTOR, bsaclpresent : windows_sys::core::BOOL, psacl : *const super::winnt::ACL, bsacldefaulted : windows_sys::core::BOOL) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("advapi32.dll" "system" fn SetTokenInformation(tokenhandle : super::winnt::HANDLE, tokeninformationclass : super::winnt::TOKEN_INFORMATION_CLASS, tokeninformation : *const core::ffi::c_void, tokeninformationlength : u32) -> windows_sys::core::BOOL);
pub const SIGNING_LEVEL_FILE_CACHE_FLAG_NOT_VALIDATED: u32 = 1;
pub const SIGNING_LEVEL_FILE_CACHE_FLAG_VALIDATE_ONLY: u32 = 4;
pub const SIGNING_LEVEL_MICROSOFT: u32 = 8;
