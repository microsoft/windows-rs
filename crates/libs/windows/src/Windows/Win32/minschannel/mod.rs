#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PSecPkgCred_CipherStrengths(pub *mut SecPkgCred_CipherStrengths);
impl PSecPkgCred_CipherStrengths {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PSecPkgCred_CipherStrengths {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PSecPkgCred_ClientCertPolicy(pub *mut SecPkgCred_ClientCertPolicy);
impl PSecPkgCred_ClientCertPolicy {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PSecPkgCred_ClientCertPolicy {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PSecPkgCred_SessionTicketKey(pub *mut SecPkgCred_SessionTicketKey);
impl PSecPkgCred_SessionTicketKey {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PSecPkgCred_SessionTicketKey {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PSecPkgCred_SessionTicketKeys(pub *mut SecPkgCred_SessionTicketKeys);
impl PSecPkgCred_SessionTicketKeys {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PSecPkgCred_SessionTicketKeys {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "wincrypt")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PSecPkgCred_SupportedAlgs(pub *mut SecPkgCred_SupportedAlgs);
#[cfg(feature = "wincrypt")]
impl PSecPkgCred_SupportedAlgs {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(feature = "wincrypt")]
impl Default for PSecPkgCred_SupportedAlgs {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PSecPkgCred_SupportedProtocols(pub *mut SecPkgCred_SupportedProtocols);
impl PSecPkgCred_SupportedProtocols {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PSecPkgCred_SupportedProtocols {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const SECPKG_ATTR_APP_DATA: u32 = 94;
pub const SECPKG_ATTR_CC_POLICY_RESULT: u32 = 97;
pub const SECPKG_ATTR_CERT_CHECK_RESULT: u32 = 113;
pub const SECPKG_ATTR_CERT_CHECK_RESULT_INPROC: u32 = 114;
pub const SECPKG_ATTR_CIPHER_INFO: u32 = 100;
pub const SECPKG_ATTR_CIPHER_STRENGTHS: u32 = 87;
pub const SECPKG_ATTR_CLIENT_CERT_POLICY: u32 = 96;
pub const SECPKG_ATTR_CONNECTION_INFO: u32 = 90;
pub const SECPKG_ATTR_CONNECTION_INFO_EX: u32 = 110;
pub const SECPKG_ATTR_EAP_KEY_BLOCK: u32 = 91;
pub const SECPKG_ATTR_EAP_PRF_INFO: u32 = 101;
pub const SECPKG_ATTR_EARLY_START: u32 = 105;
pub const SECPKG_ATTR_ISSUER_LIST: u32 = 80;
pub const SECPKG_ATTR_ISSUER_LIST_EX: u32 = 89;
pub const SECPKG_ATTR_KEYING_MATERIAL: u32 = 107;
pub const SECPKG_ATTR_KEYING_MATERIAL_INFO: u32 = 106;
pub const SECPKG_ATTR_KEYING_MATERIAL_INPROC: u32 = 112;
pub const SECPKG_ATTR_KEYING_MATERIAL_TOKEN_BINDING: u32 = 111;
pub const SECPKG_ATTR_LOCAL_CERT_CONTEXT: u32 = 84;
pub const SECPKG_ATTR_LOCAL_CERT_INFO: u32 = 99;
pub const SECPKG_ATTR_LOCAL_CRED: u32 = 82;
pub const SECPKG_ATTR_MAPPED_CRED_ATTR: u32 = 92;
pub const SECPKG_ATTR_REMOTE_CERTIFICATES: u32 = 95;
pub const SECPKG_ATTR_REMOTE_CERT_CHAIN: u32 = 103;
pub const SECPKG_ATTR_REMOTE_CERT_CONTEXT: u32 = 83;
pub const SECPKG_ATTR_REMOTE_CRED: u32 = 81;
pub const SECPKG_ATTR_ROOT_STORE: u32 = 85;
pub const SECPKG_ATTR_SERIALIZED_REMOTE_CERT_CONTEXT: u32 = 117;
pub const SECPKG_ATTR_SERIALIZED_REMOTE_CERT_CONTEXT_INPROC: u32 = 116;
pub const SECPKG_ATTR_SESSION_INFO: u32 = 93;
pub const SECPKG_ATTR_SESSION_TICKET_KEYS: u32 = 115;
pub const SECPKG_ATTR_SRTP_PARAMETERS: u32 = 108;
pub const SECPKG_ATTR_SUPPORTED_ALGS: u32 = 86;
pub const SECPKG_ATTR_SUPPORTED_PROTOCOLS: u32 = 88;
pub const SECPKG_ATTR_SUPPORTED_SIGNATURES: u32 = 102;
pub const SECPKG_ATTR_TOKEN_BINDING: u32 = 109;
pub const SECPKG_ATTR_UI_INFO: u32 = 104;
pub const SECPKG_ATTR_USE_NCRYPT: u32 = 98;
pub const SESSION_TICKET_INFO_V0: u32 = 0;
pub const SESSION_TICKET_INFO_VERSION: u32 = 0;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct SecPkgCred_CipherStrengths {
    pub dwMinimumCipherStrength: u32,
    pub dwMaximumCipherStrength: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
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
#[derive(Clone, Copy, Debug, PartialEq)]
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
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct SecPkgCred_SessionTicketKeys {
    pub cSessionTicketKeys: u32,
    pub pSessionTicketKeys: PSecPkgCred_SessionTicketKey,
}
#[repr(C)]
#[cfg(feature = "wincrypt")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SecPkgCred_SupportedAlgs {
    pub cSupportedAlgs: u32,
    pub palgSupportedAlgs: *mut super::wincrypt::ALG_ID,
}
#[cfg(feature = "wincrypt")]
impl Default for SecPkgCred_SupportedAlgs {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct SecPkgCred_SupportedProtocols {
    pub grbitProtocol: u32,
}
