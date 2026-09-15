pub type D2D1_ALPHA_MODE = i32;
pub const D2D1_ALPHA_MODE_FORCE_DWORD: D2D1_ALPHA_MODE = -1;
pub const D2D1_ALPHA_MODE_IGNORE: D2D1_ALPHA_MODE = 3;
pub const D2D1_ALPHA_MODE_PREMULTIPLIED: D2D1_ALPHA_MODE = 1;
pub const D2D1_ALPHA_MODE_STRAIGHT: D2D1_ALPHA_MODE = 2;
pub const D2D1_ALPHA_MODE_UNKNOWN: D2D1_ALPHA_MODE = 0;
#[cfg(feature = "minwindef")]
pub type D2D1_MATRIX_3X2_F = D2D_MATRIX_3X2_F;
#[repr(C)]
#[cfg(feature = "dxgi")]
#[derive(Clone, Copy, Default)]
pub struct D2D1_PIXEL_FORMAT {
    pub format: super::DXGI_FORMAT,
    pub alphaMode: D2D1_ALPHA_MODE,
}
#[cfg(feature = "minwindef")]
pub type D2D1_POINT_2F = D2D_POINT_2F;
#[cfg(feature = "windef")]
pub type D2D1_POINT_2L = D2D_POINT_2L;
pub type D2D1_POINT_2U = D2D_POINT_2U;
#[cfg(feature = "minwindef")]
pub type D2D1_RECT_F = D2D_RECT_F;
#[cfg(feature = "windef")]
pub type D2D1_RECT_L = D2D_RECT_L;
pub type D2D1_RECT_U = D2D_RECT_U;
#[cfg(feature = "minwindef")]
pub type D2D1_SIZE_F = D2D_SIZE_F;
pub type D2D1_SIZE_U = D2D_SIZE_U;
#[repr(C)]
#[cfg(feature = "minwindef")]
#[derive(Clone, Copy)]
pub struct D2D_MATRIX_3X2_F {
    pub Anonymous: D2D_MATRIX_3X2_F_0,
}
#[cfg(feature = "minwindef")]
impl Default for D2D_MATRIX_3X2_F {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "minwindef")]
#[derive(Clone, Copy)]
pub union D2D_MATRIX_3X2_F_0 {
    pub Anonymous: D2D_MATRIX_3X2_F_0_0,
    pub Anonymous2: D2D_MATRIX_3X2_F_0_1,
    pub m: [[super::FLOAT; 2]; 3],
}
#[cfg(feature = "minwindef")]
impl Default for D2D_MATRIX_3X2_F_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "minwindef")]
#[derive(Clone, Copy, Default)]
pub struct D2D_MATRIX_3X2_F_0_0 {
    pub m11: super::FLOAT,
    pub m12: super::FLOAT,
    pub m21: super::FLOAT,
    pub m22: super::FLOAT,
    pub dx: super::FLOAT,
    pub dy: super::FLOAT,
}
#[repr(C)]
#[cfg(feature = "minwindef")]
#[derive(Clone, Copy, Default)]
pub struct D2D_MATRIX_3X2_F_0_1 {
    pub _11: super::FLOAT,
    pub _12: super::FLOAT,
    pub _21: super::FLOAT,
    pub _22: super::FLOAT,
    pub _31: super::FLOAT,
    pub _32: super::FLOAT,
}
#[repr(C)]
#[cfg(feature = "minwindef")]
#[derive(Clone, Copy)]
pub struct D2D_MATRIX_4X3_F {
    pub Anonymous: D2D_MATRIX_4X3_F_0,
}
#[cfg(feature = "minwindef")]
impl Default for D2D_MATRIX_4X3_F {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "minwindef")]
#[derive(Clone, Copy)]
pub union D2D_MATRIX_4X3_F_0 {
    pub Anonymous: D2D_MATRIX_4X3_F_0_0,
    pub m: [[super::FLOAT; 3]; 4],
}
#[cfg(feature = "minwindef")]
impl Default for D2D_MATRIX_4X3_F_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "minwindef")]
#[derive(Clone, Copy, Default)]
pub struct D2D_MATRIX_4X3_F_0_0 {
    pub _11: super::FLOAT,
    pub _12: super::FLOAT,
    pub _13: super::FLOAT,
    pub _21: super::FLOAT,
    pub _22: super::FLOAT,
    pub _23: super::FLOAT,
    pub _31: super::FLOAT,
    pub _32: super::FLOAT,
    pub _33: super::FLOAT,
    pub _41: super::FLOAT,
    pub _42: super::FLOAT,
    pub _43: super::FLOAT,
}
#[repr(C)]
#[cfg(feature = "minwindef")]
#[derive(Clone, Copy)]
pub struct D2D_MATRIX_4X4_F {
    pub Anonymous: D2D_MATRIX_4X4_F_0,
}
#[cfg(feature = "minwindef")]
impl Default for D2D_MATRIX_4X4_F {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "minwindef")]
#[derive(Clone, Copy)]
pub union D2D_MATRIX_4X4_F_0 {
    pub Anonymous: D2D_MATRIX_4X4_F_0_0,
    pub m: [[super::FLOAT; 4]; 4],
}
#[cfg(feature = "minwindef")]
impl Default for D2D_MATRIX_4X4_F_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "minwindef")]
#[derive(Clone, Copy, Default)]
pub struct D2D_MATRIX_4X4_F_0_0 {
    pub _11: super::FLOAT,
    pub _12: super::FLOAT,
    pub _13: super::FLOAT,
    pub _14: super::FLOAT,
    pub _21: super::FLOAT,
    pub _22: super::FLOAT,
    pub _23: super::FLOAT,
    pub _24: super::FLOAT,
    pub _31: super::FLOAT,
    pub _32: super::FLOAT,
    pub _33: super::FLOAT,
    pub _34: super::FLOAT,
    pub _41: super::FLOAT,
    pub _42: super::FLOAT,
    pub _43: super::FLOAT,
    pub _44: super::FLOAT,
}
#[repr(C)]
#[cfg(feature = "minwindef")]
#[derive(Clone, Copy)]
pub struct D2D_MATRIX_5X4_F {
    pub Anonymous: D2D_MATRIX_5X4_F_0,
}
#[cfg(feature = "minwindef")]
impl Default for D2D_MATRIX_5X4_F {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "minwindef")]
#[derive(Clone, Copy)]
pub union D2D_MATRIX_5X4_F_0 {
    pub Anonymous: D2D_MATRIX_5X4_F_0_0,
    pub m: [[super::FLOAT; 4]; 5],
}
#[cfg(feature = "minwindef")]
impl Default for D2D_MATRIX_5X4_F_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "minwindef")]
#[derive(Clone, Copy, Default)]
pub struct D2D_MATRIX_5X4_F_0_0 {
    pub _11: super::FLOAT,
    pub _12: super::FLOAT,
    pub _13: super::FLOAT,
    pub _14: super::FLOAT,
    pub _21: super::FLOAT,
    pub _22: super::FLOAT,
    pub _23: super::FLOAT,
    pub _24: super::FLOAT,
    pub _31: super::FLOAT,
    pub _32: super::FLOAT,
    pub _33: super::FLOAT,
    pub _34: super::FLOAT,
    pub _41: super::FLOAT,
    pub _42: super::FLOAT,
    pub _43: super::FLOAT,
    pub _44: super::FLOAT,
    pub _51: super::FLOAT,
    pub _52: super::FLOAT,
    pub _53: super::FLOAT,
    pub _54: super::FLOAT,
}
#[repr(C)]
#[cfg(feature = "minwindef")]
#[derive(Clone, Copy, Default)]
pub struct D2D_POINT_2F {
    pub x: super::FLOAT,
    pub y: super::FLOAT,
}
#[cfg(feature = "windef")]
pub type D2D_POINT_2L = super::POINT;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct D2D_POINT_2U {
    pub x: u32,
    pub y: u32,
}
#[repr(C)]
#[cfg(feature = "minwindef")]
#[derive(Clone, Copy, Default)]
pub struct D2D_RECT_F {
    pub left: super::FLOAT,
    pub top: super::FLOAT,
    pub right: super::FLOAT,
    pub bottom: super::FLOAT,
}
#[cfg(feature = "windef")]
pub type D2D_RECT_L = super::RECT;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct D2D_RECT_U {
    pub left: u32,
    pub top: u32,
    pub right: u32,
    pub bottom: u32,
}
#[repr(C)]
#[cfg(feature = "minwindef")]
#[derive(Clone, Copy, Default)]
pub struct D2D_SIZE_F {
    pub width: super::FLOAT,
    pub height: super::FLOAT,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct D2D_SIZE_U {
    pub width: u32,
    pub height: u32,
}
#[repr(C)]
#[cfg(feature = "minwindef")]
#[derive(Clone, Copy, Default)]
pub struct D2D_VECTOR_2F {
    pub x: super::FLOAT,
    pub y: super::FLOAT,
}
#[repr(C)]
#[cfg(feature = "minwindef")]
#[derive(Clone, Copy, Default)]
pub struct D2D_VECTOR_3F {
    pub x: super::FLOAT,
    pub y: super::FLOAT,
    pub z: super::FLOAT,
}
#[repr(C)]
#[cfg(feature = "minwindef")]
#[derive(Clone, Copy, Default)]
pub struct D2D_VECTOR_4F {
    pub x: super::FLOAT,
    pub y: super::FLOAT,
    pub z: super::FLOAT,
    pub w: super::FLOAT,
}
pub type DWRITE_GLYPH_IMAGE_FORMATS = u32;
pub const DWRITE_GLYPH_IMAGE_FORMATS_CFF: DWRITE_GLYPH_IMAGE_FORMATS = 2;
pub const DWRITE_GLYPH_IMAGE_FORMATS_COLR: DWRITE_GLYPH_IMAGE_FORMATS = 4;
pub const DWRITE_GLYPH_IMAGE_FORMATS_COLR_PAINT_TREE: DWRITE_GLYPH_IMAGE_FORMATS = 256;
pub const DWRITE_GLYPH_IMAGE_FORMATS_JPEG: DWRITE_GLYPH_IMAGE_FORMATS = 32;
pub const DWRITE_GLYPH_IMAGE_FORMATS_NONE: DWRITE_GLYPH_IMAGE_FORMATS = 0;
pub const DWRITE_GLYPH_IMAGE_FORMATS_PNG: DWRITE_GLYPH_IMAGE_FORMATS = 16;
pub const DWRITE_GLYPH_IMAGE_FORMATS_PREMULTIPLIED_B8G8R8A8: DWRITE_GLYPH_IMAGE_FORMATS = 128;
pub const DWRITE_GLYPH_IMAGE_FORMATS_SVG: DWRITE_GLYPH_IMAGE_FORMATS = 8;
pub const DWRITE_GLYPH_IMAGE_FORMATS_TIFF: DWRITE_GLYPH_IMAGE_FORMATS = 64;
pub const DWRITE_GLYPH_IMAGE_FORMATS_TRUETYPE: DWRITE_GLYPH_IMAGE_FORMATS = 1;
pub type DWRITE_MEASURING_MODE = i32;
pub const DWRITE_MEASURING_MODE_GDI_CLASSIC: DWRITE_MEASURING_MODE = 1;
pub const DWRITE_MEASURING_MODE_GDI_NATURAL: DWRITE_MEASURING_MODE = 2;
pub const DWRITE_MEASURING_MODE_NATURAL: DWRITE_MEASURING_MODE = 0;
