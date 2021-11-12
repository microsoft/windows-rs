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
#[repr(transparent)]
pub struct AppWindowPresentationConfiguration(pub *mut ::core::ffi::c_void);
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
#[repr(transparent)]
pub struct AppWindowTitleBar(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct AppWindowTitleBarOcclusion(pub *mut ::core::ffi::c_void);
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
