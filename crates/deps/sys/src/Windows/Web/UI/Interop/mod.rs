#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct IWebViewControlAcceleratorKeyPressedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IWebViewControlMoveFocusRequestedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IWebViewControlProcess(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IWebViewControlProcessFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IWebViewControlProcessOptions(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IWebViewControlSite(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IWebViewControlSite2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct WebViewControl(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct WebViewControlAcceleratorKeyPressedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(C)]
pub struct WebViewControlAcceleratorKeyRoutingStage(i32);
#[repr(C)]
pub struct WebViewControlMoveFocusReason(i32);
#[repr(transparent)]
pub struct WebViewControlMoveFocusRequestedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct WebViewControlProcess(pub *mut ::core::ffi::c_void);
#[repr(C)]
pub struct WebViewControlProcessCapabilityState(i32);
#[repr(transparent)]
pub struct WebViewControlProcessOptions(pub *mut ::core::ffi::c_void);
