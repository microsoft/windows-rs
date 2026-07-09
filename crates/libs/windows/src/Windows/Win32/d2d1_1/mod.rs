#[cfg(all(feature = "Win32_d2dbasetypes", feature = "Win32_dxgitype"))]
#[inline]
pub unsafe fn D2D1ConvertColorSpace(sourcecolorspace: D2D1_COLOR_SPACE, destinationcolorspace: D2D1_COLOR_SPACE, color: *const super::d2dbasetypes::D2D_COLOR_F) -> super::d2dbasetypes::D2D_COLOR_F {
    windows_core::link!("d2d1.dll" "system" fn D2D1ConvertColorSpace(sourcecolorspace : D2D1_COLOR_SPACE, destinationcolorspace : D2D1_COLOR_SPACE, color : *const super::d2dbasetypes::D2D_COLOR_F) -> super::d2dbasetypes::D2D_COLOR_F);
    unsafe { D2D1ConvertColorSpace(sourcecolorspace, destinationcolorspace, color) }
}
#[cfg(all(feature = "Win32_d2d1", feature = "Win32_dxgi"))]
#[inline]
pub unsafe fn D2D1CreateDevice<P0>(dxgidevice: P0, creationproperties: Option<*const D2D1_CREATION_PROPERTIES>) -> windows_core::Result<ID2D1Device>
where
    P0: windows_core::Param<super::dxgi::IDXGIDevice>,
{
    windows_core::link!("d2d1.dll" "system" fn D2D1CreateDevice(dxgidevice : *mut core::ffi::c_void, creationproperties : *const D2D1_CREATION_PROPERTIES, d2ddevice : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        D2D1CreateDevice(dxgidevice.param().abi(), creationproperties.unwrap_or(core::mem::zeroed()) as _, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[cfg(all(feature = "Win32_d2d1", feature = "Win32_dxgi"))]
#[inline]
pub unsafe fn D2D1CreateDeviceContext<P0>(dxgisurface: P0, creationproperties: Option<*const D2D1_CREATION_PROPERTIES>) -> windows_core::Result<ID2D1DeviceContext>
where
    P0: windows_core::Param<super::dxgi::IDXGISurface>,
{
    windows_core::link!("d2d1.dll" "system" fn D2D1CreateDeviceContext(dxgisurface : *mut core::ffi::c_void, creationproperties : *const D2D1_CREATION_PROPERTIES, d2ddevicecontext : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        D2D1CreateDeviceContext(dxgisurface.param().abi(), creationproperties.unwrap_or(core::mem::zeroed()) as _, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[inline]
pub unsafe fn D2D1SinCos(angle: f32, s: *mut f32, c: *mut f32) {
    windows_core::link!("d2d1.dll" "system" fn D2D1SinCos(angle : f32, s : *mut f32, c : *mut f32));
    unsafe { D2D1SinCos(angle, s as _, c as _) }
}
#[inline]
pub unsafe fn D2D1Tan(angle: f32) -> f32 {
    windows_core::link!("d2d1.dll" "system" fn D2D1Tan(angle : f32) -> f32);
    unsafe { D2D1Tan(angle) }
}
#[inline]
pub unsafe fn D2D1Vec3Length(x: f32, y: f32, z: f32) -> f32 {
    windows_core::link!("d2d1.dll" "system" fn D2D1Vec3Length(x : f32, y : f32, z : f32) -> f32);
    unsafe { D2D1Vec3Length(x, y, z) }
}
#[repr(C)]
#[cfg(feature = "Win32_d2d1")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
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
#[derive(Clone, Debug, Default, PartialEq)]
pub struct D2D1_BITMAP_PROPERTIES1 {
    pub pixelFormat: super::dcommon::D2D1_PIXEL_FORMAT,
    pub dpiX: f32,
    pub dpiY: f32,
    pub bitmapOptions: D2D1_BITMAP_OPTIONS,
    pub colorContext: core::mem::ManuallyDrop<Option<ID2D1ColorContext>>,
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
#[derive(Clone, Copy, Debug, Default, PartialEq)]
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
#[cfg(feature = "Win32_d2d1")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct D2D1_DRAWING_STATE_DESCRIPTION1 {
    pub antialiasMode: super::d2d1::D2D1_ANTIALIAS_MODE,
    pub textAntialiasMode: super::d2d1::D2D1_TEXT_ANTIALIAS_MODE,
    pub tag1: super::d2d1::D2D1_TAG,
    pub tag2: super::d2d1::D2D1_TAG,
    pub transform: windows_numerics::Matrix3x2,
    pub primitiveBlend: D2D1_PRIMITIVE_BLEND,
    pub unitMode: D2D1_UNIT_MODE,
}
#[repr(C)]
#[cfg(feature = "Win32_dcommon")]
#[derive(Clone, Debug, Default, PartialEq)]
pub struct D2D1_EFFECT_INPUT_DESCRIPTION {
    pub effect: core::mem::ManuallyDrop<Option<ID2D1Effect>>,
    pub inputIndex: u32,
    pub inputRectangle: super::dcommon::D2D_RECT_F,
}
#[repr(C)]
#[cfg(all(feature = "Win32_d2d1", feature = "Win32_dcommon"))]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
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
#[cfg(all(feature = "Win32_d2d1", feature = "Win32_dcommon"))]
#[derive(Clone, Debug, Default, PartialEq)]
pub struct D2D1_LAYER_PARAMETERS1 {
    pub contentBounds: super::dcommon::D2D_RECT_F,
    pub geometricMask: core::mem::ManuallyDrop<Option<super::d2d1::ID2D1Geometry>>,
    pub maskAntialiasMode: super::d2d1::D2D1_ANTIALIAS_MODE,
    pub maskTransform: windows_numerics::Matrix3x2,
    pub opacity: f32,
    pub opacityBrush: core::mem::ManuallyDrop<Option<super::d2d1::ID2D1Brush>>,
    pub layerOptions: D2D1_LAYER_OPTIONS1,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
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
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct D2D1_POINT_DESCRIPTION {
    pub point: windows_numerics::Vector2,
    pub unitTangentVector: windows_numerics::Vector2,
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
#[derive(Clone, Copy, Debug, Default, PartialEq)]
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
#[derive(Clone, Copy, Debug, Default, PartialEq)]
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
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct D2D1_RENDERING_CONTROLS {
    pub bufferPrecision: D2D1_BUFFER_PRECISION,
    pub tileSize: super::dcommon::D2D_SIZE_U,
}
#[repr(C)]
#[cfg(feature = "Win32_d2d1")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
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
#[cfg(feature = "Win32_d2d1")]
windows_core::imp::define_interface!(ID2D1Bitmap1, ID2D1Bitmap1_Vtbl, 0xa898a84c_3873_4588_b08b_ebbf978df041);
#[cfg(feature = "Win32_d2d1")]
impl core::ops::Deref for ID2D1Bitmap1 {
    type Target = super::d2d1::ID2D1Bitmap;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_d2d1")]
windows_core::imp::interface_hierarchy!(ID2D1Bitmap1, windows_core::IUnknown, super::d2d1::ID2D1Resource, super::d2d1::ID2D1Image, super::d2d1::ID2D1Bitmap);
#[cfg(feature = "Win32_d2d1")]
impl ID2D1Bitmap1 {
    pub unsafe fn GetColorContext(&self) -> windows_core::Result<ID2D1ColorContext> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetColorContext)(windows_core::Interface::as_raw(self), &mut result__);
            windows_core::Type::from_abi(result__)
        }
    }
    pub unsafe fn GetOptions(&self) -> D2D1_BITMAP_OPTIONS {
        unsafe { (windows_core::Interface::vtable(self).GetOptions)(windows_core::Interface::as_raw(self)) }
    }
    #[cfg(feature = "Win32_dxgi")]
    pub unsafe fn GetSurface(&self) -> windows_core::Result<super::dxgi::IDXGISurface> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetSurface)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn Map(&self, options: D2D1_MAP_OPTIONS) -> windows_core::Result<D2D1_MAPPED_RECT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Map)(windows_core::Interface::as_raw(self), options, &mut result__).map(|| result__)
        }
    }
    pub unsafe fn Unmap(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Unmap)(windows_core::Interface::as_raw(self)) }
    }
}
#[cfg(feature = "Win32_d2d1")]
#[repr(C)]
#[doc(hidden)]
pub struct ID2D1Bitmap1_Vtbl {
    pub base__: super::d2d1::ID2D1Bitmap_Vtbl,
    pub GetColorContext: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void),
    pub GetOptions: unsafe extern "system" fn(*mut core::ffi::c_void) -> D2D1_BITMAP_OPTIONS,
    #[cfg(feature = "Win32_dxgi")]
    pub GetSurface: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_dxgi"))]
    GetSurface: usize,
    pub Map: unsafe extern "system" fn(*mut core::ffi::c_void, D2D1_MAP_OPTIONS, *mut D2D1_MAPPED_RECT) -> windows_core::HRESULT,
    pub Unmap: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_d2d1", feature = "Win32_dcommon", feature = "Win32_dxgi", feature = "Win32_dxgiformat"))]
pub trait ID2D1Bitmap1_Impl: super::d2d1::ID2D1Bitmap_Impl {
    fn GetColorContext(&self, colorcontext: windows_core::OutRef<ID2D1ColorContext>);
    fn GetOptions(&self) -> D2D1_BITMAP_OPTIONS;
    fn GetSurface(&self) -> windows_core::Result<super::dxgi::IDXGISurface>;
    fn Map(&self, options: D2D1_MAP_OPTIONS) -> windows_core::Result<D2D1_MAPPED_RECT>;
    fn Unmap(&self) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_d2d1", feature = "Win32_dcommon", feature = "Win32_dxgi", feature = "Win32_dxgiformat"))]
impl ID2D1Bitmap1_Vtbl {
    pub const fn new<Identity: ID2D1Bitmap1_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetColorContext<Identity: ID2D1Bitmap1_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, colorcontext: *mut *mut core::ffi::c_void) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID2D1Bitmap1_Impl::GetColorContext(this, core::mem::transmute_copy(&colorcontext));
            }
        }
        unsafe extern "system" fn GetOptions<Identity: ID2D1Bitmap1_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> D2D1_BITMAP_OPTIONS {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID2D1Bitmap1_Impl::GetOptions(this)
            }
        }
        unsafe extern "system" fn GetSurface<Identity: ID2D1Bitmap1_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dxgisurface: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ID2D1Bitmap1_Impl::GetSurface(this) {
                    Ok(ok__) => {
                        dxgisurface.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Map<Identity: ID2D1Bitmap1_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, options: D2D1_MAP_OPTIONS, mappedrect: *mut D2D1_MAPPED_RECT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ID2D1Bitmap1_Impl::Map(this, core::mem::transmute_copy(&options)) {
                    Ok(ok__) => {
                        mappedrect.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Unmap<Identity: ID2D1Bitmap1_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID2D1Bitmap1_Impl::Unmap(this).into()
            }
        }
        Self {
            base__: super::d2d1::ID2D1Bitmap_Vtbl::new::<Identity, OFFSET>(),
            GetColorContext: GetColorContext::<Identity, OFFSET>,
            GetOptions: GetOptions::<Identity, OFFSET>,
            GetSurface: GetSurface::<Identity, OFFSET>,
            Map: Map::<Identity, OFFSET>,
            Unmap: Unmap::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ID2D1Bitmap1 as windows_core::Interface>::IID || iid == &<super::d2d1::ID2D1Resource as windows_core::Interface>::IID || iid == &<super::d2d1::ID2D1Image as windows_core::Interface>::IID || iid == &<super::d2d1::ID2D1Bitmap as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_d2d1", feature = "Win32_dcommon", feature = "Win32_dxgi", feature = "Win32_dxgiformat"))]
impl windows_core::RuntimeName for ID2D1Bitmap1 {}
#[cfg(feature = "Win32_d2d1")]
windows_core::imp::define_interface!(ID2D1BitmapBrush1, ID2D1BitmapBrush1_Vtbl, 0x41343a53_e41a_49a2_91cd_21793bbb62e5);
#[cfg(feature = "Win32_d2d1")]
impl core::ops::Deref for ID2D1BitmapBrush1 {
    type Target = super::d2d1::ID2D1BitmapBrush;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_d2d1")]
windows_core::imp::interface_hierarchy!(ID2D1BitmapBrush1, windows_core::IUnknown, super::d2d1::ID2D1Resource, super::d2d1::ID2D1Brush, super::d2d1::ID2D1BitmapBrush);
#[cfg(feature = "Win32_d2d1")]
impl ID2D1BitmapBrush1 {
    pub unsafe fn SetInterpolationMode1(&self, interpolationmode: D2D1_INTERPOLATION_MODE) {
        unsafe {
            (windows_core::Interface::vtable(self).SetInterpolationMode1)(windows_core::Interface::as_raw(self), interpolationmode);
        }
    }
    pub unsafe fn GetInterpolationMode1(&self) -> D2D1_INTERPOLATION_MODE {
        unsafe { (windows_core::Interface::vtable(self).GetInterpolationMode1)(windows_core::Interface::as_raw(self)) }
    }
}
#[cfg(feature = "Win32_d2d1")]
#[repr(C)]
#[doc(hidden)]
pub struct ID2D1BitmapBrush1_Vtbl {
    pub base__: super::d2d1::ID2D1BitmapBrush_Vtbl,
    pub SetInterpolationMode1: unsafe extern "system" fn(*mut core::ffi::c_void, D2D1_INTERPOLATION_MODE),
    pub GetInterpolationMode1: unsafe extern "system" fn(*mut core::ffi::c_void) -> D2D1_INTERPOLATION_MODE,
}
#[cfg(feature = "Win32_d2d1")]
pub trait ID2D1BitmapBrush1_Impl: super::d2d1::ID2D1BitmapBrush_Impl {
    fn SetInterpolationMode1(&self, interpolationmode: D2D1_INTERPOLATION_MODE);
    fn GetInterpolationMode1(&self) -> D2D1_INTERPOLATION_MODE;
}
#[cfg(feature = "Win32_d2d1")]
impl ID2D1BitmapBrush1_Vtbl {
    pub const fn new<Identity: ID2D1BitmapBrush1_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn SetInterpolationMode1<Identity: ID2D1BitmapBrush1_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, interpolationmode: D2D1_INTERPOLATION_MODE) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID2D1BitmapBrush1_Impl::SetInterpolationMode1(this, core::mem::transmute_copy(&interpolationmode));
            }
        }
        unsafe extern "system" fn GetInterpolationMode1<Identity: ID2D1BitmapBrush1_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> D2D1_INTERPOLATION_MODE {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID2D1BitmapBrush1_Impl::GetInterpolationMode1(this)
            }
        }
        Self {
            base__: super::d2d1::ID2D1BitmapBrush_Vtbl::new::<Identity, OFFSET>(),
            SetInterpolationMode1: SetInterpolationMode1::<Identity, OFFSET>,
            GetInterpolationMode1: GetInterpolationMode1::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ID2D1BitmapBrush1 as windows_core::Interface>::IID || iid == &<super::d2d1::ID2D1Resource as windows_core::Interface>::IID || iid == &<super::d2d1::ID2D1Brush as windows_core::Interface>::IID || iid == &<super::d2d1::ID2D1BitmapBrush as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_d2d1")]
impl windows_core::RuntimeName for ID2D1BitmapBrush1 {}
#[cfg(feature = "Win32_d2d1")]
windows_core::imp::define_interface!(ID2D1ColorContext, ID2D1ColorContext_Vtbl, 0x1c4820bb_5771_4518_a581_2fe4dd0ec657);
#[cfg(feature = "Win32_d2d1")]
impl core::ops::Deref for ID2D1ColorContext {
    type Target = super::d2d1::ID2D1Resource;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_d2d1")]
windows_core::imp::interface_hierarchy!(ID2D1ColorContext, windows_core::IUnknown, super::d2d1::ID2D1Resource);
#[cfg(feature = "Win32_d2d1")]
impl ID2D1ColorContext {
    pub unsafe fn GetColorSpace(&self) -> D2D1_COLOR_SPACE {
        unsafe { (windows_core::Interface::vtable(self).GetColorSpace)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn GetProfileSize(&self) -> u32 {
        unsafe { (windows_core::Interface::vtable(self).GetProfileSize)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn GetProfile(&self, profile: &mut [u8]) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetProfile)(windows_core::Interface::as_raw(self), core::mem::transmute(profile.as_ptr()), profile.len().try_into().unwrap()) }
    }
}
#[cfg(feature = "Win32_d2d1")]
#[repr(C)]
#[doc(hidden)]
pub struct ID2D1ColorContext_Vtbl {
    pub base__: super::d2d1::ID2D1Resource_Vtbl,
    pub GetColorSpace: unsafe extern "system" fn(*mut core::ffi::c_void) -> D2D1_COLOR_SPACE,
    pub GetProfileSize: unsafe extern "system" fn(*mut core::ffi::c_void) -> u32,
    pub GetProfile: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u8, u32) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_d2d1")]
pub trait ID2D1ColorContext_Impl: super::d2d1::ID2D1Resource_Impl {
    fn GetColorSpace(&self) -> D2D1_COLOR_SPACE;
    fn GetProfileSize(&self) -> u32;
    fn GetProfile(&self, profile: *mut u8, profilesize: u32) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_d2d1")]
impl ID2D1ColorContext_Vtbl {
    pub const fn new<Identity: ID2D1ColorContext_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetColorSpace<Identity: ID2D1ColorContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> D2D1_COLOR_SPACE {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID2D1ColorContext_Impl::GetColorSpace(this)
            }
        }
        unsafe extern "system" fn GetProfileSize<Identity: ID2D1ColorContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> u32 {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID2D1ColorContext_Impl::GetProfileSize(this)
            }
        }
        unsafe extern "system" fn GetProfile<Identity: ID2D1ColorContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, profile: *mut u8, profilesize: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID2D1ColorContext_Impl::GetProfile(this, core::mem::transmute_copy(&profile), core::mem::transmute_copy(&profilesize)).into()
            }
        }
        Self {
            base__: super::d2d1::ID2D1Resource_Vtbl::new::<Identity, OFFSET>(),
            GetColorSpace: GetColorSpace::<Identity, OFFSET>,
            GetProfileSize: GetProfileSize::<Identity, OFFSET>,
            GetProfile: GetProfile::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ID2D1ColorContext as windows_core::Interface>::IID || iid == &<super::d2d1::ID2D1Resource as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_d2d1")]
impl windows_core::RuntimeName for ID2D1ColorContext {}
#[cfg(feature = "Win32_d2d1")]
windows_core::imp::define_interface!(ID2D1CommandList, ID2D1CommandList_Vtbl, 0xb4f34a19_2383_4d76_94f6_ec343657c3dc);
#[cfg(feature = "Win32_d2d1")]
impl core::ops::Deref for ID2D1CommandList {
    type Target = super::d2d1::ID2D1Image;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_d2d1")]
windows_core::imp::interface_hierarchy!(ID2D1CommandList, windows_core::IUnknown, super::d2d1::ID2D1Resource, super::d2d1::ID2D1Image);
#[cfg(feature = "Win32_d2d1")]
impl ID2D1CommandList {
    pub unsafe fn Stream<P0>(&self, sink: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<ID2D1CommandSink>,
    {
        unsafe { (windows_core::Interface::vtable(self).Stream)(windows_core::Interface::as_raw(self), sink.param().abi()) }
    }
    pub unsafe fn Close(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Close)(windows_core::Interface::as_raw(self)) }
    }
}
#[cfg(feature = "Win32_d2d1")]
#[repr(C)]
#[doc(hidden)]
pub struct ID2D1CommandList_Vtbl {
    pub base__: super::d2d1::ID2D1Image_Vtbl,
    pub Stream: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Close: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_d2d1")]
pub trait ID2D1CommandList_Impl: super::d2d1::ID2D1Image_Impl {
    fn Stream(&self, sink: windows_core::Ref<ID2D1CommandSink>) -> windows_core::Result<()>;
    fn Close(&self) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_d2d1")]
impl ID2D1CommandList_Vtbl {
    pub const fn new<Identity: ID2D1CommandList_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Stream<Identity: ID2D1CommandList_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, sink: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID2D1CommandList_Impl::Stream(this, core::mem::transmute_copy(&sink)).into()
            }
        }
        unsafe extern "system" fn Close<Identity: ID2D1CommandList_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID2D1CommandList_Impl::Close(this).into()
            }
        }
        Self { base__: super::d2d1::ID2D1Image_Vtbl::new::<Identity, OFFSET>(), Stream: Stream::<Identity, OFFSET>, Close: Close::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ID2D1CommandList as windows_core::Interface>::IID || iid == &<super::d2d1::ID2D1Resource as windows_core::Interface>::IID || iid == &<super::d2d1::ID2D1Image as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_d2d1")]
impl windows_core::RuntimeName for ID2D1CommandList {}
windows_core::imp::define_interface!(ID2D1CommandSink, ID2D1CommandSink_Vtbl, 0x54d7898a_a061_40a7_bec7_e465bcba2c4f);
windows_core::imp::interface_hierarchy!(ID2D1CommandSink, windows_core::IUnknown);
impl ID2D1CommandSink {
    pub unsafe fn BeginDraw(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).BeginDraw)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn EndDraw(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).EndDraw)(windows_core::Interface::as_raw(self)) }
    }
    #[cfg(feature = "Win32_d2d1")]
    pub unsafe fn SetAntialiasMode(&self, antialiasmode: super::d2d1::D2D1_ANTIALIAS_MODE) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetAntialiasMode)(windows_core::Interface::as_raw(self), antialiasmode) }
    }
    #[cfg(feature = "Win32_d2d1")]
    pub unsafe fn SetTags(&self, tag1: super::d2d1::D2D1_TAG, tag2: super::d2d1::D2D1_TAG) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetTags)(windows_core::Interface::as_raw(self), tag1, tag2) }
    }
    #[cfg(feature = "Win32_d2d1")]
    pub unsafe fn SetTextAntialiasMode(&self, textantialiasmode: super::d2d1::D2D1_TEXT_ANTIALIAS_MODE) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetTextAntialiasMode)(windows_core::Interface::as_raw(self), textantialiasmode) }
    }
    #[cfg(feature = "Win32_dwrite")]
    pub unsafe fn SetTextRenderingParams<P0>(&self, textrenderingparams: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<super::dwrite::IDWriteRenderingParams>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetTextRenderingParams)(windows_core::Interface::as_raw(self), textrenderingparams.param().abi()) }
    }
    pub unsafe fn SetTransform(&self, transform: *const windows_numerics::Matrix3x2) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetTransform)(windows_core::Interface::as_raw(self), transform) }
    }
    pub unsafe fn SetPrimitiveBlend(&self, primitiveblend: D2D1_PRIMITIVE_BLEND) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetPrimitiveBlend)(windows_core::Interface::as_raw(self), primitiveblend) }
    }
    pub unsafe fn SetUnitMode(&self, unitmode: D2D1_UNIT_MODE) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetUnitMode)(windows_core::Interface::as_raw(self), unitmode) }
    }
    #[cfg(all(feature = "Win32_d2dbasetypes", feature = "Win32_dxgitype"))]
    pub unsafe fn Clear(&self, color: Option<*const super::d2dbasetypes::D2D_COLOR_F>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Clear)(windows_core::Interface::as_raw(self), color.unwrap_or(core::mem::zeroed()) as _) }
    }
    #[cfg(all(feature = "Win32_d2d1", feature = "Win32_dcommon", feature = "Win32_dwrite"))]
    pub unsafe fn DrawGlyphRun<P3>(&self, baselineorigin: windows_numerics::Vector2, glyphrun: *const super::dwrite::DWRITE_GLYPH_RUN, glyphrundescription: Option<*const super::dwrite::DWRITE_GLYPH_RUN_DESCRIPTION>, foregroundbrush: P3, measuringmode: super::dcommon::DWRITE_MEASURING_MODE) -> windows_core::HRESULT
    where
        P3: windows_core::Param<super::d2d1::ID2D1Brush>,
    {
        unsafe { (windows_core::Interface::vtable(self).DrawGlyphRun)(windows_core::Interface::as_raw(self), core::mem::transmute(baselineorigin), core::mem::transmute(glyphrun), glyphrundescription.unwrap_or(core::mem::zeroed()) as _, foregroundbrush.param().abi(), measuringmode) }
    }
    #[cfg(feature = "Win32_d2d1")]
    pub unsafe fn DrawLine<P2, P4>(&self, point0: windows_numerics::Vector2, point1: windows_numerics::Vector2, brush: P2, strokewidth: f32, strokestyle: P4) -> windows_core::HRESULT
    where
        P2: windows_core::Param<super::d2d1::ID2D1Brush>,
        P4: windows_core::Param<super::d2d1::ID2D1StrokeStyle>,
    {
        unsafe { (windows_core::Interface::vtable(self).DrawLine)(windows_core::Interface::as_raw(self), core::mem::transmute(point0), core::mem::transmute(point1), brush.param().abi(), strokewidth, strokestyle.param().abi()) }
    }
    #[cfg(feature = "Win32_d2d1")]
    pub unsafe fn DrawGeometry<P0, P1, P3>(&self, geometry: P0, brush: P1, strokewidth: f32, strokestyle: P3) -> windows_core::HRESULT
    where
        P0: windows_core::Param<super::d2d1::ID2D1Geometry>,
        P1: windows_core::Param<super::d2d1::ID2D1Brush>,
        P3: windows_core::Param<super::d2d1::ID2D1StrokeStyle>,
    {
        unsafe { (windows_core::Interface::vtable(self).DrawGeometry)(windows_core::Interface::as_raw(self), geometry.param().abi(), brush.param().abi(), strokewidth, strokestyle.param().abi()) }
    }
    #[cfg(all(feature = "Win32_d2d1", feature = "Win32_dcommon"))]
    pub unsafe fn DrawRectangle<P1, P3>(&self, rect: *const super::dcommon::D2D_RECT_F, brush: P1, strokewidth: f32, strokestyle: P3) -> windows_core::HRESULT
    where
        P1: windows_core::Param<super::d2d1::ID2D1Brush>,
        P3: windows_core::Param<super::d2d1::ID2D1StrokeStyle>,
    {
        unsafe { (windows_core::Interface::vtable(self).DrawRectangle)(windows_core::Interface::as_raw(self), rect, brush.param().abi(), strokewidth, strokestyle.param().abi()) }
    }
    #[cfg(all(feature = "Win32_d2d1", feature = "Win32_dcommon"))]
    pub unsafe fn DrawBitmap<P0>(&self, bitmap: P0, destinationrectangle: Option<*const super::dcommon::D2D_RECT_F>, opacity: f32, interpolationmode: D2D1_INTERPOLATION_MODE, sourcerectangle: Option<*const super::dcommon::D2D_RECT_F>, perspectivetransform: Option<*const windows_numerics::Matrix4x4>) -> windows_core::HRESULT
    where
        P0: windows_core::Param<super::d2d1::ID2D1Bitmap>,
    {
        unsafe { (windows_core::Interface::vtable(self).DrawBitmap)(windows_core::Interface::as_raw(self), bitmap.param().abi(), destinationrectangle.unwrap_or(core::mem::zeroed()) as _, opacity, interpolationmode, sourcerectangle.unwrap_or(core::mem::zeroed()) as _, perspectivetransform.unwrap_or(core::mem::zeroed()) as _) }
    }
    #[cfg(all(feature = "Win32_d2d1", feature = "Win32_dcommon"))]
    pub unsafe fn DrawImage<P0>(&self, image: P0, targetoffset: Option<*const windows_numerics::Vector2>, imagerectangle: Option<*const super::dcommon::D2D_RECT_F>, interpolationmode: D2D1_INTERPOLATION_MODE, compositemode: D2D1_COMPOSITE_MODE) -> windows_core::HRESULT
    where
        P0: windows_core::Param<super::d2d1::ID2D1Image>,
    {
        unsafe { (windows_core::Interface::vtable(self).DrawImage)(windows_core::Interface::as_raw(self), image.param().abi(), targetoffset.unwrap_or(core::mem::zeroed()) as _, imagerectangle.unwrap_or(core::mem::zeroed()) as _, interpolationmode, compositemode) }
    }
    #[cfg(feature = "Win32_d2d1")]
    pub unsafe fn DrawGdiMetafile<P0>(&self, gdimetafile: P0, targetoffset: Option<*const windows_numerics::Vector2>) -> windows_core::HRESULT
    where
        P0: windows_core::Param<ID2D1GdiMetafile>,
    {
        unsafe { (windows_core::Interface::vtable(self).DrawGdiMetafile)(windows_core::Interface::as_raw(self), gdimetafile.param().abi(), targetoffset.unwrap_or(core::mem::zeroed()) as _) }
    }
    #[cfg(feature = "Win32_d2d1")]
    pub unsafe fn FillMesh<P0, P1>(&self, mesh: P0, brush: P1) -> windows_core::HRESULT
    where
        P0: windows_core::Param<super::d2d1::ID2D1Mesh>,
        P1: windows_core::Param<super::d2d1::ID2D1Brush>,
    {
        unsafe { (windows_core::Interface::vtable(self).FillMesh)(windows_core::Interface::as_raw(self), mesh.param().abi(), brush.param().abi()) }
    }
    #[cfg(all(feature = "Win32_d2d1", feature = "Win32_dcommon"))]
    pub unsafe fn FillOpacityMask<P0, P1>(&self, opacitymask: P0, brush: P1, destinationrectangle: Option<*const super::dcommon::D2D_RECT_F>, sourcerectangle: Option<*const super::dcommon::D2D_RECT_F>) -> windows_core::HRESULT
    where
        P0: windows_core::Param<super::d2d1::ID2D1Bitmap>,
        P1: windows_core::Param<super::d2d1::ID2D1Brush>,
    {
        unsafe { (windows_core::Interface::vtable(self).FillOpacityMask)(windows_core::Interface::as_raw(self), opacitymask.param().abi(), brush.param().abi(), destinationrectangle.unwrap_or(core::mem::zeroed()) as _, sourcerectangle.unwrap_or(core::mem::zeroed()) as _) }
    }
    #[cfg(feature = "Win32_d2d1")]
    pub unsafe fn FillGeometry<P0, P1, P2>(&self, geometry: P0, brush: P1, opacitybrush: P2) -> windows_core::HRESULT
    where
        P0: windows_core::Param<super::d2d1::ID2D1Geometry>,
        P1: windows_core::Param<super::d2d1::ID2D1Brush>,
        P2: windows_core::Param<super::d2d1::ID2D1Brush>,
    {
        unsafe { (windows_core::Interface::vtable(self).FillGeometry)(windows_core::Interface::as_raw(self), geometry.param().abi(), brush.param().abi(), opacitybrush.param().abi()) }
    }
    #[cfg(all(feature = "Win32_d2d1", feature = "Win32_dcommon"))]
    pub unsafe fn FillRectangle<P1>(&self, rect: *const super::dcommon::D2D_RECT_F, brush: P1) -> windows_core::HRESULT
    where
        P1: windows_core::Param<super::d2d1::ID2D1Brush>,
    {
        unsafe { (windows_core::Interface::vtable(self).FillRectangle)(windows_core::Interface::as_raw(self), rect, brush.param().abi()) }
    }
    #[cfg(all(feature = "Win32_d2d1", feature = "Win32_dcommon"))]
    pub unsafe fn PushAxisAlignedClip(&self, cliprect: *const super::dcommon::D2D_RECT_F, antialiasmode: super::d2d1::D2D1_ANTIALIAS_MODE) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).PushAxisAlignedClip)(windows_core::Interface::as_raw(self), cliprect, antialiasmode) }
    }
    #[cfg(all(feature = "Win32_d2d1", feature = "Win32_dcommon"))]
    pub unsafe fn PushLayer<P1>(&self, layerparameters1: *const D2D1_LAYER_PARAMETERS1, layer: P1) -> windows_core::HRESULT
    where
        P1: windows_core::Param<super::d2d1::ID2D1Layer>,
    {
        unsafe { (windows_core::Interface::vtable(self).PushLayer)(windows_core::Interface::as_raw(self), core::mem::transmute(layerparameters1), layer.param().abi()) }
    }
    pub unsafe fn PopAxisAlignedClip(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).PopAxisAlignedClip)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn PopLayer(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).PopLayer)(windows_core::Interface::as_raw(self)) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ID2D1CommandSink_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub BeginDraw: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub EndDraw: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_d2d1")]
    pub SetAntialiasMode: unsafe extern "system" fn(*mut core::ffi::c_void, super::d2d1::D2D1_ANTIALIAS_MODE) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_d2d1"))]
    SetAntialiasMode: usize,
    #[cfg(feature = "Win32_d2d1")]
    pub SetTags: unsafe extern "system" fn(*mut core::ffi::c_void, super::d2d1::D2D1_TAG, super::d2d1::D2D1_TAG) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_d2d1"))]
    SetTags: usize,
    #[cfg(feature = "Win32_d2d1")]
    pub SetTextAntialiasMode: unsafe extern "system" fn(*mut core::ffi::c_void, super::d2d1::D2D1_TEXT_ANTIALIAS_MODE) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_d2d1"))]
    SetTextAntialiasMode: usize,
    #[cfg(feature = "Win32_dwrite")]
    pub SetTextRenderingParams: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_dwrite"))]
    SetTextRenderingParams: usize,
    pub SetTransform: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_numerics::Matrix3x2) -> windows_core::HRESULT,
    pub SetPrimitiveBlend: unsafe extern "system" fn(*mut core::ffi::c_void, D2D1_PRIMITIVE_BLEND) -> windows_core::HRESULT,
    pub SetUnitMode: unsafe extern "system" fn(*mut core::ffi::c_void, D2D1_UNIT_MODE) -> windows_core::HRESULT,
    #[cfg(all(feature = "Win32_d2dbasetypes", feature = "Win32_dxgitype"))]
    pub Clear: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::d2dbasetypes::D2D_COLOR_F) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_d2dbasetypes", feature = "Win32_dxgitype")))]
    Clear: usize,
    #[cfg(all(feature = "Win32_d2d1", feature = "Win32_dcommon", feature = "Win32_dwrite"))]
    pub DrawGlyphRun: unsafe extern "system" fn(*mut core::ffi::c_void, windows_numerics::Vector2, *const super::dwrite::DWRITE_GLYPH_RUN, *const super::dwrite::DWRITE_GLYPH_RUN_DESCRIPTION, *mut core::ffi::c_void, super::dcommon::DWRITE_MEASURING_MODE) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_d2d1", feature = "Win32_dcommon", feature = "Win32_dwrite")))]
    DrawGlyphRun: usize,
    #[cfg(feature = "Win32_d2d1")]
    pub DrawLine: unsafe extern "system" fn(*mut core::ffi::c_void, windows_numerics::Vector2, windows_numerics::Vector2, *mut core::ffi::c_void, f32, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_d2d1"))]
    DrawLine: usize,
    #[cfg(feature = "Win32_d2d1")]
    pub DrawGeometry: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, f32, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_d2d1"))]
    DrawGeometry: usize,
    #[cfg(all(feature = "Win32_d2d1", feature = "Win32_dcommon"))]
    pub DrawRectangle: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::dcommon::D2D_RECT_F, *mut core::ffi::c_void, f32, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_d2d1", feature = "Win32_dcommon")))]
    DrawRectangle: usize,
    #[cfg(all(feature = "Win32_d2d1", feature = "Win32_dcommon"))]
    pub DrawBitmap: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *const super::dcommon::D2D_RECT_F, f32, D2D1_INTERPOLATION_MODE, *const super::dcommon::D2D_RECT_F, *const windows_numerics::Matrix4x4) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_d2d1", feature = "Win32_dcommon")))]
    DrawBitmap: usize,
    #[cfg(all(feature = "Win32_d2d1", feature = "Win32_dcommon"))]
    pub DrawImage: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *const windows_numerics::Vector2, *const super::dcommon::D2D_RECT_F, D2D1_INTERPOLATION_MODE, D2D1_COMPOSITE_MODE) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_d2d1", feature = "Win32_dcommon")))]
    DrawImage: usize,
    #[cfg(feature = "Win32_d2d1")]
    pub DrawGdiMetafile: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *const windows_numerics::Vector2) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_d2d1"))]
    DrawGdiMetafile: usize,
    #[cfg(feature = "Win32_d2d1")]
    pub FillMesh: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_d2d1"))]
    FillMesh: usize,
    #[cfg(all(feature = "Win32_d2d1", feature = "Win32_dcommon"))]
    pub FillOpacityMask: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *const super::dcommon::D2D_RECT_F, *const super::dcommon::D2D_RECT_F) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_d2d1", feature = "Win32_dcommon")))]
    FillOpacityMask: usize,
    #[cfg(feature = "Win32_d2d1")]
    pub FillGeometry: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_d2d1"))]
    FillGeometry: usize,
    #[cfg(all(feature = "Win32_d2d1", feature = "Win32_dcommon"))]
    pub FillRectangle: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::dcommon::D2D_RECT_F, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_d2d1", feature = "Win32_dcommon")))]
    FillRectangle: usize,
    #[cfg(all(feature = "Win32_d2d1", feature = "Win32_dcommon"))]
    pub PushAxisAlignedClip: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::dcommon::D2D_RECT_F, super::d2d1::D2D1_ANTIALIAS_MODE) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_d2d1", feature = "Win32_dcommon")))]
    PushAxisAlignedClip: usize,
    #[cfg(all(feature = "Win32_d2d1", feature = "Win32_dcommon"))]
    pub PushLayer: unsafe extern "system" fn(*mut core::ffi::c_void, *const D2D1_LAYER_PARAMETERS1, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_d2d1", feature = "Win32_dcommon")))]
    PushLayer: usize,
    pub PopAxisAlignedClip: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub PopLayer: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_d2d1", feature = "Win32_d2dbasetypes", feature = "Win32_dcommon", feature = "Win32_dwrite", feature = "Win32_dxgitype"))]
pub trait ID2D1CommandSink_Impl: windows_core::IUnknownImpl {
    fn BeginDraw(&self) -> windows_core::Result<()>;
    fn EndDraw(&self) -> windows_core::Result<()>;
    fn SetAntialiasMode(&self, antialiasmode: super::d2d1::D2D1_ANTIALIAS_MODE) -> windows_core::Result<()>;
    fn SetTags(&self, tag1: super::d2d1::D2D1_TAG, tag2: super::d2d1::D2D1_TAG) -> windows_core::Result<()>;
    fn SetTextAntialiasMode(&self, textantialiasmode: super::d2d1::D2D1_TEXT_ANTIALIAS_MODE) -> windows_core::Result<()>;
    fn SetTextRenderingParams(&self, textrenderingparams: windows_core::Ref<super::dwrite::IDWriteRenderingParams>) -> windows_core::Result<()>;
    fn SetTransform(&self, transform: *const windows_numerics::Matrix3x2) -> windows_core::Result<()>;
    fn SetPrimitiveBlend(&self, primitiveblend: D2D1_PRIMITIVE_BLEND) -> windows_core::Result<()>;
    fn SetUnitMode(&self, unitmode: D2D1_UNIT_MODE) -> windows_core::Result<()>;
    fn Clear(&self, color: *const super::d2dbasetypes::D2D_COLOR_F) -> windows_core::Result<()>;
    fn DrawGlyphRun(&self, baselineorigin: &windows_numerics::Vector2, glyphrun: *const super::dwrite::DWRITE_GLYPH_RUN, glyphrundescription: *const super::dwrite::DWRITE_GLYPH_RUN_DESCRIPTION, foregroundbrush: windows_core::Ref<super::d2d1::ID2D1Brush>, measuringmode: super::dcommon::DWRITE_MEASURING_MODE) -> windows_core::Result<()>;
    fn DrawLine(&self, point0: &windows_numerics::Vector2, point1: &windows_numerics::Vector2, brush: windows_core::Ref<super::d2d1::ID2D1Brush>, strokewidth: f32, strokestyle: windows_core::Ref<super::d2d1::ID2D1StrokeStyle>) -> windows_core::Result<()>;
    fn DrawGeometry(&self, geometry: windows_core::Ref<super::d2d1::ID2D1Geometry>, brush: windows_core::Ref<super::d2d1::ID2D1Brush>, strokewidth: f32, strokestyle: windows_core::Ref<super::d2d1::ID2D1StrokeStyle>) -> windows_core::Result<()>;
    fn DrawRectangle(&self, rect: *const super::dcommon::D2D_RECT_F, brush: windows_core::Ref<super::d2d1::ID2D1Brush>, strokewidth: f32, strokestyle: windows_core::Ref<super::d2d1::ID2D1StrokeStyle>) -> windows_core::Result<()>;
    fn DrawBitmap(&self, bitmap: windows_core::Ref<super::d2d1::ID2D1Bitmap>, destinationrectangle: *const super::dcommon::D2D_RECT_F, opacity: f32, interpolationmode: D2D1_INTERPOLATION_MODE, sourcerectangle: *const super::dcommon::D2D_RECT_F, perspectivetransform: *const windows_numerics::Matrix4x4) -> windows_core::Result<()>;
    fn DrawImage(&self, image: windows_core::Ref<super::d2d1::ID2D1Image>, targetoffset: *const windows_numerics::Vector2, imagerectangle: *const super::dcommon::D2D_RECT_F, interpolationmode: D2D1_INTERPOLATION_MODE, compositemode: D2D1_COMPOSITE_MODE) -> windows_core::Result<()>;
    fn DrawGdiMetafile(&self, gdimetafile: windows_core::Ref<ID2D1GdiMetafile>, targetoffset: *const windows_numerics::Vector2) -> windows_core::Result<()>;
    fn FillMesh(&self, mesh: windows_core::Ref<super::d2d1::ID2D1Mesh>, brush: windows_core::Ref<super::d2d1::ID2D1Brush>) -> windows_core::Result<()>;
    fn FillOpacityMask(&self, opacitymask: windows_core::Ref<super::d2d1::ID2D1Bitmap>, brush: windows_core::Ref<super::d2d1::ID2D1Brush>, destinationrectangle: *const super::dcommon::D2D_RECT_F, sourcerectangle: *const super::dcommon::D2D_RECT_F) -> windows_core::Result<()>;
    fn FillGeometry(&self, geometry: windows_core::Ref<super::d2d1::ID2D1Geometry>, brush: windows_core::Ref<super::d2d1::ID2D1Brush>, opacitybrush: windows_core::Ref<super::d2d1::ID2D1Brush>) -> windows_core::Result<()>;
    fn FillRectangle(&self, rect: *const super::dcommon::D2D_RECT_F, brush: windows_core::Ref<super::d2d1::ID2D1Brush>) -> windows_core::Result<()>;
    fn PushAxisAlignedClip(&self, cliprect: *const super::dcommon::D2D_RECT_F, antialiasmode: super::d2d1::D2D1_ANTIALIAS_MODE) -> windows_core::Result<()>;
    fn PushLayer(&self, layerparameters1: *const D2D1_LAYER_PARAMETERS1, layer: windows_core::Ref<super::d2d1::ID2D1Layer>) -> windows_core::Result<()>;
    fn PopAxisAlignedClip(&self) -> windows_core::Result<()>;
    fn PopLayer(&self) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_d2d1", feature = "Win32_d2dbasetypes", feature = "Win32_dcommon", feature = "Win32_dwrite", feature = "Win32_dxgitype"))]
impl ID2D1CommandSink_Vtbl {
    pub const fn new<Identity: ID2D1CommandSink_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn BeginDraw<Identity: ID2D1CommandSink_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID2D1CommandSink_Impl::BeginDraw(this).into()
            }
        }
        unsafe extern "system" fn EndDraw<Identity: ID2D1CommandSink_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID2D1CommandSink_Impl::EndDraw(this).into()
            }
        }
        unsafe extern "system" fn SetAntialiasMode<Identity: ID2D1CommandSink_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, antialiasmode: super::d2d1::D2D1_ANTIALIAS_MODE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID2D1CommandSink_Impl::SetAntialiasMode(this, core::mem::transmute_copy(&antialiasmode)).into()
            }
        }
        unsafe extern "system" fn SetTags<Identity: ID2D1CommandSink_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, tag1: super::d2d1::D2D1_TAG, tag2: super::d2d1::D2D1_TAG) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID2D1CommandSink_Impl::SetTags(this, core::mem::transmute_copy(&tag1), core::mem::transmute_copy(&tag2)).into()
            }
        }
        unsafe extern "system" fn SetTextAntialiasMode<Identity: ID2D1CommandSink_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, textantialiasmode: super::d2d1::D2D1_TEXT_ANTIALIAS_MODE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID2D1CommandSink_Impl::SetTextAntialiasMode(this, core::mem::transmute_copy(&textantialiasmode)).into()
            }
        }
        unsafe extern "system" fn SetTextRenderingParams<Identity: ID2D1CommandSink_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, textrenderingparams: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID2D1CommandSink_Impl::SetTextRenderingParams(this, core::mem::transmute_copy(&textrenderingparams)).into()
            }
        }
        unsafe extern "system" fn SetTransform<Identity: ID2D1CommandSink_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, transform: *const windows_numerics::Matrix3x2) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID2D1CommandSink_Impl::SetTransform(this, core::mem::transmute_copy(&transform)).into()
            }
        }
        unsafe extern "system" fn SetPrimitiveBlend<Identity: ID2D1CommandSink_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, primitiveblend: D2D1_PRIMITIVE_BLEND) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID2D1CommandSink_Impl::SetPrimitiveBlend(this, core::mem::transmute_copy(&primitiveblend)).into()
            }
        }
        unsafe extern "system" fn SetUnitMode<Identity: ID2D1CommandSink_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, unitmode: D2D1_UNIT_MODE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID2D1CommandSink_Impl::SetUnitMode(this, core::mem::transmute_copy(&unitmode)).into()
            }
        }
        unsafe extern "system" fn Clear<Identity: ID2D1CommandSink_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, color: *const super::d2dbasetypes::D2D_COLOR_F) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID2D1CommandSink_Impl::Clear(this, core::mem::transmute_copy(&color)).into()
            }
        }
        unsafe extern "system" fn DrawGlyphRun<Identity: ID2D1CommandSink_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, baselineorigin: windows_numerics::Vector2, glyphrun: *const super::dwrite::DWRITE_GLYPH_RUN, glyphrundescription: *const super::dwrite::DWRITE_GLYPH_RUN_DESCRIPTION, foregroundbrush: *mut core::ffi::c_void, measuringmode: super::dcommon::DWRITE_MEASURING_MODE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID2D1CommandSink_Impl::DrawGlyphRun(this, core::mem::transmute(&baselineorigin), core::mem::transmute_copy(&glyphrun), core::mem::transmute_copy(&glyphrundescription), core::mem::transmute_copy(&foregroundbrush), core::mem::transmute_copy(&measuringmode)).into()
            }
        }
        unsafe extern "system" fn DrawLine<Identity: ID2D1CommandSink_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, point0: windows_numerics::Vector2, point1: windows_numerics::Vector2, brush: *mut core::ffi::c_void, strokewidth: f32, strokestyle: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID2D1CommandSink_Impl::DrawLine(this, core::mem::transmute(&point0), core::mem::transmute(&point1), core::mem::transmute_copy(&brush), core::mem::transmute_copy(&strokewidth), core::mem::transmute_copy(&strokestyle)).into()
            }
        }
        unsafe extern "system" fn DrawGeometry<Identity: ID2D1CommandSink_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, geometry: *mut core::ffi::c_void, brush: *mut core::ffi::c_void, strokewidth: f32, strokestyle: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID2D1CommandSink_Impl::DrawGeometry(this, core::mem::transmute_copy(&geometry), core::mem::transmute_copy(&brush), core::mem::transmute_copy(&strokewidth), core::mem::transmute_copy(&strokestyle)).into()
            }
        }
        unsafe extern "system" fn DrawRectangle<Identity: ID2D1CommandSink_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, rect: *const super::dcommon::D2D_RECT_F, brush: *mut core::ffi::c_void, strokewidth: f32, strokestyle: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID2D1CommandSink_Impl::DrawRectangle(this, core::mem::transmute_copy(&rect), core::mem::transmute_copy(&brush), core::mem::transmute_copy(&strokewidth), core::mem::transmute_copy(&strokestyle)).into()
            }
        }
        unsafe extern "system" fn DrawBitmap<Identity: ID2D1CommandSink_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bitmap: *mut core::ffi::c_void, destinationrectangle: *const super::dcommon::D2D_RECT_F, opacity: f32, interpolationmode: D2D1_INTERPOLATION_MODE, sourcerectangle: *const super::dcommon::D2D_RECT_F, perspectivetransform: *const windows_numerics::Matrix4x4) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID2D1CommandSink_Impl::DrawBitmap(this, core::mem::transmute_copy(&bitmap), core::mem::transmute_copy(&destinationrectangle), core::mem::transmute_copy(&opacity), core::mem::transmute_copy(&interpolationmode), core::mem::transmute_copy(&sourcerectangle), core::mem::transmute_copy(&perspectivetransform)).into()
            }
        }
        unsafe extern "system" fn DrawImage<Identity: ID2D1CommandSink_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, image: *mut core::ffi::c_void, targetoffset: *const windows_numerics::Vector2, imagerectangle: *const super::dcommon::D2D_RECT_F, interpolationmode: D2D1_INTERPOLATION_MODE, compositemode: D2D1_COMPOSITE_MODE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID2D1CommandSink_Impl::DrawImage(this, core::mem::transmute_copy(&image), core::mem::transmute_copy(&targetoffset), core::mem::transmute_copy(&imagerectangle), core::mem::transmute_copy(&interpolationmode), core::mem::transmute_copy(&compositemode)).into()
            }
        }
        unsafe extern "system" fn DrawGdiMetafile<Identity: ID2D1CommandSink_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, gdimetafile: *mut core::ffi::c_void, targetoffset: *const windows_numerics::Vector2) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID2D1CommandSink_Impl::DrawGdiMetafile(this, core::mem::transmute_copy(&gdimetafile), core::mem::transmute_copy(&targetoffset)).into()
            }
        }
        unsafe extern "system" fn FillMesh<Identity: ID2D1CommandSink_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, mesh: *mut core::ffi::c_void, brush: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID2D1CommandSink_Impl::FillMesh(this, core::mem::transmute_copy(&mesh), core::mem::transmute_copy(&brush)).into()
            }
        }
        unsafe extern "system" fn FillOpacityMask<Identity: ID2D1CommandSink_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, opacitymask: *mut core::ffi::c_void, brush: *mut core::ffi::c_void, destinationrectangle: *const super::dcommon::D2D_RECT_F, sourcerectangle: *const super::dcommon::D2D_RECT_F) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID2D1CommandSink_Impl::FillOpacityMask(this, core::mem::transmute_copy(&opacitymask), core::mem::transmute_copy(&brush), core::mem::transmute_copy(&destinationrectangle), core::mem::transmute_copy(&sourcerectangle)).into()
            }
        }
        unsafe extern "system" fn FillGeometry<Identity: ID2D1CommandSink_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, geometry: *mut core::ffi::c_void, brush: *mut core::ffi::c_void, opacitybrush: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID2D1CommandSink_Impl::FillGeometry(this, core::mem::transmute_copy(&geometry), core::mem::transmute_copy(&brush), core::mem::transmute_copy(&opacitybrush)).into()
            }
        }
        unsafe extern "system" fn FillRectangle<Identity: ID2D1CommandSink_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, rect: *const super::dcommon::D2D_RECT_F, brush: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID2D1CommandSink_Impl::FillRectangle(this, core::mem::transmute_copy(&rect), core::mem::transmute_copy(&brush)).into()
            }
        }
        unsafe extern "system" fn PushAxisAlignedClip<Identity: ID2D1CommandSink_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, cliprect: *const super::dcommon::D2D_RECT_F, antialiasmode: super::d2d1::D2D1_ANTIALIAS_MODE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID2D1CommandSink_Impl::PushAxisAlignedClip(this, core::mem::transmute_copy(&cliprect), core::mem::transmute_copy(&antialiasmode)).into()
            }
        }
        unsafe extern "system" fn PushLayer<Identity: ID2D1CommandSink_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, layerparameters1: *const D2D1_LAYER_PARAMETERS1, layer: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID2D1CommandSink_Impl::PushLayer(this, core::mem::transmute_copy(&layerparameters1), core::mem::transmute_copy(&layer)).into()
            }
        }
        unsafe extern "system" fn PopAxisAlignedClip<Identity: ID2D1CommandSink_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID2D1CommandSink_Impl::PopAxisAlignedClip(this).into()
            }
        }
        unsafe extern "system" fn PopLayer<Identity: ID2D1CommandSink_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID2D1CommandSink_Impl::PopLayer(this).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            BeginDraw: BeginDraw::<Identity, OFFSET>,
            EndDraw: EndDraw::<Identity, OFFSET>,
            SetAntialiasMode: SetAntialiasMode::<Identity, OFFSET>,
            SetTags: SetTags::<Identity, OFFSET>,
            SetTextAntialiasMode: SetTextAntialiasMode::<Identity, OFFSET>,
            SetTextRenderingParams: SetTextRenderingParams::<Identity, OFFSET>,
            SetTransform: SetTransform::<Identity, OFFSET>,
            SetPrimitiveBlend: SetPrimitiveBlend::<Identity, OFFSET>,
            SetUnitMode: SetUnitMode::<Identity, OFFSET>,
            Clear: Clear::<Identity, OFFSET>,
            DrawGlyphRun: DrawGlyphRun::<Identity, OFFSET>,
            DrawLine: DrawLine::<Identity, OFFSET>,
            DrawGeometry: DrawGeometry::<Identity, OFFSET>,
            DrawRectangle: DrawRectangle::<Identity, OFFSET>,
            DrawBitmap: DrawBitmap::<Identity, OFFSET>,
            DrawImage: DrawImage::<Identity, OFFSET>,
            DrawGdiMetafile: DrawGdiMetafile::<Identity, OFFSET>,
            FillMesh: FillMesh::<Identity, OFFSET>,
            FillOpacityMask: FillOpacityMask::<Identity, OFFSET>,
            FillGeometry: FillGeometry::<Identity, OFFSET>,
            FillRectangle: FillRectangle::<Identity, OFFSET>,
            PushAxisAlignedClip: PushAxisAlignedClip::<Identity, OFFSET>,
            PushLayer: PushLayer::<Identity, OFFSET>,
            PopAxisAlignedClip: PopAxisAlignedClip::<Identity, OFFSET>,
            PopLayer: PopLayer::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ID2D1CommandSink as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_d2d1", feature = "Win32_d2dbasetypes", feature = "Win32_dcommon", feature = "Win32_dwrite", feature = "Win32_dxgitype"))]
impl windows_core::RuntimeName for ID2D1CommandSink {}
#[cfg(feature = "Win32_d2d1")]
windows_core::imp::define_interface!(ID2D1Device, ID2D1Device_Vtbl, 0x47dd575d_ac05_4cdd_8049_9b02cd16f44c);
#[cfg(feature = "Win32_d2d1")]
impl core::ops::Deref for ID2D1Device {
    type Target = super::d2d1::ID2D1Resource;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_d2d1")]
windows_core::imp::interface_hierarchy!(ID2D1Device, windows_core::IUnknown, super::d2d1::ID2D1Resource);
#[cfg(feature = "Win32_d2d1")]
impl ID2D1Device {
    pub unsafe fn CreateDeviceContext(&self, options: D2D1_DEVICE_CONTEXT_OPTIONS) -> windows_core::Result<ID2D1DeviceContext> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateDeviceContext)(windows_core::Interface::as_raw(self), options, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Win32_wincodec")]
    pub unsafe fn CreatePrintControl<P0>(&self, wicfactory: P0, documenttarget: *const IPrintDocumentPackageTarget, printcontrolproperties: Option<*const D2D1_PRINT_CONTROL_PROPERTIES>) -> windows_core::Result<ID2D1PrintControl>
    where
        P0: windows_core::Param<super::wincodec::IWICImagingFactory>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreatePrintControl)(windows_core::Interface::as_raw(self), wicfactory.param().abi(), documenttarget, printcontrolproperties.unwrap_or(core::mem::zeroed()) as _, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn SetMaximumTextureMemory(&self, maximuminbytes: u64) {
        unsafe {
            (windows_core::Interface::vtable(self).SetMaximumTextureMemory)(windows_core::Interface::as_raw(self), maximuminbytes);
        }
    }
    pub unsafe fn GetMaximumTextureMemory(&self) -> u64 {
        unsafe { (windows_core::Interface::vtable(self).GetMaximumTextureMemory)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn ClearResources(&self, millisecondssinceuse: u32) {
        unsafe {
            (windows_core::Interface::vtable(self).ClearResources)(windows_core::Interface::as_raw(self), millisecondssinceuse);
        }
    }
}
#[cfg(feature = "Win32_d2d1")]
#[repr(C)]
#[doc(hidden)]
pub struct ID2D1Device_Vtbl {
    pub base__: super::d2d1::ID2D1Resource_Vtbl,
    pub CreateDeviceContext: unsafe extern "system" fn(*mut core::ffi::c_void, D2D1_DEVICE_CONTEXT_OPTIONS, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_wincodec")]
    pub CreatePrintControl: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *const IPrintDocumentPackageTarget, *const D2D1_PRINT_CONTROL_PROPERTIES, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_wincodec"))]
    CreatePrintControl: usize,
    pub SetMaximumTextureMemory: unsafe extern "system" fn(*mut core::ffi::c_void, u64),
    pub GetMaximumTextureMemory: unsafe extern "system" fn(*mut core::ffi::c_void) -> u64,
    pub ClearResources: unsafe extern "system" fn(*mut core::ffi::c_void, u32),
}
#[cfg(all(feature = "Win32_d2d1", feature = "Win32_wincodec"))]
pub trait ID2D1Device_Impl: super::d2d1::ID2D1Resource_Impl {
    fn CreateDeviceContext(&self, options: D2D1_DEVICE_CONTEXT_OPTIONS) -> windows_core::Result<ID2D1DeviceContext>;
    fn CreatePrintControl(&self, wicfactory: windows_core::Ref<super::wincodec::IWICImagingFactory>, documenttarget: *const IPrintDocumentPackageTarget, printcontrolproperties: *const D2D1_PRINT_CONTROL_PROPERTIES) -> windows_core::Result<ID2D1PrintControl>;
    fn SetMaximumTextureMemory(&self, maximuminbytes: u64);
    fn GetMaximumTextureMemory(&self) -> u64;
    fn ClearResources(&self, millisecondssinceuse: u32);
}
#[cfg(all(feature = "Win32_d2d1", feature = "Win32_wincodec"))]
impl ID2D1Device_Vtbl {
    pub const fn new<Identity: ID2D1Device_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn CreateDeviceContext<Identity: ID2D1Device_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, options: D2D1_DEVICE_CONTEXT_OPTIONS, devicecontext: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ID2D1Device_Impl::CreateDeviceContext(this, core::mem::transmute_copy(&options)) {
                    Ok(ok__) => {
                        devicecontext.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CreatePrintControl<Identity: ID2D1Device_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, wicfactory: *mut core::ffi::c_void, documenttarget: *const IPrintDocumentPackageTarget, printcontrolproperties: *const D2D1_PRINT_CONTROL_PROPERTIES, printcontrol: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ID2D1Device_Impl::CreatePrintControl(this, core::mem::transmute_copy(&wicfactory), core::mem::transmute_copy(&documenttarget), core::mem::transmute_copy(&printcontrolproperties)) {
                    Ok(ok__) => {
                        printcontrol.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetMaximumTextureMemory<Identity: ID2D1Device_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, maximuminbytes: u64) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID2D1Device_Impl::SetMaximumTextureMemory(this, core::mem::transmute_copy(&maximuminbytes));
            }
        }
        unsafe extern "system" fn GetMaximumTextureMemory<Identity: ID2D1Device_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> u64 {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID2D1Device_Impl::GetMaximumTextureMemory(this)
            }
        }
        unsafe extern "system" fn ClearResources<Identity: ID2D1Device_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, millisecondssinceuse: u32) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID2D1Device_Impl::ClearResources(this, core::mem::transmute_copy(&millisecondssinceuse));
            }
        }
        Self {
            base__: super::d2d1::ID2D1Resource_Vtbl::new::<Identity, OFFSET>(),
            CreateDeviceContext: CreateDeviceContext::<Identity, OFFSET>,
            CreatePrintControl: CreatePrintControl::<Identity, OFFSET>,
            SetMaximumTextureMemory: SetMaximumTextureMemory::<Identity, OFFSET>,
            GetMaximumTextureMemory: GetMaximumTextureMemory::<Identity, OFFSET>,
            ClearResources: ClearResources::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ID2D1Device as windows_core::Interface>::IID || iid == &<super::d2d1::ID2D1Resource as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_d2d1", feature = "Win32_wincodec"))]
impl windows_core::RuntimeName for ID2D1Device {}
#[cfg(feature = "Win32_d2d1")]
windows_core::imp::define_interface!(ID2D1DeviceContext, ID2D1DeviceContext_Vtbl, 0xe8f7fe7a_191c_466d_ad95_975678bda998);
#[cfg(feature = "Win32_d2d1")]
impl core::ops::Deref for ID2D1DeviceContext {
    type Target = super::d2d1::ID2D1RenderTarget;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_d2d1")]
windows_core::imp::interface_hierarchy!(ID2D1DeviceContext, windows_core::IUnknown, super::d2d1::ID2D1Resource, super::d2d1::ID2D1RenderTarget);
#[cfg(feature = "Win32_d2d1")]
impl ID2D1DeviceContext {
    #[cfg(all(feature = "Win32_dcommon", feature = "Win32_dxgiformat"))]
    pub unsafe fn CreateBitmap(&self, size: super::dcommon::D2D_SIZE_U, sourcedata: Option<*const core::ffi::c_void>, pitch: u32, bitmapproperties: *const D2D1_BITMAP_PROPERTIES1) -> windows_core::Result<ID2D1Bitmap1> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateBitmap)(windows_core::Interface::as_raw(self), core::mem::transmute(size), sourcedata.unwrap_or(core::mem::zeroed()) as _, pitch, core::mem::transmute(bitmapproperties), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(all(feature = "Win32_dcommon", feature = "Win32_dxgiformat", feature = "Win32_wincodec"))]
    pub unsafe fn CreateBitmapFromWicBitmap<P0>(&self, wicbitmapsource: P0, bitmapproperties: Option<*const D2D1_BITMAP_PROPERTIES1>) -> windows_core::Result<ID2D1Bitmap1>
    where
        P0: windows_core::Param<super::wincodec::IWICBitmapSource>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateBitmapFromWicBitmap)(windows_core::Interface::as_raw(self), wicbitmapsource.param().abi(), bitmapproperties.unwrap_or(core::mem::zeroed()) as _, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn CreateColorContext(&self, space: D2D1_COLOR_SPACE, profile: Option<&[u8]>) -> windows_core::Result<ID2D1ColorContext> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateColorContext)(windows_core::Interface::as_raw(self), space, core::mem::transmute(profile.map_or(core::ptr::null(), |slice| slice.as_ptr())), profile.map_or(0, |slice| slice.len().try_into().unwrap()), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn CreateColorContextFromFilename<P0>(&self, filename: P0) -> windows_core::Result<ID2D1ColorContext>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateColorContextFromFilename)(windows_core::Interface::as_raw(self), filename.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Win32_wincodec")]
    pub unsafe fn CreateColorContextFromWicColorContext<P0>(&self, wiccolorcontext: P0) -> windows_core::Result<ID2D1ColorContext>
    where
        P0: windows_core::Param<super::wincodec::IWICColorContext>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateColorContextFromWicColorContext)(windows_core::Interface::as_raw(self), wiccolorcontext.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(all(feature = "Win32_dcommon", feature = "Win32_dxgi", feature = "Win32_dxgiformat"))]
    pub unsafe fn CreateBitmapFromDxgiSurface<P0>(&self, surface: P0, bitmapproperties: Option<*const D2D1_BITMAP_PROPERTIES1>) -> windows_core::Result<ID2D1Bitmap1>
    where
        P0: windows_core::Param<super::dxgi::IDXGISurface>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateBitmapFromDxgiSurface)(windows_core::Interface::as_raw(self), surface.param().abi(), bitmapproperties.unwrap_or(core::mem::zeroed()) as _, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn CreateEffect(&self, effectid: *const windows_core::GUID) -> windows_core::Result<ID2D1Effect> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateEffect)(windows_core::Interface::as_raw(self), effectid, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(all(feature = "Win32_d2dbasetypes", feature = "Win32_dxgitype"))]
    pub unsafe fn CreateGradientStopCollection(&self, straightalphagradientstops: &[super::d2d1::D2D1_GRADIENT_STOP], preinterpolationspace: D2D1_COLOR_SPACE, postinterpolationspace: D2D1_COLOR_SPACE, bufferprecision: D2D1_BUFFER_PRECISION, extendmode: super::d2d1::D2D1_EXTEND_MODE, colorinterpolationmode: D2D1_COLOR_INTERPOLATION_MODE) -> windows_core::Result<ID2D1GradientStopCollection1> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateGradientStopCollection)(windows_core::Interface::as_raw(self), core::mem::transmute(straightalphagradientstops.as_ptr()), straightalphagradientstops.len().try_into().unwrap(), preinterpolationspace, postinterpolationspace, bufferprecision, extendmode, colorinterpolationmode, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Win32_dcommon")]
    pub unsafe fn CreateImageBrush<P0>(&self, image: P0, imagebrushproperties: *const D2D1_IMAGE_BRUSH_PROPERTIES, brushproperties: Option<*const super::d2d1::D2D1_BRUSH_PROPERTIES>) -> windows_core::Result<ID2D1ImageBrush>
    where
        P0: windows_core::Param<super::d2d1::ID2D1Image>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateImageBrush)(windows_core::Interface::as_raw(self), image.param().abi(), imagebrushproperties, brushproperties.unwrap_or(core::mem::zeroed()) as _, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn CreateBitmapBrush<P0>(&self, bitmap: P0, bitmapbrushproperties: Option<*const D2D1_BITMAP_BRUSH_PROPERTIES1>, brushproperties: Option<*const super::d2d1::D2D1_BRUSH_PROPERTIES>) -> windows_core::Result<ID2D1BitmapBrush1>
    where
        P0: windows_core::Param<super::d2d1::ID2D1Bitmap>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateBitmapBrush)(windows_core::Interface::as_raw(self), bitmap.param().abi(), bitmapbrushproperties.unwrap_or(core::mem::zeroed()) as _, brushproperties.unwrap_or(core::mem::zeroed()) as _, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn CreateCommandList(&self) -> windows_core::Result<ID2D1CommandList> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateCommandList)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Win32_dxgiformat")]
    pub unsafe fn IsDxgiFormatSupported(&self, format: super::dxgiformat::DXGI_FORMAT) -> windows_core::BOOL {
        unsafe { (windows_core::Interface::vtable(self).IsDxgiFormatSupported)(windows_core::Interface::as_raw(self), format) }
    }
    pub unsafe fn IsBufferPrecisionSupported(&self, bufferprecision: D2D1_BUFFER_PRECISION) -> windows_core::BOOL {
        unsafe { (windows_core::Interface::vtable(self).IsBufferPrecisionSupported)(windows_core::Interface::as_raw(self), bufferprecision) }
    }
    #[cfg(feature = "Win32_dcommon")]
    pub unsafe fn GetImageLocalBounds<P0>(&self, image: P0) -> windows_core::Result<super::dcommon::D2D_RECT_F>
    where
        P0: windows_core::Param<super::d2d1::ID2D1Image>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetImageLocalBounds)(windows_core::Interface::as_raw(self), image.param().abi(), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Win32_dcommon")]
    pub unsafe fn GetImageWorldBounds<P0>(&self, image: P0) -> windows_core::Result<super::dcommon::D2D_RECT_F>
    where
        P0: windows_core::Param<super::d2d1::ID2D1Image>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetImageWorldBounds)(windows_core::Interface::as_raw(self), image.param().abi(), &mut result__).map(|| result__)
        }
    }
    #[cfg(all(feature = "Win32_dcommon", feature = "Win32_dwrite"))]
    pub unsafe fn GetGlyphRunWorldBounds(&self, baselineorigin: windows_numerics::Vector2, glyphrun: *const super::dwrite::DWRITE_GLYPH_RUN, measuringmode: super::dcommon::DWRITE_MEASURING_MODE) -> windows_core::Result<super::dcommon::D2D_RECT_F> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetGlyphRunWorldBounds)(windows_core::Interface::as_raw(self), core::mem::transmute(baselineorigin), core::mem::transmute(glyphrun), measuringmode, &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetDevice(&self) -> windows_core::Result<ID2D1Device> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetDevice)(windows_core::Interface::as_raw(self), &mut result__);
            windows_core::Type::from_abi(result__)
        }
    }
    pub unsafe fn SetTarget<P0>(&self, image: P0)
    where
        P0: windows_core::Param<super::d2d1::ID2D1Image>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).SetTarget)(windows_core::Interface::as_raw(self), image.param().abi());
        }
    }
    pub unsafe fn GetTarget(&self) -> windows_core::Result<super::d2d1::ID2D1Image> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetTarget)(windows_core::Interface::as_raw(self), &mut result__);
            windows_core::Type::from_abi(result__)
        }
    }
    #[cfg(feature = "Win32_dcommon")]
    pub unsafe fn SetRenderingControls(&self, renderingcontrols: *const D2D1_RENDERING_CONTROLS) {
        unsafe {
            (windows_core::Interface::vtable(self).SetRenderingControls)(windows_core::Interface::as_raw(self), renderingcontrols);
        }
    }
    #[cfg(feature = "Win32_dcommon")]
    pub unsafe fn GetRenderingControls(&self) -> D2D1_RENDERING_CONTROLS {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetRenderingControls)(windows_core::Interface::as_raw(self), &mut result__);
            result__
        }
    }
    pub unsafe fn SetPrimitiveBlend(&self, primitiveblend: D2D1_PRIMITIVE_BLEND) {
        unsafe {
            (windows_core::Interface::vtable(self).SetPrimitiveBlend)(windows_core::Interface::as_raw(self), primitiveblend);
        }
    }
    pub unsafe fn GetPrimitiveBlend(&self) -> D2D1_PRIMITIVE_BLEND {
        unsafe { (windows_core::Interface::vtable(self).GetPrimitiveBlend)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn SetUnitMode(&self, unitmode: D2D1_UNIT_MODE) {
        unsafe {
            (windows_core::Interface::vtable(self).SetUnitMode)(windows_core::Interface::as_raw(self), unitmode);
        }
    }
    pub unsafe fn GetUnitMode(&self) -> D2D1_UNIT_MODE {
        unsafe { (windows_core::Interface::vtable(self).GetUnitMode)(windows_core::Interface::as_raw(self)) }
    }
    #[cfg(all(feature = "Win32_dcommon", feature = "Win32_dwrite"))]
    pub unsafe fn DrawGlyphRun<P3>(&self, baselineorigin: windows_numerics::Vector2, glyphrun: *const super::dwrite::DWRITE_GLYPH_RUN, glyphrundescription: Option<*const super::dwrite::DWRITE_GLYPH_RUN_DESCRIPTION>, foregroundbrush: P3, measuringmode: super::dcommon::DWRITE_MEASURING_MODE)
    where
        P3: windows_core::Param<super::d2d1::ID2D1Brush>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).DrawGlyphRun)(windows_core::Interface::as_raw(self), core::mem::transmute(baselineorigin), core::mem::transmute(glyphrun), glyphrundescription.unwrap_or(core::mem::zeroed()) as _, foregroundbrush.param().abi(), measuringmode);
        }
    }
    #[cfg(feature = "Win32_dcommon")]
    pub unsafe fn DrawImage<P0>(&self, image: P0, targetoffset: Option<*const windows_numerics::Vector2>, imagerectangle: Option<*const super::dcommon::D2D_RECT_F>, interpolationmode: D2D1_INTERPOLATION_MODE, compositemode: D2D1_COMPOSITE_MODE)
    where
        P0: windows_core::Param<super::d2d1::ID2D1Image>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).DrawImage)(windows_core::Interface::as_raw(self), image.param().abi(), targetoffset.unwrap_or(core::mem::zeroed()) as _, imagerectangle.unwrap_or(core::mem::zeroed()) as _, interpolationmode, compositemode);
        }
    }
    pub unsafe fn DrawGdiMetafile<P0>(&self, gdimetafile: P0, targetoffset: Option<*const windows_numerics::Vector2>)
    where
        P0: windows_core::Param<ID2D1GdiMetafile>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).DrawGdiMetafile)(windows_core::Interface::as_raw(self), gdimetafile.param().abi(), targetoffset.unwrap_or(core::mem::zeroed()) as _);
        }
    }
    #[cfg(feature = "Win32_dcommon")]
    pub unsafe fn DrawBitmap<P0>(&self, bitmap: P0, destinationrectangle: Option<*const super::dcommon::D2D_RECT_F>, opacity: f32, interpolationmode: D2D1_INTERPOLATION_MODE, sourcerectangle: Option<*const super::dcommon::D2D_RECT_F>, perspectivetransform: Option<*const windows_numerics::Matrix4x4>)
    where
        P0: windows_core::Param<super::d2d1::ID2D1Bitmap>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).DrawBitmap)(windows_core::Interface::as_raw(self), bitmap.param().abi(), destinationrectangle.unwrap_or(core::mem::zeroed()) as _, opacity, interpolationmode, sourcerectangle.unwrap_or(core::mem::zeroed()) as _, perspectivetransform.unwrap_or(core::mem::zeroed()) as _);
        }
    }
    #[cfg(feature = "Win32_dcommon")]
    pub unsafe fn PushLayer<P1>(&self, layerparameters: *const D2D1_LAYER_PARAMETERS1, layer: P1)
    where
        P1: windows_core::Param<super::d2d1::ID2D1Layer>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).PushLayer)(windows_core::Interface::as_raw(self), core::mem::transmute(layerparameters), layer.param().abi());
        }
    }
    #[cfg(feature = "Win32_dcommon")]
    pub unsafe fn InvalidateEffectInputRectangle<P0>(&self, effect: P0, input: u32, inputrectangle: *const super::dcommon::D2D_RECT_F) -> windows_core::HRESULT
    where
        P0: windows_core::Param<ID2D1Effect>,
    {
        unsafe { (windows_core::Interface::vtable(self).InvalidateEffectInputRectangle)(windows_core::Interface::as_raw(self), effect.param().abi(), input, inputrectangle) }
    }
    pub unsafe fn GetEffectInvalidRectangleCount<P0>(&self, effect: P0) -> windows_core::Result<u32>
    where
        P0: windows_core::Param<ID2D1Effect>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetEffectInvalidRectangleCount)(windows_core::Interface::as_raw(self), effect.param().abi(), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Win32_dcommon")]
    pub unsafe fn GetEffectInvalidRectangles<P0>(&self, effect: P0, rectangles: &mut [super::dcommon::D2D_RECT_F]) -> windows_core::HRESULT
    where
        P0: windows_core::Param<ID2D1Effect>,
    {
        unsafe { (windows_core::Interface::vtable(self).GetEffectInvalidRectangles)(windows_core::Interface::as_raw(self), effect.param().abi(), core::mem::transmute(rectangles.as_ptr()), rectangles.len().try_into().unwrap()) }
    }
    #[cfg(feature = "Win32_dcommon")]
    pub unsafe fn GetEffectRequiredInputRectangles<P0>(&self, rendereffect: P0, renderimagerectangle: Option<*const super::dcommon::D2D_RECT_F>, inputdescriptions: *const D2D1_EFFECT_INPUT_DESCRIPTION, requiredinputrects: *mut super::dcommon::D2D_RECT_F, inputcount: u32) -> windows_core::HRESULT
    where
        P0: windows_core::Param<ID2D1Effect>,
    {
        unsafe { (windows_core::Interface::vtable(self).GetEffectRequiredInputRectangles)(windows_core::Interface::as_raw(self), rendereffect.param().abi(), renderimagerectangle.unwrap_or(core::mem::zeroed()) as _, core::mem::transmute(inputdescriptions), requiredinputrects as _, inputcount) }
    }
    #[cfg(feature = "Win32_dcommon")]
    pub unsafe fn FillOpacityMask<P0, P1>(&self, opacitymask: P0, brush: P1, destinationrectangle: Option<*const super::dcommon::D2D_RECT_F>, sourcerectangle: Option<*const super::dcommon::D2D_RECT_F>)
    where
        P0: windows_core::Param<super::d2d1::ID2D1Bitmap>,
        P1: windows_core::Param<super::d2d1::ID2D1Brush>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).FillOpacityMask)(windows_core::Interface::as_raw(self), opacitymask.param().abi(), brush.param().abi(), destinationrectangle.unwrap_or(core::mem::zeroed()) as _, sourcerectangle.unwrap_or(core::mem::zeroed()) as _);
        }
    }
}
#[cfg(feature = "Win32_d2d1")]
#[repr(C)]
#[doc(hidden)]
pub struct ID2D1DeviceContext_Vtbl {
    pub base__: super::d2d1::ID2D1RenderTarget_Vtbl,
    #[cfg(all(feature = "Win32_dcommon", feature = "Win32_dxgiformat"))]
    pub CreateBitmap: unsafe extern "system" fn(*mut core::ffi::c_void, super::dcommon::D2D_SIZE_U, *const core::ffi::c_void, u32, *const D2D1_BITMAP_PROPERTIES1, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_dcommon", feature = "Win32_dxgiformat")))]
    CreateBitmap: usize,
    #[cfg(all(feature = "Win32_dcommon", feature = "Win32_dxgiformat", feature = "Win32_wincodec"))]
    pub CreateBitmapFromWicBitmap: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *const D2D1_BITMAP_PROPERTIES1, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_dcommon", feature = "Win32_dxgiformat", feature = "Win32_wincodec")))]
    CreateBitmapFromWicBitmap: usize,
    pub CreateColorContext: unsafe extern "system" fn(*mut core::ffi::c_void, D2D1_COLOR_SPACE, *const u8, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateColorContextFromFilename: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_wincodec")]
    pub CreateColorContextFromWicColorContext: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_wincodec"))]
    CreateColorContextFromWicColorContext: usize,
    #[cfg(all(feature = "Win32_dcommon", feature = "Win32_dxgi", feature = "Win32_dxgiformat"))]
    pub CreateBitmapFromDxgiSurface: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *const D2D1_BITMAP_PROPERTIES1, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_dcommon", feature = "Win32_dxgi", feature = "Win32_dxgiformat")))]
    CreateBitmapFromDxgiSurface: usize,
    pub CreateEffect: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(all(feature = "Win32_d2dbasetypes", feature = "Win32_dxgitype"))]
    pub CreateGradientStopCollection: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::d2d1::D2D1_GRADIENT_STOP, u32, D2D1_COLOR_SPACE, D2D1_COLOR_SPACE, D2D1_BUFFER_PRECISION, super::d2d1::D2D1_EXTEND_MODE, D2D1_COLOR_INTERPOLATION_MODE, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_d2dbasetypes", feature = "Win32_dxgitype")))]
    CreateGradientStopCollection: usize,
    #[cfg(feature = "Win32_dcommon")]
    pub CreateImageBrush: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *const D2D1_IMAGE_BRUSH_PROPERTIES, *const super::d2d1::D2D1_BRUSH_PROPERTIES, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_dcommon"))]
    CreateImageBrush: usize,
    pub CreateBitmapBrush: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *const D2D1_BITMAP_BRUSH_PROPERTIES1, *const super::d2d1::D2D1_BRUSH_PROPERTIES, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateCommandList: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_dxgiformat")]
    pub IsDxgiFormatSupported: unsafe extern "system" fn(*mut core::ffi::c_void, super::dxgiformat::DXGI_FORMAT) -> windows_core::BOOL,
    #[cfg(not(feature = "Win32_dxgiformat"))]
    IsDxgiFormatSupported: usize,
    pub IsBufferPrecisionSupported: unsafe extern "system" fn(*mut core::ffi::c_void, D2D1_BUFFER_PRECISION) -> windows_core::BOOL,
    #[cfg(feature = "Win32_dcommon")]
    pub GetImageLocalBounds: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::dcommon::D2D_RECT_F) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_dcommon"))]
    GetImageLocalBounds: usize,
    #[cfg(feature = "Win32_dcommon")]
    pub GetImageWorldBounds: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::dcommon::D2D_RECT_F) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_dcommon"))]
    GetImageWorldBounds: usize,
    #[cfg(all(feature = "Win32_dcommon", feature = "Win32_dwrite"))]
    pub GetGlyphRunWorldBounds: unsafe extern "system" fn(*mut core::ffi::c_void, windows_numerics::Vector2, *const super::dwrite::DWRITE_GLYPH_RUN, super::dcommon::DWRITE_MEASURING_MODE, *mut super::dcommon::D2D_RECT_F) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_dcommon", feature = "Win32_dwrite")))]
    GetGlyphRunWorldBounds: usize,
    pub GetDevice: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void),
    pub SetTarget: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void),
    pub GetTarget: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void),
    #[cfg(feature = "Win32_dcommon")]
    pub SetRenderingControls: unsafe extern "system" fn(*mut core::ffi::c_void, *const D2D1_RENDERING_CONTROLS),
    #[cfg(not(feature = "Win32_dcommon"))]
    SetRenderingControls: usize,
    #[cfg(feature = "Win32_dcommon")]
    pub GetRenderingControls: unsafe extern "system" fn(*mut core::ffi::c_void, *mut D2D1_RENDERING_CONTROLS),
    #[cfg(not(feature = "Win32_dcommon"))]
    GetRenderingControls: usize,
    pub SetPrimitiveBlend: unsafe extern "system" fn(*mut core::ffi::c_void, D2D1_PRIMITIVE_BLEND),
    pub GetPrimitiveBlend: unsafe extern "system" fn(*mut core::ffi::c_void) -> D2D1_PRIMITIVE_BLEND,
    pub SetUnitMode: unsafe extern "system" fn(*mut core::ffi::c_void, D2D1_UNIT_MODE),
    pub GetUnitMode: unsafe extern "system" fn(*mut core::ffi::c_void) -> D2D1_UNIT_MODE,
    #[cfg(all(feature = "Win32_dcommon", feature = "Win32_dwrite"))]
    pub DrawGlyphRun: unsafe extern "system" fn(*mut core::ffi::c_void, windows_numerics::Vector2, *const super::dwrite::DWRITE_GLYPH_RUN, *const super::dwrite::DWRITE_GLYPH_RUN_DESCRIPTION, *mut core::ffi::c_void, super::dcommon::DWRITE_MEASURING_MODE),
    #[cfg(not(all(feature = "Win32_dcommon", feature = "Win32_dwrite")))]
    DrawGlyphRun: usize,
    #[cfg(feature = "Win32_dcommon")]
    pub DrawImage: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *const windows_numerics::Vector2, *const super::dcommon::D2D_RECT_F, D2D1_INTERPOLATION_MODE, D2D1_COMPOSITE_MODE),
    #[cfg(not(feature = "Win32_dcommon"))]
    DrawImage: usize,
    pub DrawGdiMetafile: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *const windows_numerics::Vector2),
    #[cfg(feature = "Win32_dcommon")]
    pub DrawBitmap: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *const super::dcommon::D2D_RECT_F, f32, D2D1_INTERPOLATION_MODE, *const super::dcommon::D2D_RECT_F, *const windows_numerics::Matrix4x4),
    #[cfg(not(feature = "Win32_dcommon"))]
    DrawBitmap: usize,
    #[cfg(feature = "Win32_dcommon")]
    pub PushLayer: unsafe extern "system" fn(*mut core::ffi::c_void, *const D2D1_LAYER_PARAMETERS1, *mut core::ffi::c_void),
    #[cfg(not(feature = "Win32_dcommon"))]
    PushLayer: usize,
    #[cfg(feature = "Win32_dcommon")]
    pub InvalidateEffectInputRectangle: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, u32, *const super::dcommon::D2D_RECT_F) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_dcommon"))]
    InvalidateEffectInputRectangle: usize,
    pub GetEffectInvalidRectangleCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_dcommon")]
    pub GetEffectInvalidRectangles: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::dcommon::D2D_RECT_F, u32) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_dcommon"))]
    GetEffectInvalidRectangles: usize,
    #[cfg(feature = "Win32_dcommon")]
    pub GetEffectRequiredInputRectangles: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *const super::dcommon::D2D_RECT_F, *const D2D1_EFFECT_INPUT_DESCRIPTION, *mut super::dcommon::D2D_RECT_F, u32) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_dcommon"))]
    GetEffectRequiredInputRectangles: usize,
    #[cfg(feature = "Win32_dcommon")]
    pub FillOpacityMask: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *const super::dcommon::D2D_RECT_F, *const super::dcommon::D2D_RECT_F),
    #[cfg(not(feature = "Win32_dcommon"))]
    FillOpacityMask: usize,
}
#[cfg(all(feature = "Win32_d2d1", feature = "Win32_d2dbasetypes", feature = "Win32_dcommon", feature = "Win32_dwrite", feature = "Win32_dxgi", feature = "Win32_dxgiformat", feature = "Win32_dxgitype", feature = "Win32_wincodec"))]
pub trait ID2D1DeviceContext_Impl: super::d2d1::ID2D1RenderTarget_Impl {
    fn CreateBitmap(&self, size: &super::dcommon::D2D_SIZE_U, sourcedata: *const core::ffi::c_void, pitch: u32, bitmapproperties: *const D2D1_BITMAP_PROPERTIES1) -> windows_core::Result<ID2D1Bitmap1>;
    fn CreateBitmapFromWicBitmap(&self, wicbitmapsource: windows_core::Ref<super::wincodec::IWICBitmapSource>, bitmapproperties: *const D2D1_BITMAP_PROPERTIES1) -> windows_core::Result<ID2D1Bitmap1>;
    fn CreateColorContext(&self, space: D2D1_COLOR_SPACE, profile: *const u8, profilesize: u32) -> windows_core::Result<ID2D1ColorContext>;
    fn CreateColorContextFromFilename(&self, filename: &windows_core::PCWSTR) -> windows_core::Result<ID2D1ColorContext>;
    fn CreateColorContextFromWicColorContext(&self, wiccolorcontext: windows_core::Ref<super::wincodec::IWICColorContext>) -> windows_core::Result<ID2D1ColorContext>;
    fn CreateBitmapFromDxgiSurface(&self, surface: windows_core::Ref<super::dxgi::IDXGISurface>, bitmapproperties: *const D2D1_BITMAP_PROPERTIES1) -> windows_core::Result<ID2D1Bitmap1>;
    fn CreateEffect(&self, effectid: *const windows_core::GUID) -> windows_core::Result<ID2D1Effect>;
    fn CreateGradientStopCollection(&self, straightalphagradientstops: *const super::d2d1::D2D1_GRADIENT_STOP, straightalphagradientstopscount: u32, preinterpolationspace: D2D1_COLOR_SPACE, postinterpolationspace: D2D1_COLOR_SPACE, bufferprecision: D2D1_BUFFER_PRECISION, extendmode: super::d2d1::D2D1_EXTEND_MODE, colorinterpolationmode: D2D1_COLOR_INTERPOLATION_MODE) -> windows_core::Result<ID2D1GradientStopCollection1>;
    fn CreateImageBrush(&self, image: windows_core::Ref<super::d2d1::ID2D1Image>, imagebrushproperties: *const D2D1_IMAGE_BRUSH_PROPERTIES, brushproperties: *const super::d2d1::D2D1_BRUSH_PROPERTIES) -> windows_core::Result<ID2D1ImageBrush>;
    fn CreateBitmapBrush(&self, bitmap: windows_core::Ref<super::d2d1::ID2D1Bitmap>, bitmapbrushproperties: *const D2D1_BITMAP_BRUSH_PROPERTIES1, brushproperties: *const super::d2d1::D2D1_BRUSH_PROPERTIES) -> windows_core::Result<ID2D1BitmapBrush1>;
    fn CreateCommandList(&self) -> windows_core::Result<ID2D1CommandList>;
    fn IsDxgiFormatSupported(&self, format: super::dxgiformat::DXGI_FORMAT) -> windows_core::BOOL;
    fn IsBufferPrecisionSupported(&self, bufferprecision: D2D1_BUFFER_PRECISION) -> windows_core::BOOL;
    fn GetImageLocalBounds(&self, image: windows_core::Ref<super::d2d1::ID2D1Image>) -> windows_core::Result<super::dcommon::D2D_RECT_F>;
    fn GetImageWorldBounds(&self, image: windows_core::Ref<super::d2d1::ID2D1Image>) -> windows_core::Result<super::dcommon::D2D_RECT_F>;
    fn GetGlyphRunWorldBounds(&self, baselineorigin: &windows_numerics::Vector2, glyphrun: *const super::dwrite::DWRITE_GLYPH_RUN, measuringmode: super::dcommon::DWRITE_MEASURING_MODE) -> windows_core::Result<super::dcommon::D2D_RECT_F>;
    fn GetDevice(&self, device: windows_core::OutRef<ID2D1Device>);
    fn SetTarget(&self, image: windows_core::Ref<super::d2d1::ID2D1Image>);
    fn GetTarget(&self, image: windows_core::OutRef<super::d2d1::ID2D1Image>);
    fn SetRenderingControls(&self, renderingcontrols: *const D2D1_RENDERING_CONTROLS);
    fn GetRenderingControls(&self, renderingcontrols: *mut D2D1_RENDERING_CONTROLS);
    fn SetPrimitiveBlend(&self, primitiveblend: D2D1_PRIMITIVE_BLEND);
    fn GetPrimitiveBlend(&self) -> D2D1_PRIMITIVE_BLEND;
    fn SetUnitMode(&self, unitmode: D2D1_UNIT_MODE);
    fn GetUnitMode(&self) -> D2D1_UNIT_MODE;
    fn DrawGlyphRun(&self, baselineorigin: &windows_numerics::Vector2, glyphrun: *const super::dwrite::DWRITE_GLYPH_RUN, glyphrundescription: *const super::dwrite::DWRITE_GLYPH_RUN_DESCRIPTION, foregroundbrush: windows_core::Ref<super::d2d1::ID2D1Brush>, measuringmode: super::dcommon::DWRITE_MEASURING_MODE);
    fn DrawImage(&self, image: windows_core::Ref<super::d2d1::ID2D1Image>, targetoffset: *const windows_numerics::Vector2, imagerectangle: *const super::dcommon::D2D_RECT_F, interpolationmode: D2D1_INTERPOLATION_MODE, compositemode: D2D1_COMPOSITE_MODE);
    fn DrawGdiMetafile(&self, gdimetafile: windows_core::Ref<ID2D1GdiMetafile>, targetoffset: *const windows_numerics::Vector2);
    fn DrawBitmap(&self, bitmap: windows_core::Ref<super::d2d1::ID2D1Bitmap>, destinationrectangle: *const super::dcommon::D2D_RECT_F, opacity: f32, interpolationmode: D2D1_INTERPOLATION_MODE, sourcerectangle: *const super::dcommon::D2D_RECT_F, perspectivetransform: *const windows_numerics::Matrix4x4);
    fn PushLayer(&self, layerparameters: *const D2D1_LAYER_PARAMETERS1, layer: windows_core::Ref<super::d2d1::ID2D1Layer>);
    fn InvalidateEffectInputRectangle(&self, effect: windows_core::Ref<ID2D1Effect>, input: u32, inputrectangle: *const super::dcommon::D2D_RECT_F) -> windows_core::Result<()>;
    fn GetEffectInvalidRectangleCount(&self, effect: windows_core::Ref<ID2D1Effect>) -> windows_core::Result<u32>;
    fn GetEffectInvalidRectangles(&self, effect: windows_core::Ref<ID2D1Effect>, rectangles: *mut super::dcommon::D2D_RECT_F, rectanglescount: u32) -> windows_core::Result<()>;
    fn GetEffectRequiredInputRectangles(&self, rendereffect: windows_core::Ref<ID2D1Effect>, renderimagerectangle: *const super::dcommon::D2D_RECT_F, inputdescriptions: *const D2D1_EFFECT_INPUT_DESCRIPTION, requiredinputrects: *mut super::dcommon::D2D_RECT_F, inputcount: u32) -> windows_core::Result<()>;
    fn FillOpacityMask(&self, opacitymask: windows_core::Ref<super::d2d1::ID2D1Bitmap>, brush: windows_core::Ref<super::d2d1::ID2D1Brush>, destinationrectangle: *const super::dcommon::D2D_RECT_F, sourcerectangle: *const super::dcommon::D2D_RECT_F);
}
#[cfg(all(feature = "Win32_d2d1", feature = "Win32_d2dbasetypes", feature = "Win32_dcommon", feature = "Win32_dwrite", feature = "Win32_dxgi", feature = "Win32_dxgiformat", feature = "Win32_dxgitype", feature = "Win32_wincodec"))]
impl ID2D1DeviceContext_Vtbl {
    pub const fn new<Identity: ID2D1DeviceContext_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn CreateBitmap<Identity: ID2D1DeviceContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, size: super::dcommon::D2D_SIZE_U, sourcedata: *const core::ffi::c_void, pitch: u32, bitmapproperties: *const D2D1_BITMAP_PROPERTIES1, bitmap: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ID2D1DeviceContext_Impl::CreateBitmap(this, core::mem::transmute(&size), core::mem::transmute_copy(&sourcedata), core::mem::transmute_copy(&pitch), core::mem::transmute_copy(&bitmapproperties)) {
                    Ok(ok__) => {
                        bitmap.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CreateBitmapFromWicBitmap<Identity: ID2D1DeviceContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, wicbitmapsource: *mut core::ffi::c_void, bitmapproperties: *const D2D1_BITMAP_PROPERTIES1, bitmap: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ID2D1DeviceContext_Impl::CreateBitmapFromWicBitmap(this, core::mem::transmute_copy(&wicbitmapsource), core::mem::transmute_copy(&bitmapproperties)) {
                    Ok(ok__) => {
                        bitmap.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CreateColorContext<Identity: ID2D1DeviceContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, space: D2D1_COLOR_SPACE, profile: *const u8, profilesize: u32, colorcontext: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ID2D1DeviceContext_Impl::CreateColorContext(this, core::mem::transmute_copy(&space), core::mem::transmute_copy(&profile), core::mem::transmute_copy(&profilesize)) {
                    Ok(ok__) => {
                        colorcontext.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CreateColorContextFromFilename<Identity: ID2D1DeviceContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, filename: windows_core::PCWSTR, colorcontext: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ID2D1DeviceContext_Impl::CreateColorContextFromFilename(this, core::mem::transmute(&filename)) {
                    Ok(ok__) => {
                        colorcontext.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CreateColorContextFromWicColorContext<Identity: ID2D1DeviceContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, wiccolorcontext: *mut core::ffi::c_void, colorcontext: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ID2D1DeviceContext_Impl::CreateColorContextFromWicColorContext(this, core::mem::transmute_copy(&wiccolorcontext)) {
                    Ok(ok__) => {
                        colorcontext.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CreateBitmapFromDxgiSurface<Identity: ID2D1DeviceContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, surface: *mut core::ffi::c_void, bitmapproperties: *const D2D1_BITMAP_PROPERTIES1, bitmap: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ID2D1DeviceContext_Impl::CreateBitmapFromDxgiSurface(this, core::mem::transmute_copy(&surface), core::mem::transmute_copy(&bitmapproperties)) {
                    Ok(ok__) => {
                        bitmap.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CreateEffect<Identity: ID2D1DeviceContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, effectid: *const windows_core::GUID, effect: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ID2D1DeviceContext_Impl::CreateEffect(this, core::mem::transmute_copy(&effectid)) {
                    Ok(ok__) => {
                        effect.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CreateGradientStopCollection<Identity: ID2D1DeviceContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, straightalphagradientstops: *const super::d2d1::D2D1_GRADIENT_STOP, straightalphagradientstopscount: u32, preinterpolationspace: D2D1_COLOR_SPACE, postinterpolationspace: D2D1_COLOR_SPACE, bufferprecision: D2D1_BUFFER_PRECISION, extendmode: super::d2d1::D2D1_EXTEND_MODE, colorinterpolationmode: D2D1_COLOR_INTERPOLATION_MODE, gradientstopcollection1: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ID2D1DeviceContext_Impl::CreateGradientStopCollection(this, core::mem::transmute_copy(&straightalphagradientstops), core::mem::transmute_copy(&straightalphagradientstopscount), core::mem::transmute_copy(&preinterpolationspace), core::mem::transmute_copy(&postinterpolationspace), core::mem::transmute_copy(&bufferprecision), core::mem::transmute_copy(&extendmode), core::mem::transmute_copy(&colorinterpolationmode)) {
                    Ok(ok__) => {
                        gradientstopcollection1.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CreateImageBrush<Identity: ID2D1DeviceContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, image: *mut core::ffi::c_void, imagebrushproperties: *const D2D1_IMAGE_BRUSH_PROPERTIES, brushproperties: *const super::d2d1::D2D1_BRUSH_PROPERTIES, imagebrush: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ID2D1DeviceContext_Impl::CreateImageBrush(this, core::mem::transmute_copy(&image), core::mem::transmute_copy(&imagebrushproperties), core::mem::transmute_copy(&brushproperties)) {
                    Ok(ok__) => {
                        imagebrush.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CreateBitmapBrush<Identity: ID2D1DeviceContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bitmap: *mut core::ffi::c_void, bitmapbrushproperties: *const D2D1_BITMAP_BRUSH_PROPERTIES1, brushproperties: *const super::d2d1::D2D1_BRUSH_PROPERTIES, bitmapbrush: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ID2D1DeviceContext_Impl::CreateBitmapBrush(this, core::mem::transmute_copy(&bitmap), core::mem::transmute_copy(&bitmapbrushproperties), core::mem::transmute_copy(&brushproperties)) {
                    Ok(ok__) => {
                        bitmapbrush.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CreateCommandList<Identity: ID2D1DeviceContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, commandlist: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ID2D1DeviceContext_Impl::CreateCommandList(this) {
                    Ok(ok__) => {
                        commandlist.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn IsDxgiFormatSupported<Identity: ID2D1DeviceContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, format: super::dxgiformat::DXGI_FORMAT) -> windows_core::BOOL {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID2D1DeviceContext_Impl::IsDxgiFormatSupported(this, core::mem::transmute_copy(&format))
            }
        }
        unsafe extern "system" fn IsBufferPrecisionSupported<Identity: ID2D1DeviceContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bufferprecision: D2D1_BUFFER_PRECISION) -> windows_core::BOOL {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID2D1DeviceContext_Impl::IsBufferPrecisionSupported(this, core::mem::transmute_copy(&bufferprecision))
            }
        }
        unsafe extern "system" fn GetImageLocalBounds<Identity: ID2D1DeviceContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, image: *mut core::ffi::c_void, localbounds: *mut super::dcommon::D2D_RECT_F) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ID2D1DeviceContext_Impl::GetImageLocalBounds(this, core::mem::transmute_copy(&image)) {
                    Ok(ok__) => {
                        localbounds.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetImageWorldBounds<Identity: ID2D1DeviceContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, image: *mut core::ffi::c_void, worldbounds: *mut super::dcommon::D2D_RECT_F) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ID2D1DeviceContext_Impl::GetImageWorldBounds(this, core::mem::transmute_copy(&image)) {
                    Ok(ok__) => {
                        worldbounds.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetGlyphRunWorldBounds<Identity: ID2D1DeviceContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, baselineorigin: windows_numerics::Vector2, glyphrun: *const super::dwrite::DWRITE_GLYPH_RUN, measuringmode: super::dcommon::DWRITE_MEASURING_MODE, bounds: *mut super::dcommon::D2D_RECT_F) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ID2D1DeviceContext_Impl::GetGlyphRunWorldBounds(this, core::mem::transmute(&baselineorigin), core::mem::transmute_copy(&glyphrun), core::mem::transmute_copy(&measuringmode)) {
                    Ok(ok__) => {
                        bounds.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetDevice<Identity: ID2D1DeviceContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, device: *mut *mut core::ffi::c_void) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID2D1DeviceContext_Impl::GetDevice(this, core::mem::transmute_copy(&device));
            }
        }
        unsafe extern "system" fn SetTarget<Identity: ID2D1DeviceContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, image: *mut core::ffi::c_void) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID2D1DeviceContext_Impl::SetTarget(this, core::mem::transmute_copy(&image));
            }
        }
        unsafe extern "system" fn GetTarget<Identity: ID2D1DeviceContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, image: *mut *mut core::ffi::c_void) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID2D1DeviceContext_Impl::GetTarget(this, core::mem::transmute_copy(&image));
            }
        }
        unsafe extern "system" fn SetRenderingControls<Identity: ID2D1DeviceContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, renderingcontrols: *const D2D1_RENDERING_CONTROLS) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID2D1DeviceContext_Impl::SetRenderingControls(this, core::mem::transmute_copy(&renderingcontrols));
            }
        }
        unsafe extern "system" fn GetRenderingControls<Identity: ID2D1DeviceContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, renderingcontrols: *mut D2D1_RENDERING_CONTROLS) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID2D1DeviceContext_Impl::GetRenderingControls(this, core::mem::transmute_copy(&renderingcontrols));
            }
        }
        unsafe extern "system" fn SetPrimitiveBlend<Identity: ID2D1DeviceContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, primitiveblend: D2D1_PRIMITIVE_BLEND) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID2D1DeviceContext_Impl::SetPrimitiveBlend(this, core::mem::transmute_copy(&primitiveblend));
            }
        }
        unsafe extern "system" fn GetPrimitiveBlend<Identity: ID2D1DeviceContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> D2D1_PRIMITIVE_BLEND {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID2D1DeviceContext_Impl::GetPrimitiveBlend(this)
            }
        }
        unsafe extern "system" fn SetUnitMode<Identity: ID2D1DeviceContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, unitmode: D2D1_UNIT_MODE) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID2D1DeviceContext_Impl::SetUnitMode(this, core::mem::transmute_copy(&unitmode));
            }
        }
        unsafe extern "system" fn GetUnitMode<Identity: ID2D1DeviceContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> D2D1_UNIT_MODE {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID2D1DeviceContext_Impl::GetUnitMode(this)
            }
        }
        unsafe extern "system" fn DrawGlyphRun<Identity: ID2D1DeviceContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, baselineorigin: windows_numerics::Vector2, glyphrun: *const super::dwrite::DWRITE_GLYPH_RUN, glyphrundescription: *const super::dwrite::DWRITE_GLYPH_RUN_DESCRIPTION, foregroundbrush: *mut core::ffi::c_void, measuringmode: super::dcommon::DWRITE_MEASURING_MODE) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID2D1DeviceContext_Impl::DrawGlyphRun(this, core::mem::transmute(&baselineorigin), core::mem::transmute_copy(&glyphrun), core::mem::transmute_copy(&glyphrundescription), core::mem::transmute_copy(&foregroundbrush), core::mem::transmute_copy(&measuringmode));
            }
        }
        unsafe extern "system" fn DrawImage<Identity: ID2D1DeviceContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, image: *mut core::ffi::c_void, targetoffset: *const windows_numerics::Vector2, imagerectangle: *const super::dcommon::D2D_RECT_F, interpolationmode: D2D1_INTERPOLATION_MODE, compositemode: D2D1_COMPOSITE_MODE) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID2D1DeviceContext_Impl::DrawImage(this, core::mem::transmute_copy(&image), core::mem::transmute_copy(&targetoffset), core::mem::transmute_copy(&imagerectangle), core::mem::transmute_copy(&interpolationmode), core::mem::transmute_copy(&compositemode));
            }
        }
        unsafe extern "system" fn DrawGdiMetafile<Identity: ID2D1DeviceContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, gdimetafile: *mut core::ffi::c_void, targetoffset: *const windows_numerics::Vector2) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID2D1DeviceContext_Impl::DrawGdiMetafile(this, core::mem::transmute_copy(&gdimetafile), core::mem::transmute_copy(&targetoffset));
            }
        }
        unsafe extern "system" fn DrawBitmap<Identity: ID2D1DeviceContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bitmap: *mut core::ffi::c_void, destinationrectangle: *const super::dcommon::D2D_RECT_F, opacity: f32, interpolationmode: D2D1_INTERPOLATION_MODE, sourcerectangle: *const super::dcommon::D2D_RECT_F, perspectivetransform: *const windows_numerics::Matrix4x4) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID2D1DeviceContext_Impl::DrawBitmap(this, core::mem::transmute_copy(&bitmap), core::mem::transmute_copy(&destinationrectangle), core::mem::transmute_copy(&opacity), core::mem::transmute_copy(&interpolationmode), core::mem::transmute_copy(&sourcerectangle), core::mem::transmute_copy(&perspectivetransform));
            }
        }
        unsafe extern "system" fn PushLayer<Identity: ID2D1DeviceContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, layerparameters: *const D2D1_LAYER_PARAMETERS1, layer: *mut core::ffi::c_void) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID2D1DeviceContext_Impl::PushLayer(this, core::mem::transmute_copy(&layerparameters), core::mem::transmute_copy(&layer));
            }
        }
        unsafe extern "system" fn InvalidateEffectInputRectangle<Identity: ID2D1DeviceContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, effect: *mut core::ffi::c_void, input: u32, inputrectangle: *const super::dcommon::D2D_RECT_F) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID2D1DeviceContext_Impl::InvalidateEffectInputRectangle(this, core::mem::transmute_copy(&effect), core::mem::transmute_copy(&input), core::mem::transmute_copy(&inputrectangle)).into()
            }
        }
        unsafe extern "system" fn GetEffectInvalidRectangleCount<Identity: ID2D1DeviceContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, effect: *mut core::ffi::c_void, rectanglecount: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ID2D1DeviceContext_Impl::GetEffectInvalidRectangleCount(this, core::mem::transmute_copy(&effect)) {
                    Ok(ok__) => {
                        rectanglecount.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetEffectInvalidRectangles<Identity: ID2D1DeviceContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, effect: *mut core::ffi::c_void, rectangles: *mut super::dcommon::D2D_RECT_F, rectanglescount: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID2D1DeviceContext_Impl::GetEffectInvalidRectangles(this, core::mem::transmute_copy(&effect), core::mem::transmute_copy(&rectangles), core::mem::transmute_copy(&rectanglescount)).into()
            }
        }
        unsafe extern "system" fn GetEffectRequiredInputRectangles<Identity: ID2D1DeviceContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, rendereffect: *mut core::ffi::c_void, renderimagerectangle: *const super::dcommon::D2D_RECT_F, inputdescriptions: *const D2D1_EFFECT_INPUT_DESCRIPTION, requiredinputrects: *mut super::dcommon::D2D_RECT_F, inputcount: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID2D1DeviceContext_Impl::GetEffectRequiredInputRectangles(this, core::mem::transmute_copy(&rendereffect), core::mem::transmute_copy(&renderimagerectangle), core::mem::transmute_copy(&inputdescriptions), core::mem::transmute_copy(&requiredinputrects), core::mem::transmute_copy(&inputcount)).into()
            }
        }
        unsafe extern "system" fn FillOpacityMask<Identity: ID2D1DeviceContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, opacitymask: *mut core::ffi::c_void, brush: *mut core::ffi::c_void, destinationrectangle: *const super::dcommon::D2D_RECT_F, sourcerectangle: *const super::dcommon::D2D_RECT_F) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID2D1DeviceContext_Impl::FillOpacityMask(this, core::mem::transmute_copy(&opacitymask), core::mem::transmute_copy(&brush), core::mem::transmute_copy(&destinationrectangle), core::mem::transmute_copy(&sourcerectangle));
            }
        }
        Self {
            base__: super::d2d1::ID2D1RenderTarget_Vtbl::new::<Identity, OFFSET>(),
            CreateBitmap: CreateBitmap::<Identity, OFFSET>,
            CreateBitmapFromWicBitmap: CreateBitmapFromWicBitmap::<Identity, OFFSET>,
            CreateColorContext: CreateColorContext::<Identity, OFFSET>,
            CreateColorContextFromFilename: CreateColorContextFromFilename::<Identity, OFFSET>,
            CreateColorContextFromWicColorContext: CreateColorContextFromWicColorContext::<Identity, OFFSET>,
            CreateBitmapFromDxgiSurface: CreateBitmapFromDxgiSurface::<Identity, OFFSET>,
            CreateEffect: CreateEffect::<Identity, OFFSET>,
            CreateGradientStopCollection: CreateGradientStopCollection::<Identity, OFFSET>,
            CreateImageBrush: CreateImageBrush::<Identity, OFFSET>,
            CreateBitmapBrush: CreateBitmapBrush::<Identity, OFFSET>,
            CreateCommandList: CreateCommandList::<Identity, OFFSET>,
            IsDxgiFormatSupported: IsDxgiFormatSupported::<Identity, OFFSET>,
            IsBufferPrecisionSupported: IsBufferPrecisionSupported::<Identity, OFFSET>,
            GetImageLocalBounds: GetImageLocalBounds::<Identity, OFFSET>,
            GetImageWorldBounds: GetImageWorldBounds::<Identity, OFFSET>,
            GetGlyphRunWorldBounds: GetGlyphRunWorldBounds::<Identity, OFFSET>,
            GetDevice: GetDevice::<Identity, OFFSET>,
            SetTarget: SetTarget::<Identity, OFFSET>,
            GetTarget: GetTarget::<Identity, OFFSET>,
            SetRenderingControls: SetRenderingControls::<Identity, OFFSET>,
            GetRenderingControls: GetRenderingControls::<Identity, OFFSET>,
            SetPrimitiveBlend: SetPrimitiveBlend::<Identity, OFFSET>,
            GetPrimitiveBlend: GetPrimitiveBlend::<Identity, OFFSET>,
            SetUnitMode: SetUnitMode::<Identity, OFFSET>,
            GetUnitMode: GetUnitMode::<Identity, OFFSET>,
            DrawGlyphRun: DrawGlyphRun::<Identity, OFFSET>,
            DrawImage: DrawImage::<Identity, OFFSET>,
            DrawGdiMetafile: DrawGdiMetafile::<Identity, OFFSET>,
            DrawBitmap: DrawBitmap::<Identity, OFFSET>,
            PushLayer: PushLayer::<Identity, OFFSET>,
            InvalidateEffectInputRectangle: InvalidateEffectInputRectangle::<Identity, OFFSET>,
            GetEffectInvalidRectangleCount: GetEffectInvalidRectangleCount::<Identity, OFFSET>,
            GetEffectInvalidRectangles: GetEffectInvalidRectangles::<Identity, OFFSET>,
            GetEffectRequiredInputRectangles: GetEffectRequiredInputRectangles::<Identity, OFFSET>,
            FillOpacityMask: FillOpacityMask::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ID2D1DeviceContext as windows_core::Interface>::IID || iid == &<super::d2d1::ID2D1Resource as windows_core::Interface>::IID || iid == &<super::d2d1::ID2D1RenderTarget as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_d2d1", feature = "Win32_d2dbasetypes", feature = "Win32_dcommon", feature = "Win32_dwrite", feature = "Win32_dxgi", feature = "Win32_dxgiformat", feature = "Win32_dxgitype", feature = "Win32_wincodec"))]
impl windows_core::RuntimeName for ID2D1DeviceContext {}
#[cfg(feature = "Win32_d2d1")]
windows_core::imp::define_interface!(ID2D1DrawingStateBlock1, ID2D1DrawingStateBlock1_Vtbl, 0x689f1f85_c72e_4e33_8f19_85754efd5ace);
#[cfg(feature = "Win32_d2d1")]
impl core::ops::Deref for ID2D1DrawingStateBlock1 {
    type Target = super::d2d1::ID2D1DrawingStateBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_d2d1")]
windows_core::imp::interface_hierarchy!(ID2D1DrawingStateBlock1, windows_core::IUnknown, super::d2d1::ID2D1Resource, super::d2d1::ID2D1DrawingStateBlock);
#[cfg(feature = "Win32_d2d1")]
impl ID2D1DrawingStateBlock1 {
    pub unsafe fn GetDescription(&self, statedescription: *mut D2D1_DRAWING_STATE_DESCRIPTION1) {
        unsafe {
            (windows_core::Interface::vtable(self).GetDescription)(windows_core::Interface::as_raw(self), statedescription as _);
        }
    }
    pub unsafe fn SetDescription(&self, statedescription: *const D2D1_DRAWING_STATE_DESCRIPTION1) {
        unsafe {
            (windows_core::Interface::vtable(self).SetDescription)(windows_core::Interface::as_raw(self), statedescription);
        }
    }
}
#[cfg(feature = "Win32_d2d1")]
#[repr(C)]
#[doc(hidden)]
pub struct ID2D1DrawingStateBlock1_Vtbl {
    pub base__: super::d2d1::ID2D1DrawingStateBlock_Vtbl,
    pub GetDescription: unsafe extern "system" fn(*mut core::ffi::c_void, *mut D2D1_DRAWING_STATE_DESCRIPTION1),
    pub SetDescription: unsafe extern "system" fn(*mut core::ffi::c_void, *const D2D1_DRAWING_STATE_DESCRIPTION1),
}
#[cfg(all(feature = "Win32_d2d1", feature = "Win32_dwrite"))]
pub trait ID2D1DrawingStateBlock1_Impl: super::d2d1::ID2D1DrawingStateBlock_Impl {
    fn GetDescription(&self, statedescription: *mut D2D1_DRAWING_STATE_DESCRIPTION1);
    fn SetDescription(&self, statedescription: *const D2D1_DRAWING_STATE_DESCRIPTION1);
}
#[cfg(all(feature = "Win32_d2d1", feature = "Win32_dwrite"))]
impl ID2D1DrawingStateBlock1_Vtbl {
    pub const fn new<Identity: ID2D1DrawingStateBlock1_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetDescription<Identity: ID2D1DrawingStateBlock1_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, statedescription: *mut D2D1_DRAWING_STATE_DESCRIPTION1) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID2D1DrawingStateBlock1_Impl::GetDescription(this, core::mem::transmute_copy(&statedescription));
            }
        }
        unsafe extern "system" fn SetDescription<Identity: ID2D1DrawingStateBlock1_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, statedescription: *const D2D1_DRAWING_STATE_DESCRIPTION1) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID2D1DrawingStateBlock1_Impl::SetDescription(this, core::mem::transmute_copy(&statedescription));
            }
        }
        Self {
            base__: super::d2d1::ID2D1DrawingStateBlock_Vtbl::new::<Identity, OFFSET>(),
            GetDescription: GetDescription::<Identity, OFFSET>,
            SetDescription: SetDescription::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ID2D1DrawingStateBlock1 as windows_core::Interface>::IID || iid == &<super::d2d1::ID2D1Resource as windows_core::Interface>::IID || iid == &<super::d2d1::ID2D1DrawingStateBlock as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_d2d1", feature = "Win32_dwrite"))]
impl windows_core::RuntimeName for ID2D1DrawingStateBlock1 {}
windows_core::imp::define_interface!(ID2D1Effect, ID2D1Effect_Vtbl, 0x28211a43_7d89_476f_8181_2d6159b220ad);
impl core::ops::Deref for ID2D1Effect {
    type Target = ID2D1Properties;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ID2D1Effect, windows_core::IUnknown, ID2D1Properties);
impl ID2D1Effect {
    #[cfg(feature = "Win32_d2d1")]
    pub unsafe fn SetInput<P1>(&self, index: u32, input: P1, invalidate: bool)
    where
        P1: windows_core::Param<super::d2d1::ID2D1Image>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).SetInput)(windows_core::Interface::as_raw(self), index, input.param().abi(), invalidate.into());
        }
    }
    pub unsafe fn SetInputCount(&self, inputcount: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetInputCount)(windows_core::Interface::as_raw(self), inputcount) }
    }
    #[cfg(feature = "Win32_d2d1")]
    pub unsafe fn GetInput(&self, index: u32) -> windows_core::Result<super::d2d1::ID2D1Image> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetInput)(windows_core::Interface::as_raw(self), index, &mut result__);
            windows_core::Type::from_abi(result__)
        }
    }
    pub unsafe fn GetInputCount(&self) -> u32 {
        unsafe { (windows_core::Interface::vtable(self).GetInputCount)(windows_core::Interface::as_raw(self)) }
    }
    #[cfg(feature = "Win32_d2d1")]
    pub unsafe fn GetOutput(&self) -> windows_core::Result<super::d2d1::ID2D1Image> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetOutput)(windows_core::Interface::as_raw(self), &mut result__);
            windows_core::Type::from_abi(result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ID2D1Effect_Vtbl {
    pub base__: ID2D1Properties_Vtbl,
    #[cfg(feature = "Win32_d2d1")]
    pub SetInput: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut core::ffi::c_void, windows_core::BOOL),
    #[cfg(not(feature = "Win32_d2d1"))]
    SetInput: usize,
    pub SetInputCount: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_d2d1")]
    pub GetInput: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void),
    #[cfg(not(feature = "Win32_d2d1"))]
    GetInput: usize,
    pub GetInputCount: unsafe extern "system" fn(*mut core::ffi::c_void) -> u32,
    #[cfg(feature = "Win32_d2d1")]
    pub GetOutput: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void),
    #[cfg(not(feature = "Win32_d2d1"))]
    GetOutput: usize,
}
#[cfg(feature = "Win32_d2d1")]
pub trait ID2D1Effect_Impl: ID2D1Properties_Impl {
    fn SetInput(&self, index: u32, input: windows_core::Ref<super::d2d1::ID2D1Image>, invalidate: windows_core::BOOL);
    fn SetInputCount(&self, inputcount: u32) -> windows_core::Result<()>;
    fn GetInput(&self, index: u32, input: windows_core::OutRef<super::d2d1::ID2D1Image>);
    fn GetInputCount(&self) -> u32;
    fn GetOutput(&self, outputimage: windows_core::OutRef<super::d2d1::ID2D1Image>);
}
#[cfg(feature = "Win32_d2d1")]
impl ID2D1Effect_Vtbl {
    pub const fn new<Identity: ID2D1Effect_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn SetInput<Identity: ID2D1Effect_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: u32, input: *mut core::ffi::c_void, invalidate: windows_core::BOOL) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID2D1Effect_Impl::SetInput(this, core::mem::transmute_copy(&index), core::mem::transmute_copy(&input), core::mem::transmute_copy(&invalidate));
            }
        }
        unsafe extern "system" fn SetInputCount<Identity: ID2D1Effect_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, inputcount: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID2D1Effect_Impl::SetInputCount(this, core::mem::transmute_copy(&inputcount)).into()
            }
        }
        unsafe extern "system" fn GetInput<Identity: ID2D1Effect_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: u32, input: *mut *mut core::ffi::c_void) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID2D1Effect_Impl::GetInput(this, core::mem::transmute_copy(&index), core::mem::transmute_copy(&input));
            }
        }
        unsafe extern "system" fn GetInputCount<Identity: ID2D1Effect_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> u32 {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID2D1Effect_Impl::GetInputCount(this)
            }
        }
        unsafe extern "system" fn GetOutput<Identity: ID2D1Effect_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, outputimage: *mut *mut core::ffi::c_void) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID2D1Effect_Impl::GetOutput(this, core::mem::transmute_copy(&outputimage));
            }
        }
        Self {
            base__: ID2D1Properties_Vtbl::new::<Identity, OFFSET>(),
            SetInput: SetInput::<Identity, OFFSET>,
            SetInputCount: SetInputCount::<Identity, OFFSET>,
            GetInput: GetInput::<Identity, OFFSET>,
            GetInputCount: GetInputCount::<Identity, OFFSET>,
            GetOutput: GetOutput::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ID2D1Effect as windows_core::Interface>::IID || iid == &<ID2D1Properties as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_d2d1")]
impl windows_core::RuntimeName for ID2D1Effect {}
#[cfg(feature = "Win32_d2d1")]
windows_core::imp::define_interface!(ID2D1Factory1, ID2D1Factory1_Vtbl, 0xbb12d362_daee_4b9a_aa1d_14ba401cfa1f);
#[cfg(feature = "Win32_d2d1")]
impl core::ops::Deref for ID2D1Factory1 {
    type Target = super::d2d1::ID2D1Factory;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_d2d1")]
windows_core::imp::interface_hierarchy!(ID2D1Factory1, windows_core::IUnknown, super::d2d1::ID2D1Factory);
#[cfg(feature = "Win32_d2d1")]
impl ID2D1Factory1 {
    #[cfg(feature = "Win32_dxgi")]
    pub unsafe fn CreateDevice<P0>(&self, dxgidevice: P0) -> windows_core::Result<ID2D1Device>
    where
        P0: windows_core::Param<super::dxgi::IDXGIDevice>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateDevice)(windows_core::Interface::as_raw(self), dxgidevice.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn CreateStrokeStyle(&self, strokestyleproperties: *const D2D1_STROKE_STYLE_PROPERTIES1, dashes: Option<&[f32]>) -> windows_core::Result<ID2D1StrokeStyle1> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateStrokeStyle)(windows_core::Interface::as_raw(self), strokestyleproperties, core::mem::transmute(dashes.map_or(core::ptr::null(), |slice| slice.as_ptr())), dashes.map_or(0, |slice| slice.len().try_into().unwrap()), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn CreatePathGeometry(&self) -> windows_core::Result<ID2D1PathGeometry1> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreatePathGeometry)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Win32_dwrite")]
    pub unsafe fn CreateDrawingStateBlock<P1>(&self, drawingstatedescription: Option<*const D2D1_DRAWING_STATE_DESCRIPTION1>, textrenderingparams: P1) -> windows_core::Result<ID2D1DrawingStateBlock1>
    where
        P1: windows_core::Param<super::dwrite::IDWriteRenderingParams>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateDrawingStateBlock)(windows_core::Interface::as_raw(self), drawingstatedescription.unwrap_or(core::mem::zeroed()) as _, textrenderingparams.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Win32_objidlbase")]
    pub unsafe fn CreateGdiMetafile<P0>(&self, metafilestream: P0) -> windows_core::Result<ID2D1GdiMetafile>
    where
        P0: windows_core::Param<super::objidlbase::IStream>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateGdiMetafile)(windows_core::Interface::as_raw(self), metafilestream.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Win32_objidlbase")]
    pub unsafe fn RegisterEffectFromStream<P1>(&self, classid: *const windows_core::GUID, propertyxml: P1, bindings: Option<&[D2D1_PROPERTY_BINDING]>, effectfactory: PD2D1_EFFECT_FACTORY) -> windows_core::HRESULT
    where
        P1: windows_core::Param<super::objidlbase::IStream>,
    {
        unsafe { (windows_core::Interface::vtable(self).RegisterEffectFromStream)(windows_core::Interface::as_raw(self), classid, propertyxml.param().abi(), core::mem::transmute(bindings.map_or(core::ptr::null(), |slice| slice.as_ptr())), bindings.map_or(0, |slice| slice.len().try_into().unwrap()), effectfactory) }
    }
    pub unsafe fn RegisterEffectFromString<P1>(&self, classid: *const windows_core::GUID, propertyxml: P1, bindings: Option<&[D2D1_PROPERTY_BINDING]>, effectfactory: PD2D1_EFFECT_FACTORY) -> windows_core::HRESULT
    where
        P1: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).RegisterEffectFromString)(windows_core::Interface::as_raw(self), classid, propertyxml.param().abi(), core::mem::transmute(bindings.map_or(core::ptr::null(), |slice| slice.as_ptr())), bindings.map_or(0, |slice| slice.len().try_into().unwrap()), effectfactory) }
    }
    pub unsafe fn UnregisterEffect(&self, classid: *const windows_core::GUID) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).UnregisterEffect)(windows_core::Interface::as_raw(self), classid) }
    }
    pub unsafe fn GetRegisteredEffects(&self, effects: Option<&mut [windows_core::GUID]>, effectsreturned: Option<*mut u32>, effectsregistered: Option<*mut u32>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetRegisteredEffects)(windows_core::Interface::as_raw(self), core::mem::transmute(effects.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), effects.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), effectsreturned.unwrap_or(core::mem::zeroed()) as _, effectsregistered.unwrap_or(core::mem::zeroed()) as _) }
    }
    pub unsafe fn GetEffectProperties(&self, effectid: *const windows_core::GUID) -> windows_core::Result<ID2D1Properties> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetEffectProperties)(windows_core::Interface::as_raw(self), effectid, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[cfg(feature = "Win32_d2d1")]
#[repr(C)]
#[doc(hidden)]
pub struct ID2D1Factory1_Vtbl {
    pub base__: super::d2d1::ID2D1Factory_Vtbl,
    #[cfg(feature = "Win32_dxgi")]
    pub CreateDevice: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_dxgi"))]
    CreateDevice: usize,
    pub CreateStrokeStyle: unsafe extern "system" fn(*mut core::ffi::c_void, *const D2D1_STROKE_STYLE_PROPERTIES1, *const f32, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreatePathGeometry: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_dwrite")]
    pub CreateDrawingStateBlock: unsafe extern "system" fn(*mut core::ffi::c_void, *const D2D1_DRAWING_STATE_DESCRIPTION1, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_dwrite"))]
    CreateDrawingStateBlock: usize,
    #[cfg(feature = "Win32_objidlbase")]
    pub CreateGdiMetafile: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_objidlbase"))]
    CreateGdiMetafile: usize,
    #[cfg(feature = "Win32_objidlbase")]
    pub RegisterEffectFromStream: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *mut core::ffi::c_void, *const D2D1_PROPERTY_BINDING, u32, PD2D1_EFFECT_FACTORY) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_objidlbase"))]
    RegisterEffectFromStream: usize,
    pub RegisterEffectFromString: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, windows_core::PCWSTR, *const D2D1_PROPERTY_BINDING, u32, PD2D1_EFFECT_FACTORY) -> windows_core::HRESULT,
    pub UnregisterEffect: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID) -> windows_core::HRESULT,
    pub GetRegisteredEffects: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::GUID, u32, *mut u32, *mut u32) -> windows_core::HRESULT,
    pub GetEffectProperties: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_d2d1", feature = "Win32_dcommon", feature = "Win32_dwrite", feature = "Win32_dxgi", feature = "Win32_dxgiformat", feature = "Win32_objidlbase", feature = "Win32_wincodec", feature = "Win32_windef"))]
pub trait ID2D1Factory1_Impl: super::d2d1::ID2D1Factory_Impl {
    fn CreateDevice(&self, dxgidevice: windows_core::Ref<super::dxgi::IDXGIDevice>) -> windows_core::Result<ID2D1Device>;
    fn CreateStrokeStyle(&self, strokestyleproperties: *const D2D1_STROKE_STYLE_PROPERTIES1, dashes: *const f32, dashescount: u32) -> windows_core::Result<ID2D1StrokeStyle1>;
    fn CreatePathGeometry(&self) -> windows_core::Result<ID2D1PathGeometry1>;
    fn CreateDrawingStateBlock(&self, drawingstatedescription: *const D2D1_DRAWING_STATE_DESCRIPTION1, textrenderingparams: windows_core::Ref<super::dwrite::IDWriteRenderingParams>) -> windows_core::Result<ID2D1DrawingStateBlock1>;
    fn CreateGdiMetafile(&self, metafilestream: windows_core::Ref<super::objidlbase::IStream>) -> windows_core::Result<ID2D1GdiMetafile>;
    fn RegisterEffectFromStream(&self, classid: *const windows_core::GUID, propertyxml: windows_core::Ref<super::objidlbase::IStream>, bindings: *const D2D1_PROPERTY_BINDING, bindingscount: u32, effectfactory: PD2D1_EFFECT_FACTORY) -> windows_core::Result<()>;
    fn RegisterEffectFromString(&self, classid: *const windows_core::GUID, propertyxml: &windows_core::PCWSTR, bindings: *const D2D1_PROPERTY_BINDING, bindingscount: u32, effectfactory: PD2D1_EFFECT_FACTORY) -> windows_core::Result<()>;
    fn UnregisterEffect(&self, classid: *const windows_core::GUID) -> windows_core::Result<()>;
    fn GetRegisteredEffects(&self, effects: *mut windows_core::GUID, effectscount: u32, effectsreturned: *mut u32, effectsregistered: *mut u32) -> windows_core::Result<()>;
    fn GetEffectProperties(&self, effectid: *const windows_core::GUID) -> windows_core::Result<ID2D1Properties>;
}
#[cfg(all(feature = "Win32_d2d1", feature = "Win32_dcommon", feature = "Win32_dwrite", feature = "Win32_dxgi", feature = "Win32_dxgiformat", feature = "Win32_objidlbase", feature = "Win32_wincodec", feature = "Win32_windef"))]
impl ID2D1Factory1_Vtbl {
    pub const fn new<Identity: ID2D1Factory1_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn CreateDevice<Identity: ID2D1Factory1_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dxgidevice: *mut core::ffi::c_void, d2ddevice: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ID2D1Factory1_Impl::CreateDevice(this, core::mem::transmute_copy(&dxgidevice)) {
                    Ok(ok__) => {
                        d2ddevice.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CreateStrokeStyle<Identity: ID2D1Factory1_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strokestyleproperties: *const D2D1_STROKE_STYLE_PROPERTIES1, dashes: *const f32, dashescount: u32, strokestyle: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ID2D1Factory1_Impl::CreateStrokeStyle(this, core::mem::transmute_copy(&strokestyleproperties), core::mem::transmute_copy(&dashes), core::mem::transmute_copy(&dashescount)) {
                    Ok(ok__) => {
                        strokestyle.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CreatePathGeometry<Identity: ID2D1Factory1_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pathgeometry: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ID2D1Factory1_Impl::CreatePathGeometry(this) {
                    Ok(ok__) => {
                        pathgeometry.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CreateDrawingStateBlock<Identity: ID2D1Factory1_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, drawingstatedescription: *const D2D1_DRAWING_STATE_DESCRIPTION1, textrenderingparams: *mut core::ffi::c_void, drawingstateblock: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ID2D1Factory1_Impl::CreateDrawingStateBlock(this, core::mem::transmute_copy(&drawingstatedescription), core::mem::transmute_copy(&textrenderingparams)) {
                    Ok(ok__) => {
                        drawingstateblock.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CreateGdiMetafile<Identity: ID2D1Factory1_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, metafilestream: *mut core::ffi::c_void, metafile: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ID2D1Factory1_Impl::CreateGdiMetafile(this, core::mem::transmute_copy(&metafilestream)) {
                    Ok(ok__) => {
                        metafile.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn RegisterEffectFromStream<Identity: ID2D1Factory1_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, classid: *const windows_core::GUID, propertyxml: *mut core::ffi::c_void, bindings: *const D2D1_PROPERTY_BINDING, bindingscount: u32, effectfactory: PD2D1_EFFECT_FACTORY) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID2D1Factory1_Impl::RegisterEffectFromStream(this, core::mem::transmute_copy(&classid), core::mem::transmute_copy(&propertyxml), core::mem::transmute_copy(&bindings), core::mem::transmute_copy(&bindingscount), core::mem::transmute_copy(&effectfactory)).into()
            }
        }
        unsafe extern "system" fn RegisterEffectFromString<Identity: ID2D1Factory1_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, classid: *const windows_core::GUID, propertyxml: windows_core::PCWSTR, bindings: *const D2D1_PROPERTY_BINDING, bindingscount: u32, effectfactory: PD2D1_EFFECT_FACTORY) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID2D1Factory1_Impl::RegisterEffectFromString(this, core::mem::transmute_copy(&classid), core::mem::transmute(&propertyxml), core::mem::transmute_copy(&bindings), core::mem::transmute_copy(&bindingscount), core::mem::transmute_copy(&effectfactory)).into()
            }
        }
        unsafe extern "system" fn UnregisterEffect<Identity: ID2D1Factory1_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, classid: *const windows_core::GUID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID2D1Factory1_Impl::UnregisterEffect(this, core::mem::transmute_copy(&classid)).into()
            }
        }
        unsafe extern "system" fn GetRegisteredEffects<Identity: ID2D1Factory1_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, effects: *mut windows_core::GUID, effectscount: u32, effectsreturned: *mut u32, effectsregistered: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID2D1Factory1_Impl::GetRegisteredEffects(this, core::mem::transmute_copy(&effects), core::mem::transmute_copy(&effectscount), core::mem::transmute_copy(&effectsreturned), core::mem::transmute_copy(&effectsregistered)).into()
            }
        }
        unsafe extern "system" fn GetEffectProperties<Identity: ID2D1Factory1_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, effectid: *const windows_core::GUID, properties: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ID2D1Factory1_Impl::GetEffectProperties(this, core::mem::transmute_copy(&effectid)) {
                    Ok(ok__) => {
                        properties.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: super::d2d1::ID2D1Factory_Vtbl::new::<Identity, OFFSET>(),
            CreateDevice: CreateDevice::<Identity, OFFSET>,
            CreateStrokeStyle: CreateStrokeStyle::<Identity, OFFSET>,
            CreatePathGeometry: CreatePathGeometry::<Identity, OFFSET>,
            CreateDrawingStateBlock: CreateDrawingStateBlock::<Identity, OFFSET>,
            CreateGdiMetafile: CreateGdiMetafile::<Identity, OFFSET>,
            RegisterEffectFromStream: RegisterEffectFromStream::<Identity, OFFSET>,
            RegisterEffectFromString: RegisterEffectFromString::<Identity, OFFSET>,
            UnregisterEffect: UnregisterEffect::<Identity, OFFSET>,
            GetRegisteredEffects: GetRegisteredEffects::<Identity, OFFSET>,
            GetEffectProperties: GetEffectProperties::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ID2D1Factory1 as windows_core::Interface>::IID || iid == &<super::d2d1::ID2D1Factory as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_d2d1", feature = "Win32_dcommon", feature = "Win32_dwrite", feature = "Win32_dxgi", feature = "Win32_dxgiformat", feature = "Win32_objidlbase", feature = "Win32_wincodec", feature = "Win32_windef"))]
impl windows_core::RuntimeName for ID2D1Factory1 {}
#[cfg(feature = "Win32_d2d1")]
windows_core::imp::define_interface!(ID2D1GdiMetafile, ID2D1GdiMetafile_Vtbl, 0x2f543dc3_cfc1_4211_864f_cfd91c6f3395);
#[cfg(feature = "Win32_d2d1")]
impl core::ops::Deref for ID2D1GdiMetafile {
    type Target = super::d2d1::ID2D1Resource;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_d2d1")]
windows_core::imp::interface_hierarchy!(ID2D1GdiMetafile, windows_core::IUnknown, super::d2d1::ID2D1Resource);
#[cfg(feature = "Win32_d2d1")]
impl ID2D1GdiMetafile {
    pub unsafe fn Stream<P0>(&self, sink: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<ID2D1GdiMetafileSink>,
    {
        unsafe { (windows_core::Interface::vtable(self).Stream)(windows_core::Interface::as_raw(self), sink.param().abi()) }
    }
    #[cfg(feature = "Win32_dcommon")]
    pub unsafe fn GetBounds(&self) -> windows_core::Result<super::dcommon::D2D_RECT_F> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetBounds)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[cfg(feature = "Win32_d2d1")]
#[repr(C)]
#[doc(hidden)]
pub struct ID2D1GdiMetafile_Vtbl {
    pub base__: super::d2d1::ID2D1Resource_Vtbl,
    pub Stream: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_dcommon")]
    pub GetBounds: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::dcommon::D2D_RECT_F) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_dcommon"))]
    GetBounds: usize,
}
#[cfg(all(feature = "Win32_d2d1", feature = "Win32_dcommon"))]
pub trait ID2D1GdiMetafile_Impl: super::d2d1::ID2D1Resource_Impl {
    fn Stream(&self, sink: windows_core::Ref<ID2D1GdiMetafileSink>) -> windows_core::Result<()>;
    fn GetBounds(&self) -> windows_core::Result<super::dcommon::D2D_RECT_F>;
}
#[cfg(all(feature = "Win32_d2d1", feature = "Win32_dcommon"))]
impl ID2D1GdiMetafile_Vtbl {
    pub const fn new<Identity: ID2D1GdiMetafile_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Stream<Identity: ID2D1GdiMetafile_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, sink: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID2D1GdiMetafile_Impl::Stream(this, core::mem::transmute_copy(&sink)).into()
            }
        }
        unsafe extern "system" fn GetBounds<Identity: ID2D1GdiMetafile_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bounds: *mut super::dcommon::D2D_RECT_F) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ID2D1GdiMetafile_Impl::GetBounds(this) {
                    Ok(ok__) => {
                        bounds.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self { base__: super::d2d1::ID2D1Resource_Vtbl::new::<Identity, OFFSET>(), Stream: Stream::<Identity, OFFSET>, GetBounds: GetBounds::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ID2D1GdiMetafile as windows_core::Interface>::IID || iid == &<super::d2d1::ID2D1Resource as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_d2d1", feature = "Win32_dcommon"))]
impl windows_core::RuntimeName for ID2D1GdiMetafile {}
windows_core::imp::define_interface!(ID2D1GdiMetafileSink, ID2D1GdiMetafileSink_Vtbl, 0x82237326_8111_4f7c_bcf4_b5c1175564fe);
windows_core::imp::interface_hierarchy!(ID2D1GdiMetafileSink, windows_core::IUnknown);
impl ID2D1GdiMetafileSink {
    pub unsafe fn ProcessRecord(&self, recordtype: u32, recorddata: Option<*const core::ffi::c_void>, recorddatasize: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).ProcessRecord)(windows_core::Interface::as_raw(self), recordtype, recorddata.unwrap_or(core::mem::zeroed()) as _, recorddatasize) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ID2D1GdiMetafileSink_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub ProcessRecord: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const core::ffi::c_void, u32) -> windows_core::HRESULT,
}
pub trait ID2D1GdiMetafileSink_Impl: windows_core::IUnknownImpl {
    fn ProcessRecord(&self, recordtype: u32, recorddata: *const core::ffi::c_void, recorddatasize: u32) -> windows_core::Result<()>;
}
impl ID2D1GdiMetafileSink_Vtbl {
    pub const fn new<Identity: ID2D1GdiMetafileSink_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn ProcessRecord<Identity: ID2D1GdiMetafileSink_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, recordtype: u32, recorddata: *const core::ffi::c_void, recorddatasize: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID2D1GdiMetafileSink_Impl::ProcessRecord(this, core::mem::transmute_copy(&recordtype), core::mem::transmute_copy(&recorddata), core::mem::transmute_copy(&recorddatasize)).into()
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), ProcessRecord: ProcessRecord::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ID2D1GdiMetafileSink as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for ID2D1GdiMetafileSink {}
#[cfg(feature = "Win32_d2d1")]
windows_core::imp::define_interface!(ID2D1GradientStopCollection1, ID2D1GradientStopCollection1_Vtbl, 0xae1572f4_5dd0_4777_998b_9279472ae63b);
#[cfg(feature = "Win32_d2d1")]
impl core::ops::Deref for ID2D1GradientStopCollection1 {
    type Target = super::d2d1::ID2D1GradientStopCollection;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_d2d1")]
windows_core::imp::interface_hierarchy!(ID2D1GradientStopCollection1, windows_core::IUnknown, super::d2d1::ID2D1Resource, super::d2d1::ID2D1GradientStopCollection);
#[cfg(feature = "Win32_d2d1")]
impl ID2D1GradientStopCollection1 {
    #[cfg(all(feature = "Win32_d2dbasetypes", feature = "Win32_dxgitype"))]
    pub unsafe fn GetGradientStops1(&self, gradientstops: &mut [super::d2d1::D2D1_GRADIENT_STOP]) {
        unsafe {
            (windows_core::Interface::vtable(self).GetGradientStops1)(windows_core::Interface::as_raw(self), core::mem::transmute(gradientstops.as_ptr()), gradientstops.len().try_into().unwrap());
        }
    }
    pub unsafe fn GetPreInterpolationSpace(&self) -> D2D1_COLOR_SPACE {
        unsafe { (windows_core::Interface::vtable(self).GetPreInterpolationSpace)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn GetPostInterpolationSpace(&self) -> D2D1_COLOR_SPACE {
        unsafe { (windows_core::Interface::vtable(self).GetPostInterpolationSpace)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn GetBufferPrecision(&self) -> D2D1_BUFFER_PRECISION {
        unsafe { (windows_core::Interface::vtable(self).GetBufferPrecision)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn GetColorInterpolationMode(&self) -> D2D1_COLOR_INTERPOLATION_MODE {
        unsafe { (windows_core::Interface::vtable(self).GetColorInterpolationMode)(windows_core::Interface::as_raw(self)) }
    }
}
#[cfg(feature = "Win32_d2d1")]
#[repr(C)]
#[doc(hidden)]
pub struct ID2D1GradientStopCollection1_Vtbl {
    pub base__: super::d2d1::ID2D1GradientStopCollection_Vtbl,
    #[cfg(all(feature = "Win32_d2dbasetypes", feature = "Win32_dxgitype"))]
    pub GetGradientStops1: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::d2d1::D2D1_GRADIENT_STOP, u32),
    #[cfg(not(all(feature = "Win32_d2dbasetypes", feature = "Win32_dxgitype")))]
    GetGradientStops1: usize,
    pub GetPreInterpolationSpace: unsafe extern "system" fn(*mut core::ffi::c_void) -> D2D1_COLOR_SPACE,
    pub GetPostInterpolationSpace: unsafe extern "system" fn(*mut core::ffi::c_void) -> D2D1_COLOR_SPACE,
    pub GetBufferPrecision: unsafe extern "system" fn(*mut core::ffi::c_void) -> D2D1_BUFFER_PRECISION,
    pub GetColorInterpolationMode: unsafe extern "system" fn(*mut core::ffi::c_void) -> D2D1_COLOR_INTERPOLATION_MODE,
}
#[cfg(all(feature = "Win32_d2d1", feature = "Win32_d2dbasetypes", feature = "Win32_dxgitype"))]
pub trait ID2D1GradientStopCollection1_Impl: super::d2d1::ID2D1GradientStopCollection_Impl {
    fn GetGradientStops1(&self, gradientstops: *mut super::d2d1::D2D1_GRADIENT_STOP, gradientstopscount: u32);
    fn GetPreInterpolationSpace(&self) -> D2D1_COLOR_SPACE;
    fn GetPostInterpolationSpace(&self) -> D2D1_COLOR_SPACE;
    fn GetBufferPrecision(&self) -> D2D1_BUFFER_PRECISION;
    fn GetColorInterpolationMode(&self) -> D2D1_COLOR_INTERPOLATION_MODE;
}
#[cfg(all(feature = "Win32_d2d1", feature = "Win32_d2dbasetypes", feature = "Win32_dxgitype"))]
impl ID2D1GradientStopCollection1_Vtbl {
    pub const fn new<Identity: ID2D1GradientStopCollection1_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetGradientStops1<Identity: ID2D1GradientStopCollection1_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, gradientstops: *mut super::d2d1::D2D1_GRADIENT_STOP, gradientstopscount: u32) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID2D1GradientStopCollection1_Impl::GetGradientStops1(this, core::mem::transmute_copy(&gradientstops), core::mem::transmute_copy(&gradientstopscount));
            }
        }
        unsafe extern "system" fn GetPreInterpolationSpace<Identity: ID2D1GradientStopCollection1_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> D2D1_COLOR_SPACE {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID2D1GradientStopCollection1_Impl::GetPreInterpolationSpace(this)
            }
        }
        unsafe extern "system" fn GetPostInterpolationSpace<Identity: ID2D1GradientStopCollection1_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> D2D1_COLOR_SPACE {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID2D1GradientStopCollection1_Impl::GetPostInterpolationSpace(this)
            }
        }
        unsafe extern "system" fn GetBufferPrecision<Identity: ID2D1GradientStopCollection1_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> D2D1_BUFFER_PRECISION {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID2D1GradientStopCollection1_Impl::GetBufferPrecision(this)
            }
        }
        unsafe extern "system" fn GetColorInterpolationMode<Identity: ID2D1GradientStopCollection1_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> D2D1_COLOR_INTERPOLATION_MODE {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID2D1GradientStopCollection1_Impl::GetColorInterpolationMode(this)
            }
        }
        Self {
            base__: super::d2d1::ID2D1GradientStopCollection_Vtbl::new::<Identity, OFFSET>(),
            GetGradientStops1: GetGradientStops1::<Identity, OFFSET>,
            GetPreInterpolationSpace: GetPreInterpolationSpace::<Identity, OFFSET>,
            GetPostInterpolationSpace: GetPostInterpolationSpace::<Identity, OFFSET>,
            GetBufferPrecision: GetBufferPrecision::<Identity, OFFSET>,
            GetColorInterpolationMode: GetColorInterpolationMode::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ID2D1GradientStopCollection1 as windows_core::Interface>::IID || iid == &<super::d2d1::ID2D1Resource as windows_core::Interface>::IID || iid == &<super::d2d1::ID2D1GradientStopCollection as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_d2d1", feature = "Win32_d2dbasetypes", feature = "Win32_dxgitype"))]
impl windows_core::RuntimeName for ID2D1GradientStopCollection1 {}
#[cfg(feature = "Win32_d2d1")]
windows_core::imp::define_interface!(ID2D1ImageBrush, ID2D1ImageBrush_Vtbl, 0xfe9e984d_3f95_407c_b5db_cb94d4e8f87c);
#[cfg(feature = "Win32_d2d1")]
impl core::ops::Deref for ID2D1ImageBrush {
    type Target = super::d2d1::ID2D1Brush;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_d2d1")]
windows_core::imp::interface_hierarchy!(ID2D1ImageBrush, windows_core::IUnknown, super::d2d1::ID2D1Resource, super::d2d1::ID2D1Brush);
#[cfg(feature = "Win32_d2d1")]
impl ID2D1ImageBrush {
    pub unsafe fn SetImage<P0>(&self, image: P0)
    where
        P0: windows_core::Param<super::d2d1::ID2D1Image>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).SetImage)(windows_core::Interface::as_raw(self), image.param().abi());
        }
    }
    pub unsafe fn SetExtendModeX(&self, extendmodex: super::d2d1::D2D1_EXTEND_MODE) {
        unsafe {
            (windows_core::Interface::vtable(self).SetExtendModeX)(windows_core::Interface::as_raw(self), extendmodex);
        }
    }
    pub unsafe fn SetExtendModeY(&self, extendmodey: super::d2d1::D2D1_EXTEND_MODE) {
        unsafe {
            (windows_core::Interface::vtable(self).SetExtendModeY)(windows_core::Interface::as_raw(self), extendmodey);
        }
    }
    pub unsafe fn SetInterpolationMode(&self, interpolationmode: D2D1_INTERPOLATION_MODE) {
        unsafe {
            (windows_core::Interface::vtable(self).SetInterpolationMode)(windows_core::Interface::as_raw(self), interpolationmode);
        }
    }
    #[cfg(feature = "Win32_dcommon")]
    pub unsafe fn SetSourceRectangle(&self, sourcerectangle: *const super::dcommon::D2D_RECT_F) {
        unsafe {
            (windows_core::Interface::vtable(self).SetSourceRectangle)(windows_core::Interface::as_raw(self), sourcerectangle);
        }
    }
    pub unsafe fn GetImage(&self) -> windows_core::Result<super::d2d1::ID2D1Image> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetImage)(windows_core::Interface::as_raw(self), &mut result__);
            windows_core::Type::from_abi(result__)
        }
    }
    pub unsafe fn GetExtendModeX(&self) -> super::d2d1::D2D1_EXTEND_MODE {
        unsafe { (windows_core::Interface::vtable(self).GetExtendModeX)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn GetExtendModeY(&self) -> super::d2d1::D2D1_EXTEND_MODE {
        unsafe { (windows_core::Interface::vtable(self).GetExtendModeY)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn GetInterpolationMode(&self) -> D2D1_INTERPOLATION_MODE {
        unsafe { (windows_core::Interface::vtable(self).GetInterpolationMode)(windows_core::Interface::as_raw(self)) }
    }
    #[cfg(feature = "Win32_dcommon")]
    pub unsafe fn GetSourceRectangle(&self) -> super::dcommon::D2D_RECT_F {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetSourceRectangle)(windows_core::Interface::as_raw(self), &mut result__);
            result__
        }
    }
}
#[cfg(feature = "Win32_d2d1")]
#[repr(C)]
#[doc(hidden)]
pub struct ID2D1ImageBrush_Vtbl {
    pub base__: super::d2d1::ID2D1Brush_Vtbl,
    pub SetImage: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void),
    pub SetExtendModeX: unsafe extern "system" fn(*mut core::ffi::c_void, super::d2d1::D2D1_EXTEND_MODE),
    pub SetExtendModeY: unsafe extern "system" fn(*mut core::ffi::c_void, super::d2d1::D2D1_EXTEND_MODE),
    pub SetInterpolationMode: unsafe extern "system" fn(*mut core::ffi::c_void, D2D1_INTERPOLATION_MODE),
    #[cfg(feature = "Win32_dcommon")]
    pub SetSourceRectangle: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::dcommon::D2D_RECT_F),
    #[cfg(not(feature = "Win32_dcommon"))]
    SetSourceRectangle: usize,
    pub GetImage: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void),
    pub GetExtendModeX: unsafe extern "system" fn(*mut core::ffi::c_void) -> super::d2d1::D2D1_EXTEND_MODE,
    pub GetExtendModeY: unsafe extern "system" fn(*mut core::ffi::c_void) -> super::d2d1::D2D1_EXTEND_MODE,
    pub GetInterpolationMode: unsafe extern "system" fn(*mut core::ffi::c_void) -> D2D1_INTERPOLATION_MODE,
    #[cfg(feature = "Win32_dcommon")]
    pub GetSourceRectangle: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::dcommon::D2D_RECT_F),
    #[cfg(not(feature = "Win32_dcommon"))]
    GetSourceRectangle: usize,
}
#[cfg(all(feature = "Win32_d2d1", feature = "Win32_dcommon"))]
pub trait ID2D1ImageBrush_Impl: super::d2d1::ID2D1Brush_Impl {
    fn SetImage(&self, image: windows_core::Ref<super::d2d1::ID2D1Image>);
    fn SetExtendModeX(&self, extendmodex: super::d2d1::D2D1_EXTEND_MODE);
    fn SetExtendModeY(&self, extendmodey: super::d2d1::D2D1_EXTEND_MODE);
    fn SetInterpolationMode(&self, interpolationmode: D2D1_INTERPOLATION_MODE);
    fn SetSourceRectangle(&self, sourcerectangle: *const super::dcommon::D2D_RECT_F);
    fn GetImage(&self, image: windows_core::OutRef<super::d2d1::ID2D1Image>);
    fn GetExtendModeX(&self) -> super::d2d1::D2D1_EXTEND_MODE;
    fn GetExtendModeY(&self) -> super::d2d1::D2D1_EXTEND_MODE;
    fn GetInterpolationMode(&self) -> D2D1_INTERPOLATION_MODE;
    fn GetSourceRectangle(&self, sourcerectangle: *mut super::dcommon::D2D_RECT_F);
}
#[cfg(all(feature = "Win32_d2d1", feature = "Win32_dcommon"))]
impl ID2D1ImageBrush_Vtbl {
    pub const fn new<Identity: ID2D1ImageBrush_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn SetImage<Identity: ID2D1ImageBrush_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, image: *mut core::ffi::c_void) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID2D1ImageBrush_Impl::SetImage(this, core::mem::transmute_copy(&image));
            }
        }
        unsafe extern "system" fn SetExtendModeX<Identity: ID2D1ImageBrush_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, extendmodex: super::d2d1::D2D1_EXTEND_MODE) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID2D1ImageBrush_Impl::SetExtendModeX(this, core::mem::transmute_copy(&extendmodex));
            }
        }
        unsafe extern "system" fn SetExtendModeY<Identity: ID2D1ImageBrush_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, extendmodey: super::d2d1::D2D1_EXTEND_MODE) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID2D1ImageBrush_Impl::SetExtendModeY(this, core::mem::transmute_copy(&extendmodey));
            }
        }
        unsafe extern "system" fn SetInterpolationMode<Identity: ID2D1ImageBrush_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, interpolationmode: D2D1_INTERPOLATION_MODE) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID2D1ImageBrush_Impl::SetInterpolationMode(this, core::mem::transmute_copy(&interpolationmode));
            }
        }
        unsafe extern "system" fn SetSourceRectangle<Identity: ID2D1ImageBrush_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, sourcerectangle: *const super::dcommon::D2D_RECT_F) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID2D1ImageBrush_Impl::SetSourceRectangle(this, core::mem::transmute_copy(&sourcerectangle));
            }
        }
        unsafe extern "system" fn GetImage<Identity: ID2D1ImageBrush_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, image: *mut *mut core::ffi::c_void) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID2D1ImageBrush_Impl::GetImage(this, core::mem::transmute_copy(&image));
            }
        }
        unsafe extern "system" fn GetExtendModeX<Identity: ID2D1ImageBrush_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> super::d2d1::D2D1_EXTEND_MODE {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID2D1ImageBrush_Impl::GetExtendModeX(this)
            }
        }
        unsafe extern "system" fn GetExtendModeY<Identity: ID2D1ImageBrush_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> super::d2d1::D2D1_EXTEND_MODE {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID2D1ImageBrush_Impl::GetExtendModeY(this)
            }
        }
        unsafe extern "system" fn GetInterpolationMode<Identity: ID2D1ImageBrush_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> D2D1_INTERPOLATION_MODE {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID2D1ImageBrush_Impl::GetInterpolationMode(this)
            }
        }
        unsafe extern "system" fn GetSourceRectangle<Identity: ID2D1ImageBrush_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, sourcerectangle: *mut super::dcommon::D2D_RECT_F) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID2D1ImageBrush_Impl::GetSourceRectangle(this, core::mem::transmute_copy(&sourcerectangle));
            }
        }
        Self {
            base__: super::d2d1::ID2D1Brush_Vtbl::new::<Identity, OFFSET>(),
            SetImage: SetImage::<Identity, OFFSET>,
            SetExtendModeX: SetExtendModeX::<Identity, OFFSET>,
            SetExtendModeY: SetExtendModeY::<Identity, OFFSET>,
            SetInterpolationMode: SetInterpolationMode::<Identity, OFFSET>,
            SetSourceRectangle: SetSourceRectangle::<Identity, OFFSET>,
            GetImage: GetImage::<Identity, OFFSET>,
            GetExtendModeX: GetExtendModeX::<Identity, OFFSET>,
            GetExtendModeY: GetExtendModeY::<Identity, OFFSET>,
            GetInterpolationMode: GetInterpolationMode::<Identity, OFFSET>,
            GetSourceRectangle: GetSourceRectangle::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ID2D1ImageBrush as windows_core::Interface>::IID || iid == &<super::d2d1::ID2D1Resource as windows_core::Interface>::IID || iid == &<super::d2d1::ID2D1Brush as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_d2d1", feature = "Win32_dcommon"))]
impl windows_core::RuntimeName for ID2D1ImageBrush {}
windows_core::imp::define_interface!(ID2D1Multithread, ID2D1Multithread_Vtbl, 0x31e6e7bc_e0ff_4d46_8c64_a0a8c41c15d3);
windows_core::imp::interface_hierarchy!(ID2D1Multithread, windows_core::IUnknown);
impl ID2D1Multithread {
    pub unsafe fn GetMultithreadProtected(&self) -> windows_core::BOOL {
        unsafe { (windows_core::Interface::vtable(self).GetMultithreadProtected)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn Enter(&self) {
        unsafe {
            (windows_core::Interface::vtable(self).Enter)(windows_core::Interface::as_raw(self));
        }
    }
    pub unsafe fn Leave(&self) {
        unsafe {
            (windows_core::Interface::vtable(self).Leave)(windows_core::Interface::as_raw(self));
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ID2D1Multithread_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetMultithreadProtected: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::BOOL,
    pub Enter: unsafe extern "system" fn(*mut core::ffi::c_void),
    pub Leave: unsafe extern "system" fn(*mut core::ffi::c_void),
}
pub trait ID2D1Multithread_Impl: windows_core::IUnknownImpl {
    fn GetMultithreadProtected(&self) -> windows_core::BOOL;
    fn Enter(&self);
    fn Leave(&self);
}
impl ID2D1Multithread_Vtbl {
    pub const fn new<Identity: ID2D1Multithread_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetMultithreadProtected<Identity: ID2D1Multithread_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::BOOL {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID2D1Multithread_Impl::GetMultithreadProtected(this)
            }
        }
        unsafe extern "system" fn Enter<Identity: ID2D1Multithread_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID2D1Multithread_Impl::Enter(this);
            }
        }
        unsafe extern "system" fn Leave<Identity: ID2D1Multithread_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID2D1Multithread_Impl::Leave(this);
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetMultithreadProtected: GetMultithreadProtected::<Identity, OFFSET>,
            Enter: Enter::<Identity, OFFSET>,
            Leave: Leave::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ID2D1Multithread as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for ID2D1Multithread {}
#[cfg(feature = "Win32_d2d1")]
windows_core::imp::define_interface!(ID2D1PathGeometry1, ID2D1PathGeometry1_Vtbl, 0x62baa2d2_ab54_41b7_b872_787e0106a421);
#[cfg(feature = "Win32_d2d1")]
impl core::ops::Deref for ID2D1PathGeometry1 {
    type Target = super::d2d1::ID2D1PathGeometry;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_d2d1")]
windows_core::imp::interface_hierarchy!(ID2D1PathGeometry1, windows_core::IUnknown, super::d2d1::ID2D1Resource, super::d2d1::ID2D1Geometry, super::d2d1::ID2D1PathGeometry);
#[cfg(feature = "Win32_d2d1")]
impl ID2D1PathGeometry1 {
    pub unsafe fn ComputePointAndSegmentAtLength(&self, length: f32, startsegment: u32, worldtransform: Option<*const windows_numerics::Matrix3x2>, flatteningtolerance: f32, pointdescription: *mut D2D1_POINT_DESCRIPTION) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).ComputePointAndSegmentAtLength)(windows_core::Interface::as_raw(self), length, startsegment, worldtransform.unwrap_or(core::mem::zeroed()) as _, flatteningtolerance, pointdescription as _) }
    }
}
#[cfg(feature = "Win32_d2d1")]
#[repr(C)]
#[doc(hidden)]
pub struct ID2D1PathGeometry1_Vtbl {
    pub base__: super::d2d1::ID2D1PathGeometry_Vtbl,
    pub ComputePointAndSegmentAtLength: unsafe extern "system" fn(*mut core::ffi::c_void, f32, u32, *const windows_numerics::Matrix3x2, f32, *mut D2D1_POINT_DESCRIPTION) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_d2d1", feature = "Win32_dcommon"))]
pub trait ID2D1PathGeometry1_Impl: super::d2d1::ID2D1PathGeometry_Impl {
    fn ComputePointAndSegmentAtLength(&self, length: f32, startsegment: u32, worldtransform: *const windows_numerics::Matrix3x2, flatteningtolerance: f32, pointdescription: *mut D2D1_POINT_DESCRIPTION) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_d2d1", feature = "Win32_dcommon"))]
impl ID2D1PathGeometry1_Vtbl {
    pub const fn new<Identity: ID2D1PathGeometry1_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn ComputePointAndSegmentAtLength<Identity: ID2D1PathGeometry1_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, length: f32, startsegment: u32, worldtransform: *const windows_numerics::Matrix3x2, flatteningtolerance: f32, pointdescription: *mut D2D1_POINT_DESCRIPTION) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID2D1PathGeometry1_Impl::ComputePointAndSegmentAtLength(this, core::mem::transmute_copy(&length), core::mem::transmute_copy(&startsegment), core::mem::transmute_copy(&worldtransform), core::mem::transmute_copy(&flatteningtolerance), core::mem::transmute_copy(&pointdescription)).into()
            }
        }
        Self {
            base__: super::d2d1::ID2D1PathGeometry_Vtbl::new::<Identity, OFFSET>(),
            ComputePointAndSegmentAtLength: ComputePointAndSegmentAtLength::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ID2D1PathGeometry1 as windows_core::Interface>::IID || iid == &<super::d2d1::ID2D1Resource as windows_core::Interface>::IID || iid == &<super::d2d1::ID2D1Geometry as windows_core::Interface>::IID || iid == &<super::d2d1::ID2D1PathGeometry as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_d2d1", feature = "Win32_dcommon"))]
impl windows_core::RuntimeName for ID2D1PathGeometry1 {}
windows_core::imp::define_interface!(ID2D1PrintControl, ID2D1PrintControl_Vtbl, 0x2c1d867d_c290_41c8_ae7e_34a98702e9a5);
windows_core::imp::interface_hierarchy!(ID2D1PrintControl, windows_core::IUnknown);
impl ID2D1PrintControl {
    #[cfg(all(feature = "Win32_d2d1", feature = "Win32_dcommon", feature = "Win32_objidlbase"))]
    pub unsafe fn AddPage<P0, P2>(&self, commandlist: P0, pagesize: super::dcommon::D2D_SIZE_F, pageprintticketstream: P2, tag1: Option<*mut super::d2d1::D2D1_TAG>, tag2: Option<*mut super::d2d1::D2D1_TAG>) -> windows_core::HRESULT
    where
        P0: windows_core::Param<ID2D1CommandList>,
        P2: windows_core::Param<super::objidlbase::IStream>,
    {
        unsafe { (windows_core::Interface::vtable(self).AddPage)(windows_core::Interface::as_raw(self), commandlist.param().abi(), core::mem::transmute(pagesize), pageprintticketstream.param().abi(), tag1.unwrap_or(core::mem::zeroed()) as _, tag2.unwrap_or(core::mem::zeroed()) as _) }
    }
    pub unsafe fn Close(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Close)(windows_core::Interface::as_raw(self)) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ID2D1PrintControl_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(all(feature = "Win32_d2d1", feature = "Win32_dcommon", feature = "Win32_objidlbase"))]
    pub AddPage: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, super::dcommon::D2D_SIZE_F, *mut core::ffi::c_void, *mut super::d2d1::D2D1_TAG, *mut super::d2d1::D2D1_TAG) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_d2d1", feature = "Win32_dcommon", feature = "Win32_objidlbase")))]
    AddPage: usize,
    pub Close: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_d2d1", feature = "Win32_dcommon", feature = "Win32_objidlbase"))]
pub trait ID2D1PrintControl_Impl: windows_core::IUnknownImpl {
    fn AddPage(&self, commandlist: windows_core::Ref<ID2D1CommandList>, pagesize: &super::dcommon::D2D_SIZE_F, pageprintticketstream: windows_core::Ref<super::objidlbase::IStream>, tag1: *mut super::d2d1::D2D1_TAG, tag2: *mut super::d2d1::D2D1_TAG) -> windows_core::Result<()>;
    fn Close(&self) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_d2d1", feature = "Win32_dcommon", feature = "Win32_objidlbase"))]
impl ID2D1PrintControl_Vtbl {
    pub const fn new<Identity: ID2D1PrintControl_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn AddPage<Identity: ID2D1PrintControl_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, commandlist: *mut core::ffi::c_void, pagesize: super::dcommon::D2D_SIZE_F, pageprintticketstream: *mut core::ffi::c_void, tag1: *mut super::d2d1::D2D1_TAG, tag2: *mut super::d2d1::D2D1_TAG) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID2D1PrintControl_Impl::AddPage(this, core::mem::transmute_copy(&commandlist), core::mem::transmute(&pagesize), core::mem::transmute_copy(&pageprintticketstream), core::mem::transmute_copy(&tag1), core::mem::transmute_copy(&tag2)).into()
            }
        }
        unsafe extern "system" fn Close<Identity: ID2D1PrintControl_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID2D1PrintControl_Impl::Close(this).into()
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), AddPage: AddPage::<Identity, OFFSET>, Close: Close::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ID2D1PrintControl as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_d2d1", feature = "Win32_dcommon", feature = "Win32_objidlbase"))]
impl windows_core::RuntimeName for ID2D1PrintControl {}
windows_core::imp::define_interface!(ID2D1Properties, ID2D1Properties_Vtbl, 0x483473d7_cd46_4f9d_9d3a_3112aa80159d);
windows_core::imp::interface_hierarchy!(ID2D1Properties, windows_core::IUnknown);
impl ID2D1Properties {
    pub unsafe fn GetPropertyCount(&self) -> u32 {
        unsafe { (windows_core::Interface::vtable(self).GetPropertyCount)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn GetPropertyName(&self, index: u32, name: &mut [u16]) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetPropertyName)(windows_core::Interface::as_raw(self), index, core::mem::transmute(name.as_ptr()), name.len().try_into().unwrap()) }
    }
    pub unsafe fn GetPropertyNameLength(&self, index: u32) -> u32 {
        unsafe { (windows_core::Interface::vtable(self).GetPropertyNameLength)(windows_core::Interface::as_raw(self), index) }
    }
    pub unsafe fn GetType(&self, index: u32) -> D2D1_PROPERTY_TYPE {
        unsafe { (windows_core::Interface::vtable(self).GetType)(windows_core::Interface::as_raw(self), index) }
    }
    pub unsafe fn GetPropertyIndex<P0>(&self, name: P0) -> u32
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).GetPropertyIndex)(windows_core::Interface::as_raw(self), name.param().abi()) }
    }
    pub unsafe fn SetValueByName<P0>(&self, name: P0, r#type: D2D1_PROPERTY_TYPE, data: &[u8]) -> windows_core::HRESULT
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetValueByName)(windows_core::Interface::as_raw(self), name.param().abi(), r#type, core::mem::transmute(data.as_ptr()), data.len().try_into().unwrap()) }
    }
    pub unsafe fn SetValue(&self, index: u32, r#type: D2D1_PROPERTY_TYPE, data: &[u8]) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetValue)(windows_core::Interface::as_raw(self), index, r#type, core::mem::transmute(data.as_ptr()), data.len().try_into().unwrap()) }
    }
    pub unsafe fn GetValueByName<P0>(&self, name: P0, r#type: D2D1_PROPERTY_TYPE, data: &mut [u8]) -> windows_core::HRESULT
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).GetValueByName)(windows_core::Interface::as_raw(self), name.param().abi(), r#type, core::mem::transmute(data.as_ptr()), data.len().try_into().unwrap()) }
    }
    pub unsafe fn GetValue(&self, index: u32, r#type: D2D1_PROPERTY_TYPE, data: &mut [u8]) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetValue)(windows_core::Interface::as_raw(self), index, r#type, core::mem::transmute(data.as_ptr()), data.len().try_into().unwrap()) }
    }
    pub unsafe fn GetValueSize(&self, index: u32) -> u32 {
        unsafe { (windows_core::Interface::vtable(self).GetValueSize)(windows_core::Interface::as_raw(self), index) }
    }
    pub unsafe fn GetSubProperties(&self, index: u32) -> windows_core::Result<Self> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetSubProperties)(windows_core::Interface::as_raw(self), index, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ID2D1Properties_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetPropertyCount: unsafe extern "system" fn(*mut core::ffi::c_void) -> u32,
    pub GetPropertyName: unsafe extern "system" fn(*mut core::ffi::c_void, u32, windows_core::PWSTR, u32) -> windows_core::HRESULT,
    pub GetPropertyNameLength: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> u32,
    pub GetType: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> D2D1_PROPERTY_TYPE,
    pub GetPropertyIndex: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR) -> u32,
    pub SetValueByName: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, D2D1_PROPERTY_TYPE, *const u8, u32) -> windows_core::HRESULT,
    pub SetValue: unsafe extern "system" fn(*mut core::ffi::c_void, u32, D2D1_PROPERTY_TYPE, *const u8, u32) -> windows_core::HRESULT,
    pub GetValueByName: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, D2D1_PROPERTY_TYPE, *mut u8, u32) -> windows_core::HRESULT,
    pub GetValue: unsafe extern "system" fn(*mut core::ffi::c_void, u32, D2D1_PROPERTY_TYPE, *mut u8, u32) -> windows_core::HRESULT,
    pub GetValueSize: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> u32,
    pub GetSubProperties: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait ID2D1Properties_Impl: windows_core::IUnknownImpl {
    fn GetPropertyCount(&self) -> u32;
    fn GetPropertyName(&self, index: u32, name: windows_core::PWSTR, namecount: u32) -> windows_core::Result<()>;
    fn GetPropertyNameLength(&self, index: u32) -> u32;
    fn GetType(&self, index: u32) -> D2D1_PROPERTY_TYPE;
    fn GetPropertyIndex(&self, name: &windows_core::PCWSTR) -> u32;
    fn SetValueByName(&self, name: &windows_core::PCWSTR, r#type: D2D1_PROPERTY_TYPE, data: *const u8, datasize: u32) -> windows_core::Result<()>;
    fn SetValue(&self, index: u32, r#type: D2D1_PROPERTY_TYPE, data: *const u8, datasize: u32) -> windows_core::Result<()>;
    fn GetValueByName(&self, name: &windows_core::PCWSTR, r#type: D2D1_PROPERTY_TYPE, data: *mut u8, datasize: u32) -> windows_core::Result<()>;
    fn GetValue(&self, index: u32, r#type: D2D1_PROPERTY_TYPE, data: *mut u8, datasize: u32) -> windows_core::Result<()>;
    fn GetValueSize(&self, index: u32) -> u32;
    fn GetSubProperties(&self, index: u32) -> windows_core::Result<ID2D1Properties>;
}
impl ID2D1Properties_Vtbl {
    pub const fn new<Identity: ID2D1Properties_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetPropertyCount<Identity: ID2D1Properties_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> u32 {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID2D1Properties_Impl::GetPropertyCount(this)
            }
        }
        unsafe extern "system" fn GetPropertyName<Identity: ID2D1Properties_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: u32, name: windows_core::PWSTR, namecount: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID2D1Properties_Impl::GetPropertyName(this, core::mem::transmute_copy(&index), core::mem::transmute_copy(&name), core::mem::transmute_copy(&namecount)).into()
            }
        }
        unsafe extern "system" fn GetPropertyNameLength<Identity: ID2D1Properties_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: u32) -> u32 {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID2D1Properties_Impl::GetPropertyNameLength(this, core::mem::transmute_copy(&index))
            }
        }
        unsafe extern "system" fn GetType<Identity: ID2D1Properties_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: u32) -> D2D1_PROPERTY_TYPE {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID2D1Properties_Impl::GetType(this, core::mem::transmute_copy(&index))
            }
        }
        unsafe extern "system" fn GetPropertyIndex<Identity: ID2D1Properties_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, name: windows_core::PCWSTR) -> u32 {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID2D1Properties_Impl::GetPropertyIndex(this, core::mem::transmute(&name))
            }
        }
        unsafe extern "system" fn SetValueByName<Identity: ID2D1Properties_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, name: windows_core::PCWSTR, r#type: D2D1_PROPERTY_TYPE, data: *const u8, datasize: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID2D1Properties_Impl::SetValueByName(this, core::mem::transmute(&name), core::mem::transmute_copy(&r#type), core::mem::transmute_copy(&data), core::mem::transmute_copy(&datasize)).into()
            }
        }
        unsafe extern "system" fn SetValue<Identity: ID2D1Properties_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: u32, r#type: D2D1_PROPERTY_TYPE, data: *const u8, datasize: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID2D1Properties_Impl::SetValue(this, core::mem::transmute_copy(&index), core::mem::transmute_copy(&r#type), core::mem::transmute_copy(&data), core::mem::transmute_copy(&datasize)).into()
            }
        }
        unsafe extern "system" fn GetValueByName<Identity: ID2D1Properties_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, name: windows_core::PCWSTR, r#type: D2D1_PROPERTY_TYPE, data: *mut u8, datasize: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID2D1Properties_Impl::GetValueByName(this, core::mem::transmute(&name), core::mem::transmute_copy(&r#type), core::mem::transmute_copy(&data), core::mem::transmute_copy(&datasize)).into()
            }
        }
        unsafe extern "system" fn GetValue<Identity: ID2D1Properties_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: u32, r#type: D2D1_PROPERTY_TYPE, data: *mut u8, datasize: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID2D1Properties_Impl::GetValue(this, core::mem::transmute_copy(&index), core::mem::transmute_copy(&r#type), core::mem::transmute_copy(&data), core::mem::transmute_copy(&datasize)).into()
            }
        }
        unsafe extern "system" fn GetValueSize<Identity: ID2D1Properties_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: u32) -> u32 {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID2D1Properties_Impl::GetValueSize(this, core::mem::transmute_copy(&index))
            }
        }
        unsafe extern "system" fn GetSubProperties<Identity: ID2D1Properties_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: u32, subproperties: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ID2D1Properties_Impl::GetSubProperties(this, core::mem::transmute_copy(&index)) {
                    Ok(ok__) => {
                        subproperties.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetPropertyCount: GetPropertyCount::<Identity, OFFSET>,
            GetPropertyName: GetPropertyName::<Identity, OFFSET>,
            GetPropertyNameLength: GetPropertyNameLength::<Identity, OFFSET>,
            GetType: GetType::<Identity, OFFSET>,
            GetPropertyIndex: GetPropertyIndex::<Identity, OFFSET>,
            SetValueByName: SetValueByName::<Identity, OFFSET>,
            SetValue: SetValue::<Identity, OFFSET>,
            GetValueByName: GetValueByName::<Identity, OFFSET>,
            GetValue: GetValue::<Identity, OFFSET>,
            GetValueSize: GetValueSize::<Identity, OFFSET>,
            GetSubProperties: GetSubProperties::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ID2D1Properties as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for ID2D1Properties {}
#[cfg(feature = "Win32_d2d1")]
windows_core::imp::define_interface!(ID2D1StrokeStyle1, ID2D1StrokeStyle1_Vtbl, 0x10a72a66_e91c_43f4_993f_ddf4b82b0b4a);
#[cfg(feature = "Win32_d2d1")]
impl core::ops::Deref for ID2D1StrokeStyle1 {
    type Target = super::d2d1::ID2D1StrokeStyle;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_d2d1")]
windows_core::imp::interface_hierarchy!(ID2D1StrokeStyle1, windows_core::IUnknown, super::d2d1::ID2D1Resource, super::d2d1::ID2D1StrokeStyle);
#[cfg(feature = "Win32_d2d1")]
impl ID2D1StrokeStyle1 {
    pub unsafe fn GetStrokeTransformType(&self) -> D2D1_STROKE_TRANSFORM_TYPE {
        unsafe { (windows_core::Interface::vtable(self).GetStrokeTransformType)(windows_core::Interface::as_raw(self)) }
    }
}
#[cfg(feature = "Win32_d2d1")]
#[repr(C)]
#[doc(hidden)]
pub struct ID2D1StrokeStyle1_Vtbl {
    pub base__: super::d2d1::ID2D1StrokeStyle_Vtbl,
    pub GetStrokeTransformType: unsafe extern "system" fn(*mut core::ffi::c_void) -> D2D1_STROKE_TRANSFORM_TYPE,
}
#[cfg(feature = "Win32_d2d1")]
pub trait ID2D1StrokeStyle1_Impl: super::d2d1::ID2D1StrokeStyle_Impl {
    fn GetStrokeTransformType(&self) -> D2D1_STROKE_TRANSFORM_TYPE;
}
#[cfg(feature = "Win32_d2d1")]
impl ID2D1StrokeStyle1_Vtbl {
    pub const fn new<Identity: ID2D1StrokeStyle1_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetStrokeTransformType<Identity: ID2D1StrokeStyle1_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> D2D1_STROKE_TRANSFORM_TYPE {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID2D1StrokeStyle1_Impl::GetStrokeTransformType(this)
            }
        }
        Self { base__: super::d2d1::ID2D1StrokeStyle_Vtbl::new::<Identity, OFFSET>(), GetStrokeTransformType: GetStrokeTransformType::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ID2D1StrokeStyle1 as windows_core::Interface>::IID || iid == &<super::d2d1::ID2D1Resource as windows_core::Interface>::IID || iid == &<super::d2d1::ID2D1StrokeStyle as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_d2d1")]
impl windows_core::RuntimeName for ID2D1StrokeStyle1 {}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct IPrintDocumentPackageTarget(pub u8);
pub type PD2D1_EFFECT_FACTORY = Option<unsafe extern "system" fn(effectimpl: windows_core::OutRef<windows_core::IUnknown>) -> windows_core::HRESULT>;
