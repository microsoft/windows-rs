#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {}
pub struct HolographicApplicationPreview(i32);
pub struct HolographicKeyboardPlacementOverridePreview(i32);
pub struct IHolographicApplicationPreviewStatics(pub *mut ::core::ffi::c_void);
pub struct IHolographicKeyboardPlacementOverridePreview(pub *mut ::core::ffi::c_void);
pub struct IHolographicKeyboardPlacementOverridePreviewStatics(pub *mut ::core::ffi::c_void);
