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
    pub const ProtectedToOtherIdentity: Self = Self(0i32);
    pub const Protected: Self = Self(1i32);
    pub const Revoked: Self = Self(2i32);
    pub const Unprotected: Self = Self(3i32);
    pub const LicenseExpired: Self = Self(4i32);
    pub const AccessSuspended: Self = Self(5i32);
}
#[repr(transparent)]
pub struct EnforcementLevel(pub i32);
impl EnforcementLevel {
    pub const NoProtection: Self = Self(0i32);
    pub const Silent: Self = Self(1i32);
    pub const Override: Self = Self(2i32);
    pub const Block: Self = Self(3i32);
}
#[repr(transparent)]
pub struct FileProtectionInfo(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct FileProtectionStatus(pub i32);
impl FileProtectionStatus {
    pub const Undetermined: Self = Self(0i32);
    pub const Unknown: Self = Self(0i32);
    pub const Unprotected: Self = Self(1i32);
    pub const Revoked: Self = Self(2i32);
    pub const Protected: Self = Self(3i32);
    pub const ProtectedByOtherUser: Self = Self(4i32);
    pub const ProtectedToOtherEnterprise: Self = Self(5i32);
    pub const NotProtectable: Self = Self(6i32);
    pub const ProtectedToOtherIdentity: Self = Self(7i32);
    pub const LicenseExpired: Self = Self(8i32);
    pub const AccessSuspended: Self = Self(9i32);
    pub const FileInUse: Self = Self(10i32);
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
    pub const Ok: Self = Self(0i32);
    pub const Undetermined: Self = Self(1i32);
    pub const Unprotected: Self = Self(2i32);
    pub const Revoked: Self = Self(3i32);
    pub const NotRoamable: Self = Self(4i32);
    pub const ProtectedToOtherIdentity: Self = Self(5i32);
    pub const LicenseExpired: Self = Self(6i32);
    pub const AccessSuspended: Self = Self(7i32);
}
#[repr(transparent)]
pub struct ProtectionPolicyAuditAction(pub i32);
impl ProtectionPolicyAuditAction {
    pub const Decrypt: Self = Self(0i32);
    pub const CopyToLocation: Self = Self(1i32);
    pub const SendToRecipient: Self = Self(2i32);
    pub const Other: Self = Self(3i32);
}
#[repr(transparent)]
pub struct ProtectionPolicyAuditInfo(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ProtectionPolicyEvaluationResult(pub i32);
impl ProtectionPolicyEvaluationResult {
    pub const Allowed: Self = Self(0i32);
    pub const Blocked: Self = Self(1i32);
    pub const ConsentRequired: Self = Self(2i32);
}
#[repr(transparent)]
pub struct ProtectionPolicyManager(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ProtectionPolicyRequestAccessBehavior(pub i32);
impl ProtectionPolicyRequestAccessBehavior {
    pub const Decrypt: Self = Self(0i32);
    pub const TreatOverridePolicyAsBlock: Self = Self(1i32);
}
#[repr(transparent)]
pub struct ThreadNetworkContext(pub *mut ::core::ffi::c_void);
