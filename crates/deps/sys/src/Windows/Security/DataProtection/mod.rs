#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct IUserDataAvailabilityStateChangedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IUserDataBufferUnprotectResult(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IUserDataProtectionManager(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IUserDataProtectionManagerStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IUserDataStorageItemProtectionInfo(pub *mut ::core::ffi::c_void);
#[repr(C)]
pub struct UserDataAvailability(i32);
#[repr(transparent)]
pub struct UserDataAvailabilityStateChangedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct UserDataBufferUnprotectResult(pub *mut ::core::ffi::c_void);
#[repr(C)]
pub struct UserDataBufferUnprotectStatus(i32);
#[repr(transparent)]
pub struct UserDataProtectionManager(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct UserDataStorageItemProtectionInfo(pub *mut ::core::ffi::c_void);
#[repr(C)]
pub struct UserDataStorageItemProtectionStatus(i32);
