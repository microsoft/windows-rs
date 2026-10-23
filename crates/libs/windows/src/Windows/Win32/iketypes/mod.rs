pub const IKEEXT_ANONYMOUS: IKEEXT_AUTHENTICATION_METHOD_TYPE = 3;
pub type IKEEXT_AUTHENTICATION_IMPERSONATION_TYPE = i32;
#[repr(C)]
#[cfg(all(feature = "fwptypes", feature = "shared"))]
#[derive(Clone, Copy)]
pub struct IKEEXT_AUTHENTICATION_METHOD0 {
    pub authenticationMethodType: IKEEXT_AUTHENTICATION_METHOD_TYPE,
    pub Anonymous: IKEEXT_AUTHENTICATION_METHOD0_0,
}
#[cfg(all(feature = "fwptypes", feature = "shared"))]
impl Default for IKEEXT_AUTHENTICATION_METHOD0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "fwptypes", feature = "shared"))]
#[derive(Clone, Copy)]
pub union IKEEXT_AUTHENTICATION_METHOD0_0 {
    pub presharedKeyAuthentication: IKEEXT_PRESHARED_KEY_AUTHENTICATION0,
    pub certificateAuthentication: IKEEXT_CERTIFICATE_AUTHENTICATION0,
    pub kerberosAuthentication: IKEEXT_KERBEROS_AUTHENTICATION0,
    pub ntlmV2Authentication: IKEEXT_NTLM_V2_AUTHENTICATION0,
    pub sslAuthentication: IKEEXT_CERTIFICATE_AUTHENTICATION0,
    pub cgaAuthentication: IKEEXT_IPV6_CGA_AUTHENTICATION0,
}
#[cfg(all(feature = "fwptypes", feature = "shared"))]
impl Default for IKEEXT_AUTHENTICATION_METHOD0_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "fwptypes", feature = "shared"))]
#[derive(Clone, Copy)]
pub struct IKEEXT_AUTHENTICATION_METHOD1 {
    pub authenticationMethodType: IKEEXT_AUTHENTICATION_METHOD_TYPE,
    pub Anonymous: IKEEXT_AUTHENTICATION_METHOD1_0,
}
#[cfg(all(feature = "fwptypes", feature = "shared"))]
impl Default for IKEEXT_AUTHENTICATION_METHOD1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "fwptypes", feature = "shared"))]
#[derive(Clone, Copy)]
pub union IKEEXT_AUTHENTICATION_METHOD1_0 {
    pub presharedKeyAuthentication: IKEEXT_PRESHARED_KEY_AUTHENTICATION1,
    pub certificateAuthentication: IKEEXT_CERTIFICATE_AUTHENTICATION1,
    pub kerberosAuthentication: IKEEXT_KERBEROS_AUTHENTICATION0,
    pub ntlmV2Authentication: IKEEXT_NTLM_V2_AUTHENTICATION0,
    pub sslAuthentication: IKEEXT_CERTIFICATE_AUTHENTICATION1,
    pub cgaAuthentication: IKEEXT_IPV6_CGA_AUTHENTICATION0,
    pub eapAuthentication: IKEEXT_EAP_AUTHENTICATION0,
}
#[cfg(all(feature = "fwptypes", feature = "shared"))]
impl Default for IKEEXT_AUTHENTICATION_METHOD1_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "fwptypes", feature = "shared"))]
#[derive(Clone, Copy)]
pub struct IKEEXT_AUTHENTICATION_METHOD2 {
    pub authenticationMethodType: IKEEXT_AUTHENTICATION_METHOD_TYPE,
    pub Anonymous: IKEEXT_AUTHENTICATION_METHOD2_0,
}
#[cfg(all(feature = "fwptypes", feature = "shared"))]
impl Default for IKEEXT_AUTHENTICATION_METHOD2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "fwptypes", feature = "shared"))]
#[derive(Clone, Copy)]
pub union IKEEXT_AUTHENTICATION_METHOD2_0 {
    pub presharedKeyAuthentication: IKEEXT_PRESHARED_KEY_AUTHENTICATION1,
    pub certificateAuthentication: IKEEXT_CERTIFICATE_AUTHENTICATION2,
    pub kerberosAuthentication: IKEEXT_KERBEROS_AUTHENTICATION1,
    pub reservedAuthentication: IKEEXT_RESERVED_AUTHENTICATION0,
    pub ntlmV2Authentication: IKEEXT_NTLM_V2_AUTHENTICATION0,
    pub sslAuthentication: IKEEXT_CERTIFICATE_AUTHENTICATION2,
    pub cgaAuthentication: IKEEXT_IPV6_CGA_AUTHENTICATION0,
    pub eapAuthentication: IKEEXT_EAP_AUTHENTICATION0,
}
#[cfg(all(feature = "fwptypes", feature = "shared"))]
impl Default for IKEEXT_AUTHENTICATION_METHOD2_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type IKEEXT_AUTHENTICATION_METHOD_TYPE = i32;
pub const IKEEXT_AUTHENTICATION_METHOD_TYPE_MAX: IKEEXT_AUTHENTICATION_METHOD_TYPE = 13;
pub const IKEEXT_CERTIFICATE: IKEEXT_AUTHENTICATION_METHOD_TYPE = 1;
#[repr(C)]
#[cfg(all(feature = "fwptypes", feature = "shared"))]
#[derive(Clone, Copy)]
pub struct IKEEXT_CERTIFICATE_AUTHENTICATION0 {
    pub inboundConfigType: IKEEXT_CERT_CONFIG_TYPE,
    pub Anonymous: IKEEXT_CERTIFICATE_AUTHENTICATION0_0,
    pub outboundConfigType: IKEEXT_CERT_CONFIG_TYPE,
    pub Anonymous2: IKEEXT_CERTIFICATE_AUTHENTICATION0_1,
    pub flags: super::UINT32,
}
#[cfg(all(feature = "fwptypes", feature = "shared"))]
impl Default for IKEEXT_CERTIFICATE_AUTHENTICATION0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "fwptypes", feature = "shared"))]
#[derive(Clone, Copy)]
pub union IKEEXT_CERTIFICATE_AUTHENTICATION0_0 {
    pub Anonymous: IKEEXT_CERTIFICATE_AUTHENTICATION0_0_0,
    pub inboundEnterpriseStoreConfig: *mut IKEEXT_CERT_ROOT_CONFIG0,
    pub inboundTrustedRootStoreConfig: *mut IKEEXT_CERT_ROOT_CONFIG0,
}
#[cfg(all(feature = "fwptypes", feature = "shared"))]
impl Default for IKEEXT_CERTIFICATE_AUTHENTICATION0_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "fwptypes", feature = "shared"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct IKEEXT_CERTIFICATE_AUTHENTICATION0_0_0 {
    pub inboundRootArraySize: super::UINT32,
    pub inboundRootArray: *mut IKEEXT_CERT_ROOT_CONFIG0,
}
#[repr(C)]
#[cfg(all(feature = "fwptypes", feature = "shared"))]
#[derive(Clone, Copy)]
pub union IKEEXT_CERTIFICATE_AUTHENTICATION0_1 {
    pub Anonymous: IKEEXT_CERTIFICATE_AUTHENTICATION0_1_0,
    pub outboundEnterpriseStoreConfig: *mut IKEEXT_CERT_ROOT_CONFIG0,
    pub outboundTrustedRootStoreConfig: *mut IKEEXT_CERT_ROOT_CONFIG0,
}
#[cfg(all(feature = "fwptypes", feature = "shared"))]
impl Default for IKEEXT_CERTIFICATE_AUTHENTICATION0_1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "fwptypes", feature = "shared"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct IKEEXT_CERTIFICATE_AUTHENTICATION0_1_0 {
    pub outboundRootArraySize: super::UINT32,
    pub outboundRootArray: *mut IKEEXT_CERT_ROOT_CONFIG0,
}
#[repr(C)]
#[cfg(all(feature = "fwptypes", feature = "shared"))]
#[derive(Clone, Copy)]
pub struct IKEEXT_CERTIFICATE_AUTHENTICATION1 {
    pub inboundConfigType: IKEEXT_CERT_CONFIG_TYPE,
    pub Anonymous: IKEEXT_CERTIFICATE_AUTHENTICATION1_0,
    pub outboundConfigType: IKEEXT_CERT_CONFIG_TYPE,
    pub Anonymous2: IKEEXT_CERTIFICATE_AUTHENTICATION1_1,
    pub flags: super::UINT32,
    pub localCertLocationUrl: super::FWP_BYTE_BLOB,
}
#[cfg(all(feature = "fwptypes", feature = "shared"))]
impl Default for IKEEXT_CERTIFICATE_AUTHENTICATION1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "fwptypes", feature = "shared"))]
#[derive(Clone, Copy)]
pub union IKEEXT_CERTIFICATE_AUTHENTICATION1_0 {
    pub Anonymous: IKEEXT_CERTIFICATE_AUTHENTICATION1_0_0,
    pub inboundEnterpriseStoreConfig: *mut IKEEXT_CERT_ROOT_CONFIG0,
    pub inboundTrustedRootStoreConfig: *mut IKEEXT_CERT_ROOT_CONFIG0,
}
#[cfg(all(feature = "fwptypes", feature = "shared"))]
impl Default for IKEEXT_CERTIFICATE_AUTHENTICATION1_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "fwptypes", feature = "shared"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct IKEEXT_CERTIFICATE_AUTHENTICATION1_0_0 {
    pub inboundRootArraySize: super::UINT32,
    pub inboundRootArray: *mut IKEEXT_CERT_ROOT_CONFIG0,
}
#[repr(C)]
#[cfg(all(feature = "fwptypes", feature = "shared"))]
#[derive(Clone, Copy)]
pub union IKEEXT_CERTIFICATE_AUTHENTICATION1_1 {
    pub Anonymous: IKEEXT_CERTIFICATE_AUTHENTICATION1_1_0,
    pub outboundEnterpriseStoreConfig: *mut IKEEXT_CERT_ROOT_CONFIG0,
    pub outboundTrustedRootStoreConfig: *mut IKEEXT_CERT_ROOT_CONFIG0,
}
#[cfg(all(feature = "fwptypes", feature = "shared"))]
impl Default for IKEEXT_CERTIFICATE_AUTHENTICATION1_1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "fwptypes", feature = "shared"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct IKEEXT_CERTIFICATE_AUTHENTICATION1_1_0 {
    pub outboundRootArraySize: super::UINT32,
    pub outboundRootArray: *mut IKEEXT_CERT_ROOT_CONFIG0,
}
#[repr(C)]
#[cfg(all(feature = "fwptypes", feature = "shared"))]
#[derive(Clone, Copy)]
pub struct IKEEXT_CERTIFICATE_AUTHENTICATION2 {
    pub inboundConfigType: IKEEXT_CERT_CONFIG_TYPE,
    pub Anonymous: IKEEXT_CERTIFICATE_AUTHENTICATION2_0,
    pub outboundConfigType: IKEEXT_CERT_CONFIG_TYPE,
    pub Anonymous2: IKEEXT_CERTIFICATE_AUTHENTICATION2_1,
    pub flags: super::UINT32,
    pub localCertLocationUrl: super::FWP_BYTE_BLOB,
}
#[cfg(all(feature = "fwptypes", feature = "shared"))]
impl Default for IKEEXT_CERTIFICATE_AUTHENTICATION2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "fwptypes", feature = "shared"))]
#[derive(Clone, Copy)]
pub union IKEEXT_CERTIFICATE_AUTHENTICATION2_0 {
    pub Anonymous: IKEEXT_CERTIFICATE_AUTHENTICATION2_0_0,
    pub Anonymous2: IKEEXT_CERTIFICATE_AUTHENTICATION2_0_1,
    pub Anonymous3: IKEEXT_CERTIFICATE_AUTHENTICATION2_0_2,
}
#[cfg(all(feature = "fwptypes", feature = "shared"))]
impl Default for IKEEXT_CERTIFICATE_AUTHENTICATION2_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "fwptypes", feature = "shared"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct IKEEXT_CERTIFICATE_AUTHENTICATION2_0_0 {
    pub inboundRootArraySize: super::UINT32,
    pub inboundRootCriteria: *mut IKEEXT_CERTIFICATE_CRITERIA0,
}
#[repr(C)]
#[cfg(all(feature = "fwptypes", feature = "shared"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct IKEEXT_CERTIFICATE_AUTHENTICATION2_0_1 {
    pub inboundEnterpriseStoreArraySize: super::UINT32,
    pub inboundEnterpriseStoreCriteria: *mut IKEEXT_CERTIFICATE_CRITERIA0,
}
#[repr(C)]
#[cfg(all(feature = "fwptypes", feature = "shared"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct IKEEXT_CERTIFICATE_AUTHENTICATION2_0_2 {
    pub inboundRootStoreArraySize: super::UINT32,
    pub inboundTrustedRootStoreCriteria: *mut IKEEXT_CERTIFICATE_CRITERIA0,
}
#[repr(C)]
#[cfg(all(feature = "fwptypes", feature = "shared"))]
#[derive(Clone, Copy)]
pub union IKEEXT_CERTIFICATE_AUTHENTICATION2_1 {
    pub Anonymous: IKEEXT_CERTIFICATE_AUTHENTICATION2_1_0,
    pub Anonymous2: IKEEXT_CERTIFICATE_AUTHENTICATION2_1_1,
    pub Anonymous3: IKEEXT_CERTIFICATE_AUTHENTICATION2_1_2,
}
#[cfg(all(feature = "fwptypes", feature = "shared"))]
impl Default for IKEEXT_CERTIFICATE_AUTHENTICATION2_1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "fwptypes", feature = "shared"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct IKEEXT_CERTIFICATE_AUTHENTICATION2_1_0 {
    pub outboundRootArraySize: super::UINT32,
    pub outboundRootCriteria: *mut IKEEXT_CERTIFICATE_CRITERIA0,
}
#[repr(C)]
#[cfg(all(feature = "fwptypes", feature = "shared"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct IKEEXT_CERTIFICATE_AUTHENTICATION2_1_1 {
    pub outboundEnterpriseStoreArraySize: super::UINT32,
    pub outboundEnterpriseStoreCriteria: *mut IKEEXT_CERTIFICATE_CRITERIA0,
}
#[repr(C)]
#[cfg(all(feature = "fwptypes", feature = "shared"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct IKEEXT_CERTIFICATE_AUTHENTICATION2_1_2 {
    pub outboundRootStoreArraySize: super::UINT32,
    pub outboundTrustedRootStoreCriteria: *mut IKEEXT_CERTIFICATE_CRITERIA0,
}
#[repr(C)]
#[cfg(all(feature = "fwptypes", feature = "shared"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct IKEEXT_CERTIFICATE_CREDENTIAL0 {
    pub subjectName: super::FWP_BYTE_BLOB,
    pub certHash: super::FWP_BYTE_BLOB,
    pub flags: super::UINT32,
}
#[repr(C)]
#[cfg(all(feature = "fwptypes", feature = "shared"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct IKEEXT_CERTIFICATE_CREDENTIAL1 {
    pub subjectName: super::FWP_BYTE_BLOB,
    pub certHash: super::FWP_BYTE_BLOB,
    pub flags: super::UINT32,
    pub certificate: super::FWP_BYTE_BLOB,
}
#[repr(C)]
#[cfg(all(feature = "fwptypes", feature = "shared"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct IKEEXT_CERTIFICATE_CRITERIA0 {
    pub certData: super::FWP_BYTE_BLOB,
    pub certHash: super::FWP_BYTE_BLOB,
    pub eku: *mut IKEEXT_CERT_EKUS0,
    pub name: *mut IKEEXT_CERT_NAME0,
    pub flags: super::UINT32,
}
pub const IKEEXT_CERTIFICATE_ECDSA_P256: IKEEXT_AUTHENTICATION_METHOD_TYPE = 7;
pub const IKEEXT_CERTIFICATE_ECDSA_P384: IKEEXT_AUTHENTICATION_METHOD_TYPE = 8;
pub const IKEEXT_CERT_AUTH_ALLOW_HTTP_CERT_LOOKUP: i32 = 16;
pub const IKEEXT_CERT_AUTH_DISABLE_SSL_CERT_VALIDATION: i32 = 8;
pub const IKEEXT_CERT_AUTH_ENABLE_CRL_CHECK_STRONG: i32 = 4;
pub const IKEEXT_CERT_AUTH_FLAG_DISABLE_CRL_CHECK: i32 = 2;
pub const IKEEXT_CERT_AUTH_FLAG_DISABLE_REQUEST_PAYLOAD: i32 = 64;
pub const IKEEXT_CERT_AUTH_FLAG_SSL_ONE_WAY: i32 = 1;
pub const IKEEXT_CERT_AUTH_URL_CONTAINS_BUNDLE: i32 = 32;
pub const IKEEXT_CERT_CONFIG_ENTERPRISE_STORE: IKEEXT_CERT_CONFIG_TYPE = 1;
pub const IKEEXT_CERT_CONFIG_EXPLICIT_TRUST_LIST: IKEEXT_CERT_CONFIG_TYPE = 0;
pub const IKEEXT_CERT_CONFIG_TRUSTED_ROOT_STORE: IKEEXT_CERT_CONFIG_TYPE = 2;
pub type IKEEXT_CERT_CONFIG_TYPE = i32;
pub const IKEEXT_CERT_CONFIG_TYPE_MAX: IKEEXT_CERT_CONFIG_TYPE = 4;
pub const IKEEXT_CERT_CONFIG_UNSPECIFIED: IKEEXT_CERT_CONFIG_TYPE = 3;
pub const IKEEXT_CERT_CREDENTIAL_FLAG_NAP_CERT: i32 = 1;
pub const IKEEXT_CERT_CRITERIA_CN: IKEEXT_CERT_CRITERIA_NAME_TYPE = 3;
pub const IKEEXT_CERT_CRITERIA_DC: IKEEXT_CERT_CRITERIA_NAME_TYPE = 6;
pub const IKEEXT_CERT_CRITERIA_DNS: IKEEXT_CERT_CRITERIA_NAME_TYPE = 0;
pub type IKEEXT_CERT_CRITERIA_NAME_TYPE = i32;
pub const IKEEXT_CERT_CRITERIA_NAME_TYPE_MAX: IKEEXT_CERT_CRITERIA_NAME_TYPE = 7;
pub const IKEEXT_CERT_CRITERIA_O: IKEEXT_CERT_CRITERIA_NAME_TYPE = 5;
pub const IKEEXT_CERT_CRITERIA_OU: IKEEXT_CERT_CRITERIA_NAME_TYPE = 4;
pub const IKEEXT_CERT_CRITERIA_RFC822: IKEEXT_CERT_CRITERIA_NAME_TYPE = 2;
pub const IKEEXT_CERT_CRITERIA_UPN: IKEEXT_CERT_CRITERIA_NAME_TYPE = 1;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct IKEEXT_CERT_EKUS0 {
    pub numEku: super::ULONG,
    pub eku: *mut windows_core::PSTR,
}
pub const IKEEXT_CERT_FLAG_DISABLE_REQUEST_PAYLOAD: i32 = 2;
pub const IKEEXT_CERT_FLAG_ENABLE_ACCOUNT_MAPPING: i32 = 1;
pub const IKEEXT_CERT_FLAG_FOLLOW_RENEWAL_CERTIFICATE: i32 = 256;
pub const IKEEXT_CERT_FLAG_IGNORE_INIT_CERT_MAP_FAILURE: i32 = 16;
pub const IKEEXT_CERT_FLAG_INTERMEDIATE_CA: i32 = 8;
pub const IKEEXT_CERT_FLAG_PREFER_NAP_CERTIFICATE_OUTBOUND: i32 = 32;
pub const IKEEXT_CERT_FLAG_SELECT_NAP_CERTIFICATE: i32 = 64;
pub const IKEEXT_CERT_FLAG_USE_NAP_CERTIFICATE: i32 = 4;
pub const IKEEXT_CERT_FLAG_VERIFY_NAP_CERTIFICATE: i32 = 128;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct IKEEXT_CERT_NAME0 {
    pub nameType: IKEEXT_CERT_CRITERIA_NAME_TYPE,
    pub certName: windows_core::PWSTR,
}
#[repr(C)]
#[cfg(all(feature = "fwptypes", feature = "shared"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct IKEEXT_CERT_ROOT_CONFIG0 {
    pub certData: super::FWP_BYTE_BLOB,
    pub flags: super::UINT32,
}
pub const IKEEXT_CIPHER_3DES: IKEEXT_CIPHER_TYPE = 1;
pub const IKEEXT_CIPHER_AES_128: IKEEXT_CIPHER_TYPE = 2;
pub const IKEEXT_CIPHER_AES_192: IKEEXT_CIPHER_TYPE = 3;
pub const IKEEXT_CIPHER_AES_256: IKEEXT_CIPHER_TYPE = 4;
pub const IKEEXT_CIPHER_AES_GCM_128_16ICV: IKEEXT_CIPHER_TYPE = 5;
pub const IKEEXT_CIPHER_AES_GCM_256_16ICV: IKEEXT_CIPHER_TYPE = 6;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct IKEEXT_CIPHER_ALGORITHM0 {
    pub algoIdentifier: IKEEXT_CIPHER_TYPE,
    pub keyLen: super::UINT32,
    pub rounds: super::UINT32,
}
pub const IKEEXT_CIPHER_DES: IKEEXT_CIPHER_TYPE = 0;
pub type IKEEXT_CIPHER_TYPE = i32;
pub const IKEEXT_CIPHER_TYPE_MAX: IKEEXT_CIPHER_TYPE = 7;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct IKEEXT_COMMON_STATISTICS0 {
    pub v4Statistics: IKEEXT_IP_VERSION_SPECIFIC_COMMON_STATISTICS0,
    pub v6Statistics: IKEEXT_IP_VERSION_SPECIFIC_COMMON_STATISTICS0,
    pub totalPacketsReceived: super::UINT32,
    pub totalInvalidPacketsReceived: super::UINT32,
    pub currentQueuedWorkitems: super::UINT32,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct IKEEXT_COMMON_STATISTICS1 {
    pub v4Statistics: IKEEXT_IP_VERSION_SPECIFIC_COMMON_STATISTICS1,
    pub v6Statistics: IKEEXT_IP_VERSION_SPECIFIC_COMMON_STATISTICS1,
    pub totalPacketsReceived: super::UINT32,
    pub totalInvalidPacketsReceived: super::UINT32,
    pub currentQueuedWorkitems: super::UINT32,
}
pub type IKEEXT_COOKIE = u64;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct IKEEXT_COOKIE_PAIR0 {
    pub initiator: IKEEXT_COOKIE,
    pub responder: IKEEXT_COOKIE,
}
#[repr(C)]
#[cfg(all(feature = "fwptypes", feature = "shared"))]
#[derive(Clone, Copy)]
pub struct IKEEXT_CREDENTIAL0 {
    pub authenticationMethodType: IKEEXT_AUTHENTICATION_METHOD_TYPE,
    pub impersonationType: IKEEXT_AUTHENTICATION_IMPERSONATION_TYPE,
    pub Anonymous: IKEEXT_CREDENTIAL0_0,
}
#[cfg(all(feature = "fwptypes", feature = "shared"))]
impl Default for IKEEXT_CREDENTIAL0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "fwptypes", feature = "shared"))]
#[derive(Clone, Copy)]
pub union IKEEXT_CREDENTIAL0_0 {
    pub presharedKey: *mut IKEEXT_PRESHARED_KEY_AUTHENTICATION0,
    pub certificate: *mut IKEEXT_CERTIFICATE_CREDENTIAL0,
    pub name: *mut IKEEXT_NAME_CREDENTIAL0,
}
#[cfg(all(feature = "fwptypes", feature = "shared"))]
impl Default for IKEEXT_CREDENTIAL0_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "fwptypes", feature = "shared"))]
#[derive(Clone, Copy)]
pub struct IKEEXT_CREDENTIAL1 {
    pub authenticationMethodType: IKEEXT_AUTHENTICATION_METHOD_TYPE,
    pub impersonationType: IKEEXT_AUTHENTICATION_IMPERSONATION_TYPE,
    pub Anonymous: IKEEXT_CREDENTIAL1_0,
}
#[cfg(all(feature = "fwptypes", feature = "shared"))]
impl Default for IKEEXT_CREDENTIAL1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "fwptypes", feature = "shared"))]
#[derive(Clone, Copy)]
pub union IKEEXT_CREDENTIAL1_0 {
    pub presharedKey: *mut IKEEXT_PRESHARED_KEY_AUTHENTICATION1,
    pub certificate: *mut IKEEXT_CERTIFICATE_CREDENTIAL1,
    pub name: *mut IKEEXT_NAME_CREDENTIAL0,
}
#[cfg(all(feature = "fwptypes", feature = "shared"))]
impl Default for IKEEXT_CREDENTIAL1_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "fwptypes", feature = "shared"))]
#[derive(Clone, Copy)]
pub struct IKEEXT_CREDENTIAL2 {
    pub authenticationMethodType: IKEEXT_AUTHENTICATION_METHOD_TYPE,
    pub impersonationType: IKEEXT_AUTHENTICATION_IMPERSONATION_TYPE,
    pub Anonymous: IKEEXT_CREDENTIAL2_0,
}
#[cfg(all(feature = "fwptypes", feature = "shared"))]
impl Default for IKEEXT_CREDENTIAL2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "fwptypes", feature = "shared"))]
#[derive(Clone, Copy)]
pub union IKEEXT_CREDENTIAL2_0 {
    pub presharedKey: *mut IKEEXT_PRESHARED_KEY_AUTHENTICATION1,
    pub certificate: *mut IKEEXT_CERTIFICATE_CREDENTIAL1,
    pub name: *mut IKEEXT_NAME_CREDENTIAL0,
}
#[cfg(all(feature = "fwptypes", feature = "shared"))]
impl Default for IKEEXT_CREDENTIAL2_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "fwptypes", feature = "shared"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct IKEEXT_CREDENTIALS0 {
    pub numCredentials: super::UINT32,
    pub credentials: *mut IKEEXT_CREDENTIAL_PAIR0,
}
#[repr(C)]
#[cfg(all(feature = "fwptypes", feature = "shared"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct IKEEXT_CREDENTIALS1 {
    pub numCredentials: super::UINT32,
    pub credentials: *mut IKEEXT_CREDENTIAL_PAIR1,
}
#[repr(C)]
#[cfg(all(feature = "fwptypes", feature = "shared"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct IKEEXT_CREDENTIALS2 {
    pub numCredentials: super::UINT32,
    pub credentials: *mut IKEEXT_CREDENTIAL_PAIR2,
}
#[repr(C)]
#[cfg(all(feature = "fwptypes", feature = "shared"))]
#[derive(Clone, Copy)]
pub struct IKEEXT_CREDENTIAL_PAIR0 {
    pub localCredentials: IKEEXT_CREDENTIAL0,
    pub peerCredentials: IKEEXT_CREDENTIAL0,
}
#[cfg(all(feature = "fwptypes", feature = "shared"))]
impl Default for IKEEXT_CREDENTIAL_PAIR0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "fwptypes", feature = "shared"))]
#[derive(Clone, Copy)]
pub struct IKEEXT_CREDENTIAL_PAIR1 {
    pub localCredentials: IKEEXT_CREDENTIAL1,
    pub peerCredentials: IKEEXT_CREDENTIAL1,
}
#[cfg(all(feature = "fwptypes", feature = "shared"))]
impl Default for IKEEXT_CREDENTIAL_PAIR1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "fwptypes", feature = "shared"))]
#[derive(Clone, Copy)]
pub struct IKEEXT_CREDENTIAL_PAIR2 {
    pub localCredentials: IKEEXT_CREDENTIAL2,
    pub peerCredentials: IKEEXT_CREDENTIAL2,
}
#[cfg(all(feature = "fwptypes", feature = "shared"))]
impl Default for IKEEXT_CREDENTIAL_PAIR2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const IKEEXT_DH_ECP_256: IKEEXT_DH_GROUP = 4;
pub const IKEEXT_DH_ECP_384: IKEEXT_DH_GROUP = 5;
pub type IKEEXT_DH_GROUP = i32;
pub const IKEEXT_DH_GROUP_1: IKEEXT_DH_GROUP = 1;
pub const IKEEXT_DH_GROUP_14: IKEEXT_DH_GROUP = 3;
pub const IKEEXT_DH_GROUP_2: IKEEXT_DH_GROUP = 2;
pub const IKEEXT_DH_GROUP_2048: IKEEXT_DH_GROUP = 3;
pub const IKEEXT_DH_GROUP_24: IKEEXT_DH_GROUP = 6;
pub const IKEEXT_DH_GROUP_MAX: IKEEXT_DH_GROUP = 7;
pub const IKEEXT_DH_GROUP_NONE: IKEEXT_DH_GROUP = 0;
pub const IKEEXT_EAP: IKEEXT_AUTHENTICATION_METHOD_TYPE = 11;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct IKEEXT_EAP_AUTHENTICATION0 {
    pub flags: super::UINT32,
}
pub const IKEEXT_EAP_FLAG_LOCAL_AUTH_ONLY: i32 = 1;
pub const IKEEXT_EAP_FLAG_REMOTE_AUTH_ONLY: i32 = 2;
#[repr(C)]
#[cfg(all(feature = "fwptypes", feature = "shared"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct IKEEXT_EM_POLICY0 {
    pub numAuthenticationMethods: super::UINT32,
    pub authenticationMethods: *mut IKEEXT_AUTHENTICATION_METHOD0,
    pub initiatorImpersonationType: IKEEXT_AUTHENTICATION_IMPERSONATION_TYPE,
}
#[repr(C)]
#[cfg(all(feature = "fwptypes", feature = "shared"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct IKEEXT_EM_POLICY1 {
    pub numAuthenticationMethods: super::UINT32,
    pub authenticationMethods: *mut IKEEXT_AUTHENTICATION_METHOD1,
    pub initiatorImpersonationType: IKEEXT_AUTHENTICATION_IMPERSONATION_TYPE,
}
#[repr(C)]
#[cfg(all(feature = "fwptypes", feature = "shared"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct IKEEXT_EM_POLICY2 {
    pub numAuthenticationMethods: super::UINT32,
    pub authenticationMethods: *mut IKEEXT_AUTHENTICATION_METHOD2,
    pub initiatorImpersonationType: IKEEXT_AUTHENTICATION_IMPERSONATION_TYPE,
}
pub type IKEEXT_EM_SA_STATE = i32;
pub const IKEEXT_EM_SA_STATE_AUTH_COMPLETE: IKEEXT_EM_SA_STATE = 3;
pub const IKEEXT_EM_SA_STATE_COMPLETE: IKEEXT_EM_SA_STATE = 5;
pub const IKEEXT_EM_SA_STATE_FINAL: IKEEXT_EM_SA_STATE = 4;
pub const IKEEXT_EM_SA_STATE_MAX: IKEEXT_EM_SA_STATE = 6;
pub const IKEEXT_EM_SA_STATE_NONE: IKEEXT_EM_SA_STATE = 0;
pub const IKEEXT_EM_SA_STATE_SENT_ATTS: IKEEXT_EM_SA_STATE = 1;
pub const IKEEXT_EM_SA_STATE_SSPI_SENT: IKEEXT_EM_SA_STATE = 2;
pub const IKEEXT_ERROR_CODE_COUNT: i32 = 97;
pub const IKEEXT_IMPERSONATION_MAX: IKEEXT_AUTHENTICATION_IMPERSONATION_TYPE = 2;
pub const IKEEXT_IMPERSONATION_NONE: IKEEXT_AUTHENTICATION_IMPERSONATION_TYPE = 0;
pub const IKEEXT_IMPERSONATION_SOCKET_PRINCIPAL: IKEEXT_AUTHENTICATION_IMPERSONATION_TYPE = 1;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct IKEEXT_INTEGRITY_ALGORITHM0 {
    pub algoIdentifier: IKEEXT_INTEGRITY_TYPE,
}
pub const IKEEXT_INTEGRITY_MD5: IKEEXT_INTEGRITY_TYPE = 0;
pub const IKEEXT_INTEGRITY_SHA1: IKEEXT_INTEGRITY_TYPE = 1;
pub const IKEEXT_INTEGRITY_SHA_256: IKEEXT_INTEGRITY_TYPE = 2;
pub const IKEEXT_INTEGRITY_SHA_384: IKEEXT_INTEGRITY_TYPE = 3;
pub type IKEEXT_INTEGRITY_TYPE = i32;
pub const IKEEXT_INTEGRITY_TYPE_MAX: IKEEXT_INTEGRITY_TYPE = 4;
pub const IKEEXT_IPV6_CGA: IKEEXT_AUTHENTICATION_METHOD_TYPE = 6;
#[repr(C)]
#[cfg(all(feature = "fwptypes", feature = "shared"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct IKEEXT_IPV6_CGA_AUTHENTICATION0 {
    pub keyContainerName: *mut u16,
    pub cspName: *mut u16,
    pub cspType: super::UINT32,
    pub cgaModifier: super::FWP_BYTE_ARRAY16,
    pub cgaCollisionCount: super::BYTE,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct IKEEXT_IP_VERSION_SPECIFIC_COMMON_STATISTICS0 {
    pub totalSocketReceiveFailures: super::UINT32,
    pub totalSocketSendFailures: super::UINT32,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct IKEEXT_IP_VERSION_SPECIFIC_COMMON_STATISTICS1 {
    pub totalSocketReceiveFailures: super::UINT32,
    pub totalSocketSendFailures: super::UINT32,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct IKEEXT_IP_VERSION_SPECIFIC_KEYMODULE_STATISTICS0 {
    pub currentActiveMainModes: super::UINT32,
    pub totalMainModesStarted: super::UINT32,
    pub totalSuccessfulMainModes: super::UINT32,
    pub totalFailedMainModes: super::UINT32,
    pub totalResponderMainModes: super::UINT32,
    pub currentNewResponderMainModes: super::UINT32,
    pub currentActiveQuickModes: super::UINT32,
    pub totalQuickModesStarted: super::UINT32,
    pub totalSuccessfulQuickModes: super::UINT32,
    pub totalFailedQuickModes: super::UINT32,
    pub totalAcquires: super::UINT32,
    pub totalReinitAcquires: super::UINT32,
    pub currentActiveExtendedModes: super::UINT32,
    pub totalExtendedModesStarted: super::UINT32,
    pub totalSuccessfulExtendedModes: super::UINT32,
    pub totalFailedExtendedModes: super::UINT32,
    pub totalImpersonationExtendedModes: super::UINT32,
    pub totalImpersonationMainModes: super::UINT32,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct IKEEXT_IP_VERSION_SPECIFIC_KEYMODULE_STATISTICS1 {
    pub currentActiveMainModes: super::UINT32,
    pub totalMainModesStarted: super::UINT32,
    pub totalSuccessfulMainModes: super::UINT32,
    pub totalFailedMainModes: super::UINT32,
    pub totalResponderMainModes: super::UINT32,
    pub currentNewResponderMainModes: super::UINT32,
    pub currentActiveQuickModes: super::UINT32,
    pub totalQuickModesStarted: super::UINT32,
    pub totalSuccessfulQuickModes: super::UINT32,
    pub totalFailedQuickModes: super::UINT32,
    pub totalAcquires: super::UINT32,
    pub totalReinitAcquires: super::UINT32,
    pub currentActiveExtendedModes: super::UINT32,
    pub totalExtendedModesStarted: super::UINT32,
    pub totalSuccessfulExtendedModes: super::UINT32,
    pub totalFailedExtendedModes: super::UINT32,
    pub totalImpersonationExtendedModes: super::UINT32,
    pub totalImpersonationMainModes: super::UINT32,
}
pub const IKEEXT_KERBEROS: IKEEXT_AUTHENTICATION_METHOD_TYPE = 2;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct IKEEXT_KERBEROS_AUTHENTICATION0 {
    pub flags: super::UINT32,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct IKEEXT_KERBEROS_AUTHENTICATION1 {
    pub flags: super::UINT32,
    pub proxyServer: *mut u16,
}
pub const IKEEXT_KERB_AUTH_DISABLE_INITIATOR_TOKEN_GENERATION: i32 = 1;
pub const IKEEXT_KERB_AUTH_DONT_ACCEPT_EXPLICIT_CREDENTIALS: i32 = 2;
pub const IKEEXT_KERB_AUTH_FORCE_PROXY_ON_INITIATOR: i32 = 4;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct IKEEXT_KEYMODULE_STATISTICS0 {
    pub v4Statistics: IKEEXT_IP_VERSION_SPECIFIC_KEYMODULE_STATISTICS0,
    pub v6Statistics: IKEEXT_IP_VERSION_SPECIFIC_KEYMODULE_STATISTICS0,
    pub errorFrequencyTable: [super::UINT32; 97],
    pub mainModeNegotiationTime: super::UINT32,
    pub quickModeNegotiationTime: super::UINT32,
    pub extendedModeNegotiationTime: super::UINT32,
}
#[cfg(feature = "shared")]
impl Default for IKEEXT_KEYMODULE_STATISTICS0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct IKEEXT_KEYMODULE_STATISTICS1 {
    pub v4Statistics: IKEEXT_IP_VERSION_SPECIFIC_KEYMODULE_STATISTICS1,
    pub v6Statistics: IKEEXT_IP_VERSION_SPECIFIC_KEYMODULE_STATISTICS1,
    pub errorFrequencyTable: [super::UINT32; 97],
    pub mainModeNegotiationTime: super::UINT32,
    pub quickModeNegotiationTime: super::UINT32,
    pub extendedModeNegotiationTime: super::UINT32,
}
#[cfg(feature = "shared")]
impl Default for IKEEXT_KEYMODULE_STATISTICS1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const IKEEXT_KEY_MODULE_AUTHIP: IKEEXT_KEY_MODULE_TYPE = 1;
pub const IKEEXT_KEY_MODULE_IKE: IKEEXT_KEY_MODULE_TYPE = 0;
pub const IKEEXT_KEY_MODULE_IKEV2: IKEEXT_KEY_MODULE_TYPE = 2;
pub const IKEEXT_KEY_MODULE_MAX: IKEEXT_KEY_MODULE_TYPE = 3;
pub type IKEEXT_KEY_MODULE_TYPE = i32;
pub type IKEEXT_MM_SA_STATE = i32;
pub const IKEEXT_MM_SA_STATE_COMPLETE: IKEEXT_MM_SA_STATE = 5;
pub const IKEEXT_MM_SA_STATE_FINAL: IKEEXT_MM_SA_STATE = 3;
pub const IKEEXT_MM_SA_STATE_FINAL_SENT: IKEEXT_MM_SA_STATE = 4;
pub const IKEEXT_MM_SA_STATE_MAX: IKEEXT_MM_SA_STATE = 6;
pub const IKEEXT_MM_SA_STATE_NONE: IKEEXT_MM_SA_STATE = 0;
pub const IKEEXT_MM_SA_STATE_SA_SENT: IKEEXT_MM_SA_STATE = 1;
pub const IKEEXT_MM_SA_STATE_SSPI_SENT: IKEEXT_MM_SA_STATE = 2;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct IKEEXT_NAME_CREDENTIAL0 {
    pub principalName: *mut u16,
}
pub const IKEEXT_NTLM_V2: IKEEXT_AUTHENTICATION_METHOD_TYPE = 5;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct IKEEXT_NTLM_V2_AUTHENTICATION0 {
    pub flags: super::UINT32,
}
pub const IKEEXT_NTLM_V2_AUTH_DONT_ACCEPT_EXPLICIT_CREDENTIALS: i32 = 1;
#[repr(C)]
#[cfg(all(feature = "fwptypes", feature = "shared"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct IKEEXT_POLICY0 {
    pub softExpirationTime: super::UINT32,
    pub numAuthenticationMethods: super::UINT32,
    pub authenticationMethods: *mut IKEEXT_AUTHENTICATION_METHOD0,
    pub initiatorImpersonationType: IKEEXT_AUTHENTICATION_IMPERSONATION_TYPE,
    pub numIkeProposals: super::UINT32,
    pub ikeProposals: *mut IKEEXT_PROPOSAL0,
    pub flags: super::UINT32,
    pub maxDynamicFilters: super::UINT32,
}
#[repr(C)]
#[cfg(all(feature = "fwptypes", feature = "shared"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct IKEEXT_POLICY1 {
    pub softExpirationTime: super::UINT32,
    pub numAuthenticationMethods: super::UINT32,
    pub authenticationMethods: *mut IKEEXT_AUTHENTICATION_METHOD1,
    pub initiatorImpersonationType: IKEEXT_AUTHENTICATION_IMPERSONATION_TYPE,
    pub numIkeProposals: super::UINT32,
    pub ikeProposals: *mut IKEEXT_PROPOSAL0,
    pub flags: super::UINT32,
    pub maxDynamicFilters: super::UINT32,
    pub retransmitDurationSecs: super::UINT32,
}
#[repr(C)]
#[cfg(all(feature = "fwptypes", feature = "shared"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct IKEEXT_POLICY2 {
    pub softExpirationTime: super::UINT32,
    pub numAuthenticationMethods: super::UINT32,
    pub authenticationMethods: *mut IKEEXT_AUTHENTICATION_METHOD2,
    pub initiatorImpersonationType: IKEEXT_AUTHENTICATION_IMPERSONATION_TYPE,
    pub numIkeProposals: super::UINT32,
    pub ikeProposals: *mut IKEEXT_PROPOSAL0,
    pub flags: super::UINT32,
    pub maxDynamicFilters: super::UINT32,
    pub retransmitDurationSecs: super::UINT32,
}
pub const IKEEXT_POLICY_ENABLE_IKEV2_FRAGMENTATION: i32 = 128;
pub const IKEEXT_POLICY_FLAG_DISABLE_DIAGNOSTICS: i32 = 1;
pub const IKEEXT_POLICY_FLAG_ENABLE_OPTIONAL_DH: i32 = 8;
pub const IKEEXT_POLICY_FLAG_IMS_VPN: i32 = 64;
pub const IKEEXT_POLICY_FLAG_MOBIKE_NOT_SUPPORTED: i32 = 16;
pub const IKEEXT_POLICY_FLAG_NO_IMPERSONATION_LUID_VERIFY: i32 = 4;
pub const IKEEXT_POLICY_FLAG_NO_MACHINE_LUID_VERIFY: i32 = 2;
pub const IKEEXT_POLICY_FLAG_POINT_TO_SITE: i32 = 512;
pub const IKEEXT_POLICY_FLAG_SITE_TO_SITE: i32 = 32;
pub const IKEEXT_POLICY_SUPPORT_LOW_POWER_MODE: i32 = 256;
pub const IKEEXT_PRESHARED_KEY: IKEEXT_AUTHENTICATION_METHOD_TYPE = 0;
#[repr(C)]
#[cfg(all(feature = "fwptypes", feature = "shared"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct IKEEXT_PRESHARED_KEY_AUTHENTICATION0 {
    pub presharedKey: super::FWP_BYTE_BLOB,
}
#[repr(C)]
#[cfg(all(feature = "fwptypes", feature = "shared"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct IKEEXT_PRESHARED_KEY_AUTHENTICATION1 {
    pub presharedKey: super::FWP_BYTE_BLOB,
    pub flags: super::UINT32,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct IKEEXT_PROPOSAL0 {
    pub cipherAlgorithm: IKEEXT_CIPHER_ALGORITHM0,
    pub integrityAlgorithm: IKEEXT_INTEGRITY_ALGORITHM0,
    pub maxLifetimeSeconds: super::UINT32,
    pub dhGroup: IKEEXT_DH_GROUP,
    pub quickModeLimit: super::UINT32,
}
pub const IKEEXT_PSK_FLAG_LOCAL_AUTH_ONLY: i32 = 1;
pub const IKEEXT_PSK_FLAG_REMOTE_AUTH_ONLY: i32 = 2;
pub type IKEEXT_QM_SA_STATE = i32;
pub const IKEEXT_QM_SA_STATE_COMPLETE: IKEEXT_QM_SA_STATE = 3;
pub const IKEEXT_QM_SA_STATE_FINAL: IKEEXT_QM_SA_STATE = 2;
pub const IKEEXT_QM_SA_STATE_INITIAL: IKEEXT_QM_SA_STATE = 1;
pub const IKEEXT_QM_SA_STATE_MAX: IKEEXT_QM_SA_STATE = 4;
pub const IKEEXT_QM_SA_STATE_NONE: IKEEXT_QM_SA_STATE = 0;
pub const IKEEXT_RESERVED: IKEEXT_AUTHENTICATION_METHOD_TYPE = 12;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct IKEEXT_RESERVED_AUTHENTICATION0 {
    pub flags: super::UINT32,
}
pub const IKEEXT_RESERVED_AUTH_DISABLE_INITIATOR_TOKEN_GENERATION: i32 = 1;
#[repr(C)]
#[cfg(all(feature = "fwptypes", feature = "ipsectypes", feature = "shared"))]
#[derive(Clone, Copy)]
pub struct IKEEXT_SA_DETAILS0 {
    pub saId: super::UINT64,
    pub keyModuleType: IKEEXT_KEY_MODULE_TYPE,
    pub ipVersion: super::FWP_IP_VERSION,
    pub Anonymous: IKEEXT_SA_DETAILS0_0,
    pub ikeTraffic: IKEEXT_TRAFFIC0,
    pub ikeProposal: IKEEXT_PROPOSAL0,
    pub cookiePair: IKEEXT_COOKIE_PAIR0,
    pub ikeCredentials: IKEEXT_CREDENTIALS0,
    pub ikePolicyKey: windows_core::GUID,
    pub virtualIfTunnelId: super::UINT64,
}
#[cfg(all(feature = "fwptypes", feature = "ipsectypes", feature = "shared"))]
impl Default for IKEEXT_SA_DETAILS0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "fwptypes", feature = "ipsectypes", feature = "shared"))]
#[derive(Clone, Copy)]
pub union IKEEXT_SA_DETAILS0_0 {
    pub v4UdpEncapsulation: *mut super::IPSEC_V4_UDP_ENCAPSULATION0,
}
#[cfg(all(feature = "fwptypes", feature = "ipsectypes", feature = "shared"))]
impl Default for IKEEXT_SA_DETAILS0_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "fwptypes", feature = "ipsectypes", feature = "shared"))]
#[derive(Clone, Copy)]
pub struct IKEEXT_SA_DETAILS1 {
    pub saId: super::UINT64,
    pub keyModuleType: IKEEXT_KEY_MODULE_TYPE,
    pub ipVersion: super::FWP_IP_VERSION,
    pub Anonymous: IKEEXT_SA_DETAILS1_0,
    pub ikeTraffic: IKEEXT_TRAFFIC0,
    pub ikeProposal: IKEEXT_PROPOSAL0,
    pub cookiePair: IKEEXT_COOKIE_PAIR0,
    pub ikeCredentials: IKEEXT_CREDENTIALS1,
    pub ikePolicyKey: windows_core::GUID,
    pub virtualIfTunnelId: super::UINT64,
    pub correlationKey: super::FWP_BYTE_BLOB,
}
#[cfg(all(feature = "fwptypes", feature = "ipsectypes", feature = "shared"))]
impl Default for IKEEXT_SA_DETAILS1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "fwptypes", feature = "ipsectypes", feature = "shared"))]
#[derive(Clone, Copy)]
pub union IKEEXT_SA_DETAILS1_0 {
    pub v4UdpEncapsulation: *mut super::IPSEC_V4_UDP_ENCAPSULATION0,
}
#[cfg(all(feature = "fwptypes", feature = "ipsectypes", feature = "shared"))]
impl Default for IKEEXT_SA_DETAILS1_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "fwptypes", feature = "ipsectypes", feature = "shared"))]
#[derive(Clone, Copy)]
pub struct IKEEXT_SA_DETAILS2 {
    pub saId: super::UINT64,
    pub keyModuleType: IKEEXT_KEY_MODULE_TYPE,
    pub ipVersion: super::FWP_IP_VERSION,
    pub Anonymous: IKEEXT_SA_DETAILS2_0,
    pub ikeTraffic: IKEEXT_TRAFFIC0,
    pub ikeProposal: IKEEXT_PROPOSAL0,
    pub cookiePair: IKEEXT_COOKIE_PAIR0,
    pub ikeCredentials: IKEEXT_CREDENTIALS2,
    pub ikePolicyKey: windows_core::GUID,
    pub virtualIfTunnelId: super::UINT64,
    pub correlationKey: super::FWP_BYTE_BLOB,
}
#[cfg(all(feature = "fwptypes", feature = "ipsectypes", feature = "shared"))]
impl Default for IKEEXT_SA_DETAILS2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "fwptypes", feature = "ipsectypes", feature = "shared"))]
#[derive(Clone, Copy)]
pub union IKEEXT_SA_DETAILS2_0 {
    pub v4UdpEncapsulation: *mut super::IPSEC_V4_UDP_ENCAPSULATION0,
}
#[cfg(all(feature = "fwptypes", feature = "ipsectypes", feature = "shared"))]
impl Default for IKEEXT_SA_DETAILS2_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "fwptypes", feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct IKEEXT_SA_ENUM_TEMPLATE0 {
    pub localSubNet: super::FWP_CONDITION_VALUE0,
    pub remoteSubNet: super::FWP_CONDITION_VALUE0,
    pub localMainModeCertHash: super::FWP_BYTE_BLOB,
}
#[cfg(all(feature = "fwptypes", feature = "shared", feature = "winnt"))]
impl Default for IKEEXT_SA_ENUM_TEMPLATE0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type IKEEXT_SA_ROLE = i32;
pub const IKEEXT_SA_ROLE_INITIATOR: IKEEXT_SA_ROLE = 0;
pub const IKEEXT_SA_ROLE_MAX: IKEEXT_SA_ROLE = 2;
pub const IKEEXT_SA_ROLE_RESPONDER: IKEEXT_SA_ROLE = 1;
pub const IKEEXT_SSL: IKEEXT_AUTHENTICATION_METHOD_TYPE = 4;
pub const IKEEXT_SSL_ECDSA_P256: IKEEXT_AUTHENTICATION_METHOD_TYPE = 9;
pub const IKEEXT_SSL_ECDSA_P384: IKEEXT_AUTHENTICATION_METHOD_TYPE = 10;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct IKEEXT_STATISTICS0 {
    pub ikeStatistics: IKEEXT_KEYMODULE_STATISTICS0,
    pub authipStatistics: IKEEXT_KEYMODULE_STATISTICS0,
    pub commonStatistics: IKEEXT_COMMON_STATISTICS0,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct IKEEXT_STATISTICS1 {
    pub ikeStatistics: IKEEXT_KEYMODULE_STATISTICS1,
    pub authipStatistics: IKEEXT_KEYMODULE_STATISTICS1,
    pub ikeV2Statistics: IKEEXT_KEYMODULE_STATISTICS1,
    pub commonStatistics: IKEEXT_COMMON_STATISTICS1,
}
#[repr(C)]
#[cfg(all(feature = "fwptypes", feature = "shared"))]
#[derive(Clone, Copy)]
pub struct IKEEXT_TRAFFIC0 {
    pub ipVersion: super::FWP_IP_VERSION,
    pub Anonymous: IKEEXT_TRAFFIC0_0,
    pub Anonymous2: IKEEXT_TRAFFIC0_1,
    pub authIpFilterId: super::UINT64,
}
#[cfg(all(feature = "fwptypes", feature = "shared"))]
impl Default for IKEEXT_TRAFFIC0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "fwptypes", feature = "shared"))]
#[derive(Clone, Copy)]
pub union IKEEXT_TRAFFIC0_0 {
    pub localV4Address: super::UINT32,
    pub localV6Address: [super::UINT8; 16],
}
#[cfg(all(feature = "fwptypes", feature = "shared"))]
impl Default for IKEEXT_TRAFFIC0_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "fwptypes", feature = "shared"))]
#[derive(Clone, Copy)]
pub union IKEEXT_TRAFFIC0_1 {
    pub remoteV4Address: super::UINT32,
    pub remoteV6Address: [super::UINT8; 16],
}
#[cfg(all(feature = "fwptypes", feature = "shared"))]
impl Default for IKEEXT_TRAFFIC0_1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
