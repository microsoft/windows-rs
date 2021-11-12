#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct Certificate(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct CertificateChain(pub *mut ::core::ffi::c_void);
#[repr(C)]
pub struct CertificateChainPolicy(i32);
#[repr(transparent)]
pub struct CertificateEnrollmentManager(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct CertificateExtension(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct CertificateKeyUsages(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct CertificateQuery(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct CertificateRequestProperties(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct CertificateStore(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct CertificateStores(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ChainBuildingParameters(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ChainValidationParameters(pub *mut ::core::ffi::c_void);
#[repr(C)]
pub struct ChainValidationResult(i32);
#[repr(transparent)]
pub struct CmsAttachedSignature(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct CmsDetachedSignature(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct CmsSignerInfo(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct CmsTimestampInfo(pub *mut ::core::ffi::c_void);
#[repr(C)]
pub struct EnrollKeyUsages(i32);
#[repr(C)]
pub struct ExportOption(i32);
#[repr(transparent)]
pub struct ICertificate(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICertificate2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICertificate3(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICertificateChain(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICertificateEnrollmentManagerStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICertificateEnrollmentManagerStatics2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICertificateEnrollmentManagerStatics3(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICertificateExtension(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICertificateFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICertificateKeyUsages(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICertificateQuery(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICertificateQuery2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICertificateRequestProperties(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICertificateRequestProperties2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICertificateRequestProperties3(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICertificateRequestProperties4(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICertificateStore(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICertificateStore2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICertificateStoresStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICertificateStoresStatics2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IChainBuildingParameters(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IChainValidationParameters(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICmsAttachedSignature(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICmsAttachedSignatureFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICmsAttachedSignatureStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICmsDetachedSignature(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICmsDetachedSignatureFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICmsDetachedSignatureStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICmsSignerInfo(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICmsTimestampInfo(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IKeyAlgorithmNamesStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IKeyAlgorithmNamesStatics2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IKeyAttestationHelperStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IKeyAttestationHelperStatics2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IKeyStorageProviderNamesStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IKeyStorageProviderNamesStatics2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPfxImportParameters(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IStandardCertificateStoreNamesStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISubjectAlternativeNameInfo(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISubjectAlternativeNameInfo2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IUserCertificateEnrollmentManager(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IUserCertificateEnrollmentManager2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IUserCertificateStore(pub *mut ::core::ffi::c_void);
#[repr(C)]
pub struct InstallOptions(i32);
#[repr(transparent)]
pub struct KeyAlgorithmNames(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct KeyAttestationHelper(pub *mut ::core::ffi::c_void);
#[repr(C)]
pub struct KeyProtectionLevel(i32);
#[repr(C)]
pub struct KeySize(i32);
#[repr(transparent)]
pub struct KeyStorageProviderNames(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct PfxImportParameters(pub *mut ::core::ffi::c_void);
#[repr(C)]
pub struct SignatureValidationResult(i32);
#[repr(transparent)]
pub struct StandardCertificateStoreNames(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct SubjectAlternativeNameInfo(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct UserCertificateEnrollmentManager(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct UserCertificateStore(pub *mut ::core::ffi::c_void);
