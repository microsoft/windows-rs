#![allow(non_snake_case, non_camel_case_types)]
#[cfg(feature = "Win32_Security_Authentication_Identity_Provider")]
pub mod Provider;
#[link(name = "windows")]
extern "system" {
    #[doc = "*Required features: `Win32_Security_Authentication_Identity`, `Win32_Security_Credentials`*"]
    #[cfg(feature = "Win32_Security_Credentials")]
    pub fn AcceptSecurityContext(phcredential: *const super::super::Credentials::SecHandle, phcontext: *const super::super::Credentials::SecHandle, pinput: *const SecBufferDesc, fcontextreq: ACCEPT_SECURITY_CONTEXT_CONTEXT_REQ, targetdatarep: u32, phnewcontext: *mut super::super::Credentials::SecHandle, poutput: *mut SecBufferDesc, pfcontextattr: *mut u32, ptsexpiry: *mut i64) -> i32;
    #[doc = "*Required features: `Win32_Security_Authentication_Identity`, `Win32_Foundation`, `Win32_Security_Credentials`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Credentials"))]
    pub fn AcquireCredentialsHandleA(pszprincipal: super::super::super::Foundation::PSTR, pszpackage: super::super::super::Foundation::PSTR, fcredentialuse: SECPKG_CRED, pvlogonid: *const ::core::ffi::c_void, pauthdata: *const ::core::ffi::c_void, pgetkeyfn: SEC_GET_KEY_FN, pvgetkeyargument: *const ::core::ffi::c_void, phcredential: *mut super::super::Credentials::SecHandle, ptsexpiry: *mut i64) -> i32;
    #[doc = "*Required features: `Win32_Security_Authentication_Identity`, `Win32_Foundation`, `Win32_Security_Credentials`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Credentials"))]
    pub fn AcquireCredentialsHandleW(pszprincipal: super::super::super::Foundation::PWSTR, pszpackage: super::super::super::Foundation::PWSTR, fcredentialuse: SECPKG_CRED, pvlogonid: *const ::core::ffi::c_void, pauthdata: *const ::core::ffi::c_void, pgetkeyfn: SEC_GET_KEY_FN, pvgetkeyargument: *const ::core::ffi::c_void, phcredential: *mut super::super::Credentials::SecHandle, ptsexpiry: *mut i64) -> i32;
    #[doc = "*Required features: `Win32_Security_Authentication_Identity`, `Win32_Foundation`, `Win32_Security_Credentials`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Credentials"))]
    pub fn AddCredentialsA(hcredentials: *const super::super::Credentials::SecHandle, pszprincipal: super::super::super::Foundation::PSTR, pszpackage: super::super::super::Foundation::PSTR, fcredentialuse: u32, pauthdata: *const ::core::ffi::c_void, pgetkeyfn: SEC_GET_KEY_FN, pvgetkeyargument: *const ::core::ffi::c_void, ptsexpiry: *mut i64) -> i32;
    #[doc = "*Required features: `Win32_Security_Authentication_Identity`, `Win32_Foundation`, `Win32_Security_Credentials`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Credentials"))]
    pub fn AddCredentialsW(hcredentials: *const super::super::Credentials::SecHandle, pszprincipal: super::super::super::Foundation::PWSTR, pszpackage: super::super::super::Foundation::PWSTR, fcredentialuse: u32, pauthdata: *const ::core::ffi::c_void, pgetkeyfn: SEC_GET_KEY_FN, pvgetkeyargument: *const ::core::ffi::c_void, ptsexpiry: *mut i64) -> i32;
    #[doc = "*Required features: `Win32_Security_Authentication_Identity`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn AddSecurityPackageA(pszpackagename: super::super::super::Foundation::PSTR, poptions: *const SECURITY_PACKAGE_OPTIONS) -> i32;
    #[doc = "*Required features: `Win32_Security_Authentication_Identity`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn AddSecurityPackageW(pszpackagename: super::super::super::Foundation::PWSTR, poptions: *const SECURITY_PACKAGE_OPTIONS) -> i32;
    #[doc = "*Required features: `Win32_Security_Authentication_Identity`, `Win32_Security_Credentials`*"]
    #[cfg(feature = "Win32_Security_Credentials")]
    pub fn ApplyControlToken(phcontext: *const super::super::Credentials::SecHandle, pinput: *const SecBufferDesc) -> i32;
    #[doc = "*Required features: `Win32_Security_Authentication_Identity`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn AuditComputeEffectivePolicyBySid(psid: super::super::super::Foundation::PSID, psubcategoryguids: *const ::windows_sys::core::GUID, dwpolicycount: u32, ppauditpolicy: *mut *mut AUDIT_POLICY_INFORMATION) -> super::super::super::Foundation::BOOLEAN;
    #[doc = "*Required features: `Win32_Security_Authentication_Identity`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn AuditComputeEffectivePolicyByToken(htokenhandle: super::super::super::Foundation::HANDLE, psubcategoryguids: *const ::windows_sys::core::GUID, dwpolicycount: u32, ppauditpolicy: *mut *mut AUDIT_POLICY_INFORMATION) -> super::super::super::Foundation::BOOLEAN;
    #[doc = "*Required features: `Win32_Security_Authentication_Identity`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn AuditEnumerateCategories(ppauditcategoriesarray: *mut *mut ::windows_sys::core::GUID, pdwcountreturned: *mut u32) -> super::super::super::Foundation::BOOLEAN;
    #[doc = "*Required features: `Win32_Security_Authentication_Identity`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn AuditEnumeratePerUserPolicy(ppauditsidarray: *mut *mut POLICY_AUDIT_SID_ARRAY) -> super::super::super::Foundation::BOOLEAN;
    #[doc = "*Required features: `Win32_Security_Authentication_Identity`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn AuditEnumerateSubCategories(pauditcategoryguid: *const ::windows_sys::core::GUID, bretrieveallsubcategories: super::super::super::Foundation::BOOLEAN, ppauditsubcategoriesarray: *mut *mut ::windows_sys::core::GUID, pdwcountreturned: *mut u32) -> super::super::super::Foundation::BOOLEAN;
    #[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
    pub fn AuditFree(buffer: *const ::core::ffi::c_void);
    #[doc = "*Required features: `Win32_Security_Authentication_Identity`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn AuditLookupCategoryGuidFromCategoryId(auditcategoryid: POLICY_AUDIT_EVENT_TYPE, pauditcategoryguid: *mut ::windows_sys::core::GUID) -> super::super::super::Foundation::BOOLEAN;
    #[doc = "*Required features: `Win32_Security_Authentication_Identity`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn AuditLookupCategoryIdFromCategoryGuid(pauditcategoryguid: *const ::windows_sys::core::GUID, pauditcategoryid: *mut POLICY_AUDIT_EVENT_TYPE) -> super::super::super::Foundation::BOOLEAN;
    #[doc = "*Required features: `Win32_Security_Authentication_Identity`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn AuditLookupCategoryNameA(pauditcategoryguid: *const ::windows_sys::core::GUID, ppszcategoryname: *mut super::super::super::Foundation::PSTR) -> super::super::super::Foundation::BOOLEAN;
    #[doc = "*Required features: `Win32_Security_Authentication_Identity`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn AuditLookupCategoryNameW(pauditcategoryguid: *const ::windows_sys::core::GUID, ppszcategoryname: *mut super::super::super::Foundation::PWSTR) -> super::super::super::Foundation::BOOLEAN;
    #[doc = "*Required features: `Win32_Security_Authentication_Identity`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn AuditLookupSubCategoryNameA(pauditsubcategoryguid: *const ::windows_sys::core::GUID, ppszsubcategoryname: *mut super::super::super::Foundation::PSTR) -> super::super::super::Foundation::BOOLEAN;
    #[doc = "*Required features: `Win32_Security_Authentication_Identity`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn AuditLookupSubCategoryNameW(pauditsubcategoryguid: *const ::windows_sys::core::GUID, ppszsubcategoryname: *mut super::super::super::Foundation::PWSTR) -> super::super::super::Foundation::BOOLEAN;
    #[doc = "*Required features: `Win32_Security_Authentication_Identity`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn AuditQueryGlobalSaclA(objecttypename: super::super::super::Foundation::PSTR, acl: *mut *mut super::super::ACL) -> super::super::super::Foundation::BOOLEAN;
    #[doc = "*Required features: `Win32_Security_Authentication_Identity`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn AuditQueryGlobalSaclW(objecttypename: super::super::super::Foundation::PWSTR, acl: *mut *mut super::super::ACL) -> super::super::super::Foundation::BOOLEAN;
    #[doc = "*Required features: `Win32_Security_Authentication_Identity`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn AuditQueryPerUserPolicy(psid: super::super::super::Foundation::PSID, psubcategoryguids: *const ::windows_sys::core::GUID, dwpolicycount: u32, ppauditpolicy: *mut *mut AUDIT_POLICY_INFORMATION) -> super::super::super::Foundation::BOOLEAN;
    #[doc = "*Required features: `Win32_Security_Authentication_Identity`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn AuditQuerySecurity(securityinformation: u32, ppsecuritydescriptor: *mut *mut super::super::SECURITY_DESCRIPTOR) -> super::super::super::Foundation::BOOLEAN;
    #[doc = "*Required features: `Win32_Security_Authentication_Identity`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn AuditQuerySystemPolicy(psubcategoryguids: *const ::windows_sys::core::GUID, dwpolicycount: u32, ppauditpolicy: *mut *mut AUDIT_POLICY_INFORMATION) -> super::super::super::Foundation::BOOLEAN;
    #[doc = "*Required features: `Win32_Security_Authentication_Identity`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn AuditSetGlobalSaclA(objecttypename: super::super::super::Foundation::PSTR, acl: *const super::super::ACL) -> super::super::super::Foundation::BOOLEAN;
    #[doc = "*Required features: `Win32_Security_Authentication_Identity`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn AuditSetGlobalSaclW(objecttypename: super::super::super::Foundation::PWSTR, acl: *const super::super::ACL) -> super::super::super::Foundation::BOOLEAN;
    #[doc = "*Required features: `Win32_Security_Authentication_Identity`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn AuditSetPerUserPolicy(psid: super::super::super::Foundation::PSID, pauditpolicy: *const AUDIT_POLICY_INFORMATION, dwpolicycount: u32) -> super::super::super::Foundation::BOOLEAN;
    #[doc = "*Required features: `Win32_Security_Authentication_Identity`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn AuditSetSecurity(securityinformation: u32, psecuritydescriptor: *const super::super::SECURITY_DESCRIPTOR) -> super::super::super::Foundation::BOOLEAN;
    #[doc = "*Required features: `Win32_Security_Authentication_Identity`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn AuditSetSystemPolicy(pauditpolicy: *const AUDIT_POLICY_INFORMATION, dwpolicycount: u32) -> super::super::super::Foundation::BOOLEAN;
    #[doc = "*Required features: `Win32_Security_Authentication_Identity`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ChangeAccountPasswordA(pszpackagename: *const i8, pszdomainname: *const i8, pszaccountname: *const i8, pszoldpassword: *const i8, psznewpassword: *const i8, bimpersonating: super::super::super::Foundation::BOOLEAN, dwreserved: u32, poutput: *mut SecBufferDesc) -> i32;
    #[doc = "*Required features: `Win32_Security_Authentication_Identity`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ChangeAccountPasswordW(pszpackagename: *const u16, pszdomainname: *const u16, pszaccountname: *const u16, pszoldpassword: *const u16, psznewpassword: *const u16, bimpersonating: super::super::super::Foundation::BOOLEAN, dwreserved: u32, poutput: *mut SecBufferDesc) -> i32;
    #[doc = "*Required features: `Win32_Security_Authentication_Identity`, `Win32_Security_Credentials`*"]
    #[cfg(feature = "Win32_Security_Credentials")]
    pub fn CompleteAuthToken(phcontext: *const super::super::Credentials::SecHandle, ptoken: *const SecBufferDesc) -> i32;
    #[doc = "*Required features: `Win32_Security_Authentication_Identity`, `Win32_Foundation`, `Win32_Security_Credentials`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Credentials"))]
    pub fn CredMarshalTargetInfo(intargetinfo: *const super::super::Credentials::CREDENTIAL_TARGET_INFORMATIONW, buffer: *mut *mut u16, buffersize: *mut u32) -> super::super::super::Foundation::NTSTATUS;
    #[doc = "*Required features: `Win32_Security_Authentication_Identity`, `Win32_Foundation`, `Win32_Security_Credentials`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Credentials"))]
    pub fn CredUnmarshalTargetInfo(buffer: *const u16, buffersize: u32, rettargetinfo: *mut *mut super::super::Credentials::CREDENTIAL_TARGET_INFORMATIONW, retactualsize: *mut u32) -> super::super::super::Foundation::NTSTATUS;
    #[doc = "*Required features: `Win32_Security_Authentication_Identity`, `Win32_Security_Credentials`*"]
    #[cfg(feature = "Win32_Security_Credentials")]
    pub fn DecryptMessage(phcontext: *const super::super::Credentials::SecHandle, pmessage: *const SecBufferDesc, messageseqno: u32, pfqop: *mut u32) -> i32;
    #[doc = "*Required features: `Win32_Security_Authentication_Identity`, `Win32_Security_Credentials`*"]
    #[cfg(feature = "Win32_Security_Credentials")]
    pub fn DeleteSecurityContext(phcontext: *const super::super::Credentials::SecHandle) -> i32;
    #[doc = "*Required features: `Win32_Security_Authentication_Identity`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DeleteSecurityPackageA(pszpackagename: super::super::super::Foundation::PSTR) -> i32;
    #[doc = "*Required features: `Win32_Security_Authentication_Identity`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DeleteSecurityPackageW(pszpackagename: super::super::super::Foundation::PWSTR) -> i32;
    #[doc = "*Required features: `Win32_Security_Authentication_Identity`, `Win32_Security_Credentials`*"]
    #[cfg(feature = "Win32_Security_Credentials")]
    pub fn EncryptMessage(phcontext: *const super::super::Credentials::SecHandle, fqop: u32, pmessage: *const SecBufferDesc, messageseqno: u32) -> i32;
    #[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
    pub fn EnumerateSecurityPackagesA(pcpackages: *mut u32, pppackageinfo: *mut *mut SecPkgInfoA) -> i32;
    #[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
    pub fn EnumerateSecurityPackagesW(pcpackages: *mut u32, pppackageinfo: *mut *mut SecPkgInfoW) -> i32;
    #[doc = "*Required features: `Win32_Security_Authentication_Identity`, `Win32_Security_Credentials`*"]
    #[cfg(feature = "Win32_Security_Credentials")]
    pub fn ExportSecurityContext(phcontext: *const super::super::Credentials::SecHandle, fflags: EXPORT_SECURITY_CONTEXT_FLAGS, ppackedcontext: *mut SecBuffer, ptoken: *mut *mut ::core::ffi::c_void) -> i32;
    #[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
    pub fn FreeContextBuffer(pvcontextbuffer: *mut ::core::ffi::c_void) -> i32;
    #[doc = "*Required features: `Win32_Security_Authentication_Identity`, `Win32_Security_Credentials`*"]
    #[cfg(feature = "Win32_Security_Credentials")]
    pub fn FreeCredentialsHandle(phcredential: *const super::super::Credentials::SecHandle) -> i32;
    #[doc = "*Required features: `Win32_Security_Authentication_Identity`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetComputerObjectNameA(nameformat: EXTENDED_NAME_FORMAT, lpnamebuffer: super::super::super::Foundation::PSTR, nsize: *mut u32) -> super::super::super::Foundation::BOOLEAN;
    #[doc = "*Required features: `Win32_Security_Authentication_Identity`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetComputerObjectNameW(nameformat: EXTENDED_NAME_FORMAT, lpnamebuffer: super::super::super::Foundation::PWSTR, nsize: *mut u32) -> super::super::super::Foundation::BOOLEAN;
    #[doc = "*Required features: `Win32_Security_Authentication_Identity`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetUserNameExA(nameformat: EXTENDED_NAME_FORMAT, lpnamebuffer: super::super::super::Foundation::PSTR, nsize: *mut u32) -> super::super::super::Foundation::BOOLEAN;
    #[doc = "*Required features: `Win32_Security_Authentication_Identity`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetUserNameExW(nameformat: EXTENDED_NAME_FORMAT, lpnamebuffer: super::super::super::Foundation::PWSTR, nsize: *mut u32) -> super::super::super::Foundation::BOOLEAN;
    #[doc = "*Required features: `Win32_Security_Authentication_Identity`, `Win32_Security_Credentials`*"]
    #[cfg(feature = "Win32_Security_Credentials")]
    pub fn ImpersonateSecurityContext(phcontext: *const super::super::Credentials::SecHandle) -> i32;
    #[doc = "*Required features: `Win32_Security_Authentication_Identity`, `Win32_Foundation`, `Win32_Security_Credentials`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Credentials"))]
    pub fn ImportSecurityContextA(pszpackage: super::super::super::Foundation::PSTR, ppackedcontext: *const SecBuffer, token: *const ::core::ffi::c_void, phcontext: *mut super::super::Credentials::SecHandle) -> i32;
    #[doc = "*Required features: `Win32_Security_Authentication_Identity`, `Win32_Foundation`, `Win32_Security_Credentials`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Credentials"))]
    pub fn ImportSecurityContextW(pszpackage: super::super::super::Foundation::PWSTR, ppackedcontext: *const SecBuffer, token: *const ::core::ffi::c_void, phcontext: *mut super::super::Credentials::SecHandle) -> i32;
    #[doc = "*Required features: `Win32_Security_Authentication_Identity`, `Win32_Foundation`, `Win32_Security_Credentials`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Credentials"))]
    pub fn InitSecurityInterfaceA() -> *mut SecurityFunctionTableA;
    #[doc = "*Required features: `Win32_Security_Authentication_Identity`, `Win32_Foundation`, `Win32_Security_Credentials`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Credentials"))]
    pub fn InitSecurityInterfaceW() -> *mut SecurityFunctionTableW;
    #[doc = "*Required features: `Win32_Security_Authentication_Identity`, `Win32_Security_Credentials`*"]
    #[cfg(feature = "Win32_Security_Credentials")]
    pub fn InitializeSecurityContextA(phcredential: *const super::super::Credentials::SecHandle, phcontext: *const super::super::Credentials::SecHandle, psztargetname: *const i8, fcontextreq: u32, reserved1: u32, targetdatarep: u32, pinput: *const SecBufferDesc, reserved2: u32, phnewcontext: *mut super::super::Credentials::SecHandle, poutput: *mut SecBufferDesc, pfcontextattr: *mut u32, ptsexpiry: *mut i64) -> i32;
    #[doc = "*Required features: `Win32_Security_Authentication_Identity`, `Win32_Security_Credentials`*"]
    #[cfg(feature = "Win32_Security_Credentials")]
    pub fn InitializeSecurityContextW(phcredential: *const super::super::Credentials::SecHandle, phcontext: *const super::super::Credentials::SecHandle, psztargetname: *const u16, fcontextreq: u32, reserved1: u32, targetdatarep: u32, pinput: *const SecBufferDesc, reserved2: u32, phnewcontext: *mut super::super::Credentials::SecHandle, poutput: *mut SecBufferDesc, pfcontextattr: *mut u32, ptsexpiry: *mut i64) -> i32;
    #[doc = "*Required features: `Win32_Security_Authentication_Identity`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn LsaAddAccountRights(policyhandle: *const ::core::ffi::c_void, accountsid: super::super::super::Foundation::PSID, userrights: *const super::super::super::Foundation::UNICODE_STRING, countofrights: u32) -> super::super::super::Foundation::NTSTATUS;
    #[doc = "*Required features: `Win32_Security_Authentication_Identity`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn LsaCallAuthenticationPackage(lsahandle: super::super::super::Foundation::HANDLE, authenticationpackage: u32, protocolsubmitbuffer: *const ::core::ffi::c_void, submitbufferlength: u32, protocolreturnbuffer: *mut *mut ::core::ffi::c_void, returnbufferlength: *mut u32, protocolstatus: *mut i32) -> super::super::super::Foundation::NTSTATUS;
    #[doc = "*Required features: `Win32_Security_Authentication_Identity`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn LsaClose(objecthandle: *const ::core::ffi::c_void) -> super::super::super::Foundation::NTSTATUS;
    #[doc = "*Required features: `Win32_Security_Authentication_Identity`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn LsaConnectUntrusted(lsahandle: *mut super::super::super::Foundation::HANDLE) -> super::super::super::Foundation::NTSTATUS;
    #[doc = "*Required features: `Win32_Security_Authentication_Identity`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn LsaCreateTrustedDomainEx(policyhandle: *const ::core::ffi::c_void, trusteddomaininformation: *const TRUSTED_DOMAIN_INFORMATION_EX, authenticationinformation: *const TRUSTED_DOMAIN_AUTH_INFORMATION, desiredaccess: u32, trusteddomainhandle: *mut *mut ::core::ffi::c_void) -> super::super::super::Foundation::NTSTATUS;
    #[doc = "*Required features: `Win32_Security_Authentication_Identity`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn LsaDeleteTrustedDomain(policyhandle: *const ::core::ffi::c_void, trusteddomainsid: super::super::super::Foundation::PSID) -> super::super::super::Foundation::NTSTATUS;
    #[doc = "*Required features: `Win32_Security_Authentication_Identity`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn LsaDeregisterLogonProcess(lsahandle: LsaHandle) -> super::super::super::Foundation::NTSTATUS;
    #[doc = "*Required features: `Win32_Security_Authentication_Identity`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn LsaEnumerateAccountRights(policyhandle: *const ::core::ffi::c_void, accountsid: super::super::super::Foundation::PSID, userrights: *mut *mut super::super::super::Foundation::UNICODE_STRING, countofrights: *mut u32) -> super::super::super::Foundation::NTSTATUS;
    #[doc = "*Required features: `Win32_Security_Authentication_Identity`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn LsaEnumerateAccountsWithUserRight(policyhandle: *const ::core::ffi::c_void, userright: *const super::super::super::Foundation::UNICODE_STRING, buffer: *mut *mut ::core::ffi::c_void, countreturned: *mut u32) -> super::super::super::Foundation::NTSTATUS;
    #[doc = "*Required features: `Win32_Security_Authentication_Identity`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn LsaEnumerateLogonSessions(logonsessioncount: *mut u32, logonsessionlist: *mut *mut super::super::super::Foundation::LUID) -> super::super::super::Foundation::NTSTATUS;
    #[doc = "*Required features: `Win32_Security_Authentication_Identity`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn LsaEnumerateTrustedDomains(policyhandle: *const ::core::ffi::c_void, enumerationcontext: *mut u32, buffer: *mut *mut ::core::ffi::c_void, preferedmaximumlength: u32, countreturned: *mut u32) -> super::super::super::Foundation::NTSTATUS;
    #[doc = "*Required features: `Win32_Security_Authentication_Identity`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn LsaEnumerateTrustedDomainsEx(policyhandle: *const ::core::ffi::c_void, enumerationcontext: *mut u32, buffer: *mut *mut ::core::ffi::c_void, preferedmaximumlength: u32, countreturned: *mut u32) -> super::super::super::Foundation::NTSTATUS;
    #[doc = "*Required features: `Win32_Security_Authentication_Identity`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn LsaFreeMemory(buffer: *const ::core::ffi::c_void) -> super::super::super::Foundation::NTSTATUS;
    #[doc = "*Required features: `Win32_Security_Authentication_Identity`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn LsaFreeReturnBuffer(buffer: *const ::core::ffi::c_void) -> super::super::super::Foundation::NTSTATUS;
    #[doc = "*Required features: `Win32_Security_Authentication_Identity`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn LsaGetAppliedCAPIDs(systemname: *const super::super::super::Foundation::UNICODE_STRING, capids: *mut *mut super::super::super::Foundation::PSID, capidcount: *mut u32) -> super::super::super::Foundation::NTSTATUS;
    #[doc = "*Required features: `Win32_Security_Authentication_Identity`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn LsaGetLogonSessionData(logonid: *const super::super::super::Foundation::LUID, pplogonsessiondata: *mut *mut SECURITY_LOGON_SESSION_DATA) -> super::super::super::Foundation::NTSTATUS;
    #[doc = "*Required features: `Win32_Security_Authentication_Identity`, `Win32_Foundation`, `Win32_System_Kernel`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
    pub fn LsaLogonUser(
        lsahandle: super::super::super::Foundation::HANDLE,
        originname: *const super::super::super::System::Kernel::STRING,
        logontype: SECURITY_LOGON_TYPE,
        authenticationpackage: u32,
        authenticationinformation: *const ::core::ffi::c_void,
        authenticationinformationlength: u32,
        localgroups: *const super::super::TOKEN_GROUPS,
        sourcecontext: *const super::super::TOKEN_SOURCE,
        profilebuffer: *mut *mut ::core::ffi::c_void,
        profilebufferlength: *mut u32,
        logonid: *mut super::super::super::Foundation::LUID,
        token: *mut super::super::super::Foundation::HANDLE,
        quotas: *mut super::super::QUOTA_LIMITS,
        substatus: *mut i32,
    ) -> super::super::super::Foundation::NTSTATUS;
    #[doc = "*Required features: `Win32_Security_Authentication_Identity`, `Win32_Foundation`, `Win32_System_Kernel`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
    pub fn LsaLookupAuthenticationPackage(lsahandle: super::super::super::Foundation::HANDLE, packagename: *const super::super::super::System::Kernel::STRING, authenticationpackage: *mut u32) -> super::super::super::Foundation::NTSTATUS;
    #[doc = "*Required features: `Win32_Security_Authentication_Identity`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn LsaLookupNames(policyhandle: *const ::core::ffi::c_void, count: u32, names: *const super::super::super::Foundation::UNICODE_STRING, referenceddomains: *mut *mut LSA_REFERENCED_DOMAIN_LIST, sids: *mut *mut LSA_TRANSLATED_SID) -> super::super::super::Foundation::NTSTATUS;
    #[doc = "*Required features: `Win32_Security_Authentication_Identity`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn LsaLookupNames2(policyhandle: *const ::core::ffi::c_void, flags: u32, count: u32, names: *const super::super::super::Foundation::UNICODE_STRING, referenceddomains: *mut *mut LSA_REFERENCED_DOMAIN_LIST, sids: *mut *mut LSA_TRANSLATED_SID2) -> super::super::super::Foundation::NTSTATUS;
    #[doc = "*Required features: `Win32_Security_Authentication_Identity`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn LsaLookupSids(policyhandle: *const ::core::ffi::c_void, count: u32, sids: *const super::super::super::Foundation::PSID, referenceddomains: *mut *mut LSA_REFERENCED_DOMAIN_LIST, names: *mut *mut LSA_TRANSLATED_NAME) -> super::super::super::Foundation::NTSTATUS;
    #[doc = "*Required features: `Win32_Security_Authentication_Identity`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn LsaLookupSids2(policyhandle: *const ::core::ffi::c_void, lookupoptions: u32, count: u32, sids: *const super::super::super::Foundation::PSID, referenceddomains: *mut *mut LSA_REFERENCED_DOMAIN_LIST, names: *mut *mut LSA_TRANSLATED_NAME) -> super::super::super::Foundation::NTSTATUS;
    #[doc = "*Required features: `Win32_Security_Authentication_Identity`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn LsaNtStatusToWinError(status: super::super::super::Foundation::NTSTATUS) -> u32;
    #[doc = "*Required features: `Win32_Security_Authentication_Identity`, `Win32_Foundation`, `Win32_System_WindowsProgramming`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_WindowsProgramming"))]
    pub fn LsaOpenPolicy(systemname: *const super::super::super::Foundation::UNICODE_STRING, objectattributes: *const super::super::super::System::WindowsProgramming::OBJECT_ATTRIBUTES, desiredaccess: u32, policyhandle: *mut *mut ::core::ffi::c_void) -> super::super::super::Foundation::NTSTATUS;
    #[doc = "*Required features: `Win32_Security_Authentication_Identity`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn LsaOpenTrustedDomainByName(policyhandle: *const ::core::ffi::c_void, trusteddomainname: *const super::super::super::Foundation::UNICODE_STRING, desiredaccess: u32, trusteddomainhandle: *mut *mut ::core::ffi::c_void) -> super::super::super::Foundation::NTSTATUS;
    #[doc = "*Required features: `Win32_Security_Authentication_Identity`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn LsaQueryCAPs(capids: *const super::super::super::Foundation::PSID, capidcount: u32, caps: *mut *mut CENTRAL_ACCESS_POLICY, capcount: *mut u32) -> super::super::super::Foundation::NTSTATUS;
    #[doc = "*Required features: `Win32_Security_Authentication_Identity`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn LsaQueryDomainInformationPolicy(policyhandle: *const ::core::ffi::c_void, informationclass: POLICY_DOMAIN_INFORMATION_CLASS, buffer: *mut *mut ::core::ffi::c_void) -> super::super::super::Foundation::NTSTATUS;
    #[doc = "*Required features: `Win32_Security_Authentication_Identity`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn LsaQueryForestTrustInformation(policyhandle: *const ::core::ffi::c_void, trusteddomainname: *const super::super::super::Foundation::UNICODE_STRING, foresttrustinfo: *mut *mut LSA_FOREST_TRUST_INFORMATION) -> super::super::super::Foundation::NTSTATUS;
    #[doc = "*Required features: `Win32_Security_Authentication_Identity`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn LsaQueryInformationPolicy(policyhandle: *const ::core::ffi::c_void, informationclass: POLICY_INFORMATION_CLASS, buffer: *mut *mut ::core::ffi::c_void) -> super::super::super::Foundation::NTSTATUS;
    #[doc = "*Required features: `Win32_Security_Authentication_Identity`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn LsaQueryTrustedDomainInfo(policyhandle: *const ::core::ffi::c_void, trusteddomainsid: super::super::super::Foundation::PSID, informationclass: TRUSTED_INFORMATION_CLASS, buffer: *mut *mut ::core::ffi::c_void) -> super::super::super::Foundation::NTSTATUS;
    #[doc = "*Required features: `Win32_Security_Authentication_Identity`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn LsaQueryTrustedDomainInfoByName(policyhandle: *const ::core::ffi::c_void, trusteddomainname: *const super::super::super::Foundation::UNICODE_STRING, informationclass: TRUSTED_INFORMATION_CLASS, buffer: *mut *mut ::core::ffi::c_void) -> super::super::super::Foundation::NTSTATUS;
    #[doc = "*Required features: `Win32_Security_Authentication_Identity`, `Win32_Foundation`, `Win32_System_Kernel`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
    pub fn LsaRegisterLogonProcess(logonprocessname: *const super::super::super::System::Kernel::STRING, lsahandle: *mut LsaHandle, securitymode: *mut u32) -> super::super::super::Foundation::NTSTATUS;
    #[doc = "*Required features: `Win32_Security_Authentication_Identity`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn LsaRegisterPolicyChangeNotification(informationclass: POLICY_NOTIFICATION_INFORMATION_CLASS, notificationeventhandle: super::super::super::Foundation::HANDLE) -> super::super::super::Foundation::NTSTATUS;
    #[doc = "*Required features: `Win32_Security_Authentication_Identity`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn LsaRemoveAccountRights(policyhandle: *const ::core::ffi::c_void, accountsid: super::super::super::Foundation::PSID, allrights: super::super::super::Foundation::BOOLEAN, userrights: *const super::super::super::Foundation::UNICODE_STRING, countofrights: u32) -> super::super::super::Foundation::NTSTATUS;
    #[doc = "*Required features: `Win32_Security_Authentication_Identity`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn LsaRetrievePrivateData(policyhandle: *const ::core::ffi::c_void, keyname: *const super::super::super::Foundation::UNICODE_STRING, privatedata: *mut *mut super::super::super::Foundation::UNICODE_STRING) -> super::super::super::Foundation::NTSTATUS;
    #[doc = "*Required features: `Win32_Security_Authentication_Identity`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn LsaSetCAPs(capdns: *const super::super::super::Foundation::UNICODE_STRING, capdncount: u32, flags: u32) -> super::super::super::Foundation::NTSTATUS;
    #[doc = "*Required features: `Win32_Security_Authentication_Identity`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn LsaSetDomainInformationPolicy(policyhandle: *const ::core::ffi::c_void, informationclass: POLICY_DOMAIN_INFORMATION_CLASS, buffer: *const ::core::ffi::c_void) -> super::super::super::Foundation::NTSTATUS;
    #[doc = "*Required features: `Win32_Security_Authentication_Identity`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn LsaSetForestTrustInformation(policyhandle: *const ::core::ffi::c_void, trusteddomainname: *const super::super::super::Foundation::UNICODE_STRING, foresttrustinfo: *const LSA_FOREST_TRUST_INFORMATION, checkonly: super::super::super::Foundation::BOOLEAN, collisioninfo: *mut *mut LSA_FOREST_TRUST_COLLISION_INFORMATION) -> super::super::super::Foundation::NTSTATUS;
    #[doc = "*Required features: `Win32_Security_Authentication_Identity`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn LsaSetInformationPolicy(policyhandle: *const ::core::ffi::c_void, informationclass: POLICY_INFORMATION_CLASS, buffer: *const ::core::ffi::c_void) -> super::super::super::Foundation::NTSTATUS;
    #[doc = "*Required features: `Win32_Security_Authentication_Identity`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn LsaSetTrustedDomainInfoByName(policyhandle: *const ::core::ffi::c_void, trusteddomainname: *const super::super::super::Foundation::UNICODE_STRING, informationclass: TRUSTED_INFORMATION_CLASS, buffer: *const ::core::ffi::c_void) -> super::super::super::Foundation::NTSTATUS;
    #[doc = "*Required features: `Win32_Security_Authentication_Identity`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn LsaSetTrustedDomainInformation(policyhandle: *const ::core::ffi::c_void, trusteddomainsid: super::super::super::Foundation::PSID, informationclass: TRUSTED_INFORMATION_CLASS, buffer: *const ::core::ffi::c_void) -> super::super::super::Foundation::NTSTATUS;
    #[doc = "*Required features: `Win32_Security_Authentication_Identity`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn LsaStorePrivateData(policyhandle: *const ::core::ffi::c_void, keyname: *const super::super::super::Foundation::UNICODE_STRING, privatedata: *const super::super::super::Foundation::UNICODE_STRING) -> super::super::super::Foundation::NTSTATUS;
    #[doc = "*Required features: `Win32_Security_Authentication_Identity`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn LsaUnregisterPolicyChangeNotification(informationclass: POLICY_NOTIFICATION_INFORMATION_CLASS, notificationeventhandle: super::super::super::Foundation::HANDLE) -> super::super::super::Foundation::NTSTATUS;
    #[doc = "*Required features: `Win32_Security_Authentication_Identity`, `Win32_Security_Credentials`*"]
    #[cfg(feature = "Win32_Security_Credentials")]
    pub fn MakeSignature(phcontext: *const super::super::Credentials::SecHandle, fqop: u32, pmessage: *const SecBufferDesc, messageseqno: u32) -> i32;
    #[doc = "*Required features: `Win32_Security_Authentication_Identity`, `Win32_Security_Credentials`*"]
    #[cfg(feature = "Win32_Security_Credentials")]
    pub fn QueryContextAttributesA(phcontext: *const super::super::Credentials::SecHandle, ulattribute: SECPKG_ATTR, pbuffer: *mut ::core::ffi::c_void) -> i32;
    #[doc = "*Required features: `Win32_Security_Authentication_Identity`, `Win32_Security_Credentials`*"]
    #[cfg(feature = "Win32_Security_Credentials")]
    pub fn QueryContextAttributesExA(phcontext: *const super::super::Credentials::SecHandle, ulattribute: SECPKG_ATTR, pbuffer: *mut ::core::ffi::c_void, cbbuffer: u32) -> i32;
    #[doc = "*Required features: `Win32_Security_Authentication_Identity`, `Win32_Security_Credentials`*"]
    #[cfg(feature = "Win32_Security_Credentials")]
    pub fn QueryContextAttributesExW(phcontext: *const super::super::Credentials::SecHandle, ulattribute: SECPKG_ATTR, pbuffer: *mut ::core::ffi::c_void, cbbuffer: u32) -> i32;
    #[doc = "*Required features: `Win32_Security_Authentication_Identity`, `Win32_Security_Credentials`*"]
    #[cfg(feature = "Win32_Security_Credentials")]
    pub fn QueryContextAttributesW(phcontext: *const super::super::Credentials::SecHandle, ulattribute: SECPKG_ATTR, pbuffer: *mut ::core::ffi::c_void) -> i32;
    #[doc = "*Required features: `Win32_Security_Authentication_Identity`, `Win32_Security_Credentials`*"]
    #[cfg(feature = "Win32_Security_Credentials")]
    pub fn QueryCredentialsAttributesA(phcredential: *const super::super::Credentials::SecHandle, ulattribute: u32, pbuffer: *mut ::core::ffi::c_void) -> i32;
    #[doc = "*Required features: `Win32_Security_Authentication_Identity`, `Win32_Security_Credentials`*"]
    #[cfg(feature = "Win32_Security_Credentials")]
    pub fn QueryCredentialsAttributesExA(phcredential: *const super::super::Credentials::SecHandle, ulattribute: u32, pbuffer: *mut ::core::ffi::c_void, cbbuffer: u32) -> i32;
    #[doc = "*Required features: `Win32_Security_Authentication_Identity`, `Win32_Security_Credentials`*"]
    #[cfg(feature = "Win32_Security_Credentials")]
    pub fn QueryCredentialsAttributesExW(phcredential: *const super::super::Credentials::SecHandle, ulattribute: u32, pbuffer: *mut ::core::ffi::c_void, cbbuffer: u32) -> i32;
    #[doc = "*Required features: `Win32_Security_Authentication_Identity`, `Win32_Security_Credentials`*"]
    #[cfg(feature = "Win32_Security_Credentials")]
    pub fn QueryCredentialsAttributesW(phcredential: *const super::super::Credentials::SecHandle, ulattribute: u32, pbuffer: *mut ::core::ffi::c_void) -> i32;
    #[doc = "*Required features: `Win32_Security_Authentication_Identity`, `Win32_Security_Credentials`*"]
    #[cfg(feature = "Win32_Security_Credentials")]
    pub fn QuerySecurityContextToken(phcontext: *const super::super::Credentials::SecHandle, token: *mut *mut ::core::ffi::c_void) -> i32;
    #[doc = "*Required features: `Win32_Security_Authentication_Identity`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn QuerySecurityPackageInfoA(pszpackagename: super::super::super::Foundation::PSTR, pppackageinfo: *mut *mut SecPkgInfoA) -> i32;
    #[doc = "*Required features: `Win32_Security_Authentication_Identity`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn QuerySecurityPackageInfoW(pszpackagename: super::super::super::Foundation::PWSTR, pppackageinfo: *mut *mut SecPkgInfoW) -> i32;
    #[doc = "*Required features: `Win32_Security_Authentication_Identity`, `Win32_Security_Credentials`*"]
    #[cfg(feature = "Win32_Security_Credentials")]
    pub fn RevertSecurityContext(phcontext: *const super::super::Credentials::SecHandle) -> i32;
    #[doc = "*Required features: `Win32_Security_Authentication_Identity`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SLAcquireGenuineTicket(ppticketblob: *mut *mut ::core::ffi::c_void, pcbticketblob: *mut u32, pwsztemplateid: super::super::super::Foundation::PWSTR, pwszserverurl: super::super::super::Foundation::PWSTR, pwszclienttoken: super::super::super::Foundation::PWSTR) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_Security_Authentication_Identity`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SLActivateProduct(hslc: *const ::core::ffi::c_void, pproductskuid: *const ::windows_sys::core::GUID, cbappspecificdata: u32, pvappspecificdata: *const ::core::ffi::c_void, pactivationinfo: *const SL_ACTIVATION_INFO_HEADER, pwszproxyserver: super::super::super::Foundation::PWSTR, wproxyport: u16) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
    pub fn SLClose(hslc: *const ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_Security_Authentication_Identity`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SLConsumeRight(hslc: *const ::core::ffi::c_void, pappid: *const ::windows_sys::core::GUID, pproductskuid: *const ::windows_sys::core::GUID, pwszrightname: super::super::super::Foundation::PWSTR, pvreserved: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_Security_Authentication_Identity`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SLDepositOfflineConfirmationId(hslc: *const ::core::ffi::c_void, pproductskuid: *const ::windows_sys::core::GUID, pwszinstallationid: super::super::super::Foundation::PWSTR, pwszconfirmationid: super::super::super::Foundation::PWSTR) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_Security_Authentication_Identity`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SLDepositOfflineConfirmationIdEx(hslc: *const ::core::ffi::c_void, pproductskuid: *const ::windows_sys::core::GUID, pactivationinfo: *const SL_ACTIVATION_INFO_HEADER, pwszinstallationid: super::super::super::Foundation::PWSTR, pwszconfirmationid: super::super::super::Foundation::PWSTR) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_Security_Authentication_Identity`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SLFireEvent(hslc: *const ::core::ffi::c_void, pwszeventid: super::super::super::Foundation::PWSTR, papplicationid: *const ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_Security_Authentication_Identity`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SLGenerateOfflineInstallationId(hslc: *const ::core::ffi::c_void, pproductskuid: *const ::windows_sys::core::GUID, ppwszinstallationid: *mut super::super::super::Foundation::PWSTR) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_Security_Authentication_Identity`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SLGenerateOfflineInstallationIdEx(hslc: *const ::core::ffi::c_void, pproductskuid: *const ::windows_sys::core::GUID, pactivationinfo: *const SL_ACTIVATION_INFO_HEADER, ppwszinstallationid: *mut super::super::super::Foundation::PWSTR) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_Security_Authentication_Identity`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SLGetApplicationInformation(hslc: *const ::core::ffi::c_void, papplicationid: *const ::windows_sys::core::GUID, pwszvaluename: super::super::super::Foundation::PWSTR, pedatatype: *mut SLDATATYPE, pcbvalue: *mut u32, ppbvalue: *mut *mut u8) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_Security_Authentication_Identity`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SLGetGenuineInformation(pqueryid: *const ::windows_sys::core::GUID, pwszvaluename: super::super::super::Foundation::PWSTR, pedatatype: *mut SLDATATYPE, pcbvalue: *mut u32, ppbvalue: *mut *mut u8) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
    pub fn SLGetInstalledProductKeyIds(hslc: *const ::core::ffi::c_void, pproductskuid: *const ::windows_sys::core::GUID, pnproductkeyids: *mut u32, ppproductkeyids: *mut *mut ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
    pub fn SLGetLicense(hslc: *const ::core::ffi::c_void, plicensefileid: *const ::windows_sys::core::GUID, pcblicensefile: *mut u32, ppblicensefile: *mut *mut u8) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
    pub fn SLGetLicenseFileId(hslc: *const ::core::ffi::c_void, cblicenseblob: u32, pblicenseblob: *const u8, plicensefileid: *mut ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_Security_Authentication_Identity`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SLGetLicenseInformation(hslc: *const ::core::ffi::c_void, psllicenseid: *const ::windows_sys::core::GUID, pwszvaluename: super::super::super::Foundation::PWSTR, pedatatype: *mut SLDATATYPE, pcbvalue: *mut u32, ppbvalue: *mut *mut u8) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_Security_Authentication_Identity`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SLGetLicensingStatusInformation(hslc: *const ::core::ffi::c_void, pappid: *const ::windows_sys::core::GUID, pproductskuid: *const ::windows_sys::core::GUID, pwszrightname: super::super::super::Foundation::PWSTR, pnstatuscount: *mut u32, pplicensingstatus: *mut *mut SL_LICENSING_STATUS) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_Security_Authentication_Identity`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SLGetPKeyId(hslc: *const ::core::ffi::c_void, pwszpkeyalgorithm: super::super::super::Foundation::PWSTR, pwszpkeystring: super::super::super::Foundation::PWSTR, cbpkeyspecificdata: u32, pbpkeyspecificdata: *const u8, ppkeyid: *mut ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_Security_Authentication_Identity`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SLGetPKeyInformation(hslc: *const ::core::ffi::c_void, ppkeyid: *const ::windows_sys::core::GUID, pwszvaluename: super::super::super::Foundation::PWSTR, pedatatype: *mut SLDATATYPE, pcbvalue: *mut u32, ppbvalue: *mut *mut u8) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_Security_Authentication_Identity`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SLGetPolicyInformation(hslc: *const ::core::ffi::c_void, pwszvaluename: super::super::super::Foundation::PWSTR, pedatatype: *mut SLDATATYPE, pcbvalue: *mut u32, ppbvalue: *mut *mut u8) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_Security_Authentication_Identity`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SLGetPolicyInformationDWORD(hslc: *const ::core::ffi::c_void, pwszvaluename: super::super::super::Foundation::PWSTR, pdwvalue: *mut u32) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_Security_Authentication_Identity`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SLGetProductSkuInformation(hslc: *const ::core::ffi::c_void, pproductskuid: *const ::windows_sys::core::GUID, pwszvaluename: super::super::super::Foundation::PWSTR, pedatatype: *mut SLDATATYPE, pcbvalue: *mut u32, ppbvalue: *mut *mut u8) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_Security_Authentication_Identity`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SLGetReferralInformation(hslc: *const ::core::ffi::c_void, ereferraltype: SLREFERRALTYPE, pskuorappid: *const ::windows_sys::core::GUID, pwszvaluename: super::super::super::Foundation::PWSTR, ppwszvalue: *mut super::super::super::Foundation::PWSTR) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
    pub fn SLGetSLIDList(hslc: *const ::core::ffi::c_void, equeryidtype: SLIDTYPE, pqueryid: *const ::windows_sys::core::GUID, ereturnidtype: SLIDTYPE, pnreturnids: *mut u32, ppreturnids: *mut *mut ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_Security_Authentication_Identity`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SLGetServerStatus(pwszserverurl: super::super::super::Foundation::PWSTR, pwszacquisitiontype: super::super::super::Foundation::PWSTR, pwszproxyserver: super::super::super::Foundation::PWSTR, wproxyport: u16, phrstatus: *mut ::windows_sys::core::HRESULT) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_Security_Authentication_Identity`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SLGetServiceInformation(hslc: *const ::core::ffi::c_void, pwszvaluename: super::super::super::Foundation::PWSTR, pedatatype: *mut SLDATATYPE, pcbvalue: *mut u32, ppbvalue: *mut *mut u8) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_Security_Authentication_Identity`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SLGetWindowsInformation(pwszvaluename: super::super::super::Foundation::PWSTR, pedatatype: *mut SLDATATYPE, pcbvalue: *mut u32, ppbvalue: *mut *mut u8) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_Security_Authentication_Identity`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SLGetWindowsInformationDWORD(pwszvaluename: super::super::super::Foundation::PWSTR, pdwvalue: *mut u32) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
    pub fn SLInstallLicense(hslc: *const ::core::ffi::c_void, cblicenseblob: u32, pblicenseblob: *const u8, plicensefileid: *mut ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_Security_Authentication_Identity`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SLInstallProofOfPurchase(hslc: *const ::core::ffi::c_void, pwszpkeyalgorithm: super::super::super::Foundation::PWSTR, pwszpkeystring: super::super::super::Foundation::PWSTR, cbpkeyspecificdata: u32, pbpkeyspecificdata: *const u8, ppkeyid: *mut ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
    pub fn SLIsGenuineLocal(pappid: *const ::windows_sys::core::GUID, pgenuinestate: *mut SL_GENUINE_STATE, puioptions: *mut SL_NONGENUINE_UI_OPTIONS) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
    pub fn SLOpen(phslc: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_Security_Authentication_Identity`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SLQueryLicenseValueFromApp(valuename: super::super::super::Foundation::PWSTR, valuetype: *mut u32, databuffer: *mut ::core::ffi::c_void, datasize: u32, resultdatasize: *mut u32) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_Security_Authentication_Identity`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SLRegisterEvent(hslc: *const ::core::ffi::c_void, pwszeventid: super::super::super::Foundation::PWSTR, papplicationid: *const ::windows_sys::core::GUID, hevent: super::super::super::Foundation::HANDLE) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
    pub fn SLSetCurrentProductKey(hslc: *const ::core::ffi::c_void, pproductskuid: *const ::windows_sys::core::GUID, pproductkeyid: *const ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_Security_Authentication_Identity`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SLSetGenuineInformation(pqueryid: *const ::windows_sys::core::GUID, pwszvaluename: super::super::super::Foundation::PWSTR, edatatype: SLDATATYPE, cbvalue: u32, pbvalue: *const u8) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
    pub fn SLUninstallLicense(hslc: *const ::core::ffi::c_void, plicensefileid: *const ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
    pub fn SLUninstallProofOfPurchase(hslc: *const ::core::ffi::c_void, ppkeyid: *const ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_Security_Authentication_Identity`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SLUnregisterEvent(hslc: *const ::core::ffi::c_void, pwszeventid: super::super::super::Foundation::PWSTR, papplicationid: *const ::windows_sys::core::GUID, hevent: super::super::super::Foundation::HANDLE) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_Security_Authentication_Identity`, `Win32_Security_Credentials`*"]
    #[cfg(feature = "Win32_Security_Credentials")]
    pub fn SaslAcceptSecurityContext(phcredential: *const super::super::Credentials::SecHandle, phcontext: *const super::super::Credentials::SecHandle, pinput: *const SecBufferDesc, fcontextreq: u32, targetdatarep: u32, phnewcontext: *mut super::super::Credentials::SecHandle, poutput: *mut SecBufferDesc, pfcontextattr: *mut u32, ptsexpiry: *mut i64) -> i32;
    #[doc = "*Required features: `Win32_Security_Authentication_Identity`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SaslEnumerateProfilesA(profilelist: *mut super::super::super::Foundation::PSTR, profilecount: *mut u32) -> i32;
    #[doc = "*Required features: `Win32_Security_Authentication_Identity`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SaslEnumerateProfilesW(profilelist: *mut super::super::super::Foundation::PWSTR, profilecount: *mut u32) -> i32;
    #[doc = "*Required features: `Win32_Security_Authentication_Identity`, `Win32_Security_Credentials`*"]
    #[cfg(feature = "Win32_Security_Credentials")]
    pub fn SaslGetContextOption(contexthandle: *const super::super::Credentials::SecHandle, option: u32, value: *mut ::core::ffi::c_void, size: u32, needed: *mut u32) -> i32;
    #[doc = "*Required features: `Win32_Security_Authentication_Identity`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SaslGetProfilePackageA(profilename: super::super::super::Foundation::PSTR, packageinfo: *mut *mut SecPkgInfoA) -> i32;
    #[doc = "*Required features: `Win32_Security_Authentication_Identity`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SaslGetProfilePackageW(profilename: super::super::super::Foundation::PWSTR, packageinfo: *mut *mut SecPkgInfoW) -> i32;
    #[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
    pub fn SaslIdentifyPackageA(pinput: *const SecBufferDesc, packageinfo: *mut *mut SecPkgInfoA) -> i32;
    #[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
    pub fn SaslIdentifyPackageW(pinput: *const SecBufferDesc, packageinfo: *mut *mut SecPkgInfoW) -> i32;
    #[doc = "*Required features: `Win32_Security_Authentication_Identity`, `Win32_Foundation`, `Win32_Security_Credentials`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Credentials"))]
    pub fn SaslInitializeSecurityContextA(phcredential: *const super::super::Credentials::SecHandle, phcontext: *const super::super::Credentials::SecHandle, psztargetname: super::super::super::Foundation::PSTR, fcontextreq: u32, reserved1: u32, targetdatarep: u32, pinput: *const SecBufferDesc, reserved2: u32, phnewcontext: *mut super::super::Credentials::SecHandle, poutput: *mut SecBufferDesc, pfcontextattr: *mut u32, ptsexpiry: *mut i64) -> i32;
    #[doc = "*Required features: `Win32_Security_Authentication_Identity`, `Win32_Foundation`, `Win32_Security_Credentials`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Credentials"))]
    pub fn SaslInitializeSecurityContextW(phcredential: *const super::super::Credentials::SecHandle, phcontext: *const super::super::Credentials::SecHandle, psztargetname: super::super::super::Foundation::PWSTR, fcontextreq: u32, reserved1: u32, targetdatarep: u32, pinput: *const SecBufferDesc, reserved2: u32, phnewcontext: *mut super::super::Credentials::SecHandle, poutput: *mut SecBufferDesc, pfcontextattr: *mut u32, ptsexpiry: *mut i64) -> i32;
    #[doc = "*Required features: `Win32_Security_Authentication_Identity`, `Win32_Security_Credentials`*"]
    #[cfg(feature = "Win32_Security_Credentials")]
    pub fn SaslSetContextOption(contexthandle: *const super::super::Credentials::SecHandle, option: u32, value: *const ::core::ffi::c_void, size: u32) -> i32;
    #[doc = "*Required features: `Win32_Security_Authentication_Identity`, `Win32_Security_Credentials`*"]
    #[cfg(feature = "Win32_Security_Credentials")]
    pub fn SetContextAttributesA(phcontext: *const super::super::Credentials::SecHandle, ulattribute: SECPKG_ATTR, pbuffer: *const ::core::ffi::c_void, cbbuffer: u32) -> i32;
    #[doc = "*Required features: `Win32_Security_Authentication_Identity`, `Win32_Security_Credentials`*"]
    #[cfg(feature = "Win32_Security_Credentials")]
    pub fn SetContextAttributesW(phcontext: *const super::super::Credentials::SecHandle, ulattribute: SECPKG_ATTR, pbuffer: *const ::core::ffi::c_void, cbbuffer: u32) -> i32;
    #[doc = "*Required features: `Win32_Security_Authentication_Identity`, `Win32_Security_Credentials`*"]
    #[cfg(feature = "Win32_Security_Credentials")]
    pub fn SetCredentialsAttributesA(phcredential: *const super::super::Credentials::SecHandle, ulattribute: u32, pbuffer: *const ::core::ffi::c_void, cbbuffer: u32) -> i32;
    #[doc = "*Required features: `Win32_Security_Authentication_Identity`, `Win32_Security_Credentials`*"]
    #[cfg(feature = "Win32_Security_Credentials")]
    pub fn SetCredentialsAttributesW(phcredential: *const super::super::Credentials::SecHandle, ulattribute: u32, pbuffer: *const ::core::ffi::c_void, cbbuffer: u32) -> i32;
    #[doc = "*Required features: `Win32_Security_Authentication_Identity`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SslCrackCertificate(pbcertificate: *mut u8, cbcertificate: u32, dwflags: u32, ppcertificate: *mut *mut X509Certificate) -> super::super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Security_Authentication_Identity`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SslEmptyCacheA(psztargetname: super::super::super::Foundation::PSTR, dwflags: u32) -> super::super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Security_Authentication_Identity`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SslEmptyCacheW(psztargetname: super::super::super::Foundation::PWSTR, dwflags: u32) -> super::super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Security_Authentication_Identity`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SslFreeCertificate(pcertificate: *mut X509Certificate);
    #[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
    pub fn SslGenerateRandomBits(prandomdata: *mut u8, crandomdata: i32);
    #[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
    pub fn SslGetExtensions(clienthello: *const u8, clienthellobytesize: u32, genericextensions: *mut SCH_EXTENSION_DATA, genericextensionscount: u8, bytestoread: *mut u32, flags: SchGetExtensionsOptions) -> i32;
    #[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
    pub fn SslGetMaximumKeySize(reserved: u32) -> u32;
    #[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
    pub fn SslGetServerIdentity(clienthello: *const u8, clienthellosize: u32, serveridentity: *mut *mut u8, serveridentitysize: *mut u32, flags: u32) -> i32;
    #[doc = "*Required features: `Win32_Security_Authentication_Identity`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SspiCompareAuthIdentities(authidentity1: *const ::core::ffi::c_void, authidentity2: *const ::core::ffi::c_void, samesupplieduser: *mut super::super::super::Foundation::BOOLEAN, samesuppliedidentity: *mut super::super::super::Foundation::BOOLEAN) -> i32;
    #[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
    pub fn SspiCopyAuthIdentity(authdata: *const ::core::ffi::c_void, authdatacopy: *mut *mut ::core::ffi::c_void) -> i32;
    #[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
    pub fn SspiDecryptAuthIdentity(encryptedauthdata: *mut ::core::ffi::c_void) -> i32;
    #[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
    pub fn SspiDecryptAuthIdentityEx(options: u32, encryptedauthdata: *mut ::core::ffi::c_void) -> i32;
    #[doc = "*Required features: `Win32_Security_Authentication_Identity`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SspiEncodeAuthIdentityAsStrings(pauthidentity: *const ::core::ffi::c_void, ppszusername: *mut super::super::super::Foundation::PWSTR, ppszdomainname: *mut super::super::super::Foundation::PWSTR, ppszpackedcredentialsstring: *mut super::super::super::Foundation::PWSTR) -> i32;
    #[doc = "*Required features: `Win32_Security_Authentication_Identity`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SspiEncodeStringsAsAuthIdentity(pszusername: super::super::super::Foundation::PWSTR, pszdomainname: super::super::super::Foundation::PWSTR, pszpackedcredentialsstring: super::super::super::Foundation::PWSTR, ppauthidentity: *mut *mut ::core::ffi::c_void) -> i32;
    #[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
    pub fn SspiEncryptAuthIdentity(authdata: *mut ::core::ffi::c_void) -> i32;
    #[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
    pub fn SspiEncryptAuthIdentityEx(options: u32, authdata: *mut ::core::ffi::c_void) -> i32;
    #[doc = "*Required features: `Win32_Security_Authentication_Identity`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SspiExcludePackage(authidentity: *const ::core::ffi::c_void, pszpackagename: super::super::super::Foundation::PWSTR, ppnewauthidentity: *mut *mut ::core::ffi::c_void) -> i32;
    #[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
    pub fn SspiFreeAuthIdentity(authdata: *const ::core::ffi::c_void);
    #[doc = "*Required features: `Win32_Security_Authentication_Identity`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SspiGetTargetHostName(psztargetname: super::super::super::Foundation::PWSTR, pszhostname: *mut super::super::super::Foundation::PWSTR) -> i32;
    #[doc = "*Required features: `Win32_Security_Authentication_Identity`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SspiIsAuthIdentityEncrypted(encryptedauthdata: *const ::core::ffi::c_void) -> super::super::super::Foundation::BOOLEAN;
    #[doc = "*Required features: `Win32_Security_Authentication_Identity`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SspiIsPromptingNeeded(errororntstatus: u32) -> super::super::super::Foundation::BOOLEAN;
    #[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
    pub fn SspiLocalFree(databuffer: *const ::core::ffi::c_void);
    #[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
    pub fn SspiMarshalAuthIdentity(authidentity: *const ::core::ffi::c_void, authidentitylength: *mut u32, authidentitybytearray: *mut *mut i8) -> i32;
    #[doc = "*Required features: `Win32_Security_Authentication_Identity`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SspiPrepareForCredRead(authidentity: *const ::core::ffi::c_void, psztargetname: super::super::super::Foundation::PWSTR, pcredmancredentialtype: *mut u32, ppszcredmantargetname: *mut super::super::super::Foundation::PWSTR) -> i32;
    #[doc = "*Required features: `Win32_Security_Authentication_Identity`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SspiPrepareForCredWrite(authidentity: *const ::core::ffi::c_void, psztargetname: super::super::super::Foundation::PWSTR, pcredmancredentialtype: *mut u32, ppszcredmantargetname: *mut super::super::super::Foundation::PWSTR, ppszcredmanusername: *mut super::super::super::Foundation::PWSTR, ppcredentialblob: *mut *mut u8, pcredentialblobsize: *mut u32) -> i32;
    #[doc = "*Required features: `Win32_Security_Authentication_Identity`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SspiPromptForCredentialsA(psztargetname: super::super::super::Foundation::PSTR, puiinfo: *const ::core::ffi::c_void, dwautherror: u32, pszpackage: super::super::super::Foundation::PSTR, pinputauthidentity: *const ::core::ffi::c_void, ppauthidentity: *mut *mut ::core::ffi::c_void, pfsave: *mut i32, dwflags: u32) -> u32;
    #[doc = "*Required features: `Win32_Security_Authentication_Identity`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SspiPromptForCredentialsW(psztargetname: super::super::super::Foundation::PWSTR, puiinfo: *const ::core::ffi::c_void, dwautherror: u32, pszpackage: super::super::super::Foundation::PWSTR, pinputauthidentity: *const ::core::ffi::c_void, ppauthidentity: *mut *mut ::core::ffi::c_void, pfsave: *mut i32, dwflags: u32) -> u32;
    #[doc = "*Required features: `Win32_Security_Authentication_Identity`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SspiUnmarshalAuthIdentity(authidentitylength: u32, authidentitybytearray: super::super::super::Foundation::PSTR, ppauthidentity: *mut *mut ::core::ffi::c_void) -> i32;
    #[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
    pub fn SspiValidateAuthIdentity(authdata: *const ::core::ffi::c_void) -> i32;
    #[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
    pub fn SspiZeroAuthIdentity(authdata: *const ::core::ffi::c_void);
    #[doc = "*Required features: `Win32_Security_Authentication_Identity`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SystemFunction036(randombuffer: *mut ::core::ffi::c_void, randombufferlength: u32) -> super::super::super::Foundation::BOOLEAN;
    #[doc = "*Required features: `Win32_Security_Authentication_Identity`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SystemFunction040(memory: *mut ::core::ffi::c_void, memorysize: u32, optionflags: u32) -> super::super::super::Foundation::NTSTATUS;
    #[doc = "*Required features: `Win32_Security_Authentication_Identity`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SystemFunction041(memory: *mut ::core::ffi::c_void, memorysize: u32, optionflags: u32) -> super::super::super::Foundation::NTSTATUS;
    #[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
    pub fn TokenBindingDeleteAllBindings() -> i32;
    #[doc = "*Required features: `Win32_Security_Authentication_Identity`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn TokenBindingDeleteBinding(targeturl: super::super::super::Foundation::PWSTR) -> i32;
    #[doc = "*Required features: `Win32_Security_Authentication_Identity`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn TokenBindingGenerateBinding(keytype: TOKENBINDING_KEY_PARAMETERS_TYPE, targeturl: super::super::super::Foundation::PWSTR, bindingtype: TOKENBINDING_TYPE, tlsekm: *const ::core::ffi::c_void, tlsekmsize: u32, extensionformat: TOKENBINDING_EXTENSION_FORMAT, extensiondata: *const ::core::ffi::c_void, tokenbinding: *mut *mut ::core::ffi::c_void, tokenbindingsize: *mut u32, resultdata: *mut *mut TOKENBINDING_RESULT_DATA) -> i32;
    #[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
    pub fn TokenBindingGenerateID(keytype: TOKENBINDING_KEY_PARAMETERS_TYPE, publickey: *const ::core::ffi::c_void, publickeysize: u32, resultdata: *mut *mut TOKENBINDING_RESULT_DATA) -> i32;
    #[doc = "*Required features: `Win32_Security_Authentication_Identity`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn TokenBindingGenerateIDForUri(keytype: TOKENBINDING_KEY_PARAMETERS_TYPE, targeturi: super::super::super::Foundation::PWSTR, resultdata: *mut *mut TOKENBINDING_RESULT_DATA) -> i32;
    #[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
    pub fn TokenBindingGenerateMessage(tokenbindings: *const *const ::core::ffi::c_void, tokenbindingssize: *const u32, tokenbindingscount: u32, tokenbindingmessage: *mut *mut ::core::ffi::c_void, tokenbindingmessagesize: *mut u32) -> i32;
    #[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
    pub fn TokenBindingGetHighestSupportedVersion(majorversion: *mut u8, minorversion: *mut u8) -> i32;
    #[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
    pub fn TokenBindingGetKeyTypesClient(keytypes: *mut *mut TOKENBINDING_KEY_TYPES) -> i32;
    #[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
    pub fn TokenBindingGetKeyTypesServer(keytypes: *mut *mut TOKENBINDING_KEY_TYPES) -> i32;
    #[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
    pub fn TokenBindingVerifyMessage(tokenbindingmessage: *const ::core::ffi::c_void, tokenbindingmessagesize: u32, keytype: TOKENBINDING_KEY_PARAMETERS_TYPE, tlsekm: *const ::core::ffi::c_void, tlsekmsize: u32, resultlist: *mut *mut TOKENBINDING_RESULT_LIST) -> i32;
    #[doc = "*Required features: `Win32_Security_Authentication_Identity`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn TranslateNameA(lpaccountname: super::super::super::Foundation::PSTR, accountnameformat: EXTENDED_NAME_FORMAT, desirednameformat: EXTENDED_NAME_FORMAT, lptranslatedname: super::super::super::Foundation::PSTR, nsize: *mut u32) -> super::super::super::Foundation::BOOLEAN;
    #[doc = "*Required features: `Win32_Security_Authentication_Identity`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn TranslateNameW(lpaccountname: super::super::super::Foundation::PWSTR, accountnameformat: EXTENDED_NAME_FORMAT, desirednameformat: EXTENDED_NAME_FORMAT, lptranslatedname: super::super::super::Foundation::PWSTR, nsize: *mut u32) -> super::super::super::Foundation::BOOLEAN;
    #[doc = "*Required features: `Win32_Security_Authentication_Identity`, `Win32_Security_Credentials`*"]
    #[cfg(feature = "Win32_Security_Credentials")]
    pub fn VerifySignature(phcontext: *const super::super::Credentials::SecHandle, pmessage: *const SecBufferDesc, messageseqno: u32, pfqop: *mut u32) -> i32;
}
pub struct ACCEPT_SECURITY_CONTEXT_CONTEXT_REQ(i32);
pub struct ACCEPT_SECURITY_CONTEXT_FN(i32);
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const ACCOUNT_ADJUST_PRIVILEGES: i32 = 2i32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const ACCOUNT_ADJUST_QUOTAS: i32 = 4i32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const ACCOUNT_ADJUST_SYSTEM_ACCESS: i32 = 8i32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const ACCOUNT_VIEW: i32 = 1i32;
pub struct ACQUIRE_CREDENTIALS_HANDLE_FN_A(i32);
pub struct ACQUIRE_CREDENTIALS_HANDLE_FN_W(i32);
pub struct ADD_CREDENTIALS_FN_A(i32);
pub struct ADD_CREDENTIALS_FN_W(i32);
pub struct APPLY_CONTROL_TOKEN_FN(i32);
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const ASC_REQ_ALLOW_CONTEXT_REPLAY: u32 = 4194304u32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const ASC_REQ_ALLOW_MISSING_BINDINGS: u32 = 268435456u32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const ASC_REQ_ALLOW_NON_USER_LOGONS: u32 = 2097152u32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const ASC_REQ_ALLOW_NULL_SESSION: u32 = 1048576u32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const ASC_REQ_CALL_LEVEL: u32 = 4096u32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const ASC_REQ_CONFIDENTIALITY: u32 = 16u32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const ASC_REQ_DATAGRAM: u32 = 1024u32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const ASC_REQ_FRAGMENT_SUPPLIED: u32 = 8192u32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const ASC_REQ_FRAGMENT_TO_FIT: u32 = 8388608u32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const ASC_REQ_IDENTIFY: u32 = 524288u32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const ASC_REQ_INTEGRITY: u32 = 131072u32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const ASC_REQ_LICENSING: u32 = 262144u32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const ASC_REQ_MESSAGES: u64 = 4294967296u64;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const ASC_REQ_MUTUAL_AUTH: u32 = 2u32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const ASC_REQ_NO_TOKEN: u32 = 16777216u32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const ASC_REQ_PROXY_BINDINGS: u32 = 67108864u32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const ASC_REQ_SESSION_TICKET: u32 = 64u32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const ASC_REQ_USE_DCE_STYLE: u32 = 512u32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const ASC_REQ_USE_SESSION_KEY: u32 = 32u32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const ASC_RET_ALLOCATED_MEMORY: u32 = 256u32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const ASC_RET_ALLOW_CONTEXT_REPLAY: u32 = 4194304u32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const ASC_RET_ALLOW_NON_USER_LOGONS: u32 = 2097152u32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const ASC_RET_CALL_LEVEL: u32 = 8192u32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const ASC_RET_CONFIDENTIALITY: u32 = 16u32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const ASC_RET_CONNECTION: u32 = 2048u32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const ASC_RET_DATAGRAM: u32 = 1024u32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const ASC_RET_DELEGATE: u32 = 1u32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const ASC_RET_EXTENDED_ERROR: u32 = 32768u32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const ASC_RET_FRAGMENT_ONLY: u32 = 8388608u32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const ASC_RET_IDENTIFY: u32 = 524288u32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const ASC_RET_INTEGRITY: u32 = 131072u32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const ASC_RET_LICENSING: u32 = 262144u32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const ASC_RET_MESSAGES: u64 = 4294967296u64;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const ASC_RET_MUTUAL_AUTH: u32 = 2u32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const ASC_RET_NO_ADDITIONAL_TOKEN: u32 = 33554432u32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const ASC_RET_NO_TOKEN: u32 = 16777216u32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const ASC_RET_NULL_SESSION: u32 = 1048576u32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const ASC_RET_REPLAY_DETECT: u32 = 4u32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const ASC_RET_SEQUENCE_DETECT: u32 = 8u32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const ASC_RET_SESSION_TICKET: u32 = 64u32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const ASC_RET_STREAM: u32 = 65536u32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const ASC_RET_THIRD_LEG_FAILED: u32 = 16384u32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const ASC_RET_USED_DCE_STYLE: u32 = 512u32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const ASC_RET_USE_SESSION_KEY: u32 = 32u32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const AUDIT_ENUMERATE_USERS: u32 = 16u32;
pub struct AUDIT_POLICY_INFORMATION(i32);
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const AUDIT_QUERY_MISC_POLICY: u32 = 64u32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const AUDIT_QUERY_SYSTEM_POLICY: u32 = 2u32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const AUDIT_QUERY_USER_POLICY: u32 = 8u32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const AUDIT_SET_MISC_POLICY: u32 = 32u32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const AUDIT_SET_SYSTEM_POLICY: u32 = 1u32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const AUDIT_SET_USER_POLICY: u32 = 4u32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const AUTH_REQ_ALLOW_ENC_TKT_IN_SKEY: u32 = 32u32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const AUTH_REQ_ALLOW_FORWARDABLE: u32 = 1u32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const AUTH_REQ_ALLOW_NOADDRESS: u32 = 16u32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const AUTH_REQ_ALLOW_POSTDATE: u32 = 4u32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const AUTH_REQ_ALLOW_PROXIABLE: u32 = 2u32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const AUTH_REQ_ALLOW_RENEWABLE: u32 = 8u32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const AUTH_REQ_ALLOW_S4U_DELEGATE: u32 = 2048u32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const AUTH_REQ_ALLOW_VALIDATE: u32 = 64u32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const AUTH_REQ_OK_AS_DELEGATE: u32 = 256u32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const AUTH_REQ_PREAUTH_REQUIRED: u32 = 512u32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const AUTH_REQ_TRANSITIVE_TRUST: u32 = 1024u32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const AUTH_REQ_VALIDATE_CLIENT: u32 = 128u32;
pub const Audit_AccountLogon: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 1771542608, data2: 31098, data3: 4569, data4: [190, 211, 80, 80, 84, 80, 48, 48] };
pub const Audit_AccountLogon_CredentialValidation: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 214864447, data2: 27054, data3: 4569, data4: [190, 211, 80, 80, 84, 80, 48, 48] };
pub const Audit_AccountLogon_KerbCredentialValidation: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 214864450, data2: 27054, data3: 4569, data4: [190, 211, 80, 80, 84, 80, 48, 48] };
pub const Audit_AccountLogon_Kerberos: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 214864448, data2: 27054, data3: 4569, data4: [190, 211, 80, 80, 84, 80, 48, 48] };
pub const Audit_AccountLogon_Others: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 214864449, data2: 27054, data3: 4569, data4: [190, 211, 80, 80, 84, 80, 48, 48] };
pub const Audit_AccountManagement: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 1771542606, data2: 31098, data3: 4569, data4: [190, 211, 80, 80, 84, 80, 48, 48] };
pub const Audit_AccountManagement_ApplicationGroup: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 214864441, data2: 27054, data3: 4569, data4: [190, 211, 80, 80, 84, 80, 48, 48] };
pub const Audit_AccountManagement_ComputerAccount: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 214864438, data2: 27054, data3: 4569, data4: [190, 211, 80, 80, 84, 80, 48, 48] };
pub const Audit_AccountManagement_DistributionGroup: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 214864440, data2: 27054, data3: 4569, data4: [190, 211, 80, 80, 84, 80, 48, 48] };
pub const Audit_AccountManagement_Others: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 214864442, data2: 27054, data3: 4569, data4: [190, 211, 80, 80, 84, 80, 48, 48] };
pub const Audit_AccountManagement_SecurityGroup: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 214864439, data2: 27054, data3: 4569, data4: [190, 211, 80, 80, 84, 80, 48, 48] };
pub const Audit_AccountManagement_UserAccount: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 214864437, data2: 27054, data3: 4569, data4: [190, 211, 80, 80, 84, 80, 48, 48] };
pub const Audit_DSAccess_DSAccess: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 214864443, data2: 27054, data3: 4569, data4: [190, 211, 80, 80, 84, 80, 48, 48] };
pub const Audit_DetailedTracking: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 1771542604, data2: 31098, data3: 4569, data4: [190, 211, 80, 80, 84, 80, 48, 48] };
pub const Audit_DetailedTracking_DpapiActivity: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 214864429, data2: 27054, data3: 4569, data4: [190, 211, 80, 80, 84, 80, 48, 48] };
pub const Audit_DetailedTracking_PnpActivity: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 214864456, data2: 27054, data3: 4569, data4: [190, 211, 80, 80, 84, 80, 48, 48] };
pub const Audit_DetailedTracking_ProcessCreation: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 214864427, data2: 27054, data3: 4569, data4: [190, 211, 80, 80, 84, 80, 48, 48] };
pub const Audit_DetailedTracking_ProcessTermination: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 214864428, data2: 27054, data3: 4569, data4: [190, 211, 80, 80, 84, 80, 48, 48] };
pub const Audit_DetailedTracking_RpcCall: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 214864430, data2: 27054, data3: 4569, data4: [190, 211, 80, 80, 84, 80, 48, 48] };
pub const Audit_DetailedTracking_TokenRightAdjusted: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 214864458, data2: 27054, data3: 4569, data4: [190, 211, 80, 80, 84, 80, 48, 48] };
pub const Audit_DirectoryServiceAccess: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 1771542607, data2: 31098, data3: 4569, data4: [190, 211, 80, 80, 84, 80, 48, 48] };
pub const Audit_DsAccess_AdAuditChanges: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 214864444, data2: 27054, data3: 4569, data4: [190, 211, 80, 80, 84, 80, 48, 48] };
pub const Audit_Ds_DetailedReplication: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 214864446, data2: 27054, data3: 4569, data4: [190, 211, 80, 80, 84, 80, 48, 48] };
pub const Audit_Ds_Replication: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 214864445, data2: 27054, data3: 4569, data4: [190, 211, 80, 80, 84, 80, 48, 48] };
pub const Audit_Logon: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 1771542601, data2: 31098, data3: 4569, data4: [190, 211, 80, 80, 84, 80, 48, 48] };
pub const Audit_Logon_AccountLockout: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 214864407, data2: 27054, data3: 4569, data4: [190, 211, 80, 80, 84, 80, 48, 48] };
pub const Audit_Logon_Claims: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 214864455, data2: 27054, data3: 4569, data4: [190, 211, 80, 80, 84, 80, 48, 48] };
pub const Audit_Logon_Groups: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 214864457, data2: 27054, data3: 4569, data4: [190, 211, 80, 80, 84, 80, 48, 48] };
pub const Audit_Logon_IPSecMainMode: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 214864408, data2: 27054, data3: 4569, data4: [190, 211, 80, 80, 84, 80, 48, 48] };
pub const Audit_Logon_IPSecQuickMode: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 214864409, data2: 27054, data3: 4569, data4: [190, 211, 80, 80, 84, 80, 48, 48] };
pub const Audit_Logon_IPSecUserMode: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 214864410, data2: 27054, data3: 4569, data4: [190, 211, 80, 80, 84, 80, 48, 48] };
pub const Audit_Logon_Logoff: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 214864406, data2: 27054, data3: 4569, data4: [190, 211, 80, 80, 84, 80, 48, 48] };
pub const Audit_Logon_Logon: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 214864405, data2: 27054, data3: 4569, data4: [190, 211, 80, 80, 84, 80, 48, 48] };
pub const Audit_Logon_NPS: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 214864451, data2: 27054, data3: 4569, data4: [190, 211, 80, 80, 84, 80, 48, 48] };
pub const Audit_Logon_Others: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 214864412, data2: 27054, data3: 4569, data4: [190, 211, 80, 80, 84, 80, 48, 48] };
pub const Audit_Logon_SpecialLogon: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 214864411, data2: 27054, data3: 4569, data4: [190, 211, 80, 80, 84, 80, 48, 48] };
pub const Audit_ObjectAccess: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 1771542602, data2: 31098, data3: 4569, data4: [190, 211, 80, 80, 84, 80, 48, 48] };
pub const Audit_ObjectAccess_ApplicationGenerated: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 214864418, data2: 27054, data3: 4569, data4: [190, 211, 80, 80, 84, 80, 48, 48] };
pub const Audit_ObjectAccess_CbacStaging: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 214864454, data2: 27054, data3: 4569, data4: [190, 211, 80, 80, 84, 80, 48, 48] };
pub const Audit_ObjectAccess_CertificationServices: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 214864417, data2: 27054, data3: 4569, data4: [190, 211, 80, 80, 84, 80, 48, 48] };
pub const Audit_ObjectAccess_DetailedFileShare: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 214864452, data2: 27054, data3: 4569, data4: [190, 211, 80, 80, 84, 80, 48, 48] };
pub const Audit_ObjectAccess_FileSystem: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 214864413, data2: 27054, data3: 4569, data4: [190, 211, 80, 80, 84, 80, 48, 48] };
pub const Audit_ObjectAccess_FirewallConnection: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 214864422, data2: 27054, data3: 4569, data4: [190, 211, 80, 80, 84, 80, 48, 48] };
pub const Audit_ObjectAccess_FirewallPacketDrops: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 214864421, data2: 27054, data3: 4569, data4: [190, 211, 80, 80, 84, 80, 48, 48] };
pub const Audit_ObjectAccess_Handle: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 214864419, data2: 27054, data3: 4569, data4: [190, 211, 80, 80, 84, 80, 48, 48] };
pub const Audit_ObjectAccess_Kernel: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 214864415, data2: 27054, data3: 4569, data4: [190, 211, 80, 80, 84, 80, 48, 48] };
pub const Audit_ObjectAccess_Other: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 214864423, data2: 27054, data3: 4569, data4: [190, 211, 80, 80, 84, 80, 48, 48] };
pub const Audit_ObjectAccess_Registry: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 214864414, data2: 27054, data3: 4569, data4: [190, 211, 80, 80, 84, 80, 48, 48] };
pub const Audit_ObjectAccess_RemovableStorage: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 214864453, data2: 27054, data3: 4569, data4: [190, 211, 80, 80, 84, 80, 48, 48] };
pub const Audit_ObjectAccess_Sam: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 214864416, data2: 27054, data3: 4569, data4: [190, 211, 80, 80, 84, 80, 48, 48] };
pub const Audit_ObjectAccess_Share: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 214864420, data2: 27054, data3: 4569, data4: [190, 211, 80, 80, 84, 80, 48, 48] };
pub const Audit_PolicyChange: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 1771542605, data2: 31098, data3: 4569, data4: [190, 211, 80, 80, 84, 80, 48, 48] };
pub const Audit_PolicyChange_AuditPolicy: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 214864431, data2: 27054, data3: 4569, data4: [190, 211, 80, 80, 84, 80, 48, 48] };
pub const Audit_PolicyChange_AuthenticationPolicy: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 214864432, data2: 27054, data3: 4569, data4: [190, 211, 80, 80, 84, 80, 48, 48] };
pub const Audit_PolicyChange_AuthorizationPolicy: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 214864433, data2: 27054, data3: 4569, data4: [190, 211, 80, 80, 84, 80, 48, 48] };
pub const Audit_PolicyChange_MpsscvRulePolicy: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 214864434, data2: 27054, data3: 4569, data4: [190, 211, 80, 80, 84, 80, 48, 48] };
pub const Audit_PolicyChange_Others: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 214864436, data2: 27054, data3: 4569, data4: [190, 211, 80, 80, 84, 80, 48, 48] };
pub const Audit_PolicyChange_WfpIPSecPolicy: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 214864435, data2: 27054, data3: 4569, data4: [190, 211, 80, 80, 84, 80, 48, 48] };
pub const Audit_PrivilegeUse: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 1771542603, data2: 31098, data3: 4569, data4: [190, 211, 80, 80, 84, 80, 48, 48] };
pub const Audit_PrivilegeUse_NonSensitive: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 214864425, data2: 27054, data3: 4569, data4: [190, 211, 80, 80, 84, 80, 48, 48] };
pub const Audit_PrivilegeUse_Others: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 214864426, data2: 27054, data3: 4569, data4: [190, 211, 80, 80, 84, 80, 48, 48] };
pub const Audit_PrivilegeUse_Sensitive: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 214864424, data2: 27054, data3: 4569, data4: [190, 211, 80, 80, 84, 80, 48, 48] };
pub const Audit_System: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 1771542600, data2: 31098, data3: 4569, data4: [190, 211, 80, 80, 84, 80, 48, 48] };
pub const Audit_System_IPSecDriverEvents: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 214864403, data2: 27054, data3: 4569, data4: [190, 211, 80, 80, 84, 80, 48, 48] };
pub const Audit_System_Integrity: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 214864402, data2: 27054, data3: 4569, data4: [190, 211, 80, 80, 84, 80, 48, 48] };
pub const Audit_System_Others: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 214864404, data2: 27054, data3: 4569, data4: [190, 211, 80, 80, 84, 80, 48, 48] };
pub const Audit_System_SecurityStateChange: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 214864400, data2: 27054, data3: 4569, data4: [190, 211, 80, 80, 84, 80, 48, 48] };
pub const Audit_System_SecuritySubsystemExtension: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 214864401, data2: 27054, data3: 4569, data4: [190, 211, 80, 80, 84, 80, 48, 48] };
pub struct CENTRAL_ACCESS_POLICY(i32);
pub struct CENTRAL_ACCESS_POLICY_ENTRY(i32);
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const CENTRAL_ACCESS_POLICY_OWNER_RIGHTS_PRESENT_FLAG: u32 = 1u32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const CENTRAL_ACCESS_POLICY_STAGED_FLAG: u32 = 65536u32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const CENTRAL_ACCESS_POLICY_STAGED_OWNER_RIGHTS_PRESENT_FLAG: u32 = 256u32;
pub struct CHANGE_PASSWORD_FN_A(i32);
pub struct CHANGE_PASSWORD_FN_W(i32);
pub struct CLEAR_BLOCK(i32);
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const CLEAR_BLOCK_LENGTH: u32 = 8u32;
pub struct COMPLETE_AUTH_TOKEN_FN(i32);
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const CREDP_FLAGS_CLEAR_PASSWORD: u32 = 8u32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const CREDP_FLAGS_DONT_CACHE_TI: u32 = 4u32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const CREDP_FLAGS_IN_PROCESS: u32 = 1u32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const CREDP_FLAGS_TRUSTED_CALLER: u32 = 32u32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const CREDP_FLAGS_USER_ENCRYPTED_PASSWORD: u32 = 16u32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const CREDP_FLAGS_USE_MIDL_HEAP: u32 = 2u32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const CREDP_FLAGS_VALIDATE_PROXY_TARGET: u32 = 64u32;
pub struct CRED_FETCH(i32);
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const CRED_MARSHALED_TI_SIZE_SIZE: u32 = 12u32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const CYPHER_BLOCK_LENGTH: u32 = 8u32;
pub struct CredFreeCredentialsFn(i32);
pub struct CredReadDomainCredentialsFn(i32);
pub struct CredReadFn(i32);
pub struct CredWriteFn(i32);
pub struct CrediUnmarshalandDecodeStringFn(i32);
pub struct DECRYPT_MESSAGE_FN(i32);
pub struct DELETE_SECURITY_CONTEXT_FN(i32);
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const DOMAIN_NO_LM_OWF_CHANGE: i32 = 64i32;
pub struct DOMAIN_PASSWORD_INFORMATION(i32);
pub struct DOMAIN_PASSWORD_PROPERTIES(i32);
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const DS_UNKNOWN_ADDRESS_TYPE: u32 = 0u32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const ENABLE_TLS_CLIENT_EARLY_START: u32 = 1u32;
pub struct ENCRYPTED_CREDENTIALW(i32);
pub struct ENCRYPT_MESSAGE_FN(i32);
pub struct ENUMERATE_SECURITY_PACKAGES_FN_A(i32);
pub struct ENUMERATE_SECURITY_PACKAGES_FN_W(i32);
pub struct EXPORT_SECURITY_CONTEXT_FLAGS(i32);
pub struct EXPORT_SECURITY_CONTEXT_FN(i32);
pub struct EXTENDED_NAME_FORMAT(i32);
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const E_RM_UNKNOWN_ERROR: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1073415165i32 as _);
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const FACILITY_SL_ITF: u32 = 4u32;
pub struct FREE_CONTEXT_BUFFER_FN(i32);
pub struct FREE_CREDENTIALS_HANDLE_FN(i32);
pub struct ICcgDomainAuthCredentials(i32);
pub struct IMPERSONATE_SECURITY_CONTEXT_FN(i32);
pub struct IMPORT_SECURITY_CONTEXT_FN_A(i32);
pub struct IMPORT_SECURITY_CONTEXT_FN_W(i32);
pub struct INITIALIZE_SECURITY_CONTEXT_FN_A(i32);
pub struct INITIALIZE_SECURITY_CONTEXT_FN_W(i32);
pub struct INIT_SECURITY_INTERFACE_A(i32);
pub struct INIT_SECURITY_INTERFACE_W(i32);
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const ISC_REQ_ALLOCATE_MEMORY: u32 = 256u32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const ISC_REQ_CALL_LEVEL: u32 = 4096u32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const ISC_REQ_CONFIDENTIALITY: u32 = 16u32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const ISC_REQ_CONFIDENTIALITY_ONLY: u32 = 1073741824u32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const ISC_REQ_CONNECTION: u32 = 2048u32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const ISC_REQ_DATAGRAM: u32 = 1024u32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const ISC_REQ_DEFERRED_CRED_VALIDATION: u64 = 8589934592u64;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const ISC_REQ_DELEGATE: u32 = 1u32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const ISC_REQ_EXTENDED_ERROR: u32 = 16384u32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const ISC_REQ_FORWARD_CREDENTIALS: u32 = 4194304u32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const ISC_REQ_FRAGMENT_SUPPLIED: u32 = 8192u32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const ISC_REQ_FRAGMENT_TO_FIT: u32 = 2097152u32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const ISC_REQ_IDENTIFY: u32 = 131072u32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const ISC_REQ_INTEGRITY: u32 = 65536u32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const ISC_REQ_MANUAL_CRED_VALIDATION: u32 = 524288u32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const ISC_REQ_MESSAGES: u64 = 4294967296u64;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const ISC_REQ_MUTUAL_AUTH: u32 = 2u32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const ISC_REQ_NO_INTEGRITY: u32 = 8388608u32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const ISC_REQ_NULL_SESSION: u32 = 262144u32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const ISC_REQ_PROMPT_FOR_CREDS: u32 = 64u32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const ISC_REQ_REPLAY_DETECT: u32 = 4u32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const ISC_REQ_RESERVED1: u32 = 1048576u32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const ISC_REQ_SEQUENCE_DETECT: u32 = 8u32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const ISC_REQ_STREAM: u32 = 32768u32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const ISC_REQ_UNVERIFIED_TARGET_NAME: u32 = 536870912u32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const ISC_REQ_USE_DCE_STYLE: u32 = 512u32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const ISC_REQ_USE_HTTP_STYLE: u32 = 16777216u32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const ISC_REQ_USE_SESSION_KEY: u32 = 32u32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const ISC_REQ_USE_SUPPLIED_CREDS: u32 = 128u32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const ISC_RET_ALLOCATED_MEMORY: u32 = 256u32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const ISC_RET_CALL_LEVEL: u32 = 8192u32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const ISC_RET_CONFIDENTIALITY: u32 = 16u32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const ISC_RET_CONFIDENTIALITY_ONLY: u32 = 1073741824u32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const ISC_RET_CONNECTION: u32 = 2048u32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const ISC_RET_DATAGRAM: u32 = 1024u32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const ISC_RET_DEFERRED_CRED_VALIDATION: u64 = 8589934592u64;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const ISC_RET_DELEGATE: u32 = 1u32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const ISC_RET_EXTENDED_ERROR: u32 = 16384u32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const ISC_RET_FORWARD_CREDENTIALS: u32 = 4194304u32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const ISC_RET_FRAGMENT_ONLY: u32 = 2097152u32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const ISC_RET_IDENTIFY: u32 = 131072u32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const ISC_RET_INTEGRITY: u32 = 65536u32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const ISC_RET_INTERMEDIATE_RETURN: u32 = 4096u32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const ISC_RET_MANUAL_CRED_VALIDATION: u32 = 524288u32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const ISC_RET_MESSAGES: u64 = 4294967296u64;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const ISC_RET_MUTUAL_AUTH: u32 = 2u32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const ISC_RET_NO_ADDITIONAL_TOKEN: u32 = 33554432u32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const ISC_RET_NULL_SESSION: u32 = 262144u32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const ISC_RET_REAUTHENTICATION: u32 = 134217728u32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const ISC_RET_REPLAY_DETECT: u32 = 4u32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const ISC_RET_RESERVED1: u32 = 1048576u32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const ISC_RET_SEQUENCE_DETECT: u32 = 8u32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const ISC_RET_STREAM: u32 = 32768u32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const ISC_RET_USED_COLLECTED_CREDS: u32 = 64u32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const ISC_RET_USED_DCE_STYLE: u32 = 512u32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const ISC_RET_USED_HTTP_STYLE: u32 = 16777216u32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const ISC_RET_USED_SUPPLIED_CREDS: u32 = 128u32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const ISC_RET_USE_SESSION_KEY: u32 = 32u32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const ISSP_LEVEL: u32 = 32u32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const ISSP_MODE: u32 = 1u32;
pub struct KDC_PROXY_CACHE_ENTRY_DATA(i32);
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const KDC_PROXY_SETTINGS_FLAGS_FORCEPROXY: u32 = 1u32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const KDC_PROXY_SETTINGS_V1: u32 = 1u32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const KERBEROS_REVISION: u32 = 6u32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const KERBEROS_VERSION: u32 = 5u32;
pub struct KERB_ADDRESS_TYPE(i32);
pub struct KERB_ADD_BINDING_CACHE_ENTRY_EX_REQUEST(i32);
pub struct KERB_ADD_BINDING_CACHE_ENTRY_REQUEST(i32);
pub struct KERB_ADD_CREDENTIALS_REQUEST(i32);
pub struct KERB_ADD_CREDENTIALS_REQUEST_EX(i32);
pub struct KERB_AUTH_DATA(i32);
pub struct KERB_BINDING_CACHE_ENTRY_DATA(i32);
pub struct KERB_CERTIFICATE_HASHINFO(i32);
pub struct KERB_CERTIFICATE_INFO(i32);
pub struct KERB_CERTIFICATE_INFO_TYPE(i32);
pub struct KERB_CERTIFICATE_LOGON(i32);
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const KERB_CERTIFICATE_LOGON_FLAG_CHECK_DUPLICATES: u32 = 1u32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const KERB_CERTIFICATE_LOGON_FLAG_USE_CERTIFICATE_INFO: u32 = 2u32;
pub struct KERB_CERTIFICATE_S4U_LOGON(i32);
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const KERB_CERTIFICATE_S4U_LOGON_FLAG_CHECK_DUPLICATES: u32 = 1u32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const KERB_CERTIFICATE_S4U_LOGON_FLAG_CHECK_LOGONHOURS: u32 = 2u32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const KERB_CERTIFICATE_S4U_LOGON_FLAG_FAIL_IF_NT_AUTH_POLICY_REQUIRED: u32 = 4u32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const KERB_CERTIFICATE_S4U_LOGON_FLAG_IDENTIFY: u32 = 8u32;
pub struct KERB_CERTIFICATE_UNLOCK_LOGON(i32);
pub struct KERB_CHANGEPASSWORD_REQUEST(i32);
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const KERB_CHECKSUM_CRC32: u32 = 1u32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const KERB_CHECKSUM_DES_MAC: i32 = -133i32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const KERB_CHECKSUM_DES_MAC_MD5: i32 = -134i32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const KERB_CHECKSUM_HMAC_MD5: i32 = -138i32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const KERB_CHECKSUM_HMAC_SHA1_96_AES128: u32 = 15u32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const KERB_CHECKSUM_HMAC_SHA1_96_AES128_Ki: i32 = -150i32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const KERB_CHECKSUM_HMAC_SHA1_96_AES256: u32 = 16u32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const KERB_CHECKSUM_HMAC_SHA1_96_AES256_Ki: i32 = -151i32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const KERB_CHECKSUM_KRB_DES_MAC: u32 = 4u32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const KERB_CHECKSUM_KRB_DES_MAC_K: u32 = 5u32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const KERB_CHECKSUM_LM: i32 = -130i32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const KERB_CHECKSUM_MD25: i32 = -135i32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const KERB_CHECKSUM_MD4: u32 = 2u32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const KERB_CHECKSUM_MD5: u32 = 7u32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const KERB_CHECKSUM_MD5_DES: u32 = 8u32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const KERB_CHECKSUM_MD5_HMAC: i32 = -137i32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const KERB_CHECKSUM_NONE: u32 = 0u32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const KERB_CHECKSUM_RC4_MD5: i32 = -136i32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const KERB_CHECKSUM_REAL_CRC32: i32 = -132i32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const KERB_CHECKSUM_SHA1: i32 = -131i32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const KERB_CHECKSUM_SHA1_NEW: u32 = 14u32;
pub struct KERB_CLEANUP_MACHINE_PKINIT_CREDS_REQUEST(i32);
pub struct KERB_CLOUD_KERBEROS_DEBUG_DATA_V0(i32);
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const KERB_CLOUD_KERBEROS_DEBUG_DATA_VERSION: u32 = 0u32;
pub struct KERB_CLOUD_KERBEROS_DEBUG_REQUEST(i32);
pub struct KERB_CLOUD_KERBEROS_DEBUG_RESPONSE(i32);
pub struct KERB_CRYPTO_KEY(i32);
pub struct KERB_CRYPTO_KEY32(i32);
pub struct KERB_CRYPTO_KEY_TYPE(i32);
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const KERB_DECRYPT_FLAG_DEFAULT_KEY: u32 = 1u32;
pub struct KERB_DECRYPT_REQUEST(i32);
pub struct KERB_DECRYPT_RESPONSE(i32);
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const KERB_ETYPE_AES128_CTS_HMAC_SHA1_96: u32 = 17u32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const KERB_ETYPE_AES128_CTS_HMAC_SHA1_96_PLAIN: i32 = -148i32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const KERB_ETYPE_AES256_CTS_HMAC_SHA1_96: u32 = 18u32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const KERB_ETYPE_AES256_CTS_HMAC_SHA1_96_PLAIN: i32 = -149i32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const KERB_ETYPE_DEFAULT: u32 = 0u32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const KERB_ETYPE_DES3_CBC_MD5: u32 = 5u32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const KERB_ETYPE_DES3_CBC_SHA1: u32 = 7u32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const KERB_ETYPE_DES3_CBC_SHA1_KD: u32 = 16u32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const KERB_ETYPE_DES_CBC_MD5_NT: u32 = 20u32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const KERB_ETYPE_DES_EDE3_CBC_ENV: u32 = 15u32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const KERB_ETYPE_DES_PLAIN: i32 = -132i32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const KERB_ETYPE_DSA_SHA1_CMS: u32 = 9u32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const KERB_ETYPE_DSA_SIGN: u32 = 8u32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const KERB_ETYPE_PKCS7_PUB: u32 = 13u32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const KERB_ETYPE_RC2_CBC_ENV: u32 = 12u32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const KERB_ETYPE_RC4_HMAC_NT_EXP: u32 = 24u32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const KERB_ETYPE_RC4_HMAC_OLD: i32 = -133i32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const KERB_ETYPE_RC4_HMAC_OLD_EXP: i32 = -135i32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const KERB_ETYPE_RC4_LM: i32 = -130i32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const KERB_ETYPE_RC4_PLAIN: i32 = -140i32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const KERB_ETYPE_RC4_PLAIN2: i32 = -129i32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const KERB_ETYPE_RC4_PLAIN_EXP: i32 = -141i32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const KERB_ETYPE_RC4_PLAIN_OLD: i32 = -134i32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const KERB_ETYPE_RC4_PLAIN_OLD_EXP: i32 = -136i32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const KERB_ETYPE_RC4_SHA: i32 = -131i32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const KERB_ETYPE_RSA_ENV: u32 = 13u32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const KERB_ETYPE_RSA_ES_OEAP_ENV: u32 = 14u32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const KERB_ETYPE_RSA_MD5_CMS: u32 = 10u32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const KERB_ETYPE_RSA_PRIV: u32 = 9u32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const KERB_ETYPE_RSA_PUB: u32 = 10u32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const KERB_ETYPE_RSA_PUB_MD5: u32 = 11u32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const KERB_ETYPE_RSA_PUB_SHA1: u32 = 12u32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const KERB_ETYPE_RSA_SHA1_CMS: u32 = 11u32;
pub struct KERB_EXTERNAL_NAME(i32);
pub struct KERB_EXTERNAL_TICKET(i32);
pub struct KERB_INTERACTIVE_LOGON(i32);
pub struct KERB_INTERACTIVE_PROFILE(i32);
pub struct KERB_INTERACTIVE_UNLOCK_LOGON(i32);
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const KERB_LOGON_FLAG_ALLOW_EXPIRED_TICKET: u32 = 1u32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const KERB_LOGON_FLAG_REDIRECTED: u32 = 2u32;
pub struct KERB_LOGON_SUBMIT_TYPE(i32);
pub struct KERB_NET_ADDRESS(i32);
pub struct KERB_NET_ADDRESSES(i32);
pub struct KERB_PROFILE_BUFFER_TYPE(i32);
pub struct KERB_PROTOCOL_MESSAGE_TYPE(i32);
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const KERB_PURGE_ALL_TICKETS: u32 = 1u32;
pub struct KERB_PURGE_BINDING_CACHE_REQUEST(i32);
pub struct KERB_PURGE_KDC_PROXY_CACHE_REQUEST(i32);
pub struct KERB_PURGE_KDC_PROXY_CACHE_RESPONSE(i32);
pub struct KERB_PURGE_TKT_CACHE_EX_REQUEST(i32);
pub struct KERB_PURGE_TKT_CACHE_REQUEST(i32);
pub struct KERB_QUERY_BINDING_CACHE_REQUEST(i32);
pub struct KERB_QUERY_BINDING_CACHE_RESPONSE(i32);
pub struct KERB_QUERY_DOMAIN_EXTENDED_POLICIES_REQUEST(i32);
pub struct KERB_QUERY_DOMAIN_EXTENDED_POLICIES_RESPONSE(i32);
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const KERB_QUERY_DOMAIN_EXTENDED_POLICIES_RESPONSE_FLAG_DAC_DISABLED: u32 = 1u32;
pub struct KERB_QUERY_KDC_PROXY_CACHE_REQUEST(i32);
pub struct KERB_QUERY_KDC_PROXY_CACHE_RESPONSE(i32);
pub struct KERB_QUERY_S4U2PROXY_CACHE_REQUEST(i32);
pub struct KERB_QUERY_S4U2PROXY_CACHE_RESPONSE(i32);
pub struct KERB_QUERY_TKT_CACHE_EX2_RESPONSE(i32);
pub struct KERB_QUERY_TKT_CACHE_EX3_RESPONSE(i32);
pub struct KERB_QUERY_TKT_CACHE_EX_RESPONSE(i32);
pub struct KERB_QUERY_TKT_CACHE_REQUEST(i32);
pub struct KERB_QUERY_TKT_CACHE_RESPONSE(i32);
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const KERB_REFRESH_POLICY_KDC: u32 = 2u32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const KERB_REFRESH_POLICY_KERBEROS: u32 = 1u32;
pub struct KERB_REFRESH_POLICY_REQUEST(i32);
pub struct KERB_REFRESH_POLICY_RESPONSE(i32);
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const KERB_REFRESH_SCCRED_GETTGT: u32 = 1u32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const KERB_REFRESH_SCCRED_RELEASE: u32 = 0u32;
pub struct KERB_REFRESH_SCCRED_REQUEST(i32);
pub struct KERB_REQUEST_FLAGS(i32);
pub struct KERB_RETRIEVE_KEY_TAB_REQUEST(i32);
pub struct KERB_RETRIEVE_KEY_TAB_RESPONSE(i32);
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const KERB_RETRIEVE_TICKET_AS_KERB_CRED: u32 = 8u32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const KERB_RETRIEVE_TICKET_CACHE_TICKET: u32 = 32u32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const KERB_RETRIEVE_TICKET_DEFAULT: u32 = 0u32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const KERB_RETRIEVE_TICKET_DONT_USE_CACHE: u32 = 1u32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const KERB_RETRIEVE_TICKET_MAX_LIFETIME: u32 = 64u32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const KERB_RETRIEVE_TICKET_USE_CACHE_ONLY: u32 = 2u32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const KERB_RETRIEVE_TICKET_USE_CREDHANDLE: u32 = 4u32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const KERB_RETRIEVE_TICKET_WITH_SEC_CRED: u32 = 16u32;
pub struct KERB_RETRIEVE_TKT_REQUEST(i32);
pub struct KERB_RETRIEVE_TKT_RESPONSE(i32);
pub struct KERB_S4U2PROXY_CACHE_ENTRY_INFO(i32);
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const KERB_S4U2PROXY_CACHE_ENTRY_INFO_FLAG_NEGATIVE: u32 = 1u32;
pub struct KERB_S4U2PROXY_CRED(i32);
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const KERB_S4U2PROXY_CRED_FLAG_NEGATIVE: u32 = 1u32;
pub struct KERB_S4U_LOGON(i32);
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const KERB_S4U_LOGON_FLAG_CHECK_LOGONHOURS: u32 = 2u32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const KERB_S4U_LOGON_FLAG_IDENTIFY: u32 = 8u32;
pub struct KERB_SETPASSWORD_EX_REQUEST(i32);
pub struct KERB_SETPASSWORD_REQUEST(i32);
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const KERB_SETPASS_USE_CREDHANDLE: u32 = 2u32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const KERB_SETPASS_USE_LOGONID: u32 = 1u32;
pub struct KERB_SMART_CARD_LOGON(i32);
pub struct KERB_SMART_CARD_PROFILE(i32);
pub struct KERB_SMART_CARD_UNLOCK_LOGON(i32);
pub struct KERB_SUBMIT_TKT_REQUEST(i32);
pub struct KERB_TICKET_CACHE_INFO(i32);
pub struct KERB_TICKET_CACHE_INFO_EX(i32);
pub struct KERB_TICKET_CACHE_INFO_EX2(i32);
pub struct KERB_TICKET_CACHE_INFO_EX3(i32);
pub struct KERB_TICKET_FLAGS(i32);
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const KERB_TICKET_FLAGS_cname_in_pa_data: u32 = 262144u32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const KERB_TICKET_FLAGS_enc_pa_rep: u32 = 65536u32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const KERB_TICKET_FLAGS_name_canonicalize: u32 = 65536u32;
pub struct KERB_TICKET_LOGON(i32);
pub struct KERB_TICKET_PROFILE(i32);
pub struct KERB_TICKET_UNLOCK_LOGON(i32);
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const KERB_TRANSFER_CRED_CLEANUP_CREDENTIALS: u32 = 2u32;
pub struct KERB_TRANSFER_CRED_REQUEST(i32);
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const KERB_TRANSFER_CRED_WITH_TICKETS: u32 = 1u32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const KERB_USE_DEFAULT_TICKET_FLAGS: u32 = 0u32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const KERB_WRAP_NO_ENCRYPT: u32 = 2147483649u32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const KERN_CONTEXT_CERT_INFO_V1: u32 = 0u32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const KRB_NT_ENTERPRISE_PRINCIPAL: u32 = 10u32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const KRB_NT_ENT_PRINCIPAL_AND_ID: i32 = -130i32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const KRB_NT_MS_BRANCH_ID: i32 = -133i32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const KRB_NT_MS_PRINCIPAL: i32 = -128i32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const KRB_NT_MS_PRINCIPAL_AND_ID: i32 = -129i32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const KRB_NT_PRINCIPAL: u32 = 1u32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const KRB_NT_PRINCIPAL_AND_ID: i32 = -131i32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const KRB_NT_SRV_HST: u32 = 3u32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const KRB_NT_SRV_INST: u32 = 2u32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const KRB_NT_SRV_INST_AND_ID: i32 = -132i32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const KRB_NT_SRV_XHST: u32 = 4u32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const KRB_NT_UID: u32 = 5u32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const KRB_NT_UNKNOWN: u32 = 0u32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const KRB_NT_WELLKNOWN: u32 = 11u32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const KRB_NT_X500_PRINCIPAL: u32 = 6u32;
pub struct KSEC_CONTEXT_TYPE(i32);
pub struct KSEC_LIST_ENTRY(i32);
pub struct KspCompleteTokenFn(i32);
pub struct KspDeleteContextFn(i32);
pub struct KspGetTokenFn(i32);
pub struct KspInitContextFn(i32);
pub struct KspInitPackageFn(i32);
pub struct KspMakeSignatureFn(i32);
pub struct KspMapHandleFn(i32);
pub struct KspQueryAttributesFn(i32);
pub struct KspSealMessageFn(i32);
pub struct KspSerializeAuthDataFn(i32);
pub struct KspSetPagingModeFn(i32);
pub struct KspUnsealMessageFn(i32);
pub struct KspVerifySignatureFn(i32);
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const LCRED_CRED_EXISTS: u32 = 1u32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const LCRED_STATUS_NOCRED: u32 = 0u32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const LCRED_STATUS_UNKNOWN_ISSUER: u32 = 2u32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const LOGON_GRACE_LOGON: u32 = 16777216u32;
pub struct LOGON_HOURS(i32);
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const LOGON_LM_V2: u32 = 4096u32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const LOGON_MANAGED_SERVICE: u32 = 524288u32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const LOGON_NO_ELEVATION: u32 = 262144u32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const LOGON_NO_OPTIMIZED: u32 = 131072u32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const LOGON_NTLMV2_ENABLED: u32 = 256u32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const LOGON_NTLM_V2: u32 = 8192u32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const LOGON_NT_V2: u32 = 2048u32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const LOGON_OPTIMIZED: u32 = 16384u32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const LOGON_PKINIT: u32 = 65536u32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const LOGON_WINLOGON: u32 = 32768u32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const LOOKUP_TRANSLATE_NAMES: u32 = 2048u32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const LOOKUP_VIEW_LOCAL_INFORMATION: u32 = 1u32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const LSASETCAPS_RELOAD_FLAG: u32 = 1u32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const LSASETCAPS_VALID_FLAG_MASK: u32 = 1u32;
pub struct LSA_AP_POST_LOGON_USER(i32);
pub struct LSA_AUTH_INFORMATION(i32);
pub struct LSA_AUTH_INFORMATION_AUTH_TYPE(i32);
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const LSA_CALL_LICENSE_SERVER: u32 = 2147483648u32;
pub struct LSA_DISPATCH_TABLE(i32);
pub struct LSA_ENUMERATION_INFORMATION(i32);
pub struct LSA_FOREST_TRUST_BINARY_DATA(i32);
pub struct LSA_FOREST_TRUST_COLLISION_INFORMATION(i32);
pub struct LSA_FOREST_TRUST_COLLISION_RECORD(i32);
pub struct LSA_FOREST_TRUST_COLLISION_RECORD_TYPE(i32);
pub struct LSA_FOREST_TRUST_DOMAIN_INFO(i32);
pub struct LSA_FOREST_TRUST_INFORMATION(i32);
pub struct LSA_FOREST_TRUST_RECORD(i32);
pub struct LSA_FOREST_TRUST_RECORD_TYPE(i32);
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const LSA_FOREST_TRUST_RECORD_TYPE_UNRECOGNIZED: u32 = 2147483648u32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const LSA_FTRECORD_DISABLED_REASONS: i32 = 65535i32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const LSA_GLOBAL_SECRET_PREFIX_LENGTH: u32 = 2u32;
pub struct LSA_LAST_INTER_LOGON_INFO(i32);
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const LSA_LOCAL_SECRET_PREFIX_LENGTH: u32 = 2u32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const LSA_LOOKUP_DISALLOW_CONNECTED_ACCOUNT_INTERNET_SID: u32 = 2147483648u32;
pub struct LSA_LOOKUP_DOMAIN_INFO_CLASS(i32);
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const LSA_LOOKUP_ISOLATED_AS_LOCAL: u32 = 2147483648u32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const LSA_LOOKUP_PREFER_INTERNET_NAMES: u32 = 1073741824u32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const LSA_MAXIMUM_ENUMERATION_LENGTH: u32 = 32000u32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const LSA_MAXIMUM_SID_COUNT: i32 = 256i32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const LSA_MODE_INDIVIDUAL_ACCOUNTS: i32 = 2i32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const LSA_MODE_LOG_FULL: i32 = 8i32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const LSA_MODE_MANDATORY_ACCESS: i32 = 4i32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const LSA_MODE_PASSWORD_PROTECTED: i32 = 1i32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const LSA_NB_DISABLED_ADMIN: i32 = 4i32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const LSA_NB_DISABLED_CONFLICT: i32 = 8i32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const LSA_QUERY_CLIENT_PRELOGON_SESSION_ID: u32 = 1u32;
pub struct LSA_REFERENCED_DOMAIN_LIST(i32);
pub struct LSA_SECPKG_FUNCTION_TABLE(i32);
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const LSA_SECRET_MAXIMUM_COUNT: i32 = 4096i32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const LSA_SECRET_MAXIMUM_LENGTH: i32 = 512i32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const LSA_SID_DISABLED_ADMIN: i32 = 1i32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const LSA_SID_DISABLED_CONFLICT: i32 = 2i32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const LSA_TLN_DISABLED_ADMIN: i32 = 2i32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const LSA_TLN_DISABLED_CONFLICT: i32 = 4i32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const LSA_TLN_DISABLED_NEW: i32 = 1i32;
pub struct LSA_TOKEN_INFORMATION_NULL(i32);
pub struct LSA_TOKEN_INFORMATION_TYPE(i32);
pub struct LSA_TOKEN_INFORMATION_V1(i32);
pub struct LSA_TOKEN_INFORMATION_V3(i32);
pub struct LSA_TRANSLATED_NAME(i32);
pub struct LSA_TRANSLATED_SID(i32);
pub struct LSA_TRANSLATED_SID2(i32);
pub struct LSA_TRUST_INFORMATION(i32);
pub struct LsaHandle(i32);
pub struct MAKE_SIGNATURE_FN(i32);
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const MAXIMUM_CAPES_PER_CAP: u32 = 127u32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const MAX_CRED_SIZE: u32 = 1024u32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const MAX_PROTOCOL_ID_SIZE: u32 = 255u32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const MAX_RECORDS_IN_FOREST_TRUST_INFO: u32 = 4000u32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const MAX_USER_RECORDS: u32 = 1000u32;
pub struct MSV1_0(i32);
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const MSV1_0_ALLOW_FORCE_GUEST: u32 = 8192u32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const MSV1_0_ALLOW_MSVCHAPV2: u32 = 65536u32;
pub struct MSV1_0_AVID(i32);
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const MSV1_0_AV_FLAG_FORCE_GUEST: u32 = 1u32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const MSV1_0_AV_FLAG_MIC_HANDSHAKE_MESSAGES: u32 = 2u32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const MSV1_0_AV_FLAG_UNVERIFIED_TARGET: u32 = 4u32;
pub struct MSV1_0_AV_PAIR(i32);
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const MSV1_0_CHALLENGE_LENGTH: u32 = 8u32;
pub struct MSV1_0_CHANGEPASSWORD_REQUEST(i32);
pub struct MSV1_0_CHANGEPASSWORD_RESPONSE(i32);
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const MSV1_0_CHECK_LOGONHOURS_FOR_S4U: u32 = 262144u32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const MSV1_0_CLEARTEXT_PASSWORD_SUPPLIED: u32 = 16384u32;
pub struct MSV1_0_CREDENTIAL_KEY(i32);
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const MSV1_0_CREDENTIAL_KEY_LENGTH: u32 = 20u32;
pub struct MSV1_0_CREDENTIAL_KEY_TYPE(i32);
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const MSV1_0_CRED_CREDKEY_PRESENT: u32 = 8u32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const MSV1_0_CRED_REMOVED: u32 = 4u32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const MSV1_0_CRED_SHA_PRESENT: u32 = 16u32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const MSV1_0_CRED_VERSION_ARSO: u32 = 4294901763u32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const MSV1_0_CRED_VERSION_INVALID: u32 = 4294967295u32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const MSV1_0_CRED_VERSION_IUM: u32 = 4294901761u32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const MSV1_0_CRED_VERSION_REMOTE: u32 = 4294901762u32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const MSV1_0_CRED_VERSION_RESERVED_1: u32 = 4294967294u32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const MSV1_0_CRED_VERSION_V2: u32 = 2u32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const MSV1_0_CRED_VERSION_V3: u32 = 4u32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const MSV1_0_DISABLE_PERSONAL_FALLBACK: u32 = 4096u32;
pub struct MSV1_0_INTERACTIVE_LOGON(i32);
pub struct MSV1_0_INTERACTIVE_PROFILE(i32);
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const MSV1_0_INTERNET_DOMAIN: u32 = 524288u32;
pub struct MSV1_0_IUM_SUPPLEMENTAL_CREDENTIAL(i32);
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const MSV1_0_LANMAN_SESSION_KEY_LENGTH: u32 = 8u32;
pub struct MSV1_0_LM20_LOGON(i32);
pub struct MSV1_0_LM20_LOGON_PROFILE(i32);
pub struct MSV1_0_LOGON_SUBMIT_TYPE(i32);
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const MSV1_0_MAX_AVL_SIZE: u32 = 64000u32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const MSV1_0_MAX_NTLM3_LIFE: u32 = 1800u32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const MSV1_0_MNS_LOGON: u32 = 16777216u32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const MSV1_0_NTLM3_OWF_LENGTH: u32 = 16u32;
pub struct MSV1_0_NTLM3_RESPONSE(i32);
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const MSV1_0_NTLM3_RESPONSE_LENGTH: u32 = 16u32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const MSV1_0_OWF_PASSWORD_LENGTH: u32 = 16u32;
pub struct MSV1_0_PASSTHROUGH_REQUEST(i32);
pub struct MSV1_0_PASSTHROUGH_RESPONSE(i32);
pub struct MSV1_0_PROFILE_BUFFER_TYPE(i32);
pub struct MSV1_0_PROTOCOL_MESSAGE_TYPE(i32);
pub struct MSV1_0_REMOTE_SUPPLEMENTAL_CREDENTIAL(i32);
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const MSV1_0_S4U2SELF: u32 = 131072u32;
pub struct MSV1_0_S4U_LOGON(i32);
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const MSV1_0_S4U_LOGON_FLAG_CHECK_LOGONHOURS: u32 = 2u32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const MSV1_0_SHA_PASSWORD_LENGTH: u32 = 20u32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const MSV1_0_SUBAUTHENTICATION_DLL: u32 = 4278190080u32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const MSV1_0_SUBAUTHENTICATION_DLL_EX: u32 = 1048576u32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const MSV1_0_SUBAUTHENTICATION_DLL_IIS: u32 = 132u32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const MSV1_0_SUBAUTHENTICATION_DLL_RAS: u32 = 2u32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const MSV1_0_SUBAUTHENTICATION_DLL_SHIFT: u32 = 24u32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const MSV1_0_SUBAUTHENTICATION_FLAGS: u32 = 4278190080u32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const MSV1_0_SUBAUTH_ACCOUNT_DISABLED: u32 = 1u32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const MSV1_0_SUBAUTH_ACCOUNT_EXPIRY: u32 = 16u32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const MSV1_0_SUBAUTH_ACCOUNT_TYPE: u32 = 64u32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const MSV1_0_SUBAUTH_LOCKOUT: u32 = 128u32;
pub struct MSV1_0_SUBAUTH_LOGON(i32);
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const MSV1_0_SUBAUTH_LOGON_HOURS: u32 = 8u32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const MSV1_0_SUBAUTH_PASSWORD: u32 = 2u32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const MSV1_0_SUBAUTH_PASSWORD_EXPIRY: u32 = 32u32;
pub struct MSV1_0_SUBAUTH_REQUEST(i32);
pub struct MSV1_0_SUBAUTH_RESPONSE(i32);
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const MSV1_0_SUBAUTH_WORKSTATIONS: u32 = 4u32;
pub struct MSV1_0_SUPPLEMENTAL_CREDENTIAL(i32);
pub struct MSV1_0_SUPPLEMENTAL_CREDENTIAL_V2(i32);
pub struct MSV1_0_SUPPLEMENTAL_CREDENTIAL_V3(i32);
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const MSV1_0_USER_SESSION_KEY_LENGTH: u32 = 16u32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const MSV1_0_USE_CLIENT_CHALLENGE: u32 = 128u32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const MSV1_0_USE_DOMAIN_FOR_ROUTING_ONLY: u32 = 32768u32;
pub struct MSV1_0_VALIDATION_INFO(i32);
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const MSV1_0_VALIDATION_KICKOFF_TIME: u32 = 2u32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const MSV1_0_VALIDATION_LOGOFF_TIME: u32 = 1u32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const MSV1_0_VALIDATION_LOGON_DOMAIN: u32 = 8u32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const MSV1_0_VALIDATION_LOGON_SERVER: u32 = 4u32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const MSV1_0_VALIDATION_SESSION_KEY: u32 = 16u32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const MSV1_0_VALIDATION_USER_FLAGS: u32 = 32u32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const MSV1_0_VALIDATION_USER_ID: u32 = 64u32;
pub struct MSV_SUBAUTH_LOGON_PARAMETER_CONTROL(i32);
pub struct MSV_SUB_AUTHENTICATION_FILTER(i32);
pub struct MSV_SUPPLEMENTAL_CREDENTIAL_FLAGS(i32);
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const NEGOTIATE_ALLOW_NTLM: u32 = 268435456u32;
pub struct NEGOTIATE_CALLER_NAME_REQUEST(i32);
pub struct NEGOTIATE_CALLER_NAME_RESPONSE(i32);
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const NEGOTIATE_MAX_PREFIX: u32 = 32u32;
pub struct NEGOTIATE_MESSAGES(i32);
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const NEGOTIATE_NEG_NTLM: u32 = 536870912u32;
pub struct NEGOTIATE_PACKAGE_PREFIX(i32);
pub struct NEGOTIATE_PACKAGE_PREFIXES(i32);
pub struct NETLOGON_GENERIC_INFO(i32);
pub struct NETLOGON_INTERACTIVE_INFO(i32);
pub struct NETLOGON_LOGON_IDENTITY_INFO(i32);
pub struct NETLOGON_LOGON_INFO_CLASS(i32);
pub struct NETLOGON_NETWORK_INFO(i32);
pub struct NETLOGON_SERVICE_INFO(i32);
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const NGC_DATA_FLAG_IS_CLOUD_TRUST_CRED: u32 = 8u32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const NGC_DATA_FLAG_IS_SMARTCARD_DATA: u32 = 4u32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const NGC_DATA_FLAG_KERB_CERTIFICATE_LOGON_FLAG_CHECK_DUPLICATES: u32 = 1u32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const NGC_DATA_FLAG_KERB_CERTIFICATE_LOGON_FLAG_USE_CERTIFICATE_INFO: u32 = 2u32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const NOTIFIER_FLAG_NEW_THREAD: u32 = 1u32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const NOTIFIER_FLAG_ONE_SHOT: u32 = 2u32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const NOTIFIER_FLAG_SECONDS: u32 = 2147483648u32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const NOTIFIER_TYPE_HANDLE_WAIT: u32 = 2u32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const NOTIFIER_TYPE_IMMEDIATE: u32 = 16u32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const NOTIFIER_TYPE_INTERVAL: u32 = 1u32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const NOTIFIER_TYPE_NOTIFY_EVENT: u32 = 4u32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const NOTIFIER_TYPE_STATE_CHANGE: u32 = 3u32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const NOTIFY_CLASS_DOMAIN_CHANGE: u32 = 3u32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const NOTIFY_CLASS_PACKAGE_CHANGE: u32 = 1u32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const NOTIFY_CLASS_REGISTRY_CHANGE: u32 = 4u32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const NOTIFY_CLASS_ROLE_CHANGE: u32 = 2u32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const NO_LONG_NAMES: u32 = 2u32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const PER_USER_AUDIT_FAILURE_EXCLUDE: u32 = 8u32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const PER_USER_AUDIT_FAILURE_INCLUDE: u32 = 4u32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const PER_USER_AUDIT_NONE: u32 = 16u32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const PER_USER_AUDIT_SUCCESS_EXCLUDE: u32 = 2u32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const PER_USER_AUDIT_SUCCESS_INCLUDE: u32 = 1u32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const PER_USER_POLICY_UNCHANGED: u32 = 0u32;
pub struct PKSEC_CREATE_CONTEXT_LIST(i32);
pub struct PKSEC_DEREFERENCE_LIST_ENTRY(i32);
pub struct PKSEC_INSERT_LIST_ENTRY(i32);
pub struct PKSEC_LOCATE_PKG_BY_ID(i32);
pub struct PKSEC_REFERENCE_LIST_ENTRY(i32);
pub struct PKSEC_SERIALIZE_SCHANNEL_AUTH_DATA(i32);
pub struct PKSEC_SERIALIZE_WINNT_AUTH_DATA(i32);
pub struct PKU2U_CERTIFICATE_S4U_LOGON(i32);
pub struct PKU2U_CERT_BLOB(i32);
pub struct PKU2U_CREDUI_CONTEXT(i32);
pub struct PKU2U_LOGON_SUBMIT_TYPE(i32);
pub struct PLSA_ADD_CREDENTIAL(i32);
pub struct PLSA_ALLOCATE_CLIENT_BUFFER(i32);
pub struct PLSA_ALLOCATE_LSA_HEAP(i32);
pub struct PLSA_ALLOCATE_PRIVATE_HEAP(i32);
pub struct PLSA_ALLOCATE_SHARED_MEMORY(i32);
pub struct PLSA_AP_CALL_PACKAGE(i32);
pub struct PLSA_AP_CALL_PACKAGE_PASSTHROUGH(i32);
pub struct PLSA_AP_INITIALIZE_PACKAGE(i32);
pub struct PLSA_AP_LOGON_TERMINATED(i32);
pub struct PLSA_AP_LOGON_USER(i32);
pub struct PLSA_AP_LOGON_USER_EX(i32);
pub struct PLSA_AP_LOGON_USER_EX2(i32);
pub struct PLSA_AP_LOGON_USER_EX3(i32);
pub struct PLSA_AP_POST_LOGON_USER_SURROGATE(i32);
pub struct PLSA_AP_PRE_LOGON_USER_SURROGATE(i32);
pub struct PLSA_AUDIT_ACCOUNT_LOGON(i32);
pub struct PLSA_AUDIT_LOGON(i32);
pub struct PLSA_AUDIT_LOGON_EX(i32);
pub struct PLSA_CALLBACK_FUNCTION(i32);
pub struct PLSA_CALL_PACKAGE(i32);
pub struct PLSA_CALL_PACKAGEEX(i32);
pub struct PLSA_CALL_PACKAGE_PASSTHROUGH(i32);
pub struct PLSA_CANCEL_NOTIFICATION(i32);
pub struct PLSA_CHECK_PROTECTED_USER_BY_TOKEN(i32);
pub struct PLSA_CLIENT_CALLBACK(i32);
pub struct PLSA_CLOSE_SAM_USER(i32);
pub struct PLSA_CONVERT_AUTH_DATA_TO_TOKEN(i32);
pub struct PLSA_COPY_FROM_CLIENT_BUFFER(i32);
pub struct PLSA_COPY_TO_CLIENT_BUFFER(i32);
pub struct PLSA_CRACK_SINGLE_NAME(i32);
pub struct PLSA_CREATE_LOGON_SESSION(i32);
pub struct PLSA_CREATE_SHARED_MEMORY(i32);
pub struct PLSA_CREATE_THREAD(i32);
pub struct PLSA_CREATE_TOKEN(i32);
pub struct PLSA_CREATE_TOKEN_EX(i32);
pub struct PLSA_DELETE_CREDENTIAL(i32);
pub struct PLSA_DELETE_LOGON_SESSION(i32);
pub struct PLSA_DELETE_SHARED_MEMORY(i32);
pub struct PLSA_DUPLICATE_HANDLE(i32);
pub struct PLSA_EXPAND_AUTH_DATA_FOR_DOMAIN(i32);
pub struct PLSA_FREE_CLIENT_BUFFER(i32);
pub struct PLSA_FREE_LSA_HEAP(i32);
pub struct PLSA_FREE_PRIVATE_HEAP(i32);
pub struct PLSA_FREE_SHARED_MEMORY(i32);
pub struct PLSA_GET_APP_MODE_INFO(i32);
pub struct PLSA_GET_AUTH_DATA_FOR_USER(i32);
pub struct PLSA_GET_CALL_INFO(i32);
pub struct PLSA_GET_CLIENT_INFO(i32);
pub struct PLSA_GET_CREDENTIALS(i32);
pub struct PLSA_GET_EXTENDED_CALL_FLAGS(i32);
pub struct PLSA_GET_SERVICE_ACCOUNT_PASSWORD(i32);
pub struct PLSA_GET_USER_AUTH_DATA(i32);
pub struct PLSA_GET_USER_CREDENTIALS(i32);
pub struct PLSA_IMPERSONATE_CLIENT(i32);
pub struct PLSA_LOCATE_PKG_BY_ID(i32);
pub struct PLSA_MAP_BUFFER(i32);
pub struct PLSA_OPEN_SAM_USER(i32);
pub struct PLSA_OPEN_TOKEN_BY_LOGON_ID(i32);
pub struct PLSA_PROTECT_MEMORY(i32);
pub struct PLSA_QUERY_CLIENT_REQUEST(i32);
pub struct PLSA_REDIRECTED_LOGON_CALLBACK(i32);
pub struct PLSA_REDIRECTED_LOGON_CLEANUP_CALLBACK(i32);
pub struct PLSA_REDIRECTED_LOGON_GET_LOGON_CREDS(i32);
pub struct PLSA_REDIRECTED_LOGON_GET_SUPP_CREDS(i32);
pub struct PLSA_REDIRECTED_LOGON_INIT(i32);
pub struct PLSA_REGISTER_CALLBACK(i32);
pub struct PLSA_REGISTER_NOTIFICATION(i32);
pub struct PLSA_SAVE_SUPPLEMENTAL_CREDENTIALS(i32);
pub struct PLSA_SET_APP_MODE_INFO(i32);
pub struct PLSA_UNLOAD_PACKAGE(i32);
pub struct PLSA_UPDATE_PRIMARY_CREDENTIALS(i32);
pub struct POLICY_ACCOUNT_DOMAIN_INFO(i32);
pub struct POLICY_AUDIT_CATEGORIES_INFO(i32);
pub struct POLICY_AUDIT_EVENTS_INFO(i32);
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const POLICY_AUDIT_EVENT_FAILURE: i32 = 2i32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const POLICY_AUDIT_EVENT_NONE: i32 = 4i32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const POLICY_AUDIT_EVENT_SUCCESS: i32 = 1i32;
pub struct POLICY_AUDIT_EVENT_TYPE(i32);
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const POLICY_AUDIT_EVENT_UNCHANGED: i32 = 0i32;
pub struct POLICY_AUDIT_FULL_QUERY_INFO(i32);
pub struct POLICY_AUDIT_FULL_SET_INFO(i32);
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const POLICY_AUDIT_LOG_ADMIN: i32 = 512i32;
pub struct POLICY_AUDIT_LOG_INFO(i32);
pub struct POLICY_AUDIT_SID_ARRAY(i32);
pub struct POLICY_AUDIT_SUBCATEGORIES_INFO(i32);
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const POLICY_CREATE_ACCOUNT: i32 = 16i32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const POLICY_CREATE_PRIVILEGE: i32 = 64i32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const POLICY_CREATE_SECRET: i32 = 32i32;
pub struct POLICY_DEFAULT_QUOTA_INFO(i32);
pub struct POLICY_DNS_DOMAIN_INFO(i32);
pub struct POLICY_DOMAIN_EFS_INFO(i32);
pub struct POLICY_DOMAIN_INFORMATION_CLASS(i32);
pub struct POLICY_DOMAIN_KERBEROS_TICKET_INFO(i32);
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const POLICY_GET_PRIVATE_INFORMATION: i32 = 4i32;
pub struct POLICY_INFORMATION_CLASS(i32);
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const POLICY_KERBEROS_VALIDATE_CLIENT: u32 = 128u32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const POLICY_LOOKUP_NAMES: i32 = 2048i32;
pub struct POLICY_LSA_SERVER_ROLE(i32);
pub struct POLICY_LSA_SERVER_ROLE_INFO(i32);
pub struct POLICY_MACHINE_ACCT_INFO(i32);
pub struct POLICY_MODIFICATION_INFO(i32);
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const POLICY_NOTIFICATION: i32 = 4096i32;
pub struct POLICY_NOTIFICATION_INFORMATION_CLASS(i32);
pub struct POLICY_PD_ACCOUNT_INFO(i32);
pub struct POLICY_PRIMARY_DOMAIN_INFO(i32);
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const POLICY_QOS_ALLOW_LOCAL_ROOT_CERT_STORE: u32 = 32u32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const POLICY_QOS_DHCP_SERVER_ALLOWED: u32 = 128u32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const POLICY_QOS_INBOUND_CONFIDENTIALITY: u32 = 16u32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const POLICY_QOS_INBOUND_INTEGRITY: u32 = 8u32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const POLICY_QOS_OUTBOUND_CONFIDENTIALITY: u32 = 4u32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const POLICY_QOS_OUTBOUND_INTEGRITY: u32 = 2u32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const POLICY_QOS_RAS_SERVER_ALLOWED: u32 = 64u32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const POLICY_QOS_SCHANNEL_REQUIRED: u32 = 1u32;
pub struct POLICY_REPLICA_SOURCE_INFO(i32);
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const POLICY_SERVER_ADMIN: i32 = 1024i32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const POLICY_SET_AUDIT_REQUIREMENTS: i32 = 256i32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const POLICY_SET_DEFAULT_QUOTA_LIMITS: i32 = 128i32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const POLICY_TRUST_ADMIN: i32 = 8i32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const POLICY_VIEW_AUDIT_INFORMATION: i32 = 2i32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const POLICY_VIEW_LOCAL_INFORMATION: i32 = 1i32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const PRIMARY_CRED_ARSO_LOGON: u32 = 2097152u32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const PRIMARY_CRED_AUTH_ID: u32 = 512u32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const PRIMARY_CRED_CACHED_INTERACTIVE_LOGON: u32 = 262144u32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const PRIMARY_CRED_CACHED_LOGON: u32 = 8u32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const PRIMARY_CRED_CLEAR_PASSWORD: u32 = 1u32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const PRIMARY_CRED_DO_NOT_SPLIT: u32 = 1024u32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const PRIMARY_CRED_ENCRYPTED_CREDGUARD_PASSWORD: u32 = 131072u32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const PRIMARY_CRED_ENTERPRISE_INTERNET_USER: u32 = 65536u32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const PRIMARY_CRED_EX: u32 = 4096u32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const PRIMARY_CRED_INTERACTIVE_FIDO_LOGON: u32 = 1048576u32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const PRIMARY_CRED_INTERACTIVE_NGC_LOGON: u32 = 524288u32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const PRIMARY_CRED_INTERACTIVE_SMARTCARD_LOGON: u32 = 64u32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const PRIMARY_CRED_INTERNET_USER: u32 = 256u32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const PRIMARY_CRED_LOGON_LUA: u32 = 32u32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const PRIMARY_CRED_LOGON_NO_TCB: u32 = 16u32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const PRIMARY_CRED_LOGON_PACKAGE_SHIFT: u32 = 24u32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const PRIMARY_CRED_OWF_PASSWORD: u32 = 2u32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const PRIMARY_CRED_PACKAGE_MASK: u32 = 4278190080u32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const PRIMARY_CRED_PACKED_CREDS: u32 = 32768u32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const PRIMARY_CRED_PROTECTED_USER: u32 = 2048u32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const PRIMARY_CRED_REFRESH_NEEDED: u32 = 128u32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const PRIMARY_CRED_RESTRICTED_TS: u32 = 16384u32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const PRIMARY_CRED_SUPPLEMENTAL: u32 = 4194304u32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const PRIMARY_CRED_TRANSFER: u32 = 8192u32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const PRIMARY_CRED_UPDATE: u32 = 4u32;
pub struct PSAM_CREDENTIAL_UPDATE_FREE_ROUTINE(i32);
pub struct PSAM_CREDENTIAL_UPDATE_NOTIFY_ROUTINE(i32);
pub struct PSAM_CREDENTIAL_UPDATE_REGISTER_MAPPED_ENTRYPOINTS_ROUTINE(i32);
pub struct PSAM_CREDENTIAL_UPDATE_REGISTER_ROUTINE(i32);
pub struct PSAM_INIT_NOTIFICATION_ROUTINE(i32);
pub struct PSAM_PASSWORD_FILTER_ROUTINE(i32);
pub struct PSAM_PASSWORD_NOTIFICATION_ROUTINE(i32);
pub struct PctPublicKey(i32);
pub struct QUERY_CONTEXT_ATTRIBUTES_EX_FN_A(i32);
pub struct QUERY_CONTEXT_ATTRIBUTES_EX_FN_W(i32);
pub struct QUERY_CONTEXT_ATTRIBUTES_FN_A(i32);
pub struct QUERY_CONTEXT_ATTRIBUTES_FN_W(i32);
pub struct QUERY_CREDENTIALS_ATTRIBUTES_EX_FN_A(i32);
pub struct QUERY_CREDENTIALS_ATTRIBUTES_EX_FN_W(i32);
pub struct QUERY_CREDENTIALS_ATTRIBUTES_FN_A(i32);
pub struct QUERY_CREDENTIALS_ATTRIBUTES_FN_W(i32);
pub struct QUERY_SECURITY_CONTEXT_TOKEN_FN(i32);
pub struct QUERY_SECURITY_PACKAGE_INFO_FN_A(i32);
pub struct QUERY_SECURITY_PACKAGE_INFO_FN_W(i32);
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const RCRED_CRED_EXISTS: u32 = 1u32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const RCRED_STATUS_NOCRED: u32 = 0u32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const RCRED_STATUS_UNKNOWN_ISSUER: u32 = 2u32;
pub struct REVERT_SECURITY_CONTEXT_FN(i32);
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const RTL_ENCRYPT_MEMORY_SIZE: u32 = 8u32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const RTL_ENCRYPT_OPTION_CROSS_PROCESS: u32 = 1u32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const RTL_ENCRYPT_OPTION_FOR_SYSTEM: u32 = 4u32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const RTL_ENCRYPT_OPTION_SAME_LOGON: u32 = 2u32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SAM_DAYS_PER_WEEK: u32 = 7u32;
pub struct SAM_REGISTER_MAPPING_ELEMENT(i32);
pub struct SAM_REGISTER_MAPPING_LIST(i32);
pub struct SAM_REGISTER_MAPPING_TABLE(i32);
pub struct SASL_AUTHZID_STATE(i32);
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SASL_OPTION_AUTHZ_PROCESSING: u32 = 4u32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SASL_OPTION_AUTHZ_STRING: u32 = 3u32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SASL_OPTION_RECV_SIZE: u32 = 2u32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SASL_OPTION_SEND_SIZE: u32 = 1u32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SCHANNEL_ALERT: u32 = 2u32;
pub struct SCHANNEL_ALERT_TOKEN(i32);
pub struct SCHANNEL_ALERT_TOKEN_ALERT_TYPE(i32);
pub struct SCHANNEL_CERT_HASH(i32);
pub struct SCHANNEL_CERT_HASH_STORE(i32);
pub struct SCHANNEL_CLIENT_SIGNATURE(i32);
pub struct SCHANNEL_CRED(i32);
pub struct SCHANNEL_CRED_FLAGS(i32);
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SCHANNEL_CRED_VERSION: u32 = 4u32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SCHANNEL_RENEGOTIATE: u32 = 0u32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SCHANNEL_SECRET_PRIVKEY: u32 = 2u32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SCHANNEL_SECRET_TYPE_CAPI: u32 = 1u32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SCHANNEL_SESSION: u32 = 3u32;
pub struct SCHANNEL_SESSION_TOKEN(i32);
pub struct SCHANNEL_SESSION_TOKEN_FLAGS(i32);
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SCHANNEL_SHUTDOWN: u32 = 1u32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SCH_ALLOW_NULL_ENCRYPTION: u32 = 33554432u32;
pub struct SCH_CRED(i32);
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SCH_CREDENTIALS_VERSION: u32 = 5u32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SCH_CRED_CACHE_ONLY_URL_RETRIEVAL: u32 = 32768u32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SCH_CRED_CERT_CONTEXT: u32 = 3u32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SCH_CRED_DEFERRED_CRED_VALIDATION: u32 = 67108864u32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SCH_CRED_DISABLE_RECONNECTS: u32 = 128u32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SCH_CRED_FORMAT_CERT_CONTEXT: u32 = 0u32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SCH_CRED_FORMAT_CERT_HASH: u32 = 1u32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SCH_CRED_FORMAT_CERT_HASH_STORE: u32 = 2u32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SCH_CRED_MAX_STORE_NAME_SIZE: u32 = 128u32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SCH_CRED_MAX_SUPPORTED_ALGS: u32 = 256u32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SCH_CRED_MAX_SUPPORTED_ALPN_IDS: u32 = 16u32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SCH_CRED_MAX_SUPPORTED_CERTS: u32 = 100u32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SCH_CRED_MAX_SUPPORTED_CHAINING_MODES: u32 = 16u32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SCH_CRED_MAX_SUPPORTED_CRYPTO_SETTINGS: u32 = 16u32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SCH_CRED_MAX_SUPPORTED_PARAMETERS: u32 = 16u32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SCH_CRED_MEMORY_STORE_CERT: u32 = 65536u32;
pub struct SCH_CRED_PUBLIC_CERTCHAIN(i32);
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SCH_CRED_RESTRICTED_ROOTS: u32 = 8192u32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SCH_CRED_REVOCATION_CHECK_CACHE_ONLY: u32 = 16384u32;
pub struct SCH_CRED_SECRET_CAPI(i32);
pub struct SCH_CRED_SECRET_PRIVKEY(i32);
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SCH_CRED_SNI_CREDENTIAL: u32 = 524288u32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SCH_CRED_SNI_ENABLE_OCSP: u32 = 1048576u32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SCH_CRED_V1: u32 = 1u32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SCH_CRED_V2: u32 = 2u32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SCH_CRED_V3: u32 = 3u32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SCH_CRED_VERSION: u32 = 2u32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SCH_CRED_X509_CAPI: u32 = 2u32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SCH_CRED_X509_CERTCHAIN: u32 = 1u32;
pub struct SCH_EXTENSION_DATA(i32);
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SCH_MACHINE_CERT_HASH: u32 = 1u32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SCH_MAX_EXT_SUBSCRIPTIONS: u32 = 2u32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SCH_USE_DTLS_ONLY: u32 = 16777216u32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SECBUFFER_ALERT: u32 = 17u32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SECBUFFER_APPLICATION_PROTOCOLS: u32 = 18u32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SECBUFFER_ATTRMASK: u32 = 4026531840u32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SECBUFFER_CHANGE_PASS_RESPONSE: u32 = 15u32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SECBUFFER_CHANNEL_BINDINGS: u32 = 14u32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SECBUFFER_DATA: u32 = 1u32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SECBUFFER_DTLS_MTU: u32 = 24u32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SECBUFFER_EMPTY: u32 = 0u32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SECBUFFER_EXTRA: u32 = 5u32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SECBUFFER_FLAGS: u32 = 27u32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SECBUFFER_KERNEL_MAP: u32 = 536870912u32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SECBUFFER_MECHLIST: u32 = 11u32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SECBUFFER_MECHLIST_SIGNATURE: u32 = 12u32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SECBUFFER_MISSING: u32 = 4u32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SECBUFFER_NEGOTIATION_INFO: u32 = 8u32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SECBUFFER_PADDING: u32 = 9u32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SECBUFFER_PKG_PARAMS: u32 = 3u32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SECBUFFER_PRESHARED_KEY: u32 = 22u32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SECBUFFER_PRESHARED_KEY_IDENTITY: u32 = 23u32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SECBUFFER_READONLY: u32 = 2147483648u32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SECBUFFER_READONLY_WITH_CHECKSUM: u32 = 268435456u32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SECBUFFER_RESERVED: u32 = 1610612736u32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SECBUFFER_SEND_GENERIC_TLS_EXTENSION: u32 = 25u32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SECBUFFER_SRTP_MASTER_KEY_IDENTIFIER: u32 = 20u32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SECBUFFER_SRTP_PROTECTION_PROFILES: u32 = 19u32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SECBUFFER_STREAM: u32 = 10u32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SECBUFFER_STREAM_HEADER: u32 = 7u32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SECBUFFER_STREAM_TRAILER: u32 = 6u32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SECBUFFER_SUBSCRIBE_GENERIC_TLS_EXTENSION: u32 = 26u32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SECBUFFER_TARGET: u32 = 13u32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SECBUFFER_TARGET_HOST: u32 = 16u32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SECBUFFER_TOKEN: u32 = 2u32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SECBUFFER_TOKEN_BINDING: u32 = 21u32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SECBUFFER_TRAFFIC_SECRETS: u32 = 28u32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SECBUFFER_UNMAPPED: u32 = 1073741824u32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SECBUFFER_VERSION: u32 = 0u32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SECPKGCONTEXT_CIPHERINFO_V1: u32 = 1u32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SECPKGCONTEXT_CONNECTION_INFO_EX_V1: u32 = 1u32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SECPKG_ANSI_ATTRIBUTE: u32 = 0u32;
pub struct SECPKG_APP_MODE_INFO(i32);
pub struct SECPKG_ATTR(i32);
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SECPKG_ATTR_APPLICATION_PROTOCOL: u32 = 35u32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SECPKG_ATTR_AUTHENTICATION_ID: u32 = 20u32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SECPKG_ATTR_CC_POLICY_RESULT: u32 = 97u32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SECPKG_ATTR_CERT_CHECK_RESULT: u32 = 113u32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SECPKG_ATTR_CERT_CHECK_RESULT_INPROC: u32 = 114u32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SECPKG_ATTR_CIPHER_INFO: u32 = 100u32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SECPKG_ATTR_CIPHER_STRENGTHS: u32 = 87u32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SECPKG_ATTR_CLIENT_CERT_POLICY: u32 = 96u32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SECPKG_ATTR_CONNECTION_INFO_EX: u32 = 110u32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SECPKG_ATTR_CONTEXT_DELETED: u32 = 33u32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SECPKG_ATTR_CREDENTIAL_NAME: u32 = 16u32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SECPKG_ATTR_ISSUER_LIST: u32 = 80u32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SECPKG_ATTR_IS_LOOPBACK: u32 = 37u32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SECPKG_ATTR_KEYING_MATERIAL: u32 = 107u32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SECPKG_ATTR_KEYING_MATERIAL_INPROC: u32 = 112u32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SECPKG_ATTR_KEYING_MATERIAL_TOKEN_BINDING: u32 = 111u32;
pub struct SECPKG_ATTR_LCT_STATUS(i32);
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SECPKG_ATTR_LOCAL_CERT_INFO: u32 = 99u32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SECPKG_ATTR_LOGOFF_TIME: u32 = 21u32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SECPKG_ATTR_MAPPED_CRED_ATTR: u32 = 92u32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SECPKG_ATTR_NEGOTIATED_TLS_EXTENSIONS: u32 = 36u32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SECPKG_ATTR_NEGO_INFO_FLAG_NO_KERBEROS: u32 = 1u32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SECPKG_ATTR_NEGO_INFO_FLAG_NO_NTLM: u32 = 2u32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SECPKG_ATTR_NEGO_KEYS: u32 = 22u32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SECPKG_ATTR_NEGO_PKG_INFO: u32 = 31u32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SECPKG_ATTR_NEGO_STATUS: u32 = 32u32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SECPKG_ATTR_PROMPTING_NEEDED: u32 = 24u32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SECPKG_ATTR_PROTO_INFO: u32 = 7u32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SECPKG_ATTR_REMOTE_CERTIFICATES: u32 = 95u32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SECPKG_ATTR_REMOTE_CERT_CHAIN: u32 = 103u32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SECPKG_ATTR_REMOTE_CRED: u32 = 81u32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SECPKG_ATTR_SASL_CONTEXT: u32 = 65536u32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SECPKG_ATTR_SESSION_TICKET_KEYS: u32 = 115u32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SECPKG_ATTR_SRTP_PARAMETERS: u32 = 108u32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SECPKG_ATTR_SUPPORTED_ALGS: u32 = 86u32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SECPKG_ATTR_SUPPORTED_PROTOCOLS: u32 = 88u32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SECPKG_ATTR_TARGET: u32 = 19u32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SECPKG_ATTR_THUNK_ALL: u32 = 65536u32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SECPKG_ATTR_TOKEN_BINDING: u32 = 109u32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SECPKG_ATTR_UI_INFO: u32 = 104u32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SECPKG_ATTR_USER_FLAGS: u32 = 11u32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SECPKG_ATTR_USE_NCRYPT: u32 = 98u32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SECPKG_ATTR_USE_VALIDATED: u32 = 15u32;
pub struct SECPKG_BYTE_VECTOR(i32);
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SECPKG_CALLFLAGS_APPCONTAINER: u32 = 1u32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SECPKG_CALLFLAGS_APPCONTAINER_AUTHCAPABLE: u32 = 2u32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SECPKG_CALLFLAGS_APPCONTAINER_UPNCAPABLE: u32 = 8u32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SECPKG_CALLFLAGS_FORCE_SUPPLIED: u32 = 4u32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SECPKG_CALL_ANSI: u32 = 2u32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SECPKG_CALL_ASYNC_UPDATE: u32 = 4096u32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SECPKG_CALL_BUFFER_MARSHAL: u32 = 65536u32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SECPKG_CALL_CLEANUP: u32 = 32u32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SECPKG_CALL_CLOUDAP_CONNECT: u32 = 262144u32;
pub struct SECPKG_CALL_INFO(i32);
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SECPKG_CALL_IN_PROC: u32 = 16u32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SECPKG_CALL_IS_TCB: u32 = 512u32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SECPKG_CALL_KERNEL_MODE: u32 = 1u32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SECPKG_CALL_NEGO: u32 = 16384u32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SECPKG_CALL_NEGO_EXTENDER: u32 = 32768u32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SECPKG_CALL_NETWORK_ONLY: u32 = 1024u32;
pub struct SECPKG_CALL_PACKAGE_MESSAGE_TYPE(i32);
pub struct SECPKG_CALL_PACKAGE_PIN_DC_REQUEST(i32);
pub struct SECPKG_CALL_PACKAGE_TRANSFER_CRED_REQUEST(i32);
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SECPKG_CALL_PACKAGE_TRANSFER_CRED_REQUEST_FLAG_CLEANUP_CREDENTIALS: u32 = 2u32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SECPKG_CALL_PACKAGE_TRANSFER_CRED_REQUEST_FLAG_OPTIMISTIC_LOGON: u32 = 1u32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SECPKG_CALL_PACKAGE_TRANSFER_CRED_REQUEST_FLAG_TO_SSO_SESSION: u32 = 4u32;
pub struct SECPKG_CALL_PACKAGE_UNPIN_ALL_DCS_REQUEST(i32);
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SECPKG_CALL_PROCESS_TERM: u32 = 256u32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SECPKG_CALL_RECURSIVE: u32 = 8u32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SECPKG_CALL_SYSTEM_PROC: u32 = 8192u32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SECPKG_CALL_THREAD_TERM: u32 = 128u32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SECPKG_CALL_UNLOCK: u32 = 131072u32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SECPKG_CALL_URGENT: u32 = 4u32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SECPKG_CALL_WINLOGON: u32 = 2048u32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SECPKG_CALL_WOWA32: u32 = 262144u32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SECPKG_CALL_WOWCLIENT: u32 = 64u32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SECPKG_CALL_WOWX86: u32 = 64u32;
pub struct SECPKG_CLIENT_INFO(i32);
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SECPKG_CLIENT_PROCESS_TERMINATED: u32 = 1u32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SECPKG_CLIENT_THREAD_TERMINATED: u32 = 2u32;
pub struct SECPKG_CONTEXT_THUNKS(i32);
pub struct SECPKG_CRED(i32);
pub struct SECPKG_CREDENTIAL(i32);
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SECPKG_CREDENTIAL_ATTRIBUTE: u32 = 0u32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SECPKG_CREDENTIAL_FLAGS_CALLER_HAS_TCB: u32 = 1u32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SECPKG_CREDENTIAL_FLAGS_CREDMAN_CRED: u32 = 2u32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SECPKG_CREDENTIAL_VERSION: u32 = 201u32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SECPKG_CRED_ATTR_CERT: u32 = 4u32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SECPKG_CRED_ATTR_KDC_PROXY_SETTINGS: u32 = 3u32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SECPKG_CRED_ATTR_NAMES: u32 = 1u32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SECPKG_CRED_ATTR_PAC_BYPASS: u32 = 5u32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SECPKG_CRED_ATTR_SSI_PROVIDER: u32 = 2u32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SECPKG_CRED_AUTOLOGON_RESTRICTED: u32 = 16u32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SECPKG_CRED_BOTH: u32 = 3u32;
pub struct SECPKG_CRED_CLASS(i32);
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SECPKG_CRED_DEFAULT: u32 = 4u32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SECPKG_CRED_PROCESS_POLICY_ONLY: u32 = 32u32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SECPKG_CRED_RESERVED: u32 = 4026531840u32;
pub struct SECPKG_DLL_FUNCTIONS(i32);
pub struct SECPKG_EVENT_NOTIFY(i32);
pub struct SECPKG_EVENT_PACKAGE_CHANGE(i32);
pub struct SECPKG_EVENT_ROLE_CHANGE(i32);
pub struct SECPKG_EXTENDED_INFORMATION(i32);
pub struct SECPKG_EXTENDED_INFORMATION_CLASS(i32);
pub struct SECPKG_EXTRA_OIDS(i32);
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SECPKG_FLAG_ACCEPT_WIN32_NAME: u32 = 512u32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SECPKG_FLAG_APPCONTAINER_CHECKS: u32 = 8388608u32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SECPKG_FLAG_APPCONTAINER_PASSTHROUGH: u32 = 4194304u32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SECPKG_FLAG_APPLY_LOOPBACK: u32 = 33554432u32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SECPKG_FLAG_ASCII_BUFFERS: u32 = 16384u32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SECPKG_FLAG_CLIENT_ONLY: u32 = 64u32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SECPKG_FLAG_CONNECTION: u32 = 16u32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SECPKG_FLAG_CREDENTIAL_ISOLATION_ENABLED: u32 = 16777216u32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SECPKG_FLAG_DATAGRAM: u32 = 8u32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SECPKG_FLAG_DELEGATION: u32 = 131072u32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SECPKG_FLAG_EXTENDED_ERROR: u32 = 128u32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SECPKG_FLAG_FRAGMENT: u32 = 32768u32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SECPKG_FLAG_GSS_COMPATIBLE: u32 = 4096u32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SECPKG_FLAG_IMPERSONATION: u32 = 256u32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SECPKG_FLAG_INTEGRITY: u32 = 1u32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SECPKG_FLAG_LOGON: u32 = 8192u32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SECPKG_FLAG_MULTI_REQUIRED: u32 = 32u32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SECPKG_FLAG_MUTUAL_AUTH: u32 = 65536u32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SECPKG_FLAG_NEGOTIABLE: u32 = 2048u32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SECPKG_FLAG_NEGOTIABLE2: u32 = 2097152u32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SECPKG_FLAG_NEGO_EXTENDER: u32 = 1048576u32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SECPKG_FLAG_PRIVACY: u32 = 2u32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SECPKG_FLAG_READONLY_WITH_CHECKSUM: u32 = 262144u32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SECPKG_FLAG_RESTRICTED_TOKENS: u32 = 524288u32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SECPKG_FLAG_STREAM: u32 = 1024u32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SECPKG_FLAG_TOKEN_ONLY: u32 = 4u32;
pub struct SECPKG_FUNCTION_TABLE(i32);
pub struct SECPKG_GSS_INFO(i32);
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SECPKG_ID_NONE: u32 = 65535u32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SECPKG_INTERFACE_VERSION: u32 = 65536u32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SECPKG_INTERFACE_VERSION_10: u32 = 33554432u32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SECPKG_INTERFACE_VERSION_2: u32 = 131072u32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SECPKG_INTERFACE_VERSION_3: u32 = 262144u32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SECPKG_INTERFACE_VERSION_4: u32 = 524288u32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SECPKG_INTERFACE_VERSION_5: u32 = 1048576u32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SECPKG_INTERFACE_VERSION_6: u32 = 2097152u32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SECPKG_INTERFACE_VERSION_7: u32 = 4194304u32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SECPKG_INTERFACE_VERSION_8: u32 = 8388608u32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SECPKG_INTERFACE_VERSION_9: u32 = 16777216u32;
pub struct SECPKG_KERNEL_FUNCTIONS(i32);
pub struct SECPKG_KERNEL_FUNCTION_TABLE(i32);
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SECPKG_MAX_OID_LENGTH: u32 = 32u32;
pub struct SECPKG_MUTUAL_AUTH_LEVEL(i32);
pub struct SECPKG_NAME_TYPE(i32);
pub struct SECPKG_NEGO2_INFO(i32);
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SECPKG_NEGOTIATION_COMPLETE: u32 = 0u32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SECPKG_NEGOTIATION_DIRECT: u32 = 3u32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SECPKG_NEGOTIATION_IN_PROGRESS: u32 = 2u32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SECPKG_NEGOTIATION_OPTIMISTIC: u32 = 1u32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SECPKG_NEGOTIATION_TRY_MULTICRED: u32 = 4u32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SECPKG_OPTIONS_PERMANENT: u32 = 1u32;
pub struct SECPKG_PACKAGE_CHANGE_TYPE(i32);
pub struct SECPKG_PARAMETERS(i32);
pub struct SECPKG_POST_LOGON_USER_INFO(i32);
pub struct SECPKG_PRIMARY_CRED(i32);
pub struct SECPKG_PRIMARY_CRED_EX(i32);
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SECPKG_PRIMARY_CRED_EX_FLAGS_EX_DELEGATION_TOKEN: u32 = 1u32;
pub struct SECPKG_REDIRECTED_LOGON_BUFFER(i32);
pub struct SECPKG_SERIALIZED_OID(i32);
pub struct SECPKG_SESSIONINFO_TYPE(i32);
pub struct SECPKG_SHORT_VECTOR(i32);
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SECPKG_STATE_CRED_ISOLATION_ENABLED: u32 = 32u32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SECPKG_STATE_DOMAIN_CONTROLLER: u32 = 4u32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SECPKG_STATE_ENCRYPTION_PERMITTED: u32 = 1u32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SECPKG_STATE_RESERVED_1: u32 = 2147483648u32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SECPKG_STATE_STANDALONE: u32 = 16u32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SECPKG_STATE_STRONG_ENCRYPTION_PERMITTED: u32 = 2u32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SECPKG_STATE_WORKSTATION: u32 = 8u32;
pub struct SECPKG_SUPPLEMENTAL_CRED(i32);
pub struct SECPKG_SUPPLEMENTAL_CRED_ARRAY(i32);
pub struct SECPKG_SUPPLIED_CREDENTIAL(i32);
pub struct SECPKG_SURROGATE_LOGON(i32);
pub struct SECPKG_SURROGATE_LOGON_ENTRY(i32);
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SECPKG_SURROGATE_LOGON_VERSION_1: u32 = 1u32;
pub struct SECPKG_TARGETINFO(i32);
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SECPKG_UNICODE_ATTRIBUTE: u32 = 2147483648u32;
pub struct SECPKG_USER_FUNCTION_TABLE(i32);
pub struct SECPKG_WOW_CLIENT_DLL(i32);
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SECQOP_WRAP_NO_ENCRYPT: u32 = 2147483649u32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SECQOP_WRAP_OOB_DATA: u32 = 1073741824u32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SECRET_QUERY_VALUE: i32 = 2i32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SECRET_SET_VALUE: i32 = 1i32;
pub struct SECURITY_LOGON_SESSION_DATA(i32);
pub struct SECURITY_LOGON_TYPE(i32);
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SECURITY_NATIVE_DREP: u32 = 16u32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SECURITY_NETWORK_DREP: u32 = 0u32;
pub struct SECURITY_PACKAGE_OPTIONS(i32);
pub struct SECURITY_PACKAGE_OPTIONS_TYPE(i32);
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SECURITY_SUPPORT_PROVIDER_INTERFACE_VERSION: u32 = 1u32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SECURITY_SUPPORT_PROVIDER_INTERFACE_VERSION_2: u32 = 2u32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SECURITY_SUPPORT_PROVIDER_INTERFACE_VERSION_3: u32 = 3u32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SECURITY_SUPPORT_PROVIDER_INTERFACE_VERSION_4: u32 = 4u32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SECURITY_SUPPORT_PROVIDER_INTERFACE_VERSION_5: u32 = 5u32;
pub struct SECURITY_USER_DATA(i32);
pub struct SEC_APPLICATION_PROTOCOLS(i32);
pub struct SEC_APPLICATION_PROTOCOL_LIST(i32);
pub struct SEC_APPLICATION_PROTOCOL_NEGOTIATION_EXT(i32);
pub struct SEC_APPLICATION_PROTOCOL_NEGOTIATION_STATUS(i32);
pub struct SEC_CHANNEL_BINDINGS(i32);
pub struct SEC_DTLS_MTU(i32);
pub struct SEC_FLAGS(i32);
pub struct SEC_GET_KEY_FN(i32);
pub struct SEC_NEGOTIATION_INFO(i32);
pub struct SEC_PRESHAREDKEY(i32);
pub struct SEC_PRESHAREDKEY_IDENTITY(i32);
pub struct SEC_SRTP_MASTER_KEY_IDENTIFIER(i32);
pub struct SEC_SRTP_PROTECTION_PROFILES(i32);
pub struct SEC_TOKEN_BINDING(i32);
pub struct SEC_TRAFFIC_SECRETS(i32);
pub struct SEC_TRAFFIC_SECRET_TYPE(i32);
pub struct SEC_WINNT_AUTH_IDENTITY32(i32);
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SEC_WINNT_AUTH_IDENTITY_ENCRYPT_FOR_SYSTEM: u32 = 4u32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SEC_WINNT_AUTH_IDENTITY_ENCRYPT_SAME_LOGON: u32 = 1u32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SEC_WINNT_AUTH_IDENTITY_ENCRYPT_SAME_PROCESS: u32 = 2u32;
pub struct SEC_WINNT_AUTH_IDENTITY_EX2(i32);
pub struct SEC_WINNT_AUTH_IDENTITY_EX32(i32);
pub struct SEC_WINNT_AUTH_IDENTITY_EXA(i32);
pub struct SEC_WINNT_AUTH_IDENTITY_EXW(i32);
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SEC_WINNT_AUTH_IDENTITY_FLAGS_ID_PROVIDER: u32 = 524288u32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SEC_WINNT_AUTH_IDENTITY_FLAGS_NULL_DOMAIN: u32 = 262144u32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SEC_WINNT_AUTH_IDENTITY_FLAGS_NULL_USER: u32 = 131072u32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SEC_WINNT_AUTH_IDENTITY_FLAGS_PROCESS_ENCRYPTED: u32 = 16u32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SEC_WINNT_AUTH_IDENTITY_FLAGS_RESERVED: u32 = 65536u32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SEC_WINNT_AUTH_IDENTITY_FLAGS_SSPIPFC_CREDPROV_DO_NOT_LOAD: u32 = 268435456u32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SEC_WINNT_AUTH_IDENTITY_FLAGS_SSPIPFC_CREDPROV_DO_NOT_SAVE: u32 = 2147483648u32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SEC_WINNT_AUTH_IDENTITY_FLAGS_SSPIPFC_NO_CHECKBOX: u32 = 536870912u32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SEC_WINNT_AUTH_IDENTITY_FLAGS_SSPIPFC_SAVE_CRED_BY_CALLER: u32 = 2147483648u32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SEC_WINNT_AUTH_IDENTITY_FLAGS_SSPIPFC_SAVE_CRED_CHECKED: u32 = 1073741824u32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SEC_WINNT_AUTH_IDENTITY_FLAGS_SSPIPFC_USE_MASK: u32 = 4278190080u32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SEC_WINNT_AUTH_IDENTITY_FLAGS_SYSTEM_ENCRYPTED: u32 = 128u32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SEC_WINNT_AUTH_IDENTITY_FLAGS_SYSTEM_PROTECTED: u32 = 32u32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SEC_WINNT_AUTH_IDENTITY_FLAGS_USER_PROTECTED: u32 = 64u32;
pub struct SEC_WINNT_AUTH_IDENTITY_INFO(i32);
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SEC_WINNT_AUTH_IDENTITY_MARSHALLED: u32 = 4u32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SEC_WINNT_AUTH_IDENTITY_ONLY: u32 = 8u32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SEC_WINNT_AUTH_IDENTITY_VERSION: u32 = 512u32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SEC_WINNT_AUTH_IDENTITY_VERSION_2: u32 = 513u32;
pub struct SEND_GENERIC_TLS_EXTENSION(i32);
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SESSION_TICKET_INFO_V0: u32 = 0u32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SESSION_TICKET_INFO_VERSION: u32 = 0u32;
pub struct SET_CONTEXT_ATTRIBUTES_FN_A(i32);
pub struct SET_CONTEXT_ATTRIBUTES_FN_W(i32);
pub struct SET_CREDENTIALS_ATTRIBUTES_FN_A(i32);
pub struct SET_CREDENTIALS_ATTRIBUTES_FN_W(i32);
pub struct SE_ADT_ACCESS_REASON(i32);
pub struct SE_ADT_CLAIMS(i32);
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SE_ADT_OBJECT_ONLY: u32 = 1u32;
pub struct SE_ADT_OBJECT_TYPE(i32);
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SE_ADT_PARAMETERS_SELF_RELATIVE: u32 = 1u32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SE_ADT_PARAMETERS_SEND_TO_LSA: u32 = 2u32;
pub struct SE_ADT_PARAMETER_ARRAY(i32);
pub struct SE_ADT_PARAMETER_ARRAY_ENTRY(i32);
pub struct SE_ADT_PARAMETER_ARRAY_EX(i32);
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SE_ADT_PARAMETER_EXTENSIBLE_AUDIT: u32 = 4u32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SE_ADT_PARAMETER_GENERIC_AUDIT: u32 = 8u32;
pub struct SE_ADT_PARAMETER_TYPE(i32);
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SE_ADT_PARAMETER_WRITE_SYNCHRONOUS: u32 = 16u32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SE_ADT_POLICY_AUDIT_EVENT_TYPE_EX_BEGIN: u32 = 100u32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SE_MAX_AUDIT_PARAMETERS: u32 = 32u32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SE_MAX_GENERIC_AUDIT_PARAMETERS: u32 = 28u32;
pub struct SLDATATYPE(i32);
pub struct SLIDTYPE(i32);
pub struct SLLICENSINGSTATUS(i32);
pub struct SLREFERRALTYPE(i32);
pub struct SL_ACTIVATION_INFO_HEADER(i32);
pub struct SL_ACTIVATION_TYPE(i32);
pub struct SL_AD_ACTIVATION_INFO(i32);
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SL_CLIENTAPI_ZONE: u32 = 61440u32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SL_E_ACTIVATION_IN_PROGRESS: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1073422296i32 as _);
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SL_E_APPLICATION_POLICIES_MISSING: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1073418126i32 as _);
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SL_E_APPLICATION_POLICIES_NOT_LOADED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1073418125i32 as _);
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SL_E_AUTHN_CANT_VERIFY: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1073418118i32 as _);
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SL_E_AUTHN_CHALLENGE_NOT_SET: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1073418119i32 as _);
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SL_E_AUTHN_MISMATCHED_KEY: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1073418120i32 as _);
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SL_E_AUTHN_WRONG_VERSION: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1073418121i32 as _);
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SL_E_BASE_SKU_NOT_AVAILABLE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1073418155i32 as _);
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SL_E_BIOS_KEY: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1073417707i32 as _);
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SL_E_BLOCKED_PRODUCT_KEY: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1073418159i32 as _);
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SL_E_CHPA_ACTCONFIG_ID_NOT_FOUND: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1073430519i32 as _);
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SL_E_CHPA_BINDING_MAPPING_NOT_FOUND: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1073430522i32 as _);
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SL_E_CHPA_BINDING_NOT_FOUND: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1073430523i32 as _);
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SL_E_CHPA_BUSINESS_RULE_INPUT_NOT_FOUND: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1073428736i32 as _);
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SL_E_CHPA_DATABASE_ERROR: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1073430509i32 as _);
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SL_E_CHPA_DIGITALMARKER_BINDING_NOT_CONFIGURED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1073430446i32 as _);
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SL_E_CHPA_DIGITALMARKER_INVALID_BINDING: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1073430447i32 as _);
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SL_E_CHPA_DMAK_EXTENSION_LIMIT_EXCEEDED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1073430495i32 as _);
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SL_E_CHPA_DMAK_LIMIT_EXCEEDED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1073430496i32 as _);
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SL_E_CHPA_DYNAMICALLY_BLOCKED_PRODUCT_KEY: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1073430432i32 as _);
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SL_E_CHPA_FAILED_TO_DELETE_PRODUCTKEY_BINDING: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1073428649i32 as _);
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SL_E_CHPA_FAILED_TO_DELETE_PRODUCT_KEY_PROPERTY: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1073428644i32 as _);
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SL_E_CHPA_FAILED_TO_INSERT_PRODUCTKEY_BINDING: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1073428650i32 as _);
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SL_E_CHPA_FAILED_TO_INSERT_PRODUCT_KEY_PROPERTY: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1073428646i32 as _);
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SL_E_CHPA_FAILED_TO_INSERT_PRODUCT_KEY_RECORD: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1073428608i32 as _);
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SL_E_CHPA_FAILED_TO_PROCESS_PRODUCT_KEY_BINDINGS_XML: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1073428648i32 as _);
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SL_E_CHPA_FAILED_TO_UPDATE_PRODUCTKEY_BINDING: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1073428651i32 as _);
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SL_E_CHPA_FAILED_TO_UPDATE_PRODUCT_KEY_PROPERTY: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1073428645i32 as _);
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SL_E_CHPA_FAILED_TO_UPDATE_PRODUCT_KEY_RECORD: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1073428607i32 as _);
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SL_E_CHPA_GENERAL_ERROR: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1073430448i32 as _);
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SL_E_CHPA_INVALID_ACTCONFIG_ID: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1073430515i32 as _);
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SL_E_CHPA_INVALID_ARGUMENT: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1073430508i32 as _);
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SL_E_CHPA_INVALID_BINDING: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1073430526i32 as _);
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SL_E_CHPA_INVALID_BINDING_URI: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1073430511i32 as _);
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SL_E_CHPA_INVALID_PRODUCT_DATA: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1073430517i32 as _);
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SL_E_CHPA_INVALID_PRODUCT_DATA_ID: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1073430518i32 as _);
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SL_E_CHPA_INVALID_PRODUCT_KEY: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1073430524i32 as _);
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SL_E_CHPA_INVALID_PRODUCT_KEY_CHAR: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1073430512i32 as _);
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SL_E_CHPA_INVALID_PRODUCT_KEY_FORMAT: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1073430513i32 as _);
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SL_E_CHPA_INVALID_PRODUCT_KEY_LENGTH: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1073430514i32 as _);
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SL_E_CHPA_MAXIMUM_UNLOCK_EXCEEDED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1073430520i32 as _);
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SL_E_CHPA_MSCH_RESPONSE_NOT_AVAILABLE_VGA: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1073429505i32 as _);
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SL_E_CHPA_NETWORK_ERROR: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1073430510i32 as _);
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SL_E_CHPA_NO_RULES_TO_ACTIVATE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1073430449i32 as _);
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SL_E_CHPA_NULL_VALUE_FOR_PROPERTY_NAME_OR_ID: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1073428656i32 as _);
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SL_E_CHPA_OEM_SLP_COA0: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1073430506i32 as _);
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SL_E_CHPA_OVERRIDE_REQUEST_NOT_FOUND: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1073430493i32 as _);
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SL_E_CHPA_PRODUCT_KEY_BEING_USED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1073428624i32 as _);
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SL_E_CHPA_PRODUCT_KEY_BLOCKED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1073430525i32 as _);
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SL_E_CHPA_PRODUCT_KEY_BLOCKED_IPLOCATION: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1073430505i32 as _);
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SL_E_CHPA_PRODUCT_KEY_OUT_OF_RANGE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1073430527i32 as _);
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SL_E_CHPA_REISSUANCE_LIMIT_NOT_FOUND: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1073430494i32 as _);
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SL_E_CHPA_RESPONSE_NOT_AVAILABLE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1073430507i32 as _);
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SL_E_CHPA_SYSTEM_ERROR: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1073430516i32 as _);
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SL_E_CHPA_TIMEBASED_ACTIVATION_AFTER_END_DATE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1073430479i32 as _);
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SL_E_CHPA_TIMEBASED_ACTIVATION_BEFORE_START_DATE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1073430480i32 as _);
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SL_E_CHPA_TIMEBASED_ACTIVATION_NOT_AVAILABLE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1073430478i32 as _);
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SL_E_CHPA_TIMEBASED_PRODUCT_KEY_NOT_CONFIGURED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1073430477i32 as _);
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SL_E_CHPA_UNKNOWN_PRODUCT_KEY_TYPE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1073428636i32 as _);
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SL_E_CHPA_UNKNOWN_PROPERTY_ID: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1073428654i32 as _);
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SL_E_CHPA_UNKNOWN_PROPERTY_NAME: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1073428655i32 as _);
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SL_E_CHPA_UNSUPPORTED_PRODUCT_KEY: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1073430521i32 as _);
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SL_E_CIDIID_INVALID_CHECK_DIGITS: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1073418163i32 as _);
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SL_E_CIDIID_INVALID_DATA: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1073418196i32 as _);
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SL_E_CIDIID_INVALID_DATA_LENGTH: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1073418193i32 as _);
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SL_E_CIDIID_INVALID_VERSION: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1073418195i32 as _);
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SL_E_CIDIID_MISMATCHED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1073418191i32 as _);
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SL_E_CIDIID_MISMATCHED_PKEY: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1073418114i32 as _);
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SL_E_CIDIID_NOT_BOUND: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1073418113i32 as _);
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SL_E_CIDIID_NOT_DEPOSITED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1073418192i32 as _);
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SL_E_CIDIID_VERSION_NOT_SUPPORTED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1073418194i32 as _);
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SL_E_DATATYPE_MISMATCHED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1073418210i32 as _);
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SL_E_DECRYPTION_LICENSES_NOT_AVAILABLE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1073418212i32 as _);
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SL_E_DEPENDENT_PROPERTY_NOT_SET: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1073418138i32 as _);
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SL_E_DOWNLEVEL_SETUP_KEY: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1073417708i32 as _);
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SL_E_DUPLICATE_POLICY: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1073418158i32 as _);
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SL_E_EDITION_MISMATCHED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1073417712i32 as _);
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SL_E_ENGINE_DETECTED_EXPLOIT: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1073429327i32 as _);
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SL_E_EUL_CONSUMPTION_FAILED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1073422315i32 as _);
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SL_E_EUL_NOT_AVAILABLE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1073418188i32 as _);
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SL_E_EVALUATION_FAILED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1073422333i32 as _);
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SL_E_EVENT_ALREADY_REGISTERED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1073418213i32 as _);
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SL_E_EVENT_NOT_REGISTERED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1073418214i32 as _);
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SL_E_EXTERNAL_SIGNATURE_NOT_FOUND: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1073418234i32 as _);
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SL_E_GRACE_TIME_EXPIRED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1073418231i32 as _);
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SL_E_HEALTH_CHECK_FAILED_MUI_FILES: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1073429330i32 as _);
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SL_E_HEALTH_CHECK_FAILED_NEUTRAL_FILES: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1073429331i32 as _);
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SL_E_HWID_CHANGED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1073417711i32 as _);
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SL_E_HWID_ERROR: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1073422309i32 as _);
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SL_E_IA_ID_MISMATCH: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1073414909i32 as _);
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SL_E_IA_INVALID_VIRTUALIZATION_PLATFORM: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1073414911i32 as _);
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SL_E_IA_MACHINE_NOT_BOUND: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1073414908i32 as _);
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SL_E_IA_PARENT_PARTITION_NOT_ACTIVATED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1073414910i32 as _);
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SL_E_IA_THROTTLE_LIMIT_EXCEEDED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1073414912i32 as _);
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SL_E_INTERNAL_ERROR: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1073418239i32 as _);
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SL_E_INVALID_AD_DATA: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1073429329i32 as _);
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SL_E_INVALID_BINDING_BLOB: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1073418190i32 as _);
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SL_E_INVALID_CLIENT_TOKEN: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1073429720i32 as _);
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SL_E_INVALID_CONTEXT: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1073422335i32 as _);
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SL_E_INVALID_CONTEXT_DATA: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1073422300i32 as _);
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SL_E_INVALID_EVENT_ID: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1073418215i32 as _);
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SL_E_INVALID_FILE_HASH: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1073429343i32 as _);
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SL_E_INVALID_GUID: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1073422330i32 as _);
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SL_E_INVALID_HASH: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1073422299i32 as _);
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SL_E_INVALID_LICENSE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1073418209i32 as _);
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SL_E_INVALID_LICENSE_STATE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1073429336i32 as _);
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SL_E_INVALID_LICENSE_STATE_BREACH_GRACE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1073429871i32 as _);
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SL_E_INVALID_LICENSE_STATE_BREACH_GRACE_EXPIRED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1073429870i32 as _);
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SL_E_INVALID_OEM_OR_VOLUME_BINDING_DATA: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1073429337i32 as _);
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SL_E_INVALID_OFFLINE_BLOB: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1073429719i32 as _);
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SL_E_INVALID_OSVERSION_TEMPLATEID: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1073429717i32 as _);
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SL_E_INVALID_OS_FOR_PRODUCT_KEY: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1073429503i32 as _);
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SL_E_INVALID_PACKAGE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1073418208i32 as _);
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SL_E_INVALID_PACKAGE_VERSION: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1073418144i32 as _);
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SL_E_INVALID_PKEY: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1073418224i32 as _);
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SL_E_INVALID_PRODUCT_KEY: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1073418160i32 as _);
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SL_E_INVALID_PRODUCT_KEY_TYPE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1073418115i32 as _);
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SL_E_INVALID_RSDP_COUNT: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1073429328i32 as _);
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SL_E_INVALID_RULESET_RULE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1073422301i32 as _);
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SL_E_INVALID_RUNNING_MODE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1073418199i32 as _);
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SL_E_INVALID_TEMPLATE_ID: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1073429770i32 as _);
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SL_E_INVALID_TOKEN_DATA: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1073429332i32 as _);
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SL_E_INVALID_USE_OF_ADD_ON_PKEY: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147164122i32 as _);
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SL_E_INVALID_XML_BLOB: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1073429766i32 as _);
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SL_E_IP_LOCATION_FALIED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1073429335i32 as _);
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SL_E_ISSUANCE_LICENSE_NOT_INSTALLED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1073418142i32 as _);
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SL_E_LICENSE_AUTHORIZATION_FAILED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1073418206i32 as _);
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SL_E_LICENSE_DECRYPTION_FAILED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1073418205i32 as _);
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SL_E_LICENSE_FILE_NOT_INSTALLED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1073418223i32 as _);
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SL_E_LICENSE_INVALID_ADDON_INFO: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1073422310i32 as _);
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SL_E_LICENSE_MANAGEMENT_DATA_DUPLICATED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1073418156i32 as _);
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SL_E_LICENSE_MANAGEMENT_DATA_NOT_FOUND: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1073418161i32 as _);
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SL_E_LICENSE_NOT_BOUND: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1073418112i32 as _);
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SL_E_LICENSE_SERVER_URL_NOT_FOUND: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1073418216i32 as _);
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SL_E_LICENSE_SIGNATURE_VERIFICATION_FAILED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1073418211i32 as _);
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SL_E_LUA_ACCESSDENIED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1073418203i32 as _);
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SL_E_MISMATCHED_APPID: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1073418230i32 as _);
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SL_E_MISMATCHED_KEY_TYPES: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1073429340i32 as _);
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SL_E_MISMATCHED_PID: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1073418235i32 as _);
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SL_E_MISMATCHED_PKEY_RANGE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1073418236i32 as _);
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SL_E_MISMATCHED_PRODUCT_SKU: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1073418135i32 as _);
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SL_E_MISMATCHED_SECURITY_PROCESSOR: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1073418226i32 as _);
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SL_E_MISSING_OVERRIDE_ONLY_ATTRIBUTE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1073418157i32 as _);
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SL_E_NONGENUINE_GRACE_TIME_EXPIRED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1073418140i32 as _);
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SL_E_NONGENUINE_GRACE_TIME_EXPIRED_2: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1073418137i32 as _);
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SL_E_NON_GENUINE_STATUS_LAST: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1073428992i32 as _);
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SL_E_NOTIFICATION_BREACH_DETECTED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1073429199i32 as _);
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SL_E_NOTIFICATION_GRACE_EXPIRED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1073429198i32 as _);
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SL_E_NOTIFICATION_OTHER_REASONS: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1073429197i32 as _);
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SL_E_NOT_ACTIVATED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1073422331i32 as _);
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SL_E_NOT_EVALUATED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1073422332i32 as _);
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SL_E_NOT_GENUINE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1073417728i32 as _);
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SL_E_NOT_SUPPORTED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1073418218i32 as _);
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SL_E_NO_PID_CONFIG_DATA: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1073418229i32 as _);
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SL_E_NO_PRODUCT_KEY_FOUND: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1073417709i32 as _);
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SL_E_OEM_KEY_EDITION_MISMATCH: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1073417710i32 as _);
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SL_E_OFFLINE_GENUINE_BLOB_NOT_FOUND: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1073429715i32 as _);
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SL_E_OFFLINE_GENUINE_BLOB_REVOKED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1073429716i32 as _);
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SL_E_OFFLINE_VALIDATION_BLOB_PARAM_NOT_FOUND: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1073429718i32 as _);
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SL_E_OPERATION_NOT_ALLOWED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1073418134i32 as _);
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SL_E_OUT_OF_TOLERANCE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1073418225i32 as _);
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SL_E_PKEY_INTERNAL_ERROR: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1073422311i32 as _);
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SL_E_PKEY_INVALID_ALGORITHM: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1073422312i32 as _);
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SL_E_PKEY_INVALID_CONFIG: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1073422314i32 as _);
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SL_E_PKEY_INVALID_KEYCHANGE1: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1073422308i32 as _);
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SL_E_PKEY_INVALID_KEYCHANGE2: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1073422307i32 as _);
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SL_E_PKEY_INVALID_KEYCHANGE3: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1073422306i32 as _);
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SL_E_PKEY_INVALID_UNIQUEID: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1073422313i32 as _);
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SL_E_PKEY_INVALID_UPGRADE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1073418143i32 as _);
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SL_E_PKEY_NOT_INSTALLED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1073418220i32 as _);
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SL_E_PLUGIN_INVALID_MANIFEST: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1073418127i32 as _);
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SL_E_PLUGIN_NOT_REGISTERED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1073418122i32 as _);
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SL_E_POLICY_CACHE_INVALID: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1073418200i32 as _);
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SL_E_POLICY_OTHERINFO_MISMATCH: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1073422304i32 as _);
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SL_E_PRODUCT_KEY_INSTALLATION_NOT_ALLOWED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1073418189i32 as _);
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SL_E_PRODUCT_SKU_NOT_INSTALLED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1073418219i32 as _);
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SL_E_PRODUCT_UNIQUENESS_GROUP_ID_INVALID: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1073422303i32 as _);
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SL_E_PROXY_KEY_NOT_FOUND: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1073418202i32 as _);
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SL_E_PROXY_POLICY_NOT_UPDATED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1073418169i32 as _);
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SL_E_PUBLISHING_LICENSE_NOT_INSTALLED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1073418217i32 as _);
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SL_E_RAC_NOT_AVAILABLE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1073418233i32 as _);
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SL_E_RIGHT_NOT_CONSUMED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1073418238i32 as _);
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SL_E_RIGHT_NOT_GRANTED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1073418221i32 as _);
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SL_E_SECURE_STORE_ID_MISMATCH: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1073422302i32 as _);
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SL_E_SERVICE_RUNNING: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1073418117i32 as _);
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SL_E_SERVICE_STOPPING: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1073418123i32 as _);
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SL_E_SFS_BAD_TOKEN_EXT: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147163899i32 as _);
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SL_E_SFS_BAD_TOKEN_NAME: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147163900i32 as _);
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SL_E_SFS_DUPLICATE_TOKEN_NAME: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147163898i32 as _);
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SL_E_SFS_FILE_READ_ERROR: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147163895i32 as _);
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SL_E_SFS_FILE_WRITE_ERROR: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147163894i32 as _);
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SL_E_SFS_INVALID_FD_TABLE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147163902i32 as _);
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SL_E_SFS_INVALID_FILE_POSITION: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147163893i32 as _);
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SL_E_SFS_INVALID_FS_HEADER: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147163891i32 as _);
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SL_E_SFS_INVALID_FS_VERSION: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147163903i32 as _);
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SL_E_SFS_INVALID_SYNC: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147163901i32 as _);
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SL_E_SFS_INVALID_TOKEN_DATA_HASH: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147163896i32 as _);
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SL_E_SFS_INVALID_TOKEN_DESCRIPTOR: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147163890i32 as _);
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SL_E_SFS_NO_ACTIVE_TRANSACTION: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147163892i32 as _);
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SL_E_SFS_TOKEN_SIZE_MISMATCH: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147163897i32 as _);
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SL_E_SLP_BAD_FORMAT: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1073418151i32 as _);
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SL_E_SLP_INVALID_MARKER_VERSION: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1073418116i32 as _);
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SL_E_SLP_MISSING_ACPI_SLIC: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1073418153i32 as _);
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SL_E_SLP_MISSING_SLP_MARKER: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1073418152i32 as _);
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SL_E_SLP_NOT_SIGNED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1073418198i32 as _);
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SL_E_SLP_OEM_CERT_MISSING: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1073418141i32 as _);
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SL_E_SOFTMOD_EXPLOIT_DETECTED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1073429333i32 as _);
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SL_E_SPC_NOT_AVAILABLE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1073418232i32 as _);
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SL_E_SRV_AUTHORIZATION_FAILED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1073434619i32 as _);
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SL_E_SRV_BUSINESS_TOKEN_ENTRY_NOT_FOUND: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1073434608i32 as _);
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SL_E_SRV_CLIENT_CLOCK_OUT_OF_SYNC: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1073434607i32 as _);
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SL_E_SRV_GENERAL_ERROR: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1073434368i32 as _);
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SL_E_SRV_INVALID_BINDING: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1073434618i32 as _);
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SL_E_SRV_INVALID_LICENSE_STRUCTURE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1073434620i32 as _);
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SL_E_SRV_INVALID_PAYLOAD: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1073434616i32 as _);
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SL_E_SRV_INVALID_PRODUCT_KEY_LICENSE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1073434622i32 as _);
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SL_E_SRV_INVALID_PUBLISH_LICENSE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1073434623i32 as _);
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SL_E_SRV_INVALID_RIGHTS_ACCOUNT_LICENSE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1073434621i32 as _);
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SL_E_SRV_INVALID_SECURITY_PROCESSOR_LICENSE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1073434615i32 as _);
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SL_E_SRV_SERVER_PONG: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1073434617i32 as _);
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SL_E_STORE_UPGRADE_TOKEN_NOT_AUTHORIZED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1073422290i32 as _);
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SL_E_STORE_UPGRADE_TOKEN_NOT_PRS_SIGNED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1073422292i32 as _);
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SL_E_STORE_UPGRADE_TOKEN_REQUIRED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1073422295i32 as _);
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SL_E_STORE_UPGRADE_TOKEN_WRONG_EDITION: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1073422294i32 as _);
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SL_E_STORE_UPGRADE_TOKEN_WRONG_PID: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1073422293i32 as _);
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SL_E_STORE_UPGRADE_TOKEN_WRONG_VERSION: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1073422291i32 as _);
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SL_E_TAMPER_DETECTED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1073418201i32 as _);
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SL_E_TAMPER_RECOVERY_REQUIRES_ACTIVATION: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1073414656i32 as _);
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SL_E_TKA_CERT_CNG_NOT_AVAILABLE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1073417453i32 as _);
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SL_E_TKA_CERT_NOT_FOUND: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1073417467i32 as _);
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SL_E_TKA_CHALLENGE_EXPIRED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1073417471i32 as _);
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SL_E_TKA_CHALLENGE_MISMATCH: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1073417463i32 as _);
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SL_E_TKA_CRITERIA_MISMATCH: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1073417457i32 as _);
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SL_E_TKA_FAILED_GRANT_PARSING: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1073417460i32 as _);
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SL_E_TKA_GRANT_NOT_FOUND: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1073417468i32 as _);
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SL_E_TKA_INVALID_BLOB: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1073417465i32 as _);
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SL_E_TKA_INVALID_CERTIFICATE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1073417462i32 as _);
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SL_E_TKA_INVALID_CERT_CHAIN: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1073417469i32 as _);
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SL_E_TKA_INVALID_SKU_ID: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1073417466i32 as _);
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SL_E_TKA_INVALID_SMARTCARD: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1073417461i32 as _);
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SL_E_TKA_INVALID_THUMBPRINT: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1073417459i32 as _);
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SL_E_TKA_SILENT_ACTIVATION_FAILURE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1073417470i32 as _);
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SL_E_TKA_SOFT_CERT_DISALLOWED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1073417455i32 as _);
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SL_E_TKA_SOFT_CERT_INVALID: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1073417454i32 as _);
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SL_E_TKA_TAMPERED_CERT_CHAIN: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1073417464i32 as _);
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SL_E_TKA_THUMBPRINT_CERT_NOT_FOUND: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1073417458i32 as _);
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SL_E_TKA_TPID_MISMATCH: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1073417456i32 as _);
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SL_E_TOKEN_STORE_INVALID_STATE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1073422334i32 as _);
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SL_E_TOKSTO_ALREADY_INITIALIZED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1073422326i32 as _);
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SL_E_TOKSTO_CANT_ACQUIRE_MUTEX: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1073422317i32 as _);
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SL_E_TOKSTO_CANT_CREATE_FILE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1073422324i32 as _);
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SL_E_TOKSTO_CANT_CREATE_MUTEX: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1073422318i32 as _);
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SL_E_TOKSTO_CANT_PARSE_PROPERTIES: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1073422321i32 as _);
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SL_E_TOKSTO_CANT_READ_FILE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1073422322i32 as _);
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SL_E_TOKSTO_CANT_WRITE_TO_FILE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1073422323i32 as _);
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SL_E_TOKSTO_INVALID_FILE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1073422319i32 as _);
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SL_E_TOKSTO_NOT_INITIALIZED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1073422327i32 as _);
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SL_E_TOKSTO_NO_ID_SET: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1073422325i32 as _);
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SL_E_TOKSTO_NO_PROPERTIES: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1073422328i32 as _);
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SL_E_TOKSTO_NO_TOKEN_DATA: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1073422316i32 as _);
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SL_E_TOKSTO_PROPERTY_NOT_FOUND: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1073422320i32 as _);
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SL_E_TOKSTO_TOKEN_NOT_FOUND: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1073422329i32 as _);
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SL_E_USE_LICENSE_NOT_INSTALLED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1073418237i32 as _);
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SL_E_VALIDATION_BLOB_PARAM_NOT_FOUND: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1073429721i32 as _);
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SL_E_VALIDATION_BLOCKED_PRODUCT_KEY: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1073429342i32 as _);
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SL_E_VALIDATION_INVALID_PRODUCT_KEY: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1073429339i32 as _);
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SL_E_VALIDITY_PERIOD_EXPIRED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1073415161i32 as _);
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SL_E_VALIDITY_TIME_EXPIRED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1073418207i32 as _);
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SL_E_VALUE_NOT_FOUND: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1073418222i32 as _);
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SL_E_VL_AD_AO_NAME_TOO_LONG: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1073418110i32 as _);
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SL_E_VL_AD_AO_NOT_FOUND: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1073418111i32 as _);
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SL_E_VL_AD_SCHEMA_VERSION_NOT_SUPPORTED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1073418109i32 as _);
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SL_E_VL_BINDING_SERVICE_NOT_ENABLED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1073418183i32 as _);
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SL_E_VL_BINDING_SERVICE_UNAVAILABLE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1073418124i32 as _);
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SL_E_VL_INFO_PRODUCT_USER_RIGHT: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(1074065472i32 as _);
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SL_E_VL_INVALID_TIMESTAMP: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1073418132i32 as _);
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SL_E_VL_KEY_MANAGEMENT_SERVICE_ID_MISMATCH: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1073418174i32 as _);
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SL_E_VL_KEY_MANAGEMENT_SERVICE_NOT_ACTIVATED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1073418175i32 as _);
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SL_E_VL_KEY_MANAGEMENT_SERVICE_VM_NOT_SUPPORTED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1073418133i32 as _);
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SL_E_VL_MACHINE_NOT_BOUND: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1073418154i32 as _);
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SL_E_VL_NOT_ENOUGH_COUNT: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1073418184i32 as _);
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SL_E_VL_NOT_WINDOWS_SLP: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1073418187i32 as _);
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SL_E_WINDOWS_INVALID_LICENSE_STATE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1073418204i32 as _);
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SL_E_WINDOWS_VERSION_MISMATCH: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1073422297i32 as _);
pub struct SL_GENUINE_STATE(i32);
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SL_INTERNAL_ZONE: u32 = 57344u32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SL_I_NONGENUINE_GRACE_PERIOD: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(1074065509i32 as _);
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SL_I_NONGENUINE_GRACE_PERIOD_2: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(1074065512i32 as _);
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SL_I_OOB_GRACE_PERIOD: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(1074065420i32 as _);
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SL_I_OOT_GRACE_PERIOD: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(1074065421i32 as _);
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SL_I_PERPETUAL_OOB_GRACE_PERIOD: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(1074068485i32 as _);
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SL_I_STORE_BASED_ACTIVATION: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(1074066433i32 as _);
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SL_I_TIMEBASED_EXTENDED_GRACE_PERIOD: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(1074068486i32 as _);
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SL_I_TIMEBASED_VALIDITY_PERIOD: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(1074068484i32 as _);
pub struct SL_LICENSING_STATUS(i32);
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SL_MDOLLAR_ZONE: u32 = 40960u32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SL_MSCH_ZONE: u32 = 49152u32;
pub struct SL_NONGENUINE_UI_OPTIONS(i32);
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SL_REARM_REBOOT_REQUIRED: u32 = 1u32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SL_REMAPPING_MDOLLAR_CIDIID_INVALID_CHECK_DIGITS: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2143313776i32 as _);
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SL_REMAPPING_MDOLLAR_CIDIID_INVALID_DATA: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2143313778i32 as _);
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SL_REMAPPING_MDOLLAR_CIDIID_INVALID_DATA_LENGTH: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2143313777i32 as _);
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SL_REMAPPING_MDOLLAR_CIDIID_INVALID_VERSION: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2143313779i32 as _);
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SL_REMAPPING_MDOLLAR_DIGITALMARKER_BINDING_NOT_CONFIGURED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2143313708i32 as _);
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SL_REMAPPING_MDOLLAR_DIGITALMARKER_INVALID_BINDING: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2143313709i32 as _);
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SL_REMAPPING_MDOLLAR_DMAK_EXTENSION_LIMIT_EXCEEDED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2143313792i32 as _);
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SL_REMAPPING_MDOLLAR_DMAK_LIMIT_EXCEEDED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2143313793i32 as _);
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SL_REMAPPING_MDOLLAR_DMAK_OVERRIDE_LIMIT_REACHED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2143313706i32 as _);
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SL_REMAPPING_MDOLLAR_FREE_OFFER_EXPIRED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2143312896i32 as _);
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SL_REMAPPING_MDOLLAR_INVALID_ACTCONFIG_ID: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2143313802i32 as _);
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SL_REMAPPING_MDOLLAR_INVALID_ARGUMENT: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2143313795i32 as _);
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SL_REMAPPING_MDOLLAR_INVALID_BINDING: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2143313818i32 as _);
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SL_REMAPPING_MDOLLAR_INVALID_BINDING_URI: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2143313798i32 as _);
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SL_REMAPPING_MDOLLAR_INVALID_PRODUCT_DATA: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2143313804i32 as _);
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SL_REMAPPING_MDOLLAR_INVALID_PRODUCT_DATA_ID: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2143313805i32 as _);
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SL_REMAPPING_MDOLLAR_INVALID_PRODUCT_KEY: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2143313816i32 as _);
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SL_REMAPPING_MDOLLAR_INVALID_PRODUCT_KEY_FORMAT: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2143313800i32 as _);
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SL_REMAPPING_MDOLLAR_INVALID_PRODUCT_KEY_LENGTH: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2143313801i32 as _);
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SL_REMAPPING_MDOLLAR_MAXIMUM_UNLOCK_EXCEEDED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2143313807i32 as _);
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SL_REMAPPING_MDOLLAR_NO_RULES_TO_ACTIVATE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2143313720i32 as _);
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SL_REMAPPING_MDOLLAR_OEM_SLP_COA0: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2143313789i32 as _);
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SL_REMAPPING_MDOLLAR_OSR_DEVICE_BLOCKED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2143310909i32 as _);
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SL_REMAPPING_MDOLLAR_OSR_DEVICE_THROTTLED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2143310914i32 as _);
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SL_REMAPPING_MDOLLAR_OSR_DONOR_HWID_NO_ENTITLEMENT: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2143310920i32 as _);
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SL_REMAPPING_MDOLLAR_OSR_GENERIC_ERROR: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2143310919i32 as _);
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SL_REMAPPING_MDOLLAR_OSR_GP_DISABLED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2143310913i32 as _);
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SL_REMAPPING_MDOLLAR_OSR_HARDWARE_BLOCKED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2143310912i32 as _);
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SL_REMAPPING_MDOLLAR_OSR_LICENSE_BLOCKED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2143310910i32 as _);
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SL_REMAPPING_MDOLLAR_OSR_LICENSE_THROTTLED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2143310915i32 as _);
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SL_REMAPPING_MDOLLAR_OSR_NOT_ADMIN: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2143310917i32 as _);
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SL_REMAPPING_MDOLLAR_OSR_NO_ASSOCIATION: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2143310918i32 as _);
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SL_REMAPPING_MDOLLAR_OSR_USER_BLOCKED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2143310911i32 as _);
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SL_REMAPPING_MDOLLAR_OSR_USER_THROTTLED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2143310916i32 as _);
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SL_REMAPPING_MDOLLAR_PRODUCT_KEY_BLOCKED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2143313817i32 as _);
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SL_REMAPPING_MDOLLAR_PRODUCT_KEY_BLOCKED_IPLOCATION: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2143313717i32 as _);
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SL_REMAPPING_MDOLLAR_PRODUCT_KEY_OUT_OF_RANGE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2143313819i32 as _);
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SL_REMAPPING_MDOLLAR_ROT_OVERRIDE_LIMIT_REACHED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2143313707i32 as _);
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SL_REMAPPING_MDOLLAR_TIMEBASED_ACTIVATION_AFTER_END_DATE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2143313768i32 as _);
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SL_REMAPPING_MDOLLAR_TIMEBASED_ACTIVATION_BEFORE_START_DATE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2143313769i32 as _);
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SL_REMAPPING_MDOLLAR_TIMEBASED_ACTIVATION_NOT_AVAILABLE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2143313767i32 as _);
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SL_REMAPPING_MDOLLAR_TIMEBASED_PRODUCT_KEY_NOT_CONFIGURED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2143313766i32 as _);
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SL_REMAPPING_MDOLLAR_UNSUPPORTED_PRODUCT_KEY: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2143313812i32 as _);
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SL_REMAPPING_SP_PUB_API_BAD_GET_INFO_QUERY: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1073426414i32 as _);
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SL_REMAPPING_SP_PUB_API_HANDLE_NOT_COMMITED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1073426303i32 as _);
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SL_REMAPPING_SP_PUB_API_INVALID_ALGORITHM_TYPE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1073426423i32 as _);
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SL_REMAPPING_SP_PUB_API_INVALID_HANDLE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1073426388i32 as _);
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SL_REMAPPING_SP_PUB_API_INVALID_KEY_LENGTH: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1073426347i32 as _);
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SL_REMAPPING_SP_PUB_API_INVALID_LICENSE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1073426432i32 as _);
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SL_REMAPPING_SP_PUB_API_NO_AES_PROVIDER: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1073426317i32 as _);
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SL_REMAPPING_SP_PUB_API_TOO_MANY_LOADED_ENVIRONMENTS: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1073426420i32 as _);
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SL_REMAPPING_SP_PUB_CRYPTO_HASH_FINALIZED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1073425911i32 as _);
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SL_REMAPPING_SP_PUB_CRYPTO_INVALID_BLOCK: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1073425905i32 as _);
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SL_REMAPPING_SP_PUB_CRYPTO_INVALID_BLOCKLENGTH: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1073425918i32 as _);
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SL_REMAPPING_SP_PUB_CRYPTO_INVALID_CIPHER: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1073425917i32 as _);
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SL_REMAPPING_SP_PUB_CRYPTO_INVALID_CIPHERMODE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1073425916i32 as _);
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SL_REMAPPING_SP_PUB_CRYPTO_INVALID_FORMAT: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1073425904i32 as _);
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SL_REMAPPING_SP_PUB_CRYPTO_INVALID_KEYLENGTH: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1073425919i32 as _);
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SL_REMAPPING_SP_PUB_CRYPTO_INVALID_PADDING: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1073425903i32 as _);
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SL_REMAPPING_SP_PUB_CRYPTO_INVALID_SIGNATURE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1073425906i32 as _);
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SL_REMAPPING_SP_PUB_CRYPTO_INVALID_SIGNATURELENGTH: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1073425907i32 as _);
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SL_REMAPPING_SP_PUB_CRYPTO_KEY_NOT_AVAILABLE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1073425910i32 as _);
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SL_REMAPPING_SP_PUB_CRYPTO_KEY_NOT_FOUND: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1073425909i32 as _);
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SL_REMAPPING_SP_PUB_CRYPTO_NOT_BLOCK_ALIGNED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1073425908i32 as _);
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SL_REMAPPING_SP_PUB_CRYPTO_UNKNOWN_ATTRIBUTEID: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1073425912i32 as _);
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SL_REMAPPING_SP_PUB_CRYPTO_UNKNOWN_HASHID: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1073425913i32 as _);
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SL_REMAPPING_SP_PUB_CRYPTO_UNKNOWN_KEYID: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1073425914i32 as _);
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SL_REMAPPING_SP_PUB_CRYPTO_UNKNOWN_PROVIDERID: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1073425915i32 as _);
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SL_REMAPPING_SP_PUB_GENERAL_NOT_INITIALIZED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1073426175i32 as _);
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SL_REMAPPING_SP_PUB_KM_CACHE_IDENTICAL: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(1074058753i32 as _);
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SL_REMAPPING_SP_PUB_KM_CACHE_POLICY_CHANGED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(1074058754i32 as _);
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SL_REMAPPING_SP_PUB_KM_CACHE_TAMPER: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1073425151i32 as _);
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SL_REMAPPING_SP_PUB_KM_CACHE_TAMPER_RESTORE_FAILED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1073425150i32 as _);
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SL_REMAPPING_SP_PUB_PROXY_SOFT_TAMPER: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1073424638i32 as _);
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SL_REMAPPING_SP_PUB_TAMPER_MODULE_AUTHENTICATION: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1073425407i32 as _);
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SL_REMAPPING_SP_PUB_TAMPER_SECURITY_PROCESSOR_PATCHED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1073425406i32 as _);
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SL_REMAPPING_SP_PUB_TIMER_ALREADY_EXISTS: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1073425654i32 as _);
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SL_REMAPPING_SP_PUB_TIMER_EXPIRED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1073425652i32 as _);
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SL_REMAPPING_SP_PUB_TIMER_NAME_SIZE_TOO_BIG: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1073425651i32 as _);
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SL_REMAPPING_SP_PUB_TIMER_NOT_FOUND: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1073425653i32 as _);
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SL_REMAPPING_SP_PUB_TIMER_READ_ONLY: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1073425647i32 as _);
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SL_REMAPPING_SP_PUB_TRUSTED_TIME_OK: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(1074057999i32 as _);
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SL_REMAPPING_SP_PUB_TS_ACCESS_DENIED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1073425644i32 as _);
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SL_REMAPPING_SP_PUB_TS_ATTRIBUTE_NOT_FOUND: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1073425645i32 as _);
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SL_REMAPPING_SP_PUB_TS_ATTRIBUTE_READ_ONLY: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1073425646i32 as _);
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SL_REMAPPING_SP_PUB_TS_DATA_SIZE_TOO_BIG: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1073425656i32 as _);
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SL_REMAPPING_SP_PUB_TS_ENTRY_KEY_ALREADY_EXISTS: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1073425659i32 as _);
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SL_REMAPPING_SP_PUB_TS_ENTRY_KEY_NOT_FOUND: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1073425660i32 as _);
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SL_REMAPPING_SP_PUB_TS_ENTRY_KEY_SIZE_TOO_BIG: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1073425658i32 as _);
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SL_REMAPPING_SP_PUB_TS_ENTRY_READ_ONLY: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1073425648i32 as _);
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SL_REMAPPING_SP_PUB_TS_FULL: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1073425650i32 as _);
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SL_REMAPPING_SP_PUB_TS_INVALID_HW_BINDING: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1073425655i32 as _);
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SL_REMAPPING_SP_PUB_TS_MAX_REARM_REACHED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1073425657i32 as _);
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SL_REMAPPING_SP_PUB_TS_NAMESPACE_IN_USE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1073425642i32 as _);
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SL_REMAPPING_SP_PUB_TS_NAMESPACE_NOT_FOUND: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1073425643i32 as _);
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SL_REMAPPING_SP_PUB_TS_REARMED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1073425662i32 as _);
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SL_REMAPPING_SP_PUB_TS_RECREATED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1073425661i32 as _);
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SL_REMAPPING_SP_PUB_TS_TAMPERED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1073425663i32 as _);
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SL_REMAPPING_SP_PUB_TS_TAMPERED_BREADCRUMB_GENERATION: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1073425640i32 as _);
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SL_REMAPPING_SP_PUB_TS_TAMPERED_BREADCRUMB_LOAD_INVALID: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1073425641i32 as _);
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SL_REMAPPING_SP_PUB_TS_TAMPERED_DATA_BREADCRUMB_MISMATCH: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1073425637i32 as _);
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SL_REMAPPING_SP_PUB_TS_TAMPERED_DATA_VERSION_MISMATCH: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1073425636i32 as _);
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SL_REMAPPING_SP_PUB_TS_TAMPERED_INVALID_DATA: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1073425639i32 as _);
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SL_REMAPPING_SP_PUB_TS_TAMPERED_NO_DATA: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1073425638i32 as _);
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SL_REMAPPING_SP_STATUS_ALREADY_EXISTS: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1073426171i32 as _);
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SL_REMAPPING_SP_STATUS_DEBUGGER_DETECTED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147167989i32 as _);
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SL_REMAPPING_SP_STATUS_GENERIC_FAILURE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1073426173i32 as _);
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SL_REMAPPING_SP_STATUS_INSUFFICIENT_BUFFER: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1073426169i32 as _);
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SL_REMAPPING_SP_STATUS_INVALIDARG: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1073426172i32 as _);
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SL_REMAPPING_SP_STATUS_INVALIDDATA: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1073426168i32 as _);
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SL_REMAPPING_SP_STATUS_INVALID_SPAPI_CALL: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1073426167i32 as _);
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SL_REMAPPING_SP_STATUS_INVALID_SPAPI_VERSION: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1073426166i32 as _);
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SL_REMAPPING_SP_STATUS_NO_MORE_DATA: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1073426164i32 as _);
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SL_REMAPPING_SP_STATUS_PUSHKEY_CONFLICT: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1073424639i32 as _);
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SL_REMAPPING_SP_STATUS_SYSTEM_TIME_SKEWED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147167998i32 as _);
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SL_SERVER_ZONE: u32 = 45056u32;
pub struct SL_SYSTEM_POLICY_INFORMATION(i32);
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SL_SYSTEM_STATE_REBOOT_POLICY_FOUND: u32 = 1u32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SL_SYSTEM_STATE_TAMPERED: u32 = 2u32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SPP_MIGRATION_GATHER_ACTIVATED_WINDOWS_STATE: u32 = 2u32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SPP_MIGRATION_GATHER_ALL: u32 = 4294967295u32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SPP_MIGRATION_GATHER_MIGRATABLE_APPS: u32 = 1u32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SP_PROT_ALL: u32 = 4294967295u32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SP_PROT_DTLS1_0_CLIENT: u32 = 131072u32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SP_PROT_DTLS1_0_SERVER: u32 = 65536u32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SP_PROT_DTLS1_2_CLIENT: u32 = 524288u32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SP_PROT_DTLS1_2_SERVER: u32 = 262144u32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SP_PROT_DTLS_CLIENT: u32 = 131072u32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SP_PROT_DTLS_SERVER: u32 = 65536u32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SP_PROT_NONE: u32 = 0u32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SP_PROT_PCT1_CLIENT: u32 = 2u32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SP_PROT_PCT1_SERVER: u32 = 1u32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SP_PROT_SSL2_CLIENT: u32 = 8u32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SP_PROT_SSL2_SERVER: u32 = 4u32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SP_PROT_SSL3_CLIENT: u32 = 32u32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SP_PROT_SSL3_SERVER: u32 = 16u32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SP_PROT_TLS1_0_CLIENT: u32 = 128u32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SP_PROT_TLS1_0_SERVER: u32 = 64u32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SP_PROT_TLS1_1_CLIENT: u32 = 512u32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SP_PROT_TLS1_1_SERVER: u32 = 256u32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SP_PROT_TLS1_2_CLIENT: u32 = 2048u32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SP_PROT_TLS1_2_SERVER: u32 = 1024u32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SP_PROT_TLS1_3PLUS_CLIENT: u32 = 8192u32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SP_PROT_TLS1_3PLUS_SERVER: u32 = 4096u32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SP_PROT_TLS1_3_CLIENT: u32 = 8192u32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SP_PROT_TLS1_3_SERVER: u32 = 4096u32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SP_PROT_TLS1_CLIENT: u32 = 128u32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SP_PROT_TLS1_SERVER: u32 = 64u32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SP_PROT_UNI_CLIENT: u32 = 2147483648u32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SP_PROT_UNI_SERVER: u32 = 1073741824u32;
pub struct SR_SECURITY_DESCRIPTOR(i32);
pub struct SSL_CRACK_CERTIFICATE_FN(i32);
pub struct SSL_CREDENTIAL_CERTIFICATE(i32);
pub struct SSL_EMPTY_CACHE_FN_A(i32);
pub struct SSL_EMPTY_CACHE_FN_W(i32);
pub struct SSL_FREE_CERTIFICATE_FN(i32);
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SSL_SESSION_RECONNECT: u32 = 1u32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SSPIPFC_CREDPROV_DO_NOT_LOAD: u32 = 4u32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SSPIPFC_CREDPROV_DO_NOT_SAVE: u32 = 1u32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SSPIPFC_NO_CHECKBOX: u32 = 2u32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SSPIPFC_SAVE_CRED_BY_CALLER: u32 = 1u32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SSPIPFC_USE_CREDUIBROKER: u32 = 8u32;
pub struct SUBSCRIBE_GENERIC_TLS_EXTENSION(i32);
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const SZ_ALG_MAX_SIZE: u32 = 64u32;
pub struct SchGetExtensionsOptions(i32);
pub struct SecBuffer(i32);
pub struct SecBufferDesc(i32);
pub struct SecDelegationType(i32);
pub struct SecPkgContext_AccessToken(i32);
pub struct SecPkgContext_ApplicationProtocol(i32);
pub struct SecPkgContext_AuthorityA(i32);
pub struct SecPkgContext_AuthorityW(i32);
pub struct SecPkgContext_AuthzID(i32);
pub struct SecPkgContext_Bindings(i32);
pub struct SecPkgContext_CertInfo(i32);
pub struct SecPkgContext_CertificateValidationResult(i32);
pub struct SecPkgContext_Certificates(i32);
pub struct SecPkgContext_CipherInfo(i32);
pub struct SecPkgContext_ClientCertPolicyResult(i32);
pub struct SecPkgContext_ClientSpecifiedTarget(i32);
pub struct SecPkgContext_ConnectionInfo(i32);
pub struct SecPkgContext_ConnectionInfoEx(i32);
pub struct SecPkgContext_CredInfo(i32);
pub struct SecPkgContext_CredentialNameA(i32);
pub struct SecPkgContext_CredentialNameW(i32);
pub struct SecPkgContext_DceInfo(i32);
pub struct SecPkgContext_EapKeyBlock(i32);
pub struct SecPkgContext_EapPrfInfo(i32);
pub struct SecPkgContext_EarlyStart(i32);
pub struct SecPkgContext_Flags(i32);
pub struct SecPkgContext_IssuerListInfoEx(i32);
pub struct SecPkgContext_KeyInfoA(i32);
pub struct SecPkgContext_KeyInfoW(i32);
pub struct SecPkgContext_KeyingMaterial(i32);
pub struct SecPkgContext_KeyingMaterialInfo(i32);
pub struct SecPkgContext_KeyingMaterial_Inproc(i32);
pub struct SecPkgContext_LastClientTokenStatus(i32);
pub struct SecPkgContext_Lifespan(i32);
pub struct SecPkgContext_LocalCredentialInfo(i32);
pub struct SecPkgContext_LogoffTime(i32);
pub struct SecPkgContext_MappedCredAttr(i32);
pub struct SecPkgContext_NamesA(i32);
pub struct SecPkgContext_NamesW(i32);
pub struct SecPkgContext_NativeNamesA(i32);
pub struct SecPkgContext_NativeNamesW(i32);
pub struct SecPkgContext_NegoKeys(i32);
pub struct SecPkgContext_NegoPackageInfo(i32);
pub struct SecPkgContext_NegoStatus(i32);
pub struct SecPkgContext_NegotiatedTlsExtensions(i32);
pub struct SecPkgContext_NegotiationInfoA(i32);
pub struct SecPkgContext_NegotiationInfoW(i32);
pub struct SecPkgContext_PackageInfoA(i32);
pub struct SecPkgContext_PackageInfoW(i32);
pub struct SecPkgContext_PasswordExpiry(i32);
pub struct SecPkgContext_ProtoInfoA(i32);
pub struct SecPkgContext_ProtoInfoW(i32);
pub struct SecPkgContext_RemoteCredentialInfo(i32);
pub struct SecPkgContext_SaslContext(i32);
pub struct SecPkgContext_SessionAppData(i32);
pub struct SecPkgContext_SessionInfo(i32);
pub struct SecPkgContext_SessionKey(i32);
pub struct SecPkgContext_Sizes(i32);
pub struct SecPkgContext_SrtpParameters(i32);
pub struct SecPkgContext_StreamSizes(i32);
pub struct SecPkgContext_SubjectAttributes(i32);
pub struct SecPkgContext_SupportedSignatures(i32);
pub struct SecPkgContext_Target(i32);
pub struct SecPkgContext_TargetInformation(i32);
pub struct SecPkgContext_TokenBinding(i32);
pub struct SecPkgContext_UiInfo(i32);
pub struct SecPkgContext_UserFlags(i32);
pub struct SecPkgCred_CipherStrengths(i32);
pub struct SecPkgCred_ClientCertPolicy(i32);
pub struct SecPkgCred_SessionTicketKey(i32);
pub struct SecPkgCred_SessionTicketKeys(i32);
pub struct SecPkgCred_SupportedAlgs(i32);
pub struct SecPkgCred_SupportedProtocols(i32);
pub struct SecPkgCredentials_Cert(i32);
pub struct SecPkgCredentials_KdcProxySettingsW(i32);
pub struct SecPkgCredentials_NamesA(i32);
pub struct SecPkgCredentials_NamesW(i32);
pub struct SecPkgCredentials_SSIProviderA(i32);
pub struct SecPkgCredentials_SSIProviderW(i32);
pub struct SecPkgInfoA(i32);
pub struct SecPkgInfoW(i32);
pub struct SecurityFunctionTableA(i32);
pub struct SecurityFunctionTableW(i32);
pub struct SpAcceptCredentialsFn(i32);
pub struct SpAcceptLsaModeContextFn(i32);
pub struct SpAcquireCredentialsHandleFn(i32);
pub struct SpAddCredentialsFn(i32);
pub struct SpApplyControlTokenFn(i32);
pub struct SpChangeAccountPasswordFn(i32);
pub struct SpCompleteAuthTokenFn(i32);
pub struct SpDeleteContextFn(i32);
pub struct SpDeleteCredentialsFn(i32);
pub struct SpExchangeMetaDataFn(i32);
pub struct SpExportSecurityContextFn(i32);
pub struct SpFormatCredentialsFn(i32);
pub struct SpFreeCredentialsHandleFn(i32);
pub struct SpGetContextTokenFn(i32);
pub struct SpGetCredUIContextFn(i32);
pub struct SpGetCredentialsFn(i32);
pub struct SpGetExtendedInformationFn(i32);
pub struct SpGetInfoFn(i32);
pub struct SpGetRemoteCredGuardLogonBufferFn(i32);
pub struct SpGetRemoteCredGuardSupplementalCredsFn(i32);
pub struct SpGetTbalSupplementalCredsFn(i32);
pub struct SpGetUserInfoFn(i32);
pub struct SpImportSecurityContextFn(i32);
pub struct SpInitLsaModeContextFn(i32);
pub struct SpInitUserModeContextFn(i32);
pub struct SpInitializeFn(i32);
pub struct SpInstanceInitFn(i32);
pub struct SpLsaModeInitializeFn(i32);
pub struct SpMakeSignatureFn(i32);
pub struct SpMarshalAttributeDataFn(i32);
pub struct SpMarshallSupplementalCredsFn(i32);
pub struct SpQueryContextAttributesFn(i32);
pub struct SpQueryCredentialsAttributesFn(i32);
pub struct SpQueryMetaDataFn(i32);
pub struct SpSaveCredentialsFn(i32);
pub struct SpSealMessageFn(i32);
pub struct SpSetContextAttributesFn(i32);
pub struct SpSetCredentialsAttributesFn(i32);
pub struct SpSetExtendedInformationFn(i32);
pub struct SpShutdownFn(i32);
pub struct SpUnsealMessageFn(i32);
pub struct SpUpdateCredentialsFn(i32);
pub struct SpUserModeInitializeFn(i32);
pub struct SpValidateTargetInfoFn(i32);
pub struct SpVerifySignatureFn(i32);
pub struct SslGetExtensionsFn(i32);
pub struct SslGetServerIdentityFn(i32);
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const TLS1_ALERT_ACCESS_DENIED: u32 = 49u32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const TLS1_ALERT_BAD_CERTIFICATE: u32 = 42u32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const TLS1_ALERT_BAD_RECORD_MAC: u32 = 20u32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const TLS1_ALERT_CERTIFICATE_EXPIRED: u32 = 45u32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const TLS1_ALERT_CERTIFICATE_REVOKED: u32 = 44u32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const TLS1_ALERT_CERTIFICATE_UNKNOWN: u32 = 46u32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const TLS1_ALERT_CLOSE_NOTIFY: u32 = 0u32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const TLS1_ALERT_DECODE_ERROR: u32 = 50u32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const TLS1_ALERT_DECOMPRESSION_FAIL: u32 = 30u32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const TLS1_ALERT_DECRYPTION_FAILED: u32 = 21u32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const TLS1_ALERT_DECRYPT_ERROR: u32 = 51u32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const TLS1_ALERT_EXPORT_RESTRICTION: u32 = 60u32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const TLS1_ALERT_HANDSHAKE_FAILURE: u32 = 40u32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const TLS1_ALERT_ILLEGAL_PARAMETER: u32 = 47u32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const TLS1_ALERT_INSUFFIENT_SECURITY: u32 = 71u32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const TLS1_ALERT_INTERNAL_ERROR: u32 = 80u32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const TLS1_ALERT_NO_APP_PROTOCOL: u32 = 120u32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const TLS1_ALERT_NO_RENEGOTIATION: u32 = 100u32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const TLS1_ALERT_PROTOCOL_VERSION: u32 = 70u32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const TLS1_ALERT_RECORD_OVERFLOW: u32 = 22u32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const TLS1_ALERT_UNEXPECTED_MESSAGE: u32 = 10u32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const TLS1_ALERT_UNKNOWN_CA: u32 = 48u32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const TLS1_ALERT_UNKNOWN_PSK_IDENTITY: u32 = 115u32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const TLS1_ALERT_UNSUPPORTED_CERT: u32 = 43u32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const TLS1_ALERT_UNSUPPORTED_EXT: u32 = 110u32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const TLS1_ALERT_USER_CANCELED: u32 = 90u32;
pub struct TLS_EXTENSION_SUBSCRIPTION(i32);
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const TLS_PARAMS_OPTIONAL: u32 = 1u32;
pub struct TOKENBINDING_EXTENSION_FORMAT(i32);
pub struct TOKENBINDING_IDENTIFIER(i32);
pub struct TOKENBINDING_KEY_PARAMETERS_TYPE(i32);
pub struct TOKENBINDING_KEY_TYPES(i32);
pub struct TOKENBINDING_RESULT_DATA(i32);
pub struct TOKENBINDING_RESULT_LIST(i32);
pub struct TOKENBINDING_TYPE(i32);
pub struct TRUSTED_CONTROLLERS_INFO(i32);
pub struct TRUSTED_DOMAIN_AUTH_INFORMATION(i32);
pub struct TRUSTED_DOMAIN_FULL_INFORMATION(i32);
pub struct TRUSTED_DOMAIN_FULL_INFORMATION2(i32);
pub struct TRUSTED_DOMAIN_INFORMATION_EX(i32);
pub struct TRUSTED_DOMAIN_INFORMATION_EX2(i32);
pub struct TRUSTED_DOMAIN_NAME_INFO(i32);
pub struct TRUSTED_DOMAIN_SUPPORTED_ENCRYPTION_TYPES(i32);
pub struct TRUSTED_DOMAIN_TRUST_ATTRIBUTES(i32);
pub struct TRUSTED_DOMAIN_TRUST_DIRECTION(i32);
pub struct TRUSTED_DOMAIN_TRUST_TYPE(i32);
pub struct TRUSTED_INFORMATION_CLASS(i32);
pub struct TRUSTED_PASSWORD_INFO(i32);
pub struct TRUSTED_POSIX_OFFSET_INFO(i32);
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const TRUSTED_QUERY_AUTH: i32 = 64i32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const TRUSTED_QUERY_CONTROLLERS: i32 = 2i32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const TRUSTED_QUERY_DOMAIN_NAME: i32 = 1i32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const TRUSTED_QUERY_POSIX: i32 = 8i32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const TRUSTED_SET_AUTH: i32 = 32i32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const TRUSTED_SET_CONTROLLERS: i32 = 4i32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const TRUSTED_SET_POSIX: i32 = 16i32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const TRUST_ATTRIBUTES_USER: u32 = 4278190080u32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const TRUST_ATTRIBUTES_VALID: u32 = 4278386687u32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const TRUST_ATTRIBUTE_CROSS_ORGANIZATION_ENABLE_TGT_DELEGATION: u32 = 2048u32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const TRUST_ATTRIBUTE_CROSS_ORGANIZATION_NO_TGT_DELEGATION: u32 = 512u32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const TRUST_ATTRIBUTE_PIM_TRUST: u32 = 1024u32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const TRUST_ATTRIBUTE_QUARANTINED_DOMAIN: u32 = 4u32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const TRUST_ATTRIBUTE_TREE_PARENT: u32 = 4194304u32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const TRUST_ATTRIBUTE_TREE_ROOT: u32 = 8388608u32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const TRUST_ATTRIBUTE_TRUST_USES_AES_KEYS: u32 = 256u32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const TRUST_ATTRIBUTE_TRUST_USES_RC4_ENCRYPTION: u32 = 128u32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const UNDERSTANDS_LONG_NAMES: u32 = 1u32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const UNISP_RPC_ID: u32 = 14u32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const USER_ACCOUNT_AUTO_LOCKED: u32 = 1024u32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const USER_ACCOUNT_DISABLED: u32 = 1u32;
pub struct USER_ALL_INFORMATION(i32);
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const USER_ALL_PARAMETERS: u32 = 2097152u32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const USER_DONT_EXPIRE_PASSWORD: u32 = 512u32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const USER_DONT_REQUIRE_PREAUTH: u32 = 65536u32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const USER_ENCRYPTED_TEXT_PASSWORD_ALLOWED: u32 = 2048u32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const USER_HOME_DIRECTORY_REQUIRED: u32 = 2u32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const USER_INTERDOMAIN_TRUST_ACCOUNT: u32 = 64u32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const USER_MNS_LOGON_ACCOUNT: u32 = 32u32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const USER_NORMAL_ACCOUNT: u32 = 16u32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const USER_NOT_DELEGATED: u32 = 16384u32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const USER_NO_AUTH_DATA_REQUIRED: u32 = 524288u32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const USER_PARTIAL_SECRETS_ACCOUNT: u32 = 1048576u32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const USER_PASSWORD_EXPIRED: u32 = 131072u32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const USER_PASSWORD_NOT_REQUIRED: u32 = 4u32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const USER_SERVER_TRUST_ACCOUNT: u32 = 256u32;
pub struct USER_SESSION_KEY(i32);
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const USER_SMARTCARD_REQUIRED: u32 = 4096u32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const USER_TEMP_DUPLICATE_ACCOUNT: u32 = 8u32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const USER_TRUSTED_FOR_DELEGATION: u32 = 8192u32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const USER_TRUSTED_TO_AUTHENTICATE_FOR_DELEGATION: u32 = 262144u32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const USER_USE_AES_KEYS: u32 = 2097152u32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const USER_USE_DES_KEY_ONLY: u32 = 32768u32;
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const USER_WORKSTATION_TRUST_ACCOUNT: u32 = 128u32;
pub struct VERIFY_SIGNATURE_FN(i32);
pub const WINDOWS_SLID: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 1439246132, data2: 54914, data3: 19825, data4: [152, 62, 214, 236, 63, 22, 5, 159] };
pub struct X509Certificate(i32);
#[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
pub const _FACILITY_WINDOWS_STORE: u32 = 63u32;
pub struct _HMAPPER(i32);
pub struct eTlsHashAlgorithm(i32);
pub struct eTlsSignatureAlgorithm(i32);
