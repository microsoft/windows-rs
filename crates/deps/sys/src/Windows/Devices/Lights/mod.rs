#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[cfg(feature = "Devices_Lights_Effects")]
pub mod Effects;
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct ILamp(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ILampArray(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ILampArrayStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ILampAvailabilityChangedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ILampInfo(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ILampStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct Lamp(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct LampArray(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct LampArrayKind(pub i32);
impl LampArrayKind {
    pub const Undefined: LampArrayKind = LampArrayKind(0i32);
    pub const Keyboard: LampArrayKind = LampArrayKind(1i32);
    pub const Mouse: LampArrayKind = LampArrayKind(2i32);
    pub const GameController: LampArrayKind = LampArrayKind(3i32);
    pub const Peripheral: LampArrayKind = LampArrayKind(4i32);
    pub const Scene: LampArrayKind = LampArrayKind(5i32);
    pub const Notification: LampArrayKind = LampArrayKind(6i32);
    pub const Chassis: LampArrayKind = LampArrayKind(7i32);
    pub const Wearable: LampArrayKind = LampArrayKind(8i32);
    pub const Furniture: LampArrayKind = LampArrayKind(9i32);
    pub const Art: LampArrayKind = LampArrayKind(10i32);
}
#[repr(transparent)]
pub struct LampAvailabilityChangedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct LampInfo(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct LampPurposes(pub u32);
impl LampPurposes {
    pub const Undefined: LampPurposes = LampPurposes(0u32);
    pub const Control: LampPurposes = LampPurposes(1u32);
    pub const Accent: LampPurposes = LampPurposes(2u32);
    pub const Branding: LampPurposes = LampPurposes(4u32);
    pub const Status: LampPurposes = LampPurposes(8u32);
    pub const Illumination: LampPurposes = LampPurposes(16u32);
    pub const Presentation: LampPurposes = LampPurposes(32u32);
}
