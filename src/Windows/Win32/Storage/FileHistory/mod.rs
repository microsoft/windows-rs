#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
pub const FHCFG_E_CONFIGURATION_PREVIOUSLY_LOADED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147220731i32);
pub const FHCFG_E_CONFIG_ALREADY_EXISTS: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147220734i32);
pub const FHCFG_E_CONFIG_FILE_NOT_FOUND: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147220735i32);
pub const FHCFG_E_CORRUPT_CONFIG_FILE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147220736i32);
pub const FHCFG_E_INVALID_REHYDRATION_STATE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147220726i32);
pub const FHCFG_E_LEGACY_BACKUP_NOT_FOUND: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147220715i32);
pub const FHCFG_E_LEGACY_BACKUP_USER_EXCLUDED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147220716i32);
pub const FHCFG_E_LEGACY_TARGET_UNSUPPORTED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147220718i32);
pub const FHCFG_E_LEGACY_TARGET_VALIDATION_UNSUPPORTED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147220717i32);
pub const FHCFG_E_NO_VALID_CONFIGURATION_LOADED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147220733i32);
pub const FHCFG_E_RECOMMENDATION_CHANGE_NOT_ALLOWED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147220720i32);
pub const FHCFG_E_TARGET_CANNOT_BE_USED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147220727i32);
pub const FHCFG_E_TARGET_NOT_CONFIGURED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147220729i32);
pub const FHCFG_E_TARGET_NOT_CONNECTED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147220732i32);
pub const FHCFG_E_TARGET_NOT_ENOUGH_FREE_SPACE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147220728i32);
pub const FHCFG_E_TARGET_REHYDRATED_ELSEWHERE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147220719i32);
pub const FHCFG_E_TARGET_VERIFICATION_FAILED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147220730i32);
pub const FHSVC_E_BACKUP_BLOCKED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147219968i32);
pub const FHSVC_E_CONFIG_DISABLED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147219966i32);
pub const FHSVC_E_CONFIG_DISABLED_GP: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147219965i32);
pub const FHSVC_E_CONFIG_REHYDRATING: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147219963i32);
pub const FHSVC_E_FATAL_CONFIG_ERROR: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147219964i32);
pub const FHSVC_E_NOT_CONFIGURED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147219967i32);
pub type FH_BACKUP_STATUS = i32;
pub const FH_STATUS_DISABLED: FH_BACKUP_STATUS = 0i32;
pub const FH_STATUS_DISABLED_BY_GP: FH_BACKUP_STATUS = 1i32;
pub const FH_STATUS_ENABLED: FH_BACKUP_STATUS = 2i32;
pub const FH_STATUS_REHYDRATING: FH_BACKUP_STATUS = 3i32;
pub const MAX_BACKUP_STATUS: FH_BACKUP_STATUS = 4i32;
pub type FH_DEVICE_VALIDATION_RESULT = i32;
pub const FH_ACCESS_DENIED: FH_DEVICE_VALIDATION_RESULT = 0i32;
pub const FH_INVALID_DRIVE_TYPE: FH_DEVICE_VALIDATION_RESULT = 1i32;
pub const FH_READ_ONLY_PERMISSION: FH_DEVICE_VALIDATION_RESULT = 2i32;
pub const FH_CURRENT_DEFAULT: FH_DEVICE_VALIDATION_RESULT = 3i32;
pub const FH_NAMESPACE_EXISTS: FH_DEVICE_VALIDATION_RESULT = 4i32;
pub const FH_TARGET_PART_OF_LIBRARY: FH_DEVICE_VALIDATION_RESULT = 5i32;
pub const FH_VALID_TARGET: FH_DEVICE_VALIDATION_RESULT = 6i32;
pub const MAX_VALIDATION_RESULT: FH_DEVICE_VALIDATION_RESULT = 7i32;
pub type FH_LOCAL_POLICY_TYPE = i32;
pub const FH_FREQUENCY: FH_LOCAL_POLICY_TYPE = 0i32;
pub const FH_RETENTION_TYPE: FH_LOCAL_POLICY_TYPE = 1i32;
pub const FH_RETENTION_AGE: FH_LOCAL_POLICY_TYPE = 2i32;
pub const MAX_LOCAL_POLICY: FH_LOCAL_POLICY_TYPE = 3i32;
pub type FH_PROTECTED_ITEM_CATEGORY = i32;
pub const FH_FOLDER: FH_PROTECTED_ITEM_CATEGORY = 0i32;
pub const FH_LIBRARY: FH_PROTECTED_ITEM_CATEGORY = 1i32;
pub const MAX_PROTECTED_ITEM_CATEGORY: FH_PROTECTED_ITEM_CATEGORY = 2i32;
pub type FH_RETENTION_TYPES = i32;
pub const FH_RETENTION_DISABLED: FH_RETENTION_TYPES = 0i32;
pub const FH_RETENTION_UNLIMITED: FH_RETENTION_TYPES = 1i32;
pub const FH_RETENTION_AGE_BASED: FH_RETENTION_TYPES = 2i32;
pub const MAX_RETENTION_TYPE: FH_RETENTION_TYPES = 3i32;
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
pub type FH_TARGET_DRIVE_TYPES = i32;
pub const FH_DRIVE_UNKNOWN: FH_TARGET_DRIVE_TYPES = 0i32;
pub const FH_DRIVE_REMOVABLE: FH_TARGET_DRIVE_TYPES = 2i32;
pub const FH_DRIVE_FIXED: FH_TARGET_DRIVE_TYPES = 3i32;
pub const FH_DRIVE_REMOTE: FH_TARGET_DRIVE_TYPES = 4i32;
pub type FH_TARGET_PROPERTY_TYPE = i32;
pub const FH_TARGET_NAME: FH_TARGET_PROPERTY_TYPE = 0i32;
pub const FH_TARGET_URL: FH_TARGET_PROPERTY_TYPE = 1i32;
pub const FH_TARGET_DRIVE_TYPE: FH_TARGET_PROPERTY_TYPE = 2i32;
pub const MAX_TARGET_PROPERTY: FH_TARGET_PROPERTY_TYPE = 3i32;
pub type FhBackupStopReason = i32;
pub const BackupInvalidStopReason: FhBackupStopReason = 0i32;
pub const BackupLimitUserBusyMachineOnAC: FhBackupStopReason = 1i32;
pub const BackupLimitUserIdleMachineOnDC: FhBackupStopReason = 2i32;
pub const BackupLimitUserBusyMachineOnDC: FhBackupStopReason = 3i32;
pub const BackupCancelled: FhBackupStopReason = 4i32;
pub const FhConfigMgr: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xed43bb3c_09e9_498a_9df6_2177244c6db4);
pub const FhReassociation: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4d728e35_16fa_4320_9e8b_bfd7100a8846);
#[cfg(feature = "Win32_System_WindowsProgramming")]
#[inline]
pub unsafe fn FhServiceBlockBackup<'a, Param0: ::windows::core::IntoParam<'a, super::super::System::WindowsProgramming::FH_SERVICE_PIPE_HANDLE>>(pipe: Param0) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FhServiceBlockBackup(pipe: super::super::System::WindowsProgramming::FH_SERVICE_PIPE_HANDLE) -> ::windows::core::HRESULT;
        }
        FhServiceBlockBackup(pipe.into_param().abi()).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_System_WindowsProgramming")]
#[inline]
pub unsafe fn FhServiceClosePipe<'a, Param0: ::windows::core::IntoParam<'a, super::super::System::WindowsProgramming::FH_SERVICE_PIPE_HANDLE>>(pipe: Param0) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FhServiceClosePipe(pipe: super::super::System::WindowsProgramming::FH_SERVICE_PIPE_HANDLE) -> ::windows::core::HRESULT;
        }
        FhServiceClosePipe(pipe.into_param().abi()).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_WindowsProgramming"))]
#[inline]
pub unsafe fn FhServiceOpenPipe<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(startserviceifstopped: Param0) -> ::windows::core::Result<super::super::System::WindowsProgramming::FH_SERVICE_PIPE_HANDLE> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FhServiceOpenPipe(startserviceifstopped: super::super::Foundation::BOOL, pipe: *mut super::super::System::WindowsProgramming::FH_SERVICE_PIPE_HANDLE) -> ::windows::core::HRESULT;
        }
        let mut result__: <super::super::System::WindowsProgramming::FH_SERVICE_PIPE_HANDLE as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        FhServiceOpenPipe(startserviceifstopped.into_param().abi(), &mut result__).from_abi::<super::super::System::WindowsProgramming::FH_SERVICE_PIPE_HANDLE>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_System_WindowsProgramming")]
#[inline]
pub unsafe fn FhServiceReloadConfiguration<'a, Param0: ::windows::core::IntoParam<'a, super::super::System::WindowsProgramming::FH_SERVICE_PIPE_HANDLE>>(pipe: Param0) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FhServiceReloadConfiguration(pipe: super::super::System::WindowsProgramming::FH_SERVICE_PIPE_HANDLE) -> ::windows::core::HRESULT;
        }
        FhServiceReloadConfiguration(pipe.into_param().abi()).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_WindowsProgramming"))]
#[inline]
pub unsafe fn FhServiceStartBackup<'a, Param0: ::windows::core::IntoParam<'a, super::super::System::WindowsProgramming::FH_SERVICE_PIPE_HANDLE>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(pipe: Param0, lowpriorityio: Param1) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FhServiceStartBackup(pipe: super::super::System::WindowsProgramming::FH_SERVICE_PIPE_HANDLE, lowpriorityio: super::super::Foundation::BOOL) -> ::windows::core::HRESULT;
        }
        FhServiceStartBackup(pipe.into_param().abi(), lowpriorityio.into_param().abi()).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_WindowsProgramming"))]
#[inline]
pub unsafe fn FhServiceStopBackup<'a, Param0: ::windows::core::IntoParam<'a, super::super::System::WindowsProgramming::FH_SERVICE_PIPE_HANDLE>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(pipe: Param0, stoptracking: Param1) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FhServiceStopBackup(pipe: super::super::System::WindowsProgramming::FH_SERVICE_PIPE_HANDLE, stoptracking: super::super::Foundation::BOOL) -> ::windows::core::HRESULT;
        }
        FhServiceStopBackup(pipe.into_param().abi(), stoptracking.into_param().abi()).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_System_WindowsProgramming")]
#[inline]
pub unsafe fn FhServiceUnblockBackup<'a, Param0: ::windows::core::IntoParam<'a, super::super::System::WindowsProgramming::FH_SERVICE_PIPE_HANDLE>>(pipe: Param0) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FhServiceUnblockBackup(pipe: super::super::System::WindowsProgramming::FH_SERVICE_PIPE_HANDLE) -> ::windows::core::HRESULT;
        }
        FhServiceUnblockBackup(pipe.into_param().abi()).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[repr(transparent)]
pub struct IFhConfigMgr(::windows::core::IUnknown);
impl IFhConfigMgr {
    pub unsafe fn LoadConfiguration(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CreateDefaultConfiguration<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(&self, overwriteifexists: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), overwriteifexists.into_param().abi()).ok()
    }
    pub unsafe fn SaveConfiguration(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn AddRemoveExcludeRule<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, add: Param0, category: FH_PROTECTED_ITEM_CATEGORY, item: Param2) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), add.into_param().abi(), ::core::mem::transmute(category), item.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetIncludeExcludeRules<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(&self, include: Param0, category: FH_PROTECTED_ITEM_CATEGORY) -> ::windows::core::Result<IFhScopeIterator> {
        let mut result__: <IFhScopeIterator as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), include.into_param().abi(), ::core::mem::transmute(category), &mut result__).from_abi::<IFhScopeIterator>(result__)
    }
    pub unsafe fn GetLocalPolicy(&self, localpolicytype: FH_LOCAL_POLICY_TYPE) -> ::windows::core::Result<u64> {
        let mut result__: <u64 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(localpolicytype), &mut result__).from_abi::<u64>(result__)
    }
    pub unsafe fn SetLocalPolicy(&self, localpolicytype: FH_LOCAL_POLICY_TYPE, policyvalue: u64) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), ::core::mem::transmute(localpolicytype), ::core::mem::transmute(policyvalue)).ok()
    }
    pub unsafe fn GetBackupStatus(&self) -> ::windows::core::Result<FH_BACKUP_STATUS> {
        let mut result__: <FH_BACKUP_STATUS as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), &mut result__).from_abi::<FH_BACKUP_STATUS>(result__)
    }
    pub unsafe fn SetBackupStatus(&self, backupstatus: FH_BACKUP_STATUS) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self), ::core::mem::transmute(backupstatus)).ok()
    }
    pub unsafe fn GetDefaultTarget(&self) -> ::windows::core::Result<IFhTarget> {
        let mut result__: <IFhTarget as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).12)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IFhTarget>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ValidateTarget<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, targeturl: Param0) -> ::windows::core::Result<FH_DEVICE_VALIDATION_RESULT> {
        let mut result__: <FH_DEVICE_VALIDATION_RESULT as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).13)(::core::mem::transmute_copy(self), targeturl.into_param().abi(), &mut result__).from_abi::<FH_DEVICE_VALIDATION_RESULT>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ProvisionAndSetNewTarget<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, targeturl: Param0, targetname: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).14)(::core::mem::transmute_copy(self), targeturl.into_param().abi(), targetname.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ChangeDefaultTargetRecommendation<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(&self, recommend: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).15)(::core::mem::transmute_copy(self), recommend.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn QueryProtectionStatus(&self, protectionstate: *mut u32, protecteduntiltime: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).16)(::core::mem::transmute_copy(self), ::core::mem::transmute(protectionstate), ::core::mem::transmute(protecteduntiltime)).ok()
    }
}
impl ::core::convert::From<IFhConfigMgr> for ::windows::core::IUnknown {
    fn from(value: IFhConfigMgr) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IFhConfigMgr> for ::windows::core::IUnknown {
    fn from(value: &IFhConfigMgr) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IFhConfigMgr {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IFhConfigMgr {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IFhConfigMgr {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IFhConfigMgr {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IFhConfigMgr {}
unsafe impl ::windows::core::Interface for IFhConfigMgr {
    type Vtable = IFhConfigMgrVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6a5fea5b_bf8f_4ee5_b8c3_44d8a0d7331c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFhConfigMgrVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, overwriteifexists: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, add: super::super::Foundation::BOOL, category: FH_PROTECTED_ITEM_CATEGORY, item: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, include: super::super::Foundation::BOOL, category: FH_PROTECTED_ITEM_CATEGORY, iterator: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, localpolicytype: FH_LOCAL_POLICY_TYPE, policyvalue: *mut u64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, localpolicytype: FH_LOCAL_POLICY_TYPE, policyvalue: u64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, backupstatus: *mut FH_BACKUP_STATUS) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, backupstatus: FH_BACKUP_STATUS) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, defaulttarget: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, targeturl: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, validationresult: *mut FH_DEVICE_VALIDATION_RESULT) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, targeturl: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, targetname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, recommend: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, protectionstate: *mut u32, protecteduntiltime: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[repr(transparent)]
pub struct IFhReassociation(::windows::core::IUnknown);
impl IFhReassociation {
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ValidateTarget<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, targeturl: Param0) -> ::windows::core::Result<FH_DEVICE_VALIDATION_RESULT> {
        let mut result__: <FH_DEVICE_VALIDATION_RESULT as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), targeturl.into_param().abi(), &mut result__).from_abi::<FH_DEVICE_VALIDATION_RESULT>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ScanTargetForConfigurations<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, targeturl: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), targeturl.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetConfigurationDetails(&self, index: u32, username: *mut super::super::Foundation::BSTR, pcname: *mut super::super::Foundation::BSTR, backuptime: *mut super::super::Foundation::FILETIME) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(index), ::core::mem::transmute(username), ::core::mem::transmute(pcname), ::core::mem::transmute(backuptime)).ok()
    }
    pub unsafe fn SelectConfiguration(&self, index: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(index)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn PerformReassociation<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(&self, overwriteifexists: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), overwriteifexists.into_param().abi()).ok()
    }
}
impl ::core::convert::From<IFhReassociation> for ::windows::core::IUnknown {
    fn from(value: IFhReassociation) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IFhReassociation> for ::windows::core::IUnknown {
    fn from(value: &IFhReassociation) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IFhReassociation {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IFhReassociation {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IFhReassociation {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IFhReassociation {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IFhReassociation {}
unsafe impl ::windows::core::Interface for IFhReassociation {
    type Vtable = IFhReassociationVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6544a28a_f68d_47ac_91ef_16b2b36aa3ee);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFhReassociationVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, targeturl: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, validationresult: *mut FH_DEVICE_VALIDATION_RESULT) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, targeturl: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: u32, username: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pcname: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, backuptime: *mut super::super::Foundation::FILETIME) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, overwriteifexists: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[repr(transparent)]
pub struct IFhScopeIterator(::windows::core::IUnknown);
impl IFhScopeIterator {
    pub unsafe fn MoveToNextItem(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetItem(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
}
impl ::core::convert::From<IFhScopeIterator> for ::windows::core::IUnknown {
    fn from(value: IFhScopeIterator) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IFhScopeIterator> for ::windows::core::IUnknown {
    fn from(value: &IFhScopeIterator) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IFhScopeIterator {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IFhScopeIterator {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IFhScopeIterator {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IFhScopeIterator {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IFhScopeIterator {}
unsafe impl ::windows::core::Interface for IFhScopeIterator {
    type Vtable = IFhScopeIteratorVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3197abce_532a_44c6_8615_f3666566a720);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFhScopeIteratorVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, item: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[repr(transparent)]
pub struct IFhTarget(::windows::core::IUnknown);
impl IFhTarget {
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetStringProperty(&self, propertytype: FH_TARGET_PROPERTY_TYPE) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(propertytype), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    pub unsafe fn GetNumericalProperty(&self, propertytype: FH_TARGET_PROPERTY_TYPE) -> ::windows::core::Result<u64> {
        let mut result__: <u64 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(propertytype), &mut result__).from_abi::<u64>(result__)
    }
}
impl ::core::convert::From<IFhTarget> for ::windows::core::IUnknown {
    fn from(value: IFhTarget) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IFhTarget> for ::windows::core::IUnknown {
    fn from(value: &IFhTarget) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IFhTarget {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IFhTarget {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IFhTarget {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IFhTarget {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IFhTarget {}
unsafe impl ::windows::core::Interface for IFhTarget {
    type Vtable = IFhTargetVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd87965fd_2bad_4657_bd3b_9567eb300ced);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFhTargetVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, propertytype: FH_TARGET_PROPERTY_TYPE, propertyvalue: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, propertytype: FH_TARGET_PROPERTY_TYPE, propertyvalue: *mut u64) -> ::windows::core::HRESULT,
);
