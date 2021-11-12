#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct BackPressedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for BackPressedEventArgs {}
impl ::core::clone::Clone for BackPressedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct CameraEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for CameraEventArgs {}
impl ::core::clone::Clone for CameraEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IBackPressedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IBackPressedEventArgs {}
impl ::core::clone::Clone for IBackPressedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ICameraEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ICameraEventArgs {}
impl ::core::clone::Clone for ICameraEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IHardwareButtonsStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IHardwareButtonsStatics {}
impl ::core::clone::Clone for IHardwareButtonsStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IHardwareButtonsStatics2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IHardwareButtonsStatics2 {}
impl ::core::clone::Clone for IHardwareButtonsStatics2 {
    fn clone(&self) -> Self {
        *self
    }
}
