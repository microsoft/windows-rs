#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct IKnownSimpleHapticsControllerWaveformsStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IKnownSimpleHapticsControllerWaveformsStatics {}
impl ::core::clone::Clone for IKnownSimpleHapticsControllerWaveformsStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IKnownSimpleHapticsControllerWaveformsStatics2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IKnownSimpleHapticsControllerWaveformsStatics2 {}
impl ::core::clone::Clone for IKnownSimpleHapticsControllerWaveformsStatics2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISimpleHapticsController(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISimpleHapticsController {}
impl ::core::clone::Clone for ISimpleHapticsController {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISimpleHapticsControllerFeedback(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISimpleHapticsControllerFeedback {}
impl ::core::clone::Clone for ISimpleHapticsControllerFeedback {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IVibrationDevice(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IVibrationDevice {}
impl ::core::clone::Clone for IVibrationDevice {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IVibrationDeviceStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IVibrationDeviceStatics {}
impl ::core::clone::Clone for IVibrationDeviceStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct SimpleHapticsController(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for SimpleHapticsController {}
impl ::core::clone::Clone for SimpleHapticsController {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct SimpleHapticsControllerFeedback(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for SimpleHapticsControllerFeedback {}
impl ::core::clone::Clone for SimpleHapticsControllerFeedback {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct VibrationAccessStatus(pub i32);
impl VibrationAccessStatus {
    pub const Allowed: Self = Self(0i32);
    pub const DeniedByUser: Self = Self(1i32);
    pub const DeniedBySystem: Self = Self(2i32);
    pub const DeniedByEnergySaver: Self = Self(3i32);
}
impl ::core::marker::Copy for VibrationAccessStatus {}
impl ::core::clone::Clone for VibrationAccessStatus {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct VibrationDevice(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for VibrationDevice {}
impl ::core::clone::Clone for VibrationDevice {
    fn clone(&self) -> Self {
        *self
    }
}
