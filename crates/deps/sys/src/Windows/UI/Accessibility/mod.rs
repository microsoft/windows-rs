#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct IScreenReaderPositionChangedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IScreenReaderService(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ScreenReaderPositionChangedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ScreenReaderService(pub *mut ::core::ffi::c_void);
