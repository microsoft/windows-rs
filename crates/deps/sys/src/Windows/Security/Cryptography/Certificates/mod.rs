#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct Certificate(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct CertificateChain(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct CertificateChainPolicy(pub i32);
impl CertificateChainPolicy {
    pub const Base: Self = Self(0i32);
    pub const Ssl: Self = Self(1i32);
    pub const NTAuthentication: Self = Self(2i32);
    pub const MicrosoftRoot: Self = Self(3i32);
}
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
pub struct ChainBuildingParameters(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ChainValidationParameters(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ChainValidationResult(pub i32);
impl ChainValidationResult {
    pub const Success: Self = Self(0i32);
    pub const Untrusted: Self = Self(1i32);
    pub const Revoked: Self = Self(2i32);
    pub const Expired: Self = Self(3i32);
    pub const IncompleteChain: Self = Self(4i32);
    pub const InvalidSignature: Self = Self(5i32);
    pub const WrongUsage: Self = Self(6i32);
    pub const InvalidName: Self = Self(7i32);
    pub const InvalidCertificateAuthorityPolicy: Self = Self(8i32);
    pub const BasicConstraintsError: Self = Self(9i32);
    pub const UnknownCriticalExtension: Self = Self(10i32);
    pub const RevocationInformationMissing: Self = Self(11i32);
    pub const RevocationFailure: Self = Self(12i32);
    pub const OtherErrors: Self = Self(13i32);
}
#[repr(transparent)]
pub struct CmsAttachedSignature(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct CmsDetachedSignature(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct CmsSignerInfo(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct CmsTimestampInfo(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct EnrollKeyUsages(pub u32);
impl EnrollKeyUsages {
    pub const None: Self = Self(0u32);
    pub const Decryption: Self = Self(1u32);
    pub const Signing: Self = Self(2u32);
    pub const KeyAgreement: Self = Self(4u32);
    pub const All: Self = Self(16777215u32);
}
#[repr(transparent)]
pub struct ExportOption(pub i32);
impl ExportOption {
    pub const NotExportable: Self = Self(0i32);
    pub const Exportable: Self = Self(1i32);
}
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
#[repr(transparent)]
pub struct InstallOptions(pub u32);
impl InstallOptions {
    pub const None: Self = Self(0u32);
    pub const DeleteExpired: Self = Self(1u32);
}
#[repr(transparent)]
pub struct KeyProtectionLevel(pub i32);
impl KeyProtectionLevel {
    pub const NoConsent: Self = Self(0i32);
    pub const ConsentOnly: Self = Self(1i32);
    pub const ConsentWithPassword: Self = Self(2i32);
    pub const ConsentWithFingerprint: Self = Self(3i32);
}
#[repr(transparent)]
pub struct KeySize(pub i32);
impl KeySize {
    pub const Invalid: Self = Self(0i32);
    pub const Rsa2048: Self = Self(2048i32);
    pub const Rsa4096: Self = Self(4096i32);
}
#[repr(transparent)]
pub struct PfxImportParameters(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct SignatureValidationResult(pub i32);
impl SignatureValidationResult {
    pub const Success: Self = Self(0i32);
    pub const InvalidParameter: Self = Self(1i32);
    pub const BadMessage: Self = Self(2i32);
    pub const InvalidSignature: Self = Self(3i32);
    pub const OtherErrors: Self = Self(4i32);
}
#[repr(transparent)]
pub struct SubjectAlternativeNameInfo(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct UserCertificateEnrollmentManager(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct UserCertificateStore(pub *mut ::core::ffi::c_void);
