#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct IPalmRejectionDelayZonePreview(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPalmRejectionDelayZonePreviewStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct PalmRejectionDelayZonePreview(pub *mut ::core::ffi::c_void);
