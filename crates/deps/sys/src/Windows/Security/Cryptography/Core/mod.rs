#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct AsymmetricKeyAlgorithmProvider(pub *mut ::core::ffi::c_void);
#[repr(C)]
pub struct Capi1KdfTargetAlgorithm(i32);
#[repr(transparent)]
pub struct CryptographicHash(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct CryptographicKey(pub *mut ::core::ffi::c_void);
#[repr(C)]
pub struct CryptographicPadding(i32);
#[repr(C)]
pub struct CryptographicPrivateKeyBlobType(i32);
#[repr(C)]
pub struct CryptographicPublicKeyBlobType(i32);
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
