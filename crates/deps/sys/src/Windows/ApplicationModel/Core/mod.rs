#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {}
pub struct AppListEntry(i32);
pub struct AppRestartFailureReason(i32);
pub struct CoreApplication(i32);
pub struct CoreApplicationView(i32);
pub struct CoreApplicationViewTitleBar(i32);
pub struct HostedViewClosingEventArgs(i32);
pub struct IAppListEntry(pub *mut ::core::ffi::c_void);
pub struct IAppListEntry2(pub *mut ::core::ffi::c_void);
pub struct IAppListEntry3(pub *mut ::core::ffi::c_void);
pub struct IAppListEntry4(pub *mut ::core::ffi::c_void);
pub struct ICoreApplication(pub *mut ::core::ffi::c_void);
pub struct ICoreApplication2(pub *mut ::core::ffi::c_void);
pub struct ICoreApplication3(pub *mut ::core::ffi::c_void);
pub struct ICoreApplicationExit(pub *mut ::core::ffi::c_void);
pub struct ICoreApplicationUnhandledError(pub *mut ::core::ffi::c_void);
pub struct ICoreApplicationUseCount(pub *mut ::core::ffi::c_void);
pub struct ICoreApplicationView(pub *mut ::core::ffi::c_void);
pub struct ICoreApplicationView2(pub *mut ::core::ffi::c_void);
pub struct ICoreApplicationView3(pub *mut ::core::ffi::c_void);
pub struct ICoreApplicationView5(pub *mut ::core::ffi::c_void);
pub struct ICoreApplicationView6(pub *mut ::core::ffi::c_void);
pub struct ICoreApplicationViewTitleBar(pub *mut ::core::ffi::c_void);
pub struct ICoreImmersiveApplication(pub *mut ::core::ffi::c_void);
pub struct ICoreImmersiveApplication2(pub *mut ::core::ffi::c_void);
pub struct ICoreImmersiveApplication3(pub *mut ::core::ffi::c_void);
pub struct IFrameworkView(pub *mut ::core::ffi::c_void);
pub struct IFrameworkViewSource(pub *mut ::core::ffi::c_void);
pub struct IHostedViewClosingEventArgs(pub *mut ::core::ffi::c_void);
pub struct IUnhandledError(pub *mut ::core::ffi::c_void);
pub struct IUnhandledErrorDetectedEventArgs(pub *mut ::core::ffi::c_void);
pub struct UnhandledError(i32);
pub struct UnhandledErrorDetectedEventArgs(i32);
