#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct BufferProtectUnprotectResult(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for BufferProtectUnprotectResult {}
impl ::core::clone::Clone for BufferProtectUnprotectResult {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct DataProtectionInfo(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for DataProtectionInfo {}
impl ::core::clone::Clone for DataProtectionInfo {
    fn clone(&self) -> Self {
        *self
    }
}
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
impl ::core::marker::Copy for DataProtectionStatus {}
impl ::core::clone::Clone for DataProtectionStatus {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct EnforcementLevel(pub i32);
impl EnforcementLevel {
    pub const NoProtection: Self = Self(0i32);
    pub const Silent: Self = Self(1i32);
    pub const Override: Self = Self(2i32);
    pub const Block: Self = Self(3i32);
}
impl ::core::marker::Copy for EnforcementLevel {}
impl ::core::clone::Clone for EnforcementLevel {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct FileProtectionInfo(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for FileProtectionInfo {}
impl ::core::clone::Clone for FileProtectionInfo {
    fn clone(&self) -> Self {
        *self
    }
}
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
impl ::core::marker::Copy for FileProtectionStatus {}
impl ::core::clone::Clone for FileProtectionStatus {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct FileUnprotectOptions(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for FileUnprotectOptions {}
impl ::core::clone::Clone for FileUnprotectOptions {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IBufferProtectUnprotectResult(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IBufferProtectUnprotectResult {}
impl ::core::clone::Clone for IBufferProtectUnprotectResult {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IDataProtectionInfo(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IDataProtectionInfo {}
impl ::core::clone::Clone for IDataProtectionInfo {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IDataProtectionManagerStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IDataProtectionManagerStatics {}
impl ::core::clone::Clone for IDataProtectionManagerStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IFileProtectionInfo(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IFileProtectionInfo {}
impl ::core::clone::Clone for IFileProtectionInfo {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IFileProtectionInfo2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IFileProtectionInfo2 {}
impl ::core::clone::Clone for IFileProtectionInfo2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IFileProtectionManagerStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IFileProtectionManagerStatics {}
impl ::core::clone::Clone for IFileProtectionManagerStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IFileProtectionManagerStatics2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IFileProtectionManagerStatics2 {}
impl ::core::clone::Clone for IFileProtectionManagerStatics2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IFileProtectionManagerStatics3(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IFileProtectionManagerStatics3 {}
impl ::core::clone::Clone for IFileProtectionManagerStatics3 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IFileRevocationManagerStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IFileRevocationManagerStatics {}
impl ::core::clone::Clone for IFileRevocationManagerStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IFileUnprotectOptions(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IFileUnprotectOptions {}
impl ::core::clone::Clone for IFileUnprotectOptions {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IFileUnprotectOptionsFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IFileUnprotectOptionsFactory {}
impl ::core::clone::Clone for IFileUnprotectOptionsFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IProtectedAccessResumedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IProtectedAccessResumedEventArgs {}
impl ::core::clone::Clone for IProtectedAccessResumedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IProtectedAccessSuspendingEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IProtectedAccessSuspendingEventArgs {}
impl ::core::clone::Clone for IProtectedAccessSuspendingEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IProtectedContainerExportResult(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IProtectedContainerExportResult {}
impl ::core::clone::Clone for IProtectedContainerExportResult {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IProtectedContainerImportResult(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IProtectedContainerImportResult {}
impl ::core::clone::Clone for IProtectedContainerImportResult {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IProtectedContentRevokedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IProtectedContentRevokedEventArgs {}
impl ::core::clone::Clone for IProtectedContentRevokedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IProtectedFileCreateResult(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IProtectedFileCreateResult {}
impl ::core::clone::Clone for IProtectedFileCreateResult {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IProtectionPolicyAuditInfo(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IProtectionPolicyAuditInfo {}
impl ::core::clone::Clone for IProtectionPolicyAuditInfo {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IProtectionPolicyAuditInfoFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IProtectionPolicyAuditInfoFactory {}
impl ::core::clone::Clone for IProtectionPolicyAuditInfoFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IProtectionPolicyManager(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IProtectionPolicyManager {}
impl ::core::clone::Clone for IProtectionPolicyManager {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IProtectionPolicyManager2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IProtectionPolicyManager2 {}
impl ::core::clone::Clone for IProtectionPolicyManager2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IProtectionPolicyManagerStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IProtectionPolicyManagerStatics {}
impl ::core::clone::Clone for IProtectionPolicyManagerStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IProtectionPolicyManagerStatics2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IProtectionPolicyManagerStatics2 {}
impl ::core::clone::Clone for IProtectionPolicyManagerStatics2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IProtectionPolicyManagerStatics3(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IProtectionPolicyManagerStatics3 {}
impl ::core::clone::Clone for IProtectionPolicyManagerStatics3 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IProtectionPolicyManagerStatics4(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IProtectionPolicyManagerStatics4 {}
impl ::core::clone::Clone for IProtectionPolicyManagerStatics4 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IThreadNetworkContext(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IThreadNetworkContext {}
impl ::core::clone::Clone for IThreadNetworkContext {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ProtectedAccessResumedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ProtectedAccessResumedEventArgs {}
impl ::core::clone::Clone for ProtectedAccessResumedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ProtectedAccessSuspendingEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ProtectedAccessSuspendingEventArgs {}
impl ::core::clone::Clone for ProtectedAccessSuspendingEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ProtectedContainerExportResult(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ProtectedContainerExportResult {}
impl ::core::clone::Clone for ProtectedContainerExportResult {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ProtectedContainerImportResult(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ProtectedContainerImportResult {}
impl ::core::clone::Clone for ProtectedContainerImportResult {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ProtectedContentRevokedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ProtectedContentRevokedEventArgs {}
impl ::core::clone::Clone for ProtectedContentRevokedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ProtectedFileCreateResult(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ProtectedFileCreateResult {}
impl ::core::clone::Clone for ProtectedFileCreateResult {
    fn clone(&self) -> Self {
        *self
    }
}
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
impl ::core::marker::Copy for ProtectedImportExportStatus {}
impl ::core::clone::Clone for ProtectedImportExportStatus {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ProtectionPolicyAuditAction(pub i32);
impl ProtectionPolicyAuditAction {
    pub const Decrypt: Self = Self(0i32);
    pub const CopyToLocation: Self = Self(1i32);
    pub const SendToRecipient: Self = Self(2i32);
    pub const Other: Self = Self(3i32);
}
impl ::core::marker::Copy for ProtectionPolicyAuditAction {}
impl ::core::clone::Clone for ProtectionPolicyAuditAction {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ProtectionPolicyAuditInfo(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ProtectionPolicyAuditInfo {}
impl ::core::clone::Clone for ProtectionPolicyAuditInfo {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ProtectionPolicyEvaluationResult(pub i32);
impl ProtectionPolicyEvaluationResult {
    pub const Allowed: Self = Self(0i32);
    pub const Blocked: Self = Self(1i32);
    pub const ConsentRequired: Self = Self(2i32);
}
impl ::core::marker::Copy for ProtectionPolicyEvaluationResult {}
impl ::core::clone::Clone for ProtectionPolicyEvaluationResult {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ProtectionPolicyManager(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ProtectionPolicyManager {}
impl ::core::clone::Clone for ProtectionPolicyManager {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ProtectionPolicyRequestAccessBehavior(pub i32);
impl ProtectionPolicyRequestAccessBehavior {
    pub const Decrypt: Self = Self(0i32);
    pub const TreatOverridePolicyAsBlock: Self = Self(1i32);
}
impl ::core::marker::Copy for ProtectionPolicyRequestAccessBehavior {}
impl ::core::clone::Clone for ProtectionPolicyRequestAccessBehavior {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ThreadNetworkContext(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ThreadNetworkContext {}
impl ::core::clone::Clone for ThreadNetworkContext {
    fn clone(&self) -> Self {
        *self
    }
}
