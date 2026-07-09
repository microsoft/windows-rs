windows_link::link!("d2d1.dll" "system" fn D2D1CreateFactory(factorytype : D2D1_FACTORY_TYPE, riid : *const windows_sys::core::GUID, pfactoryoptions : *const D2D1_FACTORY_OPTIONS, ppifactory : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
#[cfg(feature = "Foundation_Numerics")]
windows_link::link!("d2d1.dll" "system" fn D2D1InvertMatrix(matrix : *mut super::super::Foundation::Numerics::Matrix3x2) -> windows_sys::core::BOOL);
#[cfg(feature = "Foundation_Numerics")]
windows_link::link!("d2d1.dll" "system" fn D2D1IsMatrixInvertible(matrix : *const super::super::Foundation::Numerics::Matrix3x2) -> windows_sys::core::BOOL);
#[cfg(feature = "Foundation_Numerics")]
windows_link::link!("d2d1.dll" "system" fn D2D1MakeRotateMatrix(angle : f32, center : super::super::Foundation::Numerics::Vector2, matrix : *mut super::super::Foundation::Numerics::Matrix3x2));
#[cfg(feature = "Foundation_Numerics")]
windows_link::link!("d2d1.dll" "system" fn D2D1MakeSkewMatrix(anglex : f32, angley : f32, center : super::super::Foundation::Numerics::Vector2, matrix : *mut super::super::Foundation::Numerics::Matrix3x2));
pub type D2D1_ANTIALIAS_MODE = i32;
pub const D2D1_ANTIALIAS_MODE_ALIASED: D2D1_ANTIALIAS_MODE = 1;
pub const D2D1_ANTIALIAS_MODE_FORCE_DWORD: D2D1_ANTIALIAS_MODE = -1;
pub const D2D1_ANTIALIAS_MODE_PER_PRIMITIVE: D2D1_ANTIALIAS_MODE = 0;
#[repr(C)]
#[cfg(all(feature = "Foundation_Numerics", feature = "Win32_dcommon"))]
#[derive(Clone, Copy, Default)]
pub struct D2D1_ARC_SEGMENT {
    pub point: super::super::Foundation::Numerics::Vector2,
    pub size: super::dcommon::D2D_SIZE_F,
    pub rotationAngle: f32,
    pub sweepDirection: D2D1_SWEEP_DIRECTION,
    pub arcSize: D2D1_ARC_SIZE,
}
pub type D2D1_ARC_SIZE = i32;
pub const D2D1_ARC_SIZE_FORCE_DWORD: D2D1_ARC_SIZE = -1;
pub const D2D1_ARC_SIZE_LARGE: D2D1_ARC_SIZE = 1;
pub const D2D1_ARC_SIZE_SMALL: D2D1_ARC_SIZE = 0;
#[repr(C)]
#[cfg(feature = "Foundation_Numerics")]
#[derive(Clone, Copy, Default)]
pub struct D2D1_BEZIER_SEGMENT {
    pub point1: super::super::Foundation::Numerics::Vector2,
    pub point2: super::super::Foundation::Numerics::Vector2,
    pub point3: super::super::Foundation::Numerics::Vector2,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct D2D1_BITMAP_BRUSH_PROPERTIES {
    pub extendModeX: D2D1_EXTEND_MODE,
    pub extendModeY: D2D1_EXTEND_MODE,
    pub interpolationMode: D2D1_BITMAP_INTERPOLATION_MODE,
}
pub type D2D1_BITMAP_INTERPOLATION_MODE = i32;
pub const D2D1_BITMAP_INTERPOLATION_MODE_FORCE_DWORD: D2D1_BITMAP_INTERPOLATION_MODE = -1;
pub const D2D1_BITMAP_INTERPOLATION_MODE_LINEAR: D2D1_BITMAP_INTERPOLATION_MODE = 1;
pub const D2D1_BITMAP_INTERPOLATION_MODE_NEAREST_NEIGHBOR: D2D1_BITMAP_INTERPOLATION_MODE = 0;
#[repr(C)]
#[cfg(all(feature = "Win32_dcommon", feature = "Win32_dxgiformat"))]
#[derive(Clone, Copy, Default)]
pub struct D2D1_BITMAP_PROPERTIES {
    pub pixelFormat: super::dcommon::D2D1_PIXEL_FORMAT,
    pub dpiX: f32,
    pub dpiY: f32,
}
#[repr(C)]
#[cfg(feature = "Foundation_Numerics")]
#[derive(Clone, Copy, Default)]
pub struct D2D1_BRUSH_PROPERTIES {
    pub opacity: f32,
    pub transform: super::super::Foundation::Numerics::Matrix3x2,
}
pub type D2D1_CAP_STYLE = i32;
pub const D2D1_CAP_STYLE_FLAT: D2D1_CAP_STYLE = 0;
pub const D2D1_CAP_STYLE_FORCE_DWORD: D2D1_CAP_STYLE = -1;
pub const D2D1_CAP_STYLE_ROUND: D2D1_CAP_STYLE = 2;
pub const D2D1_CAP_STYLE_SQUARE: D2D1_CAP_STYLE = 1;
pub const D2D1_CAP_STYLE_TRIANGLE: D2D1_CAP_STYLE = 3;
pub type D2D1_COMBINE_MODE = i32;
pub const D2D1_COMBINE_MODE_EXCLUDE: D2D1_COMBINE_MODE = 3;
pub const D2D1_COMBINE_MODE_FORCE_DWORD: D2D1_COMBINE_MODE = -1;
pub const D2D1_COMBINE_MODE_INTERSECT: D2D1_COMBINE_MODE = 1;
pub const D2D1_COMBINE_MODE_UNION: D2D1_COMBINE_MODE = 0;
pub const D2D1_COMBINE_MODE_XOR: D2D1_COMBINE_MODE = 2;
pub type D2D1_COMPATIBLE_RENDER_TARGET_OPTIONS = u32;
pub const D2D1_COMPATIBLE_RENDER_TARGET_OPTIONS_FORCE_DWORD: D2D1_COMPATIBLE_RENDER_TARGET_OPTIONS = 4294967295;
pub const D2D1_COMPATIBLE_RENDER_TARGET_OPTIONS_GDI_COMPATIBLE: D2D1_COMPATIBLE_RENDER_TARGET_OPTIONS = 1;
pub const D2D1_COMPATIBLE_RENDER_TARGET_OPTIONS_NONE: D2D1_COMPATIBLE_RENDER_TARGET_OPTIONS = 0;
pub type D2D1_DASH_STYLE = i32;
pub const D2D1_DASH_STYLE_CUSTOM: D2D1_DASH_STYLE = 5;
pub const D2D1_DASH_STYLE_DASH: D2D1_DASH_STYLE = 1;
pub const D2D1_DASH_STYLE_DASH_DOT: D2D1_DASH_STYLE = 3;
pub const D2D1_DASH_STYLE_DASH_DOT_DOT: D2D1_DASH_STYLE = 4;
pub const D2D1_DASH_STYLE_DOT: D2D1_DASH_STYLE = 2;
pub const D2D1_DASH_STYLE_FORCE_DWORD: D2D1_DASH_STYLE = -1;
pub const D2D1_DASH_STYLE_SOLID: D2D1_DASH_STYLE = 0;
pub type D2D1_DC_INITIALIZE_MODE = i32;
pub const D2D1_DC_INITIALIZE_MODE_CLEAR: D2D1_DC_INITIALIZE_MODE = 1;
pub const D2D1_DC_INITIALIZE_MODE_COPY: D2D1_DC_INITIALIZE_MODE = 0;
pub const D2D1_DC_INITIALIZE_MODE_FORCE_DWORD: D2D1_DC_INITIALIZE_MODE = -1;
pub type D2D1_DEBUG_LEVEL = i32;
pub const D2D1_DEBUG_LEVEL_ERROR: D2D1_DEBUG_LEVEL = 1;
pub const D2D1_DEBUG_LEVEL_FORCE_DWORD: D2D1_DEBUG_LEVEL = -1;
pub const D2D1_DEBUG_LEVEL_INFORMATION: D2D1_DEBUG_LEVEL = 3;
pub const D2D1_DEBUG_LEVEL_NONE: D2D1_DEBUG_LEVEL = 0;
pub const D2D1_DEBUG_LEVEL_WARNING: D2D1_DEBUG_LEVEL = 2;
#[repr(C)]
#[cfg(feature = "Foundation_Numerics")]
#[derive(Clone, Copy, Default)]
pub struct D2D1_DRAWING_STATE_DESCRIPTION {
    pub antialiasMode: D2D1_ANTIALIAS_MODE,
    pub textAntialiasMode: D2D1_TEXT_ANTIALIAS_MODE,
    pub tag1: D2D1_TAG,
    pub tag2: D2D1_TAG,
    pub transform: super::super::Foundation::Numerics::Matrix3x2,
}
pub type D2D1_DRAW_TEXT_OPTIONS = u32;
pub const D2D1_DRAW_TEXT_OPTIONS_CLIP: D2D1_DRAW_TEXT_OPTIONS = 2;
pub const D2D1_DRAW_TEXT_OPTIONS_DISABLE_COLOR_BITMAP_SNAPPING: D2D1_DRAW_TEXT_OPTIONS = 8;
pub const D2D1_DRAW_TEXT_OPTIONS_ENABLE_COLOR_FONT: D2D1_DRAW_TEXT_OPTIONS = 4;
pub const D2D1_DRAW_TEXT_OPTIONS_FORCE_DWORD: D2D1_DRAW_TEXT_OPTIONS = 4294967295;
pub const D2D1_DRAW_TEXT_OPTIONS_NONE: D2D1_DRAW_TEXT_OPTIONS = 0;
pub const D2D1_DRAW_TEXT_OPTIONS_NO_SNAP: D2D1_DRAW_TEXT_OPTIONS = 1;
#[repr(C)]
#[cfg(feature = "Foundation_Numerics")]
#[derive(Clone, Copy, Default)]
pub struct D2D1_ELLIPSE {
    pub point: super::super::Foundation::Numerics::Vector2,
    pub radiusX: f32,
    pub radiusY: f32,
}
pub type D2D1_EXTEND_MODE = i32;
pub const D2D1_EXTEND_MODE_CLAMP: D2D1_EXTEND_MODE = 0;
pub const D2D1_EXTEND_MODE_FORCE_DWORD: D2D1_EXTEND_MODE = -1;
pub const D2D1_EXTEND_MODE_MIRROR: D2D1_EXTEND_MODE = 2;
pub const D2D1_EXTEND_MODE_WRAP: D2D1_EXTEND_MODE = 1;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct D2D1_FACTORY_OPTIONS {
    pub debugLevel: D2D1_DEBUG_LEVEL,
}
pub type D2D1_FACTORY_TYPE = i32;
pub const D2D1_FACTORY_TYPE_FORCE_DWORD: D2D1_FACTORY_TYPE = -1;
pub const D2D1_FACTORY_TYPE_MULTI_THREADED: D2D1_FACTORY_TYPE = 1;
pub const D2D1_FACTORY_TYPE_SINGLE_THREADED: D2D1_FACTORY_TYPE = 0;
pub type D2D1_FEATURE_LEVEL = i32;
pub const D2D1_FEATURE_LEVEL_10: D2D1_FEATURE_LEVEL = 40960;
pub const D2D1_FEATURE_LEVEL_9: D2D1_FEATURE_LEVEL = 37120;
pub const D2D1_FEATURE_LEVEL_DEFAULT: D2D1_FEATURE_LEVEL = 0;
pub const D2D1_FEATURE_LEVEL_FORCE_DWORD: D2D1_FEATURE_LEVEL = -1;
pub type D2D1_FIGURE_BEGIN = i32;
pub const D2D1_FIGURE_BEGIN_FILLED: D2D1_FIGURE_BEGIN = 0;
pub const D2D1_FIGURE_BEGIN_FORCE_DWORD: D2D1_FIGURE_BEGIN = -1;
pub const D2D1_FIGURE_BEGIN_HOLLOW: D2D1_FIGURE_BEGIN = 1;
pub type D2D1_FIGURE_END = i32;
pub const D2D1_FIGURE_END_CLOSED: D2D1_FIGURE_END = 1;
pub const D2D1_FIGURE_END_FORCE_DWORD: D2D1_FIGURE_END = -1;
pub const D2D1_FIGURE_END_OPEN: D2D1_FIGURE_END = 0;
pub type D2D1_FILL_MODE = i32;
pub const D2D1_FILL_MODE_ALTERNATE: D2D1_FILL_MODE = 0;
pub const D2D1_FILL_MODE_FORCE_DWORD: D2D1_FILL_MODE = -1;
pub const D2D1_FILL_MODE_WINDING: D2D1_FILL_MODE = 1;
pub type D2D1_GAMMA = i32;
pub const D2D1_GAMMA_1_0: D2D1_GAMMA = 1;
pub const D2D1_GAMMA_2_2: D2D1_GAMMA = 0;
pub const D2D1_GAMMA_FORCE_DWORD: D2D1_GAMMA = -1;
pub type D2D1_GEOMETRY_RELATION = i32;
pub const D2D1_GEOMETRY_RELATION_CONTAINS: D2D1_GEOMETRY_RELATION = 3;
pub const D2D1_GEOMETRY_RELATION_DISJOINT: D2D1_GEOMETRY_RELATION = 1;
pub const D2D1_GEOMETRY_RELATION_FORCE_DWORD: D2D1_GEOMETRY_RELATION = -1;
pub const D2D1_GEOMETRY_RELATION_IS_CONTAINED: D2D1_GEOMETRY_RELATION = 2;
pub const D2D1_GEOMETRY_RELATION_OVERLAP: D2D1_GEOMETRY_RELATION = 4;
pub const D2D1_GEOMETRY_RELATION_UNKNOWN: D2D1_GEOMETRY_RELATION = 0;
pub type D2D1_GEOMETRY_SIMPLIFICATION_OPTION = i32;
pub const D2D1_GEOMETRY_SIMPLIFICATION_OPTION_CUBICS_AND_LINES: D2D1_GEOMETRY_SIMPLIFICATION_OPTION = 0;
pub const D2D1_GEOMETRY_SIMPLIFICATION_OPTION_FORCE_DWORD: D2D1_GEOMETRY_SIMPLIFICATION_OPTION = -1;
pub const D2D1_GEOMETRY_SIMPLIFICATION_OPTION_LINES: D2D1_GEOMETRY_SIMPLIFICATION_OPTION = 1;
#[repr(C)]
#[cfg(all(feature = "Win32_d2dbasetypes", feature = "Win32_dxgitype"))]
#[derive(Clone, Copy, Default)]
pub struct D2D1_GRADIENT_STOP {
    pub position: f32,
    pub color: super::d2dbasetypes::D2D_COLOR_F,
}
#[repr(C)]
#[cfg(all(feature = "Win32_dcommon", feature = "Win32_windef"))]
#[derive(Clone, Copy)]
pub struct D2D1_HWND_RENDER_TARGET_PROPERTIES {
    pub hwnd: super::windef::HWND,
    pub pixelSize: super::dcommon::D2D_SIZE_U,
    pub presentOptions: D2D1_PRESENT_OPTIONS,
}
#[cfg(all(feature = "Win32_dcommon", feature = "Win32_windef"))]
impl Default for D2D1_HWND_RENDER_TARGET_PROPERTIES {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const D2D1_INTERPOLATION_MODE_DEFINITION_ANISOTROPIC: i32 = 4;
pub const D2D1_INTERPOLATION_MODE_DEFINITION_CUBIC: i32 = 2;
pub const D2D1_INTERPOLATION_MODE_DEFINITION_FANT: i32 = 6;
pub const D2D1_INTERPOLATION_MODE_DEFINITION_HIGH_QUALITY_CUBIC: i32 = 5;
pub const D2D1_INTERPOLATION_MODE_DEFINITION_LINEAR: i32 = 1;
pub const D2D1_INTERPOLATION_MODE_DEFINITION_MIPMAP_LINEAR: i32 = 7;
pub const D2D1_INTERPOLATION_MODE_DEFINITION_MULTI_SAMPLE_LINEAR: i32 = 3;
pub const D2D1_INTERPOLATION_MODE_DEFINITION_NEAREST_NEIGHBOR: i32 = 0;
pub const D2D1_INVALID_TAG: i32 = -1;
pub type D2D1_LAYER_OPTIONS = u32;
pub const D2D1_LAYER_OPTIONS_FORCE_DWORD: D2D1_LAYER_OPTIONS = 4294967295;
pub const D2D1_LAYER_OPTIONS_INITIALIZE_FOR_CLEARTYPE: D2D1_LAYER_OPTIONS = 1;
pub const D2D1_LAYER_OPTIONS_NONE: D2D1_LAYER_OPTIONS = 0;
#[repr(C)]
#[cfg(all(feature = "Foundation_Numerics", feature = "Win32_dcommon"))]
#[derive(Clone, Copy)]
pub struct D2D1_LAYER_PARAMETERS {
    pub contentBounds: super::dcommon::D2D_RECT_F,
    pub geometricMask: *mut core::ffi::c_void,
    pub maskAntialiasMode: D2D1_ANTIALIAS_MODE,
    pub maskTransform: super::super::Foundation::Numerics::Matrix3x2,
    pub opacity: f32,
    pub opacityBrush: *mut core::ffi::c_void,
    pub layerOptions: D2D1_LAYER_OPTIONS,
}
#[cfg(all(feature = "Foundation_Numerics", feature = "Win32_dcommon"))]
impl Default for D2D1_LAYER_PARAMETERS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Foundation_Numerics")]
#[derive(Clone, Copy, Default)]
pub struct D2D1_LINEAR_GRADIENT_BRUSH_PROPERTIES {
    pub startPoint: super::super::Foundation::Numerics::Vector2,
    pub endPoint: super::super::Foundation::Numerics::Vector2,
}
pub type D2D1_LINE_JOIN = i32;
pub const D2D1_LINE_JOIN_BEVEL: D2D1_LINE_JOIN = 1;
pub const D2D1_LINE_JOIN_FORCE_DWORD: D2D1_LINE_JOIN = -1;
pub const D2D1_LINE_JOIN_MITER: D2D1_LINE_JOIN = 0;
pub const D2D1_LINE_JOIN_MITER_OR_BEVEL: D2D1_LINE_JOIN = 3;
pub const D2D1_LINE_JOIN_ROUND: D2D1_LINE_JOIN = 2;
pub type D2D1_OPACITY_MASK_CONTENT = i32;
pub const D2D1_OPACITY_MASK_CONTENT_FORCE_DWORD: D2D1_OPACITY_MASK_CONTENT = -1;
pub const D2D1_OPACITY_MASK_CONTENT_GRAPHICS: D2D1_OPACITY_MASK_CONTENT = 0;
pub const D2D1_OPACITY_MASK_CONTENT_TEXT_GDI_COMPATIBLE: D2D1_OPACITY_MASK_CONTENT = 2;
pub const D2D1_OPACITY_MASK_CONTENT_TEXT_NATURAL: D2D1_OPACITY_MASK_CONTENT = 1;
pub type D2D1_PATH_SEGMENT = u32;
pub const D2D1_PATH_SEGMENT_FORCE_DWORD: D2D1_PATH_SEGMENT = 4294967295;
pub const D2D1_PATH_SEGMENT_FORCE_ROUND_LINE_JOIN: D2D1_PATH_SEGMENT = 2;
pub const D2D1_PATH_SEGMENT_FORCE_UNSTROKED: D2D1_PATH_SEGMENT = 1;
pub const D2D1_PATH_SEGMENT_NONE: D2D1_PATH_SEGMENT = 0;
pub type D2D1_PRESENT_OPTIONS = u32;
pub const D2D1_PRESENT_OPTIONS_FORCE_DWORD: D2D1_PRESENT_OPTIONS = 4294967295;
pub const D2D1_PRESENT_OPTIONS_IMMEDIATELY: D2D1_PRESENT_OPTIONS = 2;
pub const D2D1_PRESENT_OPTIONS_NONE: D2D1_PRESENT_OPTIONS = 0;
pub const D2D1_PRESENT_OPTIONS_RETAIN_CONTENTS: D2D1_PRESENT_OPTIONS = 1;
#[repr(C)]
#[cfg(feature = "Foundation_Numerics")]
#[derive(Clone, Copy, Default)]
pub struct D2D1_QUADRATIC_BEZIER_SEGMENT {
    pub point1: super::super::Foundation::Numerics::Vector2,
    pub point2: super::super::Foundation::Numerics::Vector2,
}
#[repr(C)]
#[cfg(feature = "Foundation_Numerics")]
#[derive(Clone, Copy, Default)]
pub struct D2D1_RADIAL_GRADIENT_BRUSH_PROPERTIES {
    pub center: super::super::Foundation::Numerics::Vector2,
    pub gradientOriginOffset: super::super::Foundation::Numerics::Vector2,
    pub radiusX: f32,
    pub radiusY: f32,
}
#[repr(C)]
#[cfg(all(feature = "Win32_dcommon", feature = "Win32_dxgiformat"))]
#[derive(Clone, Copy, Default)]
pub struct D2D1_RENDER_TARGET_PROPERTIES {
    pub r#type: D2D1_RENDER_TARGET_TYPE,
    pub pixelFormat: super::dcommon::D2D1_PIXEL_FORMAT,
    pub dpiX: f32,
    pub dpiY: f32,
    pub usage: D2D1_RENDER_TARGET_USAGE,
    pub minLevel: D2D1_FEATURE_LEVEL,
}
pub type D2D1_RENDER_TARGET_TYPE = i32;
pub const D2D1_RENDER_TARGET_TYPE_DEFAULT: D2D1_RENDER_TARGET_TYPE = 0;
pub const D2D1_RENDER_TARGET_TYPE_FORCE_DWORD: D2D1_RENDER_TARGET_TYPE = -1;
pub const D2D1_RENDER_TARGET_TYPE_HARDWARE: D2D1_RENDER_TARGET_TYPE = 2;
pub const D2D1_RENDER_TARGET_TYPE_SOFTWARE: D2D1_RENDER_TARGET_TYPE = 1;
pub type D2D1_RENDER_TARGET_USAGE = u32;
pub const D2D1_RENDER_TARGET_USAGE_FORCE_BITMAP_REMOTING: D2D1_RENDER_TARGET_USAGE = 1;
pub const D2D1_RENDER_TARGET_USAGE_FORCE_DWORD: D2D1_RENDER_TARGET_USAGE = 4294967295;
pub const D2D1_RENDER_TARGET_USAGE_GDI_COMPATIBLE: D2D1_RENDER_TARGET_USAGE = 2;
pub const D2D1_RENDER_TARGET_USAGE_NONE: D2D1_RENDER_TARGET_USAGE = 0;
#[repr(C)]
#[cfg(feature = "Win32_dcommon")]
#[derive(Clone, Copy, Default)]
pub struct D2D1_ROUNDED_RECT {
    pub rect: super::dcommon::D2D_RECT_F,
    pub radiusX: f32,
    pub radiusY: f32,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct D2D1_STROKE_STYLE_PROPERTIES {
    pub startCap: D2D1_CAP_STYLE,
    pub endCap: D2D1_CAP_STYLE,
    pub dashCap: D2D1_CAP_STYLE,
    pub lineJoin: D2D1_LINE_JOIN,
    pub miterLimit: f32,
    pub dashStyle: D2D1_DASH_STYLE,
    pub dashOffset: f32,
}
pub type D2D1_SWEEP_DIRECTION = i32;
pub const D2D1_SWEEP_DIRECTION_CLOCKWISE: D2D1_SWEEP_DIRECTION = 1;
pub const D2D1_SWEEP_DIRECTION_COUNTER_CLOCKWISE: D2D1_SWEEP_DIRECTION = 0;
pub const D2D1_SWEEP_DIRECTION_FORCE_DWORD: D2D1_SWEEP_DIRECTION = -1;
pub type D2D1_TAG = u64;
pub type D2D1_TEXT_ANTIALIAS_MODE = i32;
pub const D2D1_TEXT_ANTIALIAS_MODE_ALIASED: D2D1_TEXT_ANTIALIAS_MODE = 3;
pub const D2D1_TEXT_ANTIALIAS_MODE_CLEARTYPE: D2D1_TEXT_ANTIALIAS_MODE = 1;
pub const D2D1_TEXT_ANTIALIAS_MODE_DEFAULT: D2D1_TEXT_ANTIALIAS_MODE = 0;
pub const D2D1_TEXT_ANTIALIAS_MODE_FORCE_DWORD: D2D1_TEXT_ANTIALIAS_MODE = -1;
pub const D2D1_TEXT_ANTIALIAS_MODE_GRAYSCALE: D2D1_TEXT_ANTIALIAS_MODE = 2;
#[repr(C)]
#[cfg(feature = "Foundation_Numerics")]
#[derive(Clone, Copy, Default)]
pub struct D2D1_TRIANGLE {
    pub point1: super::super::Foundation::Numerics::Vector2,
    pub point2: super::super::Foundation::Numerics::Vector2,
    pub point3: super::super::Foundation::Numerics::Vector2,
}
pub type D2D1_WINDOW_STATE = u32;
pub const D2D1_WINDOW_STATE_FORCE_DWORD: D2D1_WINDOW_STATE = 4294967295;
pub const D2D1_WINDOW_STATE_NONE: D2D1_WINDOW_STATE = 0;
pub const D2D1_WINDOW_STATE_OCCLUDED: D2D1_WINDOW_STATE = 1;
