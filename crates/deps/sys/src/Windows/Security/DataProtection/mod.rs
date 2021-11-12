#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct IUserDataAvailabilityStateChangedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IUserDataAvailabilityStateChangedEventArgs {}
impl ::core::clone::Clone for IUserDataAvailabilityStateChangedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IUserDataBufferUnprotectResult(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IUserDataBufferUnprotectResult {}
impl ::core::clone::Clone for IUserDataBufferUnprotectResult {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IUserDataProtectionManager(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IUserDataProtectionManager {}
impl ::core::clone::Clone for IUserDataProtectionManager {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IUserDataProtectionManagerStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IUserDataProtectionManagerStatics {}
impl ::core::clone::Clone for IUserDataProtectionManagerStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IUserDataStorageItemProtectionInfo(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IUserDataStorageItemProtectionInfo {}
impl ::core::clone::Clone for IUserDataStorageItemProtectionInfo {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct UserDataAvailability(pub i32);
impl UserDataAvailability {
    pub const Always: Self = Self(0i32);
    pub const AfterFirstUnlock: Self = Self(1i32);
    pub const WhileUnlocked: Self = Self(2i32);
}
impl ::core::marker::Copy for UserDataAvailability {}
impl ::core::clone::Clone for UserDataAvailability {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct UserDataAvailabilityStateChangedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for UserDataAvailabilityStateChangedEventArgs {}
impl ::core::clone::Clone for UserDataAvailabilityStateChangedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct UserDataBufferUnprotectResult(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for UserDataBufferUnprotectResult {}
impl ::core::clone::Clone for UserDataBufferUnprotectResult {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct UserDataBufferUnprotectStatus(pub i32);
impl UserDataBufferUnprotectStatus {
    pub const Succeeded: Self = Self(0i32);
    pub const Unavailable: Self = Self(1i32);
}
impl ::core::marker::Copy for UserDataBufferUnprotectStatus {}
impl ::core::clone::Clone for UserDataBufferUnprotectStatus {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct UserDataProtectionManager(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for UserDataProtectionManager {}
impl ::core::clone::Clone for UserDataProtectionManager {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct UserDataStorageItemProtectionInfo(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for UserDataStorageItemProtectionInfo {}
impl ::core::clone::Clone for UserDataStorageItemProtectionInfo {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct UserDataStorageItemProtectionStatus(pub i32);
impl UserDataStorageItemProtectionStatus {
    pub const Succeeded: Self = Self(0i32);
    pub const NotProtectable: Self = Self(1i32);
    pub const DataUnavailable: Self = Self(2i32);
}
impl ::core::marker::Copy for UserDataStorageItemProtectionStatus {}
impl ::core::clone::Clone for UserDataStorageItemProtectionStatus {
    fn clone(&self) -> Self {
        *self
    }
}
