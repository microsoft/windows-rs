#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct AppInstallItem(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct AppInstallManager(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct AppInstallManagerItemEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct AppInstallOptions(pub *mut ::core::ffi::c_void);
pub struct AppInstallState(i32);
#[repr(transparent)]
pub struct AppInstallStatus(pub *mut ::core::ffi::c_void);
pub struct AppInstallType(i32);
pub struct AppInstallationToastNotificationMode(i32);
#[repr(transparent)]
pub struct AppUpdateOptions(pub *mut ::core::ffi::c_void);
pub struct AutoUpdateSetting(i32);
#[repr(transparent)]
pub struct GetEntitlementResult(pub *mut ::core::ffi::c_void);
pub struct GetEntitlementStatus(i32);
#[repr(transparent)]
pub struct IAppInstallItem(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAppInstallItem2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAppInstallItem3(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAppInstallItem4(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAppInstallItem5(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAppInstallManager(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAppInstallManager2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAppInstallManager3(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAppInstallManager4(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAppInstallManager5(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAppInstallManager6(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAppInstallManager7(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAppInstallManagerItemEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAppInstallOptions(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAppInstallOptions2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAppInstallStatus(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAppInstallStatus2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAppInstallStatus3(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAppUpdateOptions(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAppUpdateOptions2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IGetEntitlementResult(pub *mut ::core::ffi::c_void);
