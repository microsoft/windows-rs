#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct HolographicKeyboardPlacementOverridePreview(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IHolographicApplicationPreviewStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IHolographicKeyboardPlacementOverridePreview(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IHolographicKeyboardPlacementOverridePreviewStatics(pub *mut ::core::ffi::c_void);
