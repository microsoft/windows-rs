#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
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
#[repr(transparent)]
pub struct WebViewControlAcceleratorKeyRoutingStage(pub i32);
impl WebViewControlAcceleratorKeyRoutingStage {
    pub const Tunneling: WebViewControlAcceleratorKeyRoutingStage = WebViewControlAcceleratorKeyRoutingStage(0i32);
    pub const Bubbling: WebViewControlAcceleratorKeyRoutingStage = WebViewControlAcceleratorKeyRoutingStage(1i32);
}
#[repr(transparent)]
pub struct WebViewControlMoveFocusReason(pub i32);
impl WebViewControlMoveFocusReason {
    pub const Programmatic: WebViewControlMoveFocusReason = WebViewControlMoveFocusReason(0i32);
    pub const Next: WebViewControlMoveFocusReason = WebViewControlMoveFocusReason(1i32);
    pub const Previous: WebViewControlMoveFocusReason = WebViewControlMoveFocusReason(2i32);
}
#[repr(transparent)]
pub struct WebViewControlMoveFocusRequestedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct WebViewControlProcess(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct WebViewControlProcessCapabilityState(pub i32);
impl WebViewControlProcessCapabilityState {
    pub const Default: WebViewControlProcessCapabilityState = WebViewControlProcessCapabilityState(0i32);
    pub const Disabled: WebViewControlProcessCapabilityState = WebViewControlProcessCapabilityState(1i32);
    pub const Enabled: WebViewControlProcessCapabilityState = WebViewControlProcessCapabilityState(2i32);
}
#[repr(transparent)]
pub struct WebViewControlProcessOptions(pub *mut ::core::ffi::c_void);
