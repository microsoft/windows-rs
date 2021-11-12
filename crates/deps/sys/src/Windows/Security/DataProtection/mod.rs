#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
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
#[repr(transparent)]
pub struct UserDataAvailability(pub i32);
impl UserDataAvailability {
    pub const Always: UserDataAvailability = UserDataAvailability(0i32);
    pub const AfterFirstUnlock: UserDataAvailability = UserDataAvailability(1i32);
    pub const WhileUnlocked: UserDataAvailability = UserDataAvailability(2i32);
}
#[repr(transparent)]
pub struct UserDataAvailabilityStateChangedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct UserDataBufferUnprotectResult(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct UserDataBufferUnprotectStatus(pub i32);
impl UserDataBufferUnprotectStatus {
    pub const Succeeded: UserDataBufferUnprotectStatus = UserDataBufferUnprotectStatus(0i32);
    pub const Unavailable: UserDataBufferUnprotectStatus = UserDataBufferUnprotectStatus(1i32);
}
#[repr(transparent)]
pub struct UserDataProtectionManager(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct UserDataStorageItemProtectionInfo(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct UserDataStorageItemProtectionStatus(pub i32);
impl UserDataStorageItemProtectionStatus {
    pub const Succeeded: UserDataStorageItemProtectionStatus = UserDataStorageItemProtectionStatus(0i32);
    pub const NotProtectable: UserDataStorageItemProtectionStatus = UserDataStorageItemProtectionStatus(1i32);
    pub const DataUnavailable: UserDataStorageItemProtectionStatus = UserDataStorageItemProtectionStatus(2i32);
}
