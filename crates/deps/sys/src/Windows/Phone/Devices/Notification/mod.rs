#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[link(name = "windows")]
extern "system" {}
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
pub struct VibrationDevice(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for VibrationDevice {}
impl ::core::clone::Clone for VibrationDevice {
    fn clone(&self) -> Self {
        *self
    }
}
