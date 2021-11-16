#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[link(name = "windows")]
extern "system" {}
pub const GRAPHICS_EFFECT_PROPERTY_MAPPING_UNKNOWN: i32 = 0i32;
pub const GRAPHICS_EFFECT_PROPERTY_MAPPING_DIRECT: i32 = 1i32;
pub const GRAPHICS_EFFECT_PROPERTY_MAPPING_VECTORX: i32 = 2i32;
pub const GRAPHICS_EFFECT_PROPERTY_MAPPING_VECTORY: i32 = 3i32;
pub const GRAPHICS_EFFECT_PROPERTY_MAPPING_VECTORZ: i32 = 4i32;
pub const GRAPHICS_EFFECT_PROPERTY_MAPPING_VECTORW: i32 = 5i32;
pub const GRAPHICS_EFFECT_PROPERTY_MAPPING_RECT_TO_VECTOR4: i32 = 6i32;
pub const GRAPHICS_EFFECT_PROPERTY_MAPPING_RADIANS_TO_DEGREES: i32 = 7i32;
pub const GRAPHICS_EFFECT_PROPERTY_MAPPING_COLORMATRIX_ALPHA_MODE: i32 = 8i32;
pub const GRAPHICS_EFFECT_PROPERTY_MAPPING_COLOR_TO_VECTOR3: i32 = 9i32;
pub const GRAPHICS_EFFECT_PROPERTY_MAPPING_COLOR_TO_VECTOR4: i32 = 10i32;
#[repr(transparent)]
pub struct IGeometrySource2DInterop(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IGeometrySource2DInterop {}
impl ::core::clone::Clone for IGeometrySource2DInterop {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IGraphicsEffectD2D1Interop(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IGraphicsEffectD2D1Interop {}
impl ::core::clone::Clone for IGraphicsEffectD2D1Interop {
    fn clone(&self) -> Self {
        *self
    }
}
