#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {}
pub struct IWebViewControlAcceleratorKeyPressedEventArgs(pub *mut ::core::ffi::c_void);
pub struct IWebViewControlMoveFocusRequestedEventArgs(pub *mut ::core::ffi::c_void);
pub struct IWebViewControlProcess(pub *mut ::core::ffi::c_void);
pub struct IWebViewControlProcessFactory(pub *mut ::core::ffi::c_void);
pub struct IWebViewControlProcessOptions(pub *mut ::core::ffi::c_void);
pub struct IWebViewControlSite(pub *mut ::core::ffi::c_void);
pub struct IWebViewControlSite2(pub *mut ::core::ffi::c_void);
pub struct WebViewControl(i32);
pub struct WebViewControlAcceleratorKeyPressedEventArgs(i32);
pub struct WebViewControlAcceleratorKeyRoutingStage(i32);
pub struct WebViewControlMoveFocusReason(i32);
pub struct WebViewControlMoveFocusRequestedEventArgs(i32);
pub struct WebViewControlProcess(i32);
pub struct WebViewControlProcessCapabilityState(i32);
pub struct WebViewControlProcessOptions(i32);
