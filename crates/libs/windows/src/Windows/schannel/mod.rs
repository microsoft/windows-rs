#[cfg(all(feature = "minwindef", feature = "wincrypt"))]
#[inline]
pub unsafe fn SslCrackCertificate(pbcertificate: *mut u8, cbcertificate: u32, dwflags: u32, ppcertificate: *mut PX509Certificate) -> windows_core::BOOL {
    windows_core::link!("schannel.dll" "system" fn SslCrackCertificate(pbcertificate : *mut u8, cbcertificate : u32, dwflags : u32, ppcertificate : *mut PX509Certificate) -> windows_core::BOOL);
    unsafe { SslCrackCertificate(pbcertificate as _, cbcertificate, dwflags, ppcertificate as _) }
}
#[cfg(all(feature = "minwindef", feature = "ncrypt", feature = "wincrypt"))]
#[inline]
pub unsafe fn SslDeserializeCertificateStore(serializedcertificatestore: super::wincrypt::CERT_BLOB, ppcertcontext: *mut super::wincrypt::PCCERT_CONTEXT) -> super::ncrypt::SECURITY_STATUS {
    windows_core::link!("schannel.dll" "system" fn SslDeserializeCertificateStore(serializedcertificatestore : super::wincrypt::CERT_BLOB, ppcertcontext : *mut super::wincrypt::PCCERT_CONTEXT) -> super::ncrypt::SECURITY_STATUS);
    unsafe { SslDeserializeCertificateStore(core::mem::transmute(serializedcertificatestore), ppcertcontext as _) }
}
#[inline]
pub unsafe fn SslEmptyCacheA<P0>(psztargetname: P0, dwflags: u32) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("schannel.dll" "system" fn SslEmptyCacheA(psztargetname : windows_core::PCSTR, dwflags : u32) -> windows_core::BOOL);
    unsafe { SslEmptyCacheA(psztargetname.param().abi(), dwflags) }
}
#[inline]
pub unsafe fn SslEmptyCacheW<P0>(psztargetname: P0, dwflags: u32) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("schannel.dll" "system" fn SslEmptyCacheW(psztargetname : windows_core::PCWSTR, dwflags : u32) -> windows_core::BOOL);
    unsafe { SslEmptyCacheW(psztargetname.param().abi(), dwflags) }
}
#[cfg(all(feature = "minwindef", feature = "wincrypt"))]
#[inline]
pub unsafe fn SslFreeCertificate(pcertificate: *mut X509Certificate) {
    windows_core::link!("schannel.dll" "system" fn SslFreeCertificate(pcertificate : *mut X509Certificate));
    unsafe { SslFreeCertificate(pcertificate as _) }
}
#[inline]
pub unsafe fn SslGenerateRandomBits(prandomdata: *mut u8, crandomdata: i32) {
    windows_core::link!("schannel.dll" "system" fn SslGenerateRandomBits(prandomdata : *mut u8, crandomdata : i32));
    unsafe { SslGenerateRandomBits(prandomdata as _, crandomdata) }
}
#[cfg(feature = "ncrypt")]
#[inline]
pub unsafe fn SslGetExtensions(clienthello: &[u8], genericextensions: &mut [SCH_EXTENSION_DATA], bytestoread: *mut u32, flags: SchGetExtensionsOptions) -> super::ncrypt::SECURITY_STATUS {
    windows_core::link!("schannel.dll" "system" fn SslGetExtensions(clienthello : *const u8, clienthellobytesize : u32, genericextensions : *mut SCH_EXTENSION_DATA, genericextensionscount : u8, bytestoread : *mut u32, flags : SchGetExtensionsOptions) -> super::ncrypt::SECURITY_STATUS);
    unsafe { SslGetExtensions(core::mem::transmute(clienthello.as_ptr()), clienthello.len().try_into().unwrap(), core::mem::transmute(genericextensions.as_ptr()), genericextensions.len().try_into().unwrap(), bytestoread as _, flags) }
}
#[inline]
pub unsafe fn SslGetMaximumKeySize(reserved: u32) -> u32 {
    windows_core::link!("schannel.dll" "system" fn SslGetMaximumKeySize(reserved : u32) -> u32);
    unsafe { SslGetMaximumKeySize(reserved) }
}
#[cfg(all(feature = "minwindef", feature = "ncrypt"))]
#[inline]
pub unsafe fn SslGetServerIdentity(clienthello: &[u8], serveridentity: *mut super::minwindef::PBYTE, serveridentitysize: *mut u32, flags: u32) -> super::ncrypt::SECURITY_STATUS {
    windows_core::link!("schannel.dll" "system" fn SslGetServerIdentity(clienthello : *const u8, clienthellosize : u32, serveridentity : *mut super::minwindef::PBYTE, serveridentitysize : *mut u32, flags : u32) -> super::ncrypt::SECURITY_STATUS);
    unsafe { SslGetServerIdentity(core::mem::transmute(clienthello.as_ptr()), clienthello.len().try_into().unwrap(), serveridentity as _, serveridentitysize as _, flags) }
}
pub const DEFAULT_TLS_SSP_NAME_A: windows_core::PCSTR = windows_core::s!("Default TLS SSP");
pub const DEFAULT_TLS_SSP_NAME_W: windows_core::PCWSTR = windows_core::w!("Default TLS SSP");
pub const ENABLE_TLS_CLIENT_EARLY_START: u32 = 1;
pub const KERN_CONTEXT_CERT_INFO_V1: u32 = 0;
pub const LCRED_CRED_EXISTS: u32 = 1;
pub const LCRED_STATUS_NOCRED: u32 = 0;
pub const LCRED_STATUS_UNKNOWN_ISSUER: u32 = 2;
pub const PCT1SP_NAME_A: windows_core::PCSTR = windows_core::s!("Microsoft PCT 1.0");
pub const PCT1SP_NAME_W: windows_core::PCWSTR = windows_core::w!("Microsoft PCT 1.0");
#[cfg(feature = "wincrypt")]
pub type PSCHANNEL_CERT_HASH = *mut SCHANNEL_CERT_HASH;
#[cfg(feature = "wincrypt")]
pub type PSCHANNEL_CERT_HASH_STORE = *mut SCHANNEL_CERT_HASH_STORE;
#[cfg(feature = "wincrypt")]
pub type PSCHANNEL_CLIENT_SIGNATURE = *mut SCHANNEL_CLIENT_SIGNATURE;
#[cfg(all(feature = "minwindef", feature = "wincrypt"))]
pub type PSCHANNEL_CRED = *mut SCHANNEL_CRED;
pub type PSCH_CRED = *mut SCH_CRED;
#[cfg(feature = "minwindef")]
pub type PSCH_CRED_PUBLIC_CERTCHAIN = *mut SCH_CRED_PUBLIC_CERTCHAIN;
#[cfg(feature = "wincrypt")]
pub type PSCH_CRED_SECRET_CAPI = *mut SCH_CRED_SECRET_CAPI;
#[cfg(feature = "minwindef")]
pub type PSCH_CRED_SECRET_PRIVKEY = *mut SCH_CRED_SECRET_PRIVKEY;
pub type PSEND_GENERIC_TLS_EXTENSION = *mut SEND_GENERIC_TLS_EXTENSION;
#[cfg(feature = "minwindef")]
pub type PSSL_CREDENTIAL_CERTIFICATE = *mut SSL_CREDENTIAL_CERTIFICATE;
pub type PSUBSCRIBE_GENERIC_TLS_EXTENSION = *mut SUBSCRIBE_GENERIC_TLS_EXTENSION;
pub type PSecPkgContext_CertInfo = *mut SecPkgContext_CertInfo;
pub type PSecPkgContext_CertificateValidationResult = *mut SecPkgContext_CertificateValidationResult;
#[cfg(feature = "minwindef")]
pub type PSecPkgContext_Certificates = *mut SecPkgContext_Certificates;
pub type PSecPkgContext_CipherInfo = *mut SecPkgContext_CipherInfo;
pub type PSecPkgContext_ClientCertPolicyResult = *mut SecPkgContext_ClientCertPolicyResult;
#[cfg(feature = "wincrypt")]
pub type PSecPkgContext_ConnectionInfo = *mut SecPkgContext_ConnectionInfo;
pub type PSecPkgContext_ConnectionInfoEx = *mut SecPkgContext_ConnectionInfoEx;
pub type PSecPkgContext_EapKeyBlock = *mut SecPkgContext_EapKeyBlock;
#[cfg(feature = "minwindef")]
pub type PSecPkgContext_EapPrfInfo = *mut SecPkgContext_EapPrfInfo;
pub type PSecPkgContext_EarlyStart = *mut SecPkgContext_EarlyStart;
#[cfg(feature = "wincrypt")]
pub type PSecPkgContext_IssuerListInfoEx = *mut SecPkgContext_IssuerListInfoEx;
#[cfg(feature = "minwindef")]
pub type PSecPkgContext_KeyingMaterial = *mut SecPkgContext_KeyingMaterial;
#[cfg(feature = "minwindef")]
pub type PSecPkgContext_KeyingMaterialInfo = *mut SecPkgContext_KeyingMaterialInfo;
#[cfg(feature = "minwindef")]
pub type PSecPkgContext_KeyingMaterial_Inproc = *mut SecPkgContext_KeyingMaterial_Inproc;
#[cfg(feature = "minwindef")]
pub type PSecPkgContext_LocalCredenitalInfo = *mut SecPkgContext_LocalCredentialInfo;
#[cfg(feature = "minwindef")]
pub type PSecPkgContext_LocalCredentialInfo = *mut SecPkgContext_LocalCredentialInfo;
pub type PSecPkgContext_MappedCredAttr = *mut SecPkgContext_MappedCredAttr;
#[cfg(feature = "minwindef")]
pub type PSecPkgContext_RemoteCredenitalInfo = *mut SecPkgContext_RemoteCredentialInfo;
#[cfg(feature = "minwindef")]
pub type PSecPkgContext_RemoteCredentialInfo = *mut SecPkgContext_RemoteCredentialInfo;
#[cfg(feature = "minwindef")]
pub type PSecPkgContext_SessionAppData = *mut SecPkgContext_SessionAppData;
pub type PSecPkgContext_SessionInfo = *mut SecPkgContext_SessionInfo;
#[cfg(feature = "minwindef")]
pub type PSecPkgContext_SrtpParameters = *mut SecPkgContext_SrtpParameters;
pub type PSecPkgContext_SupportedSignatures = *mut SecPkgContext_SupportedSignatures;
#[cfg(feature = "minwindef")]
pub type PSecPkgContext_TokenBinding = *mut SecPkgContext_TokenBinding;
#[cfg(feature = "windef")]
pub type PSecPkgContext_UiInfo = *mut SecPkgContext_UiInfo;
pub type PTLS_EXTENSION_SUBSCRIPTION = *mut TLS_EXTENSION_SUBSCRIPTION;
#[cfg(all(feature = "minwindef", feature = "wincrypt"))]
pub type PX509Certificate = *mut X509Certificate;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PctPublicKey {
    pub Type: u32,
    pub cbKey: u32,
    pub pKey: [u8; 1],
}
impl Default for PctPublicKey {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const RCRED_CRED_EXISTS: u32 = 1;
pub const RCRED_STATUS_NOCRED: u32 = 0;
pub const RCRED_STATUS_UNKNOWN_ISSUER: u32 = 2;
pub const SCHANNEL_ALERT: u32 = 2;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct SCHANNEL_ALERT_TOKEN {
    pub dwTokenType: u32,
    pub dwAlertType: u32,
    pub dwAlertNumber: u32,
}
#[repr(C)]
#[cfg(feature = "wincrypt")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SCHANNEL_CERT_HASH {
    pub dwLength: u32,
    pub dwFlags: u32,
    pub hProv: super::wincrypt::HCRYPTPROV,
    pub ShaHash: [u8; 20],
}
#[cfg(feature = "wincrypt")]
impl Default for SCHANNEL_CERT_HASH {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "wincrypt")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SCHANNEL_CERT_HASH_STORE {
    pub dwLength: u32,
    pub dwFlags: u32,
    pub hProv: super::wincrypt::HCRYPTPROV,
    pub ShaHash: [u8; 20],
    pub pwszStoreName: [u16; 128],
}
#[cfg(feature = "wincrypt")]
impl Default for SCHANNEL_CERT_HASH_STORE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "wincrypt")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SCHANNEL_CLIENT_SIGNATURE {
    pub cbLength: u32,
    pub aiHash: super::wincrypt::ALG_ID,
    pub cbHash: u32,
    pub HashValue: [u8; 36],
    pub CertThumbprint: [u8; 20],
}
#[cfg(feature = "wincrypt")]
impl Default for SCHANNEL_CLIENT_SIGNATURE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "minwindef", feature = "wincrypt"))]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SCHANNEL_CRED {
    pub dwVersion: u32,
    pub cCreds: u32,
    pub paCred: *mut super::wincrypt::PCCERT_CONTEXT,
    pub hRootStore: super::wincrypt::HCERTSTORE,
    pub cMappers: u32,
    pub aphMappers: *mut *mut _HMAPPER,
    pub cSupportedAlgs: u32,
    pub palgSupportedAlgs: *mut super::wincrypt::ALG_ID,
    pub grbitEnabledProtocols: u32,
    pub dwMinimumCipherStrength: u32,
    pub dwMaximumCipherStrength: u32,
    pub dwSessionLifespan: u32,
    pub dwFlags: u32,
    pub dwCredFormat: u32,
}
#[cfg(all(feature = "minwindef", feature = "wincrypt"))]
impl Default for SCHANNEL_CRED {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const SCHANNEL_CRED_VERSION: u32 = 4;
pub const SCHANNEL_NAME_A: windows_core::PCSTR = windows_core::s!("Schannel");
pub const SCHANNEL_NAME_W: windows_core::PCWSTR = windows_core::w!("Schannel");
pub const SCHANNEL_RENEGOTIATE: u32 = 0;
pub const SCHANNEL_SECRET_PRIVKEY: u32 = 2;
pub const SCHANNEL_SECRET_TYPE_CAPI: u32 = 1;
pub const SCHANNEL_SESSION: u32 = 3;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct SCHANNEL_SESSION_TOKEN {
    pub dwTokenType: u32,
    pub dwFlags: u32,
}
pub const SCHANNEL_SHUTDOWN: u32 = 1;
pub const SCH_ALLOW_NULL_ENCRYPTION: u32 = 33554432;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SCH_CRED {
    pub dwVersion: u32,
    pub cCreds: u32,
    pub paSecret: *mut *mut core::ffi::c_void,
    pub paPublic: *mut *mut core::ffi::c_void,
    pub cMappers: u32,
    pub aphMappers: *mut *mut _HMAPPER,
}
impl Default for SCH_CRED {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const SCH_CREDENTIALS_VERSION: u32 = 5;
pub const SCH_CRED_AUTO_CRED_VALIDATION: u32 = 32;
pub const SCH_CRED_CACHE_ONLY_URL_RETRIEVAL: u32 = 32768;
pub const SCH_CRED_CACHE_ONLY_URL_RETRIEVAL_ON_CREATE: u32 = 131072;
pub const SCH_CRED_CERT_CONTEXT: u32 = 3;
pub const SCH_CRED_DEFERRED_CRED_VALIDATION: u32 = 67108864;
pub const SCH_CRED_DISABLE_RECONNECTS: u32 = 128;
pub const SCH_CRED_FORMAT_CERT_CONTEXT: u32 = 0;
pub const SCH_CRED_FORMAT_CERT_HASH: u32 = 1;
pub const SCH_CRED_FORMAT_CERT_HASH_STORE: u32 = 2;
pub const SCH_CRED_IGNORE_NO_REVOCATION_CHECK: u32 = 2048;
pub const SCH_CRED_IGNORE_REVOCATION_OFFLINE: u32 = 4096;
pub const SCH_CRED_MANUAL_CRED_VALIDATION: u32 = 8;
pub const SCH_CRED_MAX_STORE_NAME_SIZE: u32 = 128;
pub const SCH_CRED_MAX_SUPPORTED_ALGS: u32 = 256;
pub const SCH_CRED_MAX_SUPPORTED_CERTS: u32 = 100;
pub const SCH_CRED_MEMORY_STORE_CERT: u32 = 65536;
pub const SCH_CRED_NO_DEFAULT_CREDS: u32 = 16;
pub const SCH_CRED_NO_SERVERNAME_CHECK: u32 = 4;
pub const SCH_CRED_NO_SYSTEM_MAPPER: u32 = 2;
#[repr(C)]
#[cfg(feature = "minwindef")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct SCH_CRED_PUBLIC_CERTCHAIN {
    pub dwType: u32,
    pub cbCertChain: u32,
    pub pCertChain: super::minwindef::PBYTE,
}
pub const SCH_CRED_RESTRICTED_ROOTS: u32 = 8192;
pub const SCH_CRED_REVOCATION_CHECK_CACHE_ONLY: u32 = 16384;
pub const SCH_CRED_REVOCATION_CHECK_CHAIN: u32 = 512;
pub const SCH_CRED_REVOCATION_CHECK_CHAIN_EXCLUDE_ROOT: u32 = 1024;
pub const SCH_CRED_REVOCATION_CHECK_END_CERT: u32 = 256;
#[repr(C)]
#[cfg(feature = "wincrypt")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct SCH_CRED_SECRET_CAPI {
    pub dwType: u32,
    pub hProv: super::wincrypt::HCRYPTPROV,
}
#[repr(C)]
#[cfg(feature = "minwindef")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct SCH_CRED_SECRET_PRIVKEY {
    pub dwType: u32,
    pub pPrivateKey: super::minwindef::PBYTE,
    pub cbPrivateKey: u32,
    pub pszPassword: windows_core::PSTR,
}
pub const SCH_CRED_SNI_CREDENTIAL: u32 = 524288;
pub const SCH_CRED_SNI_ENABLE_OCSP: u32 = 1048576;
pub const SCH_CRED_USE_DEFAULT_CREDS: u32 = 64;
pub const SCH_CRED_V1: u32 = 1;
pub const SCH_CRED_V2: u32 = 2;
pub const SCH_CRED_V3: u32 = 3;
pub const SCH_CRED_VERSION: u32 = 2;
pub const SCH_CRED_X509_CAPI: u32 = 2;
pub const SCH_CRED_X509_CERTCHAIN: u32 = 1;
pub const SCH_EXTENSIONS_OPTIONS_NONE: SchGetExtensionsOptions = 0;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SCH_EXTENSION_DATA {
    pub ExtensionType: u16,
    pub pExtData: *const u8,
    pub cbExtData: u32,
}
impl Default for SCH_EXTENSION_DATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const SCH_MACHINE_CERT_HASH: u32 = 1;
pub const SCH_MAX_EXT_SUBSCRIPTIONS: u32 = 2;
pub const SCH_NO_RECORD_HEADER: SchGetExtensionsOptions = 1;
pub const SCH_SEND_AUX_RECORD: u32 = 2097152;
pub const SCH_SEND_ROOT_CERT: u32 = 262144;
pub const SCH_USE_DTLS_ONLY: u32 = 16777216;
pub const SCH_USE_PRESHAREDKEY_ONLY: u32 = 8388608;
pub const SCH_USE_STRONG_CRYPTO: u32 = 4194304;
pub const SECPKGCONTEXT_CIPHERINFO_V1: u32 = 1;
pub const SECPKGCONTEXT_CONNECTION_INFO_EX_V1: u32 = 1;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SEND_GENERIC_TLS_EXTENSION {
    pub ExtensionType: u16,
    pub HandshakeType: u16,
    pub Flags: u32,
    pub BufferSize: u16,
    pub Buffer: [u8; 1],
}
impl Default for SEND_GENERIC_TLS_EXTENSION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const SP_PROT_ALL: u32 = 4294967295;
pub const SP_PROT_CLIENTS: i32 = -2147483478;
pub const SP_PROT_DTLS: u32 = 196608;
pub const SP_PROT_DTLS1_0: u32 = 196608;
pub const SP_PROT_DTLS1_0_CLIENT: u32 = 131072;
pub const SP_PROT_DTLS1_0_SERVER: u32 = 65536;
pub const SP_PROT_DTLS1_2: u32 = 786432;
pub const SP_PROT_DTLS1_2_CLIENT: u32 = 524288;
pub const SP_PROT_DTLS1_2_SERVER: u32 = 262144;
pub const SP_PROT_DTLS1_X: u32 = 983040;
pub const SP_PROT_DTLS1_X_CLIENT: u32 = 655360;
pub const SP_PROT_DTLS1_X_SERVER: u32 = 327680;
pub const SP_PROT_DTLS_CLIENT: u32 = 131072;
pub const SP_PROT_DTLS_SERVER: u32 = 65536;
pub const SP_PROT_NONE: u32 = 0;
pub const SP_PROT_PCT1: u32 = 3;
pub const SP_PROT_PCT1_CLIENT: u32 = 2;
pub const SP_PROT_PCT1_SERVER: u32 = 1;
pub const SP_PROT_SERVERS: u32 = 1073741909;
pub const SP_PROT_SSL2: u32 = 12;
pub const SP_PROT_SSL2_CLIENT: u32 = 8;
pub const SP_PROT_SSL2_SERVER: u32 = 4;
pub const SP_PROT_SSL3: u32 = 48;
pub const SP_PROT_SSL3TLS1: u32 = 240;
pub const SP_PROT_SSL3TLS1_CLIENTS: u32 = 160;
pub const SP_PROT_SSL3TLS1_SERVERS: u32 = 80;
pub const SP_PROT_SSL3TLS1_X: u32 = 16368;
pub const SP_PROT_SSL3TLS1_X_CLIENTS: u32 = 10912;
pub const SP_PROT_SSL3TLS1_X_SERVERS: u32 = 5456;
pub const SP_PROT_SSL3_CLIENT: u32 = 32;
pub const SP_PROT_SSL3_SERVER: u32 = 16;
pub const SP_PROT_TLS1: u32 = 192;
pub const SP_PROT_TLS1_0: u32 = 192;
pub const SP_PROT_TLS1_0_CLIENT: u32 = 128;
pub const SP_PROT_TLS1_0_SERVER: u32 = 64;
pub const SP_PROT_TLS1_1: u32 = 768;
pub const SP_PROT_TLS1_1PLUS: u32 = 16128;
pub const SP_PROT_TLS1_1PLUS_CLIENT: u32 = 10752;
pub const SP_PROT_TLS1_1PLUS_SERVER: u32 = 5376;
pub const SP_PROT_TLS1_1_CLIENT: u32 = 512;
pub const SP_PROT_TLS1_1_SERVER: u32 = 256;
pub const SP_PROT_TLS1_2: u32 = 3072;
pub const SP_PROT_TLS1_2_CLIENT: u32 = 2048;
pub const SP_PROT_TLS1_2_SERVER: u32 = 1024;
pub const SP_PROT_TLS1_3: u32 = 12288;
pub const SP_PROT_TLS1_3PLUS: u32 = 12288;
pub const SP_PROT_TLS1_3PLUS_CLIENT: u32 = 8192;
pub const SP_PROT_TLS1_3PLUS_SERVER: u32 = 4096;
pub const SP_PROT_TLS1_3_CLIENT: u32 = 8192;
pub const SP_PROT_TLS1_3_SERVER: u32 = 4096;
pub const SP_PROT_TLS1_CLIENT: u32 = 128;
pub const SP_PROT_TLS1_SERVER: u32 = 64;
pub const SP_PROT_TLS1_X: u32 = 16320;
pub const SP_PROT_TLS1_X_CLIENT: u32 = 10880;
pub const SP_PROT_TLS1_X_SERVER: u32 = 5440;
pub const SP_PROT_UNI: i32 = -1073741824;
pub const SP_PROT_UNI_CLIENT: u32 = 2147483648;
pub const SP_PROT_UNI_SERVER: u32 = 1073741824;
pub const SP_PROT_X_CLIENTS: i32 = -2146817366;
pub const SP_PROT_X_SERVERS: u32 = 1074074965;
pub const SSL2SP_NAME_A: windows_core::PCSTR = windows_core::s!("Microsoft SSL 2.0");
pub const SSL2SP_NAME_W: windows_core::PCWSTR = windows_core::w!("Microsoft SSL 2.0");
pub const SSL3SP_NAME_A: windows_core::PCSTR = windows_core::s!("Microsoft SSL 3.0");
pub const SSL3SP_NAME_W: windows_core::PCWSTR = windows_core::w!("Microsoft SSL 3.0");
#[cfg(all(feature = "minwindef", feature = "wincrypt"))]
pub type SSL_CRACK_CERTIFICATE_FN = Option<unsafe extern "system" fn(pbcertificate: *mut u8, cbcertificate: u32, verifysignature: windows_core::BOOL, ppcertificate: *mut PX509Certificate) -> windows_core::BOOL>;
#[repr(C)]
#[cfg(feature = "minwindef")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct SSL_CREDENTIAL_CERTIFICATE {
    pub cbPrivateKey: u32,
    pub pPrivateKey: super::minwindef::PBYTE,
    pub cbCertificate: u32,
    pub pCertificate: super::minwindef::PBYTE,
    pub pszPassword: windows_core::PSTR,
}
pub type SSL_EMPTY_CACHE_FN_A = Option<unsafe extern "system" fn(psztargetname: windows_core::PCSTR, dwflags: u32) -> windows_core::BOOL>;
pub type SSL_EMPTY_CACHE_FN_W = Option<unsafe extern "system" fn(psztargetname: windows_core::PCWSTR, dwflags: u32) -> windows_core::BOOL>;
#[cfg(all(feature = "minwindef", feature = "wincrypt"))]
pub type SSL_FREE_CERTIFICATE_FN = Option<unsafe extern "system" fn(pcertificate: *mut X509Certificate)>;
pub const SSL_SESSION_DISABLE_RECONNECTS: u32 = 2;
pub const SSL_SESSION_ENABLE_RECONNECTS: u32 = 1;
pub const SSL_SESSION_RECONNECT: u32 = 1;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SUBSCRIBE_GENERIC_TLS_EXTENSION {
    pub Flags: u32,
    pub SubscriptionsCount: u32,
    pub Subscriptions: [TLS_EXTENSION_SUBSCRIPTION; 1],
}
impl Default for SUBSCRIBE_GENERIC_TLS_EXTENSION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type SchGetExtensionsOptions = u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct SecPkgContext_CertInfo {
    pub dwVersion: u32,
    pub cbSubjectName: u32,
    pub pwszSubjectName: windows_core::PWSTR,
    pub cbIssuerName: u32,
    pub pwszIssuerName: windows_core::PWSTR,
    pub dwKeySize: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct SecPkgContext_CertificateValidationResult {
    pub dwChainErrorStatus: u32,
    pub hrVerifyChainStatus: windows_core::HRESULT,
}
#[repr(C)]
#[cfg(feature = "minwindef")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct SecPkgContext_Certificates {
    pub cCertificates: u32,
    pub cbCertificateChain: u32,
    pub pbCertificateChain: super::minwindef::PBYTE,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SecPkgContext_CipherInfo {
    pub dwVersion: u32,
    pub dwProtocol: u32,
    pub dwCipherSuite: u32,
    pub dwBaseCipherSuite: u32,
    pub szCipherSuite: [u16; 64],
    pub szCipher: [u16; 64],
    pub dwCipherLen: u32,
    pub dwCipherBlockLen: u32,
    pub szHash: [u16; 64],
    pub dwHashLen: u32,
    pub szExchange: [u16; 64],
    pub dwMinExchangeLen: u32,
    pub dwMaxExchangeLen: u32,
    pub szCertificate: [u16; 64],
    pub dwKeyType: u32,
}
impl Default for SecPkgContext_CipherInfo {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct SecPkgContext_ClientCertPolicyResult {
    pub dwPolicyResult: windows_core::HRESULT,
    pub guidPolicyId: windows_core::GUID,
}
#[repr(C)]
#[cfg(feature = "wincrypt")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct SecPkgContext_ConnectionInfo {
    pub dwProtocol: u32,
    pub aiCipher: super::wincrypt::ALG_ID,
    pub dwCipherStrength: u32,
    pub aiHash: super::wincrypt::ALG_ID,
    pub dwHashStrength: u32,
    pub aiExch: super::wincrypt::ALG_ID,
    pub dwExchStrength: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SecPkgContext_ConnectionInfoEx {
    pub dwVersion: u32,
    pub dwProtocol: u32,
    pub szCipher: [u16; 64],
    pub dwCipherStrength: u32,
    pub szHash: [u16; 64],
    pub dwHashStrength: u32,
    pub szExchange: [u16; 64],
    pub dwExchStrength: u32,
}
impl Default for SecPkgContext_ConnectionInfoEx {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SecPkgContext_EapKeyBlock {
    pub rgbKeys: [u8; 128],
    pub rgbIVs: [u8; 64],
}
impl Default for SecPkgContext_EapKeyBlock {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "minwindef")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct SecPkgContext_EapPrfInfo {
    pub dwVersion: u32,
    pub cbPrfData: u32,
    pub pbPrfData: super::minwindef::PBYTE,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct SecPkgContext_EarlyStart {
    pub dwEarlyStartFlags: u32,
}
#[repr(C)]
#[cfg(feature = "wincrypt")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct SecPkgContext_IssuerListInfoEx {
    pub aIssuers: super::wincrypt::PCERT_NAME_BLOB,
    pub cIssuers: u32,
}
#[repr(C)]
#[cfg(feature = "minwindef")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct SecPkgContext_KeyingMaterial {
    pub cbKeyingMaterial: u32,
    pub pbKeyingMaterial: super::minwindef::PBYTE,
}
#[repr(C)]
#[cfg(feature = "minwindef")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct SecPkgContext_KeyingMaterialInfo {
    pub cbLabel: u16,
    pub pszLabel: windows_core::PSTR,
    pub cbContextValue: u16,
    pub pbContextValue: super::minwindef::PBYTE,
    pub cbKeyingMaterial: u32,
}
#[repr(C)]
#[cfg(feature = "minwindef")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct SecPkgContext_KeyingMaterial_Inproc {
    pub cbLabel: u16,
    pub pszLabel: windows_core::PSTR,
    pub cbContextValue: u16,
    pub pbContextValue: super::minwindef::PBYTE,
    pub cbKeyingMaterial: u32,
    pub pbKeyingMaterial: super::minwindef::PBYTE,
}
#[cfg(feature = "minwindef")]
pub type SecPkgContext_LocalCredenitalInfo = SecPkgContext_LocalCredentialInfo;
#[repr(C)]
#[cfg(feature = "minwindef")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct SecPkgContext_LocalCredentialInfo {
    pub cbCertificateChain: u32,
    pub pbCertificateChain: super::minwindef::PBYTE,
    pub cCertificates: u32,
    pub fFlags: u32,
    pub dwBits: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SecPkgContext_MappedCredAttr {
    pub dwAttribute: u32,
    pub pvBuffer: *mut core::ffi::c_void,
}
impl Default for SecPkgContext_MappedCredAttr {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "minwindef")]
pub type SecPkgContext_RemoteCredenitalInfo = SecPkgContext_RemoteCredentialInfo;
#[repr(C)]
#[cfg(feature = "minwindef")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct SecPkgContext_RemoteCredentialInfo {
    pub cbCertificateChain: u32,
    pub pbCertificateChain: super::minwindef::PBYTE,
    pub cCertificates: u32,
    pub fFlags: u32,
    pub dwBits: u32,
}
#[repr(C)]
#[cfg(feature = "minwindef")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct SecPkgContext_SessionAppData {
    pub dwFlags: u32,
    pub cbAppData: u32,
    pub pbAppData: super::minwindef::PBYTE,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SecPkgContext_SessionInfo {
    pub dwFlags: u32,
    pub cbSessionId: u32,
    pub rgbSessionId: [u8; 32],
}
impl Default for SecPkgContext_SessionInfo {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "minwindef")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct SecPkgContext_SrtpParameters {
    pub ProtectionProfile: u16,
    pub MasterKeyIdentifierSize: u8,
    pub MasterKeyIdentifier: super::minwindef::PBYTE,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SecPkgContext_SupportedSignatures {
    pub cSignatureAndHashAlgorithms: u16,
    pub pSignatureAndHashAlgorithms: *mut u16,
}
impl Default for SecPkgContext_SupportedSignatures {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "minwindef")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct SecPkgContext_TokenBinding {
    pub MajorVersion: u8,
    pub MinorVersion: u8,
    pub KeyParametersSize: u16,
    pub KeyParameters: super::minwindef::PBYTE,
}
#[repr(C)]
#[cfg(feature = "windef")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct SecPkgContext_UiInfo {
    pub hParentWindow: super::windef::HWND,
}
#[cfg(all(feature = "minwindef", feature = "ncrypt", feature = "wincrypt"))]
pub type SslDeserializeCertificateStoreFn = Option<unsafe extern "system" fn(serializedcertificatestore: super::wincrypt::CERT_BLOB, ppcertcontext: *mut super::wincrypt::PCCERT_CONTEXT) -> super::ncrypt::SECURITY_STATUS>;
#[cfg(feature = "ncrypt")]
pub type SslGetExtensionsFn = Option<unsafe extern "system" fn(clienthello: *const u8, clienthellobytesize: u32, genericextensions: *mut SCH_EXTENSION_DATA, genericextensionscount: u8, bytestoread: *mut u32, flags: SchGetExtensionsOptions) -> super::ncrypt::SECURITY_STATUS>;
#[cfg(all(feature = "minwindef", feature = "ncrypt"))]
pub type SslGetServerIdentityFn = Option<unsafe extern "system" fn(clienthello: *const u8, clienthellosize: u32, serveridentity: *mut super::minwindef::PBYTE, serveridentitysize: *mut u32, flags: u32) -> super::ncrypt::SECURITY_STATUS>;
pub const TLS1SP_NAME_A: windows_core::PCSTR = windows_core::s!("Microsoft TLS 1.0");
pub const TLS1SP_NAME_W: windows_core::PCWSTR = windows_core::w!("Microsoft TLS 1.0");
pub const TLS1_ALERT_ACCESS_DENIED: u32 = 49;
pub const TLS1_ALERT_BAD_CERTIFICATE: u32 = 42;
pub const TLS1_ALERT_BAD_RECORD_MAC: u32 = 20;
pub const TLS1_ALERT_CERTIFICATE_EXPIRED: u32 = 45;
pub const TLS1_ALERT_CERTIFICATE_REVOKED: u32 = 44;
pub const TLS1_ALERT_CERTIFICATE_UNKNOWN: u32 = 46;
pub const TLS1_ALERT_CLOSE_NOTIFY: u32 = 0;
pub const TLS1_ALERT_DECODE_ERROR: u32 = 50;
pub const TLS1_ALERT_DECOMPRESSION_FAIL: u32 = 30;
pub const TLS1_ALERT_DECRYPTION_FAILED: u32 = 21;
pub const TLS1_ALERT_DECRYPT_ERROR: u32 = 51;
pub const TLS1_ALERT_EXPORT_RESTRICTION: u32 = 60;
pub const TLS1_ALERT_FATAL: u32 = 2;
pub const TLS1_ALERT_HANDSHAKE_FAILURE: u32 = 40;
pub const TLS1_ALERT_ILLEGAL_PARAMETER: u32 = 47;
pub const TLS1_ALERT_INSUFFIENT_SECURITY: u32 = 71;
pub const TLS1_ALERT_INTERNAL_ERROR: u32 = 80;
pub const TLS1_ALERT_NO_APP_PROTOCOL: u32 = 120;
pub const TLS1_ALERT_NO_RENEGOTIATION: u32 = 100;
pub const TLS1_ALERT_PROTOCOL_VERSION: u32 = 70;
pub const TLS1_ALERT_RECORD_OVERFLOW: u32 = 22;
pub const TLS1_ALERT_UNEXPECTED_MESSAGE: u32 = 10;
pub const TLS1_ALERT_UNKNOWN_CA: u32 = 48;
pub const TLS1_ALERT_UNKNOWN_PSK_IDENTITY: u32 = 115;
pub const TLS1_ALERT_UNSUPPORTED_CERT: u32 = 43;
pub const TLS1_ALERT_UNSUPPORTED_EXT: u32 = 110;
pub const TLS1_ALERT_USER_CANCELED: u32 = 90;
pub const TLS1_ALERT_WARNING: u32 = 1;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct TLS_EXTENSION_SUBSCRIPTION {
    pub ExtensionType: u16,
    pub HandshakeType: u16,
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
pub const UNISP_NAME_A: windows_core::PCSTR = windows_core::s!("Microsoft Unified Security Protocol Provider");
pub const UNISP_NAME_W: windows_core::PCWSTR = windows_core::w!("Microsoft Unified Security Protocol Provider");
pub const UNISP_RPC_ID: u32 = 14;
#[repr(C)]
#[cfg(all(feature = "minwindef", feature = "wincrypt"))]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct X509Certificate {
    pub Version: u32,
    pub SerialNumber: [u32; 4],
    pub SignatureAlgorithm: super::wincrypt::ALG_ID,
    pub ValidFrom: super::minwindef::FILETIME,
    pub ValidUntil: super::minwindef::FILETIME,
    pub pszIssuer: windows_core::PSTR,
    pub pszSubject: windows_core::PSTR,
    pub pPublicKey: *mut PctPublicKey,
}
#[cfg(all(feature = "minwindef", feature = "wincrypt"))]
impl Default for X509Certificate {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct _HMAPPER(pub u8);
pub type eTlsHashAlgorithm = i32;
pub type eTlsSignatureAlgorithm = i32;
