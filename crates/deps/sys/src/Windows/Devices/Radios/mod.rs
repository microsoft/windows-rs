#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct IRadio(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IRadioStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct Radio(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct RadioAccessStatus(pub i32);
impl RadioAccessStatus {
    pub const Unspecified: RadioAccessStatus = RadioAccessStatus(0i32);
    pub const Allowed: RadioAccessStatus = RadioAccessStatus(1i32);
    pub const DeniedByUser: RadioAccessStatus = RadioAccessStatus(2i32);
    pub const DeniedBySystem: RadioAccessStatus = RadioAccessStatus(3i32);
}
#[repr(transparent)]
pub struct RadioKind(pub i32);
impl RadioKind {
    pub const Other: RadioKind = RadioKind(0i32);
    pub const WiFi: RadioKind = RadioKind(1i32);
    pub const MobileBroadband: RadioKind = RadioKind(2i32);
    pub const Bluetooth: RadioKind = RadioKind(3i32);
    pub const FM: RadioKind = RadioKind(4i32);
}
#[repr(transparent)]
pub struct RadioState(pub i32);
impl RadioState {
    pub const Unknown: RadioState = RadioState(0i32);
    pub const On: RadioState = RadioState(1i32);
    pub const Off: RadioState = RadioState(2i32);
    pub const Disabled: RadioState = RadioState(3i32);
}
