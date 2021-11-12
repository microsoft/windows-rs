#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {}
#[repr(C)]
pub struct GRAPHICS_EFFECT_PROPERTY_MAPPING(i32);
#[repr(transparent)]
pub struct IGeometrySource2DInterop(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IGraphicsEffectD2D1Interop(pub *mut ::core::ffi::c_void);
