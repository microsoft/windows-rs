#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
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
#[repr(transparent)]
pub struct ADDED_CERT_TYPE(pub u32);
pub const XECT_EXTENSION_V1: ADDED_CERT_TYPE = ADDED_CERT_TYPE(1u32);
pub const XECT_EXTENSION_V2: ADDED_CERT_TYPE = ADDED_CERT_TYPE(2u32);
impl ::core::marker::Copy for ADDED_CERT_TYPE {}
impl ::core::clone::Clone for ADDED_CERT_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct AlgorithmFlags(pub i32);
pub const AlgorithmFlagsNone: AlgorithmFlags = AlgorithmFlags(0i32);
pub const AlgorithmFlagsWrap: AlgorithmFlags = AlgorithmFlags(1i32);
impl ::core::marker::Copy for AlgorithmFlags {}
impl ::core::clone::Clone for AlgorithmFlags {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct AlgorithmOperationFlags(pub i32);
pub const XCN_NCRYPT_NO_OPERATION: AlgorithmOperationFlags = AlgorithmOperationFlags(0i32);
pub const XCN_NCRYPT_CIPHER_OPERATION: AlgorithmOperationFlags = AlgorithmOperationFlags(1i32);
pub const XCN_NCRYPT_HASH_OPERATION: AlgorithmOperationFlags = AlgorithmOperationFlags(2i32);
pub const XCN_NCRYPT_ASYMMETRIC_ENCRYPTION_OPERATION: AlgorithmOperationFlags = AlgorithmOperationFlags(4i32);
pub const XCN_NCRYPT_SECRET_AGREEMENT_OPERATION: AlgorithmOperationFlags = AlgorithmOperationFlags(8i32);
pub const XCN_NCRYPT_SIGNATURE_OPERATION: AlgorithmOperationFlags = AlgorithmOperationFlags(16i32);
pub const XCN_NCRYPT_RNG_OPERATION: AlgorithmOperationFlags = AlgorithmOperationFlags(32i32);
pub const XCN_NCRYPT_KEY_DERIVATION_OPERATION: AlgorithmOperationFlags = AlgorithmOperationFlags(64i32);
pub const XCN_NCRYPT_ANY_ASYMMETRIC_OPERATION: AlgorithmOperationFlags = AlgorithmOperationFlags(28i32);
pub const XCN_NCRYPT_PREFER_SIGNATURE_ONLY_OPERATION: AlgorithmOperationFlags = AlgorithmOperationFlags(2097152i32);
pub const XCN_NCRYPT_PREFER_NON_SIGNATURE_OPERATION: AlgorithmOperationFlags = AlgorithmOperationFlags(4194304i32);
pub const XCN_NCRYPT_EXACT_MATCH_OPERATION: AlgorithmOperationFlags = AlgorithmOperationFlags(8388608i32);
pub const XCN_NCRYPT_PREFERENCE_MASK_OPERATION: AlgorithmOperationFlags = AlgorithmOperationFlags(14680064i32);
impl ::core::marker::Copy for AlgorithmOperationFlags {}
impl ::core::clone::Clone for AlgorithmOperationFlags {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct AlgorithmType(pub i32);
pub const XCN_BCRYPT_UNKNOWN_INTERFACE: AlgorithmType = AlgorithmType(0i32);
pub const XCN_BCRYPT_CIPHER_INTERFACE: AlgorithmType = AlgorithmType(1i32);
pub const XCN_BCRYPT_HASH_INTERFACE: AlgorithmType = AlgorithmType(2i32);
pub const XCN_BCRYPT_ASYMMETRIC_ENCRYPTION_INTERFACE: AlgorithmType = AlgorithmType(3i32);
pub const XCN_BCRYPT_SIGNATURE_INTERFACE: AlgorithmType = AlgorithmType(5i32);
pub const XCN_BCRYPT_SECRET_AGREEMENT_INTERFACE: AlgorithmType = AlgorithmType(4i32);
pub const XCN_BCRYPT_RNG_INTERFACE: AlgorithmType = AlgorithmType(6i32);
pub const XCN_BCRYPT_KEY_DERIVATION_INTERFACE: AlgorithmType = AlgorithmType(7i32);
impl ::core::marker::Copy for AlgorithmType {}
impl ::core::clone::Clone for AlgorithmType {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct AlternativeNameType(pub i32);
pub const XCN_CERT_ALT_NAME_UNKNOWN: AlternativeNameType = AlternativeNameType(0i32);
pub const XCN_CERT_ALT_NAME_OTHER_NAME: AlternativeNameType = AlternativeNameType(1i32);
pub const XCN_CERT_ALT_NAME_RFC822_NAME: AlternativeNameType = AlternativeNameType(2i32);
pub const XCN_CERT_ALT_NAME_DNS_NAME: AlternativeNameType = AlternativeNameType(3i32);
pub const XCN_CERT_ALT_NAME_X400_ADDRESS: AlternativeNameType = AlternativeNameType(4i32);
pub const XCN_CERT_ALT_NAME_DIRECTORY_NAME: AlternativeNameType = AlternativeNameType(5i32);
pub const XCN_CERT_ALT_NAME_EDI_PARTY_NAME: AlternativeNameType = AlternativeNameType(6i32);
pub const XCN_CERT_ALT_NAME_URL: AlternativeNameType = AlternativeNameType(7i32);
pub const XCN_CERT_ALT_NAME_IP_ADDRESS: AlternativeNameType = AlternativeNameType(8i32);
pub const XCN_CERT_ALT_NAME_REGISTERED_ID: AlternativeNameType = AlternativeNameType(9i32);
pub const XCN_CERT_ALT_NAME_GUID: AlternativeNameType = AlternativeNameType(10i32);
pub const XCN_CERT_ALT_NAME_USER_PRINCIPLE_NAME: AlternativeNameType = AlternativeNameType(11i32);
impl ::core::marker::Copy for AlternativeNameType {}
impl ::core::clone::Clone for AlternativeNameType {
    fn clone(&self) -> Self {
        *self
    }
}
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
pub const CAlternativeName: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 2286821395, data2: 8573, data3: 4570, data4: [178, 164, 0, 14, 123, 187, 43, 9] };
pub const CAlternativeNames: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 2286821396, data2: 8573, data3: 4570, data4: [178, 164, 0, 14, 123, 187, 43, 9] };
pub const CBinaryConverter: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 2286821378, data2: 8573, data3: 4570, data4: [178, 164, 0, 14, 123, 187, 43, 9] };
pub const CCLOCKSKEWMINUTESDEFAULT: u32 = 10u32;
pub const CC_UIPICKCONFIGSKIPLOCALCA: u32 = 5u32;
pub const CCertAdmin: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 938130160, data2: 32694, data3: 4560, data4: [136, 23, 0, 160, 201, 3, 184, 60] };
pub const CCertConfig: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 925879864, data2: 17188, data3: 4560, data4: [136, 16, 0, 160, 201, 3, 184, 60] };
pub const CCertEncodeAltName: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 486296794, data2: 4721, data3: 4561, data4: [155, 212, 0, 192, 79, 182, 131, 250] };
pub const CCertEncodeBitString: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 1835744472, data2: 4728, data3: 4561, data4: [155, 212, 0, 192, 79, 182, 131, 250] };
pub const CCertEncodeCRLDistInfo: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 33185952, data2: 48127, data3: 4560, data4: [136, 37, 0, 160, 201, 3, 184, 60] };
pub const CCertEncodeDateArray: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 807368624, data2: 42096, data3: 4560, data4: [136, 33, 0, 160, 201, 3, 184, 60] };
pub const CCertEncodeLongArray: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 1309048992, data2: 41122, data3: 4560, data4: [136, 33, 0, 160, 201, 3, 184, 60] };
pub const CCertEncodeStringArray: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 430403552, data2: 29844, data3: 4560, data4: [136, 22, 0, 160, 201, 3, 184, 60] };
pub const CCertGetConfig: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 3335276976, data2: 52759, data3: 4560, data4: [136, 51, 0, 160, 201, 3, 184, 60] };
pub const CCertProperties: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 2286821423, data2: 8573, data3: 4570, data4: [178, 164, 0, 14, 123, 187, 43, 9] };
pub const CCertProperty: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 2286821422, data2: 8573, data3: 4570, data4: [178, 164, 0, 14, 123, 187, 43, 9] };
pub const CCertPropertyArchived: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 2286821431, data2: 8573, data3: 4570, data4: [178, 164, 0, 14, 123, 187, 43, 9] };
pub const CCertPropertyArchivedKeyHash: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 2286821435, data2: 8573, data3: 4570, data4: [178, 164, 0, 14, 123, 187, 43, 9] };
pub const CCertPropertyAutoEnroll: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 2286821426, data2: 8573, data3: 4570, data4: [178, 164, 0, 14, 123, 187, 43, 9] };
pub const CCertPropertyBackedUp: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 2286821432, data2: 8573, data3: 4570, data4: [178, 164, 0, 14, 123, 187, 43, 9] };
pub const CCertPropertyDescription: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 2286821425, data2: 8573, data3: 4570, data4: [178, 164, 0, 14, 123, 187, 43, 9] };
pub const CCertPropertyEnrollment: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 2286821433, data2: 8573, data3: 4570, data4: [178, 164, 0, 14, 123, 187, 43, 9] };
pub const CCertPropertyEnrollmentPolicyServer: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 2286821452, data2: 8573, data3: 4570, data4: [178, 164, 0, 14, 123, 187, 43, 9] };
pub const CCertPropertyFriendlyName: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 2286821424, data2: 8573, data3: 4570, data4: [178, 164, 0, 14, 123, 187, 43, 9] };
pub const CCertPropertyKeyProvInfo: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 2286821430, data2: 8573, data3: 4570, data4: [178, 164, 0, 14, 123, 187, 43, 9] };
pub const CCertPropertyRenewal: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 2286821434, data2: 8573, data3: 4570, data4: [178, 164, 0, 14, 123, 187, 43, 9] };
pub const CCertPropertyRequestOriginator: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 2286821427, data2: 8573, data3: 4570, data4: [178, 164, 0, 14, 123, 187, 43, 9] };
pub const CCertPropertySHA1Hash: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 2286821428, data2: 8573, data3: 4570, data4: [178, 164, 0, 14, 123, 187, 43, 9] };
pub const CCertRequest: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 2561668080, data2: 21796, data3: 4560, data4: [136, 18, 0, 160, 201, 3, 184, 60] };
pub const CCertServerExit: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 1279942208, data2: 29484, data3: 4560, data4: [136, 22, 0, 160, 201, 3, 184, 60] };
pub const CCertServerPolicy: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 2852129062, data2: 65470, data3: 4559, data4: [136, 0, 0, 160, 201, 3, 184, 60] };
pub const CCertView: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 2704084858, data2: 7812, data3: 4561, data4: [155, 214, 0, 192, 79, 182, 131, 250] };
pub const CCertificateAttestationChallenge: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 325234081, data2: 60256, data3: 17770, data4: [182, 225, 17, 128, 80, 219, 116, 27] };
pub const CCertificatePolicies: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 2286821407, data2: 8573, data3: 4570, data4: [178, 164, 0, 14, 123, 187, 43, 9] };
pub const CCertificatePolicy: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 2286821406, data2: 8573, data3: 4570, data4: [178, 164, 0, 14, 123, 187, 43, 9] };
pub const CCryptAttribute: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 2286821420, data2: 8573, data3: 4570, data4: [178, 164, 0, 14, 123, 187, 43, 9] };
pub const CCryptAttributes: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 2286821421, data2: 8573, data3: 4570, data4: [178, 164, 0, 14, 123, 187, 43, 9] };
pub const CCspInformation: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 2286821383, data2: 8573, data3: 4570, data4: [178, 164, 0, 14, 123, 187, 43, 9] };
pub const CCspInformations: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 2286821384, data2: 8573, data3: 4570, data4: [178, 164, 0, 14, 123, 187, 43, 9] };
pub const CCspStatus: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 2286821385, data2: 8573, data3: 4570, data4: [178, 164, 0, 14, 123, 187, 43, 9] };
#[repr(transparent)]
pub struct CERTADMIN_GET_ROLES_FLAGS(pub u32);
pub const CA_ACCESS_ADMIN: CERTADMIN_GET_ROLES_FLAGS = CERTADMIN_GET_ROLES_FLAGS(1u32);
pub const CA_ACCESS_AUDITOR: CERTADMIN_GET_ROLES_FLAGS = CERTADMIN_GET_ROLES_FLAGS(4u32);
pub const CA_ACCESS_ENROLL: CERTADMIN_GET_ROLES_FLAGS = CERTADMIN_GET_ROLES_FLAGS(512u32);
pub const CA_ACCESS_OFFICER: CERTADMIN_GET_ROLES_FLAGS = CERTADMIN_GET_ROLES_FLAGS(2u32);
pub const CA_ACCESS_OPERATOR: CERTADMIN_GET_ROLES_FLAGS = CERTADMIN_GET_ROLES_FLAGS(8u32);
pub const CA_ACCESS_READ: CERTADMIN_GET_ROLES_FLAGS = CERTADMIN_GET_ROLES_FLAGS(256u32);
impl ::core::marker::Copy for CERTADMIN_GET_ROLES_FLAGS {}
impl ::core::clone::Clone for CERTADMIN_GET_ROLES_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
pub const CERTENROLL_INDEX_BASE: u32 = 0u32;
#[repr(transparent)]
pub struct CERTENROLL_OBJECTID(pub i32);
pub const XCN_OID_NONE: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(0i32);
pub const XCN_OID_RSA: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(1i32);
pub const XCN_OID_PKCS: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(2i32);
pub const XCN_OID_RSA_HASH: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(3i32);
pub const XCN_OID_RSA_ENCRYPT: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(4i32);
pub const XCN_OID_PKCS_1: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(5i32);
pub const XCN_OID_PKCS_2: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(6i32);
pub const XCN_OID_PKCS_3: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(7i32);
pub const XCN_OID_PKCS_4: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(8i32);
pub const XCN_OID_PKCS_5: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(9i32);
pub const XCN_OID_PKCS_6: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(10i32);
pub const XCN_OID_PKCS_7: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(11i32);
pub const XCN_OID_PKCS_8: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(12i32);
pub const XCN_OID_PKCS_9: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(13i32);
pub const XCN_OID_PKCS_10: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(14i32);
pub const XCN_OID_PKCS_12: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(15i32);
pub const XCN_OID_RSA_RSA: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(16i32);
pub const XCN_OID_RSA_MD2RSA: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(17i32);
pub const XCN_OID_RSA_MD4RSA: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(18i32);
pub const XCN_OID_RSA_MD5RSA: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(19i32);
pub const XCN_OID_RSA_SHA1RSA: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(20i32);
pub const XCN_OID_RSA_SETOAEP_RSA: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(21i32);
pub const XCN_OID_RSA_DH: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(22i32);
pub const XCN_OID_RSA_data: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(23i32);
pub const XCN_OID_RSA_signedData: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(24i32);
pub const XCN_OID_RSA_envelopedData: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(25i32);
pub const XCN_OID_RSA_signEnvData: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(26i32);
pub const XCN_OID_RSA_digestedData: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(27i32);
pub const XCN_OID_RSA_hashedData: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(28i32);
pub const XCN_OID_RSA_encryptedData: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(29i32);
pub const XCN_OID_RSA_emailAddr: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(30i32);
pub const XCN_OID_RSA_unstructName: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(31i32);
pub const XCN_OID_RSA_contentType: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(32i32);
pub const XCN_OID_RSA_messageDigest: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(33i32);
pub const XCN_OID_RSA_signingTime: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(34i32);
pub const XCN_OID_RSA_counterSign: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(35i32);
pub const XCN_OID_RSA_challengePwd: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(36i32);
pub const XCN_OID_RSA_unstructAddr: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(37i32);
pub const XCN_OID_RSA_extCertAttrs: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(38i32);
pub const XCN_OID_RSA_certExtensions: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(39i32);
pub const XCN_OID_RSA_SMIMECapabilities: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(40i32);
pub const XCN_OID_RSA_preferSignedData: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(41i32);
pub const XCN_OID_RSA_SMIMEalg: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(42i32);
pub const XCN_OID_RSA_SMIMEalgESDH: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(43i32);
pub const XCN_OID_RSA_SMIMEalgCMS3DESwrap: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(44i32);
pub const XCN_OID_RSA_SMIMEalgCMSRC2wrap: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(45i32);
pub const XCN_OID_RSA_MD2: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(46i32);
pub const XCN_OID_RSA_MD4: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(47i32);
pub const XCN_OID_RSA_MD5: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(48i32);
pub const XCN_OID_RSA_RC2CBC: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(49i32);
pub const XCN_OID_RSA_RC4: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(50i32);
pub const XCN_OID_RSA_DES_EDE3_CBC: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(51i32);
pub const XCN_OID_RSA_RC5_CBCPad: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(52i32);
pub const XCN_OID_ANSI_X942: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(53i32);
pub const XCN_OID_ANSI_X942_DH: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(54i32);
pub const XCN_OID_X957: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(55i32);
pub const XCN_OID_X957_DSA: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(56i32);
pub const XCN_OID_X957_SHA1DSA: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(57i32);
pub const XCN_OID_DS: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(58i32);
pub const XCN_OID_DSALG: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(59i32);
pub const XCN_OID_DSALG_CRPT: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(60i32);
pub const XCN_OID_DSALG_HASH: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(61i32);
pub const XCN_OID_DSALG_SIGN: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(62i32);
pub const XCN_OID_DSALG_RSA: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(63i32);
pub const XCN_OID_OIW: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(64i32);
pub const XCN_OID_OIWSEC: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(65i32);
pub const XCN_OID_OIWSEC_md4RSA: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(66i32);
pub const XCN_OID_OIWSEC_md5RSA: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(67i32);
pub const XCN_OID_OIWSEC_md4RSA2: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(68i32);
pub const XCN_OID_OIWSEC_desECB: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(69i32);
pub const XCN_OID_OIWSEC_desCBC: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(70i32);
pub const XCN_OID_OIWSEC_desOFB: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(71i32);
pub const XCN_OID_OIWSEC_desCFB: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(72i32);
pub const XCN_OID_OIWSEC_desMAC: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(73i32);
pub const XCN_OID_OIWSEC_rsaSign: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(74i32);
pub const XCN_OID_OIWSEC_dsa: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(75i32);
pub const XCN_OID_OIWSEC_shaDSA: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(76i32);
pub const XCN_OID_OIWSEC_mdc2RSA: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(77i32);
pub const XCN_OID_OIWSEC_shaRSA: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(78i32);
pub const XCN_OID_OIWSEC_dhCommMod: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(79i32);
pub const XCN_OID_OIWSEC_desEDE: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(80i32);
pub const XCN_OID_OIWSEC_sha: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(81i32);
pub const XCN_OID_OIWSEC_mdc2: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(82i32);
pub const XCN_OID_OIWSEC_dsaComm: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(83i32);
pub const XCN_OID_OIWSEC_dsaCommSHA: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(84i32);
pub const XCN_OID_OIWSEC_rsaXchg: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(85i32);
pub const XCN_OID_OIWSEC_keyHashSeal: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(86i32);
pub const XCN_OID_OIWSEC_md2RSASign: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(87i32);
pub const XCN_OID_OIWSEC_md5RSASign: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(88i32);
pub const XCN_OID_OIWSEC_sha1: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(89i32);
pub const XCN_OID_OIWSEC_dsaSHA1: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(90i32);
pub const XCN_OID_OIWSEC_dsaCommSHA1: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(91i32);
pub const XCN_OID_OIWSEC_sha1RSASign: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(92i32);
pub const XCN_OID_OIWDIR: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(93i32);
pub const XCN_OID_OIWDIR_CRPT: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(94i32);
pub const XCN_OID_OIWDIR_HASH: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(95i32);
pub const XCN_OID_OIWDIR_SIGN: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(96i32);
pub const XCN_OID_OIWDIR_md2: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(97i32);
pub const XCN_OID_OIWDIR_md2RSA: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(98i32);
pub const XCN_OID_INFOSEC: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(99i32);
pub const XCN_OID_INFOSEC_sdnsSignature: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(100i32);
pub const XCN_OID_INFOSEC_mosaicSignature: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(101i32);
pub const XCN_OID_INFOSEC_sdnsConfidentiality: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(102i32);
pub const XCN_OID_INFOSEC_mosaicConfidentiality: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(103i32);
pub const XCN_OID_INFOSEC_sdnsIntegrity: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(104i32);
pub const XCN_OID_INFOSEC_mosaicIntegrity: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(105i32);
pub const XCN_OID_INFOSEC_sdnsTokenProtection: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(106i32);
pub const XCN_OID_INFOSEC_mosaicTokenProtection: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(107i32);
pub const XCN_OID_INFOSEC_sdnsKeyManagement: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(108i32);
pub const XCN_OID_INFOSEC_mosaicKeyManagement: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(109i32);
pub const XCN_OID_INFOSEC_sdnsKMandSig: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(110i32);
pub const XCN_OID_INFOSEC_mosaicKMandSig: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(111i32);
pub const XCN_OID_INFOSEC_SuiteASignature: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(112i32);
pub const XCN_OID_INFOSEC_SuiteAConfidentiality: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(113i32);
pub const XCN_OID_INFOSEC_SuiteAIntegrity: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(114i32);
pub const XCN_OID_INFOSEC_SuiteATokenProtection: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(115i32);
pub const XCN_OID_INFOSEC_SuiteAKeyManagement: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(116i32);
pub const XCN_OID_INFOSEC_SuiteAKMandSig: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(117i32);
pub const XCN_OID_INFOSEC_mosaicUpdatedSig: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(118i32);
pub const XCN_OID_INFOSEC_mosaicKMandUpdSig: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(119i32);
pub const XCN_OID_INFOSEC_mosaicUpdatedInteg: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(120i32);
pub const XCN_OID_COMMON_NAME: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(121i32);
pub const XCN_OID_SUR_NAME: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(122i32);
pub const XCN_OID_DEVICE_SERIAL_NUMBER: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(123i32);
pub const XCN_OID_COUNTRY_NAME: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(124i32);
pub const XCN_OID_LOCALITY_NAME: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(125i32);
pub const XCN_OID_STATE_OR_PROVINCE_NAME: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(126i32);
pub const XCN_OID_STREET_ADDRESS: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(127i32);
pub const XCN_OID_ORGANIZATION_NAME: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(128i32);
pub const XCN_OID_ORGANIZATIONAL_UNIT_NAME: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(129i32);
pub const XCN_OID_TITLE: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(130i32);
pub const XCN_OID_DESCRIPTION: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(131i32);
pub const XCN_OID_SEARCH_GUIDE: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(132i32);
pub const XCN_OID_BUSINESS_CATEGORY: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(133i32);
pub const XCN_OID_POSTAL_ADDRESS: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(134i32);
pub const XCN_OID_POSTAL_CODE: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(135i32);
pub const XCN_OID_POST_OFFICE_BOX: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(136i32);
pub const XCN_OID_PHYSICAL_DELIVERY_OFFICE_NAME: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(137i32);
pub const XCN_OID_TELEPHONE_NUMBER: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(138i32);
pub const XCN_OID_TELEX_NUMBER: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(139i32);
pub const XCN_OID_TELETEXT_TERMINAL_IDENTIFIER: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(140i32);
pub const XCN_OID_FACSIMILE_TELEPHONE_NUMBER: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(141i32);
pub const XCN_OID_X21_ADDRESS: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(142i32);
pub const XCN_OID_INTERNATIONAL_ISDN_NUMBER: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(143i32);
pub const XCN_OID_REGISTERED_ADDRESS: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(144i32);
pub const XCN_OID_DESTINATION_INDICATOR: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(145i32);
pub const XCN_OID_PREFERRED_DELIVERY_METHOD: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(146i32);
pub const XCN_OID_PRESENTATION_ADDRESS: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(147i32);
pub const XCN_OID_SUPPORTED_APPLICATION_CONTEXT: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(148i32);
pub const XCN_OID_MEMBER: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(149i32);
pub const XCN_OID_OWNER: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(150i32);
pub const XCN_OID_ROLE_OCCUPANT: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(151i32);
pub const XCN_OID_SEE_ALSO: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(152i32);
pub const XCN_OID_USER_PASSWORD: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(153i32);
pub const XCN_OID_USER_CERTIFICATE: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(154i32);
pub const XCN_OID_CA_CERTIFICATE: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(155i32);
pub const XCN_OID_AUTHORITY_REVOCATION_LIST: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(156i32);
pub const XCN_OID_CERTIFICATE_REVOCATION_LIST: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(157i32);
pub const XCN_OID_CROSS_CERTIFICATE_PAIR: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(158i32);
pub const XCN_OID_GIVEN_NAME: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(159i32);
pub const XCN_OID_INITIALS: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(160i32);
pub const XCN_OID_DN_QUALIFIER: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(161i32);
pub const XCN_OID_DOMAIN_COMPONENT: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(162i32);
pub const XCN_OID_PKCS_12_FRIENDLY_NAME_ATTR: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(163i32);
pub const XCN_OID_PKCS_12_LOCAL_KEY_ID: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(164i32);
pub const XCN_OID_PKCS_12_KEY_PROVIDER_NAME_ATTR: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(165i32);
pub const XCN_OID_LOCAL_MACHINE_KEYSET: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(166i32);
pub const XCN_OID_PKCS_12_EXTENDED_ATTRIBUTES: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(167i32);
pub const XCN_OID_KEYID_RDN: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(168i32);
pub const XCN_OID_AUTHORITY_KEY_IDENTIFIER: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(169i32);
pub const XCN_OID_KEY_ATTRIBUTES: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(170i32);
pub const XCN_OID_CERT_POLICIES_95: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(171i32);
pub const XCN_OID_KEY_USAGE_RESTRICTION: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(172i32);
pub const XCN_OID_SUBJECT_ALT_NAME: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(173i32);
pub const XCN_OID_ISSUER_ALT_NAME: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(174i32);
pub const XCN_OID_BASIC_CONSTRAINTS: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(175i32);
pub const XCN_OID_KEY_USAGE: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(176i32);
pub const XCN_OID_PRIVATEKEY_USAGE_PERIOD: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(177i32);
pub const XCN_OID_BASIC_CONSTRAINTS2: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(178i32);
pub const XCN_OID_CERT_POLICIES: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(179i32);
pub const XCN_OID_ANY_CERT_POLICY: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(180i32);
pub const XCN_OID_AUTHORITY_KEY_IDENTIFIER2: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(181i32);
pub const XCN_OID_SUBJECT_KEY_IDENTIFIER: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(182i32);
pub const XCN_OID_SUBJECT_ALT_NAME2: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(183i32);
pub const XCN_OID_ISSUER_ALT_NAME2: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(184i32);
pub const XCN_OID_CRL_REASON_CODE: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(185i32);
pub const XCN_OID_REASON_CODE_HOLD: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(186i32);
pub const XCN_OID_CRL_DIST_POINTS: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(187i32);
pub const XCN_OID_ENHANCED_KEY_USAGE: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(188i32);
pub const XCN_OID_CRL_NUMBER: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(189i32);
pub const XCN_OID_DELTA_CRL_INDICATOR: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(190i32);
pub const XCN_OID_ISSUING_DIST_POINT: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(191i32);
pub const XCN_OID_FRESHEST_CRL: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(192i32);
pub const XCN_OID_NAME_CONSTRAINTS: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(193i32);
pub const XCN_OID_POLICY_MAPPINGS: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(194i32);
pub const XCN_OID_LEGACY_POLICY_MAPPINGS: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(195i32);
pub const XCN_OID_POLICY_CONSTRAINTS: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(196i32);
pub const XCN_OID_RENEWAL_CERTIFICATE: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(197i32);
pub const XCN_OID_ENROLLMENT_NAME_VALUE_PAIR: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(198i32);
pub const XCN_OID_ENROLLMENT_CSP_PROVIDER: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(199i32);
pub const XCN_OID_OS_VERSION: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(200i32);
pub const XCN_OID_ENROLLMENT_AGENT: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(201i32);
pub const XCN_OID_PKIX: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(202i32);
pub const XCN_OID_PKIX_PE: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(203i32);
pub const XCN_OID_AUTHORITY_INFO_ACCESS: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(204i32);
pub const XCN_OID_BIOMETRIC_EXT: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(205i32);
pub const XCN_OID_LOGOTYPE_EXT: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(206i32);
pub const XCN_OID_CERT_EXTENSIONS: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(207i32);
pub const XCN_OID_NEXT_UPDATE_LOCATION: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(208i32);
pub const XCN_OID_REMOVE_CERTIFICATE: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(209i32);
pub const XCN_OID_CROSS_CERT_DIST_POINTS: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(210i32);
pub const XCN_OID_CTL: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(211i32);
pub const XCN_OID_SORTED_CTL: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(212i32);
pub const XCN_OID_SERIALIZED: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(213i32);
pub const XCN_OID_NT_PRINCIPAL_NAME: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(214i32);
pub const XCN_OID_PRODUCT_UPDATE: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(215i32);
pub const XCN_OID_ANY_APPLICATION_POLICY: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(216i32);
pub const XCN_OID_AUTO_ENROLL_CTL_USAGE: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(217i32);
pub const XCN_OID_ENROLL_CERTTYPE_EXTENSION: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(218i32);
pub const XCN_OID_CERT_MANIFOLD: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(219i32);
pub const XCN_OID_CERTSRV_CA_VERSION: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(220i32);
pub const XCN_OID_CERTSRV_PREVIOUS_CERT_HASH: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(221i32);
pub const XCN_OID_CRL_VIRTUAL_BASE: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(222i32);
pub const XCN_OID_CRL_NEXT_PUBLISH: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(223i32);
pub const XCN_OID_KP_CA_EXCHANGE: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(224i32);
pub const XCN_OID_KP_KEY_RECOVERY_AGENT: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(225i32);
pub const XCN_OID_CERTIFICATE_TEMPLATE: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(226i32);
pub const XCN_OID_ENTERPRISE_OID_ROOT: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(227i32);
pub const XCN_OID_RDN_DUMMY_SIGNER: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(228i32);
pub const XCN_OID_APPLICATION_CERT_POLICIES: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(229i32);
pub const XCN_OID_APPLICATION_POLICY_MAPPINGS: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(230i32);
pub const XCN_OID_APPLICATION_POLICY_CONSTRAINTS: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(231i32);
pub const XCN_OID_ARCHIVED_KEY_ATTR: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(232i32);
pub const XCN_OID_CRL_SELF_CDP: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(233i32);
pub const XCN_OID_REQUIRE_CERT_CHAIN_POLICY: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(234i32);
pub const XCN_OID_ARCHIVED_KEY_CERT_HASH: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(235i32);
pub const XCN_OID_ISSUED_CERT_HASH: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(236i32);
pub const XCN_OID_DS_EMAIL_REPLICATION: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(237i32);
pub const XCN_OID_REQUEST_CLIENT_INFO: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(238i32);
pub const XCN_OID_ENCRYPTED_KEY_HASH: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(239i32);
pub const XCN_OID_CERTSRV_CROSSCA_VERSION: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(240i32);
pub const XCN_OID_NTDS_REPLICATION: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(241i32);
pub const XCN_OID_SUBJECT_DIR_ATTRS: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(242i32);
pub const XCN_OID_PKIX_KP: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(243i32);
pub const XCN_OID_PKIX_KP_SERVER_AUTH: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(244i32);
pub const XCN_OID_PKIX_KP_CLIENT_AUTH: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(245i32);
pub const XCN_OID_PKIX_KP_CODE_SIGNING: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(246i32);
pub const XCN_OID_PKIX_KP_EMAIL_PROTECTION: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(247i32);
pub const XCN_OID_PKIX_KP_IPSEC_END_SYSTEM: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(248i32);
pub const XCN_OID_PKIX_KP_IPSEC_TUNNEL: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(249i32);
pub const XCN_OID_PKIX_KP_IPSEC_USER: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(250i32);
pub const XCN_OID_PKIX_KP_TIMESTAMP_SIGNING: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(251i32);
pub const XCN_OID_PKIX_KP_OCSP_SIGNING: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(252i32);
pub const XCN_OID_PKIX_OCSP_NOCHECK: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(253i32);
pub const XCN_OID_IPSEC_KP_IKE_INTERMEDIATE: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(254i32);
pub const XCN_OID_KP_CTL_USAGE_SIGNING: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(255i32);
pub const XCN_OID_KP_TIME_STAMP_SIGNING: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(256i32);
pub const XCN_OID_SERVER_GATED_CRYPTO: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(257i32);
pub const XCN_OID_SGC_NETSCAPE: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(258i32);
pub const XCN_OID_KP_EFS: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(259i32);
pub const XCN_OID_EFS_RECOVERY: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(260i32);
pub const XCN_OID_WHQL_CRYPTO: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(261i32);
pub const XCN_OID_NT5_CRYPTO: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(262i32);
pub const XCN_OID_OEM_WHQL_CRYPTO: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(263i32);
pub const XCN_OID_EMBEDDED_NT_CRYPTO: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(264i32);
pub const XCN_OID_ROOT_LIST_SIGNER: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(265i32);
pub const XCN_OID_KP_QUALIFIED_SUBORDINATION: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(266i32);
pub const XCN_OID_KP_KEY_RECOVERY: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(267i32);
pub const XCN_OID_KP_DOCUMENT_SIGNING: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(268i32);
pub const XCN_OID_KP_LIFETIME_SIGNING: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(269i32);
pub const XCN_OID_KP_MOBILE_DEVICE_SOFTWARE: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(270i32);
pub const XCN_OID_KP_SMART_DISPLAY: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(271i32);
pub const XCN_OID_KP_CSP_SIGNATURE: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(272i32);
pub const XCN_OID_DRM: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(273i32);
pub const XCN_OID_DRM_INDIVIDUALIZATION: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(274i32);
pub const XCN_OID_LICENSES: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(275i32);
pub const XCN_OID_LICENSE_SERVER: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(276i32);
pub const XCN_OID_KP_SMARTCARD_LOGON: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(277i32);
pub const XCN_OID_YESNO_TRUST_ATTR: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(278i32);
pub const XCN_OID_PKIX_POLICY_QUALIFIER_CPS: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(279i32);
pub const XCN_OID_PKIX_POLICY_QUALIFIER_USERNOTICE: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(280i32);
pub const XCN_OID_CERT_POLICIES_95_QUALIFIER1: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(281i32);
pub const XCN_OID_PKIX_ACC_DESCR: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(282i32);
pub const XCN_OID_PKIX_OCSP: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(283i32);
pub const XCN_OID_PKIX_CA_ISSUERS: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(284i32);
pub const XCN_OID_VERISIGN_PRIVATE_6_9: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(285i32);
pub const XCN_OID_VERISIGN_ONSITE_JURISDICTION_HASH: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(286i32);
pub const XCN_OID_VERISIGN_BITSTRING_6_13: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(287i32);
pub const XCN_OID_VERISIGN_ISS_STRONG_CRYPTO: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(288i32);
pub const XCN_OID_NETSCAPE: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(289i32);
pub const XCN_OID_NETSCAPE_CERT_EXTENSION: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(290i32);
pub const XCN_OID_NETSCAPE_CERT_TYPE: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(291i32);
pub const XCN_OID_NETSCAPE_BASE_URL: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(292i32);
pub const XCN_OID_NETSCAPE_REVOCATION_URL: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(293i32);
pub const XCN_OID_NETSCAPE_CA_REVOCATION_URL: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(294i32);
pub const XCN_OID_NETSCAPE_CERT_RENEWAL_URL: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(295i32);
pub const XCN_OID_NETSCAPE_CA_POLICY_URL: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(296i32);
pub const XCN_OID_NETSCAPE_SSL_SERVER_NAME: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(297i32);
pub const XCN_OID_NETSCAPE_COMMENT: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(298i32);
pub const XCN_OID_NETSCAPE_DATA_TYPE: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(299i32);
pub const XCN_OID_NETSCAPE_CERT_SEQUENCE: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(300i32);
pub const XCN_OID_CT_PKI_DATA: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(301i32);
pub const XCN_OID_CT_PKI_RESPONSE: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(302i32);
pub const XCN_OID_PKIX_NO_SIGNATURE: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(303i32);
pub const XCN_OID_CMC: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(304i32);
pub const XCN_OID_CMC_STATUS_INFO: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(305i32);
pub const XCN_OID_CMC_IDENTIFICATION: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(306i32);
pub const XCN_OID_CMC_IDENTITY_PROOF: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(307i32);
pub const XCN_OID_CMC_DATA_RETURN: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(308i32);
pub const XCN_OID_CMC_TRANSACTION_ID: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(309i32);
pub const XCN_OID_CMC_SENDER_NONCE: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(310i32);
pub const XCN_OID_CMC_RECIPIENT_NONCE: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(311i32);
pub const XCN_OID_CMC_ADD_EXTENSIONS: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(312i32);
pub const XCN_OID_CMC_ENCRYPTED_POP: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(313i32);
pub const XCN_OID_CMC_DECRYPTED_POP: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(314i32);
pub const XCN_OID_CMC_LRA_POP_WITNESS: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(315i32);
pub const XCN_OID_CMC_GET_CERT: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(316i32);
pub const XCN_OID_CMC_GET_CRL: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(317i32);
pub const XCN_OID_CMC_REVOKE_REQUEST: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(318i32);
pub const XCN_OID_CMC_REG_INFO: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(319i32);
pub const XCN_OID_CMC_RESPONSE_INFO: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(320i32);
pub const XCN_OID_CMC_QUERY_PENDING: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(321i32);
pub const XCN_OID_CMC_ID_POP_LINK_RANDOM: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(322i32);
pub const XCN_OID_CMC_ID_POP_LINK_WITNESS: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(323i32);
pub const XCN_OID_CMC_ID_CONFIRM_CERT_ACCEPTANCE: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(324i32);
pub const XCN_OID_CMC_ADD_ATTRIBUTES: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(325i32);
pub const XCN_OID_LOYALTY_OTHER_LOGOTYPE: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(326i32);
pub const XCN_OID_BACKGROUND_OTHER_LOGOTYPE: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(327i32);
pub const XCN_OID_PKIX_OCSP_BASIC_SIGNED_RESPONSE: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(328i32);
pub const XCN_OID_PKCS_7_DATA: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(329i32);
pub const XCN_OID_PKCS_7_SIGNED: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(330i32);
pub const XCN_OID_PKCS_7_ENVELOPED: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(331i32);
pub const XCN_OID_PKCS_7_SIGNEDANDENVELOPED: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(332i32);
pub const XCN_OID_PKCS_7_DIGESTED: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(333i32);
pub const XCN_OID_PKCS_7_ENCRYPTED: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(334i32);
pub const XCN_OID_PKCS_9_CONTENT_TYPE: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(335i32);
pub const XCN_OID_PKCS_9_MESSAGE_DIGEST: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(336i32);
pub const XCN_OID_CERT_PROP_ID_PREFIX: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(337i32);
pub const XCN_OID_CERT_KEY_IDENTIFIER_PROP_ID: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(338i32);
pub const XCN_OID_CERT_ISSUER_SERIAL_NUMBER_MD5_HASH_PROP_ID: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(339i32);
pub const XCN_OID_CERT_SUBJECT_NAME_MD5_HASH_PROP_ID: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(340i32);
pub const XCN_OID_CERT_MD5_HASH_PROP_ID: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(341i32);
pub const XCN_OID_RSA_SHA256RSA: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(342i32);
pub const XCN_OID_RSA_SHA384RSA: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(343i32);
pub const XCN_OID_RSA_SHA512RSA: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(344i32);
pub const XCN_OID_NIST_sha256: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(345i32);
pub const XCN_OID_NIST_sha384: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(346i32);
pub const XCN_OID_NIST_sha512: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(347i32);
pub const XCN_OID_RSA_MGF1: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(348i32);
pub const XCN_OID_ECC_PUBLIC_KEY: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(349i32);
pub const XCN_OID_ECDSA_SHA1: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(350i32);
pub const XCN_OID_ECDSA_SPECIFIED: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(351i32);
pub const XCN_OID_ANY_ENHANCED_KEY_USAGE: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(352i32);
pub const XCN_OID_RSA_SSA_PSS: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(353i32);
pub const XCN_OID_ATTR_SUPPORTED_ALGORITHMS: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(355i32);
pub const XCN_OID_ATTR_TPM_SECURITY_ASSERTIONS: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(356i32);
pub const XCN_OID_ATTR_TPM_SPECIFICATION: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(357i32);
pub const XCN_OID_CERT_DISALLOWED_FILETIME_PROP_ID: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(358i32);
pub const XCN_OID_CERT_SIGNATURE_HASH_PROP_ID: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(359i32);
pub const XCN_OID_CERT_STRONG_KEY_OS_1: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(360i32);
pub const XCN_OID_CERT_STRONG_KEY_OS_CURRENT: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(361i32);
pub const XCN_OID_CERT_STRONG_KEY_OS_PREFIX: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(362i32);
pub const XCN_OID_CERT_STRONG_SIGN_OS_1: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(363i32);
pub const XCN_OID_CERT_STRONG_SIGN_OS_CURRENT: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(364i32);
pub const XCN_OID_CERT_STRONG_SIGN_OS_PREFIX: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(365i32);
pub const XCN_OID_DH_SINGLE_PASS_STDDH_SHA1_KDF: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(366i32);
pub const XCN_OID_DH_SINGLE_PASS_STDDH_SHA256_KDF: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(367i32);
pub const XCN_OID_DH_SINGLE_PASS_STDDH_SHA384_KDF: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(368i32);
pub const XCN_OID_DISALLOWED_HASH: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(369i32);
pub const XCN_OID_DISALLOWED_LIST: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(370i32);
pub const XCN_OID_ECC_CURVE_P256: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(371i32);
pub const XCN_OID_ECC_CURVE_P384: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(372i32);
pub const XCN_OID_ECC_CURVE_P521: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(373i32);
pub const XCN_OID_ECDSA_SHA256: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(374i32);
pub const XCN_OID_ECDSA_SHA384: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(375i32);
pub const XCN_OID_ECDSA_SHA512: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(376i32);
pub const XCN_OID_ENROLL_CAXCHGCERT_HASH: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(377i32);
pub const XCN_OID_ENROLL_EK_INFO: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(378i32);
pub const XCN_OID_ENROLL_EKPUB_CHALLENGE: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(379i32);
pub const XCN_OID_ENROLL_EKVERIFYCERT: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(380i32);
pub const XCN_OID_ENROLL_EKVERIFYCREDS: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(381i32);
pub const XCN_OID_ENROLL_EKVERIFYKEY: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(382i32);
pub const XCN_OID_EV_RDN_COUNTRY: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(383i32);
pub const XCN_OID_EV_RDN_LOCALE: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(384i32);
pub const XCN_OID_EV_RDN_STATE_OR_PROVINCE: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(385i32);
pub const XCN_OID_INHIBIT_ANY_POLICY: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(386i32);
pub const XCN_OID_INTERNATIONALIZED_EMAIL_ADDRESS: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(387i32);
pub const XCN_OID_KP_KERNEL_MODE_CODE_SIGNING: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(388i32);
pub const XCN_OID_KP_KERNEL_MODE_HAL_EXTENSION_SIGNING: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(389i32);
pub const XCN_OID_KP_KERNEL_MODE_TRUSTED_BOOT_SIGNING: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(390i32);
pub const XCN_OID_KP_TPM_AIK_CERTIFICATE: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(391i32);
pub const XCN_OID_KP_TPM_EK_CERTIFICATE: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(392i32);
pub const XCN_OID_KP_TPM_PLATFORM_CERTIFICATE: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(393i32);
pub const XCN_OID_NIST_AES128_CBC: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(394i32);
pub const XCN_OID_NIST_AES128_WRAP: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(395i32);
pub const XCN_OID_NIST_AES192_CBC: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(396i32);
pub const XCN_OID_NIST_AES192_WRAP: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(397i32);
pub const XCN_OID_NIST_AES256_CBC: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(398i32);
pub const XCN_OID_NIST_AES256_WRAP: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(399i32);
pub const XCN_OID_PKCS_12_PbeIds: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(400i32);
pub const XCN_OID_PKCS_12_pbeWithSHA1And128BitRC2: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(401i32);
pub const XCN_OID_PKCS_12_pbeWithSHA1And128BitRC4: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(402i32);
pub const XCN_OID_PKCS_12_pbeWithSHA1And2KeyTripleDES: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(403i32);
pub const XCN_OID_PKCS_12_pbeWithSHA1And3KeyTripleDES: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(404i32);
pub const XCN_OID_PKCS_12_pbeWithSHA1And40BitRC2: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(405i32);
pub const XCN_OID_PKCS_12_pbeWithSHA1And40BitRC4: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(406i32);
pub const XCN_OID_PKCS_12_PROTECTED_PASSWORD_SECRET_BAG_TYPE_ID: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(407i32);
pub const XCN_OID_PKINIT_KP_KDC: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(408i32);
pub const XCN_OID_PKIX_CA_REPOSITORY: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(409i32);
pub const XCN_OID_PKIX_OCSP_NONCE: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(410i32);
pub const XCN_OID_PKIX_TIME_STAMPING: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(411i32);
pub const XCN_OID_QC_EU_COMPLIANCE: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(412i32);
pub const XCN_OID_QC_SSCD: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(413i32);
pub const XCN_OID_QC_STATEMENTS_EXT: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(414i32);
pub const XCN_OID_RDN_TPM_MANUFACTURER: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(415i32);
pub const XCN_OID_RDN_TPM_MODEL: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(416i32);
pub const XCN_OID_RDN_TPM_VERSION: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(417i32);
pub const XCN_OID_REVOKED_LIST_SIGNER: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(418i32);
pub const XCN_OID_RFC3161_counterSign: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(419i32);
pub const XCN_OID_ROOT_PROGRAM_AUTO_UPDATE_CA_REVOCATION: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(420i32);
pub const XCN_OID_ROOT_PROGRAM_AUTO_UPDATE_END_REVOCATION: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(421i32);
pub const XCN_OID_ROOT_PROGRAM_FLAGS: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(422i32);
pub const XCN_OID_ROOT_PROGRAM_NO_OCSP_FAILOVER_TO_CRL: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(423i32);
pub const XCN_OID_RSA_PSPECIFIED: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(424i32);
pub const XCN_OID_RSAES_OAEP: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(425i32);
pub const XCN_OID_SUBJECT_INFO_ACCESS: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(426i32);
pub const XCN_OID_TIMESTAMP_TOKEN: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(427i32);
pub const XCN_OID_ENROLL_SCEP_ERROR: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(428i32);
pub const XCN_OIDVerisign_MessageType: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(429i32);
pub const XCN_OIDVerisign_PkiStatus: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(430i32);
pub const XCN_OIDVerisign_FailInfo: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(431i32);
pub const XCN_OIDVerisign_SenderNonce: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(432i32);
pub const XCN_OIDVerisign_RecipientNonce: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(433i32);
pub const XCN_OIDVerisign_TransactionID: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(434i32);
pub const XCN_OID_ENROLL_ATTESTATION_CHALLENGE: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(435i32);
pub const XCN_OID_ENROLL_ATTESTATION_STATEMENT: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(436i32);
pub const XCN_OID_ENROLL_ENCRYPTION_ALGORITHM: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(437i32);
pub const XCN_OID_ENROLL_KSP_NAME: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(438i32);
impl ::core::marker::Copy for CERTENROLL_OBJECTID {}
impl ::core::clone::Clone for CERTENROLL_OBJECTID {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct CERTENROLL_PROPERTYID(pub i32);
pub const XCN_PROPERTYID_NONE: CERTENROLL_PROPERTYID = CERTENROLL_PROPERTYID(0i32);
pub const XCN_CERT_KEY_PROV_HANDLE_PROP_ID: CERTENROLL_PROPERTYID = CERTENROLL_PROPERTYID(1i32);
pub const XCN_CERT_KEY_PROV_INFO_PROP_ID: CERTENROLL_PROPERTYID = CERTENROLL_PROPERTYID(2i32);
pub const XCN_CERT_SHA1_HASH_PROP_ID: CERTENROLL_PROPERTYID = CERTENROLL_PROPERTYID(3i32);
pub const XCN_CERT_MD5_HASH_PROP_ID: CERTENROLL_PROPERTYID = CERTENROLL_PROPERTYID(4i32);
pub const XCN_CERT_HASH_PROP_ID: CERTENROLL_PROPERTYID = CERTENROLL_PROPERTYID(3i32);
pub const XCN_CERT_KEY_CONTEXT_PROP_ID: CERTENROLL_PROPERTYID = CERTENROLL_PROPERTYID(5i32);
pub const XCN_CERT_KEY_SPEC_PROP_ID: CERTENROLL_PROPERTYID = CERTENROLL_PROPERTYID(6i32);
pub const XCN_CERT_IE30_RESERVED_PROP_ID: CERTENROLL_PROPERTYID = CERTENROLL_PROPERTYID(7i32);
pub const XCN_CERT_PUBKEY_HASH_RESERVED_PROP_ID: CERTENROLL_PROPERTYID = CERTENROLL_PROPERTYID(8i32);
pub const XCN_CERT_ENHKEY_USAGE_PROP_ID: CERTENROLL_PROPERTYID = CERTENROLL_PROPERTYID(9i32);
pub const XCN_CERT_CTL_USAGE_PROP_ID: CERTENROLL_PROPERTYID = CERTENROLL_PROPERTYID(9i32);
pub const XCN_CERT_NEXT_UPDATE_LOCATION_PROP_ID: CERTENROLL_PROPERTYID = CERTENROLL_PROPERTYID(10i32);
pub const XCN_CERT_FRIENDLY_NAME_PROP_ID: CERTENROLL_PROPERTYID = CERTENROLL_PROPERTYID(11i32);
pub const XCN_CERT_PVK_FILE_PROP_ID: CERTENROLL_PROPERTYID = CERTENROLL_PROPERTYID(12i32);
pub const XCN_CERT_DESCRIPTION_PROP_ID: CERTENROLL_PROPERTYID = CERTENROLL_PROPERTYID(13i32);
pub const XCN_CERT_ACCESS_STATE_PROP_ID: CERTENROLL_PROPERTYID = CERTENROLL_PROPERTYID(14i32);
pub const XCN_CERT_SIGNATURE_HASH_PROP_ID: CERTENROLL_PROPERTYID = CERTENROLL_PROPERTYID(15i32);
pub const XCN_CERT_SMART_CARD_DATA_PROP_ID: CERTENROLL_PROPERTYID = CERTENROLL_PROPERTYID(16i32);
pub const XCN_CERT_EFS_PROP_ID: CERTENROLL_PROPERTYID = CERTENROLL_PROPERTYID(17i32);
pub const XCN_CERT_FORTEZZA_DATA_PROP_ID: CERTENROLL_PROPERTYID = CERTENROLL_PROPERTYID(18i32);
pub const XCN_CERT_ARCHIVED_PROP_ID: CERTENROLL_PROPERTYID = CERTENROLL_PROPERTYID(19i32);
pub const XCN_CERT_KEY_IDENTIFIER_PROP_ID: CERTENROLL_PROPERTYID = CERTENROLL_PROPERTYID(20i32);
pub const XCN_CERT_AUTO_ENROLL_PROP_ID: CERTENROLL_PROPERTYID = CERTENROLL_PROPERTYID(21i32);
pub const XCN_CERT_PUBKEY_ALG_PARA_PROP_ID: CERTENROLL_PROPERTYID = CERTENROLL_PROPERTYID(22i32);
pub const XCN_CERT_CROSS_CERT_DIST_POINTS_PROP_ID: CERTENROLL_PROPERTYID = CERTENROLL_PROPERTYID(23i32);
pub const XCN_CERT_ISSUER_PUBLIC_KEY_MD5_HASH_PROP_ID: CERTENROLL_PROPERTYID = CERTENROLL_PROPERTYID(24i32);
pub const XCN_CERT_SUBJECT_PUBLIC_KEY_MD5_HASH_PROP_ID: CERTENROLL_PROPERTYID = CERTENROLL_PROPERTYID(25i32);
pub const XCN_CERT_ENROLLMENT_PROP_ID: CERTENROLL_PROPERTYID = CERTENROLL_PROPERTYID(26i32);
pub const XCN_CERT_DATE_STAMP_PROP_ID: CERTENROLL_PROPERTYID = CERTENROLL_PROPERTYID(27i32);
pub const XCN_CERT_ISSUER_SERIAL_NUMBER_MD5_HASH_PROP_ID: CERTENROLL_PROPERTYID = CERTENROLL_PROPERTYID(28i32);
pub const XCN_CERT_SUBJECT_NAME_MD5_HASH_PROP_ID: CERTENROLL_PROPERTYID = CERTENROLL_PROPERTYID(29i32);
pub const XCN_CERT_EXTENDED_ERROR_INFO_PROP_ID: CERTENROLL_PROPERTYID = CERTENROLL_PROPERTYID(30i32);
pub const XCN_CERT_RENEWAL_PROP_ID: CERTENROLL_PROPERTYID = CERTENROLL_PROPERTYID(64i32);
pub const XCN_CERT_ARCHIVED_KEY_HASH_PROP_ID: CERTENROLL_PROPERTYID = CERTENROLL_PROPERTYID(65i32);
pub const XCN_CERT_AUTO_ENROLL_RETRY_PROP_ID: CERTENROLL_PROPERTYID = CERTENROLL_PROPERTYID(66i32);
pub const XCN_CERT_AIA_URL_RETRIEVED_PROP_ID: CERTENROLL_PROPERTYID = CERTENROLL_PROPERTYID(67i32);
pub const XCN_CERT_AUTHORITY_INFO_ACCESS_PROP_ID: CERTENROLL_PROPERTYID = CERTENROLL_PROPERTYID(68i32);
pub const XCN_CERT_BACKED_UP_PROP_ID: CERTENROLL_PROPERTYID = CERTENROLL_PROPERTYID(69i32);
pub const XCN_CERT_OCSP_RESPONSE_PROP_ID: CERTENROLL_PROPERTYID = CERTENROLL_PROPERTYID(70i32);
pub const XCN_CERT_REQUEST_ORIGINATOR_PROP_ID: CERTENROLL_PROPERTYID = CERTENROLL_PROPERTYID(71i32);
pub const XCN_CERT_SOURCE_LOCATION_PROP_ID: CERTENROLL_PROPERTYID = CERTENROLL_PROPERTYID(72i32);
pub const XCN_CERT_SOURCE_URL_PROP_ID: CERTENROLL_PROPERTYID = CERTENROLL_PROPERTYID(73i32);
pub const XCN_CERT_NEW_KEY_PROP_ID: CERTENROLL_PROPERTYID = CERTENROLL_PROPERTYID(74i32);
pub const XCN_CERT_OCSP_CACHE_PREFIX_PROP_ID: CERTENROLL_PROPERTYID = CERTENROLL_PROPERTYID(75i32);
pub const XCN_CERT_SMART_CARD_ROOT_INFO_PROP_ID: CERTENROLL_PROPERTYID = CERTENROLL_PROPERTYID(76i32);
pub const XCN_CERT_NO_AUTO_EXPIRE_CHECK_PROP_ID: CERTENROLL_PROPERTYID = CERTENROLL_PROPERTYID(77i32);
pub const XCN_CERT_NCRYPT_KEY_HANDLE_PROP_ID: CERTENROLL_PROPERTYID = CERTENROLL_PROPERTYID(78i32);
pub const XCN_CERT_HCRYPTPROV_OR_NCRYPT_KEY_HANDLE_PROP_ID: CERTENROLL_PROPERTYID = CERTENROLL_PROPERTYID(79i32);
pub const XCN_CERT_SUBJECT_INFO_ACCESS_PROP_ID: CERTENROLL_PROPERTYID = CERTENROLL_PROPERTYID(80i32);
pub const XCN_CERT_CA_OCSP_AUTHORITY_INFO_ACCESS_PROP_ID: CERTENROLL_PROPERTYID = CERTENROLL_PROPERTYID(81i32);
pub const XCN_CERT_CA_DISABLE_CRL_PROP_ID: CERTENROLL_PROPERTYID = CERTENROLL_PROPERTYID(82i32);
pub const XCN_CERT_ROOT_PROGRAM_CERT_POLICIES_PROP_ID: CERTENROLL_PROPERTYID = CERTENROLL_PROPERTYID(83i32);
pub const XCN_CERT_ROOT_PROGRAM_NAME_CONSTRAINTS_PROP_ID: CERTENROLL_PROPERTYID = CERTENROLL_PROPERTYID(84i32);
pub const XCN_CERT_SUBJECT_OCSP_AUTHORITY_INFO_ACCESS_PROP_ID: CERTENROLL_PROPERTYID = CERTENROLL_PROPERTYID(85i32);
pub const XCN_CERT_SUBJECT_DISABLE_CRL_PROP_ID: CERTENROLL_PROPERTYID = CERTENROLL_PROPERTYID(86i32);
pub const XCN_CERT_CEP_PROP_ID: CERTENROLL_PROPERTYID = CERTENROLL_PROPERTYID(87i32);
pub const XCN_CERT_SIGN_HASH_CNG_ALG_PROP_ID: CERTENROLL_PROPERTYID = CERTENROLL_PROPERTYID(89i32);
pub const XCN_CERT_SCARD_PIN_ID_PROP_ID: CERTENROLL_PROPERTYID = CERTENROLL_PROPERTYID(90i32);
pub const XCN_CERT_SCARD_PIN_INFO_PROP_ID: CERTENROLL_PROPERTYID = CERTENROLL_PROPERTYID(91i32);
pub const XCN_CERT_SUBJECT_PUB_KEY_BIT_LENGTH_PROP_ID: CERTENROLL_PROPERTYID = CERTENROLL_PROPERTYID(92i32);
pub const XCN_CERT_PUB_KEY_CNG_ALG_BIT_LENGTH_PROP_ID: CERTENROLL_PROPERTYID = CERTENROLL_PROPERTYID(93i32);
pub const XCN_CERT_ISSUER_PUB_KEY_BIT_LENGTH_PROP_ID: CERTENROLL_PROPERTYID = CERTENROLL_PROPERTYID(94i32);
pub const XCN_CERT_ISSUER_CHAIN_SIGN_HASH_CNG_ALG_PROP_ID: CERTENROLL_PROPERTYID = CERTENROLL_PROPERTYID(95i32);
pub const XCN_CERT_ISSUER_CHAIN_PUB_KEY_CNG_ALG_BIT_LENGTH_PROP_ID: CERTENROLL_PROPERTYID = CERTENROLL_PROPERTYID(96i32);
pub const XCN_CERT_NO_EXPIRE_NOTIFICATION_PROP_ID: CERTENROLL_PROPERTYID = CERTENROLL_PROPERTYID(97i32);
pub const XCN_CERT_AUTH_ROOT_SHA256_HASH_PROP_ID: CERTENROLL_PROPERTYID = CERTENROLL_PROPERTYID(98i32);
pub const XCN_CERT_NCRYPT_KEY_HANDLE_TRANSFER_PROP_ID: CERTENROLL_PROPERTYID = CERTENROLL_PROPERTYID(99i32);
pub const XCN_CERT_HCRYPTPROV_TRANSFER_PROP_ID: CERTENROLL_PROPERTYID = CERTENROLL_PROPERTYID(100i32);
pub const XCN_CERT_SMART_CARD_READER_PROP_ID: CERTENROLL_PROPERTYID = CERTENROLL_PROPERTYID(101i32);
pub const XCN_CERT_SEND_AS_TRUSTED_ISSUER_PROP_ID: CERTENROLL_PROPERTYID = CERTENROLL_PROPERTYID(102i32);
pub const XCN_CERT_KEY_REPAIR_ATTEMPTED_PROP_ID: CERTENROLL_PROPERTYID = CERTENROLL_PROPERTYID(103i32);
pub const XCN_CERT_DISALLOWED_FILETIME_PROP_ID: CERTENROLL_PROPERTYID = CERTENROLL_PROPERTYID(104i32);
pub const XCN_CERT_ROOT_PROGRAM_CHAIN_POLICIES_PROP_ID: CERTENROLL_PROPERTYID = CERTENROLL_PROPERTYID(105i32);
pub const XCN_CERT_SMART_CARD_READER_NON_REMOVABLE_PROP_ID: CERTENROLL_PROPERTYID = CERTENROLL_PROPERTYID(106i32);
pub const XCN_CERT_SHA256_HASH_PROP_ID: CERTENROLL_PROPERTYID = CERTENROLL_PROPERTYID(107i32);
pub const XCN_CERT_SCEP_SERVER_CERTS_PROP_ID: CERTENROLL_PROPERTYID = CERTENROLL_PROPERTYID(108i32);
pub const XCN_CERT_SCEP_RA_SIGNATURE_CERT_PROP_ID: CERTENROLL_PROPERTYID = CERTENROLL_PROPERTYID(109i32);
pub const XCN_CERT_SCEP_RA_ENCRYPTION_CERT_PROP_ID: CERTENROLL_PROPERTYID = CERTENROLL_PROPERTYID(110i32);
pub const XCN_CERT_SCEP_CA_CERT_PROP_ID: CERTENROLL_PROPERTYID = CERTENROLL_PROPERTYID(111i32);
pub const XCN_CERT_SCEP_SIGNER_CERT_PROP_ID: CERTENROLL_PROPERTYID = CERTENROLL_PROPERTYID(112i32);
pub const XCN_CERT_SCEP_NONCE_PROP_ID: CERTENROLL_PROPERTYID = CERTENROLL_PROPERTYID(113i32);
pub const XCN_CERT_SCEP_ENCRYPT_HASH_CNG_ALG_PROP_ID: CERTENROLL_PROPERTYID = CERTENROLL_PROPERTYID(114i32);
pub const XCN_CERT_SCEP_FLAGS_PROP_ID: CERTENROLL_PROPERTYID = CERTENROLL_PROPERTYID(115i32);
pub const XCN_CERT_SCEP_GUID_PROP_ID: CERTENROLL_PROPERTYID = CERTENROLL_PROPERTYID(116i32);
pub const XCN_CERT_SERIALIZABLE_KEY_CONTEXT_PROP_ID: CERTENROLL_PROPERTYID = CERTENROLL_PROPERTYID(117i32);
pub const XCN_CERT_ISOLATED_KEY_PROP_ID: CERTENROLL_PROPERTYID = CERTENROLL_PROPERTYID(118i32);
pub const XCN_CERT_SERIAL_CHAIN_PROP_ID: CERTENROLL_PROPERTYID = CERTENROLL_PROPERTYID(119i32);
pub const XCN_CERT_KEY_CLASSIFICATION_PROP_ID: CERTENROLL_PROPERTYID = CERTENROLL_PROPERTYID(120i32);
pub const XCN_CERT_DISALLOWED_ENHKEY_USAGE_PROP_ID: CERTENROLL_PROPERTYID = CERTENROLL_PROPERTYID(122i32);
pub const XCN_CERT_NONCOMPLIANT_ROOT_URL_PROP_ID: CERTENROLL_PROPERTYID = CERTENROLL_PROPERTYID(123i32);
pub const XCN_CERT_PIN_SHA256_HASH_PROP_ID: CERTENROLL_PROPERTYID = CERTENROLL_PROPERTYID(124i32);
pub const XCN_CERT_CLR_DELETE_KEY_PROP_ID: CERTENROLL_PROPERTYID = CERTENROLL_PROPERTYID(125i32);
pub const XCN_CERT_NOT_BEFORE_FILETIME_PROP_ID: CERTENROLL_PROPERTYID = CERTENROLL_PROPERTYID(126i32);
pub const XCN_CERT_CERT_NOT_BEFORE_ENHKEY_USAGE_PROP_ID: CERTENROLL_PROPERTYID = CERTENROLL_PROPERTYID(127i32);
pub const XCN_CERT_FIRST_RESERVED_PROP_ID: CERTENROLL_PROPERTYID = CERTENROLL_PROPERTYID(128i32);
pub const XCN_CERT_LAST_RESERVED_PROP_ID: CERTENROLL_PROPERTYID = CERTENROLL_PROPERTYID(32767i32);
pub const XCN_CERT_FIRST_USER_PROP_ID: CERTENROLL_PROPERTYID = CERTENROLL_PROPERTYID(32768i32);
pub const XCN_CERT_LAST_USER_PROP_ID: CERTENROLL_PROPERTYID = CERTENROLL_PROPERTYID(65535i32);
pub const XCN_CERT_STORE_LOCALIZED_NAME_PROP_ID: CERTENROLL_PROPERTYID = CERTENROLL_PROPERTYID(4096i32);
impl ::core::marker::Copy for CERTENROLL_PROPERTYID {}
impl ::core::clone::Clone for CERTENROLL_PROPERTYID {
    fn clone(&self) -> Self {
        *self
    }
}
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
#[repr(transparent)]
pub struct CERT_ALT_NAME(pub u32);
pub const CERT_ALT_NAME_RFC822_NAME: CERT_ALT_NAME = CERT_ALT_NAME(2u32);
pub const CERT_ALT_NAME_DNS_NAME: CERT_ALT_NAME = CERT_ALT_NAME(3u32);
pub const CERT_ALT_NAME_URL: CERT_ALT_NAME = CERT_ALT_NAME(7u32);
pub const CERT_ALT_NAME_REGISTERED_ID: CERT_ALT_NAME = CERT_ALT_NAME(9u32);
pub const CERT_ALT_NAME_DIRECTORY_NAME: CERT_ALT_NAME = CERT_ALT_NAME(5u32);
pub const CERT_ALT_NAME_IP_ADDRESS: CERT_ALT_NAME = CERT_ALT_NAME(8u32);
pub const CERT_ALT_NAME_OTHER_NAME: CERT_ALT_NAME = CERT_ALT_NAME(1u32);
impl ::core::marker::Copy for CERT_ALT_NAME {}
impl ::core::clone::Clone for CERT_ALT_NAME {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct CERT_CREATE_REQUEST_FLAGS(pub u32);
pub const XECR_CMC: CERT_CREATE_REQUEST_FLAGS = CERT_CREATE_REQUEST_FLAGS(3u32);
pub const XECR_PKCS10_V1_5: CERT_CREATE_REQUEST_FLAGS = CERT_CREATE_REQUEST_FLAGS(4u32);
pub const XECR_PKCS10_V2_0: CERT_CREATE_REQUEST_FLAGS = CERT_CREATE_REQUEST_FLAGS(1u32);
pub const XECR_PKCS7: CERT_CREATE_REQUEST_FLAGS = CERT_CREATE_REQUEST_FLAGS(2u32);
impl ::core::marker::Copy for CERT_CREATE_REQUEST_FLAGS {}
impl ::core::clone::Clone for CERT_CREATE_REQUEST_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct CERT_DELETE_ROW_FLAGS(pub u32);
pub const CDR_EXPIRED: CERT_DELETE_ROW_FLAGS = CERT_DELETE_ROW_FLAGS(1u32);
pub const CDR_REQUEST_LAST_CHANGED: CERT_DELETE_ROW_FLAGS = CERT_DELETE_ROW_FLAGS(2u32);
impl ::core::marker::Copy for CERT_DELETE_ROW_FLAGS {}
impl ::core::clone::Clone for CERT_DELETE_ROW_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct CERT_EXIT_EVENT_MASK(pub u32);
pub const EXITEVENT_CERTDENIED: CERT_EXIT_EVENT_MASK = CERT_EXIT_EVENT_MASK(4u32);
pub const EXITEVENT_CERTISSUED: CERT_EXIT_EVENT_MASK = CERT_EXIT_EVENT_MASK(1u32);
pub const EXITEVENT_CERTPENDING: CERT_EXIT_EVENT_MASK = CERT_EXIT_EVENT_MASK(2u32);
pub const EXITEVENT_CERTRETRIEVEPENDING: CERT_EXIT_EVENT_MASK = CERT_EXIT_EVENT_MASK(16u32);
pub const EXITEVENT_CERTREVOKED: CERT_EXIT_EVENT_MASK = CERT_EXIT_EVENT_MASK(8u32);
pub const EXITEVENT_CRLISSUED: CERT_EXIT_EVENT_MASK = CERT_EXIT_EVENT_MASK(32u32);
pub const EXITEVENT_SHUTDOWN: CERT_EXIT_EVENT_MASK = CERT_EXIT_EVENT_MASK(64u32);
impl ::core::marker::Copy for CERT_EXIT_EVENT_MASK {}
impl ::core::clone::Clone for CERT_EXIT_EVENT_MASK {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct CERT_GET_CONFIG_FLAGS(pub u32);
pub const CC_DEFAULTCONFIG: CERT_GET_CONFIG_FLAGS = CERT_GET_CONFIG_FLAGS(0u32);
pub const CC_FIRSTCONFIG: CERT_GET_CONFIG_FLAGS = CERT_GET_CONFIG_FLAGS(2u32);
pub const CC_LOCALACTIVECONFIG: CERT_GET_CONFIG_FLAGS = CERT_GET_CONFIG_FLAGS(4u32);
pub const CC_LOCALCONFIG: CERT_GET_CONFIG_FLAGS = CERT_GET_CONFIG_FLAGS(3u32);
pub const CC_UIPICKCONFIG: CERT_GET_CONFIG_FLAGS = CERT_GET_CONFIG_FLAGS(1u32);
pub const CC_UIPICKCONFIGSKIPLOCALCA_: CERT_GET_CONFIG_FLAGS = CERT_GET_CONFIG_FLAGS(5u32);
impl ::core::marker::Copy for CERT_GET_CONFIG_FLAGS {}
impl ::core::clone::Clone for CERT_GET_CONFIG_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct CERT_IMPORT_FLAGS(pub u32);
pub const CR_IN_BASE64HEADER: CERT_IMPORT_FLAGS = CERT_IMPORT_FLAGS(0u32);
pub const CR_IN_BASE64: CERT_IMPORT_FLAGS = CERT_IMPORT_FLAGS(1u32);
pub const CR_IN_BINARY: CERT_IMPORT_FLAGS = CERT_IMPORT_FLAGS(2u32);
impl ::core::marker::Copy for CERT_IMPORT_FLAGS {}
impl ::core::clone::Clone for CERT_IMPORT_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct CERT_PROPERTY_TYPE(pub u32);
pub const PROPTYPE_BINARY: CERT_PROPERTY_TYPE = CERT_PROPERTY_TYPE(3u32);
pub const PROPTYPE_DATE: CERT_PROPERTY_TYPE = CERT_PROPERTY_TYPE(2u32);
pub const PROPTYPE_LONG: CERT_PROPERTY_TYPE = CERT_PROPERTY_TYPE(1u32);
pub const PROPTYPE_STRING: CERT_PROPERTY_TYPE = CERT_PROPERTY_TYPE(4u32);
impl ::core::marker::Copy for CERT_PROPERTY_TYPE {}
impl ::core::clone::Clone for CERT_PROPERTY_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct CERT_REQUEST_OUT_TYPE(pub u32);
pub const CR_OUT_BASE64HEADER: CERT_REQUEST_OUT_TYPE = CERT_REQUEST_OUT_TYPE(0u32);
pub const CR_OUT_BASE64: CERT_REQUEST_OUT_TYPE = CERT_REQUEST_OUT_TYPE(1u32);
pub const CR_OUT_BINARY: CERT_REQUEST_OUT_TYPE = CERT_REQUEST_OUT_TYPE(2u32);
impl ::core::marker::Copy for CERT_REQUEST_OUT_TYPE {}
impl ::core::clone::Clone for CERT_REQUEST_OUT_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct CERT_VIEW_COLUMN_INDEX(pub i32);
pub const CV_COLUMN_LOG_DEFAULT: CERT_VIEW_COLUMN_INDEX = CERT_VIEW_COLUMN_INDEX(-2i32);
pub const CV_COLUMN_LOG_FAILED_DEFAULT: CERT_VIEW_COLUMN_INDEX = CERT_VIEW_COLUMN_INDEX(-3i32);
pub const CV_COLUMN_QUEUE_DEFAULT: CERT_VIEW_COLUMN_INDEX = CERT_VIEW_COLUMN_INDEX(-1i32);
impl ::core::marker::Copy for CERT_VIEW_COLUMN_INDEX {}
impl ::core::clone::Clone for CERT_VIEW_COLUMN_INDEX {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct CERT_VIEW_SEEK_OPERATOR_FLAGS(pub u32);
pub const CVR_SEEK_EQ: CERT_VIEW_SEEK_OPERATOR_FLAGS = CERT_VIEW_SEEK_OPERATOR_FLAGS(1u32);
pub const CVR_SEEK_LE: CERT_VIEW_SEEK_OPERATOR_FLAGS = CERT_VIEW_SEEK_OPERATOR_FLAGS(4u32);
pub const CVR_SEEK_LT: CERT_VIEW_SEEK_OPERATOR_FLAGS = CERT_VIEW_SEEK_OPERATOR_FLAGS(2u32);
pub const CVR_SEEK_GE: CERT_VIEW_SEEK_OPERATOR_FLAGS = CERT_VIEW_SEEK_OPERATOR_FLAGS(8u32);
pub const CVR_SEEK_GT: CERT_VIEW_SEEK_OPERATOR_FLAGS = CERT_VIEW_SEEK_OPERATOR_FLAGS(16u32);
impl ::core::marker::Copy for CERT_VIEW_SEEK_OPERATOR_FLAGS {}
impl ::core::clone::Clone for CERT_VIEW_SEEK_OPERATOR_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
pub const CEnroll: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 1140388489, data2: 31264, data3: 4560, data4: [143, 6, 0, 192, 79, 194, 149, 225] };
pub const CEnroll2: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 309762276, data2: 59184, data3: 20060, data4: [162, 177, 33, 73, 10, 112, 200, 161] };
pub const CMM_READONLY: u32 = 2u32;
pub const CMM_REFRESHONLY: u32 = 1u32;
pub const CObjectId: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 2286821376, data2: 8573, data3: 4570, data4: [178, 164, 0, 14, 123, 187, 43, 9] };
pub const CObjectIds: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 2286821377, data2: 8573, data3: 4570, data4: [178, 164, 0, 14, 123, 187, 43, 9] };
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
pub const CPolicyQualifier: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 2286821404, data2: 8573, data3: 4570, data4: [178, 164, 0, 14, 123, 187, 43, 9] };
pub const CPolicyQualifiers: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 2286821405, data2: 8573, data3: 4570, data4: [178, 164, 0, 14, 123, 187, 43, 9] };
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
#[repr(transparent)]
pub struct CRLRevocationReason(pub i32);
pub const XCN_CRL_REASON_UNSPECIFIED: CRLRevocationReason = CRLRevocationReason(0i32);
pub const XCN_CRL_REASON_KEY_COMPROMISE: CRLRevocationReason = CRLRevocationReason(1i32);
pub const XCN_CRL_REASON_CA_COMPROMISE: CRLRevocationReason = CRLRevocationReason(2i32);
pub const XCN_CRL_REASON_AFFILIATION_CHANGED: CRLRevocationReason = CRLRevocationReason(3i32);
pub const XCN_CRL_REASON_SUPERSEDED: CRLRevocationReason = CRLRevocationReason(4i32);
pub const XCN_CRL_REASON_CESSATION_OF_OPERATION: CRLRevocationReason = CRLRevocationReason(5i32);
pub const XCN_CRL_REASON_CERTIFICATE_HOLD: CRLRevocationReason = CRLRevocationReason(6i32);
pub const XCN_CRL_REASON_REMOVE_FROM_CRL: CRLRevocationReason = CRLRevocationReason(8i32);
pub const XCN_CRL_REASON_PRIVILEGE_WITHDRAWN: CRLRevocationReason = CRLRevocationReason(9i32);
pub const XCN_CRL_REASON_AA_COMPROMISE: CRLRevocationReason = CRLRevocationReason(10i32);
impl ::core::marker::Copy for CRLRevocationReason {}
impl ::core::clone::Clone for CRLRevocationReason {
    fn clone(&self) -> Self {
        *self
    }
}
pub const CRYPT_ENUM_ALL_PROVIDERS: u32 = 1u32;
#[repr(transparent)]
pub struct CR_DISP(pub u32);
pub const CR_DISP_DENIED: CR_DISP = CR_DISP(2u32);
pub const CR_DISP_ERROR: CR_DISP = CR_DISP(1u32);
pub const CR_DISP_INCOMPLETE: CR_DISP = CR_DISP(0u32);
pub const CR_DISP_ISSUED: CR_DISP = CR_DISP(3u32);
pub const CR_DISP_ISSUED_OUT_OF_BAND: CR_DISP = CR_DISP(4u32);
pub const CR_DISP_UNDER_SUBMISSION: CR_DISP = CR_DISP(5u32);
impl ::core::marker::Copy for CR_DISP {}
impl ::core::clone::Clone for CR_DISP {
    fn clone(&self) -> Self {
        *self
    }
}
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
#[repr(transparent)]
pub struct CSBACKUP_TYPE(pub u32);
pub const CSBACKUP_TYPE_FULL: CSBACKUP_TYPE = CSBACKUP_TYPE(1u32);
pub const CSBACKUP_TYPE_LOGS_ONLY: CSBACKUP_TYPE = CSBACKUP_TYPE(2u32);
impl ::core::marker::Copy for CSBACKUP_TYPE {}
impl ::core::clone::Clone for CSBACKUP_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
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
pub const CSignerCertificate: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 2286821437, data2: 8573, data3: 4570, data4: [178, 164, 0, 14, 123, 187, 43, 9] };
pub const CSmimeCapabilities: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 2286821402, data2: 8573, data3: 4570, data4: [178, 164, 0, 14, 123, 187, 43, 9] };
pub const CSmimeCapability: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 2286821401, data2: 8573, data3: 4570, data4: [178, 164, 0, 14, 123, 187, 43, 9] };
pub const CVIEWAGEMINUTESDEFAULT: u32 = 16u32;
#[repr(transparent)]
pub struct CVRC_COLUMN(pub u32);
pub const CVRC_COLUMN_SCHEMA: CVRC_COLUMN = CVRC_COLUMN(0u32);
pub const CVRC_COLUMN_RESULT: CVRC_COLUMN = CVRC_COLUMN(1u32);
pub const CVRC_COLUMN_VALUE: CVRC_COLUMN = CVRC_COLUMN(2u32);
pub const CVRC_COLUMN_MASK: CVRC_COLUMN = CVRC_COLUMN(4095u32);
impl ::core::marker::Copy for CVRC_COLUMN {}
impl ::core::clone::Clone for CVRC_COLUMN {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct CVRC_TABLE(pub u32);
pub const CVRC_TABLE_ATTRIBUTES: CVRC_TABLE = CVRC_TABLE(16384u32);
pub const CVRC_TABLE_CRL: CVRC_TABLE = CVRC_TABLE(20480u32);
pub const CVRC_TABLE_EXTENSIONS: CVRC_TABLE = CVRC_TABLE(12288u32);
pub const CVRC_TABLE_REQCERT: CVRC_TABLE = CVRC_TABLE(0u32);
impl ::core::marker::Copy for CVRC_TABLE {}
impl ::core::clone::Clone for CVRC_TABLE {
    fn clone(&self) -> Self {
        *self
    }
}
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
pub const CX500DistinguishedName: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 2286821379, data2: 8573, data3: 4570, data4: [178, 164, 0, 14, 123, 187, 43, 9] };
pub const CX509Attribute: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 2286821410, data2: 8573, data3: 4570, data4: [178, 164, 0, 14, 123, 187, 43, 9] };
pub const CX509AttributeArchiveKey: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 2286821415, data2: 8573, data3: 4570, data4: [178, 164, 0, 14, 123, 187, 43, 9] };
pub const CX509AttributeArchiveKeyHash: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 2286821416, data2: 8573, data3: 4570, data4: [178, 164, 0, 14, 123, 187, 43, 9] };
pub const CX509AttributeClientId: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 2286821413, data2: 8573, data3: 4570, data4: [178, 164, 0, 14, 123, 187, 43, 9] };
pub const CX509AttributeCspProvider: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 2286821419, data2: 8573, data3: 4570, data4: [178, 164, 0, 14, 123, 187, 43, 9] };
pub const CX509AttributeExtensions: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 2286821412, data2: 8573, data3: 4570, data4: [178, 164, 0, 14, 123, 187, 43, 9] };
pub const CX509AttributeOSVersion: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 2286821418, data2: 8573, data3: 4570, data4: [178, 164, 0, 14, 123, 187, 43, 9] };
pub const CX509AttributeRenewalCertificate: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 2286821414, data2: 8573, data3: 4570, data4: [178, 164, 0, 14, 123, 187, 43, 9] };
pub const CX509Attributes: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 2286821411, data2: 8573, data3: 4570, data4: [178, 164, 0, 14, 123, 187, 43, 9] };
pub const CX509CertificateRequestCertificate: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 2286821443, data2: 8573, data3: 4570, data4: [178, 164, 0, 14, 123, 187, 43, 9] };
pub const CX509CertificateRequestCmc: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 2286821445, data2: 8573, data3: 4570, data4: [178, 164, 0, 14, 123, 187, 43, 9] };
pub const CX509CertificateRequestPkcs10: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 2286821442, data2: 8573, data3: 4570, data4: [178, 164, 0, 14, 123, 187, 43, 9] };
pub const CX509CertificateRequestPkcs7: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 2286821444, data2: 8573, data3: 4570, data4: [178, 164, 0, 14, 123, 187, 43, 9] };
pub const CX509CertificateRevocationList: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 2286821472, data2: 8573, data3: 4570, data4: [178, 164, 0, 14, 123, 187, 43, 9] };
pub const CX509CertificateRevocationListEntries: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 2286821471, data2: 8573, data3: 4570, data4: [178, 164, 0, 14, 123, 187, 43, 9] };
pub const CX509CertificateRevocationListEntry: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 2286821470, data2: 8573, data3: 4570, data4: [178, 164, 0, 14, 123, 187, 43, 9] };
pub const CX509CertificateTemplateADWritable: ::windows_sys::core::GUID = ::windows_sys::GUID {
    data1: 2201412387,
    data2: 11882,
    data3: 18948,
    data4: [147, 124, 84, 143, 104, 24, 57, 179],
};
pub const CX509EndorsementKey: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 295852573, data2: 47523, data3: 20189, data4: [175, 131, 59, 89, 173, 190, 211, 97] };
pub const CX509Enrollment: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 2286821446, data2: 8573, data3: 4570, data4: [178, 164, 0, 14, 123, 187, 43, 9] };
pub const CX509EnrollmentHelper: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 2286821456, data2: 8573, data3: 4570, data4: [178, 164, 0, 14, 123, 187, 43, 9] };
pub const CX509EnrollmentPolicyActiveDirectory: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 2448658471, data2: 8575, data3: 4570, data4: [178, 164, 0, 14, 123, 187, 43, 9] };
pub const CX509EnrollmentPolicyWebService: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 2448658472, data2: 8575, data3: 4570, data4: [178, 164, 0, 14, 123, 187, 43, 9] };
pub const CX509EnrollmentWebClassFactory: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 2286821449, data2: 8573, data3: 4570, data4: [178, 164, 0, 14, 123, 187, 43, 9] };
pub const CX509Extension: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 2286821389, data2: 8573, data3: 4570, data4: [178, 164, 0, 14, 123, 187, 43, 9] };
pub const CX509ExtensionAlternativeNames: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 2286821397, data2: 8573, data3: 4570, data4: [178, 164, 0, 14, 123, 187, 43, 9] };
pub const CX509ExtensionAuthorityKeyIdentifier: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 2286821400, data2: 8573, data3: 4570, data4: [178, 164, 0, 14, 123, 187, 43, 9] };
pub const CX509ExtensionBasicConstraints: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 2286821398, data2: 8573, data3: 4570, data4: [178, 164, 0, 14, 123, 187, 43, 9] };
pub const CX509ExtensionCertificatePolicies: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 2286821408, data2: 8573, data3: 4570, data4: [178, 164, 0, 14, 123, 187, 43, 9] };
pub const CX509ExtensionEnhancedKeyUsage: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 2286821392, data2: 8573, data3: 4570, data4: [178, 164, 0, 14, 123, 187, 43, 9] };
pub const CX509ExtensionKeyUsage: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 2286821391, data2: 8573, data3: 4570, data4: [178, 164, 0, 14, 123, 187, 43, 9] };
pub const CX509ExtensionMSApplicationPolicies: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 2286821409, data2: 8573, data3: 4570, data4: [178, 164, 0, 14, 123, 187, 43, 9] };
pub const CX509ExtensionSmimeCapabilities: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 2286821403, data2: 8573, data3: 4570, data4: [178, 164, 0, 14, 123, 187, 43, 9] };
pub const CX509ExtensionSubjectKeyIdentifier: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 2286821399, data2: 8573, data3: 4570, data4: [178, 164, 0, 14, 123, 187, 43, 9] };
pub const CX509ExtensionTemplate: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 2286821394, data2: 8573, data3: 4570, data4: [178, 164, 0, 14, 123, 187, 43, 9] };
pub const CX509ExtensionTemplateName: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 2286821393, data2: 8573, data3: 4570, data4: [178, 164, 0, 14, 123, 187, 43, 9] };
pub const CX509Extensions: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 2286821390, data2: 8573, data3: 4570, data4: [178, 164, 0, 14, 123, 187, 43, 9] };
pub const CX509MachineEnrollmentFactory: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 2286821457, data2: 8573, data3: 4570, data4: [178, 164, 0, 14, 123, 187, 43, 9] };
pub const CX509NameValuePair: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 2286821439, data2: 8573, data3: 4570, data4: [178, 164, 0, 14, 123, 187, 43, 9] };
pub const CX509PolicyServerListManager: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 2448658473, data2: 8575, data3: 4570, data4: [178, 164, 0, 14, 123, 187, 43, 9] };
pub const CX509PolicyServerUrl: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 2448658474, data2: 8575, data3: 4570, data4: [178, 164, 0, 14, 123, 187, 43, 9] };
pub const CX509PrivateKey: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 2286821388, data2: 8573, data3: 4570, data4: [178, 164, 0, 14, 123, 187, 43, 9] };
pub const CX509PublicKey: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 2286821387, data2: 8573, data3: 4570, data4: [178, 164, 0, 14, 123, 187, 43, 9] };
pub const CX509SCEPEnrollment: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 2286821473, data2: 8573, data3: 4570, data4: [178, 164, 0, 14, 123, 187, 43, 9] };
pub const CX509SCEPEnrollmentHelper: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 2286821474, data2: 8573, data3: 4570, data4: [178, 164, 0, 14, 123, 187, 43, 9] };
#[repr(transparent)]
pub struct CommitTemplateFlags(pub i32);
pub const CommitFlagSaveTemplateGenerateOID: CommitTemplateFlags = CommitTemplateFlags(1i32);
pub const CommitFlagSaveTemplateUseCurrentOID: CommitTemplateFlags = CommitTemplateFlags(2i32);
pub const CommitFlagSaveTemplateOverwrite: CommitTemplateFlags = CommitTemplateFlags(3i32);
pub const CommitFlagDeleteTemplate: CommitTemplateFlags = CommitTemplateFlags(4i32);
impl ::core::marker::Copy for CommitTemplateFlags {}
impl ::core::clone::Clone for CommitTemplateFlags {
    fn clone(&self) -> Self {
        *self
    }
}
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
#[repr(transparent)]
pub struct DelayRetryAction(pub i32);
pub const DelayRetryUnknown: DelayRetryAction = DelayRetryAction(0i32);
pub const DelayRetryNone: DelayRetryAction = DelayRetryAction(1i32);
pub const DelayRetryShort: DelayRetryAction = DelayRetryAction(2i32);
pub const DelayRetryLong: DelayRetryAction = DelayRetryAction(3i32);
pub const DelayRetrySuccess: DelayRetryAction = DelayRetryAction(4i32);
pub const DelayRetryPastSuccess: DelayRetryAction = DelayRetryAction(5i32);
impl ::core::marker::Copy for DelayRetryAction {}
impl ::core::clone::Clone for DelayRetryAction {
    fn clone(&self) -> Self {
        *self
    }
}
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
#[repr(transparent)]
pub struct ENUM_CATYPES(pub i32);
pub const ENUM_ENTERPRISE_ROOTCA: ENUM_CATYPES = ENUM_CATYPES(0i32);
pub const ENUM_ENTERPRISE_SUBCA: ENUM_CATYPES = ENUM_CATYPES(1i32);
pub const ENUM_STANDALONE_ROOTCA: ENUM_CATYPES = ENUM_CATYPES(3i32);
pub const ENUM_STANDALONE_SUBCA: ENUM_CATYPES = ENUM_CATYPES(4i32);
pub const ENUM_UNKNOWN_CA: ENUM_CATYPES = ENUM_CATYPES(5i32);
impl ::core::marker::Copy for ENUM_CATYPES {}
impl ::core::clone::Clone for ENUM_CATYPES {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ENUM_CERT_COLUMN_VALUE_FLAGS(pub u32);
pub const CV_OUT_BASE64: ENUM_CERT_COLUMN_VALUE_FLAGS = ENUM_CERT_COLUMN_VALUE_FLAGS(1u32);
pub const CV_OUT_BASE64HEADER: ENUM_CERT_COLUMN_VALUE_FLAGS = ENUM_CERT_COLUMN_VALUE_FLAGS(0u32);
pub const CV_OUT_BASE64REQUESTHEADER: ENUM_CERT_COLUMN_VALUE_FLAGS = ENUM_CERT_COLUMN_VALUE_FLAGS(3u32);
pub const CV_OUT_BASE64X509CRLHEADER: ENUM_CERT_COLUMN_VALUE_FLAGS = ENUM_CERT_COLUMN_VALUE_FLAGS(9u32);
pub const CV_OUT_BINARY: ENUM_CERT_COLUMN_VALUE_FLAGS = ENUM_CERT_COLUMN_VALUE_FLAGS(2u32);
pub const CV_OUT_HEX: ENUM_CERT_COLUMN_VALUE_FLAGS = ENUM_CERT_COLUMN_VALUE_FLAGS(4u32);
pub const CV_OUT_HEXADDR: ENUM_CERT_COLUMN_VALUE_FLAGS = ENUM_CERT_COLUMN_VALUE_FLAGS(10u32);
pub const CV_OUT_HEXASCII: ENUM_CERT_COLUMN_VALUE_FLAGS = ENUM_CERT_COLUMN_VALUE_FLAGS(5u32);
pub const CV_OUT_HEXASCIIADDR: ENUM_CERT_COLUMN_VALUE_FLAGS = ENUM_CERT_COLUMN_VALUE_FLAGS(11u32);
impl ::core::marker::Copy for ENUM_CERT_COLUMN_VALUE_FLAGS {}
impl ::core::clone::Clone for ENUM_CERT_COLUMN_VALUE_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
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
#[repr(transparent)]
pub struct EncodingType(pub i32);
pub const XCN_CRYPT_STRING_BASE64HEADER: EncodingType = EncodingType(0i32);
pub const XCN_CRYPT_STRING_BASE64: EncodingType = EncodingType(1i32);
pub const XCN_CRYPT_STRING_BINARY: EncodingType = EncodingType(2i32);
pub const XCN_CRYPT_STRING_BASE64REQUESTHEADER: EncodingType = EncodingType(3i32);
pub const XCN_CRYPT_STRING_HEX: EncodingType = EncodingType(4i32);
pub const XCN_CRYPT_STRING_HEXASCII: EncodingType = EncodingType(5i32);
pub const XCN_CRYPT_STRING_BASE64_ANY: EncodingType = EncodingType(6i32);
pub const XCN_CRYPT_STRING_ANY: EncodingType = EncodingType(7i32);
pub const XCN_CRYPT_STRING_HEX_ANY: EncodingType = EncodingType(8i32);
pub const XCN_CRYPT_STRING_BASE64X509CRLHEADER: EncodingType = EncodingType(9i32);
pub const XCN_CRYPT_STRING_HEXADDR: EncodingType = EncodingType(10i32);
pub const XCN_CRYPT_STRING_HEXASCIIADDR: EncodingType = EncodingType(11i32);
pub const XCN_CRYPT_STRING_HEXRAW: EncodingType = EncodingType(12i32);
pub const XCN_CRYPT_STRING_BASE64URI: EncodingType = EncodingType(13i32);
pub const XCN_CRYPT_STRING_ENCODEMASK: EncodingType = EncodingType(255i32);
pub const XCN_CRYPT_STRING_CHAIN: EncodingType = EncodingType(256i32);
pub const XCN_CRYPT_STRING_TEXT: EncodingType = EncodingType(512i32);
pub const XCN_CRYPT_STRING_PERCENTESCAPE: EncodingType = EncodingType(134217728i32);
pub const XCN_CRYPT_STRING_HASHDATA: EncodingType = EncodingType(268435456i32);
pub const XCN_CRYPT_STRING_STRICT: EncodingType = EncodingType(536870912i32);
pub const XCN_CRYPT_STRING_NOCRLF: EncodingType = EncodingType(1073741824i32);
pub const XCN_CRYPT_STRING_NOCR: EncodingType = EncodingType(-2147483648i32);
impl ::core::marker::Copy for EncodingType {}
impl ::core::clone::Clone for EncodingType {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct EnrollmentCAProperty(pub i32);
pub const CAPropCommonName: EnrollmentCAProperty = EnrollmentCAProperty(1i32);
pub const CAPropDistinguishedName: EnrollmentCAProperty = EnrollmentCAProperty(2i32);
pub const CAPropSanitizedName: EnrollmentCAProperty = EnrollmentCAProperty(3i32);
pub const CAPropSanitizedShortName: EnrollmentCAProperty = EnrollmentCAProperty(4i32);
pub const CAPropDNSName: EnrollmentCAProperty = EnrollmentCAProperty(5i32);
pub const CAPropCertificateTypes: EnrollmentCAProperty = EnrollmentCAProperty(6i32);
pub const CAPropCertificate: EnrollmentCAProperty = EnrollmentCAProperty(7i32);
pub const CAPropDescription: EnrollmentCAProperty = EnrollmentCAProperty(8i32);
pub const CAPropWebServers: EnrollmentCAProperty = EnrollmentCAProperty(9i32);
pub const CAPropSiteName: EnrollmentCAProperty = EnrollmentCAProperty(10i32);
pub const CAPropSecurity: EnrollmentCAProperty = EnrollmentCAProperty(11i32);
pub const CAPropRenewalOnly: EnrollmentCAProperty = EnrollmentCAProperty(12i32);
impl ::core::marker::Copy for EnrollmentCAProperty {}
impl ::core::clone::Clone for EnrollmentCAProperty {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct EnrollmentDisplayStatus(pub i32);
pub const DisplayNo: EnrollmentDisplayStatus = EnrollmentDisplayStatus(0i32);
pub const DisplayYes: EnrollmentDisplayStatus = EnrollmentDisplayStatus(1i32);
impl ::core::marker::Copy for EnrollmentDisplayStatus {}
impl ::core::clone::Clone for EnrollmentDisplayStatus {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct EnrollmentEnrollStatus(pub i32);
pub const Enrolled: EnrollmentEnrollStatus = EnrollmentEnrollStatus(1i32);
pub const EnrollPended: EnrollmentEnrollStatus = EnrollmentEnrollStatus(2i32);
pub const EnrollUIDeferredEnrollmentRequired: EnrollmentEnrollStatus = EnrollmentEnrollStatus(4i32);
pub const EnrollError: EnrollmentEnrollStatus = EnrollmentEnrollStatus(16i32);
pub const EnrollUnknown: EnrollmentEnrollStatus = EnrollmentEnrollStatus(32i32);
pub const EnrollSkipped: EnrollmentEnrollStatus = EnrollmentEnrollStatus(64i32);
pub const EnrollDenied: EnrollmentEnrollStatus = EnrollmentEnrollStatus(256i32);
impl ::core::marker::Copy for EnrollmentEnrollStatus {}
impl ::core::clone::Clone for EnrollmentEnrollStatus {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct EnrollmentPolicyFlags(pub i32);
pub const DisableGroupPolicyList: EnrollmentPolicyFlags = EnrollmentPolicyFlags(2i32);
pub const DisableUserServerList: EnrollmentPolicyFlags = EnrollmentPolicyFlags(4i32);
impl ::core::marker::Copy for EnrollmentPolicyFlags {}
impl ::core::clone::Clone for EnrollmentPolicyFlags {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct EnrollmentPolicyServerPropertyFlags(pub i32);
pub const DefaultNone: EnrollmentPolicyServerPropertyFlags = EnrollmentPolicyServerPropertyFlags(0i32);
pub const DefaultPolicyServer: EnrollmentPolicyServerPropertyFlags = EnrollmentPolicyServerPropertyFlags(1i32);
impl ::core::marker::Copy for EnrollmentPolicyServerPropertyFlags {}
impl ::core::clone::Clone for EnrollmentPolicyServerPropertyFlags {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct EnrollmentSelectionStatus(pub i32);
pub const SelectedNo: EnrollmentSelectionStatus = EnrollmentSelectionStatus(0i32);
pub const SelectedYes: EnrollmentSelectionStatus = EnrollmentSelectionStatus(1i32);
impl ::core::marker::Copy for EnrollmentSelectionStatus {}
impl ::core::clone::Clone for EnrollmentSelectionStatus {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct EnrollmentTemplateProperty(pub i32);
pub const TemplatePropCommonName: EnrollmentTemplateProperty = EnrollmentTemplateProperty(1i32);
pub const TemplatePropFriendlyName: EnrollmentTemplateProperty = EnrollmentTemplateProperty(2i32);
pub const TemplatePropEKUs: EnrollmentTemplateProperty = EnrollmentTemplateProperty(3i32);
pub const TemplatePropCryptoProviders: EnrollmentTemplateProperty = EnrollmentTemplateProperty(4i32);
pub const TemplatePropMajorRevision: EnrollmentTemplateProperty = EnrollmentTemplateProperty(5i32);
pub const TemplatePropDescription: EnrollmentTemplateProperty = EnrollmentTemplateProperty(6i32);
pub const TemplatePropKeySpec: EnrollmentTemplateProperty = EnrollmentTemplateProperty(7i32);
pub const TemplatePropSchemaVersion: EnrollmentTemplateProperty = EnrollmentTemplateProperty(8i32);
pub const TemplatePropMinorRevision: EnrollmentTemplateProperty = EnrollmentTemplateProperty(9i32);
pub const TemplatePropRASignatureCount: EnrollmentTemplateProperty = EnrollmentTemplateProperty(10i32);
pub const TemplatePropMinimumKeySize: EnrollmentTemplateProperty = EnrollmentTemplateProperty(11i32);
pub const TemplatePropOID: EnrollmentTemplateProperty = EnrollmentTemplateProperty(12i32);
pub const TemplatePropSupersede: EnrollmentTemplateProperty = EnrollmentTemplateProperty(13i32);
pub const TemplatePropRACertificatePolicies: EnrollmentTemplateProperty = EnrollmentTemplateProperty(14i32);
pub const TemplatePropRAEKUs: EnrollmentTemplateProperty = EnrollmentTemplateProperty(15i32);
pub const TemplatePropCertificatePolicies: EnrollmentTemplateProperty = EnrollmentTemplateProperty(16i32);
pub const TemplatePropV1ApplicationPolicy: EnrollmentTemplateProperty = EnrollmentTemplateProperty(17i32);
pub const TemplatePropAsymmetricAlgorithm: EnrollmentTemplateProperty = EnrollmentTemplateProperty(18i32);
pub const TemplatePropKeySecurityDescriptor: EnrollmentTemplateProperty = EnrollmentTemplateProperty(19i32);
pub const TemplatePropSymmetricAlgorithm: EnrollmentTemplateProperty = EnrollmentTemplateProperty(20i32);
pub const TemplatePropSymmetricKeyLength: EnrollmentTemplateProperty = EnrollmentTemplateProperty(21i32);
pub const TemplatePropHashAlgorithm: EnrollmentTemplateProperty = EnrollmentTemplateProperty(22i32);
pub const TemplatePropKeyUsage: EnrollmentTemplateProperty = EnrollmentTemplateProperty(23i32);
pub const TemplatePropEnrollmentFlags: EnrollmentTemplateProperty = EnrollmentTemplateProperty(24i32);
pub const TemplatePropSubjectNameFlags: EnrollmentTemplateProperty = EnrollmentTemplateProperty(25i32);
pub const TemplatePropPrivateKeyFlags: EnrollmentTemplateProperty = EnrollmentTemplateProperty(26i32);
pub const TemplatePropGeneralFlags: EnrollmentTemplateProperty = EnrollmentTemplateProperty(27i32);
pub const TemplatePropSecurityDescriptor: EnrollmentTemplateProperty = EnrollmentTemplateProperty(28i32);
pub const TemplatePropExtensions: EnrollmentTemplateProperty = EnrollmentTemplateProperty(29i32);
pub const TemplatePropValidityPeriod: EnrollmentTemplateProperty = EnrollmentTemplateProperty(30i32);
pub const TemplatePropRenewalPeriod: EnrollmentTemplateProperty = EnrollmentTemplateProperty(31i32);
impl ::core::marker::Copy for EnrollmentTemplateProperty {}
impl ::core::clone::Clone for EnrollmentTemplateProperty {
    fn clone(&self) -> Self {
        *self
    }
}
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
#[repr(transparent)]
pub struct FULL_RESPONSE_PROPERTY_ID(pub u32);
pub const FR_PROP_NONE: FULL_RESPONSE_PROPERTY_ID = FULL_RESPONSE_PROPERTY_ID(0u32);
pub const FR_PROP_FULLRESPONSE: FULL_RESPONSE_PROPERTY_ID = FULL_RESPONSE_PROPERTY_ID(1u32);
pub const FR_PROP_STATUSINFOCOUNT: FULL_RESPONSE_PROPERTY_ID = FULL_RESPONSE_PROPERTY_ID(2u32);
pub const FR_PROP_BODYPARTSTRING: FULL_RESPONSE_PROPERTY_ID = FULL_RESPONSE_PROPERTY_ID(3u32);
pub const FR_PROP_STATUS: FULL_RESPONSE_PROPERTY_ID = FULL_RESPONSE_PROPERTY_ID(4u32);
pub const FR_PROP_STATUSSTRING: FULL_RESPONSE_PROPERTY_ID = FULL_RESPONSE_PROPERTY_ID(5u32);
pub const FR_PROP_OTHERINFOCHOICE: FULL_RESPONSE_PROPERTY_ID = FULL_RESPONSE_PROPERTY_ID(6u32);
pub const FR_PROP_FAILINFO: FULL_RESPONSE_PROPERTY_ID = FULL_RESPONSE_PROPERTY_ID(7u32);
pub const FR_PROP_PENDINFOTOKEN: FULL_RESPONSE_PROPERTY_ID = FULL_RESPONSE_PROPERTY_ID(8u32);
pub const FR_PROP_PENDINFOTIME: FULL_RESPONSE_PROPERTY_ID = FULL_RESPONSE_PROPERTY_ID(9u32);
pub const FR_PROP_ISSUEDCERTIFICATEHASH: FULL_RESPONSE_PROPERTY_ID = FULL_RESPONSE_PROPERTY_ID(10u32);
pub const FR_PROP_ISSUEDCERTIFICATE: FULL_RESPONSE_PROPERTY_ID = FULL_RESPONSE_PROPERTY_ID(11u32);
pub const FR_PROP_ISSUEDCERTIFICATECHAIN: FULL_RESPONSE_PROPERTY_ID = FULL_RESPONSE_PROPERTY_ID(12u32);
pub const FR_PROP_ISSUEDCERTIFICATECRLCHAIN: FULL_RESPONSE_PROPERTY_ID = FULL_RESPONSE_PROPERTY_ID(13u32);
pub const FR_PROP_ENCRYPTEDKEYHASH: FULL_RESPONSE_PROPERTY_ID = FULL_RESPONSE_PROPERTY_ID(14u32);
pub const FR_PROP_FULLRESPONSENOPKCS7: FULL_RESPONSE_PROPERTY_ID = FULL_RESPONSE_PROPERTY_ID(15u32);
pub const FR_PROP_CAEXCHANGECERTIFICATEHASH: FULL_RESPONSE_PROPERTY_ID = FULL_RESPONSE_PROPERTY_ID(16u32);
pub const FR_PROP_CAEXCHANGECERTIFICATE: FULL_RESPONSE_PROPERTY_ID = FULL_RESPONSE_PROPERTY_ID(17u32);
pub const FR_PROP_CAEXCHANGECERTIFICATECHAIN: FULL_RESPONSE_PROPERTY_ID = FULL_RESPONSE_PROPERTY_ID(18u32);
pub const FR_PROP_CAEXCHANGECERTIFICATECRLCHAIN: FULL_RESPONSE_PROPERTY_ID = FULL_RESPONSE_PROPERTY_ID(19u32);
pub const FR_PROP_ATTESTATIONCHALLENGE: FULL_RESPONSE_PROPERTY_ID = FULL_RESPONSE_PROPERTY_ID(20u32);
pub const FR_PROP_ATTESTATIONPROVIDERNAME: FULL_RESPONSE_PROPERTY_ID = FULL_RESPONSE_PROPERTY_ID(21u32);
impl ::core::marker::Copy for FULL_RESPONSE_PROPERTY_ID {}
impl ::core::clone::Clone for FULL_RESPONSE_PROPERTY_ID {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IAlternativeName(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAlternativeNames(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IBinaryConverter(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IBinaryConverter2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICEnroll(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICEnroll2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICEnroll3(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICEnroll4(pub *mut ::core::ffi::c_void);
pub const ICF_ALLOWFOREIGN: u32 = 65536u32;
pub const ICF_EXISTINGROW: u32 = 131072u32;
#[repr(transparent)]
pub struct ICertAdmin(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICertAdmin2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICertConfig(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICertConfig2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICertEncodeAltName(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICertEncodeAltName2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICertEncodeBitString(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICertEncodeBitString2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICertEncodeCRLDistInfo(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICertEncodeCRLDistInfo2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICertEncodeDateArray(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICertEncodeDateArray2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICertEncodeLongArray(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICertEncodeLongArray2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICertEncodeStringArray(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICertEncodeStringArray2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICertExit(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICertExit2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICertGetConfig(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICertManageModule(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICertPolicy(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICertPolicy2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICertProperties(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICertProperty(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICertPropertyArchived(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICertPropertyArchivedKeyHash(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICertPropertyAutoEnroll(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICertPropertyBackedUp(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICertPropertyDescription(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICertPropertyEnrollment(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICertPropertyEnrollmentPolicyServer(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICertPropertyFriendlyName(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICertPropertyKeyProvInfo(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICertPropertyRenewal(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICertPropertyRequestOriginator(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICertPropertySHA1Hash(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICertRequest(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICertRequest2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICertRequest3(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICertRequestD(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICertRequestD2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICertServerExit(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICertServerPolicy(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICertView(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICertView2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICertificateAttestationChallenge(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICertificateAttestationChallenge2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICertificatePolicies(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICertificatePolicy(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICertificationAuthorities(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICertificationAuthority(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICryptAttribute(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICryptAttributes(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICspAlgorithm(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICspAlgorithms(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICspInformation(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICspInformations(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICspStatus(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICspStatuses(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IEnroll(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IEnroll2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IEnroll4(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IEnumCERTVIEWATTRIBUTE(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IEnumCERTVIEWCOLUMN(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IEnumCERTVIEWEXTENSION(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IEnumCERTVIEWROW(pub *mut ::core::ffi::c_void);
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
#[repr(transparent)]
pub struct IOCSPAdmin(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IOCSPCAConfiguration(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IOCSPCAConfigurationCollection(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IOCSPProperty(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IOCSPPropertyCollection(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IObjectId(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IObjectIds(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPolicyQualifier(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPolicyQualifiers(pub *mut ::core::ffi::c_void);
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
#[repr(transparent)]
pub struct ISignerCertificates(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISmimeCapabilities(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISmimeCapability(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IX500DistinguishedName(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IX509Attribute(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IX509AttributeArchiveKey(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IX509AttributeArchiveKeyHash(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IX509AttributeClientId(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IX509AttributeCspProvider(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IX509AttributeExtensions(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IX509AttributeOSVersion(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IX509AttributeRenewalCertificate(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IX509Attributes(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IX509CertificateRequest(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IX509CertificateRequestCertificate(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IX509CertificateRequestCertificate2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IX509CertificateRequestCmc(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IX509CertificateRequestCmc2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IX509CertificateRequestPkcs10(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IX509CertificateRequestPkcs10V2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IX509CertificateRequestPkcs10V3(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IX509CertificateRequestPkcs10V4(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IX509CertificateRequestPkcs7(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IX509CertificateRequestPkcs7V2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IX509CertificateRevocationList(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IX509CertificateRevocationListEntries(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IX509CertificateRevocationListEntry(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IX509CertificateTemplate(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IX509CertificateTemplateWritable(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IX509CertificateTemplates(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IX509EndorsementKey(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IX509Enrollment(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IX509Enrollment2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IX509EnrollmentHelper(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IX509EnrollmentPolicyServer(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IX509EnrollmentStatus(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IX509EnrollmentWebClassFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IX509Extension(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IX509ExtensionAlternativeNames(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IX509ExtensionAuthorityKeyIdentifier(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IX509ExtensionBasicConstraints(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IX509ExtensionCertificatePolicies(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IX509ExtensionEnhancedKeyUsage(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IX509ExtensionKeyUsage(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IX509ExtensionMSApplicationPolicies(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IX509ExtensionSmimeCapabilities(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IX509ExtensionSubjectKeyIdentifier(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IX509ExtensionTemplate(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IX509ExtensionTemplateName(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IX509Extensions(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IX509MachineEnrollmentFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IX509NameValuePair(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IX509NameValuePairs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IX509PolicyServerListManager(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IX509PolicyServerUrl(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IX509PrivateKey(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IX509PrivateKey2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IX509PublicKey(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IX509SCEPEnrollment(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IX509SCEPEnrollment2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IX509SCEPEnrollmentHelper(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IX509SignatureInformation(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ImportPFXFlags(pub i32);
pub const ImportNone: ImportPFXFlags = ImportPFXFlags(0i32);
pub const ImportMachineContext: ImportPFXFlags = ImportPFXFlags(1i32);
pub const ImportForceOverwrite: ImportPFXFlags = ImportPFXFlags(2i32);
pub const ImportSilent: ImportPFXFlags = ImportPFXFlags(4i32);
pub const ImportSaveProperties: ImportPFXFlags = ImportPFXFlags(8i32);
pub const ImportExportable: ImportPFXFlags = ImportPFXFlags(16i32);
pub const ImportExportableEncrypted: ImportPFXFlags = ImportPFXFlags(32i32);
pub const ImportNoUserProtected: ImportPFXFlags = ImportPFXFlags(64i32);
pub const ImportUserProtected: ImportPFXFlags = ImportPFXFlags(128i32);
pub const ImportUserProtectedHigh: ImportPFXFlags = ImportPFXFlags(256i32);
pub const ImportInstallCertificate: ImportPFXFlags = ImportPFXFlags(512i32);
pub const ImportInstallChain: ImportPFXFlags = ImportPFXFlags(1024i32);
pub const ImportInstallChainAndRoot: ImportPFXFlags = ImportPFXFlags(2048i32);
impl ::core::marker::Copy for ImportPFXFlags {}
impl ::core::clone::Clone for ImportPFXFlags {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct InnerRequestLevel(pub i32);
pub const LevelInnermost: InnerRequestLevel = InnerRequestLevel(0i32);
pub const LevelNext: InnerRequestLevel = InnerRequestLevel(1i32);
impl ::core::marker::Copy for InnerRequestLevel {}
impl ::core::clone::Clone for InnerRequestLevel {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct InstallResponseRestrictionFlags(pub i32);
pub const AllowNone: InstallResponseRestrictionFlags = InstallResponseRestrictionFlags(0i32);
pub const AllowNoOutstandingRequest: InstallResponseRestrictionFlags = InstallResponseRestrictionFlags(1i32);
pub const AllowUntrustedCertificate: InstallResponseRestrictionFlags = InstallResponseRestrictionFlags(2i32);
pub const AllowUntrustedRoot: InstallResponseRestrictionFlags = InstallResponseRestrictionFlags(4i32);
impl ::core::marker::Copy for InstallResponseRestrictionFlags {}
impl ::core::clone::Clone for InstallResponseRestrictionFlags {
    fn clone(&self) -> Self {
        *self
    }
}
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
#[repr(transparent)]
pub struct KeyAttestationClaimType(pub i32);
pub const XCN_NCRYPT_CLAIM_NONE: KeyAttestationClaimType = KeyAttestationClaimType(0i32);
pub const XCN_NCRYPT_CLAIM_AUTHORITY_AND_SUBJECT: KeyAttestationClaimType = KeyAttestationClaimType(3i32);
pub const XCN_NCRYPT_CLAIM_AUTHORITY_ONLY: KeyAttestationClaimType = KeyAttestationClaimType(1i32);
pub const XCN_NCRYPT_CLAIM_SUBJECT_ONLY: KeyAttestationClaimType = KeyAttestationClaimType(2i32);
pub const XCN_NCRYPT_CLAIM_UNKNOWN: KeyAttestationClaimType = KeyAttestationClaimType(4096i32);
impl ::core::marker::Copy for KeyAttestationClaimType {}
impl ::core::clone::Clone for KeyAttestationClaimType {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct KeyIdentifierHashAlgorithm(pub i32);
pub const SKIHashDefault: KeyIdentifierHashAlgorithm = KeyIdentifierHashAlgorithm(0i32);
pub const SKIHashSha1: KeyIdentifierHashAlgorithm = KeyIdentifierHashAlgorithm(1i32);
pub const SKIHashCapiSha1: KeyIdentifierHashAlgorithm = KeyIdentifierHashAlgorithm(2i32);
pub const SKIHashSha256: KeyIdentifierHashAlgorithm = KeyIdentifierHashAlgorithm(3i32);
pub const SKIHashHPKP: KeyIdentifierHashAlgorithm = KeyIdentifierHashAlgorithm(5i32);
impl ::core::marker::Copy for KeyIdentifierHashAlgorithm {}
impl ::core::clone::Clone for KeyIdentifierHashAlgorithm {
    fn clone(&self) -> Self {
        *self
    }
}
pub const LDAPF_SIGNDISABLE: u32 = 2u32;
pub const LDAPF_SSLENABLE: u32 = 1u32;
pub const OCSPAdmin: ::windows_sys::core::GUID = ::windows_sys::GUID {
    data1: 3556193553,
    data2: 37577,
    data3: 18379,
    data4: [143, 242, 141, 137, 26, 124, 77, 228],
};
pub const OCSPPropertyCollection: ::windows_sys::core::GUID = ::windows_sys::GUID {
    data1: 4181042472,
    data2: 47754,
    data3: 19929,
    data4: [186, 121, 242, 131, 39, 92, 178, 222],
};
#[repr(transparent)]
pub struct OCSPRequestFlag(pub i32);
pub const OCSP_RF_REJECT_SIGNED_REQUESTS: OCSPRequestFlag = OCSPRequestFlag(1i32);
impl ::core::marker::Copy for OCSPRequestFlag {}
impl ::core::clone::Clone for OCSPRequestFlag {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct OCSPSigningFlag(pub i32);
pub const OCSP_SF_SILENT: OCSPSigningFlag = OCSPSigningFlag(1i32);
pub const OCSP_SF_USE_CACERT: OCSPSigningFlag = OCSPSigningFlag(2i32);
pub const OCSP_SF_ALLOW_SIGNINGCERT_AUTORENEWAL: OCSPSigningFlag = OCSPSigningFlag(4i32);
pub const OCSP_SF_FORCE_SIGNINGCERT_ISSUER_ISCA: OCSPSigningFlag = OCSPSigningFlag(8i32);
pub const OCSP_SF_AUTODISCOVER_SIGNINGCERT: OCSPSigningFlag = OCSPSigningFlag(16i32);
pub const OCSP_SF_MANUAL_ASSIGN_SIGNINGCERT: OCSPSigningFlag = OCSPSigningFlag(32i32);
pub const OCSP_SF_RESPONDER_ID_KEYHASH: OCSPSigningFlag = OCSPSigningFlag(64i32);
pub const OCSP_SF_RESPONDER_ID_NAME: OCSPSigningFlag = OCSPSigningFlag(128i32);
pub const OCSP_SF_ALLOW_NONCE_EXTENSION: OCSPSigningFlag = OCSPSigningFlag(256i32);
pub const OCSP_SF_ALLOW_SIGNINGCERT_AUTOENROLLMENT: OCSPSigningFlag = OCSPSigningFlag(512i32);
impl ::core::marker::Copy for OCSPSigningFlag {}
impl ::core::clone::Clone for OCSPSigningFlag {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ObjectIdGroupId(pub i32);
pub const XCN_CRYPT_ANY_GROUP_ID: ObjectIdGroupId = ObjectIdGroupId(0i32);
pub const XCN_CRYPT_HASH_ALG_OID_GROUP_ID: ObjectIdGroupId = ObjectIdGroupId(1i32);
pub const XCN_CRYPT_ENCRYPT_ALG_OID_GROUP_ID: ObjectIdGroupId = ObjectIdGroupId(2i32);
pub const XCN_CRYPT_PUBKEY_ALG_OID_GROUP_ID: ObjectIdGroupId = ObjectIdGroupId(3i32);
pub const XCN_CRYPT_SIGN_ALG_OID_GROUP_ID: ObjectIdGroupId = ObjectIdGroupId(4i32);
pub const XCN_CRYPT_RDN_ATTR_OID_GROUP_ID: ObjectIdGroupId = ObjectIdGroupId(5i32);
pub const XCN_CRYPT_EXT_OR_ATTR_OID_GROUP_ID: ObjectIdGroupId = ObjectIdGroupId(6i32);
pub const XCN_CRYPT_ENHKEY_USAGE_OID_GROUP_ID: ObjectIdGroupId = ObjectIdGroupId(7i32);
pub const XCN_CRYPT_POLICY_OID_GROUP_ID: ObjectIdGroupId = ObjectIdGroupId(8i32);
pub const XCN_CRYPT_TEMPLATE_OID_GROUP_ID: ObjectIdGroupId = ObjectIdGroupId(9i32);
pub const XCN_CRYPT_KDF_OID_GROUP_ID: ObjectIdGroupId = ObjectIdGroupId(10i32);
pub const XCN_CRYPT_LAST_OID_GROUP_ID: ObjectIdGroupId = ObjectIdGroupId(10i32);
pub const XCN_CRYPT_FIRST_ALG_OID_GROUP_ID: ObjectIdGroupId = ObjectIdGroupId(1i32);
pub const XCN_CRYPT_LAST_ALG_OID_GROUP_ID: ObjectIdGroupId = ObjectIdGroupId(4i32);
pub const XCN_CRYPT_GROUP_ID_MASK: ObjectIdGroupId = ObjectIdGroupId(65535i32);
pub const XCN_CRYPT_OID_PREFER_CNG_ALGID_FLAG: ObjectIdGroupId = ObjectIdGroupId(1073741824i32);
pub const XCN_CRYPT_OID_DISABLE_SEARCH_DS_FLAG: ObjectIdGroupId = ObjectIdGroupId(-2147483648i32);
pub const XCN_CRYPT_OID_INFO_OID_GROUP_BIT_LEN_MASK: ObjectIdGroupId = ObjectIdGroupId(268369920i32);
pub const XCN_CRYPT_OID_INFO_OID_GROUP_BIT_LEN_SHIFT: ObjectIdGroupId = ObjectIdGroupId(16i32);
pub const XCN_CRYPT_KEY_LENGTH_MASK: ObjectIdGroupId = ObjectIdGroupId(268369920i32);
impl ::core::marker::Copy for ObjectIdGroupId {}
impl ::core::clone::Clone for ObjectIdGroupId {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ObjectIdPublicKeyFlags(pub i32);
pub const XCN_CRYPT_OID_INFO_PUBKEY_ANY: ObjectIdPublicKeyFlags = ObjectIdPublicKeyFlags(0i32);
pub const XCN_CRYPT_OID_INFO_PUBKEY_SIGN_KEY_FLAG: ObjectIdPublicKeyFlags = ObjectIdPublicKeyFlags(-2147483648i32);
pub const XCN_CRYPT_OID_INFO_PUBKEY_ENCRYPT_KEY_FLAG: ObjectIdPublicKeyFlags = ObjectIdPublicKeyFlags(1073741824i32);
impl ::core::marker::Copy for ObjectIdPublicKeyFlags {}
impl ::core::clone::Clone for ObjectIdPublicKeyFlags {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct PENDING_REQUEST_DESIRED_PROPERTY(pub u32);
pub const XEPR_CADNS: PENDING_REQUEST_DESIRED_PROPERTY = PENDING_REQUEST_DESIRED_PROPERTY(1u32);
pub const XEPR_CAFRIENDLYNAME: PENDING_REQUEST_DESIRED_PROPERTY = PENDING_REQUEST_DESIRED_PROPERTY(3u32);
pub const XEPR_CANAME: PENDING_REQUEST_DESIRED_PROPERTY = PENDING_REQUEST_DESIRED_PROPERTY(2u32);
pub const XEPR_HASH: PENDING_REQUEST_DESIRED_PROPERTY = PENDING_REQUEST_DESIRED_PROPERTY(8u32);
pub const XEPR_REQUESTID: PENDING_REQUEST_DESIRED_PROPERTY = PENDING_REQUEST_DESIRED_PROPERTY(4u32);
impl ::core::marker::Copy for PENDING_REQUEST_DESIRED_PROPERTY {}
impl ::core::clone::Clone for PENDING_REQUEST_DESIRED_PROPERTY {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct PFXExportOptions(pub i32);
pub const PFXExportEEOnly: PFXExportOptions = PFXExportOptions(0i32);
pub const PFXExportChainNoRoot: PFXExportOptions = PFXExportOptions(1i32);
pub const PFXExportChainWithRoot: PFXExportOptions = PFXExportOptions(2i32);
impl ::core::marker::Copy for PFXExportOptions {}
impl ::core::clone::Clone for PFXExportOptions {
    fn clone(&self) -> Self {
        *self
    }
}
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
#[repr(transparent)]
pub struct Pkcs10AllowedSignatureTypes(pub i32);
pub const AllowedKeySignature: Pkcs10AllowedSignatureTypes = Pkcs10AllowedSignatureTypes(1i32);
pub const AllowedNullSignature: Pkcs10AllowedSignatureTypes = Pkcs10AllowedSignatureTypes(2i32);
impl ::core::marker::Copy for Pkcs10AllowedSignatureTypes {}
impl ::core::clone::Clone for Pkcs10AllowedSignatureTypes {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct PolicyQualifierType(pub i32);
pub const PolicyQualifierTypeUnknown: PolicyQualifierType = PolicyQualifierType(0i32);
pub const PolicyQualifierTypeUrl: PolicyQualifierType = PolicyQualifierType(1i32);
pub const PolicyQualifierTypeUserNotice: PolicyQualifierType = PolicyQualifierType(2i32);
pub const PolicyQualifierTypeFlags: PolicyQualifierType = PolicyQualifierType(3i32);
impl ::core::marker::Copy for PolicyQualifierType {}
impl ::core::clone::Clone for PolicyQualifierType {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct PolicyServerUrlFlags(pub i32);
pub const PsfNone: PolicyServerUrlFlags = PolicyServerUrlFlags(0i32);
pub const PsfLocationGroupPolicy: PolicyServerUrlFlags = PolicyServerUrlFlags(1i32);
pub const PsfLocationRegistry: PolicyServerUrlFlags = PolicyServerUrlFlags(2i32);
pub const PsfUseClientId: PolicyServerUrlFlags = PolicyServerUrlFlags(4i32);
pub const PsfAutoEnrollmentEnabled: PolicyServerUrlFlags = PolicyServerUrlFlags(16i32);
pub const PsfAllowUnTrustedCA: PolicyServerUrlFlags = PolicyServerUrlFlags(32i32);
impl ::core::marker::Copy for PolicyServerUrlFlags {}
impl ::core::clone::Clone for PolicyServerUrlFlags {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct PolicyServerUrlPropertyID(pub i32);
pub const PsPolicyID: PolicyServerUrlPropertyID = PolicyServerUrlPropertyID(0i32);
pub const PsFriendlyName: PolicyServerUrlPropertyID = PolicyServerUrlPropertyID(1i32);
impl ::core::marker::Copy for PolicyServerUrlPropertyID {}
impl ::core::clone::Clone for PolicyServerUrlPropertyID {
    fn clone(&self) -> Self {
        *self
    }
}
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
#[repr(transparent)]
pub struct RequestClientInfoClientId(pub i32);
pub const ClientIdNone: RequestClientInfoClientId = RequestClientInfoClientId(0i32);
pub const ClientIdXEnroll2003: RequestClientInfoClientId = RequestClientInfoClientId(1i32);
pub const ClientIdAutoEnroll2003: RequestClientInfoClientId = RequestClientInfoClientId(2i32);
pub const ClientIdWizard2003: RequestClientInfoClientId = RequestClientInfoClientId(3i32);
pub const ClientIdCertReq2003: RequestClientInfoClientId = RequestClientInfoClientId(4i32);
pub const ClientIdDefaultRequest: RequestClientInfoClientId = RequestClientInfoClientId(5i32);
pub const ClientIdAutoEnroll: RequestClientInfoClientId = RequestClientInfoClientId(6i32);
pub const ClientIdRequestWizard: RequestClientInfoClientId = RequestClientInfoClientId(7i32);
pub const ClientIdEOBO: RequestClientInfoClientId = RequestClientInfoClientId(8i32);
pub const ClientIdCertReq: RequestClientInfoClientId = RequestClientInfoClientId(9i32);
pub const ClientIdTest: RequestClientInfoClientId = RequestClientInfoClientId(10i32);
pub const ClientIdWinRT: RequestClientInfoClientId = RequestClientInfoClientId(11i32);
pub const ClientIdUserStart: RequestClientInfoClientId = RequestClientInfoClientId(1000i32);
impl ::core::marker::Copy for RequestClientInfoClientId {}
impl ::core::clone::Clone for RequestClientInfoClientId {
    fn clone(&self) -> Self {
        *self
    }
}
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
#[repr(transparent)]
pub struct WebEnrollmentFlags(pub i32);
pub const EnrollPrompt: WebEnrollmentFlags = WebEnrollmentFlags(1i32);
impl ::core::marker::Copy for WebEnrollmentFlags {}
impl ::core::clone::Clone for WebEnrollmentFlags {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct WebSecurityLevel(pub i32);
pub const LevelUnsafe: WebSecurityLevel = WebSecurityLevel(0i32);
pub const LevelSafe: WebSecurityLevel = WebSecurityLevel(1i32);
impl ::core::marker::Copy for WebSecurityLevel {}
impl ::core::clone::Clone for WebSecurityLevel {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct X500NameFlags(pub i32);
pub const XCN_CERT_NAME_STR_NONE: X500NameFlags = X500NameFlags(0i32);
pub const XCN_CERT_SIMPLE_NAME_STR: X500NameFlags = X500NameFlags(1i32);
pub const XCN_CERT_OID_NAME_STR: X500NameFlags = X500NameFlags(2i32);
pub const XCN_CERT_X500_NAME_STR: X500NameFlags = X500NameFlags(3i32);
pub const XCN_CERT_XML_NAME_STR: X500NameFlags = X500NameFlags(4i32);
pub const XCN_CERT_NAME_STR_SEMICOLON_FLAG: X500NameFlags = X500NameFlags(1073741824i32);
pub const XCN_CERT_NAME_STR_NO_PLUS_FLAG: X500NameFlags = X500NameFlags(536870912i32);
pub const XCN_CERT_NAME_STR_NO_QUOTING_FLAG: X500NameFlags = X500NameFlags(268435456i32);
pub const XCN_CERT_NAME_STR_CRLF_FLAG: X500NameFlags = X500NameFlags(134217728i32);
pub const XCN_CERT_NAME_STR_COMMA_FLAG: X500NameFlags = X500NameFlags(67108864i32);
pub const XCN_CERT_NAME_STR_REVERSE_FLAG: X500NameFlags = X500NameFlags(33554432i32);
pub const XCN_CERT_NAME_STR_FORWARD_FLAG: X500NameFlags = X500NameFlags(16777216i32);
pub const XCN_CERT_NAME_STR_AMBIGUOUS_SEPARATOR_FLAGS: X500NameFlags = X500NameFlags(1275068416i32);
pub const XCN_CERT_NAME_STR_DISABLE_IE4_UTF8_FLAG: X500NameFlags = X500NameFlags(65536i32);
pub const XCN_CERT_NAME_STR_ENABLE_T61_UNICODE_FLAG: X500NameFlags = X500NameFlags(131072i32);
pub const XCN_CERT_NAME_STR_ENABLE_UTF8_UNICODE_FLAG: X500NameFlags = X500NameFlags(262144i32);
pub const XCN_CERT_NAME_STR_FORCE_UTF8_DIR_STR_FLAG: X500NameFlags = X500NameFlags(524288i32);
pub const XCN_CERT_NAME_STR_DISABLE_UTF8_DIR_STR_FLAG: X500NameFlags = X500NameFlags(1048576i32);
pub const XCN_CERT_NAME_STR_ENABLE_PUNYCODE_FLAG: X500NameFlags = X500NameFlags(2097152i32);
pub const XCN_CERT_NAME_STR_DS_ESCAPED: X500NameFlags = X500NameFlags(8388608i32);
impl ::core::marker::Copy for X500NameFlags {}
impl ::core::clone::Clone for X500NameFlags {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct X509CertificateEnrollmentContext(pub i32);
pub const ContextNone: X509CertificateEnrollmentContext = X509CertificateEnrollmentContext(0i32);
pub const ContextUser: X509CertificateEnrollmentContext = X509CertificateEnrollmentContext(1i32);
pub const ContextMachine: X509CertificateEnrollmentContext = X509CertificateEnrollmentContext(2i32);
pub const ContextAdministratorForceMachine: X509CertificateEnrollmentContext = X509CertificateEnrollmentContext(3i32);
impl ::core::marker::Copy for X509CertificateEnrollmentContext {}
impl ::core::clone::Clone for X509CertificateEnrollmentContext {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct X509CertificateTemplateEnrollmentFlag(pub i32);
pub const EnrollmentIncludeSymmetricAlgorithms: X509CertificateTemplateEnrollmentFlag = X509CertificateTemplateEnrollmentFlag(1i32);
pub const EnrollmentPendAllRequests: X509CertificateTemplateEnrollmentFlag = X509CertificateTemplateEnrollmentFlag(2i32);
pub const EnrollmentPublishToKRAContainer: X509CertificateTemplateEnrollmentFlag = X509CertificateTemplateEnrollmentFlag(4i32);
pub const EnrollmentPublishToDS: X509CertificateTemplateEnrollmentFlag = X509CertificateTemplateEnrollmentFlag(8i32);
pub const EnrollmentAutoEnrollmentCheckUserDSCertificate: X509CertificateTemplateEnrollmentFlag = X509CertificateTemplateEnrollmentFlag(16i32);
pub const EnrollmentAutoEnrollment: X509CertificateTemplateEnrollmentFlag = X509CertificateTemplateEnrollmentFlag(32i32);
pub const EnrollmentDomainAuthenticationNotRequired: X509CertificateTemplateEnrollmentFlag = X509CertificateTemplateEnrollmentFlag(128i32);
pub const EnrollmentPreviousApprovalValidateReenrollment: X509CertificateTemplateEnrollmentFlag = X509CertificateTemplateEnrollmentFlag(64i32);
pub const EnrollmentUserInteractionRequired: X509CertificateTemplateEnrollmentFlag = X509CertificateTemplateEnrollmentFlag(256i32);
pub const EnrollmentAddTemplateName: X509CertificateTemplateEnrollmentFlag = X509CertificateTemplateEnrollmentFlag(512i32);
pub const EnrollmentRemoveInvalidCertificateFromPersonalStore: X509CertificateTemplateEnrollmentFlag = X509CertificateTemplateEnrollmentFlag(1024i32);
pub const EnrollmentAllowEnrollOnBehalfOf: X509CertificateTemplateEnrollmentFlag = X509CertificateTemplateEnrollmentFlag(2048i32);
pub const EnrollmentAddOCSPNoCheck: X509CertificateTemplateEnrollmentFlag = X509CertificateTemplateEnrollmentFlag(4096i32);
pub const EnrollmentReuseKeyOnFullSmartCard: X509CertificateTemplateEnrollmentFlag = X509CertificateTemplateEnrollmentFlag(8192i32);
pub const EnrollmentNoRevocationInfoInCerts: X509CertificateTemplateEnrollmentFlag = X509CertificateTemplateEnrollmentFlag(16384i32);
pub const EnrollmentIncludeBasicConstraintsForEECerts: X509CertificateTemplateEnrollmentFlag = X509CertificateTemplateEnrollmentFlag(32768i32);
pub const EnrollmentPreviousApprovalKeyBasedValidateReenrollment: X509CertificateTemplateEnrollmentFlag = X509CertificateTemplateEnrollmentFlag(65536i32);
pub const EnrollmentCertificateIssuancePoliciesFromRequest: X509CertificateTemplateEnrollmentFlag = X509CertificateTemplateEnrollmentFlag(131072i32);
pub const EnrollmentSkipAutoRenewal: X509CertificateTemplateEnrollmentFlag = X509CertificateTemplateEnrollmentFlag(262144i32);
impl ::core::marker::Copy for X509CertificateTemplateEnrollmentFlag {}
impl ::core::clone::Clone for X509CertificateTemplateEnrollmentFlag {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct X509CertificateTemplateGeneralFlag(pub i32);
pub const GeneralMachineType: X509CertificateTemplateGeneralFlag = X509CertificateTemplateGeneralFlag(64i32);
pub const GeneralCA: X509CertificateTemplateGeneralFlag = X509CertificateTemplateGeneralFlag(128i32);
pub const GeneralCrossCA: X509CertificateTemplateGeneralFlag = X509CertificateTemplateGeneralFlag(2048i32);
pub const GeneralDefault: X509CertificateTemplateGeneralFlag = X509CertificateTemplateGeneralFlag(65536i32);
pub const GeneralModified: X509CertificateTemplateGeneralFlag = X509CertificateTemplateGeneralFlag(131072i32);
pub const GeneralDonotPersist: X509CertificateTemplateGeneralFlag = X509CertificateTemplateGeneralFlag(4096i32);
impl ::core::marker::Copy for X509CertificateTemplateGeneralFlag {}
impl ::core::clone::Clone for X509CertificateTemplateGeneralFlag {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct X509CertificateTemplatePrivateKeyFlag(pub i32);
pub const PrivateKeyRequireArchival: X509CertificateTemplatePrivateKeyFlag = X509CertificateTemplatePrivateKeyFlag(1i32);
pub const PrivateKeyExportable: X509CertificateTemplatePrivateKeyFlag = X509CertificateTemplatePrivateKeyFlag(16i32);
pub const PrivateKeyRequireStrongKeyProtection: X509CertificateTemplatePrivateKeyFlag = X509CertificateTemplatePrivateKeyFlag(32i32);
pub const PrivateKeyRequireAlternateSignatureAlgorithm: X509CertificateTemplatePrivateKeyFlag = X509CertificateTemplatePrivateKeyFlag(64i32);
pub const PrivateKeyRequireSameKeyRenewal: X509CertificateTemplatePrivateKeyFlag = X509CertificateTemplatePrivateKeyFlag(128i32);
pub const PrivateKeyUseLegacyProvider: X509CertificateTemplatePrivateKeyFlag = X509CertificateTemplatePrivateKeyFlag(256i32);
pub const PrivateKeyEKTrustOnUse: X509CertificateTemplatePrivateKeyFlag = X509CertificateTemplatePrivateKeyFlag(512i32);
pub const PrivateKeyEKValidateCert: X509CertificateTemplatePrivateKeyFlag = X509CertificateTemplatePrivateKeyFlag(1024i32);
pub const PrivateKeyEKValidateKey: X509CertificateTemplatePrivateKeyFlag = X509CertificateTemplatePrivateKeyFlag(2048i32);
pub const PrivateKeyAttestNone: X509CertificateTemplatePrivateKeyFlag = X509CertificateTemplatePrivateKeyFlag(0i32);
pub const PrivateKeyAttestPreferred: X509CertificateTemplatePrivateKeyFlag = X509CertificateTemplatePrivateKeyFlag(4096i32);
pub const PrivateKeyAttestRequired: X509CertificateTemplatePrivateKeyFlag = X509CertificateTemplatePrivateKeyFlag(8192i32);
pub const PrivateKeyAttestMask: X509CertificateTemplatePrivateKeyFlag = X509CertificateTemplatePrivateKeyFlag(12288i32);
pub const PrivateKeyAttestWithoutPolicy: X509CertificateTemplatePrivateKeyFlag = X509CertificateTemplatePrivateKeyFlag(16384i32);
pub const PrivateKeyServerVersionMask: X509CertificateTemplatePrivateKeyFlag = X509CertificateTemplatePrivateKeyFlag(983040i32);
pub const PrivateKeyServerVersionShift: X509CertificateTemplatePrivateKeyFlag = X509CertificateTemplatePrivateKeyFlag(16i32);
pub const PrivateKeyHelloKspKey: X509CertificateTemplatePrivateKeyFlag = X509CertificateTemplatePrivateKeyFlag(1048576i32);
pub const PrivateKeyHelloLogonKey: X509CertificateTemplatePrivateKeyFlag = X509CertificateTemplatePrivateKeyFlag(2097152i32);
pub const PrivateKeyClientVersionMask: X509CertificateTemplatePrivateKeyFlag = X509CertificateTemplatePrivateKeyFlag(251658240i32);
pub const PrivateKeyClientVersionShift: X509CertificateTemplatePrivateKeyFlag = X509CertificateTemplatePrivateKeyFlag(24i32);
impl ::core::marker::Copy for X509CertificateTemplatePrivateKeyFlag {}
impl ::core::clone::Clone for X509CertificateTemplatePrivateKeyFlag {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct X509CertificateTemplateSubjectNameFlag(pub i32);
pub const SubjectNameEnrolleeSupplies: X509CertificateTemplateSubjectNameFlag = X509CertificateTemplateSubjectNameFlag(1i32);
pub const SubjectNameRequireDirectoryPath: X509CertificateTemplateSubjectNameFlag = X509CertificateTemplateSubjectNameFlag(-2147483648i32);
pub const SubjectNameRequireCommonName: X509CertificateTemplateSubjectNameFlag = X509CertificateTemplateSubjectNameFlag(1073741824i32);
pub const SubjectNameRequireEmail: X509CertificateTemplateSubjectNameFlag = X509CertificateTemplateSubjectNameFlag(536870912i32);
pub const SubjectNameRequireDNS: X509CertificateTemplateSubjectNameFlag = X509CertificateTemplateSubjectNameFlag(268435456i32);
pub const SubjectNameAndAlternativeNameOldCertSupplies: X509CertificateTemplateSubjectNameFlag = X509CertificateTemplateSubjectNameFlag(8i32);
pub const SubjectAlternativeNameEnrolleeSupplies: X509CertificateTemplateSubjectNameFlag = X509CertificateTemplateSubjectNameFlag(65536i32);
pub const SubjectAlternativeNameRequireDirectoryGUID: X509CertificateTemplateSubjectNameFlag = X509CertificateTemplateSubjectNameFlag(16777216i32);
pub const SubjectAlternativeNameRequireUPN: X509CertificateTemplateSubjectNameFlag = X509CertificateTemplateSubjectNameFlag(33554432i32);
pub const SubjectAlternativeNameRequireEmail: X509CertificateTemplateSubjectNameFlag = X509CertificateTemplateSubjectNameFlag(67108864i32);
pub const SubjectAlternativeNameRequireSPN: X509CertificateTemplateSubjectNameFlag = X509CertificateTemplateSubjectNameFlag(8388608i32);
pub const SubjectAlternativeNameRequireDNS: X509CertificateTemplateSubjectNameFlag = X509CertificateTemplateSubjectNameFlag(134217728i32);
pub const SubjectAlternativeNameRequireDomainDNS: X509CertificateTemplateSubjectNameFlag = X509CertificateTemplateSubjectNameFlag(4194304i32);
impl ::core::marker::Copy for X509CertificateTemplateSubjectNameFlag {}
impl ::core::clone::Clone for X509CertificateTemplateSubjectNameFlag {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct X509EnrollmentAuthFlags(pub i32);
pub const X509AuthNone: X509EnrollmentAuthFlags = X509EnrollmentAuthFlags(0i32);
pub const X509AuthAnonymous: X509EnrollmentAuthFlags = X509EnrollmentAuthFlags(1i32);
pub const X509AuthKerberos: X509EnrollmentAuthFlags = X509EnrollmentAuthFlags(2i32);
pub const X509AuthUsername: X509EnrollmentAuthFlags = X509EnrollmentAuthFlags(4i32);
pub const X509AuthCertificate: X509EnrollmentAuthFlags = X509EnrollmentAuthFlags(8i32);
impl ::core::marker::Copy for X509EnrollmentAuthFlags {}
impl ::core::clone::Clone for X509EnrollmentAuthFlags {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct X509EnrollmentPolicyExportFlags(pub i32);
pub const ExportTemplates: X509EnrollmentPolicyExportFlags = X509EnrollmentPolicyExportFlags(1i32);
pub const ExportOIDs: X509EnrollmentPolicyExportFlags = X509EnrollmentPolicyExportFlags(2i32);
pub const ExportCAs: X509EnrollmentPolicyExportFlags = X509EnrollmentPolicyExportFlags(4i32);
impl ::core::marker::Copy for X509EnrollmentPolicyExportFlags {}
impl ::core::clone::Clone for X509EnrollmentPolicyExportFlags {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct X509EnrollmentPolicyLoadOption(pub i32);
pub const LoadOptionDefault: X509EnrollmentPolicyLoadOption = X509EnrollmentPolicyLoadOption(0i32);
pub const LoadOptionCacheOnly: X509EnrollmentPolicyLoadOption = X509EnrollmentPolicyLoadOption(1i32);
pub const LoadOptionReload: X509EnrollmentPolicyLoadOption = X509EnrollmentPolicyLoadOption(2i32);
pub const LoadOptionRegisterForADChanges: X509EnrollmentPolicyLoadOption = X509EnrollmentPolicyLoadOption(4i32);
impl ::core::marker::Copy for X509EnrollmentPolicyLoadOption {}
impl ::core::clone::Clone for X509EnrollmentPolicyLoadOption {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct X509HardwareKeyUsageFlags(pub i32);
pub const XCN_NCRYPT_PCP_NONE: X509HardwareKeyUsageFlags = X509HardwareKeyUsageFlags(0i32);
pub const XCN_NCRYPT_TPM12_PROVIDER: X509HardwareKeyUsageFlags = X509HardwareKeyUsageFlags(65536i32);
pub const XCN_NCRYPT_PCP_SIGNATURE_KEY: X509HardwareKeyUsageFlags = X509HardwareKeyUsageFlags(1i32);
pub const XCN_NCRYPT_PCP_ENCRYPTION_KEY: X509HardwareKeyUsageFlags = X509HardwareKeyUsageFlags(2i32);
pub const XCN_NCRYPT_PCP_GENERIC_KEY: X509HardwareKeyUsageFlags = X509HardwareKeyUsageFlags(3i32);
pub const XCN_NCRYPT_PCP_STORAGE_KEY: X509HardwareKeyUsageFlags = X509HardwareKeyUsageFlags(4i32);
pub const XCN_NCRYPT_PCP_IDENTITY_KEY: X509HardwareKeyUsageFlags = X509HardwareKeyUsageFlags(8i32);
impl ::core::marker::Copy for X509HardwareKeyUsageFlags {}
impl ::core::clone::Clone for X509HardwareKeyUsageFlags {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct X509KeyParametersExportType(pub i32);
pub const XCN_CRYPT_OID_USE_CURVE_NONE: X509KeyParametersExportType = X509KeyParametersExportType(0i32);
pub const XCN_CRYPT_OID_USE_CURVE_NAME_FOR_ENCODE_FLAG: X509KeyParametersExportType = X509KeyParametersExportType(536870912i32);
pub const XCN_CRYPT_OID_USE_CURVE_PARAMETERS_FOR_ENCODE_FLAG: X509KeyParametersExportType = X509KeyParametersExportType(268435456i32);
impl ::core::marker::Copy for X509KeyParametersExportType {}
impl ::core::clone::Clone for X509KeyParametersExportType {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct X509KeySpec(pub i32);
pub const XCN_AT_NONE: X509KeySpec = X509KeySpec(0i32);
pub const XCN_AT_KEYEXCHANGE: X509KeySpec = X509KeySpec(1i32);
pub const XCN_AT_SIGNATURE: X509KeySpec = X509KeySpec(2i32);
impl ::core::marker::Copy for X509KeySpec {}
impl ::core::clone::Clone for X509KeySpec {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct X509KeyUsageFlags(pub i32);
pub const XCN_CERT_NO_KEY_USAGE: X509KeyUsageFlags = X509KeyUsageFlags(0i32);
pub const XCN_CERT_DIGITAL_SIGNATURE_KEY_USAGE: X509KeyUsageFlags = X509KeyUsageFlags(128i32);
pub const XCN_CERT_NON_REPUDIATION_KEY_USAGE: X509KeyUsageFlags = X509KeyUsageFlags(64i32);
pub const XCN_CERT_KEY_ENCIPHERMENT_KEY_USAGE: X509KeyUsageFlags = X509KeyUsageFlags(32i32);
pub const XCN_CERT_DATA_ENCIPHERMENT_KEY_USAGE: X509KeyUsageFlags = X509KeyUsageFlags(16i32);
pub const XCN_CERT_KEY_AGREEMENT_KEY_USAGE: X509KeyUsageFlags = X509KeyUsageFlags(8i32);
pub const XCN_CERT_KEY_CERT_SIGN_KEY_USAGE: X509KeyUsageFlags = X509KeyUsageFlags(4i32);
pub const XCN_CERT_OFFLINE_CRL_SIGN_KEY_USAGE: X509KeyUsageFlags = X509KeyUsageFlags(2i32);
pub const XCN_CERT_CRL_SIGN_KEY_USAGE: X509KeyUsageFlags = X509KeyUsageFlags(2i32);
pub const XCN_CERT_ENCIPHER_ONLY_KEY_USAGE: X509KeyUsageFlags = X509KeyUsageFlags(1i32);
pub const XCN_CERT_DECIPHER_ONLY_KEY_USAGE: X509KeyUsageFlags = X509KeyUsageFlags(32768i32);
impl ::core::marker::Copy for X509KeyUsageFlags {}
impl ::core::clone::Clone for X509KeyUsageFlags {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct X509PrivateKeyExportFlags(pub i32);
pub const XCN_NCRYPT_ALLOW_EXPORT_NONE: X509PrivateKeyExportFlags = X509PrivateKeyExportFlags(0i32);
pub const XCN_NCRYPT_ALLOW_EXPORT_FLAG: X509PrivateKeyExportFlags = X509PrivateKeyExportFlags(1i32);
pub const XCN_NCRYPT_ALLOW_PLAINTEXT_EXPORT_FLAG: X509PrivateKeyExportFlags = X509PrivateKeyExportFlags(2i32);
pub const XCN_NCRYPT_ALLOW_ARCHIVING_FLAG: X509PrivateKeyExportFlags = X509PrivateKeyExportFlags(4i32);
pub const XCN_NCRYPT_ALLOW_PLAINTEXT_ARCHIVING_FLAG: X509PrivateKeyExportFlags = X509PrivateKeyExportFlags(8i32);
impl ::core::marker::Copy for X509PrivateKeyExportFlags {}
impl ::core::clone::Clone for X509PrivateKeyExportFlags {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct X509PrivateKeyProtection(pub i32);
pub const XCN_NCRYPT_UI_NO_PROTECTION_FLAG: X509PrivateKeyProtection = X509PrivateKeyProtection(0i32);
pub const XCN_NCRYPT_UI_PROTECT_KEY_FLAG: X509PrivateKeyProtection = X509PrivateKeyProtection(1i32);
pub const XCN_NCRYPT_UI_FORCE_HIGH_PROTECTION_FLAG: X509PrivateKeyProtection = X509PrivateKeyProtection(2i32);
pub const XCN_NCRYPT_UI_FINGERPRINT_PROTECTION_FLAG: X509PrivateKeyProtection = X509PrivateKeyProtection(4i32);
pub const XCN_NCRYPT_UI_APPCONTAINER_ACCESS_MEDIUM_FLAG: X509PrivateKeyProtection = X509PrivateKeyProtection(8i32);
impl ::core::marker::Copy for X509PrivateKeyProtection {}
impl ::core::clone::Clone for X509PrivateKeyProtection {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct X509PrivateKeyUsageFlags(pub i32);
pub const XCN_NCRYPT_ALLOW_USAGES_NONE: X509PrivateKeyUsageFlags = X509PrivateKeyUsageFlags(0i32);
pub const XCN_NCRYPT_ALLOW_DECRYPT_FLAG: X509PrivateKeyUsageFlags = X509PrivateKeyUsageFlags(1i32);
pub const XCN_NCRYPT_ALLOW_SIGNING_FLAG: X509PrivateKeyUsageFlags = X509PrivateKeyUsageFlags(2i32);
pub const XCN_NCRYPT_ALLOW_KEY_AGREEMENT_FLAG: X509PrivateKeyUsageFlags = X509PrivateKeyUsageFlags(4i32);
pub const XCN_NCRYPT_ALLOW_KEY_IMPORT_FLAG: X509PrivateKeyUsageFlags = X509PrivateKeyUsageFlags(8i32);
pub const XCN_NCRYPT_ALLOW_ALL_USAGES: X509PrivateKeyUsageFlags = X509PrivateKeyUsageFlags(16777215i32);
impl ::core::marker::Copy for X509PrivateKeyUsageFlags {}
impl ::core::clone::Clone for X509PrivateKeyUsageFlags {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct X509PrivateKeyVerify(pub i32);
pub const VerifyNone: X509PrivateKeyVerify = X509PrivateKeyVerify(0i32);
pub const VerifySilent: X509PrivateKeyVerify = X509PrivateKeyVerify(1i32);
pub const VerifySmartCardNone: X509PrivateKeyVerify = X509PrivateKeyVerify(2i32);
pub const VerifySmartCardSilent: X509PrivateKeyVerify = X509PrivateKeyVerify(3i32);
pub const VerifyAllowUI: X509PrivateKeyVerify = X509PrivateKeyVerify(4i32);
impl ::core::marker::Copy for X509PrivateKeyVerify {}
impl ::core::clone::Clone for X509PrivateKeyVerify {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct X509ProviderType(pub i32);
pub const XCN_PROV_NONE: X509ProviderType = X509ProviderType(0i32);
pub const XCN_PROV_RSA_FULL: X509ProviderType = X509ProviderType(1i32);
pub const XCN_PROV_RSA_SIG: X509ProviderType = X509ProviderType(2i32);
pub const XCN_PROV_DSS: X509ProviderType = X509ProviderType(3i32);
pub const XCN_PROV_FORTEZZA: X509ProviderType = X509ProviderType(4i32);
pub const XCN_PROV_MS_EXCHANGE: X509ProviderType = X509ProviderType(5i32);
pub const XCN_PROV_SSL: X509ProviderType = X509ProviderType(6i32);
pub const XCN_PROV_RSA_SCHANNEL: X509ProviderType = X509ProviderType(12i32);
pub const XCN_PROV_DSS_DH: X509ProviderType = X509ProviderType(13i32);
pub const XCN_PROV_EC_ECDSA_SIG: X509ProviderType = X509ProviderType(14i32);
pub const XCN_PROV_EC_ECNRA_SIG: X509ProviderType = X509ProviderType(15i32);
pub const XCN_PROV_EC_ECDSA_FULL: X509ProviderType = X509ProviderType(16i32);
pub const XCN_PROV_EC_ECNRA_FULL: X509ProviderType = X509ProviderType(17i32);
pub const XCN_PROV_DH_SCHANNEL: X509ProviderType = X509ProviderType(18i32);
pub const XCN_PROV_SPYRUS_LYNKS: X509ProviderType = X509ProviderType(20i32);
pub const XCN_PROV_RNG: X509ProviderType = X509ProviderType(21i32);
pub const XCN_PROV_INTEL_SEC: X509ProviderType = X509ProviderType(22i32);
pub const XCN_PROV_REPLACE_OWF: X509ProviderType = X509ProviderType(23i32);
pub const XCN_PROV_RSA_AES: X509ProviderType = X509ProviderType(24i32);
impl ::core::marker::Copy for X509ProviderType {}
impl ::core::clone::Clone for X509ProviderType {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct X509RequestInheritOptions(pub i32);
pub const InheritDefault: X509RequestInheritOptions = X509RequestInheritOptions(0i32);
pub const InheritNewDefaultKey: X509RequestInheritOptions = X509RequestInheritOptions(1i32);
pub const InheritNewSimilarKey: X509RequestInheritOptions = X509RequestInheritOptions(2i32);
pub const InheritPrivateKey: X509RequestInheritOptions = X509RequestInheritOptions(3i32);
pub const InheritPublicKey: X509RequestInheritOptions = X509RequestInheritOptions(4i32);
pub const InheritKeyMask: X509RequestInheritOptions = X509RequestInheritOptions(15i32);
pub const InheritNone: X509RequestInheritOptions = X509RequestInheritOptions(16i32);
pub const InheritRenewalCertificateFlag: X509RequestInheritOptions = X509RequestInheritOptions(32i32);
pub const InheritTemplateFlag: X509RequestInheritOptions = X509RequestInheritOptions(64i32);
pub const InheritSubjectFlag: X509RequestInheritOptions = X509RequestInheritOptions(128i32);
pub const InheritExtensionsFlag: X509RequestInheritOptions = X509RequestInheritOptions(256i32);
pub const InheritSubjectAltNameFlag: X509RequestInheritOptions = X509RequestInheritOptions(512i32);
pub const InheritValidityPeriodFlag: X509RequestInheritOptions = X509RequestInheritOptions(1024i32);
pub const InheritReserved80000000: X509RequestInheritOptions = X509RequestInheritOptions(-2147483648i32);
impl ::core::marker::Copy for X509RequestInheritOptions {}
impl ::core::clone::Clone for X509RequestInheritOptions {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct X509RequestType(pub i32);
pub const TypeAny: X509RequestType = X509RequestType(0i32);
pub const TypePkcs10: X509RequestType = X509RequestType(1i32);
pub const TypePkcs7: X509RequestType = X509RequestType(2i32);
pub const TypeCmc: X509RequestType = X509RequestType(3i32);
pub const TypeCertificate: X509RequestType = X509RequestType(4i32);
impl ::core::marker::Copy for X509RequestType {}
impl ::core::clone::Clone for X509RequestType {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct X509SCEPDisposition(pub i32);
pub const SCEPDispositionUnknown: X509SCEPDisposition = X509SCEPDisposition(-1i32);
pub const SCEPDispositionSuccess: X509SCEPDisposition = X509SCEPDisposition(0i32);
pub const SCEPDispositionFailure: X509SCEPDisposition = X509SCEPDisposition(2i32);
pub const SCEPDispositionPending: X509SCEPDisposition = X509SCEPDisposition(3i32);
pub const SCEPDispositionPendingChallenge: X509SCEPDisposition = X509SCEPDisposition(11i32);
impl ::core::marker::Copy for X509SCEPDisposition {}
impl ::core::clone::Clone for X509SCEPDisposition {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct X509SCEPFailInfo(pub i32);
pub const SCEPFailUnknown: X509SCEPFailInfo = X509SCEPFailInfo(-1i32);
pub const SCEPFailBadAlgorithm: X509SCEPFailInfo = X509SCEPFailInfo(0i32);
pub const SCEPFailBadMessageCheck: X509SCEPFailInfo = X509SCEPFailInfo(1i32);
pub const SCEPFailBadRequest: X509SCEPFailInfo = X509SCEPFailInfo(2i32);
pub const SCEPFailBadTime: X509SCEPFailInfo = X509SCEPFailInfo(3i32);
pub const SCEPFailBadCertId: X509SCEPFailInfo = X509SCEPFailInfo(4i32);
impl ::core::marker::Copy for X509SCEPFailInfo {}
impl ::core::clone::Clone for X509SCEPFailInfo {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct X509SCEPMessageType(pub i32);
pub const SCEPMessageUnknown: X509SCEPMessageType = X509SCEPMessageType(-1i32);
pub const SCEPMessageCertResponse: X509SCEPMessageType = X509SCEPMessageType(3i32);
pub const SCEPMessagePKCSRequest: X509SCEPMessageType = X509SCEPMessageType(19i32);
pub const SCEPMessageGetCertInitial: X509SCEPMessageType = X509SCEPMessageType(20i32);
pub const SCEPMessageGetCert: X509SCEPMessageType = X509SCEPMessageType(21i32);
pub const SCEPMessageGetCRL: X509SCEPMessageType = X509SCEPMessageType(22i32);
pub const SCEPMessageClaimChallengeAnswer: X509SCEPMessageType = X509SCEPMessageType(41i32);
impl ::core::marker::Copy for X509SCEPMessageType {}
impl ::core::clone::Clone for X509SCEPMessageType {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct X509SCEPProcessMessageFlags(pub i32);
pub const SCEPProcessDefault: X509SCEPProcessMessageFlags = X509SCEPProcessMessageFlags(0i32);
pub const SCEPProcessSkipCertInstall: X509SCEPProcessMessageFlags = X509SCEPProcessMessageFlags(1i32);
impl ::core::marker::Copy for X509SCEPProcessMessageFlags {}
impl ::core::clone::Clone for X509SCEPProcessMessageFlags {
    fn clone(&self) -> Self {
        *self
    }
}
pub const XECI_AUTOENROLL: u32 = 2u32;
pub const XECI_CERTREQ: u32 = 4u32;
pub const XECI_DISABLE: u32 = 0u32;
pub const XECI_REQWIZARD: u32 = 3u32;
pub const XECI_XENROLL: u32 = 1u32;
pub const XECP_STRING_PROPERTY: u32 = 1u32;
#[repr(transparent)]
pub struct XEKL_KEYSIZE(pub u32);
pub const XEKL_KEYSIZE_MIN: XEKL_KEYSIZE = XEKL_KEYSIZE(1u32);
pub const XEKL_KEYSIZE_MAX: XEKL_KEYSIZE = XEKL_KEYSIZE(2u32);
pub const XEKL_KEYSIZE_INC: XEKL_KEYSIZE = XEKL_KEYSIZE(3u32);
impl ::core::marker::Copy for XEKL_KEYSIZE {}
impl ::core::clone::Clone for XEKL_KEYSIZE {
    fn clone(&self) -> Self {
        *self
    }
}
pub const XEKL_KEYSIZE_DEFAULT: u32 = 4u32;
#[repr(transparent)]
pub struct XEKL_KEYSPEC(pub u32);
pub const XEKL_KEYSPEC_KEYX: XEKL_KEYSPEC = XEKL_KEYSPEC(1u32);
pub const XEKL_KEYSPEC_SIG: XEKL_KEYSPEC = XEKL_KEYSPEC(2u32);
impl ::core::marker::Copy for XEKL_KEYSPEC {}
impl ::core::clone::Clone for XEKL_KEYSPEC {
    fn clone(&self) -> Self {
        *self
    }
}
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
