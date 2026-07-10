pub const IKEEXT_ANONYMOUS: IKEEXT_AUTHENTICATION_METHOD_TYPE = 3;
pub type IKEEXT_AUTHENTICATION_IMPERSONATION_TYPE = i32;
#[repr(C)]
#[cfg(feature = "fwptypes")]
#[derive(Clone, Copy)]
pub struct IKEEXT_AUTHENTICATION_METHOD0 {
    pub authenticationMethodType: IKEEXT_AUTHENTICATION_METHOD_TYPE,
    pub Anonymous: IKEEXT_AUTHENTICATION_METHOD0_0,
}
#[cfg(feature = "fwptypes")]
impl Default for IKEEXT_AUTHENTICATION_METHOD0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "fwptypes")]
#[derive(Clone, Copy)]
pub union IKEEXT_AUTHENTICATION_METHOD0_0 {
    pub presharedKeyAuthentication: IKEEXT_PRESHARED_KEY_AUTHENTICATION0,
    pub certificateAuthentication: IKEEXT_CERTIFICATE_AUTHENTICATION0,
    pub kerberosAuthentication: IKEEXT_KERBEROS_AUTHENTICATION0,
    pub ntlmV2Authentication: IKEEXT_NTLM_V2_AUTHENTICATION0,
    pub sslAuthentication: IKEEXT_CERTIFICATE_AUTHENTICATION0,
    pub cgaAuthentication: IKEEXT_IPV6_CGA_AUTHENTICATION0,
}
#[cfg(feature = "fwptypes")]
impl Default for IKEEXT_AUTHENTICATION_METHOD0_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "fwptypes")]
#[derive(Clone, Copy)]
pub struct IKEEXT_AUTHENTICATION_METHOD1 {
    pub authenticationMethodType: IKEEXT_AUTHENTICATION_METHOD_TYPE,
    pub Anonymous: IKEEXT_AUTHENTICATION_METHOD1_0,
}
#[cfg(feature = "fwptypes")]
impl Default for IKEEXT_AUTHENTICATION_METHOD1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "fwptypes")]
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
#[cfg(feature = "fwptypes")]
impl Default for IKEEXT_AUTHENTICATION_METHOD1_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "fwptypes")]
#[derive(Clone, Copy)]
pub struct IKEEXT_AUTHENTICATION_METHOD2 {
    pub authenticationMethodType: IKEEXT_AUTHENTICATION_METHOD_TYPE,
    pub Anonymous: IKEEXT_AUTHENTICATION_METHOD2_0,
}
#[cfg(feature = "fwptypes")]
impl Default for IKEEXT_AUTHENTICATION_METHOD2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "fwptypes")]
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
#[cfg(feature = "fwptypes")]
impl Default for IKEEXT_AUTHENTICATION_METHOD2_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type IKEEXT_AUTHENTICATION_METHOD_TYPE = i32;
pub const IKEEXT_AUTHENTICATION_METHOD_TYPE_MAX: IKEEXT_AUTHENTICATION_METHOD_TYPE = 13;
pub const IKEEXT_CERTIFICATE: IKEEXT_AUTHENTICATION_METHOD_TYPE = 1;
#[repr(C)]
#[cfg(feature = "fwptypes")]
#[derive(Clone, Copy)]
pub struct IKEEXT_CERTIFICATE_AUTHENTICATION0 {
    pub inboundConfigType: IKEEXT_CERT_CONFIG_TYPE,
    pub Anonymous: IKEEXT_CERTIFICATE_AUTHENTICATION0_0,
    pub outboundConfigType: IKEEXT_CERT_CONFIG_TYPE,
    pub Anonymous2: IKEEXT_CERTIFICATE_AUTHENTICATION0_1,
    pub flags: u32,
}
#[cfg(feature = "fwptypes")]
impl Default for IKEEXT_CERTIFICATE_AUTHENTICATION0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "fwptypes")]
#[derive(Clone, Copy)]
pub union IKEEXT_CERTIFICATE_AUTHENTICATION0_0 {
    pub Anonymous: IKEEXT_CERTIFICATE_AUTHENTICATION0_0_0,
    pub inboundEnterpriseStoreConfig: *mut IKEEXT_CERT_ROOT_CONFIG0,
    pub inboundTrustedRootStoreConfig: *mut IKEEXT_CERT_ROOT_CONFIG0,
}
#[cfg(feature = "fwptypes")]
impl Default for IKEEXT_CERTIFICATE_AUTHENTICATION0_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "fwptypes")]
#[derive(Clone, Copy)]
pub struct IKEEXT_CERTIFICATE_AUTHENTICATION0_0_0 {
    pub inboundRootArraySize: u32,
    pub inboundRootArray: *mut IKEEXT_CERT_ROOT_CONFIG0,
}
#[cfg(feature = "fwptypes")]
impl Default for IKEEXT_CERTIFICATE_AUTHENTICATION0_0_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "fwptypes")]
#[derive(Clone, Copy)]
pub union IKEEXT_CERTIFICATE_AUTHENTICATION0_1 {
    pub Anonymous: IKEEXT_CERTIFICATE_AUTHENTICATION0_1_0,
    pub outboundEnterpriseStoreConfig: *mut IKEEXT_CERT_ROOT_CONFIG0,
    pub outboundTrustedRootStoreConfig: *mut IKEEXT_CERT_ROOT_CONFIG0,
}
#[cfg(feature = "fwptypes")]
impl Default for IKEEXT_CERTIFICATE_AUTHENTICATION0_1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "fwptypes")]
#[derive(Clone, Copy)]
pub struct IKEEXT_CERTIFICATE_AUTHENTICATION0_1_0 {
    pub outboundRootArraySize: u32,
    pub outboundRootArray: *mut IKEEXT_CERT_ROOT_CONFIG0,
}
#[cfg(feature = "fwptypes")]
impl Default for IKEEXT_CERTIFICATE_AUTHENTICATION0_1_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "fwptypes")]
#[derive(Clone, Copy)]
pub struct IKEEXT_CERTIFICATE_AUTHENTICATION1 {
    pub inboundConfigType: IKEEXT_CERT_CONFIG_TYPE,
    pub Anonymous: IKEEXT_CERTIFICATE_AUTHENTICATION1_0,
    pub outboundConfigType: IKEEXT_CERT_CONFIG_TYPE,
    pub Anonymous2: IKEEXT_CERTIFICATE_AUTHENTICATION1_1,
    pub flags: u32,
    pub localCertLocationUrl: super::fwptypes::FWP_BYTE_BLOB,
}
#[cfg(feature = "fwptypes")]
impl Default for IKEEXT_CERTIFICATE_AUTHENTICATION1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "fwptypes")]
#[derive(Clone, Copy)]
pub union IKEEXT_CERTIFICATE_AUTHENTICATION1_0 {
    pub Anonymous: IKEEXT_CERTIFICATE_AUTHENTICATION1_0_0,
    pub inboundEnterpriseStoreConfig: *mut IKEEXT_CERT_ROOT_CONFIG0,
    pub inboundTrustedRootStoreConfig: *mut IKEEXT_CERT_ROOT_CONFIG0,
}
#[cfg(feature = "fwptypes")]
impl Default for IKEEXT_CERTIFICATE_AUTHENTICATION1_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "fwptypes")]
#[derive(Clone, Copy)]
pub struct IKEEXT_CERTIFICATE_AUTHENTICATION1_0_0 {
    pub inboundRootArraySize: u32,
    pub inboundRootArray: *mut IKEEXT_CERT_ROOT_CONFIG0,
}
#[cfg(feature = "fwptypes")]
impl Default for IKEEXT_CERTIFICATE_AUTHENTICATION1_0_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "fwptypes")]
#[derive(Clone, Copy)]
pub union IKEEXT_CERTIFICATE_AUTHENTICATION1_1 {
    pub Anonymous: IKEEXT_CERTIFICATE_AUTHENTICATION1_1_0,
    pub outboundEnterpriseStoreConfig: *mut IKEEXT_CERT_ROOT_CONFIG0,
    pub outboundTrustedRootStoreConfig: *mut IKEEXT_CERT_ROOT_CONFIG0,
}
#[cfg(feature = "fwptypes")]
impl Default for IKEEXT_CERTIFICATE_AUTHENTICATION1_1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "fwptypes")]
#[derive(Clone, Copy)]
pub struct IKEEXT_CERTIFICATE_AUTHENTICATION1_1_0 {
    pub outboundRootArraySize: u32,
    pub outboundRootArray: *mut IKEEXT_CERT_ROOT_CONFIG0,
}
#[cfg(feature = "fwptypes")]
impl Default for IKEEXT_CERTIFICATE_AUTHENTICATION1_1_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "fwptypes")]
#[derive(Clone, Copy)]
pub struct IKEEXT_CERTIFICATE_AUTHENTICATION2 {
    pub inboundConfigType: IKEEXT_CERT_CONFIG_TYPE,
    pub Anonymous: IKEEXT_CERTIFICATE_AUTHENTICATION2_0,
    pub outboundConfigType: IKEEXT_CERT_CONFIG_TYPE,
    pub Anonymous2: IKEEXT_CERTIFICATE_AUTHENTICATION2_1,
    pub flags: u32,
    pub localCertLocationUrl: super::fwptypes::FWP_BYTE_BLOB,
}
#[cfg(feature = "fwptypes")]
impl Default for IKEEXT_CERTIFICATE_AUTHENTICATION2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "fwptypes")]
#[derive(Clone, Copy)]
pub union IKEEXT_CERTIFICATE_AUTHENTICATION2_0 {
    pub Anonymous: IKEEXT_CERTIFICATE_AUTHENTICATION2_0_0,
    pub Anonymous2: IKEEXT_CERTIFICATE_AUTHENTICATION2_0_1,
    pub Anonymous3: IKEEXT_CERTIFICATE_AUTHENTICATION2_0_2,
}
#[cfg(feature = "fwptypes")]
impl Default for IKEEXT_CERTIFICATE_AUTHENTICATION2_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "fwptypes")]
#[derive(Clone, Copy)]
pub struct IKEEXT_CERTIFICATE_AUTHENTICATION2_0_0 {
    pub inboundRootArraySize: u32,
    pub inboundRootCriteria: *mut IKEEXT_CERTIFICATE_CRITERIA0,
}
#[cfg(feature = "fwptypes")]
impl Default for IKEEXT_CERTIFICATE_AUTHENTICATION2_0_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "fwptypes")]
#[derive(Clone, Copy)]
pub struct IKEEXT_CERTIFICATE_AUTHENTICATION2_0_1 {
    pub inboundEnterpriseStoreArraySize: u32,
    pub inboundEnterpriseStoreCriteria: *mut IKEEXT_CERTIFICATE_CRITERIA0,
}
#[cfg(feature = "fwptypes")]
impl Default for IKEEXT_CERTIFICATE_AUTHENTICATION2_0_1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "fwptypes")]
#[derive(Clone, Copy)]
pub struct IKEEXT_CERTIFICATE_AUTHENTICATION2_0_2 {
    pub inboundRootStoreArraySize: u32,
    pub inboundTrustedRootStoreCriteria: *mut IKEEXT_CERTIFICATE_CRITERIA0,
}
#[cfg(feature = "fwptypes")]
impl Default for IKEEXT_CERTIFICATE_AUTHENTICATION2_0_2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "fwptypes")]
#[derive(Clone, Copy)]
pub union IKEEXT_CERTIFICATE_AUTHENTICATION2_1 {
    pub Anonymous: IKEEXT_CERTIFICATE_AUTHENTICATION2_1_0,
    pub Anonymous2: IKEEXT_CERTIFICATE_AUTHENTICATION2_1_1,
    pub Anonymous3: IKEEXT_CERTIFICATE_AUTHENTICATION2_1_2,
}
#[cfg(feature = "fwptypes")]
impl Default for IKEEXT_CERTIFICATE_AUTHENTICATION2_1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "fwptypes")]
#[derive(Clone, Copy)]
pub struct IKEEXT_CERTIFICATE_AUTHENTICATION2_1_0 {
    pub outboundRootArraySize: u32,
    pub outboundRootCriteria: *mut IKEEXT_CERTIFICATE_CRITERIA0,
}
#[cfg(feature = "fwptypes")]
impl Default for IKEEXT_CERTIFICATE_AUTHENTICATION2_1_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "fwptypes")]
#[derive(Clone, Copy)]
pub struct IKEEXT_CERTIFICATE_AUTHENTICATION2_1_1 {
    pub outboundEnterpriseStoreArraySize: u32,
    pub outboundEnterpriseStoreCriteria: *mut IKEEXT_CERTIFICATE_CRITERIA0,
}
#[cfg(feature = "fwptypes")]
impl Default for IKEEXT_CERTIFICATE_AUTHENTICATION2_1_1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "fwptypes")]
#[derive(Clone, Copy)]
pub struct IKEEXT_CERTIFICATE_AUTHENTICATION2_1_2 {
    pub outboundRootStoreArraySize: u32,
    pub outboundTrustedRootStoreCriteria: *mut IKEEXT_CERTIFICATE_CRITERIA0,
}
#[cfg(feature = "fwptypes")]
impl Default for IKEEXT_CERTIFICATE_AUTHENTICATION2_1_2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "fwptypes")]
#[derive(Clone, Copy, Default)]
pub struct IKEEXT_CERTIFICATE_CREDENTIAL0 {
    pub subjectName: super::fwptypes::FWP_BYTE_BLOB,
    pub certHash: super::fwptypes::FWP_BYTE_BLOB,
    pub flags: u32,
}
#[repr(C)]
#[cfg(feature = "fwptypes")]
#[derive(Clone, Copy, Default)]
pub struct IKEEXT_CERTIFICATE_CREDENTIAL1 {
    pub subjectName: super::fwptypes::FWP_BYTE_BLOB,
    pub certHash: super::fwptypes::FWP_BYTE_BLOB,
    pub flags: u32,
    pub certificate: super::fwptypes::FWP_BYTE_BLOB,
}
#[repr(C)]
#[cfg(feature = "fwptypes")]
#[derive(Clone, Copy)]
pub struct IKEEXT_CERTIFICATE_CRITERIA0 {
    pub certData: super::fwptypes::FWP_BYTE_BLOB,
    pub certHash: super::fwptypes::FWP_BYTE_BLOB,
    pub eku: *mut IKEEXT_CERT_EKUS0,
    pub name: *mut IKEEXT_CERT_NAME0,
    pub flags: u32,
}
#[cfg(feature = "fwptypes")]
impl Default for IKEEXT_CERTIFICATE_CRITERIA0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const IKEEXT_CERTIFICATE_ECDSA_P256: IKEEXT_AUTHENTICATION_METHOD_TYPE = 7;
pub const IKEEXT_CERTIFICATE_ECDSA_P384: IKEEXT_AUTHENTICATION_METHOD_TYPE = 8;
pub const IKEEXT_CERT_AUTH_ALLOW_HTTP_CERT_LOOKUP: u32 = 16;
pub const IKEEXT_CERT_AUTH_DISABLE_SSL_CERT_VALIDATION: u32 = 8;
pub const IKEEXT_CERT_AUTH_ENABLE_CRL_CHECK_STRONG: u32 = 4;
pub const IKEEXT_CERT_AUTH_FLAG_DISABLE_CRL_CHECK: u32 = 2;
pub const IKEEXT_CERT_AUTH_FLAG_DISABLE_REQUEST_PAYLOAD: u32 = 64;
pub const IKEEXT_CERT_AUTH_FLAG_SSL_ONE_WAY: u32 = 1;
pub const IKEEXT_CERT_AUTH_URL_CONTAINS_BUNDLE: u32 = 32;
pub const IKEEXT_CERT_CONFIG_ENTERPRISE_STORE: IKEEXT_CERT_CONFIG_TYPE = 1;
pub const IKEEXT_CERT_CONFIG_EXPLICIT_TRUST_LIST: IKEEXT_CERT_CONFIG_TYPE = 0;
pub const IKEEXT_CERT_CONFIG_TRUSTED_ROOT_STORE: IKEEXT_CERT_CONFIG_TYPE = 2;
pub type IKEEXT_CERT_CONFIG_TYPE = i32;
pub const IKEEXT_CERT_CONFIG_TYPE_MAX: IKEEXT_CERT_CONFIG_TYPE = 4;
pub const IKEEXT_CERT_CONFIG_UNSPECIFIED: IKEEXT_CERT_CONFIG_TYPE = 3;
pub const IKEEXT_CERT_CREDENTIAL_FLAG_NAP_CERT: u32 = 1;
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
#[derive(Clone, Copy)]
pub struct IKEEXT_CERT_EKUS0 {
    pub numEku: u32,
    pub eku: *mut windows_sys::core::PSTR,
}
impl Default for IKEEXT_CERT_EKUS0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const IKEEXT_CERT_FLAG_DISABLE_REQUEST_PAYLOAD: u32 = 2;
pub const IKEEXT_CERT_FLAG_ENABLE_ACCOUNT_MAPPING: u32 = 1;
pub const IKEEXT_CERT_FLAG_FOLLOW_RENEWAL_CERTIFICATE: u32 = 256;
pub const IKEEXT_CERT_FLAG_IGNORE_INIT_CERT_MAP_FAILURE: u32 = 16;
pub const IKEEXT_CERT_FLAG_INTERMEDIATE_CA: u32 = 8;
pub const IKEEXT_CERT_FLAG_PREFER_NAP_CERTIFICATE_OUTBOUND: u32 = 32;
pub const IKEEXT_CERT_FLAG_SELECT_NAP_CERTIFICATE: u32 = 64;
pub const IKEEXT_CERT_FLAG_USE_NAP_CERTIFICATE: u32 = 4;
pub const IKEEXT_CERT_FLAG_VERIFY_NAP_CERTIFICATE: u32 = 128;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct IKEEXT_CERT_NAME0 {
    pub nameType: IKEEXT_CERT_CRITERIA_NAME_TYPE,
    pub certName: windows_sys::core::PWSTR,
}
impl Default for IKEEXT_CERT_NAME0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "fwptypes")]
#[derive(Clone, Copy, Default)]
pub struct IKEEXT_CERT_ROOT_CONFIG0 {
    pub certData: super::fwptypes::FWP_BYTE_BLOB,
    pub flags: u32,
}
pub const IKEEXT_CIPHER_3DES: IKEEXT_CIPHER_TYPE = 1;
pub const IKEEXT_CIPHER_AES_128: IKEEXT_CIPHER_TYPE = 2;
pub const IKEEXT_CIPHER_AES_192: IKEEXT_CIPHER_TYPE = 3;
pub const IKEEXT_CIPHER_AES_256: IKEEXT_CIPHER_TYPE = 4;
pub const IKEEXT_CIPHER_AES_GCM_128_16ICV: IKEEXT_CIPHER_TYPE = 5;
pub const IKEEXT_CIPHER_AES_GCM_256_16ICV: IKEEXT_CIPHER_TYPE = 6;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct IKEEXT_CIPHER_ALGORITHM0 {
    pub algoIdentifier: IKEEXT_CIPHER_TYPE,
    pub keyLen: u32,
    pub rounds: u32,
}
pub const IKEEXT_CIPHER_DES: IKEEXT_CIPHER_TYPE = 0;
pub type IKEEXT_CIPHER_TYPE = i32;
pub const IKEEXT_CIPHER_TYPE_MAX: IKEEXT_CIPHER_TYPE = 7;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct IKEEXT_COMMON_STATISTICS0 {
    pub v4Statistics: IKEEXT_IP_VERSION_SPECIFIC_COMMON_STATISTICS0,
    pub v6Statistics: IKEEXT_IP_VERSION_SPECIFIC_COMMON_STATISTICS0,
    pub totalPacketsReceived: u32,
    pub totalInvalidPacketsReceived: u32,
    pub currentQueuedWorkitems: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct IKEEXT_COMMON_STATISTICS1 {
    pub v4Statistics: IKEEXT_IP_VERSION_SPECIFIC_COMMON_STATISTICS1,
    pub v6Statistics: IKEEXT_IP_VERSION_SPECIFIC_COMMON_STATISTICS1,
    pub totalPacketsReceived: u32,
    pub totalInvalidPacketsReceived: u32,
    pub currentQueuedWorkitems: u32,
}
pub type IKEEXT_COOKIE = u64;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct IKEEXT_COOKIE_PAIR0 {
    pub initiator: IKEEXT_COOKIE,
    pub responder: IKEEXT_COOKIE,
}
#[repr(C)]
#[cfg(feature = "fwptypes")]
#[derive(Clone, Copy)]
pub struct IKEEXT_CREDENTIAL0 {
    pub authenticationMethodType: IKEEXT_AUTHENTICATION_METHOD_TYPE,
    pub impersonationType: IKEEXT_AUTHENTICATION_IMPERSONATION_TYPE,
    pub Anonymous: IKEEXT_CREDENTIAL0_0,
}
#[cfg(feature = "fwptypes")]
impl Default for IKEEXT_CREDENTIAL0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "fwptypes")]
#[derive(Clone, Copy)]
pub union IKEEXT_CREDENTIAL0_0 {
    pub presharedKey: *mut IKEEXT_PRESHARED_KEY_AUTHENTICATION0,
    pub certificate: *mut IKEEXT_CERTIFICATE_CREDENTIAL0,
    pub name: *mut IKEEXT_NAME_CREDENTIAL0,
}
#[cfg(feature = "fwptypes")]
impl Default for IKEEXT_CREDENTIAL0_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "fwptypes")]
#[derive(Clone, Copy)]
pub struct IKEEXT_CREDENTIAL1 {
    pub authenticationMethodType: IKEEXT_AUTHENTICATION_METHOD_TYPE,
    pub impersonationType: IKEEXT_AUTHENTICATION_IMPERSONATION_TYPE,
    pub Anonymous: IKEEXT_CREDENTIAL1_0,
}
#[cfg(feature = "fwptypes")]
impl Default for IKEEXT_CREDENTIAL1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "fwptypes")]
#[derive(Clone, Copy)]
pub union IKEEXT_CREDENTIAL1_0 {
    pub presharedKey: *mut IKEEXT_PRESHARED_KEY_AUTHENTICATION1,
    pub certificate: *mut IKEEXT_CERTIFICATE_CREDENTIAL1,
    pub name: *mut IKEEXT_NAME_CREDENTIAL0,
}
#[cfg(feature = "fwptypes")]
impl Default for IKEEXT_CREDENTIAL1_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "fwptypes")]
#[derive(Clone, Copy)]
pub struct IKEEXT_CREDENTIAL2 {
    pub authenticationMethodType: IKEEXT_AUTHENTICATION_METHOD_TYPE,
    pub impersonationType: IKEEXT_AUTHENTICATION_IMPERSONATION_TYPE,
    pub Anonymous: IKEEXT_CREDENTIAL2_0,
}
#[cfg(feature = "fwptypes")]
impl Default for IKEEXT_CREDENTIAL2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "fwptypes")]
#[derive(Clone, Copy)]
pub union IKEEXT_CREDENTIAL2_0 {
    pub presharedKey: *mut IKEEXT_PRESHARED_KEY_AUTHENTICATION1,
    pub certificate: *mut IKEEXT_CERTIFICATE_CREDENTIAL1,
    pub name: *mut IKEEXT_NAME_CREDENTIAL0,
}
#[cfg(feature = "fwptypes")]
impl Default for IKEEXT_CREDENTIAL2_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "fwptypes")]
#[derive(Clone, Copy)]
pub struct IKEEXT_CREDENTIALS0 {
    pub numCredentials: u32,
    pub credentials: *mut IKEEXT_CREDENTIAL_PAIR0,
}
#[cfg(feature = "fwptypes")]
impl Default for IKEEXT_CREDENTIALS0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "fwptypes")]
#[derive(Clone, Copy)]
pub struct IKEEXT_CREDENTIALS1 {
    pub numCredentials: u32,
    pub credentials: *mut IKEEXT_CREDENTIAL_PAIR1,
}
#[cfg(feature = "fwptypes")]
impl Default for IKEEXT_CREDENTIALS1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "fwptypes")]
#[derive(Clone, Copy)]
pub struct IKEEXT_CREDENTIALS2 {
    pub numCredentials: u32,
    pub credentials: *mut IKEEXT_CREDENTIAL_PAIR2,
}
#[cfg(feature = "fwptypes")]
impl Default for IKEEXT_CREDENTIALS2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "fwptypes")]
#[derive(Clone, Copy)]
pub struct IKEEXT_CREDENTIAL_PAIR0 {
    pub localCredentials: IKEEXT_CREDENTIAL0,
    pub peerCredentials: IKEEXT_CREDENTIAL0,
}
#[cfg(feature = "fwptypes")]
impl Default for IKEEXT_CREDENTIAL_PAIR0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "fwptypes")]
#[derive(Clone, Copy)]
pub struct IKEEXT_CREDENTIAL_PAIR1 {
    pub localCredentials: IKEEXT_CREDENTIAL1,
    pub peerCredentials: IKEEXT_CREDENTIAL1,
}
#[cfg(feature = "fwptypes")]
impl Default for IKEEXT_CREDENTIAL_PAIR1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "fwptypes")]
#[derive(Clone, Copy)]
pub struct IKEEXT_CREDENTIAL_PAIR2 {
    pub localCredentials: IKEEXT_CREDENTIAL2,
    pub peerCredentials: IKEEXT_CREDENTIAL2,
}
#[cfg(feature = "fwptypes")]
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
#[derive(Clone, Copy, Default)]
pub struct IKEEXT_EAP_AUTHENTICATION0 {
    pub flags: u32,
}
pub const IKEEXT_EAP_FLAG_LOCAL_AUTH_ONLY: u32 = 1;
pub const IKEEXT_EAP_FLAG_REMOTE_AUTH_ONLY: u32 = 2;
#[repr(C)]
#[cfg(feature = "fwptypes")]
#[derive(Clone, Copy)]
pub struct IKEEXT_EM_POLICY0 {
    pub numAuthenticationMethods: u32,
    pub authenticationMethods: *mut IKEEXT_AUTHENTICATION_METHOD0,
    pub initiatorImpersonationType: IKEEXT_AUTHENTICATION_IMPERSONATION_TYPE,
}
#[cfg(feature = "fwptypes")]
impl Default for IKEEXT_EM_POLICY0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "fwptypes")]
#[derive(Clone, Copy)]
pub struct IKEEXT_EM_POLICY1 {
    pub numAuthenticationMethods: u32,
    pub authenticationMethods: *mut IKEEXT_AUTHENTICATION_METHOD1,
    pub initiatorImpersonationType: IKEEXT_AUTHENTICATION_IMPERSONATION_TYPE,
}
#[cfg(feature = "fwptypes")]
impl Default for IKEEXT_EM_POLICY1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "fwptypes")]
#[derive(Clone, Copy)]
pub struct IKEEXT_EM_POLICY2 {
    pub numAuthenticationMethods: u32,
    pub authenticationMethods: *mut IKEEXT_AUTHENTICATION_METHOD2,
    pub initiatorImpersonationType: IKEEXT_AUTHENTICATION_IMPERSONATION_TYPE,
}
#[cfg(feature = "fwptypes")]
impl Default for IKEEXT_EM_POLICY2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type IKEEXT_EM_SA_STATE = i32;
pub const IKEEXT_EM_SA_STATE_AUTH_COMPLETE: IKEEXT_EM_SA_STATE = 3;
pub const IKEEXT_EM_SA_STATE_COMPLETE: IKEEXT_EM_SA_STATE = 5;
pub const IKEEXT_EM_SA_STATE_FINAL: IKEEXT_EM_SA_STATE = 4;
pub const IKEEXT_EM_SA_STATE_MAX: IKEEXT_EM_SA_STATE = 6;
pub const IKEEXT_EM_SA_STATE_NONE: IKEEXT_EM_SA_STATE = 0;
pub const IKEEXT_EM_SA_STATE_SENT_ATTS: IKEEXT_EM_SA_STATE = 1;
pub const IKEEXT_EM_SA_STATE_SSPI_SENT: IKEEXT_EM_SA_STATE = 2;
pub const IKEEXT_ERROR_CODE_COUNT: u32 = 97;
pub const IKEEXT_IMPERSONATION_MAX: IKEEXT_AUTHENTICATION_IMPERSONATION_TYPE = 2;
pub const IKEEXT_IMPERSONATION_NONE: IKEEXT_AUTHENTICATION_IMPERSONATION_TYPE = 0;
pub const IKEEXT_IMPERSONATION_SOCKET_PRINCIPAL: IKEEXT_AUTHENTICATION_IMPERSONATION_TYPE = 1;
#[repr(C)]
#[derive(Clone, Copy, Default)]
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
#[cfg(feature = "fwptypes")]
#[derive(Clone, Copy)]
pub struct IKEEXT_IPV6_CGA_AUTHENTICATION0 {
    pub keyContainerName: *mut u16,
    pub cspName: *mut u16,
    pub cspType: u32,
    pub cgaModifier: super::fwptypes::FWP_BYTE_ARRAY16,
    pub cgaCollisionCount: u8,
}
#[cfg(feature = "fwptypes")]
impl Default for IKEEXT_IPV6_CGA_AUTHENTICATION0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct IKEEXT_IP_VERSION_SPECIFIC_COMMON_STATISTICS0 {
    pub totalSocketReceiveFailures: u32,
    pub totalSocketSendFailures: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct IKEEXT_IP_VERSION_SPECIFIC_COMMON_STATISTICS1 {
    pub totalSocketReceiveFailures: u32,
    pub totalSocketSendFailures: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct IKEEXT_IP_VERSION_SPECIFIC_KEYMODULE_STATISTICS0 {
    pub currentActiveMainModes: u32,
    pub totalMainModesStarted: u32,
    pub totalSuccessfulMainModes: u32,
    pub totalFailedMainModes: u32,
    pub totalResponderMainModes: u32,
    pub currentNewResponderMainModes: u32,
    pub currentActiveQuickModes: u32,
    pub totalQuickModesStarted: u32,
    pub totalSuccessfulQuickModes: u32,
    pub totalFailedQuickModes: u32,
    pub totalAcquires: u32,
    pub totalReinitAcquires: u32,
    pub currentActiveExtendedModes: u32,
    pub totalExtendedModesStarted: u32,
    pub totalSuccessfulExtendedModes: u32,
    pub totalFailedExtendedModes: u32,
    pub totalImpersonationExtendedModes: u32,
    pub totalImpersonationMainModes: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct IKEEXT_IP_VERSION_SPECIFIC_KEYMODULE_STATISTICS1 {
    pub currentActiveMainModes: u32,
    pub totalMainModesStarted: u32,
    pub totalSuccessfulMainModes: u32,
    pub totalFailedMainModes: u32,
    pub totalResponderMainModes: u32,
    pub currentNewResponderMainModes: u32,
    pub currentActiveQuickModes: u32,
    pub totalQuickModesStarted: u32,
    pub totalSuccessfulQuickModes: u32,
    pub totalFailedQuickModes: u32,
    pub totalAcquires: u32,
    pub totalReinitAcquires: u32,
    pub currentActiveExtendedModes: u32,
    pub totalExtendedModesStarted: u32,
    pub totalSuccessfulExtendedModes: u32,
    pub totalFailedExtendedModes: u32,
    pub totalImpersonationExtendedModes: u32,
    pub totalImpersonationMainModes: u32,
}
pub const IKEEXT_KERBEROS: IKEEXT_AUTHENTICATION_METHOD_TYPE = 2;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct IKEEXT_KERBEROS_AUTHENTICATION0 {
    pub flags: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct IKEEXT_KERBEROS_AUTHENTICATION1 {
    pub flags: u32,
    pub proxyServer: *mut u16,
}
impl Default for IKEEXT_KERBEROS_AUTHENTICATION1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const IKEEXT_KERB_AUTH_DISABLE_INITIATOR_TOKEN_GENERATION: u32 = 1;
pub const IKEEXT_KERB_AUTH_DONT_ACCEPT_EXPLICIT_CREDENTIALS: u32 = 2;
pub const IKEEXT_KERB_AUTH_FORCE_PROXY_ON_INITIATOR: u32 = 4;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct IKEEXT_KEYMODULE_STATISTICS0 {
    pub v4Statistics: IKEEXT_IP_VERSION_SPECIFIC_KEYMODULE_STATISTICS0,
    pub v6Statistics: IKEEXT_IP_VERSION_SPECIFIC_KEYMODULE_STATISTICS0,
    pub errorFrequencyTable: [u32; 97],
    pub mainModeNegotiationTime: u32,
    pub quickModeNegotiationTime: u32,
    pub extendedModeNegotiationTime: u32,
}
impl Default for IKEEXT_KEYMODULE_STATISTICS0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct IKEEXT_KEYMODULE_STATISTICS1 {
    pub v4Statistics: IKEEXT_IP_VERSION_SPECIFIC_KEYMODULE_STATISTICS1,
    pub v6Statistics: IKEEXT_IP_VERSION_SPECIFIC_KEYMODULE_STATISTICS1,
    pub errorFrequencyTable: [u32; 97],
    pub mainModeNegotiationTime: u32,
    pub quickModeNegotiationTime: u32,
    pub extendedModeNegotiationTime: u32,
}
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
#[derive(Clone, Copy)]
pub struct IKEEXT_NAME_CREDENTIAL0 {
    pub principalName: *mut u16,
}
impl Default for IKEEXT_NAME_CREDENTIAL0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const IKEEXT_NTLM_V2: IKEEXT_AUTHENTICATION_METHOD_TYPE = 5;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct IKEEXT_NTLM_V2_AUTHENTICATION0 {
    pub flags: u32,
}
pub const IKEEXT_NTLM_V2_AUTH_DONT_ACCEPT_EXPLICIT_CREDENTIALS: u32 = 1;
#[repr(C)]
#[cfg(feature = "fwptypes")]
#[derive(Clone, Copy)]
pub struct IKEEXT_POLICY0 {
    pub softExpirationTime: u32,
    pub numAuthenticationMethods: u32,
    pub authenticationMethods: *mut IKEEXT_AUTHENTICATION_METHOD0,
    pub initiatorImpersonationType: IKEEXT_AUTHENTICATION_IMPERSONATION_TYPE,
    pub numIkeProposals: u32,
    pub ikeProposals: *mut IKEEXT_PROPOSAL0,
    pub flags: u32,
    pub maxDynamicFilters: u32,
}
#[cfg(feature = "fwptypes")]
impl Default for IKEEXT_POLICY0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "fwptypes")]
#[derive(Clone, Copy)]
pub struct IKEEXT_POLICY1 {
    pub softExpirationTime: u32,
    pub numAuthenticationMethods: u32,
    pub authenticationMethods: *mut IKEEXT_AUTHENTICATION_METHOD1,
    pub initiatorImpersonationType: IKEEXT_AUTHENTICATION_IMPERSONATION_TYPE,
    pub numIkeProposals: u32,
    pub ikeProposals: *mut IKEEXT_PROPOSAL0,
    pub flags: u32,
    pub maxDynamicFilters: u32,
    pub retransmitDurationSecs: u32,
}
#[cfg(feature = "fwptypes")]
impl Default for IKEEXT_POLICY1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "fwptypes")]
#[derive(Clone, Copy)]
pub struct IKEEXT_POLICY2 {
    pub softExpirationTime: u32,
    pub numAuthenticationMethods: u32,
    pub authenticationMethods: *mut IKEEXT_AUTHENTICATION_METHOD2,
    pub initiatorImpersonationType: IKEEXT_AUTHENTICATION_IMPERSONATION_TYPE,
    pub numIkeProposals: u32,
    pub ikeProposals: *mut IKEEXT_PROPOSAL0,
    pub flags: u32,
    pub maxDynamicFilters: u32,
    pub retransmitDurationSecs: u32,
}
#[cfg(feature = "fwptypes")]
impl Default for IKEEXT_POLICY2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const IKEEXT_POLICY_ENABLE_IKEV2_FRAGMENTATION: u32 = 128;
pub const IKEEXT_POLICY_FLAG_DISABLE_DIAGNOSTICS: u32 = 1;
pub const IKEEXT_POLICY_FLAG_ENABLE_OPTIONAL_DH: u32 = 8;
pub const IKEEXT_POLICY_FLAG_IMS_VPN: u32 = 64;
pub const IKEEXT_POLICY_FLAG_MOBIKE_NOT_SUPPORTED: u32 = 16;
pub const IKEEXT_POLICY_FLAG_NO_IMPERSONATION_LUID_VERIFY: u32 = 4;
pub const IKEEXT_POLICY_FLAG_NO_MACHINE_LUID_VERIFY: u32 = 2;
pub const IKEEXT_POLICY_FLAG_POINT_TO_SITE: u32 = 512;
pub const IKEEXT_POLICY_FLAG_SITE_TO_SITE: u32 = 32;
pub const IKEEXT_POLICY_SUPPORT_LOW_POWER_MODE: u32 = 256;
pub const IKEEXT_PRESHARED_KEY: IKEEXT_AUTHENTICATION_METHOD_TYPE = 0;
#[repr(C)]
#[cfg(feature = "fwptypes")]
#[derive(Clone, Copy, Default)]
pub struct IKEEXT_PRESHARED_KEY_AUTHENTICATION0 {
    pub presharedKey: super::fwptypes::FWP_BYTE_BLOB,
}
#[repr(C)]
#[cfg(feature = "fwptypes")]
#[derive(Clone, Copy, Default)]
pub struct IKEEXT_PRESHARED_KEY_AUTHENTICATION1 {
    pub presharedKey: super::fwptypes::FWP_BYTE_BLOB,
    pub flags: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct IKEEXT_PROPOSAL0 {
    pub cipherAlgorithm: IKEEXT_CIPHER_ALGORITHM0,
    pub integrityAlgorithm: IKEEXT_INTEGRITY_ALGORITHM0,
    pub maxLifetimeSeconds: u32,
    pub dhGroup: IKEEXT_DH_GROUP,
    pub quickModeLimit: u32,
}
pub const IKEEXT_PSK_FLAG_LOCAL_AUTH_ONLY: u32 = 1;
pub const IKEEXT_PSK_FLAG_REMOTE_AUTH_ONLY: u32 = 2;
pub type IKEEXT_QM_SA_STATE = i32;
pub const IKEEXT_QM_SA_STATE_COMPLETE: IKEEXT_QM_SA_STATE = 3;
pub const IKEEXT_QM_SA_STATE_FINAL: IKEEXT_QM_SA_STATE = 2;
pub const IKEEXT_QM_SA_STATE_INITIAL: IKEEXT_QM_SA_STATE = 1;
pub const IKEEXT_QM_SA_STATE_MAX: IKEEXT_QM_SA_STATE = 4;
pub const IKEEXT_QM_SA_STATE_NONE: IKEEXT_QM_SA_STATE = 0;
pub const IKEEXT_RESERVED: IKEEXT_AUTHENTICATION_METHOD_TYPE = 12;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct IKEEXT_RESERVED_AUTHENTICATION0 {
    pub flags: u32,
}
pub const IKEEXT_RESERVED_AUTH_DISABLE_INITIATOR_TOKEN_GENERATION: u32 = 1;
#[repr(C)]
#[cfg(all(feature = "fwptypes", feature = "ipsectypes"))]
#[derive(Clone, Copy)]
pub struct IKEEXT_SA_DETAILS0 {
    pub saId: u64,
    pub keyModuleType: IKEEXT_KEY_MODULE_TYPE,
    pub ipVersion: super::fwptypes::FWP_IP_VERSION,
    pub Anonymous: IKEEXT_SA_DETAILS0_0,
    pub ikeTraffic: IKEEXT_TRAFFIC0,
    pub ikeProposal: IKEEXT_PROPOSAL0,
    pub cookiePair: IKEEXT_COOKIE_PAIR0,
    pub ikeCredentials: IKEEXT_CREDENTIALS0,
    pub ikePolicyKey: windows_sys::core::GUID,
    pub virtualIfTunnelId: u64,
}
#[cfg(all(feature = "fwptypes", feature = "ipsectypes"))]
impl Default for IKEEXT_SA_DETAILS0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "fwptypes", feature = "ipsectypes"))]
#[derive(Clone, Copy)]
pub union IKEEXT_SA_DETAILS0_0 {
    pub v4UdpEncapsulation: *mut super::ipsectypes::IPSEC_V4_UDP_ENCAPSULATION0,
}
#[cfg(all(feature = "fwptypes", feature = "ipsectypes"))]
impl Default for IKEEXT_SA_DETAILS0_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "fwptypes", feature = "ipsectypes"))]
#[derive(Clone, Copy)]
pub struct IKEEXT_SA_DETAILS1 {
    pub saId: u64,
    pub keyModuleType: IKEEXT_KEY_MODULE_TYPE,
    pub ipVersion: super::fwptypes::FWP_IP_VERSION,
    pub Anonymous: IKEEXT_SA_DETAILS1_0,
    pub ikeTraffic: IKEEXT_TRAFFIC0,
    pub ikeProposal: IKEEXT_PROPOSAL0,
    pub cookiePair: IKEEXT_COOKIE_PAIR0,
    pub ikeCredentials: IKEEXT_CREDENTIALS1,
    pub ikePolicyKey: windows_sys::core::GUID,
    pub virtualIfTunnelId: u64,
    pub correlationKey: super::fwptypes::FWP_BYTE_BLOB,
}
#[cfg(all(feature = "fwptypes", feature = "ipsectypes"))]
impl Default for IKEEXT_SA_DETAILS1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "fwptypes", feature = "ipsectypes"))]
#[derive(Clone, Copy)]
pub union IKEEXT_SA_DETAILS1_0 {
    pub v4UdpEncapsulation: *mut super::ipsectypes::IPSEC_V4_UDP_ENCAPSULATION0,
}
#[cfg(all(feature = "fwptypes", feature = "ipsectypes"))]
impl Default for IKEEXT_SA_DETAILS1_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "fwptypes", feature = "ipsectypes"))]
#[derive(Clone, Copy)]
pub struct IKEEXT_SA_DETAILS2 {
    pub saId: u64,
    pub keyModuleType: IKEEXT_KEY_MODULE_TYPE,
    pub ipVersion: super::fwptypes::FWP_IP_VERSION,
    pub Anonymous: IKEEXT_SA_DETAILS2_0,
    pub ikeTraffic: IKEEXT_TRAFFIC0,
    pub ikeProposal: IKEEXT_PROPOSAL0,
    pub cookiePair: IKEEXT_COOKIE_PAIR0,
    pub ikeCredentials: IKEEXT_CREDENTIALS2,
    pub ikePolicyKey: windows_sys::core::GUID,
    pub virtualIfTunnelId: u64,
    pub correlationKey: super::fwptypes::FWP_BYTE_BLOB,
}
#[cfg(all(feature = "fwptypes", feature = "ipsectypes"))]
impl Default for IKEEXT_SA_DETAILS2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "fwptypes", feature = "ipsectypes"))]
#[derive(Clone, Copy)]
pub union IKEEXT_SA_DETAILS2_0 {
    pub v4UdpEncapsulation: *mut super::ipsectypes::IPSEC_V4_UDP_ENCAPSULATION0,
}
#[cfg(all(feature = "fwptypes", feature = "ipsectypes"))]
impl Default for IKEEXT_SA_DETAILS2_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "fwptypes", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct IKEEXT_SA_ENUM_TEMPLATE0 {
    pub localSubNet: super::fwptypes::FWP_CONDITION_VALUE0,
    pub remoteSubNet: super::fwptypes::FWP_CONDITION_VALUE0,
    pub localMainModeCertHash: super::fwptypes::FWP_BYTE_BLOB,
}
#[cfg(all(feature = "fwptypes", feature = "winnt"))]
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
#[derive(Clone, Copy, Default)]
pub struct IKEEXT_STATISTICS0 {
    pub ikeStatistics: IKEEXT_KEYMODULE_STATISTICS0,
    pub authipStatistics: IKEEXT_KEYMODULE_STATISTICS0,
    pub commonStatistics: IKEEXT_COMMON_STATISTICS0,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct IKEEXT_STATISTICS1 {
    pub ikeStatistics: IKEEXT_KEYMODULE_STATISTICS1,
    pub authipStatistics: IKEEXT_KEYMODULE_STATISTICS1,
    pub ikeV2Statistics: IKEEXT_KEYMODULE_STATISTICS1,
    pub commonStatistics: IKEEXT_COMMON_STATISTICS1,
}
#[repr(C)]
#[cfg(feature = "fwptypes")]
#[derive(Clone, Copy)]
pub struct IKEEXT_TRAFFIC0 {
    pub ipVersion: super::fwptypes::FWP_IP_VERSION,
    pub Anonymous: IKEEXT_TRAFFIC0_0,
    pub Anonymous2: IKEEXT_TRAFFIC0_1,
    pub authIpFilterId: u64,
}
#[cfg(feature = "fwptypes")]
impl Default for IKEEXT_TRAFFIC0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "fwptypes")]
#[derive(Clone, Copy)]
pub union IKEEXT_TRAFFIC0_0 {
    pub localV4Address: u32,
    pub localV6Address: [u8; 16],
}
#[cfg(feature = "fwptypes")]
impl Default for IKEEXT_TRAFFIC0_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "fwptypes")]
#[derive(Clone, Copy)]
pub union IKEEXT_TRAFFIC0_1 {
    pub remoteV4Address: u32,
    pub remoteV6Address: [u8; 16],
}
#[cfg(feature = "fwptypes")]
impl Default for IKEEXT_TRAFFIC0_1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
