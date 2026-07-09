#[cfg(all(feature = "Win32_d2dbasetypes", feature = "Win32_dxgitype"))]
windows_link::link!("d2d1.dll" "system" fn D2D1ConvertColorSpace(sourcecolorspace : D2D1_COLOR_SPACE, destinationcolorspace : D2D1_COLOR_SPACE, color : *const super::d2dbasetypes::D2D_COLOR_F) -> super::d2dbasetypes::D2D_COLOR_F);
#[cfg(all(feature = "Win32_d2d1", feature = "Win32_dxgi"))]
windows_link::link!("d2d1.dll" "system" fn D2D1CreateDevice(dxgidevice : *mut core::ffi::c_void, creationproperties : *const D2D1_CREATION_PROPERTIES, d2ddevice : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "Win32_d2d1", feature = "Win32_dxgi"))]
windows_link::link!("d2d1.dll" "system" fn D2D1CreateDeviceContext(dxgisurface : *mut core::ffi::c_void, creationproperties : *const D2D1_CREATION_PROPERTIES, d2ddevicecontext : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
windows_link::link!("d2d1.dll" "system" fn D2D1SinCos(angle : f32, s : *mut f32, c : *mut f32));
windows_link::link!("d2d1.dll" "system" fn D2D1Tan(angle : f32) -> f32);
windows_link::link!("d2d1.dll" "system" fn D2D1Vec3Length(x : f32, y : f32, z : f32) -> f32);
#[repr(C)]
#[cfg(feature = "Win32_d2d1")]
#[derive(Clone, Copy, Default)]
pub struct D2D1_BITMAP_BRUSH_PROPERTIES1 {
    pub extendModeX: super::d2d1::D2D1_EXTEND_MODE,
    pub extendModeY: super::d2d1::D2D1_EXTEND_MODE,
    pub interpolationMode: D2D1_INTERPOLATION_MODE,
}
pub type D2D1_BITMAP_OPTIONS = u32;
pub const D2D1_BITMAP_OPTIONS_CANNOT_DRAW: D2D1_BITMAP_OPTIONS = 2;
pub const D2D1_BITMAP_OPTIONS_CPU_READ: D2D1_BITMAP_OPTIONS = 4;
pub const D2D1_BITMAP_OPTIONS_FORCE_DWORD: D2D1_BITMAP_OPTIONS = 4294967295;
pub const D2D1_BITMAP_OPTIONS_GDI_COMPATIBLE: D2D1_BITMAP_OPTIONS = 8;
pub const D2D1_BITMAP_OPTIONS_NONE: D2D1_BITMAP_OPTIONS = 0;
pub const D2D1_BITMAP_OPTIONS_TARGET: D2D1_BITMAP_OPTIONS = 1;
#[repr(C)]
#[cfg(all(feature = "Win32_d2d1", feature = "Win32_dcommon", feature = "Win32_dxgiformat"))]
#[derive(Clone, Copy)]
pub struct D2D1_BITMAP_PROPERTIES1 {
    pub pixelFormat: super::dcommon::D2D1_PIXEL_FORMAT,
    pub dpiX: f32,
    pub dpiY: f32,
    pub bitmapOptions: D2D1_BITMAP_OPTIONS,
    pub colorContext: *mut core::ffi::c_void,
}
#[cfg(all(feature = "Win32_d2d1", feature = "Win32_dcommon", feature = "Win32_dxgiformat"))]
impl Default for D2D1_BITMAP_PROPERTIES1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type D2D1_BUFFER_PRECISION = i32;
pub const D2D1_BUFFER_PRECISION_16BPC_FLOAT: D2D1_BUFFER_PRECISION = 4;
pub const D2D1_BUFFER_PRECISION_16BPC_UNORM: D2D1_BUFFER_PRECISION = 3;
pub const D2D1_BUFFER_PRECISION_32BPC_FLOAT: D2D1_BUFFER_PRECISION = 5;
pub const D2D1_BUFFER_PRECISION_8BPC_UNORM: D2D1_BUFFER_PRECISION = 1;
pub const D2D1_BUFFER_PRECISION_8BPC_UNORM_SRGB: D2D1_BUFFER_PRECISION = 2;
pub const D2D1_BUFFER_PRECISION_FORCE_DWORD: D2D1_BUFFER_PRECISION = -1;
pub const D2D1_BUFFER_PRECISION_UNKNOWN: D2D1_BUFFER_PRECISION = 0;
pub type D2D1_COLOR_INTERPOLATION_MODE = i32;
pub const D2D1_COLOR_INTERPOLATION_MODE_FORCE_DWORD: D2D1_COLOR_INTERPOLATION_MODE = -1;
pub const D2D1_COLOR_INTERPOLATION_MODE_PREMULTIPLIED: D2D1_COLOR_INTERPOLATION_MODE = 1;
pub const D2D1_COLOR_INTERPOLATION_MODE_STRAIGHT: D2D1_COLOR_INTERPOLATION_MODE = 0;
pub type D2D1_COLOR_SPACE = i32;
pub const D2D1_COLOR_SPACE_CUSTOM: D2D1_COLOR_SPACE = 0;
pub const D2D1_COLOR_SPACE_FORCE_DWORD: D2D1_COLOR_SPACE = -1;
pub const D2D1_COLOR_SPACE_SCRGB: D2D1_COLOR_SPACE = 2;
pub const D2D1_COLOR_SPACE_SRGB: D2D1_COLOR_SPACE = 1;
pub type D2D1_COMPOSITE_MODE = i32;
pub const D2D1_COMPOSITE_MODE_BOUNDED_SOURCE_COPY: D2D1_COMPOSITE_MODE = 11;
pub const D2D1_COMPOSITE_MODE_DESTINATION_ATOP: D2D1_COMPOSITE_MODE = 7;
pub const D2D1_COMPOSITE_MODE_DESTINATION_IN: D2D1_COMPOSITE_MODE = 3;
pub const D2D1_COMPOSITE_MODE_DESTINATION_OUT: D2D1_COMPOSITE_MODE = 5;
pub const D2D1_COMPOSITE_MODE_DESTINATION_OVER: D2D1_COMPOSITE_MODE = 1;
pub const D2D1_COMPOSITE_MODE_FORCE_DWORD: D2D1_COMPOSITE_MODE = -1;
pub const D2D1_COMPOSITE_MODE_MASK_INVERT: D2D1_COMPOSITE_MODE = 12;
pub const D2D1_COMPOSITE_MODE_PLUS: D2D1_COMPOSITE_MODE = 9;
pub const D2D1_COMPOSITE_MODE_SOURCE_ATOP: D2D1_COMPOSITE_MODE = 6;
pub const D2D1_COMPOSITE_MODE_SOURCE_COPY: D2D1_COMPOSITE_MODE = 10;
pub const D2D1_COMPOSITE_MODE_SOURCE_IN: D2D1_COMPOSITE_MODE = 2;
pub const D2D1_COMPOSITE_MODE_SOURCE_OUT: D2D1_COMPOSITE_MODE = 4;
pub const D2D1_COMPOSITE_MODE_SOURCE_OVER: D2D1_COMPOSITE_MODE = 0;
pub const D2D1_COMPOSITE_MODE_XOR: D2D1_COMPOSITE_MODE = 8;
#[repr(C)]
#[cfg(feature = "Win32_d2d1")]
#[derive(Clone, Copy, Default)]
pub struct D2D1_CREATION_PROPERTIES {
    pub threadingMode: D2D1_THREADING_MODE,
    pub debugLevel: super::d2d1::D2D1_DEBUG_LEVEL,
    pub options: D2D1_DEVICE_CONTEXT_OPTIONS,
}
pub type D2D1_DEVICE_CONTEXT_OPTIONS = u32;
pub const D2D1_DEVICE_CONTEXT_OPTIONS_ENABLE_MULTITHREADED_OPTIMIZATIONS: D2D1_DEVICE_CONTEXT_OPTIONS = 1;
pub const D2D1_DEVICE_CONTEXT_OPTIONS_FORCE_DWORD: D2D1_DEVICE_CONTEXT_OPTIONS = 4294967295;
pub const D2D1_DEVICE_CONTEXT_OPTIONS_NONE: D2D1_DEVICE_CONTEXT_OPTIONS = 0;
#[repr(C)]
#[cfg(all(feature = "Foundation_Numerics", feature = "Win32_d2d1"))]
#[derive(Clone, Copy, Default)]
pub struct D2D1_DRAWING_STATE_DESCRIPTION1 {
    pub antialiasMode: super::d2d1::D2D1_ANTIALIAS_MODE,
    pub textAntialiasMode: super::d2d1::D2D1_TEXT_ANTIALIAS_MODE,
    pub tag1: super::d2d1::D2D1_TAG,
    pub tag2: super::d2d1::D2D1_TAG,
    pub transform: super::super::Foundation::Numerics::Matrix3x2,
    pub primitiveBlend: D2D1_PRIMITIVE_BLEND,
    pub unitMode: D2D1_UNIT_MODE,
}
#[repr(C)]
#[cfg(feature = "Win32_dcommon")]
#[derive(Clone, Copy)]
pub struct D2D1_EFFECT_INPUT_DESCRIPTION {
    pub effect: *mut core::ffi::c_void,
    pub inputIndex: u32,
    pub inputRectangle: super::dcommon::D2D_RECT_F,
}
#[cfg(feature = "Win32_dcommon")]
impl Default for D2D1_EFFECT_INPUT_DESCRIPTION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_d2d1", feature = "Win32_dcommon"))]
#[derive(Clone, Copy, Default)]
pub struct D2D1_IMAGE_BRUSH_PROPERTIES {
    pub sourceRectangle: super::dcommon::D2D_RECT_F,
    pub extendModeX: super::d2d1::D2D1_EXTEND_MODE,
    pub extendModeY: super::d2d1::D2D1_EXTEND_MODE,
    pub interpolationMode: D2D1_INTERPOLATION_MODE,
}
pub type D2D1_INTERPOLATION_MODE = i32;
pub const D2D1_INTERPOLATION_MODE_ANISOTROPIC: D2D1_INTERPOLATION_MODE = 4;
pub const D2D1_INTERPOLATION_MODE_CUBIC: D2D1_INTERPOLATION_MODE = 2;
pub const D2D1_INTERPOLATION_MODE_FORCE_DWORD: D2D1_INTERPOLATION_MODE = -1;
pub const D2D1_INTERPOLATION_MODE_HIGH_QUALITY_CUBIC: D2D1_INTERPOLATION_MODE = 5;
pub const D2D1_INTERPOLATION_MODE_LINEAR: D2D1_INTERPOLATION_MODE = 1;
pub const D2D1_INTERPOLATION_MODE_MULTI_SAMPLE_LINEAR: D2D1_INTERPOLATION_MODE = 3;
pub const D2D1_INTERPOLATION_MODE_NEAREST_NEIGHBOR: D2D1_INTERPOLATION_MODE = 0;
pub const D2D1_INVALID_PROPERTY_INDEX: i32 = -1;
pub type D2D1_LAYER_OPTIONS1 = u32;
pub const D2D1_LAYER_OPTIONS1_FORCE_DWORD: D2D1_LAYER_OPTIONS1 = 4294967295;
pub const D2D1_LAYER_OPTIONS1_IGNORE_ALPHA: D2D1_LAYER_OPTIONS1 = 2;
pub const D2D1_LAYER_OPTIONS1_INITIALIZE_FROM_BACKGROUND: D2D1_LAYER_OPTIONS1 = 1;
pub const D2D1_LAYER_OPTIONS1_NONE: D2D1_LAYER_OPTIONS1 = 0;
#[repr(C)]
#[cfg(all(feature = "Foundation_Numerics", feature = "Win32_d2d1", feature = "Win32_dcommon"))]
#[derive(Clone, Copy)]
pub struct D2D1_LAYER_PARAMETERS1 {
    pub contentBounds: super::dcommon::D2D_RECT_F,
    pub geometricMask: *mut core::ffi::c_void,
    pub maskAntialiasMode: super::d2d1::D2D1_ANTIALIAS_MODE,
    pub maskTransform: super::super::Foundation::Numerics::Matrix3x2,
    pub opacity: f32,
    pub opacityBrush: *mut core::ffi::c_void,
    pub layerOptions: D2D1_LAYER_OPTIONS1,
}
#[cfg(all(feature = "Foundation_Numerics", feature = "Win32_d2d1", feature = "Win32_dcommon"))]
impl Default for D2D1_LAYER_PARAMETERS1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D2D1_MAPPED_RECT {
    pub pitch: u32,
    pub bits: *mut u8,
}
impl Default for D2D1_MAPPED_RECT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type D2D1_MAP_OPTIONS = u32;
pub const D2D1_MAP_OPTIONS_DISCARD: D2D1_MAP_OPTIONS = 4;
pub const D2D1_MAP_OPTIONS_FORCE_DWORD: D2D1_MAP_OPTIONS = 4294967295;
pub const D2D1_MAP_OPTIONS_NONE: D2D1_MAP_OPTIONS = 0;
pub const D2D1_MAP_OPTIONS_READ: D2D1_MAP_OPTIONS = 1;
pub const D2D1_MAP_OPTIONS_WRITE: D2D1_MAP_OPTIONS = 2;
#[repr(C)]
#[cfg(feature = "Foundation_Numerics")]
#[derive(Clone, Copy, Default)]
pub struct D2D1_POINT_DESCRIPTION {
    pub point: super::super::Foundation::Numerics::Vector2,
    pub unitTangentVector: super::super::Foundation::Numerics::Vector2,
    pub endSegment: u32,
    pub endFigure: u32,
    pub lengthToEndSegment: f32,
}
pub type D2D1_PRIMITIVE_BLEND = i32;
pub const D2D1_PRIMITIVE_BLEND_ADD: D2D1_PRIMITIVE_BLEND = 3;
pub const D2D1_PRIMITIVE_BLEND_COPY: D2D1_PRIMITIVE_BLEND = 1;
pub const D2D1_PRIMITIVE_BLEND_FORCE_DWORD: D2D1_PRIMITIVE_BLEND = -1;
pub const D2D1_PRIMITIVE_BLEND_MAX: D2D1_PRIMITIVE_BLEND = 4;
pub const D2D1_PRIMITIVE_BLEND_MIN: D2D1_PRIMITIVE_BLEND = 2;
pub const D2D1_PRIMITIVE_BLEND_SOURCE_OVER: D2D1_PRIMITIVE_BLEND = 0;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct D2D1_PRINT_CONTROL_PROPERTIES {
    pub fontSubset: D2D1_PRINT_FONT_SUBSET_MODE,
    pub rasterDPI: f32,
    pub colorSpace: D2D1_COLOR_SPACE,
}
pub type D2D1_PRINT_FONT_SUBSET_MODE = i32;
pub const D2D1_PRINT_FONT_SUBSET_MODE_DEFAULT: D2D1_PRINT_FONT_SUBSET_MODE = 0;
pub const D2D1_PRINT_FONT_SUBSET_MODE_EACHPAGE: D2D1_PRINT_FONT_SUBSET_MODE = 1;
pub const D2D1_PRINT_FONT_SUBSET_MODE_FORCE_DWORD: D2D1_PRINT_FONT_SUBSET_MODE = -1;
pub const D2D1_PRINT_FONT_SUBSET_MODE_NONE: D2D1_PRINT_FONT_SUBSET_MODE = 2;
pub type D2D1_PROPERTY = i32;
pub const D2D1_PROPERTY_AUTHOR: D2D1_PROPERTY = -2147483646;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct D2D1_PROPERTY_BINDING(pub u8);
pub const D2D1_PROPERTY_CACHED: D2D1_PROPERTY = -2147483642;
pub const D2D1_PROPERTY_CATEGORY: D2D1_PROPERTY = -2147483645;
pub const D2D1_PROPERTY_CLSID: D2D1_PROPERTY = -2147483648;
pub const D2D1_PROPERTY_DESCRIPTION: D2D1_PROPERTY = -2147483644;
pub const D2D1_PROPERTY_DISPLAYNAME: D2D1_PROPERTY = -2147483647;
pub const D2D1_PROPERTY_FORCE_DWORD: D2D1_PROPERTY = -1;
pub const D2D1_PROPERTY_INPUTS: D2D1_PROPERTY = -2147483643;
pub const D2D1_PROPERTY_MAX_INPUTS: D2D1_PROPERTY = -2147483639;
pub const D2D1_PROPERTY_MIN_INPUTS: D2D1_PROPERTY = -2147483640;
pub const D2D1_PROPERTY_PRECISION: D2D1_PROPERTY = -2147483641;
pub type D2D1_PROPERTY_TYPE = i32;
pub const D2D1_PROPERTY_TYPE_ARRAY: D2D1_PROPERTY_TYPE = 12;
pub const D2D1_PROPERTY_TYPE_BLOB: D2D1_PROPERTY_TYPE = 9;
pub const D2D1_PROPERTY_TYPE_BOOL: D2D1_PROPERTY_TYPE = 2;
pub const D2D1_PROPERTY_TYPE_CLSID: D2D1_PROPERTY_TYPE = 13;
pub const D2D1_PROPERTY_TYPE_COLOR_CONTEXT: D2D1_PROPERTY_TYPE = 18;
pub const D2D1_PROPERTY_TYPE_ENUM: D2D1_PROPERTY_TYPE = 11;
pub const D2D1_PROPERTY_TYPE_FLOAT: D2D1_PROPERTY_TYPE = 5;
pub const D2D1_PROPERTY_TYPE_FORCE_DWORD: D2D1_PROPERTY_TYPE = -1;
pub const D2D1_PROPERTY_TYPE_INT32: D2D1_PROPERTY_TYPE = 4;
pub const D2D1_PROPERTY_TYPE_IUNKNOWN: D2D1_PROPERTY_TYPE = 10;
pub const D2D1_PROPERTY_TYPE_MATRIX_3X2: D2D1_PROPERTY_TYPE = 14;
pub const D2D1_PROPERTY_TYPE_MATRIX_4X3: D2D1_PROPERTY_TYPE = 15;
pub const D2D1_PROPERTY_TYPE_MATRIX_4X4: D2D1_PROPERTY_TYPE = 16;
pub const D2D1_PROPERTY_TYPE_MATRIX_5X4: D2D1_PROPERTY_TYPE = 17;
pub const D2D1_PROPERTY_TYPE_STRING: D2D1_PROPERTY_TYPE = 1;
pub const D2D1_PROPERTY_TYPE_UINT32: D2D1_PROPERTY_TYPE = 3;
pub const D2D1_PROPERTY_TYPE_UNKNOWN: D2D1_PROPERTY_TYPE = 0;
pub const D2D1_PROPERTY_TYPE_VECTOR2: D2D1_PROPERTY_TYPE = 6;
pub const D2D1_PROPERTY_TYPE_VECTOR3: D2D1_PROPERTY_TYPE = 7;
pub const D2D1_PROPERTY_TYPE_VECTOR4: D2D1_PROPERTY_TYPE = 8;
#[repr(C)]
#[cfg(feature = "Win32_dcommon")]
#[derive(Clone, Copy, Default)]
pub struct D2D1_RENDERING_CONTROLS {
    pub bufferPrecision: D2D1_BUFFER_PRECISION,
    pub tileSize: super::dcommon::D2D_SIZE_U,
}
#[repr(C)]
#[cfg(feature = "Win32_d2d1")]
#[derive(Clone, Copy, Default)]
pub struct D2D1_STROKE_STYLE_PROPERTIES1 {
    pub startCap: super::d2d1::D2D1_CAP_STYLE,
    pub endCap: super::d2d1::D2D1_CAP_STYLE,
    pub dashCap: super::d2d1::D2D1_CAP_STYLE,
    pub lineJoin: super::d2d1::D2D1_LINE_JOIN,
    pub miterLimit: f32,
    pub dashStyle: super::d2d1::D2D1_DASH_STYLE,
    pub dashOffset: f32,
    pub transformType: D2D1_STROKE_TRANSFORM_TYPE,
}
pub type D2D1_STROKE_TRANSFORM_TYPE = i32;
pub const D2D1_STROKE_TRANSFORM_TYPE_FIXED: D2D1_STROKE_TRANSFORM_TYPE = 1;
pub const D2D1_STROKE_TRANSFORM_TYPE_FORCE_DWORD: D2D1_STROKE_TRANSFORM_TYPE = -1;
pub const D2D1_STROKE_TRANSFORM_TYPE_HAIRLINE: D2D1_STROKE_TRANSFORM_TYPE = 2;
pub const D2D1_STROKE_TRANSFORM_TYPE_NORMAL: D2D1_STROKE_TRANSFORM_TYPE = 0;
pub type D2D1_SUBPROPERTY = i32;
pub const D2D1_SUBPROPERTY_DEFAULT: D2D1_SUBPROPERTY = -2147483644;
pub const D2D1_SUBPROPERTY_DISPLAYNAME: D2D1_SUBPROPERTY = -2147483648;
pub const D2D1_SUBPROPERTY_FIELDS: D2D1_SUBPROPERTY = -2147483643;
pub const D2D1_SUBPROPERTY_FORCE_DWORD: D2D1_SUBPROPERTY = -1;
pub const D2D1_SUBPROPERTY_INDEX: D2D1_SUBPROPERTY = -2147483642;
pub const D2D1_SUBPROPERTY_ISREADONLY: D2D1_SUBPROPERTY = -2147483647;
pub const D2D1_SUBPROPERTY_MAX: D2D1_SUBPROPERTY = -2147483645;
pub const D2D1_SUBPROPERTY_MIN: D2D1_SUBPROPERTY = -2147483646;
pub type D2D1_THREADING_MODE = i32;
pub const D2D1_THREADING_MODE_FORCE_DWORD: D2D1_THREADING_MODE = -1;
pub const D2D1_THREADING_MODE_MULTI_THREADED: D2D1_THREADING_MODE = 1;
pub const D2D1_THREADING_MODE_SINGLE_THREADED: D2D1_THREADING_MODE = 0;
pub type D2D1_UNIT_MODE = i32;
pub const D2D1_UNIT_MODE_DIPS: D2D1_UNIT_MODE = 0;
pub const D2D1_UNIT_MODE_FORCE_DWORD: D2D1_UNIT_MODE = -1;
pub const D2D1_UNIT_MODE_PIXELS: D2D1_UNIT_MODE = 1;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct IPrintDocumentPackageTarget(pub u8);
pub type PD2D1_EFFECT_FACTORY = Option<unsafe extern "system" fn(effectimpl: *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT>;
