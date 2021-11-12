#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {}
pub struct ClosedCaptionColor(i32);
pub struct ClosedCaptionEdgeEffect(i32);
pub struct ClosedCaptionOpacity(i32);
pub struct ClosedCaptionProperties(i32);
pub struct ClosedCaptionSize(i32);
pub struct ClosedCaptionStyle(i32);
pub struct IClosedCaptionPropertiesStatics(pub *mut ::core::ffi::c_void);
