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
    pub const Tunneling: Self = Self(0i32);
    pub const Bubbling: Self = Self(1i32);
}
impl ::core::marker::Copy for WebViewControlAcceleratorKeyRoutingStage {}
impl ::core::clone::Clone for WebViewControlAcceleratorKeyRoutingStage {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct WebViewControlMoveFocusReason(pub i32);
impl WebViewControlMoveFocusReason {
    pub const Programmatic: Self = Self(0i32);
    pub const Next: Self = Self(1i32);
    pub const Previous: Self = Self(2i32);
}
impl ::core::marker::Copy for WebViewControlMoveFocusReason {}
impl ::core::clone::Clone for WebViewControlMoveFocusReason {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct WebViewControlMoveFocusRequestedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct WebViewControlProcess(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct WebViewControlProcessCapabilityState(pub i32);
impl WebViewControlProcessCapabilityState {
    pub const Default: Self = Self(0i32);
    pub const Disabled: Self = Self(1i32);
    pub const Enabled: Self = Self(2i32);
}
impl ::core::marker::Copy for WebViewControlProcessCapabilityState {}
impl ::core::clone::Clone for WebViewControlProcessCapabilityState {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct WebViewControlProcessOptions(pub *mut ::core::ffi::c_void);
