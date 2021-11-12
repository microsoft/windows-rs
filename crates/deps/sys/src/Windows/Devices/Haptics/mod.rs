#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct IKnownSimpleHapticsControllerWaveformsStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IKnownSimpleHapticsControllerWaveformsStatics2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISimpleHapticsController(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISimpleHapticsControllerFeedback(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IVibrationDevice(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IVibrationDeviceStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct SimpleHapticsController(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct SimpleHapticsControllerFeedback(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct VibrationAccessStatus(pub i32);
impl VibrationAccessStatus {
    pub const Allowed: VibrationAccessStatus = VibrationAccessStatus(0i32);
    pub const DeniedByUser: VibrationAccessStatus = VibrationAccessStatus(1i32);
    pub const DeniedBySystem: VibrationAccessStatus = VibrationAccessStatus(2i32);
    pub const DeniedByEnergySaver: VibrationAccessStatus = VibrationAccessStatus(3i32);
}
#[repr(transparent)]
pub struct VibrationDevice(pub *mut ::core::ffi::c_void);
