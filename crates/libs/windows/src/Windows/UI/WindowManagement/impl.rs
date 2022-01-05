#[cfg(feature = "implement_exclusive")]
pub trait IAppWindowImpl: Sized {
    fn Content(&self) -> ::windows::core::Result<super::UIContentRoot>;
    fn DispatcherQueue(&self) -> ::windows::core::Result<super::super::System::DispatcherQueue>;
    fn Frame(&self) -> ::windows::core::Result<AppWindowFrame>;
    fn IsVisible(&self) -> ::windows::core::Result<bool>;
    fn PersistedStateId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetPersistedStateId(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn Presenter(&self) -> ::windows::core::Result<AppWindowPresenter>;
    fn Title(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetTitle(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn TitleBar(&self) -> ::windows::core::Result<AppWindowTitleBar>;
    fn UIContext(&self) -> ::windows::core::Result<super::UIContext>;
    fn WindowingEnvironment(&self) -> ::windows::core::Result<WindowingEnvironment>;
    fn CloseAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
    fn GetPlacement(&self) -> ::windows::core::Result<AppWindowPlacement>;
    fn GetDisplayRegions(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<DisplayRegion>>;
    fn RequestMoveToDisplayRegion(&self, displayregion: &::core::option::Option<DisplayRegion>) -> ::windows::core::Result<()>;
    fn RequestMoveAdjacentToCurrentView(&self) -> ::windows::core::Result<()>;
    fn RequestMoveAdjacentToWindow(&self, anchorwindow: &::core::option::Option<AppWindow>) -> ::windows::core::Result<()>;
    fn RequestMoveRelativeToWindowContent(&self, anchorwindow: &::core::option::Option<AppWindow>, contentoffset: &super::super::Foundation::Point) -> ::windows::core::Result<()>;
    fn RequestMoveRelativeToCurrentViewContent(&self, contentoffset: &super::super::Foundation::Point) -> ::windows::core::Result<()>;
    fn RequestMoveRelativeToDisplayRegion(&self, displayregion: &::core::option::Option<DisplayRegion>, displayregionoffset: &super::super::Foundation::Point) -> ::windows::core::Result<()>;
    fn RequestSize(&self, framesize: &super::super::Foundation::Size) -> ::windows::core::Result<()>;
    fn TryShowAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>>;
    fn Changed(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<AppWindow, AppWindowChangedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveChanged(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn Closed(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<AppWindow, AppWindowClosedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveClosed(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn CloseRequested(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<AppWindow, AppWindowCloseRequestedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveCloseRequested(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IAppWindowChangedEventArgsImpl: Sized {
    fn DidAvailableWindowPresentationsChange(&self) -> ::windows::core::Result<bool>;
    fn DidDisplayRegionsChange(&self) -> ::windows::core::Result<bool>;
    fn DidFrameChange(&self) -> ::windows::core::Result<bool>;
    fn DidSizeChange(&self) -> ::windows::core::Result<bool>;
    fn DidTitleBarChange(&self) -> ::windows::core::Result<bool>;
    fn DidVisibilityChange(&self) -> ::windows::core::Result<bool>;
    fn DidWindowingEnvironmentChange(&self) -> ::windows::core::Result<bool>;
    fn DidWindowPresentationChange(&self) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IAppWindowCloseRequestedEventArgsImpl: Sized {
    fn Cancel(&self) -> ::windows::core::Result<bool>;
    fn SetCancel(&self, value: bool) -> ::windows::core::Result<()>;
    fn GetDeferral(&self) -> ::windows::core::Result<super::super::Foundation::Deferral>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IAppWindowClosedEventArgsImpl: Sized {
    fn Reason(&self) -> ::windows::core::Result<AppWindowClosedReason>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IAppWindowFrameImpl: Sized {
    fn DragRegionVisuals(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<super::Composition::IVisualElement>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IAppWindowFrameStyleImpl: Sized {
    fn GetFrameStyle(&self) -> ::windows::core::Result<AppWindowFrameStyle>;
    fn SetFrameStyle(&self, framestyle: AppWindowFrameStyle) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IAppWindowPlacementImpl: Sized {
    fn DisplayRegion(&self) -> ::windows::core::Result<DisplayRegion>;
    fn Offset(&self) -> ::windows::core::Result<super::super::Foundation::Point>;
    fn Size(&self) -> ::windows::core::Result<super::super::Foundation::Size>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IAppWindowPresentationConfigurationImpl: Sized {
    fn Kind(&self) -> ::windows::core::Result<AppWindowPresentationKind>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IAppWindowPresentationConfigurationFactoryImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
pub trait IAppWindowPresenterImpl: Sized {
    fn GetConfiguration(&self) -> ::windows::core::Result<AppWindowPresentationConfiguration>;
    fn IsPresentationSupported(&self, presentationkind: AppWindowPresentationKind) -> ::windows::core::Result<bool>;
    fn RequestPresentation(&self, configuration: &::core::option::Option<AppWindowPresentationConfiguration>) -> ::windows::core::Result<bool>;
    fn RequestPresentationByKind(&self, presentationkind: AppWindowPresentationKind) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IAppWindowStaticsImpl: Sized {
    fn TryCreateAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<AppWindow>>;
    fn ClearAllPersistedState(&self) -> ::windows::core::Result<()>;
    fn ClearPersistedState(&self, key: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IAppWindowTitleBarImpl: Sized {
    fn BackgroundColor(&self) -> ::windows::core::Result<super::super::Foundation::IReference<super::Color>>;
    fn SetBackgroundColor(&self, value: &::core::option::Option<super::super::Foundation::IReference<super::Color>>) -> ::windows::core::Result<()>;
    fn ButtonBackgroundColor(&self) -> ::windows::core::Result<super::super::Foundation::IReference<super::Color>>;
    fn SetButtonBackgroundColor(&self, value: &::core::option::Option<super::super::Foundation::IReference<super::Color>>) -> ::windows::core::Result<()>;
    fn ButtonForegroundColor(&self) -> ::windows::core::Result<super::super::Foundation::IReference<super::Color>>;
    fn SetButtonForegroundColor(&self, value: &::core::option::Option<super::super::Foundation::IReference<super::Color>>) -> ::windows::core::Result<()>;
    fn ButtonHoverBackgroundColor(&self) -> ::windows::core::Result<super::super::Foundation::IReference<super::Color>>;
    fn SetButtonHoverBackgroundColor(&self, value: &::core::option::Option<super::super::Foundation::IReference<super::Color>>) -> ::windows::core::Result<()>;
    fn ButtonHoverForegroundColor(&self) -> ::windows::core::Result<super::super::Foundation::IReference<super::Color>>;
    fn SetButtonHoverForegroundColor(&self, value: &::core::option::Option<super::super::Foundation::IReference<super::Color>>) -> ::windows::core::Result<()>;
    fn ButtonInactiveBackgroundColor(&self) -> ::windows::core::Result<super::super::Foundation::IReference<super::Color>>;
    fn SetButtonInactiveBackgroundColor(&self, value: &::core::option::Option<super::super::Foundation::IReference<super::Color>>) -> ::windows::core::Result<()>;
    fn ButtonInactiveForegroundColor(&self) -> ::windows::core::Result<super::super::Foundation::IReference<super::Color>>;
    fn SetButtonInactiveForegroundColor(&self, value: &::core::option::Option<super::super::Foundation::IReference<super::Color>>) -> ::windows::core::Result<()>;
    fn ButtonPressedBackgroundColor(&self) -> ::windows::core::Result<super::super::Foundation::IReference<super::Color>>;
    fn SetButtonPressedBackgroundColor(&self, value: &::core::option::Option<super::super::Foundation::IReference<super::Color>>) -> ::windows::core::Result<()>;
    fn ButtonPressedForegroundColor(&self) -> ::windows::core::Result<super::super::Foundation::IReference<super::Color>>;
    fn SetButtonPressedForegroundColor(&self, value: &::core::option::Option<super::super::Foundation::IReference<super::Color>>) -> ::windows::core::Result<()>;
    fn ExtendsContentIntoTitleBar(&self) -> ::windows::core::Result<bool>;
    fn SetExtendsContentIntoTitleBar(&self, value: bool) -> ::windows::core::Result<()>;
    fn ForegroundColor(&self) -> ::windows::core::Result<super::super::Foundation::IReference<super::Color>>;
    fn SetForegroundColor(&self, value: &::core::option::Option<super::super::Foundation::IReference<super::Color>>) -> ::windows::core::Result<()>;
    fn InactiveBackgroundColor(&self) -> ::windows::core::Result<super::super::Foundation::IReference<super::Color>>;
    fn SetInactiveBackgroundColor(&self, value: &::core::option::Option<super::super::Foundation::IReference<super::Color>>) -> ::windows::core::Result<()>;
    fn InactiveForegroundColor(&self) -> ::windows::core::Result<super::super::Foundation::IReference<super::Color>>;
    fn SetInactiveForegroundColor(&self, value: &::core::option::Option<super::super::Foundation::IReference<super::Color>>) -> ::windows::core::Result<()>;
    fn IsVisible(&self) -> ::windows::core::Result<bool>;
    fn GetTitleBarOcclusions(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<AppWindowTitleBarOcclusion>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IAppWindowTitleBarOcclusionImpl: Sized {
    fn OccludingRect(&self) -> ::windows::core::Result<super::super::Foundation::Rect>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IAppWindowTitleBarVisibilityImpl: Sized {
    fn GetPreferredVisibility(&self) -> ::windows::core::Result<AppWindowTitleBarVisibility>;
    fn SetPreferredVisibility(&self, visibilitymode: AppWindowTitleBarVisibility) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ICompactOverlayPresentationConfigurationImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
pub trait IDefaultPresentationConfigurationImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
pub trait IDisplayRegionImpl: Sized {
    fn DisplayMonitorDeviceId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn IsVisible(&self) -> ::windows::core::Result<bool>;
    fn WorkAreaOffset(&self) -> ::windows::core::Result<super::super::Foundation::Point>;
    fn WorkAreaSize(&self) -> ::windows::core::Result<super::super::Foundation::Size>;
    fn WindowingEnvironment(&self) -> ::windows::core::Result<WindowingEnvironment>;
    fn Changed(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<DisplayRegion, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveChanged(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IFullScreenPresentationConfigurationImpl: Sized {
    fn IsExclusive(&self) -> ::windows::core::Result<bool>;
    fn SetIsExclusive(&self, value: bool) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IWindowServicesStaticsImpl: Sized {
    fn FindAllTopLevelWindowIds(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<super::WindowId>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IWindowingEnvironmentImpl: Sized {
    fn IsEnabled(&self) -> ::windows::core::Result<bool>;
    fn Kind(&self) -> ::windows::core::Result<WindowingEnvironmentKind>;
    fn GetDisplayRegions(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<DisplayRegion>>;
    fn Changed(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<WindowingEnvironment, WindowingEnvironmentChangedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveChanged(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IWindowingEnvironmentAddedEventArgsImpl: Sized {
    fn WindowingEnvironment(&self) -> ::windows::core::Result<WindowingEnvironment>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IWindowingEnvironmentChangedEventArgsImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
pub trait IWindowingEnvironmentRemovedEventArgsImpl: Sized {
    fn WindowingEnvironment(&self) -> ::windows::core::Result<WindowingEnvironment>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IWindowingEnvironmentStaticsImpl: Sized {
    fn FindAll(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<WindowingEnvironment>>;
    fn FindAllWithKind(&self, kind: WindowingEnvironmentKind) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<WindowingEnvironment>>;
}
