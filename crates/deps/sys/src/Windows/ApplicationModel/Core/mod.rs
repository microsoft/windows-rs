#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct AppListEntry(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for AppListEntry {}
impl ::core::clone::Clone for AppListEntry {
    fn clone(&self) -> Self {
        *self
    }
}
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
impl ::core::marker::Copy for CoreApplicationView {}
impl ::core::clone::Clone for CoreApplicationView {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct CoreApplicationViewTitleBar(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for CoreApplicationViewTitleBar {}
impl ::core::clone::Clone for CoreApplicationViewTitleBar {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct HostedViewClosingEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for HostedViewClosingEventArgs {}
impl ::core::clone::Clone for HostedViewClosingEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IAppListEntry(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IAppListEntry {}
impl ::core::clone::Clone for IAppListEntry {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IAppListEntry2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IAppListEntry2 {}
impl ::core::clone::Clone for IAppListEntry2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IAppListEntry3(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IAppListEntry3 {}
impl ::core::clone::Clone for IAppListEntry3 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IAppListEntry4(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IAppListEntry4 {}
impl ::core::clone::Clone for IAppListEntry4 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ICoreApplication(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ICoreApplication {}
impl ::core::clone::Clone for ICoreApplication {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ICoreApplication2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ICoreApplication2 {}
impl ::core::clone::Clone for ICoreApplication2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ICoreApplication3(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ICoreApplication3 {}
impl ::core::clone::Clone for ICoreApplication3 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ICoreApplicationExit(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ICoreApplicationExit {}
impl ::core::clone::Clone for ICoreApplicationExit {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ICoreApplicationUnhandledError(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ICoreApplicationUnhandledError {}
impl ::core::clone::Clone for ICoreApplicationUnhandledError {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ICoreApplicationUseCount(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ICoreApplicationUseCount {}
impl ::core::clone::Clone for ICoreApplicationUseCount {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ICoreApplicationView(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ICoreApplicationView {}
impl ::core::clone::Clone for ICoreApplicationView {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ICoreApplicationView2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ICoreApplicationView2 {}
impl ::core::clone::Clone for ICoreApplicationView2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ICoreApplicationView3(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ICoreApplicationView3 {}
impl ::core::clone::Clone for ICoreApplicationView3 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ICoreApplicationView5(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ICoreApplicationView5 {}
impl ::core::clone::Clone for ICoreApplicationView5 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ICoreApplicationView6(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ICoreApplicationView6 {}
impl ::core::clone::Clone for ICoreApplicationView6 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ICoreApplicationViewTitleBar(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ICoreApplicationViewTitleBar {}
impl ::core::clone::Clone for ICoreApplicationViewTitleBar {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ICoreImmersiveApplication(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ICoreImmersiveApplication {}
impl ::core::clone::Clone for ICoreImmersiveApplication {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ICoreImmersiveApplication2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ICoreImmersiveApplication2 {}
impl ::core::clone::Clone for ICoreImmersiveApplication2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ICoreImmersiveApplication3(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ICoreImmersiveApplication3 {}
impl ::core::clone::Clone for ICoreImmersiveApplication3 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IFrameworkView(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IFrameworkView {}
impl ::core::clone::Clone for IFrameworkView {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IFrameworkViewSource(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IFrameworkViewSource {}
impl ::core::clone::Clone for IFrameworkViewSource {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IHostedViewClosingEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IHostedViewClosingEventArgs {}
impl ::core::clone::Clone for IHostedViewClosingEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IUnhandledError(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IUnhandledError {}
impl ::core::clone::Clone for IUnhandledError {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IUnhandledErrorDetectedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IUnhandledErrorDetectedEventArgs {}
impl ::core::clone::Clone for IUnhandledErrorDetectedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct UnhandledError(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for UnhandledError {}
impl ::core::clone::Clone for UnhandledError {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct UnhandledErrorDetectedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for UnhandledErrorDetectedEventArgs {}
impl ::core::clone::Clone for UnhandledErrorDetectedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
