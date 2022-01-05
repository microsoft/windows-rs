#[cfg(feature = "implement_exclusive")]
pub trait IAccessibilitySettingsImpl: Sized {
    fn HighContrast(&self) -> ::windows::core::Result<bool>;
    fn HighContrastScheme(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn HighContrastChanged(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<AccessibilitySettings, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveHighContrastChanged(&self, cookie: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IActivationViewSwitcherImpl: Sized {
    fn ShowAsStandaloneAsync(&self, viewid: i32) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
    fn ShowAsStandaloneWithSizePreferenceAsync(&self, viewid: i32, sizepreference: ViewSizePreference) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
    fn IsViewPresentedOnActivationVirtualDesktop(&self, viewid: i32) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IApplicationViewImpl: Sized {
    fn Orientation(&self) -> ::windows::core::Result<ApplicationViewOrientation>;
    fn AdjacentToLeftDisplayEdge(&self) -> ::windows::core::Result<bool>;
    fn AdjacentToRightDisplayEdge(&self) -> ::windows::core::Result<bool>;
    fn IsFullScreen(&self) -> ::windows::core::Result<bool>;
    fn IsOnLockScreen(&self) -> ::windows::core::Result<bool>;
    fn IsScreenCaptureEnabled(&self) -> ::windows::core::Result<bool>;
    fn SetIsScreenCaptureEnabled(&self, value: bool) -> ::windows::core::Result<()>;
    fn SetTitle(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn Title(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Id(&self) -> ::windows::core::Result<i32>;
    fn Consolidated(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<ApplicationView, ApplicationViewConsolidatedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveConsolidated(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IApplicationView2Impl: Sized {
    fn SuppressSystemOverlays(&self) -> ::windows::core::Result<bool>;
    fn SetSuppressSystemOverlays(&self, value: bool) -> ::windows::core::Result<()>;
    fn VisibleBounds(&self) -> ::windows::core::Result<super::super::Foundation::Rect>;
    fn VisibleBoundsChanged(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<ApplicationView, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveVisibleBoundsChanged(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn SetDesiredBoundsMode(&self, boundsmode: ApplicationViewBoundsMode) -> ::windows::core::Result<bool>;
    fn DesiredBoundsMode(&self) -> ::windows::core::Result<ApplicationViewBoundsMode>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IApplicationView3Impl: Sized {
    fn TitleBar(&self) -> ::windows::core::Result<ApplicationViewTitleBar>;
    fn FullScreenSystemOverlayMode(&self) -> ::windows::core::Result<FullScreenSystemOverlayMode>;
    fn SetFullScreenSystemOverlayMode(&self, value: FullScreenSystemOverlayMode) -> ::windows::core::Result<()>;
    fn IsFullScreenMode(&self) -> ::windows::core::Result<bool>;
    fn TryEnterFullScreenMode(&self) -> ::windows::core::Result<bool>;
    fn ExitFullScreenMode(&self) -> ::windows::core::Result<()>;
    fn ShowStandardSystemOverlays(&self) -> ::windows::core::Result<()>;
    fn TryResizeView(&self, value: &super::super::Foundation::Size) -> ::windows::core::Result<bool>;
    fn SetPreferredMinSize(&self, minsize: &super::super::Foundation::Size) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IApplicationView4Impl: Sized {
    fn ViewMode(&self) -> ::windows::core::Result<ApplicationViewMode>;
    fn IsViewModeSupported(&self, viewmode: ApplicationViewMode) -> ::windows::core::Result<bool>;
    fn TryEnterViewModeAsync(&self, viewmode: ApplicationViewMode) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>>;
    fn TryEnterViewModeWithPreferencesAsync(&self, viewmode: ApplicationViewMode, viewmodepreferences: &::core::option::Option<ViewModePreferences>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>>;
    fn TryConsolidateAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IApplicationView7Impl: Sized {
    fn PersistedStateId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetPersistedStateId(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IApplicationView9Impl: Sized {
    fn WindowingEnvironment(&self) -> ::windows::core::Result<super::WindowManagement::WindowingEnvironment>;
    fn GetDisplayRegions(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<super::WindowManagement::DisplayRegion>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IApplicationViewConsolidatedEventArgsImpl: Sized {
    fn IsUserInitiated(&self) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IApplicationViewConsolidatedEventArgs2Impl: Sized {
    fn IsAppInitiated(&self) -> ::windows::core::Result<bool>;
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
pub trait IApplicationViewFullscreenStaticsImpl: Sized {
    fn TryUnsnapToFullscreen(&self) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IApplicationViewInteropStaticsImpl: Sized {
    fn GetApplicationViewIdForWindow(&self, window: &::core::option::Option<super::Core::ICoreWindow>) -> ::windows::core::Result<i32>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IApplicationViewScalingImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
pub trait IApplicationViewScalingStaticsImpl: Sized {
    fn DisableLayoutScaling(&self) -> ::windows::core::Result<bool>;
    fn TrySetDisableLayoutScaling(&self, disablelayoutscaling: bool) -> ::windows::core::Result<bool>;
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
pub trait IApplicationViewStaticsImpl: Sized {
    fn Value(&self) -> ::windows::core::Result<ApplicationViewState>;
    fn TryUnsnap(&self) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IApplicationViewStatics2Impl: Sized {
    fn GetForCurrentView(&self) -> ::windows::core::Result<ApplicationView>;
    fn TerminateAppOnFinalViewClose(&self) -> ::windows::core::Result<bool>;
    fn SetTerminateAppOnFinalViewClose(&self, value: bool) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IApplicationViewStatics3Impl: Sized {
    fn PreferredLaunchWindowingMode(&self) -> ::windows::core::Result<ApplicationViewWindowingMode>;
    fn SetPreferredLaunchWindowingMode(&self, value: ApplicationViewWindowingMode) -> ::windows::core::Result<()>;
    fn PreferredLaunchViewSize(&self) -> ::windows::core::Result<super::super::Foundation::Size>;
    fn SetPreferredLaunchViewSize(&self, value: &super::super::Foundation::Size) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IApplicationViewStatics4Impl: Sized {
    fn ClearAllPersistedState(&self) -> ::windows::core::Result<()>;
    fn ClearPersistedState(&self, key: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IApplicationViewSwitcherStaticsImpl: Sized {
    fn DisableShowingMainViewOnActivation(&self) -> ::windows::core::Result<()>;
    fn TryShowAsStandaloneAsync(&self, viewid: i32) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>>;
    fn TryShowAsStandaloneWithSizePreferenceAsync(&self, viewid: i32, sizepreference: ViewSizePreference) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>>;
    fn TryShowAsStandaloneWithAnchorViewAndSizePreferenceAsync(&self, viewid: i32, sizepreference: ViewSizePreference, anchorviewid: i32, anchorsizepreference: ViewSizePreference) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>>;
    fn SwitchAsync(&self, viewid: i32) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
    fn SwitchFromViewAsync(&self, toviewid: i32, fromviewid: i32) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
    fn SwitchFromViewWithOptionsAsync(&self, toviewid: i32, fromviewid: i32, options: ApplicationViewSwitchingOptions) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
    fn PrepareForCustomAnimatedSwitchAsync(&self, toviewid: i32, fromviewid: i32, options: ApplicationViewSwitchingOptions) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IApplicationViewSwitcherStatics2Impl: Sized {
    fn DisableSystemViewActivationPolicy(&self) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IApplicationViewSwitcherStatics3Impl: Sized {
    fn TryShowAsViewModeAsync(&self, viewid: i32, viewmode: ApplicationViewMode) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>>;
    fn TryShowAsViewModeWithPreferencesAsync(&self, viewid: i32, viewmode: ApplicationViewMode, viewmodepreferences: &::core::option::Option<ViewModePreferences>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IApplicationViewTitleBarImpl: Sized {
    fn SetForegroundColor(&self, value: &::core::option::Option<super::super::Foundation::IReference<super::Color>>) -> ::windows::core::Result<()>;
    fn ForegroundColor(&self) -> ::windows::core::Result<super::super::Foundation::IReference<super::Color>>;
    fn SetBackgroundColor(&self, value: &::core::option::Option<super::super::Foundation::IReference<super::Color>>) -> ::windows::core::Result<()>;
    fn BackgroundColor(&self) -> ::windows::core::Result<super::super::Foundation::IReference<super::Color>>;
    fn SetButtonForegroundColor(&self, value: &::core::option::Option<super::super::Foundation::IReference<super::Color>>) -> ::windows::core::Result<()>;
    fn ButtonForegroundColor(&self) -> ::windows::core::Result<super::super::Foundation::IReference<super::Color>>;
    fn SetButtonBackgroundColor(&self, value: &::core::option::Option<super::super::Foundation::IReference<super::Color>>) -> ::windows::core::Result<()>;
    fn ButtonBackgroundColor(&self) -> ::windows::core::Result<super::super::Foundation::IReference<super::Color>>;
    fn SetButtonHoverForegroundColor(&self, value: &::core::option::Option<super::super::Foundation::IReference<super::Color>>) -> ::windows::core::Result<()>;
    fn ButtonHoverForegroundColor(&self) -> ::windows::core::Result<super::super::Foundation::IReference<super::Color>>;
    fn SetButtonHoverBackgroundColor(&self, value: &::core::option::Option<super::super::Foundation::IReference<super::Color>>) -> ::windows::core::Result<()>;
    fn ButtonHoverBackgroundColor(&self) -> ::windows::core::Result<super::super::Foundation::IReference<super::Color>>;
    fn SetButtonPressedForegroundColor(&self, value: &::core::option::Option<super::super::Foundation::IReference<super::Color>>) -> ::windows::core::Result<()>;
    fn ButtonPressedForegroundColor(&self) -> ::windows::core::Result<super::super::Foundation::IReference<super::Color>>;
    fn SetButtonPressedBackgroundColor(&self, value: &::core::option::Option<super::super::Foundation::IReference<super::Color>>) -> ::windows::core::Result<()>;
    fn ButtonPressedBackgroundColor(&self) -> ::windows::core::Result<super::super::Foundation::IReference<super::Color>>;
    fn SetInactiveForegroundColor(&self, value: &::core::option::Option<super::super::Foundation::IReference<super::Color>>) -> ::windows::core::Result<()>;
    fn InactiveForegroundColor(&self) -> ::windows::core::Result<super::super::Foundation::IReference<super::Color>>;
    fn SetInactiveBackgroundColor(&self, value: &::core::option::Option<super::super::Foundation::IReference<super::Color>>) -> ::windows::core::Result<()>;
    fn InactiveBackgroundColor(&self) -> ::windows::core::Result<super::super::Foundation::IReference<super::Color>>;
    fn SetButtonInactiveForegroundColor(&self, value: &::core::option::Option<super::super::Foundation::IReference<super::Color>>) -> ::windows::core::Result<()>;
    fn ButtonInactiveForegroundColor(&self) -> ::windows::core::Result<super::super::Foundation::IReference<super::Color>>;
    fn SetButtonInactiveBackgroundColor(&self, value: &::core::option::Option<super::super::Foundation::IReference<super::Color>>) -> ::windows::core::Result<()>;
    fn ButtonInactiveBackgroundColor(&self) -> ::windows::core::Result<super::super::Foundation::IReference<super::Color>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IApplicationViewTransferContextImpl: Sized {
    fn ViewId(&self) -> ::windows::core::Result<i32>;
    fn SetViewId(&self, value: i32) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IApplicationViewTransferContextStaticsImpl: Sized {
    fn DataPackageFormatId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IApplicationViewWithContextImpl: Sized {
    fn UIContext(&self) -> ::windows::core::Result<super::UIContext>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IInputPaneImpl: Sized {
    fn Showing(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<InputPane, InputPaneVisibilityEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveShowing(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn Hiding(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<InputPane, InputPaneVisibilityEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveHiding(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn OccludedRect(&self) -> ::windows::core::Result<super::super::Foundation::Rect>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IInputPane2Impl: Sized {
    fn TryShow(&self) -> ::windows::core::Result<bool>;
    fn TryHide(&self) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IInputPaneControlImpl: Sized {
    fn Visible(&self) -> ::windows::core::Result<bool>;
    fn SetVisible(&self, value: bool) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IInputPaneStaticsImpl: Sized {
    fn GetForCurrentView(&self) -> ::windows::core::Result<InputPane>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IInputPaneStatics2Impl: Sized {
    fn GetForUIContext(&self, context: &::core::option::Option<super::UIContext>) -> ::windows::core::Result<InputPane>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IInputPaneVisibilityEventArgsImpl: Sized {
    fn OccludedRect(&self) -> ::windows::core::Result<super::super::Foundation::Rect>;
    fn SetEnsuredFocusedElementInView(&self, value: bool) -> ::windows::core::Result<()>;
    fn EnsuredFocusedElementInView(&self) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IProjectionManagerStaticsImpl: Sized {
    fn StartProjectingAsync(&self, projectionviewid: i32, anchorviewid: i32) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
    fn SwapDisplaysForViewsAsync(&self, projectionviewid: i32, anchorviewid: i32) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
    fn StopProjectingAsync(&self, projectionviewid: i32, anchorviewid: i32) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
    fn ProjectionDisplayAvailable(&self) -> ::windows::core::Result<bool>;
    fn ProjectionDisplayAvailableChanged(&self, handler: &::core::option::Option<super::super::Foundation::EventHandler<::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveProjectionDisplayAvailableChanged(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IProjectionManagerStatics2Impl: Sized {
    fn StartProjectingWithDeviceInfoAsync(&self, projectionviewid: i32, anchorviewid: i32, displaydeviceinfo: &::core::option::Option<super::super::Devices::Enumeration::DeviceInformation>) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
    fn RequestStartProjectingAsync(&self, projectionviewid: i32, anchorviewid: i32, selection: &super::super::Foundation::Rect) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>>;
    fn RequestStartProjectingWithPlacementAsync(&self, projectionviewid: i32, anchorviewid: i32, selection: &super::super::Foundation::Rect, prefferedplacement: super::Popups::Placement) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>>;
    fn GetDeviceSelector(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IStatusBarImpl: Sized {
    fn ShowAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
    fn HideAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
    fn BackgroundOpacity(&self) -> ::windows::core::Result<f64>;
    fn SetBackgroundOpacity(&self, value: f64) -> ::windows::core::Result<()>;
    fn ForegroundColor(&self) -> ::windows::core::Result<super::super::Foundation::IReference<super::Color>>;
    fn SetForegroundColor(&self, value: &::core::option::Option<super::super::Foundation::IReference<super::Color>>) -> ::windows::core::Result<()>;
    fn BackgroundColor(&self) -> ::windows::core::Result<super::super::Foundation::IReference<super::Color>>;
    fn SetBackgroundColor(&self, value: &::core::option::Option<super::super::Foundation::IReference<super::Color>>) -> ::windows::core::Result<()>;
    fn ProgressIndicator(&self) -> ::windows::core::Result<StatusBarProgressIndicator>;
    fn OccludedRect(&self) -> ::windows::core::Result<super::super::Foundation::Rect>;
    fn Showing(&self, eventhandler: &::core::option::Option<super::super::Foundation::TypedEventHandler<StatusBar, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveShowing(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn Hiding(&self, eventhandler: &::core::option::Option<super::super::Foundation::TypedEventHandler<StatusBar, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveHiding(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IStatusBarProgressIndicatorImpl: Sized {
    fn ShowAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
    fn HideAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
    fn Text(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetText(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn ProgressValue(&self) -> ::windows::core::Result<super::super::Foundation::IReference<f64>>;
    fn SetProgressValue(&self, value: &::core::option::Option<super::super::Foundation::IReference<f64>>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IStatusBarStaticsImpl: Sized {
    fn GetForCurrentView(&self) -> ::windows::core::Result<StatusBar>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IUISettingsImpl: Sized {
    fn HandPreference(&self) -> ::windows::core::Result<HandPreference>;
    fn CursorSize(&self) -> ::windows::core::Result<super::super::Foundation::Size>;
    fn ScrollBarSize(&self) -> ::windows::core::Result<super::super::Foundation::Size>;
    fn ScrollBarArrowSize(&self) -> ::windows::core::Result<super::super::Foundation::Size>;
    fn ScrollBarThumbBoxSize(&self) -> ::windows::core::Result<super::super::Foundation::Size>;
    fn MessageDuration(&self) -> ::windows::core::Result<u32>;
    fn AnimationsEnabled(&self) -> ::windows::core::Result<bool>;
    fn CaretBrowsingEnabled(&self) -> ::windows::core::Result<bool>;
    fn CaretBlinkRate(&self) -> ::windows::core::Result<u32>;
    fn CaretWidth(&self) -> ::windows::core::Result<u32>;
    fn DoubleClickTime(&self) -> ::windows::core::Result<u32>;
    fn MouseHoverTime(&self) -> ::windows::core::Result<u32>;
    fn UIElementColor(&self, desiredelement: UIElementType) -> ::windows::core::Result<super::Color>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IUISettings2Impl: Sized {
    fn TextScaleFactor(&self) -> ::windows::core::Result<f64>;
    fn TextScaleFactorChanged(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<UISettings, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveTextScaleFactorChanged(&self, cookie: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IUISettings3Impl: Sized {
    fn GetColorValue(&self, desiredcolor: UIColorType) -> ::windows::core::Result<super::Color>;
    fn ColorValuesChanged(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<UISettings, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveColorValuesChanged(&self, cookie: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IUISettings4Impl: Sized {
    fn AdvancedEffectsEnabled(&self) -> ::windows::core::Result<bool>;
    fn AdvancedEffectsEnabledChanged(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<UISettings, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveAdvancedEffectsEnabledChanged(&self, cookie: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IUISettings5Impl: Sized {
    fn AutoHideScrollBars(&self) -> ::windows::core::Result<bool>;
    fn AutoHideScrollBarsChanged(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<UISettings, UISettingsAutoHideScrollBarsChangedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveAutoHideScrollBarsChanged(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IUISettings6Impl: Sized {
    fn AnimationsEnabledChanged(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<UISettings, UISettingsAnimationsEnabledChangedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveAnimationsEnabledChanged(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn MessageDurationChanged(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<UISettings, UISettingsMessageDurationChangedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveMessageDurationChanged(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IUISettingsAnimationsEnabledChangedEventArgsImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
pub trait IUISettingsAutoHideScrollBarsChangedEventArgsImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
pub trait IUISettingsMessageDurationChangedEventArgsImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
pub trait IUIViewSettingsImpl: Sized {
    fn UserInteractionMode(&self) -> ::windows::core::Result<UserInteractionMode>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IUIViewSettingsStaticsImpl: Sized {
    fn GetForCurrentView(&self) -> ::windows::core::Result<UIViewSettings>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IViewModePreferencesImpl: Sized {
    fn ViewSizePreference(&self) -> ::windows::core::Result<ViewSizePreference>;
    fn SetViewSizePreference(&self, value: ViewSizePreference) -> ::windows::core::Result<()>;
    fn CustomSize(&self) -> ::windows::core::Result<super::super::Foundation::Size>;
    fn SetCustomSize(&self, value: &super::super::Foundation::Size) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IViewModePreferencesStaticsImpl: Sized {
    fn CreateDefault(&self, mode: ApplicationViewMode) -> ::windows::core::Result<ViewModePreferences>;
}
