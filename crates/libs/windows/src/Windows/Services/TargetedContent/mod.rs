#[doc(hidden)]
#[repr(transparent)]
pub struct ITargetedContentAction(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ITargetedContentAction {
    type Vtable = ITargetedContentAction_Vtbl;
}
impl ::core::clone::Clone for ITargetedContentAction {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for ITargetedContentAction {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xd75b691e_6cd6_4ca0_9d8f_4728b0b7e6b6);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITargetedContentAction_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub InvokeAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    InvokeAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ITargetedContentAvailabilityChangedEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ITargetedContentAvailabilityChangedEventArgs {
    type Vtable = ITargetedContentAvailabilityChangedEventArgs_Vtbl;
}
impl ::core::clone::Clone for ITargetedContentAvailabilityChangedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for ITargetedContentAvailabilityChangedEventArgs {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xe0f59d26_5927_4450_965c_1ceb7becde65);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITargetedContentAvailabilityChangedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub GetDeferral: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetDeferral: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ITargetedContentChangedEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ITargetedContentChangedEventArgs {
    type Vtable = ITargetedContentChangedEventArgs_Vtbl;
}
impl ::core::clone::Clone for ITargetedContentChangedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for ITargetedContentChangedEventArgs {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x99d488c9_587e_4586_8ef7_b54ca9453a16);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITargetedContentChangedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub GetDeferral: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetDeferral: usize,
    pub HasPreviousContentExpired: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ITargetedContentCollection(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ITargetedContentCollection {
    type Vtable = ITargetedContentCollection_Vtbl;
}
impl ::core::clone::Clone for ITargetedContentCollection {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for ITargetedContentCollection {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x2d4b66c5_f163_44ba_9f6e_e1a4c2bb559d);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITargetedContentCollection_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Id: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub ReportInteraction: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, interaction: TargetedContentInteraction) -> ::windows_core::HRESULT,
    pub ReportCustomInteraction: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, custominteractionname: ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub Path: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub Properties: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Properties: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub Collections: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Collections: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub Items: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Items: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ITargetedContentContainer(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ITargetedContentContainer {
    type Vtable = ITargetedContentContainer_Vtbl;
}
impl ::core::clone::Clone for ITargetedContentContainer {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for ITargetedContentContainer {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xbc2494c9_8837_47c2_850f_d79d64595926);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITargetedContentContainer_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Id: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub Timestamp: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::DateTime) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Timestamp: usize,
    pub Availability: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut TargetedContentAvailability) -> ::windows_core::HRESULT,
    pub Content: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub SelectSingleObject: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, path: ::std::mem::MaybeUninit<::windows_core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ITargetedContentContainerStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ITargetedContentContainerStatics {
    type Vtable = ITargetedContentContainerStatics_Vtbl;
}
impl ::core::clone::Clone for ITargetedContentContainerStatics {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for ITargetedContentContainerStatics {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x5b47e7fb_2140_4c1f_a736_c59583f227d8);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITargetedContentContainerStatics_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub GetAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, contentid: ::std::mem::MaybeUninit<::windows_core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ITargetedContentImage(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ITargetedContentImage {
    type Vtable = ITargetedContentImage_Vtbl;
}
impl ::core::clone::Clone for ITargetedContentImage {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for ITargetedContentImage {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xa7a585d9_779f_4b1e_bbb1_8eaf53fbeab2);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITargetedContentImage_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Height: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
    pub Width: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ITargetedContentItem(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ITargetedContentItem {
    type Vtable = ITargetedContentItem_Vtbl;
}
impl ::core::clone::Clone for ITargetedContentItem {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for ITargetedContentItem {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x38168dc4_276c_4c32_96ba_565c6e406e74);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITargetedContentItem_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Path: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub ReportInteraction: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, interaction: TargetedContentInteraction) -> ::windows_core::HRESULT,
    pub ReportCustomInteraction: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, custominteractionname: ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub State: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub Properties: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Properties: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub Collections: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Collections: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ITargetedContentItemState(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ITargetedContentItemState {
    type Vtable = ITargetedContentItemState_Vtbl;
}
impl ::core::clone::Clone for ITargetedContentItemState {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for ITargetedContentItemState {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x73935454_4c65_4b47_a441_472de53c79b6);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITargetedContentItemState_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub ShouldDisplay: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub AppInstallationState: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut TargetedContentAppInstallationState) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ITargetedContentObject(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ITargetedContentObject {
    type Vtable = ITargetedContentObject_Vtbl;
}
impl ::core::clone::Clone for ITargetedContentObject {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for ITargetedContentObject {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x041d7969_2212_42d1_9dfa_88a8e3033aa3);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITargetedContentObject_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub ObjectKind: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut TargetedContentObjectKind) -> ::windows_core::HRESULT,
    pub Collection: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub Item: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub Value: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ITargetedContentStateChangedEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ITargetedContentStateChangedEventArgs {
    type Vtable = ITargetedContentStateChangedEventArgs_Vtbl;
}
impl ::core::clone::Clone for ITargetedContentStateChangedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for ITargetedContentStateChangedEventArgs {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x9a1cef3d_8073_4416_8df2_546835a6414f);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITargetedContentStateChangedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub GetDeferral: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetDeferral: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ITargetedContentSubscription(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ITargetedContentSubscription {
    type Vtable = ITargetedContentSubscription_Vtbl;
}
impl ::core::clone::Clone for ITargetedContentSubscription {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for ITargetedContentSubscription {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x882c2c49_c652_4c7a_acad_1f7fa2986c73);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITargetedContentSubscription_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Id: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub GetContentContainerAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetContentContainerAsync: usize,
    #[cfg(feature = "Foundation")]
    pub ContentChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ContentChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveContentChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cookie: super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveContentChanged: usize,
    #[cfg(feature = "Foundation")]
    pub AvailabilityChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    AvailabilityChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveAvailabilityChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cookie: super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveAvailabilityChanged: usize,
    #[cfg(feature = "Foundation")]
    pub StateChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    StateChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveStateChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cookie: super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveStateChanged: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ITargetedContentSubscriptionOptions(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ITargetedContentSubscriptionOptions {
    type Vtable = ITargetedContentSubscriptionOptions_Vtbl;
}
impl ::core::clone::Clone for ITargetedContentSubscriptionOptions {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for ITargetedContentSubscriptionOptions {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x61ee6ad0_2c83_421b_8467_413eaf1aeb97);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITargetedContentSubscriptionOptions_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub SubscriptionId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub AllowPartialContentAvailability: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub SetAllowPartialContentAvailability: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub CloudQueryParameters: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    CloudQueryParameters: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub LocalFilters: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    LocalFilters: usize,
    pub Update: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ITargetedContentSubscriptionStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ITargetedContentSubscriptionStatics {
    type Vtable = ITargetedContentSubscriptionStatics_Vtbl;
}
impl ::core::clone::Clone for ITargetedContentSubscriptionStatics {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for ITargetedContentSubscriptionStatics {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xfaddfe80_360d_4916_b53c_7ea27090d02a);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITargetedContentSubscriptionStatics_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub GetAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, subscriptionid: ::std::mem::MaybeUninit<::windows_core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetAsync: usize,
    pub GetOptions: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, subscriptionid: ::std::mem::MaybeUninit<::windows_core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ITargetedContentValue(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ITargetedContentValue {
    type Vtable = ITargetedContentValue_Vtbl;
}
impl ::core::clone::Clone for ITargetedContentValue {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for ITargetedContentValue {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xaafde4b3_4215_4bf8_867f_43f04865f9bf);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITargetedContentValue_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub ValueKind: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut TargetedContentValueKind) -> ::windows_core::HRESULT,
    pub Path: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub String: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub Uri: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Uri: usize,
    pub Number: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows_core::HRESULT,
    pub Boolean: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    #[cfg(feature = "Storage_Streams")]
    pub File: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    File: usize,
    pub ImageFile: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub Action: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub Strings: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Strings: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub Uris: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Uris: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub Numbers: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Numbers: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub Booleans: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Booleans: usize,
    #[cfg(all(feature = "Foundation_Collections", feature = "Storage_Streams"))]
    pub Files: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Storage_Streams")))]
    Files: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub ImageFiles: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    ImageFiles: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub Actions: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Actions: usize,
}
#[doc = "*Required features: `\"Services_TargetedContent\"`*"]
#[repr(transparent)]
pub struct TargetedContentAction(::windows_core::IUnknown);
impl TargetedContentAction {
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn InvokeAsync(&self) -> ::windows_core::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).InvokeAsync)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for TargetedContentAction {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for TargetedContentAction {}
impl ::core::fmt::Debug for TargetedContentAction {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TargetedContentAction").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for TargetedContentAction {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Services.TargetedContent.TargetedContentAction;{d75b691e-6cd6-4ca0-9d8f-4728b0b7e6b6})");
}
impl ::core::clone::Clone for TargetedContentAction {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for TargetedContentAction {
    type Vtable = ITargetedContentAction_Vtbl;
}
unsafe impl ::windows_core::ComInterface for TargetedContentAction {
    const IID: ::windows_core::GUID = <ITargetedContentAction as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for TargetedContentAction {
    const NAME: &'static str = "Windows.Services.TargetedContent.TargetedContentAction";
}
::windows_core::imp::interface_hierarchy!(TargetedContentAction, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for TargetedContentAction {}
unsafe impl ::core::marker::Sync for TargetedContentAction {}
#[doc = "*Required features: `\"Services_TargetedContent\"`*"]
#[repr(transparent)]
pub struct TargetedContentAvailabilityChangedEventArgs(::windows_core::IUnknown);
impl TargetedContentAvailabilityChangedEventArgs {
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn GetDeferral(&self) -> ::windows_core::Result<super::super::Foundation::Deferral> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetDeferral)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for TargetedContentAvailabilityChangedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for TargetedContentAvailabilityChangedEventArgs {}
impl ::core::fmt::Debug for TargetedContentAvailabilityChangedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TargetedContentAvailabilityChangedEventArgs").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for TargetedContentAvailabilityChangedEventArgs {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Services.TargetedContent.TargetedContentAvailabilityChangedEventArgs;{e0f59d26-5927-4450-965c-1ceb7becde65})");
}
impl ::core::clone::Clone for TargetedContentAvailabilityChangedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for TargetedContentAvailabilityChangedEventArgs {
    type Vtable = ITargetedContentAvailabilityChangedEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for TargetedContentAvailabilityChangedEventArgs {
    const IID: ::windows_core::GUID = <ITargetedContentAvailabilityChangedEventArgs as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for TargetedContentAvailabilityChangedEventArgs {
    const NAME: &'static str = "Windows.Services.TargetedContent.TargetedContentAvailabilityChangedEventArgs";
}
::windows_core::imp::interface_hierarchy!(TargetedContentAvailabilityChangedEventArgs, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for TargetedContentAvailabilityChangedEventArgs {}
unsafe impl ::core::marker::Sync for TargetedContentAvailabilityChangedEventArgs {}
#[doc = "*Required features: `\"Services_TargetedContent\"`*"]
#[repr(transparent)]
pub struct TargetedContentChangedEventArgs(::windows_core::IUnknown);
impl TargetedContentChangedEventArgs {
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn GetDeferral(&self) -> ::windows_core::Result<super::super::Foundation::Deferral> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetDeferral)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn HasPreviousContentExpired(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).HasPreviousContentExpired)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for TargetedContentChangedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for TargetedContentChangedEventArgs {}
impl ::core::fmt::Debug for TargetedContentChangedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TargetedContentChangedEventArgs").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for TargetedContentChangedEventArgs {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Services.TargetedContent.TargetedContentChangedEventArgs;{99d488c9-587e-4586-8ef7-b54ca9453a16})");
}
impl ::core::clone::Clone for TargetedContentChangedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for TargetedContentChangedEventArgs {
    type Vtable = ITargetedContentChangedEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for TargetedContentChangedEventArgs {
    const IID: ::windows_core::GUID = <ITargetedContentChangedEventArgs as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for TargetedContentChangedEventArgs {
    const NAME: &'static str = "Windows.Services.TargetedContent.TargetedContentChangedEventArgs";
}
::windows_core::imp::interface_hierarchy!(TargetedContentChangedEventArgs, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for TargetedContentChangedEventArgs {}
unsafe impl ::core::marker::Sync for TargetedContentChangedEventArgs {}
#[doc = "*Required features: `\"Services_TargetedContent\"`*"]
#[repr(transparent)]
pub struct TargetedContentCollection(::windows_core::IUnknown);
impl TargetedContentCollection {
    pub fn Id(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Id)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn ReportInteraction(&self, interaction: TargetedContentInteraction) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).ReportInteraction)(::windows_core::Interface::as_raw(this), interaction).ok() }
    }
    pub fn ReportCustomInteraction(&self, custominteractionname: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).ReportCustomInteraction)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(custominteractionname)).ok() }
    }
    pub fn Path(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Path)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Properties(&self) -> ::windows_core::Result<super::super::Foundation::Collections::IMapView<::windows_core::HSTRING, TargetedContentValue>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Properties)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Collections(&self) -> ::windows_core::Result<super::super::Foundation::Collections::IVectorView<TargetedContentCollection>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Collections)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Items(&self) -> ::windows_core::Result<super::super::Foundation::Collections::IVectorView<TargetedContentItem>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Items)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for TargetedContentCollection {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for TargetedContentCollection {}
impl ::core::fmt::Debug for TargetedContentCollection {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TargetedContentCollection").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for TargetedContentCollection {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Services.TargetedContent.TargetedContentCollection;{2d4b66c5-f163-44ba-9f6e-e1a4c2bb559d})");
}
impl ::core::clone::Clone for TargetedContentCollection {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for TargetedContentCollection {
    type Vtable = ITargetedContentCollection_Vtbl;
}
unsafe impl ::windows_core::ComInterface for TargetedContentCollection {
    const IID: ::windows_core::GUID = <ITargetedContentCollection as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for TargetedContentCollection {
    const NAME: &'static str = "Windows.Services.TargetedContent.TargetedContentCollection";
}
::windows_core::imp::interface_hierarchy!(TargetedContentCollection, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for TargetedContentCollection {}
unsafe impl ::core::marker::Sync for TargetedContentCollection {}
#[doc = "*Required features: `\"Services_TargetedContent\"`*"]
#[repr(transparent)]
pub struct TargetedContentContainer(::windows_core::IUnknown);
impl TargetedContentContainer {
    pub fn Id(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Id)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Timestamp(&self) -> ::windows_core::Result<super::super::Foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Timestamp)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Availability(&self) -> ::windows_core::Result<TargetedContentAvailability> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Availability)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Content(&self) -> ::windows_core::Result<TargetedContentCollection> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Content)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SelectSingleObject(&self, path: &::windows_core::HSTRING) -> ::windows_core::Result<TargetedContentObject> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SelectSingleObject)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(path), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn GetAsync(contentid: &::windows_core::HSTRING) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<TargetedContentContainer>> {
        Self::ITargetedContentContainerStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetAsync)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(contentid), &mut result__).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn ITargetedContentContainerStatics<R, F: FnOnce(&ITargetedContentContainerStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<TargetedContentContainer, ITargetedContentContainerStatics> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::cmp::PartialEq for TargetedContentContainer {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for TargetedContentContainer {}
impl ::core::fmt::Debug for TargetedContentContainer {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TargetedContentContainer").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for TargetedContentContainer {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Services.TargetedContent.TargetedContentContainer;{bc2494c9-8837-47c2-850f-d79d64595926})");
}
impl ::core::clone::Clone for TargetedContentContainer {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for TargetedContentContainer {
    type Vtable = ITargetedContentContainer_Vtbl;
}
unsafe impl ::windows_core::ComInterface for TargetedContentContainer {
    const IID: ::windows_core::GUID = <ITargetedContentContainer as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for TargetedContentContainer {
    const NAME: &'static str = "Windows.Services.TargetedContent.TargetedContentContainer";
}
::windows_core::imp::interface_hierarchy!(TargetedContentContainer, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for TargetedContentContainer {}
unsafe impl ::core::marker::Sync for TargetedContentContainer {}
#[doc = "*Required features: `\"Services_TargetedContent\"`, `\"Storage_Streams\"`*"]
#[cfg(feature = "Storage_Streams")]
#[repr(transparent)]
pub struct TargetedContentFile(::windows_core::IUnknown);
#[cfg(feature = "Storage_Streams")]
impl TargetedContentFile {
    #[doc = "*Required features: `\"Foundation\"`, `\"Storage_Streams\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub fn OpenReadAsync(&self) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<super::super::Storage::Streams::IRandomAccessStreamWithContentType>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).OpenReadAsync)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
#[cfg(feature = "Storage_Streams")]
impl ::core::cmp::PartialEq for TargetedContentFile {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Storage_Streams")]
impl ::core::cmp::Eq for TargetedContentFile {}
#[cfg(feature = "Storage_Streams")]
impl ::core::fmt::Debug for TargetedContentFile {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TargetedContentFile").field(&self.0).finish()
    }
}
#[cfg(feature = "Storage_Streams")]
impl ::windows_core::RuntimeType for TargetedContentFile {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Services.TargetedContent.TargetedContentFile;{33ee3134-1dd6-4e3a-8067-d1c162e8642b})");
}
#[cfg(feature = "Storage_Streams")]
impl ::core::clone::Clone for TargetedContentFile {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Storage_Streams")]
unsafe impl ::windows_core::Interface for TargetedContentFile {
    type Vtable = super::super::Storage::Streams::IRandomAccessStreamReference_Vtbl;
}
#[cfg(feature = "Storage_Streams")]
unsafe impl ::windows_core::ComInterface for TargetedContentFile {
    const IID: ::windows_core::GUID = <super::super::Storage::Streams::IRandomAccessStreamReference as ::windows_core::ComInterface>::IID;
}
#[cfg(feature = "Storage_Streams")]
impl ::windows_core::RuntimeName for TargetedContentFile {
    const NAME: &'static str = "Windows.Services.TargetedContent.TargetedContentFile";
}
#[cfg(feature = "Storage_Streams")]
::windows_core::imp::interface_hierarchy!(TargetedContentFile, ::windows_core::IUnknown, ::windows_core::IInspectable);
#[cfg(feature = "Storage_Streams")]
impl ::windows_core::CanTryInto<super::super::Storage::Streams::IRandomAccessStreamReference> for TargetedContentFile {}
#[cfg(feature = "Storage_Streams")]
unsafe impl ::core::marker::Send for TargetedContentFile {}
#[cfg(feature = "Storage_Streams")]
unsafe impl ::core::marker::Sync for TargetedContentFile {}
#[doc = "*Required features: `\"Services_TargetedContent\"`*"]
#[repr(transparent)]
pub struct TargetedContentImage(::windows_core::IUnknown);
impl TargetedContentImage {
    #[doc = "*Required features: `\"Foundation\"`, `\"Storage_Streams\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub fn OpenReadAsync(&self) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<super::super::Storage::Streams::IRandomAccessStreamWithContentType>> {
        let this = &::windows_core::ComInterface::cast::<super::super::Storage::Streams::IRandomAccessStreamReference>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).OpenReadAsync)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Height(&self) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Height)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Width(&self) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Width)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for TargetedContentImage {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for TargetedContentImage {}
impl ::core::fmt::Debug for TargetedContentImage {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TargetedContentImage").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for TargetedContentImage {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Services.TargetedContent.TargetedContentImage;{a7a585d9-779f-4b1e-bbb1-8eaf53fbeab2})");
}
impl ::core::clone::Clone for TargetedContentImage {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for TargetedContentImage {
    type Vtable = ITargetedContentImage_Vtbl;
}
unsafe impl ::windows_core::ComInterface for TargetedContentImage {
    const IID: ::windows_core::GUID = <ITargetedContentImage as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for TargetedContentImage {
    const NAME: &'static str = "Windows.Services.TargetedContent.TargetedContentImage";
}
::windows_core::imp::interface_hierarchy!(TargetedContentImage, ::windows_core::IUnknown, ::windows_core::IInspectable);
#[cfg(feature = "Storage_Streams")]
impl ::windows_core::CanTryInto<super::super::Storage::Streams::IRandomAccessStreamReference> for TargetedContentImage {}
unsafe impl ::core::marker::Send for TargetedContentImage {}
unsafe impl ::core::marker::Sync for TargetedContentImage {}
#[doc = "*Required features: `\"Services_TargetedContent\"`*"]
#[repr(transparent)]
pub struct TargetedContentItem(::windows_core::IUnknown);
impl TargetedContentItem {
    pub fn Path(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Path)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn ReportInteraction(&self, interaction: TargetedContentInteraction) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).ReportInteraction)(::windows_core::Interface::as_raw(this), interaction).ok() }
    }
    pub fn ReportCustomInteraction(&self, custominteractionname: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).ReportCustomInteraction)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(custominteractionname)).ok() }
    }
    pub fn State(&self) -> ::windows_core::Result<TargetedContentItemState> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).State)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Properties(&self) -> ::windows_core::Result<super::super::Foundation::Collections::IMapView<::windows_core::HSTRING, TargetedContentValue>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Properties)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Collections(&self) -> ::windows_core::Result<super::super::Foundation::Collections::IVectorView<TargetedContentCollection>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Collections)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for TargetedContentItem {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for TargetedContentItem {}
impl ::core::fmt::Debug for TargetedContentItem {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TargetedContentItem").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for TargetedContentItem {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Services.TargetedContent.TargetedContentItem;{38168dc4-276c-4c32-96ba-565c6e406e74})");
}
impl ::core::clone::Clone for TargetedContentItem {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for TargetedContentItem {
    type Vtable = ITargetedContentItem_Vtbl;
}
unsafe impl ::windows_core::ComInterface for TargetedContentItem {
    const IID: ::windows_core::GUID = <ITargetedContentItem as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for TargetedContentItem {
    const NAME: &'static str = "Windows.Services.TargetedContent.TargetedContentItem";
}
::windows_core::imp::interface_hierarchy!(TargetedContentItem, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for TargetedContentItem {}
unsafe impl ::core::marker::Sync for TargetedContentItem {}
#[doc = "*Required features: `\"Services_TargetedContent\"`*"]
#[repr(transparent)]
pub struct TargetedContentItemState(::windows_core::IUnknown);
impl TargetedContentItemState {
    pub fn ShouldDisplay(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ShouldDisplay)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn AppInstallationState(&self) -> ::windows_core::Result<TargetedContentAppInstallationState> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).AppInstallationState)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for TargetedContentItemState {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for TargetedContentItemState {}
impl ::core::fmt::Debug for TargetedContentItemState {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TargetedContentItemState").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for TargetedContentItemState {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Services.TargetedContent.TargetedContentItemState;{73935454-4c65-4b47-a441-472de53c79b6})");
}
impl ::core::clone::Clone for TargetedContentItemState {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for TargetedContentItemState {
    type Vtable = ITargetedContentItemState_Vtbl;
}
unsafe impl ::windows_core::ComInterface for TargetedContentItemState {
    const IID: ::windows_core::GUID = <ITargetedContentItemState as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for TargetedContentItemState {
    const NAME: &'static str = "Windows.Services.TargetedContent.TargetedContentItemState";
}
::windows_core::imp::interface_hierarchy!(TargetedContentItemState, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for TargetedContentItemState {}
unsafe impl ::core::marker::Sync for TargetedContentItemState {}
#[doc = "*Required features: `\"Services_TargetedContent\"`*"]
#[repr(transparent)]
pub struct TargetedContentObject(::windows_core::IUnknown);
impl TargetedContentObject {
    pub fn ObjectKind(&self) -> ::windows_core::Result<TargetedContentObjectKind> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ObjectKind)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Collection(&self) -> ::windows_core::Result<TargetedContentCollection> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Collection)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Item(&self) -> ::windows_core::Result<TargetedContentItem> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Item)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Value(&self) -> ::windows_core::Result<TargetedContentValue> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Value)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for TargetedContentObject {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for TargetedContentObject {}
impl ::core::fmt::Debug for TargetedContentObject {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TargetedContentObject").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for TargetedContentObject {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Services.TargetedContent.TargetedContentObject;{041d7969-2212-42d1-9dfa-88a8e3033aa3})");
}
impl ::core::clone::Clone for TargetedContentObject {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for TargetedContentObject {
    type Vtable = ITargetedContentObject_Vtbl;
}
unsafe impl ::windows_core::ComInterface for TargetedContentObject {
    const IID: ::windows_core::GUID = <ITargetedContentObject as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for TargetedContentObject {
    const NAME: &'static str = "Windows.Services.TargetedContent.TargetedContentObject";
}
::windows_core::imp::interface_hierarchy!(TargetedContentObject, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for TargetedContentObject {}
unsafe impl ::core::marker::Sync for TargetedContentObject {}
#[doc = "*Required features: `\"Services_TargetedContent\"`*"]
#[repr(transparent)]
pub struct TargetedContentStateChangedEventArgs(::windows_core::IUnknown);
impl TargetedContentStateChangedEventArgs {
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn GetDeferral(&self) -> ::windows_core::Result<super::super::Foundation::Deferral> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetDeferral)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for TargetedContentStateChangedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for TargetedContentStateChangedEventArgs {}
impl ::core::fmt::Debug for TargetedContentStateChangedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TargetedContentStateChangedEventArgs").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for TargetedContentStateChangedEventArgs {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Services.TargetedContent.TargetedContentStateChangedEventArgs;{9a1cef3d-8073-4416-8df2-546835a6414f})");
}
impl ::core::clone::Clone for TargetedContentStateChangedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for TargetedContentStateChangedEventArgs {
    type Vtable = ITargetedContentStateChangedEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for TargetedContentStateChangedEventArgs {
    const IID: ::windows_core::GUID = <ITargetedContentStateChangedEventArgs as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for TargetedContentStateChangedEventArgs {
    const NAME: &'static str = "Windows.Services.TargetedContent.TargetedContentStateChangedEventArgs";
}
::windows_core::imp::interface_hierarchy!(TargetedContentStateChangedEventArgs, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for TargetedContentStateChangedEventArgs {}
unsafe impl ::core::marker::Sync for TargetedContentStateChangedEventArgs {}
#[doc = "*Required features: `\"Services_TargetedContent\"`*"]
#[repr(transparent)]
pub struct TargetedContentSubscription(::windows_core::IUnknown);
impl TargetedContentSubscription {
    pub fn Id(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Id)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn GetContentContainerAsync(&self) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<TargetedContentContainer>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetContentContainerAsync)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ContentChanged<P0>(&self, handler: P0) -> ::windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::TypedEventHandler<TargetedContentSubscription, TargetedContentChangedEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ContentChanged)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveContentChanged(&self, cookie: super::super::Foundation::EventRegistrationToken) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveContentChanged)(::windows_core::Interface::as_raw(this), cookie).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn AvailabilityChanged<P0>(&self, handler: P0) -> ::windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::TypedEventHandler<TargetedContentSubscription, TargetedContentAvailabilityChangedEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).AvailabilityChanged)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveAvailabilityChanged(&self, cookie: super::super::Foundation::EventRegistrationToken) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveAvailabilityChanged)(::windows_core::Interface::as_raw(this), cookie).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn StateChanged<P0>(&self, handler: P0) -> ::windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::TypedEventHandler<TargetedContentSubscription, TargetedContentStateChangedEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).StateChanged)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveStateChanged(&self, cookie: super::super::Foundation::EventRegistrationToken) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveStateChanged)(::windows_core::Interface::as_raw(this), cookie).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn GetAsync(subscriptionid: &::windows_core::HSTRING) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<TargetedContentSubscription>> {
        Self::ITargetedContentSubscriptionStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetAsync)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(subscriptionid), &mut result__).from_abi(result__)
        })
    }
    pub fn GetOptions(subscriptionid: &::windows_core::HSTRING) -> ::windows_core::Result<TargetedContentSubscriptionOptions> {
        Self::ITargetedContentSubscriptionStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetOptions)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(subscriptionid), &mut result__).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn ITargetedContentSubscriptionStatics<R, F: FnOnce(&ITargetedContentSubscriptionStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<TargetedContentSubscription, ITargetedContentSubscriptionStatics> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::cmp::PartialEq for TargetedContentSubscription {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for TargetedContentSubscription {}
impl ::core::fmt::Debug for TargetedContentSubscription {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TargetedContentSubscription").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for TargetedContentSubscription {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Services.TargetedContent.TargetedContentSubscription;{882c2c49-c652-4c7a-acad-1f7fa2986c73})");
}
impl ::core::clone::Clone for TargetedContentSubscription {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for TargetedContentSubscription {
    type Vtable = ITargetedContentSubscription_Vtbl;
}
unsafe impl ::windows_core::ComInterface for TargetedContentSubscription {
    const IID: ::windows_core::GUID = <ITargetedContentSubscription as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for TargetedContentSubscription {
    const NAME: &'static str = "Windows.Services.TargetedContent.TargetedContentSubscription";
}
::windows_core::imp::interface_hierarchy!(TargetedContentSubscription, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for TargetedContentSubscription {}
unsafe impl ::core::marker::Sync for TargetedContentSubscription {}
#[doc = "*Required features: `\"Services_TargetedContent\"`*"]
#[repr(transparent)]
pub struct TargetedContentSubscriptionOptions(::windows_core::IUnknown);
impl TargetedContentSubscriptionOptions {
    pub fn SubscriptionId(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SubscriptionId)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn AllowPartialContentAvailability(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).AllowPartialContentAvailability)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetAllowPartialContentAvailability(&self, value: bool) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetAllowPartialContentAvailability)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn CloudQueryParameters(&self) -> ::windows_core::Result<super::super::Foundation::Collections::IMap<::windows_core::HSTRING, ::windows_core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CloudQueryParameters)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn LocalFilters(&self) -> ::windows_core::Result<super::super::Foundation::Collections::IVector<::windows_core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).LocalFilters)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Update(&self) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).Update)(::windows_core::Interface::as_raw(this)).ok() }
    }
}
impl ::core::cmp::PartialEq for TargetedContentSubscriptionOptions {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for TargetedContentSubscriptionOptions {}
impl ::core::fmt::Debug for TargetedContentSubscriptionOptions {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TargetedContentSubscriptionOptions").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for TargetedContentSubscriptionOptions {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Services.TargetedContent.TargetedContentSubscriptionOptions;{61ee6ad0-2c83-421b-8467-413eaf1aeb97})");
}
impl ::core::clone::Clone for TargetedContentSubscriptionOptions {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for TargetedContentSubscriptionOptions {
    type Vtable = ITargetedContentSubscriptionOptions_Vtbl;
}
unsafe impl ::windows_core::ComInterface for TargetedContentSubscriptionOptions {
    const IID: ::windows_core::GUID = <ITargetedContentSubscriptionOptions as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for TargetedContentSubscriptionOptions {
    const NAME: &'static str = "Windows.Services.TargetedContent.TargetedContentSubscriptionOptions";
}
::windows_core::imp::interface_hierarchy!(TargetedContentSubscriptionOptions, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for TargetedContentSubscriptionOptions {}
unsafe impl ::core::marker::Sync for TargetedContentSubscriptionOptions {}
#[doc = "*Required features: `\"Services_TargetedContent\"`*"]
#[repr(transparent)]
pub struct TargetedContentValue(::windows_core::IUnknown);
impl TargetedContentValue {
    pub fn ValueKind(&self) -> ::windows_core::Result<TargetedContentValueKind> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ValueKind)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Path(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Path)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn String(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).String)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Uri(&self) -> ::windows_core::Result<super::super::Foundation::Uri> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Uri)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Number(&self) -> ::windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Number)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Boolean(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Boolean)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn File(&self) -> ::windows_core::Result<TargetedContentFile> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).File)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn ImageFile(&self) -> ::windows_core::Result<TargetedContentImage> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ImageFile)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Action(&self) -> ::windows_core::Result<TargetedContentAction> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Action)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Strings(&self) -> ::windows_core::Result<super::super::Foundation::Collections::IVectorView<::windows_core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Strings)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Uris(&self) -> ::windows_core::Result<super::super::Foundation::Collections::IVectorView<super::super::Foundation::Uri>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Uris)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Numbers(&self) -> ::windows_core::Result<super::super::Foundation::Collections::IVectorView<f64>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Numbers)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Booleans(&self) -> ::windows_core::Result<super::super::Foundation::Collections::IVectorView<bool>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Booleans)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`, `\"Storage_Streams\"`*"]
    #[cfg(all(feature = "Foundation_Collections", feature = "Storage_Streams"))]
    pub fn Files(&self) -> ::windows_core::Result<super::super::Foundation::Collections::IVectorView<TargetedContentFile>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Files)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn ImageFiles(&self) -> ::windows_core::Result<super::super::Foundation::Collections::IVectorView<TargetedContentImage>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ImageFiles)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Actions(&self) -> ::windows_core::Result<super::super::Foundation::Collections::IVectorView<TargetedContentAction>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Actions)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for TargetedContentValue {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for TargetedContentValue {}
impl ::core::fmt::Debug for TargetedContentValue {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TargetedContentValue").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for TargetedContentValue {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Services.TargetedContent.TargetedContentValue;{aafde4b3-4215-4bf8-867f-43f04865f9bf})");
}
impl ::core::clone::Clone for TargetedContentValue {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for TargetedContentValue {
    type Vtable = ITargetedContentValue_Vtbl;
}
unsafe impl ::windows_core::ComInterface for TargetedContentValue {
    const IID: ::windows_core::GUID = <ITargetedContentValue as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for TargetedContentValue {
    const NAME: &'static str = "Windows.Services.TargetedContent.TargetedContentValue";
}
::windows_core::imp::interface_hierarchy!(TargetedContentValue, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for TargetedContentValue {}
unsafe impl ::core::marker::Sync for TargetedContentValue {}
#[doc = "*Required features: `\"Services_TargetedContent\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct TargetedContentAppInstallationState(pub i32);
impl TargetedContentAppInstallationState {
    pub const NotApplicable: Self = Self(0i32);
    pub const NotInstalled: Self = Self(1i32);
    pub const Installed: Self = Self(2i32);
}
impl ::core::marker::Copy for TargetedContentAppInstallationState {}
impl ::core::clone::Clone for TargetedContentAppInstallationState {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for TargetedContentAppInstallationState {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for TargetedContentAppInstallationState {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for TargetedContentAppInstallationState {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TargetedContentAppInstallationState").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for TargetedContentAppInstallationState {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Services.TargetedContent.TargetedContentAppInstallationState;i4)");
}
#[doc = "*Required features: `\"Services_TargetedContent\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct TargetedContentAvailability(pub i32);
impl TargetedContentAvailability {
    pub const None: Self = Self(0i32);
    pub const Partial: Self = Self(1i32);
    pub const All: Self = Self(2i32);
}
impl ::core::marker::Copy for TargetedContentAvailability {}
impl ::core::clone::Clone for TargetedContentAvailability {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for TargetedContentAvailability {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for TargetedContentAvailability {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for TargetedContentAvailability {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TargetedContentAvailability").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for TargetedContentAvailability {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Services.TargetedContent.TargetedContentAvailability;i4)");
}
#[doc = "*Required features: `\"Services_TargetedContent\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct TargetedContentInteraction(pub i32);
impl TargetedContentInteraction {
    pub const Impression: Self = Self(0i32);
    pub const ClickThrough: Self = Self(1i32);
    pub const Hover: Self = Self(2i32);
    pub const Like: Self = Self(3i32);
    pub const Dislike: Self = Self(4i32);
    pub const Dismiss: Self = Self(5i32);
    pub const Ineligible: Self = Self(6i32);
    pub const Accept: Self = Self(7i32);
    pub const Decline: Self = Self(8i32);
    pub const Defer: Self = Self(9i32);
    pub const Canceled: Self = Self(10i32);
    pub const Conversion: Self = Self(11i32);
    pub const Opportunity: Self = Self(12i32);
}
impl ::core::marker::Copy for TargetedContentInteraction {}
impl ::core::clone::Clone for TargetedContentInteraction {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for TargetedContentInteraction {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for TargetedContentInteraction {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for TargetedContentInteraction {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TargetedContentInteraction").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for TargetedContentInteraction {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Services.TargetedContent.TargetedContentInteraction;i4)");
}
#[doc = "*Required features: `\"Services_TargetedContent\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct TargetedContentObjectKind(pub i32);
impl TargetedContentObjectKind {
    pub const Collection: Self = Self(0i32);
    pub const Item: Self = Self(1i32);
    pub const Value: Self = Self(2i32);
}
impl ::core::marker::Copy for TargetedContentObjectKind {}
impl ::core::clone::Clone for TargetedContentObjectKind {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for TargetedContentObjectKind {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for TargetedContentObjectKind {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for TargetedContentObjectKind {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TargetedContentObjectKind").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for TargetedContentObjectKind {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Services.TargetedContent.TargetedContentObjectKind;i4)");
}
#[doc = "*Required features: `\"Services_TargetedContent\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct TargetedContentValueKind(pub i32);
impl TargetedContentValueKind {
    pub const String: Self = Self(0i32);
    pub const Uri: Self = Self(1i32);
    pub const Number: Self = Self(2i32);
    pub const Boolean: Self = Self(3i32);
    pub const File: Self = Self(4i32);
    pub const ImageFile: Self = Self(5i32);
    pub const Action: Self = Self(6i32);
    pub const Strings: Self = Self(7i32);
    pub const Uris: Self = Self(8i32);
    pub const Numbers: Self = Self(9i32);
    pub const Booleans: Self = Self(10i32);
    pub const Files: Self = Self(11i32);
    pub const ImageFiles: Self = Self(12i32);
    pub const Actions: Self = Self(13i32);
}
impl ::core::marker::Copy for TargetedContentValueKind {}
impl ::core::clone::Clone for TargetedContentValueKind {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for TargetedContentValueKind {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for TargetedContentValueKind {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for TargetedContentValueKind {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TargetedContentValueKind").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for TargetedContentValueKind {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Services.TargetedContent.TargetedContentValueKind;i4)");
}
