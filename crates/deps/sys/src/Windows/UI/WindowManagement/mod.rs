#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[cfg(feature = "UI_WindowManagement_Preview")]
pub mod Preview;
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct AppWindow(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct AppWindowChangedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct AppWindowCloseRequestedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct AppWindowClosedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct AppWindowClosedReason(pub i32);
impl AppWindowClosedReason {
    pub const Other: AppWindowClosedReason = AppWindowClosedReason(0i32);
    pub const AppInitiated: AppWindowClosedReason = AppWindowClosedReason(1i32);
    pub const UserInitiated: AppWindowClosedReason = AppWindowClosedReason(2i32);
}
#[repr(transparent)]
pub struct AppWindowFrame(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct AppWindowFrameStyle(pub i32);
impl AppWindowFrameStyle {
    pub const Default: AppWindowFrameStyle = AppWindowFrameStyle(0i32);
    pub const NoFrame: AppWindowFrameStyle = AppWindowFrameStyle(1i32);
}
#[repr(transparent)]
pub struct AppWindowPlacement(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct AppWindowPresentationConfiguration(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct AppWindowPresentationKind(pub i32);
impl AppWindowPresentationKind {
    pub const Default: AppWindowPresentationKind = AppWindowPresentationKind(0i32);
    pub const CompactOverlay: AppWindowPresentationKind = AppWindowPresentationKind(1i32);
    pub const FullScreen: AppWindowPresentationKind = AppWindowPresentationKind(2i32);
}
#[repr(transparent)]
pub struct AppWindowPresenter(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct AppWindowTitleBar(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct AppWindowTitleBarOcclusion(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct AppWindowTitleBarVisibility(pub i32);
impl AppWindowTitleBarVisibility {
    pub const Default: AppWindowTitleBarVisibility = AppWindowTitleBarVisibility(0i32);
    pub const AlwaysHidden: AppWindowTitleBarVisibility = AppWindowTitleBarVisibility(1i32);
}
#[repr(transparent)]
pub struct CompactOverlayPresentationConfiguration(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct DefaultPresentationConfiguration(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct DisplayRegion(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct FullScreenPresentationConfiguration(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAppWindow(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAppWindowChangedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAppWindowCloseRequestedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAppWindowClosedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAppWindowFrame(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAppWindowFrameStyle(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAppWindowPlacement(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAppWindowPresentationConfiguration(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAppWindowPresentationConfigurationFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAppWindowPresenter(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAppWindowStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAppWindowTitleBar(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAppWindowTitleBarOcclusion(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAppWindowTitleBarVisibility(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICompactOverlayPresentationConfiguration(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDefaultPresentationConfiguration(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDisplayRegion(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IFullScreenPresentationConfiguration(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IWindowServicesStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IWindowingEnvironment(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IWindowingEnvironmentAddedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IWindowingEnvironmentChangedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IWindowingEnvironmentRemovedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IWindowingEnvironmentStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct WindowingEnvironment(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct WindowingEnvironmentAddedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct WindowingEnvironmentChangedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct WindowingEnvironmentKind(pub i32);
impl WindowingEnvironmentKind {
    pub const Unknown: WindowingEnvironmentKind = WindowingEnvironmentKind(0i32);
    pub const Overlapped: WindowingEnvironmentKind = WindowingEnvironmentKind(1i32);
    pub const Tiled: WindowingEnvironmentKind = WindowingEnvironmentKind(2i32);
}
#[repr(transparent)]
pub struct WindowingEnvironmentRemovedEventArgs(pub *mut ::core::ffi::c_void);
