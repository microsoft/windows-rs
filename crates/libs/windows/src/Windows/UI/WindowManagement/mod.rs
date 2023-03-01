#[cfg(feature = "UI_WindowManagement_Preview")]
pub mod Preview;
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppWindow(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IAppWindow {
    type Vtable = IAppWindow_Vtbl;
}
impl ::core::clone::Clone for IAppWindow {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IAppWindow {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x663014a6_b75e_5dbd_995c_f0117fa3fb61);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppWindow_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Content: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "System")]
    pub DispatcherQueue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "System"))]
    DispatcherQueue: usize,
    pub Frame: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub IsVisible: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub PersistedStateId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub SetPersistedStateId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub Presenter: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Title: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub SetTitle: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub TitleBar: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub UIContext: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub WindowingEnvironment: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub CloseAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    CloseAsync: usize,
    pub GetPlacement: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub GetDisplayRegions: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetDisplayRegions: usize,
    pub RequestMoveToDisplayRegion: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, displayregion: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub RequestMoveAdjacentToCurrentView: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub RequestMoveAdjacentToWindow: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, anchorwindow: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub RequestMoveRelativeToWindowContent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, anchorwindow: *mut ::core::ffi::c_void, contentoffset: super::super::Foundation::Point) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RequestMoveRelativeToWindowContent: usize,
    #[cfg(feature = "Foundation")]
    pub RequestMoveRelativeToCurrentViewContent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, contentoffset: super::super::Foundation::Point) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RequestMoveRelativeToCurrentViewContent: usize,
    #[cfg(feature = "Foundation")]
    pub RequestMoveRelativeToDisplayRegion: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, displayregion: *mut ::core::ffi::c_void, displayregionoffset: super::super::Foundation::Point) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RequestMoveRelativeToDisplayRegion: usize,
    #[cfg(feature = "Foundation")]
    pub RequestSize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, framesize: super::super::Foundation::Size) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RequestSize: usize,
    #[cfg(feature = "Foundation")]
    pub TryShowAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    TryShowAsync: usize,
    #[cfg(feature = "Foundation")]
    pub Changed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Changed: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveChanged: usize,
    #[cfg(feature = "Foundation")]
    pub Closed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Closed: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveClosed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveClosed: usize,
    #[cfg(feature = "Foundation")]
    pub CloseRequested: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    CloseRequested: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveCloseRequested: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveCloseRequested: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppWindowChangedEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IAppWindowChangedEventArgs {
    type Vtable = IAppWindowChangedEventArgs_Vtbl;
}
impl ::core::clone::Clone for IAppWindowChangedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IAppWindowChangedEventArgs {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1de1f3be_a655_55ad_b2b6_eb240f880356);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppWindowChangedEventArgs_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub DidAvailableWindowPresentationsChange: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub DidDisplayRegionsChange: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub DidFrameChange: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub DidSizeChange: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub DidTitleBarChange: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub DidVisibilityChange: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub DidWindowingEnvironmentChange: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub DidWindowPresentationChange: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppWindowCloseRequestedEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IAppWindowCloseRequestedEventArgs {
    type Vtable = IAppWindowCloseRequestedEventArgs_Vtbl;
}
impl ::core::clone::Clone for IAppWindowCloseRequestedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IAppWindowCloseRequestedEventArgs {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe9ff01da_e7a2_57a8_8b5e_39c4003afdbb);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppWindowCloseRequestedEventArgs_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Cancel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetCancel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub GetDeferral: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetDeferral: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppWindowClosedEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IAppWindowClosedEventArgs {
    type Vtable = IAppWindowClosedEventArgs_Vtbl;
}
impl ::core::clone::Clone for IAppWindowClosedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IAppWindowClosedEventArgs {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xcc7df816_9520_5a06_821e_456ad8b358aa);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppWindowClosedEventArgs_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Reason: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut AppWindowClosedReason) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppWindowFrame(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IAppWindowFrame {
    type Vtable = IAppWindowFrame_Vtbl;
}
impl ::core::clone::Clone for IAppWindowFrame {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IAppWindowFrame {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9ee22601_7e5d_52af_846b_01dc6c296567);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppWindowFrame_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(all(feature = "Foundation_Collections", feature = "UI_Composition"))]
    pub DragRegionVisuals: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "UI_Composition")))]
    DragRegionVisuals: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppWindowFrameStyle(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IAppWindowFrameStyle {
    type Vtable = IAppWindowFrameStyle_Vtbl;
}
impl ::core::clone::Clone for IAppWindowFrameStyle {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IAppWindowFrameStyle {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xac412946_e1ac_5230_944a_c60873dcf4a9);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppWindowFrameStyle_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub GetFrameStyle: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut AppWindowFrameStyle) -> ::windows::core::HRESULT,
    pub SetFrameStyle: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, framestyle: AppWindowFrameStyle) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppWindowPlacement(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IAppWindowPlacement {
    type Vtable = IAppWindowPlacement_Vtbl;
}
impl ::core::clone::Clone for IAppWindowPlacement {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IAppWindowPlacement {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x03dc815e_e7a9_5857_9c03_7d670594410e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppWindowPlacement_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub DisplayRegion: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub Offset: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::Point) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Offset: usize,
    #[cfg(feature = "Foundation")]
    pub Size: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::Size) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Size: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppWindowPresentationConfiguration(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IAppWindowPresentationConfiguration {
    type Vtable = IAppWindowPresentationConfiguration_Vtbl;
}
impl ::core::clone::Clone for IAppWindowPresentationConfiguration {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IAppWindowPresentationConfiguration {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb5a43ee3_df33_5e67_bd31_1072457300df);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppWindowPresentationConfiguration_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Kind: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut AppWindowPresentationKind) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppWindowPresentationConfigurationFactory(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IAppWindowPresentationConfigurationFactory {
    type Vtable = IAppWindowPresentationConfigurationFactory_Vtbl;
}
impl ::core::clone::Clone for IAppWindowPresentationConfigurationFactory {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IAppWindowPresentationConfigurationFactory {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xfd3606a6_7875_5de8_84ff_6351ee13dd0d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppWindowPresentationConfigurationFactory_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppWindowPresenter(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IAppWindowPresenter {
    type Vtable = IAppWindowPresenter_Vtbl;
}
impl ::core::clone::Clone for IAppWindowPresenter {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IAppWindowPresenter {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5ae9ed73_e1fd_5317_ad78_5a3ed271bbde);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppWindowPresenter_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub GetConfiguration: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub IsPresentationSupported: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, presentationkind: AppWindowPresentationKind, result__: *mut bool) -> ::windows::core::HRESULT,
    pub RequestPresentation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, configuration: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub RequestPresentationByKind: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, presentationkind: AppWindowPresentationKind, result__: *mut bool) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppWindowStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IAppWindowStatics {
    type Vtable = IAppWindowStatics_Vtbl;
}
impl ::core::clone::Clone for IAppWindowStatics {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IAppWindowStatics {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xff1f3ea3_b769_50ef_9873_108cd0e89746);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppWindowStatics_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub TryCreateAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    TryCreateAsync: usize,
    pub ClearAllPersistedState: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub ClearPersistedState: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, key: ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppWindowTitleBar(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IAppWindowTitleBar {
    type Vtable = IAppWindowTitleBar_Vtbl;
}
impl ::core::clone::Clone for IAppWindowTitleBar {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IAppWindowTitleBar {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6e932c84_f644_541d_a2d7_0c262437842d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppWindowTitleBar_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub BackgroundColor: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    BackgroundColor: usize,
    #[cfg(feature = "Foundation")]
    pub SetBackgroundColor: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetBackgroundColor: usize,
    #[cfg(feature = "Foundation")]
    pub ButtonBackgroundColor: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ButtonBackgroundColor: usize,
    #[cfg(feature = "Foundation")]
    pub SetButtonBackgroundColor: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetButtonBackgroundColor: usize,
    #[cfg(feature = "Foundation")]
    pub ButtonForegroundColor: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ButtonForegroundColor: usize,
    #[cfg(feature = "Foundation")]
    pub SetButtonForegroundColor: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetButtonForegroundColor: usize,
    #[cfg(feature = "Foundation")]
    pub ButtonHoverBackgroundColor: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ButtonHoverBackgroundColor: usize,
    #[cfg(feature = "Foundation")]
    pub SetButtonHoverBackgroundColor: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetButtonHoverBackgroundColor: usize,
    #[cfg(feature = "Foundation")]
    pub ButtonHoverForegroundColor: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ButtonHoverForegroundColor: usize,
    #[cfg(feature = "Foundation")]
    pub SetButtonHoverForegroundColor: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetButtonHoverForegroundColor: usize,
    #[cfg(feature = "Foundation")]
    pub ButtonInactiveBackgroundColor: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ButtonInactiveBackgroundColor: usize,
    #[cfg(feature = "Foundation")]
    pub SetButtonInactiveBackgroundColor: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetButtonInactiveBackgroundColor: usize,
    #[cfg(feature = "Foundation")]
    pub ButtonInactiveForegroundColor: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ButtonInactiveForegroundColor: usize,
    #[cfg(feature = "Foundation")]
    pub SetButtonInactiveForegroundColor: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetButtonInactiveForegroundColor: usize,
    #[cfg(feature = "Foundation")]
    pub ButtonPressedBackgroundColor: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ButtonPressedBackgroundColor: usize,
    #[cfg(feature = "Foundation")]
    pub SetButtonPressedBackgroundColor: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetButtonPressedBackgroundColor: usize,
    #[cfg(feature = "Foundation")]
    pub ButtonPressedForegroundColor: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ButtonPressedForegroundColor: usize,
    #[cfg(feature = "Foundation")]
    pub SetButtonPressedForegroundColor: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetButtonPressedForegroundColor: usize,
    pub ExtendsContentIntoTitleBar: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetExtendsContentIntoTitleBar: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub ForegroundColor: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ForegroundColor: usize,
    #[cfg(feature = "Foundation")]
    pub SetForegroundColor: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetForegroundColor: usize,
    #[cfg(feature = "Foundation")]
    pub InactiveBackgroundColor: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    InactiveBackgroundColor: usize,
    #[cfg(feature = "Foundation")]
    pub SetInactiveBackgroundColor: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetInactiveBackgroundColor: usize,
    #[cfg(feature = "Foundation")]
    pub InactiveForegroundColor: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    InactiveForegroundColor: usize,
    #[cfg(feature = "Foundation")]
    pub SetInactiveForegroundColor: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetInactiveForegroundColor: usize,
    pub IsVisible: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub GetTitleBarOcclusions: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetTitleBarOcclusions: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppWindowTitleBarOcclusion(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IAppWindowTitleBarOcclusion {
    type Vtable = IAppWindowTitleBarOcclusion_Vtbl;
}
impl ::core::clone::Clone for IAppWindowTitleBarOcclusion {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IAppWindowTitleBarOcclusion {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xfea3cffd_2ccf_5fc3_aeae_f843876bf37e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppWindowTitleBarOcclusion_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub OccludingRect: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::Rect) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    OccludingRect: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppWindowTitleBarVisibility(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IAppWindowTitleBarVisibility {
    type Vtable = IAppWindowTitleBarVisibility_Vtbl;
}
impl ::core::clone::Clone for IAppWindowTitleBarVisibility {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IAppWindowTitleBarVisibility {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa215a4e3_6e7e_5651_8c3b_624819528154);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppWindowTitleBarVisibility_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub GetPreferredVisibility: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut AppWindowTitleBarVisibility) -> ::windows::core::HRESULT,
    pub SetPreferredVisibility: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, visibilitymode: AppWindowTitleBarVisibility) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICompactOverlayPresentationConfiguration(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ICompactOverlayPresentationConfiguration {
    type Vtable = ICompactOverlayPresentationConfiguration_Vtbl;
}
impl ::core::clone::Clone for ICompactOverlayPresentationConfiguration {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ICompactOverlayPresentationConfiguration {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa7e5750f_5730_56c6_8e1f_d63ff4d7980d);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICompactOverlayPresentationConfiguration_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IDefaultPresentationConfiguration(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IDefaultPresentationConfiguration {
    type Vtable = IDefaultPresentationConfiguration_Vtbl;
}
impl ::core::clone::Clone for IDefaultPresentationConfiguration {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IDefaultPresentationConfiguration {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd8c2b53b_2168_5703_a853_d525589fe2b9);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDefaultPresentationConfiguration_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IDisplayRegion(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IDisplayRegion {
    type Vtable = IDisplayRegion_Vtbl;
}
impl ::core::clone::Clone for IDisplayRegion {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IDisplayRegion {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xdb50c3a2_4094_5f47_8cb1_ea01ddafaa94);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDisplayRegion_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub DisplayMonitorDeviceId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub IsVisible: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub WorkAreaOffset: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::Point) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    WorkAreaOffset: usize,
    #[cfg(feature = "Foundation")]
    pub WorkAreaSize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::Size) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    WorkAreaSize: usize,
    pub WindowingEnvironment: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub Changed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Changed: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveChanged: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IFullScreenPresentationConfiguration(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IFullScreenPresentationConfiguration {
    type Vtable = IFullScreenPresentationConfiguration_Vtbl;
}
impl ::core::clone::Clone for IFullScreenPresentationConfiguration {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IFullScreenPresentationConfiguration {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x43d3dcd8_d2a8_503d_a626_15533d6d5f62);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFullScreenPresentationConfiguration_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub IsExclusive: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetIsExclusive: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IWindowServicesStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IWindowServicesStatics {
    type Vtable = IWindowServicesStatics_Vtbl;
}
impl ::core::clone::Clone for IWindowServicesStatics {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IWindowServicesStatics {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xcff4d519_50a6_5c64_97f6_c2d96add7f42);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWindowServicesStatics_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub FindAllTopLevelWindowIds: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    FindAllTopLevelWindowIds: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IWindowingEnvironment(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IWindowingEnvironment {
    type Vtable = IWindowingEnvironment_Vtbl;
}
impl ::core::clone::Clone for IWindowingEnvironment {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IWindowingEnvironment {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x264363c0_2a49_5417_b3ae_48a71c63a3bd);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWindowingEnvironment_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub IsEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub Kind: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut WindowingEnvironmentKind) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub GetDisplayRegions: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetDisplayRegions: usize,
    #[cfg(feature = "Foundation")]
    pub Changed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Changed: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveChanged: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IWindowingEnvironmentAddedEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IWindowingEnvironmentAddedEventArgs {
    type Vtable = IWindowingEnvironmentAddedEventArgs_Vtbl;
}
impl ::core::clone::Clone for IWindowingEnvironmentAddedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IWindowingEnvironmentAddedEventArgs {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xff2a5b7f_f183_5c66_99b2_429082069299);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWindowingEnvironmentAddedEventArgs_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub WindowingEnvironment: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IWindowingEnvironmentChangedEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IWindowingEnvironmentChangedEventArgs {
    type Vtable = IWindowingEnvironmentChangedEventArgs_Vtbl;
}
impl ::core::clone::Clone for IWindowingEnvironmentChangedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IWindowingEnvironmentChangedEventArgs {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4160cfc6_023d_5e9a_b431_350e67dc978a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWindowingEnvironmentChangedEventArgs_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IWindowingEnvironmentRemovedEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IWindowingEnvironmentRemovedEventArgs {
    type Vtable = IWindowingEnvironmentRemovedEventArgs_Vtbl;
}
impl ::core::clone::Clone for IWindowingEnvironmentRemovedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IWindowingEnvironmentRemovedEventArgs {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2e5b5473_beff_5e53_9316_7e775fe568b3);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWindowingEnvironmentRemovedEventArgs_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub WindowingEnvironment: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IWindowingEnvironmentStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IWindowingEnvironmentStatics {
    type Vtable = IWindowingEnvironmentStatics_Vtbl;
}
impl ::core::clone::Clone for IWindowingEnvironmentStatics {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IWindowingEnvironmentStatics {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x874e9fb7_c642_55ab_8aa2_162f734a9a72);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWindowingEnvironmentStatics_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub FindAll: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    FindAll: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub FindAllWithKind: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, kind: WindowingEnvironmentKind, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    FindAllWithKind: usize,
}
#[doc = "*Required features: `\"UI_WindowManagement\"`*"]
#[repr(transparent)]
pub struct AppWindow(::windows::core::IUnknown);
impl AppWindow {
    pub fn Content(&self) -> ::windows::core::Result<super::UIContentRoot> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::UIContentRoot>();
            (::windows::core::Interface::vtable(this).Content)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"System\"`*"]
    #[cfg(feature = "System")]
    pub fn DispatcherQueue(&self) -> ::windows::core::Result<super::super::System::DispatcherQueue> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::System::DispatcherQueue>();
            (::windows::core::Interface::vtable(this).DispatcherQueue)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Frame(&self) -> ::windows::core::Result<AppWindowFrame> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<AppWindowFrame>();
            (::windows::core::Interface::vtable(this).Frame)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn IsVisible(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<bool>();
            (::windows::core::Interface::vtable(this).IsVisible)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn PersistedStateId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).PersistedStateId)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetPersistedStateId(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetPersistedStateId)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn Presenter(&self) -> ::windows::core::Result<AppWindowPresenter> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<AppWindowPresenter>();
            (::windows::core::Interface::vtable(this).Presenter)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Title(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).Title)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetTitle(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetTitle)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn TitleBar(&self) -> ::windows::core::Result<AppWindowTitleBar> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<AppWindowTitleBar>();
            (::windows::core::Interface::vtable(this).TitleBar)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn UIContext(&self) -> ::windows::core::Result<super::UIContext> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::UIContext>();
            (::windows::core::Interface::vtable(this).UIContext)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn WindowingEnvironment(&self) -> ::windows::core::Result<WindowingEnvironment> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<WindowingEnvironment>();
            (::windows::core::Interface::vtable(this).WindowingEnvironment)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn CloseAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::IAsyncAction>();
            (::windows::core::Interface::vtable(this).CloseAsync)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn GetPlacement(&self) -> ::windows::core::Result<AppWindowPlacement> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<AppWindowPlacement>();
            (::windows::core::Interface::vtable(this).GetPlacement)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetDisplayRegions(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<DisplayRegion>> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::Collections::IVectorView<DisplayRegion>>();
            (::windows::core::Interface::vtable(this).GetDisplayRegions)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn RequestMoveToDisplayRegion(&self, displayregion: &DisplayRegion) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RequestMoveToDisplayRegion)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(displayregion)).ok() }
    }
    pub fn RequestMoveAdjacentToCurrentView(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RequestMoveAdjacentToCurrentView)(::windows::core::Interface::as_raw(this)).ok() }
    }
    pub fn RequestMoveAdjacentToWindow(&self, anchorwindow: &AppWindow) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RequestMoveAdjacentToWindow)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(anchorwindow)).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RequestMoveRelativeToWindowContent(&self, anchorwindow: &AppWindow, contentoffset: super::super::Foundation::Point) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RequestMoveRelativeToWindowContent)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(anchorwindow), contentoffset).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RequestMoveRelativeToCurrentViewContent(&self, contentoffset: super::super::Foundation::Point) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RequestMoveRelativeToCurrentViewContent)(::windows::core::Interface::as_raw(this), contentoffset).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RequestMoveRelativeToDisplayRegion(&self, displayregion: &DisplayRegion, displayregionoffset: super::super::Foundation::Point) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RequestMoveRelativeToDisplayRegion)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(displayregion), displayregionoffset).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RequestSize(&self, framesize: super::super::Foundation::Size) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RequestSize)(::windows::core::Interface::as_raw(this), framesize).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn TryShowAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::IAsyncOperation<bool>>();
            (::windows::core::Interface::vtable(this).TryShowAsync)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Changed(&self, handler: &super::super::Foundation::TypedEventHandler<AppWindow, AppWindowChangedEventArgs>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::EventRegistrationToken>();
            (::windows::core::Interface::vtable(this).Changed)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(handler), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveChanged(&self, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemoveChanged)(::windows::core::Interface::as_raw(this), token).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Closed(&self, handler: &super::super::Foundation::TypedEventHandler<AppWindow, AppWindowClosedEventArgs>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::EventRegistrationToken>();
            (::windows::core::Interface::vtable(this).Closed)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(handler), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveClosed(&self, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemoveClosed)(::windows::core::Interface::as_raw(this), token).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn CloseRequested(&self, handler: &super::super::Foundation::TypedEventHandler<AppWindow, AppWindowCloseRequestedEventArgs>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::EventRegistrationToken>();
            (::windows::core::Interface::vtable(this).CloseRequested)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(handler), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveCloseRequested(&self, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemoveCloseRequested)(::windows::core::Interface::as_raw(this), token).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn TryCreateAsync() -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<AppWindow>> {
        Self::IAppWindowStatics(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::IAsyncOperation<AppWindow>>();
            (::windows::core::Interface::vtable(this).TryCreateAsync)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn ClearAllPersistedState() -> ::windows::core::Result<()> {
        Self::IAppWindowStatics(|this| unsafe { (::windows::core::Interface::vtable(this).ClearAllPersistedState)(::windows::core::Interface::as_raw(this)).ok() })
    }
    pub fn ClearPersistedState(key: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        Self::IAppWindowStatics(|this| unsafe { (::windows::core::Interface::vtable(this).ClearPersistedState)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(key)).ok() })
    }
    #[doc(hidden)]
    pub fn IAppWindowStatics<R, F: FnOnce(&IAppWindowStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::imp::FactoryCache<AppWindow, IAppWindowStatics> = ::windows::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::cmp::PartialEq for AppWindow {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AppWindow {}
impl ::core::fmt::Debug for AppWindow {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AppWindow").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for AppWindow {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.UI.WindowManagement.AppWindow;{663014a6-b75e-5dbd-995c-f0117fa3fb61})");
}
impl ::core::clone::Clone for AppWindow {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for AppWindow {
    type Vtable = IAppWindow_Vtbl;
}
unsafe impl ::windows::core::ComInterface for AppWindow {
    const IID: ::windows::core::GUID = <IAppWindow as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for AppWindow {
    const NAME: &'static str = "Windows.UI.WindowManagement.AppWindow";
}
::windows::imp::interface_hierarchy!(AppWindow, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for AppWindow {}
unsafe impl ::core::marker::Sync for AppWindow {}
#[doc = "*Required features: `\"UI_WindowManagement\"`*"]
#[repr(transparent)]
pub struct AppWindowChangedEventArgs(::windows::core::IUnknown);
impl AppWindowChangedEventArgs {
    pub fn DidAvailableWindowPresentationsChange(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<bool>();
            (::windows::core::Interface::vtable(this).DidAvailableWindowPresentationsChange)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn DidDisplayRegionsChange(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<bool>();
            (::windows::core::Interface::vtable(this).DidDisplayRegionsChange)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn DidFrameChange(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<bool>();
            (::windows::core::Interface::vtable(this).DidFrameChange)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn DidSizeChange(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<bool>();
            (::windows::core::Interface::vtable(this).DidSizeChange)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn DidTitleBarChange(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<bool>();
            (::windows::core::Interface::vtable(this).DidTitleBarChange)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn DidVisibilityChange(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<bool>();
            (::windows::core::Interface::vtable(this).DidVisibilityChange)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn DidWindowingEnvironmentChange(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<bool>();
            (::windows::core::Interface::vtable(this).DidWindowingEnvironmentChange)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn DidWindowPresentationChange(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<bool>();
            (::windows::core::Interface::vtable(this).DidWindowPresentationChange)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for AppWindowChangedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AppWindowChangedEventArgs {}
impl ::core::fmt::Debug for AppWindowChangedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AppWindowChangedEventArgs").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for AppWindowChangedEventArgs {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.UI.WindowManagement.AppWindowChangedEventArgs;{1de1f3be-a655-55ad-b2b6-eb240f880356})");
}
impl ::core::clone::Clone for AppWindowChangedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for AppWindowChangedEventArgs {
    type Vtable = IAppWindowChangedEventArgs_Vtbl;
}
unsafe impl ::windows::core::ComInterface for AppWindowChangedEventArgs {
    const IID: ::windows::core::GUID = <IAppWindowChangedEventArgs as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for AppWindowChangedEventArgs {
    const NAME: &'static str = "Windows.UI.WindowManagement.AppWindowChangedEventArgs";
}
::windows::imp::interface_hierarchy!(AppWindowChangedEventArgs, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for AppWindowChangedEventArgs {}
unsafe impl ::core::marker::Sync for AppWindowChangedEventArgs {}
#[doc = "*Required features: `\"UI_WindowManagement\"`*"]
#[repr(transparent)]
pub struct AppWindowCloseRequestedEventArgs(::windows::core::IUnknown);
impl AppWindowCloseRequestedEventArgs {
    pub fn Cancel(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<bool>();
            (::windows::core::Interface::vtable(this).Cancel)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetCancel(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetCancel)(::windows::core::Interface::as_raw(this), value).ok() }
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
impl ::core::cmp::PartialEq for AppWindowCloseRequestedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AppWindowCloseRequestedEventArgs {}
impl ::core::fmt::Debug for AppWindowCloseRequestedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AppWindowCloseRequestedEventArgs").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for AppWindowCloseRequestedEventArgs {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.UI.WindowManagement.AppWindowCloseRequestedEventArgs;{e9ff01da-e7a2-57a8-8b5e-39c4003afdbb})");
}
impl ::core::clone::Clone for AppWindowCloseRequestedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for AppWindowCloseRequestedEventArgs {
    type Vtable = IAppWindowCloseRequestedEventArgs_Vtbl;
}
unsafe impl ::windows::core::ComInterface for AppWindowCloseRequestedEventArgs {
    const IID: ::windows::core::GUID = <IAppWindowCloseRequestedEventArgs as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for AppWindowCloseRequestedEventArgs {
    const NAME: &'static str = "Windows.UI.WindowManagement.AppWindowCloseRequestedEventArgs";
}
::windows::imp::interface_hierarchy!(AppWindowCloseRequestedEventArgs, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for AppWindowCloseRequestedEventArgs {}
unsafe impl ::core::marker::Sync for AppWindowCloseRequestedEventArgs {}
#[doc = "*Required features: `\"UI_WindowManagement\"`*"]
#[repr(transparent)]
pub struct AppWindowClosedEventArgs(::windows::core::IUnknown);
impl AppWindowClosedEventArgs {
    pub fn Reason(&self) -> ::windows::core::Result<AppWindowClosedReason> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<AppWindowClosedReason>();
            (::windows::core::Interface::vtable(this).Reason)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for AppWindowClosedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AppWindowClosedEventArgs {}
impl ::core::fmt::Debug for AppWindowClosedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AppWindowClosedEventArgs").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for AppWindowClosedEventArgs {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.UI.WindowManagement.AppWindowClosedEventArgs;{cc7df816-9520-5a06-821e-456ad8b358aa})");
}
impl ::core::clone::Clone for AppWindowClosedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for AppWindowClosedEventArgs {
    type Vtable = IAppWindowClosedEventArgs_Vtbl;
}
unsafe impl ::windows::core::ComInterface for AppWindowClosedEventArgs {
    const IID: ::windows::core::GUID = <IAppWindowClosedEventArgs as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for AppWindowClosedEventArgs {
    const NAME: &'static str = "Windows.UI.WindowManagement.AppWindowClosedEventArgs";
}
::windows::imp::interface_hierarchy!(AppWindowClosedEventArgs, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for AppWindowClosedEventArgs {}
unsafe impl ::core::marker::Sync for AppWindowClosedEventArgs {}
#[doc = "*Required features: `\"UI_WindowManagement\"`*"]
#[repr(transparent)]
pub struct AppWindowFrame(::windows::core::IUnknown);
impl AppWindowFrame {
    #[doc = "*Required features: `\"Foundation_Collections\"`, `\"UI_Composition\"`*"]
    #[cfg(all(feature = "Foundation_Collections", feature = "UI_Composition"))]
    pub fn DragRegionVisuals(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<super::Composition::IVisualElement>> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::Collections::IVector<super::Composition::IVisualElement>>();
            (::windows::core::Interface::vtable(this).DragRegionVisuals)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn GetFrameStyle(&self) -> ::windows::core::Result<AppWindowFrameStyle> {
        let this = &::windows::core::ComInterface::cast::<IAppWindowFrameStyle>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<AppWindowFrameStyle>();
            (::windows::core::Interface::vtable(this).GetFrameStyle)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetFrameStyle(&self, framestyle: AppWindowFrameStyle) -> ::windows::core::Result<()> {
        let this = &::windows::core::ComInterface::cast::<IAppWindowFrameStyle>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetFrameStyle)(::windows::core::Interface::as_raw(this), framestyle).ok() }
    }
}
impl ::core::cmp::PartialEq for AppWindowFrame {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AppWindowFrame {}
impl ::core::fmt::Debug for AppWindowFrame {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AppWindowFrame").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for AppWindowFrame {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.UI.WindowManagement.AppWindowFrame;{9ee22601-7e5d-52af-846b-01dc6c296567})");
}
impl ::core::clone::Clone for AppWindowFrame {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for AppWindowFrame {
    type Vtable = IAppWindowFrame_Vtbl;
}
unsafe impl ::windows::core::ComInterface for AppWindowFrame {
    const IID: ::windows::core::GUID = <IAppWindowFrame as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for AppWindowFrame {
    const NAME: &'static str = "Windows.UI.WindowManagement.AppWindowFrame";
}
::windows::imp::interface_hierarchy!(AppWindowFrame, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for AppWindowFrame {}
unsafe impl ::core::marker::Sync for AppWindowFrame {}
#[doc = "*Required features: `\"UI_WindowManagement\"`*"]
#[repr(transparent)]
pub struct AppWindowPlacement(::windows::core::IUnknown);
impl AppWindowPlacement {
    pub fn DisplayRegion(&self) -> ::windows::core::Result<DisplayRegion> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<DisplayRegion>();
            (::windows::core::Interface::vtable(this).DisplayRegion)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Offset(&self) -> ::windows::core::Result<super::super::Foundation::Point> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::Point>();
            (::windows::core::Interface::vtable(this).Offset)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Size(&self) -> ::windows::core::Result<super::super::Foundation::Size> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::Size>();
            (::windows::core::Interface::vtable(this).Size)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for AppWindowPlacement {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AppWindowPlacement {}
impl ::core::fmt::Debug for AppWindowPlacement {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AppWindowPlacement").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for AppWindowPlacement {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.UI.WindowManagement.AppWindowPlacement;{03dc815e-e7a9-5857-9c03-7d670594410e})");
}
impl ::core::clone::Clone for AppWindowPlacement {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for AppWindowPlacement {
    type Vtable = IAppWindowPlacement_Vtbl;
}
unsafe impl ::windows::core::ComInterface for AppWindowPlacement {
    const IID: ::windows::core::GUID = <IAppWindowPlacement as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for AppWindowPlacement {
    const NAME: &'static str = "Windows.UI.WindowManagement.AppWindowPlacement";
}
::windows::imp::interface_hierarchy!(AppWindowPlacement, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for AppWindowPlacement {}
unsafe impl ::core::marker::Sync for AppWindowPlacement {}
#[doc = "*Required features: `\"UI_WindowManagement\"`*"]
#[repr(transparent)]
pub struct AppWindowPresentationConfiguration(::windows::core::IUnknown);
impl AppWindowPresentationConfiguration {
    pub fn Kind(&self) -> ::windows::core::Result<AppWindowPresentationKind> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<AppWindowPresentationKind>();
            (::windows::core::Interface::vtable(this).Kind)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for AppWindowPresentationConfiguration {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AppWindowPresentationConfiguration {}
impl ::core::fmt::Debug for AppWindowPresentationConfiguration {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AppWindowPresentationConfiguration").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for AppWindowPresentationConfiguration {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.UI.WindowManagement.AppWindowPresentationConfiguration;{b5a43ee3-df33-5e67-bd31-1072457300df})");
}
impl ::core::clone::Clone for AppWindowPresentationConfiguration {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for AppWindowPresentationConfiguration {
    type Vtable = IAppWindowPresentationConfiguration_Vtbl;
}
unsafe impl ::windows::core::ComInterface for AppWindowPresentationConfiguration {
    const IID: ::windows::core::GUID = <IAppWindowPresentationConfiguration as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for AppWindowPresentationConfiguration {
    const NAME: &'static str = "Windows.UI.WindowManagement.AppWindowPresentationConfiguration";
}
::windows::imp::interface_hierarchy!(AppWindowPresentationConfiguration, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for AppWindowPresentationConfiguration {}
unsafe impl ::core::marker::Sync for AppWindowPresentationConfiguration {}
#[doc = "*Required features: `\"UI_WindowManagement\"`*"]
#[repr(transparent)]
pub struct AppWindowPresenter(::windows::core::IUnknown);
impl AppWindowPresenter {
    pub fn GetConfiguration(&self) -> ::windows::core::Result<AppWindowPresentationConfiguration> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<AppWindowPresentationConfiguration>();
            (::windows::core::Interface::vtable(this).GetConfiguration)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn IsPresentationSupported(&self, presentationkind: AppWindowPresentationKind) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<bool>();
            (::windows::core::Interface::vtable(this).IsPresentationSupported)(::windows::core::Interface::as_raw(this), presentationkind, &mut result__).from_abi(result__)
        }
    }
    pub fn RequestPresentation<P0>(&self, configuration: P0) -> ::windows::core::Result<bool>
    where
        P0: ::windows::core::TryIntoParam<AppWindowPresentationConfiguration>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<bool>();
            (::windows::core::Interface::vtable(this).RequestPresentation)(::windows::core::Interface::as_raw(this), configuration.try_into_param()?.abi(), &mut result__).from_abi(result__)
        }
    }
    pub fn RequestPresentationByKind(&self, presentationkind: AppWindowPresentationKind) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<bool>();
            (::windows::core::Interface::vtable(this).RequestPresentationByKind)(::windows::core::Interface::as_raw(this), presentationkind, &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for AppWindowPresenter {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AppWindowPresenter {}
impl ::core::fmt::Debug for AppWindowPresenter {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AppWindowPresenter").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for AppWindowPresenter {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.UI.WindowManagement.AppWindowPresenter;{5ae9ed73-e1fd-5317-ad78-5a3ed271bbde})");
}
impl ::core::clone::Clone for AppWindowPresenter {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for AppWindowPresenter {
    type Vtable = IAppWindowPresenter_Vtbl;
}
unsafe impl ::windows::core::ComInterface for AppWindowPresenter {
    const IID: ::windows::core::GUID = <IAppWindowPresenter as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for AppWindowPresenter {
    const NAME: &'static str = "Windows.UI.WindowManagement.AppWindowPresenter";
}
::windows::imp::interface_hierarchy!(AppWindowPresenter, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for AppWindowPresenter {}
unsafe impl ::core::marker::Sync for AppWindowPresenter {}
#[doc = "*Required features: `\"UI_WindowManagement\"`*"]
#[repr(transparent)]
pub struct AppWindowTitleBar(::windows::core::IUnknown);
impl AppWindowTitleBar {
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn BackgroundColor(&self) -> ::windows::core::Result<super::super::Foundation::IReference<super::Color>> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::IReference<super::Color>>();
            (::windows::core::Interface::vtable(this).BackgroundColor)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetBackgroundColor<P0>(&self, value: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::TryIntoParam<super::super::Foundation::IReference<super::Color>>,
    {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetBackgroundColor)(::windows::core::Interface::as_raw(this), value.try_into_param()?.abi()).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ButtonBackgroundColor(&self) -> ::windows::core::Result<super::super::Foundation::IReference<super::Color>> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::IReference<super::Color>>();
            (::windows::core::Interface::vtable(this).ButtonBackgroundColor)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetButtonBackgroundColor<P0>(&self, value: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::TryIntoParam<super::super::Foundation::IReference<super::Color>>,
    {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetButtonBackgroundColor)(::windows::core::Interface::as_raw(this), value.try_into_param()?.abi()).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ButtonForegroundColor(&self) -> ::windows::core::Result<super::super::Foundation::IReference<super::Color>> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::IReference<super::Color>>();
            (::windows::core::Interface::vtable(this).ButtonForegroundColor)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetButtonForegroundColor<P0>(&self, value: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::TryIntoParam<super::super::Foundation::IReference<super::Color>>,
    {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetButtonForegroundColor)(::windows::core::Interface::as_raw(this), value.try_into_param()?.abi()).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ButtonHoverBackgroundColor(&self) -> ::windows::core::Result<super::super::Foundation::IReference<super::Color>> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::IReference<super::Color>>();
            (::windows::core::Interface::vtable(this).ButtonHoverBackgroundColor)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetButtonHoverBackgroundColor<P0>(&self, value: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::TryIntoParam<super::super::Foundation::IReference<super::Color>>,
    {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetButtonHoverBackgroundColor)(::windows::core::Interface::as_raw(this), value.try_into_param()?.abi()).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ButtonHoverForegroundColor(&self) -> ::windows::core::Result<super::super::Foundation::IReference<super::Color>> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::IReference<super::Color>>();
            (::windows::core::Interface::vtable(this).ButtonHoverForegroundColor)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetButtonHoverForegroundColor<P0>(&self, value: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::TryIntoParam<super::super::Foundation::IReference<super::Color>>,
    {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetButtonHoverForegroundColor)(::windows::core::Interface::as_raw(this), value.try_into_param()?.abi()).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ButtonInactiveBackgroundColor(&self) -> ::windows::core::Result<super::super::Foundation::IReference<super::Color>> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::IReference<super::Color>>();
            (::windows::core::Interface::vtable(this).ButtonInactiveBackgroundColor)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetButtonInactiveBackgroundColor<P0>(&self, value: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::TryIntoParam<super::super::Foundation::IReference<super::Color>>,
    {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetButtonInactiveBackgroundColor)(::windows::core::Interface::as_raw(this), value.try_into_param()?.abi()).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ButtonInactiveForegroundColor(&self) -> ::windows::core::Result<super::super::Foundation::IReference<super::Color>> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::IReference<super::Color>>();
            (::windows::core::Interface::vtable(this).ButtonInactiveForegroundColor)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetButtonInactiveForegroundColor<P0>(&self, value: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::TryIntoParam<super::super::Foundation::IReference<super::Color>>,
    {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetButtonInactiveForegroundColor)(::windows::core::Interface::as_raw(this), value.try_into_param()?.abi()).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ButtonPressedBackgroundColor(&self) -> ::windows::core::Result<super::super::Foundation::IReference<super::Color>> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::IReference<super::Color>>();
            (::windows::core::Interface::vtable(this).ButtonPressedBackgroundColor)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetButtonPressedBackgroundColor<P0>(&self, value: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::TryIntoParam<super::super::Foundation::IReference<super::Color>>,
    {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetButtonPressedBackgroundColor)(::windows::core::Interface::as_raw(this), value.try_into_param()?.abi()).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ButtonPressedForegroundColor(&self) -> ::windows::core::Result<super::super::Foundation::IReference<super::Color>> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::IReference<super::Color>>();
            (::windows::core::Interface::vtable(this).ButtonPressedForegroundColor)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetButtonPressedForegroundColor<P0>(&self, value: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::TryIntoParam<super::super::Foundation::IReference<super::Color>>,
    {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetButtonPressedForegroundColor)(::windows::core::Interface::as_raw(this), value.try_into_param()?.abi()).ok() }
    }
    pub fn ExtendsContentIntoTitleBar(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<bool>();
            (::windows::core::Interface::vtable(this).ExtendsContentIntoTitleBar)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetExtendsContentIntoTitleBar(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetExtendsContentIntoTitleBar)(::windows::core::Interface::as_raw(this), value).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ForegroundColor(&self) -> ::windows::core::Result<super::super::Foundation::IReference<super::Color>> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::IReference<super::Color>>();
            (::windows::core::Interface::vtable(this).ForegroundColor)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetForegroundColor<P0>(&self, value: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::TryIntoParam<super::super::Foundation::IReference<super::Color>>,
    {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetForegroundColor)(::windows::core::Interface::as_raw(this), value.try_into_param()?.abi()).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn InactiveBackgroundColor(&self) -> ::windows::core::Result<super::super::Foundation::IReference<super::Color>> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::IReference<super::Color>>();
            (::windows::core::Interface::vtable(this).InactiveBackgroundColor)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetInactiveBackgroundColor<P0>(&self, value: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::TryIntoParam<super::super::Foundation::IReference<super::Color>>,
    {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetInactiveBackgroundColor)(::windows::core::Interface::as_raw(this), value.try_into_param()?.abi()).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn InactiveForegroundColor(&self) -> ::windows::core::Result<super::super::Foundation::IReference<super::Color>> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::IReference<super::Color>>();
            (::windows::core::Interface::vtable(this).InactiveForegroundColor)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetInactiveForegroundColor<P0>(&self, value: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::TryIntoParam<super::super::Foundation::IReference<super::Color>>,
    {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetInactiveForegroundColor)(::windows::core::Interface::as_raw(this), value.try_into_param()?.abi()).ok() }
    }
    pub fn IsVisible(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<bool>();
            (::windows::core::Interface::vtable(this).IsVisible)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetTitleBarOcclusions(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<AppWindowTitleBarOcclusion>> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::Collections::IVectorView<AppWindowTitleBarOcclusion>>();
            (::windows::core::Interface::vtable(this).GetTitleBarOcclusions)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn GetPreferredVisibility(&self) -> ::windows::core::Result<AppWindowTitleBarVisibility> {
        let this = &::windows::core::ComInterface::cast::<IAppWindowTitleBarVisibility>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<AppWindowTitleBarVisibility>();
            (::windows::core::Interface::vtable(this).GetPreferredVisibility)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetPreferredVisibility(&self, visibilitymode: AppWindowTitleBarVisibility) -> ::windows::core::Result<()> {
        let this = &::windows::core::ComInterface::cast::<IAppWindowTitleBarVisibility>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetPreferredVisibility)(::windows::core::Interface::as_raw(this), visibilitymode).ok() }
    }
}
impl ::core::cmp::PartialEq for AppWindowTitleBar {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AppWindowTitleBar {}
impl ::core::fmt::Debug for AppWindowTitleBar {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AppWindowTitleBar").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for AppWindowTitleBar {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.UI.WindowManagement.AppWindowTitleBar;{6e932c84-f644-541d-a2d7-0c262437842d})");
}
impl ::core::clone::Clone for AppWindowTitleBar {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for AppWindowTitleBar {
    type Vtable = IAppWindowTitleBar_Vtbl;
}
unsafe impl ::windows::core::ComInterface for AppWindowTitleBar {
    const IID: ::windows::core::GUID = <IAppWindowTitleBar as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for AppWindowTitleBar {
    const NAME: &'static str = "Windows.UI.WindowManagement.AppWindowTitleBar";
}
::windows::imp::interface_hierarchy!(AppWindowTitleBar, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for AppWindowTitleBar {}
unsafe impl ::core::marker::Sync for AppWindowTitleBar {}
#[doc = "*Required features: `\"UI_WindowManagement\"`*"]
#[repr(transparent)]
pub struct AppWindowTitleBarOcclusion(::windows::core::IUnknown);
impl AppWindowTitleBarOcclusion {
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn OccludingRect(&self) -> ::windows::core::Result<super::super::Foundation::Rect> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::Rect>();
            (::windows::core::Interface::vtable(this).OccludingRect)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for AppWindowTitleBarOcclusion {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AppWindowTitleBarOcclusion {}
impl ::core::fmt::Debug for AppWindowTitleBarOcclusion {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AppWindowTitleBarOcclusion").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for AppWindowTitleBarOcclusion {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.UI.WindowManagement.AppWindowTitleBarOcclusion;{fea3cffd-2ccf-5fc3-aeae-f843876bf37e})");
}
impl ::core::clone::Clone for AppWindowTitleBarOcclusion {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for AppWindowTitleBarOcclusion {
    type Vtable = IAppWindowTitleBarOcclusion_Vtbl;
}
unsafe impl ::windows::core::ComInterface for AppWindowTitleBarOcclusion {
    const IID: ::windows::core::GUID = <IAppWindowTitleBarOcclusion as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for AppWindowTitleBarOcclusion {
    const NAME: &'static str = "Windows.UI.WindowManagement.AppWindowTitleBarOcclusion";
}
::windows::imp::interface_hierarchy!(AppWindowTitleBarOcclusion, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for AppWindowTitleBarOcclusion {}
unsafe impl ::core::marker::Sync for AppWindowTitleBarOcclusion {}
#[doc = "*Required features: `\"UI_WindowManagement\"`*"]
#[repr(transparent)]
pub struct CompactOverlayPresentationConfiguration(::windows::core::IUnknown);
impl CompactOverlayPresentationConfiguration {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::imp::IGenericFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::imp::FactoryCache<CompactOverlayPresentationConfiguration, ::windows::imp::IGenericFactory> = ::windows::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn Kind(&self) -> ::windows::core::Result<AppWindowPresentationKind> {
        let this = &::windows::core::ComInterface::cast::<IAppWindowPresentationConfiguration>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<AppWindowPresentationKind>();
            (::windows::core::Interface::vtable(this).Kind)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for CompactOverlayPresentationConfiguration {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for CompactOverlayPresentationConfiguration {}
impl ::core::fmt::Debug for CompactOverlayPresentationConfiguration {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CompactOverlayPresentationConfiguration").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for CompactOverlayPresentationConfiguration {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.UI.WindowManagement.CompactOverlayPresentationConfiguration;{a7e5750f-5730-56c6-8e1f-d63ff4d7980d})");
}
impl ::core::clone::Clone for CompactOverlayPresentationConfiguration {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for CompactOverlayPresentationConfiguration {
    type Vtable = ICompactOverlayPresentationConfiguration_Vtbl;
}
unsafe impl ::windows::core::ComInterface for CompactOverlayPresentationConfiguration {
    const IID: ::windows::core::GUID = <ICompactOverlayPresentationConfiguration as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for CompactOverlayPresentationConfiguration {
    const NAME: &'static str = "Windows.UI.WindowManagement.CompactOverlayPresentationConfiguration";
}
::windows::imp::interface_hierarchy!(CompactOverlayPresentationConfiguration, ::windows::core::IUnknown, ::windows::core::IInspectable);
impl ::windows::core::CanTryInto<AppWindowPresentationConfiguration> for CompactOverlayPresentationConfiguration {}
unsafe impl ::core::marker::Send for CompactOverlayPresentationConfiguration {}
unsafe impl ::core::marker::Sync for CompactOverlayPresentationConfiguration {}
#[doc = "*Required features: `\"UI_WindowManagement\"`*"]
#[repr(transparent)]
pub struct DefaultPresentationConfiguration(::windows::core::IUnknown);
impl DefaultPresentationConfiguration {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::imp::IGenericFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::imp::FactoryCache<DefaultPresentationConfiguration, ::windows::imp::IGenericFactory> = ::windows::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn Kind(&self) -> ::windows::core::Result<AppWindowPresentationKind> {
        let this = &::windows::core::ComInterface::cast::<IAppWindowPresentationConfiguration>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<AppWindowPresentationKind>();
            (::windows::core::Interface::vtable(this).Kind)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for DefaultPresentationConfiguration {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for DefaultPresentationConfiguration {}
impl ::core::fmt::Debug for DefaultPresentationConfiguration {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DefaultPresentationConfiguration").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for DefaultPresentationConfiguration {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.UI.WindowManagement.DefaultPresentationConfiguration;{d8c2b53b-2168-5703-a853-d525589fe2b9})");
}
impl ::core::clone::Clone for DefaultPresentationConfiguration {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for DefaultPresentationConfiguration {
    type Vtable = IDefaultPresentationConfiguration_Vtbl;
}
unsafe impl ::windows::core::ComInterface for DefaultPresentationConfiguration {
    const IID: ::windows::core::GUID = <IDefaultPresentationConfiguration as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for DefaultPresentationConfiguration {
    const NAME: &'static str = "Windows.UI.WindowManagement.DefaultPresentationConfiguration";
}
::windows::imp::interface_hierarchy!(DefaultPresentationConfiguration, ::windows::core::IUnknown, ::windows::core::IInspectable);
impl ::windows::core::CanTryInto<AppWindowPresentationConfiguration> for DefaultPresentationConfiguration {}
unsafe impl ::core::marker::Send for DefaultPresentationConfiguration {}
unsafe impl ::core::marker::Sync for DefaultPresentationConfiguration {}
#[doc = "*Required features: `\"UI_WindowManagement\"`*"]
#[repr(transparent)]
pub struct DisplayRegion(::windows::core::IUnknown);
impl DisplayRegion {
    pub fn DisplayMonitorDeviceId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).DisplayMonitorDeviceId)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn IsVisible(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<bool>();
            (::windows::core::Interface::vtable(this).IsVisible)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn WorkAreaOffset(&self) -> ::windows::core::Result<super::super::Foundation::Point> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::Point>();
            (::windows::core::Interface::vtable(this).WorkAreaOffset)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn WorkAreaSize(&self) -> ::windows::core::Result<super::super::Foundation::Size> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::Size>();
            (::windows::core::Interface::vtable(this).WorkAreaSize)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn WindowingEnvironment(&self) -> ::windows::core::Result<WindowingEnvironment> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<WindowingEnvironment>();
            (::windows::core::Interface::vtable(this).WindowingEnvironment)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Changed(&self, handler: &super::super::Foundation::TypedEventHandler<DisplayRegion, ::windows::core::IInspectable>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::EventRegistrationToken>();
            (::windows::core::Interface::vtable(this).Changed)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(handler), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveChanged(&self, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemoveChanged)(::windows::core::Interface::as_raw(this), token).ok() }
    }
}
impl ::core::cmp::PartialEq for DisplayRegion {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for DisplayRegion {}
impl ::core::fmt::Debug for DisplayRegion {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DisplayRegion").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for DisplayRegion {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.UI.WindowManagement.DisplayRegion;{db50c3a2-4094-5f47-8cb1-ea01ddafaa94})");
}
impl ::core::clone::Clone for DisplayRegion {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for DisplayRegion {
    type Vtable = IDisplayRegion_Vtbl;
}
unsafe impl ::windows::core::ComInterface for DisplayRegion {
    const IID: ::windows::core::GUID = <IDisplayRegion as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for DisplayRegion {
    const NAME: &'static str = "Windows.UI.WindowManagement.DisplayRegion";
}
::windows::imp::interface_hierarchy!(DisplayRegion, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for DisplayRegion {}
unsafe impl ::core::marker::Sync for DisplayRegion {}
#[doc = "*Required features: `\"UI_WindowManagement\"`*"]
#[repr(transparent)]
pub struct FullScreenPresentationConfiguration(::windows::core::IUnknown);
impl FullScreenPresentationConfiguration {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::imp::IGenericFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::imp::FactoryCache<FullScreenPresentationConfiguration, ::windows::imp::IGenericFactory> = ::windows::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn Kind(&self) -> ::windows::core::Result<AppWindowPresentationKind> {
        let this = &::windows::core::ComInterface::cast::<IAppWindowPresentationConfiguration>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<AppWindowPresentationKind>();
            (::windows::core::Interface::vtable(this).Kind)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn IsExclusive(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<bool>();
            (::windows::core::Interface::vtable(this).IsExclusive)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetIsExclusive(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetIsExclusive)(::windows::core::Interface::as_raw(this), value).ok() }
    }
}
impl ::core::cmp::PartialEq for FullScreenPresentationConfiguration {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for FullScreenPresentationConfiguration {}
impl ::core::fmt::Debug for FullScreenPresentationConfiguration {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FullScreenPresentationConfiguration").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for FullScreenPresentationConfiguration {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.UI.WindowManagement.FullScreenPresentationConfiguration;{43d3dcd8-d2a8-503d-a626-15533d6d5f62})");
}
impl ::core::clone::Clone for FullScreenPresentationConfiguration {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for FullScreenPresentationConfiguration {
    type Vtable = IFullScreenPresentationConfiguration_Vtbl;
}
unsafe impl ::windows::core::ComInterface for FullScreenPresentationConfiguration {
    const IID: ::windows::core::GUID = <IFullScreenPresentationConfiguration as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for FullScreenPresentationConfiguration {
    const NAME: &'static str = "Windows.UI.WindowManagement.FullScreenPresentationConfiguration";
}
::windows::imp::interface_hierarchy!(FullScreenPresentationConfiguration, ::windows::core::IUnknown, ::windows::core::IInspectable);
impl ::windows::core::CanTryInto<AppWindowPresentationConfiguration> for FullScreenPresentationConfiguration {}
unsafe impl ::core::marker::Send for FullScreenPresentationConfiguration {}
unsafe impl ::core::marker::Sync for FullScreenPresentationConfiguration {}
#[doc = "*Required features: `\"UI_WindowManagement\"`*"]
pub struct WindowServices;
impl WindowServices {
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn FindAllTopLevelWindowIds() -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<super::WindowId>> {
        Self::IWindowServicesStatics(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::Collections::IVectorView<super::WindowId>>();
            (::windows::core::Interface::vtable(this).FindAllTopLevelWindowIds)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IWindowServicesStatics<R, F: FnOnce(&IWindowServicesStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::imp::FactoryCache<WindowServices, IWindowServicesStatics> = ::windows::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::windows::core::RuntimeName for WindowServices {
    const NAME: &'static str = "Windows.UI.WindowManagement.WindowServices";
}
#[doc = "*Required features: `\"UI_WindowManagement\"`*"]
#[repr(transparent)]
pub struct WindowingEnvironment(::windows::core::IUnknown);
impl WindowingEnvironment {
    pub fn IsEnabled(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<bool>();
            (::windows::core::Interface::vtable(this).IsEnabled)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Kind(&self) -> ::windows::core::Result<WindowingEnvironmentKind> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<WindowingEnvironmentKind>();
            (::windows::core::Interface::vtable(this).Kind)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetDisplayRegions(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<DisplayRegion>> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::Collections::IVectorView<DisplayRegion>>();
            (::windows::core::Interface::vtable(this).GetDisplayRegions)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Changed(&self, handler: &super::super::Foundation::TypedEventHandler<WindowingEnvironment, WindowingEnvironmentChangedEventArgs>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::EventRegistrationToken>();
            (::windows::core::Interface::vtable(this).Changed)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(handler), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveChanged(&self, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemoveChanged)(::windows::core::Interface::as_raw(this), token).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn FindAll() -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<WindowingEnvironment>> {
        Self::IWindowingEnvironmentStatics(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::Collections::IVectorView<WindowingEnvironment>>();
            (::windows::core::Interface::vtable(this).FindAll)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn FindAllWithKind(kind: WindowingEnvironmentKind) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<WindowingEnvironment>> {
        Self::IWindowingEnvironmentStatics(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::Collections::IVectorView<WindowingEnvironment>>();
            (::windows::core::Interface::vtable(this).FindAllWithKind)(::windows::core::Interface::as_raw(this), kind, &mut result__).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IWindowingEnvironmentStatics<R, F: FnOnce(&IWindowingEnvironmentStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::imp::FactoryCache<WindowingEnvironment, IWindowingEnvironmentStatics> = ::windows::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::cmp::PartialEq for WindowingEnvironment {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for WindowingEnvironment {}
impl ::core::fmt::Debug for WindowingEnvironment {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WindowingEnvironment").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for WindowingEnvironment {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.UI.WindowManagement.WindowingEnvironment;{264363c0-2a49-5417-b3ae-48a71c63a3bd})");
}
impl ::core::clone::Clone for WindowingEnvironment {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for WindowingEnvironment {
    type Vtable = IWindowingEnvironment_Vtbl;
}
unsafe impl ::windows::core::ComInterface for WindowingEnvironment {
    const IID: ::windows::core::GUID = <IWindowingEnvironment as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for WindowingEnvironment {
    const NAME: &'static str = "Windows.UI.WindowManagement.WindowingEnvironment";
}
::windows::imp::interface_hierarchy!(WindowingEnvironment, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for WindowingEnvironment {}
unsafe impl ::core::marker::Sync for WindowingEnvironment {}
#[doc = "*Required features: `\"UI_WindowManagement\"`*"]
#[repr(transparent)]
pub struct WindowingEnvironmentAddedEventArgs(::windows::core::IUnknown);
impl WindowingEnvironmentAddedEventArgs {
    pub fn WindowingEnvironment(&self) -> ::windows::core::Result<WindowingEnvironment> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<WindowingEnvironment>();
            (::windows::core::Interface::vtable(this).WindowingEnvironment)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for WindowingEnvironmentAddedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for WindowingEnvironmentAddedEventArgs {}
impl ::core::fmt::Debug for WindowingEnvironmentAddedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WindowingEnvironmentAddedEventArgs").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for WindowingEnvironmentAddedEventArgs {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.UI.WindowManagement.WindowingEnvironmentAddedEventArgs;{ff2a5b7f-f183-5c66-99b2-429082069299})");
}
impl ::core::clone::Clone for WindowingEnvironmentAddedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for WindowingEnvironmentAddedEventArgs {
    type Vtable = IWindowingEnvironmentAddedEventArgs_Vtbl;
}
unsafe impl ::windows::core::ComInterface for WindowingEnvironmentAddedEventArgs {
    const IID: ::windows::core::GUID = <IWindowingEnvironmentAddedEventArgs as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for WindowingEnvironmentAddedEventArgs {
    const NAME: &'static str = "Windows.UI.WindowManagement.WindowingEnvironmentAddedEventArgs";
}
::windows::imp::interface_hierarchy!(WindowingEnvironmentAddedEventArgs, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for WindowingEnvironmentAddedEventArgs {}
unsafe impl ::core::marker::Sync for WindowingEnvironmentAddedEventArgs {}
#[doc = "*Required features: `\"UI_WindowManagement\"`*"]
#[repr(transparent)]
pub struct WindowingEnvironmentChangedEventArgs(::windows::core::IUnknown);
impl WindowingEnvironmentChangedEventArgs {}
impl ::core::cmp::PartialEq for WindowingEnvironmentChangedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for WindowingEnvironmentChangedEventArgs {}
impl ::core::fmt::Debug for WindowingEnvironmentChangedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WindowingEnvironmentChangedEventArgs").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for WindowingEnvironmentChangedEventArgs {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.UI.WindowManagement.WindowingEnvironmentChangedEventArgs;{4160cfc6-023d-5e9a-b431-350e67dc978a})");
}
impl ::core::clone::Clone for WindowingEnvironmentChangedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for WindowingEnvironmentChangedEventArgs {
    type Vtable = IWindowingEnvironmentChangedEventArgs_Vtbl;
}
unsafe impl ::windows::core::ComInterface for WindowingEnvironmentChangedEventArgs {
    const IID: ::windows::core::GUID = <IWindowingEnvironmentChangedEventArgs as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for WindowingEnvironmentChangedEventArgs {
    const NAME: &'static str = "Windows.UI.WindowManagement.WindowingEnvironmentChangedEventArgs";
}
::windows::imp::interface_hierarchy!(WindowingEnvironmentChangedEventArgs, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for WindowingEnvironmentChangedEventArgs {}
unsafe impl ::core::marker::Sync for WindowingEnvironmentChangedEventArgs {}
#[doc = "*Required features: `\"UI_WindowManagement\"`*"]
#[repr(transparent)]
pub struct WindowingEnvironmentRemovedEventArgs(::windows::core::IUnknown);
impl WindowingEnvironmentRemovedEventArgs {
    pub fn WindowingEnvironment(&self) -> ::windows::core::Result<WindowingEnvironment> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<WindowingEnvironment>();
            (::windows::core::Interface::vtable(this).WindowingEnvironment)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for WindowingEnvironmentRemovedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for WindowingEnvironmentRemovedEventArgs {}
impl ::core::fmt::Debug for WindowingEnvironmentRemovedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WindowingEnvironmentRemovedEventArgs").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for WindowingEnvironmentRemovedEventArgs {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.UI.WindowManagement.WindowingEnvironmentRemovedEventArgs;{2e5b5473-beff-5e53-9316-7e775fe568b3})");
}
impl ::core::clone::Clone for WindowingEnvironmentRemovedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for WindowingEnvironmentRemovedEventArgs {
    type Vtable = IWindowingEnvironmentRemovedEventArgs_Vtbl;
}
unsafe impl ::windows::core::ComInterface for WindowingEnvironmentRemovedEventArgs {
    const IID: ::windows::core::GUID = <IWindowingEnvironmentRemovedEventArgs as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for WindowingEnvironmentRemovedEventArgs {
    const NAME: &'static str = "Windows.UI.WindowManagement.WindowingEnvironmentRemovedEventArgs";
}
::windows::imp::interface_hierarchy!(WindowingEnvironmentRemovedEventArgs, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for WindowingEnvironmentRemovedEventArgs {}
unsafe impl ::core::marker::Sync for WindowingEnvironmentRemovedEventArgs {}
#[doc = "*Required features: `\"UI_WindowManagement\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct AppWindowClosedReason(pub i32);
impl AppWindowClosedReason {
    pub const Other: Self = Self(0i32);
    pub const AppInitiated: Self = Self(1i32);
    pub const UserInitiated: Self = Self(2i32);
}
impl ::core::marker::Copy for AppWindowClosedReason {}
impl ::core::clone::Clone for AppWindowClosedReason {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for AppWindowClosedReason {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for AppWindowClosedReason {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for AppWindowClosedReason {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AppWindowClosedReason").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for AppWindowClosedReason {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"enum(Windows.UI.WindowManagement.AppWindowClosedReason;i4)");
}
#[doc = "*Required features: `\"UI_WindowManagement\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct AppWindowFrameStyle(pub i32);
impl AppWindowFrameStyle {
    pub const Default: Self = Self(0i32);
    pub const NoFrame: Self = Self(1i32);
}
impl ::core::marker::Copy for AppWindowFrameStyle {}
impl ::core::clone::Clone for AppWindowFrameStyle {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for AppWindowFrameStyle {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for AppWindowFrameStyle {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for AppWindowFrameStyle {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AppWindowFrameStyle").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for AppWindowFrameStyle {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"enum(Windows.UI.WindowManagement.AppWindowFrameStyle;i4)");
}
#[doc = "*Required features: `\"UI_WindowManagement\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct AppWindowPresentationKind(pub i32);
impl AppWindowPresentationKind {
    pub const Default: Self = Self(0i32);
    pub const CompactOverlay: Self = Self(1i32);
    pub const FullScreen: Self = Self(2i32);
}
impl ::core::marker::Copy for AppWindowPresentationKind {}
impl ::core::clone::Clone for AppWindowPresentationKind {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for AppWindowPresentationKind {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for AppWindowPresentationKind {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for AppWindowPresentationKind {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AppWindowPresentationKind").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for AppWindowPresentationKind {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"enum(Windows.UI.WindowManagement.AppWindowPresentationKind;i4)");
}
#[doc = "*Required features: `\"UI_WindowManagement\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct AppWindowTitleBarVisibility(pub i32);
impl AppWindowTitleBarVisibility {
    pub const Default: Self = Self(0i32);
    pub const AlwaysHidden: Self = Self(1i32);
}
impl ::core::marker::Copy for AppWindowTitleBarVisibility {}
impl ::core::clone::Clone for AppWindowTitleBarVisibility {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for AppWindowTitleBarVisibility {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for AppWindowTitleBarVisibility {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for AppWindowTitleBarVisibility {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AppWindowTitleBarVisibility").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for AppWindowTitleBarVisibility {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"enum(Windows.UI.WindowManagement.AppWindowTitleBarVisibility;i4)");
}
#[doc = "*Required features: `\"UI_WindowManagement\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct WindowingEnvironmentKind(pub i32);
impl WindowingEnvironmentKind {
    pub const Unknown: Self = Self(0i32);
    pub const Overlapped: Self = Self(1i32);
    pub const Tiled: Self = Self(2i32);
}
impl ::core::marker::Copy for WindowingEnvironmentKind {}
impl ::core::clone::Clone for WindowingEnvironmentKind {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WindowingEnvironmentKind {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for WindowingEnvironmentKind {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for WindowingEnvironmentKind {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WindowingEnvironmentKind").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for WindowingEnvironmentKind {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"enum(Windows.UI.WindowManagement.WindowingEnvironmentKind;i4)");
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
