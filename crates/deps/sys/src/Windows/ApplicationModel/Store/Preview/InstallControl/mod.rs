#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct AppInstallItem(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for AppInstallItem {}
impl ::core::clone::Clone for AppInstallItem {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct AppInstallManager(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for AppInstallManager {}
impl ::core::clone::Clone for AppInstallManager {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct AppInstallManagerItemEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for AppInstallManagerItemEventArgs {}
impl ::core::clone::Clone for AppInstallManagerItemEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct AppInstallOptions(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for AppInstallOptions {}
impl ::core::clone::Clone for AppInstallOptions {
    fn clone(&self) -> Self {
        *self
    }
}
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
impl ::core::marker::Copy for AppInstallState {}
impl ::core::clone::Clone for AppInstallState {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct AppInstallStatus(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for AppInstallStatus {}
impl ::core::clone::Clone for AppInstallStatus {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct AppInstallType(pub i32);
impl AppInstallType {
    pub const Install: Self = Self(0i32);
    pub const Update: Self = Self(1i32);
    pub const Repair: Self = Self(2i32);
}
impl ::core::marker::Copy for AppInstallType {}
impl ::core::clone::Clone for AppInstallType {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct AppInstallationToastNotificationMode(pub i32);
impl AppInstallationToastNotificationMode {
    pub const Default: Self = Self(0i32);
    pub const Toast: Self = Self(1i32);
    pub const ToastWithoutPopup: Self = Self(2i32);
    pub const NoToast: Self = Self(3i32);
}
impl ::core::marker::Copy for AppInstallationToastNotificationMode {}
impl ::core::clone::Clone for AppInstallationToastNotificationMode {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct AppUpdateOptions(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for AppUpdateOptions {}
impl ::core::clone::Clone for AppUpdateOptions {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct AutoUpdateSetting(pub i32);
impl AutoUpdateSetting {
    pub const Disabled: Self = Self(0i32);
    pub const Enabled: Self = Self(1i32);
    pub const DisabledByPolicy: Self = Self(2i32);
    pub const EnabledByPolicy: Self = Self(3i32);
}
impl ::core::marker::Copy for AutoUpdateSetting {}
impl ::core::clone::Clone for AutoUpdateSetting {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct GetEntitlementResult(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for GetEntitlementResult {}
impl ::core::clone::Clone for GetEntitlementResult {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct GetEntitlementStatus(pub i32);
impl GetEntitlementStatus {
    pub const Succeeded: Self = Self(0i32);
    pub const NoStoreAccount: Self = Self(1i32);
    pub const NetworkError: Self = Self(2i32);
    pub const ServerError: Self = Self(3i32);
}
impl ::core::marker::Copy for GetEntitlementStatus {}
impl ::core::clone::Clone for GetEntitlementStatus {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IAppInstallItem(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IAppInstallItem {}
impl ::core::clone::Clone for IAppInstallItem {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IAppInstallItem2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IAppInstallItem2 {}
impl ::core::clone::Clone for IAppInstallItem2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IAppInstallItem3(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IAppInstallItem3 {}
impl ::core::clone::Clone for IAppInstallItem3 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IAppInstallItem4(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IAppInstallItem4 {}
impl ::core::clone::Clone for IAppInstallItem4 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IAppInstallItem5(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IAppInstallItem5 {}
impl ::core::clone::Clone for IAppInstallItem5 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IAppInstallManager(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IAppInstallManager {}
impl ::core::clone::Clone for IAppInstallManager {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IAppInstallManager2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IAppInstallManager2 {}
impl ::core::clone::Clone for IAppInstallManager2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IAppInstallManager3(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IAppInstallManager3 {}
impl ::core::clone::Clone for IAppInstallManager3 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IAppInstallManager4(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IAppInstallManager4 {}
impl ::core::clone::Clone for IAppInstallManager4 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IAppInstallManager5(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IAppInstallManager5 {}
impl ::core::clone::Clone for IAppInstallManager5 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IAppInstallManager6(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IAppInstallManager6 {}
impl ::core::clone::Clone for IAppInstallManager6 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IAppInstallManager7(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IAppInstallManager7 {}
impl ::core::clone::Clone for IAppInstallManager7 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IAppInstallManagerItemEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IAppInstallManagerItemEventArgs {}
impl ::core::clone::Clone for IAppInstallManagerItemEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IAppInstallOptions(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IAppInstallOptions {}
impl ::core::clone::Clone for IAppInstallOptions {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IAppInstallOptions2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IAppInstallOptions2 {}
impl ::core::clone::Clone for IAppInstallOptions2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IAppInstallStatus(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IAppInstallStatus {}
impl ::core::clone::Clone for IAppInstallStatus {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IAppInstallStatus2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IAppInstallStatus2 {}
impl ::core::clone::Clone for IAppInstallStatus2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IAppInstallStatus3(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IAppInstallStatus3 {}
impl ::core::clone::Clone for IAppInstallStatus3 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IAppUpdateOptions(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IAppUpdateOptions {}
impl ::core::clone::Clone for IAppUpdateOptions {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IAppUpdateOptions2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IAppUpdateOptions2 {}
impl ::core::clone::Clone for IAppUpdateOptions2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IGetEntitlementResult(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IGetEntitlementResult {}
impl ::core::clone::Clone for IGetEntitlementResult {
    fn clone(&self) -> Self {
        *self
    }
}
