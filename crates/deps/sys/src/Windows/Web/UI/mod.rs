#![allow(non_snake_case, non_camel_case_types)]
#[cfg(feature = "Web_UI_Interop")]
pub mod Interop;
#[link(name = "windows")]
extern "system" {}
pub struct IWebViewControl(pub *mut ::core::ffi::c_void);
pub struct IWebViewControl2(pub *mut ::core::ffi::c_void);
pub struct IWebViewControlContentLoadingEventArgs(pub *mut ::core::ffi::c_void);
pub struct IWebViewControlDOMContentLoadedEventArgs(pub *mut ::core::ffi::c_void);
pub struct IWebViewControlDeferredPermissionRequest(pub *mut ::core::ffi::c_void);
pub struct IWebViewControlLongRunningScriptDetectedEventArgs(pub *mut ::core::ffi::c_void);
pub struct IWebViewControlNavigationCompletedEventArgs(pub *mut ::core::ffi::c_void);
pub struct IWebViewControlNavigationStartingEventArgs(pub *mut ::core::ffi::c_void);
pub struct IWebViewControlNewWindowRequestedEventArgs(pub *mut ::core::ffi::c_void);
pub struct IWebViewControlNewWindowRequestedEventArgs2(pub *mut ::core::ffi::c_void);
pub struct IWebViewControlPermissionRequest(pub *mut ::core::ffi::c_void);
pub struct IWebViewControlPermissionRequestedEventArgs(pub *mut ::core::ffi::c_void);
pub struct IWebViewControlScriptNotifyEventArgs(pub *mut ::core::ffi::c_void);
pub struct IWebViewControlSettings(pub *mut ::core::ffi::c_void);
pub struct IWebViewControlUnsupportedUriSchemeIdentifiedEventArgs(pub *mut ::core::ffi::c_void);
pub struct IWebViewControlUnviewableContentIdentifiedEventArgs(pub *mut ::core::ffi::c_void);
pub struct IWebViewControlWebResourceRequestedEventArgs(pub *mut ::core::ffi::c_void);
pub struct WebViewControlContentLoadingEventArgs(i32);
pub struct WebViewControlDOMContentLoadedEventArgs(i32);
pub struct WebViewControlDeferredPermissionRequest(i32);
pub struct WebViewControlLongRunningScriptDetectedEventArgs(i32);
pub struct WebViewControlNavigationCompletedEventArgs(i32);
pub struct WebViewControlNavigationStartingEventArgs(i32);
pub struct WebViewControlNewWindowRequestedEventArgs(i32);
pub struct WebViewControlPermissionRequest(i32);
pub struct WebViewControlPermissionRequestedEventArgs(i32);
pub struct WebViewControlPermissionState(i32);
pub struct WebViewControlPermissionType(i32);
pub struct WebViewControlScriptNotifyEventArgs(i32);
pub struct WebViewControlSettings(i32);
pub struct WebViewControlUnsupportedUriSchemeIdentifiedEventArgs(i32);
pub struct WebViewControlUnviewableContentIdentifiedEventArgs(i32);
pub struct WebViewControlWebResourceRequestedEventArgs(i32);
