#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[cfg(feature = "Web_UI_Interop")]
pub mod Interop;
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct IWebViewControl(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IWebViewControl2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IWebViewControlContentLoadingEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IWebViewControlDOMContentLoadedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IWebViewControlDeferredPermissionRequest(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IWebViewControlLongRunningScriptDetectedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IWebViewControlNavigationCompletedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IWebViewControlNavigationStartingEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IWebViewControlNewWindowRequestedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IWebViewControlNewWindowRequestedEventArgs2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IWebViewControlPermissionRequest(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IWebViewControlPermissionRequestedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IWebViewControlScriptNotifyEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IWebViewControlSettings(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IWebViewControlUnsupportedUriSchemeIdentifiedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IWebViewControlUnviewableContentIdentifiedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IWebViewControlWebResourceRequestedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct WebViewControlContentLoadingEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct WebViewControlDOMContentLoadedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct WebViewControlDeferredPermissionRequest(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct WebViewControlLongRunningScriptDetectedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct WebViewControlNavigationCompletedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct WebViewControlNavigationStartingEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct WebViewControlNewWindowRequestedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct WebViewControlPermissionRequest(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct WebViewControlPermissionRequestedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct WebViewControlPermissionState(pub i32);
impl WebViewControlPermissionState {
    pub const Unknown: Self = Self(0i32);
    pub const Defer: Self = Self(1i32);
    pub const Allow: Self = Self(2i32);
    pub const Deny: Self = Self(3i32);
}
impl ::core::marker::Copy for WebViewControlPermissionState {}
impl ::core::clone::Clone for WebViewControlPermissionState {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct WebViewControlPermissionType(pub i32);
impl WebViewControlPermissionType {
    pub const Geolocation: Self = Self(0i32);
    pub const UnlimitedIndexedDBQuota: Self = Self(1i32);
    pub const Media: Self = Self(2i32);
    pub const PointerLock: Self = Self(3i32);
    pub const WebNotifications: Self = Self(4i32);
    pub const Screen: Self = Self(5i32);
    pub const ImmersiveView: Self = Self(6i32);
}
impl ::core::marker::Copy for WebViewControlPermissionType {}
impl ::core::clone::Clone for WebViewControlPermissionType {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct WebViewControlScriptNotifyEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct WebViewControlSettings(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct WebViewControlUnsupportedUriSchemeIdentifiedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct WebViewControlUnviewableContentIdentifiedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct WebViewControlWebResourceRequestedEventArgs(pub *mut ::core::ffi::c_void);
