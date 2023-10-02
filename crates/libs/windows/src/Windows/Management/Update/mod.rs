#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IPreviewBuildsManager(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPreviewBuildsManager {
    type Vtable = IPreviewBuildsManager_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IPreviewBuildsManager {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xfa07dd61_7e4f_59f7_7c9f_def9051c5f62);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPreviewBuildsManager_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub ArePreviewBuildsAllowed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub SetArePreviewBuildsAllowed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
    pub GetCurrentState: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub SyncAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SyncAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IPreviewBuildsManagerStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPreviewBuildsManagerStatics {
    type Vtable = IPreviewBuildsManagerStatics_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IPreviewBuildsManagerStatics {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x3e422887_b112_5a70_7da1_97d78d32aa29);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPreviewBuildsManagerStatics_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub GetDefault: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub IsSupported: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IPreviewBuildsState(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPreviewBuildsState {
    type Vtable = IPreviewBuildsState_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IPreviewBuildsState {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xa2f2903e_b223_5f63_7546_3e8eac070a2e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPreviewBuildsState_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub Properties: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Properties: usize,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IWindowsUpdate(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IWindowsUpdate {
    type Vtable = IWindowsUpdate_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IWindowsUpdate {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xc3c88dd7_0ef3_52b2_a9ad_66bfc6bd9582);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWindowsUpdate_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub ProviderId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub UpdateId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub Title: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub Description: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub IsFeatureUpdate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub IsMinorImpact: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub IsSecurity: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub IsCritical: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub IsForOS: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub IsDriver: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub IsMandatory: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub IsUrgent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub IsSeeker: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub MoreInfoUrl: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    MoreInfoUrl: usize,
    #[cfg(feature = "Foundation")]
    pub SupportUrl: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SupportUrl: usize,
    pub IsEulaAccepted: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub EulaText: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub Deadline: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Deadline: usize,
    pub AttentionRequiredInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub ActionResult: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub CurrentAction: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub ActionProgress: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub GetPropertyValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, propertyname: ::std::mem::MaybeUninit<::windows_core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub AcceptEula: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IWindowsUpdateActionCompletedEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IWindowsUpdateActionCompletedEventArgs {
    type Vtable = IWindowsUpdateActionCompletedEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IWindowsUpdateActionCompletedEventArgs {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x2c44b950_a655_5321_aec1_aee762922131);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWindowsUpdateActionCompletedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Update: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub Action: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub Succeeded: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub ExtendedError: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::HRESULT) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IWindowsUpdateActionProgress(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IWindowsUpdateActionProgress {
    type Vtable = IWindowsUpdateActionProgress_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IWindowsUpdateActionProgress {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x83b22d8a_4bb0_549f_ba39_59724882d137);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWindowsUpdateActionProgress_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Action: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub Progress: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IWindowsUpdateActionResult(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IWindowsUpdateActionResult {
    type Vtable = IWindowsUpdateActionResult_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IWindowsUpdateActionResult {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xe6692c62_f697_51b7_ab7f_e73e5e688f12);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWindowsUpdateActionResult_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub Timestamp: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::DateTime) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Timestamp: usize,
    pub Succeeded: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub ExtendedError: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::HRESULT) -> ::windows_core::HRESULT,
    pub Action: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IWindowsUpdateAdministrator(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IWindowsUpdateAdministrator {
    type Vtable = IWindowsUpdateAdministrator_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IWindowsUpdateAdministrator {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x7a60181c_ba1e_5cf9_aa65_304120b73d72);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWindowsUpdateAdministrator_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub StartAdministratorScan: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub ApproveWindowsUpdateAction: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, updateid: ::std::mem::MaybeUninit<::windows_core::HSTRING>, action: ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub RevokeWindowsUpdateActionApproval: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, updateid: ::std::mem::MaybeUninit<::windows_core::HSTRING>, action: ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub ApproveWindowsUpdate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, updateid: ::std::mem::MaybeUninit<::windows_core::HSTRING>, approvaldata: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub RevokeWindowsUpdateApproval: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, updateid: ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub GetUpdates: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetUpdates: usize,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IWindowsUpdateAdministratorStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IWindowsUpdateAdministratorStatics {
    type Vtable = IWindowsUpdateAdministratorStatics_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IWindowsUpdateAdministratorStatics {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x013e6d36_ef69_53bc_8db8_c403bca550ed);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWindowsUpdateAdministratorStatics_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub GetRegisteredAdministrator: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, organizationname: ::std::mem::MaybeUninit<::windows_core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub RegisterForAdministration: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, organizationname: ::std::mem::MaybeUninit<::windows_core::HSTRING>, options: WindowsUpdateAdministratorOptions, result__: *mut WindowsUpdateAdministratorStatus) -> ::windows_core::HRESULT,
    pub UnregisterForAdministration: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, organizationname: ::std::mem::MaybeUninit<::windows_core::HSTRING>, result__: *mut WindowsUpdateAdministratorStatus) -> ::windows_core::HRESULT,
    pub GetRegisteredAdministratorName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub RequestRestart: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, restartoptions: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub CancelRestartRequest: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, requestrestarttoken: ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IWindowsUpdateApprovalData(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IWindowsUpdateApprovalData {
    type Vtable = IWindowsUpdateApprovalData_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IWindowsUpdateApprovalData {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xaadf5bfd_84db_59bc_85e2_ad4fc1f62f7c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWindowsUpdateApprovalData_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub Seeker: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Seeker: usize,
    #[cfg(feature = "Foundation")]
    pub SetSeeker: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetSeeker: usize,
    #[cfg(feature = "Foundation")]
    pub AllowDownloadOnMetered: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    AllowDownloadOnMetered: usize,
    #[cfg(feature = "Foundation")]
    pub SetAllowDownloadOnMetered: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetAllowDownloadOnMetered: usize,
    #[cfg(feature = "Foundation")]
    pub ComplianceDeadlineInDays: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ComplianceDeadlineInDays: usize,
    #[cfg(feature = "Foundation")]
    pub SetComplianceDeadlineInDays: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetComplianceDeadlineInDays: usize,
    #[cfg(feature = "Foundation")]
    pub ComplianceGracePeriodInDays: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ComplianceGracePeriodInDays: usize,
    #[cfg(feature = "Foundation")]
    pub SetComplianceGracePeriodInDays: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetComplianceGracePeriodInDays: usize,
    #[cfg(feature = "Foundation")]
    pub OptOutOfAutoReboot: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    OptOutOfAutoReboot: usize,
    #[cfg(feature = "Foundation")]
    pub SetOptOutOfAutoReboot: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetOptOutOfAutoReboot: usize,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IWindowsUpdateAttentionRequiredInfo(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IWindowsUpdateAttentionRequiredInfo {
    type Vtable = IWindowsUpdateAttentionRequiredInfo_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IWindowsUpdateAttentionRequiredInfo {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x44df2579_74d3_5ffa_b6ce_09e187e1e0ed);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWindowsUpdateAttentionRequiredInfo_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Reason: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut WindowsUpdateAttentionRequiredReason) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub Timestamp: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Timestamp: usize,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IWindowsUpdateAttentionRequiredReasonChangedEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IWindowsUpdateAttentionRequiredReasonChangedEventArgs {
    type Vtable = IWindowsUpdateAttentionRequiredReasonChangedEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IWindowsUpdateAttentionRequiredReasonChangedEventArgs {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x0627abca_dbb8_524a_b1d2_d9df004eeb31);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWindowsUpdateAttentionRequiredReasonChangedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Update: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub Reason: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut WindowsUpdateAttentionRequiredReason) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IWindowsUpdateGetAdministratorResult(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IWindowsUpdateGetAdministratorResult {
    type Vtable = IWindowsUpdateGetAdministratorResult_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IWindowsUpdateGetAdministratorResult {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xbb39ffc4_2c42_5b1c_8995_343341c92c50);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWindowsUpdateGetAdministratorResult_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Administrator: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub Status: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut WindowsUpdateAdministratorStatus) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IWindowsUpdateItem(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IWindowsUpdateItem {
    type Vtable = IWindowsUpdateItem_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IWindowsUpdateItem {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xb222e44a_49b6_59bf_a033_ef617cd73a98);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWindowsUpdateItem_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub ProviderId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub UpdateId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub Timestamp: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::DateTime) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Timestamp: usize,
    pub Title: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub Description: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub MoreInfoUrl: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    MoreInfoUrl: usize,
    pub Category: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub Operation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IWindowsUpdateManager(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IWindowsUpdateManager {
    type Vtable = IWindowsUpdateManager_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IWindowsUpdateManager {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x5dd966c0_a71a_5602_bbd0_09a70e4573fa);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWindowsUpdateManager_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub ScanningStateChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ScanningStateChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveScanningStateChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveScanningStateChanged: usize,
    #[cfg(feature = "Foundation")]
    pub WorkingStateChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    WorkingStateChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveWorkingStateChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveWorkingStateChanged: usize,
    #[cfg(feature = "Foundation")]
    pub ProgressChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ProgressChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveProgressChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveProgressChanged: usize,
    #[cfg(feature = "Foundation")]
    pub AttentionRequiredReasonChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    AttentionRequiredReasonChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveAttentionRequiredReasonChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveAttentionRequiredReasonChanged: usize,
    #[cfg(feature = "Foundation")]
    pub ActionCompleted: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ActionCompleted: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveActionCompleted: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveActionCompleted: usize,
    #[cfg(feature = "Foundation")]
    pub ScanCompleted: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ScanCompleted: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveScanCompleted: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveScanCompleted: usize,
    pub IsScanning: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub IsWorking: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub LastSuccessfulScanTimestamp: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    LastSuccessfulScanTimestamp: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub GetApplicableUpdates: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetApplicableUpdates: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub GetMostRecentCompletedUpdates: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: i32, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetMostRecentCompletedUpdates: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub GetMostRecentCompletedUpdatesAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: i32, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetMostRecentCompletedUpdatesAsync: usize,
    pub StartScan: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, userinitiated: bool) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IWindowsUpdateManagerFactory(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IWindowsUpdateManagerFactory {
    type Vtable = IWindowsUpdateManagerFactory_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IWindowsUpdateManagerFactory {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x1b394df8_decb_5f44_b47c_6ccf3bcfdb37);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWindowsUpdateManagerFactory_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub CreateInstance: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, clientid: ::std::mem::MaybeUninit<::windows_core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IWindowsUpdateProgressChangedEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IWindowsUpdateProgressChangedEventArgs {
    type Vtable = IWindowsUpdateProgressChangedEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IWindowsUpdateProgressChangedEventArgs {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xbbfbdeeb_94c8_5aa7_b0fb_66c67c233b0a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWindowsUpdateProgressChangedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Update: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub ActionProgress: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IWindowsUpdateRestartRequestOptions(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IWindowsUpdateRestartRequestOptions {
    type Vtable = IWindowsUpdateRestartRequestOptions_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IWindowsUpdateRestartRequestOptions {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x38cfb7d3_4188_5222_905c_6c4443c951ee);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWindowsUpdateRestartRequestOptions_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Title: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SetTitle: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub Description: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SetDescription: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub MoreInfoUrl: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    MoreInfoUrl: usize,
    #[cfg(feature = "Foundation")]
    pub SetMoreInfoUrl: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetMoreInfoUrl: usize,
    pub ComplianceDeadlineInDays: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows_core::HRESULT,
    pub SetComplianceDeadlineInDays: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: i32) -> ::windows_core::HRESULT,
    pub ComplianceGracePeriodInDays: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows_core::HRESULT,
    pub SetComplianceGracePeriodInDays: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: i32) -> ::windows_core::HRESULT,
    pub OrganizationName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SetOrganizationName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub OptOutOfAutoReboot: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub SetOptOutOfAutoReboot: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IWindowsUpdateRestartRequestOptionsFactory(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IWindowsUpdateRestartRequestOptionsFactory {
    type Vtable = IWindowsUpdateRestartRequestOptionsFactory_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IWindowsUpdateRestartRequestOptionsFactory {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x75f41d04_0e17_50d0_8c15_6b9d0539b3a9);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWindowsUpdateRestartRequestOptionsFactory_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub CreateInstance: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, title: ::std::mem::MaybeUninit<::windows_core::HSTRING>, description: ::std::mem::MaybeUninit<::windows_core::HSTRING>, moreinfourl: *mut ::core::ffi::c_void, compliancedeadlineindays: i32, compliancegraceperiodindays: i32, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    CreateInstance: usize,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IWindowsUpdateScanCompletedEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IWindowsUpdateScanCompletedEventArgs {
    type Vtable = IWindowsUpdateScanCompletedEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IWindowsUpdateScanCompletedEventArgs {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x95b6953e_ba5c_5fe8_b115_12de184a6bb0);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWindowsUpdateScanCompletedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub ProviderId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub Succeeded: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub ExtendedError: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::HRESULT) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub Updates: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Updates: usize,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct PreviewBuildsManager(::windows_core::IUnknown);
impl PreviewBuildsManager {
    pub fn ArePreviewBuildsAllowed(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ArePreviewBuildsAllowed)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetArePreviewBuildsAllowed(&self, value: bool) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetArePreviewBuildsAllowed)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn GetCurrentState(&self) -> ::windows_core::Result<PreviewBuildsState> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetCurrentState)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn SyncAsync(&self) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SyncAsync)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn GetDefault() -> ::windows_core::Result<PreviewBuildsManager> {
        Self::IPreviewBuildsManagerStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetDefault)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn IsSupported() -> ::windows_core::Result<bool> {
        Self::IPreviewBuildsManagerStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IsSupported)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IPreviewBuildsManagerStatics<R, F: FnOnce(&IPreviewBuildsManagerStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<PreviewBuildsManager, IPreviewBuildsManagerStatics> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::windows_core::RuntimeType for PreviewBuildsManager {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Management.Update.PreviewBuildsManager;{fa07dd61-7e4f-59f7-7c9f-def9051c5f62})");
}
unsafe impl ::windows_core::Interface for PreviewBuildsManager {
    type Vtable = IPreviewBuildsManager_Vtbl;
}
unsafe impl ::windows_core::ComInterface for PreviewBuildsManager {
    const IID: ::windows_core::GUID = <IPreviewBuildsManager as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for PreviewBuildsManager {
    const NAME: &'static str = "Windows.Management.Update.PreviewBuildsManager";
}
::windows_core::imp::interface_hierarchy!(PreviewBuildsManager, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for PreviewBuildsManager {}
unsafe impl ::core::marker::Sync for PreviewBuildsManager {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct PreviewBuildsState(::windows_core::IUnknown);
impl PreviewBuildsState {
    #[doc = "Required features: `\"Foundation_Collections\"`"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Properties(&self) -> ::windows_core::Result<super::super::Foundation::Collections::ValueSet> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Properties)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::windows_core::RuntimeType for PreviewBuildsState {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Management.Update.PreviewBuildsState;{a2f2903e-b223-5f63-7546-3e8eac070a2e})");
}
unsafe impl ::windows_core::Interface for PreviewBuildsState {
    type Vtable = IPreviewBuildsState_Vtbl;
}
unsafe impl ::windows_core::ComInterface for PreviewBuildsState {
    const IID: ::windows_core::GUID = <IPreviewBuildsState as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for PreviewBuildsState {
    const NAME: &'static str = "Windows.Management.Update.PreviewBuildsState";
}
::windows_core::imp::interface_hierarchy!(PreviewBuildsState, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for PreviewBuildsState {}
unsafe impl ::core::marker::Sync for PreviewBuildsState {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct WindowsUpdate(::windows_core::IUnknown);
impl WindowsUpdate {
    pub fn ProviderId(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ProviderId)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn UpdateId(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).UpdateId)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Title(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Title)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Description(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Description)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn IsFeatureUpdate(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IsFeatureUpdate)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn IsMinorImpact(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IsMinorImpact)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn IsSecurity(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IsSecurity)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn IsCritical(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IsCritical)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn IsForOS(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IsForOS)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn IsDriver(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IsDriver)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn IsMandatory(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IsMandatory)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn IsUrgent(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IsUrgent)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn IsSeeker(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IsSeeker)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn MoreInfoUrl(&self) -> ::windows_core::Result<super::super::Foundation::Uri> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).MoreInfoUrl)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn SupportUrl(&self) -> ::windows_core::Result<super::super::Foundation::Uri> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SupportUrl)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn IsEulaAccepted(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IsEulaAccepted)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn EulaText(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).EulaText)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn Deadline(&self) -> ::windows_core::Result<super::super::Foundation::IReference<super::super::Foundation::DateTime>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Deadline)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn AttentionRequiredInfo(&self) -> ::windows_core::Result<WindowsUpdateAttentionRequiredInfo> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).AttentionRequiredInfo)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn ActionResult(&self) -> ::windows_core::Result<WindowsUpdateActionResult> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ActionResult)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn CurrentAction(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CurrentAction)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn ActionProgress(&self) -> ::windows_core::Result<WindowsUpdateActionProgress> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ActionProgress)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn GetPropertyValue(&self, propertyname: &::windows_core::HSTRING) -> ::windows_core::Result<::windows_core::IInspectable> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetPropertyValue)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(propertyname), &mut result__).from_abi(result__)
        }
    }
    pub fn AcceptEula(&self) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).AcceptEula)(::windows_core::Interface::as_raw(this)).ok() }
    }
}
impl ::windows_core::RuntimeType for WindowsUpdate {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Management.Update.WindowsUpdate;{c3c88dd7-0ef3-52b2-a9ad-66bfc6bd9582})");
}
unsafe impl ::windows_core::Interface for WindowsUpdate {
    type Vtable = IWindowsUpdate_Vtbl;
}
unsafe impl ::windows_core::ComInterface for WindowsUpdate {
    const IID: ::windows_core::GUID = <IWindowsUpdate as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for WindowsUpdate {
    const NAME: &'static str = "Windows.Management.Update.WindowsUpdate";
}
::windows_core::imp::interface_hierarchy!(WindowsUpdate, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for WindowsUpdate {}
unsafe impl ::core::marker::Sync for WindowsUpdate {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct WindowsUpdateActionCompletedEventArgs(::windows_core::IUnknown);
impl WindowsUpdateActionCompletedEventArgs {
    pub fn Update(&self) -> ::windows_core::Result<WindowsUpdate> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Update)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Action(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Action)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Succeeded(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Succeeded)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn ExtendedError(&self) -> ::windows_core::Result<::windows_core::HRESULT> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ExtendedError)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::windows_core::RuntimeType for WindowsUpdateActionCompletedEventArgs {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Management.Update.WindowsUpdateActionCompletedEventArgs;{2c44b950-a655-5321-aec1-aee762922131})");
}
unsafe impl ::windows_core::Interface for WindowsUpdateActionCompletedEventArgs {
    type Vtable = IWindowsUpdateActionCompletedEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for WindowsUpdateActionCompletedEventArgs {
    const IID: ::windows_core::GUID = <IWindowsUpdateActionCompletedEventArgs as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for WindowsUpdateActionCompletedEventArgs {
    const NAME: &'static str = "Windows.Management.Update.WindowsUpdateActionCompletedEventArgs";
}
::windows_core::imp::interface_hierarchy!(WindowsUpdateActionCompletedEventArgs, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for WindowsUpdateActionCompletedEventArgs {}
unsafe impl ::core::marker::Sync for WindowsUpdateActionCompletedEventArgs {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct WindowsUpdateActionProgress(::windows_core::IUnknown);
impl WindowsUpdateActionProgress {
    pub fn Action(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Action)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Progress(&self) -> ::windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Progress)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::windows_core::RuntimeType for WindowsUpdateActionProgress {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Management.Update.WindowsUpdateActionProgress;{83b22d8a-4bb0-549f-ba39-59724882d137})");
}
unsafe impl ::windows_core::Interface for WindowsUpdateActionProgress {
    type Vtable = IWindowsUpdateActionProgress_Vtbl;
}
unsafe impl ::windows_core::ComInterface for WindowsUpdateActionProgress {
    const IID: ::windows_core::GUID = <IWindowsUpdateActionProgress as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for WindowsUpdateActionProgress {
    const NAME: &'static str = "Windows.Management.Update.WindowsUpdateActionProgress";
}
::windows_core::imp::interface_hierarchy!(WindowsUpdateActionProgress, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for WindowsUpdateActionProgress {}
unsafe impl ::core::marker::Sync for WindowsUpdateActionProgress {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct WindowsUpdateActionResult(::windows_core::IUnknown);
impl WindowsUpdateActionResult {
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn Timestamp(&self) -> ::windows_core::Result<super::super::Foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Timestamp)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Succeeded(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Succeeded)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn ExtendedError(&self) -> ::windows_core::Result<::windows_core::HRESULT> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ExtendedError)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Action(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Action)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::windows_core::RuntimeType for WindowsUpdateActionResult {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Management.Update.WindowsUpdateActionResult;{e6692c62-f697-51b7-ab7f-e73e5e688f12})");
}
unsafe impl ::windows_core::Interface for WindowsUpdateActionResult {
    type Vtable = IWindowsUpdateActionResult_Vtbl;
}
unsafe impl ::windows_core::ComInterface for WindowsUpdateActionResult {
    const IID: ::windows_core::GUID = <IWindowsUpdateActionResult as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for WindowsUpdateActionResult {
    const NAME: &'static str = "Windows.Management.Update.WindowsUpdateActionResult";
}
::windows_core::imp::interface_hierarchy!(WindowsUpdateActionResult, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for WindowsUpdateActionResult {}
unsafe impl ::core::marker::Sync for WindowsUpdateActionResult {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct WindowsUpdateAdministrator(::windows_core::IUnknown);
impl WindowsUpdateAdministrator {
    pub fn StartAdministratorScan(&self) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).StartAdministratorScan)(::windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn ApproveWindowsUpdateAction(&self, updateid: &::windows_core::HSTRING, action: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).ApproveWindowsUpdateAction)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(updateid), ::core::mem::transmute_copy(action)).ok() }
    }
    pub fn RevokeWindowsUpdateActionApproval(&self, updateid: &::windows_core::HSTRING, action: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RevokeWindowsUpdateActionApproval)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(updateid), ::core::mem::transmute_copy(action)).ok() }
    }
    pub fn ApproveWindowsUpdate<P0>(&self, updateid: &::windows_core::HSTRING, approvaldata: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<WindowsUpdateApprovalData>,
    {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).ApproveWindowsUpdate)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(updateid), approvaldata.into_param().abi()).ok() }
    }
    pub fn RevokeWindowsUpdateApproval(&self, updateid: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RevokeWindowsUpdateApproval)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(updateid)).ok() }
    }
    #[doc = "Required features: `\"Foundation_Collections\"`"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetUpdates(&self) -> ::windows_core::Result<super::super::Foundation::Collections::IVectorView<WindowsUpdate>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetUpdates)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn GetRegisteredAdministrator(organizationname: &::windows_core::HSTRING) -> ::windows_core::Result<WindowsUpdateGetAdministratorResult> {
        Self::IWindowsUpdateAdministratorStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetRegisteredAdministrator)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(organizationname), &mut result__).from_abi(result__)
        })
    }
    pub fn RegisterForAdministration(organizationname: &::windows_core::HSTRING, options: WindowsUpdateAdministratorOptions) -> ::windows_core::Result<WindowsUpdateAdministratorStatus> {
        Self::IWindowsUpdateAdministratorStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).RegisterForAdministration)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(organizationname), options, &mut result__).from_abi(result__)
        })
    }
    pub fn UnregisterForAdministration(organizationname: &::windows_core::HSTRING) -> ::windows_core::Result<WindowsUpdateAdministratorStatus> {
        Self::IWindowsUpdateAdministratorStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).UnregisterForAdministration)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(organizationname), &mut result__).from_abi(result__)
        })
    }
    pub fn GetRegisteredAdministratorName() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::IWindowsUpdateAdministratorStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetRegisteredAdministratorName)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn RequestRestart<P0>(restartoptions: P0) -> ::windows_core::Result<::windows_core::HSTRING>
    where
        P0: ::windows_core::IntoParam<WindowsUpdateRestartRequestOptions>,
    {
        Self::IWindowsUpdateAdministratorStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).RequestRestart)(::windows_core::Interface::as_raw(this), restartoptions.into_param().abi(), &mut result__).from_abi(result__)
        })
    }
    pub fn CancelRestartRequest(requestrestarttoken: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        Self::IWindowsUpdateAdministratorStatics(|this| unsafe { (::windows_core::Interface::vtable(this).CancelRestartRequest)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(requestrestarttoken)).ok() })
    }
    #[doc(hidden)]
    pub fn IWindowsUpdateAdministratorStatics<R, F: FnOnce(&IWindowsUpdateAdministratorStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<WindowsUpdateAdministrator, IWindowsUpdateAdministratorStatics> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::windows_core::RuntimeType for WindowsUpdateAdministrator {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Management.Update.WindowsUpdateAdministrator;{7a60181c-ba1e-5cf9-aa65-304120b73d72})");
}
unsafe impl ::windows_core::Interface for WindowsUpdateAdministrator {
    type Vtable = IWindowsUpdateAdministrator_Vtbl;
}
unsafe impl ::windows_core::ComInterface for WindowsUpdateAdministrator {
    const IID: ::windows_core::GUID = <IWindowsUpdateAdministrator as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for WindowsUpdateAdministrator {
    const NAME: &'static str = "Windows.Management.Update.WindowsUpdateAdministrator";
}
::windows_core::imp::interface_hierarchy!(WindowsUpdateAdministrator, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for WindowsUpdateAdministrator {}
unsafe impl ::core::marker::Sync for WindowsUpdateAdministrator {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct WindowsUpdateApprovalData(::windows_core::IUnknown);
impl WindowsUpdateApprovalData {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::imp::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<WindowsUpdateApprovalData, ::windows_core::imp::IGenericFactory> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn Seeker(&self) -> ::windows_core::Result<super::super::Foundation::IReference<bool>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Seeker)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn SetSeeker<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<super::super::Foundation::IReference<bool>>,
    {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetSeeker)(::windows_core::Interface::as_raw(this), value.try_into_param()?.abi()).ok() }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn AllowDownloadOnMetered(&self) -> ::windows_core::Result<super::super::Foundation::IReference<bool>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).AllowDownloadOnMetered)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn SetAllowDownloadOnMetered<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<super::super::Foundation::IReference<bool>>,
    {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetAllowDownloadOnMetered)(::windows_core::Interface::as_raw(this), value.try_into_param()?.abi()).ok() }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn ComplianceDeadlineInDays(&self) -> ::windows_core::Result<super::super::Foundation::IReference<i32>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ComplianceDeadlineInDays)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn SetComplianceDeadlineInDays<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<super::super::Foundation::IReference<i32>>,
    {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetComplianceDeadlineInDays)(::windows_core::Interface::as_raw(this), value.try_into_param()?.abi()).ok() }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn ComplianceGracePeriodInDays(&self) -> ::windows_core::Result<super::super::Foundation::IReference<i32>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ComplianceGracePeriodInDays)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn SetComplianceGracePeriodInDays<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<super::super::Foundation::IReference<i32>>,
    {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetComplianceGracePeriodInDays)(::windows_core::Interface::as_raw(this), value.try_into_param()?.abi()).ok() }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn OptOutOfAutoReboot(&self) -> ::windows_core::Result<super::super::Foundation::IReference<bool>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).OptOutOfAutoReboot)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn SetOptOutOfAutoReboot<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<super::super::Foundation::IReference<bool>>,
    {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetOptOutOfAutoReboot)(::windows_core::Interface::as_raw(this), value.try_into_param()?.abi()).ok() }
    }
}
impl ::windows_core::RuntimeType for WindowsUpdateApprovalData {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Management.Update.WindowsUpdateApprovalData;{aadf5bfd-84db-59bc-85e2-ad4fc1f62f7c})");
}
unsafe impl ::windows_core::Interface for WindowsUpdateApprovalData {
    type Vtable = IWindowsUpdateApprovalData_Vtbl;
}
unsafe impl ::windows_core::ComInterface for WindowsUpdateApprovalData {
    const IID: ::windows_core::GUID = <IWindowsUpdateApprovalData as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for WindowsUpdateApprovalData {
    const NAME: &'static str = "Windows.Management.Update.WindowsUpdateApprovalData";
}
::windows_core::imp::interface_hierarchy!(WindowsUpdateApprovalData, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for WindowsUpdateApprovalData {}
unsafe impl ::core::marker::Sync for WindowsUpdateApprovalData {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct WindowsUpdateAttentionRequiredInfo(::windows_core::IUnknown);
impl WindowsUpdateAttentionRequiredInfo {
    pub fn Reason(&self) -> ::windows_core::Result<WindowsUpdateAttentionRequiredReason> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Reason)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn Timestamp(&self) -> ::windows_core::Result<super::super::Foundation::IReference<super::super::Foundation::DateTime>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Timestamp)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::windows_core::RuntimeType for WindowsUpdateAttentionRequiredInfo {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Management.Update.WindowsUpdateAttentionRequiredInfo;{44df2579-74d3-5ffa-b6ce-09e187e1e0ed})");
}
unsafe impl ::windows_core::Interface for WindowsUpdateAttentionRequiredInfo {
    type Vtable = IWindowsUpdateAttentionRequiredInfo_Vtbl;
}
unsafe impl ::windows_core::ComInterface for WindowsUpdateAttentionRequiredInfo {
    const IID: ::windows_core::GUID = <IWindowsUpdateAttentionRequiredInfo as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for WindowsUpdateAttentionRequiredInfo {
    const NAME: &'static str = "Windows.Management.Update.WindowsUpdateAttentionRequiredInfo";
}
::windows_core::imp::interface_hierarchy!(WindowsUpdateAttentionRequiredInfo, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for WindowsUpdateAttentionRequiredInfo {}
unsafe impl ::core::marker::Sync for WindowsUpdateAttentionRequiredInfo {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct WindowsUpdateAttentionRequiredReasonChangedEventArgs(::windows_core::IUnknown);
impl WindowsUpdateAttentionRequiredReasonChangedEventArgs {
    pub fn Update(&self) -> ::windows_core::Result<WindowsUpdate> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Update)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Reason(&self) -> ::windows_core::Result<WindowsUpdateAttentionRequiredReason> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Reason)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::windows_core::RuntimeType for WindowsUpdateAttentionRequiredReasonChangedEventArgs {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Management.Update.WindowsUpdateAttentionRequiredReasonChangedEventArgs;{0627abca-dbb8-524a-b1d2-d9df004eeb31})");
}
unsafe impl ::windows_core::Interface for WindowsUpdateAttentionRequiredReasonChangedEventArgs {
    type Vtable = IWindowsUpdateAttentionRequiredReasonChangedEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for WindowsUpdateAttentionRequiredReasonChangedEventArgs {
    const IID: ::windows_core::GUID = <IWindowsUpdateAttentionRequiredReasonChangedEventArgs as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for WindowsUpdateAttentionRequiredReasonChangedEventArgs {
    const NAME: &'static str = "Windows.Management.Update.WindowsUpdateAttentionRequiredReasonChangedEventArgs";
}
::windows_core::imp::interface_hierarchy!(WindowsUpdateAttentionRequiredReasonChangedEventArgs, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for WindowsUpdateAttentionRequiredReasonChangedEventArgs {}
unsafe impl ::core::marker::Sync for WindowsUpdateAttentionRequiredReasonChangedEventArgs {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct WindowsUpdateGetAdministratorResult(::windows_core::IUnknown);
impl WindowsUpdateGetAdministratorResult {
    pub fn Administrator(&self) -> ::windows_core::Result<WindowsUpdateAdministrator> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Administrator)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Status(&self) -> ::windows_core::Result<WindowsUpdateAdministratorStatus> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Status)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::windows_core::RuntimeType for WindowsUpdateGetAdministratorResult {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Management.Update.WindowsUpdateGetAdministratorResult;{bb39ffc4-2c42-5b1c-8995-343341c92c50})");
}
unsafe impl ::windows_core::Interface for WindowsUpdateGetAdministratorResult {
    type Vtable = IWindowsUpdateGetAdministratorResult_Vtbl;
}
unsafe impl ::windows_core::ComInterface for WindowsUpdateGetAdministratorResult {
    const IID: ::windows_core::GUID = <IWindowsUpdateGetAdministratorResult as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for WindowsUpdateGetAdministratorResult {
    const NAME: &'static str = "Windows.Management.Update.WindowsUpdateGetAdministratorResult";
}
::windows_core::imp::interface_hierarchy!(WindowsUpdateGetAdministratorResult, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for WindowsUpdateGetAdministratorResult {}
unsafe impl ::core::marker::Sync for WindowsUpdateGetAdministratorResult {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct WindowsUpdateItem(::windows_core::IUnknown);
impl WindowsUpdateItem {
    pub fn ProviderId(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ProviderId)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn UpdateId(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).UpdateId)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn Timestamp(&self) -> ::windows_core::Result<super::super::Foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Timestamp)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Title(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Title)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Description(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Description)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn MoreInfoUrl(&self) -> ::windows_core::Result<super::super::Foundation::Uri> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).MoreInfoUrl)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Category(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Category)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Operation(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Operation)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::windows_core::RuntimeType for WindowsUpdateItem {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Management.Update.WindowsUpdateItem;{b222e44a-49b6-59bf-a033-ef617cd73a98})");
}
unsafe impl ::windows_core::Interface for WindowsUpdateItem {
    type Vtable = IWindowsUpdateItem_Vtbl;
}
unsafe impl ::windows_core::ComInterface for WindowsUpdateItem {
    const IID: ::windows_core::GUID = <IWindowsUpdateItem as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for WindowsUpdateItem {
    const NAME: &'static str = "Windows.Management.Update.WindowsUpdateItem";
}
::windows_core::imp::interface_hierarchy!(WindowsUpdateItem, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for WindowsUpdateItem {}
unsafe impl ::core::marker::Sync for WindowsUpdateItem {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct WindowsUpdateManager(::windows_core::IUnknown);
impl WindowsUpdateManager {
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn ScanningStateChanged<P0>(&self, handler: P0) -> ::windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::TypedEventHandler<WindowsUpdateManager, ::windows_core::IInspectable>>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ScanningStateChanged)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveScanningStateChanged(&self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveScanningStateChanged)(::windows_core::Interface::as_raw(this), token).ok() }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn WorkingStateChanged<P0>(&self, handler: P0) -> ::windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::TypedEventHandler<WindowsUpdateManager, ::windows_core::IInspectable>>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).WorkingStateChanged)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveWorkingStateChanged(&self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveWorkingStateChanged)(::windows_core::Interface::as_raw(this), token).ok() }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn ProgressChanged<P0>(&self, handler: P0) -> ::windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::TypedEventHandler<WindowsUpdateManager, WindowsUpdateProgressChangedEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ProgressChanged)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveProgressChanged(&self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveProgressChanged)(::windows_core::Interface::as_raw(this), token).ok() }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn AttentionRequiredReasonChanged<P0>(&self, handler: P0) -> ::windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::TypedEventHandler<WindowsUpdateManager, WindowsUpdateAttentionRequiredReasonChangedEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).AttentionRequiredReasonChanged)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveAttentionRequiredReasonChanged(&self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveAttentionRequiredReasonChanged)(::windows_core::Interface::as_raw(this), token).ok() }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn ActionCompleted<P0>(&self, handler: P0) -> ::windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::TypedEventHandler<WindowsUpdateManager, WindowsUpdateActionCompletedEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ActionCompleted)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveActionCompleted(&self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveActionCompleted)(::windows_core::Interface::as_raw(this), token).ok() }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn ScanCompleted<P0>(&self, handler: P0) -> ::windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::TypedEventHandler<WindowsUpdateManager, WindowsUpdateScanCompletedEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ScanCompleted)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveScanCompleted(&self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveScanCompleted)(::windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn IsScanning(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IsScanning)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn IsWorking(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IsWorking)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn LastSuccessfulScanTimestamp(&self) -> ::windows_core::Result<super::super::Foundation::IReference<super::super::Foundation::DateTime>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).LastSuccessfulScanTimestamp)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation_Collections\"`"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetApplicableUpdates(&self) -> ::windows_core::Result<super::super::Foundation::Collections::IVectorView<WindowsUpdate>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetApplicableUpdates)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation_Collections\"`"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetMostRecentCompletedUpdates(&self, count: i32) -> ::windows_core::Result<super::super::Foundation::Collections::IVectorView<WindowsUpdateItem>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetMostRecentCompletedUpdates)(::windows_core::Interface::as_raw(this), count, &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation_Collections\"`"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetMostRecentCompletedUpdatesAsync(&self, count: i32) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<WindowsUpdateItem>>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetMostRecentCompletedUpdatesAsync)(::windows_core::Interface::as_raw(this), count, &mut result__).from_abi(result__)
        }
    }
    pub fn StartScan(&self, userinitiated: bool) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).StartScan)(::windows_core::Interface::as_raw(this), userinitiated).ok() }
    }
    pub fn CreateInstance(clientid: &::windows_core::HSTRING) -> ::windows_core::Result<WindowsUpdateManager> {
        Self::IWindowsUpdateManagerFactory(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CreateInstance)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(clientid), &mut result__).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IWindowsUpdateManagerFactory<R, F: FnOnce(&IWindowsUpdateManagerFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<WindowsUpdateManager, IWindowsUpdateManagerFactory> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::windows_core::RuntimeType for WindowsUpdateManager {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Management.Update.WindowsUpdateManager;{5dd966c0-a71a-5602-bbd0-09a70e4573fa})");
}
unsafe impl ::windows_core::Interface for WindowsUpdateManager {
    type Vtable = IWindowsUpdateManager_Vtbl;
}
unsafe impl ::windows_core::ComInterface for WindowsUpdateManager {
    const IID: ::windows_core::GUID = <IWindowsUpdateManager as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for WindowsUpdateManager {
    const NAME: &'static str = "Windows.Management.Update.WindowsUpdateManager";
}
::windows_core::imp::interface_hierarchy!(WindowsUpdateManager, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for WindowsUpdateManager {}
unsafe impl ::core::marker::Sync for WindowsUpdateManager {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct WindowsUpdateProgressChangedEventArgs(::windows_core::IUnknown);
impl WindowsUpdateProgressChangedEventArgs {
    pub fn Update(&self) -> ::windows_core::Result<WindowsUpdate> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Update)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn ActionProgress(&self) -> ::windows_core::Result<WindowsUpdateActionProgress> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ActionProgress)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::windows_core::RuntimeType for WindowsUpdateProgressChangedEventArgs {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Management.Update.WindowsUpdateProgressChangedEventArgs;{bbfbdeeb-94c8-5aa7-b0fb-66c67c233b0a})");
}
unsafe impl ::windows_core::Interface for WindowsUpdateProgressChangedEventArgs {
    type Vtable = IWindowsUpdateProgressChangedEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for WindowsUpdateProgressChangedEventArgs {
    const IID: ::windows_core::GUID = <IWindowsUpdateProgressChangedEventArgs as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for WindowsUpdateProgressChangedEventArgs {
    const NAME: &'static str = "Windows.Management.Update.WindowsUpdateProgressChangedEventArgs";
}
::windows_core::imp::interface_hierarchy!(WindowsUpdateProgressChangedEventArgs, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for WindowsUpdateProgressChangedEventArgs {}
unsafe impl ::core::marker::Sync for WindowsUpdateProgressChangedEventArgs {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct WindowsUpdateRestartRequestOptions(::windows_core::IUnknown);
impl WindowsUpdateRestartRequestOptions {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::imp::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<WindowsUpdateRestartRequestOptions, ::windows_core::imp::IGenericFactory> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn Title(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Title)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetTitle(&self, value: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetTitle)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn Description(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Description)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetDescription(&self, value: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetDescription)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn MoreInfoUrl(&self) -> ::windows_core::Result<super::super::Foundation::Uri> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).MoreInfoUrl)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn SetMoreInfoUrl<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::Uri>,
    {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetMoreInfoUrl)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn ComplianceDeadlineInDays(&self) -> ::windows_core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ComplianceDeadlineInDays)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetComplianceDeadlineInDays(&self, value: i32) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetComplianceDeadlineInDays)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn ComplianceGracePeriodInDays(&self) -> ::windows_core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ComplianceGracePeriodInDays)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetComplianceGracePeriodInDays(&self, value: i32) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetComplianceGracePeriodInDays)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn OrganizationName(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).OrganizationName)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetOrganizationName(&self, value: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetOrganizationName)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn OptOutOfAutoReboot(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).OptOutOfAutoReboot)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetOptOutOfAutoReboot(&self, value: bool) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetOptOutOfAutoReboot)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn CreateInstance<P0>(title: &::windows_core::HSTRING, description: &::windows_core::HSTRING, moreinfourl: P0, compliancedeadlineindays: i32, compliancegraceperiodindays: i32) -> ::windows_core::Result<WindowsUpdateRestartRequestOptions>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::Uri>,
    {
        Self::IWindowsUpdateRestartRequestOptionsFactory(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CreateInstance)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(title), ::core::mem::transmute_copy(description), moreinfourl.into_param().abi(), compliancedeadlineindays, compliancegraceperiodindays, &mut result__).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IWindowsUpdateRestartRequestOptionsFactory<R, F: FnOnce(&IWindowsUpdateRestartRequestOptionsFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<WindowsUpdateRestartRequestOptions, IWindowsUpdateRestartRequestOptionsFactory> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::windows_core::RuntimeType for WindowsUpdateRestartRequestOptions {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Management.Update.WindowsUpdateRestartRequestOptions;{38cfb7d3-4188-5222-905c-6c4443c951ee})");
}
unsafe impl ::windows_core::Interface for WindowsUpdateRestartRequestOptions {
    type Vtable = IWindowsUpdateRestartRequestOptions_Vtbl;
}
unsafe impl ::windows_core::ComInterface for WindowsUpdateRestartRequestOptions {
    const IID: ::windows_core::GUID = <IWindowsUpdateRestartRequestOptions as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for WindowsUpdateRestartRequestOptions {
    const NAME: &'static str = "Windows.Management.Update.WindowsUpdateRestartRequestOptions";
}
::windows_core::imp::interface_hierarchy!(WindowsUpdateRestartRequestOptions, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for WindowsUpdateRestartRequestOptions {}
unsafe impl ::core::marker::Sync for WindowsUpdateRestartRequestOptions {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct WindowsUpdateScanCompletedEventArgs(::windows_core::IUnknown);
impl WindowsUpdateScanCompletedEventArgs {
    pub fn ProviderId(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ProviderId)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Succeeded(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Succeeded)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn ExtendedError(&self) -> ::windows_core::Result<::windows_core::HRESULT> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ExtendedError)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation_Collections\"`"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Updates(&self) -> ::windows_core::Result<super::super::Foundation::Collections::IVectorView<WindowsUpdate>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Updates)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::windows_core::RuntimeType for WindowsUpdateScanCompletedEventArgs {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Management.Update.WindowsUpdateScanCompletedEventArgs;{95b6953e-ba5c-5fe8-b115-12de184a6bb0})");
}
unsafe impl ::windows_core::Interface for WindowsUpdateScanCompletedEventArgs {
    type Vtable = IWindowsUpdateScanCompletedEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for WindowsUpdateScanCompletedEventArgs {
    const IID: ::windows_core::GUID = <IWindowsUpdateScanCompletedEventArgs as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for WindowsUpdateScanCompletedEventArgs {
    const NAME: &'static str = "Windows.Management.Update.WindowsUpdateScanCompletedEventArgs";
}
::windows_core::imp::interface_hierarchy!(WindowsUpdateScanCompletedEventArgs, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for WindowsUpdateScanCompletedEventArgs {}
unsafe impl ::core::marker::Sync for WindowsUpdateScanCompletedEventArgs {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct WindowsUpdateAdministratorOptions(pub u32);
impl WindowsUpdateAdministratorOptions {
    pub const None: Self = Self(0u32);
    pub const RequireAdministratorApprovalForScans: Self = Self(1u32);
    pub const RequireAdministratorApprovalForUpdates: Self = Self(2u32);
    pub const RequireAdministratorApprovalForActions: Self = Self(4u32);
}
impl ::core::marker::Copy for WindowsUpdateAdministratorOptions {}
impl ::core::clone::Clone for WindowsUpdateAdministratorOptions {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WindowsUpdateAdministratorOptions {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for WindowsUpdateAdministratorOptions {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for WindowsUpdateAdministratorOptions {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WindowsUpdateAdministratorOptions").field(&self.0).finish()
    }
}
impl WindowsUpdateAdministratorOptions {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl ::core::ops::BitOr for WindowsUpdateAdministratorOptions {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for WindowsUpdateAdministratorOptions {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for WindowsUpdateAdministratorOptions {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for WindowsUpdateAdministratorOptions {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for WindowsUpdateAdministratorOptions {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl ::windows_core::RuntimeType for WindowsUpdateAdministratorOptions {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Management.Update.WindowsUpdateAdministratorOptions;u4)");
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct WindowsUpdateAdministratorStatus(pub i32);
impl WindowsUpdateAdministratorStatus {
    pub const Succeeded: Self = Self(0i32);
    pub const NoAdministratorRegistered: Self = Self(1i32);
    pub const OtherAdministratorIsRegistered: Self = Self(2i32);
}
impl ::core::marker::Copy for WindowsUpdateAdministratorStatus {}
impl ::core::clone::Clone for WindowsUpdateAdministratorStatus {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WindowsUpdateAdministratorStatus {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for WindowsUpdateAdministratorStatus {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for WindowsUpdateAdministratorStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WindowsUpdateAdministratorStatus").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for WindowsUpdateAdministratorStatus {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Management.Update.WindowsUpdateAdministratorStatus;i4)");
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct WindowsUpdateAttentionRequiredReason(pub i32);
impl WindowsUpdateAttentionRequiredReason {
    pub const None: Self = Self(0i32);
    pub const SeekerUpdate: Self = Self(1i32);
    pub const ReadyToReboot: Self = Self(2i32);
    pub const NeedNonMeteredNetwork: Self = Self(3i32);
    pub const NeedUserAgreementForMeteredNetwork: Self = Self(4i32);
    pub const NeedNetwork: Self = Self(5i32);
    pub const NeedMoreSpace: Self = Self(6i32);
    pub const BatterySaverEnabled: Self = Self(7i32);
    pub const NeedUserInteraction: Self = Self(8i32);
    pub const NeedUserAgreementForPolicy: Self = Self(9i32);
    pub const CompatibilityError: Self = Self(10i32);
    pub const NeedUserInteractionForEula: Self = Self(11i32);
    pub const NeedUserInteractionForCta: Self = Self(12i32);
    pub const Regulated: Self = Self(13i32);
    pub const ExternalReboot: Self = Self(14i32);
    pub const OtherUpdate: Self = Self(15i32);
    pub const BlockedByProvider: Self = Self(16i32);
    pub const BlockedByPostRebootFailure: Self = Self(17i32);
    pub const UserEngaged: Self = Self(18i32);
    pub const BlockedByBattery: Self = Self(19i32);
    pub const Exclusivity: Self = Self(20i32);
    pub const BlockedBySerialization: Self = Self(21i32);
    pub const ConflictClass: Self = Self(22i32);
    pub const BlockedByAdminApproval: Self = Self(23i32);
    pub const BlockedByTooManyAttempts: Self = Self(24i32);
    pub const BlockedByFailure: Self = Self(25i32);
    pub const Demotion: Self = Self(26i32);
    pub const BlockedByActiveHours: Self = Self(27i32);
    pub const ScheduledForMaintenance: Self = Self(28i32);
    pub const PolicyScheduledInstallTime: Self = Self(29i32);
    pub const BlockedByOobe: Self = Self(30i32);
    pub const DeferredDuringOobe: Self = Self(31i32);
    pub const DeferredForSustainableTime: Self = Self(32i32);
}
impl ::core::marker::Copy for WindowsUpdateAttentionRequiredReason {}
impl ::core::clone::Clone for WindowsUpdateAttentionRequiredReason {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WindowsUpdateAttentionRequiredReason {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for WindowsUpdateAttentionRequiredReason {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for WindowsUpdateAttentionRequiredReason {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WindowsUpdateAttentionRequiredReason").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for WindowsUpdateAttentionRequiredReason {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Management.Update.WindowsUpdateAttentionRequiredReason;i4)");
}
