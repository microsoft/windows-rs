#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    #[doc = "*Required features: `Win32_Security_Cryptography_Certificates`*"]
    pub fn CertSrvBackupClose(hbc: *mut ::core::ffi::c_void) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Security_Cryptography_Certificates`*"]
    pub fn CertSrvBackupEnd(hbc: *mut ::core::ffi::c_void) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Security_Cryptography_Certificates`*"]
    pub fn CertSrvBackupFree(pv: *mut ::core::ffi::c_void);
    #[doc = "*Required features: `Win32_Security_Cryptography_Certificates`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CertSrvBackupGetBackupLogsW(hbc: *const ::core::ffi::c_void, ppwszzbackuplogfiles: *mut super::super::super::Foundation::PWSTR, pcbsize: *mut u32) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Security_Cryptography_Certificates`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CertSrvBackupGetDatabaseNamesW(hbc: *const ::core::ffi::c_void, ppwszzattachmentinformation: *mut super::super::super::Foundation::PWSTR, pcbsize: *mut u32) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Security_Cryptography_Certificates`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CertSrvBackupGetDynamicFileListW(hbc: *const ::core::ffi::c_void, ppwszzfilelist: *mut super::super::super::Foundation::PWSTR, pcbsize: *mut u32) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Security_Cryptography_Certificates`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CertSrvBackupOpenFileW(hbc: *mut ::core::ffi::c_void, pwszattachmentname: super::super::super::Foundation::PWSTR, cbreadhintsize: u32, plifilesize: *mut i64) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Security_Cryptography_Certificates`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CertSrvBackupPrepareW(pwszservername: super::super::super::Foundation::PWSTR, grbitjet: u32, dwbackupflags: CSBACKUP_TYPE, phbc: *mut *mut ::core::ffi::c_void) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Security_Cryptography_Certificates`*"]
    pub fn CertSrvBackupRead(hbc: *mut ::core::ffi::c_void, pvbuffer: *mut ::core::ffi::c_void, cbbuffer: u32, pcbread: *mut u32) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Security_Cryptography_Certificates`*"]
    pub fn CertSrvBackupTruncateLogs(hbc: *mut ::core::ffi::c_void) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Security_Cryptography_Certificates`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CertSrvIsServerOnlineW(pwszservername: super::super::super::Foundation::PWSTR, pfserveronline: *mut super::super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Security_Cryptography_Certificates`*"]
    pub fn CertSrvRestoreEnd(hbc: *mut ::core::ffi::c_void) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Security_Cryptography_Certificates`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CertSrvRestoreGetDatabaseLocationsW(hbc: *const ::core::ffi::c_void, ppwszzdatabaselocationlist: *mut super::super::super::Foundation::PWSTR, pcbsize: *mut u32) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Security_Cryptography_Certificates`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CertSrvRestorePrepareW(pwszservername: super::super::super::Foundation::PWSTR, dwrestoreflags: u32, phbc: *mut *mut ::core::ffi::c_void) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Security_Cryptography_Certificates`*"]
    pub fn CertSrvRestoreRegisterComplete(hbc: *mut ::core::ffi::c_void, hrrestorestate: ::windows::runtime::HRESULT) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Security_Cryptography_Certificates`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CertSrvRestoreRegisterThroughFile(hbc: *mut ::core::ffi::c_void, pwszcheckpointfilepath: super::super::super::Foundation::PWSTR, pwszlogpath: super::super::super::Foundation::PWSTR, rgrstmap: *mut CSEDB_RSTMAPW, crstmap: i32, pwszbackuplogpath: super::super::super::Foundation::PWSTR, genlow: u32, genhigh: u32) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Security_Cryptography_Certificates`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CertSrvRestoreRegisterW(hbc: *mut ::core::ffi::c_void, pwszcheckpointfilepath: super::super::super::Foundation::PWSTR, pwszlogpath: super::super::super::Foundation::PWSTR, rgrstmap: *mut CSEDB_RSTMAPW, crstmap: i32, pwszbackuplogpath: super::super::super::Foundation::PWSTR, genlow: u32, genhigh: u32) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Security_Cryptography_Certificates`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CertSrvServerControlW(pwszservername: super::super::super::Foundation::PWSTR, dwcontrolflags: u32, pcbout: *mut u32, ppbout: *mut *mut u8) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Security_Cryptography_Certificates`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PstAcquirePrivateKey(pcert: *const super::CERT_CONTEXT) -> super::super::super::Foundation::NTSTATUS;
    #[doc = "*Required features: `Win32_Security_Cryptography_Certificates`, `Win32_Foundation`, `Win32_Security_Authentication_Identity`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Authentication_Identity"))]
    pub fn PstGetCertificateChain(pcert: *const super::CERT_CONTEXT, ptrustedissuers: *const super::super::Authentication::Identity::SecPkgContext_IssuerListInfoEx, ppcertchaincontext: *mut *mut super::CERT_CHAIN_CONTEXT) -> super::super::super::Foundation::NTSTATUS;
    #[doc = "*Required features: `Win32_Security_Cryptography_Certificates`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PstGetCertificates(ptargetname: *const super::super::super::Foundation::UNICODE_STRING, ccriteria: u32, rgpcriteria: *const super::CERT_SELECT_CRITERIA, bisclient: super::super::super::Foundation::BOOL, pdwcertchaincontextcount: *mut u32, ppcertchaincontexts: *mut *mut *mut super::CERT_CHAIN_CONTEXT) -> super::super::super::Foundation::NTSTATUS;
    #[doc = "*Required features: `Win32_Security_Cryptography_Certificates`, `Win32_Foundation`, `Win32_Security_Authentication_Identity`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Authentication_Identity"))]
    pub fn PstGetTrustAnchors(ptargetname: *const super::super::super::Foundation::UNICODE_STRING, ccriteria: u32, rgpcriteria: *const super::CERT_SELECT_CRITERIA, pptrustedissuers: *mut *mut super::super::Authentication::Identity::SecPkgContext_IssuerListInfoEx) -> super::super::super::Foundation::NTSTATUS;
    #[doc = "*Required features: `Win32_Security_Cryptography_Certificates`, `Win32_Foundation`, `Win32_Security_Authentication_Identity`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Authentication_Identity"))]
    pub fn PstGetTrustAnchorsEx(ptargetname: *const super::super::super::Foundation::UNICODE_STRING, ccriteria: u32, rgpcriteria: *const super::CERT_SELECT_CRITERIA, pcertcontext: *const super::CERT_CONTEXT, pptrustedissuers: *mut *mut super::super::Authentication::Identity::SecPkgContext_IssuerListInfoEx) -> super::super::super::Foundation::NTSTATUS;
    #[doc = "*Required features: `Win32_Security_Cryptography_Certificates`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PstGetUserNameForCertificate(pcertcontext: *const super::CERT_CONTEXT, username: *mut super::super::super::Foundation::UNICODE_STRING) -> super::super::super::Foundation::NTSTATUS;
    #[doc = "*Required features: `Win32_Security_Cryptography_Certificates`, `Win32_Foundation`, `Win32_Security_Authentication_Identity`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Authentication_Identity"))]
    pub fn PstMapCertificate(pcert: *const super::CERT_CONTEXT, ptokeninformationtype: *mut super::super::Authentication::Identity::LSA_TOKEN_INFORMATION_TYPE, pptokeninformation: *mut *mut ::core::ffi::c_void) -> super::super::super::Foundation::NTSTATUS;
    #[doc = "*Required features: `Win32_Security_Cryptography_Certificates`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PstValidate(ptargetname: *const super::super::super::Foundation::UNICODE_STRING, bisclient: super::super::super::Foundation::BOOL, prequestedissuancepolicy: *const super::CERT_USAGE_MATCH, phadditionalcertstore: *const *const ::core::ffi::c_void, pcert: *const super::CERT_CONTEXT, pprovguid: *mut ::windows::runtime::GUID) -> super::super::super::Foundation::NTSTATUS;
}
