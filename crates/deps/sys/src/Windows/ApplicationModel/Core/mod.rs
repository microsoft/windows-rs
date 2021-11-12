#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct AppListEntry(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct AppRestartFailureReason(pub i32);
impl AppRestartFailureReason {
    pub const RestartPending: Self = Self(0i32);
    pub const NotInForeground: Self = Self(1i32);
    pub const InvalidUser: Self = Self(2i32);
    pub const Other: Self = Self(3i32);
}
impl ::core::marker::Copy for AppRestartFailureReason {}
impl ::core::clone::Clone for AppRestartFailureReason {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct CoreApplicationView(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct CoreApplicationViewTitleBar(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct HostedViewClosingEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAppListEntry(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAppListEntry2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAppListEntry3(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAppListEntry4(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICoreApplication(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICoreApplication2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICoreApplication3(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICoreApplicationExit(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICoreApplicationUnhandledError(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICoreApplicationUseCount(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICoreApplicationView(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICoreApplicationView2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICoreApplicationView3(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICoreApplicationView5(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICoreApplicationView6(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICoreApplicationViewTitleBar(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICoreImmersiveApplication(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICoreImmersiveApplication2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICoreImmersiveApplication3(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IFrameworkView(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IFrameworkViewSource(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IHostedViewClosingEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IUnhandledError(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IUnhandledErrorDetectedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct UnhandledError(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct UnhandledErrorDetectedEventArgs(pub *mut ::core::ffi::c_void);
