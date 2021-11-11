#![allow(non_snake_case, non_camel_case_types)]
#[cfg(feature = "Win32_Security_AppLocker")]
pub mod AppLocker;
#[cfg(feature = "Win32_Security_Authentication")]
pub mod Authentication;
#[cfg(feature = "Win32_Security_Authorization")]
pub mod Authorization;
#[cfg(feature = "Win32_Security_ConfigurationSnapin")]
pub mod ConfigurationSnapin;
#[cfg(feature = "Win32_Security_Credentials")]
pub mod Credentials;
#[cfg(feature = "Win32_Security_Cryptography")]
pub mod Cryptography;
#[cfg(feature = "Win32_Security_DiagnosticDataQuery")]
pub mod DiagnosticDataQuery;
#[cfg(feature = "Win32_Security_DirectoryServices")]
pub mod DirectoryServices;
#[cfg(feature = "Win32_Security_EnterpriseData")]
pub mod EnterpriseData;
#[cfg(feature = "Win32_Security_ExtensibleAuthenticationProtocol")]
pub mod ExtensibleAuthenticationProtocol;
#[cfg(feature = "Win32_Security_Isolation")]
pub mod Isolation;
#[cfg(feature = "Win32_Security_LicenseProtection")]
pub mod LicenseProtection;
#[cfg(feature = "Win32_Security_NetworkAccessProtection")]
pub mod NetworkAccessProtection;
#[cfg(feature = "Win32_Security_Tpm")]
pub mod Tpm;
#[cfg(feature = "Win32_Security_WinTrust")]
pub mod WinTrust;
#[cfg(feature = "Win32_Security_WinWlx")]
pub mod WinWlx;
#[link(name = "windows")]
extern "system" {
    #[doc = "*Required features: `Win32_Security`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn AccessCheck(psecuritydescriptor: *const SECURITY_DESCRIPTOR, clienttoken: super::Foundation::HANDLE, desiredaccess: u32, genericmapping: *const GENERIC_MAPPING, privilegeset: *mut PRIVILEGE_SET, privilegesetlength: *mut u32, grantedaccess: *mut u32, accessstatus: *mut i32) -> super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Security`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn AccessCheckAndAuditAlarmA(subsystemname: super::Foundation::PSTR, handleid: *const ::core::ffi::c_void, objecttypename: super::Foundation::PSTR, objectname: super::Foundation::PSTR, securitydescriptor: *const SECURITY_DESCRIPTOR, desiredaccess: u32, genericmapping: *const GENERIC_MAPPING, objectcreation: super::Foundation::BOOL, grantedaccess: *mut u32, accessstatus: *mut i32, pfgenerateonclose: *mut i32) -> super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Security`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn AccessCheckAndAuditAlarmW(subsystemname: super::Foundation::PWSTR, handleid: *const ::core::ffi::c_void, objecttypename: super::Foundation::PWSTR, objectname: super::Foundation::PWSTR, securitydescriptor: *const SECURITY_DESCRIPTOR, desiredaccess: u32, genericmapping: *const GENERIC_MAPPING, objectcreation: super::Foundation::BOOL, grantedaccess: *mut u32, accessstatus: *mut i32, pfgenerateonclose: *mut i32) -> super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Security`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn AccessCheckByType(psecuritydescriptor: *const SECURITY_DESCRIPTOR, principalselfsid: super::Foundation::PSID, clienttoken: super::Foundation::HANDLE, desiredaccess: u32, objecttypelist: *mut OBJECT_TYPE_LIST, objecttypelistlength: u32, genericmapping: *const GENERIC_MAPPING, privilegeset: *mut PRIVILEGE_SET, privilegesetlength: *mut u32, grantedaccess: *mut u32, accessstatus: *mut i32) -> super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Security`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn AccessCheckByTypeAndAuditAlarmA(
        subsystemname: super::Foundation::PSTR,
        handleid: *const ::core::ffi::c_void,
        objecttypename: super::Foundation::PSTR,
        objectname: super::Foundation::PSTR,
        securitydescriptor: *const SECURITY_DESCRIPTOR,
        principalselfsid: super::Foundation::PSID,
        desiredaccess: u32,
        audittype: AUDIT_EVENT_TYPE,
        flags: u32,
        objecttypelist: *mut OBJECT_TYPE_LIST,
        objecttypelistlength: u32,
        genericmapping: *const GENERIC_MAPPING,
        objectcreation: super::Foundation::BOOL,
        grantedaccess: *mut u32,
        accessstatus: *mut i32,
        pfgenerateonclose: *mut i32,
    ) -> super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Security`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn AccessCheckByTypeAndAuditAlarmW(
        subsystemname: super::Foundation::PWSTR,
        handleid: *const ::core::ffi::c_void,
        objecttypename: super::Foundation::PWSTR,
        objectname: super::Foundation::PWSTR,
        securitydescriptor: *const SECURITY_DESCRIPTOR,
        principalselfsid: super::Foundation::PSID,
        desiredaccess: u32,
        audittype: AUDIT_EVENT_TYPE,
        flags: u32,
        objecttypelist: *mut OBJECT_TYPE_LIST,
        objecttypelistlength: u32,
        genericmapping: *const GENERIC_MAPPING,
        objectcreation: super::Foundation::BOOL,
        grantedaccess: *mut u32,
        accessstatus: *mut i32,
        pfgenerateonclose: *mut i32,
    ) -> super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Security`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn AccessCheckByTypeResultList(psecuritydescriptor: *const SECURITY_DESCRIPTOR, principalselfsid: super::Foundation::PSID, clienttoken: super::Foundation::HANDLE, desiredaccess: u32, objecttypelist: *mut OBJECT_TYPE_LIST, objecttypelistlength: u32, genericmapping: *const GENERIC_MAPPING, privilegeset: *mut PRIVILEGE_SET, privilegesetlength: *mut u32, grantedaccesslist: *mut u32, accessstatuslist: *mut u32) -> super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Security`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn AccessCheckByTypeResultListAndAuditAlarmA(
        subsystemname: super::Foundation::PSTR,
        handleid: *const ::core::ffi::c_void,
        objecttypename: super::Foundation::PSTR,
        objectname: super::Foundation::PSTR,
        securitydescriptor: *const SECURITY_DESCRIPTOR,
        principalselfsid: super::Foundation::PSID,
        desiredaccess: u32,
        audittype: AUDIT_EVENT_TYPE,
        flags: u32,
        objecttypelist: *mut OBJECT_TYPE_LIST,
        objecttypelistlength: u32,
        genericmapping: *const GENERIC_MAPPING,
        objectcreation: super::Foundation::BOOL,
        grantedaccess: *mut u32,
        accessstatuslist: *mut u32,
        pfgenerateonclose: *mut i32,
    ) -> super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Security`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn AccessCheckByTypeResultListAndAuditAlarmByHandleA(
        subsystemname: super::Foundation::PSTR,
        handleid: *const ::core::ffi::c_void,
        clienttoken: super::Foundation::HANDLE,
        objecttypename: super::Foundation::PSTR,
        objectname: super::Foundation::PSTR,
        securitydescriptor: *const SECURITY_DESCRIPTOR,
        principalselfsid: super::Foundation::PSID,
        desiredaccess: u32,
        audittype: AUDIT_EVENT_TYPE,
        flags: u32,
        objecttypelist: *mut OBJECT_TYPE_LIST,
        objecttypelistlength: u32,
        genericmapping: *const GENERIC_MAPPING,
        objectcreation: super::Foundation::BOOL,
        grantedaccess: *mut u32,
        accessstatuslist: *mut u32,
        pfgenerateonclose: *mut i32,
    ) -> super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Security`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn AccessCheckByTypeResultListAndAuditAlarmByHandleW(
        subsystemname: super::Foundation::PWSTR,
        handleid: *const ::core::ffi::c_void,
        clienttoken: super::Foundation::HANDLE,
        objecttypename: super::Foundation::PWSTR,
        objectname: super::Foundation::PWSTR,
        securitydescriptor: *const SECURITY_DESCRIPTOR,
        principalselfsid: super::Foundation::PSID,
        desiredaccess: u32,
        audittype: AUDIT_EVENT_TYPE,
        flags: u32,
        objecttypelist: *mut OBJECT_TYPE_LIST,
        objecttypelistlength: u32,
        genericmapping: *const GENERIC_MAPPING,
        objectcreation: super::Foundation::BOOL,
        grantedaccesslist: *mut u32,
        accessstatuslist: *mut u32,
        pfgenerateonclose: *mut i32,
    ) -> super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Security`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn AccessCheckByTypeResultListAndAuditAlarmW(
        subsystemname: super::Foundation::PWSTR,
        handleid: *const ::core::ffi::c_void,
        objecttypename: super::Foundation::PWSTR,
        objectname: super::Foundation::PWSTR,
        securitydescriptor: *const SECURITY_DESCRIPTOR,
        principalselfsid: super::Foundation::PSID,
        desiredaccess: u32,
        audittype: AUDIT_EVENT_TYPE,
        flags: u32,
        objecttypelist: *mut OBJECT_TYPE_LIST,
        objecttypelistlength: u32,
        genericmapping: *const GENERIC_MAPPING,
        objectcreation: super::Foundation::BOOL,
        grantedaccesslist: *mut u32,
        accessstatuslist: *mut u32,
        pfgenerateonclose: *mut i32,
    ) -> super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Security`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn AddAccessAllowedAce(pacl: *mut ACL, dwacerevision: u32, accessmask: u32, psid: super::Foundation::PSID) -> super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Security`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn AddAccessAllowedAceEx(pacl: *mut ACL, dwacerevision: u32, aceflags: ACE_FLAGS, accessmask: u32, psid: super::Foundation::PSID) -> super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Security`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn AddAccessAllowedObjectAce(pacl: *mut ACL, dwacerevision: u32, aceflags: ACE_FLAGS, accessmask: u32, objecttypeguid: *const ::windows::runtime::GUID, inheritedobjecttypeguid: *const ::windows::runtime::GUID, psid: super::Foundation::PSID) -> super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Security`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn AddAccessDeniedAce(pacl: *mut ACL, dwacerevision: u32, accessmask: u32, psid: super::Foundation::PSID) -> super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Security`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn AddAccessDeniedAceEx(pacl: *mut ACL, dwacerevision: u32, aceflags: ACE_FLAGS, accessmask: u32, psid: super::Foundation::PSID) -> super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Security`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn AddAccessDeniedObjectAce(pacl: *mut ACL, dwacerevision: u32, aceflags: ACE_FLAGS, accessmask: u32, objecttypeguid: *const ::windows::runtime::GUID, inheritedobjecttypeguid: *const ::windows::runtime::GUID, psid: super::Foundation::PSID) -> super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Security`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn AddAce(pacl: *mut ACL, dwacerevision: u32, dwstartingaceindex: u32, pacelist: *const ::core::ffi::c_void, nacelistlength: u32) -> super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Security`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn AddAuditAccessAce(pacl: *mut ACL, dwacerevision: u32, dwaccessmask: u32, psid: super::Foundation::PSID, bauditsuccess: super::Foundation::BOOL, bauditfailure: super::Foundation::BOOL) -> super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Security`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn AddAuditAccessAceEx(pacl: *mut ACL, dwacerevision: u32, aceflags: ACE_FLAGS, dwaccessmask: u32, psid: super::Foundation::PSID, bauditsuccess: super::Foundation::BOOL, bauditfailure: super::Foundation::BOOL) -> super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Security`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn AddAuditAccessObjectAce(pacl: *mut ACL, dwacerevision: u32, aceflags: ACE_FLAGS, accessmask: u32, objecttypeguid: *const ::windows::runtime::GUID, inheritedobjecttypeguid: *const ::windows::runtime::GUID, psid: super::Foundation::PSID, bauditsuccess: super::Foundation::BOOL, bauditfailure: super::Foundation::BOOL) -> super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Security`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn AddConditionalAce(pacl: *mut ACL, dwacerevision: u32, aceflags: ACE_FLAGS, acetype: u8, accessmask: u32, psid: super::Foundation::PSID, conditionstr: super::Foundation::PWSTR, returnlength: *mut u32) -> super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Security`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn AddMandatoryAce(pacl: *mut ACL, dwacerevision: ACE_REVISION, aceflags: ACE_FLAGS, mandatorypolicy: u32, plabelsid: super::Foundation::PSID) -> super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Security`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn AddResourceAttributeAce(pacl: *mut ACL, dwacerevision: u32, aceflags: ACE_FLAGS, accessmask: u32, psid: super::Foundation::PSID, pattributeinfo: *const CLAIM_SECURITY_ATTRIBUTES_INFORMATION, preturnlength: *mut u32) -> super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Security`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn AddScopedPolicyIDAce(pacl: *mut ACL, dwacerevision: u32, aceflags: ACE_FLAGS, accessmask: u32, psid: super::Foundation::PSID) -> super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Security`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn AdjustTokenGroups(tokenhandle: super::Foundation::HANDLE, resettodefault: super::Foundation::BOOL, newstate: *const TOKEN_GROUPS, bufferlength: u32, previousstate: *mut TOKEN_GROUPS, returnlength: *mut u32) -> super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Security`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn AdjustTokenPrivileges(tokenhandle: super::Foundation::HANDLE, disableallprivileges: super::Foundation::BOOL, newstate: *const TOKEN_PRIVILEGES, bufferlength: u32, previousstate: *mut TOKEN_PRIVILEGES, returnlength: *mut u32) -> super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Security`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn AllocateAndInitializeSid(pidentifierauthority: *const SID_IDENTIFIER_AUTHORITY, nsubauthoritycount: u8, nsubauthority0: u32, nsubauthority1: u32, nsubauthority2: u32, nsubauthority3: u32, nsubauthority4: u32, nsubauthority5: u32, nsubauthority6: u32, nsubauthority7: u32, psid: *mut super::Foundation::PSID) -> super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Security`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn AllocateLocallyUniqueId(luid: *mut super::Foundation::LUID) -> super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Security`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn AreAllAccessesGranted(grantedaccess: u32, desiredaccess: u32) -> super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Security`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn AreAnyAccessesGranted(grantedaccess: u32, desiredaccess: u32) -> super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Security`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CheckTokenCapability(tokenhandle: super::Foundation::HANDLE, capabilitysidtocheck: super::Foundation::PSID, hascapability: *mut super::Foundation::BOOL) -> super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Security`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CheckTokenMembership(tokenhandle: super::Foundation::HANDLE, sidtocheck: super::Foundation::PSID, ismember: *mut super::Foundation::BOOL) -> super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Security`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CheckTokenMembershipEx(tokenhandle: super::Foundation::HANDLE, sidtocheck: super::Foundation::PSID, flags: u32, ismember: *mut super::Foundation::BOOL) -> super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Security`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ConvertToAutoInheritPrivateObjectSecurity(parentdescriptor: *const SECURITY_DESCRIPTOR, currentsecuritydescriptor: *const SECURITY_DESCRIPTOR, newsecuritydescriptor: *mut *mut SECURITY_DESCRIPTOR, objecttype: *const ::windows::runtime::GUID, isdirectoryobject: super::Foundation::BOOLEAN, genericmapping: *const GENERIC_MAPPING) -> super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Security`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CopySid(ndestinationsidlength: u32, pdestinationsid: super::Foundation::PSID, psourcesid: super::Foundation::PSID) -> super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Security`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CreatePrivateObjectSecurity(parentdescriptor: *const SECURITY_DESCRIPTOR, creatordescriptor: *const SECURITY_DESCRIPTOR, newdescriptor: *mut *mut SECURITY_DESCRIPTOR, isdirectoryobject: super::Foundation::BOOL, token: super::Foundation::HANDLE, genericmapping: *const GENERIC_MAPPING) -> super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Security`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CreatePrivateObjectSecurityEx(parentdescriptor: *const SECURITY_DESCRIPTOR, creatordescriptor: *const SECURITY_DESCRIPTOR, newdescriptor: *mut *mut SECURITY_DESCRIPTOR, objecttype: *const ::windows::runtime::GUID, iscontainerobject: super::Foundation::BOOL, autoinheritflags: SECURITY_AUTO_INHERIT_FLAGS, token: super::Foundation::HANDLE, genericmapping: *const GENERIC_MAPPING) -> super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Security`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CreatePrivateObjectSecurityWithMultipleInheritance(parentdescriptor: *const SECURITY_DESCRIPTOR, creatordescriptor: *const SECURITY_DESCRIPTOR, newdescriptor: *mut *mut SECURITY_DESCRIPTOR, objecttypes: *const *const ::windows::runtime::GUID, guidcount: u32, iscontainerobject: super::Foundation::BOOL, autoinheritflags: SECURITY_AUTO_INHERIT_FLAGS, token: super::Foundation::HANDLE, genericmapping: *const GENERIC_MAPPING) -> super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Security`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CreateRestrictedToken(existingtokenhandle: super::Foundation::HANDLE, flags: CREATE_RESTRICTED_TOKEN_FLAGS, disablesidcount: u32, sidstodisable: *const SID_AND_ATTRIBUTES, deleteprivilegecount: u32, privilegestodelete: *const LUID_AND_ATTRIBUTES, restrictedsidcount: u32, sidstorestrict: *const SID_AND_ATTRIBUTES, newtokenhandle: *mut super::Foundation::HANDLE) -> super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Security`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CreateWellKnownSid(wellknownsidtype: WELL_KNOWN_SID_TYPE, domainsid: super::Foundation::PSID, psid: super::Foundation::PSID, cbsid: *mut u32) -> super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Security`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DeleteAce(pacl: *mut ACL, dwaceindex: u32) -> super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Security`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DeriveCapabilitySidsFromName(capname: super::Foundation::PWSTR, capabilitygroupsids: *mut *mut super::Foundation::PSID, capabilitygroupsidcount: *mut u32, capabilitysids: *mut *mut super::Foundation::PSID, capabilitysidcount: *mut u32) -> super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Security`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DestroyPrivateObjectSecurity(objectdescriptor: *const *const SECURITY_DESCRIPTOR) -> super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Security`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DuplicateToken(existingtokenhandle: super::Foundation::HANDLE, impersonationlevel: SECURITY_IMPERSONATION_LEVEL, duplicatetokenhandle: *mut super::Foundation::HANDLE) -> super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Security`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DuplicateTokenEx(hexistingtoken: super::Foundation::HANDLE, dwdesiredaccess: TOKEN_ACCESS_MASK, lptokenattributes: *const SECURITY_ATTRIBUTES, impersonationlevel: SECURITY_IMPERSONATION_LEVEL, tokentype: TOKEN_TYPE, phnewtoken: *mut super::Foundation::HANDLE) -> super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Security`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn EqualDomainSid(psid1: super::Foundation::PSID, psid2: super::Foundation::PSID, pfequal: *mut super::Foundation::BOOL) -> super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Security`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn EqualPrefixSid(psid1: super::Foundation::PSID, psid2: super::Foundation::PSID) -> super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Security`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn EqualSid(psid1: super::Foundation::PSID, psid2: super::Foundation::PSID) -> super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Security`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FindFirstFreeAce(pacl: *const ACL, pace: *mut *mut ::core::ffi::c_void) -> super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Security`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FreeSid(psid: super::Foundation::PSID) -> *mut ::core::ffi::c_void;
    #[doc = "*Required features: `Win32_Security`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetAce(pacl: *const ACL, dwaceindex: u32, pace: *mut *mut ::core::ffi::c_void) -> super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Security`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetAclInformation(pacl: *const ACL, paclinformation: *mut ::core::ffi::c_void, naclinformationlength: u32, dwaclinformationclass: ACL_INFORMATION_CLASS) -> super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Security`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetAppContainerAce(acl: *const ACL, startingaceindex: u32, appcontainerace: *mut *mut ::core::ffi::c_void, appcontaineraceindex: *mut u32) -> super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Security`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetCachedSigningLevel(file: super::Foundation::HANDLE, flags: *mut u32, signinglevel: *mut u32, thumbprint: *mut u8, thumbprintsize: *mut u32, thumbprintalgorithm: *mut u32) -> super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Security`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetFileSecurityA(lpfilename: super::Foundation::PSTR, requestedinformation: u32, psecuritydescriptor: *mut SECURITY_DESCRIPTOR, nlength: u32, lpnlengthneeded: *mut u32) -> super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Security`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetFileSecurityW(lpfilename: super::Foundation::PWSTR, requestedinformation: u32, psecuritydescriptor: *mut SECURITY_DESCRIPTOR, nlength: u32, lpnlengthneeded: *mut u32) -> super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Security`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetKernelObjectSecurity(handle: super::Foundation::HANDLE, requestedinformation: u32, psecuritydescriptor: *mut SECURITY_DESCRIPTOR, nlength: u32, lpnlengthneeded: *mut u32) -> super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Security`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetLengthSid(psid: super::Foundation::PSID) -> u32;
    #[doc = "*Required features: `Win32_Security`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetPrivateObjectSecurity(objectdescriptor: *const SECURITY_DESCRIPTOR, securityinformation: u32, resultantdescriptor: *mut SECURITY_DESCRIPTOR, descriptorlength: u32, returnlength: *mut u32) -> super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Security`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetSecurityDescriptorControl(psecuritydescriptor: *const SECURITY_DESCRIPTOR, pcontrol: *mut u16, lpdwrevision: *mut u32) -> super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Security`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetSecurityDescriptorDacl(psecuritydescriptor: *const SECURITY_DESCRIPTOR, lpbdaclpresent: *mut i32, pdacl: *mut *mut ACL, lpbdacldefaulted: *mut i32) -> super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Security`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetSecurityDescriptorGroup(psecuritydescriptor: *const SECURITY_DESCRIPTOR, pgroup: *mut super::Foundation::PSID, lpbgroupdefaulted: *mut i32) -> super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Security`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetSecurityDescriptorLength(psecuritydescriptor: *const SECURITY_DESCRIPTOR) -> u32;
    #[doc = "*Required features: `Win32_Security`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetSecurityDescriptorOwner(psecuritydescriptor: *const SECURITY_DESCRIPTOR, powner: *mut super::Foundation::PSID, lpbownerdefaulted: *mut i32) -> super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Security`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetSecurityDescriptorRMControl(securitydescriptor: *const SECURITY_DESCRIPTOR, rmcontrol: *mut u8) -> u32;
    #[doc = "*Required features: `Win32_Security`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetSecurityDescriptorSacl(psecuritydescriptor: *const SECURITY_DESCRIPTOR, lpbsaclpresent: *mut i32, psacl: *mut *mut ACL, lpbsacldefaulted: *mut i32) -> super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Security`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetSidIdentifierAuthority(psid: super::Foundation::PSID) -> *mut SID_IDENTIFIER_AUTHORITY;
    #[doc = "*Required features: `Win32_Security`*"]
    pub fn GetSidLengthRequired(nsubauthoritycount: u8) -> u32;
    #[doc = "*Required features: `Win32_Security`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetSidSubAuthority(psid: super::Foundation::PSID, nsubauthority: u32) -> *mut u32;
    #[doc = "*Required features: `Win32_Security`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetSidSubAuthorityCount(psid: super::Foundation::PSID) -> *mut u8;
    #[doc = "*Required features: `Win32_Security`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetTokenInformation(tokenhandle: super::Foundation::HANDLE, tokeninformationclass: TOKEN_INFORMATION_CLASS, tokeninformation: *mut ::core::ffi::c_void, tokeninformationlength: u32, returnlength: *mut u32) -> super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Security`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetUserObjectSecurity(hobj: super::Foundation::HANDLE, psirequested: *const u32, psid: *mut SECURITY_DESCRIPTOR, nlength: u32, lpnlengthneeded: *mut u32) -> super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Security`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetWindowsAccountDomainSid(psid: super::Foundation::PSID, pdomainsid: super::Foundation::PSID, cbdomainsid: *mut u32) -> super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Security`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ImpersonateAnonymousToken(threadhandle: super::Foundation::HANDLE) -> super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Security`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ImpersonateLoggedOnUser(htoken: super::Foundation::HANDLE) -> super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Security`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ImpersonateSelf(impersonationlevel: SECURITY_IMPERSONATION_LEVEL) -> super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Security`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn InitializeAcl(pacl: *mut ACL, nacllength: u32, dwaclrevision: u32) -> super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Security`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn InitializeSecurityDescriptor(psecuritydescriptor: *mut SECURITY_DESCRIPTOR, dwrevision: u32) -> super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Security`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn InitializeSid(sid: super::Foundation::PSID, pidentifierauthority: *const SID_IDENTIFIER_AUTHORITY, nsubauthoritycount: u8) -> super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Security`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn IsTokenRestricted(tokenhandle: super::Foundation::HANDLE) -> super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Security`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn IsValidAcl(pacl: *const ACL) -> super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Security`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn IsValidSecurityDescriptor(psecuritydescriptor: *const SECURITY_DESCRIPTOR) -> super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Security`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn IsValidSid(psid: super::Foundation::PSID) -> super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Security`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn IsWellKnownSid(psid: super::Foundation::PSID, wellknownsidtype: WELL_KNOWN_SID_TYPE) -> super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Security`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn LogonUserA(lpszusername: super::Foundation::PSTR, lpszdomain: super::Foundation::PSTR, lpszpassword: super::Foundation::PSTR, dwlogontype: LOGON32_LOGON, dwlogonprovider: LOGON32_PROVIDER, phtoken: *mut super::Foundation::HANDLE) -> super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Security`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn LogonUserExA(lpszusername: super::Foundation::PSTR, lpszdomain: super::Foundation::PSTR, lpszpassword: super::Foundation::PSTR, dwlogontype: LOGON32_LOGON, dwlogonprovider: LOGON32_PROVIDER, phtoken: *mut super::Foundation::HANDLE, pplogonsid: *mut super::Foundation::PSID, ppprofilebuffer: *mut *mut ::core::ffi::c_void, pdwprofilelength: *mut u32, pquotalimits: *mut QUOTA_LIMITS) -> super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Security`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn LogonUserExW(lpszusername: super::Foundation::PWSTR, lpszdomain: super::Foundation::PWSTR, lpszpassword: super::Foundation::PWSTR, dwlogontype: LOGON32_LOGON, dwlogonprovider: LOGON32_PROVIDER, phtoken: *mut super::Foundation::HANDLE, pplogonsid: *mut super::Foundation::PSID, ppprofilebuffer: *mut *mut ::core::ffi::c_void, pdwprofilelength: *mut u32, pquotalimits: *mut QUOTA_LIMITS) -> super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Security`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn LogonUserW(lpszusername: super::Foundation::PWSTR, lpszdomain: super::Foundation::PWSTR, lpszpassword: super::Foundation::PWSTR, dwlogontype: LOGON32_LOGON, dwlogonprovider: LOGON32_PROVIDER, phtoken: *mut super::Foundation::HANDLE) -> super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Security`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn LookupAccountNameA(lpsystemname: super::Foundation::PSTR, lpaccountname: super::Foundation::PSTR, sid: super::Foundation::PSID, cbsid: *mut u32, referenceddomainname: super::Foundation::PSTR, cchreferenceddomainname: *mut u32, peuse: *mut SID_NAME_USE) -> super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Security`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn LookupAccountNameW(lpsystemname: super::Foundation::PWSTR, lpaccountname: super::Foundation::PWSTR, sid: super::Foundation::PSID, cbsid: *mut u32, referenceddomainname: super::Foundation::PWSTR, cchreferenceddomainname: *mut u32, peuse: *mut SID_NAME_USE) -> super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Security`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn LookupAccountSidA(lpsystemname: super::Foundation::PSTR, sid: super::Foundation::PSID, name: super::Foundation::PSTR, cchname: *mut u32, referenceddomainname: super::Foundation::PSTR, cchreferenceddomainname: *mut u32, peuse: *mut SID_NAME_USE) -> super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Security`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn LookupAccountSidW(lpsystemname: super::Foundation::PWSTR, sid: super::Foundation::PSID, name: super::Foundation::PWSTR, cchname: *mut u32, referenceddomainname: super::Foundation::PWSTR, cchreferenceddomainname: *mut u32, peuse: *mut SID_NAME_USE) -> super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Security`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn LookupPrivilegeDisplayNameA(lpsystemname: super::Foundation::PSTR, lpname: super::Foundation::PSTR, lpdisplayname: super::Foundation::PSTR, cchdisplayname: *mut u32, lplanguageid: *mut u32) -> super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Security`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn LookupPrivilegeDisplayNameW(lpsystemname: super::Foundation::PWSTR, lpname: super::Foundation::PWSTR, lpdisplayname: super::Foundation::PWSTR, cchdisplayname: *mut u32, lplanguageid: *mut u32) -> super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Security`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn LookupPrivilegeNameA(lpsystemname: super::Foundation::PSTR, lpluid: *const super::Foundation::LUID, lpname: super::Foundation::PSTR, cchname: *mut u32) -> super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Security`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn LookupPrivilegeNameW(lpsystemname: super::Foundation::PWSTR, lpluid: *const super::Foundation::LUID, lpname: super::Foundation::PWSTR, cchname: *mut u32) -> super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Security`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn LookupPrivilegeValueA(lpsystemname: super::Foundation::PSTR, lpname: super::Foundation::PSTR, lpluid: *mut super::Foundation::LUID) -> super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Security`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn LookupPrivilegeValueW(lpsystemname: super::Foundation::PWSTR, lpname: super::Foundation::PWSTR, lpluid: *mut super::Foundation::LUID) -> super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Security`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MakeAbsoluteSD(pselfrelativesecuritydescriptor: *const SECURITY_DESCRIPTOR, pabsolutesecuritydescriptor: *mut SECURITY_DESCRIPTOR, lpdwabsolutesecuritydescriptorsize: *mut u32, pdacl: *mut ACL, lpdwdaclsize: *mut u32, psacl: *mut ACL, lpdwsaclsize: *mut u32, powner: super::Foundation::PSID, lpdwownersize: *mut u32, pprimarygroup: super::Foundation::PSID, lpdwprimarygroupsize: *mut u32) -> super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Security`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MakeSelfRelativeSD(pabsolutesecuritydescriptor: *const SECURITY_DESCRIPTOR, pselfrelativesecuritydescriptor: *mut SECURITY_DESCRIPTOR, lpdwbufferlength: *mut u32) -> super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Security`*"]
    pub fn MapGenericMask(accessmask: *mut u32, genericmapping: *const GENERIC_MAPPING);
    #[doc = "*Required features: `Win32_Security`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ObjectCloseAuditAlarmA(subsystemname: super::Foundation::PSTR, handleid: *const ::core::ffi::c_void, generateonclose: super::Foundation::BOOL) -> super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Security`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ObjectCloseAuditAlarmW(subsystemname: super::Foundation::PWSTR, handleid: *const ::core::ffi::c_void, generateonclose: super::Foundation::BOOL) -> super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Security`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ObjectDeleteAuditAlarmA(subsystemname: super::Foundation::PSTR, handleid: *const ::core::ffi::c_void, generateonclose: super::Foundation::BOOL) -> super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Security`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ObjectDeleteAuditAlarmW(subsystemname: super::Foundation::PWSTR, handleid: *const ::core::ffi::c_void, generateonclose: super::Foundation::BOOL) -> super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Security`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ObjectOpenAuditAlarmA(subsystemname: super::Foundation::PSTR, handleid: *const ::core::ffi::c_void, objecttypename: super::Foundation::PSTR, objectname: super::Foundation::PSTR, psecuritydescriptor: *const SECURITY_DESCRIPTOR, clienttoken: super::Foundation::HANDLE, desiredaccess: u32, grantedaccess: u32, privileges: *const PRIVILEGE_SET, objectcreation: super::Foundation::BOOL, accessgranted: super::Foundation::BOOL, generateonclose: *mut i32) -> super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Security`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ObjectOpenAuditAlarmW(subsystemname: super::Foundation::PWSTR, handleid: *const ::core::ffi::c_void, objecttypename: super::Foundation::PWSTR, objectname: super::Foundation::PWSTR, psecuritydescriptor: *const SECURITY_DESCRIPTOR, clienttoken: super::Foundation::HANDLE, desiredaccess: u32, grantedaccess: u32, privileges: *const PRIVILEGE_SET, objectcreation: super::Foundation::BOOL, accessgranted: super::Foundation::BOOL, generateonclose: *mut i32) -> super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Security`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ObjectPrivilegeAuditAlarmA(subsystemname: super::Foundation::PSTR, handleid: *const ::core::ffi::c_void, clienttoken: super::Foundation::HANDLE, desiredaccess: u32, privileges: *const PRIVILEGE_SET, accessgranted: super::Foundation::BOOL) -> super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Security`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ObjectPrivilegeAuditAlarmW(subsystemname: super::Foundation::PWSTR, handleid: *const ::core::ffi::c_void, clienttoken: super::Foundation::HANDLE, desiredaccess: u32, privileges: *const PRIVILEGE_SET, accessgranted: super::Foundation::BOOL) -> super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Security`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PrivilegeCheck(clienttoken: super::Foundation::HANDLE, requiredprivileges: *mut PRIVILEGE_SET, pfresult: *mut i32) -> super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Security`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PrivilegedServiceAuditAlarmA(subsystemname: super::Foundation::PSTR, servicename: super::Foundation::PSTR, clienttoken: super::Foundation::HANDLE, privileges: *const PRIVILEGE_SET, accessgranted: super::Foundation::BOOL) -> super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Security`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PrivilegedServiceAuditAlarmW(subsystemname: super::Foundation::PWSTR, servicename: super::Foundation::PWSTR, clienttoken: super::Foundation::HANDLE, privileges: *const PRIVILEGE_SET, accessgranted: super::Foundation::BOOL) -> super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Security`*"]
    pub fn QuerySecurityAccessMask(securityinformation: u32, desiredaccess: *mut u32);
    #[doc = "*Required features: `Win32_Security`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RevertToSelf() -> super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Security`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RtlConvertSidToUnicodeString(unicodestring: *mut super::Foundation::UNICODE_STRING, sid: super::Foundation::PSID, allocatedestinationstring: super::Foundation::BOOLEAN) -> super::Foundation::NTSTATUS;
    #[doc = "*Required features: `Win32_Security`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RtlNormalizeSecurityDescriptor(securitydescriptor: *mut *mut SECURITY_DESCRIPTOR, securitydescriptorlength: u32, newsecuritydescriptor: *mut *mut SECURITY_DESCRIPTOR, newsecuritydescriptorlength: *mut u32, checkonly: super::Foundation::BOOLEAN) -> super::Foundation::BOOLEAN;
    #[doc = "*Required features: `Win32_Security`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetAclInformation(pacl: *mut ACL, paclinformation: *const ::core::ffi::c_void, naclinformationlength: u32, dwaclinformationclass: ACL_INFORMATION_CLASS) -> super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Security`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetCachedSigningLevel(sourcefiles: *const super::Foundation::HANDLE, sourcefilecount: u32, flags: u32, targetfile: super::Foundation::HANDLE) -> super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Security`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetFileSecurityA(lpfilename: super::Foundation::PSTR, securityinformation: u32, psecuritydescriptor: *const SECURITY_DESCRIPTOR) -> super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Security`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetFileSecurityW(lpfilename: super::Foundation::PWSTR, securityinformation: u32, psecuritydescriptor: *const SECURITY_DESCRIPTOR) -> super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Security`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetKernelObjectSecurity(handle: super::Foundation::HANDLE, securityinformation: u32, securitydescriptor: *const SECURITY_DESCRIPTOR) -> super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Security`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetPrivateObjectSecurity(securityinformation: u32, modificationdescriptor: *const SECURITY_DESCRIPTOR, objectssecuritydescriptor: *mut *mut SECURITY_DESCRIPTOR, genericmapping: *const GENERIC_MAPPING, token: super::Foundation::HANDLE) -> super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Security`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetPrivateObjectSecurityEx(securityinformation: u32, modificationdescriptor: *const SECURITY_DESCRIPTOR, objectssecuritydescriptor: *mut *mut SECURITY_DESCRIPTOR, autoinheritflags: SECURITY_AUTO_INHERIT_FLAGS, genericmapping: *const GENERIC_MAPPING, token: super::Foundation::HANDLE) -> super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Security`*"]
    pub fn SetSecurityAccessMask(securityinformation: u32, desiredaccess: *mut u32);
    #[doc = "*Required features: `Win32_Security`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetSecurityDescriptorControl(psecuritydescriptor: *const SECURITY_DESCRIPTOR, controlbitsofinterest: u16, controlbitstoset: u16) -> super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Security`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetSecurityDescriptorDacl(psecuritydescriptor: *mut SECURITY_DESCRIPTOR, bdaclpresent: super::Foundation::BOOL, pdacl: *const ACL, bdacldefaulted: super::Foundation::BOOL) -> super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Security`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetSecurityDescriptorGroup(psecuritydescriptor: *mut SECURITY_DESCRIPTOR, pgroup: super::Foundation::PSID, bgroupdefaulted: super::Foundation::BOOL) -> super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Security`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetSecurityDescriptorOwner(psecuritydescriptor: *mut SECURITY_DESCRIPTOR, powner: super::Foundation::PSID, bownerdefaulted: super::Foundation::BOOL) -> super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Security`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetSecurityDescriptorRMControl(securitydescriptor: *mut SECURITY_DESCRIPTOR, rmcontrol: *const u8) -> u32;
    #[doc = "*Required features: `Win32_Security`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetSecurityDescriptorSacl(psecuritydescriptor: *mut SECURITY_DESCRIPTOR, bsaclpresent: super::Foundation::BOOL, psacl: *const ACL, bsacldefaulted: super::Foundation::BOOL) -> super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Security`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetTokenInformation(tokenhandle: super::Foundation::HANDLE, tokeninformationclass: TOKEN_INFORMATION_CLASS, tokeninformation: *const ::core::ffi::c_void, tokeninformationlength: u32) -> super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Security`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetUserObjectSecurity(hobj: super::Foundation::HANDLE, psirequested: *const OBJECT_SECURITY_INFORMATION, psid: *const SECURITY_DESCRIPTOR) -> super::Foundation::BOOL;
}
