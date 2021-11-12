#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct AsymmetricKeyAlgorithmProvider(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct Capi1KdfTargetAlgorithm(pub i32);
impl Capi1KdfTargetAlgorithm {
    pub const NotAes: Capi1KdfTargetAlgorithm = Capi1KdfTargetAlgorithm(0i32);
    pub const Aes: Capi1KdfTargetAlgorithm = Capi1KdfTargetAlgorithm(1i32);
}
#[repr(transparent)]
pub struct CryptographicHash(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct CryptographicKey(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct CryptographicPadding(pub i32);
impl CryptographicPadding {
    pub const None: CryptographicPadding = CryptographicPadding(0i32);
    pub const RsaOaep: CryptographicPadding = CryptographicPadding(1i32);
    pub const RsaPkcs1V15: CryptographicPadding = CryptographicPadding(2i32);
    pub const RsaPss: CryptographicPadding = CryptographicPadding(3i32);
}
#[repr(transparent)]
pub struct CryptographicPrivateKeyBlobType(pub i32);
impl CryptographicPrivateKeyBlobType {
    pub const Pkcs8RawPrivateKeyInfo: CryptographicPrivateKeyBlobType = CryptographicPrivateKeyBlobType(0i32);
    pub const Pkcs1RsaPrivateKey: CryptographicPrivateKeyBlobType = CryptographicPrivateKeyBlobType(1i32);
    pub const BCryptPrivateKey: CryptographicPrivateKeyBlobType = CryptographicPrivateKeyBlobType(2i32);
    pub const Capi1PrivateKey: CryptographicPrivateKeyBlobType = CryptographicPrivateKeyBlobType(3i32);
    pub const BCryptEccFullPrivateKey: CryptographicPrivateKeyBlobType = CryptographicPrivateKeyBlobType(4i32);
}
#[repr(transparent)]
pub struct CryptographicPublicKeyBlobType(pub i32);
impl CryptographicPublicKeyBlobType {
    pub const X509SubjectPublicKeyInfo: CryptographicPublicKeyBlobType = CryptographicPublicKeyBlobType(0i32);
    pub const Pkcs1RsaPublicKey: CryptographicPublicKeyBlobType = CryptographicPublicKeyBlobType(1i32);
    pub const BCryptPublicKey: CryptographicPublicKeyBlobType = CryptographicPublicKeyBlobType(2i32);
    pub const Capi1PublicKey: CryptographicPublicKeyBlobType = CryptographicPublicKeyBlobType(3i32);
    pub const BCryptEccFullPublicKey: CryptographicPublicKeyBlobType = CryptographicPublicKeyBlobType(4i32);
}
#[repr(transparent)]
pub struct EncryptedAndAuthenticatedData(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct HashAlgorithmProvider(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAsymmetricAlgorithmNamesStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAsymmetricAlgorithmNamesStatics2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAsymmetricKeyAlgorithmProvider(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAsymmetricKeyAlgorithmProvider2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAsymmetricKeyAlgorithmProviderStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICryptographicEngineStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICryptographicEngineStatics2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICryptographicKey(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IEccCurveNamesStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IEncryptedAndAuthenticatedData(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IHashAlgorithmNamesStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IHashAlgorithmProvider(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IHashAlgorithmProviderStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IHashComputation(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IKeyDerivationAlgorithmNamesStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IKeyDerivationAlgorithmNamesStatics2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IKeyDerivationAlgorithmProvider(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IKeyDerivationAlgorithmProviderStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IKeyDerivationParameters(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IKeyDerivationParameters2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IKeyDerivationParametersStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IKeyDerivationParametersStatics2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMacAlgorithmNamesStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMacAlgorithmProvider(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMacAlgorithmProvider2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMacAlgorithmProviderStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPersistedKeyProviderStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISymmetricAlgorithmNamesStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISymmetricKeyAlgorithmProvider(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISymmetricKeyAlgorithmProviderStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct KeyDerivationAlgorithmProvider(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct KeyDerivationParameters(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct MacAlgorithmProvider(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct SymmetricKeyAlgorithmProvider(pub *mut ::core::ffi::c_void);
