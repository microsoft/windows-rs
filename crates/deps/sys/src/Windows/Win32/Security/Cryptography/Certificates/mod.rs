#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    #[doc = "*Required features: `Win32_Security_Cryptography_Certificates`*"]
    pub fn CertSrvBackupClose(hbc: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_Security_Cryptography_Certificates`*"]
    pub fn CertSrvBackupEnd(hbc: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_Security_Cryptography_Certificates`*"]
    pub fn CertSrvBackupFree(pv: *mut ::core::ffi::c_void);
    #[doc = "*Required features: `Win32_Security_Cryptography_Certificates`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CertSrvBackupGetBackupLogsW(hbc: *const ::core::ffi::c_void, ppwszzbackuplogfiles: *mut super::super::super::Foundation::PWSTR, pcbsize: *mut u32) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_Security_Cryptography_Certificates`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CertSrvBackupGetDatabaseNamesW(hbc: *const ::core::ffi::c_void, ppwszzattachmentinformation: *mut super::super::super::Foundation::PWSTR, pcbsize: *mut u32) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_Security_Cryptography_Certificates`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CertSrvBackupGetDynamicFileListW(hbc: *const ::core::ffi::c_void, ppwszzfilelist: *mut super::super::super::Foundation::PWSTR, pcbsize: *mut u32) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_Security_Cryptography_Certificates`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CertSrvBackupOpenFileW(hbc: *mut ::core::ffi::c_void, pwszattachmentname: super::super::super::Foundation::PWSTR, cbreadhintsize: u32, plifilesize: *mut i64) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_Security_Cryptography_Certificates`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CertSrvBackupPrepareW(pwszservername: super::super::super::Foundation::PWSTR, grbitjet: u32, dwbackupflags: CSBACKUP_TYPE, phbc: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_Security_Cryptography_Certificates`*"]
    pub fn CertSrvBackupRead(hbc: *mut ::core::ffi::c_void, pvbuffer: *mut ::core::ffi::c_void, cbbuffer: u32, pcbread: *mut u32) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_Security_Cryptography_Certificates`*"]
    pub fn CertSrvBackupTruncateLogs(hbc: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_Security_Cryptography_Certificates`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CertSrvIsServerOnlineW(pwszservername: super::super::super::Foundation::PWSTR, pfserveronline: *mut super::super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_Security_Cryptography_Certificates`*"]
    pub fn CertSrvRestoreEnd(hbc: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_Security_Cryptography_Certificates`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CertSrvRestoreGetDatabaseLocationsW(hbc: *const ::core::ffi::c_void, ppwszzdatabaselocationlist: *mut super::super::super::Foundation::PWSTR, pcbsize: *mut u32) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_Security_Cryptography_Certificates`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CertSrvRestorePrepareW(pwszservername: super::super::super::Foundation::PWSTR, dwrestoreflags: u32, phbc: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_Security_Cryptography_Certificates`*"]
    pub fn CertSrvRestoreRegisterComplete(hbc: *mut ::core::ffi::c_void, hrrestorestate: ::windows_sys::core::HRESULT) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_Security_Cryptography_Certificates`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CertSrvRestoreRegisterThroughFile(hbc: *mut ::core::ffi::c_void, pwszcheckpointfilepath: super::super::super::Foundation::PWSTR, pwszlogpath: super::super::super::Foundation::PWSTR, rgrstmap: *mut CSEDB_RSTMAPW, crstmap: i32, pwszbackuplogpath: super::super::super::Foundation::PWSTR, genlow: u32, genhigh: u32) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_Security_Cryptography_Certificates`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CertSrvRestoreRegisterW(hbc: *mut ::core::ffi::c_void, pwszcheckpointfilepath: super::super::super::Foundation::PWSTR, pwszlogpath: super::super::super::Foundation::PWSTR, rgrstmap: *mut CSEDB_RSTMAPW, crstmap: i32, pwszbackuplogpath: super::super::super::Foundation::PWSTR, genlow: u32, genhigh: u32) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_Security_Cryptography_Certificates`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CertSrvServerControlW(pwszservername: super::super::super::Foundation::PWSTR, dwcontrolflags: u32, pcbout: *mut u32, ppbout: *mut *mut u8) -> ::windows_sys::core::HRESULT;
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
    pub fn PstValidate(ptargetname: *const super::super::super::Foundation::UNICODE_STRING, bisclient: super::super::super::Foundation::BOOL, prequestedissuancepolicy: *const super::CERT_USAGE_MATCH, phadditionalcertstore: *const *const ::core::ffi::c_void, pcert: *const super::CERT_CONTEXT, pprovguid: *mut ::windows_sys::core::GUID) -> super::super::super::Foundation::NTSTATUS;
}
pub struct ADDED_CERT_TYPE(i32);
pub struct AlgorithmFlags(i32);
pub struct AlgorithmOperationFlags(i32);
pub struct AlgorithmType(i32);
pub struct AlternativeNameType(i32);
#[doc = "*Required features: `Win32_Security_Cryptography_Certificates`*"]
pub const CAIF_DSENTRY: u32 = 1u32;
#[doc = "*Required features: `Win32_Security_Cryptography_Certificates`*"]
pub const CAIF_LOCAL: u32 = 8u32;
#[doc = "*Required features: `Win32_Security_Cryptography_Certificates`*"]
pub const CAIF_REGISTRY: u32 = 4u32;
#[doc = "*Required features: `Win32_Security_Cryptography_Certificates`*"]
pub const CAIF_REGISTRYPARENT: u32 = 16u32;
#[doc = "*Required features: `Win32_Security_Cryptography_Certificates`*"]
pub const CAIF_SHAREDFOLDERENTRY: u32 = 2u32;
pub struct CAINFO(i32);
#[doc = "*Required features: `Win32_Security_Cryptography_Certificates`*"]
pub const CAPATHLENGTH_INFINITE: u32 = 4294967295u32;
#[doc = "*Required features: `Win32_Security_Cryptography_Certificates`*"]
pub const CA_ACCESS_MASKROLES: u32 = 255u32;
#[doc = "*Required features: `Win32_Security_Cryptography_Certificates`*"]
pub const CA_CRL_BASE: u32 = 1u32;
#[doc = "*Required features: `Win32_Security_Cryptography_Certificates`*"]
pub const CA_CRL_DELTA: u32 = 2u32;
#[doc = "*Required features: `Win32_Security_Cryptography_Certificates`*"]
pub const CA_CRL_REPUBLISH: u32 = 16u32;
#[doc = "*Required features: `Win32_Security_Cryptography_Certificates`*"]
pub const CA_DISP_ERROR: u32 = 1u32;
#[doc = "*Required features: `Win32_Security_Cryptography_Certificates`*"]
pub const CA_DISP_INCOMPLETE: u32 = 0u32;
#[doc = "*Required features: `Win32_Security_Cryptography_Certificates`*"]
pub const CA_DISP_INVALID: u32 = 4u32;
#[doc = "*Required features: `Win32_Security_Cryptography_Certificates`*"]
pub const CA_DISP_REVOKED: u32 = 2u32;
#[doc = "*Required features: `Win32_Security_Cryptography_Certificates`*"]
pub const CA_DISP_UNDER_SUBMISSION: u32 = 5u32;
#[doc = "*Required features: `Win32_Security_Cryptography_Certificates`*"]
pub const CA_DISP_VALID: u32 = 3u32;
pub struct CAlternativeName(i32);
pub struct CAlternativeNames(i32);
pub struct CBinaryConverter(i32);
#[doc = "*Required features: `Win32_Security_Cryptography_Certificates`*"]
pub const CCLOCKSKEWMINUTESDEFAULT: u32 = 10u32;
#[doc = "*Required features: `Win32_Security_Cryptography_Certificates`*"]
pub const CC_UIPICKCONFIGSKIPLOCALCA: u32 = 5u32;
pub struct CCertAdmin(i32);
pub struct CCertConfig(i32);
pub struct CCertEncodeAltName(i32);
pub struct CCertEncodeBitString(i32);
pub struct CCertEncodeCRLDistInfo(i32);
pub struct CCertEncodeDateArray(i32);
pub struct CCertEncodeLongArray(i32);
pub struct CCertEncodeStringArray(i32);
pub struct CCertGetConfig(i32);
pub struct CCertProperties(i32);
pub struct CCertProperty(i32);
pub struct CCertPropertyArchived(i32);
pub struct CCertPropertyArchivedKeyHash(i32);
pub struct CCertPropertyAutoEnroll(i32);
pub struct CCertPropertyBackedUp(i32);
pub struct CCertPropertyDescription(i32);
pub struct CCertPropertyEnrollment(i32);
pub struct CCertPropertyEnrollmentPolicyServer(i32);
pub struct CCertPropertyFriendlyName(i32);
pub struct CCertPropertyKeyProvInfo(i32);
pub struct CCertPropertyRenewal(i32);
pub struct CCertPropertyRequestOriginator(i32);
pub struct CCertPropertySHA1Hash(i32);
pub struct CCertRequest(i32);
pub struct CCertServerExit(i32);
pub struct CCertServerPolicy(i32);
pub struct CCertView(i32);
pub struct CCertificateAttestationChallenge(i32);
pub struct CCertificatePolicies(i32);
pub struct CCertificatePolicy(i32);
pub struct CCryptAttribute(i32);
pub struct CCryptAttributes(i32);
pub struct CCspInformation(i32);
pub struct CCspInformations(i32);
pub struct CCspStatus(i32);
pub struct CERTADMIN_GET_ROLES_FLAGS(i32);
#[doc = "*Required features: `Win32_Security_Cryptography_Certificates`*"]
pub const CERTENROLL_INDEX_BASE: u32 = 0u32;
pub struct CERTENROLL_OBJECTID(i32);
pub struct CERTENROLL_PROPERTYID(i32);
pub struct CERTTRANSBLOB(i32);
pub struct CERTVIEWRESTRICTION(i32);
pub struct CERT_ALT_NAME(i32);
pub struct CERT_CREATE_REQUEST_FLAGS(i32);
pub struct CERT_DELETE_ROW_FLAGS(i32);
pub struct CERT_EXIT_EVENT_MASK(i32);
pub struct CERT_GET_CONFIG_FLAGS(i32);
pub struct CERT_IMPORT_FLAGS(i32);
pub struct CERT_PROPERTY_TYPE(i32);
pub struct CERT_REQUEST_OUT_TYPE(i32);
pub struct CERT_VIEW_COLUMN_INDEX(i32);
pub struct CERT_VIEW_SEEK_OPERATOR_FLAGS(i32);
pub struct CEnroll(i32);
pub struct CEnroll2(i32);
#[doc = "*Required features: `Win32_Security_Cryptography_Certificates`*"]
pub const CMM_READONLY: u32 = 2u32;
#[doc = "*Required features: `Win32_Security_Cryptography_Certificates`*"]
pub const CMM_REFRESHONLY: u32 = 1u32;
pub struct CObjectId(i32);
pub struct CObjectIds(i32);
#[doc = "*Required features: `Win32_Security_Cryptography_Certificates`*"]
pub const CPF_BADURL_ERROR: u32 = 32u32;
#[doc = "*Required features: `Win32_Security_Cryptography_Certificates`*"]
pub const CPF_BASE: u32 = 1u32;
#[doc = "*Required features: `Win32_Security_Cryptography_Certificates`*"]
pub const CPF_CASTORE_ERROR: u32 = 16u32;
#[doc = "*Required features: `Win32_Security_Cryptography_Certificates`*"]
pub const CPF_COMPLETE: u32 = 4u32;
#[doc = "*Required features: `Win32_Security_Cryptography_Certificates`*"]
pub const CPF_DELTA: u32 = 2u32;
#[doc = "*Required features: `Win32_Security_Cryptography_Certificates`*"]
pub const CPF_FILE_ERROR: u32 = 512u32;
#[doc = "*Required features: `Win32_Security_Cryptography_Certificates`*"]
pub const CPF_FTP_ERROR: u32 = 1024u32;
#[doc = "*Required features: `Win32_Security_Cryptography_Certificates`*"]
pub const CPF_HTTP_ERROR: u32 = 2048u32;
#[doc = "*Required features: `Win32_Security_Cryptography_Certificates`*"]
pub const CPF_LDAP_ERROR: u32 = 256u32;
#[doc = "*Required features: `Win32_Security_Cryptography_Certificates`*"]
pub const CPF_MANUAL: u32 = 64u32;
#[doc = "*Required features: `Win32_Security_Cryptography_Certificates`*"]
pub const CPF_POSTPONED_BASE_FILE_ERROR: u32 = 8192u32;
#[doc = "*Required features: `Win32_Security_Cryptography_Certificates`*"]
pub const CPF_POSTPONED_BASE_LDAP_ERROR: u32 = 4096u32;
#[doc = "*Required features: `Win32_Security_Cryptography_Certificates`*"]
pub const CPF_SHADOW: u32 = 8u32;
#[doc = "*Required features: `Win32_Security_Cryptography_Certificates`*"]
pub const CPF_SIGNATURE_ERROR: u32 = 128u32;
pub struct CPolicyQualifier(i32);
pub struct CPolicyQualifiers(i32);
#[doc = "*Required features: `Win32_Security_Cryptography_Certificates`*"]
pub const CRLF_ALLOW_REQUEST_ATTRIBUTE_SUBJECT: u32 = 65536u32;
#[doc = "*Required features: `Win32_Security_Cryptography_Certificates`*"]
pub const CRLF_BUILD_ROOTCA_CRLENTRIES_BASEDONKEY: u32 = 2097152u32;
#[doc = "*Required features: `Win32_Security_Cryptography_Certificates`*"]
pub const CRLF_CRLNUMBER_CRITICAL: u32 = 4u32;
#[doc = "*Required features: `Win32_Security_Cryptography_Certificates`*"]
pub const CRLF_DELETE_EXPIRED_CRLS: u32 = 2u32;
#[doc = "*Required features: `Win32_Security_Cryptography_Certificates`*"]
pub const CRLF_DELTA_USE_OLDEST_UNEXPIRED_BASE: u32 = 1u32;
#[doc = "*Required features: `Win32_Security_Cryptography_Certificates`*"]
pub const CRLF_DISABLE_CHAIN_VERIFICATION: u32 = 1048576u32;
#[doc = "*Required features: `Win32_Security_Cryptography_Certificates`*"]
pub const CRLF_DISABLE_RDN_REORDER: u32 = 2048u32;
#[doc = "*Required features: `Win32_Security_Cryptography_Certificates`*"]
pub const CRLF_DISABLE_ROOT_CROSS_CERTS: u32 = 4096u32;
#[doc = "*Required features: `Win32_Security_Cryptography_Certificates`*"]
pub const CRLF_ENFORCE_ENROLLMENT_AGENT: u32 = 1024u32;
#[doc = "*Required features: `Win32_Security_Cryptography_Certificates`*"]
pub const CRLF_IGNORE_CROSS_CERT_TRUST_ERROR: u32 = 256u32;
#[doc = "*Required features: `Win32_Security_Cryptography_Certificates`*"]
pub const CRLF_IGNORE_INVALID_POLICIES: u32 = 16u32;
#[doc = "*Required features: `Win32_Security_Cryptography_Certificates`*"]
pub const CRLF_IGNORE_UNKNOWN_CMC_ATTRIBUTES: u32 = 128u32;
#[doc = "*Required features: `Win32_Security_Cryptography_Certificates`*"]
pub const CRLF_LOG_FULL_RESPONSE: u32 = 8192u32;
#[doc = "*Required features: `Win32_Security_Cryptography_Certificates`*"]
pub const CRLF_PRESERVE_EXPIRED_CA_CERTS: u32 = 262144u32;
#[doc = "*Required features: `Win32_Security_Cryptography_Certificates`*"]
pub const CRLF_PRESERVE_REVOKED_CA_CERTS: u32 = 524288u32;
#[doc = "*Required features: `Win32_Security_Cryptography_Certificates`*"]
pub const CRLF_PUBLISH_EXPIRED_CERT_CRLS: u32 = 512u32;
#[doc = "*Required features: `Win32_Security_Cryptography_Certificates`*"]
pub const CRLF_REBUILD_MODIFIED_SUBJECT_ONLY: u32 = 32u32;
#[doc = "*Required features: `Win32_Security_Cryptography_Certificates`*"]
pub const CRLF_REVCHECK_IGNORE_NOREVCHECK: u32 = 131072u32;
#[doc = "*Required features: `Win32_Security_Cryptography_Certificates`*"]
pub const CRLF_REVCHECK_IGNORE_OFFLINE: u32 = 8u32;
#[doc = "*Required features: `Win32_Security_Cryptography_Certificates`*"]
pub const CRLF_SAVE_FAILED_CERTS: u32 = 64u32;
#[doc = "*Required features: `Win32_Security_Cryptography_Certificates`*"]
pub const CRLF_USE_CROSS_CERT_TEMPLATE: u32 = 32768u32;
#[doc = "*Required features: `Win32_Security_Cryptography_Certificates`*"]
pub const CRLF_USE_XCHG_CERT_TEMPLATE: u32 = 16384u32;
pub struct CRLRevocationReason(i32);
#[doc = "*Required features: `Win32_Security_Cryptography_Certificates`*"]
pub const CRYPT_ENUM_ALL_PROVIDERS: u32 = 1u32;
pub struct CR_DISP(i32);
#[doc = "*Required features: `Win32_Security_Cryptography_Certificates`*"]
pub const CR_DISP_REVOKED: u32 = 6u32;
#[doc = "*Required features: `Win32_Security_Cryptography_Certificates`*"]
pub const CR_FLG_CACROSSCERT: u32 = 128u32;
#[doc = "*Required features: `Win32_Security_Cryptography_Certificates`*"]
pub const CR_FLG_CAXCHGCERT: u32 = 8u32;
#[doc = "*Required features: `Win32_Security_Cryptography_Certificates`*"]
pub const CR_FLG_CHALLENGEPENDING: u32 = 1024u32;
#[doc = "*Required features: `Win32_Security_Cryptography_Certificates`*"]
pub const CR_FLG_CHALLENGESATISFIED: u32 = 2048u32;
#[doc = "*Required features: `Win32_Security_Cryptography_Certificates`*"]
pub const CR_FLG_DEFINEDCACERT: u32 = 512u32;
#[doc = "*Required features: `Win32_Security_Cryptography_Certificates`*"]
pub const CR_FLG_ENFORCEUTF8: u32 = 256u32;
#[doc = "*Required features: `Win32_Security_Cryptography_Certificates`*"]
pub const CR_FLG_ENROLLONBEHALFOF: u32 = 16u32;
#[doc = "*Required features: `Win32_Security_Cryptography_Certificates`*"]
pub const CR_FLG_FORCETELETEX: u32 = 1u32;
#[doc = "*Required features: `Win32_Security_Cryptography_Certificates`*"]
pub const CR_FLG_FORCEUTF8: u32 = 4u32;
#[doc = "*Required features: `Win32_Security_Cryptography_Certificates`*"]
pub const CR_FLG_PUBLISHERROR: u32 = 2147483648u32;
#[doc = "*Required features: `Win32_Security_Cryptography_Certificates`*"]
pub const CR_FLG_RENEWAL: u32 = 2u32;
#[doc = "*Required features: `Win32_Security_Cryptography_Certificates`*"]
pub const CR_FLG_SUBJECTUNMODIFIED: u32 = 32u32;
#[doc = "*Required features: `Win32_Security_Cryptography_Certificates`*"]
pub const CR_FLG_TRUSTEKCERT: u32 = 8192u32;
#[doc = "*Required features: `Win32_Security_Cryptography_Certificates`*"]
pub const CR_FLG_TRUSTEKKEY: u32 = 16384u32;
#[doc = "*Required features: `Win32_Security_Cryptography_Certificates`*"]
pub const CR_FLG_TRUSTONUSE: u32 = 4096u32;
#[doc = "*Required features: `Win32_Security_Cryptography_Certificates`*"]
pub const CR_FLG_VALIDENCRYPTEDKEYHASH: u32 = 64u32;
#[doc = "*Required features: `Win32_Security_Cryptography_Certificates`*"]
pub const CR_GEMT_DEFAULT: u32 = 0u32;
#[doc = "*Required features: `Win32_Security_Cryptography_Certificates`*"]
pub const CR_GEMT_HRESULT_STRING: u32 = 1u32;
#[doc = "*Required features: `Win32_Security_Cryptography_Certificates`*"]
pub const CR_GEMT_HTTP_ERROR: u32 = 2u32;
#[doc = "*Required features: `Win32_Security_Cryptography_Certificates`*"]
pub const CR_IN_CERTIFICATETRANSPARENCY: u32 = 67108864u32;
#[doc = "*Required features: `Win32_Security_Cryptography_Certificates`*"]
pub const CR_IN_CHALLENGERESPONSE: u32 = 1280u32;
#[doc = "*Required features: `Win32_Security_Cryptography_Certificates`*"]
pub const CR_IN_CLIENTIDNONE: u32 = 4194304u32;
#[doc = "*Required features: `Win32_Security_Cryptography_Certificates`*"]
pub const CR_IN_CMC: u32 = 1024u32;
#[doc = "*Required features: `Win32_Security_Cryptography_Certificates`*"]
pub const CR_IN_CONNECTONLY: u32 = 8388608u32;
#[doc = "*Required features: `Win32_Security_Cryptography_Certificates`*"]
pub const CR_IN_CRLS: u32 = 524288u32;
#[doc = "*Required features: `Win32_Security_Cryptography_Certificates`*"]
pub const CR_IN_ENCODEANY: u32 = 255u32;
#[doc = "*Required features: `Win32_Security_Cryptography_Certificates`*"]
pub const CR_IN_ENCODEMASK: u32 = 255u32;
#[doc = "*Required features: `Win32_Security_Cryptography_Certificates`*"]
pub const CR_IN_FORMATANY: u32 = 0u32;
#[doc = "*Required features: `Win32_Security_Cryptography_Certificates`*"]
pub const CR_IN_FORMATMASK: u32 = 65280u32;
#[doc = "*Required features: `Win32_Security_Cryptography_Certificates`*"]
pub const CR_IN_FULLRESPONSE: u32 = 262144u32;
#[doc = "*Required features: `Win32_Security_Cryptography_Certificates`*"]
pub const CR_IN_HTTP: u32 = 196608u32;
#[doc = "*Required features: `Win32_Security_Cryptography_Certificates`*"]
pub const CR_IN_KEYGEN: u32 = 512u32;
#[doc = "*Required features: `Win32_Security_Cryptography_Certificates`*"]
pub const CR_IN_MACHINE: u32 = 1048576u32;
#[doc = "*Required features: `Win32_Security_Cryptography_Certificates`*"]
pub const CR_IN_PKCS10: u32 = 256u32;
#[doc = "*Required features: `Win32_Security_Cryptography_Certificates`*"]
pub const CR_IN_PKCS7: u32 = 768u32;
#[doc = "*Required features: `Win32_Security_Cryptography_Certificates`*"]
pub const CR_IN_RETURNCHALLENGE: u32 = 16777216u32;
#[doc = "*Required features: `Win32_Security_Cryptography_Certificates`*"]
pub const CR_IN_ROBO: u32 = 2097152u32;
#[doc = "*Required features: `Win32_Security_Cryptography_Certificates`*"]
pub const CR_IN_RPC: u32 = 131072u32;
#[doc = "*Required features: `Win32_Security_Cryptography_Certificates`*"]
pub const CR_IN_SCEP: u32 = 65536u32;
#[doc = "*Required features: `Win32_Security_Cryptography_Certificates`*"]
pub const CR_IN_SCEPPOST: u32 = 33554432u32;
#[doc = "*Required features: `Win32_Security_Cryptography_Certificates`*"]
pub const CR_IN_SIGNEDCERTIFICATETIMESTAMPLIST: u32 = 1536u32;
#[doc = "*Required features: `Win32_Security_Cryptography_Certificates`*"]
pub const CR_OUT_BASE64REQUESTHEADER: u32 = 3u32;
#[doc = "*Required features: `Win32_Security_Cryptography_Certificates`*"]
pub const CR_OUT_BASE64X509CRLHEADER: u32 = 9u32;
#[doc = "*Required features: `Win32_Security_Cryptography_Certificates`*"]
pub const CR_OUT_CHAIN: u32 = 256u32;
#[doc = "*Required features: `Win32_Security_Cryptography_Certificates`*"]
pub const CR_OUT_CRLS: u32 = 512u32;
#[doc = "*Required features: `Win32_Security_Cryptography_Certificates`*"]
pub const CR_OUT_ENCODEMASK: u32 = 255u32;
#[doc = "*Required features: `Win32_Security_Cryptography_Certificates`*"]
pub const CR_OUT_HEX: u32 = 4u32;
#[doc = "*Required features: `Win32_Security_Cryptography_Certificates`*"]
pub const CR_OUT_HEXADDR: u32 = 10u32;
#[doc = "*Required features: `Win32_Security_Cryptography_Certificates`*"]
pub const CR_OUT_HEXASCII: u32 = 5u32;
#[doc = "*Required features: `Win32_Security_Cryptography_Certificates`*"]
pub const CR_OUT_HEXASCIIADDR: u32 = 11u32;
#[doc = "*Required features: `Win32_Security_Cryptography_Certificates`*"]
pub const CR_OUT_HEXRAW: u32 = 12u32;
#[doc = "*Required features: `Win32_Security_Cryptography_Certificates`*"]
pub const CR_OUT_NOCR: u32 = 2147483648u32;
#[doc = "*Required features: `Win32_Security_Cryptography_Certificates`*"]
pub const CR_OUT_NOCRLF: u32 = 1073741824u32;
#[doc = "*Required features: `Win32_Security_Cryptography_Certificates`*"]
pub const CR_PROP_ADVANCEDSERVER: u32 = 28u32;
#[doc = "*Required features: `Win32_Security_Cryptography_Certificates`*"]
pub const CR_PROP_BASECRL: u32 = 17u32;
#[doc = "*Required features: `Win32_Security_Cryptography_Certificates`*"]
pub const CR_PROP_BASECRLPUBLISHSTATUS: u32 = 30u32;
#[doc = "*Required features: `Win32_Security_Cryptography_Certificates`*"]
pub const CR_PROP_CABACKWARDCROSSCERT: u32 = 36u32;
#[doc = "*Required features: `Win32_Security_Cryptography_Certificates`*"]
pub const CR_PROP_CABACKWARDCROSSCERTSTATE: u32 = 38u32;
#[doc = "*Required features: `Win32_Security_Cryptography_Certificates`*"]
pub const CR_PROP_CACERTSTATE: u32 = 19u32;
#[doc = "*Required features: `Win32_Security_Cryptography_Certificates`*"]
pub const CR_PROP_CACERTSTATUSCODE: u32 = 34u32;
#[doc = "*Required features: `Win32_Security_Cryptography_Certificates`*"]
pub const CR_PROP_CACERTVERSION: u32 = 39u32;
#[doc = "*Required features: `Win32_Security_Cryptography_Certificates`*"]
pub const CR_PROP_CAFORWARDCROSSCERT: u32 = 35u32;
#[doc = "*Required features: `Win32_Security_Cryptography_Certificates`*"]
pub const CR_PROP_CAFORWARDCROSSCERTSTATE: u32 = 37u32;
#[doc = "*Required features: `Win32_Security_Cryptography_Certificates`*"]
pub const CR_PROP_CANAME: u32 = 6u32;
#[doc = "*Required features: `Win32_Security_Cryptography_Certificates`*"]
pub const CR_PROP_CAPROPIDMAX: u32 = 21u32;
#[doc = "*Required features: `Win32_Security_Cryptography_Certificates`*"]
pub const CR_PROP_CASIGCERT: u32 = 12u32;
#[doc = "*Required features: `Win32_Security_Cryptography_Certificates`*"]
pub const CR_PROP_CASIGCERTCHAIN: u32 = 13u32;
#[doc = "*Required features: `Win32_Security_Cryptography_Certificates`*"]
pub const CR_PROP_CASIGCERTCOUNT: u32 = 11u32;
#[doc = "*Required features: `Win32_Security_Cryptography_Certificates`*"]
pub const CR_PROP_CASIGCERTCRLCHAIN: u32 = 32u32;
#[doc = "*Required features: `Win32_Security_Cryptography_Certificates`*"]
pub const CR_PROP_CATYPE: u32 = 10u32;
#[doc = "*Required features: `Win32_Security_Cryptography_Certificates`*"]
pub const CR_PROP_CAXCHGCERT: u32 = 15u32;
#[doc = "*Required features: `Win32_Security_Cryptography_Certificates`*"]
pub const CR_PROP_CAXCHGCERTCHAIN: u32 = 16u32;
#[doc = "*Required features: `Win32_Security_Cryptography_Certificates`*"]
pub const CR_PROP_CAXCHGCERTCOUNT: u32 = 14u32;
#[doc = "*Required features: `Win32_Security_Cryptography_Certificates`*"]
pub const CR_PROP_CAXCHGCERTCRLCHAIN: u32 = 33u32;
#[doc = "*Required features: `Win32_Security_Cryptography_Certificates`*"]
pub const CR_PROP_CERTAIAOCSPURLS: u32 = 43u32;
#[doc = "*Required features: `Win32_Security_Cryptography_Certificates`*"]
pub const CR_PROP_CERTAIAURLS: u32 = 42u32;
#[doc = "*Required features: `Win32_Security_Cryptography_Certificates`*"]
pub const CR_PROP_CERTCDPURLS: u32 = 41u32;
#[doc = "*Required features: `Win32_Security_Cryptography_Certificates`*"]
pub const CR_PROP_CRLSTATE: u32 = 20u32;
#[doc = "*Required features: `Win32_Security_Cryptography_Certificates`*"]
pub const CR_PROP_DELTACRL: u32 = 18u32;
#[doc = "*Required features: `Win32_Security_Cryptography_Certificates`*"]
pub const CR_PROP_DELTACRLPUBLISHSTATUS: u32 = 31u32;
#[doc = "*Required features: `Win32_Security_Cryptography_Certificates`*"]
pub const CR_PROP_DNSNAME: u32 = 22u32;
#[doc = "*Required features: `Win32_Security_Cryptography_Certificates`*"]
pub const CR_PROP_EXITCOUNT: u32 = 3u32;
#[doc = "*Required features: `Win32_Security_Cryptography_Certificates`*"]
pub const CR_PROP_EXITDESCRIPTION: u32 = 4u32;
#[doc = "*Required features: `Win32_Security_Cryptography_Certificates`*"]
pub const CR_PROP_FILEVERSION: u32 = 1u32;
#[doc = "*Required features: `Win32_Security_Cryptography_Certificates`*"]
pub const CR_PROP_KRACERT: u32 = 26u32;
#[doc = "*Required features: `Win32_Security_Cryptography_Certificates`*"]
pub const CR_PROP_KRACERTCOUNT: u32 = 25u32;
#[doc = "*Required features: `Win32_Security_Cryptography_Certificates`*"]
pub const CR_PROP_KRACERTSTATE: u32 = 27u32;
#[doc = "*Required features: `Win32_Security_Cryptography_Certificates`*"]
pub const CR_PROP_KRACERTUSEDCOUNT: u32 = 24u32;
#[doc = "*Required features: `Win32_Security_Cryptography_Certificates`*"]
pub const CR_PROP_LOCALENAME: u32 = 44u32;
#[doc = "*Required features: `Win32_Security_Cryptography_Certificates`*"]
pub const CR_PROP_NONE: u32 = 0u32;
#[doc = "*Required features: `Win32_Security_Cryptography_Certificates`*"]
pub const CR_PROP_PARENTCA: u32 = 9u32;
#[doc = "*Required features: `Win32_Security_Cryptography_Certificates`*"]
pub const CR_PROP_POLICYDESCRIPTION: u32 = 5u32;
#[doc = "*Required features: `Win32_Security_Cryptography_Certificates`*"]
pub const CR_PROP_PRODUCTVERSION: u32 = 2u32;
#[doc = "*Required features: `Win32_Security_Cryptography_Certificates`*"]
pub const CR_PROP_ROLESEPARATIONENABLED: u32 = 23u32;
#[doc = "*Required features: `Win32_Security_Cryptography_Certificates`*"]
pub const CR_PROP_SANITIZEDCANAME: u32 = 7u32;
#[doc = "*Required features: `Win32_Security_Cryptography_Certificates`*"]
pub const CR_PROP_SANITIZEDCASHORTNAME: u32 = 40u32;
#[doc = "*Required features: `Win32_Security_Cryptography_Certificates`*"]
pub const CR_PROP_SCEPMAX: u32 = 1002u32;
#[doc = "*Required features: `Win32_Security_Cryptography_Certificates`*"]
pub const CR_PROP_SCEPMIN: u32 = 1000u32;
#[doc = "*Required features: `Win32_Security_Cryptography_Certificates`*"]
pub const CR_PROP_SCEPSERVERCAPABILITIES: u32 = 1001u32;
#[doc = "*Required features: `Win32_Security_Cryptography_Certificates`*"]
pub const CR_PROP_SCEPSERVERCERTS: u32 = 1000u32;
#[doc = "*Required features: `Win32_Security_Cryptography_Certificates`*"]
pub const CR_PROP_SCEPSERVERCERTSCHAIN: u32 = 1002u32;
#[doc = "*Required features: `Win32_Security_Cryptography_Certificates`*"]
pub const CR_PROP_SHAREDFOLDER: u32 = 8u32;
#[doc = "*Required features: `Win32_Security_Cryptography_Certificates`*"]
pub const CR_PROP_SUBJECTTEMPLATE_OIDS: u32 = 45u32;
#[doc = "*Required features: `Win32_Security_Cryptography_Certificates`*"]
pub const CR_PROP_TEMPLATES: u32 = 29u32;
#[doc = "*Required features: `Win32_Security_Cryptography_Certificates`*"]
pub const CSBACKUP_DISABLE_INCREMENTAL: u32 = 4294967295u32;
pub struct CSBACKUP_TYPE(i32);
#[doc = "*Required features: `Win32_Security_Cryptography_Certificates`*"]
pub const CSBACKUP_TYPE_MASK: u32 = 3u32;
#[doc = "*Required features: `Win32_Security_Cryptography_Certificates`*"]
pub const CSBFT_DATABASE_DIRECTORY: u32 = 64u32;
#[doc = "*Required features: `Win32_Security_Cryptography_Certificates`*"]
pub const CSBFT_DIRECTORY: u32 = 128u32;
#[doc = "*Required features: `Win32_Security_Cryptography_Certificates`*"]
pub const CSBFT_LOG_DIRECTORY: u32 = 32u32;
#[doc = "*Required features: `Win32_Security_Cryptography_Certificates`*"]
pub const CSCONTROL_RESTART: u64 = 3u64;
#[doc = "*Required features: `Win32_Security_Cryptography_Certificates`*"]
pub const CSCONTROL_SHUTDOWN: u64 = 1u64;
#[doc = "*Required features: `Win32_Security_Cryptography_Certificates`*"]
pub const CSCONTROL_SUSPEND: u64 = 2u64;
#[cfg(feature = "Win32_Foundation")]
pub struct CSEDB_RSTMAPW(i32);
#[doc = "*Required features: `Win32_Security_Cryptography_Certificates`*"]
pub const CSRESTORE_TYPE_CATCHUP: u32 = 4u32;
#[doc = "*Required features: `Win32_Security_Cryptography_Certificates`*"]
pub const CSRESTORE_TYPE_FULL: u32 = 1u32;
#[doc = "*Required features: `Win32_Security_Cryptography_Certificates`*"]
pub const CSRESTORE_TYPE_MASK: u32 = 5u32;
#[doc = "*Required features: `Win32_Security_Cryptography_Certificates`*"]
pub const CSRESTORE_TYPE_ONLINE: u32 = 2u32;
#[doc = "*Required features: `Win32_Security_Cryptography_Certificates`*"]
pub const CSURL_ADDTOCERTCDP: u32 = 2u32;
#[doc = "*Required features: `Win32_Security_Cryptography_Certificates`*"]
pub const CSURL_ADDTOCERTOCSP: u32 = 32u32;
#[doc = "*Required features: `Win32_Security_Cryptography_Certificates`*"]
pub const CSURL_ADDTOCRLCDP: u32 = 8u32;
#[doc = "*Required features: `Win32_Security_Cryptography_Certificates`*"]
pub const CSURL_ADDTOFRESHESTCRL: u32 = 4u32;
#[doc = "*Required features: `Win32_Security_Cryptography_Certificates`*"]
pub const CSURL_ADDTOIDP: u32 = 128u32;
#[doc = "*Required features: `Win32_Security_Cryptography_Certificates`*"]
pub const CSURL_PUBLISHRETRY: u32 = 16u32;
#[doc = "*Required features: `Win32_Security_Cryptography_Certificates`*"]
pub const CSURL_SERVERPUBLISH: u32 = 1u32;
#[doc = "*Required features: `Win32_Security_Cryptography_Certificates`*"]
pub const CSURL_SERVERPUBLISHDELTA: u32 = 64u32;
#[doc = "*Required features: `Win32_Security_Cryptography_Certificates`*"]
pub const CSVER_MAJOR: u32 = 7u32;
#[doc = "*Required features: `Win32_Security_Cryptography_Certificates`*"]
pub const CSVER_MAJOR_LONGHORN: u32 = 3u32;
#[doc = "*Required features: `Win32_Security_Cryptography_Certificates`*"]
pub const CSVER_MAJOR_THRESHOLD: u32 = 7u32;
#[doc = "*Required features: `Win32_Security_Cryptography_Certificates`*"]
pub const CSVER_MAJOR_WHISTLER: u32 = 2u32;
#[doc = "*Required features: `Win32_Security_Cryptography_Certificates`*"]
pub const CSVER_MAJOR_WIN2K: u32 = 1u32;
#[doc = "*Required features: `Win32_Security_Cryptography_Certificates`*"]
pub const CSVER_MAJOR_WIN7: u32 = 4u32;
#[doc = "*Required features: `Win32_Security_Cryptography_Certificates`*"]
pub const CSVER_MAJOR_WIN8: u32 = 5u32;
#[doc = "*Required features: `Win32_Security_Cryptography_Certificates`*"]
pub const CSVER_MAJOR_WINBLUE: u32 = 6u32;
#[doc = "*Required features: `Win32_Security_Cryptography_Certificates`*"]
pub const CSVER_MINOR: u32 = 1u32;
#[doc = "*Required features: `Win32_Security_Cryptography_Certificates`*"]
pub const CSVER_MINOR_LONGHORN_BETA1: u32 = 1u32;
#[doc = "*Required features: `Win32_Security_Cryptography_Certificates`*"]
pub const CSVER_MINOR_THRESHOLD: u32 = 1u32;
#[doc = "*Required features: `Win32_Security_Cryptography_Certificates`*"]
pub const CSVER_MINOR_WHISTLER_BETA2: u32 = 1u32;
#[doc = "*Required features: `Win32_Security_Cryptography_Certificates`*"]
pub const CSVER_MINOR_WHISTLER_BETA3: u32 = 2u32;
#[doc = "*Required features: `Win32_Security_Cryptography_Certificates`*"]
pub const CSVER_MINOR_WIN2K: u32 = 1u32;
#[doc = "*Required features: `Win32_Security_Cryptography_Certificates`*"]
pub const CSVER_MINOR_WIN7: u32 = 1u32;
#[doc = "*Required features: `Win32_Security_Cryptography_Certificates`*"]
pub const CSVER_MINOR_WIN8: u32 = 1u32;
#[doc = "*Required features: `Win32_Security_Cryptography_Certificates`*"]
pub const CSVER_MINOR_WINBLUE: u32 = 1u32;
pub struct CSignerCertificate(i32);
pub struct CSmimeCapabilities(i32);
pub struct CSmimeCapability(i32);
#[doc = "*Required features: `Win32_Security_Cryptography_Certificates`*"]
pub const CVIEWAGEMINUTESDEFAULT: u32 = 16u32;
pub struct CVRC_COLUMN(i32);
pub struct CVRC_TABLE(i32);
#[doc = "*Required features: `Win32_Security_Cryptography_Certificates`*"]
pub const CVRC_TABLE_MASK: u32 = 61440u32;
#[doc = "*Required features: `Win32_Security_Cryptography_Certificates`*"]
pub const CVRC_TABLE_SHIFT: u32 = 12u32;
#[doc = "*Required features: `Win32_Security_Cryptography_Certificates`*"]
pub const CVR_SEEK_MASK: u32 = 255u32;
#[doc = "*Required features: `Win32_Security_Cryptography_Certificates`*"]
pub const CVR_SEEK_NODELTA: u32 = 4096u32;
#[doc = "*Required features: `Win32_Security_Cryptography_Certificates`*"]
pub const CVR_SEEK_NONE: u32 = 0u32;
#[doc = "*Required features: `Win32_Security_Cryptography_Certificates`*"]
pub const CVR_SORT_ASCEND: u32 = 1u32;
#[doc = "*Required features: `Win32_Security_Cryptography_Certificates`*"]
pub const CVR_SORT_DESCEND: u32 = 2u32;
#[doc = "*Required features: `Win32_Security_Cryptography_Certificates`*"]
pub const CVR_SORT_NONE: u32 = 0u32;
#[doc = "*Required features: `Win32_Security_Cryptography_Certificates`*"]
pub const CV_COLUMN_ATTRIBUTE_DEFAULT: i32 = -5i32;
#[doc = "*Required features: `Win32_Security_Cryptography_Certificates`*"]
pub const CV_COLUMN_CRL_DEFAULT: i32 = -6i32;
#[doc = "*Required features: `Win32_Security_Cryptography_Certificates`*"]
pub const CV_COLUMN_EXTENSION_DEFAULT: i32 = -4i32;
#[doc = "*Required features: `Win32_Security_Cryptography_Certificates`*"]
pub const CV_COLUMN_LOG_REVOKED_DEFAULT: i32 = -7i32;
#[doc = "*Required features: `Win32_Security_Cryptography_Certificates`*"]
pub const CV_OUT_ENCODEMASK: u32 = 255u32;
#[doc = "*Required features: `Win32_Security_Cryptography_Certificates`*"]
pub const CV_OUT_HEXRAW: u32 = 12u32;
#[doc = "*Required features: `Win32_Security_Cryptography_Certificates`*"]
pub const CV_OUT_NOCR: u32 = 2147483648u32;
#[doc = "*Required features: `Win32_Security_Cryptography_Certificates`*"]
pub const CV_OUT_NOCRLF: u32 = 1073741824u32;
pub struct CX500DistinguishedName(i32);
pub struct CX509Attribute(i32);
pub struct CX509AttributeArchiveKey(i32);
pub struct CX509AttributeArchiveKeyHash(i32);
pub struct CX509AttributeClientId(i32);
pub struct CX509AttributeCspProvider(i32);
pub struct CX509AttributeExtensions(i32);
pub struct CX509AttributeOSVersion(i32);
pub struct CX509AttributeRenewalCertificate(i32);
pub struct CX509Attributes(i32);
pub struct CX509CertificateRequestCertificate(i32);
pub struct CX509CertificateRequestCmc(i32);
pub struct CX509CertificateRequestPkcs10(i32);
pub struct CX509CertificateRequestPkcs7(i32);
pub struct CX509CertificateRevocationList(i32);
pub struct CX509CertificateRevocationListEntries(i32);
pub struct CX509CertificateRevocationListEntry(i32);
pub struct CX509CertificateTemplateADWritable(i32);
pub struct CX509EndorsementKey(i32);
pub struct CX509Enrollment(i32);
pub struct CX509EnrollmentHelper(i32);
pub struct CX509EnrollmentPolicyActiveDirectory(i32);
pub struct CX509EnrollmentPolicyWebService(i32);
pub struct CX509EnrollmentWebClassFactory(i32);
pub struct CX509Extension(i32);
pub struct CX509ExtensionAlternativeNames(i32);
pub struct CX509ExtensionAuthorityKeyIdentifier(i32);
pub struct CX509ExtensionBasicConstraints(i32);
pub struct CX509ExtensionCertificatePolicies(i32);
pub struct CX509ExtensionEnhancedKeyUsage(i32);
pub struct CX509ExtensionKeyUsage(i32);
pub struct CX509ExtensionMSApplicationPolicies(i32);
pub struct CX509ExtensionSmimeCapabilities(i32);
pub struct CX509ExtensionSubjectKeyIdentifier(i32);
pub struct CX509ExtensionTemplate(i32);
pub struct CX509ExtensionTemplateName(i32);
pub struct CX509Extensions(i32);
pub struct CX509MachineEnrollmentFactory(i32);
pub struct CX509NameValuePair(i32);
pub struct CX509PolicyServerListManager(i32);
pub struct CX509PolicyServerUrl(i32);
pub struct CX509PrivateKey(i32);
pub struct CX509PublicKey(i32);
pub struct CX509SCEPEnrollment(i32);
pub struct CX509SCEPEnrollmentHelper(i32);
pub struct CommitTemplateFlags(i32);
#[doc = "*Required features: `Win32_Security_Cryptography_Certificates`*"]
pub const DBFLAGS_CHECKPOINTDEPTH60MB: u32 = 32u32;
#[doc = "*Required features: `Win32_Security_Cryptography_Certificates`*"]
pub const DBFLAGS_CIRCULARLOGGING: u32 = 4u32;
#[doc = "*Required features: `Win32_Security_Cryptography_Certificates`*"]
pub const DBFLAGS_CREATEIFNEEDED: u32 = 2u32;
#[doc = "*Required features: `Win32_Security_Cryptography_Certificates`*"]
pub const DBFLAGS_DISABLESNAPSHOTBACKUP: u32 = 1024u32;
#[doc = "*Required features: `Win32_Security_Cryptography_Certificates`*"]
pub const DBFLAGS_ENABLEVOLATILEREQUESTS: u32 = 2048u32;
#[doc = "*Required features: `Win32_Security_Cryptography_Certificates`*"]
pub const DBFLAGS_LAZYFLUSH: u32 = 8u32;
#[doc = "*Required features: `Win32_Security_Cryptography_Certificates`*"]
pub const DBFLAGS_LOGBUFFERSHUGE: u32 = 128u32;
#[doc = "*Required features: `Win32_Security_Cryptography_Certificates`*"]
pub const DBFLAGS_LOGBUFFERSLARGE: u32 = 64u32;
#[doc = "*Required features: `Win32_Security_Cryptography_Certificates`*"]
pub const DBFLAGS_LOGFILESIZE16MB: u32 = 256u32;
#[doc = "*Required features: `Win32_Security_Cryptography_Certificates`*"]
pub const DBFLAGS_MAXCACHESIZEX100: u32 = 16u32;
#[doc = "*Required features: `Win32_Security_Cryptography_Certificates`*"]
pub const DBFLAGS_MULTITHREADTRANSACTIONS: u32 = 512u32;
#[doc = "*Required features: `Win32_Security_Cryptography_Certificates`*"]
pub const DBFLAGS_READONLY: u32 = 1u32;
#[doc = "*Required features: `Win32_Security_Cryptography_Certificates`*"]
pub const DBSESSIONCOUNTDEFAULT: u32 = 100u32;
#[doc = "*Required features: `Win32_Security_Cryptography_Certificates`*"]
pub const DB_DISP_ACTIVE: u32 = 8u32;
#[doc = "*Required features: `Win32_Security_Cryptography_Certificates`*"]
pub const DB_DISP_CA_CERT: u32 = 15u32;
#[doc = "*Required features: `Win32_Security_Cryptography_Certificates`*"]
pub const DB_DISP_CA_CERT_CHAIN: u32 = 16u32;
#[doc = "*Required features: `Win32_Security_Cryptography_Certificates`*"]
pub const DB_DISP_DENIED: u32 = 31u32;
#[doc = "*Required features: `Win32_Security_Cryptography_Certificates`*"]
pub const DB_DISP_ERROR: u32 = 30u32;
#[doc = "*Required features: `Win32_Security_Cryptography_Certificates`*"]
pub const DB_DISP_FOREIGN: u32 = 12u32;
#[doc = "*Required features: `Win32_Security_Cryptography_Certificates`*"]
pub const DB_DISP_ISSUED: u32 = 20u32;
#[doc = "*Required features: `Win32_Security_Cryptography_Certificates`*"]
pub const DB_DISP_KRA_CERT: u32 = 17u32;
#[doc = "*Required features: `Win32_Security_Cryptography_Certificates`*"]
pub const DB_DISP_LOG_FAILED_MIN: u32 = 30u32;
#[doc = "*Required features: `Win32_Security_Cryptography_Certificates`*"]
pub const DB_DISP_LOG_MIN: u32 = 20u32;
#[doc = "*Required features: `Win32_Security_Cryptography_Certificates`*"]
pub const DB_DISP_PENDING: u32 = 9u32;
#[doc = "*Required features: `Win32_Security_Cryptography_Certificates`*"]
pub const DB_DISP_QUEUE_MAX: u32 = 9u32;
#[doc = "*Required features: `Win32_Security_Cryptography_Certificates`*"]
pub const DB_DISP_REVOKED: u32 = 21u32;
pub struct DelayRetryAction(i32);
#[doc = "*Required features: `Win32_Security_Cryptography_Certificates`*"]
pub const EANR_SUPPRESS_IA5CONVERSION: u32 = 2147483648u32;
#[doc = "*Required features: `Win32_Security_Cryptography_Certificates`*"]
pub const EAN_NAMEOBJECTID: u32 = 2147483648u32;
#[doc = "*Required features: `Win32_Security_Cryptography_Certificates`*"]
pub const EDITF_ADDOLDCERTTYPE: u32 = 16u32;
#[doc = "*Required features: `Win32_Security_Cryptography_Certificates`*"]
pub const EDITF_ADDOLDKEYUSAGE: u32 = 8u32;
#[doc = "*Required features: `Win32_Security_Cryptography_Certificates`*"]
pub const EDITF_ATTRIBUTECA: u32 = 512u32;
#[doc = "*Required features: `Win32_Security_Cryptography_Certificates`*"]
pub const EDITF_ATTRIBUTEEKU: u32 = 32768u32;
#[doc = "*Required features: `Win32_Security_Cryptography_Certificates`*"]
pub const EDITF_ATTRIBUTEENDDATE: u32 = 32u32;
#[doc = "*Required features: `Win32_Security_Cryptography_Certificates`*"]
pub const EDITF_ATTRIBUTESUBJECTALTNAME2: u32 = 262144u32;
#[doc = "*Required features: `Win32_Security_Cryptography_Certificates`*"]
pub const EDITF_AUDITCERTTEMPLATELOAD: u32 = 2097152u32;
#[doc = "*Required features: `Win32_Security_Cryptography_Certificates`*"]
pub const EDITF_BASICCONSTRAINTSCA: u32 = 128u32;
#[doc = "*Required features: `Win32_Security_Cryptography_Certificates`*"]
pub const EDITF_BASICCONSTRAINTSCRITICAL: u32 = 64u32;
#[doc = "*Required features: `Win32_Security_Cryptography_Certificates`*"]
pub const EDITF_DISABLEEXTENSIONLIST: u32 = 4u32;
#[doc = "*Required features: `Win32_Security_Cryptography_Certificates`*"]
pub const EDITF_DISABLELDAPPACKAGELIST: u32 = 8388608u32;
#[doc = "*Required features: `Win32_Security_Cryptography_Certificates`*"]
pub const EDITF_DISABLEOLDOSCNUPN: u32 = 4194304u32;
#[doc = "*Required features: `Win32_Security_Cryptography_Certificates`*"]
pub const EDITF_EMAILOPTIONAL: u32 = 131072u32;
#[doc = "*Required features: `Win32_Security_Cryptography_Certificates`*"]
pub const EDITF_ENABLEAKICRITICAL: u32 = 8192u32;
#[doc = "*Required features: `Win32_Security_Cryptography_Certificates`*"]
pub const EDITF_ENABLEAKIISSUERNAME: u32 = 2048u32;
#[doc = "*Required features: `Win32_Security_Cryptography_Certificates`*"]
pub const EDITF_ENABLEAKIISSUERSERIAL: u32 = 4096u32;
#[doc = "*Required features: `Win32_Security_Cryptography_Certificates`*"]
pub const EDITF_ENABLEAKIKEYID: u32 = 256u32;
#[doc = "*Required features: `Win32_Security_Cryptography_Certificates`*"]
pub const EDITF_ENABLECHASECLIENTDC: u32 = 1048576u32;
#[doc = "*Required features: `Win32_Security_Cryptography_Certificates`*"]
pub const EDITF_ENABLEDEFAULTSMIME: u32 = 65536u32;
#[doc = "*Required features: `Win32_Security_Cryptography_Certificates`*"]
pub const EDITF_ENABLEKEYENCIPHERMENTCACERT: u32 = 134217728u32;
#[doc = "*Required features: `Win32_Security_Cryptography_Certificates`*"]
pub const EDITF_ENABLELDAPREFERRALS: u32 = 524288u32;
#[doc = "*Required features: `Win32_Security_Cryptography_Certificates`*"]
pub const EDITF_ENABLEOCSPREVNOCHECK: u32 = 33554432u32;
#[doc = "*Required features: `Win32_Security_Cryptography_Certificates`*"]
pub const EDITF_ENABLERENEWONBEHALFOF: u32 = 67108864u32;
#[doc = "*Required features: `Win32_Security_Cryptography_Certificates`*"]
pub const EDITF_ENABLEREQUESTEXTENSIONS: u32 = 1u32;
#[doc = "*Required features: `Win32_Security_Cryptography_Certificates`*"]
pub const EDITF_ENABLEUPNMAP: u32 = 16777216u32;
#[doc = "*Required features: `Win32_Security_Cryptography_Certificates`*"]
pub const EDITF_IGNOREREQUESTERGROUP: u32 = 1024u32;
#[doc = "*Required features: `Win32_Security_Cryptography_Certificates`*"]
pub const EDITF_REQUESTEXTENSIONLIST: u32 = 2u32;
#[doc = "*Required features: `Win32_Security_Cryptography_Certificates`*"]
pub const EDITF_SERVERUPGRADED: u32 = 16384u32;
#[doc = "*Required features: `Win32_Security_Cryptography_Certificates`*"]
pub const ENUMEXT_OBJECTID: u32 = 1u32;
pub struct ENUM_CATYPES(i32);
pub struct ENUM_CERT_COLUMN_VALUE_FLAGS(i32);
#[doc = "*Required features: `Win32_Security_Cryptography_Certificates`*"]
pub const EXITEVENT_CERTIMPORTED: u32 = 512u32;
#[doc = "*Required features: `Win32_Security_Cryptography_Certificates`*"]
pub const EXITEVENT_INVALID: u32 = 0u32;
#[doc = "*Required features: `Win32_Security_Cryptography_Certificates`*"]
pub const EXITEVENT_STARTUP: u32 = 128u32;
#[doc = "*Required features: `Win32_Security_Cryptography_Certificates`*"]
pub const EXITPUB_ACTIVEDIRECTORY: u32 = 2u32;
#[doc = "*Required features: `Win32_Security_Cryptography_Certificates`*"]
pub const EXITPUB_DEFAULT_ENTERPRISE: u32 = 2u32;
#[doc = "*Required features: `Win32_Security_Cryptography_Certificates`*"]
pub const EXITPUB_DEFAULT_STANDALONE: u32 = 1u32;
#[doc = "*Required features: `Win32_Security_Cryptography_Certificates`*"]
pub const EXITPUB_FILE: u32 = 1u32;
#[doc = "*Required features: `Win32_Security_Cryptography_Certificates`*"]
pub const EXITPUB_REMOVEOLDCERTS: u32 = 16u32;
#[doc = "*Required features: `Win32_Security_Cryptography_Certificates`*"]
pub const EXTENSION_CRITICAL_FLAG: u32 = 1u32;
#[doc = "*Required features: `Win32_Security_Cryptography_Certificates`*"]
pub const EXTENSION_DELETE_FLAG: u32 = 4u32;
#[doc = "*Required features: `Win32_Security_Cryptography_Certificates`*"]
pub const EXTENSION_DISABLE_FLAG: u32 = 2u32;
#[doc = "*Required features: `Win32_Security_Cryptography_Certificates`*"]
pub const EXTENSION_ORIGIN_ADMIN: u32 = 196608u32;
#[doc = "*Required features: `Win32_Security_Cryptography_Certificates`*"]
pub const EXTENSION_ORIGIN_CACERT: u32 = 589824u32;
#[doc = "*Required features: `Win32_Security_Cryptography_Certificates`*"]
pub const EXTENSION_ORIGIN_CMC: u32 = 524288u32;
#[doc = "*Required features: `Win32_Security_Cryptography_Certificates`*"]
pub const EXTENSION_ORIGIN_IMPORTEDCERT: u32 = 393216u32;
#[doc = "*Required features: `Win32_Security_Cryptography_Certificates`*"]
pub const EXTENSION_ORIGIN_MASK: u32 = 983040u32;
#[doc = "*Required features: `Win32_Security_Cryptography_Certificates`*"]
pub const EXTENSION_ORIGIN_PKCS7: u32 = 458752u32;
#[doc = "*Required features: `Win32_Security_Cryptography_Certificates`*"]
pub const EXTENSION_ORIGIN_POLICY: u32 = 131072u32;
#[doc = "*Required features: `Win32_Security_Cryptography_Certificates`*"]
pub const EXTENSION_ORIGIN_RENEWALCERT: u32 = 327680u32;
#[doc = "*Required features: `Win32_Security_Cryptography_Certificates`*"]
pub const EXTENSION_ORIGIN_REQUEST: u32 = 65536u32;
#[doc = "*Required features: `Win32_Security_Cryptography_Certificates`*"]
pub const EXTENSION_ORIGIN_SERVER: u32 = 262144u32;
#[doc = "*Required features: `Win32_Security_Cryptography_Certificates`*"]
pub const EXTENSION_POLICY_MASK: u32 = 65535u32;
pub struct EncodingType(i32);
pub struct EnrollmentCAProperty(i32);
pub struct EnrollmentDisplayStatus(i32);
pub struct EnrollmentEnrollStatus(i32);
pub struct EnrollmentPolicyFlags(i32);
pub struct EnrollmentPolicyServerPropertyFlags(i32);
pub struct EnrollmentSelectionStatus(i32);
pub struct EnrollmentTemplateProperty(i32);
pub struct FNCERTSRVBACKUPCLOSE(i32);
pub struct FNCERTSRVBACKUPEND(i32);
pub struct FNCERTSRVBACKUPFREE(i32);
pub struct FNCERTSRVBACKUPGETBACKUPLOGSW(i32);
pub struct FNCERTSRVBACKUPGETDATABASENAMESW(i32);
pub struct FNCERTSRVBACKUPGETDYNAMICFILELISTW(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct FNCERTSRVBACKUPOPENFILEW(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct FNCERTSRVBACKUPPREPAREW(i32);
pub struct FNCERTSRVBACKUPREAD(i32);
pub struct FNCERTSRVBACKUPTRUNCATELOGS(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct FNCERTSRVISSERVERONLINEW(i32);
pub struct FNCERTSRVRESTOREEND(i32);
pub struct FNCERTSRVRESTOREGETDATABASELOCATIONSW(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct FNCERTSRVRESTOREPREPAREW(i32);
pub struct FNCERTSRVRESTOREREGISTERCOMPLETE(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct FNCERTSRVRESTOREREGISTERW(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct FNCERTSRVSERVERCONTROLW(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct FNIMPORTPFXTOPROVIDER(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct FNIMPORTPFXTOPROVIDERFREEDATA(i32);
#[doc = "*Required features: `Win32_Security_Cryptography_Certificates`*"]
pub const FR_PROP_CLAIMCHALLENGE: u32 = 22u32;
pub struct FULL_RESPONSE_PROPERTY_ID(i32);
pub struct IAlternativeName(i32);
pub struct IAlternativeNames(i32);
pub struct IBinaryConverter(i32);
pub struct IBinaryConverter2(i32);
pub struct ICEnroll(i32);
pub struct ICEnroll2(i32);
pub struct ICEnroll3(i32);
pub struct ICEnroll4(i32);
#[doc = "*Required features: `Win32_Security_Cryptography_Certificates`*"]
pub const ICF_ALLOWFOREIGN: u32 = 65536u32;
#[doc = "*Required features: `Win32_Security_Cryptography_Certificates`*"]
pub const ICF_EXISTINGROW: u32 = 131072u32;
pub struct ICertAdmin(i32);
pub struct ICertAdmin2(i32);
pub struct ICertConfig(i32);
pub struct ICertConfig2(i32);
pub struct ICertEncodeAltName(i32);
pub struct ICertEncodeAltName2(i32);
pub struct ICertEncodeBitString(i32);
pub struct ICertEncodeBitString2(i32);
pub struct ICertEncodeCRLDistInfo(i32);
pub struct ICertEncodeCRLDistInfo2(i32);
pub struct ICertEncodeDateArray(i32);
pub struct ICertEncodeDateArray2(i32);
pub struct ICertEncodeLongArray(i32);
pub struct ICertEncodeLongArray2(i32);
pub struct ICertEncodeStringArray(i32);
pub struct ICertEncodeStringArray2(i32);
pub struct ICertExit(i32);
pub struct ICertExit2(i32);
pub struct ICertGetConfig(i32);
pub struct ICertManageModule(i32);
pub struct ICertPolicy(i32);
pub struct ICertPolicy2(i32);
pub struct ICertProperties(i32);
pub struct ICertProperty(i32);
pub struct ICertPropertyArchived(i32);
pub struct ICertPropertyArchivedKeyHash(i32);
pub struct ICertPropertyAutoEnroll(i32);
pub struct ICertPropertyBackedUp(i32);
pub struct ICertPropertyDescription(i32);
pub struct ICertPropertyEnrollment(i32);
pub struct ICertPropertyEnrollmentPolicyServer(i32);
pub struct ICertPropertyFriendlyName(i32);
pub struct ICertPropertyKeyProvInfo(i32);
pub struct ICertPropertyRenewal(i32);
pub struct ICertPropertyRequestOriginator(i32);
pub struct ICertPropertySHA1Hash(i32);
pub struct ICertRequest(i32);
pub struct ICertRequest2(i32);
pub struct ICertRequest3(i32);
pub struct ICertRequestD(i32);
pub struct ICertRequestD2(i32);
pub struct ICertServerExit(i32);
pub struct ICertServerPolicy(i32);
pub struct ICertView(i32);
pub struct ICertView2(i32);
pub struct ICertificateAttestationChallenge(i32);
pub struct ICertificateAttestationChallenge2(i32);
pub struct ICertificatePolicies(i32);
pub struct ICertificatePolicy(i32);
pub struct ICertificationAuthorities(i32);
pub struct ICertificationAuthority(i32);
pub struct ICryptAttribute(i32);
pub struct ICryptAttributes(i32);
pub struct ICspAlgorithm(i32);
pub struct ICspAlgorithms(i32);
pub struct ICspInformation(i32);
pub struct ICspInformations(i32);
pub struct ICspStatus(i32);
pub struct ICspStatuses(i32);
pub struct IEnroll(i32);
pub struct IEnroll2(i32);
pub struct IEnroll4(i32);
pub struct IEnumCERTVIEWATTRIBUTE(i32);
pub struct IEnumCERTVIEWCOLUMN(i32);
pub struct IEnumCERTVIEWEXTENSION(i32);
pub struct IEnumCERTVIEWROW(i32);
#[doc = "*Required features: `Win32_Security_Cryptography_Certificates`*"]
pub const IF_ENABLEADMINASAUDITOR: u32 = 4096u32;
#[doc = "*Required features: `Win32_Security_Cryptography_Certificates`*"]
pub const IF_ENABLEEXITKEYRETRIEVAL: u32 = 2048u32;
#[doc = "*Required features: `Win32_Security_Cryptography_Certificates`*"]
pub const IF_ENFORCEENCRYPTICERTADMIN: u32 = 1024u32;
#[doc = "*Required features: `Win32_Security_Cryptography_Certificates`*"]
pub const IF_ENFORCEENCRYPTICERTREQUEST: u32 = 512u32;
#[doc = "*Required features: `Win32_Security_Cryptography_Certificates`*"]
pub const IF_LOCKICERTREQUEST: u32 = 1u32;
#[doc = "*Required features: `Win32_Security_Cryptography_Certificates`*"]
pub const IF_NOLOCALICERTADMIN: u32 = 32u32;
#[doc = "*Required features: `Win32_Security_Cryptography_Certificates`*"]
pub const IF_NOLOCALICERTADMINBACKUP: u32 = 128u32;
#[doc = "*Required features: `Win32_Security_Cryptography_Certificates`*"]
pub const IF_NOLOCALICERTREQUEST: u32 = 4u32;
#[doc = "*Required features: `Win32_Security_Cryptography_Certificates`*"]
pub const IF_NOREMOTEICERTADMIN: u32 = 16u32;
#[doc = "*Required features: `Win32_Security_Cryptography_Certificates`*"]
pub const IF_NOREMOTEICERTADMINBACKUP: u32 = 64u32;
#[doc = "*Required features: `Win32_Security_Cryptography_Certificates`*"]
pub const IF_NOREMOTEICERTREQUEST: u32 = 2u32;
#[doc = "*Required features: `Win32_Security_Cryptography_Certificates`*"]
pub const IF_NORPCICERTREQUEST: u32 = 8u32;
#[doc = "*Required features: `Win32_Security_Cryptography_Certificates`*"]
pub const IF_NOSNAPSHOTBACKUP: u32 = 256u32;
#[doc = "*Required features: `Win32_Security_Cryptography_Certificates`*"]
pub const IKF_OVERWRITE: u32 = 65536u32;
pub struct INDESPolicy(i32);
pub struct IOCSPAdmin(i32);
pub struct IOCSPCAConfiguration(i32);
pub struct IOCSPCAConfigurationCollection(i32);
pub struct IOCSPProperty(i32);
pub struct IOCSPPropertyCollection(i32);
pub struct IObjectId(i32);
pub struct IObjectIds(i32);
pub struct IPolicyQualifier(i32);
pub struct IPolicyQualifiers(i32);
#[doc = "*Required features: `Win32_Security_Cryptography_Certificates`*"]
pub const ISSCERT_DEFAULT_DS: u32 = 256u32;
#[doc = "*Required features: `Win32_Security_Cryptography_Certificates`*"]
pub const ISSCERT_DEFAULT_NODS: u32 = 256u32;
#[doc = "*Required features: `Win32_Security_Cryptography_Certificates`*"]
pub const ISSCERT_ENABLE: u32 = 256u32;
#[doc = "*Required features: `Win32_Security_Cryptography_Certificates`*"]
pub const ISSCERT_FILEURL_OLD: u32 = 8u32;
#[doc = "*Required features: `Win32_Security_Cryptography_Certificates`*"]
pub const ISSCERT_FTPURL_OLD: u32 = 4u32;
#[doc = "*Required features: `Win32_Security_Cryptography_Certificates`*"]
pub const ISSCERT_HTTPURL_OLD: u32 = 2u32;
#[doc = "*Required features: `Win32_Security_Cryptography_Certificates`*"]
pub const ISSCERT_LDAPURL_OLD: u32 = 1u32;
#[doc = "*Required features: `Win32_Security_Cryptography_Certificates`*"]
pub const ISSCERT_URLMASK_OLD: u32 = 255u32;
pub struct ISignerCertificate(i32);
pub struct ISignerCertificates(i32);
pub struct ISmimeCapabilities(i32);
pub struct ISmimeCapability(i32);
pub struct IX500DistinguishedName(i32);
pub struct IX509Attribute(i32);
pub struct IX509AttributeArchiveKey(i32);
pub struct IX509AttributeArchiveKeyHash(i32);
pub struct IX509AttributeClientId(i32);
pub struct IX509AttributeCspProvider(i32);
pub struct IX509AttributeExtensions(i32);
pub struct IX509AttributeOSVersion(i32);
pub struct IX509AttributeRenewalCertificate(i32);
pub struct IX509Attributes(i32);
pub struct IX509CertificateRequest(i32);
pub struct IX509CertificateRequestCertificate(i32);
pub struct IX509CertificateRequestCertificate2(i32);
pub struct IX509CertificateRequestCmc(i32);
pub struct IX509CertificateRequestCmc2(i32);
pub struct IX509CertificateRequestPkcs10(i32);
pub struct IX509CertificateRequestPkcs10V2(i32);
pub struct IX509CertificateRequestPkcs10V3(i32);
pub struct IX509CertificateRequestPkcs10V4(i32);
pub struct IX509CertificateRequestPkcs7(i32);
pub struct IX509CertificateRequestPkcs7V2(i32);
pub struct IX509CertificateRevocationList(i32);
pub struct IX509CertificateRevocationListEntries(i32);
pub struct IX509CertificateRevocationListEntry(i32);
pub struct IX509CertificateTemplate(i32);
pub struct IX509CertificateTemplateWritable(i32);
pub struct IX509CertificateTemplates(i32);
pub struct IX509EndorsementKey(i32);
pub struct IX509Enrollment(i32);
pub struct IX509Enrollment2(i32);
pub struct IX509EnrollmentHelper(i32);
pub struct IX509EnrollmentPolicyServer(i32);
pub struct IX509EnrollmentStatus(i32);
pub struct IX509EnrollmentWebClassFactory(i32);
pub struct IX509Extension(i32);
pub struct IX509ExtensionAlternativeNames(i32);
pub struct IX509ExtensionAuthorityKeyIdentifier(i32);
pub struct IX509ExtensionBasicConstraints(i32);
pub struct IX509ExtensionCertificatePolicies(i32);
pub struct IX509ExtensionEnhancedKeyUsage(i32);
pub struct IX509ExtensionKeyUsage(i32);
pub struct IX509ExtensionMSApplicationPolicies(i32);
pub struct IX509ExtensionSmimeCapabilities(i32);
pub struct IX509ExtensionSubjectKeyIdentifier(i32);
pub struct IX509ExtensionTemplate(i32);
pub struct IX509ExtensionTemplateName(i32);
pub struct IX509Extensions(i32);
pub struct IX509MachineEnrollmentFactory(i32);
pub struct IX509NameValuePair(i32);
pub struct IX509NameValuePairs(i32);
pub struct IX509PolicyServerListManager(i32);
pub struct IX509PolicyServerUrl(i32);
pub struct IX509PrivateKey(i32);
pub struct IX509PrivateKey2(i32);
pub struct IX509PublicKey(i32);
pub struct IX509SCEPEnrollment(i32);
pub struct IX509SCEPEnrollment2(i32);
pub struct IX509SCEPEnrollmentHelper(i32);
pub struct IX509SignatureInformation(i32);
pub struct ImportPFXFlags(i32);
pub struct InnerRequestLevel(i32);
pub struct InstallResponseRestrictionFlags(i32);
#[doc = "*Required features: `Win32_Security_Cryptography_Certificates`*"]
pub const KRAF_DISABLEUSEDEFAULTPROVIDER: u32 = 8u32;
#[doc = "*Required features: `Win32_Security_Cryptography_Certificates`*"]
pub const KRAF_ENABLEARCHIVEALL: u32 = 4u32;
#[doc = "*Required features: `Win32_Security_Cryptography_Certificates`*"]
pub const KRAF_ENABLEFOREIGN: u32 = 1u32;
#[doc = "*Required features: `Win32_Security_Cryptography_Certificates`*"]
pub const KRAF_SAVEBADREQUESTKEY: u32 = 2u32;
#[doc = "*Required features: `Win32_Security_Cryptography_Certificates`*"]
pub const KRA_DISP_EXPIRED: u32 = 0u32;
#[doc = "*Required features: `Win32_Security_Cryptography_Certificates`*"]
pub const KRA_DISP_INVALID: u32 = 4u32;
#[doc = "*Required features: `Win32_Security_Cryptography_Certificates`*"]
pub const KRA_DISP_NOTFOUND: u32 = 1u32;
#[doc = "*Required features: `Win32_Security_Cryptography_Certificates`*"]
pub const KRA_DISP_NOTLOADED: u32 = 6u32;
#[doc = "*Required features: `Win32_Security_Cryptography_Certificates`*"]
pub const KRA_DISP_REVOKED: u32 = 2u32;
#[doc = "*Required features: `Win32_Security_Cryptography_Certificates`*"]
pub const KRA_DISP_UNTRUSTED: u32 = 5u32;
#[doc = "*Required features: `Win32_Security_Cryptography_Certificates`*"]
pub const KRA_DISP_VALID: u32 = 3u32;
#[doc = "*Required features: `Win32_Security_Cryptography_Certificates`*"]
pub const KR_ENABLE_MACHINE: u32 = 1u32;
#[doc = "*Required features: `Win32_Security_Cryptography_Certificates`*"]
pub const KR_ENABLE_USER: u32 = 2u32;
pub struct KeyAttestationClaimType(i32);
pub struct KeyIdentifierHashAlgorithm(i32);
#[doc = "*Required features: `Win32_Security_Cryptography_Certificates`*"]
pub const LDAPF_SIGNDISABLE: u32 = 2u32;
#[doc = "*Required features: `Win32_Security_Cryptography_Certificates`*"]
pub const LDAPF_SSLENABLE: u32 = 1u32;
pub struct OCSPAdmin(i32);
pub struct OCSPPropertyCollection(i32);
pub struct OCSPRequestFlag(i32);
pub struct OCSPSigningFlag(i32);
pub struct ObjectIdGroupId(i32);
pub struct ObjectIdPublicKeyFlags(i32);
pub struct PENDING_REQUEST_DESIRED_PROPERTY(i32);
pub struct PFXExportOptions(i32);
#[doc = "*Required features: `Win32_Security_Cryptography_Certificates`*"]
pub const PROCFLG_ENFORCEGOODKEYS: u32 = 1u32;
#[doc = "*Required features: `Win32_Security_Cryptography_Certificates`*"]
pub const PROCFLG_NONE: u32 = 0u32;
#[doc = "*Required features: `Win32_Security_Cryptography_Certificates`*"]
pub const PROPCALLER_ADMIN: u32 = 1024u32;
#[doc = "*Required features: `Win32_Security_Cryptography_Certificates`*"]
pub const PROPCALLER_EXIT: u32 = 768u32;
#[doc = "*Required features: `Win32_Security_Cryptography_Certificates`*"]
pub const PROPCALLER_MASK: u32 = 3840u32;
#[doc = "*Required features: `Win32_Security_Cryptography_Certificates`*"]
pub const PROPCALLER_POLICY: u32 = 512u32;
#[doc = "*Required features: `Win32_Security_Cryptography_Certificates`*"]
pub const PROPCALLER_REQUEST: u32 = 1280u32;
#[doc = "*Required features: `Win32_Security_Cryptography_Certificates`*"]
pub const PROPCALLER_SERVER: u32 = 256u32;
#[doc = "*Required features: `Win32_Security_Cryptography_Certificates`*"]
pub const PROPFLAGS_INDEXED: u32 = 65536u32;
#[doc = "*Required features: `Win32_Security_Cryptography_Certificates`*"]
pub const PROPTYPE_MASK: u32 = 255u32;
pub struct Pkcs10AllowedSignatureTypes(i32);
pub struct PolicyQualifierType(i32);
pub struct PolicyServerUrlFlags(i32);
pub struct PolicyServerUrlPropertyID(i32);
#[doc = "*Required features: `Win32_Security_Cryptography_Certificates`*"]
pub const REQDISP_DEFAULT_ENTERPRISE: u32 = 1u32;
#[doc = "*Required features: `Win32_Security_Cryptography_Certificates`*"]
pub const REQDISP_DENY: u32 = 2u32;
#[doc = "*Required features: `Win32_Security_Cryptography_Certificates`*"]
pub const REQDISP_ISSUE: u32 = 1u32;
#[doc = "*Required features: `Win32_Security_Cryptography_Certificates`*"]
pub const REQDISP_MASK: u32 = 255u32;
#[doc = "*Required features: `Win32_Security_Cryptography_Certificates`*"]
pub const REQDISP_PENDING: u32 = 0u32;
#[doc = "*Required features: `Win32_Security_Cryptography_Certificates`*"]
pub const REQDISP_PENDINGFIRST: u32 = 256u32;
#[doc = "*Required features: `Win32_Security_Cryptography_Certificates`*"]
pub const REQDISP_USEREQUESTATTRIBUTE: u32 = 3u32;
#[doc = "*Required features: `Win32_Security_Cryptography_Certificates`*"]
pub const REVEXT_ASPENABLE: u32 = 512u32;
#[doc = "*Required features: `Win32_Security_Cryptography_Certificates`*"]
pub const REVEXT_CDPENABLE: u32 = 256u32;
#[doc = "*Required features: `Win32_Security_Cryptography_Certificates`*"]
pub const REVEXT_CDPFILEURL_OLD: u32 = 8u32;
#[doc = "*Required features: `Win32_Security_Cryptography_Certificates`*"]
pub const REVEXT_CDPFTPURL_OLD: u32 = 4u32;
#[doc = "*Required features: `Win32_Security_Cryptography_Certificates`*"]
pub const REVEXT_CDPHTTPURL_OLD: u32 = 2u32;
#[doc = "*Required features: `Win32_Security_Cryptography_Certificates`*"]
pub const REVEXT_CDPLDAPURL_OLD: u32 = 1u32;
#[doc = "*Required features: `Win32_Security_Cryptography_Certificates`*"]
pub const REVEXT_CDPURLMASK_OLD: u32 = 255u32;
#[doc = "*Required features: `Win32_Security_Cryptography_Certificates`*"]
pub const REVEXT_DEFAULT_DS: u32 = 256u32;
#[doc = "*Required features: `Win32_Security_Cryptography_Certificates`*"]
pub const REVEXT_DEFAULT_NODS: u32 = 256u32;
pub struct RequestClientInfoClientId(i32);
#[doc = "*Required features: `Win32_Security_Cryptography_Certificates`*"]
pub const SETUP_ATTEMPT_VROOT_CREATE: u32 = 128u32;
#[doc = "*Required features: `Win32_Security_Cryptography_Certificates`*"]
pub const SETUP_CLIENT_FLAG: u32 = 2u32;
#[doc = "*Required features: `Win32_Security_Cryptography_Certificates`*"]
pub const SETUP_CREATEDB_FLAG: u32 = 64u32;
#[doc = "*Required features: `Win32_Security_Cryptography_Certificates`*"]
pub const SETUP_DCOM_SECURITY_UPDATED_FLAG: u32 = 8192u32;
#[doc = "*Required features: `Win32_Security_Cryptography_Certificates`*"]
pub const SETUP_DENIED_FLAG: u32 = 32u32;
#[doc = "*Required features: `Win32_Security_Cryptography_Certificates`*"]
pub const SETUP_FORCECRL_FLAG: u32 = 256u32;
#[doc = "*Required features: `Win32_Security_Cryptography_Certificates`*"]
pub const SETUP_ONLINE_FLAG: u32 = 16u32;
#[doc = "*Required features: `Win32_Security_Cryptography_Certificates`*"]
pub const SETUP_REQUEST_FLAG: u32 = 8u32;
#[doc = "*Required features: `Win32_Security_Cryptography_Certificates`*"]
pub const SETUP_SECURITY_CHANGED: u32 = 4096u32;
#[doc = "*Required features: `Win32_Security_Cryptography_Certificates`*"]
pub const SETUP_SERVER_FLAG: u32 = 1u32;
#[doc = "*Required features: `Win32_Security_Cryptography_Certificates`*"]
pub const SETUP_SERVER_IS_UP_TO_DATE_FLAG: u32 = 16384u32;
#[doc = "*Required features: `Win32_Security_Cryptography_Certificates`*"]
pub const SETUP_SERVER_UPGRADED_FLAG: u32 = 1024u32;
#[doc = "*Required features: `Win32_Security_Cryptography_Certificates`*"]
pub const SETUP_SUSPEND_FLAG: u32 = 4u32;
#[doc = "*Required features: `Win32_Security_Cryptography_Certificates`*"]
pub const SETUP_UPDATE_CAOBJECT_SVRTYPE: u32 = 512u32;
#[doc = "*Required features: `Win32_Security_Cryptography_Certificates`*"]
pub const SETUP_W2K_SECURITY_NOT_UPGRADED_FLAG: u32 = 2048u32;
#[doc = "*Required features: `Win32_Security_Cryptography_Certificates`*"]
pub const TP_MACHINEPOLICY: u32 = 1u32;
#[doc = "*Required features: `Win32_Security_Cryptography_Certificates`*"]
pub const VR_INSTANT_BAD: u32 = 2u32;
#[doc = "*Required features: `Win32_Security_Cryptography_Certificates`*"]
pub const VR_INSTANT_OK: u32 = 1u32;
#[doc = "*Required features: `Win32_Security_Cryptography_Certificates`*"]
pub const VR_PENDING: u32 = 0u32;
pub struct WebEnrollmentFlags(i32);
pub struct WebSecurityLevel(i32);
pub struct X500NameFlags(i32);
pub struct X509CertificateEnrollmentContext(i32);
pub struct X509CertificateTemplateEnrollmentFlag(i32);
pub struct X509CertificateTemplateGeneralFlag(i32);
pub struct X509CertificateTemplatePrivateKeyFlag(i32);
pub struct X509CertificateTemplateSubjectNameFlag(i32);
pub struct X509EnrollmentAuthFlags(i32);
pub struct X509EnrollmentPolicyExportFlags(i32);
pub struct X509EnrollmentPolicyLoadOption(i32);
pub struct X509HardwareKeyUsageFlags(i32);
pub struct X509KeyParametersExportType(i32);
pub struct X509KeySpec(i32);
pub struct X509KeyUsageFlags(i32);
pub struct X509PrivateKeyExportFlags(i32);
pub struct X509PrivateKeyProtection(i32);
pub struct X509PrivateKeyUsageFlags(i32);
pub struct X509PrivateKeyVerify(i32);
pub struct X509ProviderType(i32);
pub struct X509RequestInheritOptions(i32);
pub struct X509RequestType(i32);
pub struct X509SCEPDisposition(i32);
pub struct X509SCEPFailInfo(i32);
pub struct X509SCEPMessageType(i32);
pub struct X509SCEPProcessMessageFlags(i32);
#[doc = "*Required features: `Win32_Security_Cryptography_Certificates`*"]
pub const XECI_AUTOENROLL: u32 = 2u32;
#[doc = "*Required features: `Win32_Security_Cryptography_Certificates`*"]
pub const XECI_CERTREQ: u32 = 4u32;
#[doc = "*Required features: `Win32_Security_Cryptography_Certificates`*"]
pub const XECI_DISABLE: u32 = 0u32;
#[doc = "*Required features: `Win32_Security_Cryptography_Certificates`*"]
pub const XECI_REQWIZARD: u32 = 3u32;
#[doc = "*Required features: `Win32_Security_Cryptography_Certificates`*"]
pub const XECI_XENROLL: u32 = 1u32;
#[doc = "*Required features: `Win32_Security_Cryptography_Certificates`*"]
pub const XECP_STRING_PROPERTY: u32 = 1u32;
pub struct XEKL_KEYSIZE(i32);
#[doc = "*Required features: `Win32_Security_Cryptography_Certificates`*"]
pub const XEKL_KEYSIZE_DEFAULT: u32 = 4u32;
pub struct XEKL_KEYSPEC(i32);
#[doc = "*Required features: `Win32_Security_Cryptography_Certificates`*"]
pub const XEPR_DATE: u32 = 5u32;
#[doc = "*Required features: `Win32_Security_Cryptography_Certificates`*"]
pub const XEPR_ENUM_FIRST: i32 = -1i32;
#[doc = "*Required features: `Win32_Security_Cryptography_Certificates`*"]
pub const XEPR_TEMPLATENAME: u32 = 6u32;
#[doc = "*Required features: `Win32_Security_Cryptography_Certificates`*"]
pub const XEPR_V1TEMPLATENAME: u32 = 9u32;
#[doc = "*Required features: `Win32_Security_Cryptography_Certificates`*"]
pub const XEPR_V2TEMPLATEOID: u32 = 16u32;
#[doc = "*Required features: `Win32_Security_Cryptography_Certificates`*"]
pub const XEPR_VERSION: u32 = 7u32;
#[doc = "*Required features: `Win32_Security_Cryptography_Certificates`*"]
pub const wszCMM_PROP_COPYRIGHT: &'static str = "Copyright";
#[doc = "*Required features: `Win32_Security_Cryptography_Certificates`*"]
pub const wszCMM_PROP_DESCRIPTION: &'static str = "Description";
#[doc = "*Required features: `Win32_Security_Cryptography_Certificates`*"]
pub const wszCMM_PROP_DISPLAY_HWND: &'static str = "HWND";
#[doc = "*Required features: `Win32_Security_Cryptography_Certificates`*"]
pub const wszCMM_PROP_FILEVER: &'static str = "File Version";
#[doc = "*Required features: `Win32_Security_Cryptography_Certificates`*"]
pub const wszCMM_PROP_ISMULTITHREADED: &'static str = "IsMultiThreaded";
#[doc = "*Required features: `Win32_Security_Cryptography_Certificates`*"]
pub const wszCMM_PROP_NAME: &'static str = "Name";
#[doc = "*Required features: `Win32_Security_Cryptography_Certificates`*"]
pub const wszCMM_PROP_PRODUCTVER: &'static str = "Product Version";
