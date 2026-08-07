pub type PSecPkgCred_CipherStrengths = *mut SecPkgCred_CipherStrengths;
pub type PSecPkgCred_ClientCertPolicy = *mut SecPkgCred_ClientCertPolicy;
pub type PSecPkgCred_SessionTicketKey = *mut SecPkgCred_SessionTicketKey;
pub type PSecPkgCred_SessionTicketKeys = *mut SecPkgCred_SessionTicketKeys;
#[cfg(feature = "wincrypt")]
pub type PSecPkgCred_SupportedAlgs = *mut SecPkgCred_SupportedAlgs;
pub type PSecPkgCred_SupportedProtocols = *mut SecPkgCred_SupportedProtocols;
pub const SECPKG_ATTR_APP_DATA: i32 = 94;
pub const SECPKG_ATTR_CC_POLICY_RESULT: i32 = 97;
pub const SECPKG_ATTR_CERT_CHECK_RESULT: i32 = 113;
pub const SECPKG_ATTR_CERT_CHECK_RESULT_INPROC: i32 = 114;
pub const SECPKG_ATTR_CIPHER_INFO: i32 = 100;
pub const SECPKG_ATTR_CIPHER_STRENGTHS: i32 = 87;
pub const SECPKG_ATTR_CLIENT_CERT_POLICY: i32 = 96;
pub const SECPKG_ATTR_CONNECTION_INFO: i32 = 90;
pub const SECPKG_ATTR_CONNECTION_INFO_EX: i32 = 110;
pub const SECPKG_ATTR_EAP_KEY_BLOCK: i32 = 91;
pub const SECPKG_ATTR_EAP_PRF_INFO: i32 = 101;
pub const SECPKG_ATTR_EARLY_START: i32 = 105;
pub const SECPKG_ATTR_ISSUER_LIST: i32 = 80;
pub const SECPKG_ATTR_ISSUER_LIST_EX: i32 = 89;
pub const SECPKG_ATTR_KEYING_MATERIAL: i32 = 107;
pub const SECPKG_ATTR_KEYING_MATERIAL_INFO: i32 = 106;
pub const SECPKG_ATTR_KEYING_MATERIAL_INPROC: i32 = 112;
pub const SECPKG_ATTR_KEYING_MATERIAL_TOKEN_BINDING: i32 = 111;
pub const SECPKG_ATTR_LOCAL_CERT_CONTEXT: i32 = 84;
pub const SECPKG_ATTR_LOCAL_CERT_INFO: i32 = 99;
pub const SECPKG_ATTR_LOCAL_CRED: i32 = 82;
pub const SECPKG_ATTR_MAPPED_CRED_ATTR: i32 = 92;
pub const SECPKG_ATTR_REMOTE_CERTIFICATES: i32 = 95;
pub const SECPKG_ATTR_REMOTE_CERT_CHAIN: i32 = 103;
pub const SECPKG_ATTR_REMOTE_CERT_CONTEXT: i32 = 83;
pub const SECPKG_ATTR_REMOTE_CRED: i32 = 81;
pub const SECPKG_ATTR_ROOT_STORE: i32 = 85;
pub const SECPKG_ATTR_SERIALIZED_REMOTE_CERT_CONTEXT: i32 = 117;
pub const SECPKG_ATTR_SERIALIZED_REMOTE_CERT_CONTEXT_INPROC: i32 = 116;
pub const SECPKG_ATTR_SESSION_INFO: i32 = 93;
pub const SECPKG_ATTR_SESSION_TICKET_KEYS: i32 = 115;
pub const SECPKG_ATTR_SRTP_PARAMETERS: i32 = 108;
pub const SECPKG_ATTR_SUPPORTED_ALGS: i32 = 86;
pub const SECPKG_ATTR_SUPPORTED_PROTOCOLS: i32 = 88;
pub const SECPKG_ATTR_SUPPORTED_SIGNATURES: i32 = 102;
pub const SECPKG_ATTR_TOKEN_BINDING: i32 = 109;
pub const SECPKG_ATTR_UI_INFO: i32 = 104;
pub const SECPKG_ATTR_USE_NCRYPT: i32 = 98;
pub const SESSION_TICKET_INFO_V0: i32 = 0;
pub const SESSION_TICKET_INFO_VERSION: i32 = 0;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SecPkgCred_CipherStrengths {
    pub dwMinimumCipherStrength: u32,
    pub dwMaximumCipherStrength: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SecPkgCred_ClientCertPolicy {
    pub dwFlags: u32,
    pub guidPolicyId: windows_core::GUID,
    pub dwCertFlags: u32,
    pub dwUrlRetrievalTimeout: u32,
    pub fCheckRevocationFreshnessTime: windows_core::BOOL,
    pub dwRevocationFreshnessTime: u32,
    pub fOmitUsageCheck: windows_core::BOOL,
    pub pwszSslCtlStoreName: windows_core::PWSTR,
    pub pwszSslCtlIdentifier: windows_core::PWSTR,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SecPkgCred_SessionTicketKey {
    pub TicketInfoVersion: u32,
    pub KeyId: [u8; 16],
    pub KeyingMaterial: [u8; 64],
    pub KeyingMaterialSize: u8,
}
impl Default for SecPkgCred_SessionTicketKey {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SecPkgCred_SessionTicketKeys {
    pub cSessionTicketKeys: u32,
    pub pSessionTicketKeys: PSecPkgCred_SessionTicketKey,
}
#[repr(C)]
#[cfg(feature = "wincrypt")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SecPkgCred_SupportedAlgs {
    pub cSupportedAlgs: u32,
    pub palgSupportedAlgs: *mut super::ALG_ID,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SecPkgCred_SupportedProtocols {
    pub grbitProtocol: u32,
}
