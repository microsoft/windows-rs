#[inline]
pub unsafe fn D2D1CreateFactory(factorytype: D2D1_FACTORY_TYPE, riid: *const windows_core::GUID, pfactoryoptions: Option<*const D2D1_FACTORY_OPTIONS>, ppifactory: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
    windows_core::link!("d2d1.dll" "system" fn D2D1CreateFactory(factorytype : D2D1_FACTORY_TYPE, riid : *const windows_core::GUID, pfactoryoptions : *const D2D1_FACTORY_OPTIONS, ppifactory : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe { D2D1CreateFactory(factorytype, riid, pfactoryoptions.unwrap_or(core::mem::zeroed()) as _, ppifactory as _) }
}
#[inline]
pub unsafe fn D2D1InvertMatrix(matrix: *mut windows_numerics::Matrix3x2) -> windows_core::BOOL {
    windows_core::link!("d2d1.dll" "system" fn D2D1InvertMatrix(matrix : *mut windows_numerics::Matrix3x2) -> windows_core::BOOL);
    unsafe { D2D1InvertMatrix(matrix as _) }
}
#[inline]
pub unsafe fn D2D1IsMatrixInvertible(matrix: *const windows_numerics::Matrix3x2) -> windows_core::BOOL {
    windows_core::link!("d2d1.dll" "system" fn D2D1IsMatrixInvertible(matrix : *const windows_numerics::Matrix3x2) -> windows_core::BOOL);
    unsafe { D2D1IsMatrixInvertible(matrix) }
}
#[inline]
pub unsafe fn D2D1MakeRotateMatrix(angle: f32, center: windows_numerics::Vector2, matrix: *mut windows_numerics::Matrix3x2) {
    windows_core::link!("d2d1.dll" "system" fn D2D1MakeRotateMatrix(angle : f32, center : windows_numerics::Vector2, matrix : *mut windows_numerics::Matrix3x2));
    unsafe { D2D1MakeRotateMatrix(angle, core::mem::transmute(center), matrix as _) }
}
#[inline]
pub unsafe fn D2D1MakeSkewMatrix(anglex: f32, angley: f32, center: windows_numerics::Vector2, matrix: *mut windows_numerics::Matrix3x2) {
    windows_core::link!("d2d1.dll" "system" fn D2D1MakeSkewMatrix(anglex : f32, angley : f32, center : windows_numerics::Vector2, matrix : *mut windows_numerics::Matrix3x2));
    unsafe { D2D1MakeSkewMatrix(anglex, angley, core::mem::transmute(center), matrix as _) }
}
pub type D2D1_ANTIALIAS_MODE = i32;
pub const D2D1_ANTIALIAS_MODE_ALIASED: D2D1_ANTIALIAS_MODE = 1;
pub const D2D1_ANTIALIAS_MODE_FORCE_DWORD: D2D1_ANTIALIAS_MODE = -1;
pub const D2D1_ANTIALIAS_MODE_PER_PRIMITIVE: D2D1_ANTIALIAS_MODE = 0;
#[repr(C)]
#[cfg(feature = "Win32_dcommon")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct D2D1_ARC_SEGMENT {
    pub point: windows_numerics::Vector2,
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
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct D2D1_BEZIER_SEGMENT {
    pub point1: windows_numerics::Vector2,
    pub point2: windows_numerics::Vector2,
    pub point3: windows_numerics::Vector2,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
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
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct D2D1_BITMAP_PROPERTIES {
    pub pixelFormat: super::dcommon::D2D1_PIXEL_FORMAT,
    pub dpiX: f32,
    pub dpiY: f32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct D2D1_BRUSH_PROPERTIES {
    pub opacity: f32,
    pub transform: windows_numerics::Matrix3x2,
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
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct D2D1_DRAWING_STATE_DESCRIPTION {
    pub antialiasMode: D2D1_ANTIALIAS_MODE,
    pub textAntialiasMode: D2D1_TEXT_ANTIALIAS_MODE,
    pub tag1: D2D1_TAG,
    pub tag2: D2D1_TAG,
    pub transform: windows_numerics::Matrix3x2,
}
pub type D2D1_DRAW_TEXT_OPTIONS = u32;
pub const D2D1_DRAW_TEXT_OPTIONS_CLIP: D2D1_DRAW_TEXT_OPTIONS = 2;
pub const D2D1_DRAW_TEXT_OPTIONS_DISABLE_COLOR_BITMAP_SNAPPING: D2D1_DRAW_TEXT_OPTIONS = 8;
pub const D2D1_DRAW_TEXT_OPTIONS_ENABLE_COLOR_FONT: D2D1_DRAW_TEXT_OPTIONS = 4;
pub const D2D1_DRAW_TEXT_OPTIONS_FORCE_DWORD: D2D1_DRAW_TEXT_OPTIONS = 4294967295;
pub const D2D1_DRAW_TEXT_OPTIONS_NONE: D2D1_DRAW_TEXT_OPTIONS = 0;
pub const D2D1_DRAW_TEXT_OPTIONS_NO_SNAP: D2D1_DRAW_TEXT_OPTIONS = 1;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct D2D1_ELLIPSE {
    pub point: windows_numerics::Vector2,
    pub radiusX: f32,
    pub radiusY: f32,
}
pub type D2D1_EXTEND_MODE = i32;
pub const D2D1_EXTEND_MODE_CLAMP: D2D1_EXTEND_MODE = 0;
pub const D2D1_EXTEND_MODE_FORCE_DWORD: D2D1_EXTEND_MODE = -1;
pub const D2D1_EXTEND_MODE_MIRROR: D2D1_EXTEND_MODE = 2;
pub const D2D1_EXTEND_MODE_WRAP: D2D1_EXTEND_MODE = 1;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
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
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct D2D1_GRADIENT_STOP {
    pub position: f32,
    pub color: super::d2dbasetypes::D2D_COLOR_F,
}
#[repr(C)]
#[cfg(all(feature = "Win32_dcommon", feature = "Win32_windef"))]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct D2D1_HWND_RENDER_TARGET_PROPERTIES {
    pub hwnd: super::windef::HWND,
    pub pixelSize: super::dcommon::D2D_SIZE_U,
    pub presentOptions: D2D1_PRESENT_OPTIONS,
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
#[cfg(feature = "Win32_dcommon")]
#[derive(Clone, Debug, Default, PartialEq)]
pub struct D2D1_LAYER_PARAMETERS {
    pub contentBounds: super::dcommon::D2D_RECT_F,
    pub geometricMask: core::mem::ManuallyDrop<Option<ID2D1Geometry>>,
    pub maskAntialiasMode: D2D1_ANTIALIAS_MODE,
    pub maskTransform: windows_numerics::Matrix3x2,
    pub opacity: f32,
    pub opacityBrush: core::mem::ManuallyDrop<Option<ID2D1Brush>>,
    pub layerOptions: D2D1_LAYER_OPTIONS,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct D2D1_LINEAR_GRADIENT_BRUSH_PROPERTIES {
    pub startPoint: windows_numerics::Vector2,
    pub endPoint: windows_numerics::Vector2,
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
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct D2D1_QUADRATIC_BEZIER_SEGMENT {
    pub point1: windows_numerics::Vector2,
    pub point2: windows_numerics::Vector2,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct D2D1_RADIAL_GRADIENT_BRUSH_PROPERTIES {
    pub center: windows_numerics::Vector2,
    pub gradientOriginOffset: windows_numerics::Vector2,
    pub radiusX: f32,
    pub radiusY: f32,
}
#[repr(C)]
#[cfg(all(feature = "Win32_dcommon", feature = "Win32_dxgiformat"))]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
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
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct D2D1_ROUNDED_RECT {
    pub rect: super::dcommon::D2D_RECT_F,
    pub radiusX: f32,
    pub radiusY: f32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
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
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct D2D1_TAG(pub u64);
pub type D2D1_TEXT_ANTIALIAS_MODE = i32;
pub const D2D1_TEXT_ANTIALIAS_MODE_ALIASED: D2D1_TEXT_ANTIALIAS_MODE = 3;
pub const D2D1_TEXT_ANTIALIAS_MODE_CLEARTYPE: D2D1_TEXT_ANTIALIAS_MODE = 1;
pub const D2D1_TEXT_ANTIALIAS_MODE_DEFAULT: D2D1_TEXT_ANTIALIAS_MODE = 0;
pub const D2D1_TEXT_ANTIALIAS_MODE_FORCE_DWORD: D2D1_TEXT_ANTIALIAS_MODE = -1;
pub const D2D1_TEXT_ANTIALIAS_MODE_GRAYSCALE: D2D1_TEXT_ANTIALIAS_MODE = 2;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct D2D1_TRIANGLE {
    pub point1: windows_numerics::Vector2,
    pub point2: windows_numerics::Vector2,
    pub point3: windows_numerics::Vector2,
}
pub type D2D1_WINDOW_STATE = u32;
pub const D2D1_WINDOW_STATE_FORCE_DWORD: D2D1_WINDOW_STATE = 4294967295;
pub const D2D1_WINDOW_STATE_NONE: D2D1_WINDOW_STATE = 0;
pub const D2D1_WINDOW_STATE_OCCLUDED: D2D1_WINDOW_STATE = 1;
windows_core::imp::define_interface!(ID2D1Bitmap, ID2D1Bitmap_Vtbl, 0xa2296057_ea42_4099_983b_539fb6505426);
impl core::ops::Deref for ID2D1Bitmap {
    type Target = ID2D1Image;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ID2D1Bitmap, windows_core::IUnknown, ID2D1Resource, ID2D1Image);
impl ID2D1Bitmap {
    #[cfg(feature = "Win32_dcommon")]
    pub unsafe fn GetSize(&self) -> super::dcommon::D2D_SIZE_F {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetSize)(windows_core::Interface::as_raw(self), &mut result__);
            result__
        }
    }
    #[cfg(feature = "Win32_dcommon")]
    pub unsafe fn GetPixelSize(&self) -> super::dcommon::D2D_SIZE_U {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetPixelSize)(windows_core::Interface::as_raw(self), &mut result__);
            result__
        }
    }
    #[cfg(all(feature = "Win32_dcommon", feature = "Win32_dxgiformat"))]
    pub unsafe fn GetPixelFormat(&self) -> super::dcommon::D2D1_PIXEL_FORMAT {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetPixelFormat)(windows_core::Interface::as_raw(self), &mut result__);
            result__
        }
    }
    pub unsafe fn GetDpi(&self, dpix: *mut f32, dpiy: *mut f32) {
        unsafe {
            (windows_core::Interface::vtable(self).GetDpi)(windows_core::Interface::as_raw(self), dpix as _, dpiy as _);
        }
    }
    #[cfg(feature = "Win32_dcommon")]
    pub unsafe fn CopyFromBitmap<P1>(&self, destpoint: Option<*const super::dcommon::D2D_POINT_2U>, bitmap: P1, srcrect: Option<*const super::dcommon::D2D_RECT_U>) -> windows_core::HRESULT
    where
        P1: windows_core::Param<Self>,
    {
        unsafe { (windows_core::Interface::vtable(self).CopyFromBitmap)(windows_core::Interface::as_raw(self), destpoint.unwrap_or(core::mem::zeroed()) as _, bitmap.param().abi(), srcrect.unwrap_or(core::mem::zeroed()) as _) }
    }
    #[cfg(feature = "Win32_dcommon")]
    pub unsafe fn CopyFromRenderTarget<P1>(&self, destpoint: Option<*const super::dcommon::D2D_POINT_2U>, rendertarget: P1, srcrect: Option<*const super::dcommon::D2D_RECT_U>) -> windows_core::HRESULT
    where
        P1: windows_core::Param<ID2D1RenderTarget>,
    {
        unsafe { (windows_core::Interface::vtable(self).CopyFromRenderTarget)(windows_core::Interface::as_raw(self), destpoint.unwrap_or(core::mem::zeroed()) as _, rendertarget.param().abi(), srcrect.unwrap_or(core::mem::zeroed()) as _) }
    }
    #[cfg(feature = "Win32_dcommon")]
    pub unsafe fn CopyFromMemory(&self, dstrect: Option<*const super::dcommon::D2D_RECT_U>, srcdata: *const core::ffi::c_void, pitch: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).CopyFromMemory)(windows_core::Interface::as_raw(self), dstrect.unwrap_or(core::mem::zeroed()) as _, srcdata, pitch) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ID2D1Bitmap_Vtbl {
    pub base__: ID2D1Image_Vtbl,
    #[cfg(feature = "Win32_dcommon")]
    pub GetSize: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::dcommon::D2D_SIZE_F),
    #[cfg(not(feature = "Win32_dcommon"))]
    GetSize: usize,
    #[cfg(feature = "Win32_dcommon")]
    pub GetPixelSize: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::dcommon::D2D_SIZE_U),
    #[cfg(not(feature = "Win32_dcommon"))]
    GetPixelSize: usize,
    #[cfg(all(feature = "Win32_dcommon", feature = "Win32_dxgiformat"))]
    pub GetPixelFormat: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::dcommon::D2D1_PIXEL_FORMAT),
    #[cfg(not(all(feature = "Win32_dcommon", feature = "Win32_dxgiformat")))]
    GetPixelFormat: usize,
    pub GetDpi: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f32, *mut f32),
    #[cfg(feature = "Win32_dcommon")]
    pub CopyFromBitmap: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::dcommon::D2D_POINT_2U, *mut core::ffi::c_void, *const super::dcommon::D2D_RECT_U) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_dcommon"))]
    CopyFromBitmap: usize,
    #[cfg(feature = "Win32_dcommon")]
    pub CopyFromRenderTarget: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::dcommon::D2D_POINT_2U, *mut core::ffi::c_void, *const super::dcommon::D2D_RECT_U) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_dcommon"))]
    CopyFromRenderTarget: usize,
    #[cfg(feature = "Win32_dcommon")]
    pub CopyFromMemory: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::dcommon::D2D_RECT_U, *const core::ffi::c_void, u32) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_dcommon"))]
    CopyFromMemory: usize,
}
#[cfg(all(feature = "Win32_dcommon", feature = "Win32_dxgiformat"))]
pub trait ID2D1Bitmap_Impl: ID2D1Image_Impl {
    fn GetSize(&self) -> super::dcommon::D2D_SIZE_F;
    fn GetPixelSize(&self) -> super::dcommon::D2D_SIZE_U;
    fn GetPixelFormat(&self) -> super::dcommon::D2D1_PIXEL_FORMAT;
    fn GetDpi(&self, dpix: *mut f32, dpiy: *mut f32);
    fn CopyFromBitmap(&self, destpoint: *const super::dcommon::D2D_POINT_2U, bitmap: windows_core::Ref<ID2D1Bitmap>, srcrect: *const super::dcommon::D2D_RECT_U) -> windows_core::Result<()>;
    fn CopyFromRenderTarget(&self, destpoint: *const super::dcommon::D2D_POINT_2U, rendertarget: windows_core::Ref<ID2D1RenderTarget>, srcrect: *const super::dcommon::D2D_RECT_U) -> windows_core::Result<()>;
    fn CopyFromMemory(&self, dstrect: *const super::dcommon::D2D_RECT_U, srcdata: *const core::ffi::c_void, pitch: u32) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_dcommon", feature = "Win32_dxgiformat"))]
impl ID2D1Bitmap_Vtbl {
    pub const fn new<Identity: ID2D1Bitmap_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetSize<Identity: ID2D1Bitmap_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut super::dcommon::D2D_SIZE_F) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                *result__ = ID2D1Bitmap_Impl::GetSize(this);
            }
        }
        unsafe extern "system" fn GetPixelSize<Identity: ID2D1Bitmap_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut super::dcommon::D2D_SIZE_U) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                *result__ = ID2D1Bitmap_Impl::GetPixelSize(this);
            }
        }
        unsafe extern "system" fn GetPixelFormat<Identity: ID2D1Bitmap_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut super::dcommon::D2D1_PIXEL_FORMAT) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                *result__ = ID2D1Bitmap_Impl::GetPixelFormat(this);
            }
        }
        unsafe extern "system" fn GetDpi<Identity: ID2D1Bitmap_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dpix: *mut f32, dpiy: *mut f32) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID2D1Bitmap_Impl::GetDpi(this, core::mem::transmute_copy(&dpix), core::mem::transmute_copy(&dpiy));
            }
        }
        unsafe extern "system" fn CopyFromBitmap<Identity: ID2D1Bitmap_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, destpoint: *const super::dcommon::D2D_POINT_2U, bitmap: *mut core::ffi::c_void, srcrect: *const super::dcommon::D2D_RECT_U) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID2D1Bitmap_Impl::CopyFromBitmap(this, core::mem::transmute_copy(&destpoint), core::mem::transmute_copy(&bitmap), core::mem::transmute_copy(&srcrect)).into()
            }
        }
        unsafe extern "system" fn CopyFromRenderTarget<Identity: ID2D1Bitmap_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, destpoint: *const super::dcommon::D2D_POINT_2U, rendertarget: *mut core::ffi::c_void, srcrect: *const super::dcommon::D2D_RECT_U) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID2D1Bitmap_Impl::CopyFromRenderTarget(this, core::mem::transmute_copy(&destpoint), core::mem::transmute_copy(&rendertarget), core::mem::transmute_copy(&srcrect)).into()
            }
        }
        unsafe extern "system" fn CopyFromMemory<Identity: ID2D1Bitmap_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dstrect: *const super::dcommon::D2D_RECT_U, srcdata: *const core::ffi::c_void, pitch: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID2D1Bitmap_Impl::CopyFromMemory(this, core::mem::transmute_copy(&dstrect), core::mem::transmute_copy(&srcdata), core::mem::transmute_copy(&pitch)).into()
            }
        }
        Self {
            base__: ID2D1Image_Vtbl::new::<Identity, OFFSET>(),
            GetSize: GetSize::<Identity, OFFSET>,
            GetPixelSize: GetPixelSize::<Identity, OFFSET>,
            GetPixelFormat: GetPixelFormat::<Identity, OFFSET>,
            GetDpi: GetDpi::<Identity, OFFSET>,
            CopyFromBitmap: CopyFromBitmap::<Identity, OFFSET>,
            CopyFromRenderTarget: CopyFromRenderTarget::<Identity, OFFSET>,
            CopyFromMemory: CopyFromMemory::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ID2D1Bitmap as windows_core::Interface>::IID || iid == &<ID2D1Resource as windows_core::Interface>::IID || iid == &<ID2D1Image as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_dcommon", feature = "Win32_dxgiformat"))]
impl windows_core::RuntimeName for ID2D1Bitmap {}
windows_core::imp::define_interface!(ID2D1BitmapBrush, ID2D1BitmapBrush_Vtbl, 0x2cd906aa_12e2_11dc_9fed_001143a055f9);
impl core::ops::Deref for ID2D1BitmapBrush {
    type Target = ID2D1Brush;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ID2D1BitmapBrush, windows_core::IUnknown, ID2D1Resource, ID2D1Brush);
impl ID2D1BitmapBrush {
    pub unsafe fn SetExtendModeX(&self, extendmodex: D2D1_EXTEND_MODE) {
        unsafe {
            (windows_core::Interface::vtable(self).SetExtendModeX)(windows_core::Interface::as_raw(self), extendmodex);
        }
    }
    pub unsafe fn SetExtendModeY(&self, extendmodey: D2D1_EXTEND_MODE) {
        unsafe {
            (windows_core::Interface::vtable(self).SetExtendModeY)(windows_core::Interface::as_raw(self), extendmodey);
        }
    }
    pub unsafe fn SetInterpolationMode(&self, interpolationmode: D2D1_BITMAP_INTERPOLATION_MODE) {
        unsafe {
            (windows_core::Interface::vtable(self).SetInterpolationMode)(windows_core::Interface::as_raw(self), interpolationmode);
        }
    }
    pub unsafe fn SetBitmap<P0>(&self, bitmap: P0)
    where
        P0: windows_core::Param<ID2D1Bitmap>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).SetBitmap)(windows_core::Interface::as_raw(self), bitmap.param().abi());
        }
    }
    pub unsafe fn GetExtendModeX(&self) -> D2D1_EXTEND_MODE {
        unsafe { (windows_core::Interface::vtable(self).GetExtendModeX)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn GetExtendModeY(&self) -> D2D1_EXTEND_MODE {
        unsafe { (windows_core::Interface::vtable(self).GetExtendModeY)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn GetInterpolationMode(&self) -> D2D1_BITMAP_INTERPOLATION_MODE {
        unsafe { (windows_core::Interface::vtable(self).GetInterpolationMode)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn GetBitmap(&self) -> windows_core::Result<ID2D1Bitmap> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetBitmap)(windows_core::Interface::as_raw(self), &mut result__);
            windows_core::Type::from_abi(result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ID2D1BitmapBrush_Vtbl {
    pub base__: ID2D1Brush_Vtbl,
    pub SetExtendModeX: unsafe extern "system" fn(*mut core::ffi::c_void, D2D1_EXTEND_MODE),
    pub SetExtendModeY: unsafe extern "system" fn(*mut core::ffi::c_void, D2D1_EXTEND_MODE),
    pub SetInterpolationMode: unsafe extern "system" fn(*mut core::ffi::c_void, D2D1_BITMAP_INTERPOLATION_MODE),
    pub SetBitmap: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void),
    pub GetExtendModeX: unsafe extern "system" fn(*mut core::ffi::c_void) -> D2D1_EXTEND_MODE,
    pub GetExtendModeY: unsafe extern "system" fn(*mut core::ffi::c_void) -> D2D1_EXTEND_MODE,
    pub GetInterpolationMode: unsafe extern "system" fn(*mut core::ffi::c_void) -> D2D1_BITMAP_INTERPOLATION_MODE,
    pub GetBitmap: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void),
}
pub trait ID2D1BitmapBrush_Impl: ID2D1Brush_Impl {
    fn SetExtendModeX(&self, extendmodex: D2D1_EXTEND_MODE);
    fn SetExtendModeY(&self, extendmodey: D2D1_EXTEND_MODE);
    fn SetInterpolationMode(&self, interpolationmode: D2D1_BITMAP_INTERPOLATION_MODE);
    fn SetBitmap(&self, bitmap: windows_core::Ref<ID2D1Bitmap>);
    fn GetExtendModeX(&self) -> D2D1_EXTEND_MODE;
    fn GetExtendModeY(&self) -> D2D1_EXTEND_MODE;
    fn GetInterpolationMode(&self) -> D2D1_BITMAP_INTERPOLATION_MODE;
    fn GetBitmap(&self, bitmap: windows_core::OutRef<ID2D1Bitmap>);
}
impl ID2D1BitmapBrush_Vtbl {
    pub const fn new<Identity: ID2D1BitmapBrush_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn SetExtendModeX<Identity: ID2D1BitmapBrush_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, extendmodex: D2D1_EXTEND_MODE) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID2D1BitmapBrush_Impl::SetExtendModeX(this, core::mem::transmute_copy(&extendmodex));
            }
        }
        unsafe extern "system" fn SetExtendModeY<Identity: ID2D1BitmapBrush_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, extendmodey: D2D1_EXTEND_MODE) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID2D1BitmapBrush_Impl::SetExtendModeY(this, core::mem::transmute_copy(&extendmodey));
            }
        }
        unsafe extern "system" fn SetInterpolationMode<Identity: ID2D1BitmapBrush_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, interpolationmode: D2D1_BITMAP_INTERPOLATION_MODE) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID2D1BitmapBrush_Impl::SetInterpolationMode(this, core::mem::transmute_copy(&interpolationmode));
            }
        }
        unsafe extern "system" fn SetBitmap<Identity: ID2D1BitmapBrush_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bitmap: *mut core::ffi::c_void) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID2D1BitmapBrush_Impl::SetBitmap(this, core::mem::transmute_copy(&bitmap));
            }
        }
        unsafe extern "system" fn GetExtendModeX<Identity: ID2D1BitmapBrush_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> D2D1_EXTEND_MODE {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID2D1BitmapBrush_Impl::GetExtendModeX(this)
            }
        }
        unsafe extern "system" fn GetExtendModeY<Identity: ID2D1BitmapBrush_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> D2D1_EXTEND_MODE {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID2D1BitmapBrush_Impl::GetExtendModeY(this)
            }
        }
        unsafe extern "system" fn GetInterpolationMode<Identity: ID2D1BitmapBrush_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> D2D1_BITMAP_INTERPOLATION_MODE {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID2D1BitmapBrush_Impl::GetInterpolationMode(this)
            }
        }
        unsafe extern "system" fn GetBitmap<Identity: ID2D1BitmapBrush_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bitmap: *mut *mut core::ffi::c_void) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID2D1BitmapBrush_Impl::GetBitmap(this, core::mem::transmute_copy(&bitmap));
            }
        }
        Self {
            base__: ID2D1Brush_Vtbl::new::<Identity, OFFSET>(),
            SetExtendModeX: SetExtendModeX::<Identity, OFFSET>,
            SetExtendModeY: SetExtendModeY::<Identity, OFFSET>,
            SetInterpolationMode: SetInterpolationMode::<Identity, OFFSET>,
            SetBitmap: SetBitmap::<Identity, OFFSET>,
            GetExtendModeX: GetExtendModeX::<Identity, OFFSET>,
            GetExtendModeY: GetExtendModeY::<Identity, OFFSET>,
            GetInterpolationMode: GetInterpolationMode::<Identity, OFFSET>,
            GetBitmap: GetBitmap::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ID2D1BitmapBrush as windows_core::Interface>::IID || iid == &<ID2D1Resource as windows_core::Interface>::IID || iid == &<ID2D1Brush as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for ID2D1BitmapBrush {}
windows_core::imp::define_interface!(ID2D1BitmapRenderTarget, ID2D1BitmapRenderTarget_Vtbl, 0x2cd90695_12e2_11dc_9fed_001143a055f9);
impl core::ops::Deref for ID2D1BitmapRenderTarget {
    type Target = ID2D1RenderTarget;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ID2D1BitmapRenderTarget, windows_core::IUnknown, ID2D1Resource, ID2D1RenderTarget);
impl ID2D1BitmapRenderTarget {
    pub unsafe fn GetBitmap(&self) -> windows_core::Result<ID2D1Bitmap> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetBitmap)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ID2D1BitmapRenderTarget_Vtbl {
    pub base__: ID2D1RenderTarget_Vtbl,
    pub GetBitmap: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_d2dbasetypes", feature = "Win32_dcommon", feature = "Win32_dwrite", feature = "Win32_dxgiformat", feature = "Win32_dxgitype", feature = "Win32_wincodec"))]
pub trait ID2D1BitmapRenderTarget_Impl: ID2D1RenderTarget_Impl {
    fn GetBitmap(&self) -> windows_core::Result<ID2D1Bitmap>;
}
#[cfg(all(feature = "Win32_d2dbasetypes", feature = "Win32_dcommon", feature = "Win32_dwrite", feature = "Win32_dxgiformat", feature = "Win32_dxgitype", feature = "Win32_wincodec"))]
impl ID2D1BitmapRenderTarget_Vtbl {
    pub const fn new<Identity: ID2D1BitmapRenderTarget_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetBitmap<Identity: ID2D1BitmapRenderTarget_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bitmap: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ID2D1BitmapRenderTarget_Impl::GetBitmap(this) {
                    Ok(ok__) => {
                        bitmap.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self { base__: ID2D1RenderTarget_Vtbl::new::<Identity, OFFSET>(), GetBitmap: GetBitmap::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ID2D1BitmapRenderTarget as windows_core::Interface>::IID || iid == &<ID2D1Resource as windows_core::Interface>::IID || iid == &<ID2D1RenderTarget as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_d2dbasetypes", feature = "Win32_dcommon", feature = "Win32_dwrite", feature = "Win32_dxgiformat", feature = "Win32_dxgitype", feature = "Win32_wincodec"))]
impl windows_core::RuntimeName for ID2D1BitmapRenderTarget {}
windows_core::imp::define_interface!(ID2D1Brush, ID2D1Brush_Vtbl, 0x2cd906a8_12e2_11dc_9fed_001143a055f9);
impl core::ops::Deref for ID2D1Brush {
    type Target = ID2D1Resource;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ID2D1Brush, windows_core::IUnknown, ID2D1Resource);
impl ID2D1Brush {
    pub unsafe fn SetOpacity(&self, opacity: f32) {
        unsafe {
            (windows_core::Interface::vtable(self).SetOpacity)(windows_core::Interface::as_raw(self), opacity);
        }
    }
    pub unsafe fn SetTransform(&self, transform: *const windows_numerics::Matrix3x2) {
        unsafe {
            (windows_core::Interface::vtable(self).SetTransform)(windows_core::Interface::as_raw(self), transform);
        }
    }
    pub unsafe fn GetOpacity(&self) -> f32 {
        unsafe { (windows_core::Interface::vtable(self).GetOpacity)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn GetTransform(&self, transform: *mut windows_numerics::Matrix3x2) {
        unsafe {
            (windows_core::Interface::vtable(self).GetTransform)(windows_core::Interface::as_raw(self), transform as _);
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ID2D1Brush_Vtbl {
    pub base__: ID2D1Resource_Vtbl,
    pub SetOpacity: unsafe extern "system" fn(*mut core::ffi::c_void, f32),
    pub SetTransform: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_numerics::Matrix3x2),
    pub GetOpacity: unsafe extern "system" fn(*mut core::ffi::c_void) -> f32,
    pub GetTransform: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_numerics::Matrix3x2),
}
pub trait ID2D1Brush_Impl: ID2D1Resource_Impl {
    fn SetOpacity(&self, opacity: f32);
    fn SetTransform(&self, transform: *const windows_numerics::Matrix3x2);
    fn GetOpacity(&self) -> f32;
    fn GetTransform(&self, transform: *mut windows_numerics::Matrix3x2);
}
impl ID2D1Brush_Vtbl {
    pub const fn new<Identity: ID2D1Brush_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn SetOpacity<Identity: ID2D1Brush_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, opacity: f32) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID2D1Brush_Impl::SetOpacity(this, core::mem::transmute_copy(&opacity));
            }
        }
        unsafe extern "system" fn SetTransform<Identity: ID2D1Brush_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, transform: *const windows_numerics::Matrix3x2) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID2D1Brush_Impl::SetTransform(this, core::mem::transmute_copy(&transform));
            }
        }
        unsafe extern "system" fn GetOpacity<Identity: ID2D1Brush_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> f32 {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID2D1Brush_Impl::GetOpacity(this)
            }
        }
        unsafe extern "system" fn GetTransform<Identity: ID2D1Brush_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, transform: *mut windows_numerics::Matrix3x2) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID2D1Brush_Impl::GetTransform(this, core::mem::transmute_copy(&transform));
            }
        }
        Self {
            base__: ID2D1Resource_Vtbl::new::<Identity, OFFSET>(),
            SetOpacity: SetOpacity::<Identity, OFFSET>,
            SetTransform: SetTransform::<Identity, OFFSET>,
            GetOpacity: GetOpacity::<Identity, OFFSET>,
            GetTransform: GetTransform::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ID2D1Brush as windows_core::Interface>::IID || iid == &<ID2D1Resource as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for ID2D1Brush {}
windows_core::imp::define_interface!(ID2D1DCRenderTarget, ID2D1DCRenderTarget_Vtbl, 0x1c51bc64_de61_46fd_9899_63a5d8f03950);
impl core::ops::Deref for ID2D1DCRenderTarget {
    type Target = ID2D1RenderTarget;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ID2D1DCRenderTarget, windows_core::IUnknown, ID2D1Resource, ID2D1RenderTarget);
impl ID2D1DCRenderTarget {
    #[cfg(feature = "Win32_windef")]
    pub unsafe fn BindDC(&self, hdc: super::windef::HDC, psubrect: *const super::windef::RECT) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).BindDC)(windows_core::Interface::as_raw(self), hdc, psubrect) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ID2D1DCRenderTarget_Vtbl {
    pub base__: ID2D1RenderTarget_Vtbl,
    #[cfg(feature = "Win32_windef")]
    pub BindDC: unsafe extern "system" fn(*mut core::ffi::c_void, super::windef::HDC, *const super::windef::RECT) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_windef"))]
    BindDC: usize,
}
#[cfg(all(feature = "Win32_d2dbasetypes", feature = "Win32_dcommon", feature = "Win32_dwrite", feature = "Win32_dxgiformat", feature = "Win32_dxgitype", feature = "Win32_wincodec", feature = "Win32_windef"))]
pub trait ID2D1DCRenderTarget_Impl: ID2D1RenderTarget_Impl {
    fn BindDC(&self, hdc: super::windef::HDC, psubrect: *const super::windef::RECT) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_d2dbasetypes", feature = "Win32_dcommon", feature = "Win32_dwrite", feature = "Win32_dxgiformat", feature = "Win32_dxgitype", feature = "Win32_wincodec", feature = "Win32_windef"))]
impl ID2D1DCRenderTarget_Vtbl {
    pub const fn new<Identity: ID2D1DCRenderTarget_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn BindDC<Identity: ID2D1DCRenderTarget_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hdc: super::windef::HDC, psubrect: *const super::windef::RECT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID2D1DCRenderTarget_Impl::BindDC(this, core::mem::transmute_copy(&hdc), core::mem::transmute_copy(&psubrect)).into()
            }
        }
        Self { base__: ID2D1RenderTarget_Vtbl::new::<Identity, OFFSET>(), BindDC: BindDC::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ID2D1DCRenderTarget as windows_core::Interface>::IID || iid == &<ID2D1Resource as windows_core::Interface>::IID || iid == &<ID2D1RenderTarget as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_d2dbasetypes", feature = "Win32_dcommon", feature = "Win32_dwrite", feature = "Win32_dxgiformat", feature = "Win32_dxgitype", feature = "Win32_wincodec", feature = "Win32_windef"))]
impl windows_core::RuntimeName for ID2D1DCRenderTarget {}
windows_core::imp::define_interface!(ID2D1DrawingStateBlock, ID2D1DrawingStateBlock_Vtbl, 0x28506e39_ebf6_46a1_bb47_fd85565ab957);
impl core::ops::Deref for ID2D1DrawingStateBlock {
    type Target = ID2D1Resource;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ID2D1DrawingStateBlock, windows_core::IUnknown, ID2D1Resource);
impl ID2D1DrawingStateBlock {
    pub unsafe fn GetDescription(&self, statedescription: *mut D2D1_DRAWING_STATE_DESCRIPTION) {
        unsafe {
            (windows_core::Interface::vtable(self).GetDescription)(windows_core::Interface::as_raw(self), statedescription as _);
        }
    }
    pub unsafe fn SetDescription(&self, statedescription: *const D2D1_DRAWING_STATE_DESCRIPTION) {
        unsafe {
            (windows_core::Interface::vtable(self).SetDescription)(windows_core::Interface::as_raw(self), statedescription);
        }
    }
    #[cfg(feature = "Win32_dwrite")]
    pub unsafe fn SetTextRenderingParams<P0>(&self, textrenderingparams: P0)
    where
        P0: windows_core::Param<super::dwrite::IDWriteRenderingParams>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).SetTextRenderingParams)(windows_core::Interface::as_raw(self), textrenderingparams.param().abi());
        }
    }
    #[cfg(feature = "Win32_dwrite")]
    pub unsafe fn GetTextRenderingParams(&self) -> windows_core::Result<super::dwrite::IDWriteRenderingParams> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetTextRenderingParams)(windows_core::Interface::as_raw(self), &mut result__);
            windows_core::Type::from_abi(result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ID2D1DrawingStateBlock_Vtbl {
    pub base__: ID2D1Resource_Vtbl,
    pub GetDescription: unsafe extern "system" fn(*mut core::ffi::c_void, *mut D2D1_DRAWING_STATE_DESCRIPTION),
    pub SetDescription: unsafe extern "system" fn(*mut core::ffi::c_void, *const D2D1_DRAWING_STATE_DESCRIPTION),
    #[cfg(feature = "Win32_dwrite")]
    pub SetTextRenderingParams: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void),
    #[cfg(not(feature = "Win32_dwrite"))]
    SetTextRenderingParams: usize,
    #[cfg(feature = "Win32_dwrite")]
    pub GetTextRenderingParams: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void),
    #[cfg(not(feature = "Win32_dwrite"))]
    GetTextRenderingParams: usize,
}
#[cfg(feature = "Win32_dwrite")]
pub trait ID2D1DrawingStateBlock_Impl: ID2D1Resource_Impl {
    fn GetDescription(&self, statedescription: *mut D2D1_DRAWING_STATE_DESCRIPTION);
    fn SetDescription(&self, statedescription: *const D2D1_DRAWING_STATE_DESCRIPTION);
    fn SetTextRenderingParams(&self, textrenderingparams: windows_core::Ref<super::dwrite::IDWriteRenderingParams>);
    fn GetTextRenderingParams(&self, textrenderingparams: windows_core::OutRef<super::dwrite::IDWriteRenderingParams>);
}
#[cfg(feature = "Win32_dwrite")]
impl ID2D1DrawingStateBlock_Vtbl {
    pub const fn new<Identity: ID2D1DrawingStateBlock_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetDescription<Identity: ID2D1DrawingStateBlock_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, statedescription: *mut D2D1_DRAWING_STATE_DESCRIPTION) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID2D1DrawingStateBlock_Impl::GetDescription(this, core::mem::transmute_copy(&statedescription));
            }
        }
        unsafe extern "system" fn SetDescription<Identity: ID2D1DrawingStateBlock_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, statedescription: *const D2D1_DRAWING_STATE_DESCRIPTION) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID2D1DrawingStateBlock_Impl::SetDescription(this, core::mem::transmute_copy(&statedescription));
            }
        }
        unsafe extern "system" fn SetTextRenderingParams<Identity: ID2D1DrawingStateBlock_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, textrenderingparams: *mut core::ffi::c_void) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID2D1DrawingStateBlock_Impl::SetTextRenderingParams(this, core::mem::transmute_copy(&textrenderingparams));
            }
        }
        unsafe extern "system" fn GetTextRenderingParams<Identity: ID2D1DrawingStateBlock_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, textrenderingparams: *mut *mut core::ffi::c_void) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID2D1DrawingStateBlock_Impl::GetTextRenderingParams(this, core::mem::transmute_copy(&textrenderingparams));
            }
        }
        Self {
            base__: ID2D1Resource_Vtbl::new::<Identity, OFFSET>(),
            GetDescription: GetDescription::<Identity, OFFSET>,
            SetDescription: SetDescription::<Identity, OFFSET>,
            SetTextRenderingParams: SetTextRenderingParams::<Identity, OFFSET>,
            GetTextRenderingParams: GetTextRenderingParams::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ID2D1DrawingStateBlock as windows_core::Interface>::IID || iid == &<ID2D1Resource as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_dwrite")]
impl windows_core::RuntimeName for ID2D1DrawingStateBlock {}
windows_core::imp::define_interface!(ID2D1EllipseGeometry, ID2D1EllipseGeometry_Vtbl, 0x2cd906a4_12e2_11dc_9fed_001143a055f9);
impl core::ops::Deref for ID2D1EllipseGeometry {
    type Target = ID2D1Geometry;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ID2D1EllipseGeometry, windows_core::IUnknown, ID2D1Resource, ID2D1Geometry);
impl ID2D1EllipseGeometry {
    pub unsafe fn GetEllipse(&self) -> D2D1_ELLIPSE {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetEllipse)(windows_core::Interface::as_raw(self), &mut result__);
            result__
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ID2D1EllipseGeometry_Vtbl {
    pub base__: ID2D1Geometry_Vtbl,
    pub GetEllipse: unsafe extern "system" fn(*mut core::ffi::c_void, *mut D2D1_ELLIPSE),
}
#[cfg(feature = "Win32_dcommon")]
pub trait ID2D1EllipseGeometry_Impl: ID2D1Geometry_Impl {
    fn GetEllipse(&self, ellipse: *mut D2D1_ELLIPSE);
}
#[cfg(feature = "Win32_dcommon")]
impl ID2D1EllipseGeometry_Vtbl {
    pub const fn new<Identity: ID2D1EllipseGeometry_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetEllipse<Identity: ID2D1EllipseGeometry_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ellipse: *mut D2D1_ELLIPSE) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID2D1EllipseGeometry_Impl::GetEllipse(this, core::mem::transmute_copy(&ellipse));
            }
        }
        Self { base__: ID2D1Geometry_Vtbl::new::<Identity, OFFSET>(), GetEllipse: GetEllipse::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ID2D1EllipseGeometry as windows_core::Interface>::IID || iid == &<ID2D1Resource as windows_core::Interface>::IID || iid == &<ID2D1Geometry as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_dcommon")]
impl windows_core::RuntimeName for ID2D1EllipseGeometry {}
windows_core::imp::define_interface!(ID2D1Factory, ID2D1Factory_Vtbl, 0x06152247_6f50_465a_9245_118bfd3b6007);
windows_core::imp::interface_hierarchy!(ID2D1Factory, windows_core::IUnknown);
impl ID2D1Factory {
    pub unsafe fn ReloadSystemMetrics(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).ReloadSystemMetrics)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn GetDesktopDpi(&self, dpix: *mut f32, dpiy: *mut f32) {
        unsafe {
            (windows_core::Interface::vtable(self).GetDesktopDpi)(windows_core::Interface::as_raw(self), dpix as _, dpiy as _);
        }
    }
    #[cfg(feature = "Win32_dcommon")]
    pub unsafe fn CreateRectangleGeometry(&self, rectangle: *const super::dcommon::D2D_RECT_F) -> windows_core::Result<ID2D1RectangleGeometry> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateRectangleGeometry)(windows_core::Interface::as_raw(self), rectangle, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Win32_dcommon")]
    pub unsafe fn CreateRoundedRectangleGeometry(&self, roundedrectangle: *const D2D1_ROUNDED_RECT) -> windows_core::Result<ID2D1RoundedRectangleGeometry> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateRoundedRectangleGeometry)(windows_core::Interface::as_raw(self), roundedrectangle, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn CreateEllipseGeometry(&self, ellipse: *const D2D1_ELLIPSE) -> windows_core::Result<ID2D1EllipseGeometry> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateEllipseGeometry)(windows_core::Interface::as_raw(self), ellipse, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn CreateGeometryGroup(&self, fillmode: D2D1_FILL_MODE, geometries: &[Option<ID2D1Geometry>]) -> windows_core::Result<ID2D1GeometryGroup> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateGeometryGroup)(windows_core::Interface::as_raw(self), fillmode, core::mem::transmute(geometries.as_ptr()), geometries.len().try_into().unwrap(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn CreateTransformedGeometry<P0>(&self, sourcegeometry: P0, transform: *const windows_numerics::Matrix3x2) -> windows_core::Result<ID2D1TransformedGeometry>
    where
        P0: windows_core::Param<ID2D1Geometry>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateTransformedGeometry)(windows_core::Interface::as_raw(self), sourcegeometry.param().abi(), transform, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn CreatePathGeometry(&self) -> windows_core::Result<ID2D1PathGeometry> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreatePathGeometry)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn CreateStrokeStyle(&self, strokestyleproperties: *const D2D1_STROKE_STYLE_PROPERTIES, dashes: Option<&[f32]>) -> windows_core::Result<ID2D1StrokeStyle> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateStrokeStyle)(windows_core::Interface::as_raw(self), strokestyleproperties, core::mem::transmute(dashes.map_or(core::ptr::null(), |slice| slice.as_ptr())), dashes.map_or(0, |slice| slice.len().try_into().unwrap()), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Win32_dwrite")]
    pub unsafe fn CreateDrawingStateBlock<P1>(&self, drawingstatedescription: Option<*const D2D1_DRAWING_STATE_DESCRIPTION>, textrenderingparams: P1) -> windows_core::Result<ID2D1DrawingStateBlock>
    where
        P1: windows_core::Param<super::dwrite::IDWriteRenderingParams>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateDrawingStateBlock)(windows_core::Interface::as_raw(self), drawingstatedescription.unwrap_or(core::mem::zeroed()) as _, textrenderingparams.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(all(feature = "Win32_dcommon", feature = "Win32_dxgiformat", feature = "Win32_wincodec"))]
    pub unsafe fn CreateWicBitmapRenderTarget<P0>(&self, target: P0, rendertargetproperties: *const D2D1_RENDER_TARGET_PROPERTIES) -> windows_core::Result<ID2D1RenderTarget>
    where
        P0: windows_core::Param<super::wincodec::IWICBitmap>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateWicBitmapRenderTarget)(windows_core::Interface::as_raw(self), target.param().abi(), rendertargetproperties, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(all(feature = "Win32_dcommon", feature = "Win32_dxgiformat", feature = "Win32_windef"))]
    pub unsafe fn CreateHwndRenderTarget(&self, rendertargetproperties: *const D2D1_RENDER_TARGET_PROPERTIES, hwndrendertargetproperties: *const D2D1_HWND_RENDER_TARGET_PROPERTIES) -> windows_core::Result<ID2D1HwndRenderTarget> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateHwndRenderTarget)(windows_core::Interface::as_raw(self), rendertargetproperties, hwndrendertargetproperties, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(all(feature = "Win32_dcommon", feature = "Win32_dxgi", feature = "Win32_dxgiformat"))]
    pub unsafe fn CreateDxgiSurfaceRenderTarget<P0>(&self, dxgisurface: P0, rendertargetproperties: *const D2D1_RENDER_TARGET_PROPERTIES) -> windows_core::Result<ID2D1RenderTarget>
    where
        P0: windows_core::Param<super::dxgi::IDXGISurface>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateDxgiSurfaceRenderTarget)(windows_core::Interface::as_raw(self), dxgisurface.param().abi(), rendertargetproperties, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(all(feature = "Win32_dcommon", feature = "Win32_dxgiformat"))]
    pub unsafe fn CreateDCRenderTarget(&self, rendertargetproperties: *const D2D1_RENDER_TARGET_PROPERTIES) -> windows_core::Result<ID2D1DCRenderTarget> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateDCRenderTarget)(windows_core::Interface::as_raw(self), rendertargetproperties, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ID2D1Factory_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub ReloadSystemMetrics: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetDesktopDpi: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f32, *mut f32),
    #[cfg(feature = "Win32_dcommon")]
    pub CreateRectangleGeometry: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::dcommon::D2D_RECT_F, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_dcommon"))]
    CreateRectangleGeometry: usize,
    #[cfg(feature = "Win32_dcommon")]
    pub CreateRoundedRectangleGeometry: unsafe extern "system" fn(*mut core::ffi::c_void, *const D2D1_ROUNDED_RECT, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_dcommon"))]
    CreateRoundedRectangleGeometry: usize,
    pub CreateEllipseGeometry: unsafe extern "system" fn(*mut core::ffi::c_void, *const D2D1_ELLIPSE, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateGeometryGroup: unsafe extern "system" fn(*mut core::ffi::c_void, D2D1_FILL_MODE, *const *mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateTransformedGeometry: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *const windows_numerics::Matrix3x2, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreatePathGeometry: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateStrokeStyle: unsafe extern "system" fn(*mut core::ffi::c_void, *const D2D1_STROKE_STYLE_PROPERTIES, *const f32, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_dwrite")]
    pub CreateDrawingStateBlock: unsafe extern "system" fn(*mut core::ffi::c_void, *const D2D1_DRAWING_STATE_DESCRIPTION, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_dwrite"))]
    CreateDrawingStateBlock: usize,
    #[cfg(all(feature = "Win32_dcommon", feature = "Win32_dxgiformat", feature = "Win32_wincodec"))]
    pub CreateWicBitmapRenderTarget: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *const D2D1_RENDER_TARGET_PROPERTIES, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_dcommon", feature = "Win32_dxgiformat", feature = "Win32_wincodec")))]
    CreateWicBitmapRenderTarget: usize,
    #[cfg(all(feature = "Win32_dcommon", feature = "Win32_dxgiformat", feature = "Win32_windef"))]
    pub CreateHwndRenderTarget: unsafe extern "system" fn(*mut core::ffi::c_void, *const D2D1_RENDER_TARGET_PROPERTIES, *const D2D1_HWND_RENDER_TARGET_PROPERTIES, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_dcommon", feature = "Win32_dxgiformat", feature = "Win32_windef")))]
    CreateHwndRenderTarget: usize,
    #[cfg(all(feature = "Win32_dcommon", feature = "Win32_dxgi", feature = "Win32_dxgiformat"))]
    pub CreateDxgiSurfaceRenderTarget: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *const D2D1_RENDER_TARGET_PROPERTIES, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_dcommon", feature = "Win32_dxgi", feature = "Win32_dxgiformat")))]
    CreateDxgiSurfaceRenderTarget: usize,
    #[cfg(all(feature = "Win32_dcommon", feature = "Win32_dxgiformat"))]
    pub CreateDCRenderTarget: unsafe extern "system" fn(*mut core::ffi::c_void, *const D2D1_RENDER_TARGET_PROPERTIES, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_dcommon", feature = "Win32_dxgiformat")))]
    CreateDCRenderTarget: usize,
}
#[cfg(all(feature = "Win32_dcommon", feature = "Win32_dwrite", feature = "Win32_dxgi", feature = "Win32_dxgiformat", feature = "Win32_wincodec", feature = "Win32_windef"))]
pub trait ID2D1Factory_Impl: windows_core::IUnknownImpl {
    fn ReloadSystemMetrics(&self) -> windows_core::Result<()>;
    fn GetDesktopDpi(&self, dpix: *mut f32, dpiy: *mut f32);
    fn CreateRectangleGeometry(&self, rectangle: *const super::dcommon::D2D_RECT_F) -> windows_core::Result<ID2D1RectangleGeometry>;
    fn CreateRoundedRectangleGeometry(&self, roundedrectangle: *const D2D1_ROUNDED_RECT) -> windows_core::Result<ID2D1RoundedRectangleGeometry>;
    fn CreateEllipseGeometry(&self, ellipse: *const D2D1_ELLIPSE) -> windows_core::Result<ID2D1EllipseGeometry>;
    fn CreateGeometryGroup(&self, fillmode: D2D1_FILL_MODE, geometries: *const Option<ID2D1Geometry>, geometriescount: u32) -> windows_core::Result<ID2D1GeometryGroup>;
    fn CreateTransformedGeometry(&self, sourcegeometry: windows_core::Ref<ID2D1Geometry>, transform: *const windows_numerics::Matrix3x2) -> windows_core::Result<ID2D1TransformedGeometry>;
    fn CreatePathGeometry(&self) -> windows_core::Result<ID2D1PathGeometry>;
    fn CreateStrokeStyle(&self, strokestyleproperties: *const D2D1_STROKE_STYLE_PROPERTIES, dashes: *const f32, dashescount: u32) -> windows_core::Result<ID2D1StrokeStyle>;
    fn CreateDrawingStateBlock(&self, drawingstatedescription: *const D2D1_DRAWING_STATE_DESCRIPTION, textrenderingparams: windows_core::Ref<super::dwrite::IDWriteRenderingParams>) -> windows_core::Result<ID2D1DrawingStateBlock>;
    fn CreateWicBitmapRenderTarget(&self, target: windows_core::Ref<super::wincodec::IWICBitmap>, rendertargetproperties: *const D2D1_RENDER_TARGET_PROPERTIES) -> windows_core::Result<ID2D1RenderTarget>;
    fn CreateHwndRenderTarget(&self, rendertargetproperties: *const D2D1_RENDER_TARGET_PROPERTIES, hwndrendertargetproperties: *const D2D1_HWND_RENDER_TARGET_PROPERTIES) -> windows_core::Result<ID2D1HwndRenderTarget>;
    fn CreateDxgiSurfaceRenderTarget(&self, dxgisurface: windows_core::Ref<super::dxgi::IDXGISurface>, rendertargetproperties: *const D2D1_RENDER_TARGET_PROPERTIES) -> windows_core::Result<ID2D1RenderTarget>;
    fn CreateDCRenderTarget(&self, rendertargetproperties: *const D2D1_RENDER_TARGET_PROPERTIES) -> windows_core::Result<ID2D1DCRenderTarget>;
}
#[cfg(all(feature = "Win32_dcommon", feature = "Win32_dwrite", feature = "Win32_dxgi", feature = "Win32_dxgiformat", feature = "Win32_wincodec", feature = "Win32_windef"))]
impl ID2D1Factory_Vtbl {
    pub const fn new<Identity: ID2D1Factory_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn ReloadSystemMetrics<Identity: ID2D1Factory_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID2D1Factory_Impl::ReloadSystemMetrics(this).into()
            }
        }
        unsafe extern "system" fn GetDesktopDpi<Identity: ID2D1Factory_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dpix: *mut f32, dpiy: *mut f32) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID2D1Factory_Impl::GetDesktopDpi(this, core::mem::transmute_copy(&dpix), core::mem::transmute_copy(&dpiy));
            }
        }
        unsafe extern "system" fn CreateRectangleGeometry<Identity: ID2D1Factory_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, rectangle: *const super::dcommon::D2D_RECT_F, rectanglegeometry: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ID2D1Factory_Impl::CreateRectangleGeometry(this, core::mem::transmute_copy(&rectangle)) {
                    Ok(ok__) => {
                        rectanglegeometry.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CreateRoundedRectangleGeometry<Identity: ID2D1Factory_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, roundedrectangle: *const D2D1_ROUNDED_RECT, roundedrectanglegeometry: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ID2D1Factory_Impl::CreateRoundedRectangleGeometry(this, core::mem::transmute_copy(&roundedrectangle)) {
                    Ok(ok__) => {
                        roundedrectanglegeometry.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CreateEllipseGeometry<Identity: ID2D1Factory_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ellipse: *const D2D1_ELLIPSE, ellipsegeometry: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ID2D1Factory_Impl::CreateEllipseGeometry(this, core::mem::transmute_copy(&ellipse)) {
                    Ok(ok__) => {
                        ellipsegeometry.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CreateGeometryGroup<Identity: ID2D1Factory_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, fillmode: D2D1_FILL_MODE, geometries: *const *mut core::ffi::c_void, geometriescount: u32, geometrygroup: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ID2D1Factory_Impl::CreateGeometryGroup(this, core::mem::transmute_copy(&fillmode), core::mem::transmute_copy(&geometries), core::mem::transmute_copy(&geometriescount)) {
                    Ok(ok__) => {
                        geometrygroup.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CreateTransformedGeometry<Identity: ID2D1Factory_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, sourcegeometry: *mut core::ffi::c_void, transform: *const windows_numerics::Matrix3x2, transformedgeometry: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ID2D1Factory_Impl::CreateTransformedGeometry(this, core::mem::transmute_copy(&sourcegeometry), core::mem::transmute_copy(&transform)) {
                    Ok(ok__) => {
                        transformedgeometry.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CreatePathGeometry<Identity: ID2D1Factory_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pathgeometry: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ID2D1Factory_Impl::CreatePathGeometry(this) {
                    Ok(ok__) => {
                        pathgeometry.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CreateStrokeStyle<Identity: ID2D1Factory_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strokestyleproperties: *const D2D1_STROKE_STYLE_PROPERTIES, dashes: *const f32, dashescount: u32, strokestyle: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ID2D1Factory_Impl::CreateStrokeStyle(this, core::mem::transmute_copy(&strokestyleproperties), core::mem::transmute_copy(&dashes), core::mem::transmute_copy(&dashescount)) {
                    Ok(ok__) => {
                        strokestyle.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CreateDrawingStateBlock<Identity: ID2D1Factory_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, drawingstatedescription: *const D2D1_DRAWING_STATE_DESCRIPTION, textrenderingparams: *mut core::ffi::c_void, drawingstateblock: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ID2D1Factory_Impl::CreateDrawingStateBlock(this, core::mem::transmute_copy(&drawingstatedescription), core::mem::transmute_copy(&textrenderingparams)) {
                    Ok(ok__) => {
                        drawingstateblock.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CreateWicBitmapRenderTarget<Identity: ID2D1Factory_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, target: *mut core::ffi::c_void, rendertargetproperties: *const D2D1_RENDER_TARGET_PROPERTIES, rendertarget: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ID2D1Factory_Impl::CreateWicBitmapRenderTarget(this, core::mem::transmute_copy(&target), core::mem::transmute_copy(&rendertargetproperties)) {
                    Ok(ok__) => {
                        rendertarget.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CreateHwndRenderTarget<Identity: ID2D1Factory_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, rendertargetproperties: *const D2D1_RENDER_TARGET_PROPERTIES, hwndrendertargetproperties: *const D2D1_HWND_RENDER_TARGET_PROPERTIES, hwndrendertarget: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ID2D1Factory_Impl::CreateHwndRenderTarget(this, core::mem::transmute_copy(&rendertargetproperties), core::mem::transmute_copy(&hwndrendertargetproperties)) {
                    Ok(ok__) => {
                        hwndrendertarget.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CreateDxgiSurfaceRenderTarget<Identity: ID2D1Factory_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dxgisurface: *mut core::ffi::c_void, rendertargetproperties: *const D2D1_RENDER_TARGET_PROPERTIES, rendertarget: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ID2D1Factory_Impl::CreateDxgiSurfaceRenderTarget(this, core::mem::transmute_copy(&dxgisurface), core::mem::transmute_copy(&rendertargetproperties)) {
                    Ok(ok__) => {
                        rendertarget.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CreateDCRenderTarget<Identity: ID2D1Factory_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, rendertargetproperties: *const D2D1_RENDER_TARGET_PROPERTIES, dcrendertarget: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ID2D1Factory_Impl::CreateDCRenderTarget(this, core::mem::transmute_copy(&rendertargetproperties)) {
                    Ok(ok__) => {
                        dcrendertarget.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            ReloadSystemMetrics: ReloadSystemMetrics::<Identity, OFFSET>,
            GetDesktopDpi: GetDesktopDpi::<Identity, OFFSET>,
            CreateRectangleGeometry: CreateRectangleGeometry::<Identity, OFFSET>,
            CreateRoundedRectangleGeometry: CreateRoundedRectangleGeometry::<Identity, OFFSET>,
            CreateEllipseGeometry: CreateEllipseGeometry::<Identity, OFFSET>,
            CreateGeometryGroup: CreateGeometryGroup::<Identity, OFFSET>,
            CreateTransformedGeometry: CreateTransformedGeometry::<Identity, OFFSET>,
            CreatePathGeometry: CreatePathGeometry::<Identity, OFFSET>,
            CreateStrokeStyle: CreateStrokeStyle::<Identity, OFFSET>,
            CreateDrawingStateBlock: CreateDrawingStateBlock::<Identity, OFFSET>,
            CreateWicBitmapRenderTarget: CreateWicBitmapRenderTarget::<Identity, OFFSET>,
            CreateHwndRenderTarget: CreateHwndRenderTarget::<Identity, OFFSET>,
            CreateDxgiSurfaceRenderTarget: CreateDxgiSurfaceRenderTarget::<Identity, OFFSET>,
            CreateDCRenderTarget: CreateDCRenderTarget::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ID2D1Factory as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_dcommon", feature = "Win32_dwrite", feature = "Win32_dxgi", feature = "Win32_dxgiformat", feature = "Win32_wincodec", feature = "Win32_windef"))]
impl windows_core::RuntimeName for ID2D1Factory {}
windows_core::imp::define_interface!(ID2D1GdiInteropRenderTarget, ID2D1GdiInteropRenderTarget_Vtbl, 0xe0db51c3_6f77_4bae_b3d5_e47509b35838);
windows_core::imp::interface_hierarchy!(ID2D1GdiInteropRenderTarget, windows_core::IUnknown);
impl ID2D1GdiInteropRenderTarget {
    #[cfg(feature = "Win32_windef")]
    pub unsafe fn GetDC(&self, mode: D2D1_DC_INITIALIZE_MODE) -> windows_core::Result<super::windef::HDC> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetDC)(windows_core::Interface::as_raw(self), mode, &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Win32_windef")]
    pub unsafe fn ReleaseDC(&self, update: Option<*const super::windef::RECT>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).ReleaseDC)(windows_core::Interface::as_raw(self), update.unwrap_or(core::mem::zeroed()) as _) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ID2D1GdiInteropRenderTarget_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_windef")]
    pub GetDC: unsafe extern "system" fn(*mut core::ffi::c_void, D2D1_DC_INITIALIZE_MODE, *mut super::windef::HDC) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_windef"))]
    GetDC: usize,
    #[cfg(feature = "Win32_windef")]
    pub ReleaseDC: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::windef::RECT) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_windef"))]
    ReleaseDC: usize,
}
#[cfg(feature = "Win32_windef")]
pub trait ID2D1GdiInteropRenderTarget_Impl: windows_core::IUnknownImpl {
    fn GetDC(&self, mode: D2D1_DC_INITIALIZE_MODE) -> windows_core::Result<super::windef::HDC>;
    fn ReleaseDC(&self, update: *const super::windef::RECT) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_windef")]
impl ID2D1GdiInteropRenderTarget_Vtbl {
    pub const fn new<Identity: ID2D1GdiInteropRenderTarget_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetDC<Identity: ID2D1GdiInteropRenderTarget_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, mode: D2D1_DC_INITIALIZE_MODE, hdc: *mut super::windef::HDC) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ID2D1GdiInteropRenderTarget_Impl::GetDC(this, core::mem::transmute_copy(&mode)) {
                    Ok(ok__) => {
                        hdc.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn ReleaseDC<Identity: ID2D1GdiInteropRenderTarget_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, update: *const super::windef::RECT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID2D1GdiInteropRenderTarget_Impl::ReleaseDC(this, core::mem::transmute_copy(&update)).into()
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), GetDC: GetDC::<Identity, OFFSET>, ReleaseDC: ReleaseDC::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ID2D1GdiInteropRenderTarget as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_windef")]
impl windows_core::RuntimeName for ID2D1GdiInteropRenderTarget {}
windows_core::imp::define_interface!(ID2D1Geometry, ID2D1Geometry_Vtbl, 0x2cd906a1_12e2_11dc_9fed_001143a055f9);
impl core::ops::Deref for ID2D1Geometry {
    type Target = ID2D1Resource;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ID2D1Geometry, windows_core::IUnknown, ID2D1Resource);
impl ID2D1Geometry {
    #[cfg(feature = "Win32_dcommon")]
    pub unsafe fn GetBounds(&self, worldtransform: Option<*const windows_numerics::Matrix3x2>) -> windows_core::Result<super::dcommon::D2D_RECT_F> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetBounds)(windows_core::Interface::as_raw(self), worldtransform.unwrap_or(core::mem::zeroed()) as _, &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Win32_dcommon")]
    pub unsafe fn GetWidenedBounds<P1>(&self, strokewidth: f32, strokestyle: P1, worldtransform: Option<*const windows_numerics::Matrix3x2>, flatteningtolerance: f32) -> windows_core::Result<super::dcommon::D2D_RECT_F>
    where
        P1: windows_core::Param<ID2D1StrokeStyle>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetWidenedBounds)(windows_core::Interface::as_raw(self), strokewidth, strokestyle.param().abi(), worldtransform.unwrap_or(core::mem::zeroed()) as _, flatteningtolerance, &mut result__).map(|| result__)
        }
    }
    pub unsafe fn StrokeContainsPoint<P2>(&self, point: windows_numerics::Vector2, strokewidth: f32, strokestyle: P2, worldtransform: Option<*const windows_numerics::Matrix3x2>, flatteningtolerance: f32) -> windows_core::Result<windows_core::BOOL>
    where
        P2: windows_core::Param<ID2D1StrokeStyle>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).StrokeContainsPoint)(windows_core::Interface::as_raw(self), core::mem::transmute(point), strokewidth, strokestyle.param().abi(), worldtransform.unwrap_or(core::mem::zeroed()) as _, flatteningtolerance, &mut result__).map(|| result__)
        }
    }
    pub unsafe fn FillContainsPoint(&self, point: windows_numerics::Vector2, worldtransform: Option<*const windows_numerics::Matrix3x2>, flatteningtolerance: f32) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).FillContainsPoint)(windows_core::Interface::as_raw(self), core::mem::transmute(point), worldtransform.unwrap_or(core::mem::zeroed()) as _, flatteningtolerance, &mut result__).map(|| result__)
        }
    }
    pub unsafe fn CompareWithGeometry<P0>(&self, inputgeometry: P0, inputgeometrytransform: Option<*const windows_numerics::Matrix3x2>, flatteningtolerance: f32) -> windows_core::Result<D2D1_GEOMETRY_RELATION>
    where
        P0: windows_core::Param<Self>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CompareWithGeometry)(windows_core::Interface::as_raw(self), inputgeometry.param().abi(), inputgeometrytransform.unwrap_or(core::mem::zeroed()) as _, flatteningtolerance, &mut result__).map(|| result__)
        }
    }
    pub unsafe fn Simplify<P3>(&self, simplificationoption: D2D1_GEOMETRY_SIMPLIFICATION_OPTION, worldtransform: Option<*const windows_numerics::Matrix3x2>, flatteningtolerance: f32, geometrysink: P3) -> windows_core::HRESULT
    where
        P3: windows_core::Param<ID2D1SimplifiedGeometrySink>,
    {
        unsafe { (windows_core::Interface::vtable(self).Simplify)(windows_core::Interface::as_raw(self), simplificationoption, worldtransform.unwrap_or(core::mem::zeroed()) as _, flatteningtolerance, geometrysink.param().abi()) }
    }
    pub unsafe fn Tessellate<P2>(&self, worldtransform: Option<*const windows_numerics::Matrix3x2>, flatteningtolerance: f32, tessellationsink: P2) -> windows_core::HRESULT
    where
        P2: windows_core::Param<ID2D1TessellationSink>,
    {
        unsafe { (windows_core::Interface::vtable(self).Tessellate)(windows_core::Interface::as_raw(self), worldtransform.unwrap_or(core::mem::zeroed()) as _, flatteningtolerance, tessellationsink.param().abi()) }
    }
    pub unsafe fn CombineWithGeometry<P0, P4>(&self, inputgeometry: P0, combinemode: D2D1_COMBINE_MODE, inputgeometrytransform: Option<*const windows_numerics::Matrix3x2>, flatteningtolerance: f32, geometrysink: P4) -> windows_core::HRESULT
    where
        P0: windows_core::Param<Self>,
        P4: windows_core::Param<ID2D1SimplifiedGeometrySink>,
    {
        unsafe { (windows_core::Interface::vtable(self).CombineWithGeometry)(windows_core::Interface::as_raw(self), inputgeometry.param().abi(), combinemode, inputgeometrytransform.unwrap_or(core::mem::zeroed()) as _, flatteningtolerance, geometrysink.param().abi()) }
    }
    pub unsafe fn Outline<P2>(&self, worldtransform: Option<*const windows_numerics::Matrix3x2>, flatteningtolerance: f32, geometrysink: P2) -> windows_core::HRESULT
    where
        P2: windows_core::Param<ID2D1SimplifiedGeometrySink>,
    {
        unsafe { (windows_core::Interface::vtable(self).Outline)(windows_core::Interface::as_raw(self), worldtransform.unwrap_or(core::mem::zeroed()) as _, flatteningtolerance, geometrysink.param().abi()) }
    }
    pub unsafe fn ComputeArea(&self, worldtransform: Option<*const windows_numerics::Matrix3x2>, flatteningtolerance: f32) -> windows_core::Result<f32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ComputeArea)(windows_core::Interface::as_raw(self), worldtransform.unwrap_or(core::mem::zeroed()) as _, flatteningtolerance, &mut result__).map(|| result__)
        }
    }
    pub unsafe fn ComputeLength(&self, worldtransform: Option<*const windows_numerics::Matrix3x2>, flatteningtolerance: f32) -> windows_core::Result<f32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ComputeLength)(windows_core::Interface::as_raw(self), worldtransform.unwrap_or(core::mem::zeroed()) as _, flatteningtolerance, &mut result__).map(|| result__)
        }
    }
    pub unsafe fn ComputePointAtLength(&self, length: f32, worldtransform: Option<*const windows_numerics::Matrix3x2>, flatteningtolerance: f32, point: Option<*mut windows_numerics::Vector2>, unittangentvector: Option<*mut windows_numerics::Vector2>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).ComputePointAtLength)(windows_core::Interface::as_raw(self), length, worldtransform.unwrap_or(core::mem::zeroed()) as _, flatteningtolerance, point.unwrap_or(core::mem::zeroed()) as _, unittangentvector.unwrap_or(core::mem::zeroed()) as _) }
    }
    pub unsafe fn Widen<P1, P4>(&self, strokewidth: f32, strokestyle: P1, worldtransform: Option<*const windows_numerics::Matrix3x2>, flatteningtolerance: f32, geometrysink: P4) -> windows_core::HRESULT
    where
        P1: windows_core::Param<ID2D1StrokeStyle>,
        P4: windows_core::Param<ID2D1SimplifiedGeometrySink>,
    {
        unsafe { (windows_core::Interface::vtable(self).Widen)(windows_core::Interface::as_raw(self), strokewidth, strokestyle.param().abi(), worldtransform.unwrap_or(core::mem::zeroed()) as _, flatteningtolerance, geometrysink.param().abi()) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ID2D1Geometry_Vtbl {
    pub base__: ID2D1Resource_Vtbl,
    #[cfg(feature = "Win32_dcommon")]
    pub GetBounds: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_numerics::Matrix3x2, *mut super::dcommon::D2D_RECT_F) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_dcommon"))]
    GetBounds: usize,
    #[cfg(feature = "Win32_dcommon")]
    pub GetWidenedBounds: unsafe extern "system" fn(*mut core::ffi::c_void, f32, *mut core::ffi::c_void, *const windows_numerics::Matrix3x2, f32, *mut super::dcommon::D2D_RECT_F) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_dcommon"))]
    GetWidenedBounds: usize,
    pub StrokeContainsPoint: unsafe extern "system" fn(*mut core::ffi::c_void, windows_numerics::Vector2, f32, *mut core::ffi::c_void, *const windows_numerics::Matrix3x2, f32, *mut windows_core::BOOL) -> windows_core::HRESULT,
    pub FillContainsPoint: unsafe extern "system" fn(*mut core::ffi::c_void, windows_numerics::Vector2, *const windows_numerics::Matrix3x2, f32, *mut windows_core::BOOL) -> windows_core::HRESULT,
    pub CompareWithGeometry: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *const windows_numerics::Matrix3x2, f32, *mut D2D1_GEOMETRY_RELATION) -> windows_core::HRESULT,
    pub Simplify: unsafe extern "system" fn(*mut core::ffi::c_void, D2D1_GEOMETRY_SIMPLIFICATION_OPTION, *const windows_numerics::Matrix3x2, f32, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Tessellate: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_numerics::Matrix3x2, f32, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CombineWithGeometry: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, D2D1_COMBINE_MODE, *const windows_numerics::Matrix3x2, f32, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Outline: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_numerics::Matrix3x2, f32, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ComputeArea: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_numerics::Matrix3x2, f32, *mut f32) -> windows_core::HRESULT,
    pub ComputeLength: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_numerics::Matrix3x2, f32, *mut f32) -> windows_core::HRESULT,
    pub ComputePointAtLength: unsafe extern "system" fn(*mut core::ffi::c_void, f32, *const windows_numerics::Matrix3x2, f32, *mut windows_numerics::Vector2, *mut windows_numerics::Vector2) -> windows_core::HRESULT,
    pub Widen: unsafe extern "system" fn(*mut core::ffi::c_void, f32, *mut core::ffi::c_void, *const windows_numerics::Matrix3x2, f32, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_dcommon")]
pub trait ID2D1Geometry_Impl: ID2D1Resource_Impl {
    fn GetBounds(&self, worldtransform: *const windows_numerics::Matrix3x2) -> windows_core::Result<super::dcommon::D2D_RECT_F>;
    fn GetWidenedBounds(&self, strokewidth: f32, strokestyle: windows_core::Ref<ID2D1StrokeStyle>, worldtransform: *const windows_numerics::Matrix3x2, flatteningtolerance: f32) -> windows_core::Result<super::dcommon::D2D_RECT_F>;
    fn StrokeContainsPoint(&self, point: &windows_numerics::Vector2, strokewidth: f32, strokestyle: windows_core::Ref<ID2D1StrokeStyle>, worldtransform: *const windows_numerics::Matrix3x2, flatteningtolerance: f32) -> windows_core::Result<windows_core::BOOL>;
    fn FillContainsPoint(&self, point: &windows_numerics::Vector2, worldtransform: *const windows_numerics::Matrix3x2, flatteningtolerance: f32) -> windows_core::Result<windows_core::BOOL>;
    fn CompareWithGeometry(&self, inputgeometry: windows_core::Ref<ID2D1Geometry>, inputgeometrytransform: *const windows_numerics::Matrix3x2, flatteningtolerance: f32) -> windows_core::Result<D2D1_GEOMETRY_RELATION>;
    fn Simplify(&self, simplificationoption: D2D1_GEOMETRY_SIMPLIFICATION_OPTION, worldtransform: *const windows_numerics::Matrix3x2, flatteningtolerance: f32, geometrysink: windows_core::Ref<ID2D1SimplifiedGeometrySink>) -> windows_core::Result<()>;
    fn Tessellate(&self, worldtransform: *const windows_numerics::Matrix3x2, flatteningtolerance: f32, tessellationsink: windows_core::Ref<ID2D1TessellationSink>) -> windows_core::Result<()>;
    fn CombineWithGeometry(&self, inputgeometry: windows_core::Ref<ID2D1Geometry>, combinemode: D2D1_COMBINE_MODE, inputgeometrytransform: *const windows_numerics::Matrix3x2, flatteningtolerance: f32, geometrysink: windows_core::Ref<ID2D1SimplifiedGeometrySink>) -> windows_core::Result<()>;
    fn Outline(&self, worldtransform: *const windows_numerics::Matrix3x2, flatteningtolerance: f32, geometrysink: windows_core::Ref<ID2D1SimplifiedGeometrySink>) -> windows_core::Result<()>;
    fn ComputeArea(&self, worldtransform: *const windows_numerics::Matrix3x2, flatteningtolerance: f32) -> windows_core::Result<f32>;
    fn ComputeLength(&self, worldtransform: *const windows_numerics::Matrix3x2, flatteningtolerance: f32) -> windows_core::Result<f32>;
    fn ComputePointAtLength(&self, length: f32, worldtransform: *const windows_numerics::Matrix3x2, flatteningtolerance: f32, point: *mut windows_numerics::Vector2, unittangentvector: *mut windows_numerics::Vector2) -> windows_core::Result<()>;
    fn Widen(&self, strokewidth: f32, strokestyle: windows_core::Ref<ID2D1StrokeStyle>, worldtransform: *const windows_numerics::Matrix3x2, flatteningtolerance: f32, geometrysink: windows_core::Ref<ID2D1SimplifiedGeometrySink>) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_dcommon")]
impl ID2D1Geometry_Vtbl {
    pub const fn new<Identity: ID2D1Geometry_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetBounds<Identity: ID2D1Geometry_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, worldtransform: *const windows_numerics::Matrix3x2, bounds: *mut super::dcommon::D2D_RECT_F) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ID2D1Geometry_Impl::GetBounds(this, core::mem::transmute_copy(&worldtransform)) {
                    Ok(ok__) => {
                        bounds.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetWidenedBounds<Identity: ID2D1Geometry_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strokewidth: f32, strokestyle: *mut core::ffi::c_void, worldtransform: *const windows_numerics::Matrix3x2, flatteningtolerance: f32, bounds: *mut super::dcommon::D2D_RECT_F) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ID2D1Geometry_Impl::GetWidenedBounds(this, core::mem::transmute_copy(&strokewidth), core::mem::transmute_copy(&strokestyle), core::mem::transmute_copy(&worldtransform), core::mem::transmute_copy(&flatteningtolerance)) {
                    Ok(ok__) => {
                        bounds.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn StrokeContainsPoint<Identity: ID2D1Geometry_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, point: windows_numerics::Vector2, strokewidth: f32, strokestyle: *mut core::ffi::c_void, worldtransform: *const windows_numerics::Matrix3x2, flatteningtolerance: f32, contains: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ID2D1Geometry_Impl::StrokeContainsPoint(this, core::mem::transmute(&point), core::mem::transmute_copy(&strokewidth), core::mem::transmute_copy(&strokestyle), core::mem::transmute_copy(&worldtransform), core::mem::transmute_copy(&flatteningtolerance)) {
                    Ok(ok__) => {
                        contains.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn FillContainsPoint<Identity: ID2D1Geometry_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, point: windows_numerics::Vector2, worldtransform: *const windows_numerics::Matrix3x2, flatteningtolerance: f32, contains: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ID2D1Geometry_Impl::FillContainsPoint(this, core::mem::transmute(&point), core::mem::transmute_copy(&worldtransform), core::mem::transmute_copy(&flatteningtolerance)) {
                    Ok(ok__) => {
                        contains.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CompareWithGeometry<Identity: ID2D1Geometry_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, inputgeometry: *mut core::ffi::c_void, inputgeometrytransform: *const windows_numerics::Matrix3x2, flatteningtolerance: f32, relation: *mut D2D1_GEOMETRY_RELATION) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ID2D1Geometry_Impl::CompareWithGeometry(this, core::mem::transmute_copy(&inputgeometry), core::mem::transmute_copy(&inputgeometrytransform), core::mem::transmute_copy(&flatteningtolerance)) {
                    Ok(ok__) => {
                        relation.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Simplify<Identity: ID2D1Geometry_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, simplificationoption: D2D1_GEOMETRY_SIMPLIFICATION_OPTION, worldtransform: *const windows_numerics::Matrix3x2, flatteningtolerance: f32, geometrysink: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID2D1Geometry_Impl::Simplify(this, core::mem::transmute_copy(&simplificationoption), core::mem::transmute_copy(&worldtransform), core::mem::transmute_copy(&flatteningtolerance), core::mem::transmute_copy(&geometrysink)).into()
            }
        }
        unsafe extern "system" fn Tessellate<Identity: ID2D1Geometry_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, worldtransform: *const windows_numerics::Matrix3x2, flatteningtolerance: f32, tessellationsink: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID2D1Geometry_Impl::Tessellate(this, core::mem::transmute_copy(&worldtransform), core::mem::transmute_copy(&flatteningtolerance), core::mem::transmute_copy(&tessellationsink)).into()
            }
        }
        unsafe extern "system" fn CombineWithGeometry<Identity: ID2D1Geometry_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, inputgeometry: *mut core::ffi::c_void, combinemode: D2D1_COMBINE_MODE, inputgeometrytransform: *const windows_numerics::Matrix3x2, flatteningtolerance: f32, geometrysink: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID2D1Geometry_Impl::CombineWithGeometry(this, core::mem::transmute_copy(&inputgeometry), core::mem::transmute_copy(&combinemode), core::mem::transmute_copy(&inputgeometrytransform), core::mem::transmute_copy(&flatteningtolerance), core::mem::transmute_copy(&geometrysink)).into()
            }
        }
        unsafe extern "system" fn Outline<Identity: ID2D1Geometry_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, worldtransform: *const windows_numerics::Matrix3x2, flatteningtolerance: f32, geometrysink: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID2D1Geometry_Impl::Outline(this, core::mem::transmute_copy(&worldtransform), core::mem::transmute_copy(&flatteningtolerance), core::mem::transmute_copy(&geometrysink)).into()
            }
        }
        unsafe extern "system" fn ComputeArea<Identity: ID2D1Geometry_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, worldtransform: *const windows_numerics::Matrix3x2, flatteningtolerance: f32, area: *mut f32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ID2D1Geometry_Impl::ComputeArea(this, core::mem::transmute_copy(&worldtransform), core::mem::transmute_copy(&flatteningtolerance)) {
                    Ok(ok__) => {
                        area.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn ComputeLength<Identity: ID2D1Geometry_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, worldtransform: *const windows_numerics::Matrix3x2, flatteningtolerance: f32, length: *mut f32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ID2D1Geometry_Impl::ComputeLength(this, core::mem::transmute_copy(&worldtransform), core::mem::transmute_copy(&flatteningtolerance)) {
                    Ok(ok__) => {
                        length.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn ComputePointAtLength<Identity: ID2D1Geometry_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, length: f32, worldtransform: *const windows_numerics::Matrix3x2, flatteningtolerance: f32, point: *mut windows_numerics::Vector2, unittangentvector: *mut windows_numerics::Vector2) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID2D1Geometry_Impl::ComputePointAtLength(this, core::mem::transmute_copy(&length), core::mem::transmute_copy(&worldtransform), core::mem::transmute_copy(&flatteningtolerance), core::mem::transmute_copy(&point), core::mem::transmute_copy(&unittangentvector)).into()
            }
        }
        unsafe extern "system" fn Widen<Identity: ID2D1Geometry_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strokewidth: f32, strokestyle: *mut core::ffi::c_void, worldtransform: *const windows_numerics::Matrix3x2, flatteningtolerance: f32, geometrysink: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID2D1Geometry_Impl::Widen(this, core::mem::transmute_copy(&strokewidth), core::mem::transmute_copy(&strokestyle), core::mem::transmute_copy(&worldtransform), core::mem::transmute_copy(&flatteningtolerance), core::mem::transmute_copy(&geometrysink)).into()
            }
        }
        Self {
            base__: ID2D1Resource_Vtbl::new::<Identity, OFFSET>(),
            GetBounds: GetBounds::<Identity, OFFSET>,
            GetWidenedBounds: GetWidenedBounds::<Identity, OFFSET>,
            StrokeContainsPoint: StrokeContainsPoint::<Identity, OFFSET>,
            FillContainsPoint: FillContainsPoint::<Identity, OFFSET>,
            CompareWithGeometry: CompareWithGeometry::<Identity, OFFSET>,
            Simplify: Simplify::<Identity, OFFSET>,
            Tessellate: Tessellate::<Identity, OFFSET>,
            CombineWithGeometry: CombineWithGeometry::<Identity, OFFSET>,
            Outline: Outline::<Identity, OFFSET>,
            ComputeArea: ComputeArea::<Identity, OFFSET>,
            ComputeLength: ComputeLength::<Identity, OFFSET>,
            ComputePointAtLength: ComputePointAtLength::<Identity, OFFSET>,
            Widen: Widen::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ID2D1Geometry as windows_core::Interface>::IID || iid == &<ID2D1Resource as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_dcommon")]
impl windows_core::RuntimeName for ID2D1Geometry {}
windows_core::imp::define_interface!(ID2D1GeometryGroup, ID2D1GeometryGroup_Vtbl, 0x2cd906a6_12e2_11dc_9fed_001143a055f9);
impl core::ops::Deref for ID2D1GeometryGroup {
    type Target = ID2D1Geometry;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ID2D1GeometryGroup, windows_core::IUnknown, ID2D1Resource, ID2D1Geometry);
impl ID2D1GeometryGroup {
    pub unsafe fn GetFillMode(&self) -> D2D1_FILL_MODE {
        unsafe { (windows_core::Interface::vtable(self).GetFillMode)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn GetSourceGeometryCount(&self) -> u32 {
        unsafe { (windows_core::Interface::vtable(self).GetSourceGeometryCount)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn GetSourceGeometries(&self, geometries: &mut [Option<ID2D1Geometry>]) {
        unsafe {
            (windows_core::Interface::vtable(self).GetSourceGeometries)(windows_core::Interface::as_raw(self), core::mem::transmute(geometries.as_ptr()), geometries.len().try_into().unwrap());
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ID2D1GeometryGroup_Vtbl {
    pub base__: ID2D1Geometry_Vtbl,
    pub GetFillMode: unsafe extern "system" fn(*mut core::ffi::c_void) -> D2D1_FILL_MODE,
    pub GetSourceGeometryCount: unsafe extern "system" fn(*mut core::ffi::c_void) -> u32,
    pub GetSourceGeometries: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void, u32),
}
#[cfg(feature = "Win32_dcommon")]
pub trait ID2D1GeometryGroup_Impl: ID2D1Geometry_Impl {
    fn GetFillMode(&self) -> D2D1_FILL_MODE;
    fn GetSourceGeometryCount(&self) -> u32;
    fn GetSourceGeometries(&self, geometries: *mut Option<ID2D1Geometry>, geometriescount: u32);
}
#[cfg(feature = "Win32_dcommon")]
impl ID2D1GeometryGroup_Vtbl {
    pub const fn new<Identity: ID2D1GeometryGroup_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetFillMode<Identity: ID2D1GeometryGroup_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> D2D1_FILL_MODE {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID2D1GeometryGroup_Impl::GetFillMode(this)
            }
        }
        unsafe extern "system" fn GetSourceGeometryCount<Identity: ID2D1GeometryGroup_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> u32 {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID2D1GeometryGroup_Impl::GetSourceGeometryCount(this)
            }
        }
        unsafe extern "system" fn GetSourceGeometries<Identity: ID2D1GeometryGroup_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, geometries: *mut *mut core::ffi::c_void, geometriescount: u32) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID2D1GeometryGroup_Impl::GetSourceGeometries(this, core::mem::transmute_copy(&geometries), core::mem::transmute_copy(&geometriescount));
            }
        }
        Self {
            base__: ID2D1Geometry_Vtbl::new::<Identity, OFFSET>(),
            GetFillMode: GetFillMode::<Identity, OFFSET>,
            GetSourceGeometryCount: GetSourceGeometryCount::<Identity, OFFSET>,
            GetSourceGeometries: GetSourceGeometries::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ID2D1GeometryGroup as windows_core::Interface>::IID || iid == &<ID2D1Resource as windows_core::Interface>::IID || iid == &<ID2D1Geometry as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_dcommon")]
impl windows_core::RuntimeName for ID2D1GeometryGroup {}
windows_core::imp::define_interface!(ID2D1GeometrySink, ID2D1GeometrySink_Vtbl, 0x2cd9069f_12e2_11dc_9fed_001143a055f9);
impl core::ops::Deref for ID2D1GeometrySink {
    type Target = ID2D1SimplifiedGeometrySink;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ID2D1GeometrySink, windows_core::IUnknown, ID2D1SimplifiedGeometrySink);
impl ID2D1GeometrySink {
    pub unsafe fn AddLine(&self, point: windows_numerics::Vector2) {
        unsafe {
            (windows_core::Interface::vtable(self).AddLine)(windows_core::Interface::as_raw(self), core::mem::transmute(point));
        }
    }
    pub unsafe fn AddBezier(&self, bezier: *const D2D1_BEZIER_SEGMENT) {
        unsafe {
            (windows_core::Interface::vtable(self).AddBezier)(windows_core::Interface::as_raw(self), bezier);
        }
    }
    pub unsafe fn AddQuadraticBezier(&self, bezier: *const D2D1_QUADRATIC_BEZIER_SEGMENT) {
        unsafe {
            (windows_core::Interface::vtable(self).AddQuadraticBezier)(windows_core::Interface::as_raw(self), bezier);
        }
    }
    pub unsafe fn AddQuadraticBeziers(&self, beziers: &[D2D1_QUADRATIC_BEZIER_SEGMENT]) {
        unsafe {
            (windows_core::Interface::vtable(self).AddQuadraticBeziers)(windows_core::Interface::as_raw(self), core::mem::transmute(beziers.as_ptr()), beziers.len().try_into().unwrap());
        }
    }
    #[cfg(feature = "Win32_dcommon")]
    pub unsafe fn AddArc(&self, arc: *const D2D1_ARC_SEGMENT) {
        unsafe {
            (windows_core::Interface::vtable(self).AddArc)(windows_core::Interface::as_raw(self), arc);
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ID2D1GeometrySink_Vtbl {
    pub base__: ID2D1SimplifiedGeometrySink_Vtbl,
    pub AddLine: unsafe extern "system" fn(*mut core::ffi::c_void, windows_numerics::Vector2),
    pub AddBezier: unsafe extern "system" fn(*mut core::ffi::c_void, *const D2D1_BEZIER_SEGMENT),
    pub AddQuadraticBezier: unsafe extern "system" fn(*mut core::ffi::c_void, *const D2D1_QUADRATIC_BEZIER_SEGMENT),
    pub AddQuadraticBeziers: unsafe extern "system" fn(*mut core::ffi::c_void, *const D2D1_QUADRATIC_BEZIER_SEGMENT, u32),
    #[cfg(feature = "Win32_dcommon")]
    pub AddArc: unsafe extern "system" fn(*mut core::ffi::c_void, *const D2D1_ARC_SEGMENT),
    #[cfg(not(feature = "Win32_dcommon"))]
    AddArc: usize,
}
#[cfg(feature = "Win32_dcommon")]
pub trait ID2D1GeometrySink_Impl: ID2D1SimplifiedGeometrySink_Impl {
    fn AddLine(&self, point: &windows_numerics::Vector2);
    fn AddBezier(&self, bezier: *const D2D1_BEZIER_SEGMENT);
    fn AddQuadraticBezier(&self, bezier: *const D2D1_QUADRATIC_BEZIER_SEGMENT);
    fn AddQuadraticBeziers(&self, beziers: *const D2D1_QUADRATIC_BEZIER_SEGMENT, bezierscount: u32);
    fn AddArc(&self, arc: *const D2D1_ARC_SEGMENT);
}
#[cfg(feature = "Win32_dcommon")]
impl ID2D1GeometrySink_Vtbl {
    pub const fn new<Identity: ID2D1GeometrySink_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn AddLine<Identity: ID2D1GeometrySink_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, point: windows_numerics::Vector2) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID2D1GeometrySink_Impl::AddLine(this, core::mem::transmute(&point));
            }
        }
        unsafe extern "system" fn AddBezier<Identity: ID2D1GeometrySink_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bezier: *const D2D1_BEZIER_SEGMENT) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID2D1GeometrySink_Impl::AddBezier(this, core::mem::transmute_copy(&bezier));
            }
        }
        unsafe extern "system" fn AddQuadraticBezier<Identity: ID2D1GeometrySink_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bezier: *const D2D1_QUADRATIC_BEZIER_SEGMENT) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID2D1GeometrySink_Impl::AddQuadraticBezier(this, core::mem::transmute_copy(&bezier));
            }
        }
        unsafe extern "system" fn AddQuadraticBeziers<Identity: ID2D1GeometrySink_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, beziers: *const D2D1_QUADRATIC_BEZIER_SEGMENT, bezierscount: u32) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID2D1GeometrySink_Impl::AddQuadraticBeziers(this, core::mem::transmute_copy(&beziers), core::mem::transmute_copy(&bezierscount));
            }
        }
        unsafe extern "system" fn AddArc<Identity: ID2D1GeometrySink_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, arc: *const D2D1_ARC_SEGMENT) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID2D1GeometrySink_Impl::AddArc(this, core::mem::transmute_copy(&arc));
            }
        }
        Self {
            base__: ID2D1SimplifiedGeometrySink_Vtbl::new::<Identity, OFFSET>(),
            AddLine: AddLine::<Identity, OFFSET>,
            AddBezier: AddBezier::<Identity, OFFSET>,
            AddQuadraticBezier: AddQuadraticBezier::<Identity, OFFSET>,
            AddQuadraticBeziers: AddQuadraticBeziers::<Identity, OFFSET>,
            AddArc: AddArc::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ID2D1GeometrySink as windows_core::Interface>::IID || iid == &<ID2D1SimplifiedGeometrySink as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_dcommon")]
impl windows_core::RuntimeName for ID2D1GeometrySink {}
windows_core::imp::define_interface!(ID2D1GradientStopCollection, ID2D1GradientStopCollection_Vtbl, 0x2cd906a7_12e2_11dc_9fed_001143a055f9);
impl core::ops::Deref for ID2D1GradientStopCollection {
    type Target = ID2D1Resource;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ID2D1GradientStopCollection, windows_core::IUnknown, ID2D1Resource);
impl ID2D1GradientStopCollection {
    pub unsafe fn GetGradientStopCount(&self) -> u32 {
        unsafe { (windows_core::Interface::vtable(self).GetGradientStopCount)(windows_core::Interface::as_raw(self)) }
    }
    #[cfg(all(feature = "Win32_d2dbasetypes", feature = "Win32_dxgitype"))]
    pub unsafe fn GetGradientStops(&self, gradientstops: &mut [D2D1_GRADIENT_STOP]) {
        unsafe {
            (windows_core::Interface::vtable(self).GetGradientStops)(windows_core::Interface::as_raw(self), core::mem::transmute(gradientstops.as_ptr()), gradientstops.len().try_into().unwrap());
        }
    }
    pub unsafe fn GetColorInterpolationGamma(&self) -> D2D1_GAMMA {
        unsafe { (windows_core::Interface::vtable(self).GetColorInterpolationGamma)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn GetExtendMode(&self) -> D2D1_EXTEND_MODE {
        unsafe { (windows_core::Interface::vtable(self).GetExtendMode)(windows_core::Interface::as_raw(self)) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ID2D1GradientStopCollection_Vtbl {
    pub base__: ID2D1Resource_Vtbl,
    pub GetGradientStopCount: unsafe extern "system" fn(*mut core::ffi::c_void) -> u32,
    #[cfg(all(feature = "Win32_d2dbasetypes", feature = "Win32_dxgitype"))]
    pub GetGradientStops: unsafe extern "system" fn(*mut core::ffi::c_void, *mut D2D1_GRADIENT_STOP, u32),
    #[cfg(not(all(feature = "Win32_d2dbasetypes", feature = "Win32_dxgitype")))]
    GetGradientStops: usize,
    pub GetColorInterpolationGamma: unsafe extern "system" fn(*mut core::ffi::c_void) -> D2D1_GAMMA,
    pub GetExtendMode: unsafe extern "system" fn(*mut core::ffi::c_void) -> D2D1_EXTEND_MODE,
}
#[cfg(all(feature = "Win32_d2dbasetypes", feature = "Win32_dxgitype"))]
pub trait ID2D1GradientStopCollection_Impl: ID2D1Resource_Impl {
    fn GetGradientStopCount(&self) -> u32;
    fn GetGradientStops(&self, gradientstops: *mut D2D1_GRADIENT_STOP, gradientstopscount: u32);
    fn GetColorInterpolationGamma(&self) -> D2D1_GAMMA;
    fn GetExtendMode(&self) -> D2D1_EXTEND_MODE;
}
#[cfg(all(feature = "Win32_d2dbasetypes", feature = "Win32_dxgitype"))]
impl ID2D1GradientStopCollection_Vtbl {
    pub const fn new<Identity: ID2D1GradientStopCollection_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetGradientStopCount<Identity: ID2D1GradientStopCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> u32 {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID2D1GradientStopCollection_Impl::GetGradientStopCount(this)
            }
        }
        unsafe extern "system" fn GetGradientStops<Identity: ID2D1GradientStopCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, gradientstops: *mut D2D1_GRADIENT_STOP, gradientstopscount: u32) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID2D1GradientStopCollection_Impl::GetGradientStops(this, core::mem::transmute_copy(&gradientstops), core::mem::transmute_copy(&gradientstopscount));
            }
        }
        unsafe extern "system" fn GetColorInterpolationGamma<Identity: ID2D1GradientStopCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> D2D1_GAMMA {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID2D1GradientStopCollection_Impl::GetColorInterpolationGamma(this)
            }
        }
        unsafe extern "system" fn GetExtendMode<Identity: ID2D1GradientStopCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> D2D1_EXTEND_MODE {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID2D1GradientStopCollection_Impl::GetExtendMode(this)
            }
        }
        Self {
            base__: ID2D1Resource_Vtbl::new::<Identity, OFFSET>(),
            GetGradientStopCount: GetGradientStopCount::<Identity, OFFSET>,
            GetGradientStops: GetGradientStops::<Identity, OFFSET>,
            GetColorInterpolationGamma: GetColorInterpolationGamma::<Identity, OFFSET>,
            GetExtendMode: GetExtendMode::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ID2D1GradientStopCollection as windows_core::Interface>::IID || iid == &<ID2D1Resource as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_d2dbasetypes", feature = "Win32_dxgitype"))]
impl windows_core::RuntimeName for ID2D1GradientStopCollection {}
windows_core::imp::define_interface!(ID2D1HwndRenderTarget, ID2D1HwndRenderTarget_Vtbl, 0x2cd90698_12e2_11dc_9fed_001143a055f9);
impl core::ops::Deref for ID2D1HwndRenderTarget {
    type Target = ID2D1RenderTarget;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ID2D1HwndRenderTarget, windows_core::IUnknown, ID2D1Resource, ID2D1RenderTarget);
impl ID2D1HwndRenderTarget {
    pub unsafe fn CheckWindowState(&self) -> D2D1_WINDOW_STATE {
        unsafe { (windows_core::Interface::vtable(self).CheckWindowState)(windows_core::Interface::as_raw(self)) }
    }
    #[cfg(feature = "Win32_dcommon")]
    pub unsafe fn Resize(&self, pixelsize: *const super::dcommon::D2D_SIZE_U) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Resize)(windows_core::Interface::as_raw(self), pixelsize) }
    }
    #[cfg(feature = "Win32_windef")]
    pub unsafe fn GetHwnd(&self) -> super::windef::HWND {
        unsafe { (windows_core::Interface::vtable(self).GetHwnd)(windows_core::Interface::as_raw(self)) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ID2D1HwndRenderTarget_Vtbl {
    pub base__: ID2D1RenderTarget_Vtbl,
    pub CheckWindowState: unsafe extern "system" fn(*mut core::ffi::c_void) -> D2D1_WINDOW_STATE,
    #[cfg(feature = "Win32_dcommon")]
    pub Resize: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::dcommon::D2D_SIZE_U) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_dcommon"))]
    Resize: usize,
    #[cfg(feature = "Win32_windef")]
    pub GetHwnd: unsafe extern "system" fn(*mut core::ffi::c_void) -> super::windef::HWND,
    #[cfg(not(feature = "Win32_windef"))]
    GetHwnd: usize,
}
#[cfg(all(feature = "Win32_d2dbasetypes", feature = "Win32_dcommon", feature = "Win32_dwrite", feature = "Win32_dxgiformat", feature = "Win32_dxgitype", feature = "Win32_wincodec", feature = "Win32_windef"))]
pub trait ID2D1HwndRenderTarget_Impl: ID2D1RenderTarget_Impl {
    fn CheckWindowState(&self) -> D2D1_WINDOW_STATE;
    fn Resize(&self, pixelsize: *const super::dcommon::D2D_SIZE_U) -> windows_core::Result<()>;
    fn GetHwnd(&self) -> super::windef::HWND;
}
#[cfg(all(feature = "Win32_d2dbasetypes", feature = "Win32_dcommon", feature = "Win32_dwrite", feature = "Win32_dxgiformat", feature = "Win32_dxgitype", feature = "Win32_wincodec", feature = "Win32_windef"))]
impl ID2D1HwndRenderTarget_Vtbl {
    pub const fn new<Identity: ID2D1HwndRenderTarget_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn CheckWindowState<Identity: ID2D1HwndRenderTarget_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> D2D1_WINDOW_STATE {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID2D1HwndRenderTarget_Impl::CheckWindowState(this)
            }
        }
        unsafe extern "system" fn Resize<Identity: ID2D1HwndRenderTarget_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pixelsize: *const super::dcommon::D2D_SIZE_U) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID2D1HwndRenderTarget_Impl::Resize(this, core::mem::transmute_copy(&pixelsize)).into()
            }
        }
        unsafe extern "system" fn GetHwnd<Identity: ID2D1HwndRenderTarget_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> super::windef::HWND {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID2D1HwndRenderTarget_Impl::GetHwnd(this)
            }
        }
        Self {
            base__: ID2D1RenderTarget_Vtbl::new::<Identity, OFFSET>(),
            CheckWindowState: CheckWindowState::<Identity, OFFSET>,
            Resize: Resize::<Identity, OFFSET>,
            GetHwnd: GetHwnd::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ID2D1HwndRenderTarget as windows_core::Interface>::IID || iid == &<ID2D1Resource as windows_core::Interface>::IID || iid == &<ID2D1RenderTarget as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_d2dbasetypes", feature = "Win32_dcommon", feature = "Win32_dwrite", feature = "Win32_dxgiformat", feature = "Win32_dxgitype", feature = "Win32_wincodec", feature = "Win32_windef"))]
impl windows_core::RuntimeName for ID2D1HwndRenderTarget {}
windows_core::imp::define_interface!(ID2D1Image, ID2D1Image_Vtbl, 0x65019f75_8da2_497c_b32c_dfa34e48ede6);
impl core::ops::Deref for ID2D1Image {
    type Target = ID2D1Resource;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ID2D1Image, windows_core::IUnknown, ID2D1Resource);
#[repr(C)]
#[doc(hidden)]
pub struct ID2D1Image_Vtbl {
    pub base__: ID2D1Resource_Vtbl,
}
pub trait ID2D1Image_Impl: ID2D1Resource_Impl {}
impl ID2D1Image_Vtbl {
    pub const fn new<Identity: ID2D1Image_Impl, const OFFSET: isize>() -> Self {
        Self { base__: ID2D1Resource_Vtbl::new::<Identity, OFFSET>() }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ID2D1Image as windows_core::Interface>::IID || iid == &<ID2D1Resource as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for ID2D1Image {}
windows_core::imp::define_interface!(ID2D1Layer, ID2D1Layer_Vtbl, 0x2cd9069b_12e2_11dc_9fed_001143a055f9);
impl core::ops::Deref for ID2D1Layer {
    type Target = ID2D1Resource;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ID2D1Layer, windows_core::IUnknown, ID2D1Resource);
impl ID2D1Layer {
    #[cfg(feature = "Win32_dcommon")]
    pub unsafe fn GetSize(&self) -> super::dcommon::D2D_SIZE_F {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetSize)(windows_core::Interface::as_raw(self), &mut result__);
            result__
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ID2D1Layer_Vtbl {
    pub base__: ID2D1Resource_Vtbl,
    #[cfg(feature = "Win32_dcommon")]
    pub GetSize: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::dcommon::D2D_SIZE_F),
    #[cfg(not(feature = "Win32_dcommon"))]
    GetSize: usize,
}
#[cfg(feature = "Win32_dcommon")]
pub trait ID2D1Layer_Impl: ID2D1Resource_Impl {
    fn GetSize(&self) -> super::dcommon::D2D_SIZE_F;
}
#[cfg(feature = "Win32_dcommon")]
impl ID2D1Layer_Vtbl {
    pub const fn new<Identity: ID2D1Layer_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetSize<Identity: ID2D1Layer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut super::dcommon::D2D_SIZE_F) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                *result__ = ID2D1Layer_Impl::GetSize(this);
            }
        }
        Self { base__: ID2D1Resource_Vtbl::new::<Identity, OFFSET>(), GetSize: GetSize::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ID2D1Layer as windows_core::Interface>::IID || iid == &<ID2D1Resource as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_dcommon")]
impl windows_core::RuntimeName for ID2D1Layer {}
windows_core::imp::define_interface!(ID2D1LinearGradientBrush, ID2D1LinearGradientBrush_Vtbl, 0x2cd906ab_12e2_11dc_9fed_001143a055f9);
impl core::ops::Deref for ID2D1LinearGradientBrush {
    type Target = ID2D1Brush;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ID2D1LinearGradientBrush, windows_core::IUnknown, ID2D1Resource, ID2D1Brush);
impl ID2D1LinearGradientBrush {
    pub unsafe fn SetStartPoint(&self, startpoint: windows_numerics::Vector2) {
        unsafe {
            (windows_core::Interface::vtable(self).SetStartPoint)(windows_core::Interface::as_raw(self), core::mem::transmute(startpoint));
        }
    }
    pub unsafe fn SetEndPoint(&self, endpoint: windows_numerics::Vector2) {
        unsafe {
            (windows_core::Interface::vtable(self).SetEndPoint)(windows_core::Interface::as_raw(self), core::mem::transmute(endpoint));
        }
    }
    pub unsafe fn GetStartPoint(&self) -> windows_numerics::Vector2 {
        unsafe { (windows_core::Interface::vtable(self).GetStartPoint)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn GetEndPoint(&self) -> windows_numerics::Vector2 {
        unsafe { (windows_core::Interface::vtable(self).GetEndPoint)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn GetGradientStopCollection(&self) -> windows_core::Result<ID2D1GradientStopCollection> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetGradientStopCollection)(windows_core::Interface::as_raw(self), &mut result__);
            windows_core::Type::from_abi(result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ID2D1LinearGradientBrush_Vtbl {
    pub base__: ID2D1Brush_Vtbl,
    pub SetStartPoint: unsafe extern "system" fn(*mut core::ffi::c_void, windows_numerics::Vector2),
    pub SetEndPoint: unsafe extern "system" fn(*mut core::ffi::c_void, windows_numerics::Vector2),
    pub GetStartPoint: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_numerics::Vector2,
    pub GetEndPoint: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_numerics::Vector2,
    pub GetGradientStopCollection: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void),
}
pub trait ID2D1LinearGradientBrush_Impl: ID2D1Brush_Impl {
    fn SetStartPoint(&self, startpoint: &windows_numerics::Vector2);
    fn SetEndPoint(&self, endpoint: &windows_numerics::Vector2);
    fn GetStartPoint(&self) -> windows_numerics::Vector2;
    fn GetEndPoint(&self) -> windows_numerics::Vector2;
    fn GetGradientStopCollection(&self, gradientstopcollection: windows_core::OutRef<ID2D1GradientStopCollection>);
}
impl ID2D1LinearGradientBrush_Vtbl {
    pub const fn new<Identity: ID2D1LinearGradientBrush_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn SetStartPoint<Identity: ID2D1LinearGradientBrush_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, startpoint: windows_numerics::Vector2) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID2D1LinearGradientBrush_Impl::SetStartPoint(this, core::mem::transmute(&startpoint));
            }
        }
        unsafe extern "system" fn SetEndPoint<Identity: ID2D1LinearGradientBrush_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, endpoint: windows_numerics::Vector2) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID2D1LinearGradientBrush_Impl::SetEndPoint(this, core::mem::transmute(&endpoint));
            }
        }
        unsafe extern "system" fn GetStartPoint<Identity: ID2D1LinearGradientBrush_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_numerics::Vector2 {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID2D1LinearGradientBrush_Impl::GetStartPoint(this)
            }
        }
        unsafe extern "system" fn GetEndPoint<Identity: ID2D1LinearGradientBrush_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_numerics::Vector2 {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID2D1LinearGradientBrush_Impl::GetEndPoint(this)
            }
        }
        unsafe extern "system" fn GetGradientStopCollection<Identity: ID2D1LinearGradientBrush_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, gradientstopcollection: *mut *mut core::ffi::c_void) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID2D1LinearGradientBrush_Impl::GetGradientStopCollection(this, core::mem::transmute_copy(&gradientstopcollection));
            }
        }
        Self {
            base__: ID2D1Brush_Vtbl::new::<Identity, OFFSET>(),
            SetStartPoint: SetStartPoint::<Identity, OFFSET>,
            SetEndPoint: SetEndPoint::<Identity, OFFSET>,
            GetStartPoint: GetStartPoint::<Identity, OFFSET>,
            GetEndPoint: GetEndPoint::<Identity, OFFSET>,
            GetGradientStopCollection: GetGradientStopCollection::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ID2D1LinearGradientBrush as windows_core::Interface>::IID || iid == &<ID2D1Resource as windows_core::Interface>::IID || iid == &<ID2D1Brush as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for ID2D1LinearGradientBrush {}
windows_core::imp::define_interface!(ID2D1Mesh, ID2D1Mesh_Vtbl, 0x2cd906c2_12e2_11dc_9fed_001143a055f9);
impl core::ops::Deref for ID2D1Mesh {
    type Target = ID2D1Resource;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ID2D1Mesh, windows_core::IUnknown, ID2D1Resource);
impl ID2D1Mesh {
    pub unsafe fn Open(&self) -> windows_core::Result<ID2D1TessellationSink> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Open)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ID2D1Mesh_Vtbl {
    pub base__: ID2D1Resource_Vtbl,
    pub Open: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait ID2D1Mesh_Impl: ID2D1Resource_Impl {
    fn Open(&self) -> windows_core::Result<ID2D1TessellationSink>;
}
impl ID2D1Mesh_Vtbl {
    pub const fn new<Identity: ID2D1Mesh_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Open<Identity: ID2D1Mesh_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, tessellationsink: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ID2D1Mesh_Impl::Open(this) {
                    Ok(ok__) => {
                        tessellationsink.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self { base__: ID2D1Resource_Vtbl::new::<Identity, OFFSET>(), Open: Open::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ID2D1Mesh as windows_core::Interface>::IID || iid == &<ID2D1Resource as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for ID2D1Mesh {}
windows_core::imp::define_interface!(ID2D1PathGeometry, ID2D1PathGeometry_Vtbl, 0x2cd906a5_12e2_11dc_9fed_001143a055f9);
impl core::ops::Deref for ID2D1PathGeometry {
    type Target = ID2D1Geometry;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ID2D1PathGeometry, windows_core::IUnknown, ID2D1Resource, ID2D1Geometry);
impl ID2D1PathGeometry {
    pub unsafe fn Open(&self) -> windows_core::Result<ID2D1GeometrySink> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Open)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn Stream<P0>(&self, geometrysink: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<ID2D1GeometrySink>,
    {
        unsafe { (windows_core::Interface::vtable(self).Stream)(windows_core::Interface::as_raw(self), geometrysink.param().abi()) }
    }
    pub unsafe fn GetSegmentCount(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetSegmentCount)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetFigureCount(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetFigureCount)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ID2D1PathGeometry_Vtbl {
    pub base__: ID2D1Geometry_Vtbl,
    pub Open: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Stream: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetSegmentCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub GetFigureCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_dcommon")]
pub trait ID2D1PathGeometry_Impl: ID2D1Geometry_Impl {
    fn Open(&self) -> windows_core::Result<ID2D1GeometrySink>;
    fn Stream(&self, geometrysink: windows_core::Ref<ID2D1GeometrySink>) -> windows_core::Result<()>;
    fn GetSegmentCount(&self) -> windows_core::Result<u32>;
    fn GetFigureCount(&self) -> windows_core::Result<u32>;
}
#[cfg(feature = "Win32_dcommon")]
impl ID2D1PathGeometry_Vtbl {
    pub const fn new<Identity: ID2D1PathGeometry_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Open<Identity: ID2D1PathGeometry_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, geometrysink: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ID2D1PathGeometry_Impl::Open(this) {
                    Ok(ok__) => {
                        geometrysink.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Stream<Identity: ID2D1PathGeometry_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, geometrysink: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID2D1PathGeometry_Impl::Stream(this, core::mem::transmute_copy(&geometrysink)).into()
            }
        }
        unsafe extern "system" fn GetSegmentCount<Identity: ID2D1PathGeometry_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, count: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ID2D1PathGeometry_Impl::GetSegmentCount(this) {
                    Ok(ok__) => {
                        count.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetFigureCount<Identity: ID2D1PathGeometry_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, count: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ID2D1PathGeometry_Impl::GetFigureCount(this) {
                    Ok(ok__) => {
                        count.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: ID2D1Geometry_Vtbl::new::<Identity, OFFSET>(),
            Open: Open::<Identity, OFFSET>,
            Stream: Stream::<Identity, OFFSET>,
            GetSegmentCount: GetSegmentCount::<Identity, OFFSET>,
            GetFigureCount: GetFigureCount::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ID2D1PathGeometry as windows_core::Interface>::IID || iid == &<ID2D1Resource as windows_core::Interface>::IID || iid == &<ID2D1Geometry as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_dcommon")]
impl windows_core::RuntimeName for ID2D1PathGeometry {}
windows_core::imp::define_interface!(ID2D1RadialGradientBrush, ID2D1RadialGradientBrush_Vtbl, 0x2cd906ac_12e2_11dc_9fed_001143a055f9);
impl core::ops::Deref for ID2D1RadialGradientBrush {
    type Target = ID2D1Brush;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ID2D1RadialGradientBrush, windows_core::IUnknown, ID2D1Resource, ID2D1Brush);
impl ID2D1RadialGradientBrush {
    pub unsafe fn SetCenter(&self, center: windows_numerics::Vector2) {
        unsafe {
            (windows_core::Interface::vtable(self).SetCenter)(windows_core::Interface::as_raw(self), core::mem::transmute(center));
        }
    }
    pub unsafe fn SetGradientOriginOffset(&self, gradientoriginoffset: windows_numerics::Vector2) {
        unsafe {
            (windows_core::Interface::vtable(self).SetGradientOriginOffset)(windows_core::Interface::as_raw(self), core::mem::transmute(gradientoriginoffset));
        }
    }
    pub unsafe fn SetRadiusX(&self, radiusx: f32) {
        unsafe {
            (windows_core::Interface::vtable(self).SetRadiusX)(windows_core::Interface::as_raw(self), radiusx);
        }
    }
    pub unsafe fn SetRadiusY(&self, radiusy: f32) {
        unsafe {
            (windows_core::Interface::vtable(self).SetRadiusY)(windows_core::Interface::as_raw(self), radiusy);
        }
    }
    pub unsafe fn GetCenter(&self) -> windows_numerics::Vector2 {
        unsafe { (windows_core::Interface::vtable(self).GetCenter)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn GetGradientOriginOffset(&self) -> windows_numerics::Vector2 {
        unsafe { (windows_core::Interface::vtable(self).GetGradientOriginOffset)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn GetRadiusX(&self) -> f32 {
        unsafe { (windows_core::Interface::vtable(self).GetRadiusX)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn GetRadiusY(&self) -> f32 {
        unsafe { (windows_core::Interface::vtable(self).GetRadiusY)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn GetGradientStopCollection(&self) -> windows_core::Result<ID2D1GradientStopCollection> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetGradientStopCollection)(windows_core::Interface::as_raw(self), &mut result__);
            windows_core::Type::from_abi(result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ID2D1RadialGradientBrush_Vtbl {
    pub base__: ID2D1Brush_Vtbl,
    pub SetCenter: unsafe extern "system" fn(*mut core::ffi::c_void, windows_numerics::Vector2),
    pub SetGradientOriginOffset: unsafe extern "system" fn(*mut core::ffi::c_void, windows_numerics::Vector2),
    pub SetRadiusX: unsafe extern "system" fn(*mut core::ffi::c_void, f32),
    pub SetRadiusY: unsafe extern "system" fn(*mut core::ffi::c_void, f32),
    pub GetCenter: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_numerics::Vector2,
    pub GetGradientOriginOffset: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_numerics::Vector2,
    pub GetRadiusX: unsafe extern "system" fn(*mut core::ffi::c_void) -> f32,
    pub GetRadiusY: unsafe extern "system" fn(*mut core::ffi::c_void) -> f32,
    pub GetGradientStopCollection: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void),
}
pub trait ID2D1RadialGradientBrush_Impl: ID2D1Brush_Impl {
    fn SetCenter(&self, center: &windows_numerics::Vector2);
    fn SetGradientOriginOffset(&self, gradientoriginoffset: &windows_numerics::Vector2);
    fn SetRadiusX(&self, radiusx: f32);
    fn SetRadiusY(&self, radiusy: f32);
    fn GetCenter(&self) -> windows_numerics::Vector2;
    fn GetGradientOriginOffset(&self) -> windows_numerics::Vector2;
    fn GetRadiusX(&self) -> f32;
    fn GetRadiusY(&self) -> f32;
    fn GetGradientStopCollection(&self, gradientstopcollection: windows_core::OutRef<ID2D1GradientStopCollection>);
}
impl ID2D1RadialGradientBrush_Vtbl {
    pub const fn new<Identity: ID2D1RadialGradientBrush_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn SetCenter<Identity: ID2D1RadialGradientBrush_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, center: windows_numerics::Vector2) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID2D1RadialGradientBrush_Impl::SetCenter(this, core::mem::transmute(&center));
            }
        }
        unsafe extern "system" fn SetGradientOriginOffset<Identity: ID2D1RadialGradientBrush_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, gradientoriginoffset: windows_numerics::Vector2) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID2D1RadialGradientBrush_Impl::SetGradientOriginOffset(this, core::mem::transmute(&gradientoriginoffset));
            }
        }
        unsafe extern "system" fn SetRadiusX<Identity: ID2D1RadialGradientBrush_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, radiusx: f32) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID2D1RadialGradientBrush_Impl::SetRadiusX(this, core::mem::transmute_copy(&radiusx));
            }
        }
        unsafe extern "system" fn SetRadiusY<Identity: ID2D1RadialGradientBrush_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, radiusy: f32) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID2D1RadialGradientBrush_Impl::SetRadiusY(this, core::mem::transmute_copy(&radiusy));
            }
        }
        unsafe extern "system" fn GetCenter<Identity: ID2D1RadialGradientBrush_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_numerics::Vector2 {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID2D1RadialGradientBrush_Impl::GetCenter(this)
            }
        }
        unsafe extern "system" fn GetGradientOriginOffset<Identity: ID2D1RadialGradientBrush_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_numerics::Vector2 {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID2D1RadialGradientBrush_Impl::GetGradientOriginOffset(this)
            }
        }
        unsafe extern "system" fn GetRadiusX<Identity: ID2D1RadialGradientBrush_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> f32 {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID2D1RadialGradientBrush_Impl::GetRadiusX(this)
            }
        }
        unsafe extern "system" fn GetRadiusY<Identity: ID2D1RadialGradientBrush_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> f32 {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID2D1RadialGradientBrush_Impl::GetRadiusY(this)
            }
        }
        unsafe extern "system" fn GetGradientStopCollection<Identity: ID2D1RadialGradientBrush_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, gradientstopcollection: *mut *mut core::ffi::c_void) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID2D1RadialGradientBrush_Impl::GetGradientStopCollection(this, core::mem::transmute_copy(&gradientstopcollection));
            }
        }
        Self {
            base__: ID2D1Brush_Vtbl::new::<Identity, OFFSET>(),
            SetCenter: SetCenter::<Identity, OFFSET>,
            SetGradientOriginOffset: SetGradientOriginOffset::<Identity, OFFSET>,
            SetRadiusX: SetRadiusX::<Identity, OFFSET>,
            SetRadiusY: SetRadiusY::<Identity, OFFSET>,
            GetCenter: GetCenter::<Identity, OFFSET>,
            GetGradientOriginOffset: GetGradientOriginOffset::<Identity, OFFSET>,
            GetRadiusX: GetRadiusX::<Identity, OFFSET>,
            GetRadiusY: GetRadiusY::<Identity, OFFSET>,
            GetGradientStopCollection: GetGradientStopCollection::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ID2D1RadialGradientBrush as windows_core::Interface>::IID || iid == &<ID2D1Resource as windows_core::Interface>::IID || iid == &<ID2D1Brush as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for ID2D1RadialGradientBrush {}
windows_core::imp::define_interface!(ID2D1RectangleGeometry, ID2D1RectangleGeometry_Vtbl, 0x2cd906a2_12e2_11dc_9fed_001143a055f9);
impl core::ops::Deref for ID2D1RectangleGeometry {
    type Target = ID2D1Geometry;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ID2D1RectangleGeometry, windows_core::IUnknown, ID2D1Resource, ID2D1Geometry);
impl ID2D1RectangleGeometry {
    #[cfg(feature = "Win32_dcommon")]
    pub unsafe fn GetRect(&self) -> super::dcommon::D2D_RECT_F {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetRect)(windows_core::Interface::as_raw(self), &mut result__);
            result__
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ID2D1RectangleGeometry_Vtbl {
    pub base__: ID2D1Geometry_Vtbl,
    #[cfg(feature = "Win32_dcommon")]
    pub GetRect: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::dcommon::D2D_RECT_F),
    #[cfg(not(feature = "Win32_dcommon"))]
    GetRect: usize,
}
#[cfg(feature = "Win32_dcommon")]
pub trait ID2D1RectangleGeometry_Impl: ID2D1Geometry_Impl {
    fn GetRect(&self, rect: *mut super::dcommon::D2D_RECT_F);
}
#[cfg(feature = "Win32_dcommon")]
impl ID2D1RectangleGeometry_Vtbl {
    pub const fn new<Identity: ID2D1RectangleGeometry_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetRect<Identity: ID2D1RectangleGeometry_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, rect: *mut super::dcommon::D2D_RECT_F) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID2D1RectangleGeometry_Impl::GetRect(this, core::mem::transmute_copy(&rect));
            }
        }
        Self { base__: ID2D1Geometry_Vtbl::new::<Identity, OFFSET>(), GetRect: GetRect::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ID2D1RectangleGeometry as windows_core::Interface>::IID || iid == &<ID2D1Resource as windows_core::Interface>::IID || iid == &<ID2D1Geometry as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_dcommon")]
impl windows_core::RuntimeName for ID2D1RectangleGeometry {}
windows_core::imp::define_interface!(ID2D1RenderTarget, ID2D1RenderTarget_Vtbl, 0x2cd90694_12e2_11dc_9fed_001143a055f9);
impl core::ops::Deref for ID2D1RenderTarget {
    type Target = ID2D1Resource;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ID2D1RenderTarget, windows_core::IUnknown, ID2D1Resource);
impl ID2D1RenderTarget {
    #[cfg(all(feature = "Win32_dcommon", feature = "Win32_dxgiformat"))]
    pub unsafe fn CreateBitmap(&self, size: super::dcommon::D2D_SIZE_U, srcdata: Option<*const core::ffi::c_void>, pitch: u32, bitmapproperties: *const D2D1_BITMAP_PROPERTIES) -> windows_core::Result<ID2D1Bitmap> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateBitmap)(windows_core::Interface::as_raw(self), core::mem::transmute(size), srcdata.unwrap_or(core::mem::zeroed()) as _, pitch, bitmapproperties, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(all(feature = "Win32_dcommon", feature = "Win32_dxgiformat", feature = "Win32_wincodec"))]
    pub unsafe fn CreateBitmapFromWicBitmap<P0>(&self, wicbitmapsource: P0, bitmapproperties: Option<*const D2D1_BITMAP_PROPERTIES>) -> windows_core::Result<ID2D1Bitmap>
    where
        P0: windows_core::Param<super::wincodec::IWICBitmapSource>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateBitmapFromWicBitmap)(windows_core::Interface::as_raw(self), wicbitmapsource.param().abi(), bitmapproperties.unwrap_or(core::mem::zeroed()) as _, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(all(feature = "Win32_dcommon", feature = "Win32_dxgiformat"))]
    pub unsafe fn CreateSharedBitmap(&self, riid: *const windows_core::GUID, data: *mut core::ffi::c_void, bitmapproperties: Option<*const D2D1_BITMAP_PROPERTIES>, bitmap: *mut Option<ID2D1Bitmap>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).CreateSharedBitmap)(windows_core::Interface::as_raw(self), riid, data as _, bitmapproperties.unwrap_or(core::mem::zeroed()) as _, core::mem::transmute(bitmap)) }
    }
    pub unsafe fn CreateBitmapBrush<P0>(&self, bitmap: P0, bitmapbrushproperties: Option<*const D2D1_BITMAP_BRUSH_PROPERTIES>, brushproperties: Option<*const D2D1_BRUSH_PROPERTIES>) -> windows_core::Result<ID2D1BitmapBrush>
    where
        P0: windows_core::Param<ID2D1Bitmap>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateBitmapBrush)(windows_core::Interface::as_raw(self), bitmap.param().abi(), bitmapbrushproperties.unwrap_or(core::mem::zeroed()) as _, brushproperties.unwrap_or(core::mem::zeroed()) as _, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(all(feature = "Win32_d2dbasetypes", feature = "Win32_dxgitype"))]
    pub unsafe fn CreateSolidColorBrush(&self, color: *const super::d2dbasetypes::D2D_COLOR_F, brushproperties: Option<*const D2D1_BRUSH_PROPERTIES>) -> windows_core::Result<ID2D1SolidColorBrush> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateSolidColorBrush)(windows_core::Interface::as_raw(self), color, brushproperties.unwrap_or(core::mem::zeroed()) as _, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(all(feature = "Win32_d2dbasetypes", feature = "Win32_dxgitype"))]
    pub unsafe fn CreateGradientStopCollection(&self, gradientstops: &[D2D1_GRADIENT_STOP], colorinterpolationgamma: D2D1_GAMMA, extendmode: D2D1_EXTEND_MODE) -> windows_core::Result<ID2D1GradientStopCollection> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateGradientStopCollection)(windows_core::Interface::as_raw(self), core::mem::transmute(gradientstops.as_ptr()), gradientstops.len().try_into().unwrap(), colorinterpolationgamma, extendmode, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn CreateLinearGradientBrush<P2>(&self, lineargradientbrushproperties: *const D2D1_LINEAR_GRADIENT_BRUSH_PROPERTIES, brushproperties: Option<*const D2D1_BRUSH_PROPERTIES>, gradientstopcollection: P2) -> windows_core::Result<ID2D1LinearGradientBrush>
    where
        P2: windows_core::Param<ID2D1GradientStopCollection>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateLinearGradientBrush)(windows_core::Interface::as_raw(self), lineargradientbrushproperties, brushproperties.unwrap_or(core::mem::zeroed()) as _, gradientstopcollection.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn CreateRadialGradientBrush<P2>(&self, radialgradientbrushproperties: *const D2D1_RADIAL_GRADIENT_BRUSH_PROPERTIES, brushproperties: Option<*const D2D1_BRUSH_PROPERTIES>, gradientstopcollection: P2) -> windows_core::Result<ID2D1RadialGradientBrush>
    where
        P2: windows_core::Param<ID2D1GradientStopCollection>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateRadialGradientBrush)(windows_core::Interface::as_raw(self), radialgradientbrushproperties, brushproperties.unwrap_or(core::mem::zeroed()) as _, gradientstopcollection.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(all(feature = "Win32_dcommon", feature = "Win32_dxgiformat"))]
    pub unsafe fn CreateCompatibleRenderTarget(&self, desiredsize: Option<*const super::dcommon::D2D_SIZE_F>, desiredpixelsize: Option<*const super::dcommon::D2D_SIZE_U>, desiredformat: Option<*const super::dcommon::D2D1_PIXEL_FORMAT>, options: D2D1_COMPATIBLE_RENDER_TARGET_OPTIONS) -> windows_core::Result<ID2D1BitmapRenderTarget> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateCompatibleRenderTarget)(windows_core::Interface::as_raw(self), desiredsize.unwrap_or(core::mem::zeroed()) as _, desiredpixelsize.unwrap_or(core::mem::zeroed()) as _, desiredformat.unwrap_or(core::mem::zeroed()) as _, options, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Win32_dcommon")]
    pub unsafe fn CreateLayer(&self, size: Option<*const super::dcommon::D2D_SIZE_F>) -> windows_core::Result<ID2D1Layer> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateLayer)(windows_core::Interface::as_raw(self), size.unwrap_or(core::mem::zeroed()) as _, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn CreateMesh(&self) -> windows_core::Result<ID2D1Mesh> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateMesh)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn DrawLine<P2, P4>(&self, point0: windows_numerics::Vector2, point1: windows_numerics::Vector2, brush: P2, strokewidth: f32, strokestyle: P4)
    where
        P2: windows_core::Param<ID2D1Brush>,
        P4: windows_core::Param<ID2D1StrokeStyle>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).DrawLine)(windows_core::Interface::as_raw(self), core::mem::transmute(point0), core::mem::transmute(point1), brush.param().abi(), strokewidth, strokestyle.param().abi());
        }
    }
    #[cfg(feature = "Win32_dcommon")]
    pub unsafe fn DrawRectangle<P1, P3>(&self, rect: *const super::dcommon::D2D_RECT_F, brush: P1, strokewidth: f32, strokestyle: P3)
    where
        P1: windows_core::Param<ID2D1Brush>,
        P3: windows_core::Param<ID2D1StrokeStyle>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).DrawRectangle)(windows_core::Interface::as_raw(self), rect, brush.param().abi(), strokewidth, strokestyle.param().abi());
        }
    }
    #[cfg(feature = "Win32_dcommon")]
    pub unsafe fn FillRectangle<P1>(&self, rect: *const super::dcommon::D2D_RECT_F, brush: P1)
    where
        P1: windows_core::Param<ID2D1Brush>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).FillRectangle)(windows_core::Interface::as_raw(self), rect, brush.param().abi());
        }
    }
    #[cfg(feature = "Win32_dcommon")]
    pub unsafe fn DrawRoundedRectangle<P1, P3>(&self, roundedrect: *const D2D1_ROUNDED_RECT, brush: P1, strokewidth: f32, strokestyle: P3)
    where
        P1: windows_core::Param<ID2D1Brush>,
        P3: windows_core::Param<ID2D1StrokeStyle>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).DrawRoundedRectangle)(windows_core::Interface::as_raw(self), roundedrect, brush.param().abi(), strokewidth, strokestyle.param().abi());
        }
    }
    #[cfg(feature = "Win32_dcommon")]
    pub unsafe fn FillRoundedRectangle<P1>(&self, roundedrect: *const D2D1_ROUNDED_RECT, brush: P1)
    where
        P1: windows_core::Param<ID2D1Brush>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).FillRoundedRectangle)(windows_core::Interface::as_raw(self), roundedrect, brush.param().abi());
        }
    }
    pub unsafe fn DrawEllipse<P1, P3>(&self, ellipse: *const D2D1_ELLIPSE, brush: P1, strokewidth: f32, strokestyle: P3)
    where
        P1: windows_core::Param<ID2D1Brush>,
        P3: windows_core::Param<ID2D1StrokeStyle>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).DrawEllipse)(windows_core::Interface::as_raw(self), ellipse, brush.param().abi(), strokewidth, strokestyle.param().abi());
        }
    }
    pub unsafe fn FillEllipse<P1>(&self, ellipse: *const D2D1_ELLIPSE, brush: P1)
    where
        P1: windows_core::Param<ID2D1Brush>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).FillEllipse)(windows_core::Interface::as_raw(self), ellipse, brush.param().abi());
        }
    }
    pub unsafe fn DrawGeometry<P0, P1, P3>(&self, geometry: P0, brush: P1, strokewidth: f32, strokestyle: P3)
    where
        P0: windows_core::Param<ID2D1Geometry>,
        P1: windows_core::Param<ID2D1Brush>,
        P3: windows_core::Param<ID2D1StrokeStyle>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).DrawGeometry)(windows_core::Interface::as_raw(self), geometry.param().abi(), brush.param().abi(), strokewidth, strokestyle.param().abi());
        }
    }
    pub unsafe fn FillGeometry<P0, P1, P2>(&self, geometry: P0, brush: P1, opacitybrush: P2)
    where
        P0: windows_core::Param<ID2D1Geometry>,
        P1: windows_core::Param<ID2D1Brush>,
        P2: windows_core::Param<ID2D1Brush>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).FillGeometry)(windows_core::Interface::as_raw(self), geometry.param().abi(), brush.param().abi(), opacitybrush.param().abi());
        }
    }
    pub unsafe fn FillMesh<P0, P1>(&self, mesh: P0, brush: P1)
    where
        P0: windows_core::Param<ID2D1Mesh>,
        P1: windows_core::Param<ID2D1Brush>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).FillMesh)(windows_core::Interface::as_raw(self), mesh.param().abi(), brush.param().abi());
        }
    }
    #[cfg(feature = "Win32_dcommon")]
    pub unsafe fn FillOpacityMask<P0, P1>(&self, opacitymask: P0, brush: P1, content: D2D1_OPACITY_MASK_CONTENT, destinationrectangle: Option<*const super::dcommon::D2D_RECT_F>, sourcerectangle: Option<*const super::dcommon::D2D_RECT_F>)
    where
        P0: windows_core::Param<ID2D1Bitmap>,
        P1: windows_core::Param<ID2D1Brush>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).FillOpacityMask)(windows_core::Interface::as_raw(self), opacitymask.param().abi(), brush.param().abi(), content, destinationrectangle.unwrap_or(core::mem::zeroed()) as _, sourcerectangle.unwrap_or(core::mem::zeroed()) as _);
        }
    }
    #[cfg(feature = "Win32_dcommon")]
    pub unsafe fn DrawBitmap<P0>(&self, bitmap: P0, destinationrectangle: Option<*const super::dcommon::D2D_RECT_F>, opacity: f32, interpolationmode: D2D1_BITMAP_INTERPOLATION_MODE, sourcerectangle: Option<*const super::dcommon::D2D_RECT_F>)
    where
        P0: windows_core::Param<ID2D1Bitmap>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).DrawBitmap)(windows_core::Interface::as_raw(self), bitmap.param().abi(), destinationrectangle.unwrap_or(core::mem::zeroed()) as _, opacity, interpolationmode, sourcerectangle.unwrap_or(core::mem::zeroed()) as _);
        }
    }
    #[cfg(all(feature = "Win32_dcommon", feature = "Win32_dwrite"))]
    pub unsafe fn DrawText<P2, P4>(&self, string: &[u16], textformat: P2, layoutrect: *const super::dcommon::D2D_RECT_F, defaultfillbrush: P4, options: D2D1_DRAW_TEXT_OPTIONS, measuringmode: super::dcommon::DWRITE_MEASURING_MODE)
    where
        P2: windows_core::Param<super::dwrite::IDWriteTextFormat>,
        P4: windows_core::Param<ID2D1Brush>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).DrawText)(windows_core::Interface::as_raw(self), core::mem::transmute(string.as_ptr()), string.len().try_into().unwrap(), textformat.param().abi(), layoutrect, defaultfillbrush.param().abi(), options, measuringmode);
        }
    }
    #[cfg(feature = "Win32_dwrite")]
    pub unsafe fn DrawTextLayout<P1, P2>(&self, origin: windows_numerics::Vector2, textlayout: P1, defaultfillbrush: P2, options: D2D1_DRAW_TEXT_OPTIONS)
    where
        P1: windows_core::Param<super::dwrite::IDWriteTextLayout>,
        P2: windows_core::Param<ID2D1Brush>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).DrawTextLayout)(windows_core::Interface::as_raw(self), core::mem::transmute(origin), textlayout.param().abi(), defaultfillbrush.param().abi(), options);
        }
    }
    #[cfg(all(feature = "Win32_dcommon", feature = "Win32_dwrite"))]
    pub unsafe fn DrawGlyphRun<P2>(&self, baselineorigin: windows_numerics::Vector2, glyphrun: *const super::dwrite::DWRITE_GLYPH_RUN, foregroundbrush: P2, measuringmode: super::dcommon::DWRITE_MEASURING_MODE)
    where
        P2: windows_core::Param<ID2D1Brush>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).DrawGlyphRun)(windows_core::Interface::as_raw(self), core::mem::transmute(baselineorigin), core::mem::transmute(glyphrun), foregroundbrush.param().abi(), measuringmode);
        }
    }
    pub unsafe fn SetTransform(&self, transform: *const windows_numerics::Matrix3x2) {
        unsafe {
            (windows_core::Interface::vtable(self).SetTransform)(windows_core::Interface::as_raw(self), transform);
        }
    }
    pub unsafe fn GetTransform(&self, transform: *mut windows_numerics::Matrix3x2) {
        unsafe {
            (windows_core::Interface::vtable(self).GetTransform)(windows_core::Interface::as_raw(self), transform as _);
        }
    }
    pub unsafe fn SetAntialiasMode(&self, antialiasmode: D2D1_ANTIALIAS_MODE) {
        unsafe {
            (windows_core::Interface::vtable(self).SetAntialiasMode)(windows_core::Interface::as_raw(self), antialiasmode);
        }
    }
    pub unsafe fn GetAntialiasMode(&self) -> D2D1_ANTIALIAS_MODE {
        unsafe { (windows_core::Interface::vtable(self).GetAntialiasMode)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn SetTextAntialiasMode(&self, textantialiasmode: D2D1_TEXT_ANTIALIAS_MODE) {
        unsafe {
            (windows_core::Interface::vtable(self).SetTextAntialiasMode)(windows_core::Interface::as_raw(self), textantialiasmode);
        }
    }
    pub unsafe fn GetTextAntialiasMode(&self) -> D2D1_TEXT_ANTIALIAS_MODE {
        unsafe { (windows_core::Interface::vtable(self).GetTextAntialiasMode)(windows_core::Interface::as_raw(self)) }
    }
    #[cfg(feature = "Win32_dwrite")]
    pub unsafe fn SetTextRenderingParams<P0>(&self, textrenderingparams: P0)
    where
        P0: windows_core::Param<super::dwrite::IDWriteRenderingParams>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).SetTextRenderingParams)(windows_core::Interface::as_raw(self), textrenderingparams.param().abi());
        }
    }
    #[cfg(feature = "Win32_dwrite")]
    pub unsafe fn GetTextRenderingParams(&self) -> windows_core::Result<super::dwrite::IDWriteRenderingParams> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetTextRenderingParams)(windows_core::Interface::as_raw(self), &mut result__);
            windows_core::Type::from_abi(result__)
        }
    }
    pub unsafe fn SetTags(&self, tag1: D2D1_TAG, tag2: D2D1_TAG) {
        unsafe {
            (windows_core::Interface::vtable(self).SetTags)(windows_core::Interface::as_raw(self), tag1, tag2);
        }
    }
    pub unsafe fn GetTags(&self, tag1: Option<*mut D2D1_TAG>, tag2: Option<*mut D2D1_TAG>) {
        unsafe {
            (windows_core::Interface::vtable(self).GetTags)(windows_core::Interface::as_raw(self), tag1.unwrap_or(core::mem::zeroed()) as _, tag2.unwrap_or(core::mem::zeroed()) as _);
        }
    }
    #[cfg(feature = "Win32_dcommon")]
    pub unsafe fn PushLayer<P1>(&self, layerparameters: *const D2D1_LAYER_PARAMETERS, layer: P1)
    where
        P1: windows_core::Param<ID2D1Layer>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).PushLayer)(windows_core::Interface::as_raw(self), core::mem::transmute(layerparameters), layer.param().abi());
        }
    }
    pub unsafe fn PopLayer(&self) {
        unsafe {
            (windows_core::Interface::vtable(self).PopLayer)(windows_core::Interface::as_raw(self));
        }
    }
    pub unsafe fn Flush(&self, tag1: Option<*mut D2D1_TAG>, tag2: Option<*mut D2D1_TAG>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Flush)(windows_core::Interface::as_raw(self), tag1.unwrap_or(core::mem::zeroed()) as _, tag2.unwrap_or(core::mem::zeroed()) as _) }
    }
    pub unsafe fn SaveDrawingState(&self, drawingstateblock: &Option<ID2D1DrawingStateBlock>) {
        unsafe {
            (windows_core::Interface::vtable(self).SaveDrawingState)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(drawingstateblock));
        }
    }
    pub unsafe fn RestoreDrawingState<P0>(&self, drawingstateblock: P0)
    where
        P0: windows_core::Param<ID2D1DrawingStateBlock>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).RestoreDrawingState)(windows_core::Interface::as_raw(self), drawingstateblock.param().abi());
        }
    }
    #[cfg(feature = "Win32_dcommon")]
    pub unsafe fn PushAxisAlignedClip(&self, cliprect: *const super::dcommon::D2D_RECT_F, antialiasmode: D2D1_ANTIALIAS_MODE) {
        unsafe {
            (windows_core::Interface::vtable(self).PushAxisAlignedClip)(windows_core::Interface::as_raw(self), cliprect, antialiasmode);
        }
    }
    pub unsafe fn PopAxisAlignedClip(&self) {
        unsafe {
            (windows_core::Interface::vtable(self).PopAxisAlignedClip)(windows_core::Interface::as_raw(self));
        }
    }
    #[cfg(all(feature = "Win32_d2dbasetypes", feature = "Win32_dxgitype"))]
    pub unsafe fn Clear(&self, clearcolor: Option<*const super::d2dbasetypes::D2D_COLOR_F>) {
        unsafe {
            (windows_core::Interface::vtable(self).Clear)(windows_core::Interface::as_raw(self), clearcolor.unwrap_or(core::mem::zeroed()) as _);
        }
    }
    pub unsafe fn BeginDraw(&self) {
        unsafe {
            (windows_core::Interface::vtable(self).BeginDraw)(windows_core::Interface::as_raw(self));
        }
    }
    pub unsafe fn EndDraw(&self, tag1: Option<*mut D2D1_TAG>, tag2: Option<*mut D2D1_TAG>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).EndDraw)(windows_core::Interface::as_raw(self), tag1.unwrap_or(core::mem::zeroed()) as _, tag2.unwrap_or(core::mem::zeroed()) as _) }
    }
    #[cfg(all(feature = "Win32_dcommon", feature = "Win32_dxgiformat"))]
    pub unsafe fn GetPixelFormat(&self) -> super::dcommon::D2D1_PIXEL_FORMAT {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetPixelFormat)(windows_core::Interface::as_raw(self), &mut result__);
            result__
        }
    }
    pub unsafe fn SetDpi(&self, dpix: f32, dpiy: f32) {
        unsafe {
            (windows_core::Interface::vtable(self).SetDpi)(windows_core::Interface::as_raw(self), dpix, dpiy);
        }
    }
    pub unsafe fn GetDpi(&self, dpix: *mut f32, dpiy: *mut f32) {
        unsafe {
            (windows_core::Interface::vtable(self).GetDpi)(windows_core::Interface::as_raw(self), dpix as _, dpiy as _);
        }
    }
    #[cfg(feature = "Win32_dcommon")]
    pub unsafe fn GetSize(&self) -> super::dcommon::D2D_SIZE_F {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetSize)(windows_core::Interface::as_raw(self), &mut result__);
            result__
        }
    }
    #[cfg(feature = "Win32_dcommon")]
    pub unsafe fn GetPixelSize(&self) -> super::dcommon::D2D_SIZE_U {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetPixelSize)(windows_core::Interface::as_raw(self), &mut result__);
            result__
        }
    }
    pub unsafe fn GetMaximumBitmapSize(&self) -> u32 {
        unsafe { (windows_core::Interface::vtable(self).GetMaximumBitmapSize)(windows_core::Interface::as_raw(self)) }
    }
    #[cfg(all(feature = "Win32_dcommon", feature = "Win32_dxgiformat"))]
    pub unsafe fn IsSupported(&self, rendertargetproperties: *const D2D1_RENDER_TARGET_PROPERTIES) -> windows_core::BOOL {
        unsafe { (windows_core::Interface::vtable(self).IsSupported)(windows_core::Interface::as_raw(self), rendertargetproperties) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ID2D1RenderTarget_Vtbl {
    pub base__: ID2D1Resource_Vtbl,
    #[cfg(all(feature = "Win32_dcommon", feature = "Win32_dxgiformat"))]
    pub CreateBitmap: unsafe extern "system" fn(*mut core::ffi::c_void, super::dcommon::D2D_SIZE_U, *const core::ffi::c_void, u32, *const D2D1_BITMAP_PROPERTIES, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_dcommon", feature = "Win32_dxgiformat")))]
    CreateBitmap: usize,
    #[cfg(all(feature = "Win32_dcommon", feature = "Win32_dxgiformat", feature = "Win32_wincodec"))]
    pub CreateBitmapFromWicBitmap: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *const D2D1_BITMAP_PROPERTIES, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_dcommon", feature = "Win32_dxgiformat", feature = "Win32_wincodec")))]
    CreateBitmapFromWicBitmap: usize,
    #[cfg(all(feature = "Win32_dcommon", feature = "Win32_dxgiformat"))]
    pub CreateSharedBitmap: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *mut core::ffi::c_void, *const D2D1_BITMAP_PROPERTIES, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_dcommon", feature = "Win32_dxgiformat")))]
    CreateSharedBitmap: usize,
    pub CreateBitmapBrush: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *const D2D1_BITMAP_BRUSH_PROPERTIES, *const D2D1_BRUSH_PROPERTIES, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(all(feature = "Win32_d2dbasetypes", feature = "Win32_dxgitype"))]
    pub CreateSolidColorBrush: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::d2dbasetypes::D2D_COLOR_F, *const D2D1_BRUSH_PROPERTIES, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_d2dbasetypes", feature = "Win32_dxgitype")))]
    CreateSolidColorBrush: usize,
    #[cfg(all(feature = "Win32_d2dbasetypes", feature = "Win32_dxgitype"))]
    pub CreateGradientStopCollection: unsafe extern "system" fn(*mut core::ffi::c_void, *const D2D1_GRADIENT_STOP, u32, D2D1_GAMMA, D2D1_EXTEND_MODE, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_d2dbasetypes", feature = "Win32_dxgitype")))]
    CreateGradientStopCollection: usize,
    pub CreateLinearGradientBrush: unsafe extern "system" fn(*mut core::ffi::c_void, *const D2D1_LINEAR_GRADIENT_BRUSH_PROPERTIES, *const D2D1_BRUSH_PROPERTIES, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateRadialGradientBrush: unsafe extern "system" fn(*mut core::ffi::c_void, *const D2D1_RADIAL_GRADIENT_BRUSH_PROPERTIES, *const D2D1_BRUSH_PROPERTIES, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(all(feature = "Win32_dcommon", feature = "Win32_dxgiformat"))]
    pub CreateCompatibleRenderTarget: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::dcommon::D2D_SIZE_F, *const super::dcommon::D2D_SIZE_U, *const super::dcommon::D2D1_PIXEL_FORMAT, D2D1_COMPATIBLE_RENDER_TARGET_OPTIONS, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_dcommon", feature = "Win32_dxgiformat")))]
    CreateCompatibleRenderTarget: usize,
    #[cfg(feature = "Win32_dcommon")]
    pub CreateLayer: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::dcommon::D2D_SIZE_F, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_dcommon"))]
    CreateLayer: usize,
    pub CreateMesh: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub DrawLine: unsafe extern "system" fn(*mut core::ffi::c_void, windows_numerics::Vector2, windows_numerics::Vector2, *mut core::ffi::c_void, f32, *mut core::ffi::c_void),
    #[cfg(feature = "Win32_dcommon")]
    pub DrawRectangle: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::dcommon::D2D_RECT_F, *mut core::ffi::c_void, f32, *mut core::ffi::c_void),
    #[cfg(not(feature = "Win32_dcommon"))]
    DrawRectangle: usize,
    #[cfg(feature = "Win32_dcommon")]
    pub FillRectangle: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::dcommon::D2D_RECT_F, *mut core::ffi::c_void),
    #[cfg(not(feature = "Win32_dcommon"))]
    FillRectangle: usize,
    #[cfg(feature = "Win32_dcommon")]
    pub DrawRoundedRectangle: unsafe extern "system" fn(*mut core::ffi::c_void, *const D2D1_ROUNDED_RECT, *mut core::ffi::c_void, f32, *mut core::ffi::c_void),
    #[cfg(not(feature = "Win32_dcommon"))]
    DrawRoundedRectangle: usize,
    #[cfg(feature = "Win32_dcommon")]
    pub FillRoundedRectangle: unsafe extern "system" fn(*mut core::ffi::c_void, *const D2D1_ROUNDED_RECT, *mut core::ffi::c_void),
    #[cfg(not(feature = "Win32_dcommon"))]
    FillRoundedRectangle: usize,
    pub DrawEllipse: unsafe extern "system" fn(*mut core::ffi::c_void, *const D2D1_ELLIPSE, *mut core::ffi::c_void, f32, *mut core::ffi::c_void),
    pub FillEllipse: unsafe extern "system" fn(*mut core::ffi::c_void, *const D2D1_ELLIPSE, *mut core::ffi::c_void),
    pub DrawGeometry: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, f32, *mut core::ffi::c_void),
    pub FillGeometry: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void),
    pub FillMesh: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void),
    #[cfg(feature = "Win32_dcommon")]
    pub FillOpacityMask: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, D2D1_OPACITY_MASK_CONTENT, *const super::dcommon::D2D_RECT_F, *const super::dcommon::D2D_RECT_F),
    #[cfg(not(feature = "Win32_dcommon"))]
    FillOpacityMask: usize,
    #[cfg(feature = "Win32_dcommon")]
    pub DrawBitmap: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *const super::dcommon::D2D_RECT_F, f32, D2D1_BITMAP_INTERPOLATION_MODE, *const super::dcommon::D2D_RECT_F),
    #[cfg(not(feature = "Win32_dcommon"))]
    DrawBitmap: usize,
    #[cfg(all(feature = "Win32_dcommon", feature = "Win32_dwrite"))]
    pub DrawText: unsafe extern "system" fn(*mut core::ffi::c_void, *const u16, u32, *mut core::ffi::c_void, *const super::dcommon::D2D_RECT_F, *mut core::ffi::c_void, D2D1_DRAW_TEXT_OPTIONS, super::dcommon::DWRITE_MEASURING_MODE),
    #[cfg(not(all(feature = "Win32_dcommon", feature = "Win32_dwrite")))]
    DrawText: usize,
    #[cfg(feature = "Win32_dwrite")]
    pub DrawTextLayout: unsafe extern "system" fn(*mut core::ffi::c_void, windows_numerics::Vector2, *mut core::ffi::c_void, *mut core::ffi::c_void, D2D1_DRAW_TEXT_OPTIONS),
    #[cfg(not(feature = "Win32_dwrite"))]
    DrawTextLayout: usize,
    #[cfg(all(feature = "Win32_dcommon", feature = "Win32_dwrite"))]
    pub DrawGlyphRun: unsafe extern "system" fn(*mut core::ffi::c_void, windows_numerics::Vector2, *const super::dwrite::DWRITE_GLYPH_RUN, *mut core::ffi::c_void, super::dcommon::DWRITE_MEASURING_MODE),
    #[cfg(not(all(feature = "Win32_dcommon", feature = "Win32_dwrite")))]
    DrawGlyphRun: usize,
    pub SetTransform: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_numerics::Matrix3x2),
    pub GetTransform: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_numerics::Matrix3x2),
    pub SetAntialiasMode: unsafe extern "system" fn(*mut core::ffi::c_void, D2D1_ANTIALIAS_MODE),
    pub GetAntialiasMode: unsafe extern "system" fn(*mut core::ffi::c_void) -> D2D1_ANTIALIAS_MODE,
    pub SetTextAntialiasMode: unsafe extern "system" fn(*mut core::ffi::c_void, D2D1_TEXT_ANTIALIAS_MODE),
    pub GetTextAntialiasMode: unsafe extern "system" fn(*mut core::ffi::c_void) -> D2D1_TEXT_ANTIALIAS_MODE,
    #[cfg(feature = "Win32_dwrite")]
    pub SetTextRenderingParams: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void),
    #[cfg(not(feature = "Win32_dwrite"))]
    SetTextRenderingParams: usize,
    #[cfg(feature = "Win32_dwrite")]
    pub GetTextRenderingParams: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void),
    #[cfg(not(feature = "Win32_dwrite"))]
    GetTextRenderingParams: usize,
    pub SetTags: unsafe extern "system" fn(*mut core::ffi::c_void, D2D1_TAG, D2D1_TAG),
    pub GetTags: unsafe extern "system" fn(*mut core::ffi::c_void, *mut D2D1_TAG, *mut D2D1_TAG),
    #[cfg(feature = "Win32_dcommon")]
    pub PushLayer: unsafe extern "system" fn(*mut core::ffi::c_void, *const D2D1_LAYER_PARAMETERS, *mut core::ffi::c_void),
    #[cfg(not(feature = "Win32_dcommon"))]
    PushLayer: usize,
    pub PopLayer: unsafe extern "system" fn(*mut core::ffi::c_void),
    pub Flush: unsafe extern "system" fn(*mut core::ffi::c_void, *mut D2D1_TAG, *mut D2D1_TAG) -> windows_core::HRESULT,
    pub SaveDrawingState: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void),
    pub RestoreDrawingState: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void),
    #[cfg(feature = "Win32_dcommon")]
    pub PushAxisAlignedClip: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::dcommon::D2D_RECT_F, D2D1_ANTIALIAS_MODE),
    #[cfg(not(feature = "Win32_dcommon"))]
    PushAxisAlignedClip: usize,
    pub PopAxisAlignedClip: unsafe extern "system" fn(*mut core::ffi::c_void),
    #[cfg(all(feature = "Win32_d2dbasetypes", feature = "Win32_dxgitype"))]
    pub Clear: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::d2dbasetypes::D2D_COLOR_F),
    #[cfg(not(all(feature = "Win32_d2dbasetypes", feature = "Win32_dxgitype")))]
    Clear: usize,
    pub BeginDraw: unsafe extern "system" fn(*mut core::ffi::c_void),
    pub EndDraw: unsafe extern "system" fn(*mut core::ffi::c_void, *mut D2D1_TAG, *mut D2D1_TAG) -> windows_core::HRESULT,
    #[cfg(all(feature = "Win32_dcommon", feature = "Win32_dxgiformat"))]
    pub GetPixelFormat: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::dcommon::D2D1_PIXEL_FORMAT),
    #[cfg(not(all(feature = "Win32_dcommon", feature = "Win32_dxgiformat")))]
    GetPixelFormat: usize,
    pub SetDpi: unsafe extern "system" fn(*mut core::ffi::c_void, f32, f32),
    pub GetDpi: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f32, *mut f32),
    #[cfg(feature = "Win32_dcommon")]
    pub GetSize: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::dcommon::D2D_SIZE_F),
    #[cfg(not(feature = "Win32_dcommon"))]
    GetSize: usize,
    #[cfg(feature = "Win32_dcommon")]
    pub GetPixelSize: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::dcommon::D2D_SIZE_U),
    #[cfg(not(feature = "Win32_dcommon"))]
    GetPixelSize: usize,
    pub GetMaximumBitmapSize: unsafe extern "system" fn(*mut core::ffi::c_void) -> u32,
    #[cfg(all(feature = "Win32_dcommon", feature = "Win32_dxgiformat"))]
    pub IsSupported: unsafe extern "system" fn(*mut core::ffi::c_void, *const D2D1_RENDER_TARGET_PROPERTIES) -> windows_core::BOOL,
    #[cfg(not(all(feature = "Win32_dcommon", feature = "Win32_dxgiformat")))]
    IsSupported: usize,
}
#[cfg(all(feature = "Win32_d2dbasetypes", feature = "Win32_dcommon", feature = "Win32_dwrite", feature = "Win32_dxgiformat", feature = "Win32_dxgitype", feature = "Win32_wincodec"))]
pub trait ID2D1RenderTarget_Impl: ID2D1Resource_Impl {
    fn CreateBitmap(&self, size: &super::dcommon::D2D_SIZE_U, srcdata: *const core::ffi::c_void, pitch: u32, bitmapproperties: *const D2D1_BITMAP_PROPERTIES) -> windows_core::Result<ID2D1Bitmap>;
    fn CreateBitmapFromWicBitmap(&self, wicbitmapsource: windows_core::Ref<super::wincodec::IWICBitmapSource>, bitmapproperties: *const D2D1_BITMAP_PROPERTIES) -> windows_core::Result<ID2D1Bitmap>;
    fn CreateSharedBitmap(&self, riid: *const windows_core::GUID, data: *mut core::ffi::c_void, bitmapproperties: *const D2D1_BITMAP_PROPERTIES, bitmap: windows_core::OutRef<ID2D1Bitmap>) -> windows_core::Result<()>;
    fn CreateBitmapBrush(&self, bitmap: windows_core::Ref<ID2D1Bitmap>, bitmapbrushproperties: *const D2D1_BITMAP_BRUSH_PROPERTIES, brushproperties: *const D2D1_BRUSH_PROPERTIES) -> windows_core::Result<ID2D1BitmapBrush>;
    fn CreateSolidColorBrush(&self, color: *const super::d2dbasetypes::D2D_COLOR_F, brushproperties: *const D2D1_BRUSH_PROPERTIES) -> windows_core::Result<ID2D1SolidColorBrush>;
    fn CreateGradientStopCollection(&self, gradientstops: *const D2D1_GRADIENT_STOP, gradientstopscount: u32, colorinterpolationgamma: D2D1_GAMMA, extendmode: D2D1_EXTEND_MODE) -> windows_core::Result<ID2D1GradientStopCollection>;
    fn CreateLinearGradientBrush(&self, lineargradientbrushproperties: *const D2D1_LINEAR_GRADIENT_BRUSH_PROPERTIES, brushproperties: *const D2D1_BRUSH_PROPERTIES, gradientstopcollection: windows_core::Ref<ID2D1GradientStopCollection>) -> windows_core::Result<ID2D1LinearGradientBrush>;
    fn CreateRadialGradientBrush(&self, radialgradientbrushproperties: *const D2D1_RADIAL_GRADIENT_BRUSH_PROPERTIES, brushproperties: *const D2D1_BRUSH_PROPERTIES, gradientstopcollection: windows_core::Ref<ID2D1GradientStopCollection>) -> windows_core::Result<ID2D1RadialGradientBrush>;
    fn CreateCompatibleRenderTarget(&self, desiredsize: *const super::dcommon::D2D_SIZE_F, desiredpixelsize: *const super::dcommon::D2D_SIZE_U, desiredformat: *const super::dcommon::D2D1_PIXEL_FORMAT, options: D2D1_COMPATIBLE_RENDER_TARGET_OPTIONS) -> windows_core::Result<ID2D1BitmapRenderTarget>;
    fn CreateLayer(&self, size: *const super::dcommon::D2D_SIZE_F) -> windows_core::Result<ID2D1Layer>;
    fn CreateMesh(&self) -> windows_core::Result<ID2D1Mesh>;
    fn DrawLine(&self, point0: &windows_numerics::Vector2, point1: &windows_numerics::Vector2, brush: windows_core::Ref<ID2D1Brush>, strokewidth: f32, strokestyle: windows_core::Ref<ID2D1StrokeStyle>);
    fn DrawRectangle(&self, rect: *const super::dcommon::D2D_RECT_F, brush: windows_core::Ref<ID2D1Brush>, strokewidth: f32, strokestyle: windows_core::Ref<ID2D1StrokeStyle>);
    fn FillRectangle(&self, rect: *const super::dcommon::D2D_RECT_F, brush: windows_core::Ref<ID2D1Brush>);
    fn DrawRoundedRectangle(&self, roundedrect: *const D2D1_ROUNDED_RECT, brush: windows_core::Ref<ID2D1Brush>, strokewidth: f32, strokestyle: windows_core::Ref<ID2D1StrokeStyle>);
    fn FillRoundedRectangle(&self, roundedrect: *const D2D1_ROUNDED_RECT, brush: windows_core::Ref<ID2D1Brush>);
    fn DrawEllipse(&self, ellipse: *const D2D1_ELLIPSE, brush: windows_core::Ref<ID2D1Brush>, strokewidth: f32, strokestyle: windows_core::Ref<ID2D1StrokeStyle>);
    fn FillEllipse(&self, ellipse: *const D2D1_ELLIPSE, brush: windows_core::Ref<ID2D1Brush>);
    fn DrawGeometry(&self, geometry: windows_core::Ref<ID2D1Geometry>, brush: windows_core::Ref<ID2D1Brush>, strokewidth: f32, strokestyle: windows_core::Ref<ID2D1StrokeStyle>);
    fn FillGeometry(&self, geometry: windows_core::Ref<ID2D1Geometry>, brush: windows_core::Ref<ID2D1Brush>, opacitybrush: windows_core::Ref<ID2D1Brush>);
    fn FillMesh(&self, mesh: windows_core::Ref<ID2D1Mesh>, brush: windows_core::Ref<ID2D1Brush>);
    fn FillOpacityMask(&self, opacitymask: windows_core::Ref<ID2D1Bitmap>, brush: windows_core::Ref<ID2D1Brush>, content: D2D1_OPACITY_MASK_CONTENT, destinationrectangle: *const super::dcommon::D2D_RECT_F, sourcerectangle: *const super::dcommon::D2D_RECT_F);
    fn DrawBitmap(&self, bitmap: windows_core::Ref<ID2D1Bitmap>, destinationrectangle: *const super::dcommon::D2D_RECT_F, opacity: f32, interpolationmode: D2D1_BITMAP_INTERPOLATION_MODE, sourcerectangle: *const super::dcommon::D2D_RECT_F);
    fn DrawText(&self, string: *const u16, stringlength: u32, textformat: windows_core::Ref<super::dwrite::IDWriteTextFormat>, layoutrect: *const super::dcommon::D2D_RECT_F, defaultfillbrush: windows_core::Ref<ID2D1Brush>, options: D2D1_DRAW_TEXT_OPTIONS, measuringmode: super::dcommon::DWRITE_MEASURING_MODE);
    fn DrawTextLayout(&self, origin: &windows_numerics::Vector2, textlayout: windows_core::Ref<super::dwrite::IDWriteTextLayout>, defaultfillbrush: windows_core::Ref<ID2D1Brush>, options: D2D1_DRAW_TEXT_OPTIONS);
    fn DrawGlyphRun(&self, baselineorigin: &windows_numerics::Vector2, glyphrun: *const super::dwrite::DWRITE_GLYPH_RUN, foregroundbrush: windows_core::Ref<ID2D1Brush>, measuringmode: super::dcommon::DWRITE_MEASURING_MODE);
    fn SetTransform(&self, transform: *const windows_numerics::Matrix3x2);
    fn GetTransform(&self, transform: *mut windows_numerics::Matrix3x2);
    fn SetAntialiasMode(&self, antialiasmode: D2D1_ANTIALIAS_MODE);
    fn GetAntialiasMode(&self) -> D2D1_ANTIALIAS_MODE;
    fn SetTextAntialiasMode(&self, textantialiasmode: D2D1_TEXT_ANTIALIAS_MODE);
    fn GetTextAntialiasMode(&self) -> D2D1_TEXT_ANTIALIAS_MODE;
    fn SetTextRenderingParams(&self, textrenderingparams: windows_core::Ref<super::dwrite::IDWriteRenderingParams>);
    fn GetTextRenderingParams(&self, textrenderingparams: windows_core::OutRef<super::dwrite::IDWriteRenderingParams>);
    fn SetTags(&self, tag1: D2D1_TAG, tag2: D2D1_TAG);
    fn GetTags(&self, tag1: *mut D2D1_TAG, tag2: *mut D2D1_TAG);
    fn PushLayer(&self, layerparameters: *const D2D1_LAYER_PARAMETERS, layer: windows_core::Ref<ID2D1Layer>);
    fn PopLayer(&self);
    fn Flush(&self, tag1: *mut D2D1_TAG, tag2: *mut D2D1_TAG) -> windows_core::Result<()>;
    fn SaveDrawingState(&self, drawingstateblock: windows_core::OutRef<ID2D1DrawingStateBlock>);
    fn RestoreDrawingState(&self, drawingstateblock: windows_core::Ref<ID2D1DrawingStateBlock>);
    fn PushAxisAlignedClip(&self, cliprect: *const super::dcommon::D2D_RECT_F, antialiasmode: D2D1_ANTIALIAS_MODE);
    fn PopAxisAlignedClip(&self);
    fn Clear(&self, clearcolor: *const super::d2dbasetypes::D2D_COLOR_F);
    fn BeginDraw(&self);
    fn EndDraw(&self, tag1: *mut D2D1_TAG, tag2: *mut D2D1_TAG) -> windows_core::Result<()>;
    fn GetPixelFormat(&self) -> super::dcommon::D2D1_PIXEL_FORMAT;
    fn SetDpi(&self, dpix: f32, dpiy: f32);
    fn GetDpi(&self, dpix: *mut f32, dpiy: *mut f32);
    fn GetSize(&self) -> super::dcommon::D2D_SIZE_F;
    fn GetPixelSize(&self) -> super::dcommon::D2D_SIZE_U;
    fn GetMaximumBitmapSize(&self) -> u32;
    fn IsSupported(&self, rendertargetproperties: *const D2D1_RENDER_TARGET_PROPERTIES) -> windows_core::BOOL;
}
#[cfg(all(feature = "Win32_d2dbasetypes", feature = "Win32_dcommon", feature = "Win32_dwrite", feature = "Win32_dxgiformat", feature = "Win32_dxgitype", feature = "Win32_wincodec"))]
impl ID2D1RenderTarget_Vtbl {
    pub const fn new<Identity: ID2D1RenderTarget_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn CreateBitmap<Identity: ID2D1RenderTarget_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, size: super::dcommon::D2D_SIZE_U, srcdata: *const core::ffi::c_void, pitch: u32, bitmapproperties: *const D2D1_BITMAP_PROPERTIES, bitmap: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ID2D1RenderTarget_Impl::CreateBitmap(this, core::mem::transmute(&size), core::mem::transmute_copy(&srcdata), core::mem::transmute_copy(&pitch), core::mem::transmute_copy(&bitmapproperties)) {
                    Ok(ok__) => {
                        bitmap.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CreateBitmapFromWicBitmap<Identity: ID2D1RenderTarget_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, wicbitmapsource: *mut core::ffi::c_void, bitmapproperties: *const D2D1_BITMAP_PROPERTIES, bitmap: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ID2D1RenderTarget_Impl::CreateBitmapFromWicBitmap(this, core::mem::transmute_copy(&wicbitmapsource), core::mem::transmute_copy(&bitmapproperties)) {
                    Ok(ok__) => {
                        bitmap.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CreateSharedBitmap<Identity: ID2D1RenderTarget_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, riid: *const windows_core::GUID, data: *mut core::ffi::c_void, bitmapproperties: *const D2D1_BITMAP_PROPERTIES, bitmap: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID2D1RenderTarget_Impl::CreateSharedBitmap(this, core::mem::transmute_copy(&riid), core::mem::transmute_copy(&data), core::mem::transmute_copy(&bitmapproperties), core::mem::transmute_copy(&bitmap)).into()
            }
        }
        unsafe extern "system" fn CreateBitmapBrush<Identity: ID2D1RenderTarget_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bitmap: *mut core::ffi::c_void, bitmapbrushproperties: *const D2D1_BITMAP_BRUSH_PROPERTIES, brushproperties: *const D2D1_BRUSH_PROPERTIES, bitmapbrush: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ID2D1RenderTarget_Impl::CreateBitmapBrush(this, core::mem::transmute_copy(&bitmap), core::mem::transmute_copy(&bitmapbrushproperties), core::mem::transmute_copy(&brushproperties)) {
                    Ok(ok__) => {
                        bitmapbrush.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CreateSolidColorBrush<Identity: ID2D1RenderTarget_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, color: *const super::d2dbasetypes::D2D_COLOR_F, brushproperties: *const D2D1_BRUSH_PROPERTIES, solidcolorbrush: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ID2D1RenderTarget_Impl::CreateSolidColorBrush(this, core::mem::transmute_copy(&color), core::mem::transmute_copy(&brushproperties)) {
                    Ok(ok__) => {
                        solidcolorbrush.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CreateGradientStopCollection<Identity: ID2D1RenderTarget_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, gradientstops: *const D2D1_GRADIENT_STOP, gradientstopscount: u32, colorinterpolationgamma: D2D1_GAMMA, extendmode: D2D1_EXTEND_MODE, gradientstopcollection: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ID2D1RenderTarget_Impl::CreateGradientStopCollection(this, core::mem::transmute_copy(&gradientstops), core::mem::transmute_copy(&gradientstopscount), core::mem::transmute_copy(&colorinterpolationgamma), core::mem::transmute_copy(&extendmode)) {
                    Ok(ok__) => {
                        gradientstopcollection.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CreateLinearGradientBrush<Identity: ID2D1RenderTarget_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lineargradientbrushproperties: *const D2D1_LINEAR_GRADIENT_BRUSH_PROPERTIES, brushproperties: *const D2D1_BRUSH_PROPERTIES, gradientstopcollection: *mut core::ffi::c_void, lineargradientbrush: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ID2D1RenderTarget_Impl::CreateLinearGradientBrush(this, core::mem::transmute_copy(&lineargradientbrushproperties), core::mem::transmute_copy(&brushproperties), core::mem::transmute_copy(&gradientstopcollection)) {
                    Ok(ok__) => {
                        lineargradientbrush.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CreateRadialGradientBrush<Identity: ID2D1RenderTarget_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, radialgradientbrushproperties: *const D2D1_RADIAL_GRADIENT_BRUSH_PROPERTIES, brushproperties: *const D2D1_BRUSH_PROPERTIES, gradientstopcollection: *mut core::ffi::c_void, radialgradientbrush: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ID2D1RenderTarget_Impl::CreateRadialGradientBrush(this, core::mem::transmute_copy(&radialgradientbrushproperties), core::mem::transmute_copy(&brushproperties), core::mem::transmute_copy(&gradientstopcollection)) {
                    Ok(ok__) => {
                        radialgradientbrush.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CreateCompatibleRenderTarget<Identity: ID2D1RenderTarget_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, desiredsize: *const super::dcommon::D2D_SIZE_F, desiredpixelsize: *const super::dcommon::D2D_SIZE_U, desiredformat: *const super::dcommon::D2D1_PIXEL_FORMAT, options: D2D1_COMPATIBLE_RENDER_TARGET_OPTIONS, bitmaprendertarget: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ID2D1RenderTarget_Impl::CreateCompatibleRenderTarget(this, core::mem::transmute_copy(&desiredsize), core::mem::transmute_copy(&desiredpixelsize), core::mem::transmute_copy(&desiredformat), core::mem::transmute_copy(&options)) {
                    Ok(ok__) => {
                        bitmaprendertarget.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CreateLayer<Identity: ID2D1RenderTarget_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, size: *const super::dcommon::D2D_SIZE_F, layer: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ID2D1RenderTarget_Impl::CreateLayer(this, core::mem::transmute_copy(&size)) {
                    Ok(ok__) => {
                        layer.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CreateMesh<Identity: ID2D1RenderTarget_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, mesh: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ID2D1RenderTarget_Impl::CreateMesh(this) {
                    Ok(ok__) => {
                        mesh.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn DrawLine<Identity: ID2D1RenderTarget_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, point0: windows_numerics::Vector2, point1: windows_numerics::Vector2, brush: *mut core::ffi::c_void, strokewidth: f32, strokestyle: *mut core::ffi::c_void) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID2D1RenderTarget_Impl::DrawLine(this, core::mem::transmute(&point0), core::mem::transmute(&point1), core::mem::transmute_copy(&brush), core::mem::transmute_copy(&strokewidth), core::mem::transmute_copy(&strokestyle));
            }
        }
        unsafe extern "system" fn DrawRectangle<Identity: ID2D1RenderTarget_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, rect: *const super::dcommon::D2D_RECT_F, brush: *mut core::ffi::c_void, strokewidth: f32, strokestyle: *mut core::ffi::c_void) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID2D1RenderTarget_Impl::DrawRectangle(this, core::mem::transmute_copy(&rect), core::mem::transmute_copy(&brush), core::mem::transmute_copy(&strokewidth), core::mem::transmute_copy(&strokestyle));
            }
        }
        unsafe extern "system" fn FillRectangle<Identity: ID2D1RenderTarget_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, rect: *const super::dcommon::D2D_RECT_F, brush: *mut core::ffi::c_void) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID2D1RenderTarget_Impl::FillRectangle(this, core::mem::transmute_copy(&rect), core::mem::transmute_copy(&brush));
            }
        }
        unsafe extern "system" fn DrawRoundedRectangle<Identity: ID2D1RenderTarget_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, roundedrect: *const D2D1_ROUNDED_RECT, brush: *mut core::ffi::c_void, strokewidth: f32, strokestyle: *mut core::ffi::c_void) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID2D1RenderTarget_Impl::DrawRoundedRectangle(this, core::mem::transmute_copy(&roundedrect), core::mem::transmute_copy(&brush), core::mem::transmute_copy(&strokewidth), core::mem::transmute_copy(&strokestyle));
            }
        }
        unsafe extern "system" fn FillRoundedRectangle<Identity: ID2D1RenderTarget_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, roundedrect: *const D2D1_ROUNDED_RECT, brush: *mut core::ffi::c_void) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID2D1RenderTarget_Impl::FillRoundedRectangle(this, core::mem::transmute_copy(&roundedrect), core::mem::transmute_copy(&brush));
            }
        }
        unsafe extern "system" fn DrawEllipse<Identity: ID2D1RenderTarget_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ellipse: *const D2D1_ELLIPSE, brush: *mut core::ffi::c_void, strokewidth: f32, strokestyle: *mut core::ffi::c_void) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID2D1RenderTarget_Impl::DrawEllipse(this, core::mem::transmute_copy(&ellipse), core::mem::transmute_copy(&brush), core::mem::transmute_copy(&strokewidth), core::mem::transmute_copy(&strokestyle));
            }
        }
        unsafe extern "system" fn FillEllipse<Identity: ID2D1RenderTarget_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ellipse: *const D2D1_ELLIPSE, brush: *mut core::ffi::c_void) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID2D1RenderTarget_Impl::FillEllipse(this, core::mem::transmute_copy(&ellipse), core::mem::transmute_copy(&brush));
            }
        }
        unsafe extern "system" fn DrawGeometry<Identity: ID2D1RenderTarget_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, geometry: *mut core::ffi::c_void, brush: *mut core::ffi::c_void, strokewidth: f32, strokestyle: *mut core::ffi::c_void) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID2D1RenderTarget_Impl::DrawGeometry(this, core::mem::transmute_copy(&geometry), core::mem::transmute_copy(&brush), core::mem::transmute_copy(&strokewidth), core::mem::transmute_copy(&strokestyle));
            }
        }
        unsafe extern "system" fn FillGeometry<Identity: ID2D1RenderTarget_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, geometry: *mut core::ffi::c_void, brush: *mut core::ffi::c_void, opacitybrush: *mut core::ffi::c_void) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID2D1RenderTarget_Impl::FillGeometry(this, core::mem::transmute_copy(&geometry), core::mem::transmute_copy(&brush), core::mem::transmute_copy(&opacitybrush));
            }
        }
        unsafe extern "system" fn FillMesh<Identity: ID2D1RenderTarget_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, mesh: *mut core::ffi::c_void, brush: *mut core::ffi::c_void) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID2D1RenderTarget_Impl::FillMesh(this, core::mem::transmute_copy(&mesh), core::mem::transmute_copy(&brush));
            }
        }
        unsafe extern "system" fn FillOpacityMask<Identity: ID2D1RenderTarget_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, opacitymask: *mut core::ffi::c_void, brush: *mut core::ffi::c_void, content: D2D1_OPACITY_MASK_CONTENT, destinationrectangle: *const super::dcommon::D2D_RECT_F, sourcerectangle: *const super::dcommon::D2D_RECT_F) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID2D1RenderTarget_Impl::FillOpacityMask(this, core::mem::transmute_copy(&opacitymask), core::mem::transmute_copy(&brush), core::mem::transmute_copy(&content), core::mem::transmute_copy(&destinationrectangle), core::mem::transmute_copy(&sourcerectangle));
            }
        }
        unsafe extern "system" fn DrawBitmap<Identity: ID2D1RenderTarget_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bitmap: *mut core::ffi::c_void, destinationrectangle: *const super::dcommon::D2D_RECT_F, opacity: f32, interpolationmode: D2D1_BITMAP_INTERPOLATION_MODE, sourcerectangle: *const super::dcommon::D2D_RECT_F) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID2D1RenderTarget_Impl::DrawBitmap(this, core::mem::transmute_copy(&bitmap), core::mem::transmute_copy(&destinationrectangle), core::mem::transmute_copy(&opacity), core::mem::transmute_copy(&interpolationmode), core::mem::transmute_copy(&sourcerectangle));
            }
        }
        unsafe extern "system" fn DrawText<Identity: ID2D1RenderTarget_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, string: *const u16, stringlength: u32, textformat: *mut core::ffi::c_void, layoutrect: *const super::dcommon::D2D_RECT_F, defaultfillbrush: *mut core::ffi::c_void, options: D2D1_DRAW_TEXT_OPTIONS, measuringmode: super::dcommon::DWRITE_MEASURING_MODE) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID2D1RenderTarget_Impl::DrawText(this, core::mem::transmute_copy(&string), core::mem::transmute_copy(&stringlength), core::mem::transmute_copy(&textformat), core::mem::transmute_copy(&layoutrect), core::mem::transmute_copy(&defaultfillbrush), core::mem::transmute_copy(&options), core::mem::transmute_copy(&measuringmode));
            }
        }
        unsafe extern "system" fn DrawTextLayout<Identity: ID2D1RenderTarget_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, origin: windows_numerics::Vector2, textlayout: *mut core::ffi::c_void, defaultfillbrush: *mut core::ffi::c_void, options: D2D1_DRAW_TEXT_OPTIONS) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID2D1RenderTarget_Impl::DrawTextLayout(this, core::mem::transmute(&origin), core::mem::transmute_copy(&textlayout), core::mem::transmute_copy(&defaultfillbrush), core::mem::transmute_copy(&options));
            }
        }
        unsafe extern "system" fn DrawGlyphRun<Identity: ID2D1RenderTarget_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, baselineorigin: windows_numerics::Vector2, glyphrun: *const super::dwrite::DWRITE_GLYPH_RUN, foregroundbrush: *mut core::ffi::c_void, measuringmode: super::dcommon::DWRITE_MEASURING_MODE) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID2D1RenderTarget_Impl::DrawGlyphRun(this, core::mem::transmute(&baselineorigin), core::mem::transmute_copy(&glyphrun), core::mem::transmute_copy(&foregroundbrush), core::mem::transmute_copy(&measuringmode));
            }
        }
        unsafe extern "system" fn SetTransform<Identity: ID2D1RenderTarget_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, transform: *const windows_numerics::Matrix3x2) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID2D1RenderTarget_Impl::SetTransform(this, core::mem::transmute_copy(&transform));
            }
        }
        unsafe extern "system" fn GetTransform<Identity: ID2D1RenderTarget_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, transform: *mut windows_numerics::Matrix3x2) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID2D1RenderTarget_Impl::GetTransform(this, core::mem::transmute_copy(&transform));
            }
        }
        unsafe extern "system" fn SetAntialiasMode<Identity: ID2D1RenderTarget_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, antialiasmode: D2D1_ANTIALIAS_MODE) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID2D1RenderTarget_Impl::SetAntialiasMode(this, core::mem::transmute_copy(&antialiasmode));
            }
        }
        unsafe extern "system" fn GetAntialiasMode<Identity: ID2D1RenderTarget_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> D2D1_ANTIALIAS_MODE {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID2D1RenderTarget_Impl::GetAntialiasMode(this)
            }
        }
        unsafe extern "system" fn SetTextAntialiasMode<Identity: ID2D1RenderTarget_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, textantialiasmode: D2D1_TEXT_ANTIALIAS_MODE) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID2D1RenderTarget_Impl::SetTextAntialiasMode(this, core::mem::transmute_copy(&textantialiasmode));
            }
        }
        unsafe extern "system" fn GetTextAntialiasMode<Identity: ID2D1RenderTarget_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> D2D1_TEXT_ANTIALIAS_MODE {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID2D1RenderTarget_Impl::GetTextAntialiasMode(this)
            }
        }
        unsafe extern "system" fn SetTextRenderingParams<Identity: ID2D1RenderTarget_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, textrenderingparams: *mut core::ffi::c_void) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID2D1RenderTarget_Impl::SetTextRenderingParams(this, core::mem::transmute_copy(&textrenderingparams));
            }
        }
        unsafe extern "system" fn GetTextRenderingParams<Identity: ID2D1RenderTarget_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, textrenderingparams: *mut *mut core::ffi::c_void) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID2D1RenderTarget_Impl::GetTextRenderingParams(this, core::mem::transmute_copy(&textrenderingparams));
            }
        }
        unsafe extern "system" fn SetTags<Identity: ID2D1RenderTarget_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, tag1: D2D1_TAG, tag2: D2D1_TAG) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID2D1RenderTarget_Impl::SetTags(this, core::mem::transmute_copy(&tag1), core::mem::transmute_copy(&tag2));
            }
        }
        unsafe extern "system" fn GetTags<Identity: ID2D1RenderTarget_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, tag1: *mut D2D1_TAG, tag2: *mut D2D1_TAG) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID2D1RenderTarget_Impl::GetTags(this, core::mem::transmute_copy(&tag1), core::mem::transmute_copy(&tag2));
            }
        }
        unsafe extern "system" fn PushLayer<Identity: ID2D1RenderTarget_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, layerparameters: *const D2D1_LAYER_PARAMETERS, layer: *mut core::ffi::c_void) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID2D1RenderTarget_Impl::PushLayer(this, core::mem::transmute_copy(&layerparameters), core::mem::transmute_copy(&layer));
            }
        }
        unsafe extern "system" fn PopLayer<Identity: ID2D1RenderTarget_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID2D1RenderTarget_Impl::PopLayer(this);
            }
        }
        unsafe extern "system" fn Flush<Identity: ID2D1RenderTarget_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, tag1: *mut D2D1_TAG, tag2: *mut D2D1_TAG) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID2D1RenderTarget_Impl::Flush(this, core::mem::transmute_copy(&tag1), core::mem::transmute_copy(&tag2)).into()
            }
        }
        unsafe extern "system" fn SaveDrawingState<Identity: ID2D1RenderTarget_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, drawingstateblock: *mut core::ffi::c_void) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID2D1RenderTarget_Impl::SaveDrawingState(this, core::mem::transmute(&drawingstateblock));
            }
        }
        unsafe extern "system" fn RestoreDrawingState<Identity: ID2D1RenderTarget_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, drawingstateblock: *mut core::ffi::c_void) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID2D1RenderTarget_Impl::RestoreDrawingState(this, core::mem::transmute_copy(&drawingstateblock));
            }
        }
        unsafe extern "system" fn PushAxisAlignedClip<Identity: ID2D1RenderTarget_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, cliprect: *const super::dcommon::D2D_RECT_F, antialiasmode: D2D1_ANTIALIAS_MODE) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID2D1RenderTarget_Impl::PushAxisAlignedClip(this, core::mem::transmute_copy(&cliprect), core::mem::transmute_copy(&antialiasmode));
            }
        }
        unsafe extern "system" fn PopAxisAlignedClip<Identity: ID2D1RenderTarget_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID2D1RenderTarget_Impl::PopAxisAlignedClip(this);
            }
        }
        unsafe extern "system" fn Clear<Identity: ID2D1RenderTarget_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, clearcolor: *const super::d2dbasetypes::D2D_COLOR_F) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID2D1RenderTarget_Impl::Clear(this, core::mem::transmute_copy(&clearcolor));
            }
        }
        unsafe extern "system" fn BeginDraw<Identity: ID2D1RenderTarget_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID2D1RenderTarget_Impl::BeginDraw(this);
            }
        }
        unsafe extern "system" fn EndDraw<Identity: ID2D1RenderTarget_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, tag1: *mut D2D1_TAG, tag2: *mut D2D1_TAG) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID2D1RenderTarget_Impl::EndDraw(this, core::mem::transmute_copy(&tag1), core::mem::transmute_copy(&tag2)).into()
            }
        }
        unsafe extern "system" fn GetPixelFormat<Identity: ID2D1RenderTarget_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut super::dcommon::D2D1_PIXEL_FORMAT) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                *result__ = ID2D1RenderTarget_Impl::GetPixelFormat(this);
            }
        }
        unsafe extern "system" fn SetDpi<Identity: ID2D1RenderTarget_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dpix: f32, dpiy: f32) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID2D1RenderTarget_Impl::SetDpi(this, core::mem::transmute_copy(&dpix), core::mem::transmute_copy(&dpiy));
            }
        }
        unsafe extern "system" fn GetDpi<Identity: ID2D1RenderTarget_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dpix: *mut f32, dpiy: *mut f32) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID2D1RenderTarget_Impl::GetDpi(this, core::mem::transmute_copy(&dpix), core::mem::transmute_copy(&dpiy));
            }
        }
        unsafe extern "system" fn GetSize<Identity: ID2D1RenderTarget_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut super::dcommon::D2D_SIZE_F) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                *result__ = ID2D1RenderTarget_Impl::GetSize(this);
            }
        }
        unsafe extern "system" fn GetPixelSize<Identity: ID2D1RenderTarget_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut super::dcommon::D2D_SIZE_U) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                *result__ = ID2D1RenderTarget_Impl::GetPixelSize(this);
            }
        }
        unsafe extern "system" fn GetMaximumBitmapSize<Identity: ID2D1RenderTarget_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> u32 {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID2D1RenderTarget_Impl::GetMaximumBitmapSize(this)
            }
        }
        unsafe extern "system" fn IsSupported<Identity: ID2D1RenderTarget_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, rendertargetproperties: *const D2D1_RENDER_TARGET_PROPERTIES) -> windows_core::BOOL {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID2D1RenderTarget_Impl::IsSupported(this, core::mem::transmute_copy(&rendertargetproperties))
            }
        }
        Self {
            base__: ID2D1Resource_Vtbl::new::<Identity, OFFSET>(),
            CreateBitmap: CreateBitmap::<Identity, OFFSET>,
            CreateBitmapFromWicBitmap: CreateBitmapFromWicBitmap::<Identity, OFFSET>,
            CreateSharedBitmap: CreateSharedBitmap::<Identity, OFFSET>,
            CreateBitmapBrush: CreateBitmapBrush::<Identity, OFFSET>,
            CreateSolidColorBrush: CreateSolidColorBrush::<Identity, OFFSET>,
            CreateGradientStopCollection: CreateGradientStopCollection::<Identity, OFFSET>,
            CreateLinearGradientBrush: CreateLinearGradientBrush::<Identity, OFFSET>,
            CreateRadialGradientBrush: CreateRadialGradientBrush::<Identity, OFFSET>,
            CreateCompatibleRenderTarget: CreateCompatibleRenderTarget::<Identity, OFFSET>,
            CreateLayer: CreateLayer::<Identity, OFFSET>,
            CreateMesh: CreateMesh::<Identity, OFFSET>,
            DrawLine: DrawLine::<Identity, OFFSET>,
            DrawRectangle: DrawRectangle::<Identity, OFFSET>,
            FillRectangle: FillRectangle::<Identity, OFFSET>,
            DrawRoundedRectangle: DrawRoundedRectangle::<Identity, OFFSET>,
            FillRoundedRectangle: FillRoundedRectangle::<Identity, OFFSET>,
            DrawEllipse: DrawEllipse::<Identity, OFFSET>,
            FillEllipse: FillEllipse::<Identity, OFFSET>,
            DrawGeometry: DrawGeometry::<Identity, OFFSET>,
            FillGeometry: FillGeometry::<Identity, OFFSET>,
            FillMesh: FillMesh::<Identity, OFFSET>,
            FillOpacityMask: FillOpacityMask::<Identity, OFFSET>,
            DrawBitmap: DrawBitmap::<Identity, OFFSET>,
            DrawText: DrawText::<Identity, OFFSET>,
            DrawTextLayout: DrawTextLayout::<Identity, OFFSET>,
            DrawGlyphRun: DrawGlyphRun::<Identity, OFFSET>,
            SetTransform: SetTransform::<Identity, OFFSET>,
            GetTransform: GetTransform::<Identity, OFFSET>,
            SetAntialiasMode: SetAntialiasMode::<Identity, OFFSET>,
            GetAntialiasMode: GetAntialiasMode::<Identity, OFFSET>,
            SetTextAntialiasMode: SetTextAntialiasMode::<Identity, OFFSET>,
            GetTextAntialiasMode: GetTextAntialiasMode::<Identity, OFFSET>,
            SetTextRenderingParams: SetTextRenderingParams::<Identity, OFFSET>,
            GetTextRenderingParams: GetTextRenderingParams::<Identity, OFFSET>,
            SetTags: SetTags::<Identity, OFFSET>,
            GetTags: GetTags::<Identity, OFFSET>,
            PushLayer: PushLayer::<Identity, OFFSET>,
            PopLayer: PopLayer::<Identity, OFFSET>,
            Flush: Flush::<Identity, OFFSET>,
            SaveDrawingState: SaveDrawingState::<Identity, OFFSET>,
            RestoreDrawingState: RestoreDrawingState::<Identity, OFFSET>,
            PushAxisAlignedClip: PushAxisAlignedClip::<Identity, OFFSET>,
            PopAxisAlignedClip: PopAxisAlignedClip::<Identity, OFFSET>,
            Clear: Clear::<Identity, OFFSET>,
            BeginDraw: BeginDraw::<Identity, OFFSET>,
            EndDraw: EndDraw::<Identity, OFFSET>,
            GetPixelFormat: GetPixelFormat::<Identity, OFFSET>,
            SetDpi: SetDpi::<Identity, OFFSET>,
            GetDpi: GetDpi::<Identity, OFFSET>,
            GetSize: GetSize::<Identity, OFFSET>,
            GetPixelSize: GetPixelSize::<Identity, OFFSET>,
            GetMaximumBitmapSize: GetMaximumBitmapSize::<Identity, OFFSET>,
            IsSupported: IsSupported::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ID2D1RenderTarget as windows_core::Interface>::IID || iid == &<ID2D1Resource as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_d2dbasetypes", feature = "Win32_dcommon", feature = "Win32_dwrite", feature = "Win32_dxgiformat", feature = "Win32_dxgitype", feature = "Win32_wincodec"))]
impl windows_core::RuntimeName for ID2D1RenderTarget {}
windows_core::imp::define_interface!(ID2D1Resource, ID2D1Resource_Vtbl, 0x2cd90691_12e2_11dc_9fed_001143a055f9);
windows_core::imp::interface_hierarchy!(ID2D1Resource, windows_core::IUnknown);
impl ID2D1Resource {
    pub unsafe fn GetFactory(&self) -> windows_core::Result<ID2D1Factory> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetFactory)(windows_core::Interface::as_raw(self), &mut result__);
            windows_core::Type::from_abi(result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ID2D1Resource_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetFactory: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void),
}
pub trait ID2D1Resource_Impl: windows_core::IUnknownImpl {
    fn GetFactory(&self, factory: windows_core::OutRef<ID2D1Factory>);
}
impl ID2D1Resource_Vtbl {
    pub const fn new<Identity: ID2D1Resource_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetFactory<Identity: ID2D1Resource_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, factory: *mut *mut core::ffi::c_void) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID2D1Resource_Impl::GetFactory(this, core::mem::transmute_copy(&factory));
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), GetFactory: GetFactory::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ID2D1Resource as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for ID2D1Resource {}
windows_core::imp::define_interface!(ID2D1RoundedRectangleGeometry, ID2D1RoundedRectangleGeometry_Vtbl, 0x2cd906a3_12e2_11dc_9fed_001143a055f9);
impl core::ops::Deref for ID2D1RoundedRectangleGeometry {
    type Target = ID2D1Geometry;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ID2D1RoundedRectangleGeometry, windows_core::IUnknown, ID2D1Resource, ID2D1Geometry);
impl ID2D1RoundedRectangleGeometry {
    #[cfg(feature = "Win32_dcommon")]
    pub unsafe fn GetRoundedRect(&self, roundedrect: *mut D2D1_ROUNDED_RECT) {
        unsafe {
            (windows_core::Interface::vtable(self).GetRoundedRect)(windows_core::Interface::as_raw(self), roundedrect as _);
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ID2D1RoundedRectangleGeometry_Vtbl {
    pub base__: ID2D1Geometry_Vtbl,
    #[cfg(feature = "Win32_dcommon")]
    pub GetRoundedRect: unsafe extern "system" fn(*mut core::ffi::c_void, *mut D2D1_ROUNDED_RECT),
    #[cfg(not(feature = "Win32_dcommon"))]
    GetRoundedRect: usize,
}
#[cfg(feature = "Win32_dcommon")]
pub trait ID2D1RoundedRectangleGeometry_Impl: ID2D1Geometry_Impl {
    fn GetRoundedRect(&self, roundedrect: *mut D2D1_ROUNDED_RECT);
}
#[cfg(feature = "Win32_dcommon")]
impl ID2D1RoundedRectangleGeometry_Vtbl {
    pub const fn new<Identity: ID2D1RoundedRectangleGeometry_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetRoundedRect<Identity: ID2D1RoundedRectangleGeometry_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, roundedrect: *mut D2D1_ROUNDED_RECT) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID2D1RoundedRectangleGeometry_Impl::GetRoundedRect(this, core::mem::transmute_copy(&roundedrect));
            }
        }
        Self { base__: ID2D1Geometry_Vtbl::new::<Identity, OFFSET>(), GetRoundedRect: GetRoundedRect::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ID2D1RoundedRectangleGeometry as windows_core::Interface>::IID || iid == &<ID2D1Resource as windows_core::Interface>::IID || iid == &<ID2D1Geometry as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_dcommon")]
impl windows_core::RuntimeName for ID2D1RoundedRectangleGeometry {}
windows_core::imp::define_interface!(ID2D1SimplifiedGeometrySink, ID2D1SimplifiedGeometrySink_Vtbl, 0x2cd9069e_12e2_11dc_9fed_001143a055f9);
windows_core::imp::interface_hierarchy!(ID2D1SimplifiedGeometrySink, windows_core::IUnknown);
impl ID2D1SimplifiedGeometrySink {
    pub unsafe fn SetFillMode(&self, fillmode: D2D1_FILL_MODE) {
        unsafe {
            (windows_core::Interface::vtable(self).SetFillMode)(windows_core::Interface::as_raw(self), fillmode);
        }
    }
    pub unsafe fn SetSegmentFlags(&self, vertexflags: D2D1_PATH_SEGMENT) {
        unsafe {
            (windows_core::Interface::vtable(self).SetSegmentFlags)(windows_core::Interface::as_raw(self), vertexflags);
        }
    }
    pub unsafe fn BeginFigure(&self, startpoint: windows_numerics::Vector2, figurebegin: D2D1_FIGURE_BEGIN) {
        unsafe {
            (windows_core::Interface::vtable(self).BeginFigure)(windows_core::Interface::as_raw(self), core::mem::transmute(startpoint), figurebegin);
        }
    }
    pub unsafe fn AddLines(&self, points: &[windows_numerics::Vector2]) {
        unsafe {
            (windows_core::Interface::vtable(self).AddLines)(windows_core::Interface::as_raw(self), core::mem::transmute(points.as_ptr()), points.len().try_into().unwrap());
        }
    }
    pub unsafe fn AddBeziers(&self, beziers: &[D2D1_BEZIER_SEGMENT]) {
        unsafe {
            (windows_core::Interface::vtable(self).AddBeziers)(windows_core::Interface::as_raw(self), core::mem::transmute(beziers.as_ptr()), beziers.len().try_into().unwrap());
        }
    }
    pub unsafe fn EndFigure(&self, figureend: D2D1_FIGURE_END) {
        unsafe {
            (windows_core::Interface::vtable(self).EndFigure)(windows_core::Interface::as_raw(self), figureend);
        }
    }
    pub unsafe fn Close(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Close)(windows_core::Interface::as_raw(self)) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ID2D1SimplifiedGeometrySink_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub SetFillMode: unsafe extern "system" fn(*mut core::ffi::c_void, D2D1_FILL_MODE),
    pub SetSegmentFlags: unsafe extern "system" fn(*mut core::ffi::c_void, D2D1_PATH_SEGMENT),
    pub BeginFigure: unsafe extern "system" fn(*mut core::ffi::c_void, windows_numerics::Vector2, D2D1_FIGURE_BEGIN),
    pub AddLines: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_numerics::Vector2, u32),
    pub AddBeziers: unsafe extern "system" fn(*mut core::ffi::c_void, *const D2D1_BEZIER_SEGMENT, u32),
    pub EndFigure: unsafe extern "system" fn(*mut core::ffi::c_void, D2D1_FIGURE_END),
    pub Close: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait ID2D1SimplifiedGeometrySink_Impl: windows_core::IUnknownImpl {
    fn SetFillMode(&self, fillmode: D2D1_FILL_MODE);
    fn SetSegmentFlags(&self, vertexflags: D2D1_PATH_SEGMENT);
    fn BeginFigure(&self, startpoint: &windows_numerics::Vector2, figurebegin: D2D1_FIGURE_BEGIN);
    fn AddLines(&self, points: *const windows_numerics::Vector2, pointscount: u32);
    fn AddBeziers(&self, beziers: *const D2D1_BEZIER_SEGMENT, bezierscount: u32);
    fn EndFigure(&self, figureend: D2D1_FIGURE_END);
    fn Close(&self) -> windows_core::Result<()>;
}
impl ID2D1SimplifiedGeometrySink_Vtbl {
    pub const fn new<Identity: ID2D1SimplifiedGeometrySink_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn SetFillMode<Identity: ID2D1SimplifiedGeometrySink_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, fillmode: D2D1_FILL_MODE) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID2D1SimplifiedGeometrySink_Impl::SetFillMode(this, core::mem::transmute_copy(&fillmode));
            }
        }
        unsafe extern "system" fn SetSegmentFlags<Identity: ID2D1SimplifiedGeometrySink_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, vertexflags: D2D1_PATH_SEGMENT) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID2D1SimplifiedGeometrySink_Impl::SetSegmentFlags(this, core::mem::transmute_copy(&vertexflags));
            }
        }
        unsafe extern "system" fn BeginFigure<Identity: ID2D1SimplifiedGeometrySink_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, startpoint: windows_numerics::Vector2, figurebegin: D2D1_FIGURE_BEGIN) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID2D1SimplifiedGeometrySink_Impl::BeginFigure(this, core::mem::transmute(&startpoint), core::mem::transmute_copy(&figurebegin));
            }
        }
        unsafe extern "system" fn AddLines<Identity: ID2D1SimplifiedGeometrySink_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, points: *const windows_numerics::Vector2, pointscount: u32) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID2D1SimplifiedGeometrySink_Impl::AddLines(this, core::mem::transmute_copy(&points), core::mem::transmute_copy(&pointscount));
            }
        }
        unsafe extern "system" fn AddBeziers<Identity: ID2D1SimplifiedGeometrySink_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, beziers: *const D2D1_BEZIER_SEGMENT, bezierscount: u32) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID2D1SimplifiedGeometrySink_Impl::AddBeziers(this, core::mem::transmute_copy(&beziers), core::mem::transmute_copy(&bezierscount));
            }
        }
        unsafe extern "system" fn EndFigure<Identity: ID2D1SimplifiedGeometrySink_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, figureend: D2D1_FIGURE_END) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID2D1SimplifiedGeometrySink_Impl::EndFigure(this, core::mem::transmute_copy(&figureend));
            }
        }
        unsafe extern "system" fn Close<Identity: ID2D1SimplifiedGeometrySink_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID2D1SimplifiedGeometrySink_Impl::Close(this).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            SetFillMode: SetFillMode::<Identity, OFFSET>,
            SetSegmentFlags: SetSegmentFlags::<Identity, OFFSET>,
            BeginFigure: BeginFigure::<Identity, OFFSET>,
            AddLines: AddLines::<Identity, OFFSET>,
            AddBeziers: AddBeziers::<Identity, OFFSET>,
            EndFigure: EndFigure::<Identity, OFFSET>,
            Close: Close::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ID2D1SimplifiedGeometrySink as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for ID2D1SimplifiedGeometrySink {}
windows_core::imp::define_interface!(ID2D1SolidColorBrush, ID2D1SolidColorBrush_Vtbl, 0x2cd906a9_12e2_11dc_9fed_001143a055f9);
impl core::ops::Deref for ID2D1SolidColorBrush {
    type Target = ID2D1Brush;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ID2D1SolidColorBrush, windows_core::IUnknown, ID2D1Resource, ID2D1Brush);
impl ID2D1SolidColorBrush {
    #[cfg(all(feature = "Win32_d2dbasetypes", feature = "Win32_dxgitype"))]
    pub unsafe fn SetColor(&self, color: *const super::d2dbasetypes::D2D_COLOR_F) {
        unsafe {
            (windows_core::Interface::vtable(self).SetColor)(windows_core::Interface::as_raw(self), color);
        }
    }
    #[cfg(all(feature = "Win32_d2dbasetypes", feature = "Win32_dxgitype"))]
    pub unsafe fn GetColor(&self) -> super::d2dbasetypes::D2D_COLOR_F {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetColor)(windows_core::Interface::as_raw(self), &mut result__);
            result__
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ID2D1SolidColorBrush_Vtbl {
    pub base__: ID2D1Brush_Vtbl,
    #[cfg(all(feature = "Win32_d2dbasetypes", feature = "Win32_dxgitype"))]
    pub SetColor: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::d2dbasetypes::D2D_COLOR_F),
    #[cfg(not(all(feature = "Win32_d2dbasetypes", feature = "Win32_dxgitype")))]
    SetColor: usize,
    #[cfg(all(feature = "Win32_d2dbasetypes", feature = "Win32_dxgitype"))]
    pub GetColor: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::d2dbasetypes::D2D_COLOR_F),
    #[cfg(not(all(feature = "Win32_d2dbasetypes", feature = "Win32_dxgitype")))]
    GetColor: usize,
}
#[cfg(all(feature = "Win32_d2dbasetypes", feature = "Win32_dxgitype"))]
pub trait ID2D1SolidColorBrush_Impl: ID2D1Brush_Impl {
    fn SetColor(&self, color: *const super::d2dbasetypes::D2D_COLOR_F);
    fn GetColor(&self) -> super::d2dbasetypes::D2D_COLOR_F;
}
#[cfg(all(feature = "Win32_d2dbasetypes", feature = "Win32_dxgitype"))]
impl ID2D1SolidColorBrush_Vtbl {
    pub const fn new<Identity: ID2D1SolidColorBrush_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn SetColor<Identity: ID2D1SolidColorBrush_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, color: *const super::d2dbasetypes::D2D_COLOR_F) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID2D1SolidColorBrush_Impl::SetColor(this, core::mem::transmute_copy(&color));
            }
        }
        unsafe extern "system" fn GetColor<Identity: ID2D1SolidColorBrush_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut super::d2dbasetypes::D2D_COLOR_F) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                *result__ = ID2D1SolidColorBrush_Impl::GetColor(this);
            }
        }
        Self { base__: ID2D1Brush_Vtbl::new::<Identity, OFFSET>(), SetColor: SetColor::<Identity, OFFSET>, GetColor: GetColor::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ID2D1SolidColorBrush as windows_core::Interface>::IID || iid == &<ID2D1Resource as windows_core::Interface>::IID || iid == &<ID2D1Brush as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_d2dbasetypes", feature = "Win32_dxgitype"))]
impl windows_core::RuntimeName for ID2D1SolidColorBrush {}
windows_core::imp::define_interface!(ID2D1StrokeStyle, ID2D1StrokeStyle_Vtbl, 0x2cd9069d_12e2_11dc_9fed_001143a055f9);
impl core::ops::Deref for ID2D1StrokeStyle {
    type Target = ID2D1Resource;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ID2D1StrokeStyle, windows_core::IUnknown, ID2D1Resource);
impl ID2D1StrokeStyle {
    pub unsafe fn GetStartCap(&self) -> D2D1_CAP_STYLE {
        unsafe { (windows_core::Interface::vtable(self).GetStartCap)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn GetEndCap(&self) -> D2D1_CAP_STYLE {
        unsafe { (windows_core::Interface::vtable(self).GetEndCap)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn GetDashCap(&self) -> D2D1_CAP_STYLE {
        unsafe { (windows_core::Interface::vtable(self).GetDashCap)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn GetMiterLimit(&self) -> f32 {
        unsafe { (windows_core::Interface::vtable(self).GetMiterLimit)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn GetLineJoin(&self) -> D2D1_LINE_JOIN {
        unsafe { (windows_core::Interface::vtable(self).GetLineJoin)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn GetDashOffset(&self) -> f32 {
        unsafe { (windows_core::Interface::vtable(self).GetDashOffset)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn GetDashStyle(&self) -> D2D1_DASH_STYLE {
        unsafe { (windows_core::Interface::vtable(self).GetDashStyle)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn GetDashesCount(&self) -> u32 {
        unsafe { (windows_core::Interface::vtable(self).GetDashesCount)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn GetDashes(&self, dashes: &mut [f32]) {
        unsafe {
            (windows_core::Interface::vtable(self).GetDashes)(windows_core::Interface::as_raw(self), core::mem::transmute(dashes.as_ptr()), dashes.len().try_into().unwrap());
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ID2D1StrokeStyle_Vtbl {
    pub base__: ID2D1Resource_Vtbl,
    pub GetStartCap: unsafe extern "system" fn(*mut core::ffi::c_void) -> D2D1_CAP_STYLE,
    pub GetEndCap: unsafe extern "system" fn(*mut core::ffi::c_void) -> D2D1_CAP_STYLE,
    pub GetDashCap: unsafe extern "system" fn(*mut core::ffi::c_void) -> D2D1_CAP_STYLE,
    pub GetMiterLimit: unsafe extern "system" fn(*mut core::ffi::c_void) -> f32,
    pub GetLineJoin: unsafe extern "system" fn(*mut core::ffi::c_void) -> D2D1_LINE_JOIN,
    pub GetDashOffset: unsafe extern "system" fn(*mut core::ffi::c_void) -> f32,
    pub GetDashStyle: unsafe extern "system" fn(*mut core::ffi::c_void) -> D2D1_DASH_STYLE,
    pub GetDashesCount: unsafe extern "system" fn(*mut core::ffi::c_void) -> u32,
    pub GetDashes: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f32, u32),
}
pub trait ID2D1StrokeStyle_Impl: ID2D1Resource_Impl {
    fn GetStartCap(&self) -> D2D1_CAP_STYLE;
    fn GetEndCap(&self) -> D2D1_CAP_STYLE;
    fn GetDashCap(&self) -> D2D1_CAP_STYLE;
    fn GetMiterLimit(&self) -> f32;
    fn GetLineJoin(&self) -> D2D1_LINE_JOIN;
    fn GetDashOffset(&self) -> f32;
    fn GetDashStyle(&self) -> D2D1_DASH_STYLE;
    fn GetDashesCount(&self) -> u32;
    fn GetDashes(&self, dashes: *mut f32, dashescount: u32);
}
impl ID2D1StrokeStyle_Vtbl {
    pub const fn new<Identity: ID2D1StrokeStyle_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetStartCap<Identity: ID2D1StrokeStyle_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> D2D1_CAP_STYLE {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID2D1StrokeStyle_Impl::GetStartCap(this)
            }
        }
        unsafe extern "system" fn GetEndCap<Identity: ID2D1StrokeStyle_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> D2D1_CAP_STYLE {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID2D1StrokeStyle_Impl::GetEndCap(this)
            }
        }
        unsafe extern "system" fn GetDashCap<Identity: ID2D1StrokeStyle_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> D2D1_CAP_STYLE {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID2D1StrokeStyle_Impl::GetDashCap(this)
            }
        }
        unsafe extern "system" fn GetMiterLimit<Identity: ID2D1StrokeStyle_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> f32 {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID2D1StrokeStyle_Impl::GetMiterLimit(this)
            }
        }
        unsafe extern "system" fn GetLineJoin<Identity: ID2D1StrokeStyle_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> D2D1_LINE_JOIN {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID2D1StrokeStyle_Impl::GetLineJoin(this)
            }
        }
        unsafe extern "system" fn GetDashOffset<Identity: ID2D1StrokeStyle_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> f32 {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID2D1StrokeStyle_Impl::GetDashOffset(this)
            }
        }
        unsafe extern "system" fn GetDashStyle<Identity: ID2D1StrokeStyle_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> D2D1_DASH_STYLE {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID2D1StrokeStyle_Impl::GetDashStyle(this)
            }
        }
        unsafe extern "system" fn GetDashesCount<Identity: ID2D1StrokeStyle_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> u32 {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID2D1StrokeStyle_Impl::GetDashesCount(this)
            }
        }
        unsafe extern "system" fn GetDashes<Identity: ID2D1StrokeStyle_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dashes: *mut f32, dashescount: u32) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID2D1StrokeStyle_Impl::GetDashes(this, core::mem::transmute_copy(&dashes), core::mem::transmute_copy(&dashescount));
            }
        }
        Self {
            base__: ID2D1Resource_Vtbl::new::<Identity, OFFSET>(),
            GetStartCap: GetStartCap::<Identity, OFFSET>,
            GetEndCap: GetEndCap::<Identity, OFFSET>,
            GetDashCap: GetDashCap::<Identity, OFFSET>,
            GetMiterLimit: GetMiterLimit::<Identity, OFFSET>,
            GetLineJoin: GetLineJoin::<Identity, OFFSET>,
            GetDashOffset: GetDashOffset::<Identity, OFFSET>,
            GetDashStyle: GetDashStyle::<Identity, OFFSET>,
            GetDashesCount: GetDashesCount::<Identity, OFFSET>,
            GetDashes: GetDashes::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ID2D1StrokeStyle as windows_core::Interface>::IID || iid == &<ID2D1Resource as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for ID2D1StrokeStyle {}
windows_core::imp::define_interface!(ID2D1TessellationSink, ID2D1TessellationSink_Vtbl, 0x2cd906c1_12e2_11dc_9fed_001143a055f9);
windows_core::imp::interface_hierarchy!(ID2D1TessellationSink, windows_core::IUnknown);
impl ID2D1TessellationSink {
    pub unsafe fn AddTriangles(&self, triangles: &[D2D1_TRIANGLE]) {
        unsafe {
            (windows_core::Interface::vtable(self).AddTriangles)(windows_core::Interface::as_raw(self), core::mem::transmute(triangles.as_ptr()), triangles.len().try_into().unwrap());
        }
    }
    pub unsafe fn Close(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Close)(windows_core::Interface::as_raw(self)) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ID2D1TessellationSink_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub AddTriangles: unsafe extern "system" fn(*mut core::ffi::c_void, *const D2D1_TRIANGLE, u32),
    pub Close: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait ID2D1TessellationSink_Impl: windows_core::IUnknownImpl {
    fn AddTriangles(&self, triangles: *const D2D1_TRIANGLE, trianglescount: u32);
    fn Close(&self) -> windows_core::Result<()>;
}
impl ID2D1TessellationSink_Vtbl {
    pub const fn new<Identity: ID2D1TessellationSink_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn AddTriangles<Identity: ID2D1TessellationSink_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, triangles: *const D2D1_TRIANGLE, trianglescount: u32) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID2D1TessellationSink_Impl::AddTriangles(this, core::mem::transmute_copy(&triangles), core::mem::transmute_copy(&trianglescount));
            }
        }
        unsafe extern "system" fn Close<Identity: ID2D1TessellationSink_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID2D1TessellationSink_Impl::Close(this).into()
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), AddTriangles: AddTriangles::<Identity, OFFSET>, Close: Close::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ID2D1TessellationSink as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for ID2D1TessellationSink {}
windows_core::imp::define_interface!(ID2D1TransformedGeometry, ID2D1TransformedGeometry_Vtbl, 0x2cd906bb_12e2_11dc_9fed_001143a055f9);
impl core::ops::Deref for ID2D1TransformedGeometry {
    type Target = ID2D1Geometry;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ID2D1TransformedGeometry, windows_core::IUnknown, ID2D1Resource, ID2D1Geometry);
impl ID2D1TransformedGeometry {
    pub unsafe fn GetSourceGeometry(&self) -> windows_core::Result<ID2D1Geometry> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetSourceGeometry)(windows_core::Interface::as_raw(self), &mut result__);
            windows_core::Type::from_abi(result__)
        }
    }
    pub unsafe fn GetTransform(&self, transform: *mut windows_numerics::Matrix3x2) {
        unsafe {
            (windows_core::Interface::vtable(self).GetTransform)(windows_core::Interface::as_raw(self), transform as _);
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ID2D1TransformedGeometry_Vtbl {
    pub base__: ID2D1Geometry_Vtbl,
    pub GetSourceGeometry: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void),
    pub GetTransform: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_numerics::Matrix3x2),
}
#[cfg(feature = "Win32_dcommon")]
pub trait ID2D1TransformedGeometry_Impl: ID2D1Geometry_Impl {
    fn GetSourceGeometry(&self, sourcegeometry: windows_core::OutRef<ID2D1Geometry>);
    fn GetTransform(&self, transform: *mut windows_numerics::Matrix3x2);
}
#[cfg(feature = "Win32_dcommon")]
impl ID2D1TransformedGeometry_Vtbl {
    pub const fn new<Identity: ID2D1TransformedGeometry_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetSourceGeometry<Identity: ID2D1TransformedGeometry_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, sourcegeometry: *mut *mut core::ffi::c_void) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID2D1TransformedGeometry_Impl::GetSourceGeometry(this, core::mem::transmute_copy(&sourcegeometry));
            }
        }
        unsafe extern "system" fn GetTransform<Identity: ID2D1TransformedGeometry_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, transform: *mut windows_numerics::Matrix3x2) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID2D1TransformedGeometry_Impl::GetTransform(this, core::mem::transmute_copy(&transform));
            }
        }
        Self {
            base__: ID2D1Geometry_Vtbl::new::<Identity, OFFSET>(),
            GetSourceGeometry: GetSourceGeometry::<Identity, OFFSET>,
            GetTransform: GetTransform::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ID2D1TransformedGeometry as windows_core::Interface>::IID || iid == &<ID2D1Resource as windows_core::Interface>::IID || iid == &<ID2D1Geometry as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_dcommon")]
impl windows_core::RuntimeName for ID2D1TransformedGeometry {}
