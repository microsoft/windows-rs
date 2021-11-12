#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {}
pub struct IUserDataAvailabilityStateChangedEventArgs(pub *mut ::core::ffi::c_void);
pub struct IUserDataBufferUnprotectResult(pub *mut ::core::ffi::c_void);
pub struct IUserDataProtectionManager(pub *mut ::core::ffi::c_void);
pub struct IUserDataProtectionManagerStatics(pub *mut ::core::ffi::c_void);
pub struct IUserDataStorageItemProtectionInfo(pub *mut ::core::ffi::c_void);
pub struct UserDataAvailability(i32);
pub struct UserDataAvailabilityStateChangedEventArgs(i32);
pub struct UserDataBufferUnprotectResult(i32);
pub struct UserDataBufferUnprotectStatus(i32);
pub struct UserDataProtectionManager(i32);
pub struct UserDataStorageItemProtectionInfo(i32);
pub struct UserDataStorageItemProtectionStatus(i32);
