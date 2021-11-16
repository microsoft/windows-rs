#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[link(name = "windows")]
extern "system" {}
pub const D2D1_ALPHA_MODE_UNKNOWN: u32 = 0u32;
pub const D2D1_ALPHA_MODE_PREMULTIPLIED: u32 = 1u32;
pub const D2D1_ALPHA_MODE_STRAIGHT: u32 = 2u32;
pub const D2D1_ALPHA_MODE_IGNORE: u32 = 3u32;
pub const D2D1_ALPHA_MODE_FORCE_DWORD: u32 = 4294967295u32;
#[repr(C)]
pub struct D2D1_BEZIER_SEGMENT {
    pub point1: D2D_POINT_2F,
    pub point2: D2D_POINT_2F,
    pub point3: D2D_POINT_2F,
}
impl ::core::marker::Copy for D2D1_BEZIER_SEGMENT {}
impl ::core::clone::Clone for D2D1_BEZIER_SEGMENT {
    fn clone(&self) -> Self {
        *self
    }
}
pub const D2D1_BLEND_MODE_MULTIPLY: u32 = 0u32;
pub const D2D1_BLEND_MODE_SCREEN: u32 = 1u32;
pub const D2D1_BLEND_MODE_DARKEN: u32 = 2u32;
pub const D2D1_BLEND_MODE_LIGHTEN: u32 = 3u32;
pub const D2D1_BLEND_MODE_DISSOLVE: u32 = 4u32;
pub const D2D1_BLEND_MODE_COLOR_BURN: u32 = 5u32;
pub const D2D1_BLEND_MODE_LINEAR_BURN: u32 = 6u32;
pub const D2D1_BLEND_MODE_DARKER_COLOR: u32 = 7u32;
pub const D2D1_BLEND_MODE_LIGHTER_COLOR: u32 = 8u32;
pub const D2D1_BLEND_MODE_COLOR_DODGE: u32 = 9u32;
pub const D2D1_BLEND_MODE_LINEAR_DODGE: u32 = 10u32;
pub const D2D1_BLEND_MODE_OVERLAY: u32 = 11u32;
pub const D2D1_BLEND_MODE_SOFT_LIGHT: u32 = 12u32;
pub const D2D1_BLEND_MODE_HARD_LIGHT: u32 = 13u32;
pub const D2D1_BLEND_MODE_VIVID_LIGHT: u32 = 14u32;
pub const D2D1_BLEND_MODE_LINEAR_LIGHT: u32 = 15u32;
pub const D2D1_BLEND_MODE_PIN_LIGHT: u32 = 16u32;
pub const D2D1_BLEND_MODE_HARD_MIX: u32 = 17u32;
pub const D2D1_BLEND_MODE_DIFFERENCE: u32 = 18u32;
pub const D2D1_BLEND_MODE_EXCLUSION: u32 = 19u32;
pub const D2D1_BLEND_MODE_HUE: u32 = 20u32;
pub const D2D1_BLEND_MODE_SATURATION: u32 = 21u32;
pub const D2D1_BLEND_MODE_COLOR: u32 = 22u32;
pub const D2D1_BLEND_MODE_LUMINOSITY: u32 = 23u32;
pub const D2D1_BLEND_MODE_SUBTRACT: u32 = 24u32;
pub const D2D1_BLEND_MODE_DIVISION: u32 = 25u32;
pub const D2D1_BLEND_MODE_FORCE_DWORD: u32 = 4294967295u32;
pub const D2D1_BORDER_MODE_SOFT: u32 = 0u32;
pub const D2D1_BORDER_MODE_HARD: u32 = 1u32;
pub const D2D1_BORDER_MODE_FORCE_DWORD: u32 = 4294967295u32;
pub const D2D1_COLORMATRIX_ALPHA_MODE_PREMULTIPLIED: u32 = 1u32;
pub const D2D1_COLORMATRIX_ALPHA_MODE_STRAIGHT: u32 = 2u32;
pub const D2D1_COLORMATRIX_ALPHA_MODE_FORCE_DWORD: u32 = 4294967295u32;
#[repr(C)]
pub struct D2D1_COLOR_F {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
}
impl ::core::marker::Copy for D2D1_COLOR_F {}
impl ::core::clone::Clone for D2D1_COLOR_F {
    fn clone(&self) -> Self {
        *self
    }
}
pub const D2D1_COMPOSITE_MODE_SOURCE_OVER: u32 = 0u32;
pub const D2D1_COMPOSITE_MODE_DESTINATION_OVER: u32 = 1u32;
pub const D2D1_COMPOSITE_MODE_SOURCE_IN: u32 = 2u32;
pub const D2D1_COMPOSITE_MODE_DESTINATION_IN: u32 = 3u32;
pub const D2D1_COMPOSITE_MODE_SOURCE_OUT: u32 = 4u32;
pub const D2D1_COMPOSITE_MODE_DESTINATION_OUT: u32 = 5u32;
pub const D2D1_COMPOSITE_MODE_SOURCE_ATOP: u32 = 6u32;
pub const D2D1_COMPOSITE_MODE_DESTINATION_ATOP: u32 = 7u32;
pub const D2D1_COMPOSITE_MODE_XOR: u32 = 8u32;
pub const D2D1_COMPOSITE_MODE_PLUS: u32 = 9u32;
pub const D2D1_COMPOSITE_MODE_SOURCE_COPY: u32 = 10u32;
pub const D2D1_COMPOSITE_MODE_BOUNDED_SOURCE_COPY: u32 = 11u32;
pub const D2D1_COMPOSITE_MODE_MASK_INVERT: u32 = 12u32;
pub const D2D1_COMPOSITE_MODE_FORCE_DWORD: u32 = 4294967295u32;
pub const D2D1_FIGURE_BEGIN_FILLED: u32 = 0u32;
pub const D2D1_FIGURE_BEGIN_HOLLOW: u32 = 1u32;
pub const D2D1_FIGURE_BEGIN_FORCE_DWORD: u32 = 4294967295u32;
pub const D2D1_FIGURE_END_OPEN: u32 = 0u32;
pub const D2D1_FIGURE_END_CLOSED: u32 = 1u32;
pub const D2D1_FIGURE_END_FORCE_DWORD: u32 = 4294967295u32;
pub const D2D1_FILL_MODE_ALTERNATE: u32 = 0u32;
pub const D2D1_FILL_MODE_WINDING: u32 = 1u32;
pub const D2D1_FILL_MODE_FORCE_DWORD: u32 = 4294967295u32;
pub const D2D1_PATH_SEGMENT_NONE: u32 = 0u32;
pub const D2D1_PATH_SEGMENT_FORCE_UNSTROKED: u32 = 1u32;
pub const D2D1_PATH_SEGMENT_FORCE_ROUND_LINE_JOIN: u32 = 2u32;
pub const D2D1_PATH_SEGMENT_FORCE_DWORD: u32 = 4294967295u32;
#[repr(C)]
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
pub struct D2D1_PIXEL_FORMAT {
    pub format: super::super::Dxgi::Common::DXGI_FORMAT,
    pub alphaMode: D2D1_ALPHA_MODE,
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::core::marker::Copy for D2D1_PIXEL_FORMAT {}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::core::clone::Clone for D2D1_PIXEL_FORMAT {
    fn clone(&self) -> Self {
        *self
    }
}
pub const D2D1_TURBULENCE_NOISE_FRACTAL_SUM: u32 = 0u32;
pub const D2D1_TURBULENCE_NOISE_TURBULENCE: u32 = 1u32;
pub const D2D1_TURBULENCE_NOISE_FORCE_DWORD: u32 = 4294967295u32;
#[repr(C)]
pub struct D2D_COLOR_F {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
}
impl ::core::marker::Copy for D2D_COLOR_F {}
impl ::core::clone::Clone for D2D_COLOR_F {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct D2D_MATRIX_3X2_F {
    pub Anonymous: D2D_MATRIX_3X2_F_0,
}
impl ::core::marker::Copy for D2D_MATRIX_3X2_F {}
impl ::core::clone::Clone for D2D_MATRIX_3X2_F {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub union D2D_MATRIX_3X2_F_0 {
    pub Anonymous1: D2D_MATRIX_3X2_F_0_0,
    pub Anonymous2: D2D_MATRIX_3X2_F_0_1,
    pub m: [f32; 6],
}
impl ::core::marker::Copy for D2D_MATRIX_3X2_F_0 {}
impl ::core::clone::Clone for D2D_MATRIX_3X2_F_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct D2D_MATRIX_3X2_F_0_0 {
    pub m11: f32,
    pub m12: f32,
    pub m21: f32,
    pub m22: f32,
    pub dx: f32,
    pub dy: f32,
}
impl ::core::marker::Copy for D2D_MATRIX_3X2_F_0_0 {}
impl ::core::clone::Clone for D2D_MATRIX_3X2_F_0_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct D2D_MATRIX_3X2_F_0_1 {
    pub _11: f32,
    pub _12: f32,
    pub _21: f32,
    pub _22: f32,
    pub _31: f32,
    pub _32: f32,
}
impl ::core::marker::Copy for D2D_MATRIX_3X2_F_0_1 {}
impl ::core::clone::Clone for D2D_MATRIX_3X2_F_0_1 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct D2D_MATRIX_4X3_F {
    pub Anonymous: D2D_MATRIX_4X3_F_0,
}
impl ::core::marker::Copy for D2D_MATRIX_4X3_F {}
impl ::core::clone::Clone for D2D_MATRIX_4X3_F {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub union D2D_MATRIX_4X3_F_0 {
    pub Anonymous: D2D_MATRIX_4X3_F_0_0,
    pub m: [f32; 12],
}
impl ::core::marker::Copy for D2D_MATRIX_4X3_F_0 {}
impl ::core::clone::Clone for D2D_MATRIX_4X3_F_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct D2D_MATRIX_4X3_F_0_0 {
    pub _11: f32,
    pub _12: f32,
    pub _13: f32,
    pub _21: f32,
    pub _22: f32,
    pub _23: f32,
    pub _31: f32,
    pub _32: f32,
    pub _33: f32,
    pub _41: f32,
    pub _42: f32,
    pub _43: f32,
}
impl ::core::marker::Copy for D2D_MATRIX_4X3_F_0_0 {}
impl ::core::clone::Clone for D2D_MATRIX_4X3_F_0_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct D2D_MATRIX_4X4_F {
    pub Anonymous: D2D_MATRIX_4X4_F_0,
}
impl ::core::marker::Copy for D2D_MATRIX_4X4_F {}
impl ::core::clone::Clone for D2D_MATRIX_4X4_F {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub union D2D_MATRIX_4X4_F_0 {
    pub Anonymous: D2D_MATRIX_4X4_F_0_0,
    pub m: [f32; 16],
}
impl ::core::marker::Copy for D2D_MATRIX_4X4_F_0 {}
impl ::core::clone::Clone for D2D_MATRIX_4X4_F_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct D2D_MATRIX_4X4_F_0_0 {
    pub _11: f32,
    pub _12: f32,
    pub _13: f32,
    pub _14: f32,
    pub _21: f32,
    pub _22: f32,
    pub _23: f32,
    pub _24: f32,
    pub _31: f32,
    pub _32: f32,
    pub _33: f32,
    pub _34: f32,
    pub _41: f32,
    pub _42: f32,
    pub _43: f32,
    pub _44: f32,
}
impl ::core::marker::Copy for D2D_MATRIX_4X4_F_0_0 {}
impl ::core::clone::Clone for D2D_MATRIX_4X4_F_0_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct D2D_MATRIX_5X4_F {
    pub Anonymous: D2D_MATRIX_5X4_F_0,
}
impl ::core::marker::Copy for D2D_MATRIX_5X4_F {}
impl ::core::clone::Clone for D2D_MATRIX_5X4_F {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub union D2D_MATRIX_5X4_F_0 {
    pub Anonymous: D2D_MATRIX_5X4_F_0_0,
    pub m: [f32; 20],
}
impl ::core::marker::Copy for D2D_MATRIX_5X4_F_0 {}
impl ::core::clone::Clone for D2D_MATRIX_5X4_F_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct D2D_MATRIX_5X4_F_0_0 {
    pub _11: f32,
    pub _12: f32,
    pub _13: f32,
    pub _14: f32,
    pub _21: f32,
    pub _22: f32,
    pub _23: f32,
    pub _24: f32,
    pub _31: f32,
    pub _32: f32,
    pub _33: f32,
    pub _34: f32,
    pub _41: f32,
    pub _42: f32,
    pub _43: f32,
    pub _44: f32,
    pub _51: f32,
    pub _52: f32,
    pub _53: f32,
    pub _54: f32,
}
impl ::core::marker::Copy for D2D_MATRIX_5X4_F_0_0 {}
impl ::core::clone::Clone for D2D_MATRIX_5X4_F_0_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct D2D_POINT_2F {
    pub x: f32,
    pub y: f32,
}
impl ::core::marker::Copy for D2D_POINT_2F {}
impl ::core::clone::Clone for D2D_POINT_2F {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct D2D_POINT_2U {
    pub x: u32,
    pub y: u32,
}
impl ::core::marker::Copy for D2D_POINT_2U {}
impl ::core::clone::Clone for D2D_POINT_2U {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct D2D_RECT_F {
    pub left: f32,
    pub top: f32,
    pub right: f32,
    pub bottom: f32,
}
impl ::core::marker::Copy for D2D_RECT_F {}
impl ::core::clone::Clone for D2D_RECT_F {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct D2D_RECT_U {
    pub left: u32,
    pub top: u32,
    pub right: u32,
    pub bottom: u32,
}
impl ::core::marker::Copy for D2D_RECT_U {}
impl ::core::clone::Clone for D2D_RECT_U {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct D2D_SIZE_F {
    pub width: f32,
    pub height: f32,
}
impl ::core::marker::Copy for D2D_SIZE_F {}
impl ::core::clone::Clone for D2D_SIZE_F {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct D2D_SIZE_U {
    pub width: u32,
    pub height: u32,
}
impl ::core::marker::Copy for D2D_SIZE_U {}
impl ::core::clone::Clone for D2D_SIZE_U {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct D2D_VECTOR_2F {
    pub x: f32,
    pub y: f32,
}
impl ::core::marker::Copy for D2D_VECTOR_2F {}
impl ::core::clone::Clone for D2D_VECTOR_2F {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct D2D_VECTOR_3F {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}
impl ::core::marker::Copy for D2D_VECTOR_3F {}
impl ::core::clone::Clone for D2D_VECTOR_3F {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct D2D_VECTOR_4F {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}
impl ::core::marker::Copy for D2D_VECTOR_4F {}
impl ::core::clone::Clone for D2D_VECTOR_4F {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ID2D1SimplifiedGeometrySink(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ID2D1SimplifiedGeometrySink {}
impl ::core::clone::Clone for ID2D1SimplifiedGeometrySink {
    fn clone(&self) -> Self {
        *self
    }
}
