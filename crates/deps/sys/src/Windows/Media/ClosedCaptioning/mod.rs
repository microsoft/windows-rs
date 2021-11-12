#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {}
#[repr(C)]
pub struct ClosedCaptionColor(i32);
#[repr(C)]
pub struct ClosedCaptionEdgeEffect(i32);
#[repr(C)]
pub struct ClosedCaptionOpacity(i32);
#[repr(transparent)]
pub struct ClosedCaptionProperties(pub *mut ::core::ffi::c_void);
#[repr(C)]
pub struct ClosedCaptionSize(i32);
#[repr(C)]
pub struct ClosedCaptionStyle(i32);
#[repr(transparent)]
pub struct IClosedCaptionPropertiesStatics(pub *mut ::core::ffi::c_void);
