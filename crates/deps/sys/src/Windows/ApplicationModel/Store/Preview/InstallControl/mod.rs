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
    pub const Pending: Self = Self(0i32);
    pub const Starting: Self = Self(1i32);
    pub const AcquiringLicense: Self = Self(2i32);
    pub const Downloading: Self = Self(3i32);
    pub const RestoringData: Self = Self(4i32);
    pub const Installing: Self = Self(5i32);
    pub const Completed: Self = Self(6i32);
    pub const Canceled: Self = Self(7i32);
    pub const Paused: Self = Self(8i32);
    pub const Error: Self = Self(9i32);
    pub const PausedLowBattery: Self = Self(10i32);
    pub const PausedWiFiRecommended: Self = Self(11i32);
    pub const PausedWiFiRequired: Self = Self(12i32);
    pub const ReadyToDownload: Self = Self(13i32);
}
#[repr(transparent)]
pub struct AppInstallStatus(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct AppInstallType(pub i32);
impl AppInstallType {
    pub const Install: Self = Self(0i32);
    pub const Update: Self = Self(1i32);
    pub const Repair: Self = Self(2i32);
}
#[repr(transparent)]
pub struct AppInstallationToastNotificationMode(pub i32);
impl AppInstallationToastNotificationMode {
    pub const Default: Self = Self(0i32);
    pub const Toast: Self = Self(1i32);
    pub const ToastWithoutPopup: Self = Self(2i32);
    pub const NoToast: Self = Self(3i32);
}
#[repr(transparent)]
pub struct AppUpdateOptions(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct AutoUpdateSetting(pub i32);
impl AutoUpdateSetting {
    pub const Disabled: Self = Self(0i32);
    pub const Enabled: Self = Self(1i32);
    pub const DisabledByPolicy: Self = Self(2i32);
    pub const EnabledByPolicy: Self = Self(3i32);
}
#[repr(transparent)]
pub struct GetEntitlementResult(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct GetEntitlementStatus(pub i32);
impl GetEntitlementStatus {
    pub const Succeeded: Self = Self(0i32);
    pub const NoStoreAccount: Self = Self(1i32);
    pub const NetworkError: Self = Self(2i32);
    pub const ServerError: Self = Self(3i32);
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
