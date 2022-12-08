#[cfg(feature = "ApplicationModel_UserActivities_Core")]
pub mod Core;
#[doc(hidden)]
#[repr(transparent)]
pub struct IUserActivity(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IUserActivity {
    type Vtable = IUserActivity_Vtbl;
}
unsafe impl ::windows::core::Interface for IUserActivity {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xfc103e9e_2cab_4d36_aea2_b4bb556cef0f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUserActivity_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub State: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut UserActivityState) -> ::windows::core::HRESULT,
    pub ActivityId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub VisualElements: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub ContentUri: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ContentUri: usize,
    #[cfg(feature = "Foundation")]
    pub SetContentUri: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetContentUri: usize,
    pub ContentType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetContentType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub FallbackUri: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    FallbackUri: usize,
    #[cfg(feature = "Foundation")]
    pub SetFallbackUri: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetFallbackUri: usize,
    #[cfg(feature = "Foundation")]
    pub ActivationUri: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ActivationUri: usize,
    #[cfg(feature = "Foundation")]
    pub SetActivationUri: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetActivationUri: usize,
    pub ContentInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetContentInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub SaveAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SaveAsync: usize,
    pub CreateSession: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IUserActivity2(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IUserActivity2 {
    type Vtable = IUserActivity2_Vtbl;
}
unsafe impl ::windows::core::Interface for IUserActivity2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9dc40c62_08c4_47ac_aa9c_2bb2221c55fd);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUserActivity2_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub ToJson: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IUserActivity3(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IUserActivity3 {
    type Vtable = IUserActivity3_Vtbl;
}
unsafe impl ::windows::core::Interface for IUserActivity3 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe7697744_e1a2_5147_8e06_55f1eeef271c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUserActivity3_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub IsRoamable: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetIsRoamable: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IUserActivityAttribution(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IUserActivityAttribution {
    type Vtable = IUserActivityAttribution_Vtbl;
}
unsafe impl ::windows::core::Interface for IUserActivityAttribution {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x34a5c8b5_86dd_4aec_a491_6a4faea5d22e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUserActivityAttribution_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub IconUri: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    IconUri: usize,
    #[cfg(feature = "Foundation")]
    pub SetIconUri: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetIconUri: usize,
    pub AlternateText: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetAlternateText: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub AddImageQuery: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetAddImageQuery: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IUserActivityAttributionFactory(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IUserActivityAttributionFactory {
    type Vtable = IUserActivityAttributionFactory_Vtbl;
}
unsafe impl ::windows::core::Interface for IUserActivityAttributionFactory {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe62bd252_c566_4f42_9974_916c4d76377e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUserActivityAttributionFactory_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub CreateWithUri: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iconuri: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    CreateWithUri: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IUserActivityChannel(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IUserActivityChannel {
    type Vtable = IUserActivityChannel_Vtbl;
}
unsafe impl ::windows::core::Interface for IUserActivityChannel {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xbac0f8b8_a0e4_483b_b948_9cbabd06070c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUserActivityChannel_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub GetOrCreateUserActivityAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, activityid: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetOrCreateUserActivityAsync: usize,
    #[cfg(feature = "Foundation")]
    pub DeleteActivityAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, activityid: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    DeleteActivityAsync: usize,
    #[cfg(feature = "Foundation")]
    pub DeleteAllActivitiesAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    DeleteAllActivitiesAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IUserActivityChannel2(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IUserActivityChannel2 {
    type Vtable = IUserActivityChannel2_Vtbl;
}
unsafe impl ::windows::core::Interface for IUserActivityChannel2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1698e35b_eb7e_4ea0_bf17_a459e8be706c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUserActivityChannel2_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub GetRecentUserActivitiesAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, maxuniqueactivities: i32, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetRecentUserActivitiesAsync: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub GetSessionHistoryItemsForUserActivityAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, activityid: *mut ::core::ffi::c_void, starttime: super::super::Foundation::DateTime, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetSessionHistoryItemsForUserActivityAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IUserActivityChannelStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IUserActivityChannelStatics {
    type Vtable = IUserActivityChannelStatics_Vtbl;
}
unsafe impl ::windows::core::Interface for IUserActivityChannelStatics {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc8c005ab_198d_4d80_abb2_c9775ec4a729);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUserActivityChannelStatics_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub GetDefault: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IUserActivityChannelStatics2(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IUserActivityChannelStatics2 {
    type Vtable = IUserActivityChannelStatics2_Vtbl;
}
unsafe impl ::windows::core::Interface for IUserActivityChannelStatics2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8e87de30_aa4f_4624_9ad0_d40f3ba0317c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUserActivityChannelStatics2_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub DisableAutoSessionCreation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Security_Credentials")]
    pub TryGetForWebAccount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, account: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Security_Credentials"))]
    TryGetForWebAccount: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IUserActivityChannelStatics3(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IUserActivityChannelStatics3 {
    type Vtable = IUserActivityChannelStatics3_Vtbl;
}
unsafe impl ::windows::core::Interface for IUserActivityChannelStatics3 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x53bc4ddb_bbdf_5984_802a_5305874e205c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUserActivityChannelStatics3_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "System")]
    pub GetForUser: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, user: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "System"))]
    GetForUser: usize,
}
#[doc = "*Required features: `\"ApplicationModel_UserActivities\"`*"]
#[repr(transparent)]
pub struct IUserActivityContentInfo(::windows::core::IUnknown);
impl IUserActivityContentInfo {
    pub fn ToJson(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ToJson)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
}
::windows::core::interface_hierarchy!(IUserActivityContentInfo, ::windows::core::IUnknown, ::windows::core::IInspectable);
impl ::core::clone::Clone for IUserActivityContentInfo {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IUserActivityContentInfo {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IUserActivityContentInfo {}
impl ::core::fmt::Debug for IUserActivityContentInfo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IUserActivityContentInfo").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for IUserActivityContentInfo {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{b399e5ad-137f-409d-822d-e1af27ce08dc}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for IUserActivityContentInfo {
    type Vtable = IUserActivityContentInfo_Vtbl;
}
unsafe impl ::windows::core::Interface for IUserActivityContentInfo {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb399e5ad_137f_409d_822d_e1af27ce08dc);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUserActivityContentInfo_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub ToJson: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IUserActivityContentInfoStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IUserActivityContentInfoStatics {
    type Vtable = IUserActivityContentInfoStatics_Vtbl;
}
unsafe impl ::windows::core::Interface for IUserActivityContentInfoStatics {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9988c34b_0386_4bc9_968a_8200b004144f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUserActivityContentInfoStatics_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub FromJson: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IUserActivityFactory(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IUserActivityFactory {
    type Vtable = IUserActivityFactory_Vtbl;
}
unsafe impl ::windows::core::Interface for IUserActivityFactory {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7c385758_361d_4a67_8a3b_34ca2978f9a3);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUserActivityFactory_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub CreateWithActivityId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, activityid: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IUserActivityRequest(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IUserActivityRequest {
    type Vtable = IUserActivityRequest_Vtbl;
}
unsafe impl ::windows::core::Interface for IUserActivityRequest {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa0ef6355_cf35_4ff0_8833_50cb4b72e06d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUserActivityRequest_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub SetUserActivity: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, activity: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IUserActivityRequestManager(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IUserActivityRequestManager {
    type Vtable = IUserActivityRequestManager_Vtbl;
}
unsafe impl ::windows::core::Interface for IUserActivityRequestManager {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0c30be4e_903d_48d6_82d4_4043ed57791b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUserActivityRequestManager_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub UserActivityRequested: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    UserActivityRequested: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveUserActivityRequested: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveUserActivityRequested: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IUserActivityRequestManagerStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IUserActivityRequestManagerStatics {
    type Vtable = IUserActivityRequestManagerStatics_Vtbl;
}
unsafe impl ::windows::core::Interface for IUserActivityRequestManagerStatics {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc0392df1_224a_432c_81e5_0c76b4c4cefa);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUserActivityRequestManagerStatics_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub GetForCurrentView: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IUserActivityRequestedEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IUserActivityRequestedEventArgs {
    type Vtable = IUserActivityRequestedEventArgs_Vtbl;
}
unsafe impl ::windows::core::Interface for IUserActivityRequestedEventArgs {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa4cc7a4c_8229_4cfd_a3bc_c61d318575a4);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUserActivityRequestedEventArgs_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Request: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub GetDeferral: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetDeferral: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IUserActivitySession(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IUserActivitySession {
    type Vtable = IUserActivitySession_Vtbl;
}
unsafe impl ::windows::core::Interface for IUserActivitySession {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xae434d78_24fa_44a3_ad48_6eda61aa1924);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUserActivitySession_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub ActivityId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IUserActivitySessionHistoryItem(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IUserActivitySessionHistoryItem {
    type Vtable = IUserActivitySessionHistoryItem_Vtbl;
}
unsafe impl ::windows::core::Interface for IUserActivitySessionHistoryItem {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe8d59bd3_3e5d_49fd_98d7_6da97521e255);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUserActivitySessionHistoryItem_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub UserActivity: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub StartTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::DateTime) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    StartTime: usize,
    #[cfg(feature = "Foundation")]
    pub EndTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    EndTime: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IUserActivityStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IUserActivityStatics {
    type Vtable = IUserActivityStatics_Vtbl;
}
unsafe impl ::windows::core::Interface for IUserActivityStatics {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8c8fd333_0e09_47f6_9ac7_95cf5c39367b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUserActivityStatics_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub TryParseFromJson: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, json: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub TryParseFromJsonArray: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, json: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    TryParseFromJsonArray: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub ToJsonArray: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, activities: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    ToJsonArray: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IUserActivityVisualElements(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IUserActivityVisualElements {
    type Vtable = IUserActivityVisualElements_Vtbl;
}
unsafe impl ::windows::core::Interface for IUserActivityVisualElements {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x94757513_262f_49ef_bbbf_9b75d2e85250);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUserActivityVisualElements_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub DisplayText: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetDisplayText: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Description: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetDescription: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "UI")]
    pub BackgroundColor: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::UI::Color) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI"))]
    BackgroundColor: usize,
    #[cfg(feature = "UI")]
    pub SetBackgroundColor: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::super::UI::Color) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI"))]
    SetBackgroundColor: usize,
    pub Attribution: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetAttribution: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "UI_Shell")]
    pub SetContent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Shell"))]
    SetContent: usize,
    #[cfg(feature = "UI_Shell")]
    pub Content: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Shell"))]
    Content: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IUserActivityVisualElements2(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IUserActivityVisualElements2 {
    type Vtable = IUserActivityVisualElements2_Vtbl;
}
unsafe impl ::windows::core::Interface for IUserActivityVisualElements2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xcaae7fc7_3eef_4359_825c_9d51b9220de3);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUserActivityVisualElements2_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub AttributionDisplayText: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetAttributionDisplayText: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"ApplicationModel_UserActivities\"`*"]
#[repr(transparent)]
pub struct UserActivity(::windows::core::IUnknown);
impl UserActivity {
    pub fn State(&self) -> ::windows::core::Result<UserActivityState> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).State)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn ActivityId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ActivityId)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn VisualElements(&self) -> ::windows::core::Result<UserActivityVisualElements> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).VisualElements)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ContentUri(&self) -> ::windows::core::Result<super::super::Foundation::Uri> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ContentUri)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetContentUri(&self, value: &super::super::Foundation::Uri) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetContentUri)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
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
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn FallbackUri(&self) -> ::windows::core::Result<super::super::Foundation::Uri> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).FallbackUri)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetFallbackUri(&self, value: &super::super::Foundation::Uri) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetFallbackUri)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ActivationUri(&self) -> ::windows::core::Result<super::super::Foundation::Uri> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ActivationUri)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetActivationUri(&self, value: &super::super::Foundation::Uri) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetActivationUri)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn ContentInfo(&self) -> ::windows::core::Result<IUserActivityContentInfo> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ContentInfo)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetContentInfo<P0, E0>(&self, value: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<IUserActivityContentInfo>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetContentInfo)(::windows::core::Vtable::as_raw(this), value.try_into().map_err(|e| e.into())?.abi()).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SaveAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).SaveAsync)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn CreateSession(&self) -> ::windows::core::Result<UserActivitySession> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CreateSession)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn ToJson(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IUserActivity2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ToJson)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn IsRoamable(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IUserActivity3>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).IsRoamable)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetIsRoamable(&self, value: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IUserActivity3>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).SetIsRoamable)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
    pub fn CreateWithActivityId(activityid: &::windows::core::HSTRING) -> ::windows::core::Result<UserActivity> {
        Self::IUserActivityFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CreateWithActivityId)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(activityid), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    pub fn TryParseFromJson(json: &::windows::core::HSTRING) -> ::windows::core::Result<UserActivity> {
        Self::IUserActivityStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).TryParseFromJson)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(json), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn TryParseFromJsonArray(json: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<UserActivity>> {
        Self::IUserActivityStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).TryParseFromJsonArray)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(json), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn ToJsonArray<P0, E0>(activities: P0) -> ::windows::core::Result<::windows::core::HSTRING>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<super::super::Foundation::Collections::IIterable<UserActivity>>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        Self::IUserActivityStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ToJsonArray)(::windows::core::Vtable::as_raw(this), activities.try_into().map_err(|e| e.into())?.abi(), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IUserActivityFactory<R, F: FnOnce(&IUserActivityFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<UserActivity, IUserActivityFactory> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn IUserActivityStatics<R, F: FnOnce(&IUserActivityStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<UserActivity, IUserActivityStatics> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::clone::Clone for UserActivity {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for UserActivity {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for UserActivity {}
impl ::core::fmt::Debug for UserActivity {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UserActivity").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for UserActivity {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.UserActivities.UserActivity;{fc103e9e-2cab-4d36-aea2-b4bb556cef0f})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for UserActivity {
    type Vtable = IUserActivity_Vtbl;
}
unsafe impl ::windows::core::Interface for UserActivity {
    const IID: ::windows::core::GUID = <IUserActivity as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for UserActivity {
    const NAME: &'static str = "Windows.ApplicationModel.UserActivities.UserActivity";
}
::windows::core::interface_hierarchy!(UserActivity, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for UserActivity {}
unsafe impl ::core::marker::Sync for UserActivity {}
#[doc = "*Required features: `\"ApplicationModel_UserActivities\"`*"]
#[repr(transparent)]
pub struct UserActivityAttribution(::windows::core::IUnknown);
impl UserActivityAttribution {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IGenericFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<UserActivityAttribution, ::windows::core::IGenericFactory> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn IconUri(&self) -> ::windows::core::Result<super::super::Foundation::Uri> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).IconUri)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetIconUri(&self, value: &super::super::Foundation::Uri) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetIconUri)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn AlternateText(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).AlternateText)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetAlternateText(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetAlternateText)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn AddImageQuery(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).AddImageQuery)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetAddImageQuery(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetAddImageQuery)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn CreateWithUri(iconuri: &super::super::Foundation::Uri) -> ::windows::core::Result<UserActivityAttribution> {
        Self::IUserActivityAttributionFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CreateWithUri)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(iconuri), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IUserActivityAttributionFactory<R, F: FnOnce(&IUserActivityAttributionFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<UserActivityAttribution, IUserActivityAttributionFactory> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::clone::Clone for UserActivityAttribution {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for UserActivityAttribution {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for UserActivityAttribution {}
impl ::core::fmt::Debug for UserActivityAttribution {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UserActivityAttribution").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for UserActivityAttribution {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.UserActivities.UserActivityAttribution;{34a5c8b5-86dd-4aec-a491-6a4faea5d22e})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for UserActivityAttribution {
    type Vtable = IUserActivityAttribution_Vtbl;
}
unsafe impl ::windows::core::Interface for UserActivityAttribution {
    const IID: ::windows::core::GUID = <IUserActivityAttribution as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for UserActivityAttribution {
    const NAME: &'static str = "Windows.ApplicationModel.UserActivities.UserActivityAttribution";
}
::windows::core::interface_hierarchy!(UserActivityAttribution, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for UserActivityAttribution {}
unsafe impl ::core::marker::Sync for UserActivityAttribution {}
#[doc = "*Required features: `\"ApplicationModel_UserActivities\"`*"]
#[repr(transparent)]
pub struct UserActivityChannel(::windows::core::IUnknown);
impl UserActivityChannel {
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn GetOrCreateUserActivityAsync(&self, activityid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<UserActivity>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetOrCreateUserActivityAsync)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(activityid), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn DeleteActivityAsync(&self, activityid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).DeleteActivityAsync)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(activityid), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn DeleteAllActivitiesAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).DeleteAllActivitiesAsync)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetRecentUserActivitiesAsync(&self, maxuniqueactivities: i32) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVector<UserActivitySessionHistoryItem>>> {
        let this = &::windows::core::Interface::cast::<IUserActivityChannel2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetRecentUserActivitiesAsync)(::windows::core::Vtable::as_raw(this), maxuniqueactivities, result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetSessionHistoryItemsForUserActivityAsync(&self, activityid: &::windows::core::HSTRING, starttime: super::super::Foundation::DateTime) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVector<UserActivitySessionHistoryItem>>> {
        let this = &::windows::core::Interface::cast::<IUserActivityChannel2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetSessionHistoryItemsForUserActivityAsync)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(activityid), starttime, result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn GetDefault() -> ::windows::core::Result<UserActivityChannel> {
        Self::IUserActivityChannelStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetDefault)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    pub fn DisableAutoSessionCreation() -> ::windows::core::Result<()> {
        Self::IUserActivityChannelStatics2(|this| unsafe { (::windows::core::Vtable::vtable(this).DisableAutoSessionCreation)(::windows::core::Vtable::as_raw(this)).ok() })
    }
    #[doc = "*Required features: `\"Security_Credentials\"`*"]
    #[cfg(feature = "Security_Credentials")]
    pub fn TryGetForWebAccount(account: &super::super::Security::Credentials::WebAccount) -> ::windows::core::Result<UserActivityChannel> {
        Self::IUserActivityChannelStatics2(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).TryGetForWebAccount)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(account), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"System\"`*"]
    #[cfg(feature = "System")]
    pub fn GetForUser(user: &super::super::System::User) -> ::windows::core::Result<UserActivityChannel> {
        Self::IUserActivityChannelStatics3(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetForUser)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(user), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IUserActivityChannelStatics<R, F: FnOnce(&IUserActivityChannelStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<UserActivityChannel, IUserActivityChannelStatics> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn IUserActivityChannelStatics2<R, F: FnOnce(&IUserActivityChannelStatics2) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<UserActivityChannel, IUserActivityChannelStatics2> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn IUserActivityChannelStatics3<R, F: FnOnce(&IUserActivityChannelStatics3) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<UserActivityChannel, IUserActivityChannelStatics3> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::clone::Clone for UserActivityChannel {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for UserActivityChannel {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for UserActivityChannel {}
impl ::core::fmt::Debug for UserActivityChannel {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UserActivityChannel").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for UserActivityChannel {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.UserActivities.UserActivityChannel;{bac0f8b8-a0e4-483b-b948-9cbabd06070c})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for UserActivityChannel {
    type Vtable = IUserActivityChannel_Vtbl;
}
unsafe impl ::windows::core::Interface for UserActivityChannel {
    const IID: ::windows::core::GUID = <IUserActivityChannel as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for UserActivityChannel {
    const NAME: &'static str = "Windows.ApplicationModel.UserActivities.UserActivityChannel";
}
::windows::core::interface_hierarchy!(UserActivityChannel, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for UserActivityChannel {}
unsafe impl ::core::marker::Sync for UserActivityChannel {}
#[doc = "*Required features: `\"ApplicationModel_UserActivities\"`*"]
#[repr(transparent)]
pub struct UserActivityContentInfo(::windows::core::IUnknown);
impl UserActivityContentInfo {
    pub fn ToJson(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ToJson)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn FromJson(value: &::windows::core::HSTRING) -> ::windows::core::Result<UserActivityContentInfo> {
        Self::IUserActivityContentInfoStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).FromJson)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(value), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IUserActivityContentInfoStatics<R, F: FnOnce(&IUserActivityContentInfoStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<UserActivityContentInfo, IUserActivityContentInfoStatics> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::clone::Clone for UserActivityContentInfo {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for UserActivityContentInfo {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for UserActivityContentInfo {}
impl ::core::fmt::Debug for UserActivityContentInfo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UserActivityContentInfo").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for UserActivityContentInfo {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.UserActivities.UserActivityContentInfo;{b399e5ad-137f-409d-822d-e1af27ce08dc})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for UserActivityContentInfo {
    type Vtable = IUserActivityContentInfo_Vtbl;
}
unsafe impl ::windows::core::Interface for UserActivityContentInfo {
    const IID: ::windows::core::GUID = <IUserActivityContentInfo as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for UserActivityContentInfo {
    const NAME: &'static str = "Windows.ApplicationModel.UserActivities.UserActivityContentInfo";
}
::windows::core::interface_hierarchy!(UserActivityContentInfo, ::windows::core::IUnknown, ::windows::core::IInspectable);
impl ::core::convert::TryFrom<UserActivityContentInfo> for IUserActivityContentInfo {
    type Error = ::windows::core::Error;
    fn try_from(value: UserActivityContentInfo) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&UserActivityContentInfo> for IUserActivityContentInfo {
    type Error = ::windows::core::Error;
    fn try_from(value: &UserActivityContentInfo) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl ::core::convert::TryFrom<&UserActivityContentInfo> for ::windows::core::InParam<IUserActivityContentInfo> {
    type Error = ::windows::core::Error;
    fn try_from(value: &UserActivityContentInfo) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
unsafe impl ::core::marker::Send for UserActivityContentInfo {}
unsafe impl ::core::marker::Sync for UserActivityContentInfo {}
#[doc = "*Required features: `\"ApplicationModel_UserActivities\"`*"]
#[repr(transparent)]
pub struct UserActivityRequest(::windows::core::IUnknown);
impl UserActivityRequest {
    pub fn SetUserActivity(&self, activity: &UserActivity) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetUserActivity)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(activity)).ok() }
    }
}
impl ::core::clone::Clone for UserActivityRequest {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for UserActivityRequest {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for UserActivityRequest {}
impl ::core::fmt::Debug for UserActivityRequest {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UserActivityRequest").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for UserActivityRequest {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.UserActivities.UserActivityRequest;{a0ef6355-cf35-4ff0-8833-50cb4b72e06d})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for UserActivityRequest {
    type Vtable = IUserActivityRequest_Vtbl;
}
unsafe impl ::windows::core::Interface for UserActivityRequest {
    const IID: ::windows::core::GUID = <IUserActivityRequest as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for UserActivityRequest {
    const NAME: &'static str = "Windows.ApplicationModel.UserActivities.UserActivityRequest";
}
::windows::core::interface_hierarchy!(UserActivityRequest, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for UserActivityRequest {}
unsafe impl ::core::marker::Sync for UserActivityRequest {}
#[doc = "*Required features: `\"ApplicationModel_UserActivities\"`*"]
#[repr(transparent)]
pub struct UserActivityRequestManager(::windows::core::IUnknown);
impl UserActivityRequestManager {
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn UserActivityRequested(&self, handler: &super::super::Foundation::TypedEventHandler<UserActivityRequestManager, UserActivityRequestedEventArgs>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).UserActivityRequested)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(handler), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveUserActivityRequested(&self, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).RemoveUserActivityRequested)(::windows::core::Vtable::as_raw(this), token).ok() }
    }
    pub fn GetForCurrentView() -> ::windows::core::Result<UserActivityRequestManager> {
        Self::IUserActivityRequestManagerStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetForCurrentView)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IUserActivityRequestManagerStatics<R, F: FnOnce(&IUserActivityRequestManagerStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<UserActivityRequestManager, IUserActivityRequestManagerStatics> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::clone::Clone for UserActivityRequestManager {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for UserActivityRequestManager {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for UserActivityRequestManager {}
impl ::core::fmt::Debug for UserActivityRequestManager {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UserActivityRequestManager").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for UserActivityRequestManager {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.UserActivities.UserActivityRequestManager;{0c30be4e-903d-48d6-82d4-4043ed57791b})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for UserActivityRequestManager {
    type Vtable = IUserActivityRequestManager_Vtbl;
}
unsafe impl ::windows::core::Interface for UserActivityRequestManager {
    const IID: ::windows::core::GUID = <IUserActivityRequestManager as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for UserActivityRequestManager {
    const NAME: &'static str = "Windows.ApplicationModel.UserActivities.UserActivityRequestManager";
}
::windows::core::interface_hierarchy!(UserActivityRequestManager, ::windows::core::IUnknown, ::windows::core::IInspectable);
#[doc = "*Required features: `\"ApplicationModel_UserActivities\"`*"]
#[repr(transparent)]
pub struct UserActivityRequestedEventArgs(::windows::core::IUnknown);
impl UserActivityRequestedEventArgs {
    pub fn Request(&self) -> ::windows::core::Result<UserActivityRequest> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Request)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn GetDeferral(&self) -> ::windows::core::Result<super::super::Foundation::Deferral> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetDeferral)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
}
impl ::core::clone::Clone for UserActivityRequestedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for UserActivityRequestedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for UserActivityRequestedEventArgs {}
impl ::core::fmt::Debug for UserActivityRequestedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UserActivityRequestedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for UserActivityRequestedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.UserActivities.UserActivityRequestedEventArgs;{a4cc7a4c-8229-4cfd-a3bc-c61d318575a4})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for UserActivityRequestedEventArgs {
    type Vtable = IUserActivityRequestedEventArgs_Vtbl;
}
unsafe impl ::windows::core::Interface for UserActivityRequestedEventArgs {
    const IID: ::windows::core::GUID = <IUserActivityRequestedEventArgs as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for UserActivityRequestedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.UserActivities.UserActivityRequestedEventArgs";
}
::windows::core::interface_hierarchy!(UserActivityRequestedEventArgs, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for UserActivityRequestedEventArgs {}
unsafe impl ::core::marker::Sync for UserActivityRequestedEventArgs {}
#[doc = "*Required features: `\"ApplicationModel_UserActivities\"`*"]
#[repr(transparent)]
pub struct UserActivitySession(::windows::core::IUnknown);
impl UserActivitySession {
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Close(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).Close)(::windows::core::Vtable::as_raw(this)).ok() }
    }
    pub fn ActivityId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ActivityId)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
}
impl ::core::clone::Clone for UserActivitySession {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for UserActivitySession {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for UserActivitySession {}
impl ::core::fmt::Debug for UserActivitySession {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UserActivitySession").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for UserActivitySession {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.UserActivities.UserActivitySession;{ae434d78-24fa-44a3-ad48-6eda61aa1924})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for UserActivitySession {
    type Vtable = IUserActivitySession_Vtbl;
}
unsafe impl ::windows::core::Interface for UserActivitySession {
    const IID: ::windows::core::GUID = <IUserActivitySession as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for UserActivitySession {
    const NAME: &'static str = "Windows.ApplicationModel.UserActivities.UserActivitySession";
}
::windows::core::interface_hierarchy!(UserActivitySession, ::windows::core::IUnknown, ::windows::core::IInspectable);
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<UserActivitySession> for super::super::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: UserActivitySession) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<&UserActivitySession> for super::super::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: &UserActivitySession) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<&UserActivitySession> for ::windows::core::InParam<super::super::Foundation::IClosable> {
    type Error = ::windows::core::Error;
    fn try_from(value: &UserActivitySession) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
unsafe impl ::core::marker::Send for UserActivitySession {}
unsafe impl ::core::marker::Sync for UserActivitySession {}
#[doc = "*Required features: `\"ApplicationModel_UserActivities\"`*"]
#[repr(transparent)]
pub struct UserActivitySessionHistoryItem(::windows::core::IUnknown);
impl UserActivitySessionHistoryItem {
    pub fn UserActivity(&self) -> ::windows::core::Result<UserActivity> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).UserActivity)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn StartTime(&self) -> ::windows::core::Result<super::super::Foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).StartTime)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn EndTime(&self) -> ::windows::core::Result<super::super::Foundation::IReference<super::super::Foundation::DateTime>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).EndTime)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
}
impl ::core::clone::Clone for UserActivitySessionHistoryItem {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for UserActivitySessionHistoryItem {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for UserActivitySessionHistoryItem {}
impl ::core::fmt::Debug for UserActivitySessionHistoryItem {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UserActivitySessionHistoryItem").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for UserActivitySessionHistoryItem {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.UserActivities.UserActivitySessionHistoryItem;{e8d59bd3-3e5d-49fd-98d7-6da97521e255})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for UserActivitySessionHistoryItem {
    type Vtable = IUserActivitySessionHistoryItem_Vtbl;
}
unsafe impl ::windows::core::Interface for UserActivitySessionHistoryItem {
    const IID: ::windows::core::GUID = <IUserActivitySessionHistoryItem as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for UserActivitySessionHistoryItem {
    const NAME: &'static str = "Windows.ApplicationModel.UserActivities.UserActivitySessionHistoryItem";
}
::windows::core::interface_hierarchy!(UserActivitySessionHistoryItem, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for UserActivitySessionHistoryItem {}
unsafe impl ::core::marker::Sync for UserActivitySessionHistoryItem {}
#[doc = "*Required features: `\"ApplicationModel_UserActivities\"`*"]
#[repr(transparent)]
pub struct UserActivityVisualElements(::windows::core::IUnknown);
impl UserActivityVisualElements {
    pub fn DisplayText(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).DisplayText)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetDisplayText(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetDisplayText)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn Description(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Description)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetDescription(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetDescription)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    #[doc = "*Required features: `\"UI\"`*"]
    #[cfg(feature = "UI")]
    pub fn BackgroundColor(&self) -> ::windows::core::Result<super::super::UI::Color> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).BackgroundColor)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"UI\"`*"]
    #[cfg(feature = "UI")]
    pub fn SetBackgroundColor(&self, value: super::super::UI::Color) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetBackgroundColor)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
    pub fn Attribution(&self) -> ::windows::core::Result<UserActivityAttribution> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Attribution)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetAttribution(&self, value: &UserActivityAttribution) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetAttribution)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    #[doc = "*Required features: `\"UI_Shell\"`*"]
    #[cfg(feature = "UI_Shell")]
    pub fn SetContent<P0, E0>(&self, value: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<super::super::UI::Shell::IAdaptiveCard>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetContent)(::windows::core::Vtable::as_raw(this), value.try_into().map_err(|e| e.into())?.abi()).ok() }
    }
    #[doc = "*Required features: `\"UI_Shell\"`*"]
    #[cfg(feature = "UI_Shell")]
    pub fn Content(&self) -> ::windows::core::Result<super::super::UI::Shell::IAdaptiveCard> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Content)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn AttributionDisplayText(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IUserActivityVisualElements2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).AttributionDisplayText)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetAttributionDisplayText(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IUserActivityVisualElements2>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).SetAttributionDisplayText)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
}
impl ::core::clone::Clone for UserActivityVisualElements {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for UserActivityVisualElements {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for UserActivityVisualElements {}
impl ::core::fmt::Debug for UserActivityVisualElements {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UserActivityVisualElements").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for UserActivityVisualElements {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.UserActivities.UserActivityVisualElements;{94757513-262f-49ef-bbbf-9b75d2e85250})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for UserActivityVisualElements {
    type Vtable = IUserActivityVisualElements_Vtbl;
}
unsafe impl ::windows::core::Interface for UserActivityVisualElements {
    const IID: ::windows::core::GUID = <IUserActivityVisualElements as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for UserActivityVisualElements {
    const NAME: &'static str = "Windows.ApplicationModel.UserActivities.UserActivityVisualElements";
}
::windows::core::interface_hierarchy!(UserActivityVisualElements, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for UserActivityVisualElements {}
unsafe impl ::core::marker::Sync for UserActivityVisualElements {}
#[doc = "*Required features: `\"ApplicationModel_UserActivities\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct UserActivityState(pub i32);
impl UserActivityState {
    pub const New: Self = Self(0i32);
    pub const Published: Self = Self(1i32);
}
impl ::core::marker::Copy for UserActivityState {}
impl ::core::clone::Clone for UserActivityState {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for UserActivityState {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for UserActivityState {
    type Abi = Self;
}
impl ::core::fmt::Debug for UserActivityState {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UserActivityState").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for UserActivityState {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.UserActivities.UserActivityState;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
