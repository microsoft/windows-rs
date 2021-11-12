#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {}
#[repr(C)]
pub struct DeviceAccountAuthenticationType(i32);
#[repr(transparent)]
pub struct DeviceAccountConfiguration(pub *mut ::core::ffi::c_void);
#[repr(C)]
pub struct DeviceAccountIconId(i32);
#[repr(C)]
pub struct DeviceAccountMailAgeFilter(i32);
#[repr(C)]
pub struct DeviceAccountServerType(i32);
#[repr(C)]
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
