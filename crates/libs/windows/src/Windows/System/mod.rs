#[cfg(feature = "System_Diagnostics")]
pub mod Diagnostics;
#[cfg(feature = "System_Display")]
pub mod Display;
#[cfg(feature = "System_Implementation")]
pub mod Implementation;
#[cfg(feature = "System_Inventory")]
pub mod Inventory;
#[cfg(feature = "System_Power")]
pub mod Power;
#[cfg(feature = "System_Profile")]
pub mod Profile;
#[cfg(feature = "System_RemoteDesktop")]
pub mod RemoteDesktop;
#[cfg(feature = "System_RemoteSystems")]
pub mod RemoteSystems;
#[cfg(feature = "System_Threading")]
pub mod Threading;
#[cfg(feature = "System_Update")]
pub mod Update;
#[cfg(feature = "System_UserProfile")]
pub mod UserProfile;
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppActivationResult(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IAppActivationResult {
    type Vtable = IAppActivationResult_Vtbl;
}
unsafe impl ::windows::core::Interface for IAppActivationResult {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6b528900_f46e_4eb0_aa6c_38af557cf9ed);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppActivationResult_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub ExtendedError: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::HRESULT) -> ::windows::core::HRESULT,
    pub AppResourceGroupInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppDiagnosticInfo(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IAppDiagnosticInfo {
    type Vtable = IAppDiagnosticInfo_Vtbl;
}
unsafe impl ::windows::core::Interface for IAppDiagnosticInfo {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe348a69a_8889_4ca3_be07_d5ffff5f0804);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppDiagnosticInfo_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "ApplicationModel")]
    pub AppInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "ApplicationModel"))]
    AppInfo: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppDiagnosticInfo2(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IAppDiagnosticInfo2 {
    type Vtable = IAppDiagnosticInfo2_Vtbl;
}
unsafe impl ::windows::core::Interface for IAppDiagnosticInfo2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xdf46fbd7_191a_446c_9473_8fbc2374a354);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppDiagnosticInfo2_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub GetResourceGroups: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetResourceGroups: usize,
    pub CreateResourceGroupWatcher: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppDiagnosticInfo3(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IAppDiagnosticInfo3 {
    type Vtable = IAppDiagnosticInfo3_Vtbl;
}
unsafe impl ::windows::core::Interface for IAppDiagnosticInfo3 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc895c63d_dd61_4c65_babd_81a10b4f9815);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppDiagnosticInfo3_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub LaunchAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    LaunchAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppDiagnosticInfoStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IAppDiagnosticInfoStatics {
    type Vtable = IAppDiagnosticInfoStatics_Vtbl;
}
unsafe impl ::windows::core::Interface for IAppDiagnosticInfoStatics {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xce6925bf_10ca_40c8_a9ca_c5c96501866e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppDiagnosticInfoStatics_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub RequestInfoAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    RequestInfoAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppDiagnosticInfoStatics2(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IAppDiagnosticInfoStatics2 {
    type Vtable = IAppDiagnosticInfoStatics2_Vtbl;
}
unsafe impl ::windows::core::Interface for IAppDiagnosticInfoStatics2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x05b24b86_1000_4c90_bb9f_7235071c50fe);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppDiagnosticInfoStatics2_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub CreateWatcher: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub RequestAccessAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RequestAccessAsync: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub RequestInfoForPackageAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, packagefamilyname: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    RequestInfoForPackageAsync: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub RequestInfoForAppAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    RequestInfoForAppAsync: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub RequestInfoForAppUserModelId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, appusermodelid: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    RequestInfoForAppUserModelId: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppDiagnosticInfoWatcher(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IAppDiagnosticInfoWatcher {
    type Vtable = IAppDiagnosticInfoWatcher_Vtbl;
}
unsafe impl ::windows::core::Interface for IAppDiagnosticInfoWatcher {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x75575070_01d3_489a_9325_52f9cc6ede0a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppDiagnosticInfoWatcher_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub Added: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Added: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveAdded: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveAdded: usize,
    #[cfg(feature = "Foundation")]
    pub Removed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Removed: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveRemoved: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveRemoved: usize,
    #[cfg(feature = "Foundation")]
    pub EnumerationCompleted: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    EnumerationCompleted: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveEnumerationCompleted: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveEnumerationCompleted: usize,
    #[cfg(feature = "Foundation")]
    pub Stopped: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Stopped: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveStopped: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveStopped: usize,
    pub Status: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut AppDiagnosticInfoWatcherStatus) -> ::windows::core::HRESULT,
    pub Start: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Stop: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppDiagnosticInfoWatcherEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IAppDiagnosticInfoWatcherEventArgs {
    type Vtable = IAppDiagnosticInfoWatcherEventArgs_Vtbl;
}
unsafe impl ::windows::core::Interface for IAppDiagnosticInfoWatcherEventArgs {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7017c716_e1da_4c65_99df_046dff5be71a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppDiagnosticInfoWatcherEventArgs_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub AppDiagnosticInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppExecutionStateChangeResult(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IAppExecutionStateChangeResult {
    type Vtable = IAppExecutionStateChangeResult_Vtbl;
}
unsafe impl ::windows::core::Interface for IAppExecutionStateChangeResult {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6f039bf0_f91b_4df8_ae77_3033ccb69114);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppExecutionStateChangeResult_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub ExtendedError: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::HRESULT) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppMemoryReport(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IAppMemoryReport {
    type Vtable = IAppMemoryReport_Vtbl;
}
unsafe impl ::windows::core::Interface for IAppMemoryReport {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6d65339b_4d6f_45bc_9c5e_e49b3ff2758d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppMemoryReport_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub PrivateCommitUsage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u64) -> ::windows::core::HRESULT,
    pub PeakPrivateCommitUsage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u64) -> ::windows::core::HRESULT,
    pub TotalCommitUsage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u64) -> ::windows::core::HRESULT,
    pub TotalCommitLimit: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u64) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppMemoryReport2(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IAppMemoryReport2 {
    type Vtable = IAppMemoryReport2_Vtbl;
}
unsafe impl ::windows::core::Interface for IAppMemoryReport2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5f7f3738_51b7_42dc_b7ed_79ba46d28857);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppMemoryReport2_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub ExpectedTotalCommitLimit: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u64) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppMemoryUsageLimitChangingEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IAppMemoryUsageLimitChangingEventArgs {
    type Vtable = IAppMemoryUsageLimitChangingEventArgs_Vtbl;
}
unsafe impl ::windows::core::Interface for IAppMemoryUsageLimitChangingEventArgs {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x79f86664_feca_4da5_9e40_2bc63efdc979);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppMemoryUsageLimitChangingEventArgs_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub OldLimit: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u64) -> ::windows::core::HRESULT,
    pub NewLimit: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u64) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppResourceGroupBackgroundTaskReport(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IAppResourceGroupBackgroundTaskReport {
    type Vtable = IAppResourceGroupBackgroundTaskReport_Vtbl;
}
unsafe impl ::windows::core::Interface for IAppResourceGroupBackgroundTaskReport {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2566e74e_b05d_40c2_9dc1_1a4f039ea120);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppResourceGroupBackgroundTaskReport_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub TaskId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub Name: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Trigger: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub EntryPoint: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppResourceGroupInfo(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IAppResourceGroupInfo {
    type Vtable = IAppResourceGroupInfo_Vtbl;
}
unsafe impl ::windows::core::Interface for IAppResourceGroupInfo {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb913f77a_e807_49f4_845e_7b8bdcfe8ee7);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppResourceGroupInfo_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub InstanceId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub IsShared: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub GetBackgroundTaskReports: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetBackgroundTaskReports: usize,
    pub GetMemoryReport: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Foundation_Collections", feature = "System_Diagnostics"))]
    pub GetProcessDiagnosticInfos: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "System_Diagnostics")))]
    GetProcessDiagnosticInfos: usize,
    pub GetStateReport: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppResourceGroupInfo2(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IAppResourceGroupInfo2 {
    type Vtable = IAppResourceGroupInfo2_Vtbl;
}
unsafe impl ::windows::core::Interface for IAppResourceGroupInfo2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xee9b236d_d305_4d6b_92f7_6afdad72dedc);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppResourceGroupInfo2_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub StartSuspendAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    StartSuspendAsync: usize,
    #[cfg(feature = "Foundation")]
    pub StartResumeAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    StartResumeAsync: usize,
    #[cfg(feature = "Foundation")]
    pub StartTerminateAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    StartTerminateAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppResourceGroupInfoWatcher(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IAppResourceGroupInfoWatcher {
    type Vtable = IAppResourceGroupInfoWatcher_Vtbl;
}
unsafe impl ::windows::core::Interface for IAppResourceGroupInfoWatcher {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd9b0a0fd_6e5a_4c72_8b17_09fec4a212bd);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppResourceGroupInfoWatcher_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub Added: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Added: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveAdded: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveAdded: usize,
    #[cfg(feature = "Foundation")]
    pub Removed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Removed: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveRemoved: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveRemoved: usize,
    #[cfg(feature = "Foundation")]
    pub EnumerationCompleted: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    EnumerationCompleted: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveEnumerationCompleted: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveEnumerationCompleted: usize,
    #[cfg(feature = "Foundation")]
    pub Stopped: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Stopped: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveStopped: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveStopped: usize,
    #[cfg(feature = "Foundation")]
    pub ExecutionStateChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ExecutionStateChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveExecutionStateChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveExecutionStateChanged: usize,
    pub Status: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut AppResourceGroupInfoWatcherStatus) -> ::windows::core::HRESULT,
    pub Start: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Stop: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppResourceGroupInfoWatcherEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IAppResourceGroupInfoWatcherEventArgs {
    type Vtable = IAppResourceGroupInfoWatcherEventArgs_Vtbl;
}
unsafe impl ::windows::core::Interface for IAppResourceGroupInfoWatcherEventArgs {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7a787637_6302_4d2f_bf89_1c12d0b2a6b9);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppResourceGroupInfoWatcherEventArgs_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub AppDiagnosticInfos: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    AppDiagnosticInfos: usize,
    pub AppResourceGroupInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppResourceGroupInfoWatcherExecutionStateChangedEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IAppResourceGroupInfoWatcherExecutionStateChangedEventArgs {
    type Vtable = IAppResourceGroupInfoWatcherExecutionStateChangedEventArgs_Vtbl;
}
unsafe impl ::windows::core::Interface for IAppResourceGroupInfoWatcherExecutionStateChangedEventArgs {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1bdbedd7_fee6_4fd4_98dd_e92a2cc299f3);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppResourceGroupInfoWatcherExecutionStateChangedEventArgs_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub AppDiagnosticInfos: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    AppDiagnosticInfos: usize,
    pub AppResourceGroupInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppResourceGroupMemoryReport(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IAppResourceGroupMemoryReport {
    type Vtable = IAppResourceGroupMemoryReport_Vtbl;
}
unsafe impl ::windows::core::Interface for IAppResourceGroupMemoryReport {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2c8c06b1_7db1_4c51_a225_7fae2d49e431);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppResourceGroupMemoryReport_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub CommitUsageLimit: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u64) -> ::windows::core::HRESULT,
    pub CommitUsageLevel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut AppMemoryUsageLevel) -> ::windows::core::HRESULT,
    pub PrivateCommitUsage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u64) -> ::windows::core::HRESULT,
    pub TotalCommitUsage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u64) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppResourceGroupStateReport(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IAppResourceGroupStateReport {
    type Vtable = IAppResourceGroupStateReport_Vtbl;
}
unsafe impl ::windows::core::Interface for IAppResourceGroupStateReport {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x52849f18_2f70_4236_ab40_d04db0c7b931);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppResourceGroupStateReport_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub ExecutionState: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut AppResourceGroupExecutionState) -> ::windows::core::HRESULT,
    pub EnergyQuotaState: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut AppResourceGroupEnergyQuotaState) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppUriHandlerHost(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IAppUriHandlerHost {
    type Vtable = IAppUriHandlerHost_Vtbl;
}
unsafe impl ::windows::core::Interface for IAppUriHandlerHost {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5d50cac5_92d2_5409_b56f_7f73e10ea4c3);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppUriHandlerHost_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Name: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppUriHandlerHost2(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IAppUriHandlerHost2 {
    type Vtable = IAppUriHandlerHost2_Vtbl;
}
unsafe impl ::windows::core::Interface for IAppUriHandlerHost2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3a0bee95_29e4_51bf_8095_a3c068e3c72a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppUriHandlerHost2_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub IsEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetIsEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppUriHandlerHostFactory(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IAppUriHandlerHostFactory {
    type Vtable = IAppUriHandlerHostFactory_Vtbl;
}
unsafe impl ::windows::core::Interface for IAppUriHandlerHostFactory {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x257c3c96_ce04_5f98_96bb_3ebd3e9275bb);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppUriHandlerHostFactory_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub CreateInstance: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppUriHandlerRegistration(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IAppUriHandlerRegistration {
    type Vtable = IAppUriHandlerRegistration_Vtbl;
}
unsafe impl ::windows::core::Interface for IAppUriHandlerRegistration {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6f73aeb1_4569_5c3f_9ba0_99123eea32c3);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppUriHandlerRegistration_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Name: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub User: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub GetAppAddedHostsAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetAppAddedHostsAsync: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub SetAppAddedHostsAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hosts: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    SetAppAddedHostsAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppUriHandlerRegistration2(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IAppUriHandlerRegistration2 {
    type Vtable = IAppUriHandlerRegistration2_Vtbl;
}
unsafe impl ::windows::core::Interface for IAppUriHandlerRegistration2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd54dac97_cb39_5f1f_883e_01853730bd6d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppUriHandlerRegistration2_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub GetAllHosts: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetAllHosts: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub UpdateHosts: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hosts: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    UpdateHosts: usize,
    pub PackageFamilyName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppUriHandlerRegistrationManager(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IAppUriHandlerRegistrationManager {
    type Vtable = IAppUriHandlerRegistrationManager_Vtbl;
}
unsafe impl ::windows::core::Interface for IAppUriHandlerRegistrationManager {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe62c9a52_ac94_5750_ac1b_6cfb6f250263);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppUriHandlerRegistrationManager_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub User: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub TryGetRegistration: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppUriHandlerRegistrationManager2(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IAppUriHandlerRegistrationManager2 {
    type Vtable = IAppUriHandlerRegistrationManager2_Vtbl;
}
unsafe impl ::windows::core::Interface for IAppUriHandlerRegistrationManager2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xbddfcaf1_b51a_5e69_aefd_7088d9f2b123);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppUriHandlerRegistrationManager2_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub PackageFamilyName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppUriHandlerRegistrationManagerStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IAppUriHandlerRegistrationManagerStatics {
    type Vtable = IAppUriHandlerRegistrationManagerStatics_Vtbl;
}
unsafe impl ::windows::core::Interface for IAppUriHandlerRegistrationManagerStatics {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd5cedd9f_5729_5b76_a1d4_0285f295c124);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppUriHandlerRegistrationManagerStatics_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub GetDefault: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetForUser: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, user: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppUriHandlerRegistrationManagerStatics2(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IAppUriHandlerRegistrationManagerStatics2 {
    type Vtable = IAppUriHandlerRegistrationManagerStatics2_Vtbl;
}
unsafe impl ::windows::core::Interface for IAppUriHandlerRegistrationManagerStatics2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x14f78379_6890_5080_90a7_98824a7f079e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppUriHandlerRegistrationManagerStatics2_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub GetForPackage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, packagefamilyname: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetForPackageForUser: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, packagefamilyname: *mut ::core::ffi::c_void, user: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IDateTimeSettingsStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IDateTimeSettingsStatics {
    type Vtable = IDateTimeSettingsStatics_Vtbl;
}
unsafe impl ::windows::core::Interface for IDateTimeSettingsStatics {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5d2150d1_47ee_48ab_a52b_9f1954278d82);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDateTimeSettingsStatics_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub SetSystemDateTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, utcdatetime: super::Foundation::DateTime) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetSystemDateTime: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IDispatcherQueue(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IDispatcherQueue {
    type Vtable = IDispatcherQueue_Vtbl;
}
unsafe impl ::windows::core::Interface for IDispatcherQueue {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x603e88e4_a338_4ffe_a457_a5cfb9ceb899);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDispatcherQueue_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub CreateTimer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub TryEnqueue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, callback: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub TryEnqueueWithPriority: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, priority: DispatcherQueuePriority, callback: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub ShutdownStarting: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ShutdownStarting: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveShutdownStarting: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveShutdownStarting: usize,
    #[cfg(feature = "Foundation")]
    pub ShutdownCompleted: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ShutdownCompleted: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveShutdownCompleted: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveShutdownCompleted: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IDispatcherQueue2(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IDispatcherQueue2 {
    type Vtable = IDispatcherQueue2_Vtbl;
}
unsafe impl ::windows::core::Interface for IDispatcherQueue2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc822c647_30ef_506e_bd1e_a647ae6675ff);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDispatcherQueue2_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub HasThreadAccess: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IDispatcherQueueController(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IDispatcherQueueController {
    type Vtable = IDispatcherQueueController_Vtbl;
}
unsafe impl ::windows::core::Interface for IDispatcherQueueController {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x22f34e66_50db_4e36_a98d_61c01b384d20);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDispatcherQueueController_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub DispatcherQueue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub ShutdownQueueAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ShutdownQueueAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IDispatcherQueueControllerStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IDispatcherQueueControllerStatics {
    type Vtable = IDispatcherQueueControllerStatics_Vtbl;
}
unsafe impl ::windows::core::Interface for IDispatcherQueueControllerStatics {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0a6c98e0_5198_49a2_a313_3f70d1f13c27);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDispatcherQueueControllerStatics_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub CreateOnDedicatedThread: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IDispatcherQueueShutdownStartingEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IDispatcherQueueShutdownStartingEventArgs {
    type Vtable = IDispatcherQueueShutdownStartingEventArgs_Vtbl;
}
unsafe impl ::windows::core::Interface for IDispatcherQueueShutdownStartingEventArgs {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc4724c4c_ff97_40c0_a226_cc0aaa545e89);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDispatcherQueueShutdownStartingEventArgs_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub GetDeferral: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetDeferral: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IDispatcherQueueStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IDispatcherQueueStatics {
    type Vtable = IDispatcherQueueStatics_Vtbl;
}
unsafe impl ::windows::core::Interface for IDispatcherQueueStatics {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa96d83d7_9371_4517_9245_d0824ac12c74);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDispatcherQueueStatics_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub GetForCurrentThread: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IDispatcherQueueTimer(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IDispatcherQueueTimer {
    type Vtable = IDispatcherQueueTimer_Vtbl;
}
unsafe impl ::windows::core::Interface for IDispatcherQueueTimer {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5feabb1d_a31c_4727_b1ac_37454649d56a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDispatcherQueueTimer_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub Interval: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::Foundation::TimeSpan) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Interval: usize,
    #[cfg(feature = "Foundation")]
    pub SetInterval: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::Foundation::TimeSpan) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetInterval: usize,
    pub IsRunning: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub IsRepeating: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetIsRepeating: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    pub Start: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Stop: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub Tick: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Tick: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveTick: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveTick: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IFolderLauncherOptions(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IFolderLauncherOptions {
    type Vtable = IFolderLauncherOptions_Vtbl;
}
unsafe impl ::windows::core::Interface for IFolderLauncherOptions {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xbb91c27d_6b87_432a_bd04_776c6f5fb2ab);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFolderLauncherOptions_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(all(feature = "Foundation_Collections", feature = "Storage"))]
    pub ItemsToSelect: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Storage")))]
    ItemsToSelect: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IKnownUserPropertiesStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IKnownUserPropertiesStatics {
    type Vtable = IKnownUserPropertiesStatics_Vtbl;
}
unsafe impl ::windows::core::Interface for IKnownUserPropertiesStatics {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7755911a_70c5_48e5_b637_5ba3441e4ee4);
}
#[repr(C)]
#[doc(hidden)]
pub struct IKnownUserPropertiesStatics_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub DisplayName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub FirstName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub LastName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub ProviderName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub AccountName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GuestHost: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub PrincipalName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub DomainName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SessionInitiationProtocolUri: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IKnownUserPropertiesStatics2(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IKnownUserPropertiesStatics2 {
    type Vtable = IKnownUserPropertiesStatics2_Vtbl;
}
unsafe impl ::windows::core::Interface for IKnownUserPropertiesStatics2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5b450782_f620_577e_b1b3_dd56644d79b1);
}
#[repr(C)]
#[doc(hidden)]
pub struct IKnownUserPropertiesStatics2_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub AgeEnforcementRegion: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ILaunchUriResult(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for ILaunchUriResult {
    type Vtable = ILaunchUriResult_Vtbl;
}
unsafe impl ::windows::core::Interface for ILaunchUriResult {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xec27a8df_f6d5_45ca_913a_70a40c5c8221);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILaunchUriResult_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Status: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut LaunchUriStatus) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub Result: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Result: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ILauncherOptions(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for ILauncherOptions {
    type Vtable = ILauncherOptions_Vtbl;
}
unsafe impl ::windows::core::Interface for ILauncherOptions {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xbafa21d8_b071_4cd8_853e_341203e557d3);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILauncherOptions_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub TreatAsUntrusted: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetTreatAsUntrusted: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    pub DisplayApplicationPicker: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetDisplayApplicationPicker: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    pub UI: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub PreferredApplicationPackageFamilyName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetPreferredApplicationPackageFamilyName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub PreferredApplicationDisplayName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetPreferredApplicationDisplayName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub FallbackUri: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    FallbackUri: usize,
    #[cfg(feature = "Foundation")]
    pub SetFallbackUri: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetFallbackUri: usize,
    pub ContentType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetContentType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ILauncherOptions2(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for ILauncherOptions2 {
    type Vtable = ILauncherOptions2_Vtbl;
}
unsafe impl ::windows::core::Interface for ILauncherOptions2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3ba08eb4_6e40_4dce_a1a3_2f53950afb49);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILauncherOptions2_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub TargetApplicationPackageFamilyName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetTargetApplicationPackageFamilyName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Storage_Search")]
    pub NeighboringFilesQuery: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Search"))]
    NeighboringFilesQuery: usize,
    #[cfg(feature = "Storage_Search")]
    pub SetNeighboringFilesQuery: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Search"))]
    SetNeighboringFilesQuery: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ILauncherOptions3(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for ILauncherOptions3 {
    type Vtable = ILauncherOptions3_Vtbl;
}
unsafe impl ::windows::core::Interface for ILauncherOptions3 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf0770655_4b63_4e3a_9107_4e687841923a);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILauncherOptions3_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub IgnoreAppUriHandlers: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetIgnoreAppUriHandlers: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ILauncherOptions4(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for ILauncherOptions4 {
    type Vtable = ILauncherOptions4_Vtbl;
}
unsafe impl ::windows::core::Interface for ILauncherOptions4 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xef6fd10e_e6fb_4814_a44e_57e8b9d9a01b);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILauncherOptions4_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub LimitPickerToCurrentAppAndAppUriHandlers: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetLimitPickerToCurrentAppAndAppUriHandlers: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ILauncherStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for ILauncherStatics {
    type Vtable = ILauncherStatics_Vtbl;
}
unsafe impl ::windows::core::Interface for ILauncherStatics {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x277151c3_9e3e_42f6_91a4_5dfdeb232451);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILauncherStatics_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(all(feature = "Foundation", feature = "Storage"))]
    pub LaunchFileAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, file: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage")))]
    LaunchFileAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "Storage"))]
    pub LaunchFileWithOptionsAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, file: *mut ::core::ffi::c_void, options: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage")))]
    LaunchFileWithOptionsAsync: usize,
    #[cfg(feature = "Foundation")]
    pub LaunchUriAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, uri: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    LaunchUriAsync: usize,
    #[cfg(feature = "Foundation")]
    pub LaunchUriWithOptionsAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, uri: *mut ::core::ffi::c_void, options: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    LaunchUriWithOptionsAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ILauncherStatics2(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for ILauncherStatics2 {
    type Vtable = ILauncherStatics2_Vtbl;
}
unsafe impl ::windows::core::Interface for ILauncherStatics2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x59ba2fbb_24cb_4c02_a4c4_8294569d54f1);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILauncherStatics2_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub LaunchUriForResultsAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, uri: *mut ::core::ffi::c_void, options: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    LaunchUriForResultsAsync: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub LaunchUriForResultsWithDataAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, uri: *mut ::core::ffi::c_void, options: *mut ::core::ffi::c_void, inputdata: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    LaunchUriForResultsWithDataAsync: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub LaunchUriWithDataAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, uri: *mut ::core::ffi::c_void, options: *mut ::core::ffi::c_void, inputdata: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    LaunchUriWithDataAsync: usize,
    #[cfg(feature = "Foundation")]
    pub QueryUriSupportAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, uri: *mut ::core::ffi::c_void, launchquerysupporttype: LaunchQuerySupportType, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    QueryUriSupportAsync: usize,
    #[cfg(feature = "Foundation")]
    pub QueryUriSupportWithPackageFamilyNameAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, uri: *mut ::core::ffi::c_void, launchquerysupporttype: LaunchQuerySupportType, packagefamilyname: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    QueryUriSupportWithPackageFamilyNameAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "Storage"))]
    pub QueryFileSupportAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, file: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage")))]
    QueryFileSupportAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "Storage"))]
    pub QueryFileSupportWithPackageFamilyNameAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, file: *mut ::core::ffi::c_void, packagefamilyname: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage")))]
    QueryFileSupportWithPackageFamilyNameAsync: usize,
    #[cfg(all(feature = "ApplicationModel", feature = "Foundation_Collections"))]
    pub FindUriSchemeHandlersAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, scheme: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "ApplicationModel", feature = "Foundation_Collections")))]
    FindUriSchemeHandlersAsync: usize,
    #[cfg(all(feature = "ApplicationModel", feature = "Foundation_Collections"))]
    pub FindUriSchemeHandlersWithLaunchUriTypeAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, scheme: *mut ::core::ffi::c_void, launchquerysupporttype: LaunchQuerySupportType, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "ApplicationModel", feature = "Foundation_Collections")))]
    FindUriSchemeHandlersWithLaunchUriTypeAsync: usize,
    #[cfg(all(feature = "ApplicationModel", feature = "Foundation_Collections"))]
    pub FindFileHandlersAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, extension: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "ApplicationModel", feature = "Foundation_Collections")))]
    FindFileHandlersAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ILauncherStatics3(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for ILauncherStatics3 {
    type Vtable = ILauncherStatics3_Vtbl;
}
unsafe impl ::windows::core::Interface for ILauncherStatics3 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x234261a8_9db3_4683_aa42_dc6f51d33847);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILauncherStatics3_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(all(feature = "Foundation", feature = "Storage"))]
    pub LaunchFolderAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, folder: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage")))]
    LaunchFolderAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "Storage"))]
    pub LaunchFolderWithOptionsAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, folder: *mut ::core::ffi::c_void, options: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage")))]
    LaunchFolderWithOptionsAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ILauncherStatics4(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for ILauncherStatics4 {
    type Vtable = ILauncherStatics4_Vtbl;
}
unsafe impl ::windows::core::Interface for ILauncherStatics4 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb9ec819f_b5a5_41c6_b3b3_dd1b3178bcf2);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILauncherStatics4_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub QueryAppUriSupportAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, uri: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    QueryAppUriSupportAsync: usize,
    #[cfg(feature = "Foundation")]
    pub QueryAppUriSupportWithPackageFamilyNameAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, uri: *mut ::core::ffi::c_void, packagefamilyname: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    QueryAppUriSupportWithPackageFamilyNameAsync: usize,
    #[cfg(all(feature = "ApplicationModel", feature = "Foundation_Collections"))]
    pub FindAppUriHandlersAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, uri: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "ApplicationModel", feature = "Foundation_Collections")))]
    FindAppUriHandlersAsync: usize,
    #[cfg(feature = "Foundation")]
    pub LaunchUriForUserAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, user: *mut ::core::ffi::c_void, uri: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    LaunchUriForUserAsync: usize,
    #[cfg(feature = "Foundation")]
    pub LaunchUriWithOptionsForUserAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, user: *mut ::core::ffi::c_void, uri: *mut ::core::ffi::c_void, options: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    LaunchUriWithOptionsForUserAsync: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub LaunchUriWithDataForUserAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, user: *mut ::core::ffi::c_void, uri: *mut ::core::ffi::c_void, options: *mut ::core::ffi::c_void, inputdata: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    LaunchUriWithDataForUserAsync: usize,
    #[cfg(feature = "Foundation")]
    pub LaunchUriForResultsForUserAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, user: *mut ::core::ffi::c_void, uri: *mut ::core::ffi::c_void, options: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    LaunchUriForResultsForUserAsync: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub LaunchUriForResultsWithDataForUserAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, user: *mut ::core::ffi::c_void, uri: *mut ::core::ffi::c_void, options: *mut ::core::ffi::c_void, inputdata: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    LaunchUriForResultsWithDataForUserAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ILauncherStatics5(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for ILauncherStatics5 {
    type Vtable = ILauncherStatics5_Vtbl;
}
unsafe impl ::windows::core::Interface for ILauncherStatics5 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5b24ef84_d895_5fea_9153_1ac49aed9ba9);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILauncherStatics5_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub LaunchFolderPathAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, path: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    LaunchFolderPathAsync: usize,
    #[cfg(feature = "Foundation")]
    pub LaunchFolderPathWithOptionsAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, path: *mut ::core::ffi::c_void, options: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    LaunchFolderPathWithOptionsAsync: usize,
    #[cfg(feature = "Foundation")]
    pub LaunchFolderPathForUserAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, user: *mut ::core::ffi::c_void, path: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    LaunchFolderPathForUserAsync: usize,
    #[cfg(feature = "Foundation")]
    pub LaunchFolderPathWithOptionsForUserAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, user: *mut ::core::ffi::c_void, path: *mut ::core::ffi::c_void, options: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    LaunchFolderPathWithOptionsForUserAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ILauncherUIOptions(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for ILauncherUIOptions {
    type Vtable = ILauncherUIOptions_Vtbl;
}
unsafe impl ::windows::core::Interface for ILauncherUIOptions {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1b25da6e_8aa6_41e9_8251_4165f5985f49);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILauncherUIOptions_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub InvocationPoint: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    InvocationPoint: usize,
    #[cfg(feature = "Foundation")]
    pub SetInvocationPoint: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetInvocationPoint: usize,
    #[cfg(feature = "Foundation")]
    pub SelectionRect: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SelectionRect: usize,
    #[cfg(feature = "Foundation")]
    pub SetSelectionRect: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetSelectionRect: usize,
    #[cfg(feature = "UI_Popups")]
    pub PreferredPlacement: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::UI::Popups::Placement) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Popups"))]
    PreferredPlacement: usize,
    #[cfg(feature = "UI_Popups")]
    pub SetPreferredPlacement: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::UI::Popups::Placement) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Popups"))]
    SetPreferredPlacement: usize,
}
#[doc = "*Required features: `\"System\"`*"]
#[repr(transparent)]
pub struct ILauncherViewOptions(::windows::core::IUnknown);
impl ILauncherViewOptions {
    #[doc = "*Required features: `\"UI_ViewManagement\"`*"]
    #[cfg(feature = "UI_ViewManagement")]
    pub fn DesiredRemainingView(&self) -> ::windows::core::Result<super::UI::ViewManagement::ViewSizePreference> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).DesiredRemainingView)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"UI_ViewManagement\"`*"]
    #[cfg(feature = "UI_ViewManagement")]
    pub fn SetDesiredRemainingView(&self, value: super::UI::ViewManagement::ViewSizePreference) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetDesiredRemainingView)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
}
::windows::core::interface_hierarchy!(ILauncherViewOptions, ::windows::core::IUnknown, ::windows::core::IInspectable);
impl ::core::clone::Clone for ILauncherViewOptions {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ILauncherViewOptions {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ILauncherViewOptions {}
impl ::core::fmt::Debug for ILauncherViewOptions {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ILauncherViewOptions").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for ILauncherViewOptions {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{8a9b29f1-7ca7-49de-9bd3-3c5b7184f616}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for ILauncherViewOptions {
    type Vtable = ILauncherViewOptions_Vtbl;
}
unsafe impl ::windows::core::Interface for ILauncherViewOptions {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8a9b29f1_7ca7_49de_9bd3_3c5b7184f616);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILauncherViewOptions_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "UI_ViewManagement")]
    pub DesiredRemainingView: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::UI::ViewManagement::ViewSizePreference) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_ViewManagement"))]
    DesiredRemainingView: usize,
    #[cfg(feature = "UI_ViewManagement")]
    pub SetDesiredRemainingView: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::UI::ViewManagement::ViewSizePreference) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_ViewManagement"))]
    SetDesiredRemainingView: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMemoryManagerStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IMemoryManagerStatics {
    type Vtable = IMemoryManagerStatics_Vtbl;
}
unsafe impl ::windows::core::Interface for IMemoryManagerStatics {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5c6c279c_d7ca_4779_9188_4057219ce64c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMemoryManagerStatics_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub AppMemoryUsage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u64) -> ::windows::core::HRESULT,
    pub AppMemoryUsageLimit: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u64) -> ::windows::core::HRESULT,
    pub AppMemoryUsageLevel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut AppMemoryUsageLevel) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub AppMemoryUsageIncreased: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    AppMemoryUsageIncreased: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveAppMemoryUsageIncreased: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveAppMemoryUsageIncreased: usize,
    #[cfg(feature = "Foundation")]
    pub AppMemoryUsageDecreased: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    AppMemoryUsageDecreased: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveAppMemoryUsageDecreased: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveAppMemoryUsageDecreased: usize,
    #[cfg(feature = "Foundation")]
    pub AppMemoryUsageLimitChanging: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    AppMemoryUsageLimitChanging: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveAppMemoryUsageLimitChanging: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveAppMemoryUsageLimitChanging: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMemoryManagerStatics2(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IMemoryManagerStatics2 {
    type Vtable = IMemoryManagerStatics2_Vtbl;
}
unsafe impl ::windows::core::Interface for IMemoryManagerStatics2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6eee351f_6d62_423f_9479_b01f9c9f7669);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMemoryManagerStatics2_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub GetAppMemoryReport: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetProcessMemoryReport: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMemoryManagerStatics3(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IMemoryManagerStatics3 {
    type Vtable = IMemoryManagerStatics3_Vtbl;
}
unsafe impl ::windows::core::Interface for IMemoryManagerStatics3 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x149b59ce_92ad_4e35_89eb_50dfb4c0d91c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMemoryManagerStatics3_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub TrySetAppMemoryUsageLimit: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: u64, result__: *mut bool) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMemoryManagerStatics4(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IMemoryManagerStatics4 {
    type Vtable = IMemoryManagerStatics4_Vtbl;
}
unsafe impl ::windows::core::Interface for IMemoryManagerStatics4 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc5a94828_e84e_4886_8a0d_44b3190e3b72);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMemoryManagerStatics4_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub ExpectedAppMemoryUsageLimit: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u64) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IProcessLauncherOptions(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IProcessLauncherOptions {
    type Vtable = IProcessLauncherOptions_Vtbl;
}
unsafe impl ::windows::core::Interface for IProcessLauncherOptions {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3080b9cf_f444_4a83_beaf_a549a0f3229c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IProcessLauncherOptions_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Storage_Streams")]
    pub StandardInput: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    StandardInput: usize,
    #[cfg(feature = "Storage_Streams")]
    pub SetStandardInput: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    SetStandardInput: usize,
    #[cfg(feature = "Storage_Streams")]
    pub StandardOutput: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    StandardOutput: usize,
    #[cfg(feature = "Storage_Streams")]
    pub SetStandardOutput: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    SetStandardOutput: usize,
    #[cfg(feature = "Storage_Streams")]
    pub StandardError: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    StandardError: usize,
    #[cfg(feature = "Storage_Streams")]
    pub SetStandardError: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    SetStandardError: usize,
    pub WorkingDirectory: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetWorkingDirectory: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IProcessLauncherResult(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IProcessLauncherResult {
    type Vtable = IProcessLauncherResult_Vtbl;
}
unsafe impl ::windows::core::Interface for IProcessLauncherResult {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x544c8934_86d8_4991_8e75_ece8a43b6b6d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IProcessLauncherResult_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub ExitCode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IProcessLauncherStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IProcessLauncherStatics {
    type Vtable = IProcessLauncherStatics_Vtbl;
}
unsafe impl ::windows::core::Interface for IProcessLauncherStatics {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x33ab66e7_2d0e_448b_a6a0_c13c3836d09c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IProcessLauncherStatics_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub RunToCompletionAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, filename: *mut ::core::ffi::c_void, args: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RunToCompletionAsync: usize,
    #[cfg(feature = "Foundation")]
    pub RunToCompletionAsyncWithOptions: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, filename: *mut ::core::ffi::c_void, args: *mut ::core::ffi::c_void, options: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RunToCompletionAsyncWithOptions: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IProcessMemoryReport(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IProcessMemoryReport {
    type Vtable = IProcessMemoryReport_Vtbl;
}
unsafe impl ::windows::core::Interface for IProcessMemoryReport {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x087305a8_9b70_4782_8741_3a982b6ce5e4);
}
#[repr(C)]
#[doc(hidden)]
pub struct IProcessMemoryReport_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub PrivateWorkingSetUsage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u64) -> ::windows::core::HRESULT,
    pub TotalWorkingSetUsage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u64) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IProtocolForResultsOperation(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IProtocolForResultsOperation {
    type Vtable = IProtocolForResultsOperation_Vtbl;
}
unsafe impl ::windows::core::Interface for IProtocolForResultsOperation {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd581293a_6de9_4d28_9378_f86782e182bb);
}
#[repr(C)]
#[doc(hidden)]
pub struct IProtocolForResultsOperation_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub ReportCompleted: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, data: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    ReportCompleted: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IRemoteLauncherOptions(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IRemoteLauncherOptions {
    type Vtable = IRemoteLauncherOptions_Vtbl;
}
unsafe impl ::windows::core::Interface for IRemoteLauncherOptions {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9e3a2788_2891_4cdf_a2d6_9dff7d02e693);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRemoteLauncherOptions_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub FallbackUri: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    FallbackUri: usize,
    #[cfg(feature = "Foundation")]
    pub SetFallbackUri: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetFallbackUri: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub PreferredAppIds: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    PreferredAppIds: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IRemoteLauncherStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IRemoteLauncherStatics {
    type Vtable = IRemoteLauncherStatics_Vtbl;
}
unsafe impl ::windows::core::Interface for IRemoteLauncherStatics {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd7db7a93_a30c_48b7_9f21_051026a4e517);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRemoteLauncherStatics_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(all(feature = "Foundation", feature = "System_RemoteSystems"))]
    pub LaunchUriAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, remotesystemconnectionrequest: *mut ::core::ffi::c_void, uri: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "System_RemoteSystems")))]
    LaunchUriAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "System_RemoteSystems"))]
    pub LaunchUriWithOptionsAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, remotesystemconnectionrequest: *mut ::core::ffi::c_void, uri: *mut ::core::ffi::c_void, options: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "System_RemoteSystems")))]
    LaunchUriWithOptionsAsync: usize,
    #[cfg(all(feature = "Foundation_Collections", feature = "System_RemoteSystems"))]
    pub LaunchUriWithDataAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, remotesystemconnectionrequest: *mut ::core::ffi::c_void, uri: *mut ::core::ffi::c_void, options: *mut ::core::ffi::c_void, inputdata: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "System_RemoteSystems")))]
    LaunchUriWithDataAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IShutdownManagerStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IShutdownManagerStatics {
    type Vtable = IShutdownManagerStatics_Vtbl;
}
unsafe impl ::windows::core::Interface for IShutdownManagerStatics {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x72e247ed_dd5b_4d6c_b1d0_c57a7bbb5f94);
}
#[repr(C)]
#[doc(hidden)]
pub struct IShutdownManagerStatics_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub BeginShutdown: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, shutdownkind: ShutdownKind, timeout: super::Foundation::TimeSpan) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    BeginShutdown: usize,
    pub CancelShutdown: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IShutdownManagerStatics2(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IShutdownManagerStatics2 {
    type Vtable = IShutdownManagerStatics2_Vtbl;
}
unsafe impl ::windows::core::Interface for IShutdownManagerStatics2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0f69a02f_9c34_43c7_a8c3_70b30a7f7504);
}
#[repr(C)]
#[doc(hidden)]
pub struct IShutdownManagerStatics2_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub IsPowerStateSupported: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, powerstate: PowerState, result__: *mut bool) -> ::windows::core::HRESULT,
    pub EnterPowerState: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, powerstate: PowerState) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub EnterPowerStateWithTimeSpan: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, powerstate: PowerState, wakeupafter: super::Foundation::TimeSpan) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    EnterPowerStateWithTimeSpan: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ITimeZoneSettingsStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for ITimeZoneSettingsStatics {
    type Vtable = ITimeZoneSettingsStatics_Vtbl;
}
unsafe impl ::windows::core::Interface for ITimeZoneSettingsStatics {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9b3b2bea_a101_41ae_9fbd_028728bab73d);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITimeZoneSettingsStatics_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub CurrentTimeZoneDisplayName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub SupportedTimeZoneDisplayNames: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    SupportedTimeZoneDisplayNames: usize,
    pub CanChangeTimeZone: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub ChangeTimeZoneByDisplayName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, timezonedisplayname: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ITimeZoneSettingsStatics2(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for ITimeZoneSettingsStatics2 {
    type Vtable = ITimeZoneSettingsStatics2_Vtbl;
}
unsafe impl ::windows::core::Interface for ITimeZoneSettingsStatics2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x555c0db8_39a8_49fa_b4f6_a2c7fc2842ec);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITimeZoneSettingsStatics2_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub AutoUpdateTimeZoneAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, timeout: super::Foundation::TimeSpan, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    AutoUpdateTimeZoneAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IUser(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IUser {
    type Vtable = IUser_Vtbl;
}
unsafe impl ::windows::core::Interface for IUser {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xdf9a26c6_e746_4bcd_b5d4_120103c4209b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUser_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub NonRoamableId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub AuthenticationStatus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut UserAuthenticationStatus) -> ::windows::core::HRESULT,
    pub Type: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut UserType) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub GetPropertyAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetPropertyAsync: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub GetPropertiesAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, values: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetPropertiesAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub GetPictureAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, desiredsize: UserPictureSize, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_Streams")))]
    GetPictureAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IUser2(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IUser2 {
    type Vtable = IUser2_Vtbl;
}
unsafe impl ::windows::core::Interface for IUser2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x98ba5628_a6e3_518e_89d9_d3b2b1991a10);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUser2_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub CheckUserAgeConsentGroupAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, consentgroup: UserAgeConsentGroup, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    CheckUserAgeConsentGroupAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IUserAuthenticationStatusChangeDeferral(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IUserAuthenticationStatusChangeDeferral {
    type Vtable = IUserAuthenticationStatusChangeDeferral_Vtbl;
}
unsafe impl ::windows::core::Interface for IUserAuthenticationStatusChangeDeferral {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x88b59568_bb30_42fb_a270_e9902e40efa7);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUserAuthenticationStatusChangeDeferral_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Complete: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IUserAuthenticationStatusChangingEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IUserAuthenticationStatusChangingEventArgs {
    type Vtable = IUserAuthenticationStatusChangingEventArgs_Vtbl;
}
unsafe impl ::windows::core::Interface for IUserAuthenticationStatusChangingEventArgs {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8c030f28_a711_4c1e_ab48_04179c15938f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUserAuthenticationStatusChangingEventArgs_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub GetDeferral: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub User: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub NewStatus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut UserAuthenticationStatus) -> ::windows::core::HRESULT,
    pub CurrentStatus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut UserAuthenticationStatus) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IUserChangedEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IUserChangedEventArgs {
    type Vtable = IUserChangedEventArgs_Vtbl;
}
unsafe impl ::windows::core::Interface for IUserChangedEventArgs {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x086459dc_18c6_48db_bc99_724fb9203ccc);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUserChangedEventArgs_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub User: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IUserChangedEventArgs2(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IUserChangedEventArgs2 {
    type Vtable = IUserChangedEventArgs2_Vtbl;
}
unsafe impl ::windows::core::Interface for IUserChangedEventArgs2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6b2ccb44_6f01_560c_97ad_fc7f32ec581f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUserChangedEventArgs2_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub ChangedPropertyKinds: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    ChangedPropertyKinds: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IUserDeviceAssociationChangedEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IUserDeviceAssociationChangedEventArgs {
    type Vtable = IUserDeviceAssociationChangedEventArgs_Vtbl;
}
unsafe impl ::windows::core::Interface for IUserDeviceAssociationChangedEventArgs {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xbd1f6f6c_bb5d_4d7b_a5f0_c8cd11a38d42);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUserDeviceAssociationChangedEventArgs_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub DeviceId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub NewUser: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub OldUser: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IUserDeviceAssociationStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IUserDeviceAssociationStatics {
    type Vtable = IUserDeviceAssociationStatics_Vtbl;
}
unsafe impl ::windows::core::Interface for IUserDeviceAssociationStatics {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7e491e14_f85a_4c07_8da9_7fe3d0542343);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUserDeviceAssociationStatics_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub FindUserFromDeviceId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, deviceid: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub UserDeviceAssociationChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    UserDeviceAssociationChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveUserDeviceAssociationChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveUserDeviceAssociationChanged: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IUserPicker(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IUserPicker {
    type Vtable = IUserPicker_Vtbl;
}
unsafe impl ::windows::core::Interface for IUserPicker {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7d548008_f1e3_4a6c_8ddc_a9bb0f488aed);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUserPicker_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub AllowGuestAccounts: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetAllowGuestAccounts: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    pub SuggestedSelectedUser: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetSuggestedSelectedUser: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub PickSingleUserAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    PickSingleUserAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IUserPickerStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IUserPickerStatics {
    type Vtable = IUserPickerStatics_Vtbl;
}
unsafe impl ::windows::core::Interface for IUserPickerStatics {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xde3290dc_7e73_4df6_a1ae_4d7eca82b40d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUserPickerStatics_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub IsSupported: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IUserStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IUserStatics {
    type Vtable = IUserStatics_Vtbl;
}
unsafe impl ::windows::core::Interface for IUserStatics {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x155eb23b_242a_45e0_a2e9_3171fc6a7fdd);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUserStatics_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub CreateWatcher: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub FindAllAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    FindAllAsync: usize,
    #[cfg(all(feature = "Foundation_Collections", feature = "deprecated"))]
    pub FindAllAsyncByType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, r#type: UserType, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "deprecated")))]
    FindAllAsyncByType: usize,
    #[cfg(all(feature = "Foundation_Collections", feature = "deprecated"))]
    pub FindAllAsyncByTypeAndStatus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, r#type: UserType, status: UserAuthenticationStatus, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "deprecated")))]
    FindAllAsyncByTypeAndStatus: usize,
    pub GetFromId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, nonroamableid: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IUserStatics2(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IUserStatics2 {
    type Vtable = IUserStatics2_Vtbl;
}
unsafe impl ::windows::core::Interface for IUserStatics2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x74a37e11_2eb5_4487_b0d5_2c6790e013e9);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUserStatics2_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub GetDefault: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IUserWatcher(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IUserWatcher {
    type Vtable = IUserWatcher_Vtbl;
}
unsafe impl ::windows::core::Interface for IUserWatcher {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x155eb23b_242a_45e0_a2e9_3171fc6a7fbb);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUserWatcher_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Status: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut UserWatcherStatus) -> ::windows::core::HRESULT,
    pub Start: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Stop: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub Added: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Added: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveAdded: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveAdded: usize,
    #[cfg(feature = "Foundation")]
    pub Removed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Removed: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveRemoved: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveRemoved: usize,
    #[cfg(feature = "Foundation")]
    pub Updated: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Updated: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveUpdated: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveUpdated: usize,
    #[cfg(feature = "Foundation")]
    pub AuthenticationStatusChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    AuthenticationStatusChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveAuthenticationStatusChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveAuthenticationStatusChanged: usize,
    #[cfg(feature = "Foundation")]
    pub AuthenticationStatusChanging: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    AuthenticationStatusChanging: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveAuthenticationStatusChanging: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveAuthenticationStatusChanging: usize,
    #[cfg(feature = "Foundation")]
    pub EnumerationCompleted: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    EnumerationCompleted: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveEnumerationCompleted: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveEnumerationCompleted: usize,
    #[cfg(feature = "Foundation")]
    pub Stopped: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Stopped: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveStopped: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveStopped: usize,
}
#[doc = "*Required features: `\"System\"`*"]
#[repr(transparent)]
pub struct AppActivationResult(::windows::core::IUnknown);
impl AppActivationResult {
    pub fn ExtendedError(&self) -> ::windows::core::Result<::windows::core::HRESULT> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ExtendedError)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn AppResourceGroupInfo(&self) -> ::windows::core::Result<AppResourceGroupInfo> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).AppResourceGroupInfo)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
}
impl ::core::clone::Clone for AppActivationResult {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for AppActivationResult {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AppActivationResult {}
impl ::core::fmt::Debug for AppActivationResult {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AppActivationResult").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for AppActivationResult {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.System.AppActivationResult;{6b528900-f46e-4eb0-aa6c-38af557cf9ed})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for AppActivationResult {
    type Vtable = IAppActivationResult_Vtbl;
}
unsafe impl ::windows::core::Interface for AppActivationResult {
    const IID: ::windows::core::GUID = <IAppActivationResult as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for AppActivationResult {
    const NAME: &'static str = "Windows.System.AppActivationResult";
}
::windows::core::interface_hierarchy!(AppActivationResult, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for AppActivationResult {}
unsafe impl ::core::marker::Sync for AppActivationResult {}
#[doc = "*Required features: `\"System\"`*"]
#[repr(transparent)]
pub struct AppDiagnosticInfo(::windows::core::IUnknown);
impl AppDiagnosticInfo {
    #[doc = "*Required features: `\"ApplicationModel\"`*"]
    #[cfg(feature = "ApplicationModel")]
    pub fn AppInfo(&self) -> ::windows::core::Result<super::ApplicationModel::AppInfo> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).AppInfo)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetResourceGroups(&self) -> ::windows::core::Result<super::Foundation::Collections::IVector<AppResourceGroupInfo>> {
        let this = &::windows::core::Interface::cast::<IAppDiagnosticInfo2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetResourceGroups)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn CreateResourceGroupWatcher(&self) -> ::windows::core::Result<AppResourceGroupInfoWatcher> {
        let this = &::windows::core::Interface::cast::<IAppDiagnosticInfo2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CreateResourceGroupWatcher)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn LaunchAsync(&self) -> ::windows::core::Result<super::Foundation::IAsyncOperation<AppActivationResult>> {
        let this = &::windows::core::Interface::cast::<IAppDiagnosticInfo3>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).LaunchAsync)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn RequestInfoAsync() -> ::windows::core::Result<super::Foundation::IAsyncOperation<super::Foundation::Collections::IVector<AppDiagnosticInfo>>> {
        Self::IAppDiagnosticInfoStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).RequestInfoAsync)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    pub fn CreateWatcher() -> ::windows::core::Result<AppDiagnosticInfoWatcher> {
        Self::IAppDiagnosticInfoStatics2(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CreateWatcher)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RequestAccessAsync() -> ::windows::core::Result<super::Foundation::IAsyncOperation<DiagnosticAccessStatus>> {
        Self::IAppDiagnosticInfoStatics2(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).RequestAccessAsync)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn RequestInfoForPackageAsync(packagefamilyname: &::windows::core::HSTRING) -> ::windows::core::Result<super::Foundation::IAsyncOperation<super::Foundation::Collections::IVector<AppDiagnosticInfo>>> {
        Self::IAppDiagnosticInfoStatics2(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).RequestInfoForPackageAsync)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(packagefamilyname), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn RequestInfoForAppAsync() -> ::windows::core::Result<super::Foundation::IAsyncOperation<super::Foundation::Collections::IVector<AppDiagnosticInfo>>> {
        Self::IAppDiagnosticInfoStatics2(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).RequestInfoForAppAsync)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn RequestInfoForAppUserModelId(appusermodelid: &::windows::core::HSTRING) -> ::windows::core::Result<super::Foundation::IAsyncOperation<super::Foundation::Collections::IVector<AppDiagnosticInfo>>> {
        Self::IAppDiagnosticInfoStatics2(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).RequestInfoForAppUserModelId)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(appusermodelid), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IAppDiagnosticInfoStatics<R, F: FnOnce(&IAppDiagnosticInfoStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<AppDiagnosticInfo, IAppDiagnosticInfoStatics> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn IAppDiagnosticInfoStatics2<R, F: FnOnce(&IAppDiagnosticInfoStatics2) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<AppDiagnosticInfo, IAppDiagnosticInfoStatics2> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::clone::Clone for AppDiagnosticInfo {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for AppDiagnosticInfo {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AppDiagnosticInfo {}
impl ::core::fmt::Debug for AppDiagnosticInfo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AppDiagnosticInfo").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for AppDiagnosticInfo {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.System.AppDiagnosticInfo;{e348a69a-8889-4ca3-be07-d5ffff5f0804})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for AppDiagnosticInfo {
    type Vtable = IAppDiagnosticInfo_Vtbl;
}
unsafe impl ::windows::core::Interface for AppDiagnosticInfo {
    const IID: ::windows::core::GUID = <IAppDiagnosticInfo as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for AppDiagnosticInfo {
    const NAME: &'static str = "Windows.System.AppDiagnosticInfo";
}
::windows::core::interface_hierarchy!(AppDiagnosticInfo, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for AppDiagnosticInfo {}
unsafe impl ::core::marker::Sync for AppDiagnosticInfo {}
#[doc = "*Required features: `\"System\"`*"]
#[repr(transparent)]
pub struct AppDiagnosticInfoWatcher(::windows::core::IUnknown);
impl AppDiagnosticInfoWatcher {
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Added(&self, handler: &super::Foundation::TypedEventHandler<AppDiagnosticInfoWatcher, AppDiagnosticInfoWatcherEventArgs>) -> ::windows::core::Result<super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Added)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(handler), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveAdded(&self, token: super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).RemoveAdded)(::windows::core::Vtable::as_raw(this), token).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Removed(&self, handler: &super::Foundation::TypedEventHandler<AppDiagnosticInfoWatcher, AppDiagnosticInfoWatcherEventArgs>) -> ::windows::core::Result<super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Removed)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(handler), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveRemoved(&self, token: super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).RemoveRemoved)(::windows::core::Vtable::as_raw(this), token).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn EnumerationCompleted(&self, handler: &super::Foundation::TypedEventHandler<AppDiagnosticInfoWatcher, ::windows::core::IInspectable>) -> ::windows::core::Result<super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).EnumerationCompleted)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(handler), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveEnumerationCompleted(&self, token: super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).RemoveEnumerationCompleted)(::windows::core::Vtable::as_raw(this), token).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Stopped(&self, handler: &super::Foundation::TypedEventHandler<AppDiagnosticInfoWatcher, ::windows::core::IInspectable>) -> ::windows::core::Result<super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Stopped)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(handler), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveStopped(&self, token: super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).RemoveStopped)(::windows::core::Vtable::as_raw(this), token).ok() }
    }
    pub fn Status(&self) -> ::windows::core::Result<AppDiagnosticInfoWatcherStatus> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Status)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn Start(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).Start)(::windows::core::Vtable::as_raw(this)).ok() }
    }
    pub fn Stop(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).Stop)(::windows::core::Vtable::as_raw(this)).ok() }
    }
}
impl ::core::clone::Clone for AppDiagnosticInfoWatcher {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for AppDiagnosticInfoWatcher {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AppDiagnosticInfoWatcher {}
impl ::core::fmt::Debug for AppDiagnosticInfoWatcher {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AppDiagnosticInfoWatcher").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for AppDiagnosticInfoWatcher {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.System.AppDiagnosticInfoWatcher;{75575070-01d3-489a-9325-52f9cc6ede0a})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for AppDiagnosticInfoWatcher {
    type Vtable = IAppDiagnosticInfoWatcher_Vtbl;
}
unsafe impl ::windows::core::Interface for AppDiagnosticInfoWatcher {
    const IID: ::windows::core::GUID = <IAppDiagnosticInfoWatcher as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for AppDiagnosticInfoWatcher {
    const NAME: &'static str = "Windows.System.AppDiagnosticInfoWatcher";
}
::windows::core::interface_hierarchy!(AppDiagnosticInfoWatcher, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for AppDiagnosticInfoWatcher {}
unsafe impl ::core::marker::Sync for AppDiagnosticInfoWatcher {}
#[doc = "*Required features: `\"System\"`*"]
#[repr(transparent)]
pub struct AppDiagnosticInfoWatcherEventArgs(::windows::core::IUnknown);
impl AppDiagnosticInfoWatcherEventArgs {
    pub fn AppDiagnosticInfo(&self) -> ::windows::core::Result<AppDiagnosticInfo> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).AppDiagnosticInfo)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
}
impl ::core::clone::Clone for AppDiagnosticInfoWatcherEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for AppDiagnosticInfoWatcherEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AppDiagnosticInfoWatcherEventArgs {}
impl ::core::fmt::Debug for AppDiagnosticInfoWatcherEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AppDiagnosticInfoWatcherEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for AppDiagnosticInfoWatcherEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.System.AppDiagnosticInfoWatcherEventArgs;{7017c716-e1da-4c65-99df-046dff5be71a})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for AppDiagnosticInfoWatcherEventArgs {
    type Vtable = IAppDiagnosticInfoWatcherEventArgs_Vtbl;
}
unsafe impl ::windows::core::Interface for AppDiagnosticInfoWatcherEventArgs {
    const IID: ::windows::core::GUID = <IAppDiagnosticInfoWatcherEventArgs as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for AppDiagnosticInfoWatcherEventArgs {
    const NAME: &'static str = "Windows.System.AppDiagnosticInfoWatcherEventArgs";
}
::windows::core::interface_hierarchy!(AppDiagnosticInfoWatcherEventArgs, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for AppDiagnosticInfoWatcherEventArgs {}
unsafe impl ::core::marker::Sync for AppDiagnosticInfoWatcherEventArgs {}
#[doc = "*Required features: `\"System\"`*"]
#[repr(transparent)]
pub struct AppExecutionStateChangeResult(::windows::core::IUnknown);
impl AppExecutionStateChangeResult {
    pub fn ExtendedError(&self) -> ::windows::core::Result<::windows::core::HRESULT> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ExtendedError)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
}
impl ::core::clone::Clone for AppExecutionStateChangeResult {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for AppExecutionStateChangeResult {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AppExecutionStateChangeResult {}
impl ::core::fmt::Debug for AppExecutionStateChangeResult {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AppExecutionStateChangeResult").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for AppExecutionStateChangeResult {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.System.AppExecutionStateChangeResult;{6f039bf0-f91b-4df8-ae77-3033ccb69114})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for AppExecutionStateChangeResult {
    type Vtable = IAppExecutionStateChangeResult_Vtbl;
}
unsafe impl ::windows::core::Interface for AppExecutionStateChangeResult {
    const IID: ::windows::core::GUID = <IAppExecutionStateChangeResult as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for AppExecutionStateChangeResult {
    const NAME: &'static str = "Windows.System.AppExecutionStateChangeResult";
}
::windows::core::interface_hierarchy!(AppExecutionStateChangeResult, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for AppExecutionStateChangeResult {}
unsafe impl ::core::marker::Sync for AppExecutionStateChangeResult {}
#[doc = "*Required features: `\"System\"`*"]
#[repr(transparent)]
pub struct AppMemoryReport(::windows::core::IUnknown);
impl AppMemoryReport {
    pub fn PrivateCommitUsage(&self) -> ::windows::core::Result<u64> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).PrivateCommitUsage)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn PeakPrivateCommitUsage(&self) -> ::windows::core::Result<u64> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).PeakPrivateCommitUsage)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn TotalCommitUsage(&self) -> ::windows::core::Result<u64> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).TotalCommitUsage)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn TotalCommitLimit(&self) -> ::windows::core::Result<u64> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).TotalCommitLimit)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn ExpectedTotalCommitLimit(&self) -> ::windows::core::Result<u64> {
        let this = &::windows::core::Interface::cast::<IAppMemoryReport2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ExpectedTotalCommitLimit)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
}
impl ::core::clone::Clone for AppMemoryReport {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for AppMemoryReport {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AppMemoryReport {}
impl ::core::fmt::Debug for AppMemoryReport {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AppMemoryReport").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for AppMemoryReport {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.System.AppMemoryReport;{6d65339b-4d6f-45bc-9c5e-e49b3ff2758d})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for AppMemoryReport {
    type Vtable = IAppMemoryReport_Vtbl;
}
unsafe impl ::windows::core::Interface for AppMemoryReport {
    const IID: ::windows::core::GUID = <IAppMemoryReport as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for AppMemoryReport {
    const NAME: &'static str = "Windows.System.AppMemoryReport";
}
::windows::core::interface_hierarchy!(AppMemoryReport, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for AppMemoryReport {}
unsafe impl ::core::marker::Sync for AppMemoryReport {}
#[doc = "*Required features: `\"System\"`*"]
#[repr(transparent)]
pub struct AppMemoryUsageLimitChangingEventArgs(::windows::core::IUnknown);
impl AppMemoryUsageLimitChangingEventArgs {
    pub fn OldLimit(&self) -> ::windows::core::Result<u64> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).OldLimit)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn NewLimit(&self) -> ::windows::core::Result<u64> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).NewLimit)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
}
impl ::core::clone::Clone for AppMemoryUsageLimitChangingEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for AppMemoryUsageLimitChangingEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AppMemoryUsageLimitChangingEventArgs {}
impl ::core::fmt::Debug for AppMemoryUsageLimitChangingEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AppMemoryUsageLimitChangingEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for AppMemoryUsageLimitChangingEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.System.AppMemoryUsageLimitChangingEventArgs;{79f86664-feca-4da5-9e40-2bc63efdc979})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for AppMemoryUsageLimitChangingEventArgs {
    type Vtable = IAppMemoryUsageLimitChangingEventArgs_Vtbl;
}
unsafe impl ::windows::core::Interface for AppMemoryUsageLimitChangingEventArgs {
    const IID: ::windows::core::GUID = <IAppMemoryUsageLimitChangingEventArgs as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for AppMemoryUsageLimitChangingEventArgs {
    const NAME: &'static str = "Windows.System.AppMemoryUsageLimitChangingEventArgs";
}
::windows::core::interface_hierarchy!(AppMemoryUsageLimitChangingEventArgs, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for AppMemoryUsageLimitChangingEventArgs {}
unsafe impl ::core::marker::Sync for AppMemoryUsageLimitChangingEventArgs {}
#[doc = "*Required features: `\"System\"`*"]
#[repr(transparent)]
pub struct AppResourceGroupBackgroundTaskReport(::windows::core::IUnknown);
impl AppResourceGroupBackgroundTaskReport {
    pub fn TaskId(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).TaskId)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn Name(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Name)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn Trigger(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Trigger)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn EntryPoint(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).EntryPoint)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
}
impl ::core::clone::Clone for AppResourceGroupBackgroundTaskReport {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for AppResourceGroupBackgroundTaskReport {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AppResourceGroupBackgroundTaskReport {}
impl ::core::fmt::Debug for AppResourceGroupBackgroundTaskReport {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AppResourceGroupBackgroundTaskReport").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for AppResourceGroupBackgroundTaskReport {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.System.AppResourceGroupBackgroundTaskReport;{2566e74e-b05d-40c2-9dc1-1a4f039ea120})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for AppResourceGroupBackgroundTaskReport {
    type Vtable = IAppResourceGroupBackgroundTaskReport_Vtbl;
}
unsafe impl ::windows::core::Interface for AppResourceGroupBackgroundTaskReport {
    const IID: ::windows::core::GUID = <IAppResourceGroupBackgroundTaskReport as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for AppResourceGroupBackgroundTaskReport {
    const NAME: &'static str = "Windows.System.AppResourceGroupBackgroundTaskReport";
}
::windows::core::interface_hierarchy!(AppResourceGroupBackgroundTaskReport, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for AppResourceGroupBackgroundTaskReport {}
unsafe impl ::core::marker::Sync for AppResourceGroupBackgroundTaskReport {}
#[doc = "*Required features: `\"System\"`*"]
#[repr(transparent)]
pub struct AppResourceGroupInfo(::windows::core::IUnknown);
impl AppResourceGroupInfo {
    pub fn InstanceId(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).InstanceId)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn IsShared(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).IsShared)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetBackgroundTaskReports(&self) -> ::windows::core::Result<super::Foundation::Collections::IVector<AppResourceGroupBackgroundTaskReport>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetBackgroundTaskReports)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn GetMemoryReport(&self) -> ::windows::core::Result<AppResourceGroupMemoryReport> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetMemoryReport)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`, `\"System_Diagnostics\"`*"]
    #[cfg(all(feature = "Foundation_Collections", feature = "System_Diagnostics"))]
    pub fn GetProcessDiagnosticInfos(&self) -> ::windows::core::Result<super::Foundation::Collections::IVector<Diagnostics::ProcessDiagnosticInfo>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetProcessDiagnosticInfos)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn GetStateReport(&self) -> ::windows::core::Result<AppResourceGroupStateReport> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetStateReport)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn StartSuspendAsync(&self) -> ::windows::core::Result<super::Foundation::IAsyncOperation<AppExecutionStateChangeResult>> {
        let this = &::windows::core::Interface::cast::<IAppResourceGroupInfo2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).StartSuspendAsync)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn StartResumeAsync(&self) -> ::windows::core::Result<super::Foundation::IAsyncOperation<AppExecutionStateChangeResult>> {
        let this = &::windows::core::Interface::cast::<IAppResourceGroupInfo2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).StartResumeAsync)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn StartTerminateAsync(&self) -> ::windows::core::Result<super::Foundation::IAsyncOperation<AppExecutionStateChangeResult>> {
        let this = &::windows::core::Interface::cast::<IAppResourceGroupInfo2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).StartTerminateAsync)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
}
impl ::core::clone::Clone for AppResourceGroupInfo {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for AppResourceGroupInfo {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AppResourceGroupInfo {}
impl ::core::fmt::Debug for AppResourceGroupInfo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AppResourceGroupInfo").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for AppResourceGroupInfo {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.System.AppResourceGroupInfo;{b913f77a-e807-49f4-845e-7b8bdcfe8ee7})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for AppResourceGroupInfo {
    type Vtable = IAppResourceGroupInfo_Vtbl;
}
unsafe impl ::windows::core::Interface for AppResourceGroupInfo {
    const IID: ::windows::core::GUID = <IAppResourceGroupInfo as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for AppResourceGroupInfo {
    const NAME: &'static str = "Windows.System.AppResourceGroupInfo";
}
::windows::core::interface_hierarchy!(AppResourceGroupInfo, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for AppResourceGroupInfo {}
unsafe impl ::core::marker::Sync for AppResourceGroupInfo {}
#[doc = "*Required features: `\"System\"`*"]
#[repr(transparent)]
pub struct AppResourceGroupInfoWatcher(::windows::core::IUnknown);
impl AppResourceGroupInfoWatcher {
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Added(&self, handler: &super::Foundation::TypedEventHandler<AppResourceGroupInfoWatcher, AppResourceGroupInfoWatcherEventArgs>) -> ::windows::core::Result<super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Added)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(handler), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveAdded(&self, token: super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).RemoveAdded)(::windows::core::Vtable::as_raw(this), token).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Removed(&self, handler: &super::Foundation::TypedEventHandler<AppResourceGroupInfoWatcher, AppResourceGroupInfoWatcherEventArgs>) -> ::windows::core::Result<super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Removed)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(handler), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveRemoved(&self, token: super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).RemoveRemoved)(::windows::core::Vtable::as_raw(this), token).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn EnumerationCompleted(&self, handler: &super::Foundation::TypedEventHandler<AppResourceGroupInfoWatcher, ::windows::core::IInspectable>) -> ::windows::core::Result<super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).EnumerationCompleted)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(handler), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveEnumerationCompleted(&self, token: super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).RemoveEnumerationCompleted)(::windows::core::Vtable::as_raw(this), token).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Stopped(&self, handler: &super::Foundation::TypedEventHandler<AppResourceGroupInfoWatcher, ::windows::core::IInspectable>) -> ::windows::core::Result<super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Stopped)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(handler), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveStopped(&self, token: super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).RemoveStopped)(::windows::core::Vtable::as_raw(this), token).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ExecutionStateChanged(&self, handler: &super::Foundation::TypedEventHandler<AppResourceGroupInfoWatcher, AppResourceGroupInfoWatcherExecutionStateChangedEventArgs>) -> ::windows::core::Result<super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ExecutionStateChanged)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(handler), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveExecutionStateChanged(&self, token: super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).RemoveExecutionStateChanged)(::windows::core::Vtable::as_raw(this), token).ok() }
    }
    pub fn Status(&self) -> ::windows::core::Result<AppResourceGroupInfoWatcherStatus> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Status)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn Start(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).Start)(::windows::core::Vtable::as_raw(this)).ok() }
    }
    pub fn Stop(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).Stop)(::windows::core::Vtable::as_raw(this)).ok() }
    }
}
impl ::core::clone::Clone for AppResourceGroupInfoWatcher {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for AppResourceGroupInfoWatcher {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AppResourceGroupInfoWatcher {}
impl ::core::fmt::Debug for AppResourceGroupInfoWatcher {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AppResourceGroupInfoWatcher").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for AppResourceGroupInfoWatcher {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.System.AppResourceGroupInfoWatcher;{d9b0a0fd-6e5a-4c72-8b17-09fec4a212bd})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for AppResourceGroupInfoWatcher {
    type Vtable = IAppResourceGroupInfoWatcher_Vtbl;
}
unsafe impl ::windows::core::Interface for AppResourceGroupInfoWatcher {
    const IID: ::windows::core::GUID = <IAppResourceGroupInfoWatcher as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for AppResourceGroupInfoWatcher {
    const NAME: &'static str = "Windows.System.AppResourceGroupInfoWatcher";
}
::windows::core::interface_hierarchy!(AppResourceGroupInfoWatcher, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for AppResourceGroupInfoWatcher {}
unsafe impl ::core::marker::Sync for AppResourceGroupInfoWatcher {}
#[doc = "*Required features: `\"System\"`*"]
#[repr(transparent)]
pub struct AppResourceGroupInfoWatcherEventArgs(::windows::core::IUnknown);
impl AppResourceGroupInfoWatcherEventArgs {
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn AppDiagnosticInfos(&self) -> ::windows::core::Result<super::Foundation::Collections::IVectorView<AppDiagnosticInfo>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).AppDiagnosticInfos)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn AppResourceGroupInfo(&self) -> ::windows::core::Result<AppResourceGroupInfo> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).AppResourceGroupInfo)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
}
impl ::core::clone::Clone for AppResourceGroupInfoWatcherEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for AppResourceGroupInfoWatcherEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AppResourceGroupInfoWatcherEventArgs {}
impl ::core::fmt::Debug for AppResourceGroupInfoWatcherEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AppResourceGroupInfoWatcherEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for AppResourceGroupInfoWatcherEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.System.AppResourceGroupInfoWatcherEventArgs;{7a787637-6302-4d2f-bf89-1c12d0b2a6b9})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for AppResourceGroupInfoWatcherEventArgs {
    type Vtable = IAppResourceGroupInfoWatcherEventArgs_Vtbl;
}
unsafe impl ::windows::core::Interface for AppResourceGroupInfoWatcherEventArgs {
    const IID: ::windows::core::GUID = <IAppResourceGroupInfoWatcherEventArgs as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for AppResourceGroupInfoWatcherEventArgs {
    const NAME: &'static str = "Windows.System.AppResourceGroupInfoWatcherEventArgs";
}
::windows::core::interface_hierarchy!(AppResourceGroupInfoWatcherEventArgs, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for AppResourceGroupInfoWatcherEventArgs {}
unsafe impl ::core::marker::Sync for AppResourceGroupInfoWatcherEventArgs {}
#[doc = "*Required features: `\"System\"`*"]
#[repr(transparent)]
pub struct AppResourceGroupInfoWatcherExecutionStateChangedEventArgs(::windows::core::IUnknown);
impl AppResourceGroupInfoWatcherExecutionStateChangedEventArgs {
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn AppDiagnosticInfos(&self) -> ::windows::core::Result<super::Foundation::Collections::IVectorView<AppDiagnosticInfo>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).AppDiagnosticInfos)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn AppResourceGroupInfo(&self) -> ::windows::core::Result<AppResourceGroupInfo> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).AppResourceGroupInfo)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
}
impl ::core::clone::Clone for AppResourceGroupInfoWatcherExecutionStateChangedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for AppResourceGroupInfoWatcherExecutionStateChangedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AppResourceGroupInfoWatcherExecutionStateChangedEventArgs {}
impl ::core::fmt::Debug for AppResourceGroupInfoWatcherExecutionStateChangedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AppResourceGroupInfoWatcherExecutionStateChangedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for AppResourceGroupInfoWatcherExecutionStateChangedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.System.AppResourceGroupInfoWatcherExecutionStateChangedEventArgs;{1bdbedd7-fee6-4fd4-98dd-e92a2cc299f3})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for AppResourceGroupInfoWatcherExecutionStateChangedEventArgs {
    type Vtable = IAppResourceGroupInfoWatcherExecutionStateChangedEventArgs_Vtbl;
}
unsafe impl ::windows::core::Interface for AppResourceGroupInfoWatcherExecutionStateChangedEventArgs {
    const IID: ::windows::core::GUID = <IAppResourceGroupInfoWatcherExecutionStateChangedEventArgs as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for AppResourceGroupInfoWatcherExecutionStateChangedEventArgs {
    const NAME: &'static str = "Windows.System.AppResourceGroupInfoWatcherExecutionStateChangedEventArgs";
}
::windows::core::interface_hierarchy!(AppResourceGroupInfoWatcherExecutionStateChangedEventArgs, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for AppResourceGroupInfoWatcherExecutionStateChangedEventArgs {}
unsafe impl ::core::marker::Sync for AppResourceGroupInfoWatcherExecutionStateChangedEventArgs {}
#[doc = "*Required features: `\"System\"`*"]
#[repr(transparent)]
pub struct AppResourceGroupMemoryReport(::windows::core::IUnknown);
impl AppResourceGroupMemoryReport {
    pub fn CommitUsageLimit(&self) -> ::windows::core::Result<u64> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CommitUsageLimit)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn CommitUsageLevel(&self) -> ::windows::core::Result<AppMemoryUsageLevel> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CommitUsageLevel)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn PrivateCommitUsage(&self) -> ::windows::core::Result<u64> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).PrivateCommitUsage)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn TotalCommitUsage(&self) -> ::windows::core::Result<u64> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).TotalCommitUsage)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
}
impl ::core::clone::Clone for AppResourceGroupMemoryReport {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for AppResourceGroupMemoryReport {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AppResourceGroupMemoryReport {}
impl ::core::fmt::Debug for AppResourceGroupMemoryReport {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AppResourceGroupMemoryReport").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for AppResourceGroupMemoryReport {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.System.AppResourceGroupMemoryReport;{2c8c06b1-7db1-4c51-a225-7fae2d49e431})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for AppResourceGroupMemoryReport {
    type Vtable = IAppResourceGroupMemoryReport_Vtbl;
}
unsafe impl ::windows::core::Interface for AppResourceGroupMemoryReport {
    const IID: ::windows::core::GUID = <IAppResourceGroupMemoryReport as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for AppResourceGroupMemoryReport {
    const NAME: &'static str = "Windows.System.AppResourceGroupMemoryReport";
}
::windows::core::interface_hierarchy!(AppResourceGroupMemoryReport, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for AppResourceGroupMemoryReport {}
unsafe impl ::core::marker::Sync for AppResourceGroupMemoryReport {}
#[doc = "*Required features: `\"System\"`*"]
#[repr(transparent)]
pub struct AppResourceGroupStateReport(::windows::core::IUnknown);
impl AppResourceGroupStateReport {
    pub fn ExecutionState(&self) -> ::windows::core::Result<AppResourceGroupExecutionState> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ExecutionState)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn EnergyQuotaState(&self) -> ::windows::core::Result<AppResourceGroupEnergyQuotaState> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).EnergyQuotaState)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
}
impl ::core::clone::Clone for AppResourceGroupStateReport {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for AppResourceGroupStateReport {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AppResourceGroupStateReport {}
impl ::core::fmt::Debug for AppResourceGroupStateReport {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AppResourceGroupStateReport").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for AppResourceGroupStateReport {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.System.AppResourceGroupStateReport;{52849f18-2f70-4236-ab40-d04db0c7b931})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for AppResourceGroupStateReport {
    type Vtable = IAppResourceGroupStateReport_Vtbl;
}
unsafe impl ::windows::core::Interface for AppResourceGroupStateReport {
    const IID: ::windows::core::GUID = <IAppResourceGroupStateReport as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for AppResourceGroupStateReport {
    const NAME: &'static str = "Windows.System.AppResourceGroupStateReport";
}
::windows::core::interface_hierarchy!(AppResourceGroupStateReport, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for AppResourceGroupStateReport {}
unsafe impl ::core::marker::Sync for AppResourceGroupStateReport {}
#[doc = "*Required features: `\"System\"`*"]
#[repr(transparent)]
pub struct AppUriHandlerHost(::windows::core::IUnknown);
impl AppUriHandlerHost {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IGenericFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<AppUriHandlerHost, ::windows::core::IGenericFactory> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn Name(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Name)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetName(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetName)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn IsEnabled(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IAppUriHandlerHost2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).IsEnabled)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetIsEnabled(&self, value: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IAppUriHandlerHost2>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).SetIsEnabled)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
    pub fn CreateInstance(name: &::windows::core::HSTRING) -> ::windows::core::Result<AppUriHandlerHost> {
        Self::IAppUriHandlerHostFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CreateInstance)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(name), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IAppUriHandlerHostFactory<R, F: FnOnce(&IAppUriHandlerHostFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<AppUriHandlerHost, IAppUriHandlerHostFactory> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::clone::Clone for AppUriHandlerHost {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for AppUriHandlerHost {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AppUriHandlerHost {}
impl ::core::fmt::Debug for AppUriHandlerHost {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AppUriHandlerHost").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for AppUriHandlerHost {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.System.AppUriHandlerHost;{5d50cac5-92d2-5409-b56f-7f73e10ea4c3})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for AppUriHandlerHost {
    type Vtable = IAppUriHandlerHost_Vtbl;
}
unsafe impl ::windows::core::Interface for AppUriHandlerHost {
    const IID: ::windows::core::GUID = <IAppUriHandlerHost as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for AppUriHandlerHost {
    const NAME: &'static str = "Windows.System.AppUriHandlerHost";
}
::windows::core::interface_hierarchy!(AppUriHandlerHost, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for AppUriHandlerHost {}
unsafe impl ::core::marker::Sync for AppUriHandlerHost {}
#[doc = "*Required features: `\"System\"`*"]
#[repr(transparent)]
pub struct AppUriHandlerRegistration(::windows::core::IUnknown);
impl AppUriHandlerRegistration {
    pub fn Name(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Name)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn User(&self) -> ::windows::core::Result<User> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).User)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetAppAddedHostsAsync(&self) -> ::windows::core::Result<super::Foundation::IAsyncOperation<super::Foundation::Collections::IVector<AppUriHandlerHost>>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetAppAddedHostsAsync)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn SetAppAddedHostsAsync<P0, E0>(&self, hosts: P0) -> ::windows::core::Result<super::Foundation::IAsyncAction>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<super::Foundation::Collections::IIterable<AppUriHandlerHost>>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).SetAppAddedHostsAsync)(::windows::core::Vtable::as_raw(this), hosts.try_into().map_err(|e| e.into())?.abi(), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetAllHosts(&self) -> ::windows::core::Result<super::Foundation::Collections::IVector<AppUriHandlerHost>> {
        let this = &::windows::core::Interface::cast::<IAppUriHandlerRegistration2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetAllHosts)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn UpdateHosts<P0, E0>(&self, hosts: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<super::Foundation::Collections::IIterable<AppUriHandlerHost>>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = &::windows::core::Interface::cast::<IAppUriHandlerRegistration2>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).UpdateHosts)(::windows::core::Vtable::as_raw(this), hosts.try_into().map_err(|e| e.into())?.abi()).ok() }
    }
    pub fn PackageFamilyName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IAppUriHandlerRegistration2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).PackageFamilyName)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
}
impl ::core::clone::Clone for AppUriHandlerRegistration {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for AppUriHandlerRegistration {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AppUriHandlerRegistration {}
impl ::core::fmt::Debug for AppUriHandlerRegistration {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AppUriHandlerRegistration").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for AppUriHandlerRegistration {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.System.AppUriHandlerRegistration;{6f73aeb1-4569-5c3f-9ba0-99123eea32c3})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for AppUriHandlerRegistration {
    type Vtable = IAppUriHandlerRegistration_Vtbl;
}
unsafe impl ::windows::core::Interface for AppUriHandlerRegistration {
    const IID: ::windows::core::GUID = <IAppUriHandlerRegistration as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for AppUriHandlerRegistration {
    const NAME: &'static str = "Windows.System.AppUriHandlerRegistration";
}
::windows::core::interface_hierarchy!(AppUriHandlerRegistration, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for AppUriHandlerRegistration {}
unsafe impl ::core::marker::Sync for AppUriHandlerRegistration {}
#[doc = "*Required features: `\"System\"`*"]
#[repr(transparent)]
pub struct AppUriHandlerRegistrationManager(::windows::core::IUnknown);
impl AppUriHandlerRegistrationManager {
    pub fn User(&self) -> ::windows::core::Result<User> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).User)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn TryGetRegistration(&self, name: &::windows::core::HSTRING) -> ::windows::core::Result<AppUriHandlerRegistration> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).TryGetRegistration)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(name), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn PackageFamilyName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IAppUriHandlerRegistrationManager2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).PackageFamilyName)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn GetDefault() -> ::windows::core::Result<AppUriHandlerRegistrationManager> {
        Self::IAppUriHandlerRegistrationManagerStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetDefault)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    pub fn GetForUser(user: &User) -> ::windows::core::Result<AppUriHandlerRegistrationManager> {
        Self::IAppUriHandlerRegistrationManagerStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetForUser)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(user), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    pub fn GetForPackage(packagefamilyname: &::windows::core::HSTRING) -> ::windows::core::Result<AppUriHandlerRegistrationManager> {
        Self::IAppUriHandlerRegistrationManagerStatics2(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetForPackage)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(packagefamilyname), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    pub fn GetForPackageForUser(packagefamilyname: &::windows::core::HSTRING, user: &User) -> ::windows::core::Result<AppUriHandlerRegistrationManager> {
        Self::IAppUriHandlerRegistrationManagerStatics2(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetForPackageForUser)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(packagefamilyname), ::core::mem::transmute_copy(user), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IAppUriHandlerRegistrationManagerStatics<R, F: FnOnce(&IAppUriHandlerRegistrationManagerStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<AppUriHandlerRegistrationManager, IAppUriHandlerRegistrationManagerStatics> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn IAppUriHandlerRegistrationManagerStatics2<R, F: FnOnce(&IAppUriHandlerRegistrationManagerStatics2) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<AppUriHandlerRegistrationManager, IAppUriHandlerRegistrationManagerStatics2> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::clone::Clone for AppUriHandlerRegistrationManager {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for AppUriHandlerRegistrationManager {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AppUriHandlerRegistrationManager {}
impl ::core::fmt::Debug for AppUriHandlerRegistrationManager {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AppUriHandlerRegistrationManager").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for AppUriHandlerRegistrationManager {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.System.AppUriHandlerRegistrationManager;{e62c9a52-ac94-5750-ac1b-6cfb6f250263})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for AppUriHandlerRegistrationManager {
    type Vtable = IAppUriHandlerRegistrationManager_Vtbl;
}
unsafe impl ::windows::core::Interface for AppUriHandlerRegistrationManager {
    const IID: ::windows::core::GUID = <IAppUriHandlerRegistrationManager as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for AppUriHandlerRegistrationManager {
    const NAME: &'static str = "Windows.System.AppUriHandlerRegistrationManager";
}
::windows::core::interface_hierarchy!(AppUriHandlerRegistrationManager, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for AppUriHandlerRegistrationManager {}
unsafe impl ::core::marker::Sync for AppUriHandlerRegistrationManager {}
#[doc = "*Required features: `\"System\"`*"]
pub struct DateTimeSettings;
impl DateTimeSettings {
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetSystemDateTime(utcdatetime: super::Foundation::DateTime) -> ::windows::core::Result<()> {
        Self::IDateTimeSettingsStatics(|this| unsafe { (::windows::core::Vtable::vtable(this).SetSystemDateTime)(::windows::core::Vtable::as_raw(this), utcdatetime).ok() })
    }
    #[doc(hidden)]
    pub fn IDateTimeSettingsStatics<R, F: FnOnce(&IDateTimeSettingsStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<DateTimeSettings, IDateTimeSettingsStatics> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::windows::core::RuntimeName for DateTimeSettings {
    const NAME: &'static str = "Windows.System.DateTimeSettings";
}
#[doc = "*Required features: `\"System\"`*"]
#[repr(transparent)]
pub struct DispatcherQueue(::windows::core::IUnknown);
impl DispatcherQueue {
    pub fn CreateTimer(&self) -> ::windows::core::Result<DispatcherQueueTimer> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CreateTimer)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn TryEnqueue(&self, callback: &DispatcherQueueHandler) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).TryEnqueue)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(callback), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn TryEnqueueWithPriority(&self, priority: DispatcherQueuePriority, callback: &DispatcherQueueHandler) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).TryEnqueueWithPriority)(::windows::core::Vtable::as_raw(this), priority, ::core::mem::transmute_copy(callback), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ShutdownStarting(&self, handler: &super::Foundation::TypedEventHandler<DispatcherQueue, DispatcherQueueShutdownStartingEventArgs>) -> ::windows::core::Result<super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ShutdownStarting)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(handler), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveShutdownStarting(&self, token: super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).RemoveShutdownStarting)(::windows::core::Vtable::as_raw(this), token).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ShutdownCompleted(&self, handler: &super::Foundation::TypedEventHandler<DispatcherQueue, ::windows::core::IInspectable>) -> ::windows::core::Result<super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ShutdownCompleted)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(handler), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveShutdownCompleted(&self, token: super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).RemoveShutdownCompleted)(::windows::core::Vtable::as_raw(this), token).ok() }
    }
    pub fn HasThreadAccess(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IDispatcherQueue2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).HasThreadAccess)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn GetForCurrentThread() -> ::windows::core::Result<DispatcherQueue> {
        Self::IDispatcherQueueStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetForCurrentThread)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IDispatcherQueueStatics<R, F: FnOnce(&IDispatcherQueueStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<DispatcherQueue, IDispatcherQueueStatics> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::clone::Clone for DispatcherQueue {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for DispatcherQueue {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for DispatcherQueue {}
impl ::core::fmt::Debug for DispatcherQueue {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DispatcherQueue").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for DispatcherQueue {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.System.DispatcherQueue;{603e88e4-a338-4ffe-a457-a5cfb9ceb899})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for DispatcherQueue {
    type Vtable = IDispatcherQueue_Vtbl;
}
unsafe impl ::windows::core::Interface for DispatcherQueue {
    const IID: ::windows::core::GUID = <IDispatcherQueue as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for DispatcherQueue {
    const NAME: &'static str = "Windows.System.DispatcherQueue";
}
::windows::core::interface_hierarchy!(DispatcherQueue, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for DispatcherQueue {}
unsafe impl ::core::marker::Sync for DispatcherQueue {}
#[doc = "*Required features: `\"System\"`*"]
#[repr(transparent)]
pub struct DispatcherQueueController(::windows::core::IUnknown);
impl DispatcherQueueController {
    pub fn DispatcherQueue(&self) -> ::windows::core::Result<DispatcherQueue> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).DispatcherQueue)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ShutdownQueueAsync(&self) -> ::windows::core::Result<super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ShutdownQueueAsync)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn CreateOnDedicatedThread() -> ::windows::core::Result<DispatcherQueueController> {
        Self::IDispatcherQueueControllerStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CreateOnDedicatedThread)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IDispatcherQueueControllerStatics<R, F: FnOnce(&IDispatcherQueueControllerStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<DispatcherQueueController, IDispatcherQueueControllerStatics> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::clone::Clone for DispatcherQueueController {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for DispatcherQueueController {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for DispatcherQueueController {}
impl ::core::fmt::Debug for DispatcherQueueController {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DispatcherQueueController").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for DispatcherQueueController {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.System.DispatcherQueueController;{22f34e66-50db-4e36-a98d-61c01b384d20})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for DispatcherQueueController {
    type Vtable = IDispatcherQueueController_Vtbl;
}
unsafe impl ::windows::core::Interface for DispatcherQueueController {
    const IID: ::windows::core::GUID = <IDispatcherQueueController as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for DispatcherQueueController {
    const NAME: &'static str = "Windows.System.DispatcherQueueController";
}
::windows::core::interface_hierarchy!(DispatcherQueueController, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for DispatcherQueueController {}
unsafe impl ::core::marker::Sync for DispatcherQueueController {}
#[doc = "*Required features: `\"System\"`*"]
#[repr(transparent)]
pub struct DispatcherQueueShutdownStartingEventArgs(::windows::core::IUnknown);
impl DispatcherQueueShutdownStartingEventArgs {
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn GetDeferral(&self) -> ::windows::core::Result<super::Foundation::Deferral> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetDeferral)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
}
impl ::core::clone::Clone for DispatcherQueueShutdownStartingEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for DispatcherQueueShutdownStartingEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for DispatcherQueueShutdownStartingEventArgs {}
impl ::core::fmt::Debug for DispatcherQueueShutdownStartingEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DispatcherQueueShutdownStartingEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for DispatcherQueueShutdownStartingEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.System.DispatcherQueueShutdownStartingEventArgs;{c4724c4c-ff97-40c0-a226-cc0aaa545e89})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for DispatcherQueueShutdownStartingEventArgs {
    type Vtable = IDispatcherQueueShutdownStartingEventArgs_Vtbl;
}
unsafe impl ::windows::core::Interface for DispatcherQueueShutdownStartingEventArgs {
    const IID: ::windows::core::GUID = <IDispatcherQueueShutdownStartingEventArgs as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for DispatcherQueueShutdownStartingEventArgs {
    const NAME: &'static str = "Windows.System.DispatcherQueueShutdownStartingEventArgs";
}
::windows::core::interface_hierarchy!(DispatcherQueueShutdownStartingEventArgs, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for DispatcherQueueShutdownStartingEventArgs {}
unsafe impl ::core::marker::Sync for DispatcherQueueShutdownStartingEventArgs {}
#[doc = "*Required features: `\"System\"`*"]
#[repr(transparent)]
pub struct DispatcherQueueTimer(::windows::core::IUnknown);
impl DispatcherQueueTimer {
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Interval(&self) -> ::windows::core::Result<super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Interval)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetInterval(&self, value: super::Foundation::TimeSpan) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetInterval)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
    pub fn IsRunning(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).IsRunning)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn IsRepeating(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).IsRepeating)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetIsRepeating(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetIsRepeating)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
    pub fn Start(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).Start)(::windows::core::Vtable::as_raw(this)).ok() }
    }
    pub fn Stop(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).Stop)(::windows::core::Vtable::as_raw(this)).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Tick(&self, handler: &super::Foundation::TypedEventHandler<DispatcherQueueTimer, ::windows::core::IInspectable>) -> ::windows::core::Result<super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Tick)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(handler), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveTick(&self, token: super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).RemoveTick)(::windows::core::Vtable::as_raw(this), token).ok() }
    }
}
impl ::core::clone::Clone for DispatcherQueueTimer {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for DispatcherQueueTimer {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for DispatcherQueueTimer {}
impl ::core::fmt::Debug for DispatcherQueueTimer {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DispatcherQueueTimer").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for DispatcherQueueTimer {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.System.DispatcherQueueTimer;{5feabb1d-a31c-4727-b1ac-37454649d56a})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for DispatcherQueueTimer {
    type Vtable = IDispatcherQueueTimer_Vtbl;
}
unsafe impl ::windows::core::Interface for DispatcherQueueTimer {
    const IID: ::windows::core::GUID = <IDispatcherQueueTimer as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for DispatcherQueueTimer {
    const NAME: &'static str = "Windows.System.DispatcherQueueTimer";
}
::windows::core::interface_hierarchy!(DispatcherQueueTimer, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for DispatcherQueueTimer {}
unsafe impl ::core::marker::Sync for DispatcherQueueTimer {}
#[doc = "*Required features: `\"System\"`*"]
#[repr(transparent)]
pub struct FolderLauncherOptions(::windows::core::IUnknown);
impl FolderLauncherOptions {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IGenericFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<FolderLauncherOptions, ::windows::core::IGenericFactory> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`, `\"Storage\"`*"]
    #[cfg(all(feature = "Foundation_Collections", feature = "Storage"))]
    pub fn ItemsToSelect(&self) -> ::windows::core::Result<super::Foundation::Collections::IVector<super::Storage::IStorageItem>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ItemsToSelect)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"UI_ViewManagement\"`*"]
    #[cfg(feature = "UI_ViewManagement")]
    pub fn DesiredRemainingView(&self) -> ::windows::core::Result<super::UI::ViewManagement::ViewSizePreference> {
        let this = &::windows::core::Interface::cast::<ILauncherViewOptions>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).DesiredRemainingView)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"UI_ViewManagement\"`*"]
    #[cfg(feature = "UI_ViewManagement")]
    pub fn SetDesiredRemainingView(&self, value: super::UI::ViewManagement::ViewSizePreference) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ILauncherViewOptions>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).SetDesiredRemainingView)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
}
impl ::core::clone::Clone for FolderLauncherOptions {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for FolderLauncherOptions {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for FolderLauncherOptions {}
impl ::core::fmt::Debug for FolderLauncherOptions {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FolderLauncherOptions").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for FolderLauncherOptions {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.System.FolderLauncherOptions;{bb91c27d-6b87-432a-bd04-776c6f5fb2ab})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for FolderLauncherOptions {
    type Vtable = IFolderLauncherOptions_Vtbl;
}
unsafe impl ::windows::core::Interface for FolderLauncherOptions {
    const IID: ::windows::core::GUID = <IFolderLauncherOptions as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for FolderLauncherOptions {
    const NAME: &'static str = "Windows.System.FolderLauncherOptions";
}
::windows::core::interface_hierarchy!(FolderLauncherOptions, ::windows::core::IUnknown, ::windows::core::IInspectable);
impl ::core::convert::TryFrom<FolderLauncherOptions> for ILauncherViewOptions {
    type Error = ::windows::core::Error;
    fn try_from(value: FolderLauncherOptions) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&FolderLauncherOptions> for ILauncherViewOptions {
    type Error = ::windows::core::Error;
    fn try_from(value: &FolderLauncherOptions) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl ::core::convert::TryFrom<&FolderLauncherOptions> for ::windows::core::InParam<ILauncherViewOptions> {
    type Error = ::windows::core::Error;
    fn try_from(value: &FolderLauncherOptions) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
unsafe impl ::core::marker::Send for FolderLauncherOptions {}
unsafe impl ::core::marker::Sync for FolderLauncherOptions {}
#[doc = "*Required features: `\"System\"`*"]
pub struct KnownUserProperties;
impl KnownUserProperties {
    pub fn DisplayName() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IKnownUserPropertiesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).DisplayName)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    pub fn FirstName() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IKnownUserPropertiesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).FirstName)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    pub fn LastName() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IKnownUserPropertiesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).LastName)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    pub fn ProviderName() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IKnownUserPropertiesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ProviderName)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    pub fn AccountName() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IKnownUserPropertiesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).AccountName)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    pub fn GuestHost() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IKnownUserPropertiesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GuestHost)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    pub fn PrincipalName() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IKnownUserPropertiesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).PrincipalName)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    pub fn DomainName() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IKnownUserPropertiesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).DomainName)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    pub fn SessionInitiationProtocolUri() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IKnownUserPropertiesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).SessionInitiationProtocolUri)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    pub fn AgeEnforcementRegion() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IKnownUserPropertiesStatics2(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).AgeEnforcementRegion)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IKnownUserPropertiesStatics<R, F: FnOnce(&IKnownUserPropertiesStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<KnownUserProperties, IKnownUserPropertiesStatics> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn IKnownUserPropertiesStatics2<R, F: FnOnce(&IKnownUserPropertiesStatics2) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<KnownUserProperties, IKnownUserPropertiesStatics2> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::windows::core::RuntimeName for KnownUserProperties {
    const NAME: &'static str = "Windows.System.KnownUserProperties";
}
#[doc = "*Required features: `\"System\"`*"]
#[repr(transparent)]
pub struct LaunchUriResult(::windows::core::IUnknown);
impl LaunchUriResult {
    pub fn Status(&self) -> ::windows::core::Result<LaunchUriStatus> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Status)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Result(&self) -> ::windows::core::Result<super::Foundation::Collections::ValueSet> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Result)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
}
impl ::core::clone::Clone for LaunchUriResult {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for LaunchUriResult {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for LaunchUriResult {}
impl ::core::fmt::Debug for LaunchUriResult {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("LaunchUriResult").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for LaunchUriResult {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.System.LaunchUriResult;{ec27a8df-f6d5-45ca-913a-70a40c5c8221})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for LaunchUriResult {
    type Vtable = ILaunchUriResult_Vtbl;
}
unsafe impl ::windows::core::Interface for LaunchUriResult {
    const IID: ::windows::core::GUID = <ILaunchUriResult as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for LaunchUriResult {
    const NAME: &'static str = "Windows.System.LaunchUriResult";
}
::windows::core::interface_hierarchy!(LaunchUriResult, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for LaunchUriResult {}
unsafe impl ::core::marker::Sync for LaunchUriResult {}
#[doc = "*Required features: `\"System\"`*"]
pub struct Launcher;
impl Launcher {
    #[doc = "*Required features: `\"Foundation\"`, `\"Storage\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Storage"))]
    pub fn LaunchFileAsync<P0, E0>(file: P0) -> ::windows::core::Result<super::Foundation::IAsyncOperation<bool>>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<super::Storage::IStorageFile>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        Self::ILauncherStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).LaunchFileAsync)(::windows::core::Vtable::as_raw(this), file.try_into().map_err(|e| e.into())?.abi(), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"Storage\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Storage"))]
    pub fn LaunchFileWithOptionsAsync<P0, E0>(file: P0, options: &LauncherOptions) -> ::windows::core::Result<super::Foundation::IAsyncOperation<bool>>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<super::Storage::IStorageFile>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        Self::ILauncherStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).LaunchFileWithOptionsAsync)(::windows::core::Vtable::as_raw(this), file.try_into().map_err(|e| e.into())?.abi(), ::core::mem::transmute_copy(options), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn LaunchUriAsync(uri: &super::Foundation::Uri) -> ::windows::core::Result<super::Foundation::IAsyncOperation<bool>> {
        Self::ILauncherStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).LaunchUriAsync)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(uri), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn LaunchUriWithOptionsAsync(uri: &super::Foundation::Uri, options: &LauncherOptions) -> ::windows::core::Result<super::Foundation::IAsyncOperation<bool>> {
        Self::ILauncherStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).LaunchUriWithOptionsAsync)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(uri), ::core::mem::transmute_copy(options), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn LaunchUriForResultsAsync(uri: &super::Foundation::Uri, options: &LauncherOptions) -> ::windows::core::Result<super::Foundation::IAsyncOperation<LaunchUriResult>> {
        Self::ILauncherStatics2(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).LaunchUriForResultsAsync)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(uri), ::core::mem::transmute_copy(options), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn LaunchUriForResultsWithDataAsync(uri: &super::Foundation::Uri, options: &LauncherOptions, inputdata: &super::Foundation::Collections::ValueSet) -> ::windows::core::Result<super::Foundation::IAsyncOperation<LaunchUriResult>> {
        Self::ILauncherStatics2(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).LaunchUriForResultsWithDataAsync)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(uri), ::core::mem::transmute_copy(options), ::core::mem::transmute_copy(inputdata), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn LaunchUriWithDataAsync(uri: &super::Foundation::Uri, options: &LauncherOptions, inputdata: &super::Foundation::Collections::ValueSet) -> ::windows::core::Result<super::Foundation::IAsyncOperation<bool>> {
        Self::ILauncherStatics2(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).LaunchUriWithDataAsync)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(uri), ::core::mem::transmute_copy(options), ::core::mem::transmute_copy(inputdata), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn QueryUriSupportAsync(uri: &super::Foundation::Uri, launchquerysupporttype: LaunchQuerySupportType) -> ::windows::core::Result<super::Foundation::IAsyncOperation<LaunchQuerySupportStatus>> {
        Self::ILauncherStatics2(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).QueryUriSupportAsync)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(uri), launchquerysupporttype, result__.as_mut_ptr()).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn QueryUriSupportWithPackageFamilyNameAsync(uri: &super::Foundation::Uri, launchquerysupporttype: LaunchQuerySupportType, packagefamilyname: &::windows::core::HSTRING) -> ::windows::core::Result<super::Foundation::IAsyncOperation<LaunchQuerySupportStatus>> {
        Self::ILauncherStatics2(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).QueryUriSupportWithPackageFamilyNameAsync)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(uri), launchquerysupporttype, ::core::mem::transmute_copy(packagefamilyname), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"Storage\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Storage"))]
    pub fn QueryFileSupportAsync(file: &super::Storage::StorageFile) -> ::windows::core::Result<super::Foundation::IAsyncOperation<LaunchQuerySupportStatus>> {
        Self::ILauncherStatics2(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).QueryFileSupportAsync)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(file), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"Storage\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Storage"))]
    pub fn QueryFileSupportWithPackageFamilyNameAsync(file: &super::Storage::StorageFile, packagefamilyname: &::windows::core::HSTRING) -> ::windows::core::Result<super::Foundation::IAsyncOperation<LaunchQuerySupportStatus>> {
        Self::ILauncherStatics2(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).QueryFileSupportWithPackageFamilyNameAsync)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(file), ::core::mem::transmute_copy(packagefamilyname), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"ApplicationModel\"`, `\"Foundation_Collections\"`*"]
    #[cfg(all(feature = "ApplicationModel", feature = "Foundation_Collections"))]
    pub fn FindUriSchemeHandlersAsync(scheme: &::windows::core::HSTRING) -> ::windows::core::Result<super::Foundation::IAsyncOperation<super::Foundation::Collections::IVectorView<super::ApplicationModel::AppInfo>>> {
        Self::ILauncherStatics2(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).FindUriSchemeHandlersAsync)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(scheme), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"ApplicationModel\"`, `\"Foundation_Collections\"`*"]
    #[cfg(all(feature = "ApplicationModel", feature = "Foundation_Collections"))]
    pub fn FindUriSchemeHandlersWithLaunchUriTypeAsync(scheme: &::windows::core::HSTRING, launchquerysupporttype: LaunchQuerySupportType) -> ::windows::core::Result<super::Foundation::IAsyncOperation<super::Foundation::Collections::IVectorView<super::ApplicationModel::AppInfo>>> {
        Self::ILauncherStatics2(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).FindUriSchemeHandlersWithLaunchUriTypeAsync)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(scheme), launchquerysupporttype, result__.as_mut_ptr()).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"ApplicationModel\"`, `\"Foundation_Collections\"`*"]
    #[cfg(all(feature = "ApplicationModel", feature = "Foundation_Collections"))]
    pub fn FindFileHandlersAsync(extension: &::windows::core::HSTRING) -> ::windows::core::Result<super::Foundation::IAsyncOperation<super::Foundation::Collections::IVectorView<super::ApplicationModel::AppInfo>>> {
        Self::ILauncherStatics2(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).FindFileHandlersAsync)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(extension), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"Storage\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Storage"))]
    pub fn LaunchFolderAsync<P0, E0>(folder: P0) -> ::windows::core::Result<super::Foundation::IAsyncOperation<bool>>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<super::Storage::IStorageFolder>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        Self::ILauncherStatics3(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).LaunchFolderAsync)(::windows::core::Vtable::as_raw(this), folder.try_into().map_err(|e| e.into())?.abi(), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"Storage\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Storage"))]
    pub fn LaunchFolderWithOptionsAsync<P0, E0>(folder: P0, options: &FolderLauncherOptions) -> ::windows::core::Result<super::Foundation::IAsyncOperation<bool>>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<super::Storage::IStorageFolder>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        Self::ILauncherStatics3(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).LaunchFolderWithOptionsAsync)(::windows::core::Vtable::as_raw(this), folder.try_into().map_err(|e| e.into())?.abi(), ::core::mem::transmute_copy(options), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn QueryAppUriSupportAsync(uri: &super::Foundation::Uri) -> ::windows::core::Result<super::Foundation::IAsyncOperation<LaunchQuerySupportStatus>> {
        Self::ILauncherStatics4(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).QueryAppUriSupportAsync)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(uri), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn QueryAppUriSupportWithPackageFamilyNameAsync(uri: &super::Foundation::Uri, packagefamilyname: &::windows::core::HSTRING) -> ::windows::core::Result<super::Foundation::IAsyncOperation<LaunchQuerySupportStatus>> {
        Self::ILauncherStatics4(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).QueryAppUriSupportWithPackageFamilyNameAsync)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(uri), ::core::mem::transmute_copy(packagefamilyname), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"ApplicationModel\"`, `\"Foundation_Collections\"`*"]
    #[cfg(all(feature = "ApplicationModel", feature = "Foundation_Collections"))]
    pub fn FindAppUriHandlersAsync(uri: &super::Foundation::Uri) -> ::windows::core::Result<super::Foundation::IAsyncOperation<super::Foundation::Collections::IVectorView<super::ApplicationModel::AppInfo>>> {
        Self::ILauncherStatics4(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).FindAppUriHandlersAsync)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(uri), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn LaunchUriForUserAsync(user: &User, uri: &super::Foundation::Uri) -> ::windows::core::Result<super::Foundation::IAsyncOperation<LaunchUriStatus>> {
        Self::ILauncherStatics4(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).LaunchUriForUserAsync)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(user), ::core::mem::transmute_copy(uri), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn LaunchUriWithOptionsForUserAsync(user: &User, uri: &super::Foundation::Uri, options: &LauncherOptions) -> ::windows::core::Result<super::Foundation::IAsyncOperation<LaunchUriStatus>> {
        Self::ILauncherStatics4(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).LaunchUriWithOptionsForUserAsync)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(user), ::core::mem::transmute_copy(uri), ::core::mem::transmute_copy(options), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn LaunchUriWithDataForUserAsync(user: &User, uri: &super::Foundation::Uri, options: &LauncherOptions, inputdata: &super::Foundation::Collections::ValueSet) -> ::windows::core::Result<super::Foundation::IAsyncOperation<LaunchUriStatus>> {
        Self::ILauncherStatics4(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).LaunchUriWithDataForUserAsync)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(user), ::core::mem::transmute_copy(uri), ::core::mem::transmute_copy(options), ::core::mem::transmute_copy(inputdata), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn LaunchUriForResultsForUserAsync(user: &User, uri: &super::Foundation::Uri, options: &LauncherOptions) -> ::windows::core::Result<super::Foundation::IAsyncOperation<LaunchUriResult>> {
        Self::ILauncherStatics4(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).LaunchUriForResultsForUserAsync)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(user), ::core::mem::transmute_copy(uri), ::core::mem::transmute_copy(options), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn LaunchUriForResultsWithDataForUserAsync(user: &User, uri: &super::Foundation::Uri, options: &LauncherOptions, inputdata: &super::Foundation::Collections::ValueSet) -> ::windows::core::Result<super::Foundation::IAsyncOperation<LaunchUriResult>> {
        Self::ILauncherStatics4(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).LaunchUriForResultsWithDataForUserAsync)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(user), ::core::mem::transmute_copy(uri), ::core::mem::transmute_copy(options), ::core::mem::transmute_copy(inputdata), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn LaunchFolderPathAsync(path: &::windows::core::HSTRING) -> ::windows::core::Result<super::Foundation::IAsyncOperation<bool>> {
        Self::ILauncherStatics5(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).LaunchFolderPathAsync)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(path), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn LaunchFolderPathWithOptionsAsync(path: &::windows::core::HSTRING, options: &FolderLauncherOptions) -> ::windows::core::Result<super::Foundation::IAsyncOperation<bool>> {
        Self::ILauncherStatics5(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).LaunchFolderPathWithOptionsAsync)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(path), ::core::mem::transmute_copy(options), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn LaunchFolderPathForUserAsync(user: &User, path: &::windows::core::HSTRING) -> ::windows::core::Result<super::Foundation::IAsyncOperation<bool>> {
        Self::ILauncherStatics5(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).LaunchFolderPathForUserAsync)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(user), ::core::mem::transmute_copy(path), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn LaunchFolderPathWithOptionsForUserAsync(user: &User, path: &::windows::core::HSTRING, options: &FolderLauncherOptions) -> ::windows::core::Result<super::Foundation::IAsyncOperation<bool>> {
        Self::ILauncherStatics5(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).LaunchFolderPathWithOptionsForUserAsync)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(user), ::core::mem::transmute_copy(path), ::core::mem::transmute_copy(options), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn ILauncherStatics<R, F: FnOnce(&ILauncherStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<Launcher, ILauncherStatics> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn ILauncherStatics2<R, F: FnOnce(&ILauncherStatics2) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<Launcher, ILauncherStatics2> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn ILauncherStatics3<R, F: FnOnce(&ILauncherStatics3) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<Launcher, ILauncherStatics3> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn ILauncherStatics4<R, F: FnOnce(&ILauncherStatics4) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<Launcher, ILauncherStatics4> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn ILauncherStatics5<R, F: FnOnce(&ILauncherStatics5) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<Launcher, ILauncherStatics5> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::windows::core::RuntimeName for Launcher {
    const NAME: &'static str = "Windows.System.Launcher";
}
#[doc = "*Required features: `\"System\"`*"]
#[repr(transparent)]
pub struct LauncherOptions(::windows::core::IUnknown);
impl LauncherOptions {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IGenericFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<LauncherOptions, ::windows::core::IGenericFactory> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn TreatAsUntrusted(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).TreatAsUntrusted)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetTreatAsUntrusted(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetTreatAsUntrusted)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
    pub fn DisplayApplicationPicker(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).DisplayApplicationPicker)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetDisplayApplicationPicker(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetDisplayApplicationPicker)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
    pub fn UI(&self) -> ::windows::core::Result<LauncherUIOptions> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).UI)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn PreferredApplicationPackageFamilyName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).PreferredApplicationPackageFamilyName)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetPreferredApplicationPackageFamilyName(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetPreferredApplicationPackageFamilyName)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn PreferredApplicationDisplayName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).PreferredApplicationDisplayName)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetPreferredApplicationDisplayName(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetPreferredApplicationDisplayName)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn FallbackUri(&self) -> ::windows::core::Result<super::Foundation::Uri> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).FallbackUri)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetFallbackUri(&self, value: &super::Foundation::Uri) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetFallbackUri)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn ContentType(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ContentType)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetContentType(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetContentType)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn TargetApplicationPackageFamilyName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<ILauncherOptions2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).TargetApplicationPackageFamilyName)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetTargetApplicationPackageFamilyName(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ILauncherOptions2>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).SetTargetApplicationPackageFamilyName)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    #[doc = "*Required features: `\"Storage_Search\"`*"]
    #[cfg(feature = "Storage_Search")]
    pub fn NeighboringFilesQuery(&self) -> ::windows::core::Result<super::Storage::Search::StorageFileQueryResult> {
        let this = &::windows::core::Interface::cast::<ILauncherOptions2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).NeighboringFilesQuery)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_Search\"`*"]
    #[cfg(feature = "Storage_Search")]
    pub fn SetNeighboringFilesQuery(&self, value: &super::Storage::Search::StorageFileQueryResult) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ILauncherOptions2>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).SetNeighboringFilesQuery)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn IgnoreAppUriHandlers(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<ILauncherOptions3>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).IgnoreAppUriHandlers)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetIgnoreAppUriHandlers(&self, value: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ILauncherOptions3>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).SetIgnoreAppUriHandlers)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
    pub fn LimitPickerToCurrentAppAndAppUriHandlers(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<ILauncherOptions4>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).LimitPickerToCurrentAppAndAppUriHandlers)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetLimitPickerToCurrentAppAndAppUriHandlers(&self, value: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ILauncherOptions4>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).SetLimitPickerToCurrentAppAndAppUriHandlers)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
    #[doc = "*Required features: `\"UI_ViewManagement\"`*"]
    #[cfg(feature = "UI_ViewManagement")]
    pub fn DesiredRemainingView(&self) -> ::windows::core::Result<super::UI::ViewManagement::ViewSizePreference> {
        let this = &::windows::core::Interface::cast::<ILauncherViewOptions>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).DesiredRemainingView)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"UI_ViewManagement\"`*"]
    #[cfg(feature = "UI_ViewManagement")]
    pub fn SetDesiredRemainingView(&self, value: super::UI::ViewManagement::ViewSizePreference) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ILauncherViewOptions>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).SetDesiredRemainingView)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
}
impl ::core::clone::Clone for LauncherOptions {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for LauncherOptions {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for LauncherOptions {}
impl ::core::fmt::Debug for LauncherOptions {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("LauncherOptions").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for LauncherOptions {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.System.LauncherOptions;{bafa21d8-b071-4cd8-853e-341203e557d3})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for LauncherOptions {
    type Vtable = ILauncherOptions_Vtbl;
}
unsafe impl ::windows::core::Interface for LauncherOptions {
    const IID: ::windows::core::GUID = <ILauncherOptions as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for LauncherOptions {
    const NAME: &'static str = "Windows.System.LauncherOptions";
}
::windows::core::interface_hierarchy!(LauncherOptions, ::windows::core::IUnknown, ::windows::core::IInspectable);
impl ::core::convert::TryFrom<LauncherOptions> for ILauncherViewOptions {
    type Error = ::windows::core::Error;
    fn try_from(value: LauncherOptions) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&LauncherOptions> for ILauncherViewOptions {
    type Error = ::windows::core::Error;
    fn try_from(value: &LauncherOptions) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl ::core::convert::TryFrom<&LauncherOptions> for ::windows::core::InParam<ILauncherViewOptions> {
    type Error = ::windows::core::Error;
    fn try_from(value: &LauncherOptions) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
unsafe impl ::core::marker::Send for LauncherOptions {}
unsafe impl ::core::marker::Sync for LauncherOptions {}
#[doc = "*Required features: `\"System\"`*"]
#[repr(transparent)]
pub struct LauncherUIOptions(::windows::core::IUnknown);
impl LauncherUIOptions {
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn InvocationPoint(&self) -> ::windows::core::Result<super::Foundation::IReference<super::Foundation::Point>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).InvocationPoint)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetInvocationPoint<P0, E0>(&self, value: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<super::Foundation::IReference<super::Foundation::Point>>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetInvocationPoint)(::windows::core::Vtable::as_raw(this), value.try_into().map_err(|e| e.into())?.abi()).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SelectionRect(&self) -> ::windows::core::Result<super::Foundation::IReference<super::Foundation::Rect>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).SelectionRect)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetSelectionRect<P0, E0>(&self, value: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<super::Foundation::IReference<super::Foundation::Rect>>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetSelectionRect)(::windows::core::Vtable::as_raw(this), value.try_into().map_err(|e| e.into())?.abi()).ok() }
    }
    #[doc = "*Required features: `\"UI_Popups\"`*"]
    #[cfg(feature = "UI_Popups")]
    pub fn PreferredPlacement(&self) -> ::windows::core::Result<super::UI::Popups::Placement> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).PreferredPlacement)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Popups\"`*"]
    #[cfg(feature = "UI_Popups")]
    pub fn SetPreferredPlacement(&self, value: super::UI::Popups::Placement) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetPreferredPlacement)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
}
impl ::core::clone::Clone for LauncherUIOptions {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for LauncherUIOptions {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for LauncherUIOptions {}
impl ::core::fmt::Debug for LauncherUIOptions {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("LauncherUIOptions").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for LauncherUIOptions {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.System.LauncherUIOptions;{1b25da6e-8aa6-41e9-8251-4165f5985f49})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for LauncherUIOptions {
    type Vtable = ILauncherUIOptions_Vtbl;
}
unsafe impl ::windows::core::Interface for LauncherUIOptions {
    const IID: ::windows::core::GUID = <ILauncherUIOptions as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for LauncherUIOptions {
    const NAME: &'static str = "Windows.System.LauncherUIOptions";
}
::windows::core::interface_hierarchy!(LauncherUIOptions, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for LauncherUIOptions {}
unsafe impl ::core::marker::Sync for LauncherUIOptions {}
#[doc = "*Required features: `\"System\"`*"]
pub struct MemoryManager;
impl MemoryManager {
    pub fn AppMemoryUsage() -> ::windows::core::Result<u64> {
        Self::IMemoryManagerStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).AppMemoryUsage)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    pub fn AppMemoryUsageLimit() -> ::windows::core::Result<u64> {
        Self::IMemoryManagerStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).AppMemoryUsageLimit)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    pub fn AppMemoryUsageLevel() -> ::windows::core::Result<AppMemoryUsageLevel> {
        Self::IMemoryManagerStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).AppMemoryUsageLevel)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn AppMemoryUsageIncreased(handler: &super::Foundation::EventHandler<::windows::core::IInspectable>) -> ::windows::core::Result<super::Foundation::EventRegistrationToken> {
        Self::IMemoryManagerStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).AppMemoryUsageIncreased)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(handler), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveAppMemoryUsageIncreased(token: super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        Self::IMemoryManagerStatics(|this| unsafe { (::windows::core::Vtable::vtable(this).RemoveAppMemoryUsageIncreased)(::windows::core::Vtable::as_raw(this), token).ok() })
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn AppMemoryUsageDecreased(handler: &super::Foundation::EventHandler<::windows::core::IInspectable>) -> ::windows::core::Result<super::Foundation::EventRegistrationToken> {
        Self::IMemoryManagerStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).AppMemoryUsageDecreased)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(handler), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveAppMemoryUsageDecreased(token: super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        Self::IMemoryManagerStatics(|this| unsafe { (::windows::core::Vtable::vtable(this).RemoveAppMemoryUsageDecreased)(::windows::core::Vtable::as_raw(this), token).ok() })
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn AppMemoryUsageLimitChanging(handler: &super::Foundation::EventHandler<AppMemoryUsageLimitChangingEventArgs>) -> ::windows::core::Result<super::Foundation::EventRegistrationToken> {
        Self::IMemoryManagerStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).AppMemoryUsageLimitChanging)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(handler), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveAppMemoryUsageLimitChanging(token: super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        Self::IMemoryManagerStatics(|this| unsafe { (::windows::core::Vtable::vtable(this).RemoveAppMemoryUsageLimitChanging)(::windows::core::Vtable::as_raw(this), token).ok() })
    }
    pub fn GetAppMemoryReport() -> ::windows::core::Result<AppMemoryReport> {
        Self::IMemoryManagerStatics2(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetAppMemoryReport)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    pub fn GetProcessMemoryReport() -> ::windows::core::Result<ProcessMemoryReport> {
        Self::IMemoryManagerStatics2(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetProcessMemoryReport)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    pub fn TrySetAppMemoryUsageLimit(value: u64) -> ::windows::core::Result<bool> {
        Self::IMemoryManagerStatics3(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).TrySetAppMemoryUsageLimit)(::windows::core::Vtable::as_raw(this), value, result__.as_mut_ptr()).from_abi(result__)
        })
    }
    pub fn ExpectedAppMemoryUsageLimit() -> ::windows::core::Result<u64> {
        Self::IMemoryManagerStatics4(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ExpectedAppMemoryUsageLimit)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IMemoryManagerStatics<R, F: FnOnce(&IMemoryManagerStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<MemoryManager, IMemoryManagerStatics> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn IMemoryManagerStatics2<R, F: FnOnce(&IMemoryManagerStatics2) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<MemoryManager, IMemoryManagerStatics2> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn IMemoryManagerStatics3<R, F: FnOnce(&IMemoryManagerStatics3) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<MemoryManager, IMemoryManagerStatics3> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn IMemoryManagerStatics4<R, F: FnOnce(&IMemoryManagerStatics4) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<MemoryManager, IMemoryManagerStatics4> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::windows::core::RuntimeName for MemoryManager {
    const NAME: &'static str = "Windows.System.MemoryManager";
}
#[doc = "*Required features: `\"System\"`*"]
pub struct ProcessLauncher;
impl ProcessLauncher {
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RunToCompletionAsync(filename: &::windows::core::HSTRING, args: &::windows::core::HSTRING) -> ::windows::core::Result<super::Foundation::IAsyncOperation<ProcessLauncherResult>> {
        Self::IProcessLauncherStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).RunToCompletionAsync)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(filename), ::core::mem::transmute_copy(args), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RunToCompletionAsyncWithOptions(filename: &::windows::core::HSTRING, args: &::windows::core::HSTRING, options: &ProcessLauncherOptions) -> ::windows::core::Result<super::Foundation::IAsyncOperation<ProcessLauncherResult>> {
        Self::IProcessLauncherStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).RunToCompletionAsyncWithOptions)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(filename), ::core::mem::transmute_copy(args), ::core::mem::transmute_copy(options), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IProcessLauncherStatics<R, F: FnOnce(&IProcessLauncherStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<ProcessLauncher, IProcessLauncherStatics> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::windows::core::RuntimeName for ProcessLauncher {
    const NAME: &'static str = "Windows.System.ProcessLauncher";
}
#[doc = "*Required features: `\"System\"`*"]
#[repr(transparent)]
pub struct ProcessLauncherOptions(::windows::core::IUnknown);
impl ProcessLauncherOptions {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IGenericFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<ProcessLauncherOptions, ::windows::core::IGenericFactory> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc = "*Required features: `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn StandardInput(&self) -> ::windows::core::Result<super::Storage::Streams::IInputStream> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).StandardInput)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn SetStandardInput<P0, E0>(&self, value: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<super::Storage::Streams::IInputStream>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetStandardInput)(::windows::core::Vtable::as_raw(this), value.try_into().map_err(|e| e.into())?.abi()).ok() }
    }
    #[doc = "*Required features: `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn StandardOutput(&self) -> ::windows::core::Result<super::Storage::Streams::IOutputStream> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).StandardOutput)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn SetStandardOutput<P0, E0>(&self, value: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<super::Storage::Streams::IOutputStream>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetStandardOutput)(::windows::core::Vtable::as_raw(this), value.try_into().map_err(|e| e.into())?.abi()).ok() }
    }
    #[doc = "*Required features: `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn StandardError(&self) -> ::windows::core::Result<super::Storage::Streams::IOutputStream> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).StandardError)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn SetStandardError<P0, E0>(&self, value: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<super::Storage::Streams::IOutputStream>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetStandardError)(::windows::core::Vtable::as_raw(this), value.try_into().map_err(|e| e.into())?.abi()).ok() }
    }
    pub fn WorkingDirectory(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).WorkingDirectory)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetWorkingDirectory(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetWorkingDirectory)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
}
impl ::core::clone::Clone for ProcessLauncherOptions {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ProcessLauncherOptions {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ProcessLauncherOptions {}
impl ::core::fmt::Debug for ProcessLauncherOptions {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ProcessLauncherOptions").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for ProcessLauncherOptions {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.System.ProcessLauncherOptions;{3080b9cf-f444-4a83-beaf-a549a0f3229c})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for ProcessLauncherOptions {
    type Vtable = IProcessLauncherOptions_Vtbl;
}
unsafe impl ::windows::core::Interface for ProcessLauncherOptions {
    const IID: ::windows::core::GUID = <IProcessLauncherOptions as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for ProcessLauncherOptions {
    const NAME: &'static str = "Windows.System.ProcessLauncherOptions";
}
::windows::core::interface_hierarchy!(ProcessLauncherOptions, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for ProcessLauncherOptions {}
unsafe impl ::core::marker::Sync for ProcessLauncherOptions {}
#[doc = "*Required features: `\"System\"`*"]
#[repr(transparent)]
pub struct ProcessLauncherResult(::windows::core::IUnknown);
impl ProcessLauncherResult {
    pub fn ExitCode(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ExitCode)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
}
impl ::core::clone::Clone for ProcessLauncherResult {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ProcessLauncherResult {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ProcessLauncherResult {}
impl ::core::fmt::Debug for ProcessLauncherResult {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ProcessLauncherResult").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for ProcessLauncherResult {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.System.ProcessLauncherResult;{544c8934-86d8-4991-8e75-ece8a43b6b6d})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for ProcessLauncherResult {
    type Vtable = IProcessLauncherResult_Vtbl;
}
unsafe impl ::windows::core::Interface for ProcessLauncherResult {
    const IID: ::windows::core::GUID = <IProcessLauncherResult as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for ProcessLauncherResult {
    const NAME: &'static str = "Windows.System.ProcessLauncherResult";
}
::windows::core::interface_hierarchy!(ProcessLauncherResult, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for ProcessLauncherResult {}
unsafe impl ::core::marker::Sync for ProcessLauncherResult {}
#[doc = "*Required features: `\"System\"`*"]
#[repr(transparent)]
pub struct ProcessMemoryReport(::windows::core::IUnknown);
impl ProcessMemoryReport {
    pub fn PrivateWorkingSetUsage(&self) -> ::windows::core::Result<u64> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).PrivateWorkingSetUsage)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn TotalWorkingSetUsage(&self) -> ::windows::core::Result<u64> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).TotalWorkingSetUsage)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
}
impl ::core::clone::Clone for ProcessMemoryReport {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ProcessMemoryReport {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ProcessMemoryReport {}
impl ::core::fmt::Debug for ProcessMemoryReport {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ProcessMemoryReport").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for ProcessMemoryReport {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.System.ProcessMemoryReport;{087305a8-9b70-4782-8741-3a982b6ce5e4})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for ProcessMemoryReport {
    type Vtable = IProcessMemoryReport_Vtbl;
}
unsafe impl ::windows::core::Interface for ProcessMemoryReport {
    const IID: ::windows::core::GUID = <IProcessMemoryReport as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for ProcessMemoryReport {
    const NAME: &'static str = "Windows.System.ProcessMemoryReport";
}
::windows::core::interface_hierarchy!(ProcessMemoryReport, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for ProcessMemoryReport {}
unsafe impl ::core::marker::Sync for ProcessMemoryReport {}
#[doc = "*Required features: `\"System\"`*"]
#[repr(transparent)]
pub struct ProtocolForResultsOperation(::windows::core::IUnknown);
impl ProtocolForResultsOperation {
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn ReportCompleted(&self, data: &super::Foundation::Collections::ValueSet) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).ReportCompleted)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(data)).ok() }
    }
}
impl ::core::clone::Clone for ProtocolForResultsOperation {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ProtocolForResultsOperation {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ProtocolForResultsOperation {}
impl ::core::fmt::Debug for ProtocolForResultsOperation {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ProtocolForResultsOperation").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for ProtocolForResultsOperation {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.System.ProtocolForResultsOperation;{d581293a-6de9-4d28-9378-f86782e182bb})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for ProtocolForResultsOperation {
    type Vtable = IProtocolForResultsOperation_Vtbl;
}
unsafe impl ::windows::core::Interface for ProtocolForResultsOperation {
    const IID: ::windows::core::GUID = <IProtocolForResultsOperation as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for ProtocolForResultsOperation {
    const NAME: &'static str = "Windows.System.ProtocolForResultsOperation";
}
::windows::core::interface_hierarchy!(ProtocolForResultsOperation, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for ProtocolForResultsOperation {}
unsafe impl ::core::marker::Sync for ProtocolForResultsOperation {}
#[doc = "*Required features: `\"System\"`*"]
pub struct RemoteLauncher;
impl RemoteLauncher {
    #[doc = "*Required features: `\"Foundation\"`, `\"System_RemoteSystems\"`*"]
    #[cfg(all(feature = "Foundation", feature = "System_RemoteSystems"))]
    pub fn LaunchUriAsync(remotesystemconnectionrequest: &RemoteSystems::RemoteSystemConnectionRequest, uri: &super::Foundation::Uri) -> ::windows::core::Result<super::Foundation::IAsyncOperation<RemoteLaunchUriStatus>> {
        Self::IRemoteLauncherStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).LaunchUriAsync)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(remotesystemconnectionrequest), ::core::mem::transmute_copy(uri), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"System_RemoteSystems\"`*"]
    #[cfg(all(feature = "Foundation", feature = "System_RemoteSystems"))]
    pub fn LaunchUriWithOptionsAsync(remotesystemconnectionrequest: &RemoteSystems::RemoteSystemConnectionRequest, uri: &super::Foundation::Uri, options: &RemoteLauncherOptions) -> ::windows::core::Result<super::Foundation::IAsyncOperation<RemoteLaunchUriStatus>> {
        Self::IRemoteLauncherStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).LaunchUriWithOptionsAsync)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(remotesystemconnectionrequest), ::core::mem::transmute_copy(uri), ::core::mem::transmute_copy(options), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`, `\"System_RemoteSystems\"`*"]
    #[cfg(all(feature = "Foundation_Collections", feature = "System_RemoteSystems"))]
    pub fn LaunchUriWithDataAsync(remotesystemconnectionrequest: &RemoteSystems::RemoteSystemConnectionRequest, uri: &super::Foundation::Uri, options: &RemoteLauncherOptions, inputdata: &super::Foundation::Collections::ValueSet) -> ::windows::core::Result<super::Foundation::IAsyncOperation<RemoteLaunchUriStatus>> {
        Self::IRemoteLauncherStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).LaunchUriWithDataAsync)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(remotesystemconnectionrequest), ::core::mem::transmute_copy(uri), ::core::mem::transmute_copy(options), ::core::mem::transmute_copy(inputdata), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IRemoteLauncherStatics<R, F: FnOnce(&IRemoteLauncherStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<RemoteLauncher, IRemoteLauncherStatics> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::windows::core::RuntimeName for RemoteLauncher {
    const NAME: &'static str = "Windows.System.RemoteLauncher";
}
#[doc = "*Required features: `\"System\"`*"]
#[repr(transparent)]
pub struct RemoteLauncherOptions(::windows::core::IUnknown);
impl RemoteLauncherOptions {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IGenericFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<RemoteLauncherOptions, ::windows::core::IGenericFactory> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn FallbackUri(&self) -> ::windows::core::Result<super::Foundation::Uri> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).FallbackUri)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetFallbackUri(&self, value: &super::Foundation::Uri) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetFallbackUri)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn PreferredAppIds(&self) -> ::windows::core::Result<super::Foundation::Collections::IVector<::windows::core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).PreferredAppIds)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
}
impl ::core::clone::Clone for RemoteLauncherOptions {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for RemoteLauncherOptions {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for RemoteLauncherOptions {}
impl ::core::fmt::Debug for RemoteLauncherOptions {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RemoteLauncherOptions").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for RemoteLauncherOptions {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.System.RemoteLauncherOptions;{9e3a2788-2891-4cdf-a2d6-9dff7d02e693})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for RemoteLauncherOptions {
    type Vtable = IRemoteLauncherOptions_Vtbl;
}
unsafe impl ::windows::core::Interface for RemoteLauncherOptions {
    const IID: ::windows::core::GUID = <IRemoteLauncherOptions as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for RemoteLauncherOptions {
    const NAME: &'static str = "Windows.System.RemoteLauncherOptions";
}
::windows::core::interface_hierarchy!(RemoteLauncherOptions, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for RemoteLauncherOptions {}
unsafe impl ::core::marker::Sync for RemoteLauncherOptions {}
#[doc = "*Required features: `\"System\"`*"]
pub struct ShutdownManager;
impl ShutdownManager {
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn BeginShutdown(shutdownkind: ShutdownKind, timeout: super::Foundation::TimeSpan) -> ::windows::core::Result<()> {
        Self::IShutdownManagerStatics(|this| unsafe { (::windows::core::Vtable::vtable(this).BeginShutdown)(::windows::core::Vtable::as_raw(this), shutdownkind, timeout).ok() })
    }
    pub fn CancelShutdown() -> ::windows::core::Result<()> {
        Self::IShutdownManagerStatics(|this| unsafe { (::windows::core::Vtable::vtable(this).CancelShutdown)(::windows::core::Vtable::as_raw(this)).ok() })
    }
    pub fn IsPowerStateSupported(powerstate: PowerState) -> ::windows::core::Result<bool> {
        Self::IShutdownManagerStatics2(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).IsPowerStateSupported)(::windows::core::Vtable::as_raw(this), powerstate, result__.as_mut_ptr()).from_abi(result__)
        })
    }
    pub fn EnterPowerState(powerstate: PowerState) -> ::windows::core::Result<()> {
        Self::IShutdownManagerStatics2(|this| unsafe { (::windows::core::Vtable::vtable(this).EnterPowerState)(::windows::core::Vtable::as_raw(this), powerstate).ok() })
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn EnterPowerStateWithTimeSpan(powerstate: PowerState, wakeupafter: super::Foundation::TimeSpan) -> ::windows::core::Result<()> {
        Self::IShutdownManagerStatics2(|this| unsafe { (::windows::core::Vtable::vtable(this).EnterPowerStateWithTimeSpan)(::windows::core::Vtable::as_raw(this), powerstate, wakeupafter).ok() })
    }
    #[doc(hidden)]
    pub fn IShutdownManagerStatics<R, F: FnOnce(&IShutdownManagerStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<ShutdownManager, IShutdownManagerStatics> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn IShutdownManagerStatics2<R, F: FnOnce(&IShutdownManagerStatics2) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<ShutdownManager, IShutdownManagerStatics2> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::windows::core::RuntimeName for ShutdownManager {
    const NAME: &'static str = "Windows.System.ShutdownManager";
}
#[doc = "*Required features: `\"System\"`*"]
pub struct TimeZoneSettings;
impl TimeZoneSettings {
    pub fn CurrentTimeZoneDisplayName() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::ITimeZoneSettingsStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CurrentTimeZoneDisplayName)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn SupportedTimeZoneDisplayNames() -> ::windows::core::Result<super::Foundation::Collections::IVectorView<::windows::core::HSTRING>> {
        Self::ITimeZoneSettingsStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).SupportedTimeZoneDisplayNames)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    pub fn CanChangeTimeZone() -> ::windows::core::Result<bool> {
        Self::ITimeZoneSettingsStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CanChangeTimeZone)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    pub fn ChangeTimeZoneByDisplayName(timezonedisplayname: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        Self::ITimeZoneSettingsStatics(|this| unsafe { (::windows::core::Vtable::vtable(this).ChangeTimeZoneByDisplayName)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(timezonedisplayname)).ok() })
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn AutoUpdateTimeZoneAsync(timeout: super::Foundation::TimeSpan) -> ::windows::core::Result<super::Foundation::IAsyncOperation<AutoUpdateTimeZoneStatus>> {
        Self::ITimeZoneSettingsStatics2(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).AutoUpdateTimeZoneAsync)(::windows::core::Vtable::as_raw(this), timeout, result__.as_mut_ptr()).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn ITimeZoneSettingsStatics<R, F: FnOnce(&ITimeZoneSettingsStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<TimeZoneSettings, ITimeZoneSettingsStatics> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn ITimeZoneSettingsStatics2<R, F: FnOnce(&ITimeZoneSettingsStatics2) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<TimeZoneSettings, ITimeZoneSettingsStatics2> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::windows::core::RuntimeName for TimeZoneSettings {
    const NAME: &'static str = "Windows.System.TimeZoneSettings";
}
#[doc = "*Required features: `\"System\"`*"]
#[repr(transparent)]
pub struct User(::windows::core::IUnknown);
impl User {
    pub fn NonRoamableId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).NonRoamableId)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn AuthenticationStatus(&self) -> ::windows::core::Result<UserAuthenticationStatus> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).AuthenticationStatus)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn Type(&self) -> ::windows::core::Result<UserType> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Type)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn GetPropertyAsync(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<super::Foundation::IAsyncOperation<::windows::core::IInspectable>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetPropertyAsync)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(value), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetPropertiesAsync<P0, E0>(&self, values: P0) -> ::windows::core::Result<super::Foundation::IAsyncOperation<super::Foundation::Collections::IPropertySet>>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<super::Foundation::Collections::IVectorView<::windows::core::HSTRING>>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetPropertiesAsync)(::windows::core::Vtable::as_raw(this), values.try_into().map_err(|e| e.into())?.abi(), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"Storage_Streams\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub fn GetPictureAsync(&self, desiredsize: UserPictureSize) -> ::windows::core::Result<super::Foundation::IAsyncOperation<super::Storage::Streams::IRandomAccessStreamReference>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetPictureAsync)(::windows::core::Vtable::as_raw(this), desiredsize, result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn CheckUserAgeConsentGroupAsync(&self, consentgroup: UserAgeConsentGroup) -> ::windows::core::Result<super::Foundation::IAsyncOperation<UserAgeConsentResult>> {
        let this = &::windows::core::Interface::cast::<IUser2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CheckUserAgeConsentGroupAsync)(::windows::core::Vtable::as_raw(this), consentgroup, result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn CreateWatcher() -> ::windows::core::Result<UserWatcher> {
        Self::IUserStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CreateWatcher)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn FindAllAsync() -> ::windows::core::Result<super::Foundation::IAsyncOperation<super::Foundation::Collections::IVectorView<User>>> {
        Self::IUserStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).FindAllAsync)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`, `\"deprecated\"`*"]
    #[cfg(all(feature = "Foundation_Collections", feature = "deprecated"))]
    pub fn FindAllAsyncByType(r#type: UserType) -> ::windows::core::Result<super::Foundation::IAsyncOperation<super::Foundation::Collections::IVectorView<User>>> {
        Self::IUserStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).FindAllAsyncByType)(::windows::core::Vtable::as_raw(this), r#type, result__.as_mut_ptr()).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`, `\"deprecated\"`*"]
    #[cfg(all(feature = "Foundation_Collections", feature = "deprecated"))]
    pub fn FindAllAsyncByTypeAndStatus(r#type: UserType, status: UserAuthenticationStatus) -> ::windows::core::Result<super::Foundation::IAsyncOperation<super::Foundation::Collections::IVectorView<User>>> {
        Self::IUserStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).FindAllAsyncByTypeAndStatus)(::windows::core::Vtable::as_raw(this), r#type, status, result__.as_mut_ptr()).from_abi(result__)
        })
    }
    pub fn GetFromId(nonroamableid: &::windows::core::HSTRING) -> ::windows::core::Result<User> {
        Self::IUserStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetFromId)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(nonroamableid), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    pub fn GetDefault() -> ::windows::core::Result<User> {
        Self::IUserStatics2(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetDefault)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IUserStatics<R, F: FnOnce(&IUserStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<User, IUserStatics> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn IUserStatics2<R, F: FnOnce(&IUserStatics2) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<User, IUserStatics2> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::clone::Clone for User {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for User {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for User {}
impl ::core::fmt::Debug for User {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("User").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for User {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.System.User;{df9a26c6-e746-4bcd-b5d4-120103c4209b})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for User {
    type Vtable = IUser_Vtbl;
}
unsafe impl ::windows::core::Interface for User {
    const IID: ::windows::core::GUID = <IUser as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for User {
    const NAME: &'static str = "Windows.System.User";
}
::windows::core::interface_hierarchy!(User, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for User {}
unsafe impl ::core::marker::Sync for User {}
#[doc = "*Required features: `\"System\"`*"]
#[repr(transparent)]
pub struct UserAuthenticationStatusChangeDeferral(::windows::core::IUnknown);
impl UserAuthenticationStatusChangeDeferral {
    pub fn Complete(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).Complete)(::windows::core::Vtable::as_raw(this)).ok() }
    }
}
impl ::core::clone::Clone for UserAuthenticationStatusChangeDeferral {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for UserAuthenticationStatusChangeDeferral {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for UserAuthenticationStatusChangeDeferral {}
impl ::core::fmt::Debug for UserAuthenticationStatusChangeDeferral {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UserAuthenticationStatusChangeDeferral").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for UserAuthenticationStatusChangeDeferral {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.System.UserAuthenticationStatusChangeDeferral;{88b59568-bb30-42fb-a270-e9902e40efa7})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for UserAuthenticationStatusChangeDeferral {
    type Vtable = IUserAuthenticationStatusChangeDeferral_Vtbl;
}
unsafe impl ::windows::core::Interface for UserAuthenticationStatusChangeDeferral {
    const IID: ::windows::core::GUID = <IUserAuthenticationStatusChangeDeferral as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for UserAuthenticationStatusChangeDeferral {
    const NAME: &'static str = "Windows.System.UserAuthenticationStatusChangeDeferral";
}
::windows::core::interface_hierarchy!(UserAuthenticationStatusChangeDeferral, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for UserAuthenticationStatusChangeDeferral {}
unsafe impl ::core::marker::Sync for UserAuthenticationStatusChangeDeferral {}
#[doc = "*Required features: `\"System\"`*"]
#[repr(transparent)]
pub struct UserAuthenticationStatusChangingEventArgs(::windows::core::IUnknown);
impl UserAuthenticationStatusChangingEventArgs {
    pub fn GetDeferral(&self) -> ::windows::core::Result<UserAuthenticationStatusChangeDeferral> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetDeferral)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn User(&self) -> ::windows::core::Result<User> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).User)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn NewStatus(&self) -> ::windows::core::Result<UserAuthenticationStatus> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).NewStatus)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn CurrentStatus(&self) -> ::windows::core::Result<UserAuthenticationStatus> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CurrentStatus)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
}
impl ::core::clone::Clone for UserAuthenticationStatusChangingEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for UserAuthenticationStatusChangingEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for UserAuthenticationStatusChangingEventArgs {}
impl ::core::fmt::Debug for UserAuthenticationStatusChangingEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UserAuthenticationStatusChangingEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for UserAuthenticationStatusChangingEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.System.UserAuthenticationStatusChangingEventArgs;{8c030f28-a711-4c1e-ab48-04179c15938f})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for UserAuthenticationStatusChangingEventArgs {
    type Vtable = IUserAuthenticationStatusChangingEventArgs_Vtbl;
}
unsafe impl ::windows::core::Interface for UserAuthenticationStatusChangingEventArgs {
    const IID: ::windows::core::GUID = <IUserAuthenticationStatusChangingEventArgs as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for UserAuthenticationStatusChangingEventArgs {
    const NAME: &'static str = "Windows.System.UserAuthenticationStatusChangingEventArgs";
}
::windows::core::interface_hierarchy!(UserAuthenticationStatusChangingEventArgs, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for UserAuthenticationStatusChangingEventArgs {}
unsafe impl ::core::marker::Sync for UserAuthenticationStatusChangingEventArgs {}
#[doc = "*Required features: `\"System\"`*"]
#[repr(transparent)]
pub struct UserChangedEventArgs(::windows::core::IUnknown);
impl UserChangedEventArgs {
    pub fn User(&self) -> ::windows::core::Result<User> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).User)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn ChangedPropertyKinds(&self) -> ::windows::core::Result<super::Foundation::Collections::IVectorView<UserWatcherUpdateKind>> {
        let this = &::windows::core::Interface::cast::<IUserChangedEventArgs2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ChangedPropertyKinds)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
}
impl ::core::clone::Clone for UserChangedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for UserChangedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for UserChangedEventArgs {}
impl ::core::fmt::Debug for UserChangedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UserChangedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for UserChangedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.System.UserChangedEventArgs;{086459dc-18c6-48db-bc99-724fb9203ccc})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for UserChangedEventArgs {
    type Vtable = IUserChangedEventArgs_Vtbl;
}
unsafe impl ::windows::core::Interface for UserChangedEventArgs {
    const IID: ::windows::core::GUID = <IUserChangedEventArgs as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for UserChangedEventArgs {
    const NAME: &'static str = "Windows.System.UserChangedEventArgs";
}
::windows::core::interface_hierarchy!(UserChangedEventArgs, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for UserChangedEventArgs {}
unsafe impl ::core::marker::Sync for UserChangedEventArgs {}
#[doc = "*Required features: `\"System\"`*"]
pub struct UserDeviceAssociation;
impl UserDeviceAssociation {
    pub fn FindUserFromDeviceId(deviceid: &::windows::core::HSTRING) -> ::windows::core::Result<User> {
        Self::IUserDeviceAssociationStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).FindUserFromDeviceId)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(deviceid), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn UserDeviceAssociationChanged(handler: &super::Foundation::EventHandler<UserDeviceAssociationChangedEventArgs>) -> ::windows::core::Result<super::Foundation::EventRegistrationToken> {
        Self::IUserDeviceAssociationStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).UserDeviceAssociationChanged)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(handler), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveUserDeviceAssociationChanged(token: super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        Self::IUserDeviceAssociationStatics(|this| unsafe { (::windows::core::Vtable::vtable(this).RemoveUserDeviceAssociationChanged)(::windows::core::Vtable::as_raw(this), token).ok() })
    }
    #[doc(hidden)]
    pub fn IUserDeviceAssociationStatics<R, F: FnOnce(&IUserDeviceAssociationStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<UserDeviceAssociation, IUserDeviceAssociationStatics> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::windows::core::RuntimeName for UserDeviceAssociation {
    const NAME: &'static str = "Windows.System.UserDeviceAssociation";
}
#[doc = "*Required features: `\"System\"`*"]
#[repr(transparent)]
pub struct UserDeviceAssociationChangedEventArgs(::windows::core::IUnknown);
impl UserDeviceAssociationChangedEventArgs {
    pub fn DeviceId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).DeviceId)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn NewUser(&self) -> ::windows::core::Result<User> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).NewUser)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn OldUser(&self) -> ::windows::core::Result<User> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).OldUser)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
}
impl ::core::clone::Clone for UserDeviceAssociationChangedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for UserDeviceAssociationChangedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for UserDeviceAssociationChangedEventArgs {}
impl ::core::fmt::Debug for UserDeviceAssociationChangedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UserDeviceAssociationChangedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for UserDeviceAssociationChangedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.System.UserDeviceAssociationChangedEventArgs;{bd1f6f6c-bb5d-4d7b-a5f0-c8cd11a38d42})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for UserDeviceAssociationChangedEventArgs {
    type Vtable = IUserDeviceAssociationChangedEventArgs_Vtbl;
}
unsafe impl ::windows::core::Interface for UserDeviceAssociationChangedEventArgs {
    const IID: ::windows::core::GUID = <IUserDeviceAssociationChangedEventArgs as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for UserDeviceAssociationChangedEventArgs {
    const NAME: &'static str = "Windows.System.UserDeviceAssociationChangedEventArgs";
}
::windows::core::interface_hierarchy!(UserDeviceAssociationChangedEventArgs, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for UserDeviceAssociationChangedEventArgs {}
unsafe impl ::core::marker::Sync for UserDeviceAssociationChangedEventArgs {}
#[doc = "*Required features: `\"System\"`*"]
#[repr(transparent)]
pub struct UserPicker(::windows::core::IUnknown);
impl UserPicker {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IGenericFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<UserPicker, ::windows::core::IGenericFactory> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn AllowGuestAccounts(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).AllowGuestAccounts)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetAllowGuestAccounts(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetAllowGuestAccounts)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
    pub fn SuggestedSelectedUser(&self) -> ::windows::core::Result<User> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).SuggestedSelectedUser)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetSuggestedSelectedUser(&self, value: &User) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetSuggestedSelectedUser)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn PickSingleUserAsync(&self) -> ::windows::core::Result<super::Foundation::IAsyncOperation<User>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).PickSingleUserAsync)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn IsSupported() -> ::windows::core::Result<bool> {
        Self::IUserPickerStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).IsSupported)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IUserPickerStatics<R, F: FnOnce(&IUserPickerStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<UserPicker, IUserPickerStatics> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::clone::Clone for UserPicker {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for UserPicker {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for UserPicker {}
impl ::core::fmt::Debug for UserPicker {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UserPicker").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for UserPicker {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.System.UserPicker;{7d548008-f1e3-4a6c-8ddc-a9bb0f488aed})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for UserPicker {
    type Vtable = IUserPicker_Vtbl;
}
unsafe impl ::windows::core::Interface for UserPicker {
    const IID: ::windows::core::GUID = <IUserPicker as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for UserPicker {
    const NAME: &'static str = "Windows.System.UserPicker";
}
::windows::core::interface_hierarchy!(UserPicker, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for UserPicker {}
unsafe impl ::core::marker::Sync for UserPicker {}
#[doc = "*Required features: `\"System\"`*"]
#[repr(transparent)]
pub struct UserWatcher(::windows::core::IUnknown);
impl UserWatcher {
    pub fn Status(&self) -> ::windows::core::Result<UserWatcherStatus> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Status)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn Start(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).Start)(::windows::core::Vtable::as_raw(this)).ok() }
    }
    pub fn Stop(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).Stop)(::windows::core::Vtable::as_raw(this)).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Added(&self, handler: &super::Foundation::TypedEventHandler<UserWatcher, UserChangedEventArgs>) -> ::windows::core::Result<super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Added)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(handler), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveAdded(&self, token: super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).RemoveAdded)(::windows::core::Vtable::as_raw(this), token).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Removed(&self, handler: &super::Foundation::TypedEventHandler<UserWatcher, UserChangedEventArgs>) -> ::windows::core::Result<super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Removed)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(handler), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveRemoved(&self, token: super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).RemoveRemoved)(::windows::core::Vtable::as_raw(this), token).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Updated(&self, handler: &super::Foundation::TypedEventHandler<UserWatcher, UserChangedEventArgs>) -> ::windows::core::Result<super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Updated)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(handler), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveUpdated(&self, token: super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).RemoveUpdated)(::windows::core::Vtable::as_raw(this), token).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn AuthenticationStatusChanged(&self, handler: &super::Foundation::TypedEventHandler<UserWatcher, UserChangedEventArgs>) -> ::windows::core::Result<super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).AuthenticationStatusChanged)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(handler), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveAuthenticationStatusChanged(&self, token: super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).RemoveAuthenticationStatusChanged)(::windows::core::Vtable::as_raw(this), token).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn AuthenticationStatusChanging(&self, handler: &super::Foundation::TypedEventHandler<UserWatcher, UserAuthenticationStatusChangingEventArgs>) -> ::windows::core::Result<super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).AuthenticationStatusChanging)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(handler), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveAuthenticationStatusChanging(&self, token: super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).RemoveAuthenticationStatusChanging)(::windows::core::Vtable::as_raw(this), token).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn EnumerationCompleted(&self, handler: &super::Foundation::TypedEventHandler<UserWatcher, ::windows::core::IInspectable>) -> ::windows::core::Result<super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).EnumerationCompleted)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(handler), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveEnumerationCompleted(&self, token: super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).RemoveEnumerationCompleted)(::windows::core::Vtable::as_raw(this), token).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Stopped(&self, handler: &super::Foundation::TypedEventHandler<UserWatcher, ::windows::core::IInspectable>) -> ::windows::core::Result<super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Stopped)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(handler), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveStopped(&self, token: super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).RemoveStopped)(::windows::core::Vtable::as_raw(this), token).ok() }
    }
}
impl ::core::clone::Clone for UserWatcher {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for UserWatcher {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for UserWatcher {}
impl ::core::fmt::Debug for UserWatcher {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UserWatcher").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for UserWatcher {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.System.UserWatcher;{155eb23b-242a-45e0-a2e9-3171fc6a7fbb})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for UserWatcher {
    type Vtable = IUserWatcher_Vtbl;
}
unsafe impl ::windows::core::Interface for UserWatcher {
    const IID: ::windows::core::GUID = <IUserWatcher as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for UserWatcher {
    const NAME: &'static str = "Windows.System.UserWatcher";
}
::windows::core::interface_hierarchy!(UserWatcher, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for UserWatcher {}
unsafe impl ::core::marker::Sync for UserWatcher {}
#[doc = "*Required features: `\"System\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct AppDiagnosticInfoWatcherStatus(pub i32);
impl AppDiagnosticInfoWatcherStatus {
    pub const Created: Self = Self(0i32);
    pub const Started: Self = Self(1i32);
    pub const EnumerationCompleted: Self = Self(2i32);
    pub const Stopping: Self = Self(3i32);
    pub const Stopped: Self = Self(4i32);
    pub const Aborted: Self = Self(5i32);
}
impl ::core::marker::Copy for AppDiagnosticInfoWatcherStatus {}
impl ::core::clone::Clone for AppDiagnosticInfoWatcherStatus {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for AppDiagnosticInfoWatcherStatus {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for AppDiagnosticInfoWatcherStatus {
    type Abi = Self;
}
impl ::core::fmt::Debug for AppDiagnosticInfoWatcherStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AppDiagnosticInfoWatcherStatus").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for AppDiagnosticInfoWatcherStatus {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.System.AppDiagnosticInfoWatcherStatus;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"System\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct AppMemoryUsageLevel(pub i32);
impl AppMemoryUsageLevel {
    pub const Low: Self = Self(0i32);
    pub const Medium: Self = Self(1i32);
    pub const High: Self = Self(2i32);
    pub const OverLimit: Self = Self(3i32);
}
impl ::core::marker::Copy for AppMemoryUsageLevel {}
impl ::core::clone::Clone for AppMemoryUsageLevel {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for AppMemoryUsageLevel {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for AppMemoryUsageLevel {
    type Abi = Self;
}
impl ::core::fmt::Debug for AppMemoryUsageLevel {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AppMemoryUsageLevel").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for AppMemoryUsageLevel {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.System.AppMemoryUsageLevel;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"System\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct AppResourceGroupEnergyQuotaState(pub i32);
impl AppResourceGroupEnergyQuotaState {
    pub const Unknown: Self = Self(0i32);
    pub const Over: Self = Self(1i32);
    pub const Under: Self = Self(2i32);
}
impl ::core::marker::Copy for AppResourceGroupEnergyQuotaState {}
impl ::core::clone::Clone for AppResourceGroupEnergyQuotaState {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for AppResourceGroupEnergyQuotaState {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for AppResourceGroupEnergyQuotaState {
    type Abi = Self;
}
impl ::core::fmt::Debug for AppResourceGroupEnergyQuotaState {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AppResourceGroupEnergyQuotaState").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for AppResourceGroupEnergyQuotaState {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.System.AppResourceGroupEnergyQuotaState;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"System\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct AppResourceGroupExecutionState(pub i32);
impl AppResourceGroupExecutionState {
    pub const Unknown: Self = Self(0i32);
    pub const Running: Self = Self(1i32);
    pub const Suspending: Self = Self(2i32);
    pub const Suspended: Self = Self(3i32);
    pub const NotRunning: Self = Self(4i32);
}
impl ::core::marker::Copy for AppResourceGroupExecutionState {}
impl ::core::clone::Clone for AppResourceGroupExecutionState {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for AppResourceGroupExecutionState {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for AppResourceGroupExecutionState {
    type Abi = Self;
}
impl ::core::fmt::Debug for AppResourceGroupExecutionState {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AppResourceGroupExecutionState").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for AppResourceGroupExecutionState {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.System.AppResourceGroupExecutionState;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"System\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct AppResourceGroupInfoWatcherStatus(pub i32);
impl AppResourceGroupInfoWatcherStatus {
    pub const Created: Self = Self(0i32);
    pub const Started: Self = Self(1i32);
    pub const EnumerationCompleted: Self = Self(2i32);
    pub const Stopping: Self = Self(3i32);
    pub const Stopped: Self = Self(4i32);
    pub const Aborted: Self = Self(5i32);
}
impl ::core::marker::Copy for AppResourceGroupInfoWatcherStatus {}
impl ::core::clone::Clone for AppResourceGroupInfoWatcherStatus {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for AppResourceGroupInfoWatcherStatus {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for AppResourceGroupInfoWatcherStatus {
    type Abi = Self;
}
impl ::core::fmt::Debug for AppResourceGroupInfoWatcherStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AppResourceGroupInfoWatcherStatus").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for AppResourceGroupInfoWatcherStatus {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.System.AppResourceGroupInfoWatcherStatus;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"System\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct AutoUpdateTimeZoneStatus(pub i32);
impl AutoUpdateTimeZoneStatus {
    pub const Attempted: Self = Self(0i32);
    pub const TimedOut: Self = Self(1i32);
    pub const Failed: Self = Self(2i32);
}
impl ::core::marker::Copy for AutoUpdateTimeZoneStatus {}
impl ::core::clone::Clone for AutoUpdateTimeZoneStatus {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for AutoUpdateTimeZoneStatus {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for AutoUpdateTimeZoneStatus {
    type Abi = Self;
}
impl ::core::fmt::Debug for AutoUpdateTimeZoneStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AutoUpdateTimeZoneStatus").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for AutoUpdateTimeZoneStatus {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.System.AutoUpdateTimeZoneStatus;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"System\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct DiagnosticAccessStatus(pub i32);
impl DiagnosticAccessStatus {
    pub const Unspecified: Self = Self(0i32);
    pub const Denied: Self = Self(1i32);
    pub const Limited: Self = Self(2i32);
    pub const Allowed: Self = Self(3i32);
}
impl ::core::marker::Copy for DiagnosticAccessStatus {}
impl ::core::clone::Clone for DiagnosticAccessStatus {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DiagnosticAccessStatus {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for DiagnosticAccessStatus {
    type Abi = Self;
}
impl ::core::fmt::Debug for DiagnosticAccessStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DiagnosticAccessStatus").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for DiagnosticAccessStatus {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.System.DiagnosticAccessStatus;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"System\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct DispatcherQueuePriority(pub i32);
impl DispatcherQueuePriority {
    pub const Low: Self = Self(-10i32);
    pub const Normal: Self = Self(0i32);
    pub const High: Self = Self(10i32);
}
impl ::core::marker::Copy for DispatcherQueuePriority {}
impl ::core::clone::Clone for DispatcherQueuePriority {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DispatcherQueuePriority {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for DispatcherQueuePriority {
    type Abi = Self;
}
impl ::core::fmt::Debug for DispatcherQueuePriority {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DispatcherQueuePriority").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for DispatcherQueuePriority {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.System.DispatcherQueuePriority;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"System\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct LaunchFileStatus(pub i32);
impl LaunchFileStatus {
    pub const Success: Self = Self(0i32);
    pub const AppUnavailable: Self = Self(1i32);
    pub const DeniedByPolicy: Self = Self(2i32);
    pub const FileTypeNotSupported: Self = Self(3i32);
    pub const Unknown: Self = Self(4i32);
}
impl ::core::marker::Copy for LaunchFileStatus {}
impl ::core::clone::Clone for LaunchFileStatus {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for LaunchFileStatus {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for LaunchFileStatus {
    type Abi = Self;
}
impl ::core::fmt::Debug for LaunchFileStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("LaunchFileStatus").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for LaunchFileStatus {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.System.LaunchFileStatus;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"System\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct LaunchQuerySupportStatus(pub i32);
impl LaunchQuerySupportStatus {
    pub const Available: Self = Self(0i32);
    pub const AppNotInstalled: Self = Self(1i32);
    pub const AppUnavailable: Self = Self(2i32);
    pub const NotSupported: Self = Self(3i32);
    pub const Unknown: Self = Self(4i32);
}
impl ::core::marker::Copy for LaunchQuerySupportStatus {}
impl ::core::clone::Clone for LaunchQuerySupportStatus {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for LaunchQuerySupportStatus {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for LaunchQuerySupportStatus {
    type Abi = Self;
}
impl ::core::fmt::Debug for LaunchQuerySupportStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("LaunchQuerySupportStatus").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for LaunchQuerySupportStatus {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.System.LaunchQuerySupportStatus;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"System\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct LaunchQuerySupportType(pub i32);
impl LaunchQuerySupportType {
    pub const Uri: Self = Self(0i32);
    pub const UriForResults: Self = Self(1i32);
}
impl ::core::marker::Copy for LaunchQuerySupportType {}
impl ::core::clone::Clone for LaunchQuerySupportType {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for LaunchQuerySupportType {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for LaunchQuerySupportType {
    type Abi = Self;
}
impl ::core::fmt::Debug for LaunchQuerySupportType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("LaunchQuerySupportType").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for LaunchQuerySupportType {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.System.LaunchQuerySupportType;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"System\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct LaunchUriStatus(pub i32);
impl LaunchUriStatus {
    pub const Success: Self = Self(0i32);
    pub const AppUnavailable: Self = Self(1i32);
    pub const ProtocolUnavailable: Self = Self(2i32);
    pub const Unknown: Self = Self(3i32);
}
impl ::core::marker::Copy for LaunchUriStatus {}
impl ::core::clone::Clone for LaunchUriStatus {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for LaunchUriStatus {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for LaunchUriStatus {
    type Abi = Self;
}
impl ::core::fmt::Debug for LaunchUriStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("LaunchUriStatus").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for LaunchUriStatus {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.System.LaunchUriStatus;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"System\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct PowerState(pub i32);
impl PowerState {
    pub const ConnectedStandby: Self = Self(0i32);
    pub const SleepS3: Self = Self(1i32);
}
impl ::core::marker::Copy for PowerState {}
impl ::core::clone::Clone for PowerState {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PowerState {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for PowerState {
    type Abi = Self;
}
impl ::core::fmt::Debug for PowerState {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PowerState").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for PowerState {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.System.PowerState;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"System\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct ProcessorArchitecture(pub i32);
impl ProcessorArchitecture {
    pub const X86: Self = Self(0i32);
    pub const Arm: Self = Self(5i32);
    pub const X64: Self = Self(9i32);
    pub const Neutral: Self = Self(11i32);
    pub const Arm64: Self = Self(12i32);
    pub const X86OnArm64: Self = Self(14i32);
    pub const Unknown: Self = Self(65535i32);
}
impl ::core::marker::Copy for ProcessorArchitecture {}
impl ::core::clone::Clone for ProcessorArchitecture {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for ProcessorArchitecture {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for ProcessorArchitecture {
    type Abi = Self;
}
impl ::core::fmt::Debug for ProcessorArchitecture {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ProcessorArchitecture").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for ProcessorArchitecture {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.System.ProcessorArchitecture;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"System\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct RemoteLaunchUriStatus(pub i32);
impl RemoteLaunchUriStatus {
    pub const Unknown: Self = Self(0i32);
    pub const Success: Self = Self(1i32);
    pub const AppUnavailable: Self = Self(2i32);
    pub const ProtocolUnavailable: Self = Self(3i32);
    pub const RemoteSystemUnavailable: Self = Self(4i32);
    pub const ValueSetTooLarge: Self = Self(5i32);
    pub const DeniedByLocalSystem: Self = Self(6i32);
    pub const DeniedByRemoteSystem: Self = Self(7i32);
}
impl ::core::marker::Copy for RemoteLaunchUriStatus {}
impl ::core::clone::Clone for RemoteLaunchUriStatus {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for RemoteLaunchUriStatus {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for RemoteLaunchUriStatus {
    type Abi = Self;
}
impl ::core::fmt::Debug for RemoteLaunchUriStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RemoteLaunchUriStatus").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for RemoteLaunchUriStatus {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.System.RemoteLaunchUriStatus;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"System\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct ShutdownKind(pub i32);
impl ShutdownKind {
    pub const Shutdown: Self = Self(0i32);
    pub const Restart: Self = Self(1i32);
}
impl ::core::marker::Copy for ShutdownKind {}
impl ::core::clone::Clone for ShutdownKind {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for ShutdownKind {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for ShutdownKind {
    type Abi = Self;
}
impl ::core::fmt::Debug for ShutdownKind {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ShutdownKind").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for ShutdownKind {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.System.ShutdownKind;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"System\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct UserAgeConsentGroup(pub i32);
impl UserAgeConsentGroup {
    pub const Child: Self = Self(0i32);
    pub const Minor: Self = Self(1i32);
    pub const Adult: Self = Self(2i32);
}
impl ::core::marker::Copy for UserAgeConsentGroup {}
impl ::core::clone::Clone for UserAgeConsentGroup {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for UserAgeConsentGroup {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for UserAgeConsentGroup {
    type Abi = Self;
}
impl ::core::fmt::Debug for UserAgeConsentGroup {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UserAgeConsentGroup").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for UserAgeConsentGroup {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.System.UserAgeConsentGroup;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"System\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct UserAgeConsentResult(pub i32);
impl UserAgeConsentResult {
    pub const NotEnforced: Self = Self(0i32);
    pub const Included: Self = Self(1i32);
    pub const NotIncluded: Self = Self(2i32);
    pub const Unknown: Self = Self(3i32);
    pub const Ambiguous: Self = Self(4i32);
}
impl ::core::marker::Copy for UserAgeConsentResult {}
impl ::core::clone::Clone for UserAgeConsentResult {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for UserAgeConsentResult {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for UserAgeConsentResult {
    type Abi = Self;
}
impl ::core::fmt::Debug for UserAgeConsentResult {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UserAgeConsentResult").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for UserAgeConsentResult {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.System.UserAgeConsentResult;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"System\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct UserAuthenticationStatus(pub i32);
impl UserAuthenticationStatus {
    pub const Unauthenticated: Self = Self(0i32);
    pub const LocallyAuthenticated: Self = Self(1i32);
    pub const RemotelyAuthenticated: Self = Self(2i32);
}
impl ::core::marker::Copy for UserAuthenticationStatus {}
impl ::core::clone::Clone for UserAuthenticationStatus {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for UserAuthenticationStatus {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for UserAuthenticationStatus {
    type Abi = Self;
}
impl ::core::fmt::Debug for UserAuthenticationStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UserAuthenticationStatus").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for UserAuthenticationStatus {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.System.UserAuthenticationStatus;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"System\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct UserPictureSize(pub i32);
impl UserPictureSize {
    pub const Size64x64: Self = Self(0i32);
    pub const Size208x208: Self = Self(1i32);
    pub const Size424x424: Self = Self(2i32);
    pub const Size1080x1080: Self = Self(3i32);
}
impl ::core::marker::Copy for UserPictureSize {}
impl ::core::clone::Clone for UserPictureSize {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for UserPictureSize {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for UserPictureSize {
    type Abi = Self;
}
impl ::core::fmt::Debug for UserPictureSize {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UserPictureSize").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for UserPictureSize {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.System.UserPictureSize;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"System\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct UserType(pub i32);
impl UserType {
    pub const LocalUser: Self = Self(0i32);
    pub const RemoteUser: Self = Self(1i32);
    pub const LocalGuest: Self = Self(2i32);
    pub const RemoteGuest: Self = Self(3i32);
    pub const SystemManaged: Self = Self(4i32);
}
impl ::core::marker::Copy for UserType {}
impl ::core::clone::Clone for UserType {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for UserType {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for UserType {
    type Abi = Self;
}
impl ::core::fmt::Debug for UserType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UserType").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for UserType {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.System.UserType;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"System\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct UserWatcherStatus(pub i32);
impl UserWatcherStatus {
    pub const Created: Self = Self(0i32);
    pub const Started: Self = Self(1i32);
    pub const EnumerationCompleted: Self = Self(2i32);
    pub const Stopping: Self = Self(3i32);
    pub const Stopped: Self = Self(4i32);
    pub const Aborted: Self = Self(5i32);
}
impl ::core::marker::Copy for UserWatcherStatus {}
impl ::core::clone::Clone for UserWatcherStatus {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for UserWatcherStatus {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for UserWatcherStatus {
    type Abi = Self;
}
impl ::core::fmt::Debug for UserWatcherStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UserWatcherStatus").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for UserWatcherStatus {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.System.UserWatcherStatus;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"System\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct UserWatcherUpdateKind(pub i32);
impl UserWatcherUpdateKind {
    pub const Properties: Self = Self(0i32);
    pub const Picture: Self = Self(1i32);
}
impl ::core::marker::Copy for UserWatcherUpdateKind {}
impl ::core::clone::Clone for UserWatcherUpdateKind {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for UserWatcherUpdateKind {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for UserWatcherUpdateKind {
    type Abi = Self;
}
impl ::core::fmt::Debug for UserWatcherUpdateKind {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UserWatcherUpdateKind").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for UserWatcherUpdateKind {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.System.UserWatcherUpdateKind;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"System\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct VirtualKey(pub i32);
impl VirtualKey {
    pub const None: Self = Self(0i32);
    pub const LeftButton: Self = Self(1i32);
    pub const RightButton: Self = Self(2i32);
    pub const Cancel: Self = Self(3i32);
    pub const MiddleButton: Self = Self(4i32);
    pub const XButton1: Self = Self(5i32);
    pub const XButton2: Self = Self(6i32);
    pub const Back: Self = Self(8i32);
    pub const Tab: Self = Self(9i32);
    pub const Clear: Self = Self(12i32);
    pub const Enter: Self = Self(13i32);
    pub const Shift: Self = Self(16i32);
    pub const Control: Self = Self(17i32);
    pub const Menu: Self = Self(18i32);
    pub const Pause: Self = Self(19i32);
    pub const CapitalLock: Self = Self(20i32);
    pub const Kana: Self = Self(21i32);
    pub const Hangul: Self = Self(21i32);
    pub const ImeOn: Self = Self(22i32);
    pub const Junja: Self = Self(23i32);
    pub const Final: Self = Self(24i32);
    pub const Hanja: Self = Self(25i32);
    pub const Kanji: Self = Self(25i32);
    pub const ImeOff: Self = Self(26i32);
    pub const Escape: Self = Self(27i32);
    pub const Convert: Self = Self(28i32);
    pub const NonConvert: Self = Self(29i32);
    pub const Accept: Self = Self(30i32);
    pub const ModeChange: Self = Self(31i32);
    pub const Space: Self = Self(32i32);
    pub const PageUp: Self = Self(33i32);
    pub const PageDown: Self = Self(34i32);
    pub const End: Self = Self(35i32);
    pub const Home: Self = Self(36i32);
    pub const Left: Self = Self(37i32);
    pub const Up: Self = Self(38i32);
    pub const Right: Self = Self(39i32);
    pub const Down: Self = Self(40i32);
    pub const Select: Self = Self(41i32);
    pub const Print: Self = Self(42i32);
    pub const Execute: Self = Self(43i32);
    pub const Snapshot: Self = Self(44i32);
    pub const Insert: Self = Self(45i32);
    pub const Delete: Self = Self(46i32);
    pub const Help: Self = Self(47i32);
    pub const Number0: Self = Self(48i32);
    pub const Number1: Self = Self(49i32);
    pub const Number2: Self = Self(50i32);
    pub const Number3: Self = Self(51i32);
    pub const Number4: Self = Self(52i32);
    pub const Number5: Self = Self(53i32);
    pub const Number6: Self = Self(54i32);
    pub const Number7: Self = Self(55i32);
    pub const Number8: Self = Self(56i32);
    pub const Number9: Self = Self(57i32);
    pub const A: Self = Self(65i32);
    pub const B: Self = Self(66i32);
    pub const C: Self = Self(67i32);
    pub const D: Self = Self(68i32);
    pub const E: Self = Self(69i32);
    pub const F: Self = Self(70i32);
    pub const G: Self = Self(71i32);
    pub const H: Self = Self(72i32);
    pub const I: Self = Self(73i32);
    pub const J: Self = Self(74i32);
    pub const K: Self = Self(75i32);
    pub const L: Self = Self(76i32);
    pub const M: Self = Self(77i32);
    pub const N: Self = Self(78i32);
    pub const O: Self = Self(79i32);
    pub const P: Self = Self(80i32);
    pub const Q: Self = Self(81i32);
    pub const R: Self = Self(82i32);
    pub const S: Self = Self(83i32);
    pub const T: Self = Self(84i32);
    pub const U: Self = Self(85i32);
    pub const V: Self = Self(86i32);
    pub const W: Self = Self(87i32);
    pub const X: Self = Self(88i32);
    pub const Y: Self = Self(89i32);
    pub const Z: Self = Self(90i32);
    pub const LeftWindows: Self = Self(91i32);
    pub const RightWindows: Self = Self(92i32);
    pub const Application: Self = Self(93i32);
    pub const Sleep: Self = Self(95i32);
    pub const NumberPad0: Self = Self(96i32);
    pub const NumberPad1: Self = Self(97i32);
    pub const NumberPad2: Self = Self(98i32);
    pub const NumberPad3: Self = Self(99i32);
    pub const NumberPad4: Self = Self(100i32);
    pub const NumberPad5: Self = Self(101i32);
    pub const NumberPad6: Self = Self(102i32);
    pub const NumberPad7: Self = Self(103i32);
    pub const NumberPad8: Self = Self(104i32);
    pub const NumberPad9: Self = Self(105i32);
    pub const Multiply: Self = Self(106i32);
    pub const Add: Self = Self(107i32);
    pub const Separator: Self = Self(108i32);
    pub const Subtract: Self = Self(109i32);
    pub const Decimal: Self = Self(110i32);
    pub const Divide: Self = Self(111i32);
    pub const F1: Self = Self(112i32);
    pub const F2: Self = Self(113i32);
    pub const F3: Self = Self(114i32);
    pub const F4: Self = Self(115i32);
    pub const F5: Self = Self(116i32);
    pub const F6: Self = Self(117i32);
    pub const F7: Self = Self(118i32);
    pub const F8: Self = Self(119i32);
    pub const F9: Self = Self(120i32);
    pub const F10: Self = Self(121i32);
    pub const F11: Self = Self(122i32);
    pub const F12: Self = Self(123i32);
    pub const F13: Self = Self(124i32);
    pub const F14: Self = Self(125i32);
    pub const F15: Self = Self(126i32);
    pub const F16: Self = Self(127i32);
    pub const F17: Self = Self(128i32);
    pub const F18: Self = Self(129i32);
    pub const F19: Self = Self(130i32);
    pub const F20: Self = Self(131i32);
    pub const F21: Self = Self(132i32);
    pub const F22: Self = Self(133i32);
    pub const F23: Self = Self(134i32);
    pub const F24: Self = Self(135i32);
    pub const NavigationView: Self = Self(136i32);
    pub const NavigationMenu: Self = Self(137i32);
    pub const NavigationUp: Self = Self(138i32);
    pub const NavigationDown: Self = Self(139i32);
    pub const NavigationLeft: Self = Self(140i32);
    pub const NavigationRight: Self = Self(141i32);
    pub const NavigationAccept: Self = Self(142i32);
    pub const NavigationCancel: Self = Self(143i32);
    pub const NumberKeyLock: Self = Self(144i32);
    pub const Scroll: Self = Self(145i32);
    pub const LeftShift: Self = Self(160i32);
    pub const RightShift: Self = Self(161i32);
    pub const LeftControl: Self = Self(162i32);
    pub const RightControl: Self = Self(163i32);
    pub const LeftMenu: Self = Self(164i32);
    pub const RightMenu: Self = Self(165i32);
    pub const GoBack: Self = Self(166i32);
    pub const GoForward: Self = Self(167i32);
    pub const Refresh: Self = Self(168i32);
    pub const Stop: Self = Self(169i32);
    pub const Search: Self = Self(170i32);
    pub const Favorites: Self = Self(171i32);
    pub const GoHome: Self = Self(172i32);
    pub const GamepadA: Self = Self(195i32);
    pub const GamepadB: Self = Self(196i32);
    pub const GamepadX: Self = Self(197i32);
    pub const GamepadY: Self = Self(198i32);
    pub const GamepadRightShoulder: Self = Self(199i32);
    pub const GamepadLeftShoulder: Self = Self(200i32);
    pub const GamepadLeftTrigger: Self = Self(201i32);
    pub const GamepadRightTrigger: Self = Self(202i32);
    pub const GamepadDPadUp: Self = Self(203i32);
    pub const GamepadDPadDown: Self = Self(204i32);
    pub const GamepadDPadLeft: Self = Self(205i32);
    pub const GamepadDPadRight: Self = Self(206i32);
    pub const GamepadMenu: Self = Self(207i32);
    pub const GamepadView: Self = Self(208i32);
    pub const GamepadLeftThumbstickButton: Self = Self(209i32);
    pub const GamepadRightThumbstickButton: Self = Self(210i32);
    pub const GamepadLeftThumbstickUp: Self = Self(211i32);
    pub const GamepadLeftThumbstickDown: Self = Self(212i32);
    pub const GamepadLeftThumbstickRight: Self = Self(213i32);
    pub const GamepadLeftThumbstickLeft: Self = Self(214i32);
    pub const GamepadRightThumbstickUp: Self = Self(215i32);
    pub const GamepadRightThumbstickDown: Self = Self(216i32);
    pub const GamepadRightThumbstickRight: Self = Self(217i32);
    pub const GamepadRightThumbstickLeft: Self = Self(218i32);
}
impl ::core::marker::Copy for VirtualKey {}
impl ::core::clone::Clone for VirtualKey {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for VirtualKey {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for VirtualKey {
    type Abi = Self;
}
impl ::core::fmt::Debug for VirtualKey {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("VirtualKey").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for VirtualKey {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.System.VirtualKey;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"System\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct VirtualKeyModifiers(pub u32);
impl VirtualKeyModifiers {
    pub const None: Self = Self(0u32);
    pub const Control: Self = Self(1u32);
    pub const Menu: Self = Self(2u32);
    pub const Shift: Self = Self(4u32);
    pub const Windows: Self = Self(8u32);
}
impl ::core::marker::Copy for VirtualKeyModifiers {}
impl ::core::clone::Clone for VirtualKeyModifiers {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for VirtualKeyModifiers {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for VirtualKeyModifiers {
    type Abi = Self;
}
impl ::core::fmt::Debug for VirtualKeyModifiers {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("VirtualKeyModifiers").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for VirtualKeyModifiers {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for VirtualKeyModifiers {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for VirtualKeyModifiers {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for VirtualKeyModifiers {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for VirtualKeyModifiers {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
unsafe impl ::windows::core::RuntimeType for VirtualKeyModifiers {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.System.VirtualKeyModifiers;u4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"System\"`*"]
#[repr(transparent)]
pub struct DispatcherQueueHandler(pub ::windows::core::IUnknown);
impl DispatcherQueueHandler {
    pub fn new<F: FnMut() -> ::windows::core::Result<()> + ::core::marker::Send + 'static>(invoke: F) -> Self {
        let com = DispatcherQueueHandlerBox::<F> { vtable: &DispatcherQueueHandlerBox::<F>::VTABLE, count: ::windows::core::RefCount::new(1), invoke };
        unsafe { ::core::mem::transmute(::windows::core::alloc::boxed::Box::new(com)) }
    }
    pub fn Invoke(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).Invoke)(::windows::core::Vtable::as_raw(this)).ok() }
    }
}
#[repr(C)]
struct DispatcherQueueHandlerBox<F: FnMut() -> ::windows::core::Result<()> + ::core::marker::Send + 'static> {
    vtable: *const DispatcherQueueHandler_Vtbl,
    invoke: F,
    count: ::windows::core::RefCount,
}
impl<F: FnMut() -> ::windows::core::Result<()> + ::core::marker::Send + 'static> DispatcherQueueHandlerBox<F> {
    const VTABLE: DispatcherQueueHandler_Vtbl = DispatcherQueueHandler_Vtbl {
        base__: ::windows::core::IUnknown_Vtbl { QueryInterface: Self::QueryInterface, AddRef: Self::AddRef, Release: Self::Release },
        Invoke: Self::Invoke,
    };
    unsafe extern "system" fn QueryInterface(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *const ::core::ffi::c_void) -> ::windows::core::HRESULT {
        let this = this as *mut *mut ::core::ffi::c_void as *mut Self;
        *interface = if iid == &<DispatcherQueueHandler as ::windows::core::Interface>::IID || iid == &<::windows::core::IUnknown as ::windows::core::Interface>::IID || iid == &<::windows::core::IAgileObject as ::windows::core::Interface>::IID { &mut (*this).vtable as *mut _ as _ } else { ::core::ptr::null_mut() };
        if (*interface).is_null() {
            ::windows::core::HRESULT(-2147467262)
        } else {
            (*this).count.add_ref();
            ::windows::core::HRESULT(0)
        }
    }
    unsafe extern "system" fn AddRef(this: *mut ::core::ffi::c_void) -> u32 {
        let this = this as *mut *mut ::core::ffi::c_void as *mut Self;
        (*this).count.add_ref()
    }
    unsafe extern "system" fn Release(this: *mut ::core::ffi::c_void) -> u32 {
        let this = this as *mut *mut ::core::ffi::c_void as *mut Self;
        let remaining = (*this).count.release();
        if remaining == 0 {
            let _ = ::windows::core::alloc::boxed::Box::from_raw(this);
        }
        remaining
    }
    unsafe extern "system" fn Invoke(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
        let this = this as *mut *mut ::core::ffi::c_void as *mut Self;
        ((*this).invoke)().into()
    }
}
impl ::core::clone::Clone for DispatcherQueueHandler {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for DispatcherQueueHandler {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for DispatcherQueueHandler {}
impl ::core::fmt::Debug for DispatcherQueueHandler {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DispatcherQueueHandler").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Vtable for DispatcherQueueHandler {
    type Vtable = DispatcherQueueHandler_Vtbl;
}
unsafe impl ::windows::core::Interface for DispatcherQueueHandler {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xdfa2dc9c_1a2d_4917_98f2_939af1d6e0c8);
}
unsafe impl ::windows::core::RuntimeType for DispatcherQueueHandler {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{dfa2dc9c-1a2d-4917-98f2-939af1d6e0c8}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct DispatcherQueueHandler_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub Invoke: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
