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
    pub const UseVisible: Self = Self(0i32);
    pub const UseCoreWindow: Self = Self(1i32);
}
#[repr(transparent)]
pub struct ApplicationViewConsolidatedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ApplicationViewMode(pub i32);
impl ApplicationViewMode {
    pub const Default: Self = Self(0i32);
    pub const CompactOverlay: Self = Self(1i32);
}
#[repr(transparent)]
pub struct ApplicationViewOrientation(pub i32);
impl ApplicationViewOrientation {
    pub const Landscape: Self = Self(0i32);
    pub const Portrait: Self = Self(1i32);
}
#[repr(transparent)]
pub struct ApplicationViewScaling(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ApplicationViewState(pub i32);
impl ApplicationViewState {
    pub const FullScreenLandscape: Self = Self(0i32);
    pub const Filled: Self = Self(1i32);
    pub const Snapped: Self = Self(2i32);
    pub const FullScreenPortrait: Self = Self(3i32);
}
#[repr(transparent)]
pub struct ApplicationViewSwitchingOptions(pub u32);
impl ApplicationViewSwitchingOptions {
    pub const Default: Self = Self(0u32);
    pub const SkipAnimation: Self = Self(1u32);
    pub const ConsolidateViews: Self = Self(2u32);
}
#[repr(transparent)]
pub struct ApplicationViewTitleBar(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ApplicationViewTransferContext(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ApplicationViewWindowingMode(pub i32);
impl ApplicationViewWindowingMode {
    pub const Auto: Self = Self(0i32);
    pub const PreferredLaunchViewSize: Self = Self(1i32);
    pub const FullScreen: Self = Self(2i32);
    pub const CompactOverlay: Self = Self(3i32);
    pub const Maximized: Self = Self(4i32);
}
#[repr(transparent)]
pub struct FullScreenSystemOverlayMode(pub i32);
impl FullScreenSystemOverlayMode {
    pub const Standard: Self = Self(0i32);
    pub const Minimal: Self = Self(1i32);
}
#[repr(transparent)]
pub struct HandPreference(pub i32);
impl HandPreference {
    pub const LeftHanded: Self = Self(0i32);
    pub const RightHanded: Self = Self(1i32);
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
    pub const Background: Self = Self(0i32);
    pub const Foreground: Self = Self(1i32);
    pub const AccentDark3: Self = Self(2i32);
    pub const AccentDark2: Self = Self(3i32);
    pub const AccentDark1: Self = Self(4i32);
    pub const Accent: Self = Self(5i32);
    pub const AccentLight1: Self = Self(6i32);
    pub const AccentLight2: Self = Self(7i32);
    pub const AccentLight3: Self = Self(8i32);
    pub const Complement: Self = Self(9i32);
}
#[repr(transparent)]
pub struct UIElementType(pub i32);
impl UIElementType {
    pub const ActiveCaption: Self = Self(0i32);
    pub const Background: Self = Self(1i32);
    pub const ButtonFace: Self = Self(2i32);
    pub const ButtonText: Self = Self(3i32);
    pub const CaptionText: Self = Self(4i32);
    pub const GrayText: Self = Self(5i32);
    pub const Highlight: Self = Self(6i32);
    pub const HighlightText: Self = Self(7i32);
    pub const Hotlight: Self = Self(8i32);
    pub const InactiveCaption: Self = Self(9i32);
    pub const InactiveCaptionText: Self = Self(10i32);
    pub const Window: Self = Self(11i32);
    pub const WindowText: Self = Self(12i32);
    pub const AccentColor: Self = Self(1000i32);
    pub const TextHigh: Self = Self(1001i32);
    pub const TextMedium: Self = Self(1002i32);
    pub const TextLow: Self = Self(1003i32);
    pub const TextContrastWithHigh: Self = Self(1004i32);
    pub const NonTextHigh: Self = Self(1005i32);
    pub const NonTextMediumHigh: Self = Self(1006i32);
    pub const NonTextMedium: Self = Self(1007i32);
    pub const NonTextMediumLow: Self = Self(1008i32);
    pub const NonTextLow: Self = Self(1009i32);
    pub const PageBackground: Self = Self(1010i32);
    pub const PopupBackground: Self = Self(1011i32);
    pub const OverlayOutsidePopup: Self = Self(1012i32);
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
    pub const Mouse: Self = Self(0i32);
    pub const Touch: Self = Self(1i32);
}
#[repr(C)]
pub struct ViewManagementViewScalingContract(i32);
#[repr(transparent)]
pub struct ViewModePreferences(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ViewSizePreference(pub i32);
impl ViewSizePreference {
    pub const Default: Self = Self(0i32);
    pub const UseLess: Self = Self(1i32);
    pub const UseHalf: Self = Self(2i32);
    pub const UseMore: Self = Self(3i32);
    pub const UseMinimum: Self = Self(4i32);
    pub const UseNone: Self = Self(5i32);
    pub const Custom: Self = Self(6i32);
}
