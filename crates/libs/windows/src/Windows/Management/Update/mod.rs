windows_core::imp::define_winrt_interface!(IPreviewBuildsManager, IPreviewBuildsManager_Vtbl, 0xfa07dd61_7e4f_59f7_7c9f_def9051c5f62, b"Windows.Management.Update.IPreviewBuildsManager");
#[repr(C)]
#[doc(hidden)]
pub struct IPreviewBuildsManager_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub ArePreviewBuildsAllowed: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SetArePreviewBuildsAllowed: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    pub GetCurrentState: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SyncAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_winrt_interface!(IPreviewBuildsManagerStatics, IPreviewBuildsManagerStatics_Vtbl, 0x3e422887_b112_5a70_7da1_97d78d32aa29, b"Windows.Management.Update.IPreviewBuildsManagerStatics");
#[repr(C)]
#[doc(hidden)]
pub struct IPreviewBuildsManagerStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub GetDefault: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub IsSupported: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
}
windows_core::imp::define_winrt_interface!(IPreviewBuildsState, IPreviewBuildsState_Vtbl, 0xa2f2903e_b223_5f63_7546_3e8eac070a2e, b"Windows.Management.Update.IPreviewBuildsState");
#[repr(C)]
#[doc(hidden)]
pub struct IPreviewBuildsState_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub Properties: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Properties: usize,
}
windows_core::imp::define_winrt_interface!(IWindowsUpdate, IWindowsUpdate_Vtbl, 0xc3c88dd7_0ef3_52b2_a9ad_66bfc6bd9582, b"Windows.Management.Update.IWindowsUpdate");
#[repr(C)]
#[doc(hidden)]
pub struct IWindowsUpdate_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub ProviderId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub UpdateId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Title: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Description: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub IsFeatureUpdate: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub IsMinorImpact: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub IsSecurity: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub IsCritical: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub IsForOS: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub IsDriver: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub IsMandatory: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub IsUrgent: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub IsSeeker: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub MoreInfoUrl: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SupportUrl: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub IsEulaAccepted: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub EulaText: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Deadline: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub AttentionRequiredInfo: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ActionResult: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CurrentAction: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ActionProgress: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetPropertyValue: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub AcceptEula: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_winrt_interface!(IWindowsUpdateActionCompletedEventArgs, IWindowsUpdateActionCompletedEventArgs_Vtbl, 0x2c44b950_a655_5321_aec1_aee762922131, b"Windows.Management.Update.IWindowsUpdateActionCompletedEventArgs");
#[repr(C)]
#[doc(hidden)]
pub struct IWindowsUpdateActionCompletedEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Update: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Action: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Succeeded: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub ExtendedError: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::HRESULT) -> windows_core::HRESULT,
}
windows_core::imp::define_winrt_interface!(IWindowsUpdateActionProgress, IWindowsUpdateActionProgress_Vtbl, 0x83b22d8a_4bb0_549f_ba39_59724882d137, b"Windows.Management.Update.IWindowsUpdateActionProgress");
#[repr(C)]
#[doc(hidden)]
pub struct IWindowsUpdateActionProgress_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Action: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Progress: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
}
windows_core::imp::define_winrt_interface!(IWindowsUpdateActionResult, IWindowsUpdateActionResult_Vtbl, 0xe6692c62_f697_51b7_ab7f_e73e5e688f12, b"Windows.Management.Update.IWindowsUpdateActionResult");
#[repr(C)]
#[doc(hidden)]
pub struct IWindowsUpdateActionResult_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Timestamp: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_time::DateTime) -> windows_core::HRESULT,
    pub Succeeded: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub ExtendedError: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::HRESULT) -> windows_core::HRESULT,
    pub Action: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_winrt_interface!(IWindowsUpdateAdministrator, IWindowsUpdateAdministrator_Vtbl, 0x7a60181c_ba1e_5cf9_aa65_304120b73d72, b"Windows.Management.Update.IWindowsUpdateAdministrator");
#[repr(C)]
#[doc(hidden)]
pub struct IWindowsUpdateAdministrator_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub StartAdministratorScan: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ApproveWindowsUpdateAction: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub RevokeWindowsUpdateActionApproval: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ApproveWindowsUpdate: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub RevokeWindowsUpdateApproval: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetUpdates: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_winrt_interface!(IWindowsUpdateAdministratorStatics, IWindowsUpdateAdministratorStatics_Vtbl, 0x013e6d36_ef69_53bc_8db8_c403bca550ed, b"Windows.Management.Update.IWindowsUpdateAdministratorStatics");
#[repr(C)]
#[doc(hidden)]
pub struct IWindowsUpdateAdministratorStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub GetRegisteredAdministrator: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub RegisterForAdministration: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, WindowsUpdateAdministratorOptions, *mut WindowsUpdateAdministratorStatus) -> windows_core::HRESULT,
    pub UnregisterForAdministration: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut WindowsUpdateAdministratorStatus) -> windows_core::HRESULT,
    pub GetRegisteredAdministratorName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub RequestRestart: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CancelRestartRequest: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_winrt_interface!(IWindowsUpdateApprovalData, IWindowsUpdateApprovalData_Vtbl, 0xaadf5bfd_84db_59bc_85e2_ad4fc1f62f7c, b"Windows.Management.Update.IWindowsUpdateApprovalData");
#[repr(C)]
#[doc(hidden)]
pub struct IWindowsUpdateApprovalData_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Seeker: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetSeeker: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub AllowDownloadOnMetered: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetAllowDownloadOnMetered: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ComplianceDeadlineInDays: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetComplianceDeadlineInDays: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ComplianceGracePeriodInDays: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetComplianceGracePeriodInDays: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub OptOutOfAutoReboot: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetOptOutOfAutoReboot: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_winrt_interface!(IWindowsUpdateAttentionRequiredInfo, IWindowsUpdateAttentionRequiredInfo_Vtbl, 0x44df2579_74d3_5ffa_b6ce_09e187e1e0ed, b"Windows.Management.Update.IWindowsUpdateAttentionRequiredInfo");
#[repr(C)]
#[doc(hidden)]
pub struct IWindowsUpdateAttentionRequiredInfo_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Reason: unsafe extern "system" fn(*mut core::ffi::c_void, *mut WindowsUpdateAttentionRequiredReason) -> windows_core::HRESULT,
    pub Timestamp: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_winrt_interface!(IWindowsUpdateAttentionRequiredReasonChangedEventArgs, IWindowsUpdateAttentionRequiredReasonChangedEventArgs_Vtbl, 0x0627abca_dbb8_524a_b1d2_d9df004eeb31, b"Windows.Management.Update.IWindowsUpdateAttentionRequiredReasonChangedEventArgs");
#[repr(C)]
#[doc(hidden)]
pub struct IWindowsUpdateAttentionRequiredReasonChangedEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Update: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Reason: unsafe extern "system" fn(*mut core::ffi::c_void, *mut WindowsUpdateAttentionRequiredReason) -> windows_core::HRESULT,
}
windows_core::imp::define_winrt_interface!(IWindowsUpdateGetAdministratorResult, IWindowsUpdateGetAdministratorResult_Vtbl, 0xbb39ffc4_2c42_5b1c_8995_343341c92c50, b"Windows.Management.Update.IWindowsUpdateGetAdministratorResult");
#[repr(C)]
#[doc(hidden)]
pub struct IWindowsUpdateGetAdministratorResult_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Administrator: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Status: unsafe extern "system" fn(*mut core::ffi::c_void, *mut WindowsUpdateAdministratorStatus) -> windows_core::HRESULT,
}
windows_core::imp::define_winrt_interface!(IWindowsUpdateItem, IWindowsUpdateItem_Vtbl, 0xb222e44a_49b6_59bf_a033_ef617cd73a98, b"Windows.Management.Update.IWindowsUpdateItem");
#[repr(C)]
#[doc(hidden)]
pub struct IWindowsUpdateItem_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub ProviderId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub UpdateId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Timestamp: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_time::DateTime) -> windows_core::HRESULT,
    pub Title: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Description: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub MoreInfoUrl: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Category: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Operation: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_winrt_interface!(IWindowsUpdateManager, IWindowsUpdateManager_Vtbl, 0x5dd966c0_a71a_5602_bbd0_09a70e4573fa, b"Windows.Management.Update.IWindowsUpdateManager");
#[repr(C)]
#[doc(hidden)]
pub struct IWindowsUpdateManager_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub ScanningStateChanged: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut i64) -> windows_core::HRESULT,
    pub RemoveScanningStateChanged: unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
    pub WorkingStateChanged: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut i64) -> windows_core::HRESULT,
    pub RemoveWorkingStateChanged: unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
    pub ProgressChanged: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut i64) -> windows_core::HRESULT,
    pub RemoveProgressChanged: unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
    pub AttentionRequiredReasonChanged: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut i64) -> windows_core::HRESULT,
    pub RemoveAttentionRequiredReasonChanged: unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
    pub ActionCompleted: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut i64) -> windows_core::HRESULT,
    pub RemoveActionCompleted: unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
    pub ScanCompleted: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut i64) -> windows_core::HRESULT,
    pub RemoveScanCompleted: unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
    pub IsScanning: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub IsWorking: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub LastSuccessfulScanTimestamp: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetApplicableUpdates: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetMostRecentCompletedUpdates: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetMostRecentCompletedUpdatesAsync: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub StartScan: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
}
windows_core::imp::define_winrt_interface!(IWindowsUpdateManagerFactory, IWindowsUpdateManagerFactory_Vtbl, 0x1b394df8_decb_5f44_b47c_6ccf3bcfdb37, b"Windows.Management.Update.IWindowsUpdateManagerFactory");
#[repr(C)]
#[doc(hidden)]
pub struct IWindowsUpdateManagerFactory_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub CreateInstance: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_winrt_interface!(IWindowsUpdateProgressChangedEventArgs, IWindowsUpdateProgressChangedEventArgs_Vtbl, 0xbbfbdeeb_94c8_5aa7_b0fb_66c67c233b0a, b"Windows.Management.Update.IWindowsUpdateProgressChangedEventArgs");
#[repr(C)]
#[doc(hidden)]
pub struct IWindowsUpdateProgressChangedEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Update: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ActionProgress: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_winrt_interface!(IWindowsUpdateRestartRequestOptions, IWindowsUpdateRestartRequestOptions_Vtbl, 0x38cfb7d3_4188_5222_905c_6c4443c951ee, b"Windows.Management.Update.IWindowsUpdateRestartRequestOptions");
#[repr(C)]
#[doc(hidden)]
pub struct IWindowsUpdateRestartRequestOptions_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Title: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetTitle: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Description: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetDescription: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub MoreInfoUrl: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetMoreInfoUrl: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ComplianceDeadlineInDays: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetComplianceDeadlineInDays: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub ComplianceGracePeriodInDays: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetComplianceGracePeriodInDays: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub OrganizationName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetOrganizationName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub OptOutOfAutoReboot: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SetOptOutOfAutoReboot: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
}
windows_core::imp::define_winrt_interface!(IWindowsUpdateRestartRequestOptionsFactory, IWindowsUpdateRestartRequestOptionsFactory_Vtbl, 0x75f41d04_0e17_50d0_8c15_6b9d0539b3a9, b"Windows.Management.Update.IWindowsUpdateRestartRequestOptionsFactory");
#[repr(C)]
#[doc(hidden)]
pub struct IWindowsUpdateRestartRequestOptionsFactory_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub CreateInstance: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, i32, i32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_winrt_interface!(IWindowsUpdateScanCompletedEventArgs, IWindowsUpdateScanCompletedEventArgs_Vtbl, 0x95b6953e_ba5c_5fe8_b115_12de184a6bb0, b"Windows.Management.Update.IWindowsUpdateScanCompletedEventArgs");
#[repr(C)]
#[doc(hidden)]
pub struct IWindowsUpdateScanCompletedEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub ProviderId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Succeeded: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub ExtendedError: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::HRESULT) -> windows_core::HRESULT,
    pub Updates: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PreviewBuildsManager(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(PreviewBuildsManager, windows_core::IUnknown, windows_core::IInspectable);
impl PreviewBuildsManager {
    pub fn ArePreviewBuildsAllowed(&self) -> windows_core::Result<bool> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ArePreviewBuildsAllowed)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub fn SetArePreviewBuildsAllowed(&self, value: bool) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetArePreviewBuildsAllowed)(windows_core::Interface::as_raw(self), value).ok() }
    }
    pub fn GetCurrentState(&self) -> windows_core::Result<PreviewBuildsState> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetCurrentState)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::imp::from_abi(result__))
        }
    }
    pub fn SyncAsync(&self) -> windows_core::Result<windows_future::IAsyncOperation<bool>> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).SyncAsync)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::imp::from_abi(result__))
        }
    }
    pub fn GetDefault() -> windows_core::Result<Self> {
        Self::IPreviewBuildsManagerStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetDefault)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::imp::from_abi(result__))
        })
    }
    pub fn IsSupported() -> windows_core::Result<bool> {
        Self::IPreviewBuildsManagerStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsSupported)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    fn IPreviewBuildsManagerStatics<R, F: FnOnce(&IPreviewBuildsManagerStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<PreviewBuildsManager, IPreviewBuildsManagerStatics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for PreviewBuildsManager {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IPreviewBuildsManager>();
}
unsafe impl windows_core::Interface for PreviewBuildsManager {
    type Vtable = <IPreviewBuildsManager as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IPreviewBuildsManager as windows_core::Interface>::IID;
    const INSPECTABLE: bool = true;
}
impl windows_core::RuntimeName for PreviewBuildsManager {
    const NAME: &'static str = "Windows.Management.Update.PreviewBuildsManager";
}
unsafe impl Send for PreviewBuildsManager {}
unsafe impl Sync for PreviewBuildsManager {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PreviewBuildsState(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(PreviewBuildsState, windows_core::IUnknown, windows_core::IInspectable);
impl PreviewBuildsState {
    #[cfg(feature = "Foundation_Collections")]
    pub fn Properties(&self) -> windows_core::Result<super::super::Foundation::Collections::ValueSet> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Properties)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::imp::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for PreviewBuildsState {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IPreviewBuildsState>();
}
unsafe impl windows_core::Interface for PreviewBuildsState {
    type Vtable = <IPreviewBuildsState as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IPreviewBuildsState as windows_core::Interface>::IID;
    const INSPECTABLE: bool = true;
}
impl windows_core::RuntimeName for PreviewBuildsState {
    const NAME: &'static str = "Windows.Management.Update.PreviewBuildsState";
}
unsafe impl Send for PreviewBuildsState {}
unsafe impl Sync for PreviewBuildsState {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WindowsUpdate(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(WindowsUpdate, windows_core::IUnknown, windows_core::IInspectable);
impl WindowsUpdate {
    pub fn ProviderId(&self) -> windows_core::Result<windows_core::HSTRING> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ProviderId)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn UpdateId(&self) -> windows_core::Result<windows_core::HSTRING> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).UpdateId)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn Title(&self) -> windows_core::Result<windows_core::HSTRING> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Title)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn Description(&self) -> windows_core::Result<windows_core::HSTRING> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Description)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn IsFeatureUpdate(&self) -> windows_core::Result<bool> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).IsFeatureUpdate)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub fn IsMinorImpact(&self) -> windows_core::Result<bool> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).IsMinorImpact)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub fn IsSecurity(&self) -> windows_core::Result<bool> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).IsSecurity)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub fn IsCritical(&self) -> windows_core::Result<bool> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).IsCritical)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub fn IsForOS(&self) -> windows_core::Result<bool> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).IsForOS)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub fn IsDriver(&self) -> windows_core::Result<bool> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).IsDriver)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub fn IsMandatory(&self) -> windows_core::Result<bool> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).IsMandatory)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub fn IsUrgent(&self) -> windows_core::Result<bool> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).IsUrgent)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub fn IsSeeker(&self) -> windows_core::Result<bool> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).IsSeeker)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub fn MoreInfoUrl(&self) -> windows_core::Result<super::super::Foundation::Uri> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).MoreInfoUrl)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::imp::from_abi(result__))
        }
    }
    pub fn SupportUrl(&self) -> windows_core::Result<super::super::Foundation::Uri> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).SupportUrl)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::imp::from_abi(result__))
        }
    }
    pub fn IsEulaAccepted(&self) -> windows_core::Result<bool> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).IsEulaAccepted)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub fn EulaText(&self) -> windows_core::Result<windows_core::HSTRING> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).EulaText)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn Deadline(&self) -> windows_core::Result<windows_reference::IReference<windows_time::DateTime>> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Deadline)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::imp::from_abi(result__))
        }
    }
    pub fn AttentionRequiredInfo(&self) -> windows_core::Result<WindowsUpdateAttentionRequiredInfo> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).AttentionRequiredInfo)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::imp::from_abi(result__))
        }
    }
    pub fn ActionResult(&self) -> windows_core::Result<WindowsUpdateActionResult> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ActionResult)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::imp::from_abi(result__))
        }
    }
    pub fn CurrentAction(&self) -> windows_core::Result<windows_core::HSTRING> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CurrentAction)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn ActionProgress(&self) -> windows_core::Result<WindowsUpdateActionProgress> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ActionProgress)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::imp::from_abi(result__))
        }
    }
    pub fn GetPropertyValue(&self, propertyname: &windows_core::HSTRING) -> windows_core::Result<windows_core::IInspectable> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetPropertyValue)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(propertyname), &mut result__).and_then(|| windows_core::imp::from_abi(result__))
        }
    }
    pub fn AcceptEula(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).AcceptEula)(windows_core::Interface::as_raw(self)).ok() }
    }
}
impl windows_core::RuntimeType for WindowsUpdate {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IWindowsUpdate>();
}
unsafe impl windows_core::Interface for WindowsUpdate {
    type Vtable = <IWindowsUpdate as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IWindowsUpdate as windows_core::Interface>::IID;
    const INSPECTABLE: bool = true;
}
impl windows_core::RuntimeName for WindowsUpdate {
    const NAME: &'static str = "Windows.Management.Update.WindowsUpdate";
}
unsafe impl Send for WindowsUpdate {}
unsafe impl Sync for WindowsUpdate {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WindowsUpdateActionCompletedEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(WindowsUpdateActionCompletedEventArgs, windows_core::IUnknown, windows_core::IInspectable);
impl WindowsUpdateActionCompletedEventArgs {
    pub fn Update(&self) -> windows_core::Result<WindowsUpdate> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Update)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::imp::from_abi(result__))
        }
    }
    pub fn Action(&self) -> windows_core::Result<windows_core::HSTRING> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Action)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn Succeeded(&self) -> windows_core::Result<bool> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Succeeded)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub fn ExtendedError(&self) -> windows_core::Result<windows_core::HRESULT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ExtendedError)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
impl windows_core::RuntimeType for WindowsUpdateActionCompletedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IWindowsUpdateActionCompletedEventArgs>();
}
unsafe impl windows_core::Interface for WindowsUpdateActionCompletedEventArgs {
    type Vtable = <IWindowsUpdateActionCompletedEventArgs as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IWindowsUpdateActionCompletedEventArgs as windows_core::Interface>::IID;
    const INSPECTABLE: bool = true;
}
impl windows_core::RuntimeName for WindowsUpdateActionCompletedEventArgs {
    const NAME: &'static str = "Windows.Management.Update.WindowsUpdateActionCompletedEventArgs";
}
unsafe impl Send for WindowsUpdateActionCompletedEventArgs {}
unsafe impl Sync for WindowsUpdateActionCompletedEventArgs {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WindowsUpdateActionProgress(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(WindowsUpdateActionProgress, windows_core::IUnknown, windows_core::IInspectable);
impl WindowsUpdateActionProgress {
    pub fn Action(&self) -> windows_core::Result<windows_core::HSTRING> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Action)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn Progress(&self) -> windows_core::Result<f64> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Progress)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
impl windows_core::RuntimeType for WindowsUpdateActionProgress {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IWindowsUpdateActionProgress>();
}
unsafe impl windows_core::Interface for WindowsUpdateActionProgress {
    type Vtable = <IWindowsUpdateActionProgress as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IWindowsUpdateActionProgress as windows_core::Interface>::IID;
    const INSPECTABLE: bool = true;
}
impl windows_core::RuntimeName for WindowsUpdateActionProgress {
    const NAME: &'static str = "Windows.Management.Update.WindowsUpdateActionProgress";
}
unsafe impl Send for WindowsUpdateActionProgress {}
unsafe impl Sync for WindowsUpdateActionProgress {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WindowsUpdateActionResult(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(WindowsUpdateActionResult, windows_core::IUnknown, windows_core::IInspectable);
impl WindowsUpdateActionResult {
    pub fn Timestamp(&self) -> windows_core::Result<windows_time::DateTime> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Timestamp)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub fn Succeeded(&self) -> windows_core::Result<bool> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Succeeded)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub fn ExtendedError(&self) -> windows_core::Result<windows_core::HRESULT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ExtendedError)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub fn Action(&self) -> windows_core::Result<windows_core::HSTRING> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Action)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
}
impl windows_core::RuntimeType for WindowsUpdateActionResult {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IWindowsUpdateActionResult>();
}
unsafe impl windows_core::Interface for WindowsUpdateActionResult {
    type Vtable = <IWindowsUpdateActionResult as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IWindowsUpdateActionResult as windows_core::Interface>::IID;
    const INSPECTABLE: bool = true;
}
impl windows_core::RuntimeName for WindowsUpdateActionResult {
    const NAME: &'static str = "Windows.Management.Update.WindowsUpdateActionResult";
}
unsafe impl Send for WindowsUpdateActionResult {}
unsafe impl Sync for WindowsUpdateActionResult {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WindowsUpdateAdministrator(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(WindowsUpdateAdministrator, windows_core::IUnknown, windows_core::IInspectable);
impl WindowsUpdateAdministrator {
    pub fn StartAdministratorScan(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).StartAdministratorScan)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub fn ApproveWindowsUpdateAction(&self, updateid: &windows_core::HSTRING, action: &windows_core::HSTRING) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).ApproveWindowsUpdateAction)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(updateid), core::mem::transmute_copy(action)).ok() }
    }
    pub fn RevokeWindowsUpdateActionApproval(&self, updateid: &windows_core::HSTRING, action: &windows_core::HSTRING) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).RevokeWindowsUpdateActionApproval)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(updateid), core::mem::transmute_copy(action)).ok() }
    }
    pub fn ApproveWindowsUpdate<P1>(&self, updateid: &windows_core::HSTRING, approvaldata: P1) -> windows_core::Result<()>
    where
        P1: windows_core::Param<WindowsUpdateApprovalData>,
    {
        unsafe { (windows_core::Interface::vtable(self).ApproveWindowsUpdate)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(updateid), approvaldata.param().abi()).ok() }
    }
    pub fn RevokeWindowsUpdateApproval(&self, updateid: &windows_core::HSTRING) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).RevokeWindowsUpdateApproval)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(updateid)).ok() }
    }
    pub fn GetUpdates(&self) -> windows_core::Result<windows_collections::IVectorView<WindowsUpdate>> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetUpdates)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::imp::from_abi(result__))
        }
    }
    pub fn GetRegisteredAdministrator(organizationname: &windows_core::HSTRING) -> windows_core::Result<WindowsUpdateGetAdministratorResult> {
        Self::IWindowsUpdateAdministratorStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetRegisteredAdministrator)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(organizationname), &mut result__).and_then(|| windows_core::imp::from_abi(result__))
        })
    }
    pub fn RegisterForAdministration(organizationname: &windows_core::HSTRING, options: WindowsUpdateAdministratorOptions) -> windows_core::Result<WindowsUpdateAdministratorStatus> {
        Self::IWindowsUpdateAdministratorStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RegisterForAdministration)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(organizationname), options, &mut result__).map(|| result__)
        })
    }
    pub fn UnregisterForAdministration(organizationname: &windows_core::HSTRING) -> windows_core::Result<WindowsUpdateAdministratorStatus> {
        Self::IWindowsUpdateAdministratorStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).UnregisterForAdministration)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(organizationname), &mut result__).map(|| result__)
        })
    }
    pub fn GetRegisteredAdministratorName() -> windows_core::Result<windows_core::HSTRING> {
        Self::IWindowsUpdateAdministratorStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetRegisteredAdministratorName)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        })
    }
    pub fn RequestRestart<P0>(restartoptions: P0) -> windows_core::Result<windows_core::HSTRING>
    where
        P0: windows_core::Param<WindowsUpdateRestartRequestOptions>,
    {
        Self::IWindowsUpdateAdministratorStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RequestRestart)(windows_core::Interface::as_raw(this), restartoptions.param().abi(), &mut result__).map(|| core::mem::transmute(result__))
        })
    }
    pub fn CancelRestartRequest(requestrestarttoken: &windows_core::HSTRING) -> windows_core::Result<()> {
        Self::IWindowsUpdateAdministratorStatics(|this| unsafe { (windows_core::Interface::vtable(this).CancelRestartRequest)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(requestrestarttoken)).ok() })
    }
    fn IWindowsUpdateAdministratorStatics<R, F: FnOnce(&IWindowsUpdateAdministratorStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<WindowsUpdateAdministrator, IWindowsUpdateAdministratorStatics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for WindowsUpdateAdministrator {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IWindowsUpdateAdministrator>();
}
unsafe impl windows_core::Interface for WindowsUpdateAdministrator {
    type Vtable = <IWindowsUpdateAdministrator as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IWindowsUpdateAdministrator as windows_core::Interface>::IID;
    const INSPECTABLE: bool = true;
}
impl windows_core::RuntimeName for WindowsUpdateAdministrator {
    const NAME: &'static str = "Windows.Management.Update.WindowsUpdateAdministrator";
}
unsafe impl Send for WindowsUpdateAdministrator {}
unsafe impl Sync for WindowsUpdateAdministrator {}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WindowsUpdateAdministratorOptions(pub u32);
impl WindowsUpdateAdministratorOptions {
    pub const None: Self = Self(0);
    pub const RequireAdministratorApprovalForScans: Self = Self(1);
    pub const RequireAdministratorApprovalForUpdates: Self = Self(2);
    pub const RequireAdministratorApprovalForActions: Self = Self(4);
}
impl windows_core::imp::TypeKind for WindowsUpdateAdministratorOptions {
    type TypeKind = windows_core::imp::CopyType;
}
impl windows_core::RuntimeType for WindowsUpdateAdministratorOptions {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Management.Update.WindowsUpdateAdministratorOptions;u4)");
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"Windows.Management.Update.WindowsUpdateAdministratorOptions");
}
impl WindowsUpdateAdministratorOptions {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for WindowsUpdateAdministratorOptions {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for WindowsUpdateAdministratorOptions {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for WindowsUpdateAdministratorOptions {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0);
    }
}
impl core::ops::BitAndAssign for WindowsUpdateAdministratorOptions {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0);
    }
}
impl core::ops::Not for WindowsUpdateAdministratorOptions {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WindowsUpdateAdministratorStatus(pub i32);
impl WindowsUpdateAdministratorStatus {
    pub const Succeeded: Self = Self(0);
    pub const NoAdministratorRegistered: Self = Self(1);
    pub const OtherAdministratorIsRegistered: Self = Self(2);
}
impl windows_core::imp::TypeKind for WindowsUpdateAdministratorStatus {
    type TypeKind = windows_core::imp::CopyType;
}
impl windows_core::RuntimeType for WindowsUpdateAdministratorStatus {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Management.Update.WindowsUpdateAdministratorStatus;i4)");
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"Windows.Management.Update.WindowsUpdateAdministratorStatus");
}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WindowsUpdateApprovalData(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(WindowsUpdateApprovalData, windows_core::IUnknown, windows_core::IInspectable);
impl WindowsUpdateApprovalData {
    pub fn new() -> windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&windows_core::imp::IGenericFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<WindowsUpdateApprovalData, windows_core::imp::IGenericFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn Seeker(&self) -> windows_core::Result<windows_reference::IReference<bool>> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Seeker)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::imp::from_abi(result__))
        }
    }
    pub fn SetSeeker<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_reference::IReference<bool>>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetSeeker)(windows_core::Interface::as_raw(self), value.param().abi()).ok() }
    }
    pub fn AllowDownloadOnMetered(&self) -> windows_core::Result<windows_reference::IReference<bool>> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).AllowDownloadOnMetered)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::imp::from_abi(result__))
        }
    }
    pub fn SetAllowDownloadOnMetered<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_reference::IReference<bool>>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetAllowDownloadOnMetered)(windows_core::Interface::as_raw(self), value.param().abi()).ok() }
    }
    pub fn ComplianceDeadlineInDays(&self) -> windows_core::Result<windows_reference::IReference<i32>> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ComplianceDeadlineInDays)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::imp::from_abi(result__))
        }
    }
    pub fn SetComplianceDeadlineInDays<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_reference::IReference<i32>>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetComplianceDeadlineInDays)(windows_core::Interface::as_raw(self), value.param().abi()).ok() }
    }
    pub fn ComplianceGracePeriodInDays(&self) -> windows_core::Result<windows_reference::IReference<i32>> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ComplianceGracePeriodInDays)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::imp::from_abi(result__))
        }
    }
    pub fn SetComplianceGracePeriodInDays<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_reference::IReference<i32>>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetComplianceGracePeriodInDays)(windows_core::Interface::as_raw(self), value.param().abi()).ok() }
    }
    pub fn OptOutOfAutoReboot(&self) -> windows_core::Result<windows_reference::IReference<bool>> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).OptOutOfAutoReboot)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::imp::from_abi(result__))
        }
    }
    pub fn SetOptOutOfAutoReboot<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_reference::IReference<bool>>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetOptOutOfAutoReboot)(windows_core::Interface::as_raw(self), value.param().abi()).ok() }
    }
}
impl windows_core::RuntimeType for WindowsUpdateApprovalData {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IWindowsUpdateApprovalData>();
}
unsafe impl windows_core::Interface for WindowsUpdateApprovalData {
    type Vtable = <IWindowsUpdateApprovalData as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IWindowsUpdateApprovalData as windows_core::Interface>::IID;
    const INSPECTABLE: bool = true;
}
impl windows_core::RuntimeName for WindowsUpdateApprovalData {
    const NAME: &'static str = "Windows.Management.Update.WindowsUpdateApprovalData";
}
unsafe impl Send for WindowsUpdateApprovalData {}
unsafe impl Sync for WindowsUpdateApprovalData {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WindowsUpdateAttentionRequiredInfo(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(WindowsUpdateAttentionRequiredInfo, windows_core::IUnknown, windows_core::IInspectable);
impl WindowsUpdateAttentionRequiredInfo {
    pub fn Reason(&self) -> windows_core::Result<WindowsUpdateAttentionRequiredReason> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Reason)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub fn Timestamp(&self) -> windows_core::Result<windows_reference::IReference<windows_time::DateTime>> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Timestamp)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::imp::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for WindowsUpdateAttentionRequiredInfo {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IWindowsUpdateAttentionRequiredInfo>();
}
unsafe impl windows_core::Interface for WindowsUpdateAttentionRequiredInfo {
    type Vtable = <IWindowsUpdateAttentionRequiredInfo as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IWindowsUpdateAttentionRequiredInfo as windows_core::Interface>::IID;
    const INSPECTABLE: bool = true;
}
impl windows_core::RuntimeName for WindowsUpdateAttentionRequiredInfo {
    const NAME: &'static str = "Windows.Management.Update.WindowsUpdateAttentionRequiredInfo";
}
unsafe impl Send for WindowsUpdateAttentionRequiredInfo {}
unsafe impl Sync for WindowsUpdateAttentionRequiredInfo {}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WindowsUpdateAttentionRequiredReason(pub i32);
impl WindowsUpdateAttentionRequiredReason {
    pub const None: Self = Self(0);
    pub const SeekerUpdate: Self = Self(1);
    pub const ReadyToReboot: Self = Self(2);
    pub const NeedNonMeteredNetwork: Self = Self(3);
    pub const NeedUserAgreementForMeteredNetwork: Self = Self(4);
    pub const NeedNetwork: Self = Self(5);
    pub const NeedMoreSpace: Self = Self(6);
    pub const BatterySaverEnabled: Self = Self(7);
    pub const NeedUserInteraction: Self = Self(8);
    pub const NeedUserAgreementForPolicy: Self = Self(9);
    pub const CompatibilityError: Self = Self(10);
    pub const NeedUserInteractionForEula: Self = Self(11);
    pub const NeedUserInteractionForCta: Self = Self(12);
    pub const Regulated: Self = Self(13);
    pub const ExternalReboot: Self = Self(14);
    pub const OtherUpdate: Self = Self(15);
    pub const BlockedByProvider: Self = Self(16);
    pub const BlockedByPostRebootFailure: Self = Self(17);
    pub const UserEngaged: Self = Self(18);
    pub const BlockedByBattery: Self = Self(19);
    pub const Exclusivity: Self = Self(20);
    pub const BlockedBySerialization: Self = Self(21);
    pub const ConflictClass: Self = Self(22);
    pub const BlockedByAdminApproval: Self = Self(23);
    pub const BlockedByTooManyAttempts: Self = Self(24);
    pub const BlockedByFailure: Self = Self(25);
    pub const Demotion: Self = Self(26);
    pub const BlockedByActiveHours: Self = Self(27);
    pub const ScheduledForMaintenance: Self = Self(28);
    pub const PolicyScheduledInstallTime: Self = Self(29);
    pub const BlockedByOobe: Self = Self(30);
    pub const DeferredDuringOobe: Self = Self(31);
    pub const DeferredForSustainableTime: Self = Self(32);
}
impl windows_core::imp::TypeKind for WindowsUpdateAttentionRequiredReason {
    type TypeKind = windows_core::imp::CopyType;
}
impl windows_core::RuntimeType for WindowsUpdateAttentionRequiredReason {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Management.Update.WindowsUpdateAttentionRequiredReason;i4)");
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"Windows.Management.Update.WindowsUpdateAttentionRequiredReason");
}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WindowsUpdateAttentionRequiredReasonChangedEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(WindowsUpdateAttentionRequiredReasonChangedEventArgs, windows_core::IUnknown, windows_core::IInspectable);
impl WindowsUpdateAttentionRequiredReasonChangedEventArgs {
    pub fn Update(&self) -> windows_core::Result<WindowsUpdate> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Update)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::imp::from_abi(result__))
        }
    }
    pub fn Reason(&self) -> windows_core::Result<WindowsUpdateAttentionRequiredReason> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Reason)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
impl windows_core::RuntimeType for WindowsUpdateAttentionRequiredReasonChangedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IWindowsUpdateAttentionRequiredReasonChangedEventArgs>();
}
unsafe impl windows_core::Interface for WindowsUpdateAttentionRequiredReasonChangedEventArgs {
    type Vtable = <IWindowsUpdateAttentionRequiredReasonChangedEventArgs as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IWindowsUpdateAttentionRequiredReasonChangedEventArgs as windows_core::Interface>::IID;
    const INSPECTABLE: bool = true;
}
impl windows_core::RuntimeName for WindowsUpdateAttentionRequiredReasonChangedEventArgs {
    const NAME: &'static str = "Windows.Management.Update.WindowsUpdateAttentionRequiredReasonChangedEventArgs";
}
unsafe impl Send for WindowsUpdateAttentionRequiredReasonChangedEventArgs {}
unsafe impl Sync for WindowsUpdateAttentionRequiredReasonChangedEventArgs {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WindowsUpdateGetAdministratorResult(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(WindowsUpdateGetAdministratorResult, windows_core::IUnknown, windows_core::IInspectable);
impl WindowsUpdateGetAdministratorResult {
    pub fn Administrator(&self) -> windows_core::Result<WindowsUpdateAdministrator> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Administrator)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::imp::from_abi(result__))
        }
    }
    pub fn Status(&self) -> windows_core::Result<WindowsUpdateAdministratorStatus> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Status)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
impl windows_core::RuntimeType for WindowsUpdateGetAdministratorResult {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IWindowsUpdateGetAdministratorResult>();
}
unsafe impl windows_core::Interface for WindowsUpdateGetAdministratorResult {
    type Vtable = <IWindowsUpdateGetAdministratorResult as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IWindowsUpdateGetAdministratorResult as windows_core::Interface>::IID;
    const INSPECTABLE: bool = true;
}
impl windows_core::RuntimeName for WindowsUpdateGetAdministratorResult {
    const NAME: &'static str = "Windows.Management.Update.WindowsUpdateGetAdministratorResult";
}
unsafe impl Send for WindowsUpdateGetAdministratorResult {}
unsafe impl Sync for WindowsUpdateGetAdministratorResult {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WindowsUpdateItem(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(WindowsUpdateItem, windows_core::IUnknown, windows_core::IInspectable);
impl WindowsUpdateItem {
    pub fn ProviderId(&self) -> windows_core::Result<windows_core::HSTRING> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ProviderId)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn UpdateId(&self) -> windows_core::Result<windows_core::HSTRING> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).UpdateId)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn Timestamp(&self) -> windows_core::Result<windows_time::DateTime> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Timestamp)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub fn Title(&self) -> windows_core::Result<windows_core::HSTRING> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Title)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn Description(&self) -> windows_core::Result<windows_core::HSTRING> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Description)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn MoreInfoUrl(&self) -> windows_core::Result<super::super::Foundation::Uri> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).MoreInfoUrl)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::imp::from_abi(result__))
        }
    }
    pub fn Category(&self) -> windows_core::Result<windows_core::HSTRING> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Category)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn Operation(&self) -> windows_core::Result<windows_core::HSTRING> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Operation)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
}
impl windows_core::RuntimeType for WindowsUpdateItem {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IWindowsUpdateItem>();
}
unsafe impl windows_core::Interface for WindowsUpdateItem {
    type Vtable = <IWindowsUpdateItem as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IWindowsUpdateItem as windows_core::Interface>::IID;
    const INSPECTABLE: bool = true;
}
impl windows_core::RuntimeName for WindowsUpdateItem {
    const NAME: &'static str = "Windows.Management.Update.WindowsUpdateItem";
}
unsafe impl Send for WindowsUpdateItem {}
unsafe impl Sync for WindowsUpdateItem {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WindowsUpdateManager(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(WindowsUpdateManager, windows_core::IUnknown, windows_core::IInspectable);
impl WindowsUpdateManager {
    pub fn ScanningStateChanged<P0>(&self, handler: P0) -> windows_core::Result<i64>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<Self, windows_core::IInspectable>>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ScanningStateChanged)(windows_core::Interface::as_raw(self), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveScanningStateChanged(&self, token: i64) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).RemoveScanningStateChanged)(windows_core::Interface::as_raw(self), token).ok() }
    }
    pub fn WorkingStateChanged<P0>(&self, handler: P0) -> windows_core::Result<i64>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<Self, windows_core::IInspectable>>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).WorkingStateChanged)(windows_core::Interface::as_raw(self), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveWorkingStateChanged(&self, token: i64) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).RemoveWorkingStateChanged)(windows_core::Interface::as_raw(self), token).ok() }
    }
    pub fn ProgressChanged<P0>(&self, handler: P0) -> windows_core::Result<i64>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<Self, WindowsUpdateProgressChangedEventArgs>>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ProgressChanged)(windows_core::Interface::as_raw(self), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveProgressChanged(&self, token: i64) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).RemoveProgressChanged)(windows_core::Interface::as_raw(self), token).ok() }
    }
    pub fn AttentionRequiredReasonChanged<P0>(&self, handler: P0) -> windows_core::Result<i64>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<Self, WindowsUpdateAttentionRequiredReasonChangedEventArgs>>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).AttentionRequiredReasonChanged)(windows_core::Interface::as_raw(self), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveAttentionRequiredReasonChanged(&self, token: i64) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).RemoveAttentionRequiredReasonChanged)(windows_core::Interface::as_raw(self), token).ok() }
    }
    pub fn ActionCompleted<P0>(&self, handler: P0) -> windows_core::Result<i64>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<Self, WindowsUpdateActionCompletedEventArgs>>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ActionCompleted)(windows_core::Interface::as_raw(self), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveActionCompleted(&self, token: i64) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).RemoveActionCompleted)(windows_core::Interface::as_raw(self), token).ok() }
    }
    pub fn ScanCompleted<P0>(&self, handler: P0) -> windows_core::Result<i64>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<Self, WindowsUpdateScanCompletedEventArgs>>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ScanCompleted)(windows_core::Interface::as_raw(self), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveScanCompleted(&self, token: i64) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).RemoveScanCompleted)(windows_core::Interface::as_raw(self), token).ok() }
    }
    pub fn IsScanning(&self) -> windows_core::Result<bool> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).IsScanning)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub fn IsWorking(&self) -> windows_core::Result<bool> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).IsWorking)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub fn LastSuccessfulScanTimestamp(&self) -> windows_core::Result<windows_reference::IReference<windows_time::DateTime>> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).LastSuccessfulScanTimestamp)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::imp::from_abi(result__))
        }
    }
    pub fn GetApplicableUpdates(&self) -> windows_core::Result<windows_collections::IVectorView<WindowsUpdate>> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetApplicableUpdates)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::imp::from_abi(result__))
        }
    }
    pub fn GetMostRecentCompletedUpdates(&self, count: i32) -> windows_core::Result<windows_collections::IVectorView<WindowsUpdateItem>> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetMostRecentCompletedUpdates)(windows_core::Interface::as_raw(self), count, &mut result__).and_then(|| windows_core::imp::from_abi(result__))
        }
    }
    pub fn GetMostRecentCompletedUpdatesAsync(&self, count: i32) -> windows_core::Result<windows_future::IAsyncOperation<windows_collections::IVectorView<WindowsUpdateItem>>> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetMostRecentCompletedUpdatesAsync)(windows_core::Interface::as_raw(self), count, &mut result__).and_then(|| windows_core::imp::from_abi(result__))
        }
    }
    pub fn StartScan(&self, userinitiated: bool) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).StartScan)(windows_core::Interface::as_raw(self), userinitiated).ok() }
    }
    pub fn CreateInstance(clientid: &windows_core::HSTRING) -> windows_core::Result<Self> {
        Self::IWindowsUpdateManagerFactory(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateInstance)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(clientid), &mut result__).and_then(|| windows_core::imp::from_abi(result__))
        })
    }
    fn IWindowsUpdateManagerFactory<R, F: FnOnce(&IWindowsUpdateManagerFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<WindowsUpdateManager, IWindowsUpdateManagerFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for WindowsUpdateManager {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IWindowsUpdateManager>();
}
unsafe impl windows_core::Interface for WindowsUpdateManager {
    type Vtable = <IWindowsUpdateManager as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IWindowsUpdateManager as windows_core::Interface>::IID;
    const INSPECTABLE: bool = true;
}
impl windows_core::RuntimeName for WindowsUpdateManager {
    const NAME: &'static str = "Windows.Management.Update.WindowsUpdateManager";
}
unsafe impl Send for WindowsUpdateManager {}
unsafe impl Sync for WindowsUpdateManager {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WindowsUpdateProgressChangedEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(WindowsUpdateProgressChangedEventArgs, windows_core::IUnknown, windows_core::IInspectable);
impl WindowsUpdateProgressChangedEventArgs {
    pub fn Update(&self) -> windows_core::Result<WindowsUpdate> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Update)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::imp::from_abi(result__))
        }
    }
    pub fn ActionProgress(&self) -> windows_core::Result<WindowsUpdateActionProgress> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ActionProgress)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::imp::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for WindowsUpdateProgressChangedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IWindowsUpdateProgressChangedEventArgs>();
}
unsafe impl windows_core::Interface for WindowsUpdateProgressChangedEventArgs {
    type Vtable = <IWindowsUpdateProgressChangedEventArgs as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IWindowsUpdateProgressChangedEventArgs as windows_core::Interface>::IID;
    const INSPECTABLE: bool = true;
}
impl windows_core::RuntimeName for WindowsUpdateProgressChangedEventArgs {
    const NAME: &'static str = "Windows.Management.Update.WindowsUpdateProgressChangedEventArgs";
}
unsafe impl Send for WindowsUpdateProgressChangedEventArgs {}
unsafe impl Sync for WindowsUpdateProgressChangedEventArgs {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WindowsUpdateRestartRequestOptions(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(WindowsUpdateRestartRequestOptions, windows_core::IUnknown, windows_core::IInspectable);
impl WindowsUpdateRestartRequestOptions {
    pub fn new() -> windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&windows_core::imp::IGenericFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<WindowsUpdateRestartRequestOptions, windows_core::imp::IGenericFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn Title(&self) -> windows_core::Result<windows_core::HSTRING> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Title)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn SetTitle(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetTitle)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(value)).ok() }
    }
    pub fn Description(&self) -> windows_core::Result<windows_core::HSTRING> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Description)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn SetDescription(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetDescription)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(value)).ok() }
    }
    pub fn MoreInfoUrl(&self) -> windows_core::Result<super::super::Foundation::Uri> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).MoreInfoUrl)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::imp::from_abi(result__))
        }
    }
    pub fn SetMoreInfoUrl<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::Uri>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetMoreInfoUrl)(windows_core::Interface::as_raw(self), value.param().abi()).ok() }
    }
    pub fn ComplianceDeadlineInDays(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ComplianceDeadlineInDays)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub fn SetComplianceDeadlineInDays(&self, value: i32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetComplianceDeadlineInDays)(windows_core::Interface::as_raw(self), value).ok() }
    }
    pub fn ComplianceGracePeriodInDays(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ComplianceGracePeriodInDays)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub fn SetComplianceGracePeriodInDays(&self, value: i32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetComplianceGracePeriodInDays)(windows_core::Interface::as_raw(self), value).ok() }
    }
    pub fn OrganizationName(&self) -> windows_core::Result<windows_core::HSTRING> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).OrganizationName)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn SetOrganizationName(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetOrganizationName)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(value)).ok() }
    }
    pub fn OptOutOfAutoReboot(&self) -> windows_core::Result<bool> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).OptOutOfAutoReboot)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub fn SetOptOutOfAutoReboot(&self, value: bool) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetOptOutOfAutoReboot)(windows_core::Interface::as_raw(self), value).ok() }
    }
    pub fn CreateInstance<P2>(title: &windows_core::HSTRING, description: &windows_core::HSTRING, moreinfourl: P2, compliancedeadlineindays: i32, compliancegraceperiodindays: i32) -> windows_core::Result<Self>
    where
        P2: windows_core::Param<super::super::Foundation::Uri>,
    {
        Self::IWindowsUpdateRestartRequestOptionsFactory(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateInstance)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(title), core::mem::transmute_copy(description), moreinfourl.param().abi(), compliancedeadlineindays, compliancegraceperiodindays, &mut result__).and_then(|| windows_core::imp::from_abi(result__))
        })
    }
    fn IWindowsUpdateRestartRequestOptionsFactory<R, F: FnOnce(&IWindowsUpdateRestartRequestOptionsFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<WindowsUpdateRestartRequestOptions, IWindowsUpdateRestartRequestOptionsFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for WindowsUpdateRestartRequestOptions {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IWindowsUpdateRestartRequestOptions>();
}
unsafe impl windows_core::Interface for WindowsUpdateRestartRequestOptions {
    type Vtable = <IWindowsUpdateRestartRequestOptions as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IWindowsUpdateRestartRequestOptions as windows_core::Interface>::IID;
    const INSPECTABLE: bool = true;
}
impl windows_core::RuntimeName for WindowsUpdateRestartRequestOptions {
    const NAME: &'static str = "Windows.Management.Update.WindowsUpdateRestartRequestOptions";
}
unsafe impl Send for WindowsUpdateRestartRequestOptions {}
unsafe impl Sync for WindowsUpdateRestartRequestOptions {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WindowsUpdateScanCompletedEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(WindowsUpdateScanCompletedEventArgs, windows_core::IUnknown, windows_core::IInspectable);
impl WindowsUpdateScanCompletedEventArgs {
    pub fn ProviderId(&self) -> windows_core::Result<windows_core::HSTRING> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ProviderId)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn Succeeded(&self) -> windows_core::Result<bool> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Succeeded)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub fn ExtendedError(&self) -> windows_core::Result<windows_core::HRESULT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ExtendedError)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub fn Updates(&self) -> windows_core::Result<windows_collections::IVectorView<WindowsUpdate>> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Updates)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::imp::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for WindowsUpdateScanCompletedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IWindowsUpdateScanCompletedEventArgs>();
}
unsafe impl windows_core::Interface for WindowsUpdateScanCompletedEventArgs {
    type Vtable = <IWindowsUpdateScanCompletedEventArgs as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IWindowsUpdateScanCompletedEventArgs as windows_core::Interface>::IID;
    const INSPECTABLE: bool = true;
}
impl windows_core::RuntimeName for WindowsUpdateScanCompletedEventArgs {
    const NAME: &'static str = "Windows.Management.Update.WindowsUpdateScanCompletedEventArgs";
}
unsafe impl Send for WindowsUpdateScanCompletedEventArgs {}
unsafe impl Sync for WindowsUpdateScanCompletedEventArgs {}
