#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {}
pub struct BackPressedEventArgs(i32);
pub struct CameraEventArgs(i32);
pub struct HardwareButtons(i32);
pub struct IBackPressedEventArgs(pub *mut ::core::ffi::c_void);
pub struct ICameraEventArgs(pub *mut ::core::ffi::c_void);
pub struct IHardwareButtonsStatics(pub *mut ::core::ffi::c_void);
pub struct IHardwareButtonsStatics2(pub *mut ::core::ffi::c_void);
