#![allow(non_snake_case, non_camel_case_types)]
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
pub struct KnownSimpleHapticsControllerWaveforms(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct SimpleHapticsController(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct SimpleHapticsControllerFeedback(pub *mut ::core::ffi::c_void);
pub struct VibrationAccessStatus(i32);
#[repr(transparent)]
pub struct VibrationDevice(pub *mut ::core::ffi::c_void);
