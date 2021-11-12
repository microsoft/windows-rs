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
    pub const Base: CertificateChainPolicy = CertificateChainPolicy(0i32);
    pub const Ssl: CertificateChainPolicy = CertificateChainPolicy(1i32);
    pub const NTAuthentication: CertificateChainPolicy = CertificateChainPolicy(2i32);
    pub const MicrosoftRoot: CertificateChainPolicy = CertificateChainPolicy(3i32);
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
    pub const Success: ChainValidationResult = ChainValidationResult(0i32);
    pub const Untrusted: ChainValidationResult = ChainValidationResult(1i32);
    pub const Revoked: ChainValidationResult = ChainValidationResult(2i32);
    pub const Expired: ChainValidationResult = ChainValidationResult(3i32);
    pub const IncompleteChain: ChainValidationResult = ChainValidationResult(4i32);
    pub const InvalidSignature: ChainValidationResult = ChainValidationResult(5i32);
    pub const WrongUsage: ChainValidationResult = ChainValidationResult(6i32);
    pub const InvalidName: ChainValidationResult = ChainValidationResult(7i32);
    pub const InvalidCertificateAuthorityPolicy: ChainValidationResult = ChainValidationResult(8i32);
    pub const BasicConstraintsError: ChainValidationResult = ChainValidationResult(9i32);
    pub const UnknownCriticalExtension: ChainValidationResult = ChainValidationResult(10i32);
    pub const RevocationInformationMissing: ChainValidationResult = ChainValidationResult(11i32);
    pub const RevocationFailure: ChainValidationResult = ChainValidationResult(12i32);
    pub const OtherErrors: ChainValidationResult = ChainValidationResult(13i32);
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
    pub const None: EnrollKeyUsages = EnrollKeyUsages(0u32);
    pub const Decryption: EnrollKeyUsages = EnrollKeyUsages(1u32);
    pub const Signing: EnrollKeyUsages = EnrollKeyUsages(2u32);
    pub const KeyAgreement: EnrollKeyUsages = EnrollKeyUsages(4u32);
    pub const All: EnrollKeyUsages = EnrollKeyUsages(16777215u32);
}
#[repr(transparent)]
pub struct ExportOption(pub i32);
impl ExportOption {
    pub const NotExportable: ExportOption = ExportOption(0i32);
    pub const Exportable: ExportOption = ExportOption(1i32);
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
    pub const None: InstallOptions = InstallOptions(0u32);
    pub const DeleteExpired: InstallOptions = InstallOptions(1u32);
}
#[repr(transparent)]
pub struct KeyProtectionLevel(pub i32);
impl KeyProtectionLevel {
    pub const NoConsent: KeyProtectionLevel = KeyProtectionLevel(0i32);
    pub const ConsentOnly: KeyProtectionLevel = KeyProtectionLevel(1i32);
    pub const ConsentWithPassword: KeyProtectionLevel = KeyProtectionLevel(2i32);
    pub const ConsentWithFingerprint: KeyProtectionLevel = KeyProtectionLevel(3i32);
}
#[repr(transparent)]
pub struct KeySize(pub i32);
impl KeySize {
    pub const Invalid: KeySize = KeySize(0i32);
    pub const Rsa2048: KeySize = KeySize(2048i32);
    pub const Rsa4096: KeySize = KeySize(4096i32);
}
#[repr(transparent)]
pub struct PfxImportParameters(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct SignatureValidationResult(pub i32);
impl SignatureValidationResult {
    pub const Success: SignatureValidationResult = SignatureValidationResult(0i32);
    pub const InvalidParameter: SignatureValidationResult = SignatureValidationResult(1i32);
    pub const BadMessage: SignatureValidationResult = SignatureValidationResult(2i32);
    pub const InvalidSignature: SignatureValidationResult = SignatureValidationResult(3i32);
    pub const OtherErrors: SignatureValidationResult = SignatureValidationResult(4i32);
}
#[repr(transparent)]
pub struct SubjectAlternativeNameInfo(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct UserCertificateEnrollmentManager(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct UserCertificateStore(pub *mut ::core::ffi::c_void);
