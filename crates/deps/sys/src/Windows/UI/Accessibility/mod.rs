#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {}
pub struct IScreenReaderPositionChangedEventArgs(pub *mut ::core::ffi::c_void);
pub struct IScreenReaderService(pub *mut ::core::ffi::c_void);
pub struct ScreenReaderPositionChangedEventArgs(i32);
pub struct ScreenReaderService(i32);
