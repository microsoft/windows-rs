#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[link(name = "windows")]
extern "system" {
    pub fn CertSrvBackupClose(hbc: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
    pub fn CertSrvBackupEnd(hbc: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
    pub fn CertSrvBackupFree(pv: *mut ::core::ffi::c_void);
    #[cfg(feature = "Win32_Foundation")]
    pub fn CertSrvBackupGetBackupLogsW(hbc: *const ::core::ffi::c_void, ppwszzbackuplogfiles: *mut super::super::super::Foundation::PWSTR, pcbsize: *mut u32) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn CertSrvBackupGetDatabaseNamesW(hbc: *const ::core::ffi::c_void, ppwszzattachmentinformation: *mut super::super::super::Foundation::PWSTR, pcbsize: *mut u32) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn CertSrvBackupGetDynamicFileListW(hbc: *const ::core::ffi::c_void, ppwszzfilelist: *mut super::super::super::Foundation::PWSTR, pcbsize: *mut u32) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn CertSrvBackupOpenFileW(hbc: *mut ::core::ffi::c_void, pwszattachmentname: super::super::super::Foundation::PWSTR, cbreadhintsize: u32, plifilesize: *mut i64) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn CertSrvBackupPrepareW(pwszservername: super::super::super::Foundation::PWSTR, grbitjet: u32, dwbackupflags: CSBACKUP_TYPE, phbc: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
    pub fn CertSrvBackupRead(hbc: *mut ::core::ffi::c_void, pvbuffer: *mut ::core::ffi::c_void, cbbuffer: u32, pcbread: *mut u32) -> ::windows_sys::core::HRESULT;
    pub fn CertSrvBackupTruncateLogs(hbc: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn CertSrvIsServerOnlineW(pwszservername: super::super::super::Foundation::PWSTR, pfserveronline: *mut super::super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT;
    pub fn CertSrvRestoreEnd(hbc: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn CertSrvRestoreGetDatabaseLocationsW(hbc: *const ::core::ffi::c_void, ppwszzdatabaselocationlist: *mut super::super::super::Foundation::PWSTR, pcbsize: *mut u32) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn CertSrvRestorePrepareW(pwszservername: super::super::super::Foundation::PWSTR, dwrestoreflags: u32, phbc: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
    pub fn CertSrvRestoreRegisterComplete(hbc: *mut ::core::ffi::c_void, hrrestorestate: ::windows_sys::core::HRESULT) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn CertSrvRestoreRegisterThroughFile(hbc: *mut ::core::ffi::c_void, pwszcheckpointfilepath: super::super::super::Foundation::PWSTR, pwszlogpath: super::super::super::Foundation::PWSTR, rgrstmap: *mut CSEDB_RSTMAPW, crstmap: i32, pwszbackuplogpath: super::super::super::Foundation::PWSTR, genlow: u32, genhigh: u32) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn CertSrvRestoreRegisterW(hbc: *mut ::core::ffi::c_void, pwszcheckpointfilepath: super::super::super::Foundation::PWSTR, pwszlogpath: super::super::super::Foundation::PWSTR, rgrstmap: *mut CSEDB_RSTMAPW, crstmap: i32, pwszbackuplogpath: super::super::super::Foundation::PWSTR, genlow: u32, genhigh: u32) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn CertSrvServerControlW(pwszservername: super::super::super::Foundation::PWSTR, dwcontrolflags: u32, pcbout: *mut u32, ppbout: *mut *mut u8) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn PstAcquirePrivateKey(pcert: *const super::CERT_CONTEXT) -> super::super::super::Foundation::NTSTATUS;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Authentication_Identity"))]
    pub fn PstGetCertificateChain(pcert: *const super::CERT_CONTEXT, ptrustedissuers: *const super::super::Authentication::Identity::SecPkgContext_IssuerListInfoEx, ppcertchaincontext: *mut *mut super::CERT_CHAIN_CONTEXT) -> super::super::super::Foundation::NTSTATUS;
    #[cfg(feature = "Win32_Foundation")]
    pub fn PstGetCertificates(ptargetname: *const super::super::super::Foundation::UNICODE_STRING, ccriteria: u32, rgpcriteria: *const super::CERT_SELECT_CRITERIA, bisclient: super::super::super::Foundation::BOOL, pdwcertchaincontextcount: *mut u32, ppcertchaincontexts: *mut *mut *mut super::CERT_CHAIN_CONTEXT) -> super::super::super::Foundation::NTSTATUS;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Authentication_Identity"))]
    pub fn PstGetTrustAnchors(ptargetname: *const super::super::super::Foundation::UNICODE_STRING, ccriteria: u32, rgpcriteria: *const super::CERT_SELECT_CRITERIA, pptrustedissuers: *mut *mut super::super::Authentication::Identity::SecPkgContext_IssuerListInfoEx) -> super::super::super::Foundation::NTSTATUS;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Authentication_Identity"))]
    pub fn PstGetTrustAnchorsEx(ptargetname: *const super::super::super::Foundation::UNICODE_STRING, ccriteria: u32, rgpcriteria: *const super::CERT_SELECT_CRITERIA, pcertcontext: *const super::CERT_CONTEXT, pptrustedissuers: *mut *mut super::super::Authentication::Identity::SecPkgContext_IssuerListInfoEx) -> super::super::super::Foundation::NTSTATUS;
    #[cfg(feature = "Win32_Foundation")]
    pub fn PstGetUserNameForCertificate(pcertcontext: *const super::CERT_CONTEXT, username: *mut super::super::super::Foundation::UNICODE_STRING) -> super::super::super::Foundation::NTSTATUS;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Authentication_Identity"))]
    pub fn PstMapCertificate(pcert: *const super::CERT_CONTEXT, ptokeninformationtype: *mut super::super::Authentication::Identity::LSA_TOKEN_INFORMATION_TYPE, pptokeninformation: *mut *mut ::core::ffi::c_void) -> super::super::super::Foundation::NTSTATUS;
    #[cfg(feature = "Win32_Foundation")]
    pub fn PstValidate(ptargetname: *const super::super::super::Foundation::UNICODE_STRING, bisclient: super::super::super::Foundation::BOOL, prequestedissuancepolicy: *const super::CERT_USAGE_MATCH, phadditionalcertstore: *const *const ::core::ffi::c_void, pcert: *const super::CERT_CONTEXT, pprovguid: *mut ::windows_sys::core::GUID) -> super::super::super::Foundation::NTSTATUS;
}
pub const XECT_EXTENSION_V1: u32 = 1u32;
pub const XECT_EXTENSION_V2: u32 = 2u32;
pub const AlgorithmFlagsNone: i32 = 0i32;
pub const AlgorithmFlagsWrap: i32 = 1i32;
pub const XCN_NCRYPT_NO_OPERATION: i32 = 0i32;
pub const XCN_NCRYPT_CIPHER_OPERATION: i32 = 1i32;
pub const XCN_NCRYPT_HASH_OPERATION: i32 = 2i32;
pub const XCN_NCRYPT_ASYMMETRIC_ENCRYPTION_OPERATION: i32 = 4i32;
pub const XCN_NCRYPT_SECRET_AGREEMENT_OPERATION: i32 = 8i32;
pub const XCN_NCRYPT_SIGNATURE_OPERATION: i32 = 16i32;
pub const XCN_NCRYPT_RNG_OPERATION: i32 = 32i32;
pub const XCN_NCRYPT_KEY_DERIVATION_OPERATION: i32 = 64i32;
pub const XCN_NCRYPT_ANY_ASYMMETRIC_OPERATION: i32 = 28i32;
pub const XCN_NCRYPT_PREFER_SIGNATURE_ONLY_OPERATION: i32 = 2097152i32;
pub const XCN_NCRYPT_PREFER_NON_SIGNATURE_OPERATION: i32 = 4194304i32;
pub const XCN_NCRYPT_EXACT_MATCH_OPERATION: i32 = 8388608i32;
pub const XCN_NCRYPT_PREFERENCE_MASK_OPERATION: i32 = 14680064i32;
pub const XCN_BCRYPT_UNKNOWN_INTERFACE: i32 = 0i32;
pub const XCN_BCRYPT_CIPHER_INTERFACE: i32 = 1i32;
pub const XCN_BCRYPT_HASH_INTERFACE: i32 = 2i32;
pub const XCN_BCRYPT_ASYMMETRIC_ENCRYPTION_INTERFACE: i32 = 3i32;
pub const XCN_BCRYPT_SIGNATURE_INTERFACE: i32 = 5i32;
pub const XCN_BCRYPT_SECRET_AGREEMENT_INTERFACE: i32 = 4i32;
pub const XCN_BCRYPT_RNG_INTERFACE: i32 = 6i32;
pub const XCN_BCRYPT_KEY_DERIVATION_INTERFACE: i32 = 7i32;
pub const XCN_CERT_ALT_NAME_UNKNOWN: i32 = 0i32;
pub const XCN_CERT_ALT_NAME_OTHER_NAME: i32 = 1i32;
pub const XCN_CERT_ALT_NAME_RFC822_NAME: i32 = 2i32;
pub const XCN_CERT_ALT_NAME_DNS_NAME: i32 = 3i32;
pub const XCN_CERT_ALT_NAME_X400_ADDRESS: i32 = 4i32;
pub const XCN_CERT_ALT_NAME_DIRECTORY_NAME: i32 = 5i32;
pub const XCN_CERT_ALT_NAME_EDI_PARTY_NAME: i32 = 6i32;
pub const XCN_CERT_ALT_NAME_URL: i32 = 7i32;
pub const XCN_CERT_ALT_NAME_IP_ADDRESS: i32 = 8i32;
pub const XCN_CERT_ALT_NAME_REGISTERED_ID: i32 = 9i32;
pub const XCN_CERT_ALT_NAME_GUID: i32 = 10i32;
pub const XCN_CERT_ALT_NAME_USER_PRINCIPLE_NAME: i32 = 11i32;
pub const CAIF_DSENTRY: u32 = 1u32;
pub const CAIF_LOCAL: u32 = 8u32;
pub const CAIF_REGISTRY: u32 = 4u32;
pub const CAIF_REGISTRYPARENT: u32 = 16u32;
pub const CAIF_SHAREDFOLDERENTRY: u32 = 2u32;
#[repr(C)]
pub struct CAINFO {
    pub cbSize: u32,
    pub CAType: ENUM_CATYPES,
    pub cCASignatureCerts: u32,
    pub cCAExchangeCerts: u32,
    pub cExitModules: u32,
    pub lPropIdMax: i32,
    pub lRoleSeparationEnabled: i32,
    pub cKRACertUsedCount: u32,
    pub cKRACertCount: u32,
    pub fAdvancedServer: u32,
}
impl ::core::marker::Copy for CAINFO {}
impl ::core::clone::Clone for CAINFO {
    fn clone(&self) -> Self {
        *self
    }
}
pub const CAPATHLENGTH_INFINITE: u32 = 4294967295u32;
pub const CA_ACCESS_MASKROLES: u32 = 255u32;
pub const CA_CRL_BASE: u32 = 1u32;
pub const CA_CRL_DELTA: u32 = 2u32;
pub const CA_CRL_REPUBLISH: u32 = 16u32;
pub const CA_DISP_ERROR: u32 = 1u32;
pub const CA_DISP_INCOMPLETE: u32 = 0u32;
pub const CA_DISP_INVALID: u32 = 4u32;
pub const CA_DISP_REVOKED: u32 = 2u32;
pub const CA_DISP_UNDER_SUBMISSION: u32 = 5u32;
pub const CA_DISP_VALID: u32 = 3u32;
pub const CAlternativeName: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2286821395, data2: 8573, data3: 4570, data4: [178, 164, 0, 14, 123, 187, 43, 9] };
pub const CAlternativeNames: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2286821396, data2: 8573, data3: 4570, data4: [178, 164, 0, 14, 123, 187, 43, 9] };
pub const CBinaryConverter: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2286821378, data2: 8573, data3: 4570, data4: [178, 164, 0, 14, 123, 187, 43, 9] };
pub const CCLOCKSKEWMINUTESDEFAULT: u32 = 10u32;
pub const CC_UIPICKCONFIGSKIPLOCALCA: u32 = 5u32;
pub const CCertAdmin: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 938130160, data2: 32694, data3: 4560, data4: [136, 23, 0, 160, 201, 3, 184, 60] };
pub const CCertConfig: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 925879864, data2: 17188, data3: 4560, data4: [136, 16, 0, 160, 201, 3, 184, 60] };
pub const CCertEncodeAltName: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 486296794, data2: 4721, data3: 4561, data4: [155, 212, 0, 192, 79, 182, 131, 250] };
pub const CCertEncodeBitString: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1835744472, data2: 4728, data3: 4561, data4: [155, 212, 0, 192, 79, 182, 131, 250] };
pub const CCertEncodeCRLDistInfo: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 33185952, data2: 48127, data3: 4560, data4: [136, 37, 0, 160, 201, 3, 184, 60] };
pub const CCertEncodeDateArray: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 807368624, data2: 42096, data3: 4560, data4: [136, 33, 0, 160, 201, 3, 184, 60] };
pub const CCertEncodeLongArray: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1309048992, data2: 41122, data3: 4560, data4: [136, 33, 0, 160, 201, 3, 184, 60] };
pub const CCertEncodeStringArray: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 430403552, data2: 29844, data3: 4560, data4: [136, 22, 0, 160, 201, 3, 184, 60] };
pub const CCertGetConfig: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3335276976, data2: 52759, data3: 4560, data4: [136, 51, 0, 160, 201, 3, 184, 60] };
pub const CCertProperties: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2286821423, data2: 8573, data3: 4570, data4: [178, 164, 0, 14, 123, 187, 43, 9] };
pub const CCertProperty: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2286821422, data2: 8573, data3: 4570, data4: [178, 164, 0, 14, 123, 187, 43, 9] };
pub const CCertPropertyArchived: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2286821431, data2: 8573, data3: 4570, data4: [178, 164, 0, 14, 123, 187, 43, 9] };
pub const CCertPropertyArchivedKeyHash: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2286821435, data2: 8573, data3: 4570, data4: [178, 164, 0, 14, 123, 187, 43, 9] };
pub const CCertPropertyAutoEnroll: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2286821426, data2: 8573, data3: 4570, data4: [178, 164, 0, 14, 123, 187, 43, 9] };
pub const CCertPropertyBackedUp: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2286821432, data2: 8573, data3: 4570, data4: [178, 164, 0, 14, 123, 187, 43, 9] };
pub const CCertPropertyDescription: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2286821425, data2: 8573, data3: 4570, data4: [178, 164, 0, 14, 123, 187, 43, 9] };
pub const CCertPropertyEnrollment: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2286821433, data2: 8573, data3: 4570, data4: [178, 164, 0, 14, 123, 187, 43, 9] };
pub const CCertPropertyEnrollmentPolicyServer: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2286821452, data2: 8573, data3: 4570, data4: [178, 164, 0, 14, 123, 187, 43, 9] };
pub const CCertPropertyFriendlyName: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2286821424, data2: 8573, data3: 4570, data4: [178, 164, 0, 14, 123, 187, 43, 9] };
pub const CCertPropertyKeyProvInfo: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2286821430, data2: 8573, data3: 4570, data4: [178, 164, 0, 14, 123, 187, 43, 9] };
pub const CCertPropertyRenewal: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2286821434, data2: 8573, data3: 4570, data4: [178, 164, 0, 14, 123, 187, 43, 9] };
pub const CCertPropertyRequestOriginator: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2286821427, data2: 8573, data3: 4570, data4: [178, 164, 0, 14, 123, 187, 43, 9] };
pub const CCertPropertySHA1Hash: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2286821428, data2: 8573, data3: 4570, data4: [178, 164, 0, 14, 123, 187, 43, 9] };
pub const CCertRequest: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2561668080, data2: 21796, data3: 4560, data4: [136, 18, 0, 160, 201, 3, 184, 60] };
pub const CCertServerExit: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1279942208, data2: 29484, data3: 4560, data4: [136, 22, 0, 160, 201, 3, 184, 60] };
pub const CCertServerPolicy: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2852129062, data2: 65470, data3: 4559, data4: [136, 0, 0, 160, 201, 3, 184, 60] };
pub const CCertView: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2704084858, data2: 7812, data3: 4561, data4: [155, 214, 0, 192, 79, 182, 131, 250] };
pub const CCertificateAttestationChallenge: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 325234081, data2: 60256, data3: 17770, data4: [182, 225, 17, 128, 80, 219, 116, 27] };
pub const CCertificatePolicies: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2286821407, data2: 8573, data3: 4570, data4: [178, 164, 0, 14, 123, 187, 43, 9] };
pub const CCertificatePolicy: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2286821406, data2: 8573, data3: 4570, data4: [178, 164, 0, 14, 123, 187, 43, 9] };
pub const CCryptAttribute: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2286821420, data2: 8573, data3: 4570, data4: [178, 164, 0, 14, 123, 187, 43, 9] };
pub const CCryptAttributes: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2286821421, data2: 8573, data3: 4570, data4: [178, 164, 0, 14, 123, 187, 43, 9] };
pub const CCspInformation: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2286821383, data2: 8573, data3: 4570, data4: [178, 164, 0, 14, 123, 187, 43, 9] };
pub const CCspInformations: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2286821384, data2: 8573, data3: 4570, data4: [178, 164, 0, 14, 123, 187, 43, 9] };
pub const CCspStatus: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2286821385, data2: 8573, data3: 4570, data4: [178, 164, 0, 14, 123, 187, 43, 9] };
pub const CA_ACCESS_ADMIN: u32 = 1u32;
pub const CA_ACCESS_AUDITOR: u32 = 4u32;
pub const CA_ACCESS_ENROLL: u32 = 512u32;
pub const CA_ACCESS_OFFICER: u32 = 2u32;
pub const CA_ACCESS_OPERATOR: u32 = 8u32;
pub const CA_ACCESS_READ: u32 = 256u32;
pub const CERTENROLL_INDEX_BASE: u32 = 0u32;
pub const XCN_OID_NONE: i32 = 0i32;
pub const XCN_OID_RSA: i32 = 1i32;
pub const XCN_OID_PKCS: i32 = 2i32;
pub const XCN_OID_RSA_HASH: i32 = 3i32;
pub const XCN_OID_RSA_ENCRYPT: i32 = 4i32;
pub const XCN_OID_PKCS_1: i32 = 5i32;
pub const XCN_OID_PKCS_2: i32 = 6i32;
pub const XCN_OID_PKCS_3: i32 = 7i32;
pub const XCN_OID_PKCS_4: i32 = 8i32;
pub const XCN_OID_PKCS_5: i32 = 9i32;
pub const XCN_OID_PKCS_6: i32 = 10i32;
pub const XCN_OID_PKCS_7: i32 = 11i32;
pub const XCN_OID_PKCS_8: i32 = 12i32;
pub const XCN_OID_PKCS_9: i32 = 13i32;
pub const XCN_OID_PKCS_10: i32 = 14i32;
pub const XCN_OID_PKCS_12: i32 = 15i32;
pub const XCN_OID_RSA_RSA: i32 = 16i32;
pub const XCN_OID_RSA_MD2RSA: i32 = 17i32;
pub const XCN_OID_RSA_MD4RSA: i32 = 18i32;
pub const XCN_OID_RSA_MD5RSA: i32 = 19i32;
pub const XCN_OID_RSA_SHA1RSA: i32 = 20i32;
pub const XCN_OID_RSA_SETOAEP_RSA: i32 = 21i32;
pub const XCN_OID_RSA_DH: i32 = 22i32;
pub const XCN_OID_RSA_data: i32 = 23i32;
pub const XCN_OID_RSA_signedData: i32 = 24i32;
pub const XCN_OID_RSA_envelopedData: i32 = 25i32;
pub const XCN_OID_RSA_signEnvData: i32 = 26i32;
pub const XCN_OID_RSA_digestedData: i32 = 27i32;
pub const XCN_OID_RSA_hashedData: i32 = 28i32;
pub const XCN_OID_RSA_encryptedData: i32 = 29i32;
pub const XCN_OID_RSA_emailAddr: i32 = 30i32;
pub const XCN_OID_RSA_unstructName: i32 = 31i32;
pub const XCN_OID_RSA_contentType: i32 = 32i32;
pub const XCN_OID_RSA_messageDigest: i32 = 33i32;
pub const XCN_OID_RSA_signingTime: i32 = 34i32;
pub const XCN_OID_RSA_counterSign: i32 = 35i32;
pub const XCN_OID_RSA_challengePwd: i32 = 36i32;
pub const XCN_OID_RSA_unstructAddr: i32 = 37i32;
pub const XCN_OID_RSA_extCertAttrs: i32 = 38i32;
pub const XCN_OID_RSA_certExtensions: i32 = 39i32;
pub const XCN_OID_RSA_SMIMECapabilities: i32 = 40i32;
pub const XCN_OID_RSA_preferSignedData: i32 = 41i32;
pub const XCN_OID_RSA_SMIMEalg: i32 = 42i32;
pub const XCN_OID_RSA_SMIMEalgESDH: i32 = 43i32;
pub const XCN_OID_RSA_SMIMEalgCMS3DESwrap: i32 = 44i32;
pub const XCN_OID_RSA_SMIMEalgCMSRC2wrap: i32 = 45i32;
pub const XCN_OID_RSA_MD2: i32 = 46i32;
pub const XCN_OID_RSA_MD4: i32 = 47i32;
pub const XCN_OID_RSA_MD5: i32 = 48i32;
pub const XCN_OID_RSA_RC2CBC: i32 = 49i32;
pub const XCN_OID_RSA_RC4: i32 = 50i32;
pub const XCN_OID_RSA_DES_EDE3_CBC: i32 = 51i32;
pub const XCN_OID_RSA_RC5_CBCPad: i32 = 52i32;
pub const XCN_OID_ANSI_X942: i32 = 53i32;
pub const XCN_OID_ANSI_X942_DH: i32 = 54i32;
pub const XCN_OID_X957: i32 = 55i32;
pub const XCN_OID_X957_DSA: i32 = 56i32;
pub const XCN_OID_X957_SHA1DSA: i32 = 57i32;
pub const XCN_OID_DS: i32 = 58i32;
pub const XCN_OID_DSALG: i32 = 59i32;
pub const XCN_OID_DSALG_CRPT: i32 = 60i32;
pub const XCN_OID_DSALG_HASH: i32 = 61i32;
pub const XCN_OID_DSALG_SIGN: i32 = 62i32;
pub const XCN_OID_DSALG_RSA: i32 = 63i32;
pub const XCN_OID_OIW: i32 = 64i32;
pub const XCN_OID_OIWSEC: i32 = 65i32;
pub const XCN_OID_OIWSEC_md4RSA: i32 = 66i32;
pub const XCN_OID_OIWSEC_md5RSA: i32 = 67i32;
pub const XCN_OID_OIWSEC_md4RSA2: i32 = 68i32;
pub const XCN_OID_OIWSEC_desECB: i32 = 69i32;
pub const XCN_OID_OIWSEC_desCBC: i32 = 70i32;
pub const XCN_OID_OIWSEC_desOFB: i32 = 71i32;
pub const XCN_OID_OIWSEC_desCFB: i32 = 72i32;
pub const XCN_OID_OIWSEC_desMAC: i32 = 73i32;
pub const XCN_OID_OIWSEC_rsaSign: i32 = 74i32;
pub const XCN_OID_OIWSEC_dsa: i32 = 75i32;
pub const XCN_OID_OIWSEC_shaDSA: i32 = 76i32;
pub const XCN_OID_OIWSEC_mdc2RSA: i32 = 77i32;
pub const XCN_OID_OIWSEC_shaRSA: i32 = 78i32;
pub const XCN_OID_OIWSEC_dhCommMod: i32 = 79i32;
pub const XCN_OID_OIWSEC_desEDE: i32 = 80i32;
pub const XCN_OID_OIWSEC_sha: i32 = 81i32;
pub const XCN_OID_OIWSEC_mdc2: i32 = 82i32;
pub const XCN_OID_OIWSEC_dsaComm: i32 = 83i32;
pub const XCN_OID_OIWSEC_dsaCommSHA: i32 = 84i32;
pub const XCN_OID_OIWSEC_rsaXchg: i32 = 85i32;
pub const XCN_OID_OIWSEC_keyHashSeal: i32 = 86i32;
pub const XCN_OID_OIWSEC_md2RSASign: i32 = 87i32;
pub const XCN_OID_OIWSEC_md5RSASign: i32 = 88i32;
pub const XCN_OID_OIWSEC_sha1: i32 = 89i32;
pub const XCN_OID_OIWSEC_dsaSHA1: i32 = 90i32;
pub const XCN_OID_OIWSEC_dsaCommSHA1: i32 = 91i32;
pub const XCN_OID_OIWSEC_sha1RSASign: i32 = 92i32;
pub const XCN_OID_OIWDIR: i32 = 93i32;
pub const XCN_OID_OIWDIR_CRPT: i32 = 94i32;
pub const XCN_OID_OIWDIR_HASH: i32 = 95i32;
pub const XCN_OID_OIWDIR_SIGN: i32 = 96i32;
pub const XCN_OID_OIWDIR_md2: i32 = 97i32;
pub const XCN_OID_OIWDIR_md2RSA: i32 = 98i32;
pub const XCN_OID_INFOSEC: i32 = 99i32;
pub const XCN_OID_INFOSEC_sdnsSignature: i32 = 100i32;
pub const XCN_OID_INFOSEC_mosaicSignature: i32 = 101i32;
pub const XCN_OID_INFOSEC_sdnsConfidentiality: i32 = 102i32;
pub const XCN_OID_INFOSEC_mosaicConfidentiality: i32 = 103i32;
pub const XCN_OID_INFOSEC_sdnsIntegrity: i32 = 104i32;
pub const XCN_OID_INFOSEC_mosaicIntegrity: i32 = 105i32;
pub const XCN_OID_INFOSEC_sdnsTokenProtection: i32 = 106i32;
pub const XCN_OID_INFOSEC_mosaicTokenProtection: i32 = 107i32;
pub const XCN_OID_INFOSEC_sdnsKeyManagement: i32 = 108i32;
pub const XCN_OID_INFOSEC_mosaicKeyManagement: i32 = 109i32;
pub const XCN_OID_INFOSEC_sdnsKMandSig: i32 = 110i32;
pub const XCN_OID_INFOSEC_mosaicKMandSig: i32 = 111i32;
pub const XCN_OID_INFOSEC_SuiteASignature: i32 = 112i32;
pub const XCN_OID_INFOSEC_SuiteAConfidentiality: i32 = 113i32;
pub const XCN_OID_INFOSEC_SuiteAIntegrity: i32 = 114i32;
pub const XCN_OID_INFOSEC_SuiteATokenProtection: i32 = 115i32;
pub const XCN_OID_INFOSEC_SuiteAKeyManagement: i32 = 116i32;
pub const XCN_OID_INFOSEC_SuiteAKMandSig: i32 = 117i32;
pub const XCN_OID_INFOSEC_mosaicUpdatedSig: i32 = 118i32;
pub const XCN_OID_INFOSEC_mosaicKMandUpdSig: i32 = 119i32;
pub const XCN_OID_INFOSEC_mosaicUpdatedInteg: i32 = 120i32;
pub const XCN_OID_COMMON_NAME: i32 = 121i32;
pub const XCN_OID_SUR_NAME: i32 = 122i32;
pub const XCN_OID_DEVICE_SERIAL_NUMBER: i32 = 123i32;
pub const XCN_OID_COUNTRY_NAME: i32 = 124i32;
pub const XCN_OID_LOCALITY_NAME: i32 = 125i32;
pub const XCN_OID_STATE_OR_PROVINCE_NAME: i32 = 126i32;
pub const XCN_OID_STREET_ADDRESS: i32 = 127i32;
pub const XCN_OID_ORGANIZATION_NAME: i32 = 128i32;
pub const XCN_OID_ORGANIZATIONAL_UNIT_NAME: i32 = 129i32;
pub const XCN_OID_TITLE: i32 = 130i32;
pub const XCN_OID_DESCRIPTION: i32 = 131i32;
pub const XCN_OID_SEARCH_GUIDE: i32 = 132i32;
pub const XCN_OID_BUSINESS_CATEGORY: i32 = 133i32;
pub const XCN_OID_POSTAL_ADDRESS: i32 = 134i32;
pub const XCN_OID_POSTAL_CODE: i32 = 135i32;
pub const XCN_OID_POST_OFFICE_BOX: i32 = 136i32;
pub const XCN_OID_PHYSICAL_DELIVERY_OFFICE_NAME: i32 = 137i32;
pub const XCN_OID_TELEPHONE_NUMBER: i32 = 138i32;
pub const XCN_OID_TELEX_NUMBER: i32 = 139i32;
pub const XCN_OID_TELETEXT_TERMINAL_IDENTIFIER: i32 = 140i32;
pub const XCN_OID_FACSIMILE_TELEPHONE_NUMBER: i32 = 141i32;
pub const XCN_OID_X21_ADDRESS: i32 = 142i32;
pub const XCN_OID_INTERNATIONAL_ISDN_NUMBER: i32 = 143i32;
pub const XCN_OID_REGISTERED_ADDRESS: i32 = 144i32;
pub const XCN_OID_DESTINATION_INDICATOR: i32 = 145i32;
pub const XCN_OID_PREFERRED_DELIVERY_METHOD: i32 = 146i32;
pub const XCN_OID_PRESENTATION_ADDRESS: i32 = 147i32;
pub const XCN_OID_SUPPORTED_APPLICATION_CONTEXT: i32 = 148i32;
pub const XCN_OID_MEMBER: i32 = 149i32;
pub const XCN_OID_OWNER: i32 = 150i32;
pub const XCN_OID_ROLE_OCCUPANT: i32 = 151i32;
pub const XCN_OID_SEE_ALSO: i32 = 152i32;
pub const XCN_OID_USER_PASSWORD: i32 = 153i32;
pub const XCN_OID_USER_CERTIFICATE: i32 = 154i32;
pub const XCN_OID_CA_CERTIFICATE: i32 = 155i32;
pub const XCN_OID_AUTHORITY_REVOCATION_LIST: i32 = 156i32;
pub const XCN_OID_CERTIFICATE_REVOCATION_LIST: i32 = 157i32;
pub const XCN_OID_CROSS_CERTIFICATE_PAIR: i32 = 158i32;
pub const XCN_OID_GIVEN_NAME: i32 = 159i32;
pub const XCN_OID_INITIALS: i32 = 160i32;
pub const XCN_OID_DN_QUALIFIER: i32 = 161i32;
pub const XCN_OID_DOMAIN_COMPONENT: i32 = 162i32;
pub const XCN_OID_PKCS_12_FRIENDLY_NAME_ATTR: i32 = 163i32;
pub const XCN_OID_PKCS_12_LOCAL_KEY_ID: i32 = 164i32;
pub const XCN_OID_PKCS_12_KEY_PROVIDER_NAME_ATTR: i32 = 165i32;
pub const XCN_OID_LOCAL_MACHINE_KEYSET: i32 = 166i32;
pub const XCN_OID_PKCS_12_EXTENDED_ATTRIBUTES: i32 = 167i32;
pub const XCN_OID_KEYID_RDN: i32 = 168i32;
pub const XCN_OID_AUTHORITY_KEY_IDENTIFIER: i32 = 169i32;
pub const XCN_OID_KEY_ATTRIBUTES: i32 = 170i32;
pub const XCN_OID_CERT_POLICIES_95: i32 = 171i32;
pub const XCN_OID_KEY_USAGE_RESTRICTION: i32 = 172i32;
pub const XCN_OID_SUBJECT_ALT_NAME: i32 = 173i32;
pub const XCN_OID_ISSUER_ALT_NAME: i32 = 174i32;
pub const XCN_OID_BASIC_CONSTRAINTS: i32 = 175i32;
pub const XCN_OID_KEY_USAGE: i32 = 176i32;
pub const XCN_OID_PRIVATEKEY_USAGE_PERIOD: i32 = 177i32;
pub const XCN_OID_BASIC_CONSTRAINTS2: i32 = 178i32;
pub const XCN_OID_CERT_POLICIES: i32 = 179i32;
pub const XCN_OID_ANY_CERT_POLICY: i32 = 180i32;
pub const XCN_OID_AUTHORITY_KEY_IDENTIFIER2: i32 = 181i32;
pub const XCN_OID_SUBJECT_KEY_IDENTIFIER: i32 = 182i32;
pub const XCN_OID_SUBJECT_ALT_NAME2: i32 = 183i32;
pub const XCN_OID_ISSUER_ALT_NAME2: i32 = 184i32;
pub const XCN_OID_CRL_REASON_CODE: i32 = 185i32;
pub const XCN_OID_REASON_CODE_HOLD: i32 = 186i32;
pub const XCN_OID_CRL_DIST_POINTS: i32 = 187i32;
pub const XCN_OID_ENHANCED_KEY_USAGE: i32 = 188i32;
pub const XCN_OID_CRL_NUMBER: i32 = 189i32;
pub const XCN_OID_DELTA_CRL_INDICATOR: i32 = 190i32;
pub const XCN_OID_ISSUING_DIST_POINT: i32 = 191i32;
pub const XCN_OID_FRESHEST_CRL: i32 = 192i32;
pub const XCN_OID_NAME_CONSTRAINTS: i32 = 193i32;
pub const XCN_OID_POLICY_MAPPINGS: i32 = 194i32;
pub const XCN_OID_LEGACY_POLICY_MAPPINGS: i32 = 195i32;
pub const XCN_OID_POLICY_CONSTRAINTS: i32 = 196i32;
pub const XCN_OID_RENEWAL_CERTIFICATE: i32 = 197i32;
pub const XCN_OID_ENROLLMENT_NAME_VALUE_PAIR: i32 = 198i32;
pub const XCN_OID_ENROLLMENT_CSP_PROVIDER: i32 = 199i32;
pub const XCN_OID_OS_VERSION: i32 = 200i32;
pub const XCN_OID_ENROLLMENT_AGENT: i32 = 201i32;
pub const XCN_OID_PKIX: i32 = 202i32;
pub const XCN_OID_PKIX_PE: i32 = 203i32;
pub const XCN_OID_AUTHORITY_INFO_ACCESS: i32 = 204i32;
pub const XCN_OID_BIOMETRIC_EXT: i32 = 205i32;
pub const XCN_OID_LOGOTYPE_EXT: i32 = 206i32;
pub const XCN_OID_CERT_EXTENSIONS: i32 = 207i32;
pub const XCN_OID_NEXT_UPDATE_LOCATION: i32 = 208i32;
pub const XCN_OID_REMOVE_CERTIFICATE: i32 = 209i32;
pub const XCN_OID_CROSS_CERT_DIST_POINTS: i32 = 210i32;
pub const XCN_OID_CTL: i32 = 211i32;
pub const XCN_OID_SORTED_CTL: i32 = 212i32;
pub const XCN_OID_SERIALIZED: i32 = 213i32;
pub const XCN_OID_NT_PRINCIPAL_NAME: i32 = 214i32;
pub const XCN_OID_PRODUCT_UPDATE: i32 = 215i32;
pub const XCN_OID_ANY_APPLICATION_POLICY: i32 = 216i32;
pub const XCN_OID_AUTO_ENROLL_CTL_USAGE: i32 = 217i32;
pub const XCN_OID_ENROLL_CERTTYPE_EXTENSION: i32 = 218i32;
pub const XCN_OID_CERT_MANIFOLD: i32 = 219i32;
pub const XCN_OID_CERTSRV_CA_VERSION: i32 = 220i32;
pub const XCN_OID_CERTSRV_PREVIOUS_CERT_HASH: i32 = 221i32;
pub const XCN_OID_CRL_VIRTUAL_BASE: i32 = 222i32;
pub const XCN_OID_CRL_NEXT_PUBLISH: i32 = 223i32;
pub const XCN_OID_KP_CA_EXCHANGE: i32 = 224i32;
pub const XCN_OID_KP_KEY_RECOVERY_AGENT: i32 = 225i32;
pub const XCN_OID_CERTIFICATE_TEMPLATE: i32 = 226i32;
pub const XCN_OID_ENTERPRISE_OID_ROOT: i32 = 227i32;
pub const XCN_OID_RDN_DUMMY_SIGNER: i32 = 228i32;
pub const XCN_OID_APPLICATION_CERT_POLICIES: i32 = 229i32;
pub const XCN_OID_APPLICATION_POLICY_MAPPINGS: i32 = 230i32;
pub const XCN_OID_APPLICATION_POLICY_CONSTRAINTS: i32 = 231i32;
pub const XCN_OID_ARCHIVED_KEY_ATTR: i32 = 232i32;
pub const XCN_OID_CRL_SELF_CDP: i32 = 233i32;
pub const XCN_OID_REQUIRE_CERT_CHAIN_POLICY: i32 = 234i32;
pub const XCN_OID_ARCHIVED_KEY_CERT_HASH: i32 = 235i32;
pub const XCN_OID_ISSUED_CERT_HASH: i32 = 236i32;
pub const XCN_OID_DS_EMAIL_REPLICATION: i32 = 237i32;
pub const XCN_OID_REQUEST_CLIENT_INFO: i32 = 238i32;
pub const XCN_OID_ENCRYPTED_KEY_HASH: i32 = 239i32;
pub const XCN_OID_CERTSRV_CROSSCA_VERSION: i32 = 240i32;
pub const XCN_OID_NTDS_REPLICATION: i32 = 241i32;
pub const XCN_OID_SUBJECT_DIR_ATTRS: i32 = 242i32;
pub const XCN_OID_PKIX_KP: i32 = 243i32;
pub const XCN_OID_PKIX_KP_SERVER_AUTH: i32 = 244i32;
pub const XCN_OID_PKIX_KP_CLIENT_AUTH: i32 = 245i32;
pub const XCN_OID_PKIX_KP_CODE_SIGNING: i32 = 246i32;
pub const XCN_OID_PKIX_KP_EMAIL_PROTECTION: i32 = 247i32;
pub const XCN_OID_PKIX_KP_IPSEC_END_SYSTEM: i32 = 248i32;
pub const XCN_OID_PKIX_KP_IPSEC_TUNNEL: i32 = 249i32;
pub const XCN_OID_PKIX_KP_IPSEC_USER: i32 = 250i32;
pub const XCN_OID_PKIX_KP_TIMESTAMP_SIGNING: i32 = 251i32;
pub const XCN_OID_PKIX_KP_OCSP_SIGNING: i32 = 252i32;
pub const XCN_OID_PKIX_OCSP_NOCHECK: i32 = 253i32;
pub const XCN_OID_IPSEC_KP_IKE_INTERMEDIATE: i32 = 254i32;
pub const XCN_OID_KP_CTL_USAGE_SIGNING: i32 = 255i32;
pub const XCN_OID_KP_TIME_STAMP_SIGNING: i32 = 256i32;
pub const XCN_OID_SERVER_GATED_CRYPTO: i32 = 257i32;
pub const XCN_OID_SGC_NETSCAPE: i32 = 258i32;
pub const XCN_OID_KP_EFS: i32 = 259i32;
pub const XCN_OID_EFS_RECOVERY: i32 = 260i32;
pub const XCN_OID_WHQL_CRYPTO: i32 = 261i32;
pub const XCN_OID_NT5_CRYPTO: i32 = 262i32;
pub const XCN_OID_OEM_WHQL_CRYPTO: i32 = 263i32;
pub const XCN_OID_EMBEDDED_NT_CRYPTO: i32 = 264i32;
pub const XCN_OID_ROOT_LIST_SIGNER: i32 = 265i32;
pub const XCN_OID_KP_QUALIFIED_SUBORDINATION: i32 = 266i32;
pub const XCN_OID_KP_KEY_RECOVERY: i32 = 267i32;
pub const XCN_OID_KP_DOCUMENT_SIGNING: i32 = 268i32;
pub const XCN_OID_KP_LIFETIME_SIGNING: i32 = 269i32;
pub const XCN_OID_KP_MOBILE_DEVICE_SOFTWARE: i32 = 270i32;
pub const XCN_OID_KP_SMART_DISPLAY: i32 = 271i32;
pub const XCN_OID_KP_CSP_SIGNATURE: i32 = 272i32;
pub const XCN_OID_DRM: i32 = 273i32;
pub const XCN_OID_DRM_INDIVIDUALIZATION: i32 = 274i32;
pub const XCN_OID_LICENSES: i32 = 275i32;
pub const XCN_OID_LICENSE_SERVER: i32 = 276i32;
pub const XCN_OID_KP_SMARTCARD_LOGON: i32 = 277i32;
pub const XCN_OID_YESNO_TRUST_ATTR: i32 = 278i32;
pub const XCN_OID_PKIX_POLICY_QUALIFIER_CPS: i32 = 279i32;
pub const XCN_OID_PKIX_POLICY_QUALIFIER_USERNOTICE: i32 = 280i32;
pub const XCN_OID_CERT_POLICIES_95_QUALIFIER1: i32 = 281i32;
pub const XCN_OID_PKIX_ACC_DESCR: i32 = 282i32;
pub const XCN_OID_PKIX_OCSP: i32 = 283i32;
pub const XCN_OID_PKIX_CA_ISSUERS: i32 = 284i32;
pub const XCN_OID_VERISIGN_PRIVATE_6_9: i32 = 285i32;
pub const XCN_OID_VERISIGN_ONSITE_JURISDICTION_HASH: i32 = 286i32;
pub const XCN_OID_VERISIGN_BITSTRING_6_13: i32 = 287i32;
pub const XCN_OID_VERISIGN_ISS_STRONG_CRYPTO: i32 = 288i32;
pub const XCN_OID_NETSCAPE: i32 = 289i32;
pub const XCN_OID_NETSCAPE_CERT_EXTENSION: i32 = 290i32;
pub const XCN_OID_NETSCAPE_CERT_TYPE: i32 = 291i32;
pub const XCN_OID_NETSCAPE_BASE_URL: i32 = 292i32;
pub const XCN_OID_NETSCAPE_REVOCATION_URL: i32 = 293i32;
pub const XCN_OID_NETSCAPE_CA_REVOCATION_URL: i32 = 294i32;
pub const XCN_OID_NETSCAPE_CERT_RENEWAL_URL: i32 = 295i32;
pub const XCN_OID_NETSCAPE_CA_POLICY_URL: i32 = 296i32;
pub const XCN_OID_NETSCAPE_SSL_SERVER_NAME: i32 = 297i32;
pub const XCN_OID_NETSCAPE_COMMENT: i32 = 298i32;
pub const XCN_OID_NETSCAPE_DATA_TYPE: i32 = 299i32;
pub const XCN_OID_NETSCAPE_CERT_SEQUENCE: i32 = 300i32;
pub const XCN_OID_CT_PKI_DATA: i32 = 301i32;
pub const XCN_OID_CT_PKI_RESPONSE: i32 = 302i32;
pub const XCN_OID_PKIX_NO_SIGNATURE: i32 = 303i32;
pub const XCN_OID_CMC: i32 = 304i32;
pub const XCN_OID_CMC_STATUS_INFO: i32 = 305i32;
pub const XCN_OID_CMC_IDENTIFICATION: i32 = 306i32;
pub const XCN_OID_CMC_IDENTITY_PROOF: i32 = 307i32;
pub const XCN_OID_CMC_DATA_RETURN: i32 = 308i32;
pub const XCN_OID_CMC_TRANSACTION_ID: i32 = 309i32;
pub const XCN_OID_CMC_SENDER_NONCE: i32 = 310i32;
pub const XCN_OID_CMC_RECIPIENT_NONCE: i32 = 311i32;
pub const XCN_OID_CMC_ADD_EXTENSIONS: i32 = 312i32;
pub const XCN_OID_CMC_ENCRYPTED_POP: i32 = 313i32;
pub const XCN_OID_CMC_DECRYPTED_POP: i32 = 314i32;
pub const XCN_OID_CMC_LRA_POP_WITNESS: i32 = 315i32;
pub const XCN_OID_CMC_GET_CERT: i32 = 316i32;
pub const XCN_OID_CMC_GET_CRL: i32 = 317i32;
pub const XCN_OID_CMC_REVOKE_REQUEST: i32 = 318i32;
pub const XCN_OID_CMC_REG_INFO: i32 = 319i32;
pub const XCN_OID_CMC_RESPONSE_INFO: i32 = 320i32;
pub const XCN_OID_CMC_QUERY_PENDING: i32 = 321i32;
pub const XCN_OID_CMC_ID_POP_LINK_RANDOM: i32 = 322i32;
pub const XCN_OID_CMC_ID_POP_LINK_WITNESS: i32 = 323i32;
pub const XCN_OID_CMC_ID_CONFIRM_CERT_ACCEPTANCE: i32 = 324i32;
pub const XCN_OID_CMC_ADD_ATTRIBUTES: i32 = 325i32;
pub const XCN_OID_LOYALTY_OTHER_LOGOTYPE: i32 = 326i32;
pub const XCN_OID_BACKGROUND_OTHER_LOGOTYPE: i32 = 327i32;
pub const XCN_OID_PKIX_OCSP_BASIC_SIGNED_RESPONSE: i32 = 328i32;
pub const XCN_OID_PKCS_7_DATA: i32 = 329i32;
pub const XCN_OID_PKCS_7_SIGNED: i32 = 330i32;
pub const XCN_OID_PKCS_7_ENVELOPED: i32 = 331i32;
pub const XCN_OID_PKCS_7_SIGNEDANDENVELOPED: i32 = 332i32;
pub const XCN_OID_PKCS_7_DIGESTED: i32 = 333i32;
pub const XCN_OID_PKCS_7_ENCRYPTED: i32 = 334i32;
pub const XCN_OID_PKCS_9_CONTENT_TYPE: i32 = 335i32;
pub const XCN_OID_PKCS_9_MESSAGE_DIGEST: i32 = 336i32;
pub const XCN_OID_CERT_PROP_ID_PREFIX: i32 = 337i32;
pub const XCN_OID_CERT_KEY_IDENTIFIER_PROP_ID: i32 = 338i32;
pub const XCN_OID_CERT_ISSUER_SERIAL_NUMBER_MD5_HASH_PROP_ID: i32 = 339i32;
pub const XCN_OID_CERT_SUBJECT_NAME_MD5_HASH_PROP_ID: i32 = 340i32;
pub const XCN_OID_CERT_MD5_HASH_PROP_ID: i32 = 341i32;
pub const XCN_OID_RSA_SHA256RSA: i32 = 342i32;
pub const XCN_OID_RSA_SHA384RSA: i32 = 343i32;
pub const XCN_OID_RSA_SHA512RSA: i32 = 344i32;
pub const XCN_OID_NIST_sha256: i32 = 345i32;
pub const XCN_OID_NIST_sha384: i32 = 346i32;
pub const XCN_OID_NIST_sha512: i32 = 347i32;
pub const XCN_OID_RSA_MGF1: i32 = 348i32;
pub const XCN_OID_ECC_PUBLIC_KEY: i32 = 349i32;
pub const XCN_OID_ECDSA_SHA1: i32 = 350i32;
pub const XCN_OID_ECDSA_SPECIFIED: i32 = 351i32;
pub const XCN_OID_ANY_ENHANCED_KEY_USAGE: i32 = 352i32;
pub const XCN_OID_RSA_SSA_PSS: i32 = 353i32;
pub const XCN_OID_ATTR_SUPPORTED_ALGORITHMS: i32 = 355i32;
pub const XCN_OID_ATTR_TPM_SECURITY_ASSERTIONS: i32 = 356i32;
pub const XCN_OID_ATTR_TPM_SPECIFICATION: i32 = 357i32;
pub const XCN_OID_CERT_DISALLOWED_FILETIME_PROP_ID: i32 = 358i32;
pub const XCN_OID_CERT_SIGNATURE_HASH_PROP_ID: i32 = 359i32;
pub const XCN_OID_CERT_STRONG_KEY_OS_1: i32 = 360i32;
pub const XCN_OID_CERT_STRONG_KEY_OS_CURRENT: i32 = 361i32;
pub const XCN_OID_CERT_STRONG_KEY_OS_PREFIX: i32 = 362i32;
pub const XCN_OID_CERT_STRONG_SIGN_OS_1: i32 = 363i32;
pub const XCN_OID_CERT_STRONG_SIGN_OS_CURRENT: i32 = 364i32;
pub const XCN_OID_CERT_STRONG_SIGN_OS_PREFIX: i32 = 365i32;
pub const XCN_OID_DH_SINGLE_PASS_STDDH_SHA1_KDF: i32 = 366i32;
pub const XCN_OID_DH_SINGLE_PASS_STDDH_SHA256_KDF: i32 = 367i32;
pub const XCN_OID_DH_SINGLE_PASS_STDDH_SHA384_KDF: i32 = 368i32;
pub const XCN_OID_DISALLOWED_HASH: i32 = 369i32;
pub const XCN_OID_DISALLOWED_LIST: i32 = 370i32;
pub const XCN_OID_ECC_CURVE_P256: i32 = 371i32;
pub const XCN_OID_ECC_CURVE_P384: i32 = 372i32;
pub const XCN_OID_ECC_CURVE_P521: i32 = 373i32;
pub const XCN_OID_ECDSA_SHA256: i32 = 374i32;
pub const XCN_OID_ECDSA_SHA384: i32 = 375i32;
pub const XCN_OID_ECDSA_SHA512: i32 = 376i32;
pub const XCN_OID_ENROLL_CAXCHGCERT_HASH: i32 = 377i32;
pub const XCN_OID_ENROLL_EK_INFO: i32 = 378i32;
pub const XCN_OID_ENROLL_EKPUB_CHALLENGE: i32 = 379i32;
pub const XCN_OID_ENROLL_EKVERIFYCERT: i32 = 380i32;
pub const XCN_OID_ENROLL_EKVERIFYCREDS: i32 = 381i32;
pub const XCN_OID_ENROLL_EKVERIFYKEY: i32 = 382i32;
pub const XCN_OID_EV_RDN_COUNTRY: i32 = 383i32;
pub const XCN_OID_EV_RDN_LOCALE: i32 = 384i32;
pub const XCN_OID_EV_RDN_STATE_OR_PROVINCE: i32 = 385i32;
pub const XCN_OID_INHIBIT_ANY_POLICY: i32 = 386i32;
pub const XCN_OID_INTERNATIONALIZED_EMAIL_ADDRESS: i32 = 387i32;
pub const XCN_OID_KP_KERNEL_MODE_CODE_SIGNING: i32 = 388i32;
pub const XCN_OID_KP_KERNEL_MODE_HAL_EXTENSION_SIGNING: i32 = 389i32;
pub const XCN_OID_KP_KERNEL_MODE_TRUSTED_BOOT_SIGNING: i32 = 390i32;
pub const XCN_OID_KP_TPM_AIK_CERTIFICATE: i32 = 391i32;
pub const XCN_OID_KP_TPM_EK_CERTIFICATE: i32 = 392i32;
pub const XCN_OID_KP_TPM_PLATFORM_CERTIFICATE: i32 = 393i32;
pub const XCN_OID_NIST_AES128_CBC: i32 = 394i32;
pub const XCN_OID_NIST_AES128_WRAP: i32 = 395i32;
pub const XCN_OID_NIST_AES192_CBC: i32 = 396i32;
pub const XCN_OID_NIST_AES192_WRAP: i32 = 397i32;
pub const XCN_OID_NIST_AES256_CBC: i32 = 398i32;
pub const XCN_OID_NIST_AES256_WRAP: i32 = 399i32;
pub const XCN_OID_PKCS_12_PbeIds: i32 = 400i32;
pub const XCN_OID_PKCS_12_pbeWithSHA1And128BitRC2: i32 = 401i32;
pub const XCN_OID_PKCS_12_pbeWithSHA1And128BitRC4: i32 = 402i32;
pub const XCN_OID_PKCS_12_pbeWithSHA1And2KeyTripleDES: i32 = 403i32;
pub const XCN_OID_PKCS_12_pbeWithSHA1And3KeyTripleDES: i32 = 404i32;
pub const XCN_OID_PKCS_12_pbeWithSHA1And40BitRC2: i32 = 405i32;
pub const XCN_OID_PKCS_12_pbeWithSHA1And40BitRC4: i32 = 406i32;
pub const XCN_OID_PKCS_12_PROTECTED_PASSWORD_SECRET_BAG_TYPE_ID: i32 = 407i32;
pub const XCN_OID_PKINIT_KP_KDC: i32 = 408i32;
pub const XCN_OID_PKIX_CA_REPOSITORY: i32 = 409i32;
pub const XCN_OID_PKIX_OCSP_NONCE: i32 = 410i32;
pub const XCN_OID_PKIX_TIME_STAMPING: i32 = 411i32;
pub const XCN_OID_QC_EU_COMPLIANCE: i32 = 412i32;
pub const XCN_OID_QC_SSCD: i32 = 413i32;
pub const XCN_OID_QC_STATEMENTS_EXT: i32 = 414i32;
pub const XCN_OID_RDN_TPM_MANUFACTURER: i32 = 415i32;
pub const XCN_OID_RDN_TPM_MODEL: i32 = 416i32;
pub const XCN_OID_RDN_TPM_VERSION: i32 = 417i32;
pub const XCN_OID_REVOKED_LIST_SIGNER: i32 = 418i32;
pub const XCN_OID_RFC3161_counterSign: i32 = 419i32;
pub const XCN_OID_ROOT_PROGRAM_AUTO_UPDATE_CA_REVOCATION: i32 = 420i32;
pub const XCN_OID_ROOT_PROGRAM_AUTO_UPDATE_END_REVOCATION: i32 = 421i32;
pub const XCN_OID_ROOT_PROGRAM_FLAGS: i32 = 422i32;
pub const XCN_OID_ROOT_PROGRAM_NO_OCSP_FAILOVER_TO_CRL: i32 = 423i32;
pub const XCN_OID_RSA_PSPECIFIED: i32 = 424i32;
pub const XCN_OID_RSAES_OAEP: i32 = 425i32;
pub const XCN_OID_SUBJECT_INFO_ACCESS: i32 = 426i32;
pub const XCN_OID_TIMESTAMP_TOKEN: i32 = 427i32;
pub const XCN_OID_ENROLL_SCEP_ERROR: i32 = 428i32;
pub const XCN_OIDVerisign_MessageType: i32 = 429i32;
pub const XCN_OIDVerisign_PkiStatus: i32 = 430i32;
pub const XCN_OIDVerisign_FailInfo: i32 = 431i32;
pub const XCN_OIDVerisign_SenderNonce: i32 = 432i32;
pub const XCN_OIDVerisign_RecipientNonce: i32 = 433i32;
pub const XCN_OIDVerisign_TransactionID: i32 = 434i32;
pub const XCN_OID_ENROLL_ATTESTATION_CHALLENGE: i32 = 435i32;
pub const XCN_OID_ENROLL_ATTESTATION_STATEMENT: i32 = 436i32;
pub const XCN_OID_ENROLL_ENCRYPTION_ALGORITHM: i32 = 437i32;
pub const XCN_OID_ENROLL_KSP_NAME: i32 = 438i32;
pub const XCN_PROPERTYID_NONE: i32 = 0i32;
pub const XCN_CERT_KEY_PROV_HANDLE_PROP_ID: i32 = 1i32;
pub const XCN_CERT_KEY_PROV_INFO_PROP_ID: i32 = 2i32;
pub const XCN_CERT_SHA1_HASH_PROP_ID: i32 = 3i32;
pub const XCN_CERT_MD5_HASH_PROP_ID: i32 = 4i32;
pub const XCN_CERT_HASH_PROP_ID: i32 = 3i32;
pub const XCN_CERT_KEY_CONTEXT_PROP_ID: i32 = 5i32;
pub const XCN_CERT_KEY_SPEC_PROP_ID: i32 = 6i32;
pub const XCN_CERT_IE30_RESERVED_PROP_ID: i32 = 7i32;
pub const XCN_CERT_PUBKEY_HASH_RESERVED_PROP_ID: i32 = 8i32;
pub const XCN_CERT_ENHKEY_USAGE_PROP_ID: i32 = 9i32;
pub const XCN_CERT_CTL_USAGE_PROP_ID: i32 = 9i32;
pub const XCN_CERT_NEXT_UPDATE_LOCATION_PROP_ID: i32 = 10i32;
pub const XCN_CERT_FRIENDLY_NAME_PROP_ID: i32 = 11i32;
pub const XCN_CERT_PVK_FILE_PROP_ID: i32 = 12i32;
pub const XCN_CERT_DESCRIPTION_PROP_ID: i32 = 13i32;
pub const XCN_CERT_ACCESS_STATE_PROP_ID: i32 = 14i32;
pub const XCN_CERT_SIGNATURE_HASH_PROP_ID: i32 = 15i32;
pub const XCN_CERT_SMART_CARD_DATA_PROP_ID: i32 = 16i32;
pub const XCN_CERT_EFS_PROP_ID: i32 = 17i32;
pub const XCN_CERT_FORTEZZA_DATA_PROP_ID: i32 = 18i32;
pub const XCN_CERT_ARCHIVED_PROP_ID: i32 = 19i32;
pub const XCN_CERT_KEY_IDENTIFIER_PROP_ID: i32 = 20i32;
pub const XCN_CERT_AUTO_ENROLL_PROP_ID: i32 = 21i32;
pub const XCN_CERT_PUBKEY_ALG_PARA_PROP_ID: i32 = 22i32;
pub const XCN_CERT_CROSS_CERT_DIST_POINTS_PROP_ID: i32 = 23i32;
pub const XCN_CERT_ISSUER_PUBLIC_KEY_MD5_HASH_PROP_ID: i32 = 24i32;
pub const XCN_CERT_SUBJECT_PUBLIC_KEY_MD5_HASH_PROP_ID: i32 = 25i32;
pub const XCN_CERT_ENROLLMENT_PROP_ID: i32 = 26i32;
pub const XCN_CERT_DATE_STAMP_PROP_ID: i32 = 27i32;
pub const XCN_CERT_ISSUER_SERIAL_NUMBER_MD5_HASH_PROP_ID: i32 = 28i32;
pub const XCN_CERT_SUBJECT_NAME_MD5_HASH_PROP_ID: i32 = 29i32;
pub const XCN_CERT_EXTENDED_ERROR_INFO_PROP_ID: i32 = 30i32;
pub const XCN_CERT_RENEWAL_PROP_ID: i32 = 64i32;
pub const XCN_CERT_ARCHIVED_KEY_HASH_PROP_ID: i32 = 65i32;
pub const XCN_CERT_AUTO_ENROLL_RETRY_PROP_ID: i32 = 66i32;
pub const XCN_CERT_AIA_URL_RETRIEVED_PROP_ID: i32 = 67i32;
pub const XCN_CERT_AUTHORITY_INFO_ACCESS_PROP_ID: i32 = 68i32;
pub const XCN_CERT_BACKED_UP_PROP_ID: i32 = 69i32;
pub const XCN_CERT_OCSP_RESPONSE_PROP_ID: i32 = 70i32;
pub const XCN_CERT_REQUEST_ORIGINATOR_PROP_ID: i32 = 71i32;
pub const XCN_CERT_SOURCE_LOCATION_PROP_ID: i32 = 72i32;
pub const XCN_CERT_SOURCE_URL_PROP_ID: i32 = 73i32;
pub const XCN_CERT_NEW_KEY_PROP_ID: i32 = 74i32;
pub const XCN_CERT_OCSP_CACHE_PREFIX_PROP_ID: i32 = 75i32;
pub const XCN_CERT_SMART_CARD_ROOT_INFO_PROP_ID: i32 = 76i32;
pub const XCN_CERT_NO_AUTO_EXPIRE_CHECK_PROP_ID: i32 = 77i32;
pub const XCN_CERT_NCRYPT_KEY_HANDLE_PROP_ID: i32 = 78i32;
pub const XCN_CERT_HCRYPTPROV_OR_NCRYPT_KEY_HANDLE_PROP_ID: i32 = 79i32;
pub const XCN_CERT_SUBJECT_INFO_ACCESS_PROP_ID: i32 = 80i32;
pub const XCN_CERT_CA_OCSP_AUTHORITY_INFO_ACCESS_PROP_ID: i32 = 81i32;
pub const XCN_CERT_CA_DISABLE_CRL_PROP_ID: i32 = 82i32;
pub const XCN_CERT_ROOT_PROGRAM_CERT_POLICIES_PROP_ID: i32 = 83i32;
pub const XCN_CERT_ROOT_PROGRAM_NAME_CONSTRAINTS_PROP_ID: i32 = 84i32;
pub const XCN_CERT_SUBJECT_OCSP_AUTHORITY_INFO_ACCESS_PROP_ID: i32 = 85i32;
pub const XCN_CERT_SUBJECT_DISABLE_CRL_PROP_ID: i32 = 86i32;
pub const XCN_CERT_CEP_PROP_ID: i32 = 87i32;
pub const XCN_CERT_SIGN_HASH_CNG_ALG_PROP_ID: i32 = 89i32;
pub const XCN_CERT_SCARD_PIN_ID_PROP_ID: i32 = 90i32;
pub const XCN_CERT_SCARD_PIN_INFO_PROP_ID: i32 = 91i32;
pub const XCN_CERT_SUBJECT_PUB_KEY_BIT_LENGTH_PROP_ID: i32 = 92i32;
pub const XCN_CERT_PUB_KEY_CNG_ALG_BIT_LENGTH_PROP_ID: i32 = 93i32;
pub const XCN_CERT_ISSUER_PUB_KEY_BIT_LENGTH_PROP_ID: i32 = 94i32;
pub const XCN_CERT_ISSUER_CHAIN_SIGN_HASH_CNG_ALG_PROP_ID: i32 = 95i32;
pub const XCN_CERT_ISSUER_CHAIN_PUB_KEY_CNG_ALG_BIT_LENGTH_PROP_ID: i32 = 96i32;
pub const XCN_CERT_NO_EXPIRE_NOTIFICATION_PROP_ID: i32 = 97i32;
pub const XCN_CERT_AUTH_ROOT_SHA256_HASH_PROP_ID: i32 = 98i32;
pub const XCN_CERT_NCRYPT_KEY_HANDLE_TRANSFER_PROP_ID: i32 = 99i32;
pub const XCN_CERT_HCRYPTPROV_TRANSFER_PROP_ID: i32 = 100i32;
pub const XCN_CERT_SMART_CARD_READER_PROP_ID: i32 = 101i32;
pub const XCN_CERT_SEND_AS_TRUSTED_ISSUER_PROP_ID: i32 = 102i32;
pub const XCN_CERT_KEY_REPAIR_ATTEMPTED_PROP_ID: i32 = 103i32;
pub const XCN_CERT_DISALLOWED_FILETIME_PROP_ID: i32 = 104i32;
pub const XCN_CERT_ROOT_PROGRAM_CHAIN_POLICIES_PROP_ID: i32 = 105i32;
pub const XCN_CERT_SMART_CARD_READER_NON_REMOVABLE_PROP_ID: i32 = 106i32;
pub const XCN_CERT_SHA256_HASH_PROP_ID: i32 = 107i32;
pub const XCN_CERT_SCEP_SERVER_CERTS_PROP_ID: i32 = 108i32;
pub const XCN_CERT_SCEP_RA_SIGNATURE_CERT_PROP_ID: i32 = 109i32;
pub const XCN_CERT_SCEP_RA_ENCRYPTION_CERT_PROP_ID: i32 = 110i32;
pub const XCN_CERT_SCEP_CA_CERT_PROP_ID: i32 = 111i32;
pub const XCN_CERT_SCEP_SIGNER_CERT_PROP_ID: i32 = 112i32;
pub const XCN_CERT_SCEP_NONCE_PROP_ID: i32 = 113i32;
pub const XCN_CERT_SCEP_ENCRYPT_HASH_CNG_ALG_PROP_ID: i32 = 114i32;
pub const XCN_CERT_SCEP_FLAGS_PROP_ID: i32 = 115i32;
pub const XCN_CERT_SCEP_GUID_PROP_ID: i32 = 116i32;
pub const XCN_CERT_SERIALIZABLE_KEY_CONTEXT_PROP_ID: i32 = 117i32;
pub const XCN_CERT_ISOLATED_KEY_PROP_ID: i32 = 118i32;
pub const XCN_CERT_SERIAL_CHAIN_PROP_ID: i32 = 119i32;
pub const XCN_CERT_KEY_CLASSIFICATION_PROP_ID: i32 = 120i32;
pub const XCN_CERT_DISALLOWED_ENHKEY_USAGE_PROP_ID: i32 = 122i32;
pub const XCN_CERT_NONCOMPLIANT_ROOT_URL_PROP_ID: i32 = 123i32;
pub const XCN_CERT_PIN_SHA256_HASH_PROP_ID: i32 = 124i32;
pub const XCN_CERT_CLR_DELETE_KEY_PROP_ID: i32 = 125i32;
pub const XCN_CERT_NOT_BEFORE_FILETIME_PROP_ID: i32 = 126i32;
pub const XCN_CERT_CERT_NOT_BEFORE_ENHKEY_USAGE_PROP_ID: i32 = 127i32;
pub const XCN_CERT_FIRST_RESERVED_PROP_ID: i32 = 128i32;
pub const XCN_CERT_LAST_RESERVED_PROP_ID: i32 = 32767i32;
pub const XCN_CERT_FIRST_USER_PROP_ID: i32 = 32768i32;
pub const XCN_CERT_LAST_USER_PROP_ID: i32 = 65535i32;
pub const XCN_CERT_STORE_LOCALIZED_NAME_PROP_ID: i32 = 4096i32;
#[repr(C)]
pub struct CERTTRANSBLOB {
    pub cb: u32,
    pub pb: *mut u8,
}
impl ::core::marker::Copy for CERTTRANSBLOB {}
impl ::core::clone::Clone for CERTTRANSBLOB {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct CERTVIEWRESTRICTION {
    pub ColumnIndex: u32,
    pub SeekOperator: i32,
    pub SortOrder: i32,
    pub pbValue: *mut u8,
    pub cbValue: u32,
}
impl ::core::marker::Copy for CERTVIEWRESTRICTION {}
impl ::core::clone::Clone for CERTVIEWRESTRICTION {
    fn clone(&self) -> Self {
        *self
    }
}
pub const CERT_ALT_NAME_RFC822_NAME: u32 = 2u32;
pub const CERT_ALT_NAME_DNS_NAME: u32 = 3u32;
pub const CERT_ALT_NAME_URL: u32 = 7u32;
pub const CERT_ALT_NAME_REGISTERED_ID: u32 = 9u32;
pub const CERT_ALT_NAME_DIRECTORY_NAME: u32 = 5u32;
pub const CERT_ALT_NAME_IP_ADDRESS: u32 = 8u32;
pub const CERT_ALT_NAME_OTHER_NAME: u32 = 1u32;
pub const XECR_CMC: u32 = 3u32;
pub const XECR_PKCS10_V1_5: u32 = 4u32;
pub const XECR_PKCS10_V2_0: u32 = 1u32;
pub const XECR_PKCS7: u32 = 2u32;
pub const CDR_EXPIRED: u32 = 1u32;
pub const CDR_REQUEST_LAST_CHANGED: u32 = 2u32;
pub const EXITEVENT_CERTDENIED: u32 = 4u32;
pub const EXITEVENT_CERTISSUED: u32 = 1u32;
pub const EXITEVENT_CERTPENDING: u32 = 2u32;
pub const EXITEVENT_CERTRETRIEVEPENDING: u32 = 16u32;
pub const EXITEVENT_CERTREVOKED: u32 = 8u32;
pub const EXITEVENT_CRLISSUED: u32 = 32u32;
pub const EXITEVENT_SHUTDOWN: u32 = 64u32;
pub const CC_DEFAULTCONFIG: u32 = 0u32;
pub const CC_FIRSTCONFIG: u32 = 2u32;
pub const CC_LOCALACTIVECONFIG: u32 = 4u32;
pub const CC_LOCALCONFIG: u32 = 3u32;
pub const CC_UIPICKCONFIG: u32 = 1u32;
pub const CC_UIPICKCONFIGSKIPLOCALCA_: u32 = 5u32;
pub const CR_IN_BASE64HEADER: u32 = 0u32;
pub const CR_IN_BASE64: u32 = 1u32;
pub const CR_IN_BINARY: u32 = 2u32;
pub const PROPTYPE_BINARY: u32 = 3u32;
pub const PROPTYPE_DATE: u32 = 2u32;
pub const PROPTYPE_LONG: u32 = 1u32;
pub const PROPTYPE_STRING: u32 = 4u32;
pub const CR_OUT_BASE64HEADER: u32 = 0u32;
pub const CR_OUT_BASE64: u32 = 1u32;
pub const CR_OUT_BINARY: u32 = 2u32;
pub const CV_COLUMN_LOG_DEFAULT: i32 = -2i32;
pub const CV_COLUMN_LOG_FAILED_DEFAULT: i32 = -3i32;
pub const CV_COLUMN_QUEUE_DEFAULT: i32 = -1i32;
pub const CVR_SEEK_EQ: u32 = 1u32;
pub const CVR_SEEK_LE: u32 = 4u32;
pub const CVR_SEEK_LT: u32 = 2u32;
pub const CVR_SEEK_GE: u32 = 8u32;
pub const CVR_SEEK_GT: u32 = 16u32;
pub const CEnroll: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1140388489, data2: 31264, data3: 4560, data4: [143, 6, 0, 192, 79, 194, 149, 225] };
pub const CEnroll2: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 309762276, data2: 59184, data3: 20060, data4: [162, 177, 33, 73, 10, 112, 200, 161] };
pub const CMM_READONLY: u32 = 2u32;
pub const CMM_REFRESHONLY: u32 = 1u32;
pub const CObjectId: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2286821376, data2: 8573, data3: 4570, data4: [178, 164, 0, 14, 123, 187, 43, 9] };
pub const CObjectIds: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2286821377, data2: 8573, data3: 4570, data4: [178, 164, 0, 14, 123, 187, 43, 9] };
pub const CPF_BADURL_ERROR: u32 = 32u32;
pub const CPF_BASE: u32 = 1u32;
pub const CPF_CASTORE_ERROR: u32 = 16u32;
pub const CPF_COMPLETE: u32 = 4u32;
pub const CPF_DELTA: u32 = 2u32;
pub const CPF_FILE_ERROR: u32 = 512u32;
pub const CPF_FTP_ERROR: u32 = 1024u32;
pub const CPF_HTTP_ERROR: u32 = 2048u32;
pub const CPF_LDAP_ERROR: u32 = 256u32;
pub const CPF_MANUAL: u32 = 64u32;
pub const CPF_POSTPONED_BASE_FILE_ERROR: u32 = 8192u32;
pub const CPF_POSTPONED_BASE_LDAP_ERROR: u32 = 4096u32;
pub const CPF_SHADOW: u32 = 8u32;
pub const CPF_SIGNATURE_ERROR: u32 = 128u32;
pub const CPolicyQualifier: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2286821404, data2: 8573, data3: 4570, data4: [178, 164, 0, 14, 123, 187, 43, 9] };
pub const CPolicyQualifiers: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2286821405, data2: 8573, data3: 4570, data4: [178, 164, 0, 14, 123, 187, 43, 9] };
pub const CRLF_ALLOW_REQUEST_ATTRIBUTE_SUBJECT: u32 = 65536u32;
pub const CRLF_BUILD_ROOTCA_CRLENTRIES_BASEDONKEY: u32 = 2097152u32;
pub const CRLF_CRLNUMBER_CRITICAL: u32 = 4u32;
pub const CRLF_DELETE_EXPIRED_CRLS: u32 = 2u32;
pub const CRLF_DELTA_USE_OLDEST_UNEXPIRED_BASE: u32 = 1u32;
pub const CRLF_DISABLE_CHAIN_VERIFICATION: u32 = 1048576u32;
pub const CRLF_DISABLE_RDN_REORDER: u32 = 2048u32;
pub const CRLF_DISABLE_ROOT_CROSS_CERTS: u32 = 4096u32;
pub const CRLF_ENFORCE_ENROLLMENT_AGENT: u32 = 1024u32;
pub const CRLF_IGNORE_CROSS_CERT_TRUST_ERROR: u32 = 256u32;
pub const CRLF_IGNORE_INVALID_POLICIES: u32 = 16u32;
pub const CRLF_IGNORE_UNKNOWN_CMC_ATTRIBUTES: u32 = 128u32;
pub const CRLF_LOG_FULL_RESPONSE: u32 = 8192u32;
pub const CRLF_PRESERVE_EXPIRED_CA_CERTS: u32 = 262144u32;
pub const CRLF_PRESERVE_REVOKED_CA_CERTS: u32 = 524288u32;
pub const CRLF_PUBLISH_EXPIRED_CERT_CRLS: u32 = 512u32;
pub const CRLF_REBUILD_MODIFIED_SUBJECT_ONLY: u32 = 32u32;
pub const CRLF_REVCHECK_IGNORE_NOREVCHECK: u32 = 131072u32;
pub const CRLF_REVCHECK_IGNORE_OFFLINE: u32 = 8u32;
pub const CRLF_SAVE_FAILED_CERTS: u32 = 64u32;
pub const CRLF_USE_CROSS_CERT_TEMPLATE: u32 = 32768u32;
pub const CRLF_USE_XCHG_CERT_TEMPLATE: u32 = 16384u32;
pub const XCN_CRL_REASON_UNSPECIFIED: i32 = 0i32;
pub const XCN_CRL_REASON_KEY_COMPROMISE: i32 = 1i32;
pub const XCN_CRL_REASON_CA_COMPROMISE: i32 = 2i32;
pub const XCN_CRL_REASON_AFFILIATION_CHANGED: i32 = 3i32;
pub const XCN_CRL_REASON_SUPERSEDED: i32 = 4i32;
pub const XCN_CRL_REASON_CESSATION_OF_OPERATION: i32 = 5i32;
pub const XCN_CRL_REASON_CERTIFICATE_HOLD: i32 = 6i32;
pub const XCN_CRL_REASON_REMOVE_FROM_CRL: i32 = 8i32;
pub const XCN_CRL_REASON_PRIVILEGE_WITHDRAWN: i32 = 9i32;
pub const XCN_CRL_REASON_AA_COMPROMISE: i32 = 10i32;
pub const CRYPT_ENUM_ALL_PROVIDERS: u32 = 1u32;
pub const CR_DISP_DENIED: u32 = 2u32;
pub const CR_DISP_ERROR: u32 = 1u32;
pub const CR_DISP_INCOMPLETE: u32 = 0u32;
pub const CR_DISP_ISSUED: u32 = 3u32;
pub const CR_DISP_ISSUED_OUT_OF_BAND: u32 = 4u32;
pub const CR_DISP_UNDER_SUBMISSION: u32 = 5u32;
pub const CR_DISP_REVOKED: u32 = 6u32;
pub const CR_FLG_CACROSSCERT: u32 = 128u32;
pub const CR_FLG_CAXCHGCERT: u32 = 8u32;
pub const CR_FLG_CHALLENGEPENDING: u32 = 1024u32;
pub const CR_FLG_CHALLENGESATISFIED: u32 = 2048u32;
pub const CR_FLG_DEFINEDCACERT: u32 = 512u32;
pub const CR_FLG_ENFORCEUTF8: u32 = 256u32;
pub const CR_FLG_ENROLLONBEHALFOF: u32 = 16u32;
pub const CR_FLG_FORCETELETEX: u32 = 1u32;
pub const CR_FLG_FORCEUTF8: u32 = 4u32;
pub const CR_FLG_PUBLISHERROR: u32 = 2147483648u32;
pub const CR_FLG_RENEWAL: u32 = 2u32;
pub const CR_FLG_SUBJECTUNMODIFIED: u32 = 32u32;
pub const CR_FLG_TRUSTEKCERT: u32 = 8192u32;
pub const CR_FLG_TRUSTEKKEY: u32 = 16384u32;
pub const CR_FLG_TRUSTONUSE: u32 = 4096u32;
pub const CR_FLG_VALIDENCRYPTEDKEYHASH: u32 = 64u32;
pub const CR_GEMT_DEFAULT: u32 = 0u32;
pub const CR_GEMT_HRESULT_STRING: u32 = 1u32;
pub const CR_GEMT_HTTP_ERROR: u32 = 2u32;
pub const CR_IN_CERTIFICATETRANSPARENCY: u32 = 67108864u32;
pub const CR_IN_CHALLENGERESPONSE: u32 = 1280u32;
pub const CR_IN_CLIENTIDNONE: u32 = 4194304u32;
pub const CR_IN_CMC: u32 = 1024u32;
pub const CR_IN_CONNECTONLY: u32 = 8388608u32;
pub const CR_IN_CRLS: u32 = 524288u32;
pub const CR_IN_ENCODEANY: u32 = 255u32;
pub const CR_IN_ENCODEMASK: u32 = 255u32;
pub const CR_IN_FORMATANY: u32 = 0u32;
pub const CR_IN_FORMATMASK: u32 = 65280u32;
pub const CR_IN_FULLRESPONSE: u32 = 262144u32;
pub const CR_IN_HTTP: u32 = 196608u32;
pub const CR_IN_KEYGEN: u32 = 512u32;
pub const CR_IN_MACHINE: u32 = 1048576u32;
pub const CR_IN_PKCS10: u32 = 256u32;
pub const CR_IN_PKCS7: u32 = 768u32;
pub const CR_IN_RETURNCHALLENGE: u32 = 16777216u32;
pub const CR_IN_ROBO: u32 = 2097152u32;
pub const CR_IN_RPC: u32 = 131072u32;
pub const CR_IN_SCEP: u32 = 65536u32;
pub const CR_IN_SCEPPOST: u32 = 33554432u32;
pub const CR_IN_SIGNEDCERTIFICATETIMESTAMPLIST: u32 = 1536u32;
pub const CR_OUT_BASE64REQUESTHEADER: u32 = 3u32;
pub const CR_OUT_BASE64X509CRLHEADER: u32 = 9u32;
pub const CR_OUT_CHAIN: u32 = 256u32;
pub const CR_OUT_CRLS: u32 = 512u32;
pub const CR_OUT_ENCODEMASK: u32 = 255u32;
pub const CR_OUT_HEX: u32 = 4u32;
pub const CR_OUT_HEXADDR: u32 = 10u32;
pub const CR_OUT_HEXASCII: u32 = 5u32;
pub const CR_OUT_HEXASCIIADDR: u32 = 11u32;
pub const CR_OUT_HEXRAW: u32 = 12u32;
pub const CR_OUT_NOCR: u32 = 2147483648u32;
pub const CR_OUT_NOCRLF: u32 = 1073741824u32;
pub const CR_PROP_ADVANCEDSERVER: u32 = 28u32;
pub const CR_PROP_BASECRL: u32 = 17u32;
pub const CR_PROP_BASECRLPUBLISHSTATUS: u32 = 30u32;
pub const CR_PROP_CABACKWARDCROSSCERT: u32 = 36u32;
pub const CR_PROP_CABACKWARDCROSSCERTSTATE: u32 = 38u32;
pub const CR_PROP_CACERTSTATE: u32 = 19u32;
pub const CR_PROP_CACERTSTATUSCODE: u32 = 34u32;
pub const CR_PROP_CACERTVERSION: u32 = 39u32;
pub const CR_PROP_CAFORWARDCROSSCERT: u32 = 35u32;
pub const CR_PROP_CAFORWARDCROSSCERTSTATE: u32 = 37u32;
pub const CR_PROP_CANAME: u32 = 6u32;
pub const CR_PROP_CAPROPIDMAX: u32 = 21u32;
pub const CR_PROP_CASIGCERT: u32 = 12u32;
pub const CR_PROP_CASIGCERTCHAIN: u32 = 13u32;
pub const CR_PROP_CASIGCERTCOUNT: u32 = 11u32;
pub const CR_PROP_CASIGCERTCRLCHAIN: u32 = 32u32;
pub const CR_PROP_CATYPE: u32 = 10u32;
pub const CR_PROP_CAXCHGCERT: u32 = 15u32;
pub const CR_PROP_CAXCHGCERTCHAIN: u32 = 16u32;
pub const CR_PROP_CAXCHGCERTCOUNT: u32 = 14u32;
pub const CR_PROP_CAXCHGCERTCRLCHAIN: u32 = 33u32;
pub const CR_PROP_CERTAIAOCSPURLS: u32 = 43u32;
pub const CR_PROP_CERTAIAURLS: u32 = 42u32;
pub const CR_PROP_CERTCDPURLS: u32 = 41u32;
pub const CR_PROP_CRLSTATE: u32 = 20u32;
pub const CR_PROP_DELTACRL: u32 = 18u32;
pub const CR_PROP_DELTACRLPUBLISHSTATUS: u32 = 31u32;
pub const CR_PROP_DNSNAME: u32 = 22u32;
pub const CR_PROP_EXITCOUNT: u32 = 3u32;
pub const CR_PROP_EXITDESCRIPTION: u32 = 4u32;
pub const CR_PROP_FILEVERSION: u32 = 1u32;
pub const CR_PROP_KRACERT: u32 = 26u32;
pub const CR_PROP_KRACERTCOUNT: u32 = 25u32;
pub const CR_PROP_KRACERTSTATE: u32 = 27u32;
pub const CR_PROP_KRACERTUSEDCOUNT: u32 = 24u32;
pub const CR_PROP_LOCALENAME: u32 = 44u32;
pub const CR_PROP_NONE: u32 = 0u32;
pub const CR_PROP_PARENTCA: u32 = 9u32;
pub const CR_PROP_POLICYDESCRIPTION: u32 = 5u32;
pub const CR_PROP_PRODUCTVERSION: u32 = 2u32;
pub const CR_PROP_ROLESEPARATIONENABLED: u32 = 23u32;
pub const CR_PROP_SANITIZEDCANAME: u32 = 7u32;
pub const CR_PROP_SANITIZEDCASHORTNAME: u32 = 40u32;
pub const CR_PROP_SCEPMAX: u32 = 1002u32;
pub const CR_PROP_SCEPMIN: u32 = 1000u32;
pub const CR_PROP_SCEPSERVERCAPABILITIES: u32 = 1001u32;
pub const CR_PROP_SCEPSERVERCERTS: u32 = 1000u32;
pub const CR_PROP_SCEPSERVERCERTSCHAIN: u32 = 1002u32;
pub const CR_PROP_SHAREDFOLDER: u32 = 8u32;
pub const CR_PROP_SUBJECTTEMPLATE_OIDS: u32 = 45u32;
pub const CR_PROP_TEMPLATES: u32 = 29u32;
pub const CSBACKUP_DISABLE_INCREMENTAL: u32 = 4294967295u32;
pub const CSBACKUP_TYPE_FULL: u32 = 1u32;
pub const CSBACKUP_TYPE_LOGS_ONLY: u32 = 2u32;
pub const CSBACKUP_TYPE_MASK: u32 = 3u32;
pub const CSBFT_DATABASE_DIRECTORY: u32 = 64u32;
pub const CSBFT_DIRECTORY: u32 = 128u32;
pub const CSBFT_LOG_DIRECTORY: u32 = 32u32;
pub const CSCONTROL_RESTART: u64 = 3u64;
pub const CSCONTROL_SHUTDOWN: u64 = 1u64;
pub const CSCONTROL_SUSPEND: u64 = 2u64;
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct CSEDB_RSTMAPW {
    pub pwszDatabaseName: super::super::super::Foundation::PWSTR,
    pub pwszNewDatabaseName: super::super::super::Foundation::PWSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for CSEDB_RSTMAPW {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for CSEDB_RSTMAPW {
    fn clone(&self) -> Self {
        *self
    }
}
pub const CSRESTORE_TYPE_CATCHUP: u32 = 4u32;
pub const CSRESTORE_TYPE_FULL: u32 = 1u32;
pub const CSRESTORE_TYPE_MASK: u32 = 5u32;
pub const CSRESTORE_TYPE_ONLINE: u32 = 2u32;
pub const CSURL_ADDTOCERTCDP: u32 = 2u32;
pub const CSURL_ADDTOCERTOCSP: u32 = 32u32;
pub const CSURL_ADDTOCRLCDP: u32 = 8u32;
pub const CSURL_ADDTOFRESHESTCRL: u32 = 4u32;
pub const CSURL_ADDTOIDP: u32 = 128u32;
pub const CSURL_PUBLISHRETRY: u32 = 16u32;
pub const CSURL_SERVERPUBLISH: u32 = 1u32;
pub const CSURL_SERVERPUBLISHDELTA: u32 = 64u32;
pub const CSVER_MAJOR: u32 = 7u32;
pub const CSVER_MAJOR_LONGHORN: u32 = 3u32;
pub const CSVER_MAJOR_THRESHOLD: u32 = 7u32;
pub const CSVER_MAJOR_WHISTLER: u32 = 2u32;
pub const CSVER_MAJOR_WIN2K: u32 = 1u32;
pub const CSVER_MAJOR_WIN7: u32 = 4u32;
pub const CSVER_MAJOR_WIN8: u32 = 5u32;
pub const CSVER_MAJOR_WINBLUE: u32 = 6u32;
pub const CSVER_MINOR: u32 = 1u32;
pub const CSVER_MINOR_LONGHORN_BETA1: u32 = 1u32;
pub const CSVER_MINOR_THRESHOLD: u32 = 1u32;
pub const CSVER_MINOR_WHISTLER_BETA2: u32 = 1u32;
pub const CSVER_MINOR_WHISTLER_BETA3: u32 = 2u32;
pub const CSVER_MINOR_WIN2K: u32 = 1u32;
pub const CSVER_MINOR_WIN7: u32 = 1u32;
pub const CSVER_MINOR_WIN8: u32 = 1u32;
pub const CSVER_MINOR_WINBLUE: u32 = 1u32;
pub const CSignerCertificate: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2286821437, data2: 8573, data3: 4570, data4: [178, 164, 0, 14, 123, 187, 43, 9] };
pub const CSmimeCapabilities: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2286821402, data2: 8573, data3: 4570, data4: [178, 164, 0, 14, 123, 187, 43, 9] };
pub const CSmimeCapability: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2286821401, data2: 8573, data3: 4570, data4: [178, 164, 0, 14, 123, 187, 43, 9] };
pub const CVIEWAGEMINUTESDEFAULT: u32 = 16u32;
pub const CVRC_COLUMN_SCHEMA: u32 = 0u32;
pub const CVRC_COLUMN_RESULT: u32 = 1u32;
pub const CVRC_COLUMN_VALUE: u32 = 2u32;
pub const CVRC_COLUMN_MASK: u32 = 4095u32;
pub const CVRC_TABLE_ATTRIBUTES: u32 = 16384u32;
pub const CVRC_TABLE_CRL: u32 = 20480u32;
pub const CVRC_TABLE_EXTENSIONS: u32 = 12288u32;
pub const CVRC_TABLE_REQCERT: u32 = 0u32;
pub const CVRC_TABLE_MASK: u32 = 61440u32;
pub const CVRC_TABLE_SHIFT: u32 = 12u32;
pub const CVR_SEEK_MASK: u32 = 255u32;
pub const CVR_SEEK_NODELTA: u32 = 4096u32;
pub const CVR_SEEK_NONE: u32 = 0u32;
pub const CVR_SORT_ASCEND: u32 = 1u32;
pub const CVR_SORT_DESCEND: u32 = 2u32;
pub const CVR_SORT_NONE: u32 = 0u32;
pub const CV_COLUMN_ATTRIBUTE_DEFAULT: i32 = -5i32;
pub const CV_COLUMN_CRL_DEFAULT: i32 = -6i32;
pub const CV_COLUMN_EXTENSION_DEFAULT: i32 = -4i32;
pub const CV_COLUMN_LOG_REVOKED_DEFAULT: i32 = -7i32;
pub const CV_OUT_ENCODEMASK: u32 = 255u32;
pub const CV_OUT_HEXRAW: u32 = 12u32;
pub const CV_OUT_NOCR: u32 = 2147483648u32;
pub const CV_OUT_NOCRLF: u32 = 1073741824u32;
pub const CX500DistinguishedName: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2286821379, data2: 8573, data3: 4570, data4: [178, 164, 0, 14, 123, 187, 43, 9] };
pub const CX509Attribute: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2286821410, data2: 8573, data3: 4570, data4: [178, 164, 0, 14, 123, 187, 43, 9] };
pub const CX509AttributeArchiveKey: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2286821415, data2: 8573, data3: 4570, data4: [178, 164, 0, 14, 123, 187, 43, 9] };
pub const CX509AttributeArchiveKeyHash: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2286821416, data2: 8573, data3: 4570, data4: [178, 164, 0, 14, 123, 187, 43, 9] };
pub const CX509AttributeClientId: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2286821413, data2: 8573, data3: 4570, data4: [178, 164, 0, 14, 123, 187, 43, 9] };
pub const CX509AttributeCspProvider: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2286821419, data2: 8573, data3: 4570, data4: [178, 164, 0, 14, 123, 187, 43, 9] };
pub const CX509AttributeExtensions: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2286821412, data2: 8573, data3: 4570, data4: [178, 164, 0, 14, 123, 187, 43, 9] };
pub const CX509AttributeOSVersion: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2286821418, data2: 8573, data3: 4570, data4: [178, 164, 0, 14, 123, 187, 43, 9] };
pub const CX509AttributeRenewalCertificate: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2286821414, data2: 8573, data3: 4570, data4: [178, 164, 0, 14, 123, 187, 43, 9] };
pub const CX509Attributes: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2286821411, data2: 8573, data3: 4570, data4: [178, 164, 0, 14, 123, 187, 43, 9] };
pub const CX509CertificateRequestCertificate: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2286821443, data2: 8573, data3: 4570, data4: [178, 164, 0, 14, 123, 187, 43, 9] };
pub const CX509CertificateRequestCmc: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2286821445, data2: 8573, data3: 4570, data4: [178, 164, 0, 14, 123, 187, 43, 9] };
pub const CX509CertificateRequestPkcs10: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2286821442, data2: 8573, data3: 4570, data4: [178, 164, 0, 14, 123, 187, 43, 9] };
pub const CX509CertificateRequestPkcs7: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2286821444, data2: 8573, data3: 4570, data4: [178, 164, 0, 14, 123, 187, 43, 9] };
pub const CX509CertificateRevocationList: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2286821472, data2: 8573, data3: 4570, data4: [178, 164, 0, 14, 123, 187, 43, 9] };
pub const CX509CertificateRevocationListEntries: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2286821471, data2: 8573, data3: 4570, data4: [178, 164, 0, 14, 123, 187, 43, 9] };
pub const CX509CertificateRevocationListEntry: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2286821470, data2: 8573, data3: 4570, data4: [178, 164, 0, 14, 123, 187, 43, 9] };
pub const CX509CertificateTemplateADWritable: ::windows_sys::core::GUID = ::windows_sys::core::GUID {
    data1: 2201412387,
    data2: 11882,
    data3: 18948,
    data4: [147, 124, 84, 143, 104, 24, 57, 179],
};
pub const CX509EndorsementKey: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 295852573, data2: 47523, data3: 20189, data4: [175, 131, 59, 89, 173, 190, 211, 97] };
pub const CX509Enrollment: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2286821446, data2: 8573, data3: 4570, data4: [178, 164, 0, 14, 123, 187, 43, 9] };
pub const CX509EnrollmentHelper: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2286821456, data2: 8573, data3: 4570, data4: [178, 164, 0, 14, 123, 187, 43, 9] };
pub const CX509EnrollmentPolicyActiveDirectory: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2448658471, data2: 8575, data3: 4570, data4: [178, 164, 0, 14, 123, 187, 43, 9] };
pub const CX509EnrollmentPolicyWebService: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2448658472, data2: 8575, data3: 4570, data4: [178, 164, 0, 14, 123, 187, 43, 9] };
pub const CX509EnrollmentWebClassFactory: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2286821449, data2: 8573, data3: 4570, data4: [178, 164, 0, 14, 123, 187, 43, 9] };
pub const CX509Extension: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2286821389, data2: 8573, data3: 4570, data4: [178, 164, 0, 14, 123, 187, 43, 9] };
pub const CX509ExtensionAlternativeNames: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2286821397, data2: 8573, data3: 4570, data4: [178, 164, 0, 14, 123, 187, 43, 9] };
pub const CX509ExtensionAuthorityKeyIdentifier: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2286821400, data2: 8573, data3: 4570, data4: [178, 164, 0, 14, 123, 187, 43, 9] };
pub const CX509ExtensionBasicConstraints: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2286821398, data2: 8573, data3: 4570, data4: [178, 164, 0, 14, 123, 187, 43, 9] };
pub const CX509ExtensionCertificatePolicies: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2286821408, data2: 8573, data3: 4570, data4: [178, 164, 0, 14, 123, 187, 43, 9] };
pub const CX509ExtensionEnhancedKeyUsage: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2286821392, data2: 8573, data3: 4570, data4: [178, 164, 0, 14, 123, 187, 43, 9] };
pub const CX509ExtensionKeyUsage: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2286821391, data2: 8573, data3: 4570, data4: [178, 164, 0, 14, 123, 187, 43, 9] };
pub const CX509ExtensionMSApplicationPolicies: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2286821409, data2: 8573, data3: 4570, data4: [178, 164, 0, 14, 123, 187, 43, 9] };
pub const CX509ExtensionSmimeCapabilities: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2286821403, data2: 8573, data3: 4570, data4: [178, 164, 0, 14, 123, 187, 43, 9] };
pub const CX509ExtensionSubjectKeyIdentifier: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2286821399, data2: 8573, data3: 4570, data4: [178, 164, 0, 14, 123, 187, 43, 9] };
pub const CX509ExtensionTemplate: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2286821394, data2: 8573, data3: 4570, data4: [178, 164, 0, 14, 123, 187, 43, 9] };
pub const CX509ExtensionTemplateName: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2286821393, data2: 8573, data3: 4570, data4: [178, 164, 0, 14, 123, 187, 43, 9] };
pub const CX509Extensions: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2286821390, data2: 8573, data3: 4570, data4: [178, 164, 0, 14, 123, 187, 43, 9] };
pub const CX509MachineEnrollmentFactory: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2286821457, data2: 8573, data3: 4570, data4: [178, 164, 0, 14, 123, 187, 43, 9] };
pub const CX509NameValuePair: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2286821439, data2: 8573, data3: 4570, data4: [178, 164, 0, 14, 123, 187, 43, 9] };
pub const CX509PolicyServerListManager: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2448658473, data2: 8575, data3: 4570, data4: [178, 164, 0, 14, 123, 187, 43, 9] };
pub const CX509PolicyServerUrl: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2448658474, data2: 8575, data3: 4570, data4: [178, 164, 0, 14, 123, 187, 43, 9] };
pub const CX509PrivateKey: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2286821388, data2: 8573, data3: 4570, data4: [178, 164, 0, 14, 123, 187, 43, 9] };
pub const CX509PublicKey: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2286821387, data2: 8573, data3: 4570, data4: [178, 164, 0, 14, 123, 187, 43, 9] };
pub const CX509SCEPEnrollment: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2286821473, data2: 8573, data3: 4570, data4: [178, 164, 0, 14, 123, 187, 43, 9] };
pub const CX509SCEPEnrollmentHelper: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2286821474, data2: 8573, data3: 4570, data4: [178, 164, 0, 14, 123, 187, 43, 9] };
pub const CommitFlagSaveTemplateGenerateOID: i32 = 1i32;
pub const CommitFlagSaveTemplateUseCurrentOID: i32 = 2i32;
pub const CommitFlagSaveTemplateOverwrite: i32 = 3i32;
pub const CommitFlagDeleteTemplate: i32 = 4i32;
pub const DBFLAGS_CHECKPOINTDEPTH60MB: u32 = 32u32;
pub const DBFLAGS_CIRCULARLOGGING: u32 = 4u32;
pub const DBFLAGS_CREATEIFNEEDED: u32 = 2u32;
pub const DBFLAGS_DISABLESNAPSHOTBACKUP: u32 = 1024u32;
pub const DBFLAGS_ENABLEVOLATILEREQUESTS: u32 = 2048u32;
pub const DBFLAGS_LAZYFLUSH: u32 = 8u32;
pub const DBFLAGS_LOGBUFFERSHUGE: u32 = 128u32;
pub const DBFLAGS_LOGBUFFERSLARGE: u32 = 64u32;
pub const DBFLAGS_LOGFILESIZE16MB: u32 = 256u32;
pub const DBFLAGS_MAXCACHESIZEX100: u32 = 16u32;
pub const DBFLAGS_MULTITHREADTRANSACTIONS: u32 = 512u32;
pub const DBFLAGS_READONLY: u32 = 1u32;
pub const DBSESSIONCOUNTDEFAULT: u32 = 100u32;
pub const DB_DISP_ACTIVE: u32 = 8u32;
pub const DB_DISP_CA_CERT: u32 = 15u32;
pub const DB_DISP_CA_CERT_CHAIN: u32 = 16u32;
pub const DB_DISP_DENIED: u32 = 31u32;
pub const DB_DISP_ERROR: u32 = 30u32;
pub const DB_DISP_FOREIGN: u32 = 12u32;
pub const DB_DISP_ISSUED: u32 = 20u32;
pub const DB_DISP_KRA_CERT: u32 = 17u32;
pub const DB_DISP_LOG_FAILED_MIN: u32 = 30u32;
pub const DB_DISP_LOG_MIN: u32 = 20u32;
pub const DB_DISP_PENDING: u32 = 9u32;
pub const DB_DISP_QUEUE_MAX: u32 = 9u32;
pub const DB_DISP_REVOKED: u32 = 21u32;
pub const DelayRetryUnknown: i32 = 0i32;
pub const DelayRetryNone: i32 = 1i32;
pub const DelayRetryShort: i32 = 2i32;
pub const DelayRetryLong: i32 = 3i32;
pub const DelayRetrySuccess: i32 = 4i32;
pub const DelayRetryPastSuccess: i32 = 5i32;
pub const EANR_SUPPRESS_IA5CONVERSION: u32 = 2147483648u32;
pub const EAN_NAMEOBJECTID: u32 = 2147483648u32;
pub const EDITF_ADDOLDCERTTYPE: u32 = 16u32;
pub const EDITF_ADDOLDKEYUSAGE: u32 = 8u32;
pub const EDITF_ATTRIBUTECA: u32 = 512u32;
pub const EDITF_ATTRIBUTEEKU: u32 = 32768u32;
pub const EDITF_ATTRIBUTEENDDATE: u32 = 32u32;
pub const EDITF_ATTRIBUTESUBJECTALTNAME2: u32 = 262144u32;
pub const EDITF_AUDITCERTTEMPLATELOAD: u32 = 2097152u32;
pub const EDITF_BASICCONSTRAINTSCA: u32 = 128u32;
pub const EDITF_BASICCONSTRAINTSCRITICAL: u32 = 64u32;
pub const EDITF_DISABLEEXTENSIONLIST: u32 = 4u32;
pub const EDITF_DISABLELDAPPACKAGELIST: u32 = 8388608u32;
pub const EDITF_DISABLEOLDOSCNUPN: u32 = 4194304u32;
pub const EDITF_EMAILOPTIONAL: u32 = 131072u32;
pub const EDITF_ENABLEAKICRITICAL: u32 = 8192u32;
pub const EDITF_ENABLEAKIISSUERNAME: u32 = 2048u32;
pub const EDITF_ENABLEAKIISSUERSERIAL: u32 = 4096u32;
pub const EDITF_ENABLEAKIKEYID: u32 = 256u32;
pub const EDITF_ENABLECHASECLIENTDC: u32 = 1048576u32;
pub const EDITF_ENABLEDEFAULTSMIME: u32 = 65536u32;
pub const EDITF_ENABLEKEYENCIPHERMENTCACERT: u32 = 134217728u32;
pub const EDITF_ENABLELDAPREFERRALS: u32 = 524288u32;
pub const EDITF_ENABLEOCSPREVNOCHECK: u32 = 33554432u32;
pub const EDITF_ENABLERENEWONBEHALFOF: u32 = 67108864u32;
pub const EDITF_ENABLEREQUESTEXTENSIONS: u32 = 1u32;
pub const EDITF_ENABLEUPNMAP: u32 = 16777216u32;
pub const EDITF_IGNOREREQUESTERGROUP: u32 = 1024u32;
pub const EDITF_REQUESTEXTENSIONLIST: u32 = 2u32;
pub const EDITF_SERVERUPGRADED: u32 = 16384u32;
pub const ENUMEXT_OBJECTID: u32 = 1u32;
pub const ENUM_ENTERPRISE_ROOTCA: i32 = 0i32;
pub const ENUM_ENTERPRISE_SUBCA: i32 = 1i32;
pub const ENUM_STANDALONE_ROOTCA: i32 = 3i32;
pub const ENUM_STANDALONE_SUBCA: i32 = 4i32;
pub const ENUM_UNKNOWN_CA: i32 = 5i32;
pub const CV_OUT_BASE64: u32 = 1u32;
pub const CV_OUT_BASE64HEADER: u32 = 0u32;
pub const CV_OUT_BASE64REQUESTHEADER: u32 = 3u32;
pub const CV_OUT_BASE64X509CRLHEADER: u32 = 9u32;
pub const CV_OUT_BINARY: u32 = 2u32;
pub const CV_OUT_HEX: u32 = 4u32;
pub const CV_OUT_HEXADDR: u32 = 10u32;
pub const CV_OUT_HEXASCII: u32 = 5u32;
pub const CV_OUT_HEXASCIIADDR: u32 = 11u32;
pub const EXITEVENT_CERTIMPORTED: u32 = 512u32;
pub const EXITEVENT_INVALID: u32 = 0u32;
pub const EXITEVENT_STARTUP: u32 = 128u32;
pub const EXITPUB_ACTIVEDIRECTORY: u32 = 2u32;
pub const EXITPUB_DEFAULT_ENTERPRISE: u32 = 2u32;
pub const EXITPUB_DEFAULT_STANDALONE: u32 = 1u32;
pub const EXITPUB_FILE: u32 = 1u32;
pub const EXITPUB_REMOVEOLDCERTS: u32 = 16u32;
pub const EXTENSION_CRITICAL_FLAG: u32 = 1u32;
pub const EXTENSION_DELETE_FLAG: u32 = 4u32;
pub const EXTENSION_DISABLE_FLAG: u32 = 2u32;
pub const EXTENSION_ORIGIN_ADMIN: u32 = 196608u32;
pub const EXTENSION_ORIGIN_CACERT: u32 = 589824u32;
pub const EXTENSION_ORIGIN_CMC: u32 = 524288u32;
pub const EXTENSION_ORIGIN_IMPORTEDCERT: u32 = 393216u32;
pub const EXTENSION_ORIGIN_MASK: u32 = 983040u32;
pub const EXTENSION_ORIGIN_PKCS7: u32 = 458752u32;
pub const EXTENSION_ORIGIN_POLICY: u32 = 131072u32;
pub const EXTENSION_ORIGIN_RENEWALCERT: u32 = 327680u32;
pub const EXTENSION_ORIGIN_REQUEST: u32 = 65536u32;
pub const EXTENSION_ORIGIN_SERVER: u32 = 262144u32;
pub const EXTENSION_POLICY_MASK: u32 = 65535u32;
pub const XCN_CRYPT_STRING_BASE64HEADER: i32 = 0i32;
pub const XCN_CRYPT_STRING_BASE64: i32 = 1i32;
pub const XCN_CRYPT_STRING_BINARY: i32 = 2i32;
pub const XCN_CRYPT_STRING_BASE64REQUESTHEADER: i32 = 3i32;
pub const XCN_CRYPT_STRING_HEX: i32 = 4i32;
pub const XCN_CRYPT_STRING_HEXASCII: i32 = 5i32;
pub const XCN_CRYPT_STRING_BASE64_ANY: i32 = 6i32;
pub const XCN_CRYPT_STRING_ANY: i32 = 7i32;
pub const XCN_CRYPT_STRING_HEX_ANY: i32 = 8i32;
pub const XCN_CRYPT_STRING_BASE64X509CRLHEADER: i32 = 9i32;
pub const XCN_CRYPT_STRING_HEXADDR: i32 = 10i32;
pub const XCN_CRYPT_STRING_HEXASCIIADDR: i32 = 11i32;
pub const XCN_CRYPT_STRING_HEXRAW: i32 = 12i32;
pub const XCN_CRYPT_STRING_BASE64URI: i32 = 13i32;
pub const XCN_CRYPT_STRING_ENCODEMASK: i32 = 255i32;
pub const XCN_CRYPT_STRING_CHAIN: i32 = 256i32;
pub const XCN_CRYPT_STRING_TEXT: i32 = 512i32;
pub const XCN_CRYPT_STRING_PERCENTESCAPE: i32 = 134217728i32;
pub const XCN_CRYPT_STRING_HASHDATA: i32 = 268435456i32;
pub const XCN_CRYPT_STRING_STRICT: i32 = 536870912i32;
pub const XCN_CRYPT_STRING_NOCRLF: i32 = 1073741824i32;
pub const XCN_CRYPT_STRING_NOCR: i32 = -2147483648i32;
pub const CAPropCommonName: i32 = 1i32;
pub const CAPropDistinguishedName: i32 = 2i32;
pub const CAPropSanitizedName: i32 = 3i32;
pub const CAPropSanitizedShortName: i32 = 4i32;
pub const CAPropDNSName: i32 = 5i32;
pub const CAPropCertificateTypes: i32 = 6i32;
pub const CAPropCertificate: i32 = 7i32;
pub const CAPropDescription: i32 = 8i32;
pub const CAPropWebServers: i32 = 9i32;
pub const CAPropSiteName: i32 = 10i32;
pub const CAPropSecurity: i32 = 11i32;
pub const CAPropRenewalOnly: i32 = 12i32;
pub const DisplayNo: i32 = 0i32;
pub const DisplayYes: i32 = 1i32;
pub const Enrolled: i32 = 1i32;
pub const EnrollPended: i32 = 2i32;
pub const EnrollUIDeferredEnrollmentRequired: i32 = 4i32;
pub const EnrollError: i32 = 16i32;
pub const EnrollUnknown: i32 = 32i32;
pub const EnrollSkipped: i32 = 64i32;
pub const EnrollDenied: i32 = 256i32;
pub const DisableGroupPolicyList: i32 = 2i32;
pub const DisableUserServerList: i32 = 4i32;
pub const DefaultNone: i32 = 0i32;
pub const DefaultPolicyServer: i32 = 1i32;
pub const SelectedNo: i32 = 0i32;
pub const SelectedYes: i32 = 1i32;
pub const TemplatePropCommonName: i32 = 1i32;
pub const TemplatePropFriendlyName: i32 = 2i32;
pub const TemplatePropEKUs: i32 = 3i32;
pub const TemplatePropCryptoProviders: i32 = 4i32;
pub const TemplatePropMajorRevision: i32 = 5i32;
pub const TemplatePropDescription: i32 = 6i32;
pub const TemplatePropKeySpec: i32 = 7i32;
pub const TemplatePropSchemaVersion: i32 = 8i32;
pub const TemplatePropMinorRevision: i32 = 9i32;
pub const TemplatePropRASignatureCount: i32 = 10i32;
pub const TemplatePropMinimumKeySize: i32 = 11i32;
pub const TemplatePropOID: i32 = 12i32;
pub const TemplatePropSupersede: i32 = 13i32;
pub const TemplatePropRACertificatePolicies: i32 = 14i32;
pub const TemplatePropRAEKUs: i32 = 15i32;
pub const TemplatePropCertificatePolicies: i32 = 16i32;
pub const TemplatePropV1ApplicationPolicy: i32 = 17i32;
pub const TemplatePropAsymmetricAlgorithm: i32 = 18i32;
pub const TemplatePropKeySecurityDescriptor: i32 = 19i32;
pub const TemplatePropSymmetricAlgorithm: i32 = 20i32;
pub const TemplatePropSymmetricKeyLength: i32 = 21i32;
pub const TemplatePropHashAlgorithm: i32 = 22i32;
pub const TemplatePropKeyUsage: i32 = 23i32;
pub const TemplatePropEnrollmentFlags: i32 = 24i32;
pub const TemplatePropSubjectNameFlags: i32 = 25i32;
pub const TemplatePropPrivateKeyFlags: i32 = 26i32;
pub const TemplatePropGeneralFlags: i32 = 27i32;
pub const TemplatePropSecurityDescriptor: i32 = 28i32;
pub const TemplatePropExtensions: i32 = 29i32;
pub const TemplatePropValidityPeriod: i32 = 30i32;
pub const TemplatePropRenewalPeriod: i32 = 31i32;
pub type FNCERTSRVBACKUPCLOSE = unsafe extern "system" fn(hbc: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
pub type FNCERTSRVBACKUPEND = unsafe extern "system" fn(hbc: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
pub type FNCERTSRVBACKUPFREE = unsafe extern "system" fn(pv: *mut ::core::ffi::c_void);
pub type FNCERTSRVBACKUPGETBACKUPLOGSW = unsafe extern "system" fn(hbc: *mut ::core::ffi::c_void, ppwszzbackuplogfiles: *mut *mut u16, pcbsize: *mut u32) -> ::windows_sys::core::HRESULT;
pub type FNCERTSRVBACKUPGETDATABASENAMESW = unsafe extern "system" fn(hbc: *mut ::core::ffi::c_void, ppwszzattachmentinformation: *mut *mut u16, pcbsize: *mut u32) -> ::windows_sys::core::HRESULT;
pub type FNCERTSRVBACKUPGETDYNAMICFILELISTW = unsafe extern "system" fn(hbc: *mut ::core::ffi::c_void, ppwszzfilelist: *mut *mut u16, pcbsize: *mut u32) -> ::windows_sys::core::HRESULT;
#[cfg(feature = "Win32_Foundation")]
pub type FNCERTSRVBACKUPOPENFILEW = unsafe extern "system" fn(hbc: *mut ::core::ffi::c_void, pwszattachmentname: super::super::super::Foundation::PWSTR, cbreadhintsize: u32, plifilesize: *mut i64) -> ::windows_sys::core::HRESULT;
#[cfg(feature = "Win32_Foundation")]
pub type FNCERTSRVBACKUPPREPAREW = unsafe extern "system" fn(pwszservername: super::super::super::Foundation::PWSTR, grbitjet: u32, dwbackupflags: u32, phbc: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
pub type FNCERTSRVBACKUPREAD = unsafe extern "system" fn(hbc: *mut ::core::ffi::c_void, pvbuffer: *mut ::core::ffi::c_void, cbbuffer: u32, pcbread: *mut u32) -> ::windows_sys::core::HRESULT;
pub type FNCERTSRVBACKUPTRUNCATELOGS = unsafe extern "system" fn(hbc: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
#[cfg(feature = "Win32_Foundation")]
pub type FNCERTSRVISSERVERONLINEW = unsafe extern "system" fn(pwszservername: super::super::super::Foundation::PWSTR, pfserveronline: *mut super::super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT;
pub type FNCERTSRVRESTOREEND = unsafe extern "system" fn(hbc: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
pub type FNCERTSRVRESTOREGETDATABASELOCATIONSW = unsafe extern "system" fn(hbc: *mut ::core::ffi::c_void, ppwszzdatabaselocationlist: *mut *mut u16, pcbsize: *mut u32) -> ::windows_sys::core::HRESULT;
#[cfg(feature = "Win32_Foundation")]
pub type FNCERTSRVRESTOREPREPAREW = unsafe extern "system" fn(pwszservername: super::super::super::Foundation::PWSTR, dwrestoreflags: u32, phbc: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
pub type FNCERTSRVRESTOREREGISTERCOMPLETE = unsafe extern "system" fn(hbc: *mut ::core::ffi::c_void, hrrestorestate: ::windows_sys::core::HRESULT) -> ::windows_sys::core::HRESULT;
#[cfg(feature = "Win32_Foundation")]
pub type FNCERTSRVRESTOREREGISTERW = unsafe extern "system" fn(hbc: *mut ::core::ffi::c_void, pwszcheckpointfilepath: super::super::super::Foundation::PWSTR, pwszlogpath: super::super::super::Foundation::PWSTR, rgrstmap: *mut CSEDB_RSTMAPW, crstmap: i32, pwszbackuplogpath: super::super::super::Foundation::PWSTR, genlow: u32, genhigh: u32) -> ::windows_sys::core::HRESULT;
#[cfg(feature = "Win32_Foundation")]
pub type FNCERTSRVSERVERCONTROLW = unsafe extern "system" fn(pwszservername: super::super::super::Foundation::PWSTR, dwcontrolflags: u32, pcbout: *mut u32, ppbout: *mut *mut u8) -> ::windows_sys::core::HRESULT;
#[cfg(feature = "Win32_Foundation")]
pub type FNIMPORTPFXTOPROVIDER = unsafe extern "system" fn(
    hwndparent: super::super::super::Foundation::HWND,
    pbpfx: *const u8,
    cbpfx: u32,
    importflags: ImportPFXFlags,
    pwszpassword: super::super::super::Foundation::PWSTR,
    pwszprovidername: super::super::super::Foundation::PWSTR,
    pwszreadername: super::super::super::Foundation::PWSTR,
    pwszcontainernameprefix: super::super::super::Foundation::PWSTR,
    pwszpin: super::super::super::Foundation::PWSTR,
    pwszfriendlyname: super::super::super::Foundation::PWSTR,
    pccertout: *mut u32,
    prgpcertout: *mut *mut *mut super::CERT_CONTEXT,
) -> ::windows_sys::core::HRESULT;
#[cfg(feature = "Win32_Foundation")]
pub type FNIMPORTPFXTOPROVIDERFREEDATA = unsafe extern "system" fn(ccert: u32, rgpcert: *const *const super::CERT_CONTEXT);
pub const FR_PROP_CLAIMCHALLENGE: u32 = 22u32;
pub const FR_PROP_NONE: u32 = 0u32;
pub const FR_PROP_FULLRESPONSE: u32 = 1u32;
pub const FR_PROP_STATUSINFOCOUNT: u32 = 2u32;
pub const FR_PROP_BODYPARTSTRING: u32 = 3u32;
pub const FR_PROP_STATUS: u32 = 4u32;
pub const FR_PROP_STATUSSTRING: u32 = 5u32;
pub const FR_PROP_OTHERINFOCHOICE: u32 = 6u32;
pub const FR_PROP_FAILINFO: u32 = 7u32;
pub const FR_PROP_PENDINFOTOKEN: u32 = 8u32;
pub const FR_PROP_PENDINFOTIME: u32 = 9u32;
pub const FR_PROP_ISSUEDCERTIFICATEHASH: u32 = 10u32;
pub const FR_PROP_ISSUEDCERTIFICATE: u32 = 11u32;
pub const FR_PROP_ISSUEDCERTIFICATECHAIN: u32 = 12u32;
pub const FR_PROP_ISSUEDCERTIFICATECRLCHAIN: u32 = 13u32;
pub const FR_PROP_ENCRYPTEDKEYHASH: u32 = 14u32;
pub const FR_PROP_FULLRESPONSENOPKCS7: u32 = 15u32;
pub const FR_PROP_CAEXCHANGECERTIFICATEHASH: u32 = 16u32;
pub const FR_PROP_CAEXCHANGECERTIFICATE: u32 = 17u32;
pub const FR_PROP_CAEXCHANGECERTIFICATECHAIN: u32 = 18u32;
pub const FR_PROP_CAEXCHANGECERTIFICATECRLCHAIN: u32 = 19u32;
pub const FR_PROP_ATTESTATIONCHALLENGE: u32 = 20u32;
pub const FR_PROP_ATTESTATIONPROVIDERNAME: u32 = 21u32;
#[repr(transparent)]
pub struct IAlternativeName(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IAlternativeName {}
impl ::core::clone::Clone for IAlternativeName {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IAlternativeNames(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IAlternativeNames {}
impl ::core::clone::Clone for IAlternativeNames {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IBinaryConverter(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IBinaryConverter {}
impl ::core::clone::Clone for IBinaryConverter {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IBinaryConverter2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IBinaryConverter2 {}
impl ::core::clone::Clone for IBinaryConverter2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ICEnroll(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ICEnroll {}
impl ::core::clone::Clone for ICEnroll {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ICEnroll2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ICEnroll2 {}
impl ::core::clone::Clone for ICEnroll2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ICEnroll3(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ICEnroll3 {}
impl ::core::clone::Clone for ICEnroll3 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ICEnroll4(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ICEnroll4 {}
impl ::core::clone::Clone for ICEnroll4 {
    fn clone(&self) -> Self {
        *self
    }
}
pub const ICF_ALLOWFOREIGN: u32 = 65536u32;
pub const ICF_EXISTINGROW: u32 = 131072u32;
#[repr(transparent)]
pub struct ICertAdmin(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ICertAdmin {}
impl ::core::clone::Clone for ICertAdmin {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ICertAdmin2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ICertAdmin2 {}
impl ::core::clone::Clone for ICertAdmin2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ICertConfig(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ICertConfig {}
impl ::core::clone::Clone for ICertConfig {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ICertConfig2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ICertConfig2 {}
impl ::core::clone::Clone for ICertConfig2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ICertEncodeAltName(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ICertEncodeAltName {}
impl ::core::clone::Clone for ICertEncodeAltName {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ICertEncodeAltName2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ICertEncodeAltName2 {}
impl ::core::clone::Clone for ICertEncodeAltName2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ICertEncodeBitString(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ICertEncodeBitString {}
impl ::core::clone::Clone for ICertEncodeBitString {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ICertEncodeBitString2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ICertEncodeBitString2 {}
impl ::core::clone::Clone for ICertEncodeBitString2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ICertEncodeCRLDistInfo(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ICertEncodeCRLDistInfo {}
impl ::core::clone::Clone for ICertEncodeCRLDistInfo {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ICertEncodeCRLDistInfo2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ICertEncodeCRLDistInfo2 {}
impl ::core::clone::Clone for ICertEncodeCRLDistInfo2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ICertEncodeDateArray(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ICertEncodeDateArray {}
impl ::core::clone::Clone for ICertEncodeDateArray {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ICertEncodeDateArray2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ICertEncodeDateArray2 {}
impl ::core::clone::Clone for ICertEncodeDateArray2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ICertEncodeLongArray(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ICertEncodeLongArray {}
impl ::core::clone::Clone for ICertEncodeLongArray {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ICertEncodeLongArray2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ICertEncodeLongArray2 {}
impl ::core::clone::Clone for ICertEncodeLongArray2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ICertEncodeStringArray(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ICertEncodeStringArray {}
impl ::core::clone::Clone for ICertEncodeStringArray {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ICertEncodeStringArray2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ICertEncodeStringArray2 {}
impl ::core::clone::Clone for ICertEncodeStringArray2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ICertExit(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ICertExit {}
impl ::core::clone::Clone for ICertExit {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ICertExit2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ICertExit2 {}
impl ::core::clone::Clone for ICertExit2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ICertGetConfig(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ICertGetConfig {}
impl ::core::clone::Clone for ICertGetConfig {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ICertManageModule(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ICertManageModule {}
impl ::core::clone::Clone for ICertManageModule {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ICertPolicy(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ICertPolicy {}
impl ::core::clone::Clone for ICertPolicy {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ICertPolicy2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ICertPolicy2 {}
impl ::core::clone::Clone for ICertPolicy2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ICertProperties(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ICertProperties {}
impl ::core::clone::Clone for ICertProperties {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ICertProperty(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ICertProperty {}
impl ::core::clone::Clone for ICertProperty {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ICertPropertyArchived(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ICertPropertyArchived {}
impl ::core::clone::Clone for ICertPropertyArchived {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ICertPropertyArchivedKeyHash(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ICertPropertyArchivedKeyHash {}
impl ::core::clone::Clone for ICertPropertyArchivedKeyHash {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ICertPropertyAutoEnroll(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ICertPropertyAutoEnroll {}
impl ::core::clone::Clone for ICertPropertyAutoEnroll {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ICertPropertyBackedUp(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ICertPropertyBackedUp {}
impl ::core::clone::Clone for ICertPropertyBackedUp {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ICertPropertyDescription(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ICertPropertyDescription {}
impl ::core::clone::Clone for ICertPropertyDescription {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ICertPropertyEnrollment(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ICertPropertyEnrollment {}
impl ::core::clone::Clone for ICertPropertyEnrollment {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ICertPropertyEnrollmentPolicyServer(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ICertPropertyEnrollmentPolicyServer {}
impl ::core::clone::Clone for ICertPropertyEnrollmentPolicyServer {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ICertPropertyFriendlyName(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ICertPropertyFriendlyName {}
impl ::core::clone::Clone for ICertPropertyFriendlyName {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ICertPropertyKeyProvInfo(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ICertPropertyKeyProvInfo {}
impl ::core::clone::Clone for ICertPropertyKeyProvInfo {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ICertPropertyRenewal(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ICertPropertyRenewal {}
impl ::core::clone::Clone for ICertPropertyRenewal {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ICertPropertyRequestOriginator(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ICertPropertyRequestOriginator {}
impl ::core::clone::Clone for ICertPropertyRequestOriginator {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ICertPropertySHA1Hash(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ICertPropertySHA1Hash {}
impl ::core::clone::Clone for ICertPropertySHA1Hash {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ICertRequest(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ICertRequest {}
impl ::core::clone::Clone for ICertRequest {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ICertRequest2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ICertRequest2 {}
impl ::core::clone::Clone for ICertRequest2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ICertRequest3(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ICertRequest3 {}
impl ::core::clone::Clone for ICertRequest3 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ICertRequestD(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ICertRequestD {}
impl ::core::clone::Clone for ICertRequestD {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ICertRequestD2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ICertRequestD2 {}
impl ::core::clone::Clone for ICertRequestD2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ICertServerExit(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ICertServerExit {}
impl ::core::clone::Clone for ICertServerExit {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ICertServerPolicy(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ICertServerPolicy {}
impl ::core::clone::Clone for ICertServerPolicy {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ICertView(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ICertView {}
impl ::core::clone::Clone for ICertView {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ICertView2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ICertView2 {}
impl ::core::clone::Clone for ICertView2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ICertificateAttestationChallenge(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ICertificateAttestationChallenge {}
impl ::core::clone::Clone for ICertificateAttestationChallenge {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ICertificateAttestationChallenge2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ICertificateAttestationChallenge2 {}
impl ::core::clone::Clone for ICertificateAttestationChallenge2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ICertificatePolicies(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ICertificatePolicies {}
impl ::core::clone::Clone for ICertificatePolicies {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ICertificatePolicy(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ICertificatePolicy {}
impl ::core::clone::Clone for ICertificatePolicy {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ICertificationAuthorities(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ICertificationAuthorities {}
impl ::core::clone::Clone for ICertificationAuthorities {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ICertificationAuthority(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ICertificationAuthority {}
impl ::core::clone::Clone for ICertificationAuthority {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ICryptAttribute(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ICryptAttribute {}
impl ::core::clone::Clone for ICryptAttribute {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ICryptAttributes(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ICryptAttributes {}
impl ::core::clone::Clone for ICryptAttributes {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ICspAlgorithm(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ICspAlgorithm {}
impl ::core::clone::Clone for ICspAlgorithm {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ICspAlgorithms(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ICspAlgorithms {}
impl ::core::clone::Clone for ICspAlgorithms {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ICspInformation(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ICspInformation {}
impl ::core::clone::Clone for ICspInformation {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ICspInformations(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ICspInformations {}
impl ::core::clone::Clone for ICspInformations {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ICspStatus(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ICspStatus {}
impl ::core::clone::Clone for ICspStatus {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ICspStatuses(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ICspStatuses {}
impl ::core::clone::Clone for ICspStatuses {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IEnroll(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IEnroll {}
impl ::core::clone::Clone for IEnroll {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IEnroll2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IEnroll2 {}
impl ::core::clone::Clone for IEnroll2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IEnroll4(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IEnroll4 {}
impl ::core::clone::Clone for IEnroll4 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IEnumCERTVIEWATTRIBUTE(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IEnumCERTVIEWATTRIBUTE {}
impl ::core::clone::Clone for IEnumCERTVIEWATTRIBUTE {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IEnumCERTVIEWCOLUMN(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IEnumCERTVIEWCOLUMN {}
impl ::core::clone::Clone for IEnumCERTVIEWCOLUMN {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IEnumCERTVIEWEXTENSION(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IEnumCERTVIEWEXTENSION {}
impl ::core::clone::Clone for IEnumCERTVIEWEXTENSION {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IEnumCERTVIEWROW(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IEnumCERTVIEWROW {}
impl ::core::clone::Clone for IEnumCERTVIEWROW {
    fn clone(&self) -> Self {
        *self
    }
}
pub const IF_ENABLEADMINASAUDITOR: u32 = 4096u32;
pub const IF_ENABLEEXITKEYRETRIEVAL: u32 = 2048u32;
pub const IF_ENFORCEENCRYPTICERTADMIN: u32 = 1024u32;
pub const IF_ENFORCEENCRYPTICERTREQUEST: u32 = 512u32;
pub const IF_LOCKICERTREQUEST: u32 = 1u32;
pub const IF_NOLOCALICERTADMIN: u32 = 32u32;
pub const IF_NOLOCALICERTADMINBACKUP: u32 = 128u32;
pub const IF_NOLOCALICERTREQUEST: u32 = 4u32;
pub const IF_NOREMOTEICERTADMIN: u32 = 16u32;
pub const IF_NOREMOTEICERTADMINBACKUP: u32 = 64u32;
pub const IF_NOREMOTEICERTREQUEST: u32 = 2u32;
pub const IF_NORPCICERTREQUEST: u32 = 8u32;
pub const IF_NOSNAPSHOTBACKUP: u32 = 256u32;
pub const IKF_OVERWRITE: u32 = 65536u32;
#[repr(transparent)]
pub struct INDESPolicy(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for INDESPolicy {}
impl ::core::clone::Clone for INDESPolicy {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IOCSPAdmin(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IOCSPAdmin {}
impl ::core::clone::Clone for IOCSPAdmin {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IOCSPCAConfiguration(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IOCSPCAConfiguration {}
impl ::core::clone::Clone for IOCSPCAConfiguration {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IOCSPCAConfigurationCollection(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IOCSPCAConfigurationCollection {}
impl ::core::clone::Clone for IOCSPCAConfigurationCollection {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IOCSPProperty(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IOCSPProperty {}
impl ::core::clone::Clone for IOCSPProperty {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IOCSPPropertyCollection(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IOCSPPropertyCollection {}
impl ::core::clone::Clone for IOCSPPropertyCollection {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IObjectId(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IObjectId {}
impl ::core::clone::Clone for IObjectId {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IObjectIds(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IObjectIds {}
impl ::core::clone::Clone for IObjectIds {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IPolicyQualifier(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IPolicyQualifier {}
impl ::core::clone::Clone for IPolicyQualifier {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IPolicyQualifiers(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IPolicyQualifiers {}
impl ::core::clone::Clone for IPolicyQualifiers {
    fn clone(&self) -> Self {
        *self
    }
}
pub const ISSCERT_DEFAULT_DS: u32 = 256u32;
pub const ISSCERT_DEFAULT_NODS: u32 = 256u32;
pub const ISSCERT_ENABLE: u32 = 256u32;
pub const ISSCERT_FILEURL_OLD: u32 = 8u32;
pub const ISSCERT_FTPURL_OLD: u32 = 4u32;
pub const ISSCERT_HTTPURL_OLD: u32 = 2u32;
pub const ISSCERT_LDAPURL_OLD: u32 = 1u32;
pub const ISSCERT_URLMASK_OLD: u32 = 255u32;
#[repr(transparent)]
pub struct ISignerCertificate(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISignerCertificate {}
impl ::core::clone::Clone for ISignerCertificate {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISignerCertificates(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISignerCertificates {}
impl ::core::clone::Clone for ISignerCertificates {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISmimeCapabilities(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISmimeCapabilities {}
impl ::core::clone::Clone for ISmimeCapabilities {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISmimeCapability(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISmimeCapability {}
impl ::core::clone::Clone for ISmimeCapability {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IX500DistinguishedName(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IX500DistinguishedName {}
impl ::core::clone::Clone for IX500DistinguishedName {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IX509Attribute(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IX509Attribute {}
impl ::core::clone::Clone for IX509Attribute {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IX509AttributeArchiveKey(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IX509AttributeArchiveKey {}
impl ::core::clone::Clone for IX509AttributeArchiveKey {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IX509AttributeArchiveKeyHash(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IX509AttributeArchiveKeyHash {}
impl ::core::clone::Clone for IX509AttributeArchiveKeyHash {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IX509AttributeClientId(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IX509AttributeClientId {}
impl ::core::clone::Clone for IX509AttributeClientId {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IX509AttributeCspProvider(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IX509AttributeCspProvider {}
impl ::core::clone::Clone for IX509AttributeCspProvider {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IX509AttributeExtensions(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IX509AttributeExtensions {}
impl ::core::clone::Clone for IX509AttributeExtensions {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IX509AttributeOSVersion(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IX509AttributeOSVersion {}
impl ::core::clone::Clone for IX509AttributeOSVersion {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IX509AttributeRenewalCertificate(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IX509AttributeRenewalCertificate {}
impl ::core::clone::Clone for IX509AttributeRenewalCertificate {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IX509Attributes(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IX509Attributes {}
impl ::core::clone::Clone for IX509Attributes {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IX509CertificateRequest(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IX509CertificateRequest {}
impl ::core::clone::Clone for IX509CertificateRequest {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IX509CertificateRequestCertificate(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IX509CertificateRequestCertificate {}
impl ::core::clone::Clone for IX509CertificateRequestCertificate {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IX509CertificateRequestCertificate2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IX509CertificateRequestCertificate2 {}
impl ::core::clone::Clone for IX509CertificateRequestCertificate2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IX509CertificateRequestCmc(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IX509CertificateRequestCmc {}
impl ::core::clone::Clone for IX509CertificateRequestCmc {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IX509CertificateRequestCmc2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IX509CertificateRequestCmc2 {}
impl ::core::clone::Clone for IX509CertificateRequestCmc2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IX509CertificateRequestPkcs10(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IX509CertificateRequestPkcs10 {}
impl ::core::clone::Clone for IX509CertificateRequestPkcs10 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IX509CertificateRequestPkcs10V2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IX509CertificateRequestPkcs10V2 {}
impl ::core::clone::Clone for IX509CertificateRequestPkcs10V2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IX509CertificateRequestPkcs10V3(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IX509CertificateRequestPkcs10V3 {}
impl ::core::clone::Clone for IX509CertificateRequestPkcs10V3 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IX509CertificateRequestPkcs10V4(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IX509CertificateRequestPkcs10V4 {}
impl ::core::clone::Clone for IX509CertificateRequestPkcs10V4 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IX509CertificateRequestPkcs7(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IX509CertificateRequestPkcs7 {}
impl ::core::clone::Clone for IX509CertificateRequestPkcs7 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IX509CertificateRequestPkcs7V2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IX509CertificateRequestPkcs7V2 {}
impl ::core::clone::Clone for IX509CertificateRequestPkcs7V2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IX509CertificateRevocationList(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IX509CertificateRevocationList {}
impl ::core::clone::Clone for IX509CertificateRevocationList {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IX509CertificateRevocationListEntries(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IX509CertificateRevocationListEntries {}
impl ::core::clone::Clone for IX509CertificateRevocationListEntries {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IX509CertificateRevocationListEntry(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IX509CertificateRevocationListEntry {}
impl ::core::clone::Clone for IX509CertificateRevocationListEntry {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IX509CertificateTemplate(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IX509CertificateTemplate {}
impl ::core::clone::Clone for IX509CertificateTemplate {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IX509CertificateTemplateWritable(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IX509CertificateTemplateWritable {}
impl ::core::clone::Clone for IX509CertificateTemplateWritable {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IX509CertificateTemplates(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IX509CertificateTemplates {}
impl ::core::clone::Clone for IX509CertificateTemplates {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IX509EndorsementKey(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IX509EndorsementKey {}
impl ::core::clone::Clone for IX509EndorsementKey {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IX509Enrollment(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IX509Enrollment {}
impl ::core::clone::Clone for IX509Enrollment {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IX509Enrollment2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IX509Enrollment2 {}
impl ::core::clone::Clone for IX509Enrollment2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IX509EnrollmentHelper(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IX509EnrollmentHelper {}
impl ::core::clone::Clone for IX509EnrollmentHelper {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IX509EnrollmentPolicyServer(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IX509EnrollmentPolicyServer {}
impl ::core::clone::Clone for IX509EnrollmentPolicyServer {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IX509EnrollmentStatus(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IX509EnrollmentStatus {}
impl ::core::clone::Clone for IX509EnrollmentStatus {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IX509EnrollmentWebClassFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IX509EnrollmentWebClassFactory {}
impl ::core::clone::Clone for IX509EnrollmentWebClassFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IX509Extension(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IX509Extension {}
impl ::core::clone::Clone for IX509Extension {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IX509ExtensionAlternativeNames(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IX509ExtensionAlternativeNames {}
impl ::core::clone::Clone for IX509ExtensionAlternativeNames {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IX509ExtensionAuthorityKeyIdentifier(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IX509ExtensionAuthorityKeyIdentifier {}
impl ::core::clone::Clone for IX509ExtensionAuthorityKeyIdentifier {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IX509ExtensionBasicConstraints(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IX509ExtensionBasicConstraints {}
impl ::core::clone::Clone for IX509ExtensionBasicConstraints {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IX509ExtensionCertificatePolicies(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IX509ExtensionCertificatePolicies {}
impl ::core::clone::Clone for IX509ExtensionCertificatePolicies {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IX509ExtensionEnhancedKeyUsage(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IX509ExtensionEnhancedKeyUsage {}
impl ::core::clone::Clone for IX509ExtensionEnhancedKeyUsage {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IX509ExtensionKeyUsage(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IX509ExtensionKeyUsage {}
impl ::core::clone::Clone for IX509ExtensionKeyUsage {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IX509ExtensionMSApplicationPolicies(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IX509ExtensionMSApplicationPolicies {}
impl ::core::clone::Clone for IX509ExtensionMSApplicationPolicies {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IX509ExtensionSmimeCapabilities(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IX509ExtensionSmimeCapabilities {}
impl ::core::clone::Clone for IX509ExtensionSmimeCapabilities {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IX509ExtensionSubjectKeyIdentifier(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IX509ExtensionSubjectKeyIdentifier {}
impl ::core::clone::Clone for IX509ExtensionSubjectKeyIdentifier {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IX509ExtensionTemplate(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IX509ExtensionTemplate {}
impl ::core::clone::Clone for IX509ExtensionTemplate {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IX509ExtensionTemplateName(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IX509ExtensionTemplateName {}
impl ::core::clone::Clone for IX509ExtensionTemplateName {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IX509Extensions(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IX509Extensions {}
impl ::core::clone::Clone for IX509Extensions {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IX509MachineEnrollmentFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IX509MachineEnrollmentFactory {}
impl ::core::clone::Clone for IX509MachineEnrollmentFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IX509NameValuePair(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IX509NameValuePair {}
impl ::core::clone::Clone for IX509NameValuePair {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IX509NameValuePairs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IX509NameValuePairs {}
impl ::core::clone::Clone for IX509NameValuePairs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IX509PolicyServerListManager(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IX509PolicyServerListManager {}
impl ::core::clone::Clone for IX509PolicyServerListManager {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IX509PolicyServerUrl(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IX509PolicyServerUrl {}
impl ::core::clone::Clone for IX509PolicyServerUrl {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IX509PrivateKey(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IX509PrivateKey {}
impl ::core::clone::Clone for IX509PrivateKey {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IX509PrivateKey2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IX509PrivateKey2 {}
impl ::core::clone::Clone for IX509PrivateKey2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IX509PublicKey(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IX509PublicKey {}
impl ::core::clone::Clone for IX509PublicKey {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IX509SCEPEnrollment(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IX509SCEPEnrollment {}
impl ::core::clone::Clone for IX509SCEPEnrollment {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IX509SCEPEnrollment2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IX509SCEPEnrollment2 {}
impl ::core::clone::Clone for IX509SCEPEnrollment2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IX509SCEPEnrollmentHelper(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IX509SCEPEnrollmentHelper {}
impl ::core::clone::Clone for IX509SCEPEnrollmentHelper {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IX509SignatureInformation(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IX509SignatureInformation {}
impl ::core::clone::Clone for IX509SignatureInformation {
    fn clone(&self) -> Self {
        *self
    }
}
pub const ImportNone: i32 = 0i32;
pub const ImportMachineContext: i32 = 1i32;
pub const ImportForceOverwrite: i32 = 2i32;
pub const ImportSilent: i32 = 4i32;
pub const ImportSaveProperties: i32 = 8i32;
pub const ImportExportable: i32 = 16i32;
pub const ImportExportableEncrypted: i32 = 32i32;
pub const ImportNoUserProtected: i32 = 64i32;
pub const ImportUserProtected: i32 = 128i32;
pub const ImportUserProtectedHigh: i32 = 256i32;
pub const ImportInstallCertificate: i32 = 512i32;
pub const ImportInstallChain: i32 = 1024i32;
pub const ImportInstallChainAndRoot: i32 = 2048i32;
pub const LevelInnermost: i32 = 0i32;
pub const LevelNext: i32 = 1i32;
pub const AllowNone: i32 = 0i32;
pub const AllowNoOutstandingRequest: i32 = 1i32;
pub const AllowUntrustedCertificate: i32 = 2i32;
pub const AllowUntrustedRoot: i32 = 4i32;
pub const KRAF_DISABLEUSEDEFAULTPROVIDER: u32 = 8u32;
pub const KRAF_ENABLEARCHIVEALL: u32 = 4u32;
pub const KRAF_ENABLEFOREIGN: u32 = 1u32;
pub const KRAF_SAVEBADREQUESTKEY: u32 = 2u32;
pub const KRA_DISP_EXPIRED: u32 = 0u32;
pub const KRA_DISP_INVALID: u32 = 4u32;
pub const KRA_DISP_NOTFOUND: u32 = 1u32;
pub const KRA_DISP_NOTLOADED: u32 = 6u32;
pub const KRA_DISP_REVOKED: u32 = 2u32;
pub const KRA_DISP_UNTRUSTED: u32 = 5u32;
pub const KRA_DISP_VALID: u32 = 3u32;
pub const KR_ENABLE_MACHINE: u32 = 1u32;
pub const KR_ENABLE_USER: u32 = 2u32;
pub const XCN_NCRYPT_CLAIM_NONE: i32 = 0i32;
pub const XCN_NCRYPT_CLAIM_AUTHORITY_AND_SUBJECT: i32 = 3i32;
pub const XCN_NCRYPT_CLAIM_AUTHORITY_ONLY: i32 = 1i32;
pub const XCN_NCRYPT_CLAIM_SUBJECT_ONLY: i32 = 2i32;
pub const XCN_NCRYPT_CLAIM_UNKNOWN: i32 = 4096i32;
pub const SKIHashDefault: i32 = 0i32;
pub const SKIHashSha1: i32 = 1i32;
pub const SKIHashCapiSha1: i32 = 2i32;
pub const SKIHashSha256: i32 = 3i32;
pub const SKIHashHPKP: i32 = 5i32;
pub const LDAPF_SIGNDISABLE: u32 = 2u32;
pub const LDAPF_SSLENABLE: u32 = 1u32;
pub const OCSPAdmin: ::windows_sys::core::GUID = ::windows_sys::core::GUID {
    data1: 3556193553,
    data2: 37577,
    data3: 18379,
    data4: [143, 242, 141, 137, 26, 124, 77, 228],
};
pub const OCSPPropertyCollection: ::windows_sys::core::GUID = ::windows_sys::core::GUID {
    data1: 4181042472,
    data2: 47754,
    data3: 19929,
    data4: [186, 121, 242, 131, 39, 92, 178, 222],
};
pub const OCSP_RF_REJECT_SIGNED_REQUESTS: i32 = 1i32;
pub const OCSP_SF_SILENT: i32 = 1i32;
pub const OCSP_SF_USE_CACERT: i32 = 2i32;
pub const OCSP_SF_ALLOW_SIGNINGCERT_AUTORENEWAL: i32 = 4i32;
pub const OCSP_SF_FORCE_SIGNINGCERT_ISSUER_ISCA: i32 = 8i32;
pub const OCSP_SF_AUTODISCOVER_SIGNINGCERT: i32 = 16i32;
pub const OCSP_SF_MANUAL_ASSIGN_SIGNINGCERT: i32 = 32i32;
pub const OCSP_SF_RESPONDER_ID_KEYHASH: i32 = 64i32;
pub const OCSP_SF_RESPONDER_ID_NAME: i32 = 128i32;
pub const OCSP_SF_ALLOW_NONCE_EXTENSION: i32 = 256i32;
pub const OCSP_SF_ALLOW_SIGNINGCERT_AUTOENROLLMENT: i32 = 512i32;
pub const XCN_CRYPT_ANY_GROUP_ID: i32 = 0i32;
pub const XCN_CRYPT_HASH_ALG_OID_GROUP_ID: i32 = 1i32;
pub const XCN_CRYPT_ENCRYPT_ALG_OID_GROUP_ID: i32 = 2i32;
pub const XCN_CRYPT_PUBKEY_ALG_OID_GROUP_ID: i32 = 3i32;
pub const XCN_CRYPT_SIGN_ALG_OID_GROUP_ID: i32 = 4i32;
pub const XCN_CRYPT_RDN_ATTR_OID_GROUP_ID: i32 = 5i32;
pub const XCN_CRYPT_EXT_OR_ATTR_OID_GROUP_ID: i32 = 6i32;
pub const XCN_CRYPT_ENHKEY_USAGE_OID_GROUP_ID: i32 = 7i32;
pub const XCN_CRYPT_POLICY_OID_GROUP_ID: i32 = 8i32;
pub const XCN_CRYPT_TEMPLATE_OID_GROUP_ID: i32 = 9i32;
pub const XCN_CRYPT_KDF_OID_GROUP_ID: i32 = 10i32;
pub const XCN_CRYPT_LAST_OID_GROUP_ID: i32 = 10i32;
pub const XCN_CRYPT_FIRST_ALG_OID_GROUP_ID: i32 = 1i32;
pub const XCN_CRYPT_LAST_ALG_OID_GROUP_ID: i32 = 4i32;
pub const XCN_CRYPT_GROUP_ID_MASK: i32 = 65535i32;
pub const XCN_CRYPT_OID_PREFER_CNG_ALGID_FLAG: i32 = 1073741824i32;
pub const XCN_CRYPT_OID_DISABLE_SEARCH_DS_FLAG: i32 = -2147483648i32;
pub const XCN_CRYPT_OID_INFO_OID_GROUP_BIT_LEN_MASK: i32 = 268369920i32;
pub const XCN_CRYPT_OID_INFO_OID_GROUP_BIT_LEN_SHIFT: i32 = 16i32;
pub const XCN_CRYPT_KEY_LENGTH_MASK: i32 = 268369920i32;
pub const XCN_CRYPT_OID_INFO_PUBKEY_ANY: i32 = 0i32;
pub const XCN_CRYPT_OID_INFO_PUBKEY_SIGN_KEY_FLAG: i32 = -2147483648i32;
pub const XCN_CRYPT_OID_INFO_PUBKEY_ENCRYPT_KEY_FLAG: i32 = 1073741824i32;
pub const XEPR_CADNS: u32 = 1u32;
pub const XEPR_CAFRIENDLYNAME: u32 = 3u32;
pub const XEPR_CANAME: u32 = 2u32;
pub const XEPR_HASH: u32 = 8u32;
pub const XEPR_REQUESTID: u32 = 4u32;
pub const PFXExportEEOnly: i32 = 0i32;
pub const PFXExportChainNoRoot: i32 = 1i32;
pub const PFXExportChainWithRoot: i32 = 2i32;
pub const PROCFLG_ENFORCEGOODKEYS: u32 = 1u32;
pub const PROCFLG_NONE: u32 = 0u32;
pub const PROPCALLER_ADMIN: u32 = 1024u32;
pub const PROPCALLER_EXIT: u32 = 768u32;
pub const PROPCALLER_MASK: u32 = 3840u32;
pub const PROPCALLER_POLICY: u32 = 512u32;
pub const PROPCALLER_REQUEST: u32 = 1280u32;
pub const PROPCALLER_SERVER: u32 = 256u32;
pub const PROPFLAGS_INDEXED: u32 = 65536u32;
pub const PROPTYPE_MASK: u32 = 255u32;
pub const AllowedKeySignature: i32 = 1i32;
pub const AllowedNullSignature: i32 = 2i32;
pub const PolicyQualifierTypeUnknown: i32 = 0i32;
pub const PolicyQualifierTypeUrl: i32 = 1i32;
pub const PolicyQualifierTypeUserNotice: i32 = 2i32;
pub const PolicyQualifierTypeFlags: i32 = 3i32;
pub const PsfNone: i32 = 0i32;
pub const PsfLocationGroupPolicy: i32 = 1i32;
pub const PsfLocationRegistry: i32 = 2i32;
pub const PsfUseClientId: i32 = 4i32;
pub const PsfAutoEnrollmentEnabled: i32 = 16i32;
pub const PsfAllowUnTrustedCA: i32 = 32i32;
pub const PsPolicyID: i32 = 0i32;
pub const PsFriendlyName: i32 = 1i32;
pub const REQDISP_DEFAULT_ENTERPRISE: u32 = 1u32;
pub const REQDISP_DENY: u32 = 2u32;
pub const REQDISP_ISSUE: u32 = 1u32;
pub const REQDISP_MASK: u32 = 255u32;
pub const REQDISP_PENDING: u32 = 0u32;
pub const REQDISP_PENDINGFIRST: u32 = 256u32;
pub const REQDISP_USEREQUESTATTRIBUTE: u32 = 3u32;
pub const REVEXT_ASPENABLE: u32 = 512u32;
pub const REVEXT_CDPENABLE: u32 = 256u32;
pub const REVEXT_CDPFILEURL_OLD: u32 = 8u32;
pub const REVEXT_CDPFTPURL_OLD: u32 = 4u32;
pub const REVEXT_CDPHTTPURL_OLD: u32 = 2u32;
pub const REVEXT_CDPLDAPURL_OLD: u32 = 1u32;
pub const REVEXT_CDPURLMASK_OLD: u32 = 255u32;
pub const REVEXT_DEFAULT_DS: u32 = 256u32;
pub const REVEXT_DEFAULT_NODS: u32 = 256u32;
pub const ClientIdNone: i32 = 0i32;
pub const ClientIdXEnroll2003: i32 = 1i32;
pub const ClientIdAutoEnroll2003: i32 = 2i32;
pub const ClientIdWizard2003: i32 = 3i32;
pub const ClientIdCertReq2003: i32 = 4i32;
pub const ClientIdDefaultRequest: i32 = 5i32;
pub const ClientIdAutoEnroll: i32 = 6i32;
pub const ClientIdRequestWizard: i32 = 7i32;
pub const ClientIdEOBO: i32 = 8i32;
pub const ClientIdCertReq: i32 = 9i32;
pub const ClientIdTest: i32 = 10i32;
pub const ClientIdWinRT: i32 = 11i32;
pub const ClientIdUserStart: i32 = 1000i32;
pub const SETUP_ATTEMPT_VROOT_CREATE: u32 = 128u32;
pub const SETUP_CLIENT_FLAG: u32 = 2u32;
pub const SETUP_CREATEDB_FLAG: u32 = 64u32;
pub const SETUP_DCOM_SECURITY_UPDATED_FLAG: u32 = 8192u32;
pub const SETUP_DENIED_FLAG: u32 = 32u32;
pub const SETUP_FORCECRL_FLAG: u32 = 256u32;
pub const SETUP_ONLINE_FLAG: u32 = 16u32;
pub const SETUP_REQUEST_FLAG: u32 = 8u32;
pub const SETUP_SECURITY_CHANGED: u32 = 4096u32;
pub const SETUP_SERVER_FLAG: u32 = 1u32;
pub const SETUP_SERVER_IS_UP_TO_DATE_FLAG: u32 = 16384u32;
pub const SETUP_SERVER_UPGRADED_FLAG: u32 = 1024u32;
pub const SETUP_SUSPEND_FLAG: u32 = 4u32;
pub const SETUP_UPDATE_CAOBJECT_SVRTYPE: u32 = 512u32;
pub const SETUP_W2K_SECURITY_NOT_UPGRADED_FLAG: u32 = 2048u32;
pub const TP_MACHINEPOLICY: u32 = 1u32;
pub const VR_INSTANT_BAD: u32 = 2u32;
pub const VR_INSTANT_OK: u32 = 1u32;
pub const VR_PENDING: u32 = 0u32;
pub const EnrollPrompt: i32 = 1i32;
pub const LevelUnsafe: i32 = 0i32;
pub const LevelSafe: i32 = 1i32;
pub const XCN_CERT_NAME_STR_NONE: i32 = 0i32;
pub const XCN_CERT_SIMPLE_NAME_STR: i32 = 1i32;
pub const XCN_CERT_OID_NAME_STR: i32 = 2i32;
pub const XCN_CERT_X500_NAME_STR: i32 = 3i32;
pub const XCN_CERT_XML_NAME_STR: i32 = 4i32;
pub const XCN_CERT_NAME_STR_SEMICOLON_FLAG: i32 = 1073741824i32;
pub const XCN_CERT_NAME_STR_NO_PLUS_FLAG: i32 = 536870912i32;
pub const XCN_CERT_NAME_STR_NO_QUOTING_FLAG: i32 = 268435456i32;
pub const XCN_CERT_NAME_STR_CRLF_FLAG: i32 = 134217728i32;
pub const XCN_CERT_NAME_STR_COMMA_FLAG: i32 = 67108864i32;
pub const XCN_CERT_NAME_STR_REVERSE_FLAG: i32 = 33554432i32;
pub const XCN_CERT_NAME_STR_FORWARD_FLAG: i32 = 16777216i32;
pub const XCN_CERT_NAME_STR_AMBIGUOUS_SEPARATOR_FLAGS: i32 = 1275068416i32;
pub const XCN_CERT_NAME_STR_DISABLE_IE4_UTF8_FLAG: i32 = 65536i32;
pub const XCN_CERT_NAME_STR_ENABLE_T61_UNICODE_FLAG: i32 = 131072i32;
pub const XCN_CERT_NAME_STR_ENABLE_UTF8_UNICODE_FLAG: i32 = 262144i32;
pub const XCN_CERT_NAME_STR_FORCE_UTF8_DIR_STR_FLAG: i32 = 524288i32;
pub const XCN_CERT_NAME_STR_DISABLE_UTF8_DIR_STR_FLAG: i32 = 1048576i32;
pub const XCN_CERT_NAME_STR_ENABLE_PUNYCODE_FLAG: i32 = 2097152i32;
pub const XCN_CERT_NAME_STR_DS_ESCAPED: i32 = 8388608i32;
pub const ContextNone: i32 = 0i32;
pub const ContextUser: i32 = 1i32;
pub const ContextMachine: i32 = 2i32;
pub const ContextAdministratorForceMachine: i32 = 3i32;
pub const EnrollmentIncludeSymmetricAlgorithms: i32 = 1i32;
pub const EnrollmentPendAllRequests: i32 = 2i32;
pub const EnrollmentPublishToKRAContainer: i32 = 4i32;
pub const EnrollmentPublishToDS: i32 = 8i32;
pub const EnrollmentAutoEnrollmentCheckUserDSCertificate: i32 = 16i32;
pub const EnrollmentAutoEnrollment: i32 = 32i32;
pub const EnrollmentDomainAuthenticationNotRequired: i32 = 128i32;
pub const EnrollmentPreviousApprovalValidateReenrollment: i32 = 64i32;
pub const EnrollmentUserInteractionRequired: i32 = 256i32;
pub const EnrollmentAddTemplateName: i32 = 512i32;
pub const EnrollmentRemoveInvalidCertificateFromPersonalStore: i32 = 1024i32;
pub const EnrollmentAllowEnrollOnBehalfOf: i32 = 2048i32;
pub const EnrollmentAddOCSPNoCheck: i32 = 4096i32;
pub const EnrollmentReuseKeyOnFullSmartCard: i32 = 8192i32;
pub const EnrollmentNoRevocationInfoInCerts: i32 = 16384i32;
pub const EnrollmentIncludeBasicConstraintsForEECerts: i32 = 32768i32;
pub const EnrollmentPreviousApprovalKeyBasedValidateReenrollment: i32 = 65536i32;
pub const EnrollmentCertificateIssuancePoliciesFromRequest: i32 = 131072i32;
pub const EnrollmentSkipAutoRenewal: i32 = 262144i32;
pub const GeneralMachineType: i32 = 64i32;
pub const GeneralCA: i32 = 128i32;
pub const GeneralCrossCA: i32 = 2048i32;
pub const GeneralDefault: i32 = 65536i32;
pub const GeneralModified: i32 = 131072i32;
pub const GeneralDonotPersist: i32 = 4096i32;
pub const PrivateKeyRequireArchival: i32 = 1i32;
pub const PrivateKeyExportable: i32 = 16i32;
pub const PrivateKeyRequireStrongKeyProtection: i32 = 32i32;
pub const PrivateKeyRequireAlternateSignatureAlgorithm: i32 = 64i32;
pub const PrivateKeyRequireSameKeyRenewal: i32 = 128i32;
pub const PrivateKeyUseLegacyProvider: i32 = 256i32;
pub const PrivateKeyEKTrustOnUse: i32 = 512i32;
pub const PrivateKeyEKValidateCert: i32 = 1024i32;
pub const PrivateKeyEKValidateKey: i32 = 2048i32;
pub const PrivateKeyAttestNone: i32 = 0i32;
pub const PrivateKeyAttestPreferred: i32 = 4096i32;
pub const PrivateKeyAttestRequired: i32 = 8192i32;
pub const PrivateKeyAttestMask: i32 = 12288i32;
pub const PrivateKeyAttestWithoutPolicy: i32 = 16384i32;
pub const PrivateKeyServerVersionMask: i32 = 983040i32;
pub const PrivateKeyServerVersionShift: i32 = 16i32;
pub const PrivateKeyHelloKspKey: i32 = 1048576i32;
pub const PrivateKeyHelloLogonKey: i32 = 2097152i32;
pub const PrivateKeyClientVersionMask: i32 = 251658240i32;
pub const PrivateKeyClientVersionShift: i32 = 24i32;
pub const SubjectNameEnrolleeSupplies: i32 = 1i32;
pub const SubjectNameRequireDirectoryPath: i32 = -2147483648i32;
pub const SubjectNameRequireCommonName: i32 = 1073741824i32;
pub const SubjectNameRequireEmail: i32 = 536870912i32;
pub const SubjectNameRequireDNS: i32 = 268435456i32;
pub const SubjectNameAndAlternativeNameOldCertSupplies: i32 = 8i32;
pub const SubjectAlternativeNameEnrolleeSupplies: i32 = 65536i32;
pub const SubjectAlternativeNameRequireDirectoryGUID: i32 = 16777216i32;
pub const SubjectAlternativeNameRequireUPN: i32 = 33554432i32;
pub const SubjectAlternativeNameRequireEmail: i32 = 67108864i32;
pub const SubjectAlternativeNameRequireSPN: i32 = 8388608i32;
pub const SubjectAlternativeNameRequireDNS: i32 = 134217728i32;
pub const SubjectAlternativeNameRequireDomainDNS: i32 = 4194304i32;
pub const X509AuthNone: i32 = 0i32;
pub const X509AuthAnonymous: i32 = 1i32;
pub const X509AuthKerberos: i32 = 2i32;
pub const X509AuthUsername: i32 = 4i32;
pub const X509AuthCertificate: i32 = 8i32;
pub const ExportTemplates: i32 = 1i32;
pub const ExportOIDs: i32 = 2i32;
pub const ExportCAs: i32 = 4i32;
pub const LoadOptionDefault: i32 = 0i32;
pub const LoadOptionCacheOnly: i32 = 1i32;
pub const LoadOptionReload: i32 = 2i32;
pub const LoadOptionRegisterForADChanges: i32 = 4i32;
pub const XCN_NCRYPT_PCP_NONE: i32 = 0i32;
pub const XCN_NCRYPT_TPM12_PROVIDER: i32 = 65536i32;
pub const XCN_NCRYPT_PCP_SIGNATURE_KEY: i32 = 1i32;
pub const XCN_NCRYPT_PCP_ENCRYPTION_KEY: i32 = 2i32;
pub const XCN_NCRYPT_PCP_GENERIC_KEY: i32 = 3i32;
pub const XCN_NCRYPT_PCP_STORAGE_KEY: i32 = 4i32;
pub const XCN_NCRYPT_PCP_IDENTITY_KEY: i32 = 8i32;
pub const XCN_CRYPT_OID_USE_CURVE_NONE: i32 = 0i32;
pub const XCN_CRYPT_OID_USE_CURVE_NAME_FOR_ENCODE_FLAG: i32 = 536870912i32;
pub const XCN_CRYPT_OID_USE_CURVE_PARAMETERS_FOR_ENCODE_FLAG: i32 = 268435456i32;
pub const XCN_AT_NONE: i32 = 0i32;
pub const XCN_AT_KEYEXCHANGE: i32 = 1i32;
pub const XCN_AT_SIGNATURE: i32 = 2i32;
pub const XCN_CERT_NO_KEY_USAGE: i32 = 0i32;
pub const XCN_CERT_DIGITAL_SIGNATURE_KEY_USAGE: i32 = 128i32;
pub const XCN_CERT_NON_REPUDIATION_KEY_USAGE: i32 = 64i32;
pub const XCN_CERT_KEY_ENCIPHERMENT_KEY_USAGE: i32 = 32i32;
pub const XCN_CERT_DATA_ENCIPHERMENT_KEY_USAGE: i32 = 16i32;
pub const XCN_CERT_KEY_AGREEMENT_KEY_USAGE: i32 = 8i32;
pub const XCN_CERT_KEY_CERT_SIGN_KEY_USAGE: i32 = 4i32;
pub const XCN_CERT_OFFLINE_CRL_SIGN_KEY_USAGE: i32 = 2i32;
pub const XCN_CERT_CRL_SIGN_KEY_USAGE: i32 = 2i32;
pub const XCN_CERT_ENCIPHER_ONLY_KEY_USAGE: i32 = 1i32;
pub const XCN_CERT_DECIPHER_ONLY_KEY_USAGE: i32 = 32768i32;
pub const XCN_NCRYPT_ALLOW_EXPORT_NONE: i32 = 0i32;
pub const XCN_NCRYPT_ALLOW_EXPORT_FLAG: i32 = 1i32;
pub const XCN_NCRYPT_ALLOW_PLAINTEXT_EXPORT_FLAG: i32 = 2i32;
pub const XCN_NCRYPT_ALLOW_ARCHIVING_FLAG: i32 = 4i32;
pub const XCN_NCRYPT_ALLOW_PLAINTEXT_ARCHIVING_FLAG: i32 = 8i32;
pub const XCN_NCRYPT_UI_NO_PROTECTION_FLAG: i32 = 0i32;
pub const XCN_NCRYPT_UI_PROTECT_KEY_FLAG: i32 = 1i32;
pub const XCN_NCRYPT_UI_FORCE_HIGH_PROTECTION_FLAG: i32 = 2i32;
pub const XCN_NCRYPT_UI_FINGERPRINT_PROTECTION_FLAG: i32 = 4i32;
pub const XCN_NCRYPT_UI_APPCONTAINER_ACCESS_MEDIUM_FLAG: i32 = 8i32;
pub const XCN_NCRYPT_ALLOW_USAGES_NONE: i32 = 0i32;
pub const XCN_NCRYPT_ALLOW_DECRYPT_FLAG: i32 = 1i32;
pub const XCN_NCRYPT_ALLOW_SIGNING_FLAG: i32 = 2i32;
pub const XCN_NCRYPT_ALLOW_KEY_AGREEMENT_FLAG: i32 = 4i32;
pub const XCN_NCRYPT_ALLOW_KEY_IMPORT_FLAG: i32 = 8i32;
pub const XCN_NCRYPT_ALLOW_ALL_USAGES: i32 = 16777215i32;
pub const VerifyNone: i32 = 0i32;
pub const VerifySilent: i32 = 1i32;
pub const VerifySmartCardNone: i32 = 2i32;
pub const VerifySmartCardSilent: i32 = 3i32;
pub const VerifyAllowUI: i32 = 4i32;
pub const XCN_PROV_NONE: i32 = 0i32;
pub const XCN_PROV_RSA_FULL: i32 = 1i32;
pub const XCN_PROV_RSA_SIG: i32 = 2i32;
pub const XCN_PROV_DSS: i32 = 3i32;
pub const XCN_PROV_FORTEZZA: i32 = 4i32;
pub const XCN_PROV_MS_EXCHANGE: i32 = 5i32;
pub const XCN_PROV_SSL: i32 = 6i32;
pub const XCN_PROV_RSA_SCHANNEL: i32 = 12i32;
pub const XCN_PROV_DSS_DH: i32 = 13i32;
pub const XCN_PROV_EC_ECDSA_SIG: i32 = 14i32;
pub const XCN_PROV_EC_ECNRA_SIG: i32 = 15i32;
pub const XCN_PROV_EC_ECDSA_FULL: i32 = 16i32;
pub const XCN_PROV_EC_ECNRA_FULL: i32 = 17i32;
pub const XCN_PROV_DH_SCHANNEL: i32 = 18i32;
pub const XCN_PROV_SPYRUS_LYNKS: i32 = 20i32;
pub const XCN_PROV_RNG: i32 = 21i32;
pub const XCN_PROV_INTEL_SEC: i32 = 22i32;
pub const XCN_PROV_REPLACE_OWF: i32 = 23i32;
pub const XCN_PROV_RSA_AES: i32 = 24i32;
pub const InheritDefault: i32 = 0i32;
pub const InheritNewDefaultKey: i32 = 1i32;
pub const InheritNewSimilarKey: i32 = 2i32;
pub const InheritPrivateKey: i32 = 3i32;
pub const InheritPublicKey: i32 = 4i32;
pub const InheritKeyMask: i32 = 15i32;
pub const InheritNone: i32 = 16i32;
pub const InheritRenewalCertificateFlag: i32 = 32i32;
pub const InheritTemplateFlag: i32 = 64i32;
pub const InheritSubjectFlag: i32 = 128i32;
pub const InheritExtensionsFlag: i32 = 256i32;
pub const InheritSubjectAltNameFlag: i32 = 512i32;
pub const InheritValidityPeriodFlag: i32 = 1024i32;
pub const InheritReserved80000000: i32 = -2147483648i32;
pub const TypeAny: i32 = 0i32;
pub const TypePkcs10: i32 = 1i32;
pub const TypePkcs7: i32 = 2i32;
pub const TypeCmc: i32 = 3i32;
pub const TypeCertificate: i32 = 4i32;
pub const SCEPDispositionUnknown: i32 = -1i32;
pub const SCEPDispositionSuccess: i32 = 0i32;
pub const SCEPDispositionFailure: i32 = 2i32;
pub const SCEPDispositionPending: i32 = 3i32;
pub const SCEPDispositionPendingChallenge: i32 = 11i32;
pub const SCEPFailUnknown: i32 = -1i32;
pub const SCEPFailBadAlgorithm: i32 = 0i32;
pub const SCEPFailBadMessageCheck: i32 = 1i32;
pub const SCEPFailBadRequest: i32 = 2i32;
pub const SCEPFailBadTime: i32 = 3i32;
pub const SCEPFailBadCertId: i32 = 4i32;
pub const SCEPMessageUnknown: i32 = -1i32;
pub const SCEPMessageCertResponse: i32 = 3i32;
pub const SCEPMessagePKCSRequest: i32 = 19i32;
pub const SCEPMessageGetCertInitial: i32 = 20i32;
pub const SCEPMessageGetCert: i32 = 21i32;
pub const SCEPMessageGetCRL: i32 = 22i32;
pub const SCEPMessageClaimChallengeAnswer: i32 = 41i32;
pub const SCEPProcessDefault: i32 = 0i32;
pub const SCEPProcessSkipCertInstall: i32 = 1i32;
pub const XECI_AUTOENROLL: u32 = 2u32;
pub const XECI_CERTREQ: u32 = 4u32;
pub const XECI_DISABLE: u32 = 0u32;
pub const XECI_REQWIZARD: u32 = 3u32;
pub const XECI_XENROLL: u32 = 1u32;
pub const XECP_STRING_PROPERTY: u32 = 1u32;
pub const XEKL_KEYSIZE_MIN: u32 = 1u32;
pub const XEKL_KEYSIZE_MAX: u32 = 2u32;
pub const XEKL_KEYSIZE_INC: u32 = 3u32;
pub const XEKL_KEYSIZE_DEFAULT: u32 = 4u32;
pub const XEKL_KEYSPEC_KEYX: u32 = 1u32;
pub const XEKL_KEYSPEC_SIG: u32 = 2u32;
pub const XEPR_DATE: u32 = 5u32;
pub const XEPR_ENUM_FIRST: i32 = -1i32;
pub const XEPR_TEMPLATENAME: u32 = 6u32;
pub const XEPR_V1TEMPLATENAME: u32 = 9u32;
pub const XEPR_V2TEMPLATEOID: u32 = 16u32;
pub const XEPR_VERSION: u32 = 7u32;
pub const wszCMM_PROP_COPYRIGHT: &'static str = "Copyright";
pub const wszCMM_PROP_DESCRIPTION: &'static str = "Description";
pub const wszCMM_PROP_DISPLAY_HWND: &'static str = "HWND";
pub const wszCMM_PROP_FILEVER: &'static str = "File Version";
pub const wszCMM_PROP_ISMULTITHREADED: &'static str = "IsMultiThreaded";
pub const wszCMM_PROP_NAME: &'static str = "Name";
pub const wszCMM_PROP_PRODUCTVER: &'static str = "Product Version";
