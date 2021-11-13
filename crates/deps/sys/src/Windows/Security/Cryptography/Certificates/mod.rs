#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct Certificate(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for Certificate {}
impl ::core::clone::Clone for Certificate {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct CertificateChain(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for CertificateChain {}
impl ::core::clone::Clone for CertificateChain {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct CertificateChainPolicy(pub i32);
impl CertificateChainPolicy {
    pub const Base: Self = Self(0i32);
    pub const Ssl: Self = Self(1i32);
    pub const NTAuthentication: Self = Self(2i32);
    pub const MicrosoftRoot: Self = Self(3i32);
}
impl ::core::marker::Copy for CertificateChainPolicy {}
impl ::core::clone::Clone for CertificateChainPolicy {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct CertificateExtension(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for CertificateExtension {}
impl ::core::clone::Clone for CertificateExtension {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct CertificateKeyUsages(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for CertificateKeyUsages {}
impl ::core::clone::Clone for CertificateKeyUsages {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct CertificateQuery(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for CertificateQuery {}
impl ::core::clone::Clone for CertificateQuery {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct CertificateRequestProperties(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for CertificateRequestProperties {}
impl ::core::clone::Clone for CertificateRequestProperties {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct CertificateStore(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for CertificateStore {}
impl ::core::clone::Clone for CertificateStore {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ChainBuildingParameters(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ChainBuildingParameters {}
impl ::core::clone::Clone for ChainBuildingParameters {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ChainValidationParameters(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ChainValidationParameters {}
impl ::core::clone::Clone for ChainValidationParameters {
    fn clone(&self) -> Self {
        *self
    }
}
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
impl ::core::marker::Copy for ChainValidationResult {}
impl ::core::clone::Clone for ChainValidationResult {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct CmsAttachedSignature(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for CmsAttachedSignature {}
impl ::core::clone::Clone for CmsAttachedSignature {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct CmsDetachedSignature(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for CmsDetachedSignature {}
impl ::core::clone::Clone for CmsDetachedSignature {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct CmsSignerInfo(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for CmsSignerInfo {}
impl ::core::clone::Clone for CmsSignerInfo {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct CmsTimestampInfo(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for CmsTimestampInfo {}
impl ::core::clone::Clone for CmsTimestampInfo {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct EnrollKeyUsages(pub u32);
impl EnrollKeyUsages {
    pub const None: Self = Self(0u32);
    pub const Decryption: Self = Self(1u32);
    pub const Signing: Self = Self(2u32);
    pub const KeyAgreement: Self = Self(4u32);
    pub const All: Self = Self(16777215u32);
}
impl ::core::marker::Copy for EnrollKeyUsages {}
impl ::core::clone::Clone for EnrollKeyUsages {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ExportOption(pub i32);
impl ExportOption {
    pub const NotExportable: Self = Self(0i32);
    pub const Exportable: Self = Self(1i32);
}
impl ::core::marker::Copy for ExportOption {}
impl ::core::clone::Clone for ExportOption {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ICertificate(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ICertificate {}
impl ::core::clone::Clone for ICertificate {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ICertificate2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ICertificate2 {}
impl ::core::clone::Clone for ICertificate2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ICertificate3(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ICertificate3 {}
impl ::core::clone::Clone for ICertificate3 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ICertificateChain(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ICertificateChain {}
impl ::core::clone::Clone for ICertificateChain {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ICertificateEnrollmentManagerStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ICertificateEnrollmentManagerStatics {}
impl ::core::clone::Clone for ICertificateEnrollmentManagerStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ICertificateEnrollmentManagerStatics2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ICertificateEnrollmentManagerStatics2 {}
impl ::core::clone::Clone for ICertificateEnrollmentManagerStatics2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ICertificateEnrollmentManagerStatics3(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ICertificateEnrollmentManagerStatics3 {}
impl ::core::clone::Clone for ICertificateEnrollmentManagerStatics3 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ICertificateExtension(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ICertificateExtension {}
impl ::core::clone::Clone for ICertificateExtension {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ICertificateFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ICertificateFactory {}
impl ::core::clone::Clone for ICertificateFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ICertificateKeyUsages(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ICertificateKeyUsages {}
impl ::core::clone::Clone for ICertificateKeyUsages {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ICertificateQuery(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ICertificateQuery {}
impl ::core::clone::Clone for ICertificateQuery {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ICertificateQuery2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ICertificateQuery2 {}
impl ::core::clone::Clone for ICertificateQuery2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ICertificateRequestProperties(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ICertificateRequestProperties {}
impl ::core::clone::Clone for ICertificateRequestProperties {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ICertificateRequestProperties2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ICertificateRequestProperties2 {}
impl ::core::clone::Clone for ICertificateRequestProperties2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ICertificateRequestProperties3(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ICertificateRequestProperties3 {}
impl ::core::clone::Clone for ICertificateRequestProperties3 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ICertificateRequestProperties4(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ICertificateRequestProperties4 {}
impl ::core::clone::Clone for ICertificateRequestProperties4 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ICertificateStore(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ICertificateStore {}
impl ::core::clone::Clone for ICertificateStore {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ICertificateStore2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ICertificateStore2 {}
impl ::core::clone::Clone for ICertificateStore2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ICertificateStoresStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ICertificateStoresStatics {}
impl ::core::clone::Clone for ICertificateStoresStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ICertificateStoresStatics2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ICertificateStoresStatics2 {}
impl ::core::clone::Clone for ICertificateStoresStatics2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IChainBuildingParameters(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IChainBuildingParameters {}
impl ::core::clone::Clone for IChainBuildingParameters {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IChainValidationParameters(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IChainValidationParameters {}
impl ::core::clone::Clone for IChainValidationParameters {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ICmsAttachedSignature(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ICmsAttachedSignature {}
impl ::core::clone::Clone for ICmsAttachedSignature {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ICmsAttachedSignatureFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ICmsAttachedSignatureFactory {}
impl ::core::clone::Clone for ICmsAttachedSignatureFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ICmsAttachedSignatureStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ICmsAttachedSignatureStatics {}
impl ::core::clone::Clone for ICmsAttachedSignatureStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ICmsDetachedSignature(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ICmsDetachedSignature {}
impl ::core::clone::Clone for ICmsDetachedSignature {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ICmsDetachedSignatureFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ICmsDetachedSignatureFactory {}
impl ::core::clone::Clone for ICmsDetachedSignatureFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ICmsDetachedSignatureStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ICmsDetachedSignatureStatics {}
impl ::core::clone::Clone for ICmsDetachedSignatureStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ICmsSignerInfo(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ICmsSignerInfo {}
impl ::core::clone::Clone for ICmsSignerInfo {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ICmsTimestampInfo(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ICmsTimestampInfo {}
impl ::core::clone::Clone for ICmsTimestampInfo {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IKeyAlgorithmNamesStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IKeyAlgorithmNamesStatics {}
impl ::core::clone::Clone for IKeyAlgorithmNamesStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IKeyAlgorithmNamesStatics2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IKeyAlgorithmNamesStatics2 {}
impl ::core::clone::Clone for IKeyAlgorithmNamesStatics2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IKeyAttestationHelperStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IKeyAttestationHelperStatics {}
impl ::core::clone::Clone for IKeyAttestationHelperStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IKeyAttestationHelperStatics2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IKeyAttestationHelperStatics2 {}
impl ::core::clone::Clone for IKeyAttestationHelperStatics2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IKeyStorageProviderNamesStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IKeyStorageProviderNamesStatics {}
impl ::core::clone::Clone for IKeyStorageProviderNamesStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IKeyStorageProviderNamesStatics2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IKeyStorageProviderNamesStatics2 {}
impl ::core::clone::Clone for IKeyStorageProviderNamesStatics2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IPfxImportParameters(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IPfxImportParameters {}
impl ::core::clone::Clone for IPfxImportParameters {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IStandardCertificateStoreNamesStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IStandardCertificateStoreNamesStatics {}
impl ::core::clone::Clone for IStandardCertificateStoreNamesStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISubjectAlternativeNameInfo(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISubjectAlternativeNameInfo {}
impl ::core::clone::Clone for ISubjectAlternativeNameInfo {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISubjectAlternativeNameInfo2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISubjectAlternativeNameInfo2 {}
impl ::core::clone::Clone for ISubjectAlternativeNameInfo2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IUserCertificateEnrollmentManager(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IUserCertificateEnrollmentManager {}
impl ::core::clone::Clone for IUserCertificateEnrollmentManager {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IUserCertificateEnrollmentManager2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IUserCertificateEnrollmentManager2 {}
impl ::core::clone::Clone for IUserCertificateEnrollmentManager2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IUserCertificateStore(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IUserCertificateStore {}
impl ::core::clone::Clone for IUserCertificateStore {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct InstallOptions(pub u32);
impl InstallOptions {
    pub const None: Self = Self(0u32);
    pub const DeleteExpired: Self = Self(1u32);
}
impl ::core::marker::Copy for InstallOptions {}
impl ::core::clone::Clone for InstallOptions {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct KeyProtectionLevel(pub i32);
impl KeyProtectionLevel {
    pub const NoConsent: Self = Self(0i32);
    pub const ConsentOnly: Self = Self(1i32);
    pub const ConsentWithPassword: Self = Self(2i32);
    pub const ConsentWithFingerprint: Self = Self(3i32);
}
impl ::core::marker::Copy for KeyProtectionLevel {}
impl ::core::clone::Clone for KeyProtectionLevel {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct KeySize(pub i32);
impl KeySize {
    pub const Invalid: Self = Self(0i32);
    pub const Rsa2048: Self = Self(2048i32);
    pub const Rsa4096: Self = Self(4096i32);
}
impl ::core::marker::Copy for KeySize {}
impl ::core::clone::Clone for KeySize {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct PfxImportParameters(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for PfxImportParameters {}
impl ::core::clone::Clone for PfxImportParameters {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct SignatureValidationResult(pub i32);
impl SignatureValidationResult {
    pub const Success: Self = Self(0i32);
    pub const InvalidParameter: Self = Self(1i32);
    pub const BadMessage: Self = Self(2i32);
    pub const InvalidSignature: Self = Self(3i32);
    pub const OtherErrors: Self = Self(4i32);
}
impl ::core::marker::Copy for SignatureValidationResult {}
impl ::core::clone::Clone for SignatureValidationResult {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct SubjectAlternativeNameInfo(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for SubjectAlternativeNameInfo {}
impl ::core::clone::Clone for SubjectAlternativeNameInfo {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct UserCertificateEnrollmentManager(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for UserCertificateEnrollmentManager {}
impl ::core::clone::Clone for UserCertificateEnrollmentManager {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct UserCertificateStore(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for UserCertificateStore {}
impl ::core::clone::Clone for UserCertificateStore {
    fn clone(&self) -> Self {
        *self
    }
}
