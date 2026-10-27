pub type D2D1_ALPHA_MODE = i32;
pub const D2D1_ALPHA_MODE_FORCE_DWORD: D2D1_ALPHA_MODE = -1;
pub const D2D1_ALPHA_MODE_IGNORE: D2D1_ALPHA_MODE = 3;
pub const D2D1_ALPHA_MODE_PREMULTIPLIED: D2D1_ALPHA_MODE = 1;
pub const D2D1_ALPHA_MODE_STRAIGHT: D2D1_ALPHA_MODE = 2;
pub const D2D1_ALPHA_MODE_UNKNOWN: D2D1_ALPHA_MODE = 0;
#[repr(C)]
#[cfg(feature = "dxgi")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct D2D1_PIXEL_FORMAT {
    pub format: super::DXGI_FORMAT,
    pub alphaMode: D2D1_ALPHA_MODE,
}
#[cfg(feature = "shared")]
pub type D2D1_POINT_2U = D2D_POINT_2U;
#[cfg(feature = "minwindef")]
pub type D2D1_RECT_F = D2D_RECT_F;
#[cfg(feature = "shared")]
pub type D2D1_RECT_U = D2D_RECT_U;
#[cfg(feature = "minwindef")]
pub type D2D1_SIZE_F = D2D_SIZE_F;
#[cfg(feature = "shared")]
pub type D2D1_SIZE_U = D2D_SIZE_U;
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
#[derive(Clone, Copy, Debug, Default, PartialEq)]
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
#[derive(Clone, Copy, Debug, Default, PartialEq)]
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
#[cfg(all(feature = "shared", feature = "windef"))]
pub type D2D_POINT_2L = super::POINT;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct D2D_POINT_2U {
    pub x: super::UINT32,
    pub y: super::UINT32,
}
#[repr(C)]
#[cfg(feature = "minwindef")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct D2D_RECT_F {
    pub left: super::FLOAT,
    pub top: super::FLOAT,
    pub right: super::FLOAT,
    pub bottom: super::FLOAT,
}
#[cfg(all(feature = "shared", feature = "windef"))]
pub type D2D_RECT_L = super::RECT;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct D2D_RECT_U {
    pub left: super::UINT32,
    pub top: super::UINT32,
    pub right: super::UINT32,
    pub bottom: super::UINT32,
}
#[repr(C)]
#[cfg(feature = "minwindef")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct D2D_SIZE_F {
    pub width: super::FLOAT,
    pub height: super::FLOAT,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct D2D_SIZE_U {
    pub width: super::UINT32,
    pub height: super::UINT32,
}
pub type DWRITE_GLYPH_IMAGE_FORMATS = i32;
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
