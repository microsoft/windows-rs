#[doc(hidden)]
#[repr(transparent)]
pub struct ICoreTextCompositionCompletedEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for ICoreTextCompositionCompletedEventArgs {
    type Vtable = ICoreTextCompositionCompletedEventArgs_Vtbl;
}
unsafe impl ::windows::core::Interface for ICoreTextCompositionCompletedEventArgs {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1f34ebb6_b79f_4121_a5e7_fda9b8616e30);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreTextCompositionCompletedEventArgs_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub IsCanceled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub CompositionSegments: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    CompositionSegments: usize,
    #[cfg(feature = "Foundation")]
    pub GetDeferral: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetDeferral: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICoreTextCompositionSegment(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for ICoreTextCompositionSegment {
    type Vtable = ICoreTextCompositionSegment_Vtbl;
}
unsafe impl ::windows::core::Interface for ICoreTextCompositionSegment {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x776c6bd9_4ead_4da7_8f47_3a88b523cc34);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreTextCompositionSegment_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub PreconversionString: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Range: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut CoreTextRange) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICoreTextCompositionStartedEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for ICoreTextCompositionStartedEventArgs {
    type Vtable = ICoreTextCompositionStartedEventArgs_Vtbl;
}
unsafe impl ::windows::core::Interface for ICoreTextCompositionStartedEventArgs {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x276b16a9_64e7_4ab0_bc4b_a02d73835bfb);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreTextCompositionStartedEventArgs_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub IsCanceled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub GetDeferral: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetDeferral: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICoreTextEditContext(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for ICoreTextEditContext {
    type Vtable = ICoreTextEditContext_Vtbl;
}
unsafe impl ::windows::core::Interface for ICoreTextEditContext {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xbf6608af_4041_47c3_b263_a918eb5eaef2);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreTextEditContext_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Name: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub InputScope: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut CoreTextInputScope) -> ::windows::core::HRESULT,
    pub SetInputScope: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: CoreTextInputScope) -> ::windows::core::HRESULT,
    pub IsReadOnly: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetIsReadOnly: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    pub InputPaneDisplayPolicy: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut CoreTextInputPaneDisplayPolicy) -> ::windows::core::HRESULT,
    pub SetInputPaneDisplayPolicy: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: CoreTextInputPaneDisplayPolicy) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub TextRequested: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    TextRequested: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveTextRequested: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cookie: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveTextRequested: usize,
    #[cfg(feature = "Foundation")]
    pub SelectionRequested: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SelectionRequested: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveSelectionRequested: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cookie: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveSelectionRequested: usize,
    #[cfg(feature = "Foundation")]
    pub LayoutRequested: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    LayoutRequested: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveLayoutRequested: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cookie: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveLayoutRequested: usize,
    #[cfg(feature = "Foundation")]
    pub TextUpdating: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    TextUpdating: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveTextUpdating: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cookie: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveTextUpdating: usize,
    #[cfg(feature = "Foundation")]
    pub SelectionUpdating: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SelectionUpdating: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveSelectionUpdating: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cookie: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveSelectionUpdating: usize,
    #[cfg(feature = "Foundation")]
    pub FormatUpdating: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    FormatUpdating: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveFormatUpdating: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cookie: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveFormatUpdating: usize,
    #[cfg(feature = "Foundation")]
    pub CompositionStarted: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    CompositionStarted: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveCompositionStarted: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cookie: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveCompositionStarted: usize,
    #[cfg(feature = "Foundation")]
    pub CompositionCompleted: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    CompositionCompleted: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveCompositionCompleted: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cookie: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveCompositionCompleted: usize,
    #[cfg(feature = "Foundation")]
    pub FocusRemoved: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    FocusRemoved: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveFocusRemoved: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cookie: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveFocusRemoved: usize,
    pub NotifyFocusEnter: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub NotifyFocusLeave: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub NotifyTextChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, modifiedrange: CoreTextRange, newlength: i32, newselection: CoreTextRange) -> ::windows::core::HRESULT,
    pub NotifySelectionChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, selection: CoreTextRange) -> ::windows::core::HRESULT,
    pub NotifyLayoutChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICoreTextEditContext2(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for ICoreTextEditContext2 {
    type Vtable = ICoreTextEditContext2_Vtbl;
}
unsafe impl ::windows::core::Interface for ICoreTextEditContext2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb1867dbb_083b_49e1_b281_2b35d62bf466);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreTextEditContext2_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub NotifyFocusLeaveCompleted: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    NotifyFocusLeaveCompleted: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveNotifyFocusLeaveCompleted: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cookie: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveNotifyFocusLeaveCompleted: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICoreTextFormatUpdatingEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for ICoreTextFormatUpdatingEventArgs {
    type Vtable = ICoreTextFormatUpdatingEventArgs_Vtbl;
}
unsafe impl ::windows::core::Interface for ICoreTextFormatUpdatingEventArgs {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7310bd33_b4a8_43b1_b37b_0724d4aca7ab);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreTextFormatUpdatingEventArgs_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Range: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut CoreTextRange) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Foundation", feature = "UI_ViewManagement"))]
    pub TextColor: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "UI_ViewManagement")))]
    TextColor: usize,
    #[cfg(all(feature = "Foundation", feature = "UI_ViewManagement"))]
    pub BackgroundColor: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "UI_ViewManagement")))]
    BackgroundColor: usize,
    #[cfg(all(feature = "Foundation", feature = "UI_ViewManagement"))]
    pub UnderlineColor: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "UI_ViewManagement")))]
    UnderlineColor: usize,
    #[cfg(feature = "Foundation")]
    pub UnderlineType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    UnderlineType: usize,
    pub Reason: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut CoreTextFormatUpdatingReason) -> ::windows::core::HRESULT,
    pub Result: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut CoreTextFormatUpdatingResult) -> ::windows::core::HRESULT,
    pub SetResult: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: CoreTextFormatUpdatingResult) -> ::windows::core::HRESULT,
    pub IsCanceled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub GetDeferral: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetDeferral: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICoreTextLayoutBounds(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for ICoreTextLayoutBounds {
    type Vtable = ICoreTextLayoutBounds_Vtbl;
}
unsafe impl ::windows::core::Interface for ICoreTextLayoutBounds {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe972c974_4436_4917_80d0_a525e4ca6780);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreTextLayoutBounds_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub TextBounds: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::Rect) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    TextBounds: usize,
    #[cfg(feature = "Foundation")]
    pub SetTextBounds: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::super::super::Foundation::Rect) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetTextBounds: usize,
    #[cfg(feature = "Foundation")]
    pub ControlBounds: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::Rect) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ControlBounds: usize,
    #[cfg(feature = "Foundation")]
    pub SetControlBounds: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::super::super::Foundation::Rect) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetControlBounds: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICoreTextLayoutRequest(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for ICoreTextLayoutRequest {
    type Vtable = ICoreTextLayoutRequest_Vtbl;
}
unsafe impl ::windows::core::Interface for ICoreTextLayoutRequest {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2555a8cc_51fd_4f03_98bf_ac78174d68e0);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreTextLayoutRequest_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Range: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut CoreTextRange) -> ::windows::core::HRESULT,
    pub LayoutBounds: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub IsCanceled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub GetDeferral: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetDeferral: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICoreTextLayoutRequest2(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for ICoreTextLayoutRequest2 {
    type Vtable = ICoreTextLayoutRequest2_Vtbl;
}
unsafe impl ::windows::core::Interface for ICoreTextLayoutRequest2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x676de624_cd3d_4bcd_bf01_7f7110954511);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreTextLayoutRequest2_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub LayoutBoundsVisualPixels: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICoreTextLayoutRequestedEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for ICoreTextLayoutRequestedEventArgs {
    type Vtable = ICoreTextLayoutRequestedEventArgs_Vtbl;
}
unsafe impl ::windows::core::Interface for ICoreTextLayoutRequestedEventArgs {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb1dc6ae0_9a7b_4e9e_a566_4a6b5f8ad676);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreTextLayoutRequestedEventArgs_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Request: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICoreTextSelectionRequest(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for ICoreTextSelectionRequest {
    type Vtable = ICoreTextSelectionRequest_Vtbl;
}
unsafe impl ::windows::core::Interface for ICoreTextSelectionRequest {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf0a70403_208b_4301_883c_74ca7485fd8d);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreTextSelectionRequest_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Selection: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut CoreTextRange) -> ::windows::core::HRESULT,
    pub SetSelection: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: CoreTextRange) -> ::windows::core::HRESULT,
    pub IsCanceled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub GetDeferral: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetDeferral: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICoreTextSelectionRequestedEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for ICoreTextSelectionRequestedEventArgs {
    type Vtable = ICoreTextSelectionRequestedEventArgs_Vtbl;
}
unsafe impl ::windows::core::Interface for ICoreTextSelectionRequestedEventArgs {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x13c6682b_f614_421a_8f4b_9ec8a5a37fcd);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreTextSelectionRequestedEventArgs_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Request: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICoreTextSelectionUpdatingEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for ICoreTextSelectionUpdatingEventArgs {
    type Vtable = ICoreTextSelectionUpdatingEventArgs_Vtbl;
}
unsafe impl ::windows::core::Interface for ICoreTextSelectionUpdatingEventArgs {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd445839f_fe7f_4bd5_8a26_0922c1b3e639);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreTextSelectionUpdatingEventArgs_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Selection: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut CoreTextRange) -> ::windows::core::HRESULT,
    pub Result: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut CoreTextSelectionUpdatingResult) -> ::windows::core::HRESULT,
    pub SetResult: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: CoreTextSelectionUpdatingResult) -> ::windows::core::HRESULT,
    pub IsCanceled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub GetDeferral: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetDeferral: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICoreTextServicesManager(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for ICoreTextServicesManager {
    type Vtable = ICoreTextServicesManager_Vtbl;
}
unsafe impl ::windows::core::Interface for ICoreTextServicesManager {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc2507d83_6e0a_4a8a_bdf8_1948874854ba);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreTextServicesManager_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Globalization")]
    pub InputLanguage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Globalization"))]
    InputLanguage: usize,
    #[cfg(feature = "Foundation")]
    pub InputLanguageChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    InputLanguageChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveInputLanguageChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cookie: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveInputLanguageChanged: usize,
    pub CreateEditContext: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICoreTextServicesManagerStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for ICoreTextServicesManagerStatics {
    type Vtable = ICoreTextServicesManagerStatics_Vtbl;
}
unsafe impl ::windows::core::Interface for ICoreTextServicesManagerStatics {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1520a388_e2cf_4d65_aeb9_b32d86fe39b9);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreTextServicesManagerStatics_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub GetForCurrentView: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICoreTextServicesStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for ICoreTextServicesStatics {
    type Vtable = ICoreTextServicesStatics_Vtbl;
}
unsafe impl ::windows::core::Interface for ICoreTextServicesStatics {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x91859a46_eccf_47a4_8ae7_098a9c6fbb15);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreTextServicesStatics_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub HiddenCharacter: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u16) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICoreTextTextRequest(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for ICoreTextTextRequest {
    type Vtable = ICoreTextTextRequest_Vtbl;
}
unsafe impl ::windows::core::Interface for ICoreTextTextRequest {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x50d950a9_f51e_4cc1_8ca1_e6346d1a61be);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreTextTextRequest_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Range: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut CoreTextRange) -> ::windows::core::HRESULT,
    pub Text: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetText: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub IsCanceled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub GetDeferral: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetDeferral: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICoreTextTextRequestedEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for ICoreTextTextRequestedEventArgs {
    type Vtable = ICoreTextTextRequestedEventArgs_Vtbl;
}
unsafe impl ::windows::core::Interface for ICoreTextTextRequestedEventArgs {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf096a2d0_41c6_4c02_8b1a_d953b00cabb3);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreTextTextRequestedEventArgs_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Request: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICoreTextTextUpdatingEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for ICoreTextTextUpdatingEventArgs {
    type Vtable = ICoreTextTextUpdatingEventArgs_Vtbl;
}
unsafe impl ::windows::core::Interface for ICoreTextTextUpdatingEventArgs {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xeea7918d_cc2b_4f03_8ff6_02fd217db450);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreTextTextUpdatingEventArgs_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Range: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut CoreTextRange) -> ::windows::core::HRESULT,
    pub Text: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub NewSelection: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut CoreTextRange) -> ::windows::core::HRESULT,
    #[cfg(feature = "Globalization")]
    pub InputLanguage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Globalization"))]
    InputLanguage: usize,
    pub Result: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut CoreTextTextUpdatingResult) -> ::windows::core::HRESULT,
    pub SetResult: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: CoreTextTextUpdatingResult) -> ::windows::core::HRESULT,
    pub IsCanceled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub GetDeferral: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetDeferral: usize,
}
#[doc = "*Required features: `\"UI_Text_Core\"`*"]
#[repr(transparent)]
pub struct CoreTextCompositionCompletedEventArgs(::windows::core::IUnknown);
impl CoreTextCompositionCompletedEventArgs {
    pub fn IsCanceled(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).IsCanceled)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn CompositionSegments(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<CoreTextCompositionSegment>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CompositionSegments)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn GetDeferral(&self) -> ::windows::core::Result<super::super::super::Foundation::Deferral> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetDeferral)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
}
impl ::core::clone::Clone for CoreTextCompositionCompletedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for CoreTextCompositionCompletedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for CoreTextCompositionCompletedEventArgs {}
impl ::core::fmt::Debug for CoreTextCompositionCompletedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CoreTextCompositionCompletedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for CoreTextCompositionCompletedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Text.Core.CoreTextCompositionCompletedEventArgs;{1f34ebb6-b79f-4121-a5e7-fda9b8616e30})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for CoreTextCompositionCompletedEventArgs {
    type Vtable = ICoreTextCompositionCompletedEventArgs_Vtbl;
}
unsafe impl ::windows::core::Interface for CoreTextCompositionCompletedEventArgs {
    const IID: ::windows::core::GUID = <ICoreTextCompositionCompletedEventArgs as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for CoreTextCompositionCompletedEventArgs {
    const NAME: &'static str = "Windows.UI.Text.Core.CoreTextCompositionCompletedEventArgs";
}
::windows::core::interface_hierarchy!(CoreTextCompositionCompletedEventArgs, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for CoreTextCompositionCompletedEventArgs {}
unsafe impl ::core::marker::Sync for CoreTextCompositionCompletedEventArgs {}
#[doc = "*Required features: `\"UI_Text_Core\"`*"]
#[repr(transparent)]
pub struct CoreTextCompositionSegment(::windows::core::IUnknown);
impl CoreTextCompositionSegment {
    pub fn PreconversionString(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).PreconversionString)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn Range(&self) -> ::windows::core::Result<CoreTextRange> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Range)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
}
impl ::core::clone::Clone for CoreTextCompositionSegment {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for CoreTextCompositionSegment {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for CoreTextCompositionSegment {}
impl ::core::fmt::Debug for CoreTextCompositionSegment {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CoreTextCompositionSegment").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for CoreTextCompositionSegment {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Text.Core.CoreTextCompositionSegment;{776c6bd9-4ead-4da7-8f47-3a88b523cc34})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for CoreTextCompositionSegment {
    type Vtable = ICoreTextCompositionSegment_Vtbl;
}
unsafe impl ::windows::core::Interface for CoreTextCompositionSegment {
    const IID: ::windows::core::GUID = <ICoreTextCompositionSegment as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for CoreTextCompositionSegment {
    const NAME: &'static str = "Windows.UI.Text.Core.CoreTextCompositionSegment";
}
::windows::core::interface_hierarchy!(CoreTextCompositionSegment, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for CoreTextCompositionSegment {}
unsafe impl ::core::marker::Sync for CoreTextCompositionSegment {}
#[doc = "*Required features: `\"UI_Text_Core\"`*"]
#[repr(transparent)]
pub struct CoreTextCompositionStartedEventArgs(::windows::core::IUnknown);
impl CoreTextCompositionStartedEventArgs {
    pub fn IsCanceled(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).IsCanceled)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn GetDeferral(&self) -> ::windows::core::Result<super::super::super::Foundation::Deferral> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetDeferral)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
}
impl ::core::clone::Clone for CoreTextCompositionStartedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for CoreTextCompositionStartedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for CoreTextCompositionStartedEventArgs {}
impl ::core::fmt::Debug for CoreTextCompositionStartedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CoreTextCompositionStartedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for CoreTextCompositionStartedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Text.Core.CoreTextCompositionStartedEventArgs;{276b16a9-64e7-4ab0-bc4b-a02d73835bfb})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for CoreTextCompositionStartedEventArgs {
    type Vtable = ICoreTextCompositionStartedEventArgs_Vtbl;
}
unsafe impl ::windows::core::Interface for CoreTextCompositionStartedEventArgs {
    const IID: ::windows::core::GUID = <ICoreTextCompositionStartedEventArgs as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for CoreTextCompositionStartedEventArgs {
    const NAME: &'static str = "Windows.UI.Text.Core.CoreTextCompositionStartedEventArgs";
}
::windows::core::interface_hierarchy!(CoreTextCompositionStartedEventArgs, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for CoreTextCompositionStartedEventArgs {}
unsafe impl ::core::marker::Sync for CoreTextCompositionStartedEventArgs {}
#[doc = "*Required features: `\"UI_Text_Core\"`*"]
#[repr(transparent)]
pub struct CoreTextEditContext(::windows::core::IUnknown);
impl CoreTextEditContext {
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
    pub fn InputScope(&self) -> ::windows::core::Result<CoreTextInputScope> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).InputScope)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetInputScope(&self, value: CoreTextInputScope) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetInputScope)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
    pub fn IsReadOnly(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).IsReadOnly)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetIsReadOnly(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetIsReadOnly)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
    pub fn InputPaneDisplayPolicy(&self) -> ::windows::core::Result<CoreTextInputPaneDisplayPolicy> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).InputPaneDisplayPolicy)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetInputPaneDisplayPolicy(&self, value: CoreTextInputPaneDisplayPolicy) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetInputPaneDisplayPolicy)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn TextRequested(&self, handler: &super::super::super::Foundation::TypedEventHandler<CoreTextEditContext, CoreTextTextRequestedEventArgs>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).TextRequested)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(handler), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveTextRequested(&self, cookie: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).RemoveTextRequested)(::windows::core::Vtable::as_raw(this), cookie).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SelectionRequested(&self, handler: &super::super::super::Foundation::TypedEventHandler<CoreTextEditContext, CoreTextSelectionRequestedEventArgs>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).SelectionRequested)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(handler), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveSelectionRequested(&self, cookie: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).RemoveSelectionRequested)(::windows::core::Vtable::as_raw(this), cookie).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn LayoutRequested(&self, handler: &super::super::super::Foundation::TypedEventHandler<CoreTextEditContext, CoreTextLayoutRequestedEventArgs>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).LayoutRequested)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(handler), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveLayoutRequested(&self, cookie: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).RemoveLayoutRequested)(::windows::core::Vtable::as_raw(this), cookie).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn TextUpdating(&self, handler: &super::super::super::Foundation::TypedEventHandler<CoreTextEditContext, CoreTextTextUpdatingEventArgs>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).TextUpdating)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(handler), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveTextUpdating(&self, cookie: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).RemoveTextUpdating)(::windows::core::Vtable::as_raw(this), cookie).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SelectionUpdating(&self, handler: &super::super::super::Foundation::TypedEventHandler<CoreTextEditContext, CoreTextSelectionUpdatingEventArgs>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).SelectionUpdating)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(handler), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveSelectionUpdating(&self, cookie: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).RemoveSelectionUpdating)(::windows::core::Vtable::as_raw(this), cookie).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn FormatUpdating(&self, handler: &super::super::super::Foundation::TypedEventHandler<CoreTextEditContext, CoreTextFormatUpdatingEventArgs>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).FormatUpdating)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(handler), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveFormatUpdating(&self, cookie: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).RemoveFormatUpdating)(::windows::core::Vtable::as_raw(this), cookie).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn CompositionStarted(&self, handler: &super::super::super::Foundation::TypedEventHandler<CoreTextEditContext, CoreTextCompositionStartedEventArgs>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CompositionStarted)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(handler), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveCompositionStarted(&self, cookie: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).RemoveCompositionStarted)(::windows::core::Vtable::as_raw(this), cookie).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn CompositionCompleted(&self, handler: &super::super::super::Foundation::TypedEventHandler<CoreTextEditContext, CoreTextCompositionCompletedEventArgs>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CompositionCompleted)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(handler), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveCompositionCompleted(&self, cookie: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).RemoveCompositionCompleted)(::windows::core::Vtable::as_raw(this), cookie).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn FocusRemoved(&self, handler: &super::super::super::Foundation::TypedEventHandler<CoreTextEditContext, ::windows::core::IInspectable>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).FocusRemoved)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(handler), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveFocusRemoved(&self, cookie: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).RemoveFocusRemoved)(::windows::core::Vtable::as_raw(this), cookie).ok() }
    }
    pub fn NotifyFocusEnter(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).NotifyFocusEnter)(::windows::core::Vtable::as_raw(this)).ok() }
    }
    pub fn NotifyFocusLeave(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).NotifyFocusLeave)(::windows::core::Vtable::as_raw(this)).ok() }
    }
    pub fn NotifyTextChanged(&self, modifiedrange: CoreTextRange, newlength: i32, newselection: CoreTextRange) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).NotifyTextChanged)(::windows::core::Vtable::as_raw(this), modifiedrange, newlength, newselection).ok() }
    }
    pub fn NotifySelectionChanged(&self, selection: CoreTextRange) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).NotifySelectionChanged)(::windows::core::Vtable::as_raw(this), selection).ok() }
    }
    pub fn NotifyLayoutChanged(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).NotifyLayoutChanged)(::windows::core::Vtable::as_raw(this)).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn NotifyFocusLeaveCompleted(&self, handler: &super::super::super::Foundation::TypedEventHandler<CoreTextEditContext, ::windows::core::IInspectable>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = &::windows::core::Interface::cast::<ICoreTextEditContext2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).NotifyFocusLeaveCompleted)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(handler), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveNotifyFocusLeaveCompleted(&self, cookie: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ICoreTextEditContext2>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).RemoveNotifyFocusLeaveCompleted)(::windows::core::Vtable::as_raw(this), cookie).ok() }
    }
}
impl ::core::clone::Clone for CoreTextEditContext {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for CoreTextEditContext {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for CoreTextEditContext {}
impl ::core::fmt::Debug for CoreTextEditContext {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CoreTextEditContext").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for CoreTextEditContext {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Text.Core.CoreTextEditContext;{bf6608af-4041-47c3-b263-a918eb5eaef2})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for CoreTextEditContext {
    type Vtable = ICoreTextEditContext_Vtbl;
}
unsafe impl ::windows::core::Interface for CoreTextEditContext {
    const IID: ::windows::core::GUID = <ICoreTextEditContext as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for CoreTextEditContext {
    const NAME: &'static str = "Windows.UI.Text.Core.CoreTextEditContext";
}
::windows::core::interface_hierarchy!(CoreTextEditContext, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for CoreTextEditContext {}
unsafe impl ::core::marker::Sync for CoreTextEditContext {}
#[doc = "*Required features: `\"UI_Text_Core\"`*"]
#[repr(transparent)]
pub struct CoreTextFormatUpdatingEventArgs(::windows::core::IUnknown);
impl CoreTextFormatUpdatingEventArgs {
    pub fn Range(&self) -> ::windows::core::Result<CoreTextRange> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Range)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"UI_ViewManagement\"`*"]
    #[cfg(all(feature = "Foundation", feature = "UI_ViewManagement"))]
    pub fn TextColor(&self) -> ::windows::core::Result<super::super::super::Foundation::IReference<super::super::ViewManagement::UIElementType>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).TextColor)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"UI_ViewManagement\"`*"]
    #[cfg(all(feature = "Foundation", feature = "UI_ViewManagement"))]
    pub fn BackgroundColor(&self) -> ::windows::core::Result<super::super::super::Foundation::IReference<super::super::ViewManagement::UIElementType>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).BackgroundColor)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"UI_ViewManagement\"`*"]
    #[cfg(all(feature = "Foundation", feature = "UI_ViewManagement"))]
    pub fn UnderlineColor(&self) -> ::windows::core::Result<super::super::super::Foundation::IReference<super::super::ViewManagement::UIElementType>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).UnderlineColor)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn UnderlineType(&self) -> ::windows::core::Result<super::super::super::Foundation::IReference<super::UnderlineType>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).UnderlineType)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn Reason(&self) -> ::windows::core::Result<CoreTextFormatUpdatingReason> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Reason)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn Result(&self) -> ::windows::core::Result<CoreTextFormatUpdatingResult> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Result)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetResult(&self, value: CoreTextFormatUpdatingResult) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetResult)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
    pub fn IsCanceled(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).IsCanceled)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn GetDeferral(&self) -> ::windows::core::Result<super::super::super::Foundation::Deferral> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetDeferral)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
}
impl ::core::clone::Clone for CoreTextFormatUpdatingEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for CoreTextFormatUpdatingEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for CoreTextFormatUpdatingEventArgs {}
impl ::core::fmt::Debug for CoreTextFormatUpdatingEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CoreTextFormatUpdatingEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for CoreTextFormatUpdatingEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Text.Core.CoreTextFormatUpdatingEventArgs;{7310bd33-b4a8-43b1-b37b-0724d4aca7ab})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for CoreTextFormatUpdatingEventArgs {
    type Vtable = ICoreTextFormatUpdatingEventArgs_Vtbl;
}
unsafe impl ::windows::core::Interface for CoreTextFormatUpdatingEventArgs {
    const IID: ::windows::core::GUID = <ICoreTextFormatUpdatingEventArgs as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for CoreTextFormatUpdatingEventArgs {
    const NAME: &'static str = "Windows.UI.Text.Core.CoreTextFormatUpdatingEventArgs";
}
::windows::core::interface_hierarchy!(CoreTextFormatUpdatingEventArgs, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for CoreTextFormatUpdatingEventArgs {}
unsafe impl ::core::marker::Sync for CoreTextFormatUpdatingEventArgs {}
#[doc = "*Required features: `\"UI_Text_Core\"`*"]
#[repr(transparent)]
pub struct CoreTextLayoutBounds(::windows::core::IUnknown);
impl CoreTextLayoutBounds {
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn TextBounds(&self) -> ::windows::core::Result<super::super::super::Foundation::Rect> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).TextBounds)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetTextBounds(&self, value: super::super::super::Foundation::Rect) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetTextBounds)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ControlBounds(&self) -> ::windows::core::Result<super::super::super::Foundation::Rect> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ControlBounds)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetControlBounds(&self, value: super::super::super::Foundation::Rect) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetControlBounds)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
}
impl ::core::clone::Clone for CoreTextLayoutBounds {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for CoreTextLayoutBounds {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for CoreTextLayoutBounds {}
impl ::core::fmt::Debug for CoreTextLayoutBounds {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CoreTextLayoutBounds").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for CoreTextLayoutBounds {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Text.Core.CoreTextLayoutBounds;{e972c974-4436-4917-80d0-a525e4ca6780})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for CoreTextLayoutBounds {
    type Vtable = ICoreTextLayoutBounds_Vtbl;
}
unsafe impl ::windows::core::Interface for CoreTextLayoutBounds {
    const IID: ::windows::core::GUID = <ICoreTextLayoutBounds as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for CoreTextLayoutBounds {
    const NAME: &'static str = "Windows.UI.Text.Core.CoreTextLayoutBounds";
}
::windows::core::interface_hierarchy!(CoreTextLayoutBounds, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for CoreTextLayoutBounds {}
unsafe impl ::core::marker::Sync for CoreTextLayoutBounds {}
#[doc = "*Required features: `\"UI_Text_Core\"`*"]
#[repr(transparent)]
pub struct CoreTextLayoutRequest(::windows::core::IUnknown);
impl CoreTextLayoutRequest {
    pub fn Range(&self) -> ::windows::core::Result<CoreTextRange> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Range)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn LayoutBounds(&self) -> ::windows::core::Result<CoreTextLayoutBounds> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).LayoutBounds)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn IsCanceled(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).IsCanceled)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn GetDeferral(&self) -> ::windows::core::Result<super::super::super::Foundation::Deferral> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetDeferral)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn LayoutBoundsVisualPixels(&self) -> ::windows::core::Result<CoreTextLayoutBounds> {
        let this = &::windows::core::Interface::cast::<ICoreTextLayoutRequest2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).LayoutBoundsVisualPixels)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
}
impl ::core::clone::Clone for CoreTextLayoutRequest {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for CoreTextLayoutRequest {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for CoreTextLayoutRequest {}
impl ::core::fmt::Debug for CoreTextLayoutRequest {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CoreTextLayoutRequest").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for CoreTextLayoutRequest {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Text.Core.CoreTextLayoutRequest;{2555a8cc-51fd-4f03-98bf-ac78174d68e0})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for CoreTextLayoutRequest {
    type Vtable = ICoreTextLayoutRequest_Vtbl;
}
unsafe impl ::windows::core::Interface for CoreTextLayoutRequest {
    const IID: ::windows::core::GUID = <ICoreTextLayoutRequest as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for CoreTextLayoutRequest {
    const NAME: &'static str = "Windows.UI.Text.Core.CoreTextLayoutRequest";
}
::windows::core::interface_hierarchy!(CoreTextLayoutRequest, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for CoreTextLayoutRequest {}
unsafe impl ::core::marker::Sync for CoreTextLayoutRequest {}
#[doc = "*Required features: `\"UI_Text_Core\"`*"]
#[repr(transparent)]
pub struct CoreTextLayoutRequestedEventArgs(::windows::core::IUnknown);
impl CoreTextLayoutRequestedEventArgs {
    pub fn Request(&self) -> ::windows::core::Result<CoreTextLayoutRequest> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Request)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
}
impl ::core::clone::Clone for CoreTextLayoutRequestedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for CoreTextLayoutRequestedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for CoreTextLayoutRequestedEventArgs {}
impl ::core::fmt::Debug for CoreTextLayoutRequestedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CoreTextLayoutRequestedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for CoreTextLayoutRequestedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Text.Core.CoreTextLayoutRequestedEventArgs;{b1dc6ae0-9a7b-4e9e-a566-4a6b5f8ad676})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for CoreTextLayoutRequestedEventArgs {
    type Vtable = ICoreTextLayoutRequestedEventArgs_Vtbl;
}
unsafe impl ::windows::core::Interface for CoreTextLayoutRequestedEventArgs {
    const IID: ::windows::core::GUID = <ICoreTextLayoutRequestedEventArgs as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for CoreTextLayoutRequestedEventArgs {
    const NAME: &'static str = "Windows.UI.Text.Core.CoreTextLayoutRequestedEventArgs";
}
::windows::core::interface_hierarchy!(CoreTextLayoutRequestedEventArgs, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for CoreTextLayoutRequestedEventArgs {}
unsafe impl ::core::marker::Sync for CoreTextLayoutRequestedEventArgs {}
#[doc = "*Required features: `\"UI_Text_Core\"`*"]
#[repr(transparent)]
pub struct CoreTextSelectionRequest(::windows::core::IUnknown);
impl CoreTextSelectionRequest {
    pub fn Selection(&self) -> ::windows::core::Result<CoreTextRange> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Selection)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetSelection(&self, value: CoreTextRange) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetSelection)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
    pub fn IsCanceled(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).IsCanceled)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn GetDeferral(&self) -> ::windows::core::Result<super::super::super::Foundation::Deferral> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetDeferral)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
}
impl ::core::clone::Clone for CoreTextSelectionRequest {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for CoreTextSelectionRequest {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for CoreTextSelectionRequest {}
impl ::core::fmt::Debug for CoreTextSelectionRequest {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CoreTextSelectionRequest").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for CoreTextSelectionRequest {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Text.Core.CoreTextSelectionRequest;{f0a70403-208b-4301-883c-74ca7485fd8d})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for CoreTextSelectionRequest {
    type Vtable = ICoreTextSelectionRequest_Vtbl;
}
unsafe impl ::windows::core::Interface for CoreTextSelectionRequest {
    const IID: ::windows::core::GUID = <ICoreTextSelectionRequest as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for CoreTextSelectionRequest {
    const NAME: &'static str = "Windows.UI.Text.Core.CoreTextSelectionRequest";
}
::windows::core::interface_hierarchy!(CoreTextSelectionRequest, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for CoreTextSelectionRequest {}
unsafe impl ::core::marker::Sync for CoreTextSelectionRequest {}
#[doc = "*Required features: `\"UI_Text_Core\"`*"]
#[repr(transparent)]
pub struct CoreTextSelectionRequestedEventArgs(::windows::core::IUnknown);
impl CoreTextSelectionRequestedEventArgs {
    pub fn Request(&self) -> ::windows::core::Result<CoreTextSelectionRequest> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Request)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
}
impl ::core::clone::Clone for CoreTextSelectionRequestedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for CoreTextSelectionRequestedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for CoreTextSelectionRequestedEventArgs {}
impl ::core::fmt::Debug for CoreTextSelectionRequestedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CoreTextSelectionRequestedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for CoreTextSelectionRequestedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Text.Core.CoreTextSelectionRequestedEventArgs;{13c6682b-f614-421a-8f4b-9ec8a5a37fcd})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for CoreTextSelectionRequestedEventArgs {
    type Vtable = ICoreTextSelectionRequestedEventArgs_Vtbl;
}
unsafe impl ::windows::core::Interface for CoreTextSelectionRequestedEventArgs {
    const IID: ::windows::core::GUID = <ICoreTextSelectionRequestedEventArgs as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for CoreTextSelectionRequestedEventArgs {
    const NAME: &'static str = "Windows.UI.Text.Core.CoreTextSelectionRequestedEventArgs";
}
::windows::core::interface_hierarchy!(CoreTextSelectionRequestedEventArgs, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for CoreTextSelectionRequestedEventArgs {}
unsafe impl ::core::marker::Sync for CoreTextSelectionRequestedEventArgs {}
#[doc = "*Required features: `\"UI_Text_Core\"`*"]
#[repr(transparent)]
pub struct CoreTextSelectionUpdatingEventArgs(::windows::core::IUnknown);
impl CoreTextSelectionUpdatingEventArgs {
    pub fn Selection(&self) -> ::windows::core::Result<CoreTextRange> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Selection)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn Result(&self) -> ::windows::core::Result<CoreTextSelectionUpdatingResult> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Result)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetResult(&self, value: CoreTextSelectionUpdatingResult) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetResult)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
    pub fn IsCanceled(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).IsCanceled)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn GetDeferral(&self) -> ::windows::core::Result<super::super::super::Foundation::Deferral> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetDeferral)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
}
impl ::core::clone::Clone for CoreTextSelectionUpdatingEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for CoreTextSelectionUpdatingEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for CoreTextSelectionUpdatingEventArgs {}
impl ::core::fmt::Debug for CoreTextSelectionUpdatingEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CoreTextSelectionUpdatingEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for CoreTextSelectionUpdatingEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Text.Core.CoreTextSelectionUpdatingEventArgs;{d445839f-fe7f-4bd5-8a26-0922c1b3e639})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for CoreTextSelectionUpdatingEventArgs {
    type Vtable = ICoreTextSelectionUpdatingEventArgs_Vtbl;
}
unsafe impl ::windows::core::Interface for CoreTextSelectionUpdatingEventArgs {
    const IID: ::windows::core::GUID = <ICoreTextSelectionUpdatingEventArgs as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for CoreTextSelectionUpdatingEventArgs {
    const NAME: &'static str = "Windows.UI.Text.Core.CoreTextSelectionUpdatingEventArgs";
}
::windows::core::interface_hierarchy!(CoreTextSelectionUpdatingEventArgs, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for CoreTextSelectionUpdatingEventArgs {}
unsafe impl ::core::marker::Sync for CoreTextSelectionUpdatingEventArgs {}
#[doc = "*Required features: `\"UI_Text_Core\"`*"]
pub struct CoreTextServicesConstants;
impl CoreTextServicesConstants {
    pub fn HiddenCharacter() -> ::windows::core::Result<u16> {
        Self::ICoreTextServicesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).HiddenCharacter)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn ICoreTextServicesStatics<R, F: FnOnce(&ICoreTextServicesStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<CoreTextServicesConstants, ICoreTextServicesStatics> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::windows::core::RuntimeName for CoreTextServicesConstants {
    const NAME: &'static str = "Windows.UI.Text.Core.CoreTextServicesConstants";
}
#[doc = "*Required features: `\"UI_Text_Core\"`*"]
#[repr(transparent)]
pub struct CoreTextServicesManager(::windows::core::IUnknown);
impl CoreTextServicesManager {
    #[doc = "*Required features: `\"Globalization\"`*"]
    #[cfg(feature = "Globalization")]
    pub fn InputLanguage(&self) -> ::windows::core::Result<super::super::super::Globalization::Language> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).InputLanguage)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn InputLanguageChanged(&self, handler: &super::super::super::Foundation::TypedEventHandler<CoreTextServicesManager, ::windows::core::IInspectable>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).InputLanguageChanged)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(handler), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveInputLanguageChanged(&self, cookie: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).RemoveInputLanguageChanged)(::windows::core::Vtable::as_raw(this), cookie).ok() }
    }
    pub fn CreateEditContext(&self) -> ::windows::core::Result<CoreTextEditContext> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CreateEditContext)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn GetForCurrentView() -> ::windows::core::Result<CoreTextServicesManager> {
        Self::ICoreTextServicesManagerStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetForCurrentView)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn ICoreTextServicesManagerStatics<R, F: FnOnce(&ICoreTextServicesManagerStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<CoreTextServicesManager, ICoreTextServicesManagerStatics> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::clone::Clone for CoreTextServicesManager {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for CoreTextServicesManager {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for CoreTextServicesManager {}
impl ::core::fmt::Debug for CoreTextServicesManager {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CoreTextServicesManager").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for CoreTextServicesManager {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Text.Core.CoreTextServicesManager;{c2507d83-6e0a-4a8a-bdf8-1948874854ba})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for CoreTextServicesManager {
    type Vtable = ICoreTextServicesManager_Vtbl;
}
unsafe impl ::windows::core::Interface for CoreTextServicesManager {
    const IID: ::windows::core::GUID = <ICoreTextServicesManager as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for CoreTextServicesManager {
    const NAME: &'static str = "Windows.UI.Text.Core.CoreTextServicesManager";
}
::windows::core::interface_hierarchy!(CoreTextServicesManager, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for CoreTextServicesManager {}
unsafe impl ::core::marker::Sync for CoreTextServicesManager {}
#[doc = "*Required features: `\"UI_Text_Core\"`*"]
#[repr(transparent)]
pub struct CoreTextTextRequest(::windows::core::IUnknown);
impl CoreTextTextRequest {
    pub fn Range(&self) -> ::windows::core::Result<CoreTextRange> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Range)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn Text(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Text)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetText(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetText)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn IsCanceled(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).IsCanceled)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn GetDeferral(&self) -> ::windows::core::Result<super::super::super::Foundation::Deferral> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetDeferral)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
}
impl ::core::clone::Clone for CoreTextTextRequest {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for CoreTextTextRequest {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for CoreTextTextRequest {}
impl ::core::fmt::Debug for CoreTextTextRequest {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CoreTextTextRequest").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for CoreTextTextRequest {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Text.Core.CoreTextTextRequest;{50d950a9-f51e-4cc1-8ca1-e6346d1a61be})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for CoreTextTextRequest {
    type Vtable = ICoreTextTextRequest_Vtbl;
}
unsafe impl ::windows::core::Interface for CoreTextTextRequest {
    const IID: ::windows::core::GUID = <ICoreTextTextRequest as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for CoreTextTextRequest {
    const NAME: &'static str = "Windows.UI.Text.Core.CoreTextTextRequest";
}
::windows::core::interface_hierarchy!(CoreTextTextRequest, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for CoreTextTextRequest {}
unsafe impl ::core::marker::Sync for CoreTextTextRequest {}
#[doc = "*Required features: `\"UI_Text_Core\"`*"]
#[repr(transparent)]
pub struct CoreTextTextRequestedEventArgs(::windows::core::IUnknown);
impl CoreTextTextRequestedEventArgs {
    pub fn Request(&self) -> ::windows::core::Result<CoreTextTextRequest> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Request)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
}
impl ::core::clone::Clone for CoreTextTextRequestedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for CoreTextTextRequestedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for CoreTextTextRequestedEventArgs {}
impl ::core::fmt::Debug for CoreTextTextRequestedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CoreTextTextRequestedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for CoreTextTextRequestedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Text.Core.CoreTextTextRequestedEventArgs;{f096a2d0-41c6-4c02-8b1a-d953b00cabb3})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for CoreTextTextRequestedEventArgs {
    type Vtable = ICoreTextTextRequestedEventArgs_Vtbl;
}
unsafe impl ::windows::core::Interface for CoreTextTextRequestedEventArgs {
    const IID: ::windows::core::GUID = <ICoreTextTextRequestedEventArgs as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for CoreTextTextRequestedEventArgs {
    const NAME: &'static str = "Windows.UI.Text.Core.CoreTextTextRequestedEventArgs";
}
::windows::core::interface_hierarchy!(CoreTextTextRequestedEventArgs, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for CoreTextTextRequestedEventArgs {}
unsafe impl ::core::marker::Sync for CoreTextTextRequestedEventArgs {}
#[doc = "*Required features: `\"UI_Text_Core\"`*"]
#[repr(transparent)]
pub struct CoreTextTextUpdatingEventArgs(::windows::core::IUnknown);
impl CoreTextTextUpdatingEventArgs {
    pub fn Range(&self) -> ::windows::core::Result<CoreTextRange> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Range)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn Text(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Text)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn NewSelection(&self) -> ::windows::core::Result<CoreTextRange> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).NewSelection)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Globalization\"`*"]
    #[cfg(feature = "Globalization")]
    pub fn InputLanguage(&self) -> ::windows::core::Result<super::super::super::Globalization::Language> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).InputLanguage)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn Result(&self) -> ::windows::core::Result<CoreTextTextUpdatingResult> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Result)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetResult(&self, value: CoreTextTextUpdatingResult) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetResult)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
    pub fn IsCanceled(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).IsCanceled)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn GetDeferral(&self) -> ::windows::core::Result<super::super::super::Foundation::Deferral> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetDeferral)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
}
impl ::core::clone::Clone for CoreTextTextUpdatingEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for CoreTextTextUpdatingEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for CoreTextTextUpdatingEventArgs {}
impl ::core::fmt::Debug for CoreTextTextUpdatingEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CoreTextTextUpdatingEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for CoreTextTextUpdatingEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Text.Core.CoreTextTextUpdatingEventArgs;{eea7918d-cc2b-4f03-8ff6-02fd217db450})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for CoreTextTextUpdatingEventArgs {
    type Vtable = ICoreTextTextUpdatingEventArgs_Vtbl;
}
unsafe impl ::windows::core::Interface for CoreTextTextUpdatingEventArgs {
    const IID: ::windows::core::GUID = <ICoreTextTextUpdatingEventArgs as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for CoreTextTextUpdatingEventArgs {
    const NAME: &'static str = "Windows.UI.Text.Core.CoreTextTextUpdatingEventArgs";
}
::windows::core::interface_hierarchy!(CoreTextTextUpdatingEventArgs, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for CoreTextTextUpdatingEventArgs {}
unsafe impl ::core::marker::Sync for CoreTextTextUpdatingEventArgs {}
#[doc = "*Required features: `\"UI_Text_Core\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct CoreTextFormatUpdatingReason(pub i32);
impl CoreTextFormatUpdatingReason {
    pub const None: Self = Self(0i32);
    pub const CompositionUnconverted: Self = Self(1i32);
    pub const CompositionConverted: Self = Self(2i32);
    pub const CompositionTargetUnconverted: Self = Self(3i32);
    pub const CompositionTargetConverted: Self = Self(4i32);
}
impl ::core::marker::Copy for CoreTextFormatUpdatingReason {}
impl ::core::clone::Clone for CoreTextFormatUpdatingReason {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for CoreTextFormatUpdatingReason {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for CoreTextFormatUpdatingReason {
    type Abi = Self;
}
impl ::core::fmt::Debug for CoreTextFormatUpdatingReason {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CoreTextFormatUpdatingReason").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for CoreTextFormatUpdatingReason {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.UI.Text.Core.CoreTextFormatUpdatingReason;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"UI_Text_Core\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct CoreTextFormatUpdatingResult(pub i32);
impl CoreTextFormatUpdatingResult {
    pub const Succeeded: Self = Self(0i32);
    pub const Failed: Self = Self(1i32);
}
impl ::core::marker::Copy for CoreTextFormatUpdatingResult {}
impl ::core::clone::Clone for CoreTextFormatUpdatingResult {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for CoreTextFormatUpdatingResult {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for CoreTextFormatUpdatingResult {
    type Abi = Self;
}
impl ::core::fmt::Debug for CoreTextFormatUpdatingResult {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CoreTextFormatUpdatingResult").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for CoreTextFormatUpdatingResult {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.UI.Text.Core.CoreTextFormatUpdatingResult;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"UI_Text_Core\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct CoreTextInputPaneDisplayPolicy(pub i32);
impl CoreTextInputPaneDisplayPolicy {
    pub const Automatic: Self = Self(0i32);
    pub const Manual: Self = Self(1i32);
}
impl ::core::marker::Copy for CoreTextInputPaneDisplayPolicy {}
impl ::core::clone::Clone for CoreTextInputPaneDisplayPolicy {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for CoreTextInputPaneDisplayPolicy {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for CoreTextInputPaneDisplayPolicy {
    type Abi = Self;
}
impl ::core::fmt::Debug for CoreTextInputPaneDisplayPolicy {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CoreTextInputPaneDisplayPolicy").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for CoreTextInputPaneDisplayPolicy {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.UI.Text.Core.CoreTextInputPaneDisplayPolicy;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"UI_Text_Core\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct CoreTextInputScope(pub i32);
impl CoreTextInputScope {
    pub const Default: Self = Self(0i32);
    pub const Url: Self = Self(1i32);
    pub const FilePath: Self = Self(2i32);
    pub const FileName: Self = Self(3i32);
    pub const EmailUserName: Self = Self(4i32);
    pub const EmailAddress: Self = Self(5i32);
    pub const UserName: Self = Self(6i32);
    pub const PersonalFullName: Self = Self(7i32);
    pub const PersonalNamePrefix: Self = Self(8i32);
    pub const PersonalGivenName: Self = Self(9i32);
    pub const PersonalMiddleName: Self = Self(10i32);
    pub const PersonalSurname: Self = Self(11i32);
    pub const PersonalNameSuffix: Self = Self(12i32);
    pub const Address: Self = Self(13i32);
    pub const AddressPostalCode: Self = Self(14i32);
    pub const AddressStreet: Self = Self(15i32);
    pub const AddressStateOrProvince: Self = Self(16i32);
    pub const AddressCity: Self = Self(17i32);
    pub const AddressCountryName: Self = Self(18i32);
    pub const AddressCountryShortName: Self = Self(19i32);
    pub const CurrencyAmountAndSymbol: Self = Self(20i32);
    pub const CurrencyAmount: Self = Self(21i32);
    pub const Date: Self = Self(22i32);
    pub const DateMonth: Self = Self(23i32);
    pub const DateDay: Self = Self(24i32);
    pub const DateYear: Self = Self(25i32);
    pub const DateMonthName: Self = Self(26i32);
    pub const DateDayName: Self = Self(27i32);
    pub const Number: Self = Self(29i32);
    pub const SingleCharacter: Self = Self(30i32);
    pub const Password: Self = Self(31i32);
    pub const TelephoneNumber: Self = Self(32i32);
    pub const TelephoneCountryCode: Self = Self(33i32);
    pub const TelephoneAreaCode: Self = Self(34i32);
    pub const TelephoneLocalNumber: Self = Self(35i32);
    pub const Time: Self = Self(36i32);
    pub const TimeHour: Self = Self(37i32);
    pub const TimeMinuteOrSecond: Self = Self(38i32);
    pub const NumberFullWidth: Self = Self(39i32);
    pub const AlphanumericHalfWidth: Self = Self(40i32);
    pub const AlphanumericFullWidth: Self = Self(41i32);
    pub const CurrencyChinese: Self = Self(42i32);
    pub const Bopomofo: Self = Self(43i32);
    pub const Hiragana: Self = Self(44i32);
    pub const KatakanaHalfWidth: Self = Self(45i32);
    pub const KatakanaFullWidth: Self = Self(46i32);
    pub const Hanja: Self = Self(47i32);
    pub const HangulHalfWidth: Self = Self(48i32);
    pub const HangulFullWidth: Self = Self(49i32);
    pub const Search: Self = Self(50i32);
    pub const Formula: Self = Self(51i32);
    pub const SearchIncremental: Self = Self(52i32);
    pub const ChineseHalfWidth: Self = Self(53i32);
    pub const ChineseFullWidth: Self = Self(54i32);
    pub const NativeScript: Self = Self(55i32);
    pub const Text: Self = Self(57i32);
    pub const Chat: Self = Self(58i32);
    pub const NameOrPhoneNumber: Self = Self(59i32);
    pub const EmailUserNameOrAddress: Self = Self(60i32);
    pub const Private: Self = Self(61i32);
    pub const Maps: Self = Self(62i32);
    pub const PasswordNumeric: Self = Self(63i32);
    pub const FormulaNumber: Self = Self(67i32);
    pub const ChatWithoutEmoji: Self = Self(68i32);
    pub const Digits: Self = Self(28i32);
    pub const PinNumeric: Self = Self(64i32);
    pub const PinAlphanumeric: Self = Self(65i32);
}
impl ::core::marker::Copy for CoreTextInputScope {}
impl ::core::clone::Clone for CoreTextInputScope {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for CoreTextInputScope {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for CoreTextInputScope {
    type Abi = Self;
}
impl ::core::fmt::Debug for CoreTextInputScope {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CoreTextInputScope").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for CoreTextInputScope {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.UI.Text.Core.CoreTextInputScope;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"UI_Text_Core\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct CoreTextSelectionUpdatingResult(pub i32);
impl CoreTextSelectionUpdatingResult {
    pub const Succeeded: Self = Self(0i32);
    pub const Failed: Self = Self(1i32);
}
impl ::core::marker::Copy for CoreTextSelectionUpdatingResult {}
impl ::core::clone::Clone for CoreTextSelectionUpdatingResult {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for CoreTextSelectionUpdatingResult {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for CoreTextSelectionUpdatingResult {
    type Abi = Self;
}
impl ::core::fmt::Debug for CoreTextSelectionUpdatingResult {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CoreTextSelectionUpdatingResult").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for CoreTextSelectionUpdatingResult {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.UI.Text.Core.CoreTextSelectionUpdatingResult;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"UI_Text_Core\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct CoreTextTextUpdatingResult(pub i32);
impl CoreTextTextUpdatingResult {
    pub const Succeeded: Self = Self(0i32);
    pub const Failed: Self = Self(1i32);
}
impl ::core::marker::Copy for CoreTextTextUpdatingResult {}
impl ::core::clone::Clone for CoreTextTextUpdatingResult {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for CoreTextTextUpdatingResult {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for CoreTextTextUpdatingResult {
    type Abi = Self;
}
impl ::core::fmt::Debug for CoreTextTextUpdatingResult {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CoreTextTextUpdatingResult").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for CoreTextTextUpdatingResult {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.UI.Text.Core.CoreTextTextUpdatingResult;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[repr(C)]
#[doc = "*Required features: `\"UI_Text_Core\"`*"]
pub struct CoreTextRange {
    pub StartCaretPosition: i32,
    pub EndCaretPosition: i32,
}
impl ::core::marker::Copy for CoreTextRange {}
impl ::core::clone::Clone for CoreTextRange {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for CoreTextRange {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CoreTextRange").field("StartCaretPosition", &self.StartCaretPosition).field("EndCaretPosition", &self.EndCaretPosition).finish()
    }
}
unsafe impl ::windows::core::Abi for CoreTextRange {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for CoreTextRange {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"struct(Windows.UI.Text.Core.CoreTextRange;i4;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
impl ::core::cmp::PartialEq for CoreTextRange {
    fn eq(&self, other: &Self) -> bool {
        self.StartCaretPosition == other.StartCaretPosition && self.EndCaretPosition == other.EndCaretPosition
    }
}
impl ::core::cmp::Eq for CoreTextRange {}
impl ::core::default::Default for CoreTextRange {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
