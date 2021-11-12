#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[cfg(feature = "UI_ViewManagement_Core")]
pub mod Core;
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct AccessibilitySettings(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ActivationViewSwitcher(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ApplicationView(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ApplicationViewBoundsMode(pub i32);
impl ApplicationViewBoundsMode {
    pub const UseVisible: ApplicationViewBoundsMode = ApplicationViewBoundsMode(0i32);
    pub const UseCoreWindow: ApplicationViewBoundsMode = ApplicationViewBoundsMode(1i32);
}
#[repr(transparent)]
pub struct ApplicationViewConsolidatedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ApplicationViewMode(pub i32);
impl ApplicationViewMode {
    pub const Default: ApplicationViewMode = ApplicationViewMode(0i32);
    pub const CompactOverlay: ApplicationViewMode = ApplicationViewMode(1i32);
}
#[repr(transparent)]
pub struct ApplicationViewOrientation(pub i32);
impl ApplicationViewOrientation {
    pub const Landscape: ApplicationViewOrientation = ApplicationViewOrientation(0i32);
    pub const Portrait: ApplicationViewOrientation = ApplicationViewOrientation(1i32);
}
#[repr(transparent)]
pub struct ApplicationViewScaling(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ApplicationViewState(pub i32);
impl ApplicationViewState {
    pub const FullScreenLandscape: ApplicationViewState = ApplicationViewState(0i32);
    pub const Filled: ApplicationViewState = ApplicationViewState(1i32);
    pub const Snapped: ApplicationViewState = ApplicationViewState(2i32);
    pub const FullScreenPortrait: ApplicationViewState = ApplicationViewState(3i32);
}
#[repr(transparent)]
pub struct ApplicationViewSwitchingOptions(pub u32);
impl ApplicationViewSwitchingOptions {
    pub const Default: ApplicationViewSwitchingOptions = ApplicationViewSwitchingOptions(0u32);
    pub const SkipAnimation: ApplicationViewSwitchingOptions = ApplicationViewSwitchingOptions(1u32);
    pub const ConsolidateViews: ApplicationViewSwitchingOptions = ApplicationViewSwitchingOptions(2u32);
}
#[repr(transparent)]
pub struct ApplicationViewTitleBar(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ApplicationViewTransferContext(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ApplicationViewWindowingMode(pub i32);
impl ApplicationViewWindowingMode {
    pub const Auto: ApplicationViewWindowingMode = ApplicationViewWindowingMode(0i32);
    pub const PreferredLaunchViewSize: ApplicationViewWindowingMode = ApplicationViewWindowingMode(1i32);
    pub const FullScreen: ApplicationViewWindowingMode = ApplicationViewWindowingMode(2i32);
    pub const CompactOverlay: ApplicationViewWindowingMode = ApplicationViewWindowingMode(3i32);
    pub const Maximized: ApplicationViewWindowingMode = ApplicationViewWindowingMode(4i32);
}
#[repr(transparent)]
pub struct FullScreenSystemOverlayMode(pub i32);
impl FullScreenSystemOverlayMode {
    pub const Standard: FullScreenSystemOverlayMode = FullScreenSystemOverlayMode(0i32);
    pub const Minimal: FullScreenSystemOverlayMode = FullScreenSystemOverlayMode(1i32);
}
#[repr(transparent)]
pub struct HandPreference(pub i32);
impl HandPreference {
    pub const LeftHanded: HandPreference = HandPreference(0i32);
    pub const RightHanded: HandPreference = HandPreference(1i32);
}
#[repr(transparent)]
pub struct IAccessibilitySettings(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IActivationViewSwitcher(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IApplicationView(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IApplicationView2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IApplicationView3(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IApplicationView4(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IApplicationView7(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IApplicationView9(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IApplicationViewConsolidatedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IApplicationViewConsolidatedEventArgs2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IApplicationViewFullscreenStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IApplicationViewInteropStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IApplicationViewScaling(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IApplicationViewScalingStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IApplicationViewStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IApplicationViewStatics2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IApplicationViewStatics3(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IApplicationViewStatics4(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IApplicationViewSwitcherStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IApplicationViewSwitcherStatics2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IApplicationViewSwitcherStatics3(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IApplicationViewTitleBar(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IApplicationViewTransferContext(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IApplicationViewTransferContextStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IApplicationViewWithContext(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IInputPane(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IInputPane2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IInputPaneControl(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IInputPaneStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IInputPaneStatics2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IInputPaneVisibilityEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IProjectionManagerStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IProjectionManagerStatics2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IStatusBar(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IStatusBarProgressIndicator(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IStatusBarStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IUISettings(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IUISettings2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IUISettings3(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IUISettings4(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IUISettings5(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IUISettings6(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IUISettingsAnimationsEnabledChangedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IUISettingsAutoHideScrollBarsChangedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IUISettingsMessageDurationChangedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IUIViewSettings(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IUIViewSettingsStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IViewModePreferences(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IViewModePreferencesStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct InputPane(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct InputPaneVisibilityEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct StatusBar(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct StatusBarProgressIndicator(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct UIColorType(pub i32);
impl UIColorType {
    pub const Background: UIColorType = UIColorType(0i32);
    pub const Foreground: UIColorType = UIColorType(1i32);
    pub const AccentDark3: UIColorType = UIColorType(2i32);
    pub const AccentDark2: UIColorType = UIColorType(3i32);
    pub const AccentDark1: UIColorType = UIColorType(4i32);
    pub const Accent: UIColorType = UIColorType(5i32);
    pub const AccentLight1: UIColorType = UIColorType(6i32);
    pub const AccentLight2: UIColorType = UIColorType(7i32);
    pub const AccentLight3: UIColorType = UIColorType(8i32);
    pub const Complement: UIColorType = UIColorType(9i32);
}
#[repr(transparent)]
pub struct UIElementType(pub i32);
impl UIElementType {
    pub const ActiveCaption: UIElementType = UIElementType(0i32);
    pub const Background: UIElementType = UIElementType(1i32);
    pub const ButtonFace: UIElementType = UIElementType(2i32);
    pub const ButtonText: UIElementType = UIElementType(3i32);
    pub const CaptionText: UIElementType = UIElementType(4i32);
    pub const GrayText: UIElementType = UIElementType(5i32);
    pub const Highlight: UIElementType = UIElementType(6i32);
    pub const HighlightText: UIElementType = UIElementType(7i32);
    pub const Hotlight: UIElementType = UIElementType(8i32);
    pub const InactiveCaption: UIElementType = UIElementType(9i32);
    pub const InactiveCaptionText: UIElementType = UIElementType(10i32);
    pub const Window: UIElementType = UIElementType(11i32);
    pub const WindowText: UIElementType = UIElementType(12i32);
    pub const AccentColor: UIElementType = UIElementType(1000i32);
    pub const TextHigh: UIElementType = UIElementType(1001i32);
    pub const TextMedium: UIElementType = UIElementType(1002i32);
    pub const TextLow: UIElementType = UIElementType(1003i32);
    pub const TextContrastWithHigh: UIElementType = UIElementType(1004i32);
    pub const NonTextHigh: UIElementType = UIElementType(1005i32);
    pub const NonTextMediumHigh: UIElementType = UIElementType(1006i32);
    pub const NonTextMedium: UIElementType = UIElementType(1007i32);
    pub const NonTextMediumLow: UIElementType = UIElementType(1008i32);
    pub const NonTextLow: UIElementType = UIElementType(1009i32);
    pub const PageBackground: UIElementType = UIElementType(1010i32);
    pub const PopupBackground: UIElementType = UIElementType(1011i32);
    pub const OverlayOutsidePopup: UIElementType = UIElementType(1012i32);
}
#[repr(transparent)]
pub struct UISettings(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct UISettingsAnimationsEnabledChangedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct UISettingsAutoHideScrollBarsChangedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct UISettingsMessageDurationChangedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct UIViewSettings(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct UserInteractionMode(pub i32);
impl UserInteractionMode {
    pub const Mouse: UserInteractionMode = UserInteractionMode(0i32);
    pub const Touch: UserInteractionMode = UserInteractionMode(1i32);
}
#[repr(C)]
pub struct ViewManagementViewScalingContract(i32);
#[repr(transparent)]
pub struct ViewModePreferences(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ViewSizePreference(pub i32);
impl ViewSizePreference {
    pub const Default: ViewSizePreference = ViewSizePreference(0i32);
    pub const UseLess: ViewSizePreference = ViewSizePreference(1i32);
    pub const UseHalf: ViewSizePreference = ViewSizePreference(2i32);
    pub const UseMore: ViewSizePreference = ViewSizePreference(3i32);
    pub const UseMinimum: ViewSizePreference = ViewSizePreference(4i32);
    pub const UseNone: ViewSizePreference = ViewSizePreference(5i32);
    pub const Custom: ViewSizePreference = ViewSizePreference(6i32);
}
