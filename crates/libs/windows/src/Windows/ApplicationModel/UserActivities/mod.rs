#[cfg(feature = "ApplicationModel_UserActivities_Core")]
pub mod Core;
#[doc(hidden)]
#[repr(transparent)]
pub struct IUserActivity(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IUserActivity {
    type Vtable = IUserActivity_Vtbl;
}
impl ::core::clone::Clone for IUserActivity {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IUserActivity {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xfc103e9e_2cab_4d36_aea2_b4bb556cef0f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUserActivity_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub State: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut UserActivityState) -> ::windows::core::HRESULT,
    pub ActivityId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub VisualElements: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub ContentUri: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ContentUri: usize,
    #[cfg(feature = "Foundation")]
    pub SetContentUri: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetContentUri: usize,
    pub ContentType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub SetContentType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
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
unsafe impl ::windows::core::Interface for IUserActivity2 {
    type Vtable = IUserActivity2_Vtbl;
}
impl ::core::clone::Clone for IUserActivity2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IUserActivity2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9dc40c62_08c4_47ac_aa9c_2bb2221c55fd);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUserActivity2_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub ToJson: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IUserActivity3(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IUserActivity3 {
    type Vtable = IUserActivity3_Vtbl;
}
impl ::core::clone::Clone for IUserActivity3 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IUserActivity3 {
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
unsafe impl ::windows::core::Interface for IUserActivityAttribution {
    type Vtable = IUserActivityAttribution_Vtbl;
}
impl ::core::clone::Clone for IUserActivityAttribution {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IUserActivityAttribution {
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
    pub AlternateText: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub SetAlternateText: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub AddImageQuery: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetAddImageQuery: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IUserActivityAttributionFactory(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IUserActivityAttributionFactory {
    type Vtable = IUserActivityAttributionFactory_Vtbl;
}
impl ::core::clone::Clone for IUserActivityAttributionFactory {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IUserActivityAttributionFactory {
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
unsafe impl ::windows::core::Interface for IUserActivityChannel {
    type Vtable = IUserActivityChannel_Vtbl;
}
impl ::core::clone::Clone for IUserActivityChannel {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IUserActivityChannel {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xbac0f8b8_a0e4_483b_b948_9cbabd06070c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUserActivityChannel_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub GetOrCreateUserActivityAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, activityid: ::std::mem::MaybeUninit<::windows::core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetOrCreateUserActivityAsync: usize,
    #[cfg(feature = "Foundation")]
    pub DeleteActivityAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, activityid: ::std::mem::MaybeUninit<::windows::core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
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
unsafe impl ::windows::core::Interface for IUserActivityChannel2 {
    type Vtable = IUserActivityChannel2_Vtbl;
}
impl ::core::clone::Clone for IUserActivityChannel2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IUserActivityChannel2 {
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
    pub GetSessionHistoryItemsForUserActivityAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, activityid: ::std::mem::MaybeUninit<::windows::core::HSTRING>, starttime: super::super::Foundation::DateTime, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetSessionHistoryItemsForUserActivityAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IUserActivityChannelStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IUserActivityChannelStatics {
    type Vtable = IUserActivityChannelStatics_Vtbl;
}
impl ::core::clone::Clone for IUserActivityChannelStatics {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IUserActivityChannelStatics {
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
unsafe impl ::windows::core::Interface for IUserActivityChannelStatics2 {
    type Vtable = IUserActivityChannelStatics2_Vtbl;
}
impl ::core::clone::Clone for IUserActivityChannelStatics2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IUserActivityChannelStatics2 {
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
unsafe impl ::windows::core::Interface for IUserActivityChannelStatics3 {
    type Vtable = IUserActivityChannelStatics3_Vtbl;
}
impl ::core::clone::Clone for IUserActivityChannelStatics3 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IUserActivityChannelStatics3 {
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
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).ToJson)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
::windows::imp::interface_hierarchy!(IUserActivityContentInfo, ::windows::core::IUnknown, ::windows::core::IInspectable);
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
impl ::windows::core::RuntimeType for IUserActivityContentInfo {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"{b399e5ad-137f-409d-822d-e1af27ce08dc}");
}
unsafe impl ::windows::core::Interface for IUserActivityContentInfo {
    type Vtable = IUserActivityContentInfo_Vtbl;
}
impl ::core::clone::Clone for IUserActivityContentInfo {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IUserActivityContentInfo {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb399e5ad_137f_409d_822d_e1af27ce08dc);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUserActivityContentInfo_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub ToJson: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IUserActivityContentInfoStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IUserActivityContentInfoStatics {
    type Vtable = IUserActivityContentInfoStatics_Vtbl;
}
impl ::core::clone::Clone for IUserActivityContentInfoStatics {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IUserActivityContentInfoStatics {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9988c34b_0386_4bc9_968a_8200b004144f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUserActivityContentInfoStatics_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub FromJson: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::std::mem::MaybeUninit<::windows::core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IUserActivityFactory(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IUserActivityFactory {
    type Vtable = IUserActivityFactory_Vtbl;
}
impl ::core::clone::Clone for IUserActivityFactory {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IUserActivityFactory {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7c385758_361d_4a67_8a3b_34ca2978f9a3);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUserActivityFactory_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub CreateWithActivityId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, activityid: ::std::mem::MaybeUninit<::windows::core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IUserActivityRequest(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IUserActivityRequest {
    type Vtable = IUserActivityRequest_Vtbl;
}
impl ::core::clone::Clone for IUserActivityRequest {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IUserActivityRequest {
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
unsafe impl ::windows::core::Interface for IUserActivityRequestManager {
    type Vtable = IUserActivityRequestManager_Vtbl;
}
impl ::core::clone::Clone for IUserActivityRequestManager {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IUserActivityRequestManager {
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
unsafe impl ::windows::core::Interface for IUserActivityRequestManagerStatics {
    type Vtable = IUserActivityRequestManagerStatics_Vtbl;
}
impl ::core::clone::Clone for IUserActivityRequestManagerStatics {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IUserActivityRequestManagerStatics {
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
unsafe impl ::windows::core::Interface for IUserActivityRequestedEventArgs {
    type Vtable = IUserActivityRequestedEventArgs_Vtbl;
}
impl ::core::clone::Clone for IUserActivityRequestedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IUserActivityRequestedEventArgs {
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
unsafe impl ::windows::core::Interface for IUserActivitySession {
    type Vtable = IUserActivitySession_Vtbl;
}
impl ::core::clone::Clone for IUserActivitySession {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IUserActivitySession {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xae434d78_24fa_44a3_ad48_6eda61aa1924);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUserActivitySession_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub ActivityId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IUserActivitySessionHistoryItem(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IUserActivitySessionHistoryItem {
    type Vtable = IUserActivitySessionHistoryItem_Vtbl;
}
impl ::core::clone::Clone for IUserActivitySessionHistoryItem {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IUserActivitySessionHistoryItem {
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
unsafe impl ::windows::core::Interface for IUserActivityStatics {
    type Vtable = IUserActivityStatics_Vtbl;
}
impl ::core::clone::Clone for IUserActivityStatics {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IUserActivityStatics {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8c8fd333_0e09_47f6_9ac7_95cf5c39367b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUserActivityStatics_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub TryParseFromJson: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, json: ::std::mem::MaybeUninit<::windows::core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub TryParseFromJsonArray: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, json: ::std::mem::MaybeUninit<::windows::core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    TryParseFromJsonArray: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub ToJsonArray: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, activities: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    ToJsonArray: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IUserActivityVisualElements(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IUserActivityVisualElements {
    type Vtable = IUserActivityVisualElements_Vtbl;
}
impl ::core::clone::Clone for IUserActivityVisualElements {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IUserActivityVisualElements {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x94757513_262f_49ef_bbbf_9b75d2e85250);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUserActivityVisualElements_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub DisplayText: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub SetDisplayText: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub Description: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub SetDescription: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
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
unsafe impl ::windows::core::Interface for IUserActivityVisualElements2 {
    type Vtable = IUserActivityVisualElements2_Vtbl;
}
impl ::core::clone::Clone for IUserActivityVisualElements2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IUserActivityVisualElements2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xcaae7fc7_3eef_4359_825c_9d51b9220de3);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUserActivityVisualElements2_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub AttributionDisplayText: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub SetAttributionDisplayText: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"ApplicationModel_UserActivities\"`*"]
#[repr(transparent)]
pub struct UserActivity(::windows::core::IUnknown);
impl UserActivity {
    pub fn State(&self) -> ::windows::core::Result<UserActivityState> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<UserActivityState>();
            (::windows::core::Interface::vtable(this).State)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn ActivityId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).ActivityId)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn VisualElements(&self) -> ::windows::core::Result<UserActivityVisualElements> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<UserActivityVisualElements>();
            (::windows::core::Interface::vtable(this).VisualElements)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ContentUri(&self) -> ::windows::core::Result<super::super::Foundation::Uri> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::Uri>();
            (::windows::core::Interface::vtable(this).ContentUri)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetContentUri(&self, value: &super::super::Foundation::Uri) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetContentUri)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn ContentType(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).ContentType)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetContentType(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetContentType)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn FallbackUri(&self) -> ::windows::core::Result<super::super::Foundation::Uri> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::Uri>();
            (::windows::core::Interface::vtable(this).FallbackUri)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetFallbackUri(&self, value: &super::super::Foundation::Uri) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetFallbackUri)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ActivationUri(&self) -> ::windows::core::Result<super::super::Foundation::Uri> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::Uri>();
            (::windows::core::Interface::vtable(this).ActivationUri)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetActivationUri(&self, value: &super::super::Foundation::Uri) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetActivationUri)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn ContentInfo(&self) -> ::windows::core::Result<IUserActivityContentInfo> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<IUserActivityContentInfo>();
            (::windows::core::Interface::vtable(this).ContentInfo)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetContentInfo<P0>(&self, value: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::TryIntoParam<IUserActivityContentInfo>,
    {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetContentInfo)(::windows::core::Interface::as_raw(this), value.try_into_param()?.abi()).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SaveAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::IAsyncAction>();
            (::windows::core::Interface::vtable(this).SaveAsync)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn CreateSession(&self) -> ::windows::core::Result<UserActivitySession> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<UserActivitySession>();
            (::windows::core::Interface::vtable(this).CreateSession)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn ToJson(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::ComInterface::cast::<IUserActivity2>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).ToJson)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn IsRoamable(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::ComInterface::cast::<IUserActivity3>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<bool>();
            (::windows::core::Interface::vtable(this).IsRoamable)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetIsRoamable(&self, value: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::ComInterface::cast::<IUserActivity3>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetIsRoamable)(::windows::core::Interface::as_raw(this), value).ok() }
    }
    pub fn CreateWithActivityId(activityid: &::windows::core::HSTRING) -> ::windows::core::Result<UserActivity> {
        Self::IUserActivityFactory(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<UserActivity>();
            (::windows::core::Interface::vtable(this).CreateWithActivityId)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(activityid), &mut result__).from_abi(result__)
        })
    }
    pub fn TryParseFromJson(json: &::windows::core::HSTRING) -> ::windows::core::Result<UserActivity> {
        Self::IUserActivityStatics(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<UserActivity>();
            (::windows::core::Interface::vtable(this).TryParseFromJson)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(json), &mut result__).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn TryParseFromJsonArray(json: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<UserActivity>> {
        Self::IUserActivityStatics(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::Collections::IVector<UserActivity>>();
            (::windows::core::Interface::vtable(this).TryParseFromJsonArray)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(json), &mut result__).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn ToJsonArray<P0>(activities: P0) -> ::windows::core::Result<::windows::core::HSTRING>
    where
        P0: ::windows::core::TryIntoParam<super::super::Foundation::Collections::IIterable<UserActivity>>,
    {
        Self::IUserActivityStatics(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).ToJsonArray)(::windows::core::Interface::as_raw(this), activities.try_into_param()?.abi(), &mut result__).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IUserActivityFactory<R, F: FnOnce(&IUserActivityFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::imp::FactoryCache<UserActivity, IUserActivityFactory> = ::windows::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn IUserActivityStatics<R, F: FnOnce(&IUserActivityStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::imp::FactoryCache<UserActivity, IUserActivityStatics> = ::windows::imp::FactoryCache::new();
        SHARED.call(callback)
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
impl ::windows::core::RuntimeType for UserActivity {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.UserActivities.UserActivity;{fc103e9e-2cab-4d36-aea2-b4bb556cef0f})");
}
impl ::core::clone::Clone for UserActivity {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for UserActivity {
    type Vtable = IUserActivity_Vtbl;
}
unsafe impl ::windows::core::ComInterface for UserActivity {
    const IID: ::windows::core::GUID = <IUserActivity as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for UserActivity {
    const NAME: &'static str = "Windows.ApplicationModel.UserActivities.UserActivity";
}
::windows::imp::interface_hierarchy!(UserActivity, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for UserActivity {}
unsafe impl ::core::marker::Sync for UserActivity {}
#[doc = "*Required features: `\"ApplicationModel_UserActivities\"`*"]
#[repr(transparent)]
pub struct UserActivityAttribution(::windows::core::IUnknown);
impl UserActivityAttribution {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::imp::IGenericFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::imp::FactoryCache<UserActivityAttribution, ::windows::imp::IGenericFactory> = ::windows::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn IconUri(&self) -> ::windows::core::Result<super::super::Foundation::Uri> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::Uri>();
            (::windows::core::Interface::vtable(this).IconUri)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetIconUri(&self, value: &super::super::Foundation::Uri) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetIconUri)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn AlternateText(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).AlternateText)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetAlternateText(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetAlternateText)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn AddImageQuery(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<bool>();
            (::windows::core::Interface::vtable(this).AddImageQuery)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetAddImageQuery(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetAddImageQuery)(::windows::core::Interface::as_raw(this), value).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn CreateWithUri(iconuri: &super::super::Foundation::Uri) -> ::windows::core::Result<UserActivityAttribution> {
        Self::IUserActivityAttributionFactory(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<UserActivityAttribution>();
            (::windows::core::Interface::vtable(this).CreateWithUri)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(iconuri), &mut result__).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IUserActivityAttributionFactory<R, F: FnOnce(&IUserActivityAttributionFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::imp::FactoryCache<UserActivityAttribution, IUserActivityAttributionFactory> = ::windows::imp::FactoryCache::new();
        SHARED.call(callback)
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
impl ::windows::core::RuntimeType for UserActivityAttribution {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.UserActivities.UserActivityAttribution;{34a5c8b5-86dd-4aec-a491-6a4faea5d22e})");
}
impl ::core::clone::Clone for UserActivityAttribution {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for UserActivityAttribution {
    type Vtable = IUserActivityAttribution_Vtbl;
}
unsafe impl ::windows::core::ComInterface for UserActivityAttribution {
    const IID: ::windows::core::GUID = <IUserActivityAttribution as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for UserActivityAttribution {
    const NAME: &'static str = "Windows.ApplicationModel.UserActivities.UserActivityAttribution";
}
::windows::imp::interface_hierarchy!(UserActivityAttribution, ::windows::core::IUnknown, ::windows::core::IInspectable);
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
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::IAsyncOperation<UserActivity>>();
            (::windows::core::Interface::vtable(this).GetOrCreateUserActivityAsync)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(activityid), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn DeleteActivityAsync(&self, activityid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::IAsyncAction>();
            (::windows::core::Interface::vtable(this).DeleteActivityAsync)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(activityid), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn DeleteAllActivitiesAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::IAsyncAction>();
            (::windows::core::Interface::vtable(this).DeleteAllActivitiesAsync)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetRecentUserActivitiesAsync(&self, maxuniqueactivities: i32) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVector<UserActivitySessionHistoryItem>>> {
        let this = &::windows::core::ComInterface::cast::<IUserActivityChannel2>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVector<UserActivitySessionHistoryItem>>>();
            (::windows::core::Interface::vtable(this).GetRecentUserActivitiesAsync)(::windows::core::Interface::as_raw(this), maxuniqueactivities, &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetSessionHistoryItemsForUserActivityAsync(&self, activityid: &::windows::core::HSTRING, starttime: super::super::Foundation::DateTime) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVector<UserActivitySessionHistoryItem>>> {
        let this = &::windows::core::ComInterface::cast::<IUserActivityChannel2>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVector<UserActivitySessionHistoryItem>>>();
            (::windows::core::Interface::vtable(this).GetSessionHistoryItemsForUserActivityAsync)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(activityid), starttime, &mut result__).from_abi(result__)
        }
    }
    pub fn GetDefault() -> ::windows::core::Result<UserActivityChannel> {
        Self::IUserActivityChannelStatics(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<UserActivityChannel>();
            (::windows::core::Interface::vtable(this).GetDefault)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn DisableAutoSessionCreation() -> ::windows::core::Result<()> {
        Self::IUserActivityChannelStatics2(|this| unsafe { (::windows::core::Interface::vtable(this).DisableAutoSessionCreation)(::windows::core::Interface::as_raw(this)).ok() })
    }
    #[doc = "*Required features: `\"Security_Credentials\"`*"]
    #[cfg(feature = "Security_Credentials")]
    pub fn TryGetForWebAccount(account: &super::super::Security::Credentials::WebAccount) -> ::windows::core::Result<UserActivityChannel> {
        Self::IUserActivityChannelStatics2(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<UserActivityChannel>();
            (::windows::core::Interface::vtable(this).TryGetForWebAccount)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(account), &mut result__).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"System\"`*"]
    #[cfg(feature = "System")]
    pub fn GetForUser(user: &super::super::System::User) -> ::windows::core::Result<UserActivityChannel> {
        Self::IUserActivityChannelStatics3(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<UserActivityChannel>();
            (::windows::core::Interface::vtable(this).GetForUser)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(user), &mut result__).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IUserActivityChannelStatics<R, F: FnOnce(&IUserActivityChannelStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::imp::FactoryCache<UserActivityChannel, IUserActivityChannelStatics> = ::windows::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn IUserActivityChannelStatics2<R, F: FnOnce(&IUserActivityChannelStatics2) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::imp::FactoryCache<UserActivityChannel, IUserActivityChannelStatics2> = ::windows::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn IUserActivityChannelStatics3<R, F: FnOnce(&IUserActivityChannelStatics3) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::imp::FactoryCache<UserActivityChannel, IUserActivityChannelStatics3> = ::windows::imp::FactoryCache::new();
        SHARED.call(callback)
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
impl ::windows::core::RuntimeType for UserActivityChannel {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.UserActivities.UserActivityChannel;{bac0f8b8-a0e4-483b-b948-9cbabd06070c})");
}
impl ::core::clone::Clone for UserActivityChannel {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for UserActivityChannel {
    type Vtable = IUserActivityChannel_Vtbl;
}
unsafe impl ::windows::core::ComInterface for UserActivityChannel {
    const IID: ::windows::core::GUID = <IUserActivityChannel as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for UserActivityChannel {
    const NAME: &'static str = "Windows.ApplicationModel.UserActivities.UserActivityChannel";
}
::windows::imp::interface_hierarchy!(UserActivityChannel, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for UserActivityChannel {}
unsafe impl ::core::marker::Sync for UserActivityChannel {}
#[doc = "*Required features: `\"ApplicationModel_UserActivities\"`*"]
#[repr(transparent)]
pub struct UserActivityContentInfo(::windows::core::IUnknown);
impl UserActivityContentInfo {
    pub fn ToJson(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).ToJson)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn FromJson(value: &::windows::core::HSTRING) -> ::windows::core::Result<UserActivityContentInfo> {
        Self::IUserActivityContentInfoStatics(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<UserActivityContentInfo>();
            (::windows::core::Interface::vtable(this).FromJson)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(value), &mut result__).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IUserActivityContentInfoStatics<R, F: FnOnce(&IUserActivityContentInfoStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::imp::FactoryCache<UserActivityContentInfo, IUserActivityContentInfoStatics> = ::windows::imp::FactoryCache::new();
        SHARED.call(callback)
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
impl ::windows::core::RuntimeType for UserActivityContentInfo {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.UserActivities.UserActivityContentInfo;{b399e5ad-137f-409d-822d-e1af27ce08dc})");
}
impl ::core::clone::Clone for UserActivityContentInfo {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for UserActivityContentInfo {
    type Vtable = IUserActivityContentInfo_Vtbl;
}
unsafe impl ::windows::core::ComInterface for UserActivityContentInfo {
    const IID: ::windows::core::GUID = <IUserActivityContentInfo as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for UserActivityContentInfo {
    const NAME: &'static str = "Windows.ApplicationModel.UserActivities.UserActivityContentInfo";
}
::windows::imp::interface_hierarchy!(UserActivityContentInfo, ::windows::core::IUnknown, ::windows::core::IInspectable);
impl ::windows::core::CanTryInto<IUserActivityContentInfo> for UserActivityContentInfo {}
unsafe impl ::core::marker::Send for UserActivityContentInfo {}
unsafe impl ::core::marker::Sync for UserActivityContentInfo {}
#[doc = "*Required features: `\"ApplicationModel_UserActivities\"`*"]
#[repr(transparent)]
pub struct UserActivityRequest(::windows::core::IUnknown);
impl UserActivityRequest {
    pub fn SetUserActivity(&self, activity: &UserActivity) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetUserActivity)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(activity)).ok() }
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
impl ::windows::core::RuntimeType for UserActivityRequest {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.UserActivities.UserActivityRequest;{a0ef6355-cf35-4ff0-8833-50cb4b72e06d})");
}
impl ::core::clone::Clone for UserActivityRequest {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for UserActivityRequest {
    type Vtable = IUserActivityRequest_Vtbl;
}
unsafe impl ::windows::core::ComInterface for UserActivityRequest {
    const IID: ::windows::core::GUID = <IUserActivityRequest as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for UserActivityRequest {
    const NAME: &'static str = "Windows.ApplicationModel.UserActivities.UserActivityRequest";
}
::windows::imp::interface_hierarchy!(UserActivityRequest, ::windows::core::IUnknown, ::windows::core::IInspectable);
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
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::EventRegistrationToken>();
            (::windows::core::Interface::vtable(this).UserActivityRequested)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(handler), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveUserActivityRequested(&self, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemoveUserActivityRequested)(::windows::core::Interface::as_raw(this), token).ok() }
    }
    pub fn GetForCurrentView() -> ::windows::core::Result<UserActivityRequestManager> {
        Self::IUserActivityRequestManagerStatics(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<UserActivityRequestManager>();
            (::windows::core::Interface::vtable(this).GetForCurrentView)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IUserActivityRequestManagerStatics<R, F: FnOnce(&IUserActivityRequestManagerStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::imp::FactoryCache<UserActivityRequestManager, IUserActivityRequestManagerStatics> = ::windows::imp::FactoryCache::new();
        SHARED.call(callback)
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
impl ::windows::core::RuntimeType for UserActivityRequestManager {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.UserActivities.UserActivityRequestManager;{0c30be4e-903d-48d6-82d4-4043ed57791b})");
}
impl ::core::clone::Clone for UserActivityRequestManager {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for UserActivityRequestManager {
    type Vtable = IUserActivityRequestManager_Vtbl;
}
unsafe impl ::windows::core::ComInterface for UserActivityRequestManager {
    const IID: ::windows::core::GUID = <IUserActivityRequestManager as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for UserActivityRequestManager {
    const NAME: &'static str = "Windows.ApplicationModel.UserActivities.UserActivityRequestManager";
}
::windows::imp::interface_hierarchy!(UserActivityRequestManager, ::windows::core::IUnknown, ::windows::core::IInspectable);
#[doc = "*Required features: `\"ApplicationModel_UserActivities\"`*"]
#[repr(transparent)]
pub struct UserActivityRequestedEventArgs(::windows::core::IUnknown);
impl UserActivityRequestedEventArgs {
    pub fn Request(&self) -> ::windows::core::Result<UserActivityRequest> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<UserActivityRequest>();
            (::windows::core::Interface::vtable(this).Request)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn GetDeferral(&self) -> ::windows::core::Result<super::super::Foundation::Deferral> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::Deferral>();
            (::windows::core::Interface::vtable(this).GetDeferral)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
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
impl ::windows::core::RuntimeType for UserActivityRequestedEventArgs {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.UserActivities.UserActivityRequestedEventArgs;{a4cc7a4c-8229-4cfd-a3bc-c61d318575a4})");
}
impl ::core::clone::Clone for UserActivityRequestedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for UserActivityRequestedEventArgs {
    type Vtable = IUserActivityRequestedEventArgs_Vtbl;
}
unsafe impl ::windows::core::ComInterface for UserActivityRequestedEventArgs {
    const IID: ::windows::core::GUID = <IUserActivityRequestedEventArgs as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for UserActivityRequestedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.UserActivities.UserActivityRequestedEventArgs";
}
::windows::imp::interface_hierarchy!(UserActivityRequestedEventArgs, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for UserActivityRequestedEventArgs {}
unsafe impl ::core::marker::Sync for UserActivityRequestedEventArgs {}
#[doc = "*Required features: `\"ApplicationModel_UserActivities\"`*"]
#[repr(transparent)]
pub struct UserActivitySession(::windows::core::IUnknown);
impl UserActivitySession {
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Close(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::ComInterface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).Close)(::windows::core::Interface::as_raw(this)).ok() }
    }
    pub fn ActivityId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).ActivityId)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
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
impl ::windows::core::RuntimeType for UserActivitySession {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.UserActivities.UserActivitySession;{ae434d78-24fa-44a3-ad48-6eda61aa1924})");
}
impl ::core::clone::Clone for UserActivitySession {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for UserActivitySession {
    type Vtable = IUserActivitySession_Vtbl;
}
unsafe impl ::windows::core::ComInterface for UserActivitySession {
    const IID: ::windows::core::GUID = <IUserActivitySession as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for UserActivitySession {
    const NAME: &'static str = "Windows.ApplicationModel.UserActivities.UserActivitySession";
}
::windows::imp::interface_hierarchy!(UserActivitySession, ::windows::core::IUnknown, ::windows::core::IInspectable);
#[cfg(feature = "Foundation")]
impl ::windows::core::CanTryInto<super::super::Foundation::IClosable> for UserActivitySession {}
unsafe impl ::core::marker::Send for UserActivitySession {}
unsafe impl ::core::marker::Sync for UserActivitySession {}
#[doc = "*Required features: `\"ApplicationModel_UserActivities\"`*"]
#[repr(transparent)]
pub struct UserActivitySessionHistoryItem(::windows::core::IUnknown);
impl UserActivitySessionHistoryItem {
    pub fn UserActivity(&self) -> ::windows::core::Result<UserActivity> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<UserActivity>();
            (::windows::core::Interface::vtable(this).UserActivity)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn StartTime(&self) -> ::windows::core::Result<super::super::Foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::DateTime>();
            (::windows::core::Interface::vtable(this).StartTime)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn EndTime(&self) -> ::windows::core::Result<super::super::Foundation::IReference<super::super::Foundation::DateTime>> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::IReference<super::super::Foundation::DateTime>>();
            (::windows::core::Interface::vtable(this).EndTime)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
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
impl ::windows::core::RuntimeType for UserActivitySessionHistoryItem {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.UserActivities.UserActivitySessionHistoryItem;{e8d59bd3-3e5d-49fd-98d7-6da97521e255})");
}
impl ::core::clone::Clone for UserActivitySessionHistoryItem {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for UserActivitySessionHistoryItem {
    type Vtable = IUserActivitySessionHistoryItem_Vtbl;
}
unsafe impl ::windows::core::ComInterface for UserActivitySessionHistoryItem {
    const IID: ::windows::core::GUID = <IUserActivitySessionHistoryItem as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for UserActivitySessionHistoryItem {
    const NAME: &'static str = "Windows.ApplicationModel.UserActivities.UserActivitySessionHistoryItem";
}
::windows::imp::interface_hierarchy!(UserActivitySessionHistoryItem, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for UserActivitySessionHistoryItem {}
unsafe impl ::core::marker::Sync for UserActivitySessionHistoryItem {}
#[doc = "*Required features: `\"ApplicationModel_UserActivities\"`*"]
#[repr(transparent)]
pub struct UserActivityVisualElements(::windows::core::IUnknown);
impl UserActivityVisualElements {
    pub fn DisplayText(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).DisplayText)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetDisplayText(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetDisplayText)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn Description(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).Description)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetDescription(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetDescription)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    #[doc = "*Required features: `\"UI\"`*"]
    #[cfg(feature = "UI")]
    pub fn BackgroundColor(&self) -> ::windows::core::Result<super::super::UI::Color> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::UI::Color>();
            (::windows::core::Interface::vtable(this).BackgroundColor)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"UI\"`*"]
    #[cfg(feature = "UI")]
    pub fn SetBackgroundColor(&self, value: super::super::UI::Color) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetBackgroundColor)(::windows::core::Interface::as_raw(this), value).ok() }
    }
    pub fn Attribution(&self) -> ::windows::core::Result<UserActivityAttribution> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<UserActivityAttribution>();
            (::windows::core::Interface::vtable(this).Attribution)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetAttribution(&self, value: &UserActivityAttribution) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetAttribution)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    #[doc = "*Required features: `\"UI_Shell\"`*"]
    #[cfg(feature = "UI_Shell")]
    pub fn SetContent<P0>(&self, value: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::TryIntoParam<super::super::UI::Shell::IAdaptiveCard>,
    {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetContent)(::windows::core::Interface::as_raw(this), value.try_into_param()?.abi()).ok() }
    }
    #[doc = "*Required features: `\"UI_Shell\"`*"]
    #[cfg(feature = "UI_Shell")]
    pub fn Content(&self) -> ::windows::core::Result<super::super::UI::Shell::IAdaptiveCard> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::UI::Shell::IAdaptiveCard>();
            (::windows::core::Interface::vtable(this).Content)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn AttributionDisplayText(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::ComInterface::cast::<IUserActivityVisualElements2>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).AttributionDisplayText)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetAttributionDisplayText(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = &::windows::core::ComInterface::cast::<IUserActivityVisualElements2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetAttributionDisplayText)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
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
impl ::windows::core::RuntimeType for UserActivityVisualElements {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.UserActivities.UserActivityVisualElements;{94757513-262f-49ef-bbbf-9b75d2e85250})");
}
impl ::core::clone::Clone for UserActivityVisualElements {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for UserActivityVisualElements {
    type Vtable = IUserActivityVisualElements_Vtbl;
}
unsafe impl ::windows::core::ComInterface for UserActivityVisualElements {
    const IID: ::windows::core::GUID = <IUserActivityVisualElements as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for UserActivityVisualElements {
    const NAME: &'static str = "Windows.ApplicationModel.UserActivities.UserActivityVisualElements";
}
::windows::imp::interface_hierarchy!(UserActivityVisualElements, ::windows::core::IUnknown, ::windows::core::IInspectable);
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
impl ::windows::core::TypeKind for UserActivityState {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for UserActivityState {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UserActivityState").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for UserActivityState {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.UserActivities.UserActivityState;i4)");
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
