#[cfg(all(feature = "minwindef", feature = "winnt"))]
windows_link::link!("advapi32.dll" "system" fn AccessCheck(psecuritydescriptor : super::PSECURITY_DESCRIPTOR, clienttoken : super::HANDLE, desiredaccess : u32, genericmapping : super::PGENERIC_MAPPING, privilegeset : super::PPRIVILEGE_SET, privilegesetlength : super::LPDWORD, grantedaccess : super::LPDWORD, accessstatus : super::LPBOOL) -> windows_sys::core::BOOL);
#[cfg(all(feature = "minwindef", feature = "winnt"))]
windows_link::link!("advapi32.dll" "system" fn AccessCheckAndAuditAlarmW(subsystemname : windows_sys::core::PCWSTR, handleid : *const core::ffi::c_void, objecttypename : windows_sys::core::PCWSTR, objectname : windows_sys::core::PCWSTR, securitydescriptor : super::PSECURITY_DESCRIPTOR, desiredaccess : u32, genericmapping : super::PGENERIC_MAPPING, objectcreation : windows_sys::core::BOOL, grantedaccess : super::LPDWORD, accessstatus : super::LPBOOL, pfgenerateonclose : super::LPBOOL) -> windows_sys::core::BOOL);
#[cfg(all(feature = "minwindef", feature = "winnt"))]
windows_link::link!("advapi32.dll" "system" fn AccessCheckByType(psecuritydescriptor : super::PSECURITY_DESCRIPTOR, principalselfsid : super::PSID, clienttoken : super::HANDLE, desiredaccess : u32, objecttypelist : super::POBJECT_TYPE_LIST, objecttypelistlength : u32, genericmapping : super::PGENERIC_MAPPING, privilegeset : super::PPRIVILEGE_SET, privilegesetlength : super::LPDWORD, grantedaccess : super::LPDWORD, accessstatus : super::LPBOOL) -> windows_sys::core::BOOL);
#[cfg(all(feature = "minwindef", feature = "winnt"))]
windows_link::link!("advapi32.dll" "system" fn AccessCheckByTypeAndAuditAlarmW(subsystemname : windows_sys::core::PCWSTR, handleid : *const core::ffi::c_void, objecttypename : windows_sys::core::PCWSTR, objectname : windows_sys::core::PCWSTR, securitydescriptor : super::PSECURITY_DESCRIPTOR, principalselfsid : super::PSID, desiredaccess : u32, audittype : super::AUDIT_EVENT_TYPE, flags : u32, objecttypelist : super::POBJECT_TYPE_LIST, objecttypelistlength : u32, genericmapping : super::PGENERIC_MAPPING, objectcreation : windows_sys::core::BOOL, grantedaccess : super::LPDWORD, accessstatus : super::LPBOOL, pfgenerateonclose : super::LPBOOL) -> windows_sys::core::BOOL);
#[cfg(all(feature = "minwindef", feature = "winnt"))]
windows_link::link!("advapi32.dll" "system" fn AccessCheckByTypeResultList(psecuritydescriptor : super::PSECURITY_DESCRIPTOR, principalselfsid : super::PSID, clienttoken : super::HANDLE, desiredaccess : u32, objecttypelist : super::POBJECT_TYPE_LIST, objecttypelistlength : u32, genericmapping : super::PGENERIC_MAPPING, privilegeset : super::PPRIVILEGE_SET, privilegesetlength : super::LPDWORD, grantedaccesslist : super::LPDWORD, accessstatuslist : super::LPDWORD) -> windows_sys::core::BOOL);
#[cfg(all(feature = "minwindef", feature = "winnt"))]
windows_link::link!("advapi32.dll" "system" fn AccessCheckByTypeResultListAndAuditAlarmByHandleW(subsystemname : windows_sys::core::PCWSTR, handleid : *const core::ffi::c_void, clienttoken : super::HANDLE, objecttypename : windows_sys::core::PCWSTR, objectname : windows_sys::core::PCWSTR, securitydescriptor : super::PSECURITY_DESCRIPTOR, principalselfsid : super::PSID, desiredaccess : u32, audittype : super::AUDIT_EVENT_TYPE, flags : u32, objecttypelist : super::POBJECT_TYPE_LIST, objecttypelistlength : u32, genericmapping : super::PGENERIC_MAPPING, objectcreation : windows_sys::core::BOOL, grantedaccesslist : super::LPDWORD, accessstatuslist : super::LPDWORD, pfgenerateonclose : super::LPBOOL) -> windows_sys::core::BOOL);
#[cfg(all(feature = "minwindef", feature = "winnt"))]
windows_link::link!("advapi32.dll" "system" fn AccessCheckByTypeResultListAndAuditAlarmW(subsystemname : windows_sys::core::PCWSTR, handleid : *const core::ffi::c_void, objecttypename : windows_sys::core::PCWSTR, objectname : windows_sys::core::PCWSTR, securitydescriptor : super::PSECURITY_DESCRIPTOR, principalselfsid : super::PSID, desiredaccess : u32, audittype : super::AUDIT_EVENT_TYPE, flags : u32, objecttypelist : super::POBJECT_TYPE_LIST, objecttypelistlength : u32, genericmapping : super::PGENERIC_MAPPING, objectcreation : windows_sys::core::BOOL, grantedaccesslist : super::LPDWORD, accessstatuslist : super::LPDWORD, pfgenerateonclose : super::LPBOOL) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("advapi32.dll" "system" fn AddAccessAllowedAce(pacl : super::PACL, dwacerevision : u32, accessmask : u32, psid : super::PSID) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("advapi32.dll" "system" fn AddAccessAllowedAceEx(pacl : super::PACL, dwacerevision : u32, aceflags : u32, accessmask : u32, psid : super::PSID) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("advapi32.dll" "system" fn AddAccessAllowedObjectAce(pacl : super::PACL, dwacerevision : u32, aceflags : u32, accessmask : u32, objecttypeguid : *const windows_sys::core::GUID, inheritedobjecttypeguid : *const windows_sys::core::GUID, psid : super::PSID) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("advapi32.dll" "system" fn AddAccessDeniedAce(pacl : super::PACL, dwacerevision : u32, accessmask : u32, psid : super::PSID) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("advapi32.dll" "system" fn AddAccessDeniedAceEx(pacl : super::PACL, dwacerevision : u32, aceflags : u32, accessmask : u32, psid : super::PSID) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("advapi32.dll" "system" fn AddAccessDeniedObjectAce(pacl : super::PACL, dwacerevision : u32, aceflags : u32, accessmask : u32, objecttypeguid : *const windows_sys::core::GUID, inheritedobjecttypeguid : *const windows_sys::core::GUID, psid : super::PSID) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("advapi32.dll" "system" fn AddAce(pacl : super::PACL, dwacerevision : u32, dwstartingaceindex : u32, pacelist : *const core::ffi::c_void, nacelistlength : u32) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("advapi32.dll" "system" fn AddAuditAccessAce(pacl : super::PACL, dwacerevision : u32, dwaccessmask : u32, psid : super::PSID, bauditsuccess : windows_sys::core::BOOL, bauditfailure : windows_sys::core::BOOL) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("advapi32.dll" "system" fn AddAuditAccessAceEx(pacl : super::PACL, dwacerevision : u32, aceflags : u32, dwaccessmask : u32, psid : super::PSID, bauditsuccess : windows_sys::core::BOOL, bauditfailure : windows_sys::core::BOOL) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("advapi32.dll" "system" fn AddAuditAccessObjectAce(pacl : super::PACL, dwacerevision : u32, aceflags : u32, accessmask : u32, objecttypeguid : *const windows_sys::core::GUID, inheritedobjecttypeguid : *const windows_sys::core::GUID, psid : super::PSID, bauditsuccess : windows_sys::core::BOOL, bauditfailure : windows_sys::core::BOOL) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("advapi32.dll" "system" fn AddMandatoryAce(pacl : super::PACL, dwacerevision : u32, aceflags : u32, mandatorypolicy : u32, plabelsid : super::PSID) -> windows_sys::core::BOOL);
#[cfg(all(feature = "basetsd", feature = "minwindef", feature = "winnt"))]
windows_link::link!("kernel32.dll" "system" fn AddResourceAttributeAce(pacl : super::PACL, dwacerevision : u32, aceflags : u32, accessmask : u32, psid : super::PSID, pattributeinfo : super::PCLAIM_SECURITY_ATTRIBUTES_INFORMATION, preturnlength : super::PDWORD) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("kernel32.dll" "system" fn AddScopedPolicyIDAce(pacl : super::PACL, dwacerevision : u32, aceflags : u32, accessmask : u32, psid : super::PSID) -> windows_sys::core::BOOL);
#[cfg(all(feature = "minwindef", feature = "winnt"))]
windows_link::link!("advapi32.dll" "system" fn AdjustTokenGroups(tokenhandle : super::HANDLE, resettodefault : windows_sys::core::BOOL, newstate : super::PTOKEN_GROUPS, bufferlength : u32, previousstate : super::PTOKEN_GROUPS, returnlength : super::PDWORD) -> windows_sys::core::BOOL);
#[cfg(all(feature = "minwindef", feature = "winnt"))]
windows_link::link!("advapi32.dll" "system" fn AdjustTokenPrivileges(tokenhandle : super::HANDLE, disableallprivileges : windows_sys::core::BOOL, newstate : super::PTOKEN_PRIVILEGES, bufferlength : u32, previousstate : super::PTOKEN_PRIVILEGES, returnlength : super::PDWORD) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("advapi32.dll" "system" fn AllocateAndInitializeSid(pidentifierauthority : super::PSID_IDENTIFIER_AUTHORITY, nsubauthoritycount : u8, nsubauthority0 : u32, nsubauthority1 : u32, nsubauthority2 : u32, nsubauthority3 : u32, nsubauthority4 : u32, nsubauthority5 : u32, nsubauthority6 : u32, nsubauthority7 : u32, psid : *mut super::PSID) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("advapi32.dll" "system" fn AllocateLocallyUniqueId(luid : super::PLUID) -> windows_sys::core::BOOL);
windows_link::link!("advapi32.dll" "system" fn AreAllAccessesGranted(grantedaccess : u32, desiredaccess : u32) -> windows_sys::core::BOOL);
windows_link::link!("advapi32.dll" "system" fn AreAnyAccessesGranted(grantedaccess : u32, desiredaccess : u32) -> windows_sys::core::BOOL);
#[cfg(all(feature = "minwindef", feature = "winnt"))]
windows_link::link!("kernel32.dll" "system" fn CheckTokenCapability(tokenhandle : super::HANDLE, capabilitysidtocheck : super::PSID, hascapability : super::PBOOL) -> windows_sys::core::BOOL);
#[cfg(all(feature = "minwindef", feature = "winnt"))]
windows_link::link!("advapi32.dll" "system" fn CheckTokenMembership(tokenhandle : super::HANDLE, sidtocheck : super::PSID, ismember : super::PBOOL) -> windows_sys::core::BOOL);
#[cfg(all(feature = "minwindef", feature = "winnt"))]
windows_link::link!("kernel32.dll" "system" fn CheckTokenMembershipEx(tokenhandle : super::HANDLE, sidtocheck : super::PSID, flags : u32, ismember : super::PBOOL) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("advapi32.dll" "system" fn ConvertToAutoInheritPrivateObjectSecurity(parentdescriptor : super::PSECURITY_DESCRIPTOR, currentsecuritydescriptor : super::PSECURITY_DESCRIPTOR, newsecuritydescriptor : *mut super::PSECURITY_DESCRIPTOR, objecttype : *const windows_sys::core::GUID, isdirectoryobject : super::BOOLEAN, genericmapping : super::PGENERIC_MAPPING) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("advapi32.dll" "system" fn CopySid(ndestinationsidlength : u32, pdestinationsid : super::PSID, psourcesid : super::PSID) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("advapi32.dll" "system" fn CreatePrivateObjectSecurity(parentdescriptor : super::PSECURITY_DESCRIPTOR, creatordescriptor : super::PSECURITY_DESCRIPTOR, newdescriptor : *mut super::PSECURITY_DESCRIPTOR, isdirectoryobject : windows_sys::core::BOOL, token : super::HANDLE, genericmapping : super::PGENERIC_MAPPING) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("advapi32.dll" "system" fn CreatePrivateObjectSecurityEx(parentdescriptor : super::PSECURITY_DESCRIPTOR, creatordescriptor : super::PSECURITY_DESCRIPTOR, newdescriptor : *mut super::PSECURITY_DESCRIPTOR, objecttype : *const windows_sys::core::GUID, iscontainerobject : windows_sys::core::BOOL, autoinheritflags : u32, token : super::HANDLE, genericmapping : super::PGENERIC_MAPPING) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("advapi32.dll" "system" fn CreatePrivateObjectSecurityWithMultipleInheritance(parentdescriptor : super::PSECURITY_DESCRIPTOR, creatordescriptor : super::PSECURITY_DESCRIPTOR, newdescriptor : *mut super::PSECURITY_DESCRIPTOR, objecttypes : *const *const windows_sys::core::GUID, guidcount : u32, iscontainerobject : windows_sys::core::BOOL, autoinheritflags : u32, token : super::HANDLE, genericmapping : super::PGENERIC_MAPPING) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("advapi32.dll" "system" fn CreateRestrictedToken(existingtokenhandle : super::HANDLE, flags : u32, disablesidcount : u32, sidstodisable : super::PSID_AND_ATTRIBUTES, deleteprivilegecount : u32, privilegestodelete : super::PLUID_AND_ATTRIBUTES, restrictedsidcount : u32, sidstorestrict : super::PSID_AND_ATTRIBUTES, newtokenhandle : super::PHANDLE) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("advapi32.dll" "system" fn CreateWellKnownSid(wellknownsidtype : super::WELL_KNOWN_SID_TYPE, domainsid : super::PSID, psid : super::PSID, cbsid : *mut u32) -> windows_sys::core::BOOL);
windows_link::link!("advapi32.dll" "system" fn CveEventWrite(cveid : windows_sys::core::PCWSTR, additionaldetails : windows_sys::core::PCWSTR) -> i32);
#[cfg(feature = "winnt")]
windows_link::link!("advapi32.dll" "system" fn DeleteAce(pacl : super::PACL, dwaceindex : u32) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("api-ms-win-security-base-l1-2-2.dll" "system" fn DeriveCapabilitySidsFromName(capname : windows_sys::core::PCWSTR, capabilitygroupsids : *mut *mut super::PSID, capabilitygroupsidcount : *mut u32, capabilitysids : *mut *mut super::PSID, capabilitysidcount : *mut u32) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("advapi32.dll" "system" fn DestroyPrivateObjectSecurity(objectdescriptor : *mut super::PSECURITY_DESCRIPTOR) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("advapi32.dll" "system" fn DuplicateToken(existingtokenhandle : super::HANDLE, impersonationlevel : super::SECURITY_IMPERSONATION_LEVEL, duplicatetokenhandle : super::PHANDLE) -> windows_sys::core::BOOL);
#[cfg(all(feature = "minwinbase", feature = "winnt"))]
windows_link::link!("advapi32.dll" "system" fn DuplicateTokenEx(hexistingtoken : super::HANDLE, dwdesiredaccess : u32, lptokenattributes : super::LPSECURITY_ATTRIBUTES, impersonationlevel : super::SECURITY_IMPERSONATION_LEVEL, tokentype : super::TOKEN_TYPE, phnewtoken : super::PHANDLE) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("advapi32.dll" "system" fn EqualDomainSid(psid1 : super::PSID, psid2 : super::PSID, pfequal : *mut windows_sys::core::BOOL) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("advapi32.dll" "system" fn EqualPrefixSid(psid1 : super::PSID, psid2 : super::PSID) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("advapi32.dll" "system" fn EqualSid(psid1 : super::PSID, psid2 : super::PSID) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("advapi32.dll" "system" fn FindFirstFreeAce(pacl : super::PACL, pace : *mut *mut core::ffi::c_void) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("advapi32.dll" "system" fn FreeSid(psid : super::PSID) -> *mut core::ffi::c_void);
#[cfg(feature = "winnt")]
windows_link::link!("advapi32.dll" "system" fn GetAce(pacl : super::PACL, dwaceindex : u32, pace : *mut *mut core::ffi::c_void) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("advapi32.dll" "system" fn GetAclInformation(pacl : super::PACL, paclinformation : *mut core::ffi::c_void, naclinformationlength : u32, dwaclinformationclass : super::ACL_INFORMATION_CLASS) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("kernel32.dll" "system" fn GetAppContainerAce(acl : super::PACL, startingaceindex : u32, appcontainerace : *mut *mut core::ffi::c_void, appcontaineraceindex : *mut u32) -> windows_sys::core::BOOL);
#[cfg(all(feature = "minwindef", feature = "winnt"))]
windows_link::link!("kernel32.dll" "system" fn GetCachedSigningLevel(file : super::HANDLE, flags : super::PULONG, signinglevel : super::PULONG, thumbprint : super::PUCHAR, thumbprintsize : super::PULONG, thumbprintalgorithm : super::PULONG) -> windows_sys::core::BOOL);
#[cfg(all(feature = "minwindef", feature = "winnt"))]
windows_link::link!("advapi32.dll" "system" fn GetFileSecurityW(lpfilename : windows_sys::core::PCWSTR, requestedinformation : super::SECURITY_INFORMATION, psecuritydescriptor : super::PSECURITY_DESCRIPTOR, nlength : u32, lpnlengthneeded : super::LPDWORD) -> windows_sys::core::BOOL);
#[cfg(all(feature = "minwindef", feature = "winnt"))]
windows_link::link!("advapi32.dll" "system" fn GetKernelObjectSecurity(handle : super::HANDLE, requestedinformation : super::SECURITY_INFORMATION, psecuritydescriptor : super::PSECURITY_DESCRIPTOR, nlength : u32, lpnlengthneeded : super::LPDWORD) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("advapi32.dll" "system" fn GetLengthSid(psid : super::PSID) -> u32);
#[cfg(all(feature = "minwindef", feature = "winnt"))]
windows_link::link!("advapi32.dll" "system" fn GetPrivateObjectSecurity(objectdescriptor : super::PSECURITY_DESCRIPTOR, securityinformation : super::SECURITY_INFORMATION, resultantdescriptor : super::PSECURITY_DESCRIPTOR, descriptorlength : u32, returnlength : super::PDWORD) -> windows_sys::core::BOOL);
#[cfg(all(feature = "minwindef", feature = "winnt"))]
windows_link::link!("advapi32.dll" "system" fn GetSecurityDescriptorControl(psecuritydescriptor : super::PSECURITY_DESCRIPTOR, pcontrol : super::PSECURITY_DESCRIPTOR_CONTROL, lpdwrevision : super::LPDWORD) -> windows_sys::core::BOOL);
#[cfg(all(feature = "minwindef", feature = "winnt"))]
windows_link::link!("advapi32.dll" "system" fn GetSecurityDescriptorDacl(psecuritydescriptor : super::PSECURITY_DESCRIPTOR, lpbdaclpresent : super::LPBOOL, pdacl : *mut super::PACL, lpbdacldefaulted : super::LPBOOL) -> windows_sys::core::BOOL);
#[cfg(all(feature = "minwindef", feature = "winnt"))]
windows_link::link!("advapi32.dll" "system" fn GetSecurityDescriptorGroup(psecuritydescriptor : super::PSECURITY_DESCRIPTOR, pgroup : *mut super::PSID, lpbgroupdefaulted : super::LPBOOL) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("advapi32.dll" "system" fn GetSecurityDescriptorLength(psecuritydescriptor : super::PSECURITY_DESCRIPTOR) -> u32);
#[cfg(all(feature = "minwindef", feature = "winnt"))]
windows_link::link!("advapi32.dll" "system" fn GetSecurityDescriptorOwner(psecuritydescriptor : super::PSECURITY_DESCRIPTOR, powner : *mut super::PSID, lpbownerdefaulted : super::LPBOOL) -> windows_sys::core::BOOL);
#[cfg(all(feature = "minwindef", feature = "winnt"))]
windows_link::link!("advapi32.dll" "system" fn GetSecurityDescriptorRMControl(securitydescriptor : super::PSECURITY_DESCRIPTOR, rmcontrol : super::PUCHAR) -> u32);
#[cfg(all(feature = "minwindef", feature = "winnt"))]
windows_link::link!("advapi32.dll" "system" fn GetSecurityDescriptorSacl(psecuritydescriptor : super::PSECURITY_DESCRIPTOR, lpbsaclpresent : super::LPBOOL, psacl : *mut super::PACL, lpbsacldefaulted : super::LPBOOL) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("advapi32.dll" "system" fn GetSidIdentifierAuthority(psid : super::PSID) -> super::PSID_IDENTIFIER_AUTHORITY);
windows_link::link!("advapi32.dll" "system" fn GetSidLengthRequired(nsubauthoritycount : u8) -> u32);
#[cfg(all(feature = "minwindef", feature = "winnt"))]
windows_link::link!("advapi32.dll" "system" fn GetSidSubAuthority(psid : super::PSID, nsubauthority : u32) -> super::PDWORD);
#[cfg(all(feature = "minwindef", feature = "winnt"))]
windows_link::link!("advapi32.dll" "system" fn GetSidSubAuthorityCount(psid : super::PSID) -> super::PUCHAR);
#[cfg(all(feature = "minwindef", feature = "winnt"))]
windows_link::link!("advapi32.dll" "system" fn GetTokenInformation(tokenhandle : super::HANDLE, tokeninformationclass : super::TOKEN_INFORMATION_CLASS, tokeninformation : *mut core::ffi::c_void, tokeninformationlength : u32, returnlength : super::PDWORD) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("advapi32.dll" "system" fn GetWindowsAccountDomainSid(psid : super::PSID, pdomainsid : super::PSID, cbdomainsid : *mut u32) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("advapi32.dll" "system" fn ImpersonateAnonymousToken(threadhandle : super::HANDLE) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("advapi32.dll" "system" fn ImpersonateLoggedOnUser(htoken : super::HANDLE) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("advapi32.dll" "system" fn ImpersonateSelf(impersonationlevel : super::SECURITY_IMPERSONATION_LEVEL) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("advapi32.dll" "system" fn InitializeAcl(pacl : super::PACL, nacllength : u32, dwaclrevision : u32) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("advapi32.dll" "system" fn InitializeSecurityDescriptor(psecuritydescriptor : super::PSECURITY_DESCRIPTOR, dwrevision : u32) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("advapi32.dll" "system" fn InitializeSid(sid : super::PSID, pidentifierauthority : super::PSID_IDENTIFIER_AUTHORITY, nsubauthoritycount : u8) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("advapi32.dll" "system" fn IsTokenRestricted(tokenhandle : super::HANDLE) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("advapi32.dll" "system" fn IsValidAcl(pacl : super::PACL) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("advapi32.dll" "system" fn IsValidSecurityDescriptor(psecuritydescriptor : super::PSECURITY_DESCRIPTOR) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("advapi32.dll" "system" fn IsValidSid(psid : super::PSID) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("advapi32.dll" "system" fn IsWellKnownSid(psid : super::PSID, wellknownsidtype : super::WELL_KNOWN_SID_TYPE) -> windows_sys::core::BOOL);
#[cfg(all(feature = "minwindef", feature = "winnt"))]
windows_link::link!("advapi32.dll" "system" fn MakeAbsoluteSD(pselfrelativesecuritydescriptor : super::PSECURITY_DESCRIPTOR, pabsolutesecuritydescriptor : super::PSECURITY_DESCRIPTOR, lpdwabsolutesecuritydescriptorsize : super::LPDWORD, pdacl : super::PACL, lpdwdaclsize : super::LPDWORD, psacl : super::PACL, lpdwsaclsize : super::LPDWORD, powner : super::PSID, lpdwownersize : super::LPDWORD, pprimarygroup : super::PSID, lpdwprimarygroupsize : super::LPDWORD) -> windows_sys::core::BOOL);
#[cfg(all(feature = "minwindef", feature = "winnt"))]
windows_link::link!("advapi32.dll" "system" fn MakeSelfRelativeSD(pabsolutesecuritydescriptor : super::PSECURITY_DESCRIPTOR, pselfrelativesecuritydescriptor : super::PSECURITY_DESCRIPTOR, lpdwbufferlength : super::LPDWORD) -> windows_sys::core::BOOL);
#[cfg(all(feature = "minwindef", feature = "winnt"))]
windows_link::link!("advapi32.dll" "system" fn MapGenericMask(accessmask : super::PDWORD, genericmapping : super::PGENERIC_MAPPING));
windows_link::link!("advapi32.dll" "system" fn ObjectCloseAuditAlarmW(subsystemname : windows_sys::core::PCWSTR, handleid : *const core::ffi::c_void, generateonclose : windows_sys::core::BOOL) -> windows_sys::core::BOOL);
windows_link::link!("advapi32.dll" "system" fn ObjectDeleteAuditAlarmW(subsystemname : windows_sys::core::PCWSTR, handleid : *const core::ffi::c_void, generateonclose : windows_sys::core::BOOL) -> windows_sys::core::BOOL);
#[cfg(all(feature = "minwindef", feature = "winnt"))]
windows_link::link!("advapi32.dll" "system" fn ObjectOpenAuditAlarmW(subsystemname : windows_sys::core::PCWSTR, handleid : *const core::ffi::c_void, objecttypename : windows_sys::core::PCWSTR, objectname : windows_sys::core::PCWSTR, psecuritydescriptor : super::PSECURITY_DESCRIPTOR, clienttoken : super::HANDLE, desiredaccess : u32, grantedaccess : u32, privileges : super::PPRIVILEGE_SET, objectcreation : windows_sys::core::BOOL, accessgranted : windows_sys::core::BOOL, generateonclose : super::LPBOOL) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("advapi32.dll" "system" fn ObjectPrivilegeAuditAlarmW(subsystemname : windows_sys::core::PCWSTR, handleid : *const core::ffi::c_void, clienttoken : super::HANDLE, desiredaccess : u32, privileges : super::PPRIVILEGE_SET, accessgranted : windows_sys::core::BOOL) -> windows_sys::core::BOOL);
#[cfg(all(feature = "minwindef", feature = "winnt"))]
windows_link::link!("advapi32.dll" "system" fn PrivilegeCheck(clienttoken : super::HANDLE, requiredprivileges : super::PPRIVILEGE_SET, pfresult : super::LPBOOL) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("advapi32.dll" "system" fn PrivilegedServiceAuditAlarmW(subsystemname : windows_sys::core::PCWSTR, servicename : windows_sys::core::PCWSTR, clienttoken : super::HANDLE, privileges : super::PPRIVILEGE_SET, accessgranted : windows_sys::core::BOOL) -> windows_sys::core::BOOL);
#[cfg(all(feature = "minwindef", feature = "winnt"))]
windows_link::link!("advapi32.dll" "system" fn QuerySecurityAccessMask(securityinformation : super::SECURITY_INFORMATION, desiredaccess : super::LPDWORD));
windows_link::link!("advapi32.dll" "system" fn RevertToSelf() -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("advapi32.dll" "system" fn SetAclInformation(pacl : super::PACL, paclinformation : *const core::ffi::c_void, naclinformationlength : u32, dwaclinformationclass : super::ACL_INFORMATION_CLASS) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("kernel32.dll" "system" fn SetCachedSigningLevel(sourcefiles : super::PHANDLE, sourcefilecount : u32, flags : u32, targetfile : super::HANDLE) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("advapi32.dll" "system" fn SetFileSecurityW(lpfilename : windows_sys::core::PCWSTR, securityinformation : super::SECURITY_INFORMATION, psecuritydescriptor : super::PSECURITY_DESCRIPTOR) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("advapi32.dll" "system" fn SetKernelObjectSecurity(handle : super::HANDLE, securityinformation : super::SECURITY_INFORMATION, securitydescriptor : super::PSECURITY_DESCRIPTOR) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("advapi32.dll" "system" fn SetPrivateObjectSecurity(securityinformation : super::SECURITY_INFORMATION, modificationdescriptor : super::PSECURITY_DESCRIPTOR, objectssecuritydescriptor : *mut super::PSECURITY_DESCRIPTOR, genericmapping : super::PGENERIC_MAPPING, token : super::HANDLE) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("advapi32.dll" "system" fn SetPrivateObjectSecurityEx(securityinformation : super::SECURITY_INFORMATION, modificationdescriptor : super::PSECURITY_DESCRIPTOR, objectssecuritydescriptor : *mut super::PSECURITY_DESCRIPTOR, autoinheritflags : u32, genericmapping : super::PGENERIC_MAPPING, token : super::HANDLE) -> windows_sys::core::BOOL);
#[cfg(all(feature = "minwindef", feature = "winnt"))]
windows_link::link!("advapi32.dll" "system" fn SetSecurityAccessMask(securityinformation : super::SECURITY_INFORMATION, desiredaccess : super::LPDWORD));
#[cfg(feature = "winnt")]
windows_link::link!("advapi32.dll" "system" fn SetSecurityDescriptorControl(psecuritydescriptor : super::PSECURITY_DESCRIPTOR, controlbitsofinterest : super::SECURITY_DESCRIPTOR_CONTROL, controlbitstoset : super::SECURITY_DESCRIPTOR_CONTROL) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("advapi32.dll" "system" fn SetSecurityDescriptorDacl(psecuritydescriptor : super::PSECURITY_DESCRIPTOR, bdaclpresent : windows_sys::core::BOOL, pdacl : super::PACL, bdacldefaulted : windows_sys::core::BOOL) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("advapi32.dll" "system" fn SetSecurityDescriptorGroup(psecuritydescriptor : super::PSECURITY_DESCRIPTOR, pgroup : super::PSID, bgroupdefaulted : windows_sys::core::BOOL) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("advapi32.dll" "system" fn SetSecurityDescriptorOwner(psecuritydescriptor : super::PSECURITY_DESCRIPTOR, powner : super::PSID, bownerdefaulted : windows_sys::core::BOOL) -> windows_sys::core::BOOL);
#[cfg(all(feature = "minwindef", feature = "winnt"))]
windows_link::link!("advapi32.dll" "system" fn SetSecurityDescriptorRMControl(securitydescriptor : super::PSECURITY_DESCRIPTOR, rmcontrol : super::PUCHAR) -> u32);
#[cfg(feature = "winnt")]
windows_link::link!("advapi32.dll" "system" fn SetSecurityDescriptorSacl(psecuritydescriptor : super::PSECURITY_DESCRIPTOR, bsaclpresent : windows_sys::core::BOOL, psacl : super::PACL, bsacldefaulted : windows_sys::core::BOOL) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("advapi32.dll" "system" fn SetTokenInformation(tokenhandle : super::HANDLE, tokeninformationclass : super::TOKEN_INFORMATION_CLASS, tokeninformation : *const core::ffi::c_void, tokeninformationlength : u32) -> windows_sys::core::BOOL);
pub const SIGNING_LEVEL_FILE_CACHE_FLAG_NOT_VALIDATED: i32 = 1;
pub const SIGNING_LEVEL_FILE_CACHE_FLAG_VALIDATE_ONLY: i32 = 4;
pub const SIGNING_LEVEL_MICROSOFT: i32 = 8;
