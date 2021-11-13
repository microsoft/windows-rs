#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[cfg(feature = "UI_WindowManagement_Preview")]
pub mod Preview;
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct AppWindow(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for AppWindow {}
impl ::core::clone::Clone for AppWindow {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct AppWindowChangedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for AppWindowChangedEventArgs {}
impl ::core::clone::Clone for AppWindowChangedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct AppWindowCloseRequestedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for AppWindowCloseRequestedEventArgs {}
impl ::core::clone::Clone for AppWindowCloseRequestedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct AppWindowClosedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for AppWindowClosedEventArgs {}
impl ::core::clone::Clone for AppWindowClosedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
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
#[repr(transparent)]
pub struct AppWindowFrame(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for AppWindowFrame {}
impl ::core::clone::Clone for AppWindowFrame {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
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
#[repr(transparent)]
pub struct AppWindowPlacement(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for AppWindowPlacement {}
impl ::core::clone::Clone for AppWindowPlacement {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct AppWindowPresentationConfiguration(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for AppWindowPresentationConfiguration {}
impl ::core::clone::Clone for AppWindowPresentationConfiguration {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
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
#[repr(transparent)]
pub struct AppWindowPresenter(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for AppWindowPresenter {}
impl ::core::clone::Clone for AppWindowPresenter {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct AppWindowTitleBar(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for AppWindowTitleBar {}
impl ::core::clone::Clone for AppWindowTitleBar {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct AppWindowTitleBarOcclusion(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for AppWindowTitleBarOcclusion {}
impl ::core::clone::Clone for AppWindowTitleBarOcclusion {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
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
#[repr(transparent)]
pub struct CompactOverlayPresentationConfiguration(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for CompactOverlayPresentationConfiguration {}
impl ::core::clone::Clone for CompactOverlayPresentationConfiguration {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct DefaultPresentationConfiguration(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for DefaultPresentationConfiguration {}
impl ::core::clone::Clone for DefaultPresentationConfiguration {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct DisplayRegion(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for DisplayRegion {}
impl ::core::clone::Clone for DisplayRegion {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct FullScreenPresentationConfiguration(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for FullScreenPresentationConfiguration {}
impl ::core::clone::Clone for FullScreenPresentationConfiguration {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IAppWindow(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IAppWindow {}
impl ::core::clone::Clone for IAppWindow {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IAppWindowChangedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IAppWindowChangedEventArgs {}
impl ::core::clone::Clone for IAppWindowChangedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IAppWindowCloseRequestedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IAppWindowCloseRequestedEventArgs {}
impl ::core::clone::Clone for IAppWindowCloseRequestedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IAppWindowClosedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IAppWindowClosedEventArgs {}
impl ::core::clone::Clone for IAppWindowClosedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IAppWindowFrame(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IAppWindowFrame {}
impl ::core::clone::Clone for IAppWindowFrame {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IAppWindowFrameStyle(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IAppWindowFrameStyle {}
impl ::core::clone::Clone for IAppWindowFrameStyle {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IAppWindowPlacement(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IAppWindowPlacement {}
impl ::core::clone::Clone for IAppWindowPlacement {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IAppWindowPresentationConfiguration(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IAppWindowPresentationConfiguration {}
impl ::core::clone::Clone for IAppWindowPresentationConfiguration {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IAppWindowPresentationConfigurationFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IAppWindowPresentationConfigurationFactory {}
impl ::core::clone::Clone for IAppWindowPresentationConfigurationFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IAppWindowPresenter(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IAppWindowPresenter {}
impl ::core::clone::Clone for IAppWindowPresenter {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IAppWindowStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IAppWindowStatics {}
impl ::core::clone::Clone for IAppWindowStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IAppWindowTitleBar(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IAppWindowTitleBar {}
impl ::core::clone::Clone for IAppWindowTitleBar {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IAppWindowTitleBarOcclusion(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IAppWindowTitleBarOcclusion {}
impl ::core::clone::Clone for IAppWindowTitleBarOcclusion {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IAppWindowTitleBarVisibility(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IAppWindowTitleBarVisibility {}
impl ::core::clone::Clone for IAppWindowTitleBarVisibility {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ICompactOverlayPresentationConfiguration(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ICompactOverlayPresentationConfiguration {}
impl ::core::clone::Clone for ICompactOverlayPresentationConfiguration {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IDefaultPresentationConfiguration(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IDefaultPresentationConfiguration {}
impl ::core::clone::Clone for IDefaultPresentationConfiguration {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IDisplayRegion(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IDisplayRegion {}
impl ::core::clone::Clone for IDisplayRegion {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IFullScreenPresentationConfiguration(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IFullScreenPresentationConfiguration {}
impl ::core::clone::Clone for IFullScreenPresentationConfiguration {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IWindowServicesStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IWindowServicesStatics {}
impl ::core::clone::Clone for IWindowServicesStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IWindowingEnvironment(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IWindowingEnvironment {}
impl ::core::clone::Clone for IWindowingEnvironment {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IWindowingEnvironmentAddedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IWindowingEnvironmentAddedEventArgs {}
impl ::core::clone::Clone for IWindowingEnvironmentAddedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IWindowingEnvironmentChangedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IWindowingEnvironmentChangedEventArgs {}
impl ::core::clone::Clone for IWindowingEnvironmentChangedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IWindowingEnvironmentRemovedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IWindowingEnvironmentRemovedEventArgs {}
impl ::core::clone::Clone for IWindowingEnvironmentRemovedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IWindowingEnvironmentStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IWindowingEnvironmentStatics {}
impl ::core::clone::Clone for IWindowingEnvironmentStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct WindowingEnvironment(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for WindowingEnvironment {}
impl ::core::clone::Clone for WindowingEnvironment {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct WindowingEnvironmentAddedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for WindowingEnvironmentAddedEventArgs {}
impl ::core::clone::Clone for WindowingEnvironmentAddedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct WindowingEnvironmentChangedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for WindowingEnvironmentChangedEventArgs {}
impl ::core::clone::Clone for WindowingEnvironmentChangedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
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
#[repr(transparent)]
pub struct WindowingEnvironmentRemovedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for WindowingEnvironmentRemovedEventArgs {}
impl ::core::clone::Clone for WindowingEnvironmentRemovedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
