#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {}
pub struct DeviceAccountAuthenticationType(i32);
#[repr(transparent)]
pub struct DeviceAccountConfiguration(pub *mut ::core::ffi::c_void);
pub struct DeviceAccountIconId(i32);
pub struct DeviceAccountMailAgeFilter(i32);
pub struct DeviceAccountServerType(i32);
pub struct DeviceAccountSyncScheduleKind(i32);
#[repr(transparent)]
pub struct IDeviceAccountConfiguration(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDeviceAccountConfiguration2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IUserDataAccountSystemAccessManagerStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IUserDataAccountSystemAccessManagerStatics2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct UserDataAccountSystemAccessManager(pub *mut ::core::ffi::c_void);
