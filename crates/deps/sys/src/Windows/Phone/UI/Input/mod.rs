#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct BackPressedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct CameraEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IBackPressedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICameraEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IHardwareButtonsStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IHardwareButtonsStatics2(pub *mut ::core::ffi::c_void);
