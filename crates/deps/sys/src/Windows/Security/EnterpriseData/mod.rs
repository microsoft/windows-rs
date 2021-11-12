#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct BufferProtectUnprotectResult(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct DataProtectionInfo(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct DataProtectionStatus(pub i32);
impl DataProtectionStatus {
    pub const ProtectedToOtherIdentity: DataProtectionStatus = DataProtectionStatus(0i32);
    pub const Protected: DataProtectionStatus = DataProtectionStatus(1i32);
    pub const Revoked: DataProtectionStatus = DataProtectionStatus(2i32);
    pub const Unprotected: DataProtectionStatus = DataProtectionStatus(3i32);
    pub const LicenseExpired: DataProtectionStatus = DataProtectionStatus(4i32);
    pub const AccessSuspended: DataProtectionStatus = DataProtectionStatus(5i32);
}
#[repr(transparent)]
pub struct EnforcementLevel(pub i32);
impl EnforcementLevel {
    pub const NoProtection: EnforcementLevel = EnforcementLevel(0i32);
    pub const Silent: EnforcementLevel = EnforcementLevel(1i32);
    pub const Override: EnforcementLevel = EnforcementLevel(2i32);
    pub const Block: EnforcementLevel = EnforcementLevel(3i32);
}
#[repr(C)]
pub struct EnterpriseDataContract(i32);
#[repr(transparent)]
pub struct FileProtectionInfo(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct FileProtectionStatus(pub i32);
impl FileProtectionStatus {
    pub const Undetermined: FileProtectionStatus = FileProtectionStatus(0i32);
    pub const Unknown: FileProtectionStatus = FileProtectionStatus(0i32);
    pub const Unprotected: FileProtectionStatus = FileProtectionStatus(1i32);
    pub const Revoked: FileProtectionStatus = FileProtectionStatus(2i32);
    pub const Protected: FileProtectionStatus = FileProtectionStatus(3i32);
    pub const ProtectedByOtherUser: FileProtectionStatus = FileProtectionStatus(4i32);
    pub const ProtectedToOtherEnterprise: FileProtectionStatus = FileProtectionStatus(5i32);
    pub const NotProtectable: FileProtectionStatus = FileProtectionStatus(6i32);
    pub const ProtectedToOtherIdentity: FileProtectionStatus = FileProtectionStatus(7i32);
    pub const LicenseExpired: FileProtectionStatus = FileProtectionStatus(8i32);
    pub const AccessSuspended: FileProtectionStatus = FileProtectionStatus(9i32);
    pub const FileInUse: FileProtectionStatus = FileProtectionStatus(10i32);
}
#[repr(transparent)]
pub struct FileUnprotectOptions(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IBufferProtectUnprotectResult(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDataProtectionInfo(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDataProtectionManagerStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IFileProtectionInfo(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IFileProtectionInfo2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IFileProtectionManagerStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IFileProtectionManagerStatics2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IFileProtectionManagerStatics3(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IFileRevocationManagerStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IFileUnprotectOptions(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IFileUnprotectOptionsFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IProtectedAccessResumedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IProtectedAccessSuspendingEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IProtectedContainerExportResult(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IProtectedContainerImportResult(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IProtectedContentRevokedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IProtectedFileCreateResult(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IProtectionPolicyAuditInfo(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IProtectionPolicyAuditInfoFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IProtectionPolicyManager(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IProtectionPolicyManager2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IProtectionPolicyManagerStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IProtectionPolicyManagerStatics2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IProtectionPolicyManagerStatics3(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IProtectionPolicyManagerStatics4(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IThreadNetworkContext(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ProtectedAccessResumedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ProtectedAccessSuspendingEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ProtectedContainerExportResult(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ProtectedContainerImportResult(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ProtectedContentRevokedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ProtectedFileCreateResult(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ProtectedImportExportStatus(pub i32);
impl ProtectedImportExportStatus {
    pub const Ok: ProtectedImportExportStatus = ProtectedImportExportStatus(0i32);
    pub const Undetermined: ProtectedImportExportStatus = ProtectedImportExportStatus(1i32);
    pub const Unprotected: ProtectedImportExportStatus = ProtectedImportExportStatus(2i32);
    pub const Revoked: ProtectedImportExportStatus = ProtectedImportExportStatus(3i32);
    pub const NotRoamable: ProtectedImportExportStatus = ProtectedImportExportStatus(4i32);
    pub const ProtectedToOtherIdentity: ProtectedImportExportStatus = ProtectedImportExportStatus(5i32);
    pub const LicenseExpired: ProtectedImportExportStatus = ProtectedImportExportStatus(6i32);
    pub const AccessSuspended: ProtectedImportExportStatus = ProtectedImportExportStatus(7i32);
}
#[repr(transparent)]
pub struct ProtectionPolicyAuditAction(pub i32);
impl ProtectionPolicyAuditAction {
    pub const Decrypt: ProtectionPolicyAuditAction = ProtectionPolicyAuditAction(0i32);
    pub const CopyToLocation: ProtectionPolicyAuditAction = ProtectionPolicyAuditAction(1i32);
    pub const SendToRecipient: ProtectionPolicyAuditAction = ProtectionPolicyAuditAction(2i32);
    pub const Other: ProtectionPolicyAuditAction = ProtectionPolicyAuditAction(3i32);
}
#[repr(transparent)]
pub struct ProtectionPolicyAuditInfo(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ProtectionPolicyEvaluationResult(pub i32);
impl ProtectionPolicyEvaluationResult {
    pub const Allowed: ProtectionPolicyEvaluationResult = ProtectionPolicyEvaluationResult(0i32);
    pub const Blocked: ProtectionPolicyEvaluationResult = ProtectionPolicyEvaluationResult(1i32);
    pub const ConsentRequired: ProtectionPolicyEvaluationResult = ProtectionPolicyEvaluationResult(2i32);
}
#[repr(transparent)]
pub struct ProtectionPolicyManager(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ProtectionPolicyRequestAccessBehavior(pub i32);
impl ProtectionPolicyRequestAccessBehavior {
    pub const Decrypt: ProtectionPolicyRequestAccessBehavior = ProtectionPolicyRequestAccessBehavior(0i32);
    pub const TreatOverridePolicyAsBlock: ProtectionPolicyRequestAccessBehavior = ProtectionPolicyRequestAccessBehavior(1i32);
}
#[repr(transparent)]
pub struct ThreadNetworkContext(pub *mut ::core::ffi::c_void);
