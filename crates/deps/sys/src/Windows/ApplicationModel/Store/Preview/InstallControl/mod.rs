#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {}
pub struct AppInstallItem(i32);
pub struct AppInstallManager(i32);
pub struct AppInstallManagerItemEventArgs(i32);
pub struct AppInstallOptions(i32);
pub struct AppInstallState(i32);
pub struct AppInstallStatus(i32);
pub struct AppInstallType(i32);
pub struct AppInstallationToastNotificationMode(i32);
pub struct AppUpdateOptions(i32);
pub struct AutoUpdateSetting(i32);
pub struct GetEntitlementResult(i32);
pub struct GetEntitlementStatus(i32);
pub struct IAppInstallItem(pub *mut ::core::ffi::c_void);
pub struct IAppInstallItem2(pub *mut ::core::ffi::c_void);
pub struct IAppInstallItem3(pub *mut ::core::ffi::c_void);
pub struct IAppInstallItem4(pub *mut ::core::ffi::c_void);
pub struct IAppInstallItem5(pub *mut ::core::ffi::c_void);
pub struct IAppInstallManager(pub *mut ::core::ffi::c_void);
pub struct IAppInstallManager2(pub *mut ::core::ffi::c_void);
pub struct IAppInstallManager3(pub *mut ::core::ffi::c_void);
pub struct IAppInstallManager4(pub *mut ::core::ffi::c_void);
pub struct IAppInstallManager5(pub *mut ::core::ffi::c_void);
pub struct IAppInstallManager6(pub *mut ::core::ffi::c_void);
pub struct IAppInstallManager7(pub *mut ::core::ffi::c_void);
pub struct IAppInstallManagerItemEventArgs(pub *mut ::core::ffi::c_void);
pub struct IAppInstallOptions(pub *mut ::core::ffi::c_void);
pub struct IAppInstallOptions2(pub *mut ::core::ffi::c_void);
pub struct IAppInstallStatus(pub *mut ::core::ffi::c_void);
pub struct IAppInstallStatus2(pub *mut ::core::ffi::c_void);
pub struct IAppInstallStatus3(pub *mut ::core::ffi::c_void);
pub struct IAppUpdateOptions(pub *mut ::core::ffi::c_void);
pub struct IAppUpdateOptions2(pub *mut ::core::ffi::c_void);
pub struct IGetEntitlementResult(pub *mut ::core::ffi::c_void);
