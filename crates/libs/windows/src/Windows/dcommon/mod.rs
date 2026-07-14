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
    pub format: super::dxgi::DXGI_FORMAT,
    pub alphaMode: D2D1_ALPHA_MODE,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D2D_MATRIX_4X3_F {
    pub Anonymous: D2D_MATRIX_4X3_F_0,
}
impl Default for D2D_MATRIX_4X3_F {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union D2D_MATRIX_4X3_F_0 {
    pub Anonymous: D2D_MATRIX_4X3_F_0_0,
    pub m: [[f32; 3]; 4],
}
impl Default for D2D_MATRIX_4X3_F_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
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
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D2D_MATRIX_5X4_F {
    pub Anonymous: D2D_MATRIX_5X4_F_0,
}
impl Default for D2D_MATRIX_5X4_F {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union D2D_MATRIX_5X4_F_0 {
    pub Anonymous: D2D_MATRIX_5X4_F_0_0,
    pub m: [[f32; 4]; 5],
}
impl Default for D2D_MATRIX_5X4_F_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
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
#[cfg(feature = "windef")]
pub type D2D_POINT_2L = super::windef::POINT;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct D2D_POINT_2U {
    pub x: u32,
    pub y: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct D2D_RECT_F {
    pub left: f32,
    pub top: f32,
    pub right: f32,
    pub bottom: f32,
}
#[cfg(feature = "windef")]
pub type D2D_RECT_L = super::windef::RECT;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct D2D_RECT_U {
    pub left: u32,
    pub top: u32,
    pub right: u32,
    pub bottom: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct D2D_SIZE_F {
    pub width: f32,
    pub height: f32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct D2D_SIZE_U {
    pub width: u32,
    pub height: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct D2D_VECTOR_3F {
    pub x: f32,
    pub y: f32,
    pub z: f32,
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
