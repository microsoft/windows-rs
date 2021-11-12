#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct AsymmetricKeyAlgorithmProvider(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for AsymmetricKeyAlgorithmProvider {}
impl ::core::clone::Clone for AsymmetricKeyAlgorithmProvider {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct Capi1KdfTargetAlgorithm(pub i32);
impl Capi1KdfTargetAlgorithm {
    pub const NotAes: Self = Self(0i32);
    pub const Aes: Self = Self(1i32);
}
impl ::core::marker::Copy for Capi1KdfTargetAlgorithm {}
impl ::core::clone::Clone for Capi1KdfTargetAlgorithm {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct CryptographicHash(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for CryptographicHash {}
impl ::core::clone::Clone for CryptographicHash {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct CryptographicKey(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for CryptographicKey {}
impl ::core::clone::Clone for CryptographicKey {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct CryptographicPadding(pub i32);
impl CryptographicPadding {
    pub const None: Self = Self(0i32);
    pub const RsaOaep: Self = Self(1i32);
    pub const RsaPkcs1V15: Self = Self(2i32);
    pub const RsaPss: Self = Self(3i32);
}
impl ::core::marker::Copy for CryptographicPadding {}
impl ::core::clone::Clone for CryptographicPadding {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct CryptographicPrivateKeyBlobType(pub i32);
impl CryptographicPrivateKeyBlobType {
    pub const Pkcs8RawPrivateKeyInfo: Self = Self(0i32);
    pub const Pkcs1RsaPrivateKey: Self = Self(1i32);
    pub const BCryptPrivateKey: Self = Self(2i32);
    pub const Capi1PrivateKey: Self = Self(3i32);
    pub const BCryptEccFullPrivateKey: Self = Self(4i32);
}
impl ::core::marker::Copy for CryptographicPrivateKeyBlobType {}
impl ::core::clone::Clone for CryptographicPrivateKeyBlobType {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct CryptographicPublicKeyBlobType(pub i32);
impl CryptographicPublicKeyBlobType {
    pub const X509SubjectPublicKeyInfo: Self = Self(0i32);
    pub const Pkcs1RsaPublicKey: Self = Self(1i32);
    pub const BCryptPublicKey: Self = Self(2i32);
    pub const Capi1PublicKey: Self = Self(3i32);
    pub const BCryptEccFullPublicKey: Self = Self(4i32);
}
impl ::core::marker::Copy for CryptographicPublicKeyBlobType {}
impl ::core::clone::Clone for CryptographicPublicKeyBlobType {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct EncryptedAndAuthenticatedData(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for EncryptedAndAuthenticatedData {}
impl ::core::clone::Clone for EncryptedAndAuthenticatedData {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct HashAlgorithmProvider(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for HashAlgorithmProvider {}
impl ::core::clone::Clone for HashAlgorithmProvider {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IAsymmetricAlgorithmNamesStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IAsymmetricAlgorithmNamesStatics {}
impl ::core::clone::Clone for IAsymmetricAlgorithmNamesStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IAsymmetricAlgorithmNamesStatics2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IAsymmetricAlgorithmNamesStatics2 {}
impl ::core::clone::Clone for IAsymmetricAlgorithmNamesStatics2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IAsymmetricKeyAlgorithmProvider(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IAsymmetricKeyAlgorithmProvider {}
impl ::core::clone::Clone for IAsymmetricKeyAlgorithmProvider {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IAsymmetricKeyAlgorithmProvider2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IAsymmetricKeyAlgorithmProvider2 {}
impl ::core::clone::Clone for IAsymmetricKeyAlgorithmProvider2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IAsymmetricKeyAlgorithmProviderStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IAsymmetricKeyAlgorithmProviderStatics {}
impl ::core::clone::Clone for IAsymmetricKeyAlgorithmProviderStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ICryptographicEngineStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ICryptographicEngineStatics {}
impl ::core::clone::Clone for ICryptographicEngineStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ICryptographicEngineStatics2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ICryptographicEngineStatics2 {}
impl ::core::clone::Clone for ICryptographicEngineStatics2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ICryptographicKey(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ICryptographicKey {}
impl ::core::clone::Clone for ICryptographicKey {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IEccCurveNamesStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IEccCurveNamesStatics {}
impl ::core::clone::Clone for IEccCurveNamesStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IEncryptedAndAuthenticatedData(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IEncryptedAndAuthenticatedData {}
impl ::core::clone::Clone for IEncryptedAndAuthenticatedData {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IHashAlgorithmNamesStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IHashAlgorithmNamesStatics {}
impl ::core::clone::Clone for IHashAlgorithmNamesStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IHashAlgorithmProvider(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IHashAlgorithmProvider {}
impl ::core::clone::Clone for IHashAlgorithmProvider {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IHashAlgorithmProviderStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IHashAlgorithmProviderStatics {}
impl ::core::clone::Clone for IHashAlgorithmProviderStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IHashComputation(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IHashComputation {}
impl ::core::clone::Clone for IHashComputation {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IKeyDerivationAlgorithmNamesStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IKeyDerivationAlgorithmNamesStatics {}
impl ::core::clone::Clone for IKeyDerivationAlgorithmNamesStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IKeyDerivationAlgorithmNamesStatics2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IKeyDerivationAlgorithmNamesStatics2 {}
impl ::core::clone::Clone for IKeyDerivationAlgorithmNamesStatics2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IKeyDerivationAlgorithmProvider(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IKeyDerivationAlgorithmProvider {}
impl ::core::clone::Clone for IKeyDerivationAlgorithmProvider {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IKeyDerivationAlgorithmProviderStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IKeyDerivationAlgorithmProviderStatics {}
impl ::core::clone::Clone for IKeyDerivationAlgorithmProviderStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IKeyDerivationParameters(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IKeyDerivationParameters {}
impl ::core::clone::Clone for IKeyDerivationParameters {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IKeyDerivationParameters2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IKeyDerivationParameters2 {}
impl ::core::clone::Clone for IKeyDerivationParameters2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IKeyDerivationParametersStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IKeyDerivationParametersStatics {}
impl ::core::clone::Clone for IKeyDerivationParametersStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IKeyDerivationParametersStatics2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IKeyDerivationParametersStatics2 {}
impl ::core::clone::Clone for IKeyDerivationParametersStatics2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IMacAlgorithmNamesStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IMacAlgorithmNamesStatics {}
impl ::core::clone::Clone for IMacAlgorithmNamesStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IMacAlgorithmProvider(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IMacAlgorithmProvider {}
impl ::core::clone::Clone for IMacAlgorithmProvider {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IMacAlgorithmProvider2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IMacAlgorithmProvider2 {}
impl ::core::clone::Clone for IMacAlgorithmProvider2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IMacAlgorithmProviderStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IMacAlgorithmProviderStatics {}
impl ::core::clone::Clone for IMacAlgorithmProviderStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IPersistedKeyProviderStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IPersistedKeyProviderStatics {}
impl ::core::clone::Clone for IPersistedKeyProviderStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISymmetricAlgorithmNamesStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISymmetricAlgorithmNamesStatics {}
impl ::core::clone::Clone for ISymmetricAlgorithmNamesStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISymmetricKeyAlgorithmProvider(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISymmetricKeyAlgorithmProvider {}
impl ::core::clone::Clone for ISymmetricKeyAlgorithmProvider {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISymmetricKeyAlgorithmProviderStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISymmetricKeyAlgorithmProviderStatics {}
impl ::core::clone::Clone for ISymmetricKeyAlgorithmProviderStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct KeyDerivationAlgorithmProvider(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for KeyDerivationAlgorithmProvider {}
impl ::core::clone::Clone for KeyDerivationAlgorithmProvider {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct KeyDerivationParameters(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for KeyDerivationParameters {}
impl ::core::clone::Clone for KeyDerivationParameters {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct MacAlgorithmProvider(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for MacAlgorithmProvider {}
impl ::core::clone::Clone for MacAlgorithmProvider {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct SymmetricKeyAlgorithmProvider(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for SymmetricKeyAlgorithmProvider {}
impl ::core::clone::Clone for SymmetricKeyAlgorithmProvider {
    fn clone(&self) -> Self {
        *self
    }
}
