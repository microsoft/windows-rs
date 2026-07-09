#[cfg(feature = "Win32_winnt")]
windows_link::link!("advapi32.dll" "system" fn AuditComputeEffectivePolicyBySid(psid : super::winnt::PSID, psubcategoryguids : *const windows_sys::core::GUID, dwpolicycount : u32, ppauditpolicy : *mut PAUDIT_POLICY_INFORMATION) -> bool);
#[cfg(feature = "Win32_winnt")]
windows_link::link!("advapi32.dll" "system" fn AuditComputeEffectivePolicyByToken(htokenhandle : super::winnt::HANDLE, psubcategoryguids : *const windows_sys::core::GUID, dwpolicycount : u32, ppauditpolicy : *mut PAUDIT_POLICY_INFORMATION) -> bool);
windows_link::link!("advapi32.dll" "system" fn AuditEnumerateCategories(ppauditcategoriesarray : *mut *mut windows_sys::core::GUID, pdwcountreturned : *mut u32) -> bool);
#[cfg(feature = "Win32_winnt")]
windows_link::link!("advapi32.dll" "system" fn AuditEnumeratePerUserPolicy(ppauditsidarray : *mut PPOLICY_AUDIT_SID_ARRAY) -> bool);
windows_link::link!("advapi32.dll" "system" fn AuditEnumerateSubCategories(pauditcategoryguid : *const windows_sys::core::GUID, bretrieveallsubcategories : bool, ppauditsubcategoriesarray : *mut *mut windows_sys::core::GUID, pdwcountreturned : *mut u32) -> bool);
windows_link::link!("advapi32.dll" "system" fn AuditFree(buffer : *const core::ffi::c_void));
windows_link::link!("advapi32.dll" "system" fn AuditLookupCategoryGuidFromCategoryId(auditcategoryid : POLICY_AUDIT_EVENT_TYPE, pauditcategoryguid : *mut windows_sys::core::GUID) -> bool);
windows_link::link!("advapi32.dll" "system" fn AuditLookupCategoryIdFromCategoryGuid(pauditcategoryguid : *const windows_sys::core::GUID, pauditcategoryid : *mut POLICY_AUDIT_EVENT_TYPE) -> bool);
windows_link::link!("advapi32.dll" "system" fn AuditLookupCategoryNameA(pauditcategoryguid : *const windows_sys::core::GUID, ppszcategoryname : *mut windows_sys::core::PSTR) -> bool);
windows_link::link!("advapi32.dll" "system" fn AuditLookupCategoryNameW(pauditcategoryguid : *const windows_sys::core::GUID, ppszcategoryname : *mut windows_sys::core::PWSTR) -> bool);
windows_link::link!("advapi32.dll" "system" fn AuditLookupSubCategoryNameA(pauditsubcategoryguid : *const windows_sys::core::GUID, ppszsubcategoryname : *mut windows_sys::core::PSTR) -> bool);
windows_link::link!("advapi32.dll" "system" fn AuditLookupSubCategoryNameW(pauditsubcategoryguid : *const windows_sys::core::GUID, ppszsubcategoryname : *mut windows_sys::core::PWSTR) -> bool);
#[cfg(feature = "Win32_winnt")]
windows_link::link!("advapi32.dll" "system" fn AuditQueryGlobalSaclA(objecttypename : windows_sys::core::PCSTR, acl : *mut super::winnt::PACL) -> bool);
#[cfg(feature = "Win32_winnt")]
windows_link::link!("advapi32.dll" "system" fn AuditQueryGlobalSaclW(objecttypename : windows_sys::core::PCWSTR, acl : *mut super::winnt::PACL) -> bool);
#[cfg(feature = "Win32_winnt")]
windows_link::link!("advapi32.dll" "system" fn AuditQueryPerUserPolicy(psid : super::winnt::PSID, psubcategoryguids : *const windows_sys::core::GUID, dwpolicycount : u32, ppauditpolicy : *mut PAUDIT_POLICY_INFORMATION) -> bool);
#[cfg(feature = "Win32_winnt")]
windows_link::link!("advapi32.dll" "system" fn AuditQuerySecurity(securityinformation : super::winnt::SECURITY_INFORMATION, ppsecuritydescriptor : *mut super::winnt::PSECURITY_DESCRIPTOR) -> bool);
windows_link::link!("advapi32.dll" "system" fn AuditQuerySystemPolicy(psubcategoryguids : *const windows_sys::core::GUID, dwpolicycount : u32, ppauditpolicy : *mut PAUDIT_POLICY_INFORMATION) -> bool);
#[cfg(feature = "Win32_winnt")]
windows_link::link!("advapi32.dll" "system" fn AuditSetGlobalSaclA(objecttypename : windows_sys::core::PCSTR, acl : *const super::winnt::ACL) -> bool);
#[cfg(feature = "Win32_winnt")]
windows_link::link!("advapi32.dll" "system" fn AuditSetGlobalSaclW(objecttypename : windows_sys::core::PCWSTR, acl : *const super::winnt::ACL) -> bool);
#[cfg(feature = "Win32_winnt")]
windows_link::link!("advapi32.dll" "system" fn AuditSetPerUserPolicy(psid : super::winnt::PSID, pauditpolicy : PCAUDIT_POLICY_INFORMATION, dwpolicycount : u32) -> bool);
#[cfg(feature = "Win32_winnt")]
windows_link::link!("advapi32.dll" "system" fn AuditSetSecurity(securityinformation : super::winnt::SECURITY_INFORMATION, psecuritydescriptor : super::winnt::PSECURITY_DESCRIPTOR) -> bool);
windows_link::link!("advapi32.dll" "system" fn AuditSetSystemPolicy(pauditpolicy : PCAUDIT_POLICY_INFORMATION, dwpolicycount : u32) -> bool);
#[cfg(all(feature = "Win32_bcrypt", feature = "Win32_lsalookup", feature = "Win32_winnt"))]
windows_link::link!("advapi32.dll" "system" fn LsaAddAccountRights(policyhandle : LSA_HANDLE, accountsid : super::winnt::PSID, userrights : *const super::lsalookup::LSA_UNICODE_STRING, countofrights : u32) -> super::bcrypt::NTSTATUS);
#[cfg(all(feature = "Win32_bcrypt", feature = "Win32_winnt"))]
windows_link::link!("secur32.dll" "system" fn LsaCallAuthenticationPackage(lsahandle : super::winnt::HANDLE, authenticationpackage : u32, protocolsubmitbuffer : *const core::ffi::c_void, submitbufferlength : u32, protocolreturnbuffer : *mut *mut core::ffi::c_void, returnbufferlength : *mut u32, protocolstatus : *mut i32) -> super::bcrypt::NTSTATUS);
#[cfg(feature = "Win32_bcrypt")]
windows_link::link!("advapi32.dll" "system" fn LsaClose(objecthandle : LSA_HANDLE) -> super::bcrypt::NTSTATUS);
#[cfg(all(feature = "Win32_bcrypt", feature = "Win32_winnt"))]
windows_link::link!("secur32.dll" "system" fn LsaConnectUntrusted(lsahandle : *mut super::winnt::HANDLE) -> super::bcrypt::NTSTATUS);
#[cfg(all(feature = "Win32_bcrypt", feature = "Win32_lsalookup", feature = "Win32_minwindef", feature = "Win32_winnt"))]
windows_link::link!("advapi32.dll" "system" fn LsaCreateTrustedDomainEx(policyhandle : LSA_HANDLE, trusteddomaininformation : *const TRUSTED_DOMAIN_INFORMATION_EX, authenticationinformation : *const TRUSTED_DOMAIN_AUTH_INFORMATION, desiredaccess : super::winnt::ACCESS_MASK, trusteddomainhandle : *mut *mut core::ffi::c_void) -> super::bcrypt::NTSTATUS);
#[cfg(all(feature = "Win32_bcrypt", feature = "Win32_winnt"))]
windows_link::link!("advapi32.dll" "system" fn LsaDeleteTrustedDomain(policyhandle : LSA_HANDLE, trusteddomainsid : super::winnt::PSID) -> super::bcrypt::NTSTATUS);
#[cfg(all(feature = "Win32_bcrypt", feature = "Win32_winnt"))]
windows_link::link!("secur32.dll" "system" fn LsaDeregisterLogonProcess(lsahandle : super::winnt::HANDLE) -> super::bcrypt::NTSTATUS);
#[cfg(all(feature = "Win32_bcrypt", feature = "Win32_lsalookup", feature = "Win32_winnt"))]
windows_link::link!("advapi32.dll" "system" fn LsaEnumerateAccountRights(policyhandle : LSA_HANDLE, accountsid : super::winnt::PSID, userrights : *mut super::lsalookup::PLSA_UNICODE_STRING, countofrights : *mut u32) -> super::bcrypt::NTSTATUS);
#[cfg(all(feature = "Win32_bcrypt", feature = "Win32_lsalookup"))]
windows_link::link!("advapi32.dll" "system" fn LsaEnumerateAccountsWithUserRight(policyhandle : LSA_HANDLE, userright : *const super::lsalookup::LSA_UNICODE_STRING, buffer : *mut *mut core::ffi::c_void, countreturned : *mut u32) -> super::bcrypt::NTSTATUS);
#[cfg(all(feature = "Win32_bcrypt", feature = "Win32_winnt"))]
windows_link::link!("secur32.dll" "system" fn LsaEnumerateLogonSessions(logonsessioncount : *mut u32, logonsessionlist : *mut super::winnt::PLUID) -> super::bcrypt::NTSTATUS);
#[cfg(feature = "Win32_bcrypt")]
windows_link::link!("advapi32.dll" "system" fn LsaEnumerateTrustedDomains(policyhandle : LSA_HANDLE, enumerationcontext : *mut u32, buffer : *mut *mut core::ffi::c_void, preferedmaximumlength : u32, countreturned : *mut u32) -> super::bcrypt::NTSTATUS);
#[cfg(feature = "Win32_bcrypt")]
windows_link::link!("advapi32.dll" "system" fn LsaEnumerateTrustedDomainsEx(policyhandle : LSA_HANDLE, enumerationcontext : *mut u32, buffer : *mut *mut core::ffi::c_void, preferedmaximumlength : u32, countreturned : *mut u32) -> super::bcrypt::NTSTATUS);
#[cfg(feature = "Win32_bcrypt")]
windows_link::link!("advapi32.dll" "system" fn LsaFreeMemory(buffer : *const core::ffi::c_void) -> super::bcrypt::NTSTATUS);
#[cfg(feature = "Win32_bcrypt")]
windows_link::link!("secur32.dll" "system" fn LsaFreeReturnBuffer(buffer : *const core::ffi::c_void) -> super::bcrypt::NTSTATUS);
#[cfg(all(feature = "Win32_bcrypt", feature = "Win32_lsalookup", feature = "Win32_winnt"))]
windows_link::link!("advapi32.dll" "system" fn LsaGetAppliedCAPIDs(systemname : *const super::lsalookup::LSA_UNICODE_STRING, capids : *mut *mut super::winnt::PSID, capidcount : *mut u32) -> super::bcrypt::NTSTATUS);
#[cfg(all(feature = "Win32_bcrypt", feature = "Win32_lsalookup", feature = "Win32_winnt"))]
windows_link::link!("secur32.dll" "system" fn LsaGetLogonSessionData(logonid : *const super::winnt::LUID, pplogonsessiondata : *mut PSECURITY_LOGON_SESSION_DATA) -> super::bcrypt::NTSTATUS);
#[cfg(all(feature = "Win32_bcrypt", feature = "Win32_lsalookup", feature = "Win32_winnt"))]
windows_link::link!("secur32.dll" "system" fn LsaLogonUser(lsahandle : super::winnt::HANDLE, originname : *const super::lsalookup::LSA_STRING, logontype : SECURITY_LOGON_TYPE, authenticationpackage : u32, authenticationinformation : *const core::ffi::c_void, authenticationinformationlength : u32, localgroups : *const super::winnt::TOKEN_GROUPS, sourcecontext : *const super::winnt::TOKEN_SOURCE, profilebuffer : *mut *mut core::ffi::c_void, profilebufferlength : *mut u32, logonid : *mut super::winnt::LUID, token : *mut super::winnt::HANDLE, quotas : *mut super::winnt::QUOTA_LIMITS, substatus : *mut i32) -> super::bcrypt::NTSTATUS);
#[cfg(all(feature = "Win32_bcrypt", feature = "Win32_lsalookup", feature = "Win32_winnt"))]
windows_link::link!("secur32.dll" "system" fn LsaLookupAuthenticationPackage(lsahandle : super::winnt::HANDLE, packagename : *const super::lsalookup::LSA_STRING, authenticationpackage : *mut u32) -> super::bcrypt::NTSTATUS);
#[cfg(all(feature = "Win32_bcrypt", feature = "Win32_lsalookup", feature = "Win32_winnt"))]
windows_link::link!("advapi32.dll" "system" fn LsaLookupNames(policyhandle : LSA_HANDLE, count : u32, names : *const super::lsalookup::LSA_UNICODE_STRING, referenceddomains : *mut super::lsalookup::PLSA_REFERENCED_DOMAIN_LIST, sids : *mut PLSA_TRANSLATED_SID) -> super::bcrypt::NTSTATUS);
#[cfg(all(feature = "Win32_bcrypt", feature = "Win32_lsalookup", feature = "Win32_winnt"))]
windows_link::link!("advapi32.dll" "system" fn LsaLookupNames2(policyhandle : LSA_HANDLE, flags : u32, count : u32, names : *const super::lsalookup::LSA_UNICODE_STRING, referenceddomains : *mut super::lsalookup::PLSA_REFERENCED_DOMAIN_LIST, sids : *mut super::lsalookup::PLSA_TRANSLATED_SID2) -> super::bcrypt::NTSTATUS);
#[cfg(all(feature = "Win32_bcrypt", feature = "Win32_lsalookup", feature = "Win32_winnt"))]
windows_link::link!("advapi32.dll" "system" fn LsaLookupSids(policyhandle : LSA_HANDLE, count : u32, sids : *const super::winnt::PSID, referenceddomains : *mut super::lsalookup::PLSA_REFERENCED_DOMAIN_LIST, names : *mut super::lsalookup::PLSA_TRANSLATED_NAME) -> super::bcrypt::NTSTATUS);
#[cfg(all(feature = "Win32_bcrypt", feature = "Win32_lsalookup", feature = "Win32_winnt"))]
windows_link::link!("advapi32.dll" "system" fn LsaLookupSids2(policyhandle : LSA_HANDLE, lookupoptions : u32, count : u32, sids : *const super::winnt::PSID, referenceddomains : *mut super::lsalookup::PLSA_REFERENCED_DOMAIN_LIST, names : *mut super::lsalookup::PLSA_TRANSLATED_NAME) -> super::bcrypt::NTSTATUS);
#[cfg(feature = "Win32_bcrypt")]
windows_link::link!("advapi32.dll" "system" fn LsaNtStatusToWinError(status : super::bcrypt::NTSTATUS) -> u32);
#[cfg(all(feature = "Win32_bcrypt", feature = "Win32_lsalookup", feature = "Win32_winnt"))]
windows_link::link!("advapi32.dll" "system" fn LsaOpenPolicy(systemname : *const super::lsalookup::LSA_UNICODE_STRING, objectattributes : *const super::lsalookup::LSA_OBJECT_ATTRIBUTES, desiredaccess : super::winnt::ACCESS_MASK, policyhandle : *mut *mut core::ffi::c_void) -> super::bcrypt::NTSTATUS);
#[cfg(all(feature = "Win32_bcrypt", feature = "Win32_lsalookup", feature = "Win32_winnt"))]
windows_link::link!("advapi32.dll" "system" fn LsaOpenTrustedDomainByName(policyhandle : LSA_HANDLE, trusteddomainname : *const super::lsalookup::LSA_UNICODE_STRING, desiredaccess : super::winnt::ACCESS_MASK, trusteddomainhandle : *mut *mut core::ffi::c_void) -> super::bcrypt::NTSTATUS);
#[cfg(all(feature = "Win32_bcrypt", feature = "Win32_lsalookup", feature = "Win32_minwindef", feature = "Win32_winnt"))]
windows_link::link!("advapi32.dll" "system" fn LsaQueryCAPs(capids : *const super::winnt::PSID, capidcount : u32, caps : *mut PCENTRAL_ACCESS_POLICY, capcount : *mut u32) -> super::bcrypt::NTSTATUS);
#[cfg(feature = "Win32_bcrypt")]
windows_link::link!("advapi32.dll" "system" fn LsaQueryDomainInformationPolicy(policyhandle : LSA_HANDLE, informationclass : POLICY_DOMAIN_INFORMATION_CLASS, buffer : *mut *mut core::ffi::c_void) -> super::bcrypt::NTSTATUS);
#[cfg(all(feature = "Win32_bcrypt", feature = "Win32_lsalookup", feature = "Win32_minwindef", feature = "Win32_winnt"))]
windows_link::link!("advapi32.dll" "system" fn LsaQueryForestTrustInformation(policyhandle : LSA_HANDLE, trusteddomainname : *const super::lsalookup::LSA_UNICODE_STRING, foresttrustinfo : *mut PLSA_FOREST_TRUST_INFORMATION) -> super::bcrypt::NTSTATUS);
#[cfg(all(feature = "Win32_bcrypt", feature = "Win32_lsalookup", feature = "Win32_minwindef", feature = "Win32_winnt"))]
windows_link::link!("advapi32.dll" "system" fn LsaQueryForestTrustInformation2(policyhandle : LSA_HANDLE, trusteddomainname : *const super::lsalookup::LSA_UNICODE_STRING, highestrecordtype : LSA_FOREST_TRUST_RECORD_TYPE, foresttrustinfo : *mut PLSA_FOREST_TRUST_INFORMATION2) -> super::bcrypt::NTSTATUS);
#[cfg(feature = "Win32_bcrypt")]
windows_link::link!("advapi32.dll" "system" fn LsaQueryInformationPolicy(policyhandle : LSA_HANDLE, informationclass : POLICY_INFORMATION_CLASS, buffer : *mut *mut core::ffi::c_void) -> super::bcrypt::NTSTATUS);
#[cfg(all(feature = "Win32_bcrypt", feature = "Win32_winnt"))]
windows_link::link!("advapi32.dll" "system" fn LsaQueryTrustedDomainInfo(policyhandle : LSA_HANDLE, trusteddomainsid : super::winnt::PSID, informationclass : TRUSTED_INFORMATION_CLASS, buffer : *mut *mut core::ffi::c_void) -> super::bcrypt::NTSTATUS);
#[cfg(all(feature = "Win32_bcrypt", feature = "Win32_lsalookup"))]
windows_link::link!("advapi32.dll" "system" fn LsaQueryTrustedDomainInfoByName(policyhandle : LSA_HANDLE, trusteddomainname : *const super::lsalookup::LSA_UNICODE_STRING, informationclass : TRUSTED_INFORMATION_CLASS, buffer : *mut *mut core::ffi::c_void) -> super::bcrypt::NTSTATUS);
#[cfg(all(feature = "Win32_bcrypt", feature = "Win32_lsalookup", feature = "Win32_winnt"))]
windows_link::link!("secur32.dll" "system" fn LsaRegisterLogonProcess(logonprocessname : *const super::lsalookup::LSA_STRING, lsahandle : *mut super::winnt::HANDLE, securitymode : *mut u32) -> super::bcrypt::NTSTATUS);
#[cfg(all(feature = "Win32_bcrypt", feature = "Win32_winnt"))]
windows_link::link!("secur32.dll" "system" fn LsaRegisterPolicyChangeNotification(informationclass : POLICY_NOTIFICATION_INFORMATION_CLASS, notificationeventhandle : super::winnt::HANDLE) -> super::bcrypt::NTSTATUS);
#[cfg(all(feature = "Win32_bcrypt", feature = "Win32_lsalookup", feature = "Win32_winnt"))]
windows_link::link!("advapi32.dll" "system" fn LsaRemoveAccountRights(policyhandle : LSA_HANDLE, accountsid : super::winnt::PSID, allrights : bool, userrights : *const super::lsalookup::LSA_UNICODE_STRING, countofrights : u32) -> super::bcrypt::NTSTATUS);
#[cfg(all(feature = "Win32_bcrypt", feature = "Win32_lsalookup"))]
windows_link::link!("advapi32.dll" "system" fn LsaRetrievePrivateData(policyhandle : LSA_HANDLE, keyname : *const super::lsalookup::LSA_UNICODE_STRING, privatedata : *mut super::lsalookup::PLSA_UNICODE_STRING) -> super::bcrypt::NTSTATUS);
#[cfg(all(feature = "Win32_bcrypt", feature = "Win32_lsalookup"))]
windows_link::link!("advapi32.dll" "system" fn LsaSetCAPs(capdns : *const super::lsalookup::LSA_UNICODE_STRING, capdncount : u32, flags : u32) -> super::bcrypt::NTSTATUS);
#[cfg(feature = "Win32_bcrypt")]
windows_link::link!("advapi32.dll" "system" fn LsaSetDomainInformationPolicy(policyhandle : LSA_HANDLE, informationclass : POLICY_DOMAIN_INFORMATION_CLASS, buffer : *const core::ffi::c_void) -> super::bcrypt::NTSTATUS);
#[cfg(all(feature = "Win32_bcrypt", feature = "Win32_lsalookup", feature = "Win32_minwindef", feature = "Win32_winnt"))]
windows_link::link!("advapi32.dll" "system" fn LsaSetForestTrustInformation(policyhandle : LSA_HANDLE, trusteddomainname : *const super::lsalookup::LSA_UNICODE_STRING, foresttrustinfo : *const LSA_FOREST_TRUST_INFORMATION, checkonly : bool, collisioninfo : *mut PLSA_FOREST_TRUST_COLLISION_INFORMATION) -> super::bcrypt::NTSTATUS);
#[cfg(all(feature = "Win32_bcrypt", feature = "Win32_lsalookup", feature = "Win32_minwindef", feature = "Win32_winnt"))]
windows_link::link!("advapi32.dll" "system" fn LsaSetForestTrustInformation2(policyhandle : LSA_HANDLE, trusteddomainname : *const super::lsalookup::LSA_UNICODE_STRING, highestrecordtype : LSA_FOREST_TRUST_RECORD_TYPE, foresttrustinfo : *const LSA_FOREST_TRUST_INFORMATION2, checkonly : bool, collisioninfo : *mut PLSA_FOREST_TRUST_COLLISION_INFORMATION) -> super::bcrypt::NTSTATUS);
#[cfg(feature = "Win32_bcrypt")]
windows_link::link!("advapi32.dll" "system" fn LsaSetInformationPolicy(policyhandle : LSA_HANDLE, informationclass : POLICY_INFORMATION_CLASS, buffer : *const core::ffi::c_void) -> super::bcrypt::NTSTATUS);
#[cfg(all(feature = "Win32_bcrypt", feature = "Win32_lsalookup"))]
windows_link::link!("advapi32.dll" "system" fn LsaSetTrustedDomainInfoByName(policyhandle : LSA_HANDLE, trusteddomainname : *const super::lsalookup::LSA_UNICODE_STRING, informationclass : TRUSTED_INFORMATION_CLASS, buffer : *const core::ffi::c_void) -> super::bcrypt::NTSTATUS);
#[cfg(all(feature = "Win32_bcrypt", feature = "Win32_winnt"))]
windows_link::link!("advapi32.dll" "system" fn LsaSetTrustedDomainInformation(policyhandle : LSA_HANDLE, trusteddomainsid : super::winnt::PSID, informationclass : TRUSTED_INFORMATION_CLASS, buffer : *const core::ffi::c_void) -> super::bcrypt::NTSTATUS);
#[cfg(all(feature = "Win32_bcrypt", feature = "Win32_lsalookup"))]
windows_link::link!("advapi32.dll" "system" fn LsaStorePrivateData(policyhandle : LSA_HANDLE, keyname : *const super::lsalookup::LSA_UNICODE_STRING, privatedata : *const super::lsalookup::LSA_UNICODE_STRING) -> super::bcrypt::NTSTATUS);
#[cfg(all(feature = "Win32_bcrypt", feature = "Win32_winnt"))]
windows_link::link!("secur32.dll" "system" fn LsaUnregisterPolicyChangeNotification(informationclass : POLICY_NOTIFICATION_INFORMATION_CLASS, notificationeventhandle : super::winnt::HANDLE) -> super::bcrypt::NTSTATUS);
windows_link::link!("advapi32.dll" "C" fn SystemFunction036(randombuffer : *mut core::ffi::c_void, randombufferlength : u32) -> bool);
#[cfg(feature = "Win32_bcrypt")]
windows_link::link!("advapi32.dll" "C" fn SystemFunction040(memory : *mut core::ffi::c_void, memorysize : u32, optionflags : u32) -> super::bcrypt::NTSTATUS);
#[cfg(feature = "Win32_bcrypt")]
windows_link::link!("advapi32.dll" "C" fn SystemFunction041(memory : *mut core::ffi::c_void, memorysize : u32, optionflags : u32) -> super::bcrypt::NTSTATUS);
pub const AUDIT_ENUMERATE_USERS: u32 = 16;
pub const AUDIT_GENERIC_ALL: u32 = 983167;
pub const AUDIT_GENERIC_EXECUTE: u32 = 131072;
pub const AUDIT_GENERIC_READ: u32 = 131162;
pub const AUDIT_GENERIC_WRITE: u32 = 131109;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct AUDIT_POLICY_INFORMATION {
    pub AuditSubCategoryGuid: windows_sys::core::GUID,
    pub AuditingInformation: u32,
    pub AuditCategoryGuid: windows_sys::core::GUID,
}
pub const AUDIT_QUERY_MISC_POLICY: u32 = 64;
pub const AUDIT_QUERY_SYSTEM_POLICY: u32 = 2;
pub const AUDIT_QUERY_USER_POLICY: u32 = 8;
pub const AUDIT_SET_MISC_POLICY: u32 = 32;
pub const AUDIT_SET_SYSTEM_POLICY: u32 = 1;
pub const AUDIT_SET_USER_POLICY: u32 = 4;
pub const AUTH_REQ_ALLOW_ENC_TKT_IN_SKEY: u32 = 32;
pub const AUTH_REQ_ALLOW_FORWARDABLE: u32 = 1;
pub const AUTH_REQ_ALLOW_NOADDRESS: u32 = 16;
pub const AUTH_REQ_ALLOW_POSTDATE: u32 = 4;
pub const AUTH_REQ_ALLOW_PROXIABLE: u32 = 2;
pub const AUTH_REQ_ALLOW_RENEWABLE: u32 = 8;
pub const AUTH_REQ_ALLOW_S4U_DELEGATE: u32 = 2048;
pub const AUTH_REQ_ALLOW_VALIDATE: u32 = 64;
pub const AUTH_REQ_OK_AS_DELEGATE: u32 = 256;
pub const AUTH_REQ_PER_USER_FLAGS: u32 = 79;
pub const AUTH_REQ_PREAUTH_REQUIRED: u32 = 512;
pub const AUTH_REQ_TRANSITIVE_TRUST: u32 = 1024;
pub const AUTH_REQ_VALIDATE_CLIENT: u32 = 128;
pub const AuditCategoryAccountLogon: POLICY_AUDIT_EVENT_TYPE = 8;
pub const AuditCategoryAccountManagement: POLICY_AUDIT_EVENT_TYPE = 6;
pub const AuditCategoryDetailedTracking: POLICY_AUDIT_EVENT_TYPE = 4;
pub const AuditCategoryDirectoryServiceAccess: POLICY_AUDIT_EVENT_TYPE = 7;
pub const AuditCategoryLogon: POLICY_AUDIT_EVENT_TYPE = 1;
pub const AuditCategoryObjectAccess: POLICY_AUDIT_EVENT_TYPE = 2;
pub const AuditCategoryPolicyChange: POLICY_AUDIT_EVENT_TYPE = 5;
pub const AuditCategoryPrivilegeUse: POLICY_AUDIT_EVENT_TYPE = 3;
pub const AuditCategorySystem: POLICY_AUDIT_EVENT_TYPE = 0;
pub const Audit_AccountLogon: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x69979850_797a_11d9_bed3_505054503030);
pub const Audit_AccountLogon_CredentialValidation: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x0cce923f_69ae_11d9_bed3_505054503030);
pub const Audit_AccountLogon_KerbCredentialValidation: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x0cce9242_69ae_11d9_bed3_505054503030);
pub const Audit_AccountLogon_Kerberos: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x0cce9240_69ae_11d9_bed3_505054503030);
pub const Audit_AccountLogon_Others: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x0cce9241_69ae_11d9_bed3_505054503030);
pub const Audit_AccountManagement: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x6997984e_797a_11d9_bed3_505054503030);
pub const Audit_AccountManagement_ApplicationGroup: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x0cce9239_69ae_11d9_bed3_505054503030);
pub const Audit_AccountManagement_ComputerAccount: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x0cce9236_69ae_11d9_bed3_505054503030);
pub const Audit_AccountManagement_DistributionGroup: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x0cce9238_69ae_11d9_bed3_505054503030);
pub const Audit_AccountManagement_Others: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x0cce923a_69ae_11d9_bed3_505054503030);
pub const Audit_AccountManagement_SecurityGroup: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x0cce9237_69ae_11d9_bed3_505054503030);
pub const Audit_AccountManagement_UserAccount: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x0cce9235_69ae_11d9_bed3_505054503030);
pub const Audit_DSAccess_DSAccess: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x0cce923b_69ae_11d9_bed3_505054503030);
pub const Audit_DetailedTracking: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x6997984c_797a_11d9_bed3_505054503030);
pub const Audit_DetailedTracking_DpapiActivity: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x0cce922d_69ae_11d9_bed3_505054503030);
pub const Audit_DetailedTracking_PnpActivity: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x0cce9248_69ae_11d9_bed3_505054503030);
pub const Audit_DetailedTracking_ProcessCreation: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x0cce922b_69ae_11d9_bed3_505054503030);
pub const Audit_DetailedTracking_ProcessTermination: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x0cce922c_69ae_11d9_bed3_505054503030);
pub const Audit_DetailedTracking_RpcCall: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x0cce922e_69ae_11d9_bed3_505054503030);
pub const Audit_DetailedTracking_TokenRightAdjusted: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x0cce924a_69ae_11d9_bed3_505054503030);
pub const Audit_DirectoryServiceAccess: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x6997984f_797a_11d9_bed3_505054503030);
pub const Audit_DsAccess_AdAuditChanges: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x0cce923c_69ae_11d9_bed3_505054503030);
pub const Audit_Ds_DetailedReplication: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x0cce923e_69ae_11d9_bed3_505054503030);
pub const Audit_Ds_Replication: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x0cce923d_69ae_11d9_bed3_505054503030);
pub const Audit_Logon: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x69979849_797a_11d9_bed3_505054503030);
pub const Audit_Logon_AccessRights: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x0cce924b_69ae_11d9_bed3_505054503030);
pub const Audit_Logon_AccountLockout: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x0cce9217_69ae_11d9_bed3_505054503030);
pub const Audit_Logon_Claims: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x0cce9247_69ae_11d9_bed3_505054503030);
pub const Audit_Logon_Groups: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x0cce9249_69ae_11d9_bed3_505054503030);
pub const Audit_Logon_IPSecMainMode: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x0cce9218_69ae_11d9_bed3_505054503030);
pub const Audit_Logon_IPSecQuickMode: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x0cce9219_69ae_11d9_bed3_505054503030);
pub const Audit_Logon_IPSecUserMode: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x0cce921a_69ae_11d9_bed3_505054503030);
pub const Audit_Logon_Logoff: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x0cce9216_69ae_11d9_bed3_505054503030);
pub const Audit_Logon_Logon: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x0cce9215_69ae_11d9_bed3_505054503030);
pub const Audit_Logon_NPS: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x0cce9243_69ae_11d9_bed3_505054503030);
pub const Audit_Logon_Others: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x0cce921c_69ae_11d9_bed3_505054503030);
pub const Audit_Logon_SpecialLogon: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x0cce921b_69ae_11d9_bed3_505054503030);
pub const Audit_ObjectAccess: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x6997984a_797a_11d9_bed3_505054503030);
pub const Audit_ObjectAccess_ApplicationGenerated: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x0cce9222_69ae_11d9_bed3_505054503030);
pub const Audit_ObjectAccess_CbacStaging: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x0cce9246_69ae_11d9_bed3_505054503030);
pub const Audit_ObjectAccess_CertificationServices: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x0cce9221_69ae_11d9_bed3_505054503030);
pub const Audit_ObjectAccess_DetailedFileShare: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x0cce9244_69ae_11d9_bed3_505054503030);
pub const Audit_ObjectAccess_FileSystem: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x0cce921d_69ae_11d9_bed3_505054503030);
pub const Audit_ObjectAccess_FirewallConnection: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x0cce9226_69ae_11d9_bed3_505054503030);
pub const Audit_ObjectAccess_FirewallPacketDrops: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x0cce9225_69ae_11d9_bed3_505054503030);
pub const Audit_ObjectAccess_Handle: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x0cce9223_69ae_11d9_bed3_505054503030);
pub const Audit_ObjectAccess_Kernel: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x0cce921f_69ae_11d9_bed3_505054503030);
pub const Audit_ObjectAccess_Other: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x0cce9227_69ae_11d9_bed3_505054503030);
pub const Audit_ObjectAccess_Registry: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x0cce921e_69ae_11d9_bed3_505054503030);
pub const Audit_ObjectAccess_RemovableStorage: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x0cce9245_69ae_11d9_bed3_505054503030);
pub const Audit_ObjectAccess_Sam: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x0cce9220_69ae_11d9_bed3_505054503030);
pub const Audit_ObjectAccess_Share: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x0cce9224_69ae_11d9_bed3_505054503030);
pub const Audit_PolicyChange: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x6997984d_797a_11d9_bed3_505054503030);
pub const Audit_PolicyChange_AuditPolicy: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x0cce922f_69ae_11d9_bed3_505054503030);
pub const Audit_PolicyChange_AuthenticationPolicy: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x0cce9230_69ae_11d9_bed3_505054503030);
pub const Audit_PolicyChange_AuthorizationPolicy: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x0cce9231_69ae_11d9_bed3_505054503030);
pub const Audit_PolicyChange_MpsscvRulePolicy: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x0cce9232_69ae_11d9_bed3_505054503030);
pub const Audit_PolicyChange_Others: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x0cce9234_69ae_11d9_bed3_505054503030);
pub const Audit_PolicyChange_WfpIPSecPolicy: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x0cce9233_69ae_11d9_bed3_505054503030);
pub const Audit_PrivilegeUse: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x6997984b_797a_11d9_bed3_505054503030);
pub const Audit_PrivilegeUse_NonSensitive: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x0cce9229_69ae_11d9_bed3_505054503030);
pub const Audit_PrivilegeUse_Others: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x0cce922a_69ae_11d9_bed3_505054503030);
pub const Audit_PrivilegeUse_Sensitive: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x0cce9228_69ae_11d9_bed3_505054503030);
pub const Audit_System: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x69979848_797a_11d9_bed3_505054503030);
pub const Audit_System_IPSecDriverEvents: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x0cce9213_69ae_11d9_bed3_505054503030);
pub const Audit_System_Integrity: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x0cce9212_69ae_11d9_bed3_505054503030);
pub const Audit_System_Others: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x0cce9214_69ae_11d9_bed3_505054503030);
pub const Audit_System_SecurityStateChange: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x0cce9210_69ae_11d9_bed3_505054503030);
pub const Audit_System_SecuritySubsystemExtension: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x0cce9211_69ae_11d9_bed3_505054503030);
pub const Batch: SECURITY_LOGON_TYPE = 4;
#[repr(C)]
#[cfg(all(feature = "Win32_lsalookup", feature = "Win32_minwindef", feature = "Win32_winnt"))]
#[derive(Clone, Copy)]
pub struct CENTRAL_ACCESS_POLICY {
    pub CAPID: super::winnt::PSID,
    pub Name: super::lsalookup::LSA_UNICODE_STRING,
    pub Description: super::lsalookup::LSA_UNICODE_STRING,
    pub ChangeId: super::lsalookup::LSA_UNICODE_STRING,
    pub Flags: u32,
    pub CAPECount: u32,
    pub CAPEs: *mut PCENTRAL_ACCESS_POLICY_ENTRY,
}
#[cfg(all(feature = "Win32_lsalookup", feature = "Win32_minwindef", feature = "Win32_winnt"))]
impl Default for CENTRAL_ACCESS_POLICY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_lsalookup", feature = "Win32_minwindef", feature = "Win32_winnt"))]
#[derive(Clone, Copy)]
pub struct CENTRAL_ACCESS_POLICY_ENTRY {
    pub Name: super::lsalookup::LSA_UNICODE_STRING,
    pub Description: super::lsalookup::LSA_UNICODE_STRING,
    pub ChangeId: super::lsalookup::LSA_UNICODE_STRING,
    pub LengthAppliesTo: u32,
    pub AppliesTo: super::minwindef::PUCHAR,
    pub LengthSD: u32,
    pub SD: super::winnt::PSECURITY_DESCRIPTOR,
    pub LengthStagedSD: u32,
    pub StagedSD: super::winnt::PSECURITY_DESCRIPTOR,
    pub Flags: u32,
}
#[cfg(all(feature = "Win32_lsalookup", feature = "Win32_minwindef", feature = "Win32_winnt"))]
impl Default for CENTRAL_ACCESS_POLICY_ENTRY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const CENTRAL_ACCESS_POLICY_OWNER_RIGHTS_PRESENT_FLAG: u32 = 1;
pub const CENTRAL_ACCESS_POLICY_STAGED_FLAG: u32 = 65536;
pub const CENTRAL_ACCESS_POLICY_STAGED_OWNER_RIGHTS_PRESENT_FLAG: u32 = 256;
pub const CENTRAL_ACCESS_POLICY_VALID_FLAG_MASK: u32 = 65793;
pub const CachedInteractive: SECURITY_LOGON_TYPE = 11;
pub const CachedRemoteInteractive: SECURITY_LOGON_TYPE = 12;
pub const CachedUnlock: SECURITY_LOGON_TYPE = 13;
pub const CertHashInfo: KERB_CERTIFICATE_INFO_TYPE = 1;
pub const CollisionOther: LSA_FOREST_TRUST_COLLISION_RECORD_TYPE = 2;
pub const CollisionTdo: LSA_FOREST_TRUST_COLLISION_RECORD_TYPE = 0;
pub const CollisionXref: LSA_FOREST_TRUST_COLLISION_RECORD_TYPE = 1;
pub const DOMAIN_LOCKOUT_ADMINS: u32 = 8;
pub const DOMAIN_NO_LM_OWF_CHANGE: u32 = 64;
pub const DOMAIN_PASSWORD_COMPLEX: u32 = 1;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct DOMAIN_PASSWORD_INFORMATION {
    pub MinPasswordLength: u16,
    pub PasswordHistoryLength: u16,
    pub PasswordProperties: u32,
    pub MaxPasswordAge: i64,
    pub MinPasswordAge: i64,
}
pub const DOMAIN_PASSWORD_NO_ANON_CHANGE: u32 = 2;
pub const DOMAIN_PASSWORD_NO_CLEAR_CHANGE: u32 = 4;
pub const DOMAIN_PASSWORD_STORE_CLEARTEXT: u32 = 16;
pub const DOMAIN_REFUSE_PASSWORD_CHANGE: u32 = 32;
pub const DS_UNKNOWN_ADDRESS_TYPE: u32 = 0;
pub const DeprecatedIUMCredKey: MSV1_0_CREDENTIAL_KEY_TYPE = 1;
pub const DomainUserCredKey: MSV1_0_CREDENTIAL_KEY_TYPE = 2;
pub const ExternallySuppliedCredKey: MSV1_0_CREDENTIAL_KEY_TYPE = 4;
pub const ForestTrustBinaryInfo: LSA_FOREST_TRUST_RECORD_TYPE = 3;
pub const ForestTrustDomainInfo: LSA_FOREST_TRUST_RECORD_TYPE = 2;
pub const ForestTrustRecordTypeLast: LSA_FOREST_TRUST_RECORD_TYPE = 4;
pub const ForestTrustScannerInfo: LSA_FOREST_TRUST_RECORD_TYPE = 4;
pub const ForestTrustTopLevelName: LSA_FOREST_TRUST_RECORD_TYPE = 0;
pub const ForestTrustTopLevelNameEx: LSA_FOREST_TRUST_RECORD_TYPE = 1;
pub const Interactive: SECURITY_LOGON_TYPE = 2;
pub const InvalidCredKey: MSV1_0_CREDENTIAL_KEY_TYPE = 0;
#[repr(C)]
#[cfg(all(feature = "Win32_lsalookup", feature = "Win32_winnt"))]
#[derive(Clone, Copy, Default)]
pub struct KDC_PROXY_CACHE_ENTRY_DATA {
    pub SinceLastUsed: u64,
    pub DomainName: UNICODE_STRING,
    pub ProxyServerName: UNICODE_STRING,
    pub ProxyServerVdir: UNICODE_STRING,
    pub ProxyServerPort: u16,
    pub LogonId: super::winnt::LUID,
    pub CredUserName: UNICODE_STRING,
    pub CredDomainName: UNICODE_STRING,
    pub GlobalCache: bool,
}
pub const KERBEROS_REVISION: u32 = 6;
pub const KERBEROS_VERSION: u32 = 5;
#[repr(C)]
#[cfg(feature = "Win32_lsalookup")]
#[derive(Clone, Copy, Default)]
pub struct KERB_ADD_BINDING_CACHE_ENTRY_EX_REQUEST {
    pub MessageType: KERB_PROTOCOL_MESSAGE_TYPE,
    pub RealmName: UNICODE_STRING,
    pub KdcAddress: UNICODE_STRING,
    pub AddressType: u32,
    pub DcFlags: u32,
}
#[repr(C)]
#[cfg(feature = "Win32_lsalookup")]
#[derive(Clone, Copy, Default)]
pub struct KERB_ADD_BINDING_CACHE_ENTRY_REQUEST {
    pub MessageType: KERB_PROTOCOL_MESSAGE_TYPE,
    pub RealmName: UNICODE_STRING,
    pub KdcAddress: UNICODE_STRING,
    pub AddressType: u32,
}
#[repr(C)]
#[cfg(all(feature = "Win32_lsalookup", feature = "Win32_winnt"))]
#[derive(Clone, Copy, Default)]
pub struct KERB_ADD_CREDENTIALS_REQUEST {
    pub MessageType: KERB_PROTOCOL_MESSAGE_TYPE,
    pub UserName: UNICODE_STRING,
    pub DomainName: UNICODE_STRING,
    pub Password: UNICODE_STRING,
    pub LogonId: super::winnt::LUID,
    pub Flags: u32,
}
#[repr(C)]
#[cfg(all(feature = "Win32_lsalookup", feature = "Win32_winnt"))]
#[derive(Clone, Copy)]
pub struct KERB_ADD_CREDENTIALS_REQUEST_EX {
    pub Credentials: KERB_ADD_CREDENTIALS_REQUEST,
    pub PrincipalNameCount: u32,
    pub PrincipalNames: [UNICODE_STRING; 1],
}
#[cfg(all(feature = "Win32_lsalookup", feature = "Win32_winnt"))]
impl Default for KERB_ADD_CREDENTIALS_REQUEST_EX {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_minwindef")]
#[derive(Clone, Copy)]
pub struct KERB_AUTH_DATA {
    pub Type: u32,
    pub Length: u32,
    pub Data: super::minwindef::PUCHAR,
}
#[cfg(feature = "Win32_minwindef")]
impl Default for KERB_AUTH_DATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_lsalookup")]
#[derive(Clone, Copy, Default)]
pub struct KERB_BINDING_CACHE_ENTRY_DATA {
    pub DiscoveryTime: u64,
    pub RealmName: UNICODE_STRING,
    pub KdcAddress: UNICODE_STRING,
    pub AddressType: u32,
    pub Flags: u32,
    pub DcFlags: u32,
    pub CacheFlags: u32,
    pub KdcName: UNICODE_STRING,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct KERB_CERTIFICATE_HASHINFO {
    pub StoreNameLength: u16,
    pub HashLength: u16,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct KERB_CERTIFICATE_INFO {
    pub CertInfoSize: u32,
    pub InfoType: u32,
}
pub type KERB_CERTIFICATE_INFO_TYPE = i32;
#[repr(C)]
#[cfg(all(feature = "Win32_lsalookup", feature = "Win32_minwindef"))]
#[derive(Clone, Copy)]
pub struct KERB_CERTIFICATE_LOGON {
    pub MessageType: KERB_LOGON_SUBMIT_TYPE,
    pub DomainName: UNICODE_STRING,
    pub UserName: UNICODE_STRING,
    pub Pin: UNICODE_STRING,
    pub Flags: u32,
    pub CspDataLength: u32,
    pub CspData: super::minwindef::PUCHAR,
}
#[cfg(all(feature = "Win32_lsalookup", feature = "Win32_minwindef"))]
impl Default for KERB_CERTIFICATE_LOGON {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const KERB_CERTIFICATE_LOGON_FLAG_CHECK_DUPLICATES: u32 = 1;
pub const KERB_CERTIFICATE_LOGON_FLAG_USE_CERTIFICATE_INFO: u32 = 2;
#[repr(C)]
#[cfg(all(feature = "Win32_lsalookup", feature = "Win32_minwindef"))]
#[derive(Clone, Copy)]
pub struct KERB_CERTIFICATE_S4U_LOGON {
    pub MessageType: KERB_LOGON_SUBMIT_TYPE,
    pub Flags: u32,
    pub UserPrincipalName: UNICODE_STRING,
    pub DomainName: UNICODE_STRING,
    pub CertificateLength: u32,
    pub Certificate: super::minwindef::PUCHAR,
}
#[cfg(all(feature = "Win32_lsalookup", feature = "Win32_minwindef"))]
impl Default for KERB_CERTIFICATE_S4U_LOGON {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const KERB_CERTIFICATE_S4U_LOGON_FLAG_CHECK_DUPLICATES: u32 = 1;
pub const KERB_CERTIFICATE_S4U_LOGON_FLAG_CHECK_LOGONHOURS: u32 = 2;
pub const KERB_CERTIFICATE_S4U_LOGON_FLAG_FAIL_IF_NT_AUTH_POLICY_REQUIRED: u32 = 4;
pub const KERB_CERTIFICATE_S4U_LOGON_FLAG_IDENTIFY: u32 = 8;
#[repr(C)]
#[cfg(all(feature = "Win32_lsalookup", feature = "Win32_minwindef", feature = "Win32_winnt"))]
#[derive(Clone, Copy, Default)]
pub struct KERB_CERTIFICATE_UNLOCK_LOGON {
    pub Logon: KERB_CERTIFICATE_LOGON,
    pub LogonId: super::winnt::LUID,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct KERB_CHANGEMACHINEPASSWORD_REQUEST {
    pub MessageType: KERB_PROTOCOL_MESSAGE_TYPE,
    pub ForcePasswordChange: bool,
}
#[repr(C)]
#[cfg(feature = "Win32_lsalookup")]
#[derive(Clone, Copy, Default)]
pub struct KERB_CHANGEPASSWORD_REQUEST {
    pub MessageType: KERB_PROTOCOL_MESSAGE_TYPE,
    pub DomainName: UNICODE_STRING,
    pub AccountName: UNICODE_STRING,
    pub OldPassword: UNICODE_STRING,
    pub NewPassword: UNICODE_STRING,
    pub Impersonating: bool,
}
pub const KERB_CHECKSUM_CRC32: u32 = 1;
pub const KERB_CHECKSUM_DES_MAC: i32 = -133;
pub const KERB_CHECKSUM_DES_MAC_MD5: i32 = -134;
pub const KERB_CHECKSUM_HMAC_MD5: i32 = -138;
pub const KERB_CHECKSUM_HMAC_SHA1_96_AES128: u32 = 15;
pub const KERB_CHECKSUM_HMAC_SHA1_96_AES128_Ki: i32 = -150;
pub const KERB_CHECKSUM_HMAC_SHA1_96_AES256: u32 = 16;
pub const KERB_CHECKSUM_HMAC_SHA1_96_AES256_Ki: i32 = -151;
pub const KERB_CHECKSUM_KRB_DES_MAC: u32 = 4;
pub const KERB_CHECKSUM_KRB_DES_MAC_K: u32 = 5;
pub const KERB_CHECKSUM_LM: i32 = -130;
pub const KERB_CHECKSUM_MD25: i32 = -135;
pub const KERB_CHECKSUM_MD4: u32 = 2;
pub const KERB_CHECKSUM_MD5: u32 = 7;
pub const KERB_CHECKSUM_MD5_DES: u32 = 8;
pub const KERB_CHECKSUM_MD5_HMAC: i32 = -137;
pub const KERB_CHECKSUM_NONE: u32 = 0;
pub const KERB_CHECKSUM_RC4_MD5: i32 = -136;
pub const KERB_CHECKSUM_REAL_CRC32: i32 = -132;
pub const KERB_CHECKSUM_SHA1: i32 = -131;
pub const KERB_CHECKSUM_SHA1_NEW: u32 = 14;
pub const KERB_CHECKSUM_SHA256: i32 = -139;
pub const KERB_CHECKSUM_SHA384: i32 = -140;
pub const KERB_CHECKSUM_SHA512: i32 = -141;
#[repr(C)]
#[cfg(feature = "Win32_winnt")]
#[derive(Clone, Copy, Default)]
pub struct KERB_CLEANUP_MACHINE_PKINIT_CREDS_REQUEST {
    pub MessageType: KERB_PROTOCOL_MESSAGE_TYPE,
    pub LogonId: super::winnt::LUID,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct KERB_CLOUD_KERBEROS_DEBUG_DATA {
    pub _bitfield: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct KERB_CLOUD_KERBEROS_DEBUG_DATA_V0 {
    pub _bitfield: u32,
}
pub const KERB_CLOUD_KERBEROS_DEBUG_DATA_VERSION: u32 = 1;
#[repr(C)]
#[cfg(feature = "Win32_winnt")]
#[derive(Clone, Copy, Default)]
pub struct KERB_CLOUD_KERBEROS_DEBUG_REQUEST {
    pub MessageType: KERB_PROTOCOL_MESSAGE_TYPE,
    pub LogonId: super::winnt::LUID,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct KERB_CLOUD_KERBEROS_DEBUG_RESPONSE {
    pub MessageType: KERB_PROTOCOL_MESSAGE_TYPE,
    pub Version: u32,
    pub Length: u32,
    pub Data: [u32; 1],
}
impl Default for KERB_CLOUD_KERBEROS_DEBUG_RESPONSE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_minwindef")]
#[derive(Clone, Copy)]
pub struct KERB_CRYPTO_KEY {
    pub KeyType: i32,
    pub Length: u32,
    pub Value: super::minwindef::PUCHAR,
}
#[cfg(feature = "Win32_minwindef")]
impl Default for KERB_CRYPTO_KEY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct KERB_CRYPTO_KEY32 {
    pub KeyType: i32,
    pub Length: u32,
    pub Offset: u32,
}
pub const KERB_DECRYPT_FLAG_DEFAULT_KEY: u32 = 1;
#[repr(C)]
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_winnt"))]
#[derive(Clone, Copy)]
pub struct KERB_DECRYPT_REQUEST {
    pub MessageType: KERB_PROTOCOL_MESSAGE_TYPE,
    pub LogonId: super::winnt::LUID,
    pub Flags: u32,
    pub CryptoType: i32,
    pub KeyUsage: i32,
    pub Key: KERB_CRYPTO_KEY,
    pub EncryptedDataSize: u32,
    pub InitialVectorSize: u32,
    pub InitialVector: super::minwindef::PUCHAR,
    pub EncryptedData: super::minwindef::PUCHAR,
}
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_winnt"))]
impl Default for KERB_DECRYPT_REQUEST {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct KERB_DECRYPT_RESPONSE {
    pub DecryptedData: [u8; 1],
}
impl Default for KERB_DECRYPT_RESPONSE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const KERB_ETYPE_AES128_CTS_HMAC_SHA1_96: u32 = 17;
pub const KERB_ETYPE_AES128_CTS_HMAC_SHA1_96_PLAIN: i32 = -148;
pub const KERB_ETYPE_AES128_CTS_HMAC_SHA256: u32 = 19;
pub const KERB_ETYPE_AES256_CTS_HMAC_SHA1_96: u32 = 18;
pub const KERB_ETYPE_AES256_CTS_HMAC_SHA1_96_PLAIN: i32 = -149;
pub const KERB_ETYPE_AES256_CTS_HMAC_SHA384: u32 = 20;
pub const KERB_ETYPE_DEFAULT: u32 = 0;
pub const KERB_ETYPE_DES3_CBC_MD5: u32 = 5;
pub const KERB_ETYPE_DES3_CBC_SHA1: u32 = 7;
pub const KERB_ETYPE_DES3_CBC_SHA1_KD: u32 = 16;
pub const KERB_ETYPE_DES_CBC_CRC: u32 = 1;
pub const KERB_ETYPE_DES_CBC_MD4: u32 = 2;
pub const KERB_ETYPE_DES_CBC_MD5: u32 = 3;
pub const KERB_ETYPE_DES_CBC_MD5_NT: u32 = 20;
pub const KERB_ETYPE_DES_EDE3_CBC_ENV: u32 = 15;
pub const KERB_ETYPE_DES_PLAIN: i32 = -132;
pub const KERB_ETYPE_DSA_SHA1_CMS: u32 = 9;
pub const KERB_ETYPE_DSA_SIGN: u32 = 8;
pub const KERB_ETYPE_NULL: u32 = 0;
pub const KERB_ETYPE_PKCS7_PUB: u32 = 13;
pub const KERB_ETYPE_RC2_CBC_ENV: u32 = 12;
pub const KERB_ETYPE_RC4_HMAC_NT: u32 = 23;
pub const KERB_ETYPE_RC4_HMAC_NT_EXP: u32 = 24;
pub const KERB_ETYPE_RC4_HMAC_OLD: i32 = -133;
pub const KERB_ETYPE_RC4_HMAC_OLD_EXP: i32 = -135;
pub const KERB_ETYPE_RC4_LM: i32 = -130;
pub const KERB_ETYPE_RC4_MD4: i32 = -128;
pub const KERB_ETYPE_RC4_PLAIN: i32 = -140;
pub const KERB_ETYPE_RC4_PLAIN2: i32 = -129;
pub const KERB_ETYPE_RC4_PLAIN_EXP: i32 = -141;
pub const KERB_ETYPE_RC4_PLAIN_OLD: i32 = -134;
pub const KERB_ETYPE_RC4_PLAIN_OLD_EXP: i32 = -136;
pub const KERB_ETYPE_RC4_SHA: i32 = -131;
pub const KERB_ETYPE_RSA_ENV: u32 = 13;
pub const KERB_ETYPE_RSA_ES_OEAP_ENV: u32 = 14;
pub const KERB_ETYPE_RSA_MD5_CMS: u32 = 10;
pub const KERB_ETYPE_RSA_PRIV: u32 = 9;
pub const KERB_ETYPE_RSA_PUB: u32 = 10;
pub const KERB_ETYPE_RSA_PUB_MD5: u32 = 11;
pub const KERB_ETYPE_RSA_PUB_SHA1: u32 = 12;
pub const KERB_ETYPE_RSA_SHA1_CMS: u32 = 11;
#[repr(C)]
#[cfg(feature = "Win32_lsalookup")]
#[derive(Clone, Copy)]
pub struct KERB_EXTERNAL_NAME {
    pub NameType: i16,
    pub NameCount: u16,
    pub Names: [UNICODE_STRING; 1],
}
#[cfg(feature = "Win32_lsalookup")]
impl Default for KERB_EXTERNAL_NAME {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_lsalookup", feature = "Win32_minwindef"))]
#[derive(Clone, Copy)]
pub struct KERB_EXTERNAL_TICKET {
    pub ServiceName: PKERB_EXTERNAL_NAME,
    pub TargetName: PKERB_EXTERNAL_NAME,
    pub ClientName: PKERB_EXTERNAL_NAME,
    pub DomainName: UNICODE_STRING,
    pub TargetDomainName: UNICODE_STRING,
    pub AltTargetDomainName: UNICODE_STRING,
    pub SessionKey: KERB_CRYPTO_KEY,
    pub TicketFlags: u32,
    pub Flags: u32,
    pub KeyExpirationTime: i64,
    pub StartTime: i64,
    pub EndTime: i64,
    pub RenewUntil: i64,
    pub TimeSkew: i64,
    pub EncodedTicketSize: u32,
    pub EncodedTicket: super::minwindef::PUCHAR,
}
#[cfg(all(feature = "Win32_lsalookup", feature = "Win32_minwindef"))]
impl Default for KERB_EXTERNAL_TICKET {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_lsalookup")]
#[derive(Clone, Copy, Default)]
pub struct KERB_INTERACTIVE_LOGON {
    pub MessageType: KERB_LOGON_SUBMIT_TYPE,
    pub LogonDomainName: UNICODE_STRING,
    pub UserName: UNICODE_STRING,
    pub Password: UNICODE_STRING,
}
#[repr(C)]
#[cfg(feature = "Win32_lsalookup")]
#[derive(Clone, Copy, Default)]
pub struct KERB_INTERACTIVE_PROFILE {
    pub MessageType: KERB_PROFILE_BUFFER_TYPE,
    pub LogonCount: u16,
    pub BadPasswordCount: u16,
    pub LogonTime: i64,
    pub LogoffTime: i64,
    pub KickOffTime: i64,
    pub PasswordLastSet: i64,
    pub PasswordCanChange: i64,
    pub PasswordMustChange: i64,
    pub LogonScript: UNICODE_STRING,
    pub HomeDirectory: UNICODE_STRING,
    pub FullName: UNICODE_STRING,
    pub ProfilePath: UNICODE_STRING,
    pub HomeDirectoryDrive: UNICODE_STRING,
    pub LogonServer: UNICODE_STRING,
    pub UserFlags: u32,
}
#[repr(C)]
#[cfg(all(feature = "Win32_lsalookup", feature = "Win32_winnt"))]
#[derive(Clone, Copy, Default)]
pub struct KERB_INTERACTIVE_UNLOCK_LOGON {
    pub Logon: KERB_INTERACTIVE_LOGON,
    pub LogonId: super::winnt::LUID,
}
pub const KERB_LOGON_FLAG_ALLOW_EXPIRED_TICKET: u32 = 1;
pub const KERB_LOGON_FLAG_REDIRECTED: u32 = 2;
pub type KERB_LOGON_SUBMIT_TYPE = i32;
#[repr(C)]
#[cfg(feature = "Win32_winnt")]
#[derive(Clone, Copy)]
pub struct KERB_NET_ADDRESS {
    pub Family: u32,
    pub Length: u32,
    pub Address: super::winnt::PCHAR,
}
#[cfg(feature = "Win32_winnt")]
impl Default for KERB_NET_ADDRESS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_winnt")]
#[derive(Clone, Copy)]
pub struct KERB_NET_ADDRESSES {
    pub Number: u32,
    pub Addresses: [KERB_NET_ADDRESS; 1],
}
#[cfg(feature = "Win32_winnt")]
impl Default for KERB_NET_ADDRESSES {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type KERB_PROFILE_BUFFER_TYPE = i32;
pub type KERB_PROTOCOL_MESSAGE_TYPE = i32;
pub const KERB_PURGE_ALL_TICKETS: u32 = 1;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct KERB_PURGE_BINDING_CACHE_REQUEST {
    pub MessageType: KERB_PROTOCOL_MESSAGE_TYPE,
}
#[repr(C)]
#[cfg(feature = "Win32_winnt")]
#[derive(Clone, Copy, Default)]
pub struct KERB_PURGE_KDC_PROXY_CACHE_REQUEST {
    pub MessageType: KERB_PROTOCOL_MESSAGE_TYPE,
    pub Flags: u32,
    pub LogonId: super::winnt::LUID,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct KERB_PURGE_KDC_PROXY_CACHE_RESPONSE {
    pub MessageType: KERB_PROTOCOL_MESSAGE_TYPE,
    pub CountOfPurged: u32,
}
#[repr(C)]
#[cfg(all(feature = "Win32_lsalookup", feature = "Win32_winnt"))]
#[derive(Clone, Copy, Default)]
pub struct KERB_PURGE_TKT_CACHE_EX_REQUEST {
    pub MessageType: KERB_PROTOCOL_MESSAGE_TYPE,
    pub LogonId: super::winnt::LUID,
    pub Flags: u32,
    pub TicketTemplate: KERB_TICKET_CACHE_INFO_EX,
}
#[repr(C)]
#[cfg(all(feature = "Win32_lsalookup", feature = "Win32_winnt"))]
#[derive(Clone, Copy, Default)]
pub struct KERB_PURGE_TKT_CACHE_REQUEST {
    pub MessageType: KERB_PROTOCOL_MESSAGE_TYPE,
    pub LogonId: super::winnt::LUID,
    pub ServerName: UNICODE_STRING,
    pub RealmName: UNICODE_STRING,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct KERB_QUERY_BINDING_CACHE_REQUEST {
    pub MessageType: KERB_PROTOCOL_MESSAGE_TYPE,
}
#[repr(C)]
#[cfg(feature = "Win32_lsalookup")]
#[derive(Clone, Copy)]
pub struct KERB_QUERY_BINDING_CACHE_RESPONSE {
    pub MessageType: KERB_PROTOCOL_MESSAGE_TYPE,
    pub CountOfEntries: u32,
    pub Entries: PKERB_BINDING_CACHE_ENTRY_DATA,
}
#[cfg(feature = "Win32_lsalookup")]
impl Default for KERB_QUERY_BINDING_CACHE_RESPONSE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_lsalookup")]
#[derive(Clone, Copy, Default)]
pub struct KERB_QUERY_DOMAIN_EXTENDED_POLICIES_REQUEST {
    pub MessageType: KERB_PROTOCOL_MESSAGE_TYPE,
    pub Flags: u32,
    pub DomainName: UNICODE_STRING,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct KERB_QUERY_DOMAIN_EXTENDED_POLICIES_RESPONSE {
    pub MessageType: KERB_PROTOCOL_MESSAGE_TYPE,
    pub Flags: u32,
    pub ExtendedPolicies: u32,
    pub DsFlags: u32,
}
pub const KERB_QUERY_DOMAIN_EXTENDED_POLICIES_RESPONSE_FLAG_DAC_DISABLED: u32 = 1;
#[repr(C)]
#[cfg(feature = "Win32_winnt")]
#[derive(Clone, Copy, Default)]
pub struct KERB_QUERY_KDC_PROXY_CACHE_REQUEST {
    pub MessageType: KERB_PROTOCOL_MESSAGE_TYPE,
    pub Flags: u32,
    pub LogonId: super::winnt::LUID,
}
#[repr(C)]
#[cfg(all(feature = "Win32_lsalookup", feature = "Win32_winnt"))]
#[derive(Clone, Copy)]
pub struct KERB_QUERY_KDC_PROXY_CACHE_RESPONSE {
    pub MessageType: KERB_PROTOCOL_MESSAGE_TYPE,
    pub CountOfEntries: u32,
    pub Entries: PKDC_PROXY_CACHE_ENTRY_DATA,
}
#[cfg(all(feature = "Win32_lsalookup", feature = "Win32_winnt"))]
impl Default for KERB_QUERY_KDC_PROXY_CACHE_RESPONSE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_winnt")]
#[derive(Clone, Copy, Default)]
pub struct KERB_QUERY_S4U2PROXY_CACHE_REQUEST {
    pub MessageType: KERB_PROTOCOL_MESSAGE_TYPE,
    pub Flags: u32,
    pub LogonId: super::winnt::LUID,
}
#[repr(C)]
#[cfg(all(feature = "Win32_bcrypt", feature = "Win32_lsalookup"))]
#[derive(Clone, Copy)]
pub struct KERB_QUERY_S4U2PROXY_CACHE_RESPONSE {
    pub MessageType: KERB_PROTOCOL_MESSAGE_TYPE,
    pub CountOfCreds: u32,
    pub Creds: PKERB_S4U2PROXY_CRED,
}
#[cfg(all(feature = "Win32_bcrypt", feature = "Win32_lsalookup"))]
impl Default for KERB_QUERY_S4U2PROXY_CACHE_RESPONSE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_lsalookup")]
#[derive(Clone, Copy)]
pub struct KERB_QUERY_TKT_CACHE_EX2_RESPONSE {
    pub MessageType: KERB_PROTOCOL_MESSAGE_TYPE,
    pub CountOfTickets: u32,
    pub Tickets: [KERB_TICKET_CACHE_INFO_EX2; 1],
}
#[cfg(feature = "Win32_lsalookup")]
impl Default for KERB_QUERY_TKT_CACHE_EX2_RESPONSE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_lsalookup")]
#[derive(Clone, Copy)]
pub struct KERB_QUERY_TKT_CACHE_EX3_RESPONSE {
    pub MessageType: KERB_PROTOCOL_MESSAGE_TYPE,
    pub CountOfTickets: u32,
    pub Tickets: [KERB_TICKET_CACHE_INFO_EX3; 1],
}
#[cfg(feature = "Win32_lsalookup")]
impl Default for KERB_QUERY_TKT_CACHE_EX3_RESPONSE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_lsalookup")]
#[derive(Clone, Copy)]
pub struct KERB_QUERY_TKT_CACHE_EX_RESPONSE {
    pub MessageType: KERB_PROTOCOL_MESSAGE_TYPE,
    pub CountOfTickets: u32,
    pub Tickets: [KERB_TICKET_CACHE_INFO_EX; 1],
}
#[cfg(feature = "Win32_lsalookup")]
impl Default for KERB_QUERY_TKT_CACHE_EX_RESPONSE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_winnt")]
#[derive(Clone, Copy, Default)]
pub struct KERB_QUERY_TKT_CACHE_REQUEST {
    pub MessageType: KERB_PROTOCOL_MESSAGE_TYPE,
    pub LogonId: super::winnt::LUID,
}
#[repr(C)]
#[cfg(feature = "Win32_lsalookup")]
#[derive(Clone, Copy)]
pub struct KERB_QUERY_TKT_CACHE_RESPONSE {
    pub MessageType: KERB_PROTOCOL_MESSAGE_TYPE,
    pub CountOfTickets: u32,
    pub Tickets: [KERB_TICKET_CACHE_INFO; 1],
}
#[cfg(feature = "Win32_lsalookup")]
impl Default for KERB_QUERY_TKT_CACHE_RESPONSE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const KERB_REFRESH_POLICY_KDC: u32 = 2;
pub const KERB_REFRESH_POLICY_KERBEROS: u32 = 1;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct KERB_REFRESH_POLICY_REQUEST {
    pub MessageType: KERB_PROTOCOL_MESSAGE_TYPE,
    pub Flags: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct KERB_REFRESH_POLICY_RESPONSE {
    pub MessageType: KERB_PROTOCOL_MESSAGE_TYPE,
    pub Flags: u32,
}
pub const KERB_REFRESH_SCCRED_GETTGT: u32 = 1;
pub const KERB_REFRESH_SCCRED_RELEASE: u32 = 0;
#[repr(C)]
#[cfg(all(feature = "Win32_lsalookup", feature = "Win32_winnt"))]
#[derive(Clone, Copy, Default)]
pub struct KERB_REFRESH_SCCRED_REQUEST {
    pub MessageType: KERB_PROTOCOL_MESSAGE_TYPE,
    pub CredentialBlob: UNICODE_STRING,
    pub LogonId: super::winnt::LUID,
    pub Flags: u32,
}
pub const KERB_REQUEST_ADD_CREDENTIAL: u32 = 1;
pub const KERB_REQUEST_CRED_LOCAL_ACCOUNT: u32 = 8;
pub const KERB_REQUEST_REMOVE_CREDENTIAL: u32 = 4;
pub const KERB_REQUEST_REPLACE_CREDENTIAL: u32 = 2;
#[repr(C)]
#[cfg(feature = "Win32_lsalookup")]
#[derive(Clone, Copy, Default)]
pub struct KERB_RETRIEVE_KEY_TAB_REQUEST {
    pub MessageType: KERB_PROTOCOL_MESSAGE_TYPE,
    pub Flags: u32,
    pub UserName: UNICODE_STRING,
    pub DomainName: UNICODE_STRING,
    pub Password: UNICODE_STRING,
}
#[repr(C)]
#[cfg(feature = "Win32_minwindef")]
#[derive(Clone, Copy)]
pub struct KERB_RETRIEVE_KEY_TAB_RESPONSE {
    pub MessageType: KERB_PROTOCOL_MESSAGE_TYPE,
    pub KeyTabLength: u32,
    pub KeyTab: super::minwindef::PUCHAR,
}
#[cfg(feature = "Win32_minwindef")]
impl Default for KERB_RETRIEVE_KEY_TAB_RESPONSE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const KERB_RETRIEVE_TICKET_AS_KERB_CRED: u32 = 8;
pub const KERB_RETRIEVE_TICKET_CACHE_TICKET: u32 = 32;
pub const KERB_RETRIEVE_TICKET_DEFAULT: u32 = 0;
pub const KERB_RETRIEVE_TICKET_DONT_USE_CACHE: u32 = 1;
pub const KERB_RETRIEVE_TICKET_MAX_LIFETIME: u32 = 64;
pub const KERB_RETRIEVE_TICKET_USE_CACHE_ONLY: u32 = 2;
pub const KERB_RETRIEVE_TICKET_USE_CREDHANDLE: u32 = 4;
pub const KERB_RETRIEVE_TICKET_WITH_SEC_CRED: u32 = 16;
#[repr(C)]
#[cfg(all(feature = "Win32_lsalookup", feature = "Win32_sspi", feature = "Win32_winnt"))]
#[derive(Clone, Copy, Default)]
pub struct KERB_RETRIEVE_TKT_REQUEST {
    pub MessageType: KERB_PROTOCOL_MESSAGE_TYPE,
    pub LogonId: super::winnt::LUID,
    pub TargetName: UNICODE_STRING,
    pub TicketFlags: u32,
    pub CacheOptions: u32,
    pub EncryptionType: i32,
    pub CredentialsHandle: super::sspi::SecHandle,
}
#[repr(C)]
#[cfg(all(feature = "Win32_lsalookup", feature = "Win32_minwindef"))]
#[derive(Clone, Copy, Default)]
pub struct KERB_RETRIEVE_TKT_RESPONSE {
    pub Ticket: KERB_EXTERNAL_TICKET,
}
#[repr(C)]
#[cfg(all(feature = "Win32_bcrypt", feature = "Win32_lsalookup"))]
#[derive(Clone, Copy, Default)]
pub struct KERB_S4U2PROXY_CACHE_ENTRY_INFO {
    pub ServerName: UNICODE_STRING,
    pub Flags: u32,
    pub LastStatus: super::bcrypt::NTSTATUS,
    pub Expiry: i64,
}
pub const KERB_S4U2PROXY_CACHE_ENTRY_INFO_FLAG_NEGATIVE: u32 = 1;
#[repr(C)]
#[cfg(all(feature = "Win32_bcrypt", feature = "Win32_lsalookup"))]
#[derive(Clone, Copy)]
pub struct KERB_S4U2PROXY_CRED {
    pub UserName: UNICODE_STRING,
    pub DomainName: UNICODE_STRING,
    pub Flags: u32,
    pub LastStatus: super::bcrypt::NTSTATUS,
    pub Expiry: i64,
    pub CountOfEntries: u32,
    pub Entries: PKERB_S4U2PROXY_CACHE_ENTRY_INFO,
}
#[cfg(all(feature = "Win32_bcrypt", feature = "Win32_lsalookup"))]
impl Default for KERB_S4U2PROXY_CRED {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const KERB_S4U2PROXY_CRED_FLAG_NEGATIVE: u32 = 1;
#[repr(C)]
#[cfg(feature = "Win32_lsalookup")]
#[derive(Clone, Copy, Default)]
pub struct KERB_S4U_LOGON {
    pub MessageType: KERB_LOGON_SUBMIT_TYPE,
    pub Flags: u32,
    pub ClientUpn: UNICODE_STRING,
    pub ClientRealm: UNICODE_STRING,
}
pub const KERB_S4U_LOGON_FLAG_CHECK_LOGONHOURS: u32 = 2;
pub const KERB_S4U_LOGON_FLAG_IDENTIFY: u32 = 8;
#[repr(C)]
#[cfg(all(feature = "Win32_lsalookup", feature = "Win32_sspi", feature = "Win32_winnt"))]
#[derive(Clone, Copy, Default)]
pub struct KERB_SETPASSWORD_EX_REQUEST {
    pub MessageType: KERB_PROTOCOL_MESSAGE_TYPE,
    pub LogonId: super::winnt::LUID,
    pub CredentialsHandle: super::sspi::SecHandle,
    pub Flags: u32,
    pub AccountRealm: UNICODE_STRING,
    pub AccountName: UNICODE_STRING,
    pub Password: UNICODE_STRING,
    pub ClientRealm: UNICODE_STRING,
    pub ClientName: UNICODE_STRING,
    pub Impersonating: bool,
    pub KdcAddress: UNICODE_STRING,
    pub KdcAddressType: u32,
}
#[repr(C)]
#[cfg(all(feature = "Win32_lsalookup", feature = "Win32_sspi", feature = "Win32_winnt"))]
#[derive(Clone, Copy, Default)]
pub struct KERB_SETPASSWORD_REQUEST {
    pub MessageType: KERB_PROTOCOL_MESSAGE_TYPE,
    pub LogonId: super::winnt::LUID,
    pub CredentialsHandle: super::sspi::SecHandle,
    pub Flags: u32,
    pub DomainName: UNICODE_STRING,
    pub AccountName: UNICODE_STRING,
    pub Password: UNICODE_STRING,
}
pub const KERB_SETPASS_USE_CREDHANDLE: u32 = 2;
pub const KERB_SETPASS_USE_LOGONID: u32 = 1;
#[repr(C)]
#[cfg(all(feature = "Win32_lsalookup", feature = "Win32_minwindef"))]
#[derive(Clone, Copy)]
pub struct KERB_SMART_CARD_LOGON {
    pub MessageType: KERB_LOGON_SUBMIT_TYPE,
    pub Pin: UNICODE_STRING,
    pub CspDataLength: u32,
    pub CspData: super::minwindef::PUCHAR,
}
#[cfg(all(feature = "Win32_lsalookup", feature = "Win32_minwindef"))]
impl Default for KERB_SMART_CARD_LOGON {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_lsalookup", feature = "Win32_minwindef"))]
#[derive(Clone, Copy)]
pub struct KERB_SMART_CARD_PROFILE {
    pub Profile: KERB_INTERACTIVE_PROFILE,
    pub CertificateSize: u32,
    pub CertificateData: super::minwindef::PUCHAR,
}
#[cfg(all(feature = "Win32_lsalookup", feature = "Win32_minwindef"))]
impl Default for KERB_SMART_CARD_PROFILE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_lsalookup", feature = "Win32_minwindef", feature = "Win32_winnt"))]
#[derive(Clone, Copy, Default)]
pub struct KERB_SMART_CARD_UNLOCK_LOGON {
    pub Logon: KERB_SMART_CARD_LOGON,
    pub LogonId: super::winnt::LUID,
}
#[repr(C)]
#[cfg(feature = "Win32_winnt")]
#[derive(Clone, Copy, Default)]
pub struct KERB_SUBMIT_TKT_REQUEST {
    pub MessageType: KERB_PROTOCOL_MESSAGE_TYPE,
    pub LogonId: super::winnt::LUID,
    pub Flags: u32,
    pub Key: KERB_CRYPTO_KEY32,
    pub KerbCredSize: u32,
    pub KerbCredOffset: u32,
}
#[repr(C)]
#[cfg(feature = "Win32_lsalookup")]
#[derive(Clone, Copy, Default)]
pub struct KERB_TICKET_CACHE_INFO {
    pub ServerName: UNICODE_STRING,
    pub RealmName: UNICODE_STRING,
    pub StartTime: i64,
    pub EndTime: i64,
    pub RenewTime: i64,
    pub EncryptionType: i32,
    pub TicketFlags: u32,
}
#[repr(C)]
#[cfg(feature = "Win32_lsalookup")]
#[derive(Clone, Copy, Default)]
pub struct KERB_TICKET_CACHE_INFO_EX {
    pub ClientName: UNICODE_STRING,
    pub ClientRealm: UNICODE_STRING,
    pub ServerName: UNICODE_STRING,
    pub ServerRealm: UNICODE_STRING,
    pub StartTime: i64,
    pub EndTime: i64,
    pub RenewTime: i64,
    pub EncryptionType: i32,
    pub TicketFlags: u32,
}
#[repr(C)]
#[cfg(feature = "Win32_lsalookup")]
#[derive(Clone, Copy, Default)]
pub struct KERB_TICKET_CACHE_INFO_EX2 {
    pub ClientName: UNICODE_STRING,
    pub ClientRealm: UNICODE_STRING,
    pub ServerName: UNICODE_STRING,
    pub ServerRealm: UNICODE_STRING,
    pub StartTime: i64,
    pub EndTime: i64,
    pub RenewTime: i64,
    pub EncryptionType: i32,
    pub TicketFlags: u32,
    pub SessionKeyType: u32,
    pub BranchId: u32,
}
#[repr(C)]
#[cfg(feature = "Win32_lsalookup")]
#[derive(Clone, Copy, Default)]
pub struct KERB_TICKET_CACHE_INFO_EX3 {
    pub ClientName: UNICODE_STRING,
    pub ClientRealm: UNICODE_STRING,
    pub ServerName: UNICODE_STRING,
    pub ServerRealm: UNICODE_STRING,
    pub StartTime: i64,
    pub EndTime: i64,
    pub RenewTime: i64,
    pub EncryptionType: i32,
    pub TicketFlags: u32,
    pub SessionKeyType: u32,
    pub BranchId: u32,
    pub CacheFlags: u32,
    pub KdcCalled: UNICODE_STRING,
}
pub const KERB_TICKET_FLAGS_enc_pa_rep: u32 = 65536;
pub const KERB_TICKET_FLAGS_forwardable: u32 = 1073741824;
pub const KERB_TICKET_FLAGS_forwarded: u32 = 536870912;
pub const KERB_TICKET_FLAGS_hw_authent: u32 = 1048576;
pub const KERB_TICKET_FLAGS_initial: u32 = 4194304;
pub const KERB_TICKET_FLAGS_invalid: u32 = 16777216;
pub const KERB_TICKET_FLAGS_may_postdate: u32 = 67108864;
pub const KERB_TICKET_FLAGS_name_canonicalize: u32 = 65536;
pub const KERB_TICKET_FLAGS_ok_as_delegate: u32 = 262144;
pub const KERB_TICKET_FLAGS_postdated: u32 = 33554432;
pub const KERB_TICKET_FLAGS_pre_authent: u32 = 2097152;
pub const KERB_TICKET_FLAGS_proxiable: u32 = 268435456;
pub const KERB_TICKET_FLAGS_proxy: u32 = 134217728;
pub const KERB_TICKET_FLAGS_renewable: u32 = 8388608;
pub const KERB_TICKET_FLAGS_reserved: u32 = 2147483648;
pub const KERB_TICKET_FLAGS_reserved1: u32 = 1;
#[repr(C)]
#[cfg(feature = "Win32_minwindef")]
#[derive(Clone, Copy)]
pub struct KERB_TICKET_LOGON {
    pub MessageType: KERB_LOGON_SUBMIT_TYPE,
    pub Flags: u32,
    pub ServiceTicketLength: u32,
    pub TicketGrantingTicketLength: u32,
    pub ServiceTicket: super::minwindef::PUCHAR,
    pub TicketGrantingTicket: super::minwindef::PUCHAR,
}
#[cfg(feature = "Win32_minwindef")]
impl Default for KERB_TICKET_LOGON {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_lsalookup", feature = "Win32_minwindef"))]
#[derive(Clone, Copy, Default)]
pub struct KERB_TICKET_PROFILE {
    pub Profile: KERB_INTERACTIVE_PROFILE,
    pub SessionKey: KERB_CRYPTO_KEY,
}
#[repr(C)]
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_winnt"))]
#[derive(Clone, Copy, Default)]
pub struct KERB_TICKET_UNLOCK_LOGON {
    pub Logon: KERB_TICKET_LOGON,
    pub LogonId: super::winnt::LUID,
}
pub const KERB_TRANSFER_CRED_CLEANUP_CREDENTIALS: u32 = 2;
#[repr(C)]
#[cfg(feature = "Win32_winnt")]
#[derive(Clone, Copy, Default)]
pub struct KERB_TRANSFER_CRED_REQUEST {
    pub MessageType: KERB_PROTOCOL_MESSAGE_TYPE,
    pub OriginLogonId: super::winnt::LUID,
    pub DestinationLogonId: super::winnt::LUID,
    pub Flags: u32,
}
pub const KERB_TRANSFER_CRED_WITH_TICKETS: u32 = 1;
pub const KERB_USE_DEFAULT_TICKET_FLAGS: u32 = 0;
pub const KERB_WRAP_NO_ENCRYPT: u32 = 2147483649;
pub const KRB_ANONYMOUS_STRING: windows_sys::core::PCWSTR = windows_sys::core::w!("ANONYMOUS");
pub const KRB_NT_ENTERPRISE_PRINCIPAL: u32 = 10;
pub const KRB_NT_ENT_PRINCIPAL_AND_ID: i32 = -130;
pub const KRB_NT_MS_BRANCH_ID: i32 = -133;
pub const KRB_NT_MS_PRINCIPAL: i32 = -128;
pub const KRB_NT_MS_PRINCIPAL_AND_ID: i32 = -129;
pub const KRB_NT_PRINCIPAL: u32 = 1;
pub const KRB_NT_PRINCIPAL_AND_ID: i32 = -131;
pub const KRB_NT_SRV_HST: u32 = 3;
pub const KRB_NT_SRV_INST: u32 = 2;
pub const KRB_NT_SRV_INST_AND_ID: i32 = -132;
pub const KRB_NT_SRV_XHST: u32 = 4;
pub const KRB_NT_UID: u32 = 5;
pub const KRB_NT_UNKNOWN: u32 = 0;
pub const KRB_NT_WELLKNOWN: u32 = 11;
pub const KRB_NT_X500_PRINCIPAL: u32 = 6;
pub const KRB_WELLKNOWN_STRING: windows_sys::core::PCWSTR = windows_sys::core::w!("WELLKNOWN");
pub const KerbAddBindingCacheEntryExMessage: KERB_PROTOCOL_MESSAGE_TYPE = 27;
pub const KerbAddBindingCacheEntryMessage: KERB_PROTOCOL_MESSAGE_TYPE = 10;
pub const KerbAddExtraCredentialsExMessage: KERB_PROTOCOL_MESSAGE_TYPE = 22;
pub const KerbAddExtraCredentialsMessage: KERB_PROTOCOL_MESSAGE_TYPE = 17;
pub const KerbCertificateLogon: KERB_LOGON_SUBMIT_TYPE = 13;
pub const KerbCertificateS4ULogon: KERB_LOGON_SUBMIT_TYPE = 14;
pub const KerbCertificateUnlockLogon: KERB_LOGON_SUBMIT_TYPE = 15;
pub const KerbChangeMachinePasswordMessage: KERB_PROTOCOL_MESSAGE_TYPE = 2;
pub const KerbChangePasswordMessage: KERB_PROTOCOL_MESSAGE_TYPE = 7;
pub const KerbCleanupMachinePkinitCredsMessage: KERB_PROTOCOL_MESSAGE_TYPE = 26;
pub const KerbDebugRequestMessage: KERB_PROTOCOL_MESSAGE_TYPE = 0;
pub const KerbDecryptDataMessage: KERB_PROTOCOL_MESSAGE_TYPE = 9;
pub const KerbInteractiveLogon: KERB_LOGON_SUBMIT_TYPE = 2;
pub const KerbInteractiveProfile: KERB_PROFILE_BUFFER_TYPE = 2;
pub const KerbLuidLogon: KERB_LOGON_SUBMIT_TYPE = 84;
pub const KerbNetworkTicketLogonMessage: KERB_PROTOCOL_MESSAGE_TYPE = 37;
pub const KerbNlChangeMachinePasswordMessage: KERB_PROTOCOL_MESSAGE_TYPE = 38;
pub const KerbNoElevationLogon: KERB_LOGON_SUBMIT_TYPE = 83;
pub const KerbPinKdcMessage: KERB_PROTOCOL_MESSAGE_TYPE = 30;
pub const KerbPrintCloudKerberosDebugMessage: KERB_PROTOCOL_MESSAGE_TYPE = 36;
pub const KerbProxyLogon: KERB_LOGON_SUBMIT_TYPE = 9;
pub const KerbPurgeBindingCacheMessage: KERB_PROTOCOL_MESSAGE_TYPE = 29;
pub const KerbPurgeKdcProxyCacheMessage: KERB_PROTOCOL_MESSAGE_TYPE = 24;
pub const KerbPurgeTicketCacheExMessage: KERB_PROTOCOL_MESSAGE_TYPE = 15;
pub const KerbPurgeTicketCacheMessage: KERB_PROTOCOL_MESSAGE_TYPE = 6;
pub const KerbQueryBindingCacheMessage: KERB_PROTOCOL_MESSAGE_TYPE = 28;
pub const KerbQueryDomainExtendedPoliciesMessage: KERB_PROTOCOL_MESSAGE_TYPE = 32;
pub const KerbQueryKdcProxyCacheMessage: KERB_PROTOCOL_MESSAGE_TYPE = 23;
pub const KerbQueryS4U2ProxyCacheMessage: KERB_PROTOCOL_MESSAGE_TYPE = 33;
pub const KerbQuerySupplementalCredentialsMessage: KERB_PROTOCOL_MESSAGE_TYPE = 18;
pub const KerbQueryTicketCacheEx2Message: KERB_PROTOCOL_MESSAGE_TYPE = 20;
pub const KerbQueryTicketCacheEx3Message: KERB_PROTOCOL_MESSAGE_TYPE = 25;
pub const KerbQueryTicketCacheExMessage: KERB_PROTOCOL_MESSAGE_TYPE = 14;
pub const KerbQueryTicketCacheMessage: KERB_PROTOCOL_MESSAGE_TYPE = 1;
pub const KerbRefreshPolicyMessage: KERB_PROTOCOL_MESSAGE_TYPE = 35;
pub const KerbRefreshSmartcardCredentialsMessage: KERB_PROTOCOL_MESSAGE_TYPE = 16;
pub const KerbRetrieveEncodedTicketMessage: KERB_PROTOCOL_MESSAGE_TYPE = 8;
pub const KerbRetrieveKeyTabMessage: KERB_PROTOCOL_MESSAGE_TYPE = 34;
pub const KerbRetrieveTicketMessage: KERB_PROTOCOL_MESSAGE_TYPE = 4;
pub const KerbS4ULogon: KERB_LOGON_SUBMIT_TYPE = 12;
pub const KerbSetPasswordExMessage: KERB_PROTOCOL_MESSAGE_TYPE = 12;
pub const KerbSetPasswordMessage: KERB_PROTOCOL_MESSAGE_TYPE = 11;
pub const KerbSmartCardLogon: KERB_LOGON_SUBMIT_TYPE = 6;
pub const KerbSmartCardProfile: KERB_PROFILE_BUFFER_TYPE = 4;
pub const KerbSmartCardUnlockLogon: KERB_LOGON_SUBMIT_TYPE = 8;
pub const KerbSubmitTicketMessage: KERB_PROTOCOL_MESSAGE_TYPE = 21;
pub const KerbTicketLogon: KERB_LOGON_SUBMIT_TYPE = 10;
pub const KerbTicketProfile: KERB_PROFILE_BUFFER_TYPE = 6;
pub const KerbTicketUnlockLogon: KERB_LOGON_SUBMIT_TYPE = 11;
pub const KerbTransferCredentialsMessage: KERB_PROTOCOL_MESSAGE_TYPE = 19;
pub const KerbUnpinAllKdcsMessage: KERB_PROTOCOL_MESSAGE_TYPE = 31;
pub const KerbUpdateAddressesMessage: KERB_PROTOCOL_MESSAGE_TYPE = 5;
pub const KerbVerifyCredentialsMessage: KERB_PROTOCOL_MESSAGE_TYPE = 13;
pub const KerbVerifyPacMessage: KERB_PROTOCOL_MESSAGE_TYPE = 3;
pub const KerbWorkstationUnlockLogon: KERB_LOGON_SUBMIT_TYPE = 7;
pub const LOGON_CACHED_ACCOUNT: u32 = 4;
pub const LOGON_EXTRA_SIDS: u32 = 32;
pub const LOGON_GRACE_LOGON: u32 = 16777216;
pub const LOGON_GUEST: u32 = 1;
pub const LOGON_LM_V2: u32 = 4096;
pub const LOGON_MANAGED_SERVICE: u32 = 524288;
pub const LOGON_NOENCRYPTION: u32 = 2;
pub const LOGON_NO_ELEVATION: u32 = 262144;
pub const LOGON_NO_OPTIMIZED: u32 = 131072;
pub const LOGON_NTLMV2_ENABLED: u32 = 256;
pub const LOGON_NTLM_V2: u32 = 8192;
pub const LOGON_NT_V2: u32 = 2048;
pub const LOGON_OPTIMIZED: u32 = 16384;
pub const LOGON_PKINIT: u32 = 65536;
pub const LOGON_PROFILE_PATH_RETURNED: u32 = 1024;
pub const LOGON_RESOURCE_GROUPS: u32 = 512;
pub const LOGON_SERVER_TRUST_ACCOUNT: u32 = 128;
pub const LOGON_SUBAUTH_SESSION_KEY: u32 = 64;
pub const LOGON_USED_LM_PASSWORD: u32 = 8;
pub const LOGON_WINLOGON: u32 = 32768;
pub type LPCAUDIT_POLICY_INFORMATION = PAUDIT_POLICY_INFORMATION;
pub const LSAD_AES_BLOCK_SIZE: u32 = 16;
pub const LSAD_AES_CRYPT_SHA512_HASH_SIZE: u32 = 64;
pub const LSAD_AES_KEY_SIZE: u32 = 16;
pub const LSAD_AES_SALT_SIZE: u32 = 16;
pub const LSASETCAPS_RELOAD_FLAG: u32 = 1;
pub const LSASETCAPS_VALID_FLAG_MASK: u32 = 1;
#[repr(C)]
#[cfg(feature = "Win32_minwindef")]
#[derive(Clone, Copy)]
pub struct LSA_AUTH_INFORMATION {
    pub LastUpdateTime: i64,
    pub AuthType: u32,
    pub AuthInfoLength: u32,
    pub AuthInfo: super::minwindef::PUCHAR,
}
#[cfg(feature = "Win32_minwindef")]
impl Default for LSA_AUTH_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type LSA_ENUMERATION_HANDLE = u32;
#[repr(C)]
#[cfg(feature = "Win32_winnt")]
#[derive(Clone, Copy)]
pub struct LSA_ENUMERATION_INFORMATION {
    pub Sid: super::winnt::PSID,
}
#[cfg(feature = "Win32_winnt")]
impl Default for LSA_ENUMERATION_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_minwindef")]
#[derive(Clone, Copy)]
pub struct LSA_FOREST_TRUST_BINARY_DATA {
    pub Length: u32,
    pub Buffer: super::minwindef::PUCHAR,
}
#[cfg(feature = "Win32_minwindef")]
impl Default for LSA_FOREST_TRUST_BINARY_DATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_lsalookup")]
#[derive(Clone, Copy)]
pub struct LSA_FOREST_TRUST_COLLISION_INFORMATION {
    pub RecordCount: u32,
    pub Entries: *mut PLSA_FOREST_TRUST_COLLISION_RECORD,
}
#[cfg(feature = "Win32_lsalookup")]
impl Default for LSA_FOREST_TRUST_COLLISION_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_lsalookup")]
#[derive(Clone, Copy, Default)]
pub struct LSA_FOREST_TRUST_COLLISION_RECORD {
    pub Index: u32,
    pub Type: LSA_FOREST_TRUST_COLLISION_RECORD_TYPE,
    pub Flags: u32,
    pub Name: super::lsalookup::LSA_UNICODE_STRING,
}
pub type LSA_FOREST_TRUST_COLLISION_RECORD_TYPE = i32;
#[repr(C)]
#[cfg(all(feature = "Win32_lsalookup", feature = "Win32_winnt"))]
#[derive(Clone, Copy)]
pub struct LSA_FOREST_TRUST_DOMAIN_INFO {
    pub Sid: super::winnt::PSID,
    pub DnsName: super::lsalookup::LSA_UNICODE_STRING,
    pub NetbiosName: super::lsalookup::LSA_UNICODE_STRING,
}
#[cfg(all(feature = "Win32_lsalookup", feature = "Win32_winnt"))]
impl Default for LSA_FOREST_TRUST_DOMAIN_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_lsalookup", feature = "Win32_minwindef", feature = "Win32_winnt"))]
#[derive(Clone, Copy)]
pub struct LSA_FOREST_TRUST_INFORMATION {
    pub RecordCount: u32,
    pub Entries: *mut PLSA_FOREST_TRUST_RECORD,
}
#[cfg(all(feature = "Win32_lsalookup", feature = "Win32_minwindef", feature = "Win32_winnt"))]
impl Default for LSA_FOREST_TRUST_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_lsalookup", feature = "Win32_minwindef", feature = "Win32_winnt"))]
#[derive(Clone, Copy)]
pub struct LSA_FOREST_TRUST_INFORMATION2 {
    pub RecordCount: u32,
    pub Entries: *mut PLSA_FOREST_TRUST_RECORD2,
}
#[cfg(all(feature = "Win32_lsalookup", feature = "Win32_minwindef", feature = "Win32_winnt"))]
impl Default for LSA_FOREST_TRUST_INFORMATION2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_lsalookup", feature = "Win32_minwindef", feature = "Win32_winnt"))]
#[derive(Clone, Copy)]
pub struct LSA_FOREST_TRUST_RECORD {
    pub Flags: u32,
    pub ForestTrustType: LSA_FOREST_TRUST_RECORD_TYPE,
    pub Time: i64,
    pub ForestTrustData: LSA_FOREST_TRUST_RECORD_0,
}
#[cfg(all(feature = "Win32_lsalookup", feature = "Win32_minwindef", feature = "Win32_winnt"))]
impl Default for LSA_FOREST_TRUST_RECORD {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_lsalookup", feature = "Win32_minwindef", feature = "Win32_winnt"))]
#[derive(Clone, Copy)]
pub union LSA_FOREST_TRUST_RECORD_0 {
    pub TopLevelName: super::lsalookup::LSA_UNICODE_STRING,
    pub DomainInfo: LSA_FOREST_TRUST_DOMAIN_INFO,
    pub Data: LSA_FOREST_TRUST_BINARY_DATA,
}
#[cfg(all(feature = "Win32_lsalookup", feature = "Win32_minwindef", feature = "Win32_winnt"))]
impl Default for LSA_FOREST_TRUST_RECORD_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_lsalookup", feature = "Win32_minwindef", feature = "Win32_winnt"))]
#[derive(Clone, Copy)]
pub struct LSA_FOREST_TRUST_RECORD2 {
    pub Flags: u32,
    pub ForestTrustType: LSA_FOREST_TRUST_RECORD_TYPE,
    pub Time: i64,
    pub ForestTrustData: LSA_FOREST_TRUST_RECORD2_0,
}
#[cfg(all(feature = "Win32_lsalookup", feature = "Win32_minwindef", feature = "Win32_winnt"))]
impl Default for LSA_FOREST_TRUST_RECORD2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_lsalookup", feature = "Win32_minwindef", feature = "Win32_winnt"))]
#[derive(Clone, Copy)]
pub union LSA_FOREST_TRUST_RECORD2_0 {
    pub TopLevelName: super::lsalookup::LSA_UNICODE_STRING,
    pub DomainInfo: LSA_FOREST_TRUST_DOMAIN_INFO,
    pub BinaryData: LSA_FOREST_TRUST_BINARY_DATA,
    pub ScannerInfo: LSA_FOREST_TRUST_SCANNER_INFO,
}
#[cfg(all(feature = "Win32_lsalookup", feature = "Win32_minwindef", feature = "Win32_winnt"))]
impl Default for LSA_FOREST_TRUST_RECORD2_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type LSA_FOREST_TRUST_RECORD_TYPE = i32;
#[repr(C)]
#[cfg(all(feature = "Win32_lsalookup", feature = "Win32_winnt"))]
#[derive(Clone, Copy)]
pub struct LSA_FOREST_TRUST_SCANNER_INFO {
    pub DomainSid: super::winnt::PSID,
    pub DnsName: super::lsalookup::LSA_UNICODE_STRING,
    pub NetbiosName: super::lsalookup::LSA_UNICODE_STRING,
}
#[cfg(all(feature = "Win32_lsalookup", feature = "Win32_winnt"))]
impl Default for LSA_FOREST_TRUST_SCANNER_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const LSA_FTRECORD_DISABLED_REASONS: u32 = 65535;
pub type LSA_HANDLE = *mut core::ffi::c_void;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct LSA_LAST_INTER_LOGON_INFO {
    pub LastSuccessfulLogon: i64,
    pub LastFailedLogon: i64,
    pub FailedAttemptCountSinceLastSuccessfulLogon: u32,
}
pub const LSA_MODE_INDIVIDUAL_ACCOUNTS: u32 = 2;
pub const LSA_MODE_LOG_FULL: u32 = 8;
pub const LSA_MODE_MANDATORY_ACCESS: u32 = 4;
pub const LSA_MODE_PASSWORD_PROTECTED: u32 = 1;
pub const LSA_NB_DISABLED_ADMIN: u32 = 4;
pub const LSA_NB_DISABLED_CONFLICT: u32 = 8;
pub type LSA_OPERATIONAL_MODE = u32;
pub const LSA_SCANNER_INFO_ADMIN_ALL_FLAGS: u32 = 1;
pub const LSA_SCANNER_INFO_DISABLE_AUTH_TARGET_VALIDATION: u32 = 1;
pub const LSA_SID_DISABLED_ADMIN: u32 = 1;
pub const LSA_SID_DISABLED_CONFLICT: u32 = 2;
pub const LSA_TLN_DISABLED_ADMIN: u32 = 2;
pub const LSA_TLN_DISABLED_CONFLICT: u32 = 4;
pub const LSA_TLN_DISABLED_NEW: u32 = 1;
#[repr(C)]
#[cfg(feature = "Win32_winnt")]
#[derive(Clone, Copy, Default)]
pub struct LSA_TRANSLATED_SID {
    pub Use: super::winnt::SID_NAME_USE,
    pub RelativeId: u32,
    pub DomainIndex: i32,
}
pub const LocalUserCredKey: MSV1_0_CREDENTIAL_KEY_TYPE = 3;
pub const MAXIMUM_CAPES_PER_CAP: u32 = 127;
pub const MAX_FOREST_TRUST_BINARY_DATA_SIZE: u32 = 131072;
pub const MAX_RECORDS_IN_FOREST_TRUST_INFO: u32 = 4000;
pub const MICROSOFT_KERBEROS_NAME_A: windows_sys::core::PCSTR = windows_sys::core::s!("Kerberos");
pub const MICROSOFT_KERBEROS_NAME_W: windows_sys::core::PCWSTR = windows_sys::core::w!("Kerberos");
pub const MSV1_0_ALLOW_FORCE_GUEST: u32 = 8192;
pub const MSV1_0_ALLOW_MSVCHAPV2: u32 = 65536;
pub const MSV1_0_ALLOW_SERVER_TRUST_ACCOUNT: u32 = 32;
pub const MSV1_0_ALLOW_WORKSTATION_TRUST_ACCOUNT: u32 = 2048;
pub type MSV1_0_AVID = i32;
pub const MSV1_0_AV_FLAG_FORCE_GUEST: u32 = 1;
pub const MSV1_0_AV_FLAG_MIC_HANDSHAKE_MESSAGES: u32 = 2;
pub const MSV1_0_AV_FLAG_UNVERIFIED_TARGET: u32 = 4;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct MSV1_0_AV_PAIR {
    pub AvId: u16,
    pub AvLen: u16,
}
pub const MSV1_0_CHALLENGE_LENGTH: u32 = 8;
#[repr(C)]
#[cfg(feature = "Win32_lsalookup")]
#[derive(Clone, Copy, Default)]
pub struct MSV1_0_CHANGEPASSWORD_REQUEST {
    pub MessageType: MSV1_0_PROTOCOL_MESSAGE_TYPE,
    pub DomainName: UNICODE_STRING,
    pub AccountName: UNICODE_STRING,
    pub OldPassword: UNICODE_STRING,
    pub NewPassword: UNICODE_STRING,
    pub Impersonating: bool,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct MSV1_0_CHANGEPASSWORD_RESPONSE {
    pub MessageType: MSV1_0_PROTOCOL_MESSAGE_TYPE,
    pub PasswordInfoValid: bool,
    pub DomainPasswordInfo: DOMAIN_PASSWORD_INFORMATION,
}
pub const MSV1_0_CHECK_LOGONHOURS_FOR_S4U: u32 = 262144;
pub const MSV1_0_CLEARTEXT_PASSWORD_ALLOWED: u32 = 2;
pub const MSV1_0_CLEARTEXT_PASSWORD_SUPPLIED: u32 = 16384;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct MSV1_0_CREDENTIAL_KEY {
    pub Data: [u8; 20],
}
impl Default for MSV1_0_CREDENTIAL_KEY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const MSV1_0_CREDENTIAL_KEY_LENGTH: u32 = 20;
pub type MSV1_0_CREDENTIAL_KEY_TYPE = i32;
pub const MSV1_0_CRED_CREDKEY_PRESENT: u32 = 8;
pub const MSV1_0_CRED_LM_PRESENT: u32 = 1;
pub const MSV1_0_CRED_NT_PRESENT: u32 = 2;
pub const MSV1_0_CRED_REMOVED: u32 = 4;
pub const MSV1_0_CRED_SHA_PRESENT: u32 = 16;
pub const MSV1_0_CRED_VERSION: u32 = 0;
pub const MSV1_0_CRED_VERSION_ARSO: u32 = 4294901763;
pub const MSV1_0_CRED_VERSION_INVALID: u32 = 4294967295;
pub const MSV1_0_CRED_VERSION_IUM: u32 = 4294901761;
pub const MSV1_0_CRED_VERSION_REMOTE: u32 = 4294901762;
pub const MSV1_0_CRED_VERSION_RESERVED_1: u32 = 4294967294;
pub const MSV1_0_CRED_VERSION_V2: u32 = 2;
pub const MSV1_0_CRED_VERSION_V3: u32 = 4;
pub const MSV1_0_DISABLE_PERSONAL_FALLBACK: u32 = 4096;
pub const MSV1_0_DONT_TRY_GUEST_ACCOUNT: u32 = 16;
#[repr(C)]
#[cfg(feature = "Win32_lsalookup")]
#[derive(Clone, Copy, Default)]
pub struct MSV1_0_INTERACTIVE_LOGON {
    pub MessageType: MSV1_0_LOGON_SUBMIT_TYPE,
    pub LogonDomainName: UNICODE_STRING,
    pub UserName: UNICODE_STRING,
    pub Password: UNICODE_STRING,
}
#[repr(C)]
#[cfg(feature = "Win32_lsalookup")]
#[derive(Clone, Copy, Default)]
pub struct MSV1_0_INTERACTIVE_PROFILE {
    pub MessageType: MSV1_0_PROFILE_BUFFER_TYPE,
    pub LogonCount: u16,
    pub BadPasswordCount: u16,
    pub LogonTime: i64,
    pub LogoffTime: i64,
    pub KickOffTime: i64,
    pub PasswordLastSet: i64,
    pub PasswordCanChange: i64,
    pub PasswordMustChange: i64,
    pub LogonScript: UNICODE_STRING,
    pub HomeDirectory: UNICODE_STRING,
    pub FullName: UNICODE_STRING,
    pub ProfilePath: UNICODE_STRING,
    pub HomeDirectoryDrive: UNICODE_STRING,
    pub LogonServer: UNICODE_STRING,
    pub UserFlags: u32,
}
pub const MSV1_0_INTERNET_DOMAIN: u32 = 524288;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct MSV1_0_IUM_SUPPLEMENTAL_CREDENTIAL {
    pub Version: u32,
    pub EncryptedCredsSize: u32,
    pub EncryptedCreds: [u8; 1],
}
impl Default for MSV1_0_IUM_SUPPLEMENTAL_CREDENTIAL {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const MSV1_0_LANMAN_SESSION_KEY_LENGTH: u32 = 8;
#[repr(C)]
#[cfg(all(feature = "Win32_lsalookup", feature = "Win32_winnt"))]
#[derive(Clone, Copy)]
pub struct MSV1_0_LM20_LOGON {
    pub MessageType: MSV1_0_LOGON_SUBMIT_TYPE,
    pub LogonDomainName: UNICODE_STRING,
    pub UserName: UNICODE_STRING,
    pub Workstation: UNICODE_STRING,
    pub ChallengeToClient: [u8; 8],
    pub CaseSensitiveChallengeResponse: STRING,
    pub CaseInsensitiveChallengeResponse: STRING,
    pub ParameterControl: u32,
}
#[cfg(all(feature = "Win32_lsalookup", feature = "Win32_winnt"))]
impl Default for MSV1_0_LM20_LOGON {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_lsalookup")]
#[derive(Clone, Copy)]
pub struct MSV1_0_LM20_LOGON_PROFILE {
    pub MessageType: MSV1_0_PROFILE_BUFFER_TYPE,
    pub KickOffTime: i64,
    pub LogoffTime: i64,
    pub UserFlags: u32,
    pub UserSessionKey: [u8; 16],
    pub LogonDomainName: UNICODE_STRING,
    pub LanmanSessionKey: [u8; 8],
    pub LogonServer: UNICODE_STRING,
    pub UserParameters: UNICODE_STRING,
}
#[cfg(feature = "Win32_lsalookup")]
impl Default for MSV1_0_LM20_LOGON_PROFILE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type MSV1_0_LOGON_SUBMIT_TYPE = i32;
pub const MSV1_0_MAX_AVL_SIZE: u32 = 64000;
pub const MSV1_0_MAX_NTLM3_LIFE: u32 = 129600;
pub const MSV1_0_MNS_LOGON: u32 = 16777216;
pub const MSV1_0_NTLM3_MIN_NT_RESPONSE_LENGTH: u32 = 44;
pub const MSV1_0_NTLM3_OWF_LENGTH: u32 = 16;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct MSV1_0_NTLM3_RESPONSE {
    pub Response: [u8; 16],
    pub RespType: u8,
    pub HiRespType: u8,
    pub Flags: u16,
    pub MsgWord: u32,
    pub TimeStamp: u64,
    pub ChallengeFromClient: [u8; 8],
    pub AvPairsOff: u32,
    pub Buffer: [u8; 1],
}
impl Default for MSV1_0_NTLM3_RESPONSE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const MSV1_0_NTLM3_RESPONSE_LENGTH: u32 = 16;
pub const MSV1_0_OWF_PASSWORD_LENGTH: u32 = 16;
pub const MSV1_0_PACKAGE_NAME: windows_sys::core::PCSTR = windows_sys::core::s!("MICROSOFT_AUTHENTICATION_PACKAGE_V1_0");
pub const MSV1_0_PACKAGE_NAMEW: windows_sys::core::PCWSTR = windows_sys::core::w!("MICROSOFT_AUTHENTICATION_PACKAGE_V1_0");
#[repr(C)]
#[cfg(all(feature = "Win32_lsalookup", feature = "Win32_minwindef"))]
#[derive(Clone, Copy)]
pub struct MSV1_0_PASSTHROUGH_REQUEST {
    pub MessageType: MSV1_0_PROTOCOL_MESSAGE_TYPE,
    pub DomainName: UNICODE_STRING,
    pub PackageName: UNICODE_STRING,
    pub DataLength: u32,
    pub LogonData: super::minwindef::PUCHAR,
    pub Pad: u32,
}
#[cfg(all(feature = "Win32_lsalookup", feature = "Win32_minwindef"))]
impl Default for MSV1_0_PASSTHROUGH_REQUEST {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_minwindef")]
#[derive(Clone, Copy)]
pub struct MSV1_0_PASSTHROUGH_RESPONSE {
    pub MessageType: MSV1_0_PROTOCOL_MESSAGE_TYPE,
    pub Pad: u32,
    pub DataLength: u32,
    pub ValidationData: super::minwindef::PUCHAR,
}
#[cfg(feature = "Win32_minwindef")]
impl Default for MSV1_0_PASSTHROUGH_RESPONSE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type MSV1_0_PROFILE_BUFFER_TYPE = i32;
pub type MSV1_0_PROTOCOL_MESSAGE_TYPE = i32;
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct MSV1_0_REMOTE_SUPPLEMENTAL_CREDENTIAL {
    pub Version: u32,
    pub Flags: u32,
    pub CredentialKey: MSV1_0_CREDENTIAL_KEY,
    pub CredentialKeyType: MSV1_0_CREDENTIAL_KEY_TYPE,
    pub EncryptedCredsSize: u32,
    pub EncryptedCreds: [u8; 1],
}
impl Default for MSV1_0_REMOTE_SUPPLEMENTAL_CREDENTIAL {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const MSV1_0_RETURN_PASSWORD_EXPIRY: u32 = 64;
pub const MSV1_0_RETURN_PROFILE_PATH: u32 = 512;
pub const MSV1_0_RETURN_USER_PARAMETERS: u32 = 8;
pub const MSV1_0_S4U2SELF: u32 = 131072;
#[repr(C)]
#[cfg(feature = "Win32_lsalookup")]
#[derive(Clone, Copy, Default)]
pub struct MSV1_0_S4U_LOGON {
    pub MessageType: MSV1_0_LOGON_SUBMIT_TYPE,
    pub Flags: u32,
    pub UserPrincipalName: UNICODE_STRING,
    pub DomainName: UNICODE_STRING,
}
pub const MSV1_0_S4U_LOGON_FLAG_CHECK_LOGONHOURS: u32 = 2;
pub const MSV1_0_SHA_PASSWORD_LENGTH: u32 = 20;
pub const MSV1_0_SUBAUTHENTICATION_DLL: u32 = 4278190080;
pub const MSV1_0_SUBAUTHENTICATION_DLL_EX: u32 = 1048576;
pub const MSV1_0_SUBAUTHENTICATION_DLL_IIS: u32 = 132;
pub const MSV1_0_SUBAUTHENTICATION_DLL_RAS: u32 = 2;
pub const MSV1_0_SUBAUTHENTICATION_DLL_SHIFT: u32 = 24;
pub const MSV1_0_SUBAUTHENTICATION_FLAGS: u32 = 4278190080;
pub const MSV1_0_SUBAUTHENTICATION_KEY: windows_sys::core::PCSTR = windows_sys::core::s!("SYSTEM\\CurrentControlSet\\Control\\Lsa\\MSV1_0");
pub const MSV1_0_SUBAUTHENTICATION_VALUE: windows_sys::core::PCSTR = windows_sys::core::s!("Auth");
#[repr(C)]
#[cfg(all(feature = "Win32_lsalookup", feature = "Win32_winnt"))]
#[derive(Clone, Copy)]
pub struct MSV1_0_SUBAUTH_LOGON {
    pub MessageType: MSV1_0_LOGON_SUBMIT_TYPE,
    pub LogonDomainName: UNICODE_STRING,
    pub UserName: UNICODE_STRING,
    pub Workstation: UNICODE_STRING,
    pub ChallengeToClient: [u8; 8],
    pub AuthenticationInfo1: STRING,
    pub AuthenticationInfo2: STRING,
    pub ParameterControl: u32,
    pub SubAuthPackageId: u32,
}
#[cfg(all(feature = "Win32_lsalookup", feature = "Win32_winnt"))]
impl Default for MSV1_0_SUBAUTH_LOGON {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_minwindef")]
#[derive(Clone, Copy)]
pub struct MSV1_0_SUBAUTH_REQUEST {
    pub MessageType: MSV1_0_PROTOCOL_MESSAGE_TYPE,
    pub SubAuthPackageId: u32,
    pub SubAuthInfoLength: u32,
    pub SubAuthSubmitBuffer: super::minwindef::PUCHAR,
}
#[cfg(feature = "Win32_minwindef")]
impl Default for MSV1_0_SUBAUTH_REQUEST {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_minwindef")]
#[derive(Clone, Copy)]
pub struct MSV1_0_SUBAUTH_RESPONSE {
    pub MessageType: MSV1_0_PROTOCOL_MESSAGE_TYPE,
    pub SubAuthInfoLength: u32,
    pub SubAuthReturnBuffer: super::minwindef::PUCHAR,
}
#[cfg(feature = "Win32_minwindef")]
impl Default for MSV1_0_SUBAUTH_RESPONSE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct MSV1_0_SUPPLEMENTAL_CREDENTIAL {
    pub Version: u32,
    pub Flags: u32,
    pub LmPassword: [u8; 16],
    pub NtPassword: [u8; 16],
}
impl Default for MSV1_0_SUPPLEMENTAL_CREDENTIAL {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct MSV1_0_SUPPLEMENTAL_CREDENTIAL_V2 {
    pub Version: u32,
    pub Flags: u32,
    pub NtPassword: [u8; 16],
    pub CredentialKey: MSV1_0_CREDENTIAL_KEY,
}
impl Default for MSV1_0_SUPPLEMENTAL_CREDENTIAL_V2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct MSV1_0_SUPPLEMENTAL_CREDENTIAL_V3 {
    pub Version: u32,
    pub Flags: u32,
    pub CredentialKeyType: MSV1_0_CREDENTIAL_KEY_TYPE,
    pub NtPassword: [u8; 16],
    pub CredentialKey: MSV1_0_CREDENTIAL_KEY,
    pub ShaPassword: [u8; 20],
}
impl Default for MSV1_0_SUPPLEMENTAL_CREDENTIAL_V3 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const MSV1_0_TRY_GUEST_ACCOUNT_ONLY: u32 = 256;
pub const MSV1_0_TRY_SPECIFIED_DOMAIN_ONLY: u32 = 1024;
pub const MSV1_0_UPDATE_LOGON_STATISTICS: u32 = 4;
pub const MSV1_0_USER_SESSION_KEY_LENGTH: u32 = 16;
pub const MSV1_0_USE_CLIENT_CHALLENGE: u32 = 128;
pub const MSV1_0_USE_DOMAIN_FOR_ROUTING_ONLY: u32 = 32768;
pub const MsV1_0CacheLogon: MSV1_0_PROTOCOL_MESSAGE_TYPE = 8;
pub const MsV1_0CacheLookup: MSV1_0_PROTOCOL_MESSAGE_TYPE = 11;
pub const MsV1_0CacheLookupEx: MSV1_0_PROTOCOL_MESSAGE_TYPE = 17;
pub const MsV1_0ChangeCachedPassword: MSV1_0_PROTOCOL_MESSAGE_TYPE = 6;
pub const MsV1_0ChangePassword: MSV1_0_PROTOCOL_MESSAGE_TYPE = 5;
pub const MsV1_0ClearCachedCredentials: MSV1_0_PROTOCOL_MESSAGE_TYPE = 14;
pub const MsV1_0ConfigLocalAliases: MSV1_0_PROTOCOL_MESSAGE_TYPE = 13;
pub const MsV1_0DecryptDpapiMasterKey: MSV1_0_PROTOCOL_MESSAGE_TYPE = 20;
pub const MsV1_0DeleteTbalSecrets: MSV1_0_PROTOCOL_MESSAGE_TYPE = 24;
pub const MsV1_0DeriveCredential: MSV1_0_PROTOCOL_MESSAGE_TYPE = 10;
pub const MsV1_0EnumerateUsers: MSV1_0_PROTOCOL_MESSAGE_TYPE = 2;
pub const MsV1_0GenericPassthrough: MSV1_0_PROTOCOL_MESSAGE_TYPE = 7;
pub const MsV1_0GetCredentialKey: MSV1_0_PROTOCOL_MESSAGE_TYPE = 18;
pub const MsV1_0GetStrongCredentialKey: MSV1_0_PROTOCOL_MESSAGE_TYPE = 21;
pub const MsV1_0GetUserInfo: MSV1_0_PROTOCOL_MESSAGE_TYPE = 3;
pub const MsV1_0InteractiveLogon: MSV1_0_LOGON_SUBMIT_TYPE = 2;
pub const MsV1_0InteractiveProfile: MSV1_0_PROFILE_BUFFER_TYPE = 2;
pub const MsV1_0Lm20ChallengeRequest: MSV1_0_PROTOCOL_MESSAGE_TYPE = 0;
pub const MsV1_0Lm20GetChallengeResponse: MSV1_0_PROTOCOL_MESSAGE_TYPE = 1;
pub const MsV1_0Lm20Logon: MSV1_0_LOGON_SUBMIT_TYPE = 3;
pub const MsV1_0Lm20LogonProfile: MSV1_0_PROFILE_BUFFER_TYPE = 3;
pub const MsV1_0LookupToken: MSV1_0_PROTOCOL_MESSAGE_TYPE = 15;
pub const MsV1_0LuidLogon: MSV1_0_LOGON_SUBMIT_TYPE = 84;
pub const MsV1_0NetworkLogon: MSV1_0_LOGON_SUBMIT_TYPE = 4;
pub const MsV1_0NoElevationLogon: MSV1_0_LOGON_SUBMIT_TYPE = 83;
pub const MsV1_0ProvisionTbal: MSV1_0_PROTOCOL_MESSAGE_TYPE = 23;
pub const MsV1_0ReLogonUsers: MSV1_0_PROTOCOL_MESSAGE_TYPE = 4;
pub const MsV1_0S4ULogon: MSV1_0_LOGON_SUBMIT_TYPE = 12;
pub const MsV1_0SetProcessOption: MSV1_0_PROTOCOL_MESSAGE_TYPE = 12;
pub const MsV1_0SetThreadOption: MSV1_0_PROTOCOL_MESSAGE_TYPE = 19;
pub const MsV1_0SmartCardProfile: MSV1_0_PROFILE_BUFFER_TYPE = 4;
pub const MsV1_0SubAuth: MSV1_0_PROTOCOL_MESSAGE_TYPE = 9;
pub const MsV1_0SubAuthLogon: MSV1_0_LOGON_SUBMIT_TYPE = 5;
pub const MsV1_0TransferCred: MSV1_0_PROTOCOL_MESSAGE_TYPE = 22;
pub const MsV1_0ValidateAuth: MSV1_0_PROTOCOL_MESSAGE_TYPE = 16;
pub const MsV1_0VirtualLogon: MSV1_0_LOGON_SUBMIT_TYPE = 82;
pub const MsV1_0WorkstationUnlockLogon: MSV1_0_LOGON_SUBMIT_TYPE = 7;
pub const MsvAvChannelBindings: MSV1_0_AVID = 10;
pub const MsvAvDnsComputerName: MSV1_0_AVID = 3;
pub const MsvAvDnsDomainName: MSV1_0_AVID = 4;
pub const MsvAvDnsTreeName: MSV1_0_AVID = 5;
pub const MsvAvEOL: MSV1_0_AVID = 0;
pub const MsvAvFlags: MSV1_0_AVID = 6;
pub const MsvAvNbComputerName: MSV1_0_AVID = 1;
pub const MsvAvNbDomainName: MSV1_0_AVID = 2;
pub const MsvAvRestrictions: MSV1_0_AVID = 8;
pub const MsvAvTargetName: MSV1_0_AVID = 9;
pub const MsvAvTimestamp: MSV1_0_AVID = 7;
#[repr(C)]
#[cfg(feature = "Win32_winnt")]
#[derive(Clone, Copy, Default)]
pub struct NEGOTIATE_CALLER_NAME_REQUEST {
    pub MessageType: u32,
    pub LogonId: super::winnt::LUID,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct NEGOTIATE_CALLER_NAME_RESPONSE {
    pub MessageType: u32,
    pub CallerName: windows_sys::core::PWSTR,
}
impl Default for NEGOTIATE_CALLER_NAME_RESPONSE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const NEGOTIATE_MAX_PREFIX: u32 = 32;
pub type NEGOTIATE_MESSAGES = i32;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct NEGOTIATE_PACKAGE_PREFIX {
    pub PackageId: usize,
    pub PackageDataA: *mut core::ffi::c_void,
    pub PackageDataW: *mut core::ffi::c_void,
    pub PrefixLen: usize,
    pub Prefix: [u8; 32],
}
impl Default for NEGOTIATE_PACKAGE_PREFIX {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct NEGOTIATE_PACKAGE_PREFIXES {
    pub MessageType: u32,
    pub PrefixCount: u32,
    pub Offset: u32,
    pub Pad: u32,
}
pub const NegCallPackageMax: NEGOTIATE_MESSAGES = 4;
pub const NegEnumPackagePrefixes: NEGOTIATE_MESSAGES = 0;
pub const NegGetCallerName: NEGOTIATE_MESSAGES = 1;
pub const NegMsgReserved1: NEGOTIATE_MESSAGES = 3;
pub const NegTransferCredentials: NEGOTIATE_MESSAGES = 2;
pub const Network: SECURITY_LOGON_TYPE = 3;
pub const NetworkCleartext: SECURITY_LOGON_TYPE = 8;
pub const NewCredentials: SECURITY_LOGON_TYPE = 9;
pub type PAUDIT_POLICY_INFORMATION = *mut AUDIT_POLICY_INFORMATION;
pub type PCAUDIT_POLICY_INFORMATION = PAUDIT_POLICY_INFORMATION;
#[cfg(all(feature = "Win32_lsalookup", feature = "Win32_minwindef", feature = "Win32_winnt"))]
pub type PCCENTRAL_ACCESS_POLICY = *const CENTRAL_ACCESS_POLICY;
#[cfg(all(feature = "Win32_lsalookup", feature = "Win32_minwindef", feature = "Win32_winnt"))]
pub type PCCENTRAL_ACCESS_POLICY_ENTRY = *const CENTRAL_ACCESS_POLICY_ENTRY;
#[cfg(all(feature = "Win32_lsalookup", feature = "Win32_minwindef", feature = "Win32_winnt"))]
pub type PCENTRAL_ACCESS_POLICY = *mut CENTRAL_ACCESS_POLICY;
#[cfg(all(feature = "Win32_lsalookup", feature = "Win32_minwindef", feature = "Win32_winnt"))]
pub type PCENTRAL_ACCESS_POLICY_ENTRY = *mut CENTRAL_ACCESS_POLICY_ENTRY;
pub type PDOMAIN_PASSWORD_INFORMATION = *mut DOMAIN_PASSWORD_INFORMATION;
pub const PER_USER_AUDIT_FAILURE_EXCLUDE: u32 = 8;
pub const PER_USER_AUDIT_FAILURE_INCLUDE: u32 = 4;
pub const PER_USER_AUDIT_NONE: u32 = 16;
pub const PER_USER_AUDIT_SUCCESS_EXCLUDE: u32 = 2;
pub const PER_USER_AUDIT_SUCCESS_INCLUDE: u32 = 1;
pub const PER_USER_POLICY_UNCHANGED: u32 = 0;
#[cfg(all(feature = "Win32_lsalookup", feature = "Win32_winnt"))]
pub type PKDC_PROXY_CACHE_ENTRY_DATA = *mut KDC_PROXY_CACHE_ENTRY_DATA;
#[cfg(feature = "Win32_lsalookup")]
pub type PKERB_ADD_BINDING_CACHE_ENTRY_EX_REQUEST = *mut KERB_ADD_BINDING_CACHE_ENTRY_EX_REQUEST;
#[cfg(feature = "Win32_lsalookup")]
pub type PKERB_ADD_BINDING_CACHE_ENTRY_REQUEST = *mut KERB_ADD_BINDING_CACHE_ENTRY_REQUEST;
#[cfg(all(feature = "Win32_lsalookup", feature = "Win32_winnt"))]
pub type PKERB_ADD_CREDENTIALS_REQUEST = *mut KERB_ADD_CREDENTIALS_REQUEST;
#[cfg(all(feature = "Win32_lsalookup", feature = "Win32_winnt"))]
pub type PKERB_ADD_CREDENTIALS_REQUEST_EX = *mut KERB_ADD_CREDENTIALS_REQUEST_EX;
#[cfg(feature = "Win32_minwindef")]
pub type PKERB_AUTH_DATA = *mut KERB_AUTH_DATA;
#[cfg(feature = "Win32_lsalookup")]
pub type PKERB_BINDING_CACHE_ENTRY_DATA = *mut KERB_BINDING_CACHE_ENTRY_DATA;
pub type PKERB_CERTIFICATE_HASHINFO = *mut KERB_CERTIFICATE_HASHINFO;
pub type PKERB_CERTIFICATE_INFO = *mut KERB_CERTIFICATE_INFO;
pub type PKERB_CERTIFICATE_INFO_TYPE = *mut KERB_CERTIFICATE_INFO_TYPE;
#[cfg(all(feature = "Win32_lsalookup", feature = "Win32_minwindef"))]
pub type PKERB_CERTIFICATE_LOGON = *mut KERB_CERTIFICATE_LOGON;
#[cfg(all(feature = "Win32_lsalookup", feature = "Win32_minwindef"))]
pub type PKERB_CERTIFICATE_S4U_LOGON = *mut KERB_CERTIFICATE_S4U_LOGON;
#[cfg(all(feature = "Win32_lsalookup", feature = "Win32_minwindef", feature = "Win32_winnt"))]
pub type PKERB_CERTIFICATE_UNLOCK_LOGON = *mut KERB_CERTIFICATE_UNLOCK_LOGON;
pub type PKERB_CHANGEMACHINEPASSWORD_REQUEST = *mut KERB_CHANGEMACHINEPASSWORD_REQUEST;
#[cfg(feature = "Win32_lsalookup")]
pub type PKERB_CHANGEPASSWORD_REQUEST = *mut KERB_CHANGEPASSWORD_REQUEST;
#[cfg(feature = "Win32_winnt")]
pub type PKERB_CLEANUP_MACHINE_PKINIT_CREDS_REQUEST = *mut KERB_CLEANUP_MACHINE_PKINIT_CREDS_REQUEST;
pub type PKERB_CLOUD_KERBEROS_DEBUG_DATA = *mut KERB_CLOUD_KERBEROS_DEBUG_DATA;
pub type PKERB_CLOUD_KERBEROS_DEBUG_DATA_V0 = *mut KERB_CLOUD_KERBEROS_DEBUG_DATA_V0;
#[cfg(feature = "Win32_winnt")]
pub type PKERB_CLOUD_KERBEROS_DEBUG_REQUEST = *mut KERB_CLOUD_KERBEROS_DEBUG_REQUEST;
pub type PKERB_CLOUD_KERBEROS_DEBUG_RESPONSE = *mut KERB_CLOUD_KERBEROS_DEBUG_RESPONSE;
#[cfg(feature = "Win32_minwindef")]
pub type PKERB_CRYPTO_KEY = *mut KERB_CRYPTO_KEY;
pub type PKERB_CRYPTO_KEY32 = *mut KERB_CRYPTO_KEY32;
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_winnt"))]
pub type PKERB_DECRYPT_REQUEST = *mut KERB_DECRYPT_REQUEST;
pub type PKERB_DECRYPT_RESPONSE = *mut KERB_DECRYPT_RESPONSE;
#[cfg(feature = "Win32_lsalookup")]
pub type PKERB_EXTERNAL_NAME = *mut KERB_EXTERNAL_NAME;
#[cfg(all(feature = "Win32_lsalookup", feature = "Win32_minwindef"))]
pub type PKERB_EXTERNAL_TICKET = *mut KERB_EXTERNAL_TICKET;
#[cfg(feature = "Win32_lsalookup")]
pub type PKERB_INTERACTIVE_LOGON = *mut KERB_INTERACTIVE_LOGON;
#[cfg(feature = "Win32_lsalookup")]
pub type PKERB_INTERACTIVE_PROFILE = *mut KERB_INTERACTIVE_PROFILE;
#[cfg(all(feature = "Win32_lsalookup", feature = "Win32_winnt"))]
pub type PKERB_INTERACTIVE_UNLOCK_LOGON = *mut KERB_INTERACTIVE_UNLOCK_LOGON;
pub type PKERB_LOGON_SUBMIT_TYPE = *mut KERB_LOGON_SUBMIT_TYPE;
#[cfg(feature = "Win32_winnt")]
pub type PKERB_NET_ADDRESS = *mut KERB_NET_ADDRESS;
#[cfg(feature = "Win32_winnt")]
pub type PKERB_NET_ADDRESSES = *mut KERB_NET_ADDRESSES;
pub type PKERB_PROFILE_BUFFER_TYPE = *mut KERB_PROFILE_BUFFER_TYPE;
pub type PKERB_PROTOCOL_MESSAGE_TYPE = *mut KERB_PROTOCOL_MESSAGE_TYPE;
pub type PKERB_PURGE_BINDING_CACHE_REQUEST = *mut KERB_PURGE_BINDING_CACHE_REQUEST;
#[cfg(feature = "Win32_winnt")]
pub type PKERB_PURGE_KDC_PROXY_CACHE_REQUEST = *mut KERB_PURGE_KDC_PROXY_CACHE_REQUEST;
pub type PKERB_PURGE_KDC_PROXY_CACHE_RESPONSE = *mut KERB_PURGE_KDC_PROXY_CACHE_RESPONSE;
#[cfg(all(feature = "Win32_lsalookup", feature = "Win32_winnt"))]
pub type PKERB_PURGE_TKT_CACHE_EX_REQUEST = *mut KERB_PURGE_TKT_CACHE_EX_REQUEST;
#[cfg(all(feature = "Win32_lsalookup", feature = "Win32_winnt"))]
pub type PKERB_PURGE_TKT_CACHE_REQUEST = *mut KERB_PURGE_TKT_CACHE_REQUEST;
pub type PKERB_QUERY_BINDING_CACHE_REQUEST = *mut KERB_QUERY_BINDING_CACHE_REQUEST;
#[cfg(feature = "Win32_lsalookup")]
pub type PKERB_QUERY_BINDING_CACHE_RESPONSE = *mut KERB_QUERY_BINDING_CACHE_RESPONSE;
#[cfg(feature = "Win32_lsalookup")]
pub type PKERB_QUERY_DOMAIN_EXTENDED_POLICIES_REQUEST = *mut KERB_QUERY_DOMAIN_EXTENDED_POLICIES_REQUEST;
pub type PKERB_QUERY_DOMAIN_EXTENDED_POLICIES_RESPONSE = *mut KERB_QUERY_DOMAIN_EXTENDED_POLICIES_RESPONSE;
#[cfg(feature = "Win32_winnt")]
pub type PKERB_QUERY_KDC_PROXY_CACHE_REQUEST = *mut KERB_QUERY_KDC_PROXY_CACHE_REQUEST;
#[cfg(all(feature = "Win32_lsalookup", feature = "Win32_winnt"))]
pub type PKERB_QUERY_KDC_PROXY_CACHE_RESPONSE = *mut KERB_QUERY_KDC_PROXY_CACHE_RESPONSE;
#[cfg(feature = "Win32_winnt")]
pub type PKERB_QUERY_S4U2PROXY_CACHE_REQUEST = *mut KERB_QUERY_S4U2PROXY_CACHE_REQUEST;
#[cfg(all(feature = "Win32_bcrypt", feature = "Win32_lsalookup"))]
pub type PKERB_QUERY_S4U2PROXY_CACHE_RESPONSE = *mut KERB_QUERY_S4U2PROXY_CACHE_RESPONSE;
#[cfg(feature = "Win32_lsalookup")]
pub type PKERB_QUERY_TKT_CACHE_EX2_RESPONSE = *mut KERB_QUERY_TKT_CACHE_EX2_RESPONSE;
#[cfg(feature = "Win32_lsalookup")]
pub type PKERB_QUERY_TKT_CACHE_EX3_RESPONSE = *mut KERB_QUERY_TKT_CACHE_EX3_RESPONSE;
#[cfg(feature = "Win32_lsalookup")]
pub type PKERB_QUERY_TKT_CACHE_EX_RESPONSE = *mut KERB_QUERY_TKT_CACHE_EX_RESPONSE;
#[cfg(feature = "Win32_winnt")]
pub type PKERB_QUERY_TKT_CACHE_REQUEST = *mut KERB_QUERY_TKT_CACHE_REQUEST;
#[cfg(feature = "Win32_lsalookup")]
pub type PKERB_QUERY_TKT_CACHE_RESPONSE = *mut KERB_QUERY_TKT_CACHE_RESPONSE;
pub type PKERB_REFRESH_POLICY_REQUEST = *mut KERB_REFRESH_POLICY_REQUEST;
pub type PKERB_REFRESH_POLICY_RESPONSE = *mut KERB_REFRESH_POLICY_RESPONSE;
#[cfg(all(feature = "Win32_lsalookup", feature = "Win32_winnt"))]
pub type PKERB_REFRESH_SCCRED_REQUEST = *mut KERB_REFRESH_SCCRED_REQUEST;
#[cfg(feature = "Win32_lsalookup")]
pub type PKERB_RETRIEVE_KEY_TAB_REQUEST = *mut KERB_RETRIEVE_KEY_TAB_REQUEST;
#[cfg(feature = "Win32_minwindef")]
pub type PKERB_RETRIEVE_KEY_TAB_RESPONSE = *mut KERB_RETRIEVE_KEY_TAB_RESPONSE;
#[cfg(all(feature = "Win32_lsalookup", feature = "Win32_sspi", feature = "Win32_winnt"))]
pub type PKERB_RETRIEVE_TKT_REQUEST = *mut KERB_RETRIEVE_TKT_REQUEST;
#[cfg(all(feature = "Win32_lsalookup", feature = "Win32_minwindef"))]
pub type PKERB_RETRIEVE_TKT_RESPONSE = *mut KERB_RETRIEVE_TKT_RESPONSE;
#[cfg(all(feature = "Win32_bcrypt", feature = "Win32_lsalookup"))]
pub type PKERB_S4U2PROXY_CACHE_ENTRY_INFO = *mut KERB_S4U2PROXY_CACHE_ENTRY_INFO;
#[cfg(all(feature = "Win32_bcrypt", feature = "Win32_lsalookup"))]
pub type PKERB_S4U2PROXY_CRED = *mut KERB_S4U2PROXY_CRED;
#[cfg(feature = "Win32_lsalookup")]
pub type PKERB_S4U_LOGON = *mut KERB_S4U_LOGON;
#[cfg(all(feature = "Win32_lsalookup", feature = "Win32_sspi", feature = "Win32_winnt"))]
pub type PKERB_SETPASSWORD_EX_REQUEST = *mut KERB_SETPASSWORD_EX_REQUEST;
#[cfg(all(feature = "Win32_lsalookup", feature = "Win32_sspi", feature = "Win32_winnt"))]
pub type PKERB_SETPASSWORD_REQUEST = *mut KERB_SETPASSWORD_REQUEST;
#[cfg(all(feature = "Win32_lsalookup", feature = "Win32_minwindef"))]
pub type PKERB_SMART_CARD_LOGON = *mut KERB_SMART_CARD_LOGON;
#[cfg(all(feature = "Win32_lsalookup", feature = "Win32_minwindef"))]
pub type PKERB_SMART_CARD_PROFILE = *mut KERB_SMART_CARD_PROFILE;
#[cfg(all(feature = "Win32_lsalookup", feature = "Win32_minwindef", feature = "Win32_winnt"))]
pub type PKERB_SMART_CARD_UNLOCK_LOGON = *mut KERB_SMART_CARD_UNLOCK_LOGON;
#[cfg(feature = "Win32_winnt")]
pub type PKERB_SUBMIT_TKT_REQUEST = *mut KERB_SUBMIT_TKT_REQUEST;
#[cfg(feature = "Win32_lsalookup")]
pub type PKERB_TICKET_CACHE_INFO = *mut KERB_TICKET_CACHE_INFO;
#[cfg(feature = "Win32_lsalookup")]
pub type PKERB_TICKET_CACHE_INFO_EX = *mut KERB_TICKET_CACHE_INFO_EX;
#[cfg(feature = "Win32_lsalookup")]
pub type PKERB_TICKET_CACHE_INFO_EX2 = *mut KERB_TICKET_CACHE_INFO_EX2;
#[cfg(feature = "Win32_lsalookup")]
pub type PKERB_TICKET_CACHE_INFO_EX3 = *mut KERB_TICKET_CACHE_INFO_EX3;
#[cfg(feature = "Win32_minwindef")]
pub type PKERB_TICKET_LOGON = *mut KERB_TICKET_LOGON;
#[cfg(all(feature = "Win32_lsalookup", feature = "Win32_minwindef"))]
pub type PKERB_TICKET_PROFILE = *mut KERB_TICKET_PROFILE;
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_winnt"))]
pub type PKERB_TICKET_UNLOCK_LOGON = *mut KERB_TICKET_UNLOCK_LOGON;
#[cfg(feature = "Win32_winnt")]
pub type PKERB_TRANSFER_CRED_REQUEST = *mut KERB_TRANSFER_CRED_REQUEST;
#[repr(C)]
#[cfg(all(feature = "Win32_lsalookup", feature = "Win32_minwindef"))]
#[derive(Clone, Copy)]
pub struct PKU2U_CERTIFICATE_S4U_LOGON {
    pub MessageType: PKU2U_LOGON_SUBMIT_TYPE,
    pub Flags: u32,
    pub UserPrincipalName: UNICODE_STRING,
    pub DomainName: UNICODE_STRING,
    pub CertificateLength: u32,
    pub Certificate: super::minwindef::PUCHAR,
}
#[cfg(all(feature = "Win32_lsalookup", feature = "Win32_minwindef"))]
impl Default for PKU2U_CERTIFICATE_S4U_LOGON {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct PKU2U_CERT_BLOB {
    pub CertOffset: u32,
    pub CertLength: u16,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct PKU2U_CREDUI_CONTEXT {
    pub Version: u64,
    pub cbHeaderLength: u16,
    pub cbStructureLength: u32,
    pub CertArrayCount: u16,
    pub CertArrayOffset: u32,
}
pub const PKU2U_CREDUI_CONTEXT_VERSION: u32 = 1414677827;
pub type PKU2U_LOGON_SUBMIT_TYPE = i32;
pub const PKU2U_PACKAGE_NAME: windows_sys::core::PCWSTR = windows_sys::core::w!("pku2u");
pub const PKU2U_PACKAGE_NAME_A: windows_sys::core::PCSTR = windows_sys::core::s!("pku2u");
#[cfg(feature = "Win32_minwindef")]
pub type PLSA_AUTH_INFORMATION = *mut LSA_AUTH_INFORMATION;
pub type PLSA_ENUMERATION_HANDLE = *mut u32;
#[cfg(feature = "Win32_winnt")]
pub type PLSA_ENUMERATION_INFORMATION = *mut LSA_ENUMERATION_INFORMATION;
#[cfg(feature = "Win32_minwindef")]
pub type PLSA_FOREST_TRUST_BINARY_DATA = *mut LSA_FOREST_TRUST_BINARY_DATA;
#[cfg(feature = "Win32_lsalookup")]
pub type PLSA_FOREST_TRUST_COLLISION_INFORMATION = *mut LSA_FOREST_TRUST_COLLISION_INFORMATION;
#[cfg(feature = "Win32_lsalookup")]
pub type PLSA_FOREST_TRUST_COLLISION_RECORD = *mut LSA_FOREST_TRUST_COLLISION_RECORD;
#[cfg(all(feature = "Win32_lsalookup", feature = "Win32_winnt"))]
pub type PLSA_FOREST_TRUST_DOMAIN_INFO = *mut LSA_FOREST_TRUST_DOMAIN_INFO;
#[cfg(all(feature = "Win32_lsalookup", feature = "Win32_minwindef", feature = "Win32_winnt"))]
pub type PLSA_FOREST_TRUST_INFORMATION = *mut LSA_FOREST_TRUST_INFORMATION;
#[cfg(all(feature = "Win32_lsalookup", feature = "Win32_minwindef", feature = "Win32_winnt"))]
pub type PLSA_FOREST_TRUST_INFORMATION2 = *mut LSA_FOREST_TRUST_INFORMATION2;
#[cfg(all(feature = "Win32_lsalookup", feature = "Win32_minwindef", feature = "Win32_winnt"))]
pub type PLSA_FOREST_TRUST_RECORD = *mut LSA_FOREST_TRUST_RECORD;
#[cfg(all(feature = "Win32_lsalookup", feature = "Win32_minwindef", feature = "Win32_winnt"))]
pub type PLSA_FOREST_TRUST_RECORD2 = *mut LSA_FOREST_TRUST_RECORD2;
#[cfg(all(feature = "Win32_lsalookup", feature = "Win32_winnt"))]
pub type PLSA_FOREST_TRUST_SCANNER_INFO = *mut LSA_FOREST_TRUST_SCANNER_INFO;
pub type PLSA_HANDLE = *mut *mut core::ffi::c_void;
pub type PLSA_LAST_INTER_LOGON_INFO = *mut LSA_LAST_INTER_LOGON_INFO;
pub type PLSA_OPERATIONAL_MODE = *mut u32;
#[cfg(feature = "Win32_winnt")]
pub type PLSA_TRANSLATED_SID = *mut LSA_TRANSLATED_SID;
pub type PMSV1_0_AV_PAIR = *mut MSV1_0_AV_PAIR;
#[cfg(feature = "Win32_lsalookup")]
pub type PMSV1_0_CHANGEPASSWORD_REQUEST = *mut MSV1_0_CHANGEPASSWORD_REQUEST;
pub type PMSV1_0_CHANGEPASSWORD_RESPONSE = *mut MSV1_0_CHANGEPASSWORD_RESPONSE;
pub type PMSV1_0_CREDENTIAL_KEY = *mut MSV1_0_CREDENTIAL_KEY;
#[cfg(feature = "Win32_lsalookup")]
pub type PMSV1_0_INTERACTIVE_LOGON = *mut MSV1_0_INTERACTIVE_LOGON;
#[cfg(feature = "Win32_lsalookup")]
pub type PMSV1_0_INTERACTIVE_PROFILE = *mut MSV1_0_INTERACTIVE_PROFILE;
pub type PMSV1_0_IUM_SUPPLEMENTAL_CREDENTIAL = *mut MSV1_0_IUM_SUPPLEMENTAL_CREDENTIAL;
#[cfg(all(feature = "Win32_lsalookup", feature = "Win32_winnt"))]
pub type PMSV1_0_LM20_LOGON = *mut MSV1_0_LM20_LOGON;
#[cfg(feature = "Win32_lsalookup")]
pub type PMSV1_0_LM20_LOGON_PROFILE = *mut MSV1_0_LM20_LOGON_PROFILE;
pub type PMSV1_0_LOGON_SUBMIT_TYPE = *mut MSV1_0_LOGON_SUBMIT_TYPE;
pub type PMSV1_0_NTLM3_RESPONSE = *mut MSV1_0_NTLM3_RESPONSE;
#[cfg(all(feature = "Win32_lsalookup", feature = "Win32_minwindef"))]
pub type PMSV1_0_PASSTHROUGH_REQUEST = *mut MSV1_0_PASSTHROUGH_REQUEST;
#[cfg(feature = "Win32_minwindef")]
pub type PMSV1_0_PASSTHROUGH_RESPONSE = *mut MSV1_0_PASSTHROUGH_RESPONSE;
pub type PMSV1_0_PROFILE_BUFFER_TYPE = *mut MSV1_0_PROFILE_BUFFER_TYPE;
pub type PMSV1_0_PROTOCOL_MESSAGE_TYPE = *mut MSV1_0_PROTOCOL_MESSAGE_TYPE;
pub type PMSV1_0_REMOTE_SUPPLEMENTAL_CREDENTIAL = *mut MSV1_0_REMOTE_SUPPLEMENTAL_CREDENTIAL;
#[cfg(feature = "Win32_lsalookup")]
pub type PMSV1_0_S4U_LOGON = *mut MSV1_0_S4U_LOGON;
#[cfg(all(feature = "Win32_lsalookup", feature = "Win32_winnt"))]
pub type PMSV1_0_SUBAUTH_LOGON = *mut MSV1_0_SUBAUTH_LOGON;
#[cfg(feature = "Win32_minwindef")]
pub type PMSV1_0_SUBAUTH_REQUEST = *mut MSV1_0_SUBAUTH_REQUEST;
#[cfg(feature = "Win32_minwindef")]
pub type PMSV1_0_SUBAUTH_RESPONSE = *mut MSV1_0_SUBAUTH_RESPONSE;
pub type PMSV1_0_SUPPLEMENTAL_CREDENTIAL = *mut MSV1_0_SUPPLEMENTAL_CREDENTIAL;
pub type PMSV1_0_SUPPLEMENTAL_CREDENTIAL_V2 = *mut MSV1_0_SUPPLEMENTAL_CREDENTIAL_V2;
pub type PMSV1_0_SUPPLEMENTAL_CREDENTIAL_V3 = *mut MSV1_0_SUPPLEMENTAL_CREDENTIAL_V3;
#[cfg(feature = "Win32_winnt")]
pub type PNEGOTIATE_CALLER_NAME_REQUEST = *mut NEGOTIATE_CALLER_NAME_REQUEST;
pub type PNEGOTIATE_CALLER_NAME_RESPONSE = *mut NEGOTIATE_CALLER_NAME_RESPONSE;
pub type PNEGOTIATE_PACKAGE_PREFIX = *mut NEGOTIATE_PACKAGE_PREFIX;
pub type PNEGOTIATE_PACKAGE_PREFIXES = *mut NEGOTIATE_PACKAGE_PREFIXES;
pub const POLICY_ALL_ACCESS: u32 = 987135;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct POLICY_AUDIT_CATEGORIES_INFO {
    pub MaximumCategoryCount: u32,
    pub SubCategoriesInfo: PPOLICY_AUDIT_SUBCATEGORIES_INFO,
}
impl Default for POLICY_AUDIT_CATEGORIES_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct POLICY_AUDIT_EVENTS_INFO {
    pub AuditingMode: bool,
    pub EventAuditingOptions: PPOLICY_AUDIT_EVENT_OPTIONS,
    pub MaximumAuditEventCount: u32,
}
impl Default for POLICY_AUDIT_EVENTS_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const POLICY_AUDIT_EVENT_FAILURE: u32 = 2;
pub const POLICY_AUDIT_EVENT_MASK: u32 = 7;
pub const POLICY_AUDIT_EVENT_NONE: u32 = 4;
pub type POLICY_AUDIT_EVENT_OPTIONS = u32;
pub const POLICY_AUDIT_EVENT_SUCCESS: u32 = 1;
pub type POLICY_AUDIT_EVENT_TYPE = i32;
pub const POLICY_AUDIT_EVENT_UNCHANGED: u32 = 0;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct POLICY_AUDIT_FULL_QUERY_INFO {
    pub ShutDownOnFull: bool,
    pub LogIsFull: bool,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct POLICY_AUDIT_FULL_SET_INFO {
    pub ShutDownOnFull: bool,
}
pub const POLICY_AUDIT_LOG_ADMIN: u32 = 512;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct POLICY_AUDIT_LOG_INFO {
    pub AuditLogPercentFull: u32,
    pub MaximumLogSize: u32,
    pub AuditRetentionPeriod: i64,
    pub AuditLogFullShutdownInProgress: bool,
    pub TimeToShutdown: i64,
    pub NextAuditRecordId: u32,
}
#[repr(C)]
#[cfg(feature = "Win32_winnt")]
#[derive(Clone, Copy)]
pub struct POLICY_AUDIT_SID_ARRAY {
    pub UsersCount: u32,
    pub UserSidArray: *mut super::winnt::PSID,
}
#[cfg(feature = "Win32_winnt")]
impl Default for POLICY_AUDIT_SID_ARRAY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct POLICY_AUDIT_SUBCATEGORIES_INFO {
    pub MaximumSubCategoryCount: u32,
    pub EventAuditingOptions: PPOLICY_AUDIT_EVENT_OPTIONS,
}
impl Default for POLICY_AUDIT_SUBCATEGORIES_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const POLICY_CREATE_ACCOUNT: u32 = 16;
pub const POLICY_CREATE_PRIVILEGE: u32 = 64;
pub const POLICY_CREATE_SECRET: u32 = 32;
#[repr(C)]
#[cfg(feature = "Win32_winnt")]
#[derive(Clone, Copy, Default)]
pub struct POLICY_DEFAULT_QUOTA_INFO {
    pub QuotaLimits: super::winnt::QUOTA_LIMITS,
}
#[repr(C)]
#[cfg(feature = "Win32_minwindef")]
#[derive(Clone, Copy)]
pub struct POLICY_DOMAIN_EFS_INFO {
    pub InfoLength: u32,
    pub EfsBlob: super::minwindef::PUCHAR,
}
#[cfg(feature = "Win32_minwindef")]
impl Default for POLICY_DOMAIN_EFS_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type POLICY_DOMAIN_INFORMATION_CLASS = i32;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct POLICY_DOMAIN_KERBEROS_TICKET_INFO {
    pub AuthenticationOptions: u32,
    pub MaxServiceTicketAge: i64,
    pub MaxTicketAge: i64,
    pub MaxRenewAge: i64,
    pub MaxClockSkew: i64,
    pub Reserved: i64,
}
pub const POLICY_EXECUTE: u32 = 133121;
pub const POLICY_GET_PRIVATE_INFORMATION: u32 = 4;
pub type POLICY_INFORMATION_CLASS = i32;
pub const POLICY_KERBEROS_VALIDATE_CLIENT: u32 = 128;
pub const POLICY_LOOKUP_NAMES: u32 = 2048;
pub type POLICY_LSA_SERVER_ROLE = i32;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct POLICY_LSA_SERVER_ROLE_INFO {
    pub LsaServerRole: POLICY_LSA_SERVER_ROLE,
}
#[repr(C)]
#[cfg(feature = "Win32_winnt")]
#[derive(Clone, Copy)]
pub struct POLICY_MACHINE_ACCT_INFO {
    pub Rid: u32,
    pub Sid: super::winnt::PSID,
}
#[cfg(feature = "Win32_winnt")]
impl Default for POLICY_MACHINE_ACCT_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_winnt")]
#[derive(Clone, Copy)]
pub struct POLICY_MACHINE_ACCT_INFO2 {
    pub Rid: u32,
    pub Sid: super::winnt::PSID,
    pub ObjectGuid: windows_sys::core::GUID,
}
#[cfg(feature = "Win32_winnt")]
impl Default for POLICY_MACHINE_ACCT_INFO2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct POLICY_MODIFICATION_INFO {
    pub ModifiedId: i64,
    pub DatabaseCreationTime: i64,
}
pub const POLICY_NOTIFICATION: u32 = 4096;
pub type POLICY_NOTIFICATION_INFORMATION_CLASS = i32;
#[repr(C)]
#[cfg(feature = "Win32_lsalookup")]
#[derive(Clone, Copy, Default)]
pub struct POLICY_PD_ACCOUNT_INFO {
    pub Name: super::lsalookup::LSA_UNICODE_STRING,
}
#[repr(C)]
#[cfg(all(feature = "Win32_lsalookup", feature = "Win32_winnt"))]
#[derive(Clone, Copy)]
pub struct POLICY_PRIMARY_DOMAIN_INFO {
    pub Name: super::lsalookup::LSA_UNICODE_STRING,
    pub Sid: super::winnt::PSID,
}
#[cfg(all(feature = "Win32_lsalookup", feature = "Win32_winnt"))]
impl Default for POLICY_PRIMARY_DOMAIN_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const POLICY_READ: u32 = 131078;
#[repr(C)]
#[cfg(feature = "Win32_lsalookup")]
#[derive(Clone, Copy, Default)]
pub struct POLICY_REPLICA_SOURCE_INFO {
    pub ReplicaSource: super::lsalookup::LSA_UNICODE_STRING,
    pub ReplicaAccountName: super::lsalookup::LSA_UNICODE_STRING,
}
pub const POLICY_SERVER_ADMIN: u32 = 1024;
pub const POLICY_SET_AUDIT_REQUIREMENTS: u32 = 256;
pub const POLICY_SET_DEFAULT_QUOTA_LIMITS: u32 = 128;
pub const POLICY_TRUST_ADMIN: u32 = 8;
pub const POLICY_VIEW_AUDIT_INFORMATION: u32 = 2;
pub const POLICY_VIEW_LOCAL_INFORMATION: u32 = 1;
pub const POLICY_WRITE: u32 = 133112;
#[cfg(all(feature = "Win32_lsalookup", feature = "Win32_minwindef"))]
pub type PPKU2U_CERTIFICATE_S4U_LOGON = *mut PKU2U_CERTIFICATE_S4U_LOGON;
pub type PPKU2U_CERT_BLOB = *mut PKU2U_CERT_BLOB;
pub type PPKU2U_CREDUI_CONTEXT = *mut PKU2U_CREDUI_CONTEXT;
pub type PPKU2U_LOGON_SUBMIT_TYPE = *mut PKU2U_LOGON_SUBMIT_TYPE;
pub type PPOLICY_AUDIT_CATEGORIES_INFO = *mut POLICY_AUDIT_CATEGORIES_INFO;
pub type PPOLICY_AUDIT_EVENTS_INFO = *mut POLICY_AUDIT_EVENTS_INFO;
pub type PPOLICY_AUDIT_EVENT_OPTIONS = *mut u32;
pub type PPOLICY_AUDIT_EVENT_TYPE = *mut POLICY_AUDIT_EVENT_TYPE;
pub type PPOLICY_AUDIT_FULL_QUERY_INFO = *mut POLICY_AUDIT_FULL_QUERY_INFO;
pub type PPOLICY_AUDIT_FULL_SET_INFO = *mut POLICY_AUDIT_FULL_SET_INFO;
pub type PPOLICY_AUDIT_LOG_INFO = *mut POLICY_AUDIT_LOG_INFO;
#[cfg(feature = "Win32_winnt")]
pub type PPOLICY_AUDIT_SID_ARRAY = *mut POLICY_AUDIT_SID_ARRAY;
pub type PPOLICY_AUDIT_SUBCATEGORIES_INFO = *mut POLICY_AUDIT_SUBCATEGORIES_INFO;
#[cfg(feature = "Win32_winnt")]
pub type PPOLICY_DEFAULT_QUOTA_INFO = *mut POLICY_DEFAULT_QUOTA_INFO;
#[cfg(feature = "Win32_minwindef")]
pub type PPOLICY_DOMAIN_EFS_INFO = *mut POLICY_DOMAIN_EFS_INFO;
pub type PPOLICY_DOMAIN_INFORMATION_CLASS = *mut POLICY_DOMAIN_INFORMATION_CLASS;
pub type PPOLICY_DOMAIN_KERBEROS_TICKET_INFO = *mut POLICY_DOMAIN_KERBEROS_TICKET_INFO;
pub type PPOLICY_INFORMATION_CLASS = *mut POLICY_INFORMATION_CLASS;
pub type PPOLICY_LSA_SERVER_ROLE = *mut POLICY_LSA_SERVER_ROLE;
pub type PPOLICY_LSA_SERVER_ROLE_INFO = *mut POLICY_LSA_SERVER_ROLE_INFO;
#[cfg(feature = "Win32_winnt")]
pub type PPOLICY_MACHINE_ACCT_INFO = *mut POLICY_MACHINE_ACCT_INFO;
#[cfg(feature = "Win32_winnt")]
pub type PPOLICY_MACHINE_ACCT_INFO2 = *mut POLICY_MACHINE_ACCT_INFO2;
pub type PPOLICY_MODIFICATION_INFO = *mut POLICY_MODIFICATION_INFO;
pub type PPOLICY_NOTIFICATION_INFORMATION_CLASS = *mut POLICY_NOTIFICATION_INFORMATION_CLASS;
#[cfg(feature = "Win32_lsalookup")]
pub type PPOLICY_PD_ACCOUNT_INFO = *mut POLICY_PD_ACCOUNT_INFO;
#[cfg(all(feature = "Win32_lsalookup", feature = "Win32_winnt"))]
pub type PPOLICY_PRIMARY_DOMAIN_INFO = *mut POLICY_PRIMARY_DOMAIN_INFO;
#[cfg(feature = "Win32_lsalookup")]
pub type PPOLICY_REPLICA_SOURCE_INFO = *mut POLICY_REPLICA_SOURCE_INFO;
pub type PSAM_INIT_NOTIFICATION_ROUTINE = Option<unsafe extern "system" fn() -> bool>;
#[cfg(feature = "Win32_lsalookup")]
pub type PSAM_PASSWORD_FILTER_ROUTINE = Option<unsafe extern "system" fn(accountname: *const super::lsalookup::LSA_UNICODE_STRING, fullname: *const super::lsalookup::LSA_UNICODE_STRING, password: *const super::lsalookup::LSA_UNICODE_STRING, setoperation: bool) -> bool>;
#[cfg(all(feature = "Win32_bcrypt", feature = "Win32_lsalookup"))]
pub type PSAM_PASSWORD_NOTIFICATION_ROUTINE = Option<unsafe extern "system" fn(username: *mut super::lsalookup::LSA_UNICODE_STRING, relativeid: u32, newpassword: *mut super::lsalookup::LSA_UNICODE_STRING) -> super::bcrypt::NTSTATUS>;
#[cfg(all(feature = "Win32_lsalookup", feature = "Win32_winnt"))]
pub type PSECURITY_LOGON_SESSION_DATA = *mut SECURITY_LOGON_SESSION_DATA;
pub type PSECURITY_LOGON_TYPE = *mut SECURITY_LOGON_TYPE;
#[cfg(feature = "Win32_winnt")]
pub type PSE_ADT_ACCESS_REASON = *mut SE_ADT_ACCESS_REASON;
#[cfg(feature = "Win32_winnt")]
pub type PSE_ADT_CLAIMS = *mut SE_ADT_CLAIMS;
#[cfg(feature = "Win32_winnt")]
pub type PSE_ADT_OBJECT_TYPE = *mut SE_ADT_OBJECT_TYPE;
pub type PSE_ADT_PARAMETER_ARRAY = *mut SE_ADT_PARAMETER_ARRAY;
pub type PSE_ADT_PARAMETER_ARRAY_ENTRY = *mut SE_ADT_PARAMETER_ARRAY_ENTRY;
pub type PSE_ADT_PARAMETER_ARRAY_EX = *mut SE_ADT_PARAMETER_ARRAY_EX;
pub type PSE_ADT_PARAMETER_TYPE = *mut SE_ADT_PARAMETER_TYPE;
#[cfg(all(feature = "Win32_lsalookup", feature = "Win32_winnt"))]
pub type PSTRING = *mut super::lsalookup::LSA_STRING;
#[cfg(feature = "Win32_lsalookup")]
pub type PTRUSTED_CONTROLLERS_INFO = *mut TRUSTED_CONTROLLERS_INFO;
#[cfg(feature = "Win32_minwindef")]
pub type PTRUSTED_DOMAIN_AUTH_INFORMATION = *mut TRUSTED_DOMAIN_AUTH_INFORMATION;
#[cfg(all(feature = "Win32_lsalookup", feature = "Win32_minwindef", feature = "Win32_winnt"))]
pub type PTRUSTED_DOMAIN_FULL_INFORMATION = *mut TRUSTED_DOMAIN_FULL_INFORMATION;
#[cfg(all(feature = "Win32_lsalookup", feature = "Win32_minwindef", feature = "Win32_winnt"))]
pub type PTRUSTED_DOMAIN_FULL_INFORMATION2 = *mut TRUSTED_DOMAIN_FULL_INFORMATION2;
#[cfg(all(feature = "Win32_lsalookup", feature = "Win32_winnt"))]
pub type PTRUSTED_DOMAIN_INFORMATION_BASIC = super::lsalookup::PLSA_TRUST_INFORMATION;
#[cfg(all(feature = "Win32_lsalookup", feature = "Win32_winnt"))]
pub type PTRUSTED_DOMAIN_INFORMATION_EX = *mut TRUSTED_DOMAIN_INFORMATION_EX;
#[cfg(all(feature = "Win32_lsalookup", feature = "Win32_minwindef", feature = "Win32_winnt"))]
pub type PTRUSTED_DOMAIN_INFORMATION_EX2 = *mut TRUSTED_DOMAIN_INFORMATION_EX2;
#[cfg(feature = "Win32_lsalookup")]
pub type PTRUSTED_DOMAIN_NAME_INFO = *mut TRUSTED_DOMAIN_NAME_INFO;
pub type PTRUSTED_DOMAIN_SUPPORTED_ENCRYPTION_TYPES = *mut TRUSTED_DOMAIN_SUPPORTED_ENCRYPTION_TYPES;
pub type PTRUSTED_INFORMATION_CLASS = *mut TRUSTED_INFORMATION_CLASS;
#[cfg(feature = "Win32_lsalookup")]
pub type PTRUSTED_PASSWORD_INFO = *mut TRUSTED_PASSWORD_INFO;
pub type PTRUSTED_POSIX_OFFSET_INFO = *mut TRUSTED_POSIX_OFFSET_INFO;
#[cfg(feature = "Win32_lsalookup")]
pub type PUNICODE_STRING = *mut super::lsalookup::LSA_UNICODE_STRING;
pub const Pku2uCertificateS4ULogon: PKU2U_LOGON_SUBMIT_TYPE = 14;
pub const PolicyAccountDomainInformation: POLICY_INFORMATION_CLASS = 5;
pub const PolicyAuditEventsInformation: POLICY_INFORMATION_CLASS = 2;
pub const PolicyAuditFullQueryInformation: POLICY_INFORMATION_CLASS = 11;
pub const PolicyAuditFullSetInformation: POLICY_INFORMATION_CLASS = 10;
pub const PolicyAuditLogInformation: POLICY_INFORMATION_CLASS = 1;
pub const PolicyDefaultQuotaInformation: POLICY_INFORMATION_CLASS = 8;
pub const PolicyDnsDomainInformation: POLICY_INFORMATION_CLASS = 12;
pub const PolicyDnsDomainInformationInt: POLICY_INFORMATION_CLASS = 13;
pub const PolicyDomainEfsInformation: POLICY_DOMAIN_INFORMATION_CLASS = 2;
pub const PolicyDomainKerberosTicketInformation: POLICY_DOMAIN_INFORMATION_CLASS = 3;
pub const PolicyLastEntry: POLICY_INFORMATION_CLASS = 17;
pub const PolicyLocalAccountDomainInformation: POLICY_INFORMATION_CLASS = 14;
pub const PolicyLsaServerRoleInformation: POLICY_INFORMATION_CLASS = 6;
pub const PolicyMachineAccountInformation: POLICY_INFORMATION_CLASS = 15;
pub const PolicyMachineAccountInformation2: POLICY_INFORMATION_CLASS = 16;
pub const PolicyModificationInformation: POLICY_INFORMATION_CLASS = 9;
pub const PolicyNotifyAccountDomainInformation: POLICY_NOTIFICATION_INFORMATION_CLASS = 2;
pub const PolicyNotifyAuditEventsInformation: POLICY_NOTIFICATION_INFORMATION_CLASS = 1;
pub const PolicyNotifyDnsDomainInformation: POLICY_NOTIFICATION_INFORMATION_CLASS = 4;
pub const PolicyNotifyDomainEfsInformation: POLICY_NOTIFICATION_INFORMATION_CLASS = 5;
pub const PolicyNotifyDomainKerberosTicketInformation: POLICY_NOTIFICATION_INFORMATION_CLASS = 6;
pub const PolicyNotifyGlobalSaclInformation: POLICY_NOTIFICATION_INFORMATION_CLASS = 8;
pub const PolicyNotifyMachineAccountPasswordInformation: POLICY_NOTIFICATION_INFORMATION_CLASS = 7;
pub const PolicyNotifyMax: POLICY_NOTIFICATION_INFORMATION_CLASS = 9;
pub const PolicyNotifyServerRoleInformation: POLICY_NOTIFICATION_INFORMATION_CLASS = 3;
pub const PolicyPdAccountInformation: POLICY_INFORMATION_CLASS = 4;
pub const PolicyPrimaryDomainInformation: POLICY_INFORMATION_CLASS = 3;
pub const PolicyReplicaSourceInformation: POLICY_INFORMATION_CLASS = 7;
pub const PolicyServerRoleBackup: POLICY_LSA_SERVER_ROLE = 2;
pub const PolicyServerRolePrimary: POLICY_LSA_SERVER_ROLE = 3;
pub const Proxy: SECURITY_LOGON_TYPE = 6;
pub const RTL_ENCRYPT_MEMORY_SIZE: u32 = 8;
pub const RTL_ENCRYPT_OPTION_CROSS_PROCESS: u32 = 1;
pub const RTL_ENCRYPT_OPTION_FOR_SYSTEM: u32 = 4;
pub const RTL_ENCRYPT_OPTION_SAME_LOGON: u32 = 2;
pub const RemoteInteractive: SECURITY_LOGON_TYPE = 10;
pub const SAM_INIT_NOTIFICATION_ROUTINE: windows_sys::core::PCSTR = windows_sys::core::s!("InitializeChangeNotify");
pub const SAM_PASSWORD_CHANGE_NOTIFY_ROUTINE: windows_sys::core::PCSTR = windows_sys::core::s!("PasswordChangeNotify");
pub const SAM_PASSWORD_FILTER_ROUTINE: windows_sys::core::PCSTR = windows_sys::core::s!("PasswordFilter");
#[repr(C)]
#[cfg(all(feature = "Win32_lsalookup", feature = "Win32_winnt"))]
#[derive(Clone, Copy)]
pub struct SECURITY_LOGON_SESSION_DATA {
    pub Size: u32,
    pub LogonId: super::winnt::LUID,
    pub UserName: super::lsalookup::LSA_UNICODE_STRING,
    pub LogonDomain: super::lsalookup::LSA_UNICODE_STRING,
    pub AuthenticationPackage: super::lsalookup::LSA_UNICODE_STRING,
    pub LogonType: u32,
    pub Session: u32,
    pub Sid: super::winnt::PSID,
    pub LogonTime: i64,
    pub LogonServer: super::lsalookup::LSA_UNICODE_STRING,
    pub DnsDomainName: super::lsalookup::LSA_UNICODE_STRING,
    pub Upn: super::lsalookup::LSA_UNICODE_STRING,
    pub UserFlags: u32,
    pub LastLogonInfo: LSA_LAST_INTER_LOGON_INFO,
    pub LogonScript: super::lsalookup::LSA_UNICODE_STRING,
    pub ProfilePath: super::lsalookup::LSA_UNICODE_STRING,
    pub HomeDirectory: super::lsalookup::LSA_UNICODE_STRING,
    pub HomeDirectoryDrive: super::lsalookup::LSA_UNICODE_STRING,
    pub LogoffTime: i64,
    pub KickOffTime: i64,
    pub PasswordLastSet: i64,
    pub PasswordCanChange: i64,
    pub PasswordMustChange: i64,
}
#[cfg(all(feature = "Win32_lsalookup", feature = "Win32_winnt"))]
impl Default for SECURITY_LOGON_SESSION_DATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type SECURITY_LOGON_TYPE = i32;
#[repr(C)]
#[cfg(feature = "Win32_winnt")]
#[derive(Clone, Copy)]
pub struct SE_ADT_ACCESS_REASON {
    pub AccessMask: super::winnt::ACCESS_MASK,
    pub AccessReasons: [u32; 32],
    pub ObjectTypeIndex: u32,
    pub AccessGranted: u32,
    pub SecurityDescriptor: super::winnt::PSECURITY_DESCRIPTOR,
}
#[cfg(feature = "Win32_winnt")]
impl Default for SE_ADT_ACCESS_REASON {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_winnt")]
#[derive(Clone, Copy)]
pub struct SE_ADT_CLAIMS {
    pub Length: u32,
    pub Claims: super::winnt::PCLAIMS_BLOB,
}
#[cfg(feature = "Win32_winnt")]
impl Default for SE_ADT_CLAIMS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const SE_ADT_OBJECT_ONLY: u32 = 1;
#[repr(C)]
#[cfg(feature = "Win32_winnt")]
#[derive(Clone, Copy, Default)]
pub struct SE_ADT_OBJECT_TYPE {
    pub ObjectType: windows_sys::core::GUID,
    pub Flags: u16,
    pub Level: u16,
    pub AccessMask: super::winnt::ACCESS_MASK,
}
pub const SE_ADT_PARAMETERS_SELF_RELATIVE: u32 = 1;
pub const SE_ADT_PARAMETERS_SEND_TO_LSA: u32 = 2;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct SE_ADT_PARAMETER_ARRAY {
    pub CategoryId: u32,
    pub AuditId: u32,
    pub ParameterCount: u32,
    pub Length: u32,
    pub FlatSubCategoryId: u16,
    pub Type: u16,
    pub Flags: u32,
    pub Parameters: [SE_ADT_PARAMETER_ARRAY_ENTRY; 32],
}
impl Default for SE_ADT_PARAMETER_ARRAY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct SE_ADT_PARAMETER_ARRAY_ENTRY {
    pub Type: SE_ADT_PARAMETER_TYPE,
    pub Length: u32,
    pub Data: [usize; 2],
    pub Address: *mut core::ffi::c_void,
}
impl Default for SE_ADT_PARAMETER_ARRAY_ENTRY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct SE_ADT_PARAMETER_ARRAY_EX {
    pub CategoryId: u32,
    pub AuditId: u32,
    pub Version: u32,
    pub ParameterCount: u32,
    pub Length: u32,
    pub FlatSubCategoryId: u16,
    pub Type: u16,
    pub Flags: u32,
    pub Parameters: [SE_ADT_PARAMETER_ARRAY_ENTRY; 32],
}
impl Default for SE_ADT_PARAMETER_ARRAY_EX {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const SE_ADT_PARAMETER_EXTENSIBLE_AUDIT: u32 = 4;
pub const SE_ADT_PARAMETER_GENERIC_AUDIT: u32 = 8;
pub type SE_ADT_PARAMETER_TYPE = i32;
pub const SE_ADT_PARAMETER_WRITE_SYNCHRONOUS: u32 = 16;
pub const SE_MAX_AUDIT_PARAMETERS: u32 = 32;
pub const SE_MAX_GENERIC_AUDIT_PARAMETERS: u32 = 28;
#[repr(C)]
#[cfg(feature = "Win32_winnt")]
#[derive(Clone, Copy)]
pub struct STRING {
    pub Length: u16,
    pub MaximumLength: u16,
    pub Buffer: super::winnt::PCHAR,
}
#[cfg(feature = "Win32_winnt")]
impl Default for STRING {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const SeAdtParmTypeAccessMask: SE_ADT_PARAMETER_TYPE = 7;
pub const SeAdtParmTypeAccessReason: SE_ADT_PARAMETER_TYPE = 29;
pub const SeAdtParmTypeClaims: SE_ADT_PARAMETER_TYPE = 32;
pub const SeAdtParmTypeDateTime: SE_ADT_PARAMETER_TYPE = 22;
pub const SeAdtParmTypeDuration: SE_ADT_PARAMETER_TYPE = 18;
pub const SeAdtParmTypeFileSpec: SE_ADT_PARAMETER_TYPE = 2;
pub const SeAdtParmTypeGuid: SE_ADT_PARAMETER_TYPE = 13;
pub const SeAdtParmTypeHexInt64: SE_ADT_PARAMETER_TYPE = 15;
pub const SeAdtParmTypeHexUlong: SE_ADT_PARAMETER_TYPE = 10;
pub const SeAdtParmTypeLogonHours: SE_ADT_PARAMETER_TYPE = 25;
pub const SeAdtParmTypeLogonId: SE_ADT_PARAMETER_TYPE = 5;
pub const SeAdtParmTypeLogonIdAsSid: SE_ADT_PARAMETER_TYPE = 33;
pub const SeAdtParmTypeLogonIdEx: SE_ADT_PARAMETER_TYPE = 35;
pub const SeAdtParmTypeLogonIdNoSid: SE_ADT_PARAMETER_TYPE = 26;
pub const SeAdtParmTypeLuid: SE_ADT_PARAMETER_TYPE = 14;
pub const SeAdtParmTypeMessage: SE_ADT_PARAMETER_TYPE = 21;
pub const SeAdtParmTypeMultiSzString: SE_ADT_PARAMETER_TYPE = 34;
pub const SeAdtParmTypeNoLogonId: SE_ADT_PARAMETER_TYPE = 6;
pub const SeAdtParmTypeNoUac: SE_ADT_PARAMETER_TYPE = 20;
pub const SeAdtParmTypeNone: SE_ADT_PARAMETER_TYPE = 0;
pub const SeAdtParmTypeObjectTypes: SE_ADT_PARAMETER_TYPE = 9;
pub const SeAdtParmTypePrivs: SE_ADT_PARAMETER_TYPE = 8;
pub const SeAdtParmTypePtr: SE_ADT_PARAMETER_TYPE = 11;
pub const SeAdtParmTypeResourceAttribute: SE_ADT_PARAMETER_TYPE = 31;
pub const SeAdtParmTypeSD: SE_ADT_PARAMETER_TYPE = 24;
pub const SeAdtParmTypeSid: SE_ADT_PARAMETER_TYPE = 4;
pub const SeAdtParmTypeSidList: SE_ADT_PARAMETER_TYPE = 17;
pub const SeAdtParmTypeSockAddr: SE_ADT_PARAMETER_TYPE = 23;
pub const SeAdtParmTypeSockAddrNoPort: SE_ADT_PARAMETER_TYPE = 28;
pub const SeAdtParmTypeStagingReason: SE_ADT_PARAMETER_TYPE = 30;
pub const SeAdtParmTypeString: SE_ADT_PARAMETER_TYPE = 1;
pub const SeAdtParmTypeStringList: SE_ADT_PARAMETER_TYPE = 16;
pub const SeAdtParmTypeTime: SE_ADT_PARAMETER_TYPE = 12;
pub const SeAdtParmTypeUlong: SE_ADT_PARAMETER_TYPE = 3;
pub const SeAdtParmTypeUlongNoConv: SE_ADT_PARAMETER_TYPE = 27;
pub const SeAdtParmTypeUserAccountControl: SE_ADT_PARAMETER_TYPE = 19;
pub const Service: SECURITY_LOGON_TYPE = 5;
#[repr(C)]
#[cfg(feature = "Win32_lsalookup")]
#[derive(Clone, Copy)]
pub struct TRUSTED_CONTROLLERS_INFO {
    pub Entries: u32,
    pub Names: super::lsalookup::PLSA_UNICODE_STRING,
}
#[cfg(feature = "Win32_lsalookup")]
impl Default for TRUSTED_CONTROLLERS_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_minwindef")]
#[derive(Clone, Copy)]
pub struct TRUSTED_DOMAIN_AUTH_INFORMATION {
    pub IncomingAuthInfos: u32,
    pub IncomingAuthenticationInformation: PLSA_AUTH_INFORMATION,
    pub IncomingPreviousAuthenticationInformation: PLSA_AUTH_INFORMATION,
    pub OutgoingAuthInfos: u32,
    pub OutgoingAuthenticationInformation: PLSA_AUTH_INFORMATION,
    pub OutgoingPreviousAuthenticationInformation: PLSA_AUTH_INFORMATION,
}
#[cfg(feature = "Win32_minwindef")]
impl Default for TRUSTED_DOMAIN_AUTH_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_lsalookup", feature = "Win32_minwindef", feature = "Win32_winnt"))]
#[derive(Clone, Copy, Default)]
pub struct TRUSTED_DOMAIN_FULL_INFORMATION {
    pub Information: TRUSTED_DOMAIN_INFORMATION_EX,
    pub PosixOffset: TRUSTED_POSIX_OFFSET_INFO,
    pub AuthInformation: TRUSTED_DOMAIN_AUTH_INFORMATION,
}
#[repr(C)]
#[cfg(all(feature = "Win32_lsalookup", feature = "Win32_minwindef", feature = "Win32_winnt"))]
#[derive(Clone, Copy, Default)]
pub struct TRUSTED_DOMAIN_FULL_INFORMATION2 {
    pub Information: TRUSTED_DOMAIN_INFORMATION_EX2,
    pub PosixOffset: TRUSTED_POSIX_OFFSET_INFO,
    pub AuthInformation: TRUSTED_DOMAIN_AUTH_INFORMATION,
}
#[cfg(all(feature = "Win32_lsalookup", feature = "Win32_winnt"))]
pub type TRUSTED_DOMAIN_INFORMATION_BASIC = super::lsalookup::LSA_TRUST_INFORMATION;
#[repr(C)]
#[cfg(all(feature = "Win32_lsalookup", feature = "Win32_winnt"))]
#[derive(Clone, Copy)]
pub struct TRUSTED_DOMAIN_INFORMATION_EX {
    pub Name: super::lsalookup::LSA_UNICODE_STRING,
    pub FlatName: super::lsalookup::LSA_UNICODE_STRING,
    pub Sid: super::winnt::PSID,
    pub TrustDirection: u32,
    pub TrustType: u32,
    pub TrustAttributes: u32,
}
#[cfg(all(feature = "Win32_lsalookup", feature = "Win32_winnt"))]
impl Default for TRUSTED_DOMAIN_INFORMATION_EX {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_lsalookup", feature = "Win32_minwindef", feature = "Win32_winnt"))]
#[derive(Clone, Copy)]
pub struct TRUSTED_DOMAIN_INFORMATION_EX2 {
    pub Name: super::lsalookup::LSA_UNICODE_STRING,
    pub FlatName: super::lsalookup::LSA_UNICODE_STRING,
    pub Sid: super::winnt::PSID,
    pub TrustDirection: u32,
    pub TrustType: u32,
    pub TrustAttributes: u32,
    pub ForestTrustLength: u32,
    pub ForestTrustInfo: super::minwindef::PUCHAR,
}
#[cfg(all(feature = "Win32_lsalookup", feature = "Win32_minwindef", feature = "Win32_winnt"))]
impl Default for TRUSTED_DOMAIN_INFORMATION_EX2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_lsalookup")]
#[derive(Clone, Copy, Default)]
pub struct TRUSTED_DOMAIN_NAME_INFO {
    pub Name: super::lsalookup::LSA_UNICODE_STRING,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct TRUSTED_DOMAIN_SUPPORTED_ENCRYPTION_TYPES {
    pub SupportedEncryptionTypes: u32,
}
pub type TRUSTED_INFORMATION_CLASS = i32;
#[repr(C)]
#[cfg(feature = "Win32_lsalookup")]
#[derive(Clone, Copy, Default)]
pub struct TRUSTED_PASSWORD_INFO {
    pub Password: super::lsalookup::LSA_UNICODE_STRING,
    pub OldPassword: super::lsalookup::LSA_UNICODE_STRING,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct TRUSTED_POSIX_OFFSET_INFO {
    pub Offset: u32,
}
pub const TRUST_ATTRIBUTES_USER: u32 = 4278190080;
pub const TRUST_ATTRIBUTES_VALID: u32 = 4278452223;
pub const TRUST_ATTRIBUTE_CROSS_ORGANIZATION: u32 = 16;
pub const TRUST_ATTRIBUTE_CROSS_ORGANIZATION_ENABLE_TGT_DELEGATION: u32 = 2048;
pub const TRUST_ATTRIBUTE_CROSS_ORGANIZATION_NO_TGT_DELEGATION: u32 = 512;
pub const TRUST_ATTRIBUTE_DISABLE_AUTH_TARGET_VALIDATION: u32 = 4096;
pub const TRUST_ATTRIBUTE_FOREST_TRANSITIVE: u32 = 8;
pub const TRUST_ATTRIBUTE_NON_TRANSITIVE: u32 = 1;
pub const TRUST_ATTRIBUTE_PIM_TRUST: u32 = 1024;
pub const TRUST_ATTRIBUTE_QUARANTINED_DOMAIN: u32 = 4;
pub const TRUST_ATTRIBUTE_TREAT_AS_EXTERNAL: u32 = 64;
pub const TRUST_ATTRIBUTE_TRUST_USES_AES_KEYS: u32 = 256;
pub const TRUST_ATTRIBUTE_TRUST_USES_RC4_ENCRYPTION: u32 = 128;
pub const TRUST_ATTRIBUTE_UPLEVEL_ONLY: u32 = 2;
pub const TRUST_ATTRIBUTE_WITHIN_FOREST: u32 = 32;
pub const TRUST_AUTH_TYPE_CLEAR: u32 = 2;
pub const TRUST_AUTH_TYPE_NONE: u32 = 0;
pub const TRUST_AUTH_TYPE_NT4OWF: u32 = 1;
pub const TRUST_AUTH_TYPE_VERSION: u32 = 3;
pub const TRUST_DIRECTION_BIDIRECTIONAL: u32 = 3;
pub const TRUST_DIRECTION_DISABLED: u32 = 0;
pub const TRUST_DIRECTION_INBOUND: u32 = 1;
pub const TRUST_DIRECTION_OUTBOUND: u32 = 2;
pub const TRUST_TYPE_AAD: u32 = 5;
pub const TRUST_TYPE_DOWNLEVEL: u32 = 1;
pub const TRUST_TYPE_MIT: u32 = 3;
pub const TRUST_TYPE_UPLEVEL: u32 = 2;
pub const TrustedControllersInformation: TRUSTED_INFORMATION_CLASS = 2;
pub const TrustedDomainAuthInformation: TRUSTED_INFORMATION_CLASS = 7;
pub const TrustedDomainAuthInformationInternal: TRUSTED_INFORMATION_CLASS = 9;
pub const TrustedDomainAuthInformationInternalAes: TRUSTED_INFORMATION_CLASS = 14;
pub const TrustedDomainFullInformation: TRUSTED_INFORMATION_CLASS = 8;
pub const TrustedDomainFullInformation2Internal: TRUSTED_INFORMATION_CLASS = 12;
pub const TrustedDomainFullInformationInternal: TRUSTED_INFORMATION_CLASS = 10;
pub const TrustedDomainFullInformationInternalAes: TRUSTED_INFORMATION_CLASS = 15;
pub const TrustedDomainInformationBasic: TRUSTED_INFORMATION_CLASS = 5;
pub const TrustedDomainInformationEx: TRUSTED_INFORMATION_CLASS = 6;
pub const TrustedDomainInformationEx2Internal: TRUSTED_INFORMATION_CLASS = 11;
pub const TrustedDomainNameInformation: TRUSTED_INFORMATION_CLASS = 1;
pub const TrustedDomainSupportedEncryptionTypes: TRUSTED_INFORMATION_CLASS = 13;
pub const TrustedPasswordInformation: TRUSTED_INFORMATION_CLASS = 4;
pub const TrustedPosixOffsetInformation: TRUSTED_INFORMATION_CLASS = 3;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct UNICODE_STRING {
    pub Length: u16,
    pub MaximumLength: u16,
    pub Buffer: windows_sys::core::PWSTR,
}
impl Default for UNICODE_STRING {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const UndefinedLogonType: SECURITY_LOGON_TYPE = 0;
pub const Unlock: SECURITY_LOGON_TYPE = 7;
pub const VALID_PER_USER_AUDIT_POLICY_FLAG: u32 = 31;
