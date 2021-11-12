#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct BufferProtectUnprotectResult(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct DataProtectionInfo(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct DataProtectionManager(pub *mut ::core::ffi::c_void);
pub struct DataProtectionStatus(i32);
pub struct EnforcementLevel(i32);
pub struct EnterpriseDataContract(i32);
#[repr(transparent)]
pub struct FileProtectionInfo(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct FileProtectionManager(pub *mut ::core::ffi::c_void);
pub struct FileProtectionStatus(i32);
#[repr(transparent)]
pub struct FileRevocationManager(pub *mut ::core::ffi::c_void);
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
pub struct ProtectedImportExportStatus(i32);
pub struct ProtectionPolicyAuditAction(i32);
#[repr(transparent)]
pub struct ProtectionPolicyAuditInfo(pub *mut ::core::ffi::c_void);
pub struct ProtectionPolicyEvaluationResult(i32);
#[repr(transparent)]
pub struct ProtectionPolicyManager(pub *mut ::core::ffi::c_void);
pub struct ProtectionPolicyRequestAccessBehavior(i32);
#[repr(transparent)]
pub struct ThreadNetworkContext(pub *mut ::core::ffi::c_void);
