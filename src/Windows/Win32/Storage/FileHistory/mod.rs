#![allow(unused_variables, non_upper_case_globals, non_snake_case, unused_unsafe, non_camel_case_types, dead_code, clippy::all)]
#[doc = "*Required features: `Win32_Storage_FileHistory`*"]
pub const FHCFG_E_CONFIGURATION_PREVIOUSLY_LOADED: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147220731i32 as _);
#[doc = "*Required features: `Win32_Storage_FileHistory`*"]
pub const FHCFG_E_CONFIG_ALREADY_EXISTS: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147220734i32 as _);
#[doc = "*Required features: `Win32_Storage_FileHistory`*"]
pub const FHCFG_E_CONFIG_FILE_NOT_FOUND: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147220735i32 as _);
#[doc = "*Required features: `Win32_Storage_FileHistory`*"]
pub const FHCFG_E_CORRUPT_CONFIG_FILE: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147220736i32 as _);
#[doc = "*Required features: `Win32_Storage_FileHistory`*"]
pub const FHCFG_E_INVALID_REHYDRATION_STATE: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147220726i32 as _);
#[doc = "*Required features: `Win32_Storage_FileHistory`*"]
pub const FHCFG_E_LEGACY_BACKUP_NOT_FOUND: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147220715i32 as _);
#[doc = "*Required features: `Win32_Storage_FileHistory`*"]
pub const FHCFG_E_LEGACY_BACKUP_USER_EXCLUDED: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147220716i32 as _);
#[doc = "*Required features: `Win32_Storage_FileHistory`*"]
pub const FHCFG_E_LEGACY_TARGET_UNSUPPORTED: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147220718i32 as _);
#[doc = "*Required features: `Win32_Storage_FileHistory`*"]
pub const FHCFG_E_LEGACY_TARGET_VALIDATION_UNSUPPORTED: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147220717i32 as _);
#[doc = "*Required features: `Win32_Storage_FileHistory`*"]
pub const FHCFG_E_NO_VALID_CONFIGURATION_LOADED: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147220733i32 as _);
#[doc = "*Required features: `Win32_Storage_FileHistory`*"]
pub const FHCFG_E_RECOMMENDATION_CHANGE_NOT_ALLOWED: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147220720i32 as _);
#[doc = "*Required features: `Win32_Storage_FileHistory`*"]
pub const FHCFG_E_TARGET_CANNOT_BE_USED: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147220727i32 as _);
#[doc = "*Required features: `Win32_Storage_FileHistory`*"]
pub const FHCFG_E_TARGET_NOT_CONFIGURED: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147220729i32 as _);
#[doc = "*Required features: `Win32_Storage_FileHistory`*"]
pub const FHCFG_E_TARGET_NOT_CONNECTED: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147220732i32 as _);
#[doc = "*Required features: `Win32_Storage_FileHistory`*"]
pub const FHCFG_E_TARGET_NOT_ENOUGH_FREE_SPACE: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147220728i32 as _);
#[doc = "*Required features: `Win32_Storage_FileHistory`*"]
pub const FHCFG_E_TARGET_REHYDRATED_ELSEWHERE: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147220719i32 as _);
#[doc = "*Required features: `Win32_Storage_FileHistory`*"]
pub const FHCFG_E_TARGET_VERIFICATION_FAILED: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147220730i32 as _);
#[doc = "*Required features: `Win32_Storage_FileHistory`*"]
pub const FHSVC_E_BACKUP_BLOCKED: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147219968i32 as _);
#[doc = "*Required features: `Win32_Storage_FileHistory`*"]
pub const FHSVC_E_CONFIG_DISABLED: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147219966i32 as _);
#[doc = "*Required features: `Win32_Storage_FileHistory`*"]
pub const FHSVC_E_CONFIG_DISABLED_GP: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147219965i32 as _);
#[doc = "*Required features: `Win32_Storage_FileHistory`*"]
pub const FHSVC_E_CONFIG_REHYDRATING: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147219963i32 as _);
#[doc = "*Required features: `Win32_Storage_FileHistory`*"]
pub const FHSVC_E_FATAL_CONFIG_ERROR: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147219964i32 as _);
#[doc = "*Required features: `Win32_Storage_FileHistory`*"]
pub const FHSVC_E_NOT_CONFIGURED: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147219967i32 as _);
#[doc = "*Required features: `Win32_Storage_FileHistory`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct FH_BACKUP_STATUS(pub i32);
pub const FH_STATUS_DISABLED: FH_BACKUP_STATUS = FH_BACKUP_STATUS(0i32);
pub const FH_STATUS_DISABLED_BY_GP: FH_BACKUP_STATUS = FH_BACKUP_STATUS(1i32);
pub const FH_STATUS_ENABLED: FH_BACKUP_STATUS = FH_BACKUP_STATUS(2i32);
pub const FH_STATUS_REHYDRATING: FH_BACKUP_STATUS = FH_BACKUP_STATUS(3i32);
pub const MAX_BACKUP_STATUS: FH_BACKUP_STATUS = FH_BACKUP_STATUS(4i32);
impl ::std::convert::From<i32> for FH_BACKUP_STATUS {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for FH_BACKUP_STATUS {
    type Abi = Self;
    type DefaultType = Self;
}
#[doc = "*Required features: `Win32_Storage_FileHistory`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
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
impl ::std::convert::From<i32> for FH_DEVICE_VALIDATION_RESULT {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for FH_DEVICE_VALIDATION_RESULT {
    type Abi = Self;
    type DefaultType = Self;
}
#[doc = "*Required features: `Win32_Storage_FileHistory`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct FH_LOCAL_POLICY_TYPE(pub i32);
pub const FH_FREQUENCY: FH_LOCAL_POLICY_TYPE = FH_LOCAL_POLICY_TYPE(0i32);
pub const FH_RETENTION_TYPE: FH_LOCAL_POLICY_TYPE = FH_LOCAL_POLICY_TYPE(1i32);
pub const FH_RETENTION_AGE: FH_LOCAL_POLICY_TYPE = FH_LOCAL_POLICY_TYPE(2i32);
pub const MAX_LOCAL_POLICY: FH_LOCAL_POLICY_TYPE = FH_LOCAL_POLICY_TYPE(3i32);
impl ::std::convert::From<i32> for FH_LOCAL_POLICY_TYPE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for FH_LOCAL_POLICY_TYPE {
    type Abi = Self;
    type DefaultType = Self;
}
#[doc = "*Required features: `Win32_Storage_FileHistory`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct FH_PROTECTED_ITEM_CATEGORY(pub i32);
pub const FH_FOLDER: FH_PROTECTED_ITEM_CATEGORY = FH_PROTECTED_ITEM_CATEGORY(0i32);
pub const FH_LIBRARY: FH_PROTECTED_ITEM_CATEGORY = FH_PROTECTED_ITEM_CATEGORY(1i32);
pub const MAX_PROTECTED_ITEM_CATEGORY: FH_PROTECTED_ITEM_CATEGORY = FH_PROTECTED_ITEM_CATEGORY(2i32);
impl ::std::convert::From<i32> for FH_PROTECTED_ITEM_CATEGORY {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for FH_PROTECTED_ITEM_CATEGORY {
    type Abi = Self;
    type DefaultType = Self;
}
#[doc = "*Required features: `Win32_Storage_FileHistory`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct FH_RETENTION_TYPES(pub i32);
pub const FH_RETENTION_DISABLED: FH_RETENTION_TYPES = FH_RETENTION_TYPES(0i32);
pub const FH_RETENTION_UNLIMITED: FH_RETENTION_TYPES = FH_RETENTION_TYPES(1i32);
pub const FH_RETENTION_AGE_BASED: FH_RETENTION_TYPES = FH_RETENTION_TYPES(2i32);
pub const MAX_RETENTION_TYPE: FH_RETENTION_TYPES = FH_RETENTION_TYPES(3i32);
impl ::std::convert::From<i32> for FH_RETENTION_TYPES {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for FH_RETENTION_TYPES {
    type Abi = Self;
    type DefaultType = Self;
}
#[doc = "*Required features: `Win32_Storage_FileHistory`*"]
pub const FH_STATE_BACKUP_NOT_SUPPORTED: u32 = 2064u32;
#[doc = "*Required features: `Win32_Storage_FileHistory`*"]
pub const FH_STATE_DISABLED_BY_GP: u32 = 2u32;
#[doc = "*Required features: `Win32_Storage_FileHistory`*"]
pub const FH_STATE_FATAL_CONFIG_ERROR: u32 = 3u32;
#[doc = "*Required features: `Win32_Storage_FileHistory`*"]
pub const FH_STATE_MIGRATING: u32 = 4u32;
#[doc = "*Required features: `Win32_Storage_FileHistory`*"]
pub const FH_STATE_NOT_TRACKED: u32 = 0u32;
#[doc = "*Required features: `Win32_Storage_FileHistory`*"]
pub const FH_STATE_NO_ERROR: u32 = 255u32;
#[doc = "*Required features: `Win32_Storage_FileHistory`*"]
pub const FH_STATE_OFF: u32 = 1u32;
#[doc = "*Required features: `Win32_Storage_FileHistory`*"]
pub const FH_STATE_REHYDRATING: u32 = 5u32;
#[doc = "*Required features: `Win32_Storage_FileHistory`*"]
pub const FH_STATE_RUNNING: u32 = 256u32;
#[doc = "*Required features: `Win32_Storage_FileHistory`*"]
pub const FH_STATE_STAGING_FULL: u32 = 18u32;
#[doc = "*Required features: `Win32_Storage_FileHistory`*"]
pub const FH_STATE_TARGET_ABSENT: u32 = 21u32;
#[doc = "*Required features: `Win32_Storage_FileHistory`*"]
pub const FH_STATE_TARGET_ACCESS_DENIED: u32 = 14u32;
#[doc = "*Required features: `Win32_Storage_FileHistory`*"]
pub const FH_STATE_TARGET_FS_LIMITATION: u32 = 13u32;
#[doc = "*Required features: `Win32_Storage_FileHistory`*"]
pub const FH_STATE_TARGET_FULL: u32 = 17u32;
#[doc = "*Required features: `Win32_Storage_FileHistory`*"]
pub const FH_STATE_TARGET_FULL_RETENTION_MAX: u32 = 16u32;
#[doc = "*Required features: `Win32_Storage_FileHistory`*"]
pub const FH_STATE_TARGET_LOW_SPACE: u32 = 20u32;
#[doc = "*Required features: `Win32_Storage_FileHistory`*"]
pub const FH_STATE_TARGET_LOW_SPACE_RETENTION_MAX: u32 = 19u32;
#[doc = "*Required features: `Win32_Storage_FileHistory`*"]
pub const FH_STATE_TARGET_VOLUME_DIRTY: u32 = 15u32;
#[doc = "*Required features: `Win32_Storage_FileHistory`*"]
pub const FH_STATE_TOO_MUCH_BEHIND: u32 = 240u32;
#[doc = "*Required features: `Win32_Storage_FileHistory`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct FH_TARGET_DRIVE_TYPES(pub i32);
pub const FH_DRIVE_UNKNOWN: FH_TARGET_DRIVE_TYPES = FH_TARGET_DRIVE_TYPES(0i32);
pub const FH_DRIVE_REMOVABLE: FH_TARGET_DRIVE_TYPES = FH_TARGET_DRIVE_TYPES(2i32);
pub const FH_DRIVE_FIXED: FH_TARGET_DRIVE_TYPES = FH_TARGET_DRIVE_TYPES(3i32);
pub const FH_DRIVE_REMOTE: FH_TARGET_DRIVE_TYPES = FH_TARGET_DRIVE_TYPES(4i32);
impl ::std::convert::From<i32> for FH_TARGET_DRIVE_TYPES {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for FH_TARGET_DRIVE_TYPES {
    type Abi = Self;
    type DefaultType = Self;
}
#[doc = "*Required features: `Win32_Storage_FileHistory`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct FH_TARGET_PROPERTY_TYPE(pub i32);
pub const FH_TARGET_NAME: FH_TARGET_PROPERTY_TYPE = FH_TARGET_PROPERTY_TYPE(0i32);
pub const FH_TARGET_URL: FH_TARGET_PROPERTY_TYPE = FH_TARGET_PROPERTY_TYPE(1i32);
pub const FH_TARGET_DRIVE_TYPE: FH_TARGET_PROPERTY_TYPE = FH_TARGET_PROPERTY_TYPE(2i32);
pub const MAX_TARGET_PROPERTY: FH_TARGET_PROPERTY_TYPE = FH_TARGET_PROPERTY_TYPE(3i32);
impl ::std::convert::From<i32> for FH_TARGET_PROPERTY_TYPE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for FH_TARGET_PROPERTY_TYPE {
    type Abi = Self;
    type DefaultType = Self;
}
#[doc = "*Required features: `Win32_Storage_FileHistory`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct FhBackupStopReason(pub i32);
pub const BackupInvalidStopReason: FhBackupStopReason = FhBackupStopReason(0i32);
pub const BackupLimitUserBusyMachineOnAC: FhBackupStopReason = FhBackupStopReason(1i32);
pub const BackupLimitUserIdleMachineOnDC: FhBackupStopReason = FhBackupStopReason(2i32);
pub const BackupLimitUserBusyMachineOnDC: FhBackupStopReason = FhBackupStopReason(3i32);
pub const BackupCancelled: FhBackupStopReason = FhBackupStopReason(4i32);
impl ::std::convert::From<i32> for FhBackupStopReason {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for FhBackupStopReason {
    type Abi = Self;
    type DefaultType = Self;
}
pub const FhConfigMgr: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3980639036, 2537, 18826, [157, 246, 33, 119, 36, 76, 109, 180]);
pub const FhReassociation: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1299353141, 5882, 17184, [158, 139, 191, 215, 16, 10, 136, 70]);
#[cfg(feature = "Win32_System_WindowsProgramming")]
#[doc = "*Required features: `Win32_Storage_FileHistory`, `Win32_System_WindowsProgramming`*"]
#[inline]
pub unsafe fn FhServiceBlockBackup<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::System::WindowsProgramming::FH_SERVICE_PIPE_HANDLE>>(pipe: Param0) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FhServiceBlockBackup(pipe: super::super::System::WindowsProgramming::FH_SERVICE_PIPE_HANDLE) -> ::windows::runtime::HRESULT;
        }
        FhServiceBlockBackup(pipe.into_param().abi()).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_System_WindowsProgramming")]
#[doc = "*Required features: `Win32_Storage_FileHistory`, `Win32_System_WindowsProgramming`*"]
#[inline]
pub unsafe fn FhServiceClosePipe<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::System::WindowsProgramming::FH_SERVICE_PIPE_HANDLE>>(pipe: Param0) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FhServiceClosePipe(pipe: super::super::System::WindowsProgramming::FH_SERVICE_PIPE_HANDLE) -> ::windows::runtime::HRESULT;
        }
        FhServiceClosePipe(pipe.into_param().abi()).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_WindowsProgramming"))]
#[doc = "*Required features: `Win32_Storage_FileHistory`, `Win32_Foundation`, `Win32_System_WindowsProgramming`*"]
#[inline]
pub unsafe fn FhServiceOpenPipe<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>>(startserviceifstopped: Param0) -> ::windows::runtime::Result<super::super::System::WindowsProgramming::FH_SERVICE_PIPE_HANDLE> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FhServiceOpenPipe(startserviceifstopped: super::super::Foundation::BOOL, pipe: *mut super::super::System::WindowsProgramming::FH_SERVICE_PIPE_HANDLE) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <super::super::System::WindowsProgramming::FH_SERVICE_PIPE_HANDLE as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        FhServiceOpenPipe(startserviceifstopped.into_param().abi(), &mut result__).from_abi::<super::super::System::WindowsProgramming::FH_SERVICE_PIPE_HANDLE>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_System_WindowsProgramming")]
#[doc = "*Required features: `Win32_Storage_FileHistory`, `Win32_System_WindowsProgramming`*"]
#[inline]
pub unsafe fn FhServiceReloadConfiguration<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::System::WindowsProgramming::FH_SERVICE_PIPE_HANDLE>>(pipe: Param0) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FhServiceReloadConfiguration(pipe: super::super::System::WindowsProgramming::FH_SERVICE_PIPE_HANDLE) -> ::windows::runtime::HRESULT;
        }
        FhServiceReloadConfiguration(pipe.into_param().abi()).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_WindowsProgramming"))]
#[doc = "*Required features: `Win32_Storage_FileHistory`, `Win32_Foundation`, `Win32_System_WindowsProgramming`*"]
#[inline]
pub unsafe fn FhServiceStartBackup<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::System::WindowsProgramming::FH_SERVICE_PIPE_HANDLE>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>>(pipe: Param0, lowpriorityio: Param1) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FhServiceStartBackup(pipe: super::super::System::WindowsProgramming::FH_SERVICE_PIPE_HANDLE, lowpriorityio: super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT;
        }
        FhServiceStartBackup(pipe.into_param().abi(), lowpriorityio.into_param().abi()).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_WindowsProgramming"))]
#[doc = "*Required features: `Win32_Storage_FileHistory`, `Win32_Foundation`, `Win32_System_WindowsProgramming`*"]
#[inline]
pub unsafe fn FhServiceStopBackup<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::System::WindowsProgramming::FH_SERVICE_PIPE_HANDLE>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>>(pipe: Param0, stoptracking: Param1) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FhServiceStopBackup(pipe: super::super::System::WindowsProgramming::FH_SERVICE_PIPE_HANDLE, stoptracking: super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT;
        }
        FhServiceStopBackup(pipe.into_param().abi(), stoptracking.into_param().abi()).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_System_WindowsProgramming")]
#[doc = "*Required features: `Win32_Storage_FileHistory`, `Win32_System_WindowsProgramming`*"]
#[inline]
pub unsafe fn FhServiceUnblockBackup<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::System::WindowsProgramming::FH_SERVICE_PIPE_HANDLE>>(pipe: Param0) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FhServiceUnblockBackup(pipe: super::super::System::WindowsProgramming::FH_SERVICE_PIPE_HANDLE) -> ::windows::runtime::HRESULT;
        }
        FhServiceUnblockBackup(pipe.into_param().abi()).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Storage_FileHistory`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct IFhConfigMgr(::windows::runtime::IUnknown);
impl IFhConfigMgr {
    #[doc = "*Required features: `Win32_Storage_FileHistory`*"]
    pub unsafe fn LoadConfiguration(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_FileHistory`, `Win32_Foundation`*"]
    pub unsafe fn CreateDefaultConfiguration<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>>(&self, overwriteifexists: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), overwriteifexists.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_Storage_FileHistory`*"]
    pub unsafe fn SaveConfiguration(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_FileHistory`, `Win32_Foundation`*"]
    pub unsafe fn AddRemoveExcludeRule<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>, Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, add: Param0, category: FH_PROTECTED_ITEM_CATEGORY, item: Param2) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self), add.into_param().abi(), ::std::mem::transmute(category), item.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_FileHistory`, `Win32_Foundation`*"]
    pub unsafe fn GetIncludeExcludeRules<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>>(&self, include: Param0, category: FH_PROTECTED_ITEM_CATEGORY) -> ::windows::runtime::Result<IFhScopeIterator> {
        let mut result__: <IFhScopeIterator as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self), include.into_param().abi(), ::std::mem::transmute(category), &mut result__).from_abi::<IFhScopeIterator>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_FileHistory`*"]
    pub unsafe fn GetLocalPolicy(&self, localpolicytype: FH_LOCAL_POLICY_TYPE) -> ::windows::runtime::Result<u64> {
        let mut result__: <u64 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).8)(::std::mem::transmute_copy(self), ::std::mem::transmute(localpolicytype), &mut result__).from_abi::<u64>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_FileHistory`*"]
    pub unsafe fn SetLocalPolicy(&self, localpolicytype: FH_LOCAL_POLICY_TYPE, policyvalue: u64) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).9)(::std::mem::transmute_copy(self), ::std::mem::transmute(localpolicytype), ::std::mem::transmute(policyvalue)).ok()
    }
    #[doc = "*Required features: `Win32_Storage_FileHistory`*"]
    pub unsafe fn GetBackupStatus(&self) -> ::windows::runtime::Result<FH_BACKUP_STATUS> {
        let mut result__: <FH_BACKUP_STATUS as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).10)(::std::mem::transmute_copy(self), &mut result__).from_abi::<FH_BACKUP_STATUS>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_FileHistory`*"]
    pub unsafe fn SetBackupStatus(&self, backupstatus: FH_BACKUP_STATUS) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).11)(::std::mem::transmute_copy(self), ::std::mem::transmute(backupstatus)).ok()
    }
    #[doc = "*Required features: `Win32_Storage_FileHistory`*"]
    pub unsafe fn GetDefaultTarget(&self) -> ::windows::runtime::Result<IFhTarget> {
        let mut result__: <IFhTarget as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).12)(::std::mem::transmute_copy(self), &mut result__).from_abi::<IFhTarget>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_FileHistory`, `Win32_Foundation`*"]
    pub unsafe fn ValidateTarget<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, targeturl: Param0) -> ::windows::runtime::Result<FH_DEVICE_VALIDATION_RESULT> {
        let mut result__: <FH_DEVICE_VALIDATION_RESULT as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).13)(::std::mem::transmute_copy(self), targeturl.into_param().abi(), &mut result__).from_abi::<FH_DEVICE_VALIDATION_RESULT>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_FileHistory`, `Win32_Foundation`*"]
    pub unsafe fn ProvisionAndSetNewTarget<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, targeturl: Param0, targetname: Param1) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).14)(::std::mem::transmute_copy(self), targeturl.into_param().abi(), targetname.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_FileHistory`, `Win32_Foundation`*"]
    pub unsafe fn ChangeDefaultTargetRecommendation<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>>(&self, recommend: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).15)(::std::mem::transmute_copy(self), recommend.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_FileHistory`, `Win32_Foundation`*"]
    pub unsafe fn QueryProtectionStatus(&self, protectionstate: *mut u32, protecteduntiltime: *mut super::super::Foundation::BSTR) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).16)(::std::mem::transmute_copy(self), ::std::mem::transmute(protectionstate), ::std::mem::transmute(protecteduntiltime)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IFhConfigMgr {
    type Vtable = IFhConfigMgr_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1784670811, 49039, 20197, [184, 195, 68, 216, 160, 215, 51, 28]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFhConfigMgr_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, overwriteifexists: super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, add: super::super::Foundation::BOOL, category: FH_PROTECTED_ITEM_CATEGORY, item: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, include: super::super::Foundation::BOOL, category: FH_PROTECTED_ITEM_CATEGORY, iterator: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, localpolicytype: FH_LOCAL_POLICY_TYPE, policyvalue: *mut u64) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, localpolicytype: FH_LOCAL_POLICY_TYPE, policyvalue: u64) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, backupstatus: *mut FH_BACKUP_STATUS) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, backupstatus: FH_BACKUP_STATUS) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, defaulttarget: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, targeturl: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>, validationresult: *mut FH_DEVICE_VALIDATION_RESULT) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, targeturl: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>, targetname: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, recommend: super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, protectionstate: *mut u32, protecteduntiltime: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: `Win32_Storage_FileHistory`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct IFhReassociation(::windows::runtime::IUnknown);
impl IFhReassociation {
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_FileHistory`, `Win32_Foundation`*"]
    pub unsafe fn ValidateTarget<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, targeturl: Param0) -> ::windows::runtime::Result<FH_DEVICE_VALIDATION_RESULT> {
        let mut result__: <FH_DEVICE_VALIDATION_RESULT as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), targeturl.into_param().abi(), &mut result__).from_abi::<FH_DEVICE_VALIDATION_RESULT>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_FileHistory`, `Win32_Foundation`*"]
    pub unsafe fn ScanTargetForConfigurations<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, targeturl: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), targeturl.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_FileHistory`, `Win32_Foundation`*"]
    pub unsafe fn GetConfigurationDetails(&self, index: u32, username: *mut super::super::Foundation::BSTR, pcname: *mut super::super::Foundation::BSTR, backuptime: *mut super::super::Foundation::FILETIME) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), ::std::mem::transmute(index), ::std::mem::transmute(username), ::std::mem::transmute(pcname), ::std::mem::transmute(backuptime)).ok()
    }
    #[doc = "*Required features: `Win32_Storage_FileHistory`*"]
    pub unsafe fn SelectConfiguration(&self, index: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self), ::std::mem::transmute(index)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_FileHistory`, `Win32_Foundation`*"]
    pub unsafe fn PerformReassociation<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>>(&self, overwriteifexists: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self), overwriteifexists.into_param().abi()).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IFhReassociation {
    type Vtable = IFhReassociation_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1698996874, 63117, 18348, [145, 239, 22, 178, 179, 106, 163, 238]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFhReassociation_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, targeturl: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>, validationresult: *mut FH_DEVICE_VALIDATION_RESULT) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, targeturl: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, index: u32, username: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>, pcname: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>, backuptime: *mut super::super::Foundation::FILETIME) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, index: u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, overwriteifexists: super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: `Win32_Storage_FileHistory`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct IFhScopeIterator(::windows::runtime::IUnknown);
impl IFhScopeIterator {
    #[doc = "*Required features: `Win32_Storage_FileHistory`*"]
    pub unsafe fn MoveToNextItem(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_FileHistory`, `Win32_Foundation`*"]
    pub unsafe fn GetItem(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IFhScopeIterator {
    type Vtable = IFhScopeIterator_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(832023502, 21290, 17606, [134, 21, 243, 102, 101, 102, 167, 32]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFhScopeIterator_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, item: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: `Win32_Storage_FileHistory`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct IFhTarget(::windows::runtime::IUnknown);
impl IFhTarget {
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_FileHistory`, `Win32_Foundation`*"]
    pub unsafe fn GetStringProperty(&self, propertytype: FH_TARGET_PROPERTY_TYPE) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), ::std::mem::transmute(propertytype), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_FileHistory`*"]
    pub unsafe fn GetNumericalProperty(&self, propertytype: FH_TARGET_PROPERTY_TYPE) -> ::windows::runtime::Result<u64> {
        let mut result__: <u64 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), ::std::mem::transmute(propertytype), &mut result__).from_abi::<u64>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IFhTarget {
    type Vtable = IFhTarget_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3631834621, 11181, 18007, [189, 59, 149, 103, 235, 48, 12, 237]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFhTarget_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, propertytype: FH_TARGET_PROPERTY_TYPE, propertyvalue: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, propertytype: FH_TARGET_PROPERTY_TYPE, propertyvalue: *mut u64) -> ::windows::runtime::HRESULT,
);
