#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {}
#[repr(C)]
pub struct D2D1_ALPHA_MODE(i32);
#[repr(C)]
pub struct D2D1_BEZIER_SEGMENT(i32);
#[repr(C)]
pub struct D2D1_BLEND_MODE(i32);
#[repr(C)]
pub struct D2D1_BORDER_MODE(i32);
#[repr(C)]
pub struct D2D1_COLORMATRIX_ALPHA_MODE(i32);
#[repr(C)]
pub struct D2D1_COLOR_F(i32);
#[repr(C)]
pub struct D2D1_COMPOSITE_MODE(i32);
#[repr(C)]
pub struct D2D1_FIGURE_BEGIN(i32);
#[repr(C)]
pub struct D2D1_FIGURE_END(i32);
#[repr(C)]
pub struct D2D1_FILL_MODE(i32);
#[repr(C)]
pub struct D2D1_PATH_SEGMENT(i32);
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
#[repr(C)]
pub struct D2D1_PIXEL_FORMAT(i32);
#[repr(C)]
pub struct D2D1_TURBULENCE_NOISE(i32);
#[repr(C)]
pub struct D2D_COLOR_F(i32);
#[repr(C)]
pub struct D2D_MATRIX_3X2_F(i32);
#[repr(C)]
pub struct D2D_MATRIX_4X3_F(i32);
#[repr(C)]
pub struct D2D_MATRIX_4X4_F(i32);
#[repr(C)]
pub struct D2D_MATRIX_5X4_F(i32);
#[repr(C)]
pub struct D2D_POINT_2F(i32);
#[repr(C)]
pub struct D2D_POINT_2U(i32);
#[repr(C)]
pub struct D2D_RECT_F(i32);
#[repr(C)]
pub struct D2D_RECT_U(i32);
#[repr(C)]
pub struct D2D_SIZE_F(i32);
#[repr(C)]
pub struct D2D_SIZE_U(i32);
#[repr(C)]
pub struct D2D_VECTOR_2F(i32);
#[repr(C)]
pub struct D2D_VECTOR_3F(i32);
#[repr(C)]
pub struct D2D_VECTOR_4F(i32);
#[repr(transparent)]
pub struct ID2D1SimplifiedGeometrySink(pub *mut ::core::ffi::c_void);
