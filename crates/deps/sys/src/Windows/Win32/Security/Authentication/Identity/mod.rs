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
    pub fn AcquireCredentialsHandleA(pszprincipal: super::super::super::Foundation::PSTR, pszpackage: super::super::super::Foundation::PSTR, fcredentialuse: SECPKG_CRED, pvlogonid: *const ::core::ffi::c_void, pauthdata: *const ::core::ffi::c_void, pgetkeyfn: ::windows::runtime::RawPtr, pvgetkeyargument: *const ::core::ffi::c_void, phcredential: *mut super::super::Credentials::SecHandle, ptsexpiry: *mut i64) -> i32;
    #[doc = "*Required features: `Win32_Security_Authentication_Identity`, `Win32_Foundation`, `Win32_Security_Credentials`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Credentials"))]
    pub fn AcquireCredentialsHandleW(pszprincipal: super::super::super::Foundation::PWSTR, pszpackage: super::super::super::Foundation::PWSTR, fcredentialuse: SECPKG_CRED, pvlogonid: *const ::core::ffi::c_void, pauthdata: *const ::core::ffi::c_void, pgetkeyfn: ::windows::runtime::RawPtr, pvgetkeyargument: *const ::core::ffi::c_void, phcredential: *mut super::super::Credentials::SecHandle, ptsexpiry: *mut i64) -> i32;
    #[doc = "*Required features: `Win32_Security_Authentication_Identity`, `Win32_Foundation`, `Win32_Security_Credentials`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Credentials"))]
    pub fn AddCredentialsA(hcredentials: *const super::super::Credentials::SecHandle, pszprincipal: super::super::super::Foundation::PSTR, pszpackage: super::super::super::Foundation::PSTR, fcredentialuse: u32, pauthdata: *const ::core::ffi::c_void, pgetkeyfn: ::windows::runtime::RawPtr, pvgetkeyargument: *const ::core::ffi::c_void, ptsexpiry: *mut i64) -> i32;
    #[doc = "*Required features: `Win32_Security_Authentication_Identity`, `Win32_Foundation`, `Win32_Security_Credentials`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Credentials"))]
    pub fn AddCredentialsW(hcredentials: *const super::super::Credentials::SecHandle, pszprincipal: super::super::super::Foundation::PWSTR, pszpackage: super::super::super::Foundation::PWSTR, fcredentialuse: u32, pauthdata: *const ::core::ffi::c_void, pgetkeyfn: ::windows::runtime::RawPtr, pvgetkeyargument: *const ::core::ffi::c_void, ptsexpiry: *mut i64) -> i32;
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
    pub fn AuditComputeEffectivePolicyBySid(psid: super::super::super::Foundation::PSID, psubcategoryguids: *const ::windows::runtime::GUID, dwpolicycount: u32, ppauditpolicy: *mut *mut AUDIT_POLICY_INFORMATION) -> super::super::super::Foundation::BOOLEAN;
    #[doc = "*Required features: `Win32_Security_Authentication_Identity`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn AuditComputeEffectivePolicyByToken(htokenhandle: super::super::super::Foundation::HANDLE, psubcategoryguids: *const ::windows::runtime::GUID, dwpolicycount: u32, ppauditpolicy: *mut *mut AUDIT_POLICY_INFORMATION) -> super::super::super::Foundation::BOOLEAN;
    #[doc = "*Required features: `Win32_Security_Authentication_Identity`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn AuditEnumerateCategories(ppauditcategoriesarray: *mut *mut ::windows::runtime::GUID, pdwcountreturned: *mut u32) -> super::super::super::Foundation::BOOLEAN;
    #[doc = "*Required features: `Win32_Security_Authentication_Identity`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn AuditEnumeratePerUserPolicy(ppauditsidarray: *mut *mut POLICY_AUDIT_SID_ARRAY) -> super::super::super::Foundation::BOOLEAN;
    #[doc = "*Required features: `Win32_Security_Authentication_Identity`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn AuditEnumerateSubCategories(pauditcategoryguid: *const ::windows::runtime::GUID, bretrieveallsubcategories: super::super::super::Foundation::BOOLEAN, ppauditsubcategoriesarray: *mut *mut ::windows::runtime::GUID, pdwcountreturned: *mut u32) -> super::super::super::Foundation::BOOLEAN;
    #[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
    pub fn AuditFree(buffer: *const ::core::ffi::c_void);
    #[doc = "*Required features: `Win32_Security_Authentication_Identity`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn AuditLookupCategoryGuidFromCategoryId(auditcategoryid: POLICY_AUDIT_EVENT_TYPE, pauditcategoryguid: *mut ::windows::runtime::GUID) -> super::super::super::Foundation::BOOLEAN;
    #[doc = "*Required features: `Win32_Security_Authentication_Identity`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn AuditLookupCategoryIdFromCategoryGuid(pauditcategoryguid: *const ::windows::runtime::GUID, pauditcategoryid: *mut POLICY_AUDIT_EVENT_TYPE) -> super::super::super::Foundation::BOOLEAN;
    #[doc = "*Required features: `Win32_Security_Authentication_Identity`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn AuditLookupCategoryNameA(pauditcategoryguid: *const ::windows::runtime::GUID, ppszcategoryname: *mut super::super::super::Foundation::PSTR) -> super::super::super::Foundation::BOOLEAN;
    #[doc = "*Required features: `Win32_Security_Authentication_Identity`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn AuditLookupCategoryNameW(pauditcategoryguid: *const ::windows::runtime::GUID, ppszcategoryname: *mut super::super::super::Foundation::PWSTR) -> super::super::super::Foundation::BOOLEAN;
    #[doc = "*Required features: `Win32_Security_Authentication_Identity`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn AuditLookupSubCategoryNameA(pauditsubcategoryguid: *const ::windows::runtime::GUID, ppszsubcategoryname: *mut super::super::super::Foundation::PSTR) -> super::super::super::Foundation::BOOLEAN;
    #[doc = "*Required features: `Win32_Security_Authentication_Identity`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn AuditLookupSubCategoryNameW(pauditsubcategoryguid: *const ::windows::runtime::GUID, ppszsubcategoryname: *mut super::super::super::Foundation::PWSTR) -> super::super::super::Foundation::BOOLEAN;
    #[doc = "*Required features: `Win32_Security_Authentication_Identity`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn AuditQueryGlobalSaclA(objecttypename: super::super::super::Foundation::PSTR, acl: *mut *mut super::super::ACL) -> super::super::super::Foundation::BOOLEAN;
    #[doc = "*Required features: `Win32_Security_Authentication_Identity`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn AuditQueryGlobalSaclW(objecttypename: super::super::super::Foundation::PWSTR, acl: *mut *mut super::super::ACL) -> super::super::super::Foundation::BOOLEAN;
    #[doc = "*Required features: `Win32_Security_Authentication_Identity`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn AuditQueryPerUserPolicy(psid: super::super::super::Foundation::PSID, psubcategoryguids: *const ::windows::runtime::GUID, dwpolicycount: u32, ppauditpolicy: *mut *mut AUDIT_POLICY_INFORMATION) -> super::super::super::Foundation::BOOLEAN;
    #[doc = "*Required features: `Win32_Security_Authentication_Identity`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn AuditQuerySecurity(securityinformation: u32, ppsecuritydescriptor: *mut *mut super::super::SECURITY_DESCRIPTOR) -> super::super::super::Foundation::BOOLEAN;
    #[doc = "*Required features: `Win32_Security_Authentication_Identity`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn AuditQuerySystemPolicy(psubcategoryguids: *const ::windows::runtime::GUID, dwpolicycount: u32, ppauditpolicy: *mut *mut AUDIT_POLICY_INFORMATION) -> super::super::super::Foundation::BOOLEAN;
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
    pub fn SLAcquireGenuineTicket(ppticketblob: *mut *mut ::core::ffi::c_void, pcbticketblob: *mut u32, pwsztemplateid: super::super::super::Foundation::PWSTR, pwszserverurl: super::super::super::Foundation::PWSTR, pwszclienttoken: super::super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Security_Authentication_Identity`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SLActivateProduct(hslc: *const ::core::ffi::c_void, pproductskuid: *const ::windows::runtime::GUID, cbappspecificdata: u32, pvappspecificdata: *const ::core::ffi::c_void, pactivationinfo: *const SL_ACTIVATION_INFO_HEADER, pwszproxyserver: super::super::super::Foundation::PWSTR, wproxyport: u16) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
    pub fn SLClose(hslc: *const ::core::ffi::c_void) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Security_Authentication_Identity`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SLConsumeRight(hslc: *const ::core::ffi::c_void, pappid: *const ::windows::runtime::GUID, pproductskuid: *const ::windows::runtime::GUID, pwszrightname: super::super::super::Foundation::PWSTR, pvreserved: *mut ::core::ffi::c_void) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Security_Authentication_Identity`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SLDepositOfflineConfirmationId(hslc: *const ::core::ffi::c_void, pproductskuid: *const ::windows::runtime::GUID, pwszinstallationid: super::super::super::Foundation::PWSTR, pwszconfirmationid: super::super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Security_Authentication_Identity`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SLDepositOfflineConfirmationIdEx(hslc: *const ::core::ffi::c_void, pproductskuid: *const ::windows::runtime::GUID, pactivationinfo: *const SL_ACTIVATION_INFO_HEADER, pwszinstallationid: super::super::super::Foundation::PWSTR, pwszconfirmationid: super::super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Security_Authentication_Identity`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SLFireEvent(hslc: *const ::core::ffi::c_void, pwszeventid: super::super::super::Foundation::PWSTR, papplicationid: *const ::windows::runtime::GUID) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Security_Authentication_Identity`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SLGenerateOfflineInstallationId(hslc: *const ::core::ffi::c_void, pproductskuid: *const ::windows::runtime::GUID, ppwszinstallationid: *mut super::super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Security_Authentication_Identity`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SLGenerateOfflineInstallationIdEx(hslc: *const ::core::ffi::c_void, pproductskuid: *const ::windows::runtime::GUID, pactivationinfo: *const SL_ACTIVATION_INFO_HEADER, ppwszinstallationid: *mut super::super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Security_Authentication_Identity`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SLGetApplicationInformation(hslc: *const ::core::ffi::c_void, papplicationid: *const ::windows::runtime::GUID, pwszvaluename: super::super::super::Foundation::PWSTR, pedatatype: *mut SLDATATYPE, pcbvalue: *mut u32, ppbvalue: *mut *mut u8) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Security_Authentication_Identity`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SLGetGenuineInformation(pqueryid: *const ::windows::runtime::GUID, pwszvaluename: super::super::super::Foundation::PWSTR, pedatatype: *mut SLDATATYPE, pcbvalue: *mut u32, ppbvalue: *mut *mut u8) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
    pub fn SLGetInstalledProductKeyIds(hslc: *const ::core::ffi::c_void, pproductskuid: *const ::windows::runtime::GUID, pnproductkeyids: *mut u32, ppproductkeyids: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
    pub fn SLGetLicense(hslc: *const ::core::ffi::c_void, plicensefileid: *const ::windows::runtime::GUID, pcblicensefile: *mut u32, ppblicensefile: *mut *mut u8) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
    pub fn SLGetLicenseFileId(hslc: *const ::core::ffi::c_void, cblicenseblob: u32, pblicenseblob: *const u8, plicensefileid: *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Security_Authentication_Identity`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SLGetLicenseInformation(hslc: *const ::core::ffi::c_void, psllicenseid: *const ::windows::runtime::GUID, pwszvaluename: super::super::super::Foundation::PWSTR, pedatatype: *mut SLDATATYPE, pcbvalue: *mut u32, ppbvalue: *mut *mut u8) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Security_Authentication_Identity`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SLGetLicensingStatusInformation(hslc: *const ::core::ffi::c_void, pappid: *const ::windows::runtime::GUID, pproductskuid: *const ::windows::runtime::GUID, pwszrightname: super::super::super::Foundation::PWSTR, pnstatuscount: *mut u32, pplicensingstatus: *mut *mut SL_LICENSING_STATUS) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Security_Authentication_Identity`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SLGetPKeyId(hslc: *const ::core::ffi::c_void, pwszpkeyalgorithm: super::super::super::Foundation::PWSTR, pwszpkeystring: super::super::super::Foundation::PWSTR, cbpkeyspecificdata: u32, pbpkeyspecificdata: *const u8, ppkeyid: *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Security_Authentication_Identity`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SLGetPKeyInformation(hslc: *const ::core::ffi::c_void, ppkeyid: *const ::windows::runtime::GUID, pwszvaluename: super::super::super::Foundation::PWSTR, pedatatype: *mut SLDATATYPE, pcbvalue: *mut u32, ppbvalue: *mut *mut u8) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Security_Authentication_Identity`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SLGetPolicyInformation(hslc: *const ::core::ffi::c_void, pwszvaluename: super::super::super::Foundation::PWSTR, pedatatype: *mut SLDATATYPE, pcbvalue: *mut u32, ppbvalue: *mut *mut u8) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Security_Authentication_Identity`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SLGetPolicyInformationDWORD(hslc: *const ::core::ffi::c_void, pwszvaluename: super::super::super::Foundation::PWSTR, pdwvalue: *mut u32) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Security_Authentication_Identity`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SLGetProductSkuInformation(hslc: *const ::core::ffi::c_void, pproductskuid: *const ::windows::runtime::GUID, pwszvaluename: super::super::super::Foundation::PWSTR, pedatatype: *mut SLDATATYPE, pcbvalue: *mut u32, ppbvalue: *mut *mut u8) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Security_Authentication_Identity`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SLGetReferralInformation(hslc: *const ::core::ffi::c_void, ereferraltype: SLREFERRALTYPE, pskuorappid: *const ::windows::runtime::GUID, pwszvaluename: super::super::super::Foundation::PWSTR, ppwszvalue: *mut super::super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
    pub fn SLGetSLIDList(hslc: *const ::core::ffi::c_void, equeryidtype: SLIDTYPE, pqueryid: *const ::windows::runtime::GUID, ereturnidtype: SLIDTYPE, pnreturnids: *mut u32, ppreturnids: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Security_Authentication_Identity`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SLGetServerStatus(pwszserverurl: super::super::super::Foundation::PWSTR, pwszacquisitiontype: super::super::super::Foundation::PWSTR, pwszproxyserver: super::super::super::Foundation::PWSTR, wproxyport: u16, phrstatus: *mut ::windows::runtime::HRESULT) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Security_Authentication_Identity`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SLGetServiceInformation(hslc: *const ::core::ffi::c_void, pwszvaluename: super::super::super::Foundation::PWSTR, pedatatype: *mut SLDATATYPE, pcbvalue: *mut u32, ppbvalue: *mut *mut u8) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Security_Authentication_Identity`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SLGetWindowsInformation(pwszvaluename: super::super::super::Foundation::PWSTR, pedatatype: *mut SLDATATYPE, pcbvalue: *mut u32, ppbvalue: *mut *mut u8) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Security_Authentication_Identity`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SLGetWindowsInformationDWORD(pwszvaluename: super::super::super::Foundation::PWSTR, pdwvalue: *mut u32) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
    pub fn SLInstallLicense(hslc: *const ::core::ffi::c_void, cblicenseblob: u32, pblicenseblob: *const u8, plicensefileid: *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Security_Authentication_Identity`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SLInstallProofOfPurchase(hslc: *const ::core::ffi::c_void, pwszpkeyalgorithm: super::super::super::Foundation::PWSTR, pwszpkeystring: super::super::super::Foundation::PWSTR, cbpkeyspecificdata: u32, pbpkeyspecificdata: *const u8, ppkeyid: *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
    pub fn SLIsGenuineLocal(pappid: *const ::windows::runtime::GUID, pgenuinestate: *mut SL_GENUINE_STATE, puioptions: *mut SL_NONGENUINE_UI_OPTIONS) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
    pub fn SLOpen(phslc: *mut *mut ::core::ffi::c_void) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Security_Authentication_Identity`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SLQueryLicenseValueFromApp(valuename: super::super::super::Foundation::PWSTR, valuetype: *mut u32, databuffer: *mut ::core::ffi::c_void, datasize: u32, resultdatasize: *mut u32) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Security_Authentication_Identity`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SLRegisterEvent(hslc: *const ::core::ffi::c_void, pwszeventid: super::super::super::Foundation::PWSTR, papplicationid: *const ::windows::runtime::GUID, hevent: super::super::super::Foundation::HANDLE) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
    pub fn SLSetCurrentProductKey(hslc: *const ::core::ffi::c_void, pproductskuid: *const ::windows::runtime::GUID, pproductkeyid: *const ::windows::runtime::GUID) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Security_Authentication_Identity`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SLSetGenuineInformation(pqueryid: *const ::windows::runtime::GUID, pwszvaluename: super::super::super::Foundation::PWSTR, edatatype: SLDATATYPE, cbvalue: u32, pbvalue: *const u8) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
    pub fn SLUninstallLicense(hslc: *const ::core::ffi::c_void, plicensefileid: *const ::windows::runtime::GUID) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
    pub fn SLUninstallProofOfPurchase(hslc: *const ::core::ffi::c_void, ppkeyid: *const ::windows::runtime::GUID) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Security_Authentication_Identity`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SLUnregisterEvent(hslc: *const ::core::ffi::c_void, pwszeventid: super::super::super::Foundation::PWSTR, papplicationid: *const ::windows::runtime::GUID, hevent: super::super::super::Foundation::HANDLE) -> ::windows::runtime::HRESULT;
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
