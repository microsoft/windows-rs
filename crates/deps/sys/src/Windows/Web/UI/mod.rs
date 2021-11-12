#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[cfg(feature = "Web_UI_Interop")]
pub mod Interop;
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct IWebViewControl(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IWebViewControl {}
impl ::core::clone::Clone for IWebViewControl {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IWebViewControl2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IWebViewControl2 {}
impl ::core::clone::Clone for IWebViewControl2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IWebViewControlContentLoadingEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IWebViewControlContentLoadingEventArgs {}
impl ::core::clone::Clone for IWebViewControlContentLoadingEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IWebViewControlDOMContentLoadedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IWebViewControlDOMContentLoadedEventArgs {}
impl ::core::clone::Clone for IWebViewControlDOMContentLoadedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IWebViewControlDeferredPermissionRequest(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IWebViewControlDeferredPermissionRequest {}
impl ::core::clone::Clone for IWebViewControlDeferredPermissionRequest {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IWebViewControlLongRunningScriptDetectedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IWebViewControlLongRunningScriptDetectedEventArgs {}
impl ::core::clone::Clone for IWebViewControlLongRunningScriptDetectedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IWebViewControlNavigationCompletedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IWebViewControlNavigationCompletedEventArgs {}
impl ::core::clone::Clone for IWebViewControlNavigationCompletedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IWebViewControlNavigationStartingEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IWebViewControlNavigationStartingEventArgs {}
impl ::core::clone::Clone for IWebViewControlNavigationStartingEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IWebViewControlNewWindowRequestedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IWebViewControlNewWindowRequestedEventArgs {}
impl ::core::clone::Clone for IWebViewControlNewWindowRequestedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IWebViewControlNewWindowRequestedEventArgs2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IWebViewControlNewWindowRequestedEventArgs2 {}
impl ::core::clone::Clone for IWebViewControlNewWindowRequestedEventArgs2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IWebViewControlPermissionRequest(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IWebViewControlPermissionRequest {}
impl ::core::clone::Clone for IWebViewControlPermissionRequest {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IWebViewControlPermissionRequestedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IWebViewControlPermissionRequestedEventArgs {}
impl ::core::clone::Clone for IWebViewControlPermissionRequestedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IWebViewControlScriptNotifyEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IWebViewControlScriptNotifyEventArgs {}
impl ::core::clone::Clone for IWebViewControlScriptNotifyEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IWebViewControlSettings(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IWebViewControlSettings {}
impl ::core::clone::Clone for IWebViewControlSettings {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IWebViewControlUnsupportedUriSchemeIdentifiedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IWebViewControlUnsupportedUriSchemeIdentifiedEventArgs {}
impl ::core::clone::Clone for IWebViewControlUnsupportedUriSchemeIdentifiedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IWebViewControlUnviewableContentIdentifiedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IWebViewControlUnviewableContentIdentifiedEventArgs {}
impl ::core::clone::Clone for IWebViewControlUnviewableContentIdentifiedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IWebViewControlWebResourceRequestedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IWebViewControlWebResourceRequestedEventArgs {}
impl ::core::clone::Clone for IWebViewControlWebResourceRequestedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct WebViewControlContentLoadingEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for WebViewControlContentLoadingEventArgs {}
impl ::core::clone::Clone for WebViewControlContentLoadingEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct WebViewControlDOMContentLoadedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for WebViewControlDOMContentLoadedEventArgs {}
impl ::core::clone::Clone for WebViewControlDOMContentLoadedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct WebViewControlDeferredPermissionRequest(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for WebViewControlDeferredPermissionRequest {}
impl ::core::clone::Clone for WebViewControlDeferredPermissionRequest {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct WebViewControlLongRunningScriptDetectedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for WebViewControlLongRunningScriptDetectedEventArgs {}
impl ::core::clone::Clone for WebViewControlLongRunningScriptDetectedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct WebViewControlNavigationCompletedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for WebViewControlNavigationCompletedEventArgs {}
impl ::core::clone::Clone for WebViewControlNavigationCompletedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct WebViewControlNavigationStartingEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for WebViewControlNavigationStartingEventArgs {}
impl ::core::clone::Clone for WebViewControlNavigationStartingEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct WebViewControlNewWindowRequestedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for WebViewControlNewWindowRequestedEventArgs {}
impl ::core::clone::Clone for WebViewControlNewWindowRequestedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct WebViewControlPermissionRequest(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for WebViewControlPermissionRequest {}
impl ::core::clone::Clone for WebViewControlPermissionRequest {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct WebViewControlPermissionRequestedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for WebViewControlPermissionRequestedEventArgs {}
impl ::core::clone::Clone for WebViewControlPermissionRequestedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
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
impl ::core::marker::Copy for WebViewControlScriptNotifyEventArgs {}
impl ::core::clone::Clone for WebViewControlScriptNotifyEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct WebViewControlSettings(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for WebViewControlSettings {}
impl ::core::clone::Clone for WebViewControlSettings {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct WebViewControlUnsupportedUriSchemeIdentifiedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for WebViewControlUnsupportedUriSchemeIdentifiedEventArgs {}
impl ::core::clone::Clone for WebViewControlUnsupportedUriSchemeIdentifiedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct WebViewControlUnviewableContentIdentifiedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for WebViewControlUnviewableContentIdentifiedEventArgs {}
impl ::core::clone::Clone for WebViewControlUnviewableContentIdentifiedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct WebViewControlWebResourceRequestedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for WebViewControlWebResourceRequestedEventArgs {}
impl ::core::clone::Clone for WebViewControlWebResourceRequestedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
