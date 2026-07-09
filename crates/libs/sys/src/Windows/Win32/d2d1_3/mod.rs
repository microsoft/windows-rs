#[cfg(feature = "Foundation_Numerics")]
windows_link::link!("d2d1.dll" "system" fn D2D1GetGradientMeshInteriorPointsFromCoonsPatch(ppoint0 : *const super::super::Foundation::Numerics::Vector2, ppoint1 : *const super::super::Foundation::Numerics::Vector2, ppoint2 : *const super::super::Foundation::Numerics::Vector2, ppoint3 : *const super::super::Foundation::Numerics::Vector2, ppoint4 : *const super::super::Foundation::Numerics::Vector2, ppoint5 : *const super::super::Foundation::Numerics::Vector2, ppoint6 : *const super::super::Foundation::Numerics::Vector2, ppoint7 : *const super::super::Foundation::Numerics::Vector2, ppoint8 : *const super::super::Foundation::Numerics::Vector2, ppoint9 : *const super::super::Foundation::Numerics::Vector2, ppoint10 : *const super::super::Foundation::Numerics::Vector2, ppoint11 : *const super::super::Foundation::Numerics::Vector2, ptensorpoint11 : *mut super::super::Foundation::Numerics::Vector2, ptensorpoint12 : *mut super::super::Foundation::Numerics::Vector2, ptensorpoint21 : *mut super::super::Foundation::Numerics::Vector2, ptensorpoint22 : *mut super::super::Foundation::Numerics::Vector2));
pub type D2D1_COLOR_BITMAP_GLYPH_SNAP_OPTION = i32;
pub const D2D1_COLOR_BITMAP_GLYPH_SNAP_OPTION_DEFAULT: D2D1_COLOR_BITMAP_GLYPH_SNAP_OPTION = 0;
pub const D2D1_COLOR_BITMAP_GLYPH_SNAP_OPTION_DISABLE: D2D1_COLOR_BITMAP_GLYPH_SNAP_OPTION = 1;
pub const D2D1_COLOR_BITMAP_GLYPH_SNAP_OPTION_FORCE_DWORD: D2D1_COLOR_BITMAP_GLYPH_SNAP_OPTION = -1;
pub type D2D1_COLOR_CONTEXT_TYPE = i32;
pub const D2D1_COLOR_CONTEXT_TYPE_DXGI: D2D1_COLOR_CONTEXT_TYPE = 2;
pub const D2D1_COLOR_CONTEXT_TYPE_FORCE_DWORD: D2D1_COLOR_CONTEXT_TYPE = -1;
pub const D2D1_COLOR_CONTEXT_TYPE_ICC: D2D1_COLOR_CONTEXT_TYPE = 0;
pub const D2D1_COLOR_CONTEXT_TYPE_SIMPLE: D2D1_COLOR_CONTEXT_TYPE = 1;
pub type D2D1_GAMMA1 = i32;
pub const D2D1_GAMMA1_FORCE_DWORD: D2D1_GAMMA1 = -1;
pub const D2D1_GAMMA1_G10: D2D1_GAMMA1 = 1;
pub const D2D1_GAMMA1_G2084: D2D1_GAMMA1 = 2;
pub const D2D1_GAMMA1_G22: D2D1_GAMMA1 = 0;
#[repr(C)]
#[cfg(all(feature = "Foundation_Numerics", feature = "Win32_d2dbasetypes", feature = "Win32_dxgitype"))]
#[derive(Clone, Copy, Default)]
pub struct D2D1_GRADIENT_MESH_PATCH {
    pub point00: super::super::Foundation::Numerics::Vector2,
    pub point01: super::super::Foundation::Numerics::Vector2,
    pub point02: super::super::Foundation::Numerics::Vector2,
    pub point03: super::super::Foundation::Numerics::Vector2,
    pub point10: super::super::Foundation::Numerics::Vector2,
    pub point11: super::super::Foundation::Numerics::Vector2,
    pub point12: super::super::Foundation::Numerics::Vector2,
    pub point13: super::super::Foundation::Numerics::Vector2,
    pub point20: super::super::Foundation::Numerics::Vector2,
    pub point21: super::super::Foundation::Numerics::Vector2,
    pub point22: super::super::Foundation::Numerics::Vector2,
    pub point23: super::super::Foundation::Numerics::Vector2,
    pub point30: super::super::Foundation::Numerics::Vector2,
    pub point31: super::super::Foundation::Numerics::Vector2,
    pub point32: super::super::Foundation::Numerics::Vector2,
    pub point33: super::super::Foundation::Numerics::Vector2,
    pub color00: super::d2dbasetypes::D2D_COLOR_F,
    pub color03: super::d2dbasetypes::D2D_COLOR_F,
    pub color30: super::d2dbasetypes::D2D_COLOR_F,
    pub color33: super::d2dbasetypes::D2D_COLOR_F,
    pub topEdgeMode: D2D1_PATCH_EDGE_MODE,
    pub leftEdgeMode: D2D1_PATCH_EDGE_MODE,
    pub bottomEdgeMode: D2D1_PATCH_EDGE_MODE,
    pub rightEdgeMode: D2D1_PATCH_EDGE_MODE,
}
pub type D2D1_IMAGE_SOURCE_FROM_DXGI_OPTIONS = u32;
pub const D2D1_IMAGE_SOURCE_FROM_DXGI_OPTIONS_FORCE_DWORD: D2D1_IMAGE_SOURCE_FROM_DXGI_OPTIONS = 4294967295;
pub const D2D1_IMAGE_SOURCE_FROM_DXGI_OPTIONS_LOW_QUALITY_PRIMARY_CONVERSION: D2D1_IMAGE_SOURCE_FROM_DXGI_OPTIONS = 1;
pub const D2D1_IMAGE_SOURCE_FROM_DXGI_OPTIONS_NONE: D2D1_IMAGE_SOURCE_FROM_DXGI_OPTIONS = 0;
pub type D2D1_IMAGE_SOURCE_LOADING_OPTIONS = u32;
pub const D2D1_IMAGE_SOURCE_LOADING_OPTIONS_CACHE_ON_DEMAND: D2D1_IMAGE_SOURCE_LOADING_OPTIONS = 2;
pub const D2D1_IMAGE_SOURCE_LOADING_OPTIONS_FORCE_DWORD: D2D1_IMAGE_SOURCE_LOADING_OPTIONS = 4294967295;
pub const D2D1_IMAGE_SOURCE_LOADING_OPTIONS_NONE: D2D1_IMAGE_SOURCE_LOADING_OPTIONS = 0;
pub const D2D1_IMAGE_SOURCE_LOADING_OPTIONS_RELEASE_SOURCE: D2D1_IMAGE_SOURCE_LOADING_OPTIONS = 1;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct D2D1_INK_BEZIER_SEGMENT {
    pub point1: D2D1_INK_POINT,
    pub point2: D2D1_INK_POINT,
    pub point3: D2D1_INK_POINT,
}
pub type D2D1_INK_NIB_SHAPE = i32;
pub const D2D1_INK_NIB_SHAPE_FORCE_DWORD: D2D1_INK_NIB_SHAPE = -1;
pub const D2D1_INK_NIB_SHAPE_ROUND: D2D1_INK_NIB_SHAPE = 0;
pub const D2D1_INK_NIB_SHAPE_SQUARE: D2D1_INK_NIB_SHAPE = 1;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct D2D1_INK_POINT {
    pub x: f32,
    pub y: f32,
    pub radius: f32,
}
#[repr(C)]
#[cfg(feature = "Foundation_Numerics")]
#[derive(Clone, Copy, Default)]
pub struct D2D1_INK_STYLE_PROPERTIES {
    pub nibShape: D2D1_INK_NIB_SHAPE,
    pub nibTransform: super::super::Foundation::Numerics::Matrix3x2,
}
pub type D2D1_ORIENTATION = i32;
pub const D2D1_ORIENTATION_DEFAULT: D2D1_ORIENTATION = 1;
pub const D2D1_ORIENTATION_FLIP_HORIZONTAL: D2D1_ORIENTATION = 2;
pub const D2D1_ORIENTATION_FORCE_DWORD: D2D1_ORIENTATION = -1;
pub const D2D1_ORIENTATION_ROTATE_CLOCKWISE180: D2D1_ORIENTATION = 3;
pub const D2D1_ORIENTATION_ROTATE_CLOCKWISE180_FLIP_HORIZONTAL: D2D1_ORIENTATION = 4;
pub const D2D1_ORIENTATION_ROTATE_CLOCKWISE270: D2D1_ORIENTATION = 6;
pub const D2D1_ORIENTATION_ROTATE_CLOCKWISE270_FLIP_HORIZONTAL: D2D1_ORIENTATION = 7;
pub const D2D1_ORIENTATION_ROTATE_CLOCKWISE90: D2D1_ORIENTATION = 8;
pub const D2D1_ORIENTATION_ROTATE_CLOCKWISE90_FLIP_HORIZONTAL: D2D1_ORIENTATION = 5;
pub type D2D1_PATCH_EDGE_MODE = i32;
pub const D2D1_PATCH_EDGE_MODE_ALIASED: D2D1_PATCH_EDGE_MODE = 0;
pub const D2D1_PATCH_EDGE_MODE_ALIASED_INFLATED: D2D1_PATCH_EDGE_MODE = 2;
pub const D2D1_PATCH_EDGE_MODE_ANTIALIASED: D2D1_PATCH_EDGE_MODE = 1;
pub const D2D1_PATCH_EDGE_MODE_FORCE_DWORD: D2D1_PATCH_EDGE_MODE = -1;
#[repr(C)]
#[cfg(feature = "Foundation_Numerics")]
#[derive(Clone, Copy, Default)]
pub struct D2D1_SIMPLE_COLOR_PROFILE {
    pub redPrimary: super::super::Foundation::Numerics::Vector2,
    pub greenPrimary: super::super::Foundation::Numerics::Vector2,
    pub bluePrimary: super::super::Foundation::Numerics::Vector2,
    pub whitePointXZ: super::super::Foundation::Numerics::Vector2,
    pub gamma: D2D1_GAMMA1,
}
pub type D2D1_SPRITE_OPTIONS = u32;
pub const D2D1_SPRITE_OPTIONS_CLAMP_TO_SOURCE_RECTANGLE: D2D1_SPRITE_OPTIONS = 1;
pub const D2D1_SPRITE_OPTIONS_FORCE_DWORD: D2D1_SPRITE_OPTIONS = 4294967295;
pub const D2D1_SPRITE_OPTIONS_NONE: D2D1_SPRITE_OPTIONS = 0;
pub type D2D1_TRANSFORMED_IMAGE_SOURCE_OPTIONS = u32;
pub const D2D1_TRANSFORMED_IMAGE_SOURCE_OPTIONS_DISABLE_DPI_SCALE: D2D1_TRANSFORMED_IMAGE_SOURCE_OPTIONS = 1;
pub const D2D1_TRANSFORMED_IMAGE_SOURCE_OPTIONS_FORCE_DWORD: D2D1_TRANSFORMED_IMAGE_SOURCE_OPTIONS = 4294967295;
pub const D2D1_TRANSFORMED_IMAGE_SOURCE_OPTIONS_NONE: D2D1_TRANSFORMED_IMAGE_SOURCE_OPTIONS = 0;
#[repr(C)]
#[cfg(feature = "Win32_d2d1_1")]
#[derive(Clone, Copy, Default)]
pub struct D2D1_TRANSFORMED_IMAGE_SOURCE_PROPERTIES {
    pub orientation: D2D1_ORIENTATION,
    pub scaleX: f32,
    pub scaleY: f32,
    pub interpolationMode: super::d2d1_1::D2D1_INTERPOLATION_MODE,
    pub options: D2D1_TRANSFORMED_IMAGE_SOURCE_OPTIONS,
}
