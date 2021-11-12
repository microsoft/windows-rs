#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {}
pub struct DeviceAccountAuthenticationType(i32);
pub struct DeviceAccountConfiguration(i32);
pub struct DeviceAccountIconId(i32);
pub struct DeviceAccountMailAgeFilter(i32);
pub struct DeviceAccountServerType(i32);
pub struct DeviceAccountSyncScheduleKind(i32);
pub struct IDeviceAccountConfiguration(pub *mut ::core::ffi::c_void);
pub struct IDeviceAccountConfiguration2(pub *mut ::core::ffi::c_void);
pub struct IUserDataAccountSystemAccessManagerStatics(pub *mut ::core::ffi::c_void);
pub struct IUserDataAccountSystemAccessManagerStatics2(pub *mut ::core::ffi::c_void);
pub struct UserDataAccountSystemAccessManager(i32);
