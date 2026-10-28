#[cfg(all(feature = "minwindef", feature = "ncrypt", feature = "shared", feature = "wincrypt"))]
#[inline]
pub unsafe fn SslDeserializeCertificateStore(serializedcertificatestore: super::CERT_BLOB, ppcertcontext: *mut super::PCCERT_CONTEXT) -> super::SECURITY_STATUS {
    windows_core::link!("schannel.dll" "system" fn SslDeserializeCertificateStore(serializedcertificatestore : super::CERT_BLOB, ppcertcontext : *mut super::PCCERT_CONTEXT) -> super::SECURITY_STATUS);
    unsafe { SslDeserializeCertificateStore(serializedcertificatestore, ppcertcontext as _) }
}
#[cfg(all(feature = "ncrypt", feature = "shared"))]
#[inline]
pub unsafe fn SslGetExtensions(clienthello: &[super::BYTE], genericextensions: &mut [SCH_EXTENSION_DATA], bytestoread: *mut super::DWORD, flags: SchGetExtensionsOptions) -> super::SECURITY_STATUS {
    windows_core::link!("schannel.dll" "system" fn SslGetExtensions(clienthello : *const super::BYTE, clienthellobytesize : super::DWORD, genericextensions : *mut SCH_EXTENSION_DATA, genericextensionscount : super::BYTE, bytestoread : *mut super::DWORD, flags : SchGetExtensionsOptions) -> super::SECURITY_STATUS);
    unsafe { SslGetExtensions(clienthello.as_ptr(), clienthello.len().try_into().unwrap(), genericextensions.as_mut_ptr(), genericextensions.len().try_into().unwrap(), bytestoread as _, flags) }
}
#[cfg(all(feature = "minwindef", feature = "ncrypt", feature = "shared"))]
#[inline]
pub unsafe fn SslGetServerIdentity(clienthello: &[u8], serveridentity: *mut super::PBYTE, serveridentitysize: super::PDWORD, flags: super::DWORD) -> super::SECURITY_STATUS {
    windows_core::link!("schannel.dll" "system" fn SslGetServerIdentity(clienthello : super::PBYTE, clienthellosize : super::DWORD, serveridentity : *mut super::PBYTE, serveridentitysize : super::PDWORD, flags : super::DWORD) -> super::SECURITY_STATUS);
    unsafe { SslGetServerIdentity(core::mem::transmute(clienthello.as_ptr()), clienthello.len().try_into().unwrap(), serveridentity as _, serveridentitysize as _, flags) }
}
pub const DEFAULT_TLS_SSP_NAME: windows_core::PCSTR = windows_core::s!("Default TLS SSP");
pub const DEFAULT_TLS_SSP_NAME_A: windows_core::PCSTR = windows_core::s!("Default TLS SSP");
pub const DEFAULT_TLS_SSP_NAME_W: windows_core::PCWSTR = windows_core::w!("Default TLS SSP");
pub const ENABLE_TLS_CLIENT_EARLY_START: i32 = 1;
pub const KERN_CONTEXT_CERT_INFO_V1: i32 = 0;
pub const LCRED_CRED_EXISTS: i32 = 1;
pub const LCRED_STATUS_NOCRED: i32 = 0;
pub const LCRED_STATUS_UNKNOWN_ISSUER: i32 = 2;
pub const PCT1SP_NAME: windows_core::PCSTR = windows_core::s!("Microsoft PCT 1.0");
pub const PCT1SP_NAME_A: windows_core::PCSTR = windows_core::s!("Microsoft PCT 1.0");
pub const PCT1SP_NAME_W: windows_core::PCWSTR = windows_core::w!("Microsoft PCT 1.0");
#[cfg(all(feature = "shared", feature = "wincrypt"))]
pub type PSCHANNEL_CERT_HASH = *mut SCHANNEL_CERT_HASH;
#[cfg(all(feature = "shared", feature = "wincrypt", feature = "winnt"))]
pub type PSCHANNEL_CERT_HASH_STORE = *mut SCHANNEL_CERT_HASH_STORE;
#[cfg(feature = "shared")]
pub type PSCHANNEL_CLIENT_SIGNATURE = *mut SCHANNEL_CLIENT_SIGNATURE;
#[cfg(all(feature = "minwindef", feature = "shared", feature = "wincrypt"))]
pub type PSCHANNEL_CRED = *mut SCHANNEL_CRED;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PSCH_CRED = *mut SCH_CRED;
#[cfg(feature = "shared")]
pub type PSCH_CRED_PUBLIC_CERTCHAIN = *mut SCH_CRED_PUBLIC_CERTCHAIN;
#[cfg(all(feature = "shared", feature = "wincrypt"))]
pub type PSCH_CRED_SECRET_CAPI = *mut SCH_CRED_SECRET_CAPI;
#[cfg(feature = "shared")]
pub type PSCH_CRED_SECRET_PRIVKEY = *mut SCH_CRED_SECRET_PRIVKEY;
#[cfg(feature = "shared")]
pub type PSEND_GENERIC_TLS_EXTENSION = *mut SEND_GENERIC_TLS_EXTENSION;
#[cfg(feature = "shared")]
pub type PSSL_CREDENTIAL_CERTIFICATE = *mut SSL_CREDENTIAL_CERTIFICATE;
#[cfg(feature = "shared")]
pub type PSUBSCRIBE_GENERIC_TLS_EXTENSION = *mut SUBSCRIBE_GENERIC_TLS_EXTENSION;
#[cfg(feature = "shared")]
pub type PSecPkgContext_CertInfo = *mut SecPkgContext_CertInfo;
#[cfg(feature = "shared")]
pub type PSecPkgContext_CertificateValidationResult = *mut SecPkgContext_CertificateValidationResult;
#[cfg(feature = "shared")]
pub type PSecPkgContext_Certificates = *mut SecPkgContext_Certificates;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PSecPkgContext_CipherInfo = *mut SecPkgContext_CipherInfo;
pub type PSecPkgContext_ClientCertPolicyResult = *mut SecPkgContext_ClientCertPolicyResult;
#[cfg(feature = "shared")]
pub type PSecPkgContext_ConnectionInfo = *mut SecPkgContext_ConnectionInfo;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PSecPkgContext_ConnectionInfoEx = *mut SecPkgContext_ConnectionInfoEx;
#[cfg(feature = "shared")]
pub type PSecPkgContext_EapKeyBlock = *mut SecPkgContext_EapKeyBlock;
#[cfg(feature = "shared")]
pub type PSecPkgContext_EapPrfInfo = *mut SecPkgContext_EapPrfInfo;
#[cfg(feature = "shared")]
pub type PSecPkgContext_EarlyStart = *mut SecPkgContext_EarlyStart;
#[cfg(all(feature = "shared", feature = "wincrypt"))]
pub type PSecPkgContext_IssuerListInfoEx = *mut SecPkgContext_IssuerListInfoEx;
#[cfg(feature = "shared")]
pub type PSecPkgContext_KeyingMaterial = *mut SecPkgContext_KeyingMaterial;
#[cfg(feature = "shared")]
pub type PSecPkgContext_KeyingMaterialInfo = *mut SecPkgContext_KeyingMaterialInfo;
#[cfg(feature = "shared")]
pub type PSecPkgContext_KeyingMaterial_Inproc = *mut SecPkgContext_KeyingMaterial_Inproc;
#[cfg(feature = "shared")]
pub type PSecPkgContext_LocalCredenitalInfo = *mut SecPkgContext_LocalCredentialInfo;
#[cfg(feature = "shared")]
pub type PSecPkgContext_LocalCredentialInfo = *mut SecPkgContext_LocalCredentialInfo;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PSecPkgContext_MappedCredAttr = *mut SecPkgContext_MappedCredAttr;
#[cfg(feature = "shared")]
pub type PSecPkgContext_RemoteCredenitalInfo = *mut SecPkgContext_RemoteCredentialInfo;
#[cfg(feature = "shared")]
pub type PSecPkgContext_RemoteCredentialInfo = *mut SecPkgContext_RemoteCredentialInfo;
#[cfg(feature = "shared")]
pub type PSecPkgContext_SessionAppData = *mut SecPkgContext_SessionAppData;
#[cfg(feature = "shared")]
pub type PSecPkgContext_SessionInfo = *mut SecPkgContext_SessionInfo;
#[cfg(feature = "shared")]
pub type PSecPkgContext_SrtpParameters = *mut SecPkgContext_SrtpParameters;
#[cfg(feature = "shared")]
pub type PSecPkgContext_SupportedSignatures = *mut SecPkgContext_SupportedSignatures;
#[cfg(feature = "shared")]
pub type PSecPkgContext_TokenBinding = *mut SecPkgContext_TokenBinding;
#[cfg(feature = "windef")]
pub type PSecPkgContext_UiInfo = *mut SecPkgContext_UiInfo;
#[cfg(feature = "shared")]
pub type PTLS_EXTENSION_SUBSCRIPTION = *mut TLS_EXTENSION_SUBSCRIPTION;
#[cfg(all(feature = "minwindef", feature = "shared"))]
pub type PX509Certificate = *mut X509Certificate;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PctPublicKey {
    pub Type: super::DWORD,
    pub cbKey: super::DWORD,
    pub pKey: [super::UCHAR; 1],
}
#[cfg(feature = "shared")]
impl Default for PctPublicKey {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const RCRED_CRED_EXISTS: i32 = 1;
pub const RCRED_STATUS_NOCRED: i32 = 0;
pub const RCRED_STATUS_UNKNOWN_ISSUER: i32 = 2;
pub const SCHANNEL_ALERT: i32 = 2;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SCHANNEL_ALERT_TOKEN {
    pub dwTokenType: super::DWORD,
    pub dwAlertType: super::DWORD,
    pub dwAlertNumber: super::DWORD,
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "wincrypt"))]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SCHANNEL_CERT_HASH {
    pub dwLength: super::DWORD,
    pub dwFlags: super::DWORD,
    pub hProv: super::HCRYPTPROV,
    pub ShaHash: [super::BYTE; 20],
}
#[cfg(all(feature = "shared", feature = "wincrypt"))]
impl Default for SCHANNEL_CERT_HASH {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "wincrypt", feature = "winnt"))]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SCHANNEL_CERT_HASH_STORE {
    pub dwLength: super::DWORD,
    pub dwFlags: super::DWORD,
    pub hProv: super::HCRYPTPROV,
    pub ShaHash: [super::BYTE; 20],
    pub pwszStoreName: [super::WCHAR; 128],
}
#[cfg(all(feature = "shared", feature = "wincrypt", feature = "winnt"))]
impl Default for SCHANNEL_CERT_HASH_STORE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SCHANNEL_CLIENT_SIGNATURE {
    pub cbLength: super::DWORD,
    pub aiHash: super::ALG_ID,
    pub cbHash: super::DWORD,
    pub HashValue: [super::BYTE; 36],
    pub CertThumbprint: [super::BYTE; 20],
}
#[cfg(feature = "shared")]
impl Default for SCHANNEL_CLIENT_SIGNATURE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "minwindef", feature = "shared", feature = "wincrypt"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SCHANNEL_CRED {
    pub dwVersion: super::DWORD,
    pub cCreds: super::DWORD,
    pub paCred: *mut super::PCCERT_CONTEXT,
    pub hRootStore: super::HCERTSTORE,
    pub cMappers: super::DWORD,
    pub aphMappers: *mut *mut _HMAPPER,
    pub cSupportedAlgs: super::DWORD,
    pub palgSupportedAlgs: *mut super::ALG_ID,
    pub grbitEnabledProtocols: super::DWORD,
    pub dwMinimumCipherStrength: super::DWORD,
    pub dwMaximumCipherStrength: super::DWORD,
    pub dwSessionLifespan: super::DWORD,
    pub dwFlags: super::DWORD,
    pub dwCredFormat: super::DWORD,
}
pub const SCHANNEL_CRED_VERSION: i32 = 4;
pub const SCHANNEL_NAME: windows_core::PCSTR = windows_core::s!("Schannel");
pub const SCHANNEL_NAME_A: windows_core::PCSTR = windows_core::s!("Schannel");
pub const SCHANNEL_NAME_W: windows_core::PCWSTR = windows_core::w!("Schannel");
pub const SCHANNEL_RENEGOTIATE: i32 = 0;
pub const SCHANNEL_SECRET_PRIVKEY: i32 = 2;
pub const SCHANNEL_SECRET_TYPE_CAPI: i32 = 1;
pub const SCHANNEL_SESSION: i32 = 3;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SCHANNEL_SESSION_TOKEN {
    pub dwTokenType: super::DWORD,
    pub dwFlags: super::DWORD,
}
pub const SCHANNEL_SHUTDOWN: i32 = 1;
pub const SCH_ALLOW_NULL_ENCRYPTION: i32 = 33554432;
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SCH_CRED {
    pub dwVersion: super::DWORD,
    pub cCreds: super::DWORD,
    pub paSecret: *mut super::PVOID,
    pub paPublic: *mut super::PVOID,
    pub cMappers: super::DWORD,
    pub aphMappers: *mut *mut _HMAPPER,
}
pub const SCH_CREDENTIALS_VERSION: i32 = 5;
pub const SCH_CRED_AUTO_CRED_VALIDATION: i32 = 32;
pub const SCH_CRED_CACHE_ONLY_URL_RETRIEVAL: i32 = 32768;
pub const SCH_CRED_CACHE_ONLY_URL_RETRIEVAL_ON_CREATE: i32 = 131072;
pub const SCH_CRED_CERT_CONTEXT: i32 = 3;
pub const SCH_CRED_DEFERRED_CRED_VALIDATION: i32 = 67108864;
pub const SCH_CRED_DISABLE_RECONNECTS: i32 = 128;
pub const SCH_CRED_FORMAT_CERT_CONTEXT: i32 = 0;
pub const SCH_CRED_FORMAT_CERT_HASH: i32 = 1;
pub const SCH_CRED_FORMAT_CERT_HASH_STORE: i32 = 2;
pub const SCH_CRED_IGNORE_NO_REVOCATION_CHECK: i32 = 2048;
pub const SCH_CRED_IGNORE_REVOCATION_OFFLINE: i32 = 4096;
pub const SCH_CRED_MANUAL_CRED_VALIDATION: i32 = 8;
pub const SCH_CRED_MAX_STORE_NAME_SIZE: i32 = 128;
pub const SCH_CRED_MAX_SUPPORTED_ALGS: i32 = 256;
pub const SCH_CRED_MAX_SUPPORTED_CERTS: i32 = 100;
pub const SCH_CRED_MEMORY_STORE_CERT: i32 = 65536;
pub const SCH_CRED_NO_DEFAULT_CREDS: i32 = 16;
pub const SCH_CRED_NO_SERVERNAME_CHECK: i32 = 4;
pub const SCH_CRED_NO_SYSTEM_MAPPER: i32 = 2;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SCH_CRED_PUBLIC_CERTCHAIN {
    pub dwType: super::DWORD,
    pub cbCertChain: super::DWORD,
    pub pCertChain: super::PBYTE,
}
pub const SCH_CRED_RESTRICTED_ROOTS: i32 = 8192;
pub const SCH_CRED_REVOCATION_CHECK_CACHE_ONLY: i32 = 16384;
pub const SCH_CRED_REVOCATION_CHECK_CHAIN: i32 = 512;
pub const SCH_CRED_REVOCATION_CHECK_CHAIN_EXCLUDE_ROOT: i32 = 1024;
pub const SCH_CRED_REVOCATION_CHECK_END_CERT: i32 = 256;
#[repr(C)]
#[cfg(all(feature = "shared", feature = "wincrypt"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SCH_CRED_SECRET_CAPI {
    pub dwType: super::DWORD,
    pub hProv: super::HCRYPTPROV,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SCH_CRED_SECRET_PRIVKEY {
    pub dwType: super::DWORD,
    pub pPrivateKey: super::PBYTE,
    pub cbPrivateKey: super::DWORD,
    pub pszPassword: windows_core::PSTR,
}
pub const SCH_CRED_SNI_CREDENTIAL: i32 = 524288;
pub const SCH_CRED_SNI_ENABLE_OCSP: i32 = 1048576;
pub const SCH_CRED_USE_DEFAULT_CREDS: i32 = 64;
pub const SCH_CRED_V1: i32 = 1;
pub const SCH_CRED_V2: i32 = 2;
pub const SCH_CRED_V3: i32 = 3;
pub const SCH_CRED_VERSION: i32 = 2;
pub const SCH_CRED_X509_CAPI: i32 = 2;
pub const SCH_CRED_X509_CERTCHAIN: i32 = 1;
pub const SCH_EXTENSIONS_OPTIONS_NONE: SchGetExtensionsOptions = 0;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SCH_EXTENSION_DATA {
    pub ExtensionType: super::WORD,
    pub pExtData: *const super::BYTE,
    pub cbExtData: super::DWORD,
}
pub const SCH_MACHINE_CERT_HASH: i32 = 1;
pub const SCH_MAX_EXT_SUBSCRIPTIONS: i32 = 2;
pub const SCH_NO_RECORD_HEADER: SchGetExtensionsOptions = 1;
pub const SCH_SEND_AUX_RECORD: i32 = 2097152;
pub const SCH_SEND_ROOT_CERT: i32 = 262144;
pub const SCH_USE_DTLS_ONLY: i32 = 16777216;
pub const SCH_USE_PRESHAREDKEY_ONLY: i32 = 8388608;
pub const SCH_USE_STRONG_CRYPTO: i32 = 4194304;
pub const SECPKGCONTEXT_CIPHERINFO_V1: i32 = 1;
pub const SECPKGCONTEXT_CONNECTION_INFO_EX_V1: i32 = 1;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SEND_GENERIC_TLS_EXTENSION {
    pub ExtensionType: super::WORD,
    pub HandshakeType: super::WORD,
    pub Flags: super::DWORD,
    pub BufferSize: super::WORD,
    pub Buffer: [super::UCHAR; 1],
}
#[cfg(feature = "shared")]
impl Default for SEND_GENERIC_TLS_EXTENSION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const SP_PROT_ALL: u32 = 4294967295;
pub const SP_PROT_CLIENTS: u32 = 2147483818;
pub const SP_PROT_DTLS: i32 = 196608;
pub const SP_PROT_DTLS1_0: i32 = 196608;
pub const SP_PROT_DTLS1_0_CLIENT: i32 = 131072;
pub const SP_PROT_DTLS1_0_SERVER: i32 = 65536;
pub const SP_PROT_DTLS1_2: i32 = 786432;
pub const SP_PROT_DTLS1_2_CLIENT: i32 = 524288;
pub const SP_PROT_DTLS1_2_SERVER: i32 = 262144;
pub const SP_PROT_DTLS1_X: i32 = 983040;
pub const SP_PROT_DTLS1_X_CLIENT: i32 = 655360;
pub const SP_PROT_DTLS1_X_SERVER: i32 = 327680;
pub const SP_PROT_DTLS_CLIENT: i32 = 131072;
pub const SP_PROT_DTLS_SERVER: i32 = 65536;
pub const SP_PROT_NONE: i32 = 0;
pub const SP_PROT_PCT1: i32 = 3;
pub const SP_PROT_PCT1_CLIENT: i32 = 2;
pub const SP_PROT_PCT1_SERVER: i32 = 1;
pub const SP_PROT_SERVERS: i32 = 1073741909;
pub const SP_PROT_SSL2: i32 = 12;
pub const SP_PROT_SSL2_CLIENT: i32 = 8;
pub const SP_PROT_SSL2_SERVER: i32 = 4;
pub const SP_PROT_SSL3: i32 = 48;
pub const SP_PROT_SSL3TLS1: i32 = 240;
pub const SP_PROT_SSL3TLS1_CLIENTS: i32 = 160;
pub const SP_PROT_SSL3TLS1_SERVERS: i32 = 80;
pub const SP_PROT_SSL3TLS1_X: i32 = 16368;
pub const SP_PROT_SSL3TLS1_X_CLIENTS: i32 = 10912;
pub const SP_PROT_SSL3TLS1_X_SERVERS: i32 = 5456;
pub const SP_PROT_SSL3_CLIENT: i32 = 32;
pub const SP_PROT_SSL3_SERVER: i32 = 16;
pub const SP_PROT_TLS1: i32 = 192;
pub const SP_PROT_TLS1_0: i32 = 192;
pub const SP_PROT_TLS1_0_CLIENT: i32 = 128;
pub const SP_PROT_TLS1_0_SERVER: i32 = 64;
pub const SP_PROT_TLS1_1: i32 = 768;
pub const SP_PROT_TLS1_1PLUS: i32 = 16128;
pub const SP_PROT_TLS1_1PLUS_CLIENT: i32 = 10752;
pub const SP_PROT_TLS1_1PLUS_SERVER: i32 = 5376;
pub const SP_PROT_TLS1_1_CLIENT: i32 = 512;
pub const SP_PROT_TLS1_1_SERVER: i32 = 256;
pub const SP_PROT_TLS1_2: i32 = 3072;
pub const SP_PROT_TLS1_2_CLIENT: i32 = 2048;
pub const SP_PROT_TLS1_2_SERVER: i32 = 1024;
pub const SP_PROT_TLS1_3: i32 = 12288;
pub const SP_PROT_TLS1_3PLUS: i32 = 12288;
pub const SP_PROT_TLS1_3PLUS_CLIENT: i32 = 8192;
pub const SP_PROT_TLS1_3PLUS_SERVER: i32 = 4096;
pub const SP_PROT_TLS1_3_CLIENT: i32 = 8192;
pub const SP_PROT_TLS1_3_SERVER: i32 = 4096;
pub const SP_PROT_TLS1_CLIENT: i32 = 128;
pub const SP_PROT_TLS1_SERVER: i32 = 64;
pub const SP_PROT_TLS1_X: i32 = 16320;
pub const SP_PROT_TLS1_X_CLIENT: i32 = 10880;
pub const SP_PROT_TLS1_X_SERVER: i32 = 5440;
pub const SP_PROT_UNI: u32 = 3221225472;
pub const SP_PROT_UNI_CLIENT: u32 = 2147483648;
pub const SP_PROT_UNI_SERVER: i32 = 1073741824;
pub const SP_PROT_X_CLIENTS: u32 = 2148149930;
pub const SP_PROT_X_SERVERS: i32 = 1074074965;
pub const SSL2SP_NAME: windows_core::PCSTR = windows_core::s!("Microsoft SSL 2.0");
pub const SSL2SP_NAME_A: windows_core::PCSTR = windows_core::s!("Microsoft SSL 2.0");
pub const SSL2SP_NAME_W: windows_core::PCWSTR = windows_core::w!("Microsoft SSL 2.0");
pub const SSL3SP_NAME: windows_core::PCSTR = windows_core::s!("Microsoft SSL 3.0");
pub const SSL3SP_NAME_A: windows_core::PCSTR = windows_core::s!("Microsoft SSL 3.0");
pub const SSL3SP_NAME_W: windows_core::PCWSTR = windows_core::w!("Microsoft SSL 3.0");
#[cfg(all(feature = "minwindef", feature = "shared"))]
pub type SSL_CRACK_CERTIFICATE_FN = Option<unsafe extern "system" fn(pbcertificate: super::PUCHAR, cbcertificate: super::DWORD, verifysignature: windows_core::BOOL, ppcertificate: *mut PX509Certificate) -> windows_core::BOOL>;
pub const SSL_CRACK_CERTIFICATE_NAME: windows_core::PCSTR = windows_core::s!("SslCrackCertificate");
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SSL_CREDENTIAL_CERTIFICATE {
    pub cbPrivateKey: super::DWORD,
    pub pPrivateKey: super::PBYTE,
    pub cbCertificate: super::DWORD,
    pub pCertificate: super::PBYTE,
    pub pszPassword: windows_core::PSTR,
}
#[cfg(feature = "shared")]
pub type SSL_EMPTY_CACHE_FN_A = Option<unsafe extern "system" fn(psztargetname: windows_core::PSTR, dwflags: super::DWORD) -> windows_core::BOOL>;
#[cfg(feature = "shared")]
pub type SSL_EMPTY_CACHE_FN_W = Option<unsafe extern "system" fn(psztargetname: windows_core::PWSTR, dwflags: super::DWORD) -> windows_core::BOOL>;
#[cfg(all(feature = "minwindef", feature = "shared"))]
pub type SSL_FREE_CERTIFICATE_FN = Option<unsafe extern "system" fn(pcertificate: PX509Certificate)>;
pub const SSL_FREE_CERTIFICATE_NAME: windows_core::PCSTR = windows_core::s!("SslFreeCertificate");
pub const SSL_SESSION_DISABLE_RECONNECTS: i32 = 2;
pub const SSL_SESSION_ENABLE_RECONNECTS: i32 = 1;
pub const SSL_SESSION_RECONNECT: i32 = 1;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SUBSCRIBE_GENERIC_TLS_EXTENSION {
    pub Flags: super::DWORD,
    pub SubscriptionsCount: super::DWORD,
    pub Subscriptions: [TLS_EXTENSION_SUBSCRIPTION; 1],
}
#[cfg(feature = "shared")]
impl Default for SUBSCRIBE_GENERIC_TLS_EXTENSION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const SZ_ALG_MAX_SIZE: i32 = 64;
pub type SchGetExtensionsOptions = i32;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SecPkgContext_CertInfo {
    pub dwVersion: super::DWORD,
    pub cbSubjectName: super::DWORD,
    pub pwszSubjectName: windows_core::PWSTR,
    pub cbIssuerName: super::DWORD,
    pub pwszIssuerName: windows_core::PWSTR,
    pub dwKeySize: super::DWORD,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SecPkgContext_CertificateValidationResult {
    pub dwChainErrorStatus: super::DWORD,
    pub hrVerifyChainStatus: windows_core::HRESULT,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SecPkgContext_Certificates {
    pub cCertificates: super::DWORD,
    pub cbCertificateChain: super::DWORD,
    pub pbCertificateChain: super::PBYTE,
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SecPkgContext_CipherInfo {
    pub dwVersion: super::DWORD,
    pub dwProtocol: super::DWORD,
    pub dwCipherSuite: super::DWORD,
    pub dwBaseCipherSuite: super::DWORD,
    pub szCipherSuite: [super::WCHAR; 64],
    pub szCipher: [super::WCHAR; 64],
    pub dwCipherLen: super::DWORD,
    pub dwCipherBlockLen: super::DWORD,
    pub szHash: [super::WCHAR; 64],
    pub dwHashLen: super::DWORD,
    pub szExchange: [super::WCHAR; 64],
    pub dwMinExchangeLen: super::DWORD,
    pub dwMaxExchangeLen: super::DWORD,
    pub szCertificate: [super::WCHAR; 64],
    pub dwKeyType: super::DWORD,
}
#[cfg(all(feature = "shared", feature = "winnt"))]
impl Default for SecPkgContext_CipherInfo {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SecPkgContext_ClientCertPolicyResult {
    pub dwPolicyResult: windows_core::HRESULT,
    pub guidPolicyId: windows_core::GUID,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SecPkgContext_ConnectionInfo {
    pub dwProtocol: super::DWORD,
    pub aiCipher: super::ALG_ID,
    pub dwCipherStrength: super::DWORD,
    pub aiHash: super::ALG_ID,
    pub dwHashStrength: super::DWORD,
    pub aiExch: super::ALG_ID,
    pub dwExchStrength: super::DWORD,
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SecPkgContext_ConnectionInfoEx {
    pub dwVersion: super::DWORD,
    pub dwProtocol: super::DWORD,
    pub szCipher: [super::WCHAR; 64],
    pub dwCipherStrength: super::DWORD,
    pub szHash: [super::WCHAR; 64],
    pub dwHashStrength: super::DWORD,
    pub szExchange: [super::WCHAR; 64],
    pub dwExchStrength: super::DWORD,
}
#[cfg(all(feature = "shared", feature = "winnt"))]
impl Default for SecPkgContext_ConnectionInfoEx {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SecPkgContext_EapKeyBlock {
    pub rgbKeys: [super::BYTE; 128],
    pub rgbIVs: [super::BYTE; 64],
}
#[cfg(feature = "shared")]
impl Default for SecPkgContext_EapKeyBlock {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SecPkgContext_EapPrfInfo {
    pub dwVersion: super::DWORD,
    pub cbPrfData: super::DWORD,
    pub pbPrfData: super::PBYTE,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SecPkgContext_EarlyStart {
    pub dwEarlyStartFlags: super::DWORD,
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "wincrypt"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SecPkgContext_IssuerListInfoEx {
    pub aIssuers: super::PCERT_NAME_BLOB,
    pub cIssuers: super::DWORD,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SecPkgContext_KeyingMaterial {
    pub cbKeyingMaterial: super::DWORD,
    pub pbKeyingMaterial: super::PBYTE,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SecPkgContext_KeyingMaterialInfo {
    pub cbLabel: super::WORD,
    pub pszLabel: windows_core::PSTR,
    pub cbContextValue: super::WORD,
    pub pbContextValue: super::PBYTE,
    pub cbKeyingMaterial: super::DWORD,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SecPkgContext_KeyingMaterial_Inproc {
    pub cbLabel: super::WORD,
    pub pszLabel: windows_core::PSTR,
    pub cbContextValue: super::WORD,
    pub pbContextValue: super::PBYTE,
    pub cbKeyingMaterial: super::DWORD,
    pub pbKeyingMaterial: super::PBYTE,
}
#[cfg(feature = "shared")]
pub type SecPkgContext_LocalCredenitalInfo = SecPkgContext_LocalCredentialInfo;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SecPkgContext_LocalCredentialInfo {
    pub cbCertificateChain: super::DWORD,
    pub pbCertificateChain: super::PBYTE,
    pub cCertificates: super::DWORD,
    pub fFlags: super::DWORD,
    pub dwBits: super::DWORD,
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SecPkgContext_MappedCredAttr {
    pub dwAttribute: super::DWORD,
    pub pvBuffer: super::PVOID,
}
#[cfg(feature = "shared")]
pub type SecPkgContext_RemoteCredenitalInfo = SecPkgContext_RemoteCredentialInfo;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SecPkgContext_RemoteCredentialInfo {
    pub cbCertificateChain: super::DWORD,
    pub pbCertificateChain: super::PBYTE,
    pub cCertificates: super::DWORD,
    pub fFlags: super::DWORD,
    pub dwBits: super::DWORD,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SecPkgContext_SessionAppData {
    pub dwFlags: super::DWORD,
    pub cbAppData: super::DWORD,
    pub pbAppData: super::PBYTE,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SecPkgContext_SessionInfo {
    pub dwFlags: super::DWORD,
    pub cbSessionId: super::DWORD,
    pub rgbSessionId: [super::BYTE; 32],
}
#[cfg(feature = "shared")]
impl Default for SecPkgContext_SessionInfo {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SecPkgContext_SrtpParameters {
    pub ProtectionProfile: super::WORD,
    pub MasterKeyIdentifierSize: super::BYTE,
    pub MasterKeyIdentifier: super::PBYTE,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SecPkgContext_SupportedSignatures {
    pub cSignatureAndHashAlgorithms: super::WORD,
    pub pSignatureAndHashAlgorithms: *mut super::WORD,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SecPkgContext_TokenBinding {
    pub MajorVersion: super::BYTE,
    pub MinorVersion: super::BYTE,
    pub KeyParametersSize: super::WORD,
    pub KeyParameters: super::PBYTE,
}
#[repr(C)]
#[cfg(feature = "windef")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SecPkgContext_UiInfo {
    pub hParentWindow: super::HWND,
}
#[cfg(all(feature = "minwindef", feature = "ncrypt", feature = "shared", feature = "wincrypt"))]
pub type SslDeserializeCertificateStoreFn = Option<unsafe extern "system" fn(serializedcertificatestore: super::CERT_BLOB, ppcertcontext: *mut super::PCCERT_CONTEXT) -> super::SECURITY_STATUS>;
#[cfg(all(feature = "ncrypt", feature = "shared"))]
pub type SslGetExtensionsFn = Option<unsafe extern "system" fn(clienthello: *const super::BYTE, clienthellobytesize: super::DWORD, genericextensions: *mut SCH_EXTENSION_DATA, genericextensionscount: super::BYTE, bytestoread: *mut super::DWORD, flags: SchGetExtensionsOptions) -> super::SECURITY_STATUS>;
#[cfg(all(feature = "minwindef", feature = "ncrypt", feature = "shared"))]
pub type SslGetServerIdentityFn = Option<unsafe extern "system" fn(clienthello: super::PBYTE, clienthellosize: super::DWORD, serveridentity: *mut super::PBYTE, serveridentitysize: super::PDWORD, flags: super::DWORD) -> super::SECURITY_STATUS>;
pub const TLS1SP_NAME: windows_core::PCSTR = windows_core::s!("Microsoft TLS 1.0");
pub const TLS1SP_NAME_A: windows_core::PCSTR = windows_core::s!("Microsoft TLS 1.0");
pub const TLS1SP_NAME_W: windows_core::PCWSTR = windows_core::w!("Microsoft TLS 1.0");
pub const TLS1_ALERT_ACCESS_DENIED: i32 = 49;
pub const TLS1_ALERT_BAD_CERTIFICATE: i32 = 42;
pub const TLS1_ALERT_BAD_RECORD_MAC: i32 = 20;
pub const TLS1_ALERT_CERTIFICATE_EXPIRED: i32 = 45;
pub const TLS1_ALERT_CERTIFICATE_REVOKED: i32 = 44;
pub const TLS1_ALERT_CERTIFICATE_UNKNOWN: i32 = 46;
pub const TLS1_ALERT_CLOSE_NOTIFY: i32 = 0;
pub const TLS1_ALERT_DECODE_ERROR: i32 = 50;
pub const TLS1_ALERT_DECOMPRESSION_FAIL: i32 = 30;
pub const TLS1_ALERT_DECRYPTION_FAILED: i32 = 21;
pub const TLS1_ALERT_DECRYPT_ERROR: i32 = 51;
pub const TLS1_ALERT_EXPORT_RESTRICTION: i32 = 60;
pub const TLS1_ALERT_FATAL: i32 = 2;
pub const TLS1_ALERT_HANDSHAKE_FAILURE: i32 = 40;
pub const TLS1_ALERT_ILLEGAL_PARAMETER: i32 = 47;
pub const TLS1_ALERT_INSUFFIENT_SECURITY: i32 = 71;
pub const TLS1_ALERT_INTERNAL_ERROR: i32 = 80;
pub const TLS1_ALERT_NO_APP_PROTOCOL: i32 = 120;
pub const TLS1_ALERT_NO_RENEGOTIATION: i32 = 100;
pub const TLS1_ALERT_PROTOCOL_VERSION: i32 = 70;
pub const TLS1_ALERT_RECORD_OVERFLOW: i32 = 22;
pub const TLS1_ALERT_UNEXPECTED_MESSAGE: i32 = 10;
pub const TLS1_ALERT_UNKNOWN_CA: i32 = 48;
pub const TLS1_ALERT_UNKNOWN_PSK_IDENTITY: i32 = 115;
pub const TLS1_ALERT_UNSUPPORTED_CERT: i32 = 43;
pub const TLS1_ALERT_UNSUPPORTED_EXT: i32 = 110;
pub const TLS1_ALERT_USER_CANCELED: i32 = 90;
pub const TLS1_ALERT_WARNING: i32 = 1;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct TLS_EXTENSION_SUBSCRIPTION {
    pub ExtensionType: super::WORD,
    pub HandshakeType: super::WORD,
}
pub const TlsHashAlgorithm_Md5: eTlsHashAlgorithm = 1;
pub const TlsHashAlgorithm_None: eTlsHashAlgorithm = 0;
pub const TlsHashAlgorithm_Sha1: eTlsHashAlgorithm = 2;
pub const TlsHashAlgorithm_Sha224: eTlsHashAlgorithm = 3;
pub const TlsHashAlgorithm_Sha256: eTlsHashAlgorithm = 4;
pub const TlsHashAlgorithm_Sha384: eTlsHashAlgorithm = 5;
pub const TlsHashAlgorithm_Sha512: eTlsHashAlgorithm = 6;
pub const TlsSignatureAlgorithm_Anonymous: eTlsSignatureAlgorithm = 0;
pub const TlsSignatureAlgorithm_Dsa: eTlsSignatureAlgorithm = 2;
pub const TlsSignatureAlgorithm_Ecdsa: eTlsSignatureAlgorithm = 3;
pub const TlsSignatureAlgorithm_Rsa: eTlsSignatureAlgorithm = 1;
pub const UNISP_NAME: windows_core::PCSTR = windows_core::s!("Microsoft Unified Security Protocol Provider");
pub const UNISP_NAME_A: windows_core::PCSTR = windows_core::s!("Microsoft Unified Security Protocol Provider");
pub const UNISP_NAME_W: windows_core::PCWSTR = windows_core::w!("Microsoft Unified Security Protocol Provider");
pub const UNISP_RPC_ID: i32 = 14;
#[repr(C)]
#[cfg(all(feature = "minwindef", feature = "shared"))]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct X509Certificate {
    pub Version: super::DWORD,
    pub SerialNumber: [super::DWORD; 4],
    pub SignatureAlgorithm: super::ALG_ID,
    pub ValidFrom: super::FILETIME,
    pub ValidUntil: super::FILETIME,
    pub pszIssuer: windows_core::PSTR,
    pub pszSubject: windows_core::PSTR,
    pub pPublicKey: *mut PctPublicKey,
}
#[cfg(all(feature = "minwindef", feature = "shared"))]
impl Default for X509Certificate {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct _HMAPPER(pub u8);
pub type eTlsHashAlgorithm = i32;
pub type eTlsSignatureAlgorithm = i32;
