#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[link(name = "windows")]
extern "system" {
    #[cfg(feature = "Win32_System_WindowsProgramming")]
    pub fn FhServiceBlockBackup(pipe: super::super::System::WindowsProgramming::FH_SERVICE_PIPE_HANDLE) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_System_WindowsProgramming")]
    pub fn FhServiceClosePipe(pipe: super::super::System::WindowsProgramming::FH_SERVICE_PIPE_HANDLE) -> ::windows_sys::core::HRESULT;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_WindowsProgramming"))]
    pub fn FhServiceOpenPipe(startserviceifstopped: super::super::Foundation::BOOL, pipe: *mut super::super::System::WindowsProgramming::FH_SERVICE_PIPE_HANDLE) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_System_WindowsProgramming")]
    pub fn FhServiceReloadConfiguration(pipe: super::super::System::WindowsProgramming::FH_SERVICE_PIPE_HANDLE) -> ::windows_sys::core::HRESULT;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_WindowsProgramming"))]
    pub fn FhServiceStartBackup(pipe: super::super::System::WindowsProgramming::FH_SERVICE_PIPE_HANDLE, lowpriorityio: super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_WindowsProgramming"))]
    pub fn FhServiceStopBackup(pipe: super::super::System::WindowsProgramming::FH_SERVICE_PIPE_HANDLE, stoptracking: super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_System_WindowsProgramming")]
    pub fn FhServiceUnblockBackup(pipe: super::super::System::WindowsProgramming::FH_SERVICE_PIPE_HANDLE) -> ::windows_sys::core::HRESULT;
}
pub const FHCFG_E_CONFIGURATION_PREVIOUSLY_LOADED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147220731i32 as _);
pub const FHCFG_E_CONFIG_ALREADY_EXISTS: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147220734i32 as _);
pub const FHCFG_E_CONFIG_FILE_NOT_FOUND: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147220735i32 as _);
pub const FHCFG_E_CORRUPT_CONFIG_FILE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147220736i32 as _);
pub const FHCFG_E_INVALID_REHYDRATION_STATE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147220726i32 as _);
pub const FHCFG_E_LEGACY_BACKUP_NOT_FOUND: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147220715i32 as _);
pub const FHCFG_E_LEGACY_BACKUP_USER_EXCLUDED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147220716i32 as _);
pub const FHCFG_E_LEGACY_TARGET_UNSUPPORTED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147220718i32 as _);
pub const FHCFG_E_LEGACY_TARGET_VALIDATION_UNSUPPORTED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147220717i32 as _);
pub const FHCFG_E_NO_VALID_CONFIGURATION_LOADED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147220733i32 as _);
pub const FHCFG_E_RECOMMENDATION_CHANGE_NOT_ALLOWED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147220720i32 as _);
pub const FHCFG_E_TARGET_CANNOT_BE_USED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147220727i32 as _);
pub const FHCFG_E_TARGET_NOT_CONFIGURED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147220729i32 as _);
pub const FHCFG_E_TARGET_NOT_CONNECTED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147220732i32 as _);
pub const FHCFG_E_TARGET_NOT_ENOUGH_FREE_SPACE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147220728i32 as _);
pub const FHCFG_E_TARGET_REHYDRATED_ELSEWHERE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147220719i32 as _);
pub const FHCFG_E_TARGET_VERIFICATION_FAILED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147220730i32 as _);
pub const FHSVC_E_BACKUP_BLOCKED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147219968i32 as _);
pub const FHSVC_E_CONFIG_DISABLED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147219966i32 as _);
pub const FHSVC_E_CONFIG_DISABLED_GP: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147219965i32 as _);
pub const FHSVC_E_CONFIG_REHYDRATING: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147219963i32 as _);
pub const FHSVC_E_FATAL_CONFIG_ERROR: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147219964i32 as _);
pub const FHSVC_E_NOT_CONFIGURED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147219967i32 as _);
#[repr(transparent)]
pub struct FH_BACKUP_STATUS(pub i32);
pub const FH_STATUS_DISABLED: FH_BACKUP_STATUS = FH_BACKUP_STATUS(0i32);
pub const FH_STATUS_DISABLED_BY_GP: FH_BACKUP_STATUS = FH_BACKUP_STATUS(1i32);
pub const FH_STATUS_ENABLED: FH_BACKUP_STATUS = FH_BACKUP_STATUS(2i32);
pub const FH_STATUS_REHYDRATING: FH_BACKUP_STATUS = FH_BACKUP_STATUS(3i32);
pub const MAX_BACKUP_STATUS: FH_BACKUP_STATUS = FH_BACKUP_STATUS(4i32);
impl ::core::marker::Copy for FH_BACKUP_STATUS {}
impl ::core::clone::Clone for FH_BACKUP_STATUS {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct FH_DEVICE_VALIDATION_RESULT(pub i32);
pub const FH_ACCESS_DENIED: FH_DEVICE_VALIDATION_RESULT = FH_DEVICE_VALIDATION_RESULT(0i32);
pub const FH_INVALID_DRIVE_TYPE: FH_DEVICE_VALIDATION_RESULT = FH_DEVICE_VALIDATION_RESULT(1i32);
pub const FH_READ_ONLY_PERMISSION: FH_DEVICE_VALIDATION_RESULT = FH_DEVICE_VALIDATION_RESULT(2i32);
pub const FH_CURRENT_DEFAULT: FH_DEVICE_VALIDATION_RESULT = FH_DEVICE_VALIDATION_RESULT(3i32);
pub const FH_NAMESPACE_EXISTS: FH_DEVICE_VALIDATION_RESULT = FH_DEVICE_VALIDATION_RESULT(4i32);
pub const FH_TARGET_PART_OF_LIBRARY: FH_DEVICE_VALIDATION_RESULT = FH_DEVICE_VALIDATION_RESULT(5i32);
pub const FH_VALID_TARGET: FH_DEVICE_VALIDATION_RESULT = FH_DEVICE_VALIDATION_RESULT(6i32);
pub const MAX_VALIDATION_RESULT: FH_DEVICE_VALIDATION_RESULT = FH_DEVICE_VALIDATION_RESULT(7i32);
impl ::core::marker::Copy for FH_DEVICE_VALIDATION_RESULT {}
impl ::core::clone::Clone for FH_DEVICE_VALIDATION_RESULT {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct FH_LOCAL_POLICY_TYPE(pub i32);
pub const FH_FREQUENCY: FH_LOCAL_POLICY_TYPE = FH_LOCAL_POLICY_TYPE(0i32);
pub const FH_RETENTION_TYPE: FH_LOCAL_POLICY_TYPE = FH_LOCAL_POLICY_TYPE(1i32);
pub const FH_RETENTION_AGE: FH_LOCAL_POLICY_TYPE = FH_LOCAL_POLICY_TYPE(2i32);
pub const MAX_LOCAL_POLICY: FH_LOCAL_POLICY_TYPE = FH_LOCAL_POLICY_TYPE(3i32);
impl ::core::marker::Copy for FH_LOCAL_POLICY_TYPE {}
impl ::core::clone::Clone for FH_LOCAL_POLICY_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct FH_PROTECTED_ITEM_CATEGORY(pub i32);
pub const FH_FOLDER: FH_PROTECTED_ITEM_CATEGORY = FH_PROTECTED_ITEM_CATEGORY(0i32);
pub const FH_LIBRARY: FH_PROTECTED_ITEM_CATEGORY = FH_PROTECTED_ITEM_CATEGORY(1i32);
pub const MAX_PROTECTED_ITEM_CATEGORY: FH_PROTECTED_ITEM_CATEGORY = FH_PROTECTED_ITEM_CATEGORY(2i32);
impl ::core::marker::Copy for FH_PROTECTED_ITEM_CATEGORY {}
impl ::core::clone::Clone for FH_PROTECTED_ITEM_CATEGORY {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct FH_RETENTION_TYPES(pub i32);
pub const FH_RETENTION_DISABLED: FH_RETENTION_TYPES = FH_RETENTION_TYPES(0i32);
pub const FH_RETENTION_UNLIMITED: FH_RETENTION_TYPES = FH_RETENTION_TYPES(1i32);
pub const FH_RETENTION_AGE_BASED: FH_RETENTION_TYPES = FH_RETENTION_TYPES(2i32);
pub const MAX_RETENTION_TYPE: FH_RETENTION_TYPES = FH_RETENTION_TYPES(3i32);
impl ::core::marker::Copy for FH_RETENTION_TYPES {}
impl ::core::clone::Clone for FH_RETENTION_TYPES {
    fn clone(&self) -> Self {
        *self
    }
}
pub const FH_STATE_BACKUP_NOT_SUPPORTED: u32 = 2064u32;
pub const FH_STATE_DISABLED_BY_GP: u32 = 2u32;
pub const FH_STATE_FATAL_CONFIG_ERROR: u32 = 3u32;
pub const FH_STATE_MIGRATING: u32 = 4u32;
pub const FH_STATE_NOT_TRACKED: u32 = 0u32;
pub const FH_STATE_NO_ERROR: u32 = 255u32;
pub const FH_STATE_OFF: u32 = 1u32;
pub const FH_STATE_REHYDRATING: u32 = 5u32;
pub const FH_STATE_RUNNING: u32 = 256u32;
pub const FH_STATE_STAGING_FULL: u32 = 18u32;
pub const FH_STATE_TARGET_ABSENT: u32 = 21u32;
pub const FH_STATE_TARGET_ACCESS_DENIED: u32 = 14u32;
pub const FH_STATE_TARGET_FS_LIMITATION: u32 = 13u32;
pub const FH_STATE_TARGET_FULL: u32 = 17u32;
pub const FH_STATE_TARGET_FULL_RETENTION_MAX: u32 = 16u32;
pub const FH_STATE_TARGET_LOW_SPACE: u32 = 20u32;
pub const FH_STATE_TARGET_LOW_SPACE_RETENTION_MAX: u32 = 19u32;
pub const FH_STATE_TARGET_VOLUME_DIRTY: u32 = 15u32;
pub const FH_STATE_TOO_MUCH_BEHIND: u32 = 240u32;
#[repr(transparent)]
pub struct FH_TARGET_DRIVE_TYPES(pub i32);
pub const FH_DRIVE_UNKNOWN: FH_TARGET_DRIVE_TYPES = FH_TARGET_DRIVE_TYPES(0i32);
pub const FH_DRIVE_REMOVABLE: FH_TARGET_DRIVE_TYPES = FH_TARGET_DRIVE_TYPES(2i32);
pub const FH_DRIVE_FIXED: FH_TARGET_DRIVE_TYPES = FH_TARGET_DRIVE_TYPES(3i32);
pub const FH_DRIVE_REMOTE: FH_TARGET_DRIVE_TYPES = FH_TARGET_DRIVE_TYPES(4i32);
impl ::core::marker::Copy for FH_TARGET_DRIVE_TYPES {}
impl ::core::clone::Clone for FH_TARGET_DRIVE_TYPES {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct FH_TARGET_PROPERTY_TYPE(pub i32);
pub const FH_TARGET_NAME: FH_TARGET_PROPERTY_TYPE = FH_TARGET_PROPERTY_TYPE(0i32);
pub const FH_TARGET_URL: FH_TARGET_PROPERTY_TYPE = FH_TARGET_PROPERTY_TYPE(1i32);
pub const FH_TARGET_DRIVE_TYPE: FH_TARGET_PROPERTY_TYPE = FH_TARGET_PROPERTY_TYPE(2i32);
pub const MAX_TARGET_PROPERTY: FH_TARGET_PROPERTY_TYPE = FH_TARGET_PROPERTY_TYPE(3i32);
impl ::core::marker::Copy for FH_TARGET_PROPERTY_TYPE {}
impl ::core::clone::Clone for FH_TARGET_PROPERTY_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct FhBackupStopReason(pub i32);
pub const BackupInvalidStopReason: FhBackupStopReason = FhBackupStopReason(0i32);
pub const BackupLimitUserBusyMachineOnAC: FhBackupStopReason = FhBackupStopReason(1i32);
pub const BackupLimitUserIdleMachineOnDC: FhBackupStopReason = FhBackupStopReason(2i32);
pub const BackupLimitUserBusyMachineOnDC: FhBackupStopReason = FhBackupStopReason(3i32);
pub const BackupCancelled: FhBackupStopReason = FhBackupStopReason(4i32);
impl ::core::marker::Copy for FhBackupStopReason {}
impl ::core::clone::Clone for FhBackupStopReason {
    fn clone(&self) -> Self {
        *self
    }
}
pub const FhConfigMgr: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 3980639036, data2: 2537, data3: 18826, data4: [157, 246, 33, 119, 36, 76, 109, 180] };
pub const FhReassociation: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 1299353141, data2: 5882, data3: 17184, data4: [158, 139, 191, 215, 16, 10, 136, 70] };
#[repr(transparent)]
pub struct IFhConfigMgr(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IFhReassociation(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IFhScopeIterator(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IFhTarget(pub *mut ::core::ffi::c_void);
