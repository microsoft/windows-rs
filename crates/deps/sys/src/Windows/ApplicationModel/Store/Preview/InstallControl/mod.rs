#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
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
#[repr(transparent)]
pub struct AppInstallState(pub i32);
impl AppInstallState {
    pub const Pending: AppInstallState = AppInstallState(0i32);
    pub const Starting: AppInstallState = AppInstallState(1i32);
    pub const AcquiringLicense: AppInstallState = AppInstallState(2i32);
    pub const Downloading: AppInstallState = AppInstallState(3i32);
    pub const RestoringData: AppInstallState = AppInstallState(4i32);
    pub const Installing: AppInstallState = AppInstallState(5i32);
    pub const Completed: AppInstallState = AppInstallState(6i32);
    pub const Canceled: AppInstallState = AppInstallState(7i32);
    pub const Paused: AppInstallState = AppInstallState(8i32);
    pub const Error: AppInstallState = AppInstallState(9i32);
    pub const PausedLowBattery: AppInstallState = AppInstallState(10i32);
    pub const PausedWiFiRecommended: AppInstallState = AppInstallState(11i32);
    pub const PausedWiFiRequired: AppInstallState = AppInstallState(12i32);
    pub const ReadyToDownload: AppInstallState = AppInstallState(13i32);
}
#[repr(transparent)]
pub struct AppInstallStatus(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct AppInstallType(pub i32);
impl AppInstallType {
    pub const Install: AppInstallType = AppInstallType(0i32);
    pub const Update: AppInstallType = AppInstallType(1i32);
    pub const Repair: AppInstallType = AppInstallType(2i32);
}
#[repr(transparent)]
pub struct AppInstallationToastNotificationMode(pub i32);
impl AppInstallationToastNotificationMode {
    pub const Default: AppInstallationToastNotificationMode = AppInstallationToastNotificationMode(0i32);
    pub const Toast: AppInstallationToastNotificationMode = AppInstallationToastNotificationMode(1i32);
    pub const ToastWithoutPopup: AppInstallationToastNotificationMode = AppInstallationToastNotificationMode(2i32);
    pub const NoToast: AppInstallationToastNotificationMode = AppInstallationToastNotificationMode(3i32);
}
#[repr(transparent)]
pub struct AppUpdateOptions(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct AutoUpdateSetting(pub i32);
impl AutoUpdateSetting {
    pub const Disabled: AutoUpdateSetting = AutoUpdateSetting(0i32);
    pub const Enabled: AutoUpdateSetting = AutoUpdateSetting(1i32);
    pub const DisabledByPolicy: AutoUpdateSetting = AutoUpdateSetting(2i32);
    pub const EnabledByPolicy: AutoUpdateSetting = AutoUpdateSetting(3i32);
}
#[repr(transparent)]
pub struct GetEntitlementResult(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct GetEntitlementStatus(pub i32);
impl GetEntitlementStatus {
    pub const Succeeded: GetEntitlementStatus = GetEntitlementStatus(0i32);
    pub const NoStoreAccount: GetEntitlementStatus = GetEntitlementStatus(1i32);
    pub const NetworkError: GetEntitlementStatus = GetEntitlementStatus(2i32);
    pub const ServerError: GetEntitlementStatus = GetEntitlementStatus(3i32);
}
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
