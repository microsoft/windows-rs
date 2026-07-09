pub type DWRITE_AUTOMATIC_FONT_AXES = u32;
pub const DWRITE_AUTOMATIC_FONT_AXES_NONE: DWRITE_AUTOMATIC_FONT_AXES = 0;
pub const DWRITE_AUTOMATIC_FONT_AXES_OPTICAL_SIZE: DWRITE_AUTOMATIC_FONT_AXES = 1;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct DWRITE_BITMAP_DATA_BGRA32 {
    pub width: u32,
    pub height: u32,
    pub pixels: *mut u32,
}
impl Default for DWRITE_BITMAP_DATA_BGRA32 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const DWRITE_COLOR_COMPOSITE_CLEAR: DWRITE_COLOR_COMPOSITE_MODE = 0;
pub const DWRITE_COLOR_COMPOSITE_COLOR_BURN: DWRITE_COLOR_COMPOSITE_MODE = 18;
pub const DWRITE_COLOR_COMPOSITE_COLOR_DODGE: DWRITE_COLOR_COMPOSITE_MODE = 17;
pub const DWRITE_COLOR_COMPOSITE_DARKEN: DWRITE_COLOR_COMPOSITE_MODE = 15;
pub const DWRITE_COLOR_COMPOSITE_DEST: DWRITE_COLOR_COMPOSITE_MODE = 2;
pub const DWRITE_COLOR_COMPOSITE_DEST_ATOP: DWRITE_COLOR_COMPOSITE_MODE = 10;
pub const DWRITE_COLOR_COMPOSITE_DEST_IN: DWRITE_COLOR_COMPOSITE_MODE = 6;
pub const DWRITE_COLOR_COMPOSITE_DEST_OUT: DWRITE_COLOR_COMPOSITE_MODE = 8;
pub const DWRITE_COLOR_COMPOSITE_DEST_OVER: DWRITE_COLOR_COMPOSITE_MODE = 4;
pub const DWRITE_COLOR_COMPOSITE_DIFFERENCE: DWRITE_COLOR_COMPOSITE_MODE = 21;
pub const DWRITE_COLOR_COMPOSITE_EXCLUSION: DWRITE_COLOR_COMPOSITE_MODE = 22;
pub const DWRITE_COLOR_COMPOSITE_HARD_LIGHT: DWRITE_COLOR_COMPOSITE_MODE = 19;
pub const DWRITE_COLOR_COMPOSITE_HSL_COLOR: DWRITE_COLOR_COMPOSITE_MODE = 26;
pub const DWRITE_COLOR_COMPOSITE_HSL_HUE: DWRITE_COLOR_COMPOSITE_MODE = 24;
pub const DWRITE_COLOR_COMPOSITE_HSL_LUMINOSITY: DWRITE_COLOR_COMPOSITE_MODE = 27;
pub const DWRITE_COLOR_COMPOSITE_HSL_SATURATION: DWRITE_COLOR_COMPOSITE_MODE = 25;
pub const DWRITE_COLOR_COMPOSITE_LIGHTEN: DWRITE_COLOR_COMPOSITE_MODE = 16;
pub type DWRITE_COLOR_COMPOSITE_MODE = i32;
pub const DWRITE_COLOR_COMPOSITE_MULTIPLY: DWRITE_COLOR_COMPOSITE_MODE = 23;
pub const DWRITE_COLOR_COMPOSITE_OVERLAY: DWRITE_COLOR_COMPOSITE_MODE = 14;
pub const DWRITE_COLOR_COMPOSITE_PLUS: DWRITE_COLOR_COMPOSITE_MODE = 12;
pub const DWRITE_COLOR_COMPOSITE_SCREEN: DWRITE_COLOR_COMPOSITE_MODE = 13;
pub const DWRITE_COLOR_COMPOSITE_SOFT_LIGHT: DWRITE_COLOR_COMPOSITE_MODE = 20;
pub const DWRITE_COLOR_COMPOSITE_SRC: DWRITE_COLOR_COMPOSITE_MODE = 1;
pub const DWRITE_COLOR_COMPOSITE_SRC_ATOP: DWRITE_COLOR_COMPOSITE_MODE = 9;
pub const DWRITE_COLOR_COMPOSITE_SRC_IN: DWRITE_COLOR_COMPOSITE_MODE = 5;
pub const DWRITE_COLOR_COMPOSITE_SRC_OUT: DWRITE_COLOR_COMPOSITE_MODE = 7;
pub const DWRITE_COLOR_COMPOSITE_SRC_OVER: DWRITE_COLOR_COMPOSITE_MODE = 3;
pub const DWRITE_COLOR_COMPOSITE_XOR: DWRITE_COLOR_COMPOSITE_MODE = 11;
#[repr(C)]
#[cfg(all(feature = "Win32_dcommon", feature = "Win32_dwrite", feature = "Win32_dwrite_2", feature = "Win32_dxgitype"))]
#[derive(Clone, Copy, Default)]
pub struct DWRITE_COLOR_GLYPH_RUN1 {
    pub Base: super::dwrite_2::DWRITE_COLOR_GLYPH_RUN,
    pub glyphImageFormat: super::dcommon::DWRITE_GLYPH_IMAGE_FORMATS,
    pub measuringMode: super::dcommon::DWRITE_MEASURING_MODE,
}
pub type DWRITE_CONTAINER_TYPE = i32;
pub const DWRITE_CONTAINER_TYPE_UNKNOWN: DWRITE_CONTAINER_TYPE = 0;
pub const DWRITE_CONTAINER_TYPE_WOFF: DWRITE_CONTAINER_TYPE = 1;
pub const DWRITE_CONTAINER_TYPE_WOFF2: DWRITE_CONTAINER_TYPE = 2;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct DWRITE_FILE_FRAGMENT {
    pub fileOffset: u64,
    pub fragmentSize: u64,
}
pub type DWRITE_FONT_AXIS_ATTRIBUTES = u32;
pub const DWRITE_FONT_AXIS_ATTRIBUTES_HIDDEN: DWRITE_FONT_AXIS_ATTRIBUTES = 2;
pub const DWRITE_FONT_AXIS_ATTRIBUTES_NONE: DWRITE_FONT_AXIS_ATTRIBUTES = 0;
pub const DWRITE_FONT_AXIS_ATTRIBUTES_VARIABLE: DWRITE_FONT_AXIS_ATTRIBUTES = 1;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct DWRITE_FONT_AXIS_RANGE {
    pub axisTag: DWRITE_FONT_AXIS_TAG,
    pub minValue: f32,
    pub maxValue: f32,
}
pub type DWRITE_FONT_AXIS_TAG = i32;
pub const DWRITE_FONT_AXIS_TAG_ITALIC: DWRITE_FONT_AXIS_TAG = 1818326121;
pub const DWRITE_FONT_AXIS_TAG_OPTICAL_SIZE: DWRITE_FONT_AXIS_TAG = 2054385775;
pub const DWRITE_FONT_AXIS_TAG_SLANT: DWRITE_FONT_AXIS_TAG = 1953393779;
pub const DWRITE_FONT_AXIS_TAG_WEIGHT: DWRITE_FONT_AXIS_TAG = 1952999287;
pub const DWRITE_FONT_AXIS_TAG_WIDTH: DWRITE_FONT_AXIS_TAG = 1752458359;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct DWRITE_FONT_AXIS_VALUE {
    pub axisTag: DWRITE_FONT_AXIS_TAG,
    pub value: f32,
}
pub type DWRITE_FONT_FAMILY_MODEL = i32;
pub const DWRITE_FONT_FAMILY_MODEL_TYPOGRAPHIC: DWRITE_FONT_FAMILY_MODEL = 0;
pub const DWRITE_FONT_FAMILY_MODEL_WEIGHT_STRETCH_STYLE: DWRITE_FONT_FAMILY_MODEL = 1;
pub type DWRITE_FONT_LINE_GAP_USAGE = i32;
pub const DWRITE_FONT_LINE_GAP_USAGE_DEFAULT: DWRITE_FONT_LINE_GAP_USAGE = 0;
pub const DWRITE_FONT_LINE_GAP_USAGE_DISABLED: DWRITE_FONT_LINE_GAP_USAGE = 1;
pub const DWRITE_FONT_LINE_GAP_USAGE_ENABLED: DWRITE_FONT_LINE_GAP_USAGE = 2;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct DWRITE_FONT_PROPERTY {
    pub propertyId: DWRITE_FONT_PROPERTY_ID,
    pub propertyValue: *const u16,
    pub localeName: *const u16,
}
impl Default for DWRITE_FONT_PROPERTY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type DWRITE_FONT_PROPERTY_ID = i32;
pub const DWRITE_FONT_PROPERTY_ID_DESIGN_SCRIPT_LANGUAGE_TAG: DWRITE_FONT_PROPERTY_ID = 7;
pub const DWRITE_FONT_PROPERTY_ID_FACE_NAME: DWRITE_FONT_PROPERTY_ID = 3;
pub const DWRITE_FONT_PROPERTY_ID_FAMILY_NAME: DWRITE_FONT_PROPERTY_ID = 1;
pub const DWRITE_FONT_PROPERTY_ID_FULL_NAME: DWRITE_FONT_PROPERTY_ID = 4;
pub const DWRITE_FONT_PROPERTY_ID_NONE: DWRITE_FONT_PROPERTY_ID = 0;
pub const DWRITE_FONT_PROPERTY_ID_POSTSCRIPT_NAME: DWRITE_FONT_PROPERTY_ID = 6;
pub const DWRITE_FONT_PROPERTY_ID_PREFERRED_FAMILY_NAME: DWRITE_FONT_PROPERTY_ID = 2;
pub const DWRITE_FONT_PROPERTY_ID_SEMANTIC_TAG: DWRITE_FONT_PROPERTY_ID = 9;
pub const DWRITE_FONT_PROPERTY_ID_STRETCH: DWRITE_FONT_PROPERTY_ID = 11;
pub const DWRITE_FONT_PROPERTY_ID_STYLE: DWRITE_FONT_PROPERTY_ID = 12;
pub const DWRITE_FONT_PROPERTY_ID_SUPPORTED_SCRIPT_LANGUAGE_TAG: DWRITE_FONT_PROPERTY_ID = 8;
pub const DWRITE_FONT_PROPERTY_ID_TOTAL: DWRITE_FONT_PROPERTY_ID = 13;
pub const DWRITE_FONT_PROPERTY_ID_TOTAL_RS3: DWRITE_FONT_PROPERTY_ID = 14;
pub const DWRITE_FONT_PROPERTY_ID_TYPOGRAPHIC_FACE_NAME: DWRITE_FONT_PROPERTY_ID = 13;
pub const DWRITE_FONT_PROPERTY_ID_TYPOGRAPHIC_FAMILY_NAME: DWRITE_FONT_PROPERTY_ID = 2;
pub const DWRITE_FONT_PROPERTY_ID_WEIGHT: DWRITE_FONT_PROPERTY_ID = 10;
pub const DWRITE_FONT_PROPERTY_ID_WEIGHT_STRETCH_STYLE_FACE_NAME: DWRITE_FONT_PROPERTY_ID = 3;
pub const DWRITE_FONT_PROPERTY_ID_WEIGHT_STRETCH_STYLE_FAMILY_NAME: DWRITE_FONT_PROPERTY_ID = 1;
pub const DWRITE_FONT_PROPERTY_ID_WIN32_FAMILY_NAME: DWRITE_FONT_PROPERTY_ID = 5;
pub type DWRITE_FONT_SOURCE_TYPE = i32;
pub const DWRITE_FONT_SOURCE_TYPE_APPX_PACKAGE: DWRITE_FONT_SOURCE_TYPE = 3;
pub const DWRITE_FONT_SOURCE_TYPE_PER_MACHINE: DWRITE_FONT_SOURCE_TYPE = 1;
pub const DWRITE_FONT_SOURCE_TYPE_PER_USER: DWRITE_FONT_SOURCE_TYPE = 2;
pub const DWRITE_FONT_SOURCE_TYPE_REMOTE_FONT_PROVIDER: DWRITE_FONT_SOURCE_TYPE = 4;
pub const DWRITE_FONT_SOURCE_TYPE_UNKNOWN: DWRITE_FONT_SOURCE_TYPE = 0;
#[repr(C)]
#[cfg(all(feature = "Win32_dcommon", feature = "Win32_windef"))]
#[derive(Clone, Copy)]
pub struct DWRITE_GLYPH_IMAGE_DATA {
    pub imageData: *const core::ffi::c_void,
    pub imageDataSize: u32,
    pub uniqueDataId: u32,
    pub pixelsPerEm: u32,
    pub pixelSize: super::dcommon::D2D_SIZE_U,
    pub horizontalLeftOrigin: super::dcommon::D2D_POINT_2L,
    pub horizontalRightOrigin: super::dcommon::D2D_POINT_2L,
    pub verticalTopOrigin: super::dcommon::D2D_POINT_2L,
    pub verticalBottomOrigin: super::dcommon::D2D_POINT_2L,
}
#[cfg(all(feature = "Win32_dcommon", feature = "Win32_windef"))]
impl Default for DWRITE_GLYPH_IMAGE_DATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_dwrite")]
#[derive(Clone, Copy, Default)]
pub struct DWRITE_LINE_METRICS1 {
    pub Base: super::dwrite::DWRITE_LINE_METRICS,
    pub leadingBefore: f32,
    pub leadingAfter: f32,
}
#[repr(C)]
#[cfg(feature = "Win32_dwrite")]
#[derive(Clone, Copy, Default)]
pub struct DWRITE_LINE_SPACING {
    pub method: super::dwrite::DWRITE_LINE_SPACING_METHOD,
    pub height: f32,
    pub baseline: f32,
    pub leadingBefore: f32,
    pub fontLineGapUsage: DWRITE_FONT_LINE_GAP_USAGE,
}
pub type DWRITE_LOCALITY = i32;
pub const DWRITE_LOCALITY_LOCAL: DWRITE_LOCALITY = 2;
pub const DWRITE_LOCALITY_PARTIAL: DWRITE_LOCALITY = 1;
pub const DWRITE_LOCALITY_REMOTE: DWRITE_LOCALITY = 0;
pub type DWRITE_PAINT_ATTRIBUTES = u32;
pub const DWRITE_PAINT_ATTRIBUTES_NONE: DWRITE_PAINT_ATTRIBUTES = 0;
pub const DWRITE_PAINT_ATTRIBUTES_USES_PALETTE: DWRITE_PAINT_ATTRIBUTES = 1;
pub const DWRITE_PAINT_ATTRIBUTES_USES_TEXT_COLOR: DWRITE_PAINT_ATTRIBUTES = 2;
#[repr(C)]
#[cfg(all(feature = "Win32_dwrite_2", feature = "Win32_dxgitype"))]
#[derive(Clone, Copy, Default)]
pub struct DWRITE_PAINT_COLOR {
    pub value: super::dwrite_2::DWRITE_COLOR_F,
    pub paletteEntryIndex: u16,
    pub alphaMultiplier: f32,
    pub colorAttributes: DWRITE_PAINT_ATTRIBUTES,
}
#[repr(C)]
#[cfg(all(feature = "Win32_dcommon", feature = "Win32_dwrite", feature = "Win32_dwrite_2", feature = "Win32_dxgitype"))]
#[derive(Clone, Copy)]
pub struct DWRITE_PAINT_ELEMENT {
    pub paintType: DWRITE_PAINT_TYPE,
    pub paint: DWRITE_PAINT_ELEMENT_0,
}
#[cfg(all(feature = "Win32_dcommon", feature = "Win32_dwrite", feature = "Win32_dwrite_2", feature = "Win32_dxgitype"))]
impl Default for DWRITE_PAINT_ELEMENT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_dcommon", feature = "Win32_dwrite", feature = "Win32_dwrite_2", feature = "Win32_dxgitype"))]
#[derive(Clone, Copy)]
pub union DWRITE_PAINT_ELEMENT_0 {
    pub layers: DWRITE_PAINT_ELEMENT_0_0,
    pub solidGlyph: DWRITE_PAINT_ELEMENT_0_1,
    pub solid: DWRITE_PAINT_COLOR,
    pub linearGradient: DWRITE_PAINT_ELEMENT_0_2,
    pub radialGradient: DWRITE_PAINT_ELEMENT_0_3,
    pub sweepGradient: DWRITE_PAINT_ELEMENT_0_4,
    pub glyph: DWRITE_PAINT_ELEMENT_0_5,
    pub colorGlyph: DWRITE_PAINT_ELEMENT_0_6,
    pub transform: super::dwrite::DWRITE_MATRIX,
    pub composite: DWRITE_PAINT_ELEMENT_0_7,
}
#[cfg(all(feature = "Win32_dcommon", feature = "Win32_dwrite", feature = "Win32_dwrite_2", feature = "Win32_dxgitype"))]
impl Default for DWRITE_PAINT_ELEMENT_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct DWRITE_PAINT_ELEMENT_0_0 {
    pub childCount: u32,
}
#[repr(C)]
#[cfg(all(feature = "Win32_dwrite_2", feature = "Win32_dxgitype"))]
#[derive(Clone, Copy, Default)]
pub struct DWRITE_PAINT_ELEMENT_0_1 {
    pub glyphIndex: u32,
    pub color: DWRITE_PAINT_COLOR,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct DWRITE_PAINT_ELEMENT_0_2 {
    pub extendMode: u32,
    pub gradientStopCount: u32,
    pub x0: f32,
    pub y0: f32,
    pub x1: f32,
    pub y1: f32,
    pub x2: f32,
    pub y2: f32,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct DWRITE_PAINT_ELEMENT_0_3 {
    pub extendMode: u32,
    pub gradientStopCount: u32,
    pub x0: f32,
    pub y0: f32,
    pub radius0: f32,
    pub x1: f32,
    pub y1: f32,
    pub radius1: f32,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct DWRITE_PAINT_ELEMENT_0_4 {
    pub extendMode: u32,
    pub gradientStopCount: u32,
    pub centerX: f32,
    pub centerY: f32,
    pub startAngle: f32,
    pub endAngle: f32,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct DWRITE_PAINT_ELEMENT_0_5 {
    pub glyphIndex: u32,
}
#[repr(C)]
#[cfg(feature = "Win32_dcommon")]
#[derive(Clone, Copy, Default)]
pub struct DWRITE_PAINT_ELEMENT_0_6 {
    pub glyphIndex: u32,
    pub clipBox: super::dcommon::D2D_RECT_F,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct DWRITE_PAINT_ELEMENT_0_7 {
    pub mode: DWRITE_COLOR_COMPOSITE_MODE,
}
pub type DWRITE_PAINT_FEATURE_LEVEL = i32;
pub const DWRITE_PAINT_FEATURE_LEVEL_COLR_V0: DWRITE_PAINT_FEATURE_LEVEL = 1;
pub const DWRITE_PAINT_FEATURE_LEVEL_COLR_V1: DWRITE_PAINT_FEATURE_LEVEL = 2;
pub const DWRITE_PAINT_FEATURE_LEVEL_NONE: DWRITE_PAINT_FEATURE_LEVEL = 0;
pub type DWRITE_PAINT_TYPE = i32;
pub const DWRITE_PAINT_TYPE_COLOR_GLYPH: DWRITE_PAINT_TYPE = 8;
pub const DWRITE_PAINT_TYPE_COMPOSITE: DWRITE_PAINT_TYPE = 10;
pub const DWRITE_PAINT_TYPE_GLYPH: DWRITE_PAINT_TYPE = 7;
pub const DWRITE_PAINT_TYPE_LAYERS: DWRITE_PAINT_TYPE = 1;
pub const DWRITE_PAINT_TYPE_LINEAR_GRADIENT: DWRITE_PAINT_TYPE = 4;
pub const DWRITE_PAINT_TYPE_NONE: DWRITE_PAINT_TYPE = 0;
pub const DWRITE_PAINT_TYPE_RADIAL_GRADIENT: DWRITE_PAINT_TYPE = 5;
pub const DWRITE_PAINT_TYPE_SOLID: DWRITE_PAINT_TYPE = 3;
pub const DWRITE_PAINT_TYPE_SOLID_GLYPH: DWRITE_PAINT_TYPE = 2;
pub const DWRITE_PAINT_TYPE_SWEEP_GRADIENT: DWRITE_PAINT_TYPE = 6;
pub const DWRITE_PAINT_TYPE_TRANSFORM: DWRITE_PAINT_TYPE = 9;
pub type DWRITE_RENDERING_MODE1 = i32;
pub const DWRITE_RENDERING_MODE1_ALIASED: DWRITE_RENDERING_MODE1 = 1;
pub const DWRITE_RENDERING_MODE1_DEFAULT: DWRITE_RENDERING_MODE1 = 0;
pub const DWRITE_RENDERING_MODE1_GDI_CLASSIC: DWRITE_RENDERING_MODE1 = 2;
pub const DWRITE_RENDERING_MODE1_GDI_NATURAL: DWRITE_RENDERING_MODE1 = 3;
pub const DWRITE_RENDERING_MODE1_NATURAL: DWRITE_RENDERING_MODE1 = 4;
pub const DWRITE_RENDERING_MODE1_NATURAL_SYMMETRIC: DWRITE_RENDERING_MODE1 = 5;
pub const DWRITE_RENDERING_MODE1_NATURAL_SYMMETRIC_DOWNSAMPLED: DWRITE_RENDERING_MODE1 = 7;
pub const DWRITE_RENDERING_MODE1_OUTLINE: DWRITE_RENDERING_MODE1 = 6;
pub const DWRITE_STANDARD_FONT_AXIS_COUNT: u32 = 5;
