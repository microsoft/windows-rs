#![allow(non_snake_case, non_camel_case_types)]
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
#[repr(C)]
pub struct ApplicationViewBoundsMode(i32);
#[repr(transparent)]
pub struct ApplicationViewConsolidatedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(C)]
pub struct ApplicationViewMode(i32);
#[repr(C)]
pub struct ApplicationViewOrientation(i32);
#[repr(transparent)]
pub struct ApplicationViewScaling(pub *mut ::core::ffi::c_void);
#[repr(C)]
pub struct ApplicationViewState(i32);
#[repr(C)]
pub struct ApplicationViewSwitchingOptions(i32);
#[repr(transparent)]
pub struct ApplicationViewTitleBar(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ApplicationViewTransferContext(pub *mut ::core::ffi::c_void);
#[repr(C)]
pub struct ApplicationViewWindowingMode(i32);
#[repr(C)]
pub struct FullScreenSystemOverlayMode(i32);
#[repr(C)]
pub struct HandPreference(i32);
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
#[repr(C)]
pub struct UIColorType(i32);
#[repr(C)]
pub struct UIElementType(i32);
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
#[repr(C)]
pub struct UserInteractionMode(i32);
#[repr(C)]
pub struct ViewManagementViewScalingContract(i32);
#[repr(transparent)]
pub struct ViewModePreferences(pub *mut ::core::ffi::c_void);
#[repr(C)]
pub struct ViewSizePreference(i32);
